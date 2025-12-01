# fileZoom Roadmap — Rust rewrite of Midnight Commander / Krusader

Reference repos:

- Midnight Commander: <https://github.com/MidnightCommander/mc>
- Krusader: <https://invent.kde.org/utilities/krusader>
- Parity map snapshot: `doc/parity_map.md`

Guiding principles

- Mirror MC/Krusader workflows first (dual panels, F-key bar, inline command line, mouse support), matching defaults unless platform constraints force a change.
- Parity-driven development: maintain a feature matrix vs MC/Krusader for every milestone; preserve behaviour across keyboard, mouse, and VFS backends.
- Ship incremental slices with tests per feature and stable settings schema; prefer resumable/background operations and predictable error handling.

External assumptions (TrueNAS/Debian)

- Rule 1: No external runtime dependencies or package installs; everything ships via Cargo crates. No required network services or downloads at runtime.
- Target environments: Debian-stable (and TrueNAS SCALE) with standard userland (bash/sh, coreutils), terminfo, libc, and OpenSSL/Zlib from the base system. No sudo required for normal use.
- Optional hooks must degrade gracefully: inotify/kqueue presence for fs-watch (feature-gated), OpenSSH client only when users opt into system `ssh`/`sftp` flows; otherwise default to pure-Rust backends.
- Terminal: xterm-compatible terminals with mouse + 256-color; fallback to basic if capabilities are missing.

Known gaps intentionally deferred (to avoid scope creep)

- Full MC mcedit parity (advanced editing features) and Krusader-specific power-user tools are scheduled for M8 and later.
- Advanced archive formats beyond zip/tar/7z/iso (e.g., rar, dmg) are out of scope unless a safe Rust crate exists and demand is explicit.
- Deep plugin ecosystems/user scripting beyond MC/K “user menu/actions” baseline are deferred until core parity is stable.
- Desktop/environment integration (DBus, notifications beyond optional `notify-rust`, MIME handlers) remains opt-in and low priority.
- GUI port, daemon mode, and non-terminal UIs are explicitly out of scope.

Locked terminal defaults (current)

- Keymap: `q` quit; arrows/PageUp/PageDown move; `Enter` open; `Backspace` up; `Tab` swap panels; `Space` toggle selection; `r` refresh; `s/S` cycle sort and direction; `p` toggle preview; `t` toggle theme; `?` help. F-keys: `F1` menu focus, `F3` context actions, `F5` copy, `F6` move. Alt combos are unbound (reserved for parity work).
- Panel/layout: dual panels (left active), preview hidden by default, CLI-style listing enabled, file-stats column hidden (width hint 10) until enabled, sort Name/Ascending, layout ~55/45 split.
- Theme: default is dark; `light` optional. Unknown theme strings fall back to dark. Toggle via in-app `t` or CLI `--theme`.
- Config paths: settings at `$XDG_CONFIG_HOME/fileZoom/settings.toml` (fallback `~/.config/fileZoom/settings.toml`). Keybinds search order: `$XDG_CONFIG_HOME/fileZoom/keybinds.xml` (or `directories-next` platform config) then `./keybinds.xml`. Cache dir via `directories-next` (fallback `~/.cache/filezoom` or `~/.filezoom` if `ProjectDirs` is unavailable).

Initial crate picks for the first slice (layout + keymap + inline command line)

- UI/layout: `ratatui` (kept current) with `crossterm` backend.
- Inline command line: `tui-textarea` (actively maintained, well-documented widget for buffered input inside Ratatui).
- Filters/quick search: `globset` (fast, battle-tested glob/regex matcher).
- Keymaps/hints: `strum` (enum derive) + `phf` (static maps) for MC/K style keymaps and on-screen hints.
- Config: `serde` + `toml` (already present) for settings/keymaps; `once_cell` for defaults; `directories-next` for paths.
- Width/Unicode safety: `unicode-segmentation`/`unicode-width` to keep inline command + columns aligned.

Parity matrix (snapshot)

