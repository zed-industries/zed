"""Baseline-of-record management.

A "baseline" is a completed run promoted to the canonical reference for its
`(experiment, model)` pair. It only qualifies if its build came from a *clean*
commit reachable from `origin/main` (no local patch) — so baselines are both
obviously discoverable (the `baselines/` tree + `zed-eval baseline list`) and
provably pinned to a commit on main. The judge preset is recorded but not gated,
so non-leaderboard baselines are allowed.

Records live on the shared volume at:

    baselines/index.json                                  # roll-up for discovery
    baselines/<experiment>/<model-slug>/current.json      # canonical pointer
    baselines/<experiment>/<model-slug>/history/<sha>.json # superseded baselines
"""

from __future__ import annotations

import argparse
import json
from typing import Any

from . import config, source
from .common import default_namespace, deployed_function, utc_now


def _model_slug(model: str) -> str:
    return source.sanitize_namespace(model)


def _record_from_provenance(
    *,
    namespace: str,
    fallback_experiment: str,
    run_id: str,
    provenance: dict[str, Any],
) -> tuple[dict[str, Any], bool]:
    request = provenance["request"]
    summary = provenance["summary"]
    build_info = provenance["build_info"]
    source_info = build_info.get("source") or {}

    experiment = request.get("experiment_name") or fallback_experiment
    model = request.get("agent_model") or config.DEFAULT_MODEL
    base_sha = build_info.get("base_sha")

    record = {
        "experiment": experiment,
        "model": model,
        "model_slug": _model_slug(model),
        "judge": request.get("judge_preset") or request.get("judge"),
        "base_sha": base_sha,
        "base_ref": source_info.get("base_ref"),
        "on_main": None,
        "clean": not bool(build_info.get("patch_sha256")),
        "build_id": build_info.get("build_id") or request.get("build_id"),
        "run": {
            "namespace": namespace,
            "experiment_name": experiment,
            "run_id": run_id,
        },
        "resources": {
            "override_cpus": request.get("override_cpus"),
            "override_memory_mb": request.get("override_memory_mb"),
        },
        "summary": {
            "status": summary.get("status"),
            "trial_count": summary.get("trial_count"),
        },
        "recorded_at": utc_now(),
        "recorded_by": namespace,
    }
    return record, bool(build_info)


def _baseline_problems(
    *,
    record: dict[str, Any],
    has_build_info: bool,
    repo_url: str,
    allow_dirty: bool,
    allow_off_main: bool,
) -> tuple[list[str], bool | None]:
    problems: list[str] = []
    run_id = (record.get("run") or {}).get("run_id")
    status = (record.get("summary") or {}).get("status")
    base_sha = record.get("base_sha")

    if not has_build_info:
        problems.append(
            f"no build-info found for run {run_id}; cannot verify provenance"
        )
    if status != "completed":
        problems.append(f"run status is {status!r}, expected 'completed'")
    if not record.get("clean") and not allow_dirty:
        problems.append(
            "build carries a local patch (not a clean commit); pass --allow-dirty to override"
        )

    on_main: bool | None = None
    if base_sha:
        try:
            on_main = source.base_sha_on_main(base_sha, repo_url)
        except Exception as error:  # noqa: BLE001 - surface any verification failure
            if not allow_off_main:
                problems.append(f"could not verify base_sha is on origin/main: {error}")
        if on_main is False and not allow_off_main:
            problems.append(
                f"base_sha {base_sha[:12]} is not reachable from origin/main; "
                "pass --allow-off-main to override"
            )
    elif not allow_off_main:
        problems.append("build-info has no base_sha to verify against origin/main")

    return problems, on_main


def _print_refusal(run_id: str, problems: list[str]) -> None:
    print(f"refusing to record baseline for {run_id}:")
    for problem in problems:
        print(f"  - {problem}")


def _print_recorded(record: dict[str, Any]) -> None:
    print(
        f"recorded baseline: {record.get('experiment')} / {record.get('model')}  "
        f"base_sha={str(record.get('base_sha'))[:12]} "
        f"on_main={record.get('on_main')} clean={record.get('clean')} "
        f"judge={record.get('judge')}"
    )


def command_baseline_record(args: argparse.Namespace) -> int:
    namespace = default_namespace(args)
    experiment_slug = source.sanitize_namespace(args.experiment_name)
    repo_url = getattr(args, "repo_url", None) or source.DEFAULT_REPO_URL
    read_run_provenance = deployed_function(args, "read_run_provenance")
    record_baseline = deployed_function(args, "record_baseline")

    exit_code = 0
    for run_id in args.run_id:
        provenance = read_run_provenance.remote(namespace, experiment_slug, run_id)
        record, has_build_info = _record_from_provenance(
            namespace=namespace,
            fallback_experiment=args.experiment_name,
            run_id=run_id,
            provenance=provenance,
        )
        problems, on_main = _baseline_problems(
            record=record,
            has_build_info=has_build_info,
            repo_url=repo_url,
            allow_dirty=args.allow_dirty,
            allow_off_main=args.allow_off_main,
        )
        record["on_main"] = on_main

        if problems:
            _print_refusal(run_id, problems)
            exit_code = 1
            continue

        record_baseline.remote(record)
        _print_recorded(record)
    return exit_code


def command_baseline_list(args: argparse.Namespace) -> int:
    index = deployed_function(args, "read_baselines").remote(None, None, False)
    rows = index.get("baselines") or []
    if getattr(args, "json", False):
        print(json.dumps(index, indent=2))
        return 0
    if not rows:
        print("no baselines recorded")
        return 0
    header = (
        f"{'EXPERIMENT':<20} {'MODEL':<32} {'BASE_REF':<14} "
        f"{'SHA':<12} {'ON_MAIN':<8} {'JUDGE':<14} RUN"
    )
    print(header)
    for row in rows:
        print(
            f"{(row.get('experiment') or ''):<20} "
            f"{(row.get('model') or ''):<32} "
            f"{(row.get('base_ref') or '') or '-':<14} "
            f"{str(row.get('base_sha') or '')[:12]:<12} "
            f"{str(row.get('on_main')):<8} "
            f"{(row.get('judge') or '') or '-':<14} "
            f"{row.get('run_id') or ''}"
        )
    return 0


def command_baseline_show(args: argparse.Namespace) -> int:
    result = deployed_function(args, "read_baselines").remote(
        args.experiment_name,
        _model_slug(args.model),
        bool(getattr(args, "history", False)),
    )
    if not result or result.get("current") is None:
        print(f"no baseline recorded for {args.experiment_name} / {args.model}")
        return 1
    print(json.dumps(result, indent=2))
    return 0
