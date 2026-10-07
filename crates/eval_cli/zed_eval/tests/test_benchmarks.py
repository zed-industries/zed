from __future__ import annotations

import unittest

from zed_eval import benchmarks, config, harness_command


def make_run_request(benchmark_id: str, **overrides: object) -> dict[str, object]:
    benchmark = benchmarks.get_benchmark(benchmark_id)
    request: dict[str, object] = {
        "run_id": "run-1",
        "namespace": "tester",
        "experiment_name": benchmark_id,
        "benchmark": benchmarks.benchmark_metadata(benchmark),
        "volume_name": "agent-evals",
        "api_secret_name": "agent-evals-llm-providers",
        "build_id": "bld-test",
        "agent_model": "anthropic/claude-sonnet-4-6",
        "judge_preset": benchmark.default_judge or "leaderboard",
    }
    request.update(overrides)
    return request


def option_values(command: list[str], option: str) -> list[str]:
    return [
        command[index + 1]
        for index, argument in enumerate(command[:-1])
        if argument == option
    ]


class BenchmarkResolutionTests(unittest.TestCase):
    def test_group_expands_to_parts(self) -> None:
        self.assertEqual(
            benchmarks.resolve_benchmarks(["swe-atlas"]),
            ["swe-atlas-qna", "swe-atlas-rf", "swe-atlas-tw"],
        )

    def test_aliases_and_dedup(self) -> None:
        self.assertEqual(
            benchmarks.resolve_benchmarks(["qna", "tb21", "rf,tw"]),
            [
                "swe-atlas-qna",
                "terminal-bench-2.1",
                "swe-atlas-rf",
                "swe-atlas-tw",
            ],
        )


class HarnessCommandTests(unittest.TestCase):
    def test_terminal_bench_is_harbor_registry_no_judge(self) -> None:
        command = harness_command.build_harness_command(
            make_run_request("terminal-bench-2.1"), "/tmp/jobs"
        )
        self.assertEqual(command[0], "harbor")
        self.assertIn("terminal-bench/terminal-bench-2-1", command)
        self.assertIn("-d", command)
        # Test-scored benchmarks must not wire up an LLM judge.
        self.assertFalse(any(arg.startswith("EVAL_MODEL=") for arg in command))
        self.assertNotIn("--verifier-import-path", command)

    def test_swe_atlas_rf_wires_judge(self) -> None:
        command = harness_command.build_harness_command(
            make_run_request("swe-atlas-rf"), "/tmp/jobs"
        )
        verifier_env = option_values(command, "--ve")
        agent_env = option_values(command, "--ae")

        self.assertEqual(command[0], "harbor")
        self.assertIn("scale-ai/swe-atlas-rf", command)
        self.assertIn(
            config.JUDGE_PROXY_VERIFIER_IMPORT_PATH,
            option_values(command, "--verifier-import-path"),
        )
        self.assertTrue(any(arg.startswith("EVAL_MODEL=") for arg in verifier_env))
        self.assertTrue(
            any(arg.startswith("ZED_JUDGE_UPSTREAM=") for arg in verifier_env)
        )
        self.assertTrue(
            any(arg.startswith("ZED_JUDGE_AUTH_ENV=") for arg in verifier_env)
        )
        self.assertIn("ZED_JUDGE_MAX_TOKENS=8192", verifier_env)
        self.assertFalse(any(arg.startswith("ZED_JUDGE_") for arg in agent_env))

    def test_deepswe_uses_pier_path_without_cli_allowlist(self) -> None:
        command = harness_command.build_harness_command(
            make_run_request("deepswe"), "/tmp/jobs"
        )
        self.assertEqual(command[0], "pier")
        self.assertIn("-p", command)
        self.assertIn("/tmp/datasets/deepswe/tasks", command)
        # Pier uses the Pier-native agent class.
        self.assertIn("zed_eval.pier_agent:ZedPierAgent", command)
        # The Pier network allowlist is declared by the agent, not the CLI;
        # `pier run` has no --agent-allow-host option.
        self.assertNotIn("--agent-allow-host", command)

    def test_harbor_benchmarks_use_harbor_agent(self) -> None:
        command = harness_command.build_harness_command(
            make_run_request("swe-atlas-rf"), "/tmp/jobs"
        )
        self.assertIn("zed_eval.agent:ZedAgent", command)

    def test_eval_cli_timeout_override(self) -> None:
        command = harness_command.build_harness_command(
            make_run_request("swe-atlas-rf", eval_cli_timeout=123), "/tmp/jobs"
        )
        self.assertIn("EVAL_CLI_TIMEOUT=123", command)

    def test_extra_env_is_forwarded_to_harness_agent(self) -> None:
        command = harness_command.build_harness_command(
            make_run_request(
                "swe-atlas-rf",
                extra_env={
                    "ZED_EVAL_INSTRUCTION_SUFFIX_FILE": "/data/prompts/checklist.md"
                },
            ),
            "/tmp/jobs",
        )

        self.assertIn(
            "ZED_EVAL_INSTRUCTION_SUFFIX_FILE=/data/prompts/checklist.md",
            option_values(command, "--ae"),
        )

    def test_swe_atlas_tw_uses_path_dataset(self) -> None:
        command = harness_command.build_harness_command(
            make_run_request("swe-atlas-tw"), "/tmp/jobs"
        )
        self.assertIn("-p", command)
        self.assertIn("/tmp/datasets/swe-atlas-tw/data/tw", command)


if __name__ == "__main__":
    unittest.main()
