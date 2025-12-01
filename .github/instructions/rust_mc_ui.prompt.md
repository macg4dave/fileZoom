

Improved rust_mc_ui.prompt.md

🎯 Scope

UI-specific tasks only.
Typical files: - app/src/ui/* - app/src/main.rs - app/src/app.rs (if UI
state requires adjustment) - app/src/lib.rs (only for re‑exports)

All changes apply to crate fileZoom.

------------------------------------------------------------------------

🔥 Hard Constraints

-   Run full tests: cargo test -p fileZoom and paste full output.
-   Keep the patch minimal and tightly scoped to UI behaviour unless
    deeper changes are unavoidable.
-   Keep the parity matrix in `doc/roadmap.md` (and `roadmap.txt` mirror) up to date when UI/features move from missing → partial → complete.
-   Do not modify `doc/roadmap.md` or `roadmap.txt` without explicit permission; they are the canonical roadmap sources and must stay in sync.
-   Do not alter CLI flags, public API semantics, or machine-facing
    output.
-   Preserve behaviour unless tests or the task explicitly call for
    change.
-   No unsafe.

------------------------------------------------------------------------

🧱 UI Standards & Expectations

-   Follow idiomatic Rust + Ratatui/Crossterm patterns.
-   Pure helpers (formatting, layout, state logic) should be isolated
    and unit tested.
-   Keep rendering code simple, predictable, and side‑effect‑free.
-   Maintain clear separation between:
    -   State (owned by App)
    -   Rendering (ui/*)
    -   Input handling (main.rs or UI state helpers)

When adding features: - Avoid monolithic render functions.
- Introduce small helpers and test them.
- Keep event handling deterministic and order‑safe.

------------------------------------------------------------------------

🧩 Prompt Template

Use this when requesting a UI change.

Task

    <One-sentence summary of the UI change>

Details

    - Change: <description of new behaviour / rendering / input change>
    - Files: <list of target files, e.g. ui/panels.rs, ui/menu.rs>
    - Tests: <which helpers to test, or “auto-detect”>
    - Constraints: <anything that must NOT be touched>

------------------------------------------------------------------------

🧠 Assistant Instructions

When generating the patch:

1.  Provide a 2–3 bullet plan describing the approach.
2.  Implement the smallest functional patch consistent with repo style.
3.  Add unit tests for any pure helper logic.
4.  Only update integration tests if the external UI contract
    (observable behaviour) changes.
5.  Run cargo test -p fileZoom and include complete test output.
6.  If failures occur:
    -   Fix and retry up to 5 iterations
    -   Briefly explain each fix

Final response must include: - Summary of changes
- One or more apply_patch-formatted diffs
- Passing test output
- Optional small improvements follow-up

------------------------------------------------------------------------

🧪 Example Requests

Menu interaction

    Task: Make the top menu keyboard-navigable.
    Details:
    - Change: add menu state + highlight, handle arrow keys + Enter.
    - Files: ui/menu.rs, main.rs
    - Tests: unit-test menu label helpers.

Panel scroll fix

    Task: Keep selection visible after list refresh.
    Details:
    - Change: adjust logic in ensure_selection_visible.
    - Files: app/src/app.rs, ui/panels.rs
    - Tests: add targeted unit tests for visibility math.

------------------------------------------------------------------------

🛠 Usage Notes

-   Paste the Task + Details block into Copilot Chat / VSCode prompt.
-   The assistant will return a patch + test results.
-   Keep UI tasks focused; if deep architectural issues arise, escalate
    to a general Rust prompt instead.
