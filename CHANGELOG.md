# Changelog

## Unreleased

- Add Command Line and Menu Bar integration
  - New `CommandLineState` in `App` and `ui::command_line` drawing/handler.
  - Simple command registry (`app::runner::commands::execute_command`) supporting `toggle-preview`, `menu-next`, `menu-prev`, and `menu-activate`.
  - Menu bar now activates on left-click (mouse handler updated).
  - Integration tests added: `ui_menu_interaction.rs` and `menu_commandline_feature.rs`.
  - Minor wiring in `ui::mod` and `runner` handlers to route keys and mouse events.

- Replace manual recursive filesystem walks with `walkdir` where appropriate
  (improves robustness and reduces manual recursion code). Files updated:
  - `app/src/fs_op/copy.rs`
  - `app/src/fs_op/mv.rs`
  - `app/src/building/make_fakefs_lib.rs`

- Adopt `fs_extra` for file/directory copy operations and add metadata
  preservation hooks:
  - Use `fs_extra` for recursive and batch copies where appropriate to
    improve throughput and simplify implementation.
  - Preserve permissions and timestamps (best-effort) after copies to
    better retain source metadata; atomic single-file copies still use
    the project's `atomic_copy_file` helper to avoid exposing partially
    written files.
  - `CopyOptions` tuned for the project: 64 KiB buffer and `overwrite = false`.

- Filesystem watching (optional):
  - Add an optional feature `fs-watch` (gated behind Cargo features) which
    enables filesystem watching via the `notify` crate.
  - The watcher runs in a background thread and sends `FsEvent` messages
    into the runner; the event loop now maps events to affected panel(s)
    and performs a per-panel refresh rather than always refreshing both
    panels. This reduces unnecessary work and improves responsiveness.
  - Watcher behavior is recursive by default (subdirectories are observed).
  - Implementation: `app/src/fs_op/watcher.rs`, runner wiring in
    `app/src/runner/event_loop_main.rs`, and an integration test
    `app/tests/fs_watch.rs`.

  - Refactor: `app::core` module cleanup
    - Consolidated and documented `app/src/app/core/mod.rs`:
      - Introduced clear type aliases for background operation channels and
        cancel flags (`OpProgressReceiver`, `OpCancelFlag`, `OpDecisionSender`).
      - Consolidated duplicate preview-size constants into a single
        canonical `MAX_PREVIEW_BYTES` (100 KiB) while preserving the
        legacy `App::MAX_PREVIEW_BYTES` associated constant for
        compatibility.
      - Improved comments and Rustdoc for `App` and its helper accessors,
        simplified `selected_index` logic, and removed dead/duplicate code.
      - Adjusted helper visibility and kept compatibility with existing
        tests and modules.
    - Tests: added a small unit check for `MAX_PREVIEW_BYTES`; full
      behaviour remains covered by existing integration tests.

  - Refactor: navigation helpers and API rename
    - Refactored `app/src/app/core/navigation.rs` to centralise post-navigation
      behaviour into a private helper (`apply_navigation`) and reduce code
      duplication when updating panel selection and preview state.
    - Renamed public `App` navigation methods to clearer identifiers:
      - `next` -> `select_next`
      - `previous` -> `select_prev`
      - `page_down` -> `select_page_down`
      - `page_up` -> `select_page_up`
    - Updated internal call sites (runner handlers and tests) to use the
      new names. All repository tests pass after these changes.
    - Consider adding deprecated shims for the old names if external users
      require backwards compatibility.

### Notes

- Tests run locally and currently pass.
