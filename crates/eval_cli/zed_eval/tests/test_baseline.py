from __future__ import annotations

import unittest

from zed_eval import baseline


class BaselineRecordTests(unittest.TestCase):
    def test_record_uses_base_ref_from_source_info(self) -> None:
        record, has_build_info = baseline._record_from_provenance(
            namespace="alice",
            fallback_experiment="swe-atlas-rf",
            run_id="run-1",
            provenance={
                "request": {
                    "experiment_name": "swe-atlas-rf",
                    "agent_model": "anthropic/claude-sonnet-4-6",
                    "judge_preset": "kimi-k2.7-code",
                    "build_id": "bld-test",
                },
                "summary": {"status": "completed", "trial_count": 10},
                "build_info": {
                    "build_id": "bld-test",
                    "base_sha": "a" * 40,
                    "patch_sha256": None,
                    "source": {"base_ref": "v0.210.0"},
                },
            },
        )

        self.assertTrue(has_build_info)
        self.assertEqual(record["base_ref"], "v0.210.0")


if __name__ == "__main__":
    unittest.main()
