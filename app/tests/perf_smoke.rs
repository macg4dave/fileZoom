use assert_fs::prelude::*;
use fileZoom::app::{App, StartOptions};
use std::time::Instant;

/// Coarse perf/IO smoke: populate a temp tree and ensure startup+refresh stay under a generous cap.
#[test]
fn startup_and_refresh_under_smoke_cap() {
    // Build a moderate fixture set (few hundred files/dirs) to exercise IO without being flaky.
    let temp = assert_fs::TempDir::new().expect("tempdir");
    for i in 0..200u32 {
        let f = temp.child(format!("file_{i}.txt"));
        f.write_str("data").expect("write file");
    }
    for i in 0..20u32 {
        let dir = temp.child(format!("dir_{i}"));
        dir.create_dir_all().expect("create dir");
        dir.child("nested.txt").write_str("x").expect("write nested");
    }

    let opts = StartOptions { start_dir: Some(temp.path().to_path_buf()), ..Default::default() };

    let start = Instant::now();
    let mut app = App::with_options(&opts).expect("init app");
    let init_elapsed = start.elapsed();

    let refresh_start = Instant::now();
    app.refresh().expect("refresh");
    let refresh_elapsed = refresh_start.elapsed();

    // Generous caps to avoid flakiness on CI; tighten as we harden perf.
    assert!(
        init_elapsed.as_secs_f32() < 1.5,
        "startup too slow: {:.3}s",
        init_elapsed.as_secs_f32()
    );
    assert!(
        refresh_elapsed.as_secs_f32() < 1.0,
        "refresh too slow: {:.3}s",
        refresh_elapsed.as_secs_f32()
    );
}
