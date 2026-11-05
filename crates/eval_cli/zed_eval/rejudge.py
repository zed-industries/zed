"""Re-grade a finished run with a different judge, without redoing agent work.

A rejudge takes the agent output already stored in a parent run (the expensive
part: patches/answers the agent produced) and re-runs only the *judge* step with
a new judge model, producing a brand-new derived run. The original run is read
only and never modified.

This is deliberately not a re-implementation of the grader. It runs the exact
SWE-Atlas verifier script shipped in each task package (`evaluate_rubrics.py` for
RF, `evaluate_answer.py` for QnA) through the same judge proxy shim the live
in-sandbox path uses (`judge_proxy.JUDGE_PROXY_SCRIPT`), so a rejudged verdict is
produced identically to a first-pass verdict — only the judge model differs.

The verifier scripts read their inputs from fixed absolute paths (`/tests`,
`/logs/...`) and pick the judge up from `EVAL_MODEL`/`EVAL_BASE_URL`/
`EVAL_API_KEY`, so this module is meant to run inside a throwaway container (the
Modal `rejudge_controller`), where staging those absolute paths is safe. The pure
helpers (resolution, reward recombination, result patching) have no such
requirement and are unit-tested directly.
"""

from __future__ import annotations

import json
import os
import shutil
import socket
import subprocess
import sys
import time
from pathlib import Path
from typing import Any

from . import config
from .judge_proxy import JUDGE_PROXY_SCRIPT

# Each task package ships exactly one of these verifier scripts; its presence is
# how we tell an RF task from a QnA one without trusting the experiment config.
RF_VERIFIER = "evaluate_rubrics.py"
QNA_VERIFIER = "evaluate_answer.py"

PART_RF = "rf"
PART_QNA = "qna"

# Absolute paths the cached verifier scripts hardcode. Staged per trial.
TESTS_DIR = Path("/tests")
LOGS_AGENT_DIR = Path("/logs/agent")
LOGS_VERIFIER_DIR = Path("/logs/verifier")
RF_PATCH_PATH = LOGS_VERIFIER_DIR / "agent.patch"
RF_RESULTS_PATH = LOGS_VERIFIER_DIR / "rubrics_results.json"
QNA_ANSWER_PATH = LOGS_AGENT_DIR / "answer.txt"
QNA_RESULTS_PATH = LOGS_VERIFIER_DIR / "evaluation_results.json"


def load_json(path: Path) -> Any:
    try:
        return json.loads(Path(path).read_text())
    except (OSError, json.JSONDecodeError):
        return None


def task_tests_dir(task_name: str, task_ref: str, tasks_root: Path) -> Path:
    """Resolve a trial's cached task package `tests/` dir from its name + ref.

    Mirrors Harbor's content-addressed layout:
    `<tasks_root>/<org>/<task-id>/<ref-hash>/tests`.
    """
    org, task_id = task_name.split("/", 1)
    ref_hash = task_ref.split(":", 1)[1] if ":" in task_ref else task_ref
    return tasks_root / org / task_id / ref_hash / "tests"


def tests_dir_for_trial(trial_dir: Path, tasks_root: Path) -> Path | None:
    config_data = load_json(trial_dir / "config.json")
    if not isinstance(config_data, dict):
        return None
    task = config_data.get("task") or {}
    name, ref = task.get("name"), task.get("ref")
    if not (name and ref):
        return None
    tests_dir = task_tests_dir(name, ref, tasks_root)
    return tests_dir if tests_dir.is_dir() else None


def detect_part(tests_dir: Path) -> str | None:
    if (tests_dir / RF_VERIFIER).exists():
        return PART_RF
    if (tests_dir / QNA_VERIFIER).exists():
        return PART_QNA
    return None


def recompute_rf_rewards(
    rubrics_results: dict[str, Any], prior_rewards: dict[str, Any]
) -> dict[str, float]:
    """Combine the freshly re-judged rubric verdict with the *unchanged* saved
    deterministic test reward. The agent patch didn't change, so `tests_reward`
    is invariant under rejudge; only the rubric (`must_have_pass`/agg) moves.

    The overall RF reward is `must_have_pass AND tests_reward == 1`, verified to
    hold across every cached RF trial.
    """
    must_have_pass = bool(rubrics_results.get("must_have_pass"))
    agg_score = rubrics_results.get("agg_score")
    tests_reward = float(prior_rewards.get("tests_reward") or 0.0)
    overall_pass = 1.0 if (must_have_pass and tests_reward >= 1.0) else 0.0
    return {
        "reward": overall_pass,
        "overall_pass": overall_pass,
        "tests_reward": tests_reward,
        "must_have_pass": 1.0 if must_have_pass else 0.0,
        "rubrics_agg_score": float(agg_score) if agg_score is not None else 0.0,
    }


