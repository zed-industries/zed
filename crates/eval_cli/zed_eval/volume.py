from __future__ import annotations

import argparse
import subprocess
import sys
import tarfile
import time
from pathlib import Path

from . import run_index, source
from .common import (
    default_namespace,
    deployed_function,
    print_json,
    print_table,
    resolve_run_location,
    run_command,
    safe_extract_archive,
)

RECENT_RUN_COLUMNS = [
    ("created_at", "created"),
    ("run_id", "run"),
    ("experiment_name", "experiment"),
    ("namespace", "namespace"),
    ("agent_model", "model"),
    ("judge_preset", "judge"),
    ("build_id", "build"),
    ("suite_id", "suite"),
]
REMOTE_RUN_COLUMNS = [
    ("updated_at", "updated"),
    ("namespace", "namespace"),
    ("experiment_name", "experiment"),
    ("run_id", "run"),
    ("status", "status"),
    ("agent_model", "model"),
    ("judge_preset", "judge"),
    ("build_id", "build"),
    ("suite_id", "suite"),
]
BUILD_COLUMNS = [
    ("built_at_utc", "built"),
    ("build_id", "build"),
    ("ready", "ready"),
    ("base_sha", "base"),
    ("patch_sha256", "patch"),
]
SUITE_COLUMNS = [
    ("part", "part"),
    ("experiment_name", "experiment"),
    ("run_id", "run"),
    ("status", "status"),
    ("updated_at", "updated"),
    ("trial_count", "trials"),
]


def volume_get(
    args: argparse.Namespace, remote_path: str, local_path: str
) -> subprocess.CompletedProcess[str]:
    return run_command(
        ["modal", "volume", "get", args.volume, remote_path, local_path],
        capture=local_path == "-",
    )


def read_volume_text(args: argparse.Namespace, remote_path: str) -> str:
    result = volume_get(args, remote_path, "-")
    return result.stdout


def build_ready_on_volume(args: argparse.Namespace, build_id: str) -> bool:
    try:
        read_volume_text(args, f"builds/{build_id}/READY")
        return True
    except (subprocess.CalledProcessError, OSError):
        return False


def ensure_volume_exists(args: argparse.Namespace) -> int:
    print(f"\nEnsuring Modal volume '{args.volume}' exists...")
    try:
        result = subprocess.run(
            ["modal", "volume", "create", args.volume],
            text=True,
            capture_output=True,
            check=True,
        )
        if result.stdout:
            print(result.stdout, end="")
    except subprocess.CalledProcessError as error:
        output = (error.stdout or "") + (error.stderr or "")
        if "already exists" in output.lower():
            print(f"Volume '{args.volume}' already exists; continuing.")
        else:
            if output:
                print(output, end="", file=sys.stderr)
            return error.returncode
    return 0


def print_rows(
    rows: list[dict], columns: list[tuple[str, str]], *, as_json: bool = False
) -> None:
    if as_json:
        print_json(rows)
    else:
        print_table(rows, columns)


def run_remote_prefix(args: argparse.Namespace, run_id: str) -> str:
    namespace, experiment_name = resolve_run_location(args, run_id)
    return f"runs/{namespace}/{experiment_name}/{run_id}"


def read_run_file(args: argparse.Namespace, file_name: str) -> str:
    run_id = resolve_run_id(args)
    return read_volume_text(args, f"{run_remote_prefix(args, run_id)}/{file_name}")


def suite_rows(args: argparse.Namespace) -> list[dict]:
    namespace = default_namespace(args)
    return deployed_function(args, "suite_status").remote(namespace, args.suite_id)


def suite_member_args(args: argparse.Namespace, row: dict) -> argparse.Namespace:
    run_args = argparse.Namespace(**vars(args))
    run_args.experiment_name = row["experiment_name"]
    run_args.run_id = row["run_id"]
    return run_args


