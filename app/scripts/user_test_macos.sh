#!/usr/bin/env bash
# macOS-specific interactive user test helper for the `fileZoom` crate.
# Usage: ./user_test_macos.sh [prepare|reset|build|run|rebuild|run-new|help]

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
FIXTURES_DIR="$ROOT_DIR/tests/fixtures"
DEMO_DIR="$ROOT_DIR/target/demo_workspace"
BINARY_PATH="$ROOT_DIR/target/debug/fileZoom"

function usage() {
  cat <<'EOF'
Usage: ./user_test_macos.sh [prepare|reset|build|run|rebuild|run-new|help]

run-new  - Open a new macOS Terminal window and run the demo there.
help     - Show this message.
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
    if ! command -v osascript >/dev/null 2>&1; then
      echo "osascript not available; cannot open macOS Terminal."
      exit 1
    fi
    if [ ! -f "$BINARY_PATH" ]; then
      build_binary
    fi
    ESC_DEMO_DIR=$(printf '%s' "$DEMO_DIR" | sed -e "s/'/'\\''/g")
    ESC_BIN=$(printf '%s' "$BINARY_PATH" | sed -e "s/'/'\\''/g")
    APPLE_CMD="cd '$ESC_DEMO_DIR' && exec '$ESC_BIN'"
    osascript -e "tell application \"Terminal\" to do script \"${APPLE_CMD}\""
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
