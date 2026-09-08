#!/bin/sh
# Paired main->branch runs, one fixture at a time, main re-run first so both binaries see
# the same machine state. Writes CSV rows: fixture,main,branch,change_pct,lo,hi.
# Point MAIN_EDITOR/BRANCH_EDITOR/MAIN_MD/BRANCH_MD at the two builds of each bench binary
# (`cargo bench -p benchmarks --bench editor_render --no-run` prints the path).
cd "$(dirname "$0")/../../../.."
OUT=${OUT:-target/frame-times/matrix.csv}
MAIN_EDITOR=${MAIN_EDITOR:-target/frame-times/bench-editor-main}
BRANCH_EDITOR=${BRANCH_EDITOR:-target/frame-times/bench-editor-branch}
MAIN_MD=${MAIN_MD:-target/frame-times/bench-md-main}
BRANCH_MD=${BRANCH_MD:-target/frame-times/bench-md-branch}
run() { "$1" --bench --warm-up-time 2 --measurement-time 8 "$3" "$2" 2>&1 | grep -E "time:|change:"; }
row() { # main_bin branch_bin fixture
  m=$(run "$1" "$3" --save-baseline=main | grep time: | sed -E 's/.*\[([0-9.]+) ([a-zµ]+) ([0-9.]+) ([a-zµ]+) ([0-9.]+) ([a-zµ]+)\].*/\3 \4/')
  b=$(run "$2" "$3" --baseline=main)
  bt=$(echo "$b" | grep time: | sed -E 's/.*\[([0-9.]+) ([a-zµ]+) ([0-9.]+) ([a-zµ]+) ([0-9.]+) ([a-zµ]+)\].*/\3 \4/')
  ch=$(echo "$b" | grep change: | sed -E 's/.*\[([-+0-9.]+)% ([-+0-9.]+)% ([-+0-9.]+)%\].*/\2,\1,\3/')
  echo "$3,$m,$bt,$ch" | tee -a "$OUT"
}
: > "$OUT"
for f in "Workbench/update/row" "Workbench/update/editor" "Workbench/update/mixed" "Workbench/update/full" \
         "^editor_render$" "^editor_render_with_editorconfig$" "^open_editor_with_one_long_line$" \
         "Multi-cursor input/cursors/1000$" "Multi-cursor input/cursors/10000" \
         "Siblings/all dirty/64" "Siblings/all dirty/256" "Siblings/all dirty/1024" \
         "Elements/all dirty/256" "Elements/all dirty/2048" "Elements/all dirty/8192" \
         "Elements/incremental/256" "Elements/incremental/2048" "Elements/incremental/8192"; do
  row "$MAIN_EDITOR" "$BRANCH_EDITOR" "$f"
done
for f in "Markdown render/min_bytes/5000$" "Markdown render/min_bytes/10000" "Markdown render/min_bytes/50000"; do
  row "$MAIN_MD" "$BRANCH_MD" "$f"
done
uptime
