from __future__ import annotations

import os
import tempfile
import time
import unittest
from pathlib import Path

from zed_eval import cleanup


def age_path(path: Path, days: float) -> None:
    past = time.time() - days * 86400.0
    os.utime(path, (past, past))


class PruneArtifactsTests(unittest.TestCase):
    def setUp(self) -> None:
        self._tmp = tempfile.TemporaryDirectory()
        self.root = Path(self._tmp.name)
        self.addCleanup(self._tmp.cleanup)

    def _make_build(self, build_id: str, age_days: float) -> Path:
        build_dir = self.root / "builds" / build_id
        build_dir.mkdir(parents=True)
        (build_dir / "eval-cli").write_text("binary")
        ready = build_dir / "READY"
        ready.write_text("done")
        age_path(ready, age_days)
        return build_dir

    def test_eval_results_are_never_touched(self) -> None:
        runs = self.root / "runs" / "anant" / "swe-atlas-rf" / "run-1"
        runs.mkdir(parents=True)
        result_file = runs / "summary.json"
        result_file.write_text("{}")
        age_path(result_file, 999)

        cleanup.prune_artifacts(self.root, dry_run=False, build_retention_days=14)

        self.assertTrue(result_file.exists())

    def test_removes_old_builds_keeps_recent(self) -> None:
        self._make_build("bld-old", age_days=30)
        self._make_build("bld-new", age_days=1)

        result = cleanup.prune_artifacts(
            self.root, dry_run=False, build_retention_days=14
        )

        self.assertFalse((self.root / "builds" / "bld-old").exists())
        self.assertTrue((self.root / "builds" / "bld-new").exists())
        self.assertEqual(result["counts"]["builds"], 1)

    def test_dry_run_deletes_nothing(self) -> None:
        self._make_build("bld-old", age_days=30)
        result = cleanup.prune_artifacts(
            self.root, dry_run=True, build_retention_days=14
        )
        self.assertTrue((self.root / "builds" / "bld-old").exists())
        self.assertEqual(result["counts"]["builds"], 1)

    def test_stale_lock_removed(self) -> None:
        locks = self.root / "build-locks"
        locks.mkdir(parents=True)
        stale = locks / "bld-x.json"
        stale.write_text("{}")
        age_path(stale, 1)  # 1 day > 6h ttl
        fresh = locks / "bld-y.json"
        fresh.write_text("{}")

        cleanup.prune_artifacts(self.root, dry_run=False, lock_ttl_hours=6)

        self.assertFalse(stale.exists())
        self.assertTrue(fresh.exists())


if __name__ == "__main__":
    unittest.main()