def recompute_qna_rewards(evaluation_results: dict[str, Any]) -> dict[str, float]:
    """QnA has no deterministic test component; the reward is just the judged
    pass verdict, matching the single-key `{"reward": ...}` production shape."""
    return {"reward": 1.0 if bool(evaluation_results.get("pass")) else 0.0}


def proxy_environment(judge: config.JudgeConfig) -> dict[str, str]:
    """Env the judge proxy subprocess reads. The auth key it forwards
    (`judge.auth_env`, e.g. BASETEN_API_KEY) must already be present in the
    process environment via the mounted LLM-providers secret."""
    env = {
        "ZED_JUDGE_UPSTREAM": judge.upstream,
        "ZED_JUDGE_AUTH_ENV": judge.auth_env,
    }
    if judge.max_tokens is not None:
        env["ZED_JUDGE_MAX_TOKENS"] = str(judge.max_tokens)
    return env


def verifier_environment(judge_model: str, port: int) -> dict[str, str]:
    """Env the cached verifier reads to find the judge.

    This mirrors the verifier env injected by the benchmark harness command,
    pointed at the local rejudge proxy.
    """
    return {
        "EVAL_MODEL": judge_model,
        "EVAL_BASE_URL": f"http://127.0.0.1:{port}/v1",
        "EVAL_API_KEY": "unused-by-agent-evals-proxy",
    }


class JudgeProxy:
    """Runs `judge_proxy.JUDGE_PROXY_SCRIPT` as a local subprocess for the life
    of a rejudge job. One proxy serves every trial (they all target the same
    judge), mirroring the single in-sandbox daemon."""

    def __init__(self, judge: config.JudgeConfig, port: int = 8089) -> None:
        self._judge = judge
        self._port = port
        self._process: subprocess.Popen[bytes] | None = None
        self._script_path = Path("/tmp/zed_judge_proxy.py")

    @property
    def port(self) -> int:
        return self._port

    def __enter__(self) -> JudgeProxy:
        self._script_path.write_text(JUDGE_PROXY_SCRIPT)
        env = os.environ.copy()
        env.update(proxy_environment(self._judge))
        env["ZED_JUDGE_PROXY_PORT"] = str(self._port)
        self._process = subprocess.Popen(
            [sys.executable, str(self._script_path)],
            env=env,
            stdout=subprocess.DEVNULL,
            stderr=subprocess.DEVNULL,
        )
        self._wait_for_port()
        return self

    def __exit__(self, *_exc: object) -> None:
        if self._process is not None:
            self._process.terminate()
            try:
                self._process.wait(timeout=10)
            except subprocess.TimeoutExpired:
                self._process.kill()

    def _wait_for_port(self, timeout_secs: float = 30.0) -> None:
        deadline = time.time() + timeout_secs
        while time.time() < deadline:
            if self._process is not None and self._process.poll() is not None:
                raise RuntimeError("judge proxy exited before becoming ready")
            with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as probe:
                probe.settimeout(0.5)
                if probe.connect_ex(("127.0.0.1", self._port)) == 0:
                    return
            time.sleep(0.25)
        raise TimeoutError(f"judge proxy did not open port {self._port} in time")


def _stage_tests_dir(tests_dir: Path) -> None:
    """Point the absolute `/tests` path at this trial's package. Reset per trial
    so each verifier reads its own rubrics/prompts."""
    if TESTS_DIR.is_symlink() or TESTS_DIR.exists():
        if TESTS_DIR.is_dir() and not TESTS_DIR.is_symlink():
            shutil.rmtree(TESTS_DIR)
        else:
            TESTS_DIR.unlink()
    TESTS_DIR.symlink_to(tests_dir)


def _run_verifier(
    part: str, tests_dir: Path, judge_model: str, port: int
) -> dict[str, Any]:
    """Stage the verifier's expected inputs and run the real cached script."""
    LOGS_AGENT_DIR.mkdir(parents=True, exist_ok=True)
    LOGS_VERIFIER_DIR.mkdir(parents=True, exist_ok=True)
    _stage_tests_dir(tests_dir)

    env = os.environ.copy()
    env.update(verifier_environment(judge_model, port))
    script = tests_dir / (RF_VERIFIER if part == PART_RF else QNA_VERIFIER)
    results_path = RF_RESULTS_PATH if part == PART_RF else QNA_RESULTS_PATH
    if results_path.exists():
        results_path.unlink()

    completed = subprocess.run(
        [sys.executable, str(script)],
        env=env,
        capture_output=True,
        text=True,
    )
    results = load_json(results_path)
    if not isinstance(results, dict):
        raise RuntimeError(
            f"verifier produced no results (exit {completed.returncode}): "
            f"{completed.stderr.strip()[:500]}"
        )
    return results


