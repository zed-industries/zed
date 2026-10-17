from __future__ import annotations

import unittest

from zed_eval import report
from zed_eval.report import TrialRecord


def trial(
    task: str,
    passed: bool | None,
    *,
    steps: int,
    tool_calls: dict[str, int],
    input_tokens: int = 0,
    output_tokens: int = 0,
) -> TrialRecord:
    return TrialRecord(
        task_name=task,
        started_at=None,
        passed=passed,
        error_reason=None,
        input_tokens=input_tokens,
        output_tokens=output_tokens,
        step_count=steps,
        total_tool_calls=sum(tool_calls.values()),
        tool_calls=dict(tool_calls),
        has_result=True,
    )


class SliceMetricsTests(unittest.TestCase):
    def test_means_over_subset(self) -> None:
        records = [
            trial("a", True, steps=4, tool_calls={"read_file": 2, "edit_file": 1}),
            trial("b", True, steps=6, tool_calls={"read_file": 4, "edit_file": 3}),
        ]
        metrics = report.slice_metrics(records)
        self.assertEqual(metrics["n"], 2)
        self.assertEqual(metrics["mean_steps"], 5.0)
        self.assertEqual(metrics["mean_tool_calls"], 5.0)
        self.assertEqual(metrics["mean_tool_calls_by_tool"]["read_file"], 3.0)
        self.assertEqual(metrics["mean_tool_calls_by_tool"]["edit_file"], 2.0)

    def test_empty_slice(self) -> None:
        metrics = report.slice_metrics([])
        self.assertEqual(metrics["n"], 0)
        self.assertIsNone(metrics["mean_steps"])


class ReportConditioningTests(unittest.TestCase):
    def test_on_success_excludes_failures_and_errors(self) -> None:
        records = [
            trial(
                "pass1", True, steps=3, tool_calls={"read_file": 1}, input_tokens=100
            ),
            trial(
                "fail1", False, steps=9, tool_calls={"read_file": 9}, input_tokens=900
            ),
            trial("err1", None, steps=1, tool_calls={"read_file": 1}),
        ]
        scored = [r for r in records if r.passed is not None]
        passing = [r for r in scored if r.passed]

        on_success = report.slice_metrics(passing)
        overall = report.slice_metrics(scored)

        # Conditioned on success: only the passing trial counts.
        self.assertEqual(on_success["n"], 1)
        self.assertEqual(on_success["mean_steps"], 3.0)
        # Overall (scored) includes the failure, inflating the mean.
        self.assertEqual(overall["n"], 2)
        self.assertEqual(overall["mean_steps"], 6.0)

    def test_pass_rate_single_attempt(self) -> None:
        scored = [
            trial("a", True, steps=1, tool_calls={}),
            trial("b", False, steps=1, tool_calls={}),
            trial("c", True, steps=1, tool_calls={}),
            trial("d", True, steps=1, tool_calls={}),
        ]
        rate, sem, attempts = report.pass_rate_with_sem(scored)
        self.assertAlmostEqual(rate, 0.75)
        self.assertIsNone(sem)
        self.assertEqual(attempts, 1)


class TimeoutVerdictTests(unittest.TestCase):
    def _result(self, exc_type: str) -> dict:
        return {"exception_info": {"exception_type": exc_type}}

    def test_timeout_excluded_by_default(self) -> None:
        passed, reason = report.trial_verdict(self._result("AgentTimeoutError"))
        self.assertIsNone(passed)
        self.assertEqual(reason, "AgentTimeoutError")

    def test_timeout_counts_as_failure_when_enabled(self) -> None:
        passed, reason = report.trial_verdict(
            self._result("AgentTimeoutError"), timeout_is_failure=True
        )
        self.assertIs(passed, False)
        self.assertEqual(reason, "AgentTimeoutError")

    def test_other_exceptions_stay_errored_even_when_enabled(self) -> None:
        # A genuine harness/infra exception is never a scored failure.
        passed, _ = report.trial_verdict(
            self._result("SandboxCreateError"), timeout_is_failure=True
        )
        self.assertIsNone(passed)


if __name__ == "__main__":
    unittest.main()
