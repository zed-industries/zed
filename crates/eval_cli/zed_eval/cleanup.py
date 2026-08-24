"""Prune stale build artifacts and build cache from the shared volume.

Eval results are sacrosanct: this only ever touches the build-related top-level
directories listed in `PRUNABLE_DIRS`. `runs/` (the eval outputs) is never in
that set and is never walked, so a bug here cannot delete results.

The logic is pure-filesystem (no Modal dependency) so it can be unit-tested
against a temporary directory; `modal_app` wraps it with volume reload/commit and
a daily schedule.
"""

from __future__ import annotations

import datetime
import shutil
import time
from pathlib import Path
from typing import Any

# The ONLY top-level volume directories cleanup may modify. Eval results live in
# runs/, which is deliberately absent here.
PRUNABLE_DIRS = frozenset({"builds", "build-locks", "tmp"})

DEFAULT_BUILD_RETENTION_DAYS = 14.0
DEFAULT_LOCK_TTL_HOURS = 6.0
DEFAULT_TMP_MAX_AGE_HOURS = 24.0


def _age_days(path: Path) -> float:
    try:
        return (time.time() - path.stat().st_mtime) / 86400.0
    except OSError:
        return 0.0


def prune_artifacts(
    data_root: Path | str,
    *,
    dry_run: bool = False,
    build_retention_days: float = DEFAULT_BUILD_RETENTION_DAYS,
    lock_ttl_hours: float = DEFAULT_LOCK_TTL_HOURS,
    tmp_max_age_hours: float = DEFAULT_TMP_MAX_AGE_HOURS,
) -> dict[str, Any]:
    data_root = Path(data_root)
    removed: dict[str, list[Any]] = {
        "builds": [],
        "locks": [],
        "tmp": [],
    }

    # 1. Stale content-addressed builds, aged by their READY marker (or the dir
    #    itself for never-finished builds). Cheap to rebuild on demand.
    builds_dir = data_root / "builds"
    if builds_dir.is_dir():
        for build_dir in sorted(builds_dir.iterdir()):
            if not build_dir.is_dir():
                continue
            ready = build_dir / "READY"
            age = _age_days(ready if ready.exists() else build_dir)
            if age > build_retention_days:
                removed["builds"].append(
                    {"build_id": build_dir.name, "age_days": round(age, 1)}
                )
                if not dry_run:
                    shutil.rmtree(build_dir, ignore_errors=True)

    # 2. Orphaned single-flight build leases left by crashed builds.
    locks_dir = data_root / "build-locks"
    if locks_dir.is_dir():
        for lock in sorted(locks_dir.glob("*.json")):
            if _age_days(lock) * 24.0 > lock_ttl_hours:
                removed["locks"].append(lock.name)
                if not dry_run:
                    lock.unlink(missing_ok=True)

    # 3. Orphaned temp build dirs from failed/raced builds.
    tmp_builds = data_root / "tmp" / "builds"
    if tmp_builds.is_dir():
        for entry in sorted(tmp_builds.iterdir()):
            if _age_days(entry) * 24.0 > tmp_max_age_hours:
                removed["tmp"].append(entry.name)
                if not dry_run:
                    if entry.is_dir():
                        shutil.rmtree(entry, ignore_errors=True)
                    else:
                        entry.unlink(missing_ok=True)

    return {
        "dry_run": dry_run,
        "at": datetime.datetime.now(datetime.timezone.utc).isoformat(),
        "removed": removed,
        "counts": {key: len(value) for key, value in removed.items()},
    }
