#!/usr/bin/env bash
# S2 speed benchmark harness (ADR-0026, ADR-0037), re-run for S9 (ADR-0058).
# Tasks B1-B10 are in the cedian docs repo, docs/plans/done/s2-workflow-core.md.
#
#   script/cedian-bench/run.sh check <id|all>   prove each done predicate: it passes at the
#                                        reference commit and fails at the start commit.
#                                        No model, no OMP.
#   BENCH_GO=1 MANDATE=on|off script/cedian-bench/run.sh run <id|all>
#                                        run cedian (this checkout's release build) on the
#                                        task in a fresh worktree, check the predicate,
#                                        then remove the worktree. One model turn per task.
#   BENCH_GO=1 script/cedian-bench/run.sh record <lane|all>
#                                        re-record one hand-cut fixture (ADR-0058 decision 5).
#   script/cedian-bench/run.sh model all  the pin, and the pass's model in OMP's
#                                        own catalog. No model, no turn.
#   script/cedian-bench/run.sh plan all  the live pass in order, with its model turns.
#   script/cedian-bench/run.sh turns all the turns this pass has used.
#   script/cedian-bench/run.sh compare <on|off>  results against the S2 baseline.
#   script/cedian-bench/run.sh snapshot <before|after>  the person's ~/.omp around the pass.
#
# A pass is named by BENCH_PASS (required for run, record, turns, compare). Its
# turn ledger and results live in $BENCH_RESULTS/<pass>/ (default
# ~/.cache/cedian-bench), never in a temp dir a reboot clears. The cap is
# MAX_TURNS (default 46, at most 46): a step starts only if its worst case fits
# (estimate + 1, + 1 if its turn may spawn a reviewer), under flock; after it,
# the turns booked are the `agent_start`s every OMP process it started printed
# (script/cedian-bench/cap.py), and the pass stops once the cap is reached.
# Every OMP cedian starts goes through a wrapper that taps its rpc stdout;
# MANDATE=off also adds `--append-system-prompt ""`, as the reviewer is run,
# so ~/.omp/agent/APPEND_SYSTEM.md does not reach it.
# A run or fixture whose OMP reported a model other than BENCH_MODEL is not
# scored, compared or written.
# Work dir (worktrees, build): $BENCH_WORK (default /tmp/cedian-bench).
# Results: $BENCH_RESULTS/<pass>/results/<mandate>/<id>.json.
set -euo pipefail

ROOT=$(unset GIT_DIR GIT_WORK_TREE; git -C "$(dirname "$0")" rev-parse --show-toplevel)
BENCH=$ROOT/script/cedian-bench
# The tasks start from commits in the cedian docs repo, which keeps the code
# history from before the move into the fork (ADR-0042). The binary under test
# is built here.
TASKS=${BENCH_TASKS_REPO:-$HOME/cedian}
WORK=${BENCH_WORK:-/tmp/cedian-bench}
export CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-$WORK/target}
export PATH="$HOME/.cargo/bin:$HOME/.local/bin:$PATH"
# The OMP that runs is the pin, checked by version before any model turn.
PIN=$(python3 -c 'import json,sys; print(json.load(open(sys.argv[1]))["spikeVerified"]["ompVersion"])' \
  "$ROOT/vendor/omp-revision.json")
OMP=${CEDIAN_OMP_BINARY:-$HOME/.local/bin/omp}
# The S2 baseline's model (ADR-0058 decision 1): a run on another is refused.
BENCH_MODEL=${BENCH_MODEL:-opencode-go/muse-spark-1.3-contributor}
MAX_TURNS=${MAX_TURNS:-46}
[ "$MAX_TURNS" -le 46 ] 2>/dev/null || { echo "refused: MAX_TURNS=$MAX_TURNS; the cap is at most 46" >&2; exit 2; }
RESULTS_ROOT=${BENCH_RESULTS:-$HOME/.cache/cedian-bench}
PASS=${BENCH_PASS:-}
PASS_DIR=$RESULTS_ROOT/$PASS
LEDGER=$PASS_DIR/turns.log
CAP="python3 $BENCH/cap.py"
S2_RESULTS=${S2_RESULTS:-$TASKS/docs/benchmarks/2026-10-07}
ALL="b1 b2 b3 b4 b5 b6 b7 b8 b9 b10"
# B8 is open work: it starts where the harness was written.
B8_BASE=ef2f8b7