| Area | MC/K highlights | fileZoom status |
| --- | --- | --- |
| Navigation & panels | Dual panels, tree/brief/flat modes, quick view (F3), per-panel sort, selection patterns, quick filter/search | Partial: dual panels, preview pane, selection + sorting exist; missing tree/brief/flat modes, quick filter/search parity, selection pattern UX, panel history persistence |
| Menus & command line | Top menu + F-key bar, user menu (F2), context menus with accelerators, inline shell with completion/history | Partial: menu + context menu and inline command line exist; missing user menu/actions, full MC/K menu parity, completion/history parity, toolbar/profile support |
| Tabs, bookmarks, history | Tabs with lock/pin, per-tab paths, back/forward history, bookmarks/hotlist | Missing: tabs, hotlist/bookmarks, directory history back/forward |
| File ops & job queue | Background job queue with pause/resume/cancel/retry, overwrite/rename/append policies, checksum verify, trash vs delete, per-job progress | Partial: core copy/move/rename/delete/link/perm ops exist; missing central job queue, pause/resume, overwrite policy UI, checksum verify, trash integration |
| Search / panelize / compare | Find files (filters/attrs), content grep, panelize results, directory compare/sync with preview | Missing: no search/panelize/compare flows yet |
| VFS & archives | Local FS plus SFTP/FTP/SMB/WebDAV/fish, archive browse (zip/tar/7z/iso), background transfers with resume | Missing: only local backend; remote/archives VFS not yet |
| Viewer / editor | Internal viewer (text/hex/pipe), internal editor (mcedit parity), external viewer/editor hooks | Partial: preview pane exists; dedicated viewer/editor parity missing |
| Permissions & disk tools | Properties dialog, chmod/chown/chgrp/ACL UI, disk usage, checksum tools, mount management | Partial: backend permission helpers exist; UI dialogs, disk usage, checksum/mount tools missing |
| Themes, i18n, accessibility | Theme presets (MC blue, light/dark), live switch, localization, accessibility aids | Missing: single theme baseline only; no theme switcher, i18n, or accessibility tooling |
| Keymap & mouse fidelity | MC/K keymap presets, configurable bindings, mouse resize/drag/drop parity | Partial: keyboard/mouse basics present; missing preset keymaps, full F-key bar parity, drag/drop/mouse resize parity |

Milestones

## M0: Parity map + baselines

- Build a feature checklist from MC (4.8.x) and Krusader (current) covering navigation, UI, VFS, jobs, search, compare, user actions, viewer/editor.
  - Crate options: `serde`, `toml`, `serde_json`, `cargo_metadata` (tracking), `insta` (snapshotting expectations).
- Lock defaults: keymap (F1–F10, Alt+ shortcuts), panel layout, color themes, config paths, menu content, toolbar buttons.
  - Crate options: `serde`, `toml`, `directories`/`dirs`, `once_cell` for defaults, `phf` for static lookup tables.
- Baseline performance/startup/IO numbers using make_fakefs; codify contract tests for filesystem operations.
  - Crate options: `criterion`, `iai`, `assert_fs`, `tempfile`, `walkdir`.

### M0 workplan (in progress)

| Area | Status | Actions |
| --- | --- | --- |
| Navigation & panels | Partial | Add tree/brief/flat/filtered modes, quick filter/search, selection patterns, panel history/back-forward; keep per-panel sort/paging on refresh; tests for selection stability. |
| Menus & command line | Partial | Mirror MC/K menu contents + accelerators, add user menu (F2) / user actions, completion+history for inline command line, toolbar/profile presets, on-screen hints aligned to keymap. |
| Tabs, bookmarks, history | Missing | Implement tabs with lock/pin, per-tab path/focus persistence, back/forward history, bookmarks/hotlist UI; persist in settings. |
| File ops & job queue | Partial | Introduce central job queue with pause/resume/cancel/retry; overwrite/rename/append policies; checksum verify; trash vs delete; per-job + aggregate progress UI; reuse for remote ops. |
| Search / panelize / compare | Missing | Add find (filters/attrs), content grep, panelize results, directory compare/sync with preview + dry-run. |
| VFS & archives | Missing | Add SFTP/FTP/SMB/WebDAV/fish backends and capability matrix; archive browse (zip/tar/7z/iso); background transfers with resume/fallbacks. |
| Viewer / editor | Partial | Add dedicated viewer (text/hex/pipe) and editor parity (mcedit basics); external viewer/editor hooks; quick view behaviour aligned with MC/K. |
| Permissions & disk tools | Partial | Properties dialog (chmod/chown/chgrp/ACL/umask), ownership display/edit, disk usage/checksum tools, mount management; safe delete/trash toggles. |
| Themes, i18n, accessibility | Missing | Theme presets (MC blue, Krusader light/dark, high-contrast), live switch, localization workflow, accessibility (contrast/focus). |
| Defaults & config paths | Partial | Freeze keymap defaults (F1 menu, F3 context, F5 copy, F6 move, etc.), panel layout (~55/45), preview hidden by default, dark theme default; confirm config paths and keybind load order with tests. |
| Baselines & tests | Pending | Run make_fakefs startup/IO baselines; record expected timings; add contract tests for copy/move/perm behaviours and settings/schema snapshot tests. |

