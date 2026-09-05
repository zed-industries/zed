from __future__ import annotations

import argparse
import contextlib
import io
import unittest
from unittest.mock import patch

from zed_eval import config
from zed_eval.cli import build_parser
from zed_eval.launch import prepare_benchmark_suite, prepare_shared_build, prepare_suite


class LaunchPlanTests(unittest.TestCase):
    def parse(self, argv: list[str]) -> argparse.Namespace:
        return build_parser().parse_args(argv)

    def test_swe_atlas_defaults_judges_by_part(self) -> None:
        args = self.parse(
            [
                "swe-atlas",
                "--parts",
                "qna,rf",
                "--build",
                "bld-test",
                "--plan",
                "--allow-untracked",
                "--n-tasks",
                "1",
                "--yes",
            ]
        )

        prepared = prepare_suite(args)

        self.assertEqual([part for part, *_ in prepared], ["qna", "rf"])
        self.assertEqual(prepared[0][1]["build_id"], "bld-test")
        self.assertEqual(prepared[0][2]["build_id"], "bld-test")
        self.assertEqual(prepared[1][1]["build_id"], "bld-test")
        self.assertIsNone(prepared[1][2])
        self.assertEqual(prepared[0][1]["judge_preset"], "deepseek-v4-pro")
        self.assertEqual(prepared[1][1]["judge_preset"], "kimi-k2.7-code")
        self.assertEqual(prepared[0][1]["agent_model"], config.DEFAULT_MODEL)

    def test_baseten_model_preset_generates_provider_json(self) -> None:
        args = self.parse(
            [
                "swe-atlas",
                "--parts",
                "tw",
                "--build",
                "bld-test",
                "--plan",
                "--allow-untracked",
                "--model",
                "baseten:kimi-k2.7-code",
                "--yes",
            ]
        )

        prepared = prepare_suite(args)
        run_request = prepared[0][1]

        self.assertEqual([part for part, *_ in prepared], ["tw"])
        self.assertEqual(run_request["benchmark"]["id"], "swe-atlas-tw")
        self.assertEqual(
            run_request["agent_model"], "baseten/moonshotai/Kimi-K2.7-Code"
        )
        self.assertIn(
            "moonshotai/Kimi-K2.7-Code", run_request["openai_compatible_provider_json"]
        )
        self.assertIn(
            "https://inference.baseten.co/v1",
            run_request["openai_compatible_provider_json"],
        )

    def test_interactive_suite_can_include_non_swe_atlas_benchmarks(self) -> None:
        args = self.parse(
            [
                "swe-atlas",
                "--build",
                "bld-test",
                "--plan",
                "--allow-untracked",
                "--yes",
            ]
        )

        prepared = prepare_benchmark_suite(
            args, ["qna", "terminal-bench-2.1", "deepswe"]
        )

        self.assertEqual(
            [
                run_request["benchmark"]["id"]
                for _label, run_request, _build in prepared
            ],
            ["swe-atlas-qna", "terminal-bench-2.1", "deepswe"],
        )
        self.assertEqual(
            [label for label, *_ in prepared], ["qna", "terminal-bench-2.1", "deepswe"]
        )
        self.assertEqual(prepared[0][1]["judge_preset"], "deepseek-v4-pro")
        self.assertIsNone(prepared[1][1]["judge_preset"])
        self.assertIsNone(prepared[2][1]["judge_preset"])
        self.assertEqual(prepared[0][1]["build_id"], "bld-test")
        self.assertEqual(prepared[1][1]["build_id"], "bld-test")
        self.assertEqual(prepared[2][1]["build_id"], "bld-test")
        self.assertEqual(prepared[0][2]["build_id"], "bld-test")
        self.assertIsNone(prepared[1][2])
        self.assertIsNone(prepared[2][2])

    def test_explicit_existing_build_skips_source_preparation(self) -> None:
        args = self.parse(["run", "rf", "--build", "custom-build"])

        with (
            contextlib.redirect_stdout(io.StringIO()),
            patch("zed_eval.launch.build_ready_on_volume", return_value=True),
            patch("zed_eval.launch.resolve_source") as resolve_source,
        ):
            build_id, build_request = prepare_shared_build(args)

        self.assertEqual(build_id, "custom-build")
        self.assertIsNone(build_request)
        resolve_source.assert_not_called()

    def test_explicit_missing_build_uses_from_source(self) -> None:
        args = self.parse(
            ["run", "rf", "--build", "custom-build", "--from", "v0.210.0"]
        )

        with (
            patch("zed_eval.launch.build_ready_on_volume", return_value=False),
            patch("zed_eval.source.resolve_remote_ref", return_value="a" * 40),
        ):
            build_id, build_request = prepare_shared_build(args)

        self.assertEqual(build_id, "custom-build")
        self.assertEqual(build_request["build_id"], "custom-build")
        self.assertEqual(build_request["source"]["base_sha"], "a" * 40)
        self.assertEqual(build_request["source"]["base_ref"], "v0.210.0")
        self.assertTrue(build_request["source"]["clean_source"])


if __name__ == "__main__":
    unittest.main()
