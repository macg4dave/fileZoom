
**Overview**: This file explains how to run the integration tests and use the provided fixtures for `fileZoom` (crate `fileZoom`).

**Important (Windows 11 users)**: If your development machine is Windows 11, build and test inside WSL2 (Windows Subsystem for Linux) to ensure the environment matches CI and contributors. The helper scripts included here will refuse to run if they are not executed inside a WSL environment.

- **Run tests (recommended)**: from inside WSL2 (or Linux/macOS) run:

```
cd app
cargo test -p fileZoom -- --nocapture
```

- **Run tests via helper script** (inside WSL2):

```
./app/scripts/run_tests_wsl.sh
```

- **Manual exploratory testing using fixtures**:
  -- The packaged fixtures are in `app/tests/fixtures/`.
  - To try the example repository manually, open a terminal:


```
cd app/tests/fixtures
pwd
# NOTE: Running the TUI binary inside this folder will try to start a terminal UI.
```

If you want automated tests to use the fixtures, modify `app/tests/integration_tests.rs` to copy files from `tests/fixtures` into a temporary directory (the current tests create their own temporary structure).

**Interactive demo script**

Use `app/scripts/user_test_wsl.sh` to set up a demo workspace, build the binary, and run the program interactively. Run the helper scripts from inside WSL2. Examples:

```
cd app
./scripts/user_test_wsl.sh prepare
./scripts/user_test_wsl.sh build
./scripts/user_test_wsl.sh run
```

Running `run` will start the TUI and take over your terminal — quit with `q`.

**Open in new Terminal window (macOS)**

To open the demo in a new macOS Terminal window instead of running in the current shell, use:

```
./scripts/user_test_wsl.sh run-new
```

This requires `osascript` (available on macOS) and will create a new Terminal window that runs the demo workspace binary.