Cross-cutting blockers to clear early

- Inline command line UX needs a robust input widget (e.g., `tui-textarea`) with history/completion; must integrate with existing handlers and tests.
- Job queue design (pause/resume/cancel/retry) is missing; affects file ops, VFS transfers, and progress UI.
- Selection/filtering patterns and panel modes (tree/brief/flat/quick view) are absent; many milestones depend on them.
- Remote/VFS abstraction is local-only; adding capabilities/compat matrix blocks archive/remote parity and job-queue reuse.
- Theme/i18n/accessibility scaffolding is not started; later milestones require a schema and loading pipeline.
- Settings/keymap schema needs locking and tests to avoid churn once presets and toolbar/menu parity land.

Priority plan (execution order)

- P1: Defaults + inline command line + quick filter scaffolding (M0/M1 foundations); lock keymap/menu defaults and settings schema tests.
- P2: Tabs/history/bookmarks + selection patterns + panel modes (tree/brief/flat/quick view) for navigation parity (M1/M2).
- P3: Menus/user actions/toolbar + F-key bar hints aligned to presets (M3).
- P4: Job queue + overwrite/checksum policies + progress UI and trash integration (M4).
- P5: Search/panelize/compare-sync flows with conflict dialogs (M5).
- P6: VFS/archives with capability matrix and transfer integration (M6).
- P7: Permissions/properties/disk tools (M7).
- P8: Viewer/editor parity and quick view polish (M8).
- P9: Themes/i18n/accessibility + config UI polish (M9).
- P10: Quality gates/perf/soak + release packaging (M10).

## M1: Core layout + keymap fidelity

- Dual panels with status bars, F-key bar, top menu, inline command line, quick view/preview, mouse resize/click/drag; defaults mirror MC/Krusader.
  - Crate options: `ratatui`, `crossterm`, `unicode-width`, `tui-input`/`tui-textarea` for the inline command line.
- Panel modes: brief/full/tree/quick view, quick filter/search, sorted columns retained on refresh; keep selection visible.
  - Crate options: `ratatui` tables/lists, `tui-tree-widget`, `globset` for quick filter, `unicode-segmentation` for width handling.
- Tab completion and shell-style line editing in the inline command line.
  - Crate options: `reedline` or `rustyline`, `shell-words`, `which` for resolver hooks.
- Robust keybindings with on-screen hints and mouse parity for menus/context menus.
  - Crate options: `strum` for key enums, `phf` for keymap tables, `serde` for configurable bindings.

### M1 workplan (planned)

| Area | Status | Actions |
| --- | --- | --- |
| Layout & panels | Planned | Implement tree/brief/flat/quick view modes; keep per-panel sort/paging on refresh; add mouse resize/drag/drop parity; tests for selection visibility. |
| Inline command line | Planned | Embed inline command widget with history/completion; shell-style editing/resolver; hook into F-key bar/menu actions. |
| Keymap & hints | Planned | Ship MC/K presets with on-screen F-key hints; configurable bindings; mouse parity for menus/context menus. |
| Quick filters/search | Planned | Add in-panel quick filter/find-as-you-type with glob/regex; preserve selection context. |

## M2: Navigation, tabs, and histories

- Tabs with lock/pin, per-tab path/focus persistence, quick panel swap/clone; persist last layout.
  - Crate options: `indexmap` for stable ordering, `serde` + `toml`, `slotmap` for tab handles.
- Bookmarks/hotlist, directory history back/forward, hotlist menu, middle-click open; store bookmarks/keybindings/layout in settings.
  - Crate options: `directories`, `serde`, `toml`, `chrono` for timestamps.
- Marks/selection patterns (Select/Unselect/Invert) and selection stability across reload/panelize.
  - Crate options: `globset`, `regex`, `aho-corasick`, `bitflags` for selection state.

### M2 workplan (planned)

| Area | Status | Actions |
| --- | --- | --- |
| Tabs | Planned | Add tabs with lock/pin, per-tab path/focus persistence, swap/clone; persist layout and restore on start. |
| History/bookmarks | Planned | Implement back/forward history and bookmarks/hotlist UI; middle-click open; persist in settings. |
| Selection patterns | Planned | Add select/unselect/invert patterns using glob/regex; selection persists through reload/panelize. |

## M3: Command line + menu/toolbar

- Menus and context menus mirroring MC/Krusader items/accelerators; configurable keymaps with presets; mouse-driven menus.
  - Crate options: `ratatui` menu widgets, `crossterm`, `strum`, `serde`.
