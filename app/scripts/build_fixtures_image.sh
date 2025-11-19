#!/usr/bin/env bash
set -euo pipefail

tag=${1:-filezoom-fixtures:latest}
dockerfile=${2:-./docker/Dockerfile}
context=${3:-.}
count=${4:-300}
percent=${5:-40}
max_xattr=${6:-2}
apply_acl=${7:-0}
delete_after=${8:-1}

script_dir=$(dirname "$(realpath "$0")")
fixtures_path=$(realpath "$script_dir/../tests/fixtures")

echo "Generating $count fixtures at $fixtures_path..."
if command -v pwsh >/dev/null 2>&1; then
	pwsh -NoProfile -ExecutionPolicy Bypass -File "$script_dir/generate_fixtures.ps1" -count $count -manifest "$fixtures_path/fixtures_manifest.txt"
else
	echo "pwsh not found; please run the generator manually." >&2
	exit 1
fi

echo "Applying random attributes to fixtures (percent=$percent max_xattr=$max_xattr apply_acl=$apply_acl)..."
if [[ "$apply_acl" -ne 0 ]]; then
	pwsh -NoProfile -ExecutionPolicy Bypass -File "$script_dir/randomize_fixture_attrs.ps1" -fixturesPath "$fixtures_path" -percentChange $percent -maxAds $max_xattr -applyAcl
else
	# call the WSL script for unix-style changes if available; otherwise call the PowerShell randomizer
	if command -v bash >/dev/null 2>&1 && [[ -x "$script_dir/randomize_fixture_attrs_wsl.sh" ]]; then
		bash "$script_dir/randomize_fixture_attrs_wsl.sh" -p "$fixtures_path" -c $percent -x $max_xattr || true
	else
		pwsh -NoProfile -ExecutionPolicy Bypass -File "$script_dir/randomize_fixture_attrs.ps1" -fixturesPath "$fixtures_path" -percentChange $percent -maxAds $max_xattr
	fi
fi

echo "Building Docker image '$tag' using $dockerfile..."
docker build -t "$tag" -f "$dockerfile" "$context"

if [[ "$delete_after" -ne 0 ]]; then
	echo "Deleting fixtures folder $fixtures_path (delete_after enabled)..."
	rm -rf "$fixtures_path"
fi

echo "Build complete: $tag"
