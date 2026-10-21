"""Success-conditioned metrics for fetched benchmark runs.

This reads a fetched Harbor/Pier job directory and reports, for each run:

  - success rate (with SEM when there are repeated attempts)
  - token consumption *for the passing subset* (and overall, for comparison)
  - tool calls *for the passing subset*, aggregate and broken down by tool
  - total steps (agent turns) *for the passing subset*

The per-trial record has two sources, one canonical each, with no fallbacks:

  - The verdict (pass/fail/errored) comes from the harness' own trial result
    (verifier rewards) — rubric judge for SWE-Atlas, test scripts for
    Terminal-Bench and DeepSWE.
  - Resource usage (tokens / steps / tool calls) comes from the Zed agent's
    `result.json`, which eval-cli emits identically on every harness. There is
    deliberately no secondary parser: if a metric isn't in `result.json`, it's
    reported as absent rather than reconstructed from another file.
"""

from __future__ import annotations

import json
import math
import statistics
import sys
from collections import Counter, defaultdict
from dataclasses import dataclass, field
from pathlib import Path
from typing import Any

# Effective-token weights (cache reads are ~10x cheaper, output ~4x input).
ET_INPUT = 1.0
ET_CACHE_READ = 0.1
ET_CACHE_CREATION = 1.0
ET_OUTPUT = 4.0

HARBOR_TRIAL_KEYS = {"task_name", "trial_name", "verifier_result"}
ZED_RESULT_KEYS = {"status", "duration_secs"}


def load_json(path: Path) -> Any:
    try:
        return json.loads(path.read_text())
    except (OSError, json.JSONDecodeError) as error:
        print(f"warning: could not parse {path}: {error}", file=sys.stderr)
        return None


def find_trial_dirs(job_dir: Path) -> list[Path]:
    trial_dirs: set[Path] = set()
    for name in ("result.json", "results.json"):
        for path in job_dir.rglob(name):
            if path.parent == job_dir:
                continue
            data = load_json(path)
            if isinstance(data, dict) and HARBOR_TRIAL_KEYS & data.keys():
                trial_dirs.add(path.parent)
    return sorted(trial_dirs)


def load_harbor_trial_result(trial_dir: Path) -> dict | None:
    for name in ("result.json", "results.json"):
        path = trial_dir / name
        if path.exists():
            data = load_json(path)
            if isinstance(data, dict) and HARBOR_TRIAL_KEYS & data.keys():
                return data
    return None


def load_zed_result(trial_dir: Path) -> dict | None:
    for path in sorted(trial_dir.rglob("result.json")):
        if path.parent == trial_dir:
            continue
        data = load_json(path)
        if isinstance(data, dict) and ZED_RESULT_KEYS & data.keys():
            return data
    return None


def trial_verdict(
    harbor_result: dict, *, timeout_is_failure: bool = False
) -> tuple[bool | None, str | None]:
    """Returns (passed, error_reason). `passed` is None when the trial errored
    (harness/infra exception or no verifier reward); infra errors are not
    failures.

    `timeout_is_failure` reclassifies an agent timeout as a real failure
    (passed=False) rather than an excluded error: on test-scored benchmarks an
    `AgentTimeoutError` means the agent didn't solve the task within its intended
    time budget, which is an agent failure, not infra noise."""
    exception_info = harbor_result.get("exception_info")
    if exception_info:
        reason = "exception"
        if isinstance(exception_info, dict):
            reason = exception_info.get("exception_type") or "exception"
        if timeout_is_failure and reason == "AgentTimeoutError":
            return False, reason
        return None, reason

    verifier = harbor_result.get("verifier_result")
    if not isinstance(verifier, dict):
        return None, "no verifier result"
    rewards = verifier.get("rewards")
    if not isinstance(rewards, dict) or not rewards:
        return None, "no rewards"
    try:
        # Prefer the strict leaderboard metric. max(rewards) would wrongly count
        # a trial as passed when only a component reward (e.g. tests_reward) hit
        # 1.0 on SWE-Atlas RF.
        if "reward" in rewards:
            return float(rewards["reward"]) >= 1.0, None
        return max(float(value) for value in rewards.values()) >= 1.0, None
    except (TypeError, ValueError):
        return None, "unparsable rewards"