- User menu (F2) and Krusader User Actions: script execution with placeholders, working directory rules, input/output handling.
  - Crate options: `shell-words`, `which`, `duct`, `tempfile`, `xshell`.
- Toolbar + panel profiles/layouts with save/restore; hints aligned to keybinds.
  - Crate options: `serde`, `toml`, `schemars` for schema validation, `once_cell` for defaults.

### M3 workplan (planned)

| Area | Status | Actions |
| --- | --- | --- |
| Menus & accelerators | Planned | Mirror MC/K top menu + context menus with accelerators; align with keymap presets and mouse flows. |
| User menu/actions | Planned | Add F2 user menu and Krusader-style user actions (placeholders, cwd rules, I/O handling). |
| Toolbar/profiles | Planned | Implement toolbar and panel layout profiles with save/restore; hints aligned with bindings. |

## M4: File operations + job queue

- Central job queue for copy/move/delete/rename/link/chmod/chown with pause/resume/cancel/retry; background/foreground toggle.
  - Crate options: `crossbeam-channel` or `async-channel`, `rayon` for worker pools, `parking_lot` for lightweight locks.
- Overwrite policies (skip/rename/overwrite/append), checksum verify, preserve permissions/timestamps/ACL where available, temp-root/elevation path.
  - Crate options: `fs_extra`, `tempfile`, `filetime`, `sha2`/`blake3`, `acl` for POSIX ACLs, `which` for elevation helpers.
- Progress UI per job + aggregate with notifications on completion/error.
  - Crate options: `ratatui` progress widgets, `indicatif` for progress math, `notify-rust` for desktop notifications.

### M4 workplan (planned)

| Area | Status | Actions |
| --- | --- | --- |
| Job queue | Planned | Build central queue for copy/move/delete/rename/link/chmod/chown with pause/resume/cancel/retry and BG/FG toggle. |
| Policies & integrity | Planned | Add overwrite/rename/append policies, checksum verify option, temp-root/elevation hooks, ACL/timestamp preservation. |
| Progress UX | Planned | Per-job and aggregate progress UI with notifications; reuse queue across local/remote backends. |

## M5: Search, panelize, compare/sync

- Find file/content grep with filters (size/date/attributes/owner/perm); panelize results while keeping selection.
  - Crate options: `ignore`, `globset`, `regex`, `bstr`, `grep-searcher`.
- Directory compare/synchronizer (size/date/checksum/content) with conflict resolution dialogs and dry-run.
  - Crate options: `walkdir`, `sha2`/`blake3`, `same-file`, `similar`, `pathdiff`.
- Quick filter in-panel (pattern/regex/type) without losing selection context.
  - Crate options: `aho-corasick`, `globset`, `regex`.

### M5 workplan (planned)

| Area | Status | Actions |
| --- | --- | --- |
| Search & filters | Planned | Add find files/content with filters (size/date/attrs/owner/perm); panelize results retaining selection. |
| Compare/sync | Planned | Implement directory compare (size/date/checksum/content) with sync preview/dry-run and conflict dialogs. |
| Quick filter | Planned | In-panel quick filter (pattern/regex/type) that preserves selection context. |

## M6: VFS/backends + archives

- Harden local backend; add SFTP, SMB, FTP, fish; configurable credentials/cache.
  - Crate options: `ssh2`/`async-ssh2-lite` (SFTP), `suppaftp`/`async-ftp`, `smb2`/`smbclient`, `reqwest` + `webdav-client`, `directories` for paths, `rusqlite`/`sled` for caches.
- Archive navigation via VFS mount points (zip/tar/7z/iso) with transparent copy/move/extract; capability matrix per backend.
  - Crate options: `zip`, `tar`, `flate2`, `xz2`, `zstd`, `sevenz-rust`.
- Background transfers with resume where protocol allows; graceful fallback/blocks.
  - Crate options: `futures`, `async-trait`, `bytes`, `indicatif` for transfer progress.

### M6 workplan (planned)

| Area | Status | Actions |
| --- | --- | --- |
| Local backend | Planned | Harden local VFS behaviours and errors; document capability matrix. |
| Remote backends | Planned | Add SFTP/FTP/SMB/WebDAV/fish with credential/cache handling; align with job queue; capability gating. |
| Archives | Planned | Browse/mount archives (zip/tar/7z/iso) transparently; copy/move/extract via VFS. |
| Transfers | Planned | Background transfers with resume where supported; graceful fallback and progress UI. |

## M7: Permissions, properties, and disk tools

