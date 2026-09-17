from __future__ import annotations

import json
import tempfile
import unittest
from pathlib import Path

from zed_eval import config, rejudge


class TaskResolutionTests(unittest.TestCase):
    def test_task_tests_dir_layout(self) -> None:
        root = Path("/cache")
        self.assertEqual(
            rejudge.task_tests_dir("scale-ai/task-abc", "sha256:deadbeef", root),
            root / "scale-ai" / "task-abc" / "deadbeef" / "tests",
        )

    def test_task_tests_dir_without_hash_prefix(self) -> None:
        root = Path("/cache")
        self.assertEqual(
            rejudge.task_tests_dir("org/task-1", "rawref", root),
            root / "org" / "task-1" / "rawref" / "tests",
        )

    def test_detect_part(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            tests = Path(tmp)
            self.assertIsNone(rejudge.detect_part(tests))
            (tests / rejudge.RF_VERIFIER).write_text("")
            self.assertEqual(rejudge.detect_part(tests), rejudge.PART_RF)

    def test_detect_part_qna(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            tests = Path(tmp)
            (tests / rejudge.QNA_VERIFIER).write_text("")
            self.assertEqual(rejudge.detect_part(tests), rejudge.PART_QNA)


class RewardRecombinationTests(unittest.TestCase):
    def test_rf_pass_requires_rubric_and_tests(self) -> None:
        # rubric passes, tests passed -> overall pass
        rewards = rejudge.recompute_rf_rewards(
            {"must_have_pass": True, "agg_score": 0.9},
            {"tests_reward": 1.0},
        )
        self.assertEqual(rewards["reward"], 1.0)
        self.assertEqual(rewards["overall_pass"], 1.0)
        self.assertEqual(rewards["must_have_pass"], 1.0)
        self.assertEqual(rewards["tests_reward"], 1.0)

    def test_rf_rubric_pass_but_tests_fail_is_fail(self) -> None:
        rewards = rejudge.recompute_rf_rewards(
            {"must_have_pass": True, "agg_score": 0.9},
            {"tests_reward": 0.0},
        )
        self.assertEqual(rewards["reward"], 0.0)

    def test_rf_tests_pass_but_rubric_fail_is_fail(self) -> None:
        rewards = rejudge.recompute_rf_rewards(
            {"must_have_pass": False, "agg_score": 0.2},
            {"tests_reward": 1.0},
        )
        self.assertEqual(rewards["reward"], 0.0)
        self.assertEqual(rewards["rubrics_agg_score"], 0.2)

    def test_rf_preserves_tests_reward_from_parent(self) -> None:
        # tests_reward is invariant under rejudge (patch unchanged).
        rewards = rejudge.recompute_rf_rewards(
            {"must_have_pass": False, "agg_score": 0.0},
            {"tests_reward": 1.0, "reward": 1.0},
        )
        self.assertEqual(rewards["tests_reward"], 1.0)

    def test_qna_reward_is_pass_verdict(self) -> None:
        self.assertEqual(rejudge.recompute_qna_rewards({"pass": True})["reward"], 1.0)
        self.assertEqual(rejudge.recompute_qna_rewards({"pass": False})["reward"], 0.0)


class ResultPatchingTests(unittest.TestCase):
    def test_patch_preserves_agent_metrics(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            result_path = Path(tmp) / "result.json"
            result_path.write_text(
                json.dumps(
                    {
                        "task_name": "scale-ai/task-1",
                        "agent_result": {"tokens": 12345},
                        "verifier_result": {"rewards": {"reward": 0.0}},
                    }
                )
            )
            rejudge.patch_result_rewards(result_path, {"reward": 1.0})
            patched = json.loads(result_path.read_text())
            # Verdict updated...
            self.assertEqual(patched["verifier_result"]["rewards"], {"reward": 1.0})
            # ...but agent metrics and other fields untouched.
            self.assertEqual(patched["agent_result"], {"tokens": 12345})
            self.assertEqual(patched["task_name"], "scale-ai/task-1")


class JudgeEnvironmentTests(unittest.TestCase):
    def test_proxy_environment_from_judge_config(self) -> None:
        judge = config.get_judge("deepseek-v4-pro")
        env = rejudge.proxy_environment(judge)
        self.assertEqual(env["ZED_JUDGE_UPSTREAM"], judge.upstream)
        self.assertEqual(env["ZED_JUDGE_AUTH_ENV"], judge.auth_env)
        self.assertEqual(env["ZED_JUDGE_MAX_TOKENS"], str(judge.max_tokens))

    def test_proxy_environment_omits_unset_max_tokens(self) -> None:
        judge = config.get_judge("leaderboard")
        env = rejudge.proxy_environment(judge)
        self.assertNotIn("ZED_JUDGE_MAX_TOKENS", env)

    def test_verifier_environment_points_at_proxy(self) -> None:
        env = rejudge.verifier_environment("some/model", 8089)
        self.assertEqual(env["EVAL_MODEL"], "some/model")
        self.assertEqual(env["EVAL_BASE_URL"], "http://127.0.0.1:8089/v1")
        self.assertTrue(env["EVAL_API_KEY"])


if __name__ == "__main__":
    unittest.main()