@dataclass
class TrialRecord:
    task_name: str
    started_at: str | None
    passed: bool | None
    error_reason: str | None
    input_tokens: int = 0
    output_tokens: int = 0
    cache_read_input_tokens: int = 0
    cache_creation_input_tokens: int = 0
    step_count: int | None = None
    total_tool_calls: int | None = None
    tool_calls: dict[str, int] = field(default_factory=dict)
    duration_secs: float | None = None
    status: str | None = None
    model: str | None = None
    # True when the agent's result.json was found (i.e. the trial produced
    # metrics). False for trials hard-killed on timeout before writing it.
    has_result: bool = False

    @property
    def effective_tokens(self) -> float:
        return (
            ET_INPUT * self.input_tokens
            + ET_CACHE_READ * self.cache_read_input_tokens
            + ET_CACHE_CREATION * self.cache_creation_input_tokens
            + ET_OUTPUT * self.output_tokens
        )

    @property
    def total_tokens(self) -> int:
        return (
            self.input_tokens
            + self.output_tokens
            + self.cache_read_input_tokens
            + self.cache_creation_input_tokens
        )


def extract_trial(
    trial_dir: Path, *, timeout_is_failure: bool = False
) -> TrialRecord | None:
    harbor_result = load_harbor_trial_result(trial_dir)
    if harbor_result is None:
        return None

    passed, error_reason = trial_verdict(
        harbor_result, timeout_is_failure=timeout_is_failure
    )
    record = TrialRecord(
        task_name=harbor_result.get("task_name") or trial_dir.name,
        started_at=harbor_result.get("started_at"),
        passed=passed,
        error_reason=error_reason,
    )

    zed_result = load_zed_result(trial_dir)
    if zed_result:
        record.has_result = True
        record.status = zed_result.get("status")
        record.model = zed_result.get("model")
        record.input_tokens = int(zed_result.get("input_tokens") or 0)
        record.output_tokens = int(zed_result.get("output_tokens") or 0)
        record.cache_read_input_tokens = int(
            zed_result.get("cache_read_input_tokens") or 0
        )
        record.cache_creation_input_tokens = int(
            zed_result.get("cache_creation_input_tokens") or 0
        )
        duration = zed_result.get("duration_secs")
        if isinstance(duration, (int, float)):
            record.duration_secs = float(duration)
        # The canonical metrics source: counts eval-cli writes into result.json.
        if isinstance(zed_result.get("step_count"), int):
            record.step_count = zed_result["step_count"]
        if isinstance(zed_result.get("tool_call_count"), int):
            record.total_tool_calls = zed_result["tool_call_count"]
        if isinstance(zed_result.get("tool_calls"), dict):
            record.tool_calls = {
                str(name): int(count)
                for name, count in zed_result["tool_calls"].items()
                if isinstance(count, int)
            }

    return record


def sample_sem(values: list[float]) -> float | None:
    n = len(values)
    if n < 2:
        return None
    mean = sum(values) / n
    return math.sqrt(sum((x - mean) ** 2 for x in values) / (n * (n - 1)))


def pass_rate_with_sem(
    scored: list[TrialRecord],
) -> tuple[float | None, float | None, int]:
    """Pass rate grouped by task. With >=2 attempts per task the rate is the
    mean of per-attempt pass rates with sample SEM across attempts; otherwise
    it's the plain rate (SEM undefined)."""
    if not scored:
        return None, None, 0
    by_task: dict[str, list[TrialRecord]] = defaultdict(list)
    for trial in scored:
        by_task[trial.task_name].append(trial)
    for attempts in by_task.values():
        attempts.sort(key=lambda t: t.started_at or "")

    max_attempts = max(len(attempts) for attempts in by_task.values())
    if max_attempts < 2:
        rate = sum(1 for t in scored if t.passed) / len(scored)
        return rate, None, 1

    attempt_rates: list[float] = []
    for i in range(max_attempts):
        attempt_trials = [
            attempts[i] for attempts in by_task.values() if len(attempts) > i
        ]
        if attempt_trials:
            attempt_rates.append(
                sum(1 for t in attempt_trials if t.passed) / len(attempt_trials)
            )
    if not attempt_rates:
        return None, None, max_attempts
    mean = sum(attempt_rates) / len(attempt_rates)
    return mean, sample_sem(attempt_rates), max_attempts


def _mean(values: list[float]) -> float | None:
    return statistics.mean(values) if values else None


