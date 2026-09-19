from dataclasses import replace
import json
from pathlib import Path
import tempfile
import unittest

from doc_links.evaluation import evaluate_report, load_cases
from doc_links.schema import Anchor, Decision, Report, content_hash


class EvaluationTest(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory()
        self.root = Path(self.temporary.name)
        self.docs = self.root / "docs"
        self.docs.mkdir()
        self.source = "# Source\n\nUse the wrong anchor here.\n"
        self.target = "# Target\n\nDetails.\n"
        (self.docs / "source.md").write_text(self.source, encoding="utf-8")
        (self.docs / "target.md").write_text(self.target, encoding="utf-8")
        block_start = self.source.index("Use the")
        start = self.source.index("wrong anchor")
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
                text="wrong anchor",
                start=start,
                end=start + len("wrong anchor"),
                block_start=block_start,
                block_end=len(self.source),
                block_hash=content_hash(self.source[block_start:]),
                relative_target="./target.md",
            ),
        )

    def tearDown(self):
        self.temporary.cleanup()

    def write_report(self, queue: str) -> Path:
        decision = replace(self.decision, queue=queue)
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
            decisions=(decision,),
            usage={"input_tokens": 1, "output_tokens": 1},
        )
        path = self.root / "audit.json"
        path.write_text(json.dumps(report.to_dict()), encoding="utf-8")
        return path

    def write_cases(self) -> Path:
        path = self.root / "cases.json"
        path.write_text(
            json.dumps(
                {
                    "schema_version": 1,
                    "cases": [
                        {
                            "id": "case",
                            "source_path": "source.md",
                            "target_path": "target.md",
                            "forbidden_anchor": "wrong anchor",
                            "context_contains": "Use the wrong anchor",
                            "reason": "Reviewed as incorrect.",
                        }
                    ],
                }
            ),
            encoding="utf-8",
        )
        return path

    def test_automatic_forbidden_link_fails(self):
        failures = evaluate_report(
            self.write_report("automatic"),
            self.write_cases(),
            self.docs,
        )
        self.assertEqual([failure.identifier for failure in failures], ["case"])

    def test_reviewed_or_rejected_link_passes(self):
        failures = evaluate_report(
            self.write_report("strong_review"),
            self.write_cases(),
            self.docs,
        )
        self.assertEqual(failures, ())

    def test_repository_feedback_dataset_is_valid(self):
        cases = load_cases(
            Path("script/doc_links/evals/pr_64481_review.json")
        )
        self.assertEqual(len(cases), 15)


if __name__ == "__main__":
    unittest.main()