task() {
  case $1 in
  b1) BASE=e98674d^ REF=e98674d
      PROMPT="In crates/cedian_workspace, HostTools resolve() rejects buffer keys that start with a slash, like /notes.txt, even when the key names an open buffer or a file inside the workspace; OMP 18.6 sends keys in that form. Make resolve accept a /-key when it names an open buffer or a workspace file, and keep rejecting real absolute paths outside the workspace. Add tests." ;;
  b2) BASE=e98674d^ REF=e98674d
      PROMPT="OMP 18.6 runs cedian host tools as writes to xd://<tool> (for example a write whose path is xd://cedian_apply_edit). In crates/cedian_agent_ui the tool cards show these as a file write. Name such a card after the host tool instead (tool_card::card_for_tool and the thread's tool cards in message.rs); a read of xd://<tool> (OMP reading the tool's docs) stays a read and is never shown as an edit. Put the detection in pub fn tool_card::host_device<'a>(name: &str, preview: &'a str) -> Option<(&'a str, bool)>: Some((tool, true)) for a write to xd://<tool>, Some((tool, false)) for a read of it, None otherwise (other tools, other paths, an empty device). In render_thread, the write becomes a card named after the tool, titled 'Host tool' with the tool name as its preview; the read keeps the name read and is titled 'Read host tool docs'. Add tests." ;;
  b3) BASE=6b0aac0^ REF=6b0aac0
      PROMPT="cedian_worker::worktree::remove deletes a worker's branch with git branch -D, which destroys commits that were never merged. Make remove return an error and keep the branch (and its commits) when the worker branch has unmerged commits; a worker with nothing unmerged is still removed. Add tests." ;;
  b4) BASE=6b0aac0^ REF=6b0aac0
      PROMPT="HostTools::attach_lsp in crates/cedian_workspace deadlocks when a buffer is open: it holds the store lock while syncing buffers that need the same lock. Fix it so cargo test -p cedian_workspace finishes. The fake LSP bridge test (crates/cedian_workspace/tests/fake_lsp_bridge.rs) hardcodes one machine's paths; make it use paths relative to the crate manifest." ;;
  b5) BASE=264680f^ REF=264680f
      PROMPT="Give cedian_omp's EventRouter a log of finished tool calls. A call enters it only when its ToolEnd arrives (a ToolStart alone never does). Add EventRouter::finished_tool_call(tool_call_id) -> Option<FinishedToolCall>, where FinishedToolCall { tool_name: String, args_preview: String, is_error: bool } (args_preview from the ToolStart) derives Debug, Clone, PartialEq, Eq and is exported from the crate root. Add tests." ;;
  b6) BASE=378d623^ REF=378d623
      PROMPT="Headless cedian (the CLI) never answers OMP's extension UI dialogs, so an approval dialog stalls the turn until the prompt timeout. Add a module cedian_omp::headless_ui with pub fn headless_answer(&ExtensionUiRequest) -> Option<(ExtensionUiResponse, String)> that fails closed: a select gets the option named deny (case-insensitive) or is cancelled when there is none; a confirm is declined; input, editor and ask are cancelled; fire-and-forget requests (notify, status, widget and the like) get None. The String is a one-line label of the dialog title. Wire it into OmpRuntime (deny_ui_requests() turns it on; take_refused_ui_requests() returns the labels) and have the CLI print '[✗] refused (no UI to approve): <label>' for each after a turn. The replay fixture crates/cedian_cli/tests/fixtures/p5_worktree.jsonl is provided." ;;
  b7) BASE=09e88b6^ REF=09e88b6
      PROMPT="The worker registry (.cedian/workers.json, cedian_worker::Registry) has no snapshot_version, and a corrupt file is silently treated as empty. Add snapshot_version 1. Registry::open(repo) changes from returning (Registry, bool) to Result<(Registry, bool), WorkerError> and fails with a new WorkerError::Snapshot(String) when the version is missing or different, or the file is not valid JSON. Add tests." ;;
  b8) BASE=$B8_BASE REF=
      PROMPT="cedian workflow run in crates/cedian_cli/src/main.rs hand-parses the task kind and risk with string match arms. Reuse the serde names the workflow channel already uses for TaskKind and Risk instead, so there is one spelling. cedian workflow run with an unknown kind must still fail and list every kind." ;;
  b9) BASE=3d66849^ REF=3d66849
      PROMPT="Write crates/cedian_omp/tests/omp_parity.rs: a test that parses the vendored vendor/omp-rpc/src/wire.rs for every RPC command, server notification and extension UI request (by wire name) and fails, naming each feature, when docs/OMP_PARITY.md has no row mentioning it in backticks. It must also fail when the parser finds fewer than 60 commands, 40 notifications or 10 UI requests (the parser no longer matching the generated file). Add any missing ledger rows so it passes." ;;
  b10) BASE=09e88b6^ REF=09e88b6
      PROMPT="The browser store (.cedian/browser.json, crates/cedian_cli/src/browser_store.rs) has no snapshot_version. Add one; loading a file whose version is missing or different must fail with an error that says the state is too old and names the reset command, cedian browser open <url>. Keep BrowserHead's fields as they are (port, ws_url, url, seq): the version lives only in the file. Add tests." ;;
  *) echo "unknown task $1" >&2; exit 2 ;;
  esac
}