def slice_metrics(records: list[TrialRecord]) -> dict[str, Any]:
    """Mean resource usage over a set of trials (used for both the whole run
    and the passing subset)."""
    empty = {
        "n": 0,
        "n_with_metrics": 0,
        "mean_steps": None,
        "mean_total_tokens": None,
        "mean_effective_tokens": None,
        "mean_tool_calls": None,
        "mean_tool_calls_by_tool": {},
        "mean_duration_secs": None,
    }
    if not records:
        return empty
    # Resource means are taken only over trials that actually produced metrics
    # (result.json present). Trials hard-killed on timeout count toward the pass
    # rate but must not drag token/step means toward zero.
    with_result = [r for r in records if r.has_result]
    steps = [float(r.step_count) for r in records if r.step_count is not None]
    tool_totals = [
        float(r.total_tool_calls) for r in records if r.total_tool_calls is not None
    ]
    per_tool_sum: Counter[str] = Counter()
    for record in with_result:
        per_tool_sum.update(record.tool_calls)
    per_tool_n = len(with_result)
    durations = [r.duration_secs for r in records if r.duration_secs is not None]
    return {
        "n": len(records),
        "n_with_metrics": len(with_result),
        "mean_steps": _mean(steps),
        "mean_total_tokens": _mean([float(r.total_tokens) for r in with_result]),
        "mean_effective_tokens": _mean([r.effective_tokens for r in with_result]),
        "mean_tool_calls": _mean(tool_totals),
        "mean_tool_calls_by_tool": (
            {
                name: per_tool_sum[name] / per_tool_n
                for name in sorted(per_tool_sum, key=lambda n: -per_tool_sum[n])
            }
            if per_tool_n
            else {}
        ),
        "mean_duration_secs": _mean(durations),
    }


def build_report(
    job_dir: Path,
    *,
    label: str | None = None,
    timeout_is_failure: bool = False,
) -> dict[str, Any]:
    records = [
        record
        for trial_dir in find_trial_dirs(job_dir)
        if (record := extract_trial(trial_dir, timeout_is_failure=timeout_is_failure))
        is not None
    ]
    scored = [r for r in records if r.passed is not None]
    errored = [r for r in records if r.passed is None]
    passing = [r for r in scored if r.passed]
    pass_rate, pass_sem, n_attempts = pass_rate_with_sem(scored)

    models = Counter(r.model for r in records if r.model)
    statuses = Counter(r.status for r in records if r.status)

    return {
        "label": label or job_dir.name,
        "job_dir": str(job_dir),
        "n_trials": len(records),
        "n_scored": len(scored),
        "n_passed": len(passing),
        "n_failed": len(scored) - len(passing),
        "n_errored": len(errored),
        "n_attempts": n_attempts,
        "pass_rate": pass_rate,
        "pass_sem": pass_sem,
        "resolved_models": dict(models),
        "agent_statuses": dict(statuses),
        # The headline numbers the user asked for: resource usage conditioned on
        # success, with the overall slice alongside for comparison.
        "on_success": slice_metrics(passing),
        "overall": slice_metrics(scored),
        "errored_trials": [
            {"task_name": r.task_name, "reason": r.error_reason} for r in errored
        ],
    }


def _fmt(value: float | None, places: int = 1) -> str:
    return "n/a" if value is None else f"{value:,.{places}f}"


def _fmt_rate(rate: float | None, sem: float | None) -> str:
    if rate is None:
        return "n/a"
    if sem is None:
        return f"{rate * 100:.1f}%"
    return f"{rate * 100:.1f}% ± {sem * 100:.1f}"


