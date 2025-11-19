#!/usr/bin/env bash
set -euo pipefail

tag=${1:-filezoom-fixtures-baked:latest}
test_name=${2:-container_isolation_test}
test_args=${3:-"-- --nocapture"}

echo "Running test '$test_name' inside baked container $tag (no host mount)..."

docker run --rm "$tag" /bin/bash -lc "cd /work/app && BAKED_FIXTURES=1 cargo test -p fileZoom --test $test_name $test_args"

echo "Baked-container test completed. Changes were isolated to the container filesystem."
