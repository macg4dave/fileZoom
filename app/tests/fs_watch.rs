#![cfg(feature = "fs-watch")]

use std::time::Duration;

use assert_fs::prelude::*;
use fileZoom::fs_op::watcher::{FsEvent, spawn_watcher};

#[test]
fn watcher_receives_create_event() {
    let temp = assert_fs::TempDir::new().unwrap();
    let path = temp.path().to_path_buf();

    let (tx, rx) = std::sync::mpsc::channel::<FsEvent>();
    let (stop_tx, stop_rx) = std::sync::mpsc::channel::<()>();

    let handle = spawn_watcher(path.clone(), tx, stop_rx);

    // Give watcher a moment to initialize, then create a new file which should trigger an event.
    std::thread::sleep(Duration::from_millis(200));
    let file = temp.child("newfile.txt");
    file.write_str("hello").unwrap();

    let got = rx.recv_timeout(Duration::from_secs(5));
    // Stop the watcher and join thread
    let _ = stop_tx.send(());
    let _ = handle.join();

    assert!(got.is_ok(), "expected an FsEvent but timed out");
    if let Ok(evt) = got {
        match evt {
            FsEvent::Create(p) => {
                assert!(p.ends_with("newfile.txt"));
            }
            FsEvent::Modify(p) => {
                // Some platforms may report Modify instead of Create; accept either.
                assert!(p.ends_with("newfile.txt"));
            }
            other => panic!("unexpected event: {:?}", other),
        }
    }
}
