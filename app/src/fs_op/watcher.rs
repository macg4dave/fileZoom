#![cfg(feature = "fs-watch")]

use notify::{Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::PathBuf;
use std::sync::mpsc::{Receiver as MpscReceiver, Sender};

/// Filesystem event detailed enough for the app to decide what to refresh.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FsEvent {
    Create(PathBuf),
    Modify(PathBuf),
    Remove(PathBuf),
    Rename(PathBuf, PathBuf),
    Other,
}

/// Spawn a watcher thread that sends `FsEvent` into `tx` for relevant events.
/// The returned `JoinHandle` is the thread that owns the watcher and keeps it
/// alive for the program lifetime. This is intentionally simple; later we may
/// improve lifecycle management so watchers can be started/stopped on demand.
pub fn spawn_watcher(path: PathBuf, tx: Sender<FsEvent>, stop_rx: MpscReceiver<()>) -> std::thread::JoinHandle<()> {
    std::thread::spawn(move || {
        let cb_tx = tx.clone();
        let res: notify::Result<RecommendedWatcher> = RecommendedWatcher::new(
            move |res: notify::Result<Event>| match res {
                Ok(event) => {
                    // If multiple paths included, treat as a potential rename
                    if event.paths.len() >= 2 {
                        let a = event.paths.get(0).cloned();
                        let b = event.paths.get(1).cloned();
                        if let (Some(a), Some(b)) = (a, b) {
                            let _ = cb_tx.send(FsEvent::Rename(a, b));
                            return;
                        }
                    }

                    if let Some(p) = event.paths.get(0) {
                        match &event.kind {
                            EventKind::Create(_) => {
                                let _ = cb_tx.send(FsEvent::Create(p.clone()));
                            }
                            EventKind::Modify(_) => {
                                let _ = cb_tx.send(FsEvent::Modify(p.clone()));
                            }
                            EventKind::Remove(_) => {
                                let _ = cb_tx.send(FsEvent::Remove(p.clone()));
                            }
                            _ => {
                                let _ = cb_tx.send(FsEvent::Other);
                            }
                        }
                    }
                }
                Err(e) => tracing::error!("file watcher error: {:#?}", e),
            },
            Config::default(),
        );

        match res {
            Ok(mut watcher) => {
                // Use recursive watching so changes in subdirectories are observed.
                if let Err(e) = watcher.watch(&path, RecursiveMode::Recursive) {
                    tracing::error!("failed to watch {}: {:#?}", path.display(), e);
                    return;
                }
                // Block until stop signal is received to keep watcher alive.
                let _ = stop_rx.recv();
            }
            Err(e) => tracing::error!("failed to create watcher for {}: {:#?}", path.display(), e),
        }
    })
}
