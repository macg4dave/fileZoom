#!/usr/bin/env bash
set -euo pipefail

# Randomize permissions, times, xattrs, symlinks, and FIFOs under a fixtures directory (WSL/unix side)
# Usage: randomize_fixture_attrs_wsl.sh [-p fixtures_path] [-c percent_change] [-x max_xattrs] [--chown]

fixtures="$(dirname "$(realpath "$0")")/../tests/fixtures"
percent=40
max_xattr=2
do_chown=0

while [[ $# -gt 0 ]]; do
  case "$1" in
    -p|--path) fixtures="$2"; shift 2 ;;
    -c|--percent) percent="$2"; shift 2 ;;
    -x|--max-xattr) max_xattr="$2"; shift 2 ;;
    --chown) do_chown=1; shift 1 ;;
    -h|--help) echo "Usage: $0 [-p fixtures_path] [-c percent_change] [-x max_xattr] [--chown]"; exit 0 ;;
    *) echo "Unknown arg: $1"; exit 1 ;;
  esac
done

if [[ ! -d "$fixtures" ]]; then
  echo "Fixtures directory not found: $fixtures" >&2
  exit 2
fi

count_modified=0
count_files=0
count_dirs=0

should_modify() {
  local chance=$1
  (( RANDOM % 100 < chance )) && return 0 || return 1
}

pick_perm() {
  # Common Unix permission modes to choose from
  local modes=("644" "600" "666" "755" "700" "644" "444" "000" "664")
  echo "${modes[RANDOM % ${#modes[@]}]}"
}

random_time() {
  # Random date within last 365 days
  local days=$((RANDOM % 365))
  local secs=$((RANDOM % 86400))
  date -d "-$days days -$secs seconds" +"%Y-%m-%dT%H:%M:%S"
}

has_setfattr() {
  command -v setfattr >/dev/null 2>&1
}

# Iterate
while IFS= read -r -d '' path; do
  if [[ -d "$path" ]]; then
    ((count_dirs++))
  else
    ((count_files++))
  fi

  if ! should_modify "$percent"; then
    continue
  fi

  ((count_modified++))

  # Change permissions
  perm="$(pick_perm)"
  if [[ -d "$path" ]]; then
    # for directories, ensure exec bit for traversal for many perms
    chmod "0${perm}" "$path" 2>/dev/null || true
  else
    chmod "0${perm}" "$path" 2>/dev/null || true
  fi

  # Randomize timestamps
  if should_modify 60; then
    rt="$(random_time)"
    touch -d "$rt" "$path" 2>/dev/null || true
  fi

  # Add extended attributes if available
  if has_setfattr && ! [[ -d "$path" ]] && should_modify 30; then
    xcount=$((RANDOM % (max_xattr + 1)))
    for ((i=0;i<xcount;i++)); do
      val="xattr-${RANDOM}"
      setfattr -n user.random${i} -v "$val" "$path" 2>/dev/null || true
    done
  fi

  # Create a symlink pointing to other fixture occasionally
  if should_modify 10 && ! [[ -d "$path" ]]; then
    tgt_dir=$(dirname "$path")
    # Safely read directory entries into an array
    mapfile -t base_names < <(ls -A "$tgt_dir" 2>/dev/null || true)
    if [[ ${#base_names[@]} -gt 1 ]]; then
      # pick a random target different from current
      idx=$((RANDOM % ${#base_names[@]}))
      tgt="${base_names[$idx]}"
      if [[ "$tgt" != "$(basename "$path")" ]]; then
        ln -sf "$tgt_dir/$tgt" "$path.link" 2>/dev/null || true
      fi
    fi
  fi

  # Create FIFO occasionally
  if should_modify 3 && [[ -d "$path" ]]; then
    mkfifo "$path/fifo_$(date +%s_%N)" 2>/dev/null || true
  fi

  # Optionally chown to current user (if requested and script run as root)
  if [[ "$do_chown" -eq 1 ]]; then
    chown "$(id -u):$(id -g)" "$path" 2>/dev/null || true
  fi

done < <(find "$fixtures" -mindepth 1 -print0)

echo "Randomization finished: files=$count_files dirs=$count_dirs modified~=$count_modified"
exit 0
