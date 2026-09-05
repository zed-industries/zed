from __future__ import annotations

import json
import os
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch

from zed_eval import run_index


class RunIndexTests(unittest.TestCase):
    def with_index_path(self, path: Path):
        return patch.dict(os.environ, {"AGENT_EVALS_RUN_INDEX": str(path)})

    def test_record_lookup_and_recent(self) -> None:
        with tempfile.TemporaryDirectory() as temporary_directory:
            path = Path(temporary_directory) / "nested" / "run-index.json"
            with self.with_index_path(path):
                run_index.record_run(
                    {
                        "run_id": "run-1",
                        "namespace": "alice",
                        "experiment_name": "rf",
                        "volume_name": "custom-volume",
                        "agent_model": "sonnet-4.6",
                        "created_at": "2026-01-01T00:00:00+00:00",
                    }
                )
                run_index.record_run(
                    {
                        "run_id": "run-2",
                        "namespace": "alice",
                        "experiment_name": "qna",
                        "volume_name": None,
                    }
                )

                self.assertEqual(run_index.lookup("run-1")["volume"], "custom-volume")
                self.assertEqual(
                    [entry["run_id"] for entry in run_index.recent(2)],
                    ["run-2", "run-1"],
                )
                self.assertEqual(run_index.most_recent()["run_id"], "run-2")
                self.assertNotIn("volume", run_index.lookup("run-2"))

    def test_record_refreshes_existing_run(self) -> None:
        with tempfile.TemporaryDirectory() as temporary_directory:
            path = Path(temporary_directory) / "run-index.json"
            with self.with_index_path(path):
                run_index.record_run(
                    {
                        "run_id": "run-1",
                        "namespace": "alice",
                        "experiment_name": "rf",
                    }
                )
                run_index.record_run(
                    {
                        "run_id": "run-1",
                        "namespace": "bob",
                        "experiment_name": "tw",
                    }
                )

                entries = run_index.recent(10)
                self.assertEqual(len(entries), 1)
                self.assertEqual(entries[0]["namespace"], "bob")
                self.assertEqual(entries[0]["experiment_name"], "tw")

    def test_corrupt_or_malformed_index_is_ignored(self) -> None:
        with tempfile.TemporaryDirectory() as temporary_directory:
            path = Path(temporary_directory) / "run-index.json"
            with self.with_index_path(path):
                path.write_text("not json")
                self.assertIsNone(run_index.lookup("run-1"))
                self.assertEqual(run_index.recent(), [])

                path.write_text(
                    json.dumps(
                        {
                            "runs": [
                                {"run_id": "missing-location"},
                                ["not", "a", "dict"],
                                {
                                    "run_id": "run-1",
                                    "namespace": "alice",
                                    "experiment_name": "rf",
                                },
                            ]
                        }
                    )
                )
                self.assertIsNone(run_index.lookup("missing-location"))
                self.assertEqual(run_index.lookup("run-1")["experiment_name"], "rf")

    def test_caps_entries(self) -> None:
        with tempfile.TemporaryDirectory() as temporary_directory:
            path = Path(temporary_directory) / "run-index.json"
            with self.with_index_path(path), patch.object(run_index, "MAX_ENTRIES", 2):
                for index in range(3):
                    run_index.record_run(
                        {
                            "run_id": f"run-{index}",
                            "namespace": "alice",
                            "experiment_name": "rf",
                        }
                    )

                self.assertEqual(
                    [entry["run_id"] for entry in run_index.recent(10)],
                    ["run-2", "run-1"],
                )
                self.assertIsNone(run_index.lookup("run-0"))

    def test_zero_limit_returns_no_entries(self) -> None:
        with tempfile.TemporaryDirectory() as temporary_directory:
            path = Path(temporary_directory) / "run-index.json"
            with self.with_index_path(path):
                run_index.record_run(
                    {
                        "run_id": "run-1",
                        "namespace": "alice",
                        "experiment_name": "rf",
                    }
                )
                self.assertEqual(run_index.recent(0), [])


if __name__ == "__main__":
    unittest.main()
