---
name: rust_mc_tests
description: "Prompt template for creating or updating tests and fixtures for Rust_MC (`cargo test -p app`)."
---

Scope
-----
- Typical files: `app/tests/*`, unit tests in `app/src/*`, fixtures under `app/tests/fixtures`.

Hard constraints
----------------
- Always run `cargo test -p app` locally and include the full output.
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
4. Run `cargo test -p app` and paste the full output.

Example prompts
---------------
- "Add unit test for `format_entry_line` in `app/src/ui_mod/panels.rs` that ensures width/padding behavior." 
- "Add an integration test that covers copy/move operations using `assert_fs` temp directories." 
