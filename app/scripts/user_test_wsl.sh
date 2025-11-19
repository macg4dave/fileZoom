#!/usr/bin/env bash
# WSL-specific interactive user test helper for the `fileZoom` crate.
# Usage: ./user_test_wsl.sh [prepare|reset|build|run|rebuild|run-new|run-new-konsole|help]

set -euo pipefail

# Ensure we're running inside WSL
if [ -z "${WSL_DISTRO_NAME-}" ] && ! grep -qi microsoft /proc/version 2>/dev/null; then
  cat >&2 <<'MSG'
This helper is intended to run inside WSL (Windows Subsystem for Linux).
Please open your WSL distro (e.g. Ubuntu) and re-run the command there.
MSG
  exit 1
fi

ROOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"

# When the repository lives on a Windows-mounted path (e.g. /mnt/c/...),
# prefer copying the workspace into a native WSL location for builds and
# I/O heavy work. Set `WSL_BUILD_ROOT` in your environment to override.
WSL_BUILD_ROOT="${WSL_BUILD_ROOT:-$HOME/filezoom_workspace}"
BUILD_REPO_DIR="$WSL_BUILD_ROOT/fileZoom"

# ACTIVE_ROOT will point to the path where build/run operations occur.
# It defaults to `ROOT_DIR` but will be switched to `BUILD_REPO_DIR`
# when we detect a /mnt/ checkout and perform a copy.
ACTIVE_ROOT="$ROOT_DIR"
FIXTURES_DIR="$ACTIVE_ROOT/tests/fixtures"
DEMO_DIR="$ACTIVE_ROOT/target/demo_workspace"
BINARY_PATH="$ACTIVE_ROOT/target/debug/fileZoom"

USE_WSL_BUILD=0

function ensure_wsl_build_root() {
  # If the repo is on a mounted Windows filesystem, copy/sync it into WSL
  # native storage for builds. This keeps `target/` and other I/O-heavy
  # artifacts on the fast Linux filesystem.
  if [[ "$ROOT_DIR" == /mnt/* ]]; then
    mkdir -p "${BUILD_REPO_DIR%/*}"
    echo "Preparing WSL build copy at: $BUILD_REPO_DIR"
    if command -v rsync >/dev/null 2>&1; then
      rsync -a --delete --exclude target --exclude .git "$ROOT_DIR/" "$BUILD_REPO_DIR/"
    else
      # Fallback to cp if rsync isn't available
      rm -rf "$BUILD_REPO_DIR"
      mkdir -p "$BUILD_REPO_DIR"
      cp -a "$ROOT_DIR/." "$BUILD_REPO_DIR/"
    fi
    ACTIVE_ROOT="$BUILD_REPO_DIR"
    USE_WSL_BUILD=1
  else
    ACTIVE_ROOT="$ROOT_DIR"
    USE_WSL_BUILD=0
  fi
  # update dependent paths
  FIXTURES_DIR="$ACTIVE_ROOT/tests/fixtures"
  DEMO_DIR="$ACTIVE_ROOT/target/demo_workspace"
  BINARY_PATH="$ACTIVE_ROOT/target/debug/fileZoom"
}

function usage() {
  cat <<'EOF'
Usage: ./user_test_wsl.sh [prepare|reset|build|run|rebuild|run-new|run-new-konsole|help]

No-arg (default) - Open a new KDE Konsole window and run the demo there.
run-new          - Open a new terminal window under WSL (tries konsole then Windows terminal emulator if available) and run the demo.
run-new-konsole  - Open a new KDE Konsole window and run the demo there.
help             - Show this message.
EOF
}

function prepare_demo() {
  # Ensure ACTIVE_ROOT is set to the appropriate build location
  ensure_wsl_build_root
  echo "Preparing demo workspace at: $DEMO_DIR"
  rm -rf "$DEMO_DIR"
  mkdir -p "$DEMO_DIR"
  if [ -d "$FIXTURES_DIR" ]; then
    cp -a "$FIXTURES_DIR/." "$DEMO_DIR/"
  else
    mkdir -p "$DEMO_DIR/dirA"
    printf "Hello demo\n" > "$DEMO_DIR/file1.txt"
    printf "Nested demo\n" > "$DEMO_DIR/dirA/file2.txt"
  fi
}

function build_binary() {
  # Ensure we build in the WSL-native location when appropriate.
  ensure_wsl_build_root
  echo "Building in: $ACTIVE_ROOT"
  (cd "$ACTIVE_ROOT" && cargo build)
}

function run_binary() {
  # Make sure the build and demo live in the same ACTIVE_ROOT
  ensure_wsl_build_root
  if [ ! -f "$BINARY_PATH" ]; then
    build_binary
  fi
  cd "$DEMO_DIR"
  exec "$BINARY_PATH"
}

function open_in_konsole() {
  if ! command -v konsole >/dev/null 2>&1; then
    return 1
  fi
  if [ ! -f "$BINARY_PATH" ]; then build_binary; fi
  if [ ! -d "$DEMO_DIR" ]; then prepare_demo; fi
  ESC_DEMO_DIR=$(printf '%s' "$DEMO_DIR" | sed -e "s/'/'\\''/g")
  ESC_BIN=$(printf '%s' "$BINARY_PATH" | sed -e "s/'/'\\''/g")
  KON_CMD="cd '$ESC_DEMO_DIR' && exec '$ESC_BIN'"
  konsole -e bash -lc "$KON_CMD" &
}

case "${1-}" in
  "")
    # Default behavior: run in a new konsole window
    open_in_konsole || { echo "konsole not available"; exit 1; }
    ;;
  prepare)
    prepare_demo
    ;;
  reset)
    rm -rf "$DEMO_DIR"
    prepare_demo
    ;;
  build)
    build_binary
    ;;
  run)
    run_binary
    ;;
  run-new)
    if open_in_konsole; then
      exit 0
    fi
    # As a fallback, try to use the Windows Terminal (wt.exe) via cmd.exe if available
    if command -v wt.exe >/dev/null 2>&1 || command -v cmd.exe >/dev/null 2>&1; then
      if [ ! -f "$BINARY_PATH" ]; then build_binary; fi
      if [ ! -d "$DEMO_DIR" ]; then prepare_demo; fi
      ESC_DEMO_DIR=$(printf '%s' "$DEMO_DIR" | sed -e "s/'/'\\''/g")
      ESC_BIN=$(printf '%s' "$BINARY_PATH" | sed -e "s/'/'\\''/g")
      # Run via cmd.exe -> wsl path translation might be required; using wsl.exe from Windows
      # but since we are inside WSL, try launching a new konsole-like window in Linux first.
      echo "No konsole available and Windows terminal fallback is platform dependent."
      exit 1
    fi
    echo "No konsole available to open a new window."
    exit 1
    ;;
  run-new-konsole)
    open_in_konsole || { echo "konsole not available"; exit 1; }
    ;;
  rebuild)
    (cd "$ROOT_DIR" && cargo clean && cargo build)
    ;;
  help|-h|--help)
    usage
    ;;
  *)
    echo "Unknown command: ${1-}"
    usage
    exit 2
    ;;
esac