splice() { python3 "$BENCH/splice_tests.py" "$1" "$REF"; }

# The done predicate, run inside a checkout of the result. Exit 0 = done.
predicate() {
  local id=$1
  case $id in
  b1) splice crates/cedian_workspace/src/host.rs &&
      cargo test -q -p cedian_workspace --lib host:: ;;
  b2) splice crates/cedian_agent_ui/src/message.rs &&
      splice crates/cedian_agent_ui/src/tool_card.rs &&
      cargo test -q -p cedian_agent_ui --lib ;;
  b3) mkdir -p crates/cedian_worker/tests &&
      cp "$BENCH/hidden/b3_remove.rs" crates/cedian_worker/tests/bench_b3.rs &&
      cargo test -q -p cedian_worker --test bench_b3 ;;
  b4) git show "$REF:crates/cedian_workspace/tests/fake_lsp_bridge.rs" \
        > crates/cedian_workspace/tests/fake_lsp_bridge.rs &&
      cargo test -q -p cedian_workspace --no-run &&
      perl -e 'alarm shift; exec @ARGV' 60 cargo test -q -p cedian_workspace ;;
  b5) splice crates/cedian_omp/src/event_router.rs &&
      cargo test -q -p cedian_omp --lib event_router ;;
  b6) splice crates/cedian_omp/src/headless_ui.rs &&
      git show "$REF:crates/cedian_cli/tests/replay_cli.rs" > crates/cedian_cli/tests/replay_cli.rs &&
      cargo test -q -p cedian_omp --lib headless_ui &&
      cargo test -q -p cedian_cli --test replay_cli ;;
  b7) splice crates/cedian_worker/src/registry.rs &&
      cargo test -q -p cedian_worker --lib registry ;;
  b8) ! grep -q '"bug_fix" =>' crates/cedian_cli/src/main.rs &&
      cargo build -q -p cedian_cli &&
      b8_unknown_kind_lists_kinds ;;
  b9) b9_parity_test ;;
  b10) splice crates/cedian_cli/src/browser_store.rs &&
       python3 "$BENCH/insert_test.py" crates/cedian_cli/src/browser_store.rs "$BENCH/hidden/b10_reset.rs" &&
       cargo test -q -p cedian_cli --bin cedian browser_store ;;
  esac && cargo test -q --workspace
}

# The predicate's outcome, split so a correct fix whose private names differ
# from the reference commit is not scored as a failure. Prints one word:
#   pass                 the predicate holds
#   fail                 the result does not build, or builds and the predicate fails
#   hidden_tests_broken  the result builds but the hidden tests do not compile
#                        against it; neither pass nor false-done, the owner reviews it
verdict() {
  local id=$1
  cargo build -q --workspace --all-targets >&2 || { echo fail; return; }
  predicate "$id" >&2 && { echo pass; return; }
  if cargo test -q --workspace --no-run >&2; then echo fail; else echo hidden_tests_broken; fi
}

b8_unknown_kind_lists_kinds() {
  local tmp out
  tmp=$(mktemp -d)
  printf 'schema = 1\n' > "$tmp/cedian.toml"
  if out=$(CEDIAN_CONFIG=$tmp/cedian.toml CEDIAN_WORKDIR=$tmp \
      "$CARGO_TARGET_DIR/debug/cedian" workflow run nope x 2>&1); then
    echo "b8: unknown kind accepted" >&2; return 1
  fi
  for kind in investigation bug_fix feature refactor performance prototype; do
    grep -q "$kind" <<<"$out" || { echo "b8: error does not list $kind: $out" >&2; return 1; }
  done
}

