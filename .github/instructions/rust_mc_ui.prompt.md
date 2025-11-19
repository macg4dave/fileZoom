---
name: fileZoom_ui
description: "Prompt template for UI/UX changes in the fileZoom TUI (panels, menu, modal, main event loop)."
---

Scope
-----
-- Typical files: `app/src/ui/*`, `app/src/main.rs`, `app/src/lib.rs` (if re-exports are needed). (crate: `fileZoom`)

Hard constraints
----------------
-- Run `cargo test -p fileZoom` and paste the full output in your response.
- Keep changes minimal and focused to UI code only unless core changes are required.
- Do not change CLI flags or machine-facing outputs.

Prompt template
---------------
Task:
"""
<Short one-line summary of the UI change>

Details:
- What to change: <description of rendering/input/behavior change>
- Files: <list e.g., `app/src/ui/panels.rs, app/src/ui/menu.rs`>
- Tests: <which helper functions to unit test>
"""

Assistant instructions
---------------------
1. State a 2–3 bullet plan.
2. Implement a minimal patch that compiles.
3. Add unit tests for pure helpers (formatting, layout helpers) and update integration tests only if UI contract changed.
4. Run `cargo test -p fileZoom` and include the full output.
5. If tests fail, iterate and fix up to 5 times.

Example prompts
---------------
-- "Make top menu keyboard-navigable. Files: `app/src/ui/menu.rs`, `app/src/main.rs`. Add unit tests for menu label helpers." 
-- "Fix list scrolling so selection remains visible after refresh. Files: `app/src/app.rs`, `app/src/ui/panels.rs`. Add unit test for `ensure_selection_visible` behavior." 

Usage
-----
- Paste the filled Task and Details sections when invoking Copilot in VS Code. The assistant should return a concise patch and `cargo test` output.
