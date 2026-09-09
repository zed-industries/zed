#!/bin/sh
# Paired main->branch runs of every `Complexity` point (see `complexity_points()` in
# crates/benchmarks/benches/editor_render.rs), same columns as matrix.sh:
#   fixture,main,branch,change_pct,lo,hi,main_rss,branch_rss,main_rss_max,branch_rss_max
# `complexity.py` fits the frame cost model from the CSV and draws the charts.
cd "$(dirname "$0")/../../../.."
OUT=${OUT:-target/frame-times/complexity.csv}
MAIN_EDITOR=${MAIN_EDITOR:-target/frame-times/bench-editor-main}
BRANCH_EDITOR=${BRANCH_EDITOR:-target/frame-times/bench-editor-branch}
run() { "$1" --bench --warm-up-time 2 --measurement-time 8 "$3" "$2" 2>&1 | grep -E "time:|change:|first measurement:|max after any measurement:"; }
median() { grep time: | sed -E 's/.*\[([0-9.]+) ([a-zµ]+) ([0-9.]+) ([a-zµ]+) ([0-9.]+) ([a-zµ]+)\].*/\3 \4/'; }
rss() { grep 'first measurement:' | sed -E 's/.* ([0-9.]+) MB after.*/\1/'; }
rss_max() { grep 'max after any measurement:' | sed -E 's/.*: ([0-9.]+) MB.*/\1/'; }
row() { # main_bin branch_bin fixture
  m=$(run "$1" "$3" --save-baseline=main)
  b=$(run "$2" "$3" --baseline=main)
  ch=$(echo "$b" | grep change: | sed -E 's/.*\[([-+0-9.]+)% ([-+0-9.]+)% ([-+0-9.]+)%\].*/\2,\1,\3/')
  echo "$3,$(echo "$m" | median),$(echo "$b" | median),$ch,$(echo "$m" | rss),$(echo "$b" | rss),$(echo "$m" | rss_max),$(echo "$b" | rss_max)" | tee -a "$OUT"
}
ALL="v16-e128-p4-f25 v4-e128-p4-f25 v64-e128-p4-f25 v16-e32-p4-f25 v16-e512-p4-f25 \
     v16-e128-p1-f25 v16-e128-p12-f25 v16-e128-p4-f0 v16-e128-p4-f6 v16-e128-p4-f100 \
     v64-e128-p4-f6 v16-e512-p4-f6 v4-e512-p4-f100 v64-e32-p4-f100 \
     v16-e128-p1-f100 v16-e128-p12-f100 v16-e128-p1-f0 v16-e128-p12-f0 v16-e512-p4-f0 v16-e512-p4-f100"
EDITORS="k1-d0-cursor k1-d1-cursor k1-d1-scroll k4-d0-cursor k4-d1-cursor k4-d1-scroll k4-d4-cursor k4-d4-scroll"
# POINTS / EDITOR_POINTS override the lists (and append to OUT instead of truncating it), for
# adding points; set the other to a space to skip it.
[ -n "${POINTS:-}${EDITOR_POINTS:-}" ] || : > "$OUT"
for p in ${POINTS:-$ALL}; do
  row "$MAIN_EDITOR" "$BRANCH_EDITOR" "Complexity/scene/$p\$"
done
for p in ${EDITOR_POINTS:-$EDITORS}; do
  row "$MAIN_EDITOR" "$BRANCH_EDITOR" "Complexity/editors/$p\$"
done
uptime