b9_parity_test() {
  local t=crates/cedian_omp/tests/omp_parity.rs out
  test -f $t || { echo "b9: no $t" >&2; return 1; }
  for floor in 60 40 10; do
    grep -q "\b$floor\b" $t || { echo "b9: no floor $floor in $t" >&2; return 1; }
  done
  cargo test -q -p cedian_omp --test omp_parity || return 1
  cp docs/OMP_PARITY.md "$WORK/parity.bak"
  sed -i '' 's/^| `steer` |/| `steer_renamed` |/' docs/OMP_PARITY.md
  if out=$(cargo test -q -p cedian_omp --test omp_parity 2>&1); then
    cp "$WORK/parity.bak" docs/OMP_PARITY.md
    echo "b9: test still passes with the steer row renamed" >&2; return 1
  fi
  cp "$WORK/parity.bak" docs/OMP_PARITY.md
  grep -q 'steer' <<<"$out" || { echo "b9: failure does not name steer" >&2; return 1; }
}

worktree() { # <name> <rev> -> path
  local dir=$WORK/trees/$1
  git -C "$TASKS" worktree remove --force "$dir" 2>/dev/null || true
  git -C "$TASKS" worktree add -q --detach "$dir" "$2"
  echo "$dir"
}

drop() { git -C "$TASKS" worktree remove --force "$1"; }

check() {
  local id=$1 dir v
  task "$id"
  if [ -n "$REF" ]; then
    dir=$(worktree "$id-ref" "$REF")
    if (cd "$dir" && predicate "$id") >"$WORK/logs/$id-ref.log" 2>&1; then
      echo "$id: passes at $REF"
    else
      echo "$id: FAILS at its reference commit $REF (predicate wrong; see $WORK/logs/$id-ref.log)"
      drop "$dir"; return 1
    fi
    drop "$dir"
  fi
  dir=$(worktree "$id-base" "$BASE")
  v=$(cd "$dir" && verdict "$id" 2>"$WORK/logs/$id-base.log")
  if [ "$v" = pass ]; then
    echo "$id: PASSES at its start commit $BASE (predicate does not detect the task)"
    drop "$dir"; return 1
  fi
  echo "$id: $v at $BASE"
  drop "$dir"
}

# The OMP named by $OMP is the pin, or nothing runs (ADR-0058 decision 1).
pin_check() {
  local got
  got=$("$OMP" --version 2>/dev/null | head -1) || got="no omp at $OMP"
  [ "$got" = "omp/$PIN" ] || {
    echo "refused: cedian is pinned to OMP $PIN; $OMP says '$got'. Set CEDIAN_OMP_BINARY to the pinned omp." >&2
    exit 2
  }
}

# What the pass runs on: $BENCH_MODEL, the model S2's baseline ran (ADR-0058
# decision 1). `wrapper` pins it into every spawn (ADR-0061), so the person's
# own `modelRoles.default` is theirs and may be another model; this check
# reads it only to say so, and requires the pinned model to be in OMP's own
# catalog (ADR-0059 decision 3), or every step would fail after its turns.
model_check() {
  local model listed verdict
  model=$("$OMP" config get modelRoles --json | python3 -c 'import json,sys; print(json.load(sys.stdin)["value"].get("default",""))')
  listed=$("$OMP" models ls --json 2>/dev/null) || listed=""
  verdict=$(printf '%s' "$listed" | python3 -c '
import json, sys
try:
    models = json.load(sys.stdin)["models"]
except Exception:
    print("unreadable")
    raise SystemExit(0)
print("listed" if any(m.get("selector") == sys.argv[1] for m in models) else "missing")
' "$BENCH_MODEL")
  case $verdict in
  listed) ;;
  missing)
    echo "refused: OMP's catalog has no $BENCH_MODEL; a step would spend its turns finding out" >&2
    exit 2 ;;
  *)
    echo "refused: could not read OMP's model catalog, so $BENCH_MODEL is unverified" >&2
    exit 2 ;;
  esac
  echo "the pass pins $BENCH_MODEL; the person's default is '${model:-none}'" >&2
}

