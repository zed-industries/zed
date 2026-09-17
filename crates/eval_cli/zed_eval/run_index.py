"""Local, launch-time index of runs started from this machine.

Every launch writes a tiny record mapping a run id to where it lives on the
volume (namespace + experiment) plus a little metadata. This lets `status`,
`logs`, `fetch`, `report`, and `rejudge` locate a run from just its id, so the
operator no longer has to repeat `--namespace`/`--experiment-name`.

It is deliberately best-effort: a missing, unreadable, or unwritable index never
blocks a launch and never crashes a lookup. Commands that can't find a run in
the index fall back to the explicit `--experiment-name`/`--namespace` flags with
a friendly hint.
"""

from __future__ import annotations

import json
import os
import tempfile
from pathlib import Path
from typing import Any

INDEX_VERSION = 1
MAX_ENTRIES = 500
REQUIRED_FIELDS = ("run_id", "namespace", "experiment_name")
OPTIONAL_FIELDS = (
    "volume",
    "agent_model",
    "judge_preset",
    "build_id",
    "suite_id",
    "kind",
    "created_at",
)


def index_path() -> Path:
    override = os.environ.get("AGENT_EVALS_RUN_INDEX")
    if override:
        return Path(override).expanduser()
    cache_home = os.environ.get("XDG_CACHE_HOME")
    base = Path(cache_home).expanduser() if cache_home else Path.home() / ".cache"
    return base / "agent-evals" / "run-index.json"


def _string_field(data: dict[str, Any], field: str) -> str | None:
    value = data.get(field)
    if value is None:
        return None
    value = str(value)
    return value if value else None


def _normalize_entry(data: dict[str, Any]) -> dict[str, Any] | None:
    entry = {}
    for field in REQUIRED_FIELDS:
        value = _string_field(data, field)
        if value is None:
            return None
        entry[field] = value
    for field in OPTIONAL_FIELDS:
        value = data.get(field)
        if value is not None:
            entry[field] = value
    return entry


def _load_entries() -> list[dict[str, Any]]:
    try:
        data = json.loads(index_path().read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError):
        return []
    raw_entries = data.get("runs") if isinstance(data, dict) else data
    if not isinstance(raw_entries, list):
        return []

    entries = []
    for raw_entry in raw_entries:
        if not isinstance(raw_entry, dict):
            continue
        entry = _normalize_entry(raw_entry)
        if entry is not None:
            entries.append(entry)
    return entries


def _entry_from_request(run_request: dict[str, Any]) -> dict[str, Any] | None:
    entry = _normalize_entry(
        {
            "run_id": run_request.get("run_id"),
            "namespace": run_request.get("namespace"),
            "experiment_name": run_request.get("experiment_name"),
            "volume": run_request.get("volume_name"),
            "agent_model": run_request.get("agent_model"),
            "judge_preset": run_request.get("judge_preset"),
            "build_id": run_request.get("build_id"),
            "suite_id": run_request.get("suite_id"),
            "kind": run_request.get("kind"),
            "created_at": run_request.get("created_at"),
        }
    )
    return entry


def _write_entries(entries: list[dict[str, Any]]) -> None:
    path = index_path()
    payload = json.dumps({"version": INDEX_VERSION, "runs": entries}, indent=2) + "\n"
    path.parent.mkdir(parents=True, exist_ok=True)
    temporary_path: Path | None = None
    try:
        with tempfile.NamedTemporaryFile(
            "w",
            encoding="utf-8",
            dir=path.parent,
            prefix=f".{path.name}.",
            suffix=".tmp",
            delete=False,
        ) as temporary_file:
            temporary_path = Path(temporary_file.name)
            temporary_file.write(payload)
        os.replace(temporary_path, path)
    except OSError:
        if temporary_path is not None:
            temporary_path.unlink(missing_ok=True)
        raise


def record_run(run_request: dict[str, Any]) -> None:
    """Append (or refresh) the index entry for a launched run.

    Accepts the same request dict the controller is spawned with. Silently does
    nothing if the essential fields are missing or the index can't be written —
    bookkeeping must never abort a launch.
    """
    entry = _entry_from_request(run_request)
    if entry is None:
        return

    entries = [
        item for item in _load_entries() if item.get("run_id") != entry["run_id"]
    ]
    entries.append(entry)
    entries = entries[-MAX_ENTRIES:]
    try:
        _write_entries(entries)
    except OSError:
        pass


def lookup(run_id: str | None) -> dict[str, Any] | None:
    """Return the most recently recorded entry for `run_id`, if any."""
    if not run_id:
        return None
    for entry in reversed(_load_entries()):
        if entry["run_id"] == run_id:
            return entry
    return None


def recent(limit: int = 20) -> list[dict[str, Any]]:
    """Return up to `limit` most-recent entries, newest first."""
    if limit <= 0:
        return []
    return list(reversed(_load_entries()[-limit:]))


def most_recent() -> dict[str, Any] | None:
    entries = recent(1)
    return entries[0] if entries else None
