#!/usr/bin/env sh
set -eu

# Shows the 30 most recently active remote branches and where they diverged from origin/main.
# "Diverged from" here means: merge-base(origin/main, branch)

DEFAULT_REMOTE="origin"
DEFAULT_MAIN_REF="${DEFAULT_REMOTE}/main"
LIMIT="${1:-30}"

if ! git rev-parse --git-dir >/dev/null 2>&1; then
  echo "error: not inside a git repository" >&2
  exit 1
fi

if ! git show-ref --verify --quiet "refs/remotes/${DEFAULT_MAIN_REF}"; then
  echo "error: can't find ${DEFAULT_MAIN_REF}. Fetch remotes or adjust DEFAULT_MAIN_REF." >&2
  exit 1
fi

# Get top N remote branches by last commit time, excluding origin/HEAD and the bare 'origin' ref.
branches="$(git for-each-ref \
  --sort=-committerdate \
  --format='%(refname:short)' \
  "refs/remotes/${DEFAULT_REMOTE}" \
  | grep -v "^${DEFAULT_REMOTE}\$" \
  | grep -v "^${DEFAULT_REMOTE}/HEAD$" \
  | head -n "${LIMIT}")"

printf "Top %s active branches (by last commit time):\n" "${LIMIT}"
printf "Base ref: %s\n\n" "${DEFAULT_MAIN_REF}"

printf "%-44s  %-10s  %-10s  %-17s  %s\n" "branch" "ahead" "behind" "diverged_at" "branch_tip"
printf "%s\n" "--------------------------------------------------------------------------------------------------------------"

echo "${branches}" | while IFS= read -r branch; do
  [ -n "${branch}" ] || continue

  merge_base="$(git merge-base "${DEFAULT_MAIN_REF}" "${branch}" 2>/dev/null || true)"
  if [ -z "${merge_base}" ]; then
    printf "%-44s  %-10s  %-10s  %-17s  %s\n" "${branch}" "?" "?" "no-common-base" "?"
    continue
  fi

  ahead="$(git rev-list --count "${DEFAULT_MAIN_REF}..${branch}" 2>/dev/null || echo "?")"
  behind="$(git rev-list --count "${branch}..${DEFAULT_MAIN_REF}" 2>/dev/null || echo "?")"

  base_short="$(printf "%s" "${merge_base}" | cut -c1-7)"
  base_when="$(git show -s --format='%cr' "${merge_base}" 2>/dev/null || echo "?")"

  tip_short="$(git show -s --format='%h' "${branch}" 2>/dev/null || echo "?")"
  tip_when="$(git show -s --format='%cr' "${branch}" 2>/dev/null || echo "?")"

  diverged_at="${base_short} (${base_when})"
  branch_tip="${tip_short} (${tip_when})"

  printf "%-44s  %-10s  %-10s  %-17s  %s\n" "${branch}" "${ahead}" "${behind}" "${diverged_at}" "${branch_tip}"
done