# What cedian spawns, written beside the tap dir `$2`: the pin, its rpc stdout
# teed into one file per process under `$2`, `--model $BENCH_MODEL` added
# wherever `--mode` is (ADR-0061: the pass pins the model S2's baseline ran,
# so the person's own `modelRoles.default` decides their other work, not this
# pass), and `--append-system-prompt ""` when the mandate is off (as the
# reviewer's spawn profile does). Prints the wrapper's path.
wrapper() {
  local mandate=$1 tap=$2 w=$2.omp off
  case $mandate in
  on) off=0 ;; off) off=1 ;;
  *) echo "MANDATE must be on or off (ADR-0058 decision 2)" >&2; exit 2 ;;
  esac
  mkdir -p "$tap"
  cat > "$w" <<WRAP
#!/bin/bash
rpc=0
for a in "\$@"; do case \$a in --mode|--mode=*) rpc=1 ;; esac; done
[ \$rpc = 1 ] || exec $(printf %q "$OMP") "\$@"
exec > >(tee $(printf %q "$tap")/omp.\$\$.jsonl)
[ $off = 1 ] && exec $(printf %q "$OMP") "\$@" --model $(printf %q "$BENCH_MODEL") --append-system-prompt ""
exec $(printf %q "$OMP") "\$@" --model $(printf %q "$BENCH_MODEL")
WRAP
  chmod 755 "$w"
  echo "$w"
}

# Free disk before a step: refuse under the floor, 10 GB by default. `need` is
# what the step adds. `BENCH_DISK_FLOOR_GB` moves the floor so a test can leave
# this machine's free space out of what it proves (`test_bench.py` sets 0: its
# subject is the turn ledger, not this disk).
preflight() {
  local need=$1 free floor=${BENCH_DISK_FLOOR_GB:-10}
  free=$(df -Pk "$WORK" | awk 'NR==2 { print int($4 / 1048576) }')
  if [ "$free" -lt "$floor" ]; then
    echo "refused: ${free} GB free under $WORK, the floor is ${floor} GB; this step needs about ${need} GB (run script/cedian-prune 10, or free disk)" >&2
    exit 2
  fi
  echo "disk: ${free} GB free; this step needs about ${need} GB" >&2
}

need_pass() {
  [ -n "$PASS" ] || { echo "BENCH_PASS names this pass (its ledger and results)" >&2; exit 2; }
}

# A step that cannot be admitted ends the pass: 3 is the cap (a partial exit,
# ADR-0058 decision 5), anything else is a refusal (a broken ledger, a cap
# mismatch) and stops with its own code. `admit` returns 3 rather than exiting,
# because inside `reserved=$(admit …)` an `exit` only leaves the subshell and
# the step then runs unbooked.
stop_pass() { [ "$1" = 3 ] && exit 0; exit "$1"; }

# Reserve `what`'s worst case, or stop the pass before it. Prints the
# reservation.
admit() {
  local r=0 out
  out=$($CAP admit "$LEDGER" "$MAX_TURNS" "$1" "$2" "$3") || r=$?
  case $r in
  0) echo "$out" ;;
  3) echo "$out" >&2; return 3 ;;
  *) return "$r" ;;
  esac
}

# Book what `step` used; stop the pass when the cap is reached.
book() {
  local r=0
  $CAP book "$LEDGER" "$@" || r=$?
  case $r in 0) ;; 3) exit 0 ;; *) exit "$r" ;; esac
}

# Turns the step's OMP processes started, or its reservation when the tap
# saw none (nothing proves the step used none).
tapped() {
  local n
  n=$($CAP count "$1")
  if [ "$n" -gt 0 ]; then echo "$n"; else echo "$2"; fi
}