def resolve_run_id(args: argparse.Namespace) -> str:
    """Return the run id to act on, defaulting to the most recent local run.

    Commands like `status`/`logs` accept no run id at all, in which case we use
    the most recently launched run recorded in the local index so a bare
    `zed-eval status` answers "how's my latest run doing?".
    """
    run_id = getattr(args, "run_id", None)
    if run_id:
        return run_id
    entry = run_index.most_recent()
    if not entry:
        raise ValueError(
            "no run id given and the local run index is empty. Launch a run "
            "first, or pass a run id explicitly."
        )
    args.run_id = entry["run_id"]
    print(
        f"(no run id given; using most recent: {entry['run_id']} / "
        f"{entry.get('experiment_name')})",
        file=sys.stderr,
    )
    return entry["run_id"]


def command_runs(args: argparse.Namespace) -> int:
    entries = run_index.recent(args.limit)
    if getattr(args, "json", False):
        print_json(entries)
        return 0
    if not entries:
        print(
            "No runs recorded locally yet. Launch a run, or use "
            "`zed-eval list --details` to query runs on the volume."
        )
        return 0
    print_table(entries, RECENT_RUN_COLUMNS)
    return 0


def command_list(args: argparse.Namespace) -> int:
    namespace = None if args.all_namespaces else default_namespace(args)
    experiment_name = getattr(args, "experiment_name", None)
    if experiment_name:
        experiment_name = source.sanitize_namespace(experiment_name)
    if args.details or args.json:
        rows = deployed_function(args, "list_runs").remote(
            namespace, experiment_name, args.limit
        )
        print_rows(rows, REMOTE_RUN_COLUMNS, as_json=args.json)
        return 0
    remote_path = (
        f"runs/{namespace}/{experiment_name}/"
        if experiment_name
        else f"runs/{namespace}/"
    )
    run_command(["modal", "volume", "ls", args.volume, remote_path])
    return 0


def command_builds(args: argparse.Namespace) -> int:
    if args.details or args.json:
        rows = deployed_function(args, "list_builds").remote(args.limit)
        print_rows(rows, BUILD_COLUMNS, as_json=args.json)
        return 0
    run_command(["modal", "volume", "ls", args.volume, "builds/"])
    return 0


def command_status(args: argparse.Namespace) -> int:
    # One-shot status ping: the controller's state.json only carries a coarse
    # status (pending -> running -> completed/failed), so there is no per-trial
    # progress worth following. Print it once and return.
    print(read_run_file(args, "state.json"), end="")
    return 0


def command_logs(args: argparse.Namespace) -> int:
    print(read_run_file(args, "controller.log"), end="")
    return 0


def command_fetch(args: argparse.Namespace) -> int:
    jobs_dir = Path(args.jobs_dir).expanduser()
    jobs_dir.mkdir(parents=True, exist_ok=True)
    temporary_archive = jobs_dir / f".{args.run_id}.tar.gz"
    remote_path = f"{run_remote_prefix(args, args.run_id)}/harbor-job.tar.gz"
    print(f"Fetching {args.volume}:/{remote_path} -> {temporary_archive}")
    volume_get(args, remote_path, str(temporary_archive))
    with tarfile.open(temporary_archive, "r:gz") as archive:
        safe_extract_archive(archive, jobs_dir)
    temporary_archive.unlink(missing_ok=True)
    print(f"Extracted Harbor job under {jobs_dir / args.run_id}")
    return 0


def command_suite_status(args: argparse.Namespace) -> int:
    print_rows(suite_rows(args), SUITE_COLUMNS, as_json=args.json)
    return 0


def command_suite_logs(args: argparse.Namespace) -> int:
    rows = suite_rows(args)
    while True:
        for row in rows:
            print(
                f"\n=== {row['part']} / {row['experiment_name']} / {row['run_id']} ==="
            )
            run_args = suite_member_args(args, row)
            run_args.follow = False
            command_logs(run_args)
        if not args.follow:
            return 0
        time.sleep(args.interval)


def command_suite_fetch(args: argparse.Namespace) -> int:
    result = 0
    for row in suite_rows(args):
        result = command_fetch(suite_member_args(args, row)) or result
    return result
