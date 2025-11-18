#!/usr/bin/env bash
# Interactive user test helper for the `app` crate.
# Usage: ./user_test.sh [prepare|reset|build|run|rebuild|help]
# - prepare : create a demo directory populated with fixtures
# - reset   : remove demo directory and recreate fixtures
# - build   : cargo build the `app` binary
# - run     : build (if needed) and run the binary with CWD set to demo dir
# - rebuild : cargo clean && cargo build
# - help    : show this message

set -euo pipefail
ROOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
SCRIPTS_DIR="$ROOT_DIR/scripts"
FIXTURES_DIR="$ROOT_DIR/tests/fixtures"
DEMO_DIR="$ROOT_DIR/target/demo_workspace"
BINARY_PATH="$ROOT_DIR/target/debug/app"

function usage() {
  cat <<'EOF'
Usage: ./user_test.sh [prepare|reset|build|run|rebuild|help]

prepare  - Create demo workspace at target/demo_workspace and copy fixtures there.
reset    - Remove demo workspace and recreate fixtures (same as prepare after removal).
build    - Run `cargo build` for the `app` crate.
run      - Build (if necessary) and run the `app` binary with CWD set to demo workspace.
rebuild  - Run `cargo clean` then `cargo build`.
help     - Show this message.

Examples:
  ./user_test.sh prepare
  ./user_test.sh build
  ./user_test.sh run

Note: Running `run` will start the TUI and take over your terminal. Quit the UI with `q`.
EOF
}

function prepare_demo() {
  echo "Preparing demo workspace at: $DEMO_DIR"
  rm -rf "$DEMO_DIR"
  mkdir -p "$DEMO_DIR"
  if [ -d "$FIXTURES_DIR" ]; then
    echo "Copying fixtures from $FIXTURES_DIR"
    cp -a "$FIXTURES_DIR/." "$DEMO_DIR/"
  else
    echo "No fixtures found at $FIXTURES_DIR — creating minimal demo files"
    mkdir -p "$DEMO_DIR/dirA"
    printf "Hello demo\n" > "$DEMO_DIR/file1.txt"
    printf "Nested demo\n" > "$DEMO_DIR/dirA/file2.txt"
  fi
  echo "Demo workspace ready."
}

function build_binary() {
  echo "Building app (cargo build) in: $ROOT_DIR"
  (cd "$ROOT_DIR" && cargo build)
  if [ -f "$BINARY_PATH" ]; then
    echo "Build successful: $BINARY_PATH"
  else
    echo "Build failed or binary not found at $BINARY_PATH"
    exit 1
  fi
}

function run_binary() {
  if [ ! -f "$BINARY_PATH" ]; then
    echo "Binary not found, building first..."
    build_binary
  fi
  # attempt to locate the actual built executable; Cargo in a workspace may put
  # artifacts in the workspace root `target/` rather than the crate `target/`.
  if [ ! -f "$BINARY_PATH" ]; then
    # search common locations for an executable whose name begins with 'app'
    CANDIDATE=$(find "$ROOT_DIR" "$ROOT_DIR/.." -maxdepth 2 -type f -executable -name 'app*' -print -quit 2>/dev/null || true)
    if [ -n "$CANDIDATE" ]; then
      BINARY_PATH="$CANDIDATE"
    fi
  fi

  if [ ! -d "$DEMO_DIR" ]; then
    echo "Demo workspace not found — preparing now..."
    prepare_demo
  fi
  if [ ! -f "$BINARY_PATH" ]; then
    echo "Build failed or binary not found at $BINARY_PATH"
    echo "You can run: (cd $ROOT_DIR && cargo build) to build the binary"
    exit 1
  fi

  echo "Launching app with CWD=$DEMO_DIR (executable: $BINARY_PATH)"
  cd "$DEMO_DIR"
  exec "$BINARY_PATH"
}

function rebuild() {
  echo "Cleaning and rebuilding..."
  (cd "$ROOT_DIR" && cargo clean && cargo build)
}

case "${1-}" in
  prepare)
    prepare_demo
    ;;
  reset)
    echo "Resetting demo workspace..."
    rm -rf "$DEMO_DIR"
    prepare_demo
    ;;
  build)
    build_binary
    ;;
  run-new)
    # Open a new macOS Terminal window and run the demo there.
    if ! command -v osascript >/dev/null 2>&1; then
      echo "osascript not available on this system. Cannot open macOS Terminal."
      exit 1
    fi
    # ensure binary is available and demo dir exists
    if [ ! -f "$BINARY_PATH" ]; then
      echo "Binary not found — building first..."
      build_binary
    fi
    if [ ! -f "$BINARY_PATH" ]; then
      echo "Build failed or binary not found at $BINARY_PATH"
      exit 1
    fi
    if [ ! -d "$DEMO_DIR" ]; then
      prepare_demo
    fi
    # Escape single quotes and backslashes in paths for AppleScript command
    ESC_DEMO_DIR=$(printf '%s' "$DEMO_DIR" | sed -e "s/'/'\\''/g")
    ESC_BIN=$(printf '%s' "$BINARY_PATH" | sed -e "s/'/'\\''/g")
    # Construct the command to run in the new Terminal window. Use exec so closing
    # the shell ends when the program exits.
    APPLE_CMD="cd '$ESC_DEMO_DIR' && exec '$ESC_BIN'"
    echo "Opening new Terminal window and running: $APPLE_CMD"
    osascript -e "tell application \"Terminal\" to do script \"${APPLE_CMD}\""
    ;;
  run)
    run_binary
    ;;
  rebuild)
    rebuild
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