run() {
  local id=$1 dir cfg start end out verdict omp results reserved tap used observed
  [ "${BENCH_GO:-}" = 1 ] || { echo "run needs the owner's go: BENCH_GO=1 (ADR-0037)" >&2; exit 2; }
  # Two packages ship a binary named `cedian` (the app in `zed`, the headless
  # CLI in `cedian_cli`) and they share one output path, so building the app
  # after the CLI leaves the app where a run expects the CLI. `prompt` is the
  # CLI's subcommand; the app would take it as a path to open.
  "$CARGO_TARGET_DIR/release/cedian" --help 2>&1 | grep -q "prompt" || {
    echo "refused: $CARGO_TARGET_DIR/release/cedian is not the cedian CLI (the app's bin has the same name); build it with: cargo build -q --release -p cedian_cli" >&2
    exit 2
  }
  task "$id"
  preflight 5
  reserved=$(admit "$id-$MANDATE" 1 1) || stop_pass $?
  tap=$PASS_DIR/taps/$MANDATE/$id
  rm -rf "$tap"
  omp=$(wrapper "$MANDATE" "$tap")
  results=$PASS_DIR/results/$MANDATE
  mkdir -p "$results"
  dir=$(worktree "$id-run" "$BASE")
  [ "$id" = b6 ] && git show "$REF:crates/cedian_cli/tests/fixtures/p5_worktree.jsonl" \
      > "$dir/crates/cedian_cli/tests/fixtures/p5_worktree.jsonl"
  dir=$(cd "$dir" && pwd -P)
  cfg=$WORK/run-$id.toml
  # The agent must run cargo: the run uses the owner's OMP config (ADR-0035).
  printf 'schema = 1\n[projects."%s"]\npolicy = "omp"\n' "$dir" > "$cfg"
  : > "$results/$id.timing.jsonl"
  start=$(python3 -c 'import time; print(int(time.time()*1000))')
  out=$(CEDIAN_OMP_BINARY=$omp CEDIAN_CONFIG=$cfg CEDIAN_WORKDIR=$dir \
        CEDIAN_SESSION_DIR=$WORK/sessions/$MANDATE/$id CEDIAN_STATE_DIR=$WORK/state/$MANDATE/$id \
        CEDIAN_TIMING=$results/$id.timing.jsonl \
        "$CARGO_TARGET_DIR/release/cedian" prompt "$PROMPT" 2>&1) || true
  end=$(python3 -c 'import time; print(int(time.time()*1000))')
  printf '%s\n' "$out" > "$WORK/logs/$id-$MANDATE-run.out"
  used=$(tapped "$tap" "$reserved")
  observed=$($CAP models "$tap")
  if [ "$observed" = "$BENCH_MODEL" ]; then
    verdict=$(cd "$dir" && verdict "$id" 2>"$WORK/logs/$id-$MANDATE-predicate.log")
  else
    echo "$id: OMP ran '$observed', not $BENCH_MODEL: not scored" >&2
    verdict=refused_model
  fi
  (cd "$dir" && git diff > "$WORK/logs/$id-$MANDATE.patch") || true
  python3 - "$id" "$verdict" "$((end - start))" "$WORK" "$results" "$MANDATE" "$OMP" "$BENCH_MODEL" "$observed" "$used" <<'PY'
import json, re, subprocess, sys
id, verdict, wall, work, results, mandate, omp_bin, model, observed, used = sys.argv[1:11]
wall = int(wall)
out = open(f"{work}/logs/{id}-{mandate}-run.out").read()
timing = [json.loads(l) for l in open(f"{results}/{id}.timing.jsonl") if l.strip()]
claimed = bool(re.search(r"\b(done|fixed|complete[d]?|implemented)\b", out.split("\n[")[0], re.I))
omp = subprocess.run([omp_bin, "--version"], capture_output=True, text=True).stdout.strip()
json.dump({
    "task": id, "predicate": verdict, "wall_ms": wall,
    "time_to_usable_result_ms": wall if verdict == "pass" else None,
    "claimed_done": claimed, "false_done": claimed and verdict == "fail",
    "omp_version": omp, "omp_binary": omp_bin, "model": model, "mandate": mandate,
    "observed_models": observed.split(), "model_turns": int(used),
    "timing": timing,
}, open(f"{results}/{id}.json", "w"), indent=2)
print(f"{id} (mandate {mandate}): predicate {verdict}, wall {wall} ms, claimed_done {claimed}")
PY
  drop "$dir"
  book "$id-$MANDATE" "$reserved" "$used"
}

