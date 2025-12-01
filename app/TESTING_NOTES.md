Running the `start_options` test
--------------------------------

The repository includes a small integration test `start_options` which verifies
that CLI-style startup options are applied into the `App` initial state. Run it
from the repository root (or from the `app/` directory):

```bash
# from repo root
cargo test -p fileZoom start_options -- --nocapture

# or from the `app/` directory
cd app
cargo test start_options -- --nocapture
```

The test is quick and non-interactive; it exercises `App::with_options` and
the UI theme setter without launching the full TUI.

Quick checks for inline command line + quick filter
---------------------------------------------------

- Inline command line: press `:` in the TUI, type `toggle-preview`, hit Enter.
  Expect preview pane toggled; Up/Down cycle history; Tab completes known commands.
- Quick filter: press `/`, enter a glob like `*.txt`, ensure non-matching rows hide
  while selection stays on the previously selected entry when it matches. Submit an
  empty filter to clear.
