#!/usr/bin/env bash
set -euo pipefail

# Apply randomized permissions inside the image to the fixtures directory.
# This script runs during the Docker build as root.

FIXTURES=/work/app/tests/fixtures
if [[ ! -d "$FIXTURES" ]]; then
  echo "No fixtures dir at $FIXTURES, skipping in-image permission changes."
  exit 0
fi

echo "Applying randomized permissions inside image under $FIXTURES"

shopt -s globstar nullglob
files=("$FIXTURES"/**)

for f in "${files[@]}"; do
  # skip directories occasionally
  if [[ -d "$f" ]]; then
    if (( RANDOM % 100 < 50 )); then
      chmod u+rwx,g+rx,o+rx "$f" || true
    else
      chmod 755 "$f" || true
    fi
  else
    r=$((RANDOM % 100))
    if (( r < 10 )); then
      chmod 000 "$f" || true
    elif (( r < 30 )); then
      chmod 444 "$f" || true
    elif (( r < 60 )); then
      chmod 644 "$f" || true
    else
      chmod 600 "$f" || true
    fi
    # occasionally change owner to nobody
    if (( RANDOM % 100 < 5 )); then
      chown nobody:nogroup "$f" || true
    fi
  fi
done

echo "In-image permission changes applied."
