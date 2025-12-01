use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{atomic::{AtomicU64, Ordering}, mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

/// Unique identifier for a queued job.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct JobId(u64);

/// Kind of work the job queue can represent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JobKind {
    Copy { src: PathBuf, dst: PathBuf },
    Move { src: PathBuf, dst: PathBuf },
    Delete { target: PathBuf },
}

/// Status for a job as observed by callers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JobStatus {
    Pending,
    Running,
    Paused,
    Completed,
    Cancelled,
    Failed(String),
}

/// Observable state for a job.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JobState {
    pub status: JobStatus,
    pub progress: u8, // 0..=100
}

enum WorkerMsg {
    Enqueue(JobId, JobKind),
    Shutdown,
}

/// Minimal job queue with a dummy worker. Intended as scaffolding for future IO-bound work.
pub struct JobQueue {
    tx: mpsc::Sender<WorkerMsg>,
    states: Arc<Mutex<HashMap<JobId, JobState>>>,
    _handle: thread::JoinHandle<()>,
}

impl JobQueue {
    /// Create a new queue and spawn the background worker thread.
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel::<WorkerMsg>();
        let states: Arc<Mutex<HashMap<JobId, JobState>>> = Arc::new(Mutex::new(HashMap::new()));
        let states_clone = Arc::clone(&states);
        let handle = thread::spawn(move || worker_loop(rx, states_clone));
        JobQueue { tx, states, _handle: handle }
    }

    /// Enqueue a new job and return its id.
    pub fn enqueue(&self, kind: JobKind) -> JobId {
        let id = JobId(NEXT_ID.fetch_add(1, Ordering::Relaxed));
        {
            let mut st = self.states.lock().unwrap();
            st.insert(id, JobState { status: JobStatus::Pending, progress: 0 });
        }
        let _ = self.tx.send(WorkerMsg::Enqueue(id, kind));
        id
    }

    /// Pause a running job (no-op if already paused or finished).
    pub fn pause(&self, id: JobId) {
        update_status(&self.states, id, JobStatus::Paused);
    }

    /// Resume a paused job.
    pub fn resume(&self, id: JobId) {
        update_status_running(&self.states, id);
    }

    /// Cancel a job.
    pub fn cancel(&self, id: JobId) {
        update_status(&self.states, id, JobStatus::Cancelled);
    }

    /// Snapshot the current state of a job.
    pub fn state(&self, id: JobId) -> Option<JobState> {
        self.states.lock().unwrap().get(&id).cloned()
    }
}

impl Default for JobQueue {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for JobQueue {
    fn drop(&mut self) {
        let _ = self.tx.send(WorkerMsg::Shutdown);
    }
}

static NEXT_ID: AtomicU64 = AtomicU64::new(1);

fn worker_loop(rx: mpsc::Receiver<WorkerMsg>, states: Arc<Mutex<HashMap<JobId, JobState>>>) {
    while let Ok(msg) = rx.recv() {
        match msg {
            WorkerMsg::Enqueue(id, kind) => {
                let states = Arc::clone(&states);
                thread::spawn(move || run_job(id, kind, &states));
            }
            WorkerMsg::Shutdown => break,
        }
    }
}

fn run_job(id: JobId, _kind: JobKind, states: &Arc<Mutex<HashMap<JobId, JobState>>>) {
    update_status_running(states, id);
    // Dummy progress: 10 steps with pause/cancel checks.
    for step in 1..=10 {
        thread::sleep(Duration::from_millis(30));
        // Check for pause/cancel signals delivered via shared state.
        loop {
            let status = states.lock().unwrap().get(&id).map(|s| s.status.clone());
            match status {
                Some(JobStatus::Paused) => {
                    thread::sleep(Duration::from_millis(5));
                    continue;
                }
                Some(JobStatus::Cancelled) => return,
                _ => break,
            }
        }

        let progress = (step * 10).min(100) as u8;
        let mut guard = states.lock().unwrap();
        if let Some(st) = guard.get_mut(&id) {
            if matches!(st.status, JobStatus::Cancelled) {
                return;
            }
            st.progress = progress;
            st.status = JobStatus::Running;
        }
    }
    update_status(states, id, JobStatus::Completed);
}

fn update_status(states: &Arc<Mutex<HashMap<JobId, JobState>>>, id: JobId, status: JobStatus) {
    let mut guard = states.lock().unwrap();
    if let Some(st) = guard.get_mut(&id) {
        st.status = status;
    }
}

fn update_status_running(states: &Arc<Mutex<HashMap<JobId, JobState>>>, id: JobId) {
    let mut guard = states.lock().unwrap();
    if let Some(st) = guard.get_mut(&id) {
        st.status = JobStatus::Running;
    } else {
        guard.insert(id, JobState { status: JobStatus::Running, progress: 0 });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn wait_for_status(q: &JobQueue, id: JobId, want: JobStatus) {
        let deadline = std::time::Instant::now() + Duration::from_millis(500);
        loop {
            if let Some(st) = q.state(id) {
                if st.status == want {
                    return;
                }
            }
            if std::time::Instant::now() > deadline {
                panic!("timed out waiting for status {:?}", want);
            }
            thread::sleep(Duration::from_millis(5));
        }
    }

    #[test]
    fn enqueue_and_complete_job() {
        let q = JobQueue::new();
        let id = q.enqueue(JobKind::Delete { target: PathBuf::from("/tmp/x") });
        wait_for_status(&q, id, JobStatus::Completed);
    }

    #[test]
    fn pause_and_resume_job() {
        let q = JobQueue::new();
        let id = q.enqueue(JobKind::Copy { src: PathBuf::from("/a"), dst: PathBuf::from("/b") });
        q.pause(id);
        wait_for_status(&q, id, JobStatus::Paused);
        q.resume(id);
        wait_for_status(&q, id, JobStatus::Completed);
    }

    #[test]
    fn cancel_job() {
        let q = JobQueue::new();
        let id = q.enqueue(JobKind::Move { src: PathBuf::from("/a"), dst: PathBuf::from("/b") });
        q.cancel(id);
        wait_for_status(&q, id, JobStatus::Cancelled);
    }
}
