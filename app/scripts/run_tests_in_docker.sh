#!/usr/bin/env bash
set -euo pipefail

tag=${1:-filezoom-fixtures:latest}
interactive=${2:-0}
test_args=${3:-"-- --nocapture"}

if [[ "$interactive" -ne 0 ]]; then
  docker run --rm -it -v "$PWD":/work "$tag" /bin/bash
  exit 0
fi

echo "Running tests inside container $tag..."
docker run --rm -v "$PWD":/work "$tag" /bin/bash -lc "cd /work/app && cargo test -p fileZoom $test_args"
echo "Tests completed inside container."
