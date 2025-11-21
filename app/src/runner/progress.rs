use std::fmt;
use std::path::PathBuf;

/// User decision sent from the UI to a background worker when a
/// conflicting target is reported during a file operation.
///
/// Typical usage: the worker reports a conflict by sending a
/// `ProgressUpdate` with `conflict = Some(path)`. The UI prompts the
/// user and sends one of these variants back on the decision channel.
///
/// Variants:
/// - `Overwrite`: overwrite this target.
/// - `Skip`: skip this item.
/// - `OverwriteAll`: overwrite this and all subsequent conflicts.
/// - `SkipAll`: skip this and all subsequent conflicts.
/// - `Cancel`: abort the whole operation.
#[derive(Clone, Debug)]
pub enum OperationDecision {
    Overwrite,
    Skip,
    OverwriteAll,
    SkipAll,
    Cancel,
}

/// ProgressUpdate is sent by background workers to the UI to report
/// progress and to request conflict resolution.
///
/// Protocol summary:
/// - `processed` / `total`: progress counters updated as items complete.
/// - `message`: optional human-friendly status text.
/// - `done`: true when the worker finished (either successfully or
///   due to an error/cancellation).
/// - `error`: optional error message when `done == true` and an error
///   occurred.
/// - `conflict`: when `Some(path)`, the worker is blocked waiting for
///   an `OperationDecision` from the UI for that `path`.
///
/// Example sequence:
/// 1. Worker -> ProgressUpdate { processed:0, total:N, message:Some("Starting"), done:false, conflict:None }
/// 2. Worker -> ProgressUpdate { processed:i, total:N, message:Some("Copied ..."), done:false, conflict:None }
/// 3. Worker -> ProgressUpdate { processed:i, total:N, message:Some("Conflict"), done:false, conflict:Some(path) }
/// 4. UI -> OperationDecision::Skip (sent via decision channel)
/// 5. Worker continues, eventually sending ProgressUpdate { processed:N, total:N, done:true }
#[derive(Clone, Debug)]
pub struct ProgressUpdate {
    pub processed: usize,
    pub total: usize,
    /// Optional human-friendly message
    pub message: Option<String>,
    /// Whether the operation completed (successful or with error)
    pub done: bool,
    /// Optional error message if done==true and an error occurred
    pub error: Option<String>,
    /// If present, worker is reporting a conflict for this `PathBuf` and
    /// is waiting for a decision from the UI thread.
    pub conflict: Option<PathBuf>,
}

impl fmt::Display for ProgressUpdate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}/{} ({})",
            self.processed,
            self.total,
            self.message.as_deref().unwrap_or("")
        )
    }
}