- Properties/permissions dialogs (chmod/chown/chgrp/ACL/umask-aware), ownership display/edit, symlink handling.
  - Crate options: `nix`, `users`, `acl`, `umask`, `same-file`.
- Disk usage/space viewers, checksum commands, mount management (MountMan-like) with privileged/elevated actions where supported.
  - Crate options: `sysinfo`, `heim`, `walkdir`, `sha2`/`blake3`, `mountpoints`, `sudo`/`runas` for elevation hooks.
- Safe delete/trash confirmations; configurable trash vs delete.
  - Crate options: `trash`, `dialoguer` for confirmations.

### M7 workplan (planned)

| Area | Status | Actions |
| --- | --- | --- |
| Properties/permissions | Planned | Add properties dialogs with chmod/chown/chgrp/ACL/umask handling; symlink-aware UI; editable ownership. |
| Disk tools | Planned | Disk usage/space viewers, checksum commands, mount management with optional elevation. |
| Delete/trash safety | Planned | Configurable trash vs delete and confirmations aligned with MC/K flows. |

## M8: Viewer, editor, and preview

- Internal viewer (text/hex/pipe) with wrap/encoding/charset selection; external viewer hooks.
  - Crate options: `ropey`, `encoding_rs`, `bstr`, `hex`/`hexyl` for hex view, `which` for external viewer discovery.
- Internal editor parity with mcedit basics (syntax highlight presets, indent, search/replace); external editor integration.
  - Crate options: `ropey`, `syntect` for highlighting, `regex`/`sublime_fuzzy` for search, `dirs` for editor lookup.
- Quick view panel and preview refresh respecting selection changes.
  - Crate options: `notify` (fs watch), `ratatui`, `memmap2` for efficient large-file previews.

### M8 workplan (planned)

| Area | Status | Actions |
| --- | --- | --- |
| Viewer | Planned | Build text/hex/pipe viewer with wrap/encoding/charset controls; external viewer hooks. |
| Editor | Planned | Implement mcedit-parity editor basics (syntax highlighting presets, indent, search/replace); external editor integration. |
| Quick view/preview | Planned | Quick view panel with responsive refresh on selection changes; efficient large-file preview. |

## M9: Themes, i18n, and polish

- Theme system with presets (MC blue, Krusader light/dark, high-contrast), user theme loading, live switch + persisted selection; color rules for file types/states.
  - Crate options: `palette`, `serde`, `toml`, `once_cell`, `owo-colors`.
- Accessibility: contrast checks, keyboard-first flows, focus indicators.
  - Crate options: `unicode-width`, `unicode-segmentation`, `anstyle`/`console` for styling helpers.
- Localization and help workflow; config UI for keymaps, mouse behavior, VFS credentials, job defaults; docs/help parity with MC.
  - Crate options: `fluent-bundle`, `unic-langid`, `intl_pluralrules`, `pulldown-cmark` for help rendering, `schemars` for config schemas.

### M9 workplan (planned)

| Area | Status | Actions |
| --- | --- | --- |
| Themes | Planned | Ship presets (MC blue, Krusader light/dark, high-contrast), live switch, user theme loading with persistence. |
| Accessibility | Planned | Add contrast checks, focus indicators, keyboard-first UX passes. |
| Localization & help | Planned | Build localization workflow and help/docs parity; config UI for keymaps/mouse/VFS creds/job defaults with schema validation. |

## M10: Quality gates + release readiness

- Integration tests for VFS backends, job queue semantics, panelize/search/compare/sync, viewer/editor basics.
  - Crate options: `assert_fs`, `predicates`, `rstest`, `escargot` for binary runs.
- Property tests for path/permissions helpers; perf/soak runs via make_fakefs; crash reporting/logging defaults.
  - Crate options: `proptest`, `quickcheck`, `criterion`, `iai`, `tracing`, `tracing-subscriber`, `color-eyre`, `sentry` (optional telemetry).
- Release profiles for Linux/macOS/Windows terminals; docs/help parity notes and migration guide from MC/Krusader.
  - Crate options: `cargo-dist`, `cross`, `git-cliff`, `mdbook`.

### M10 workplan (planned)

| Area | Status | Actions |
| --- | --- | --- |
| Integration tests | Planned | Add end-to-end coverage for VFS backends, job queue semantics, panelize/search/compare/sync, viewer/editor basics. |
| Property/perf/soak | Planned | Property tests for path/permissions helpers; perf + soak runs via make_fakefs; crash logging defaults. |
| Release readiness | Planned | Release profiles for Linux/macOS/Windows terminals; changelog/help parity; migration notes from MC/Krusader. |
