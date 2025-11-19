---
name: fileZoom_tests
description: "Prompt template for creating or updating tests and fixtures for fileZoom (`cargo test -p fileZoom`)."
---

Scope
-----
-- Typical files: `app/tests/*`, unit tests in `app/src/*`, fixtures under `app/tests/fixtures` (crate: `fileZoom`).

Hard constraints
----------------
-- Always run `cargo test -p fileZoom` locally and include the full output.
- Tests must be deterministic and not depend on external network resources.
- Use `assert_fs` or temporary directories for filesystem fixtures.

Prompt template
---------------
Task:
"""
<Brief summary of test task>

Details:
- Which behavior to test: <describe function/feature>
- Files to change/add: <list files>
"""

Assistant instructions
---------------------
1. Provide a short plan (1-3 bullets).
2. Add the smallest test changes required and helper fixtures.
3. If new helper code is needed, add it with unit tests.
4. Run `cargo test -p fileZoom` and paste the full output.

Example prompts
---------------
 - "Add unit test for `format_entry_line` in `app/src/ui/panels.rs` that ensures width/padding behavior." 
- "Add an integration test that covers copy/move operations using `assert_fs` temp directories." 