# Record lanes: the env that arms one, the test that records it, and its
# model turns (OMP `agent_start`s); each lane prints what it used. The lanes
# keep the person's HOME, so the pinned OMP reads their auth and mandate as in
# real use (ADR-0058 decision 5); sessions go to the test's temp dir.
lane() {
  LANE_TEST="-p cedian_cli --test replay_cli"
  case $1 in
  cli|s2|channel|worktree|p8|p8deny|s2profile) LANE_TURNS=1 LANE_ENV="CEDIAN_P2_RECORD=$1" ;;
  shell|s3review|s3same) LANE_TURNS=2 LANE_ENV="CEDIAN_P2_RECORD=$1" ;;
  workflow|review|turn_lifecycle) LANE_TURNS=1 ;;
  inline_edit) LANE_TURNS=2 ;;
  # S1-dap (ADR-0070): OMP calls cedian_debug on the real lldb-dap; not in
  # LANES, it is recorded on its own. Needs `DevToolsSecurity -status` enabled.
  debug_live) LANE_TURNS=1 LANE_ENV="CEDIAN_U12_RECORD=debug_live" LANE_TEST="-p cedian_debug_live --test live_turn" ;;
  *) echo "unknown lane $1" >&2; exit 2 ;;
  esac
  # A turn of these lanes may spawn a reviewer: one more model run.
  case $1 in shell|s3review|s3same|review) LANE_REVIEWER=1 ;; *) LANE_REVIEWER=0 ;; esac
  case $1 in
  workflow|review|turn_lifecycle|inline_edit)
    LANE_ENV="CEDIAN_U12_RECORD=$1" LANE_TEST="-p cedian_panel --test $1" ;;
  esac
}
# Frame rewrites first (edit_frames.py, the U8-U10 cuts), then the tool-list-only
# edits; s2profile last. The lanes plus both arms are 38 turns; the live pass
# (ADR-0060) caps at 46, and the smoke plus the two re-recorded lanes have
# used 10, so the last lane is the first cut if a step has to fail.
LANES="cli shell s2 s3review s3same workflow review inline_edit turn_lifecycle channel worktree p8 p8deny s2profile"

record() {
  local name=$1 out printed used reserved tap omp lane_status
  [ "${BENCH_GO:-}" = 1 ] || { echo "record needs the owner's go: BENCH_GO=1 (ADR-0058 decision 5)" >&2; exit 2; }
  lane "$name"
  preflight 9
  reserved=$(admit "record-$name" "$LANE_TURNS" "$LANE_REVIEWER") || stop_pass $?
  tap=$PASS_DIR/taps/record/$name
  rm -rf "$tap"
  omp=$(wrapper on "$tap")
  out=$WORK/logs/record-$name.log
  # shellcheck disable=SC2086
  (cd "$ROOT" && env $LANE_ENV CEDIAN_RECORD_OMP="$omp" CEDIAN_RECORD_OMP_REAL="$OMP" CEDIAN_RECORD_MODEL="$BENCH_MODEL" \
     CARGO_PROFILE_DEV_DEBUG=line-tables-only cargo test -j4 $LANE_TEST) 2>&1 |
    tee "$out"
  # The lane's own tests decide the step: a recording whose test fails is
  # booked (its turns are spent) and then fails the pass, so a fixture that
  # does not replay is never reported as recorded.
  lane_status=("${PIPESTATUS[@]}")
  printed=$(sed -n 's/.*model turns: \([0-9][0-9]*\).*/\1/p' "$out" | awk '{ n += $1 } END { print n + 0 }')
  used=$(tapped "$tap" "$reserved")
  [ "$printed" -le "$used" ] || used=$printed
  book "record-$name" "$reserved" "$used"
  [ "${lane_status[0]}" = 0 ] || {
    echo "record-$name: the lane's tests failed; its fixture is not a recording to keep (see $out)" >&2
    exit 1
  }
}

plan() {
  local total=0 l m id
  echo "# the live pass, in order (MAX_TURNS=$MAX_TURNS); a step starts only if its estimate + 1 (+ 1 with a reviewer) fits, and books what its OMP processes ran"
  echo "# first, no turns: BENCH_PASS=u12 script/cedian-bench/run.sh model all   # the pin, and the pass's model in OMP's catalog"
  for l in $LANES; do
    lane "$l"; total=$((total + LANE_TURNS))
    echo "BENCH_GO=1 BENCH_PASS=u12 script/cedian-bench/run.sh record $l   # $LANE_TURNS turn(s), total $total"
  done
  for m in off on; do
    for id in $ALL; do
      total=$((total + 1))
      echo "BENCH_GO=1 BENCH_PASS=u12 MANDATE=$m script/cedian-bench/run.sh run $id   # 1 turn, total $total"
    done
  done
  [ "$total" -le "$MAX_TURNS" ] || echo "# over the cap by $((total - MAX_TURNS)): the pass stops before the last $((total - MAX_TURNS)) step(s)"
}

turns() {
  echo "ledger: $LEDGER"
  if [ -f "$LEDGER" ]; then echo "$($CAP used "$LEDGER") of $MAX_TURNS model turns used"; cat "$LEDGER"; else echo "0 of $MAX_TURNS model turns used"; fi
}