def rejudge_trial(
    trial_dir: Path,
    tasks_root: Path,
    judge: config.JudgeConfig,
    judge_model: str,
    port: int,
) -> dict[str, Any]:
    """Re-grade one trial in place (the trial dir is a copy in the derived run).

    Returns a per-trial record; on any recoverable problem it returns
    `{"ok": False, "error": ...}` so one bad trial can't sink the whole job.
    """
    task_name = trial_dir.name
    tests_dir = tests_dir_for_trial(trial_dir, tasks_root)
    if tests_dir is None:
        return {"task": task_name, "ok": False, "error": "task package not found"}
    part = detect_part(tests_dir)
    if part is None:
        return {"task": task_name, "ok": False, "error": "no verifier in task package"}

    # The agent output is already on disk from the parent run; copy it to the
    # path the verifier expects. RF reads the patch, QnA the delivered answer.
    if part == PART_RF:
        source = trial_dir / "verifier" / "agent.patch"
        if not source.exists():
            source = trial_dir / "verifier" / "agent_source_only.patch"
        if not source.exists():
            return {"task": task_name, "ok": False, "error": "no agent.patch"}
        RF_PATCH_PATH.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(source, RF_PATCH_PATH)
    else:
        source = trial_dir / "agent" / "answer.txt"
        if not source.exists():
            return {"task": task_name, "ok": False, "error": "no answer.txt"}
        QNA_ANSWER_PATH.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(source, QNA_ANSWER_PATH)

    try:
        results = _run_verifier(part, tests_dir, judge_model, port)
    except (RuntimeError, OSError) as error:
        return {"task": task_name, "ok": False, "error": str(error)}

    prior_rewards = (
        (load_json(trial_dir / "result.json") or {}).get("verifier_result") or {}
    ).get("rewards") or {}
    if part == PART_RF:
        new_rewards = recompute_rf_rewards(results, prior_rewards)
        verifier_results_name = "rubrics_results.json"
    else:
        new_rewards = recompute_qna_rewards(results)
        verifier_results_name = "evaluation_results.json"

    # Persist the fresh verifier output and verdict into the derived trial dir so
    # report.py reads the new judge's result with no special-casing.
    (trial_dir / "verifier").mkdir(parents=True, exist_ok=True)
    (trial_dir / "verifier" / verifier_results_name).write_text(
        json.dumps(results, indent=2)
    )
    patch_result_rewards(trial_dir / "result.json", new_rewards)
    return {"task": task_name, "ok": True, "part": part, "rewards": new_rewards}


def patch_result_rewards(result_path: Path, new_rewards: dict[str, float]) -> None:
    """Overwrite only `verifier_result.rewards` in a trial's `result.json`,
    leaving the agent's resource metrics (tokens/steps/tool calls) untouched —
    the agent didn't re-run, only the judge did."""
    result = load_json(result_path)
    if not isinstance(result, dict):
        result = {}
    verifier_result = result.get("verifier_result")
    if not isinstance(verifier_result, dict):
        verifier_result = {}
    verifier_result["rewards"] = new_rewards
    result["verifier_result"] = verifier_result
    Path(result_path).write_text(json.dumps(result, indent=2))


def rejudge_job(
    parent_job_dir: Path,
    out_job_dir: Path,
    tasks_root: Path,
    judge: config.JudgeConfig,
    judge_model: str,
    port: int = 8089,
    log: Any = print,
) -> dict[str, Any]:
    """Copy a parent run's Harbor job dir into the derived run and re-judge every
    trial in place. Returns a summary suitable for the run's `summary.json`."""
    if out_job_dir.exists():
        shutil.rmtree(out_job_dir)
    shutil.copytree(parent_job_dir, out_job_dir)

    trial_dirs = sorted(
        path
        for path in out_job_dir.iterdir()
        if path.is_dir() and (path / "result.json").exists()
    )
    records: list[dict[str, Any]] = []
    with JudgeProxy(judge, port=port) as proxy:
        for index, trial_dir in enumerate(trial_dirs, start=1):
            record = rejudge_trial(
                trial_dir, tasks_root, judge, judge_model, proxy.port
            )
            records.append(record)
            status = "ok" if record["ok"] else f"FAILED: {record.get('error')}"
            log(f"[{index}/{len(trial_dirs)}] {trial_dir.name}: {status}")

    rejudged = sum(1 for record in records if record["ok"])
    passed = sum(
        1
        for record in records
        if record["ok"] and record["rewards"].get("reward", 0.0) >= 1.0
    )
    return {
        "trial_count": len(trial_dirs),
        "rejudged_count": rejudged,
        "failed_count": len(trial_dirs) - rejudged,
        "passed_count": passed,
        "judge_model": judge_model,
        "trials": records,
    }
