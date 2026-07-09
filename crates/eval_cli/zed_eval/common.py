from __future__ import annotations

import argparse
import datetime
import json
import os
import pathlib
import shutil
import subprocess
import tarfile
from typing import Any

from . import config, source

ALL_SWE_ATLAS_PARTS = ["qna", "rf", "tw"]


def utc_timestamp() -> str:
    return datetime.datetime.now(datetime.timezone.utc).strftime("%Y%m%dT%H%M%SZ")


def utc_now() -> str:
    return datetime.datetime.now(datetime.timezone.utc).isoformat()


def print_json(data: Any) -> None:
    print(json.dumps(data, indent=2, sort_keys=True))


def write_json(path: pathlib.Path, data: dict[str, Any]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(data, indent=2, sort_keys=True) + "\n")


def load_json(path: pathlib.Path) -> dict[str, Any] | None:
    try:
        data = json.loads(path.read_text())
    except (OSError, json.JSONDecodeError):
        return None
    return data if isinstance(data, dict) else None


def safe_extract_archive(archive: tarfile.TarFile, destination: pathlib.Path) -> None:
    destination = destination.resolve()
    members = archive.getmembers()
    for member in members:
        if member.issym() or member.islnk():
            raise ValueError(f"archive links are not supported: {member.name}")
        target = (destination / member.name).resolve()
        if destination != target and destination not in target.parents:
            raise ValueError(f"archive member escapes destination: {member.name}")
    archive.extractall(destination, members=members)


def command_exists(name: str) -> bool:
    return shutil.which(name) is not None


def run_command(
    command: list[str], *, capture: bool = False
) -> subprocess.CompletedProcess[str]:
    return subprocess.run(command, check=True, text=True, capture_output=capture)


def dedupe_preserving_order(values: list[str]) -> list[str]:
    seen = set()
    result = []
    for value in values:
        if value in seen:
            continue
        seen.add(value)
        result.append(value)
    return result


def parse_comma_values(values: list[str] | None) -> list[str]:
    result = []
    for value in values or []:
        result.extend(part.strip() for part in value.split(",") if part.strip())
    return result


def parse_parts(values: list[str] | None) -> list[str]:
    requested = parse_comma_values(values)
    if any(value.lower() == "all" for value in requested):
        return list(ALL_SWE_ATLAS_PARTS)
    return dedupe_preserving_order(
        [config.canonical_part(value) for value in requested]
    )


def configure_modal_environment(args: argparse.Namespace) -> None:
    os.environ["AGENT_EVALS_APP_NAME"] = args.app_name
    os.environ["AGENT_EVALS_VOLUME"] = args.volume
    os.environ["AGENT_EVALS_MODAL_TOKEN_SECRET"] = args.modal_token_secret
    os.environ["AGENT_EVALS_LLM_PROVIDERS_SECRET"] = args.api_secret


def import_modal_app(args: argparse.Namespace):
    configure_modal_environment(args)
    from . import modal_app

    return modal_app


class AppNotDeployedError(RuntimeError):
    """Raised when a Modal function lookup fails because the app (or that
    specific function) has not been deployed. Surfaced as a clean CLI error
    pointing the user at `zed-eval deploy`.
    """


def deploy_app(args: argparse.Namespace) -> None:
    """Publish the Modal app. This is the ONLY code path that deploys.

    Deploying replaces the live app and cancels any in-flight eval runs, so it
    must happen exclusively via the `deploy` subcommand. Every other command
    just looks up already-deployed functions with `deployed_function`. After
    changing harness code, run `zed-eval deploy` once.
    """
    modal_app = import_modal_app(args)
    print(f"Deploying Modal app '{args.app_name}'...")
    modal_app.app.deploy(name=args.app_name)


def deployed_function(args: argparse.Namespace, function_name: str):
    """Look up an already-deployed Modal function. Never deploys.

    `modal.Function.from_name` resolves a published function without deploying,
    which is exactly what we want: deploying would cancel in-flight runs. We
    hydrate eagerly so a missing app/function fails here with an actionable
    message instead of deep inside a later `.spawn()`/`.remote()` call.
    """
    import modal
    from modal.exception import NotFoundError

    configure_modal_environment(args)
    function = modal.Function.from_name(args.app_name, function_name)
    try:
        function.hydrate()
    except NotFoundError as error:
        raise AppNotDeployedError(
            f"app '{args.app_name}' not deployed (or function "
            f"'{function_name}' missing) — run 'zed-eval deploy' first"
        ) from error
    return function


def modal_call_id(call: Any) -> str:
    for attribute in ("object_id", "id", "function_call_id"):
        value = getattr(call, attribute, None)
        if value:
            return str(value)
    return str(call)


def print_table(rows: list[dict[str, Any]], columns: list[tuple[str, str]]) -> None:
    if not rows:
        print("No rows")
        return
    widths = []
    for key, title in columns:
        width = len(title)
        for row in rows:
            width = max(width, len(str(row.get(key) or "")))
        widths.append(min(width, 80))
    print(
        "  ".join(
            title.ljust(widths[index]) for index, (_key, title) in enumerate(columns)
        )
    )
    print("  ".join("-" * width for width in widths))
    for row in rows:
        cells = []
        for index, (key, _title) in enumerate(columns):
            text = str(row.get(key) or "")
            if len(text) > widths[index]:
                text = text[: widths[index] - 1] + "…"
            cells.append(text.ljust(widths[index]))
        print("  ".join(cells))


def default_namespace(args: argparse.Namespace) -> str:
    return source.sanitize_namespace(args.namespace or source.default_namespace())


def resolve_run_location(args: argparse.Namespace, run_id: str) -> tuple[str, str]:
    """Resolve (namespace, experiment_name) for a run-lookup command.

    When `--experiment-name` is given, behaves exactly as before: namespace comes
    from `--namespace` or the local git default. When it is omitted, the run is
    located via the launch-time local run index (`run_index`) so a bare run id is
    enough. Raises a friendly error pointing at the explicit flags when the run
    can't be found.
    """
    from . import run_index

    explicit_experiment = getattr(args, "experiment_name", None)
    explicit_namespace = getattr(args, "namespace", None)
    if explicit_experiment:
        namespace = explicit_namespace or source.default_namespace()
        return (
            source.sanitize_namespace(namespace),
            source.sanitize_namespace(explicit_experiment),
        )
    entry = run_index.lookup(run_id)
    if not entry:
        raise ValueError(
            f"could not locate run '{run_id}' in the local run index "
            f"({run_index.index_path()}). Pass --experiment-name (and "
            "--namespace if it isn't yours). The index is written automatically "
            "when you launch a run from this machine."
        )
    namespace = (
        explicit_namespace or entry.get("namespace") or source.default_namespace()
    )
    return (
        source.sanitize_namespace(namespace),
        source.sanitize_namespace(entry["experiment_name"]),
    )