# This pass's results beside S2's, task by task (ADR-0037: a task more than
# 25% slower on the same model is a regression; overhead = total - omp).
compare() {
  python3 - "$PASS_DIR/results/$1" "$S2_RESULTS" "$BENCH_MODEL" <<'PY'
import json, os, statistics, sys
new_dir, s2_dir, model = sys.argv[1:4]
def load(d, id):
    p = f"{d}/{id}.json"
    return json.load(open(p)) if os.path.exists(p) else None
print("task  s2       s2_wall_s  verdict  wall_s  ratio  regression  false_done  overhead_ms")
over = []
for id in [f"b{i}" for i in range(1, 11)]:
    s2, new = load(s2_dir, id), load(new_dir, id)
    if not new:
        print(f"{id:5} not run"); continue
    if new.get("observed_models") != [model]:
        print(f"{id:5} refused: OMP ran {new.get('observed_models')}, not {model}"); continue
    o = [t["total_ms"] - t["omp_ms"] for t in new["timing"] if t.get("event") == "turn"]
    over += o
    both = s2 and s2["predicate"] == "pass" and new["predicate"] == "pass"
    ratio = new["wall_ms"] / s2["wall_ms"] if both and s2["wall_ms"] else None
    print(f"{id:5} {s2['predicate'] if s2 else '-':8} {s2['wall_ms'] / 1000 if s2 else 0:9.0f}  "
          f"{new['predicate']:7} {new['wall_ms'] / 1000:6.0f}  {f'{ratio:.2f}' if ratio else '-':5}  "
          f"{str(bool(ratio and ratio > 1.25)):10}  {str(new['false_done']):10}  {o}")
if over:
    print(f"overhead per turn: median {statistics.median(over)} ms, max {max(over)} ms, {len(over)} turns")
PY
}

# ADR-0058 decision 5: hashes of the person's OMP config before and after the
# pass, and the files the pass left in ~/.omp/run and ~/.omp/logs (listed,
# never deleted). Reads ~/.omp only.
snapshot() {
  local omp_home=$HOME/.omp dir=$WORK/omp-snapshot
  mkdir -p "$dir"
  {
    for f in agent/config.yml agent/APPEND_SYSTEM.md; do
      if [ -f "$omp_home/$f" ]; then shasum -a 256 "$omp_home/$f"; else echo "absent $f"; fi
    done
    if [ -d "$omp_home/agent/skills" ]; then
      (cd "$omp_home/agent/skills" && find . -type f -print0 | sort -z | xargs -0 shasum -a 256 | shasum -a 256 | sed 's|-$|agent/skills/|')
    fi
  } > "$dir/$1.hashes"
  (cd "$omp_home" && find run logs -type f 2>/dev/null | sort) > "$dir/$1.files"
  if [ "$1" = after ]; then
    if diff "$dir/before.hashes" "$dir/after.hashes"; then echo "OMP config, mandate and skills unchanged"; else echo "CHANGED (above)"; fi
    echo "files the pass left in ~/.omp/run and ~/.omp/logs:"
    comm -13 "$dir/before.files" "$dir/after.files"
  fi
}

mode=${1:-}; target=${2:-}
[ -n "$mode" ] && [ -n "$target" ] || { sed -n '2,34p' "$0"; exit 2; }
case $mode in run|record|turns|compare|wrapper) need_pass ;; esac
mkdir -p "$WORK/logs" "$WORK/trees"
case $mode in
check) ids=$([ "$target" = all ] && echo "$ALL" || echo "$target"); preflight 4 ;;
run)
  ids=$([ "$target" = all ] && echo "$ALL" || echo "$target")
  pin_check; model_check
  (cd "$ROOT" && cargo build -q --release -p cedian_cli)
  [ -x "$CARGO_TARGET_DIR/release/cedian" ] || { echo "no cedian binary at $CARGO_TARGET_DIR/release" >&2; exit 2; } ;;
record) ids=$([ "$target" = all ] && echo "$LANES" || echo "$target"); pin_check; model_check ;;
model) pin_check; model_check; echo "the pass runs OMP $PIN on $BENCH_MODEL"; exit 0 ;;
plan|turns) "$mode"; exit 0 ;;
wrapper) wrapper "$target" "${BENCH_TAP:?BENCH_TAP: the dir the wrapper taps into}"; exit 0 ;;
compare) compare "$target"; exit 0 ;;
snapshot) snapshot "$target"; exit 0 ;;
*) sed -n '2,34p' "$0"; exit 2 ;;
esac
status=0
for id in $ids; do "$mode" "$id" || status=1; done
exit $status
