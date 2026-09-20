from dataclasses import replace
import json
from pathlib import Path
import tempfile
import unittest

from doc_links import markdown
from doc_links.corpus import Page
from doc_links.evaluation import (
    EvaluationCase,
    evaluate_report,
    exact_anchor,
    forced_candidate,
    load_cases,
)
from doc_links.retrieval import Index
from doc_links.schema import Anchor, Decision, Report, content_hash


class EvaluationTest(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory()
        self.root = Path(self.temporary.name)
        self.docs = self.root / "docs"
        self.docs.mkdir()
        self.source = "# Source\n\nUse the correct anchor here.\n"
        self.target = "# Target\n\nDetails.\n"
        (self.docs / "source.md").write_text(self.source, encoding="utf-8")
        (self.docs / "target.md").write_text(self.target, encoding="utf-8")
        block_start = self.source.index("Use the")
        start = self.source.index("correct anchor")
        self.decision = Decision(
            identifier="decision",
            queue="automatic",
            source_path="source.md",
            target_path="target.md",
            target_hash=content_hash(self.target),
            reason_probability=0.9,
            destination_probability=0.9,
            anchor_choice="anchor_000",
            anchor_probability=0.8,
            anchor_quality_probability=0.9,
            anchor=Anchor(
                text="correct anchor",
                start=start,
                end=start + len("correct anchor"),
                block_start=block_start,
                block_end=len(self.source),
                block_hash=content_hash(self.source[block_start:]),
                relative_target="./target.md",
            ),
        )

    def tearDown(self):
        self.temporary.cleanup()

    def write_report(self, decisions: tuple[Decision, ...]) -> Path:
        report = Report(
            model="jev-1.13.0",
            thresholds={"automatic_reason": 0.8},
            pages={
                "source.md": {
                    "title": "Source",
                    "content_hash": content_hash(self.source),
                },
                "target.md": {
                    "title": "Target",
                    "content_hash": content_hash(self.target),
                },
            },
            decisions=decisions,
            usage={"input_tokens": 1, "output_tokens": 1},
        )
        path = self.root / "audit.json"
        path.write_text(json.dumps(report.to_dict()), encoding="utf-8")
        return path

    def write_cases(self, outcome: str = "link") -> Path:
        path = self.root / "cases.json"
        path.write_text(
            json.dumps(
                {
                    "schema_version": 2,
                    "cases": [
                        {
                            "id": "case",
                            "outcome": outcome,
                            "source_path": "source.md",
                            "target_path": "target.md",
                            "anchor_text": "correct anchor",
                            "reason": "Reviewed expectation.",
                        }
                    ],
                }
            ),
            encoding="utf-8",
        )
        return path

    def test_missing_pair_is_not_exercised(self):
        failures = evaluate_report(
            self.write_report(()),
            self.write_cases(),
            self.docs,
        )
        self.assertEqual(failures[0].message, "case was not exercised")

    def test_expected_link_must_be_actionable(self):
        failures = evaluate_report(
            self.write_report((self.decision,)),
            self.write_cases("link"),
            self.docs,
        )
        self.assertEqual(failures, ())
        rejected = replace(
            self.decision,
            queue="rejected",
        )
        failures = evaluate_report(
            self.write_report((rejected,)),
            self.write_cases("link"),
            self.docs,
        )
        self.assertIn("expected an actionable link", failures[0].message)

    def test_no_link_rejects_automatic_result(self):
        failures = evaluate_report(
            self.write_report((self.decision,)),
            self.write_cases("no_link"),
            self.docs,
        )
        self.assertEqual(failures[0].message, "forbidden link returned as automatic")

    def test_changed_source_is_not_evaluated(self):
        report = self.write_report((self.decision,))
        (self.docs / "source.md").write_text("Changed.\n", encoding="utf-8")
        failures = evaluate_report(report, self.write_cases(), self.docs)
        self.assertEqual(failures[0].message, "source page changed after audit")

    def test_word_boundaries_reject_mid_word_anchor(self):
        source = Page(
            path=Path("source.md"),
            title="Source",
            source="# Source\n\nVim modes are listed.\n",
            blocks=markdown.parse("# Source\n\nVim modes are listed.\n"),
            existing_links=frozenset(),
        )
        case = load_cases(self.write_cases())[0]
        case = replace(case, anchor_text="Vim mode")
        self.assertIsNone(exact_anchor(source, case))

    def test_forced_candidate_exercises_expected_anchor(self):
        source_text = (
            "# Source\n\nUse the correct anchor with several extra words for context.\n\n"
            "Configure another useful feature from the settings page.\n\n"
            "Open the project panel to inspect workspace files.\n\n"
            "Run terminal commands while editing source code.\n"
        )
        target_text = "# Target\n\nDetailed target guidance for readers.\n"
        source = Page(
            path=Path("source.md"),
            title="Source",
            source=source_text,
            blocks=markdown.parse(source_text),
            existing_links=frozenset(),
        )
        target = Page(
            path=Path("target.md"),
            title="Target",
            source=target_text,
            blocks=markdown.parse(target_text),
            existing_links=frozenset(),
        )
        case = EvaluationCase(
            identifier="case",
            outcome="link",
            source_path="source.md",
            target_path="target.md",
            anchor_text="correct anchor",
            reason="Expected link.",
        )
        candidate = forced_candidate(
            source,
            target,
            case,
            Index((source, target)),
            6,
        )
        self.assertIsNotNone(candidate)
        self.assertEqual(candidate.anchors[0].text, "correct anchor")
        self.assertEqual(len(candidate.anchors), 6)

    def test_table_case_is_explicitly_out_of_scope(self):
        source_text = (
            "# Source\n\n| Name | Value |\n| ---- | ----- |\n| Agent Panel | yes |\n"
        )
        source = Page(
            path=Path("source.md"),
            title="Source",
            source=source_text,
            blocks=markdown.parse(source_text),
            existing_links=frozenset(),
        )
        case = EvaluationCase(
            identifier="case",
            outcome="out_of_scope",
            source_path="source.md",
            target_path="target.md",
            anchor_text="Agent Panel",
            reason="Tables are out of scope.",
        )
        self.assertIsNone(exact_anchor(source, case))

    def test_calibration_matches_default_thresholds_and_dataset(self):
        calibration = json.loads(
            Path("script/doc_links/evals/jev-1.13.0-calibration.json").read_text()
        )
        from collections import Counter
        from doc_links.policy import Thresholds

        self.assertEqual(calibration["thresholds"], Thresholds().to_dict())
        cases = load_cases(Path("script/doc_links/evals/pr_64481_review.json"))
        counts = Counter(case.outcome for case in cases)
        for outcome, count in counts.items():
            self.assertEqual(calibration["results"][outcome]["total"], count)
            self.assertEqual(calibration["results"][outcome]["passed"], count)

    def test_repository_feedback_dataset_is_valid(self):
        cases = load_cases(Path("script/doc_links/evals/pr_64481_review.json"))
        self.assertEqual(len(cases), 24)
        self.assertTrue(any(case.outcome == "link" for case in cases))
        self.assertTrue(any(case.outcome == "no_link" for case in cases))
        self.assertTrue(any(case.outcome == "out_of_scope" for case in cases))


if __name__ == "__main__":
    unittest.main()
