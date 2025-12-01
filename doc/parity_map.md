# fileZoom Parity Map (MC/Krusader)

This document tracks parity against Midnight Commander (MC 4.8.x) and Krusader
and notes what the current M0 work has covered.

## Recent M0 coverage

- Inline command line upgraded to `tui-textarea` with history and tab-completion
  (open with `:`; Enter runs, Esc cancels).
- Quick filter scaffolding on panels using `globset` (open with `/`; glob
  patterns, empty to clear; selection is preserved when possible).
- Defaults locked via snapshot tests (settings, keymap, layout, config paths).

## Feature matrix (snapshot)

| Area | MC/K highlights | fileZoom status |
| --- | --- | --- |
| Navigation & panels | Dual panels, tree/brief/flat modes, quick view (F3), per-panel sort, selection patterns, quick filter/search | Partial: dual panels, preview pane, selection + sorting exist; quick filter scaffolding landed; missing tree/brief/flat modes, selection pattern UX, panel history |
| Menus & command line | Top menu + F-key bar, user menu (F2), context menus with accelerators, inline shell with completion/history | Partial: menu + context menu and inline command line exist; inline cmdline has basic history/tab-complete; missing user menu/actions, full MC/K menu parity, richer completion/history, toolbar/profile support |
| Tabs, bookmarks, history | Tabs with lock/pin, per-tab paths, back/forward history, bookmarks/hotlist | Missing: tabs/hotlist/history |
| File ops & job queue | Background job queue with pause/resume/cancel/retry, overwrite/rename/append policies, checksum verify, trash vs delete, per-job progress | Partial: core copy/move/rename/delete/link/perm ops exist; no central job queue/overwrite UI/checksum/trash |
| Search / panelize / compare | Find files (filters/attrs), content grep, panelize results, directory compare/sync with preview | Missing: no search/panelize/compare flows |
| VFS & archives | Local FS plus SFTP/FTP/SMB/WebDAV/fish, archive browse (zip/tar/7z/iso), background transfers with resume | Missing: only local backend |
| Viewer / editor | Internal viewer (text/hex/pipe), internal editor (mcedit parity), external viewer/editor hooks | Partial: preview pane only; dedicated viewer/editor parity missing |
| Permissions & disk tools | Properties dialog, chmod/chown/chgrp/ACL UI, disk usage, checksum tools, mount management | Partial: backend permission helpers exist; UI dialogs/disk tools missing |
| Themes, i18n, accessibility | Theme presets (MC blue, light/dark), live switch, localization, accessibility aids | Missing: single theme baseline; no presets/i18n/accessibility |
| Keymap & mouse fidelity | MC/K keymap presets, configurable bindings, mouse resize/drag/drop parity | Partial: basic keyboard/mouse; missing presets, F-key bar parity, drag/drop/mouse resize parity |

## Baselines/tests status

- Defaults snapshot tests: settings, keymap, layout, config paths (added).
- Inline command line + quick filter covered by unit/integration tests.
- Pending: perf/startup IO baselines via `make_fakefs`; schema change guard for settings/keybinds files.