def format_report(report: dict[str, Any]) -> str:
    lines: list[str] = []
    lines.append(f"# {report['label']}")
    lines.append("")
    lines.append(
        f"- Success rate: {_fmt_rate(report['pass_rate'], report['pass_sem'])} "
        f"({report['n_passed']}/{report['n_scored']} scored; "
        f"{report['n_errored']} errored)"
    )
    if report["resolved_models"]:
        models = ", ".join(
            f"{name} x{count}" for name, count in report["resolved_models"].items()
        )
        lines.append(f"- Models: {models}")
    lines.append("")

    on_success = report["on_success"]
    overall = report["overall"]
    lines.append("| Metric (mean) | On success | Overall (scored) |")
    lines.append("| --- | --- | --- |")
    lines.append(
        f"| Steps | {_fmt(on_success['mean_steps'])} | {_fmt(overall['mean_steps'])} |"
    )
    lines.append(
        f"| Total tokens | {_fmt(on_success['mean_total_tokens'], 0)} "
        f"| {_fmt(overall['mean_total_tokens'], 0)} |"
    )
    lines.append(
        f"| Effective tokens | {_fmt(on_success['mean_effective_tokens'], 0)} "
        f"| {_fmt(overall['mean_effective_tokens'], 0)} |"
    )
    lines.append(
        f"| Tool calls | {_fmt(on_success['mean_tool_calls'])} "
        f"| {_fmt(overall['mean_tool_calls'])} |"
    )
    lines.append(
        f"| Duration (s) | {_fmt(on_success['mean_duration_secs'])} "
        f"| {_fmt(overall['mean_duration_secs'])} |"
    )
    lines.append("")

    if on_success["mean_tool_calls_by_tool"]:
        lines.append("Tool calls by tool (mean per passing trial):")
        lines.append("")
        lines.append("| Tool | On success | Overall (scored) |")
        lines.append("| --- | --- | --- |")
        overall_by_tool = overall["mean_tool_calls_by_tool"]
        for name, value in on_success["mean_tool_calls_by_tool"].items():
            lines.append(
                f"| {name} | {_fmt(value, 2)} | {_fmt(overall_by_tool.get(name), 2)} |"
            )
        lines.append("")

    if report["errored_trials"]:
        lines.append(f"Errored trials ({len(report['errored_trials'])}):")
        for entry in report["errored_trials"][:20]:
            lines.append(f"  - {entry['task_name']}: {entry['reason']}")
        if len(report["errored_trials"]) > 20:
            lines.append(f"  ... and {len(report['errored_trials']) - 20} more")
        lines.append("")

    return "\n".join(lines)


def locate_job_dir(args: Any) -> Path:
    """Resolve the fetched Harbor/Pier job directory for a `report` invocation,
    fetching it first when asked."""
    job_dir = getattr(args, "job_dir", None)
    if job_dir:
        return Path(job_dir)

    run_id = getattr(args, "run_id", None)
    if not run_id:
        raise ValueError("provide a run_id or --job-dir")
    jobs_dir = Path(getattr(args, "jobs_dir", Path.home() / ".cache/harbor/jobs"))
    candidate = jobs_dir / run_id

    if getattr(args, "fetch", False) or not candidate.exists():
        # Reuse the existing fetch command to download + extract the archive.
        from .volume import command_fetch

        command_fetch(args)
    if not candidate.exists():
        raise ValueError(
            f"no fetched job at {candidate}. Run `zed-eval fetch {run_id} "
            f"--experiment-name <benchmark>` first, or pass --job-dir."
        )
    return candidate


def _timeout_is_failure(args: Any) -> bool:
    """Agent timeouts count as failures on test-scored benchmarks. Derived from
    the benchmark's scoring (via --experiment-name), with an explicit override."""
    override = getattr(args, "timeouts_as_failures", None)
    if override is not None:
        return bool(override)
    from . import benchmarks

    experiment = getattr(args, "experiment_name", None)
    benchmark = benchmarks.BENCHMARKS.get(experiment) if experiment else None
    return bool(benchmark and benchmark.scoring == benchmarks.SCORING_TESTS)


def _fill_run_location_from_index(args: Any) -> None:
    """Backfill --experiment-name/--namespace from the local run index so a bare
    run id is enough for `report`. Best-effort and only fills what's missing."""
    run_id = getattr(args, "run_id", None)
    if not run_id or getattr(args, "experiment_name", None):
        return
    from . import run_index

    entry = run_index.lookup(run_id)
    if not entry:
        return
    args.experiment_name = entry.get("experiment_name")
    if not getattr(args, "namespace", None):
        args.namespace = entry.get("namespace")


def command_report(args: Any) -> int:
    _fill_run_location_from_index(args)
    job_dir = locate_job_dir(args)
    label = getattr(args, "experiment_name", None) or getattr(args, "run_id", None)
    report = build_report(
        job_dir, label=label, timeout_is_failure=_timeout_is_failure(args)
    )
    if getattr(args, "as_json", False):
        print(json.dumps(report, indent=2))
    else:
        print(format_report(report))
    return 0
