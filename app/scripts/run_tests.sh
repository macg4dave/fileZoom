#!/usr/bin/env bash
# Run app crate tests with output shown (WSL-friendly wrapper)

# Ensure we're running inside WSL2 on Windows 11 when invoked from a Windows host.
if [ -z "${WSL_DISTRO_NAME-}" ] && ! grep -qi microsoft /proc/version 2>/dev/null; then
  cat >&2 <<'MSG'
This script should be run inside WSL2 (Windows Subsystem for Linux) on Windows 11.
Open your WSL distro (e.g. Ubuntu) and re-run this script from there.

From PowerShell you can run the command inside WSL like this:
  wsl -- cd /mnt/c/Users/<you>/github/Rust_MC && ./app/scripts/run_tests_wsl.sh

MSG
  exit 1
fi

set -euo pipefail
cd "$(dirname "$0")/.."
# Run tests for the `app` crate and show output
cargo test -p app -- --nocapture
