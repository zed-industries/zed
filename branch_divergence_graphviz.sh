#!/usr/bin/env sh
set -eu

# Generates a Graphviz DOT representation of the N most recently active remote branches
# and the commits from which they diverged from origin/main.
#
# Usage:
#   ./branch_divergence_graphviz.sh [limit]
#   ./branch_divergence_graphviz.sh 30 | dot -Tpng -o branches.png && open branches.png
#   ./branch_divergence_graphviz.sh 30 | dot -Tsvg -o branches.svg && open branches.svg

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

# Collect all unique merge-base commits
merge_bases=""

# Start DOT output
cat <<'HEADER'
digraph branch_divergence {
  rankdir=LR;
  node [shape=box, style=filled, fontname="Helvetica"];
  edge [fontname="Helvetica", fontsize=10];
  nodesep=0.15;
  ranksep=1.0;

  // Main branch (highlighted)
  main [label="origin/main", fillcolor="#4CAF50", fontcolor="white"];

HEADER

# Process each branch
echo "${branches}" | while IFS= read -r branch; do
  [ -n "${branch}" ] || continue

  # Skip main itself
  if [ "${branch}" = "${DEFAULT_MAIN_REF}" ]; then
    continue
  fi

  merge_base="$(git merge-base "${DEFAULT_MAIN_REF}" "${branch}" 2>/dev/null || true)"
  if [ -z "${merge_base}" ]; then
    continue
  fi

  ahead="$(git rev-list --count "${DEFAULT_MAIN_REF}..${branch}" 2>/dev/null || echo "?")"
  behind="$(git rev-list --count "${branch}..${DEFAULT_MAIN_REF}" 2>/dev/null || echo "?")"

  base_short="$(printf "%s" "${merge_base}" | cut -c1-7)"
  base_when="$(git show -s --format='%cr' "${merge_base}" 2>/dev/null || echo "?")"

  # Sanitize branch name for DOT node ID (replace special chars)
  branch_id="$(printf "%s" "${branch}" | sed 's|[^a-zA-Z0-9]|_|g')"
  base_id="base_${base_short}"

  # Short display name (remove origin/ prefix)
  branch_display="$(printf "%s" "${branch}" | sed 's|^origin/||')"

  # Color based on ahead count
  if [ "${ahead}" -gt 50 ]; then
    fillcolor="#FF5722"  # deep orange for large branches
  elif [ "${ahead}" -gt 20 ]; then
    fillcolor="#FFC107"  # amber for medium branches
  elif [ "${ahead}" -gt 5 ]; then
    fillcolor="#03A9F4"  # light blue for small branches
  else
    fillcolor="#E0E0E0"  # gray for tiny branches
  fi

  # Output branch node
  printf '  %s [label="%s\\n+%s/-%s", fillcolor="%s"];\n' \
    "${branch_id}" "${branch_display}" "${ahead}" "${behind}" "${fillcolor}"

  # Output merge-base node (will be deduplicated by graphviz)
  printf '  %s [label="%s\\n%s", fillcolor="#9E9E9E", shape=ellipse];\n' \
    "${base_id}" "${base_short}" "${base_when}"

  # Edge from main to merge-base (to show it's on main's history)
  printf '  main -> %s [style=dashed, color="#888888"];\n' "${base_id}"

  # Edge from merge-base to branch
  printf '  %s -> %s;\n' "${base_id}" "${branch_id}"

done

# Close the graph
cat <<'FOOTER'

  // Legend
  subgraph cluster_legend {
    label="Legend";
    style=filled;
    fillcolor="#FAFAFA";
    fontname="Helvetica";
    rank=sink;

    legend_large [label="50+ commits", fillcolor="#FF5722"];
    legend_medium [label="20-50 commits", fillcolor="#FFC107"];
    legend_small [label="5-20 commits", fillcolor="#03A9F4"];
    legend_tiny [label="<5 commits", fillcolor="#E0E0E0"];
    legend_base [label="fork point", fillcolor="#9E9E9E", shape=ellipse];

    legend_large -> legend_medium -> legend_small -> legend_tiny -> legend_base [style=invis];
  }
}
FOOTER
