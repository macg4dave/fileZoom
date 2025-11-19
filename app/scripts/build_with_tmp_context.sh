#!/usr/bin/env bash
set -euo pipefail

tag=${1:-filezoom-fixtures-baked:tmp}
dockerfile=${2:-./docker/Dockerfile}
count=${3:-300}
delete_after=${4:-1}

script_dir=$(dirname "$(realpath "$0")")
tmp=$(mktemp -d)
echo "Temporary build context: $tmp"

echo "Copying repository into temporary context..."
rsync -a --exclude='.git' --exclude='target' --exclude='app/target' ./ "$tmp/"

echo "Generating fixtures into temporary context..."
tmp_fixtures="$tmp/app/tests/fixtures"
mkdir -p "$tmp_fixtures"
if command -v pwsh >/dev/null 2>&1; then
  pwsh -NoProfile -ExecutionPolicy Bypass -File "$script_dir/generate_fixtures.ps1" -count $count -manifest "$tmp_fixtures/fixtures_manifest.txt"
else
  echo "pwsh not found; please run generator manually inside temp context." >&2
  exit 1
fi

echo "Building Docker image $tag using temporary context..."
docker build -t "$tag" -f "$dockerfile" "$tmp"

if [[ "$delete_after" -ne 0 ]]; then
  echo "Removing temporary context $tmp"
  rm -rf "$tmp"
else
  echo "Temporary context kept at $tmp"
fi

echo "Build complete: $tag"
