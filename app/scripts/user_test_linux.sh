#!/usr/bin/env bash
# Linux-specific interactive user test helper for the `app` crate.
# Usage: ./user_test_linux.sh [prepare|reset|build|run|rebuild|run-new|run-new-konsole|help]

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
FIXTURES_DIR="$ROOT_DIR/tests/fixtures"
DEMO_DIR="$ROOT_DIR/target/demo_workspace"
BINARY_PATH="$ROOT_DIR/target/debug/app"

function usage() {
  cat <<'EOF'
Usage: ./user_test_linux.sh [prepare|reset|build|run|rebuild|run-new|run-new-konsole|help]

run-new         - Open a new terminal window (tries konsole then xterm) and run the demo.
run-new-konsole - Open a new KDE Konsole window and run the demo there.
help            - Show this message.
EOF
}

function prepare_demo() {
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
  (cd "$ROOT_DIR" && cargo build)
}

function run_binary() {
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

function open_in_xterm() {
  if ! command -v xterm >/dev/null 2>&1; then
    return 1
  fi
  if [ ! -f "$BINARY_PATH" ]; then build_binary; fi
  if [ ! -d "$DEMO_DIR" ]; then prepare_demo; fi
  ESC_DEMO_DIR=$(printf '%s' "$DEMO_DIR" | sed -e "s/'/'\\''/g")
  ESC_BIN=$(printf '%s' "$BINARY_PATH" | sed -e "s/'/'\\''/g")
  XCMD="cd '$ESC_DEMO_DIR' && exec '$ESC_BIN'"
  xterm -e bash -lc "$XCMD" &
}

case "${1-}" in
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
    if open_in_xterm; then
      exit 0
    fi
    echo "No konsole or xterm available to open a new window."
    exit 1
    ;;
  run-new-konsole)
    open_in_konsole || { echo "konsole not available"; exit 1; }
    ;;
  rebuild)
    (cd "$ROOT_DIR" && cargo clean && cargo build)
    ;;
  help|""|-h|--help)
    usage
    ;;
  *)
    echo "Unknown command: ${1-}"
    usage
    exit 2
    ;;
esac
