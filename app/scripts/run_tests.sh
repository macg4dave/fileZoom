#!/usr/bin/env bash
# Run app crate tests with output shown
set -euo pipefail
cd "$(dirname "$0")/.."
# Run tests for the `app` crate and show output
cargo test -p app -- --nocapture
