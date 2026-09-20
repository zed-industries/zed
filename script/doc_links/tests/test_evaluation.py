import json
from pathlib import Path
import tempfile
import unittest

from doc_links import markdown
from doc_links.corpus import Page
from doc_links.evaluation import (
    EvaluationCase,
    check_evaluation,
    exact_anchor,
    forced_candidate,
    load_cases,
)
from doc_links.jev import Evaluation
from doc_links.policy import Thresholds
from doc_links.retrieval import AnchorOption, DestinationCandidate, Index


class EvaluationTest(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory()
        self.root = Path(self.temporary.name)

    def tearDown(self):
        self.temporary.cleanup()

    def page(self, path: str, title: str, source: str) -> Page:
        return Page(
            path=Path(path),
            title=title,
            source=source,
            blocks=markdown.parse(source),
            existing_links=frozenset(),
        )

    def case(
        self,
        outcome: str = "link",
        anchor_text: str = "correct anchor",
        context_contains: str | None = None,
    ) -> EvaluationCase:
        return EvaluationCase(
            identifier="case",
            outcome=outcome,
            source_path="source.md",
            target_path="target.md",
            anchor_text=anchor_text,
            reason="Reviewed expectation.",
            context_contains=context_contains,
        )

    def evaluation(
        self,
        anchor_choice: str = "anchor_000",
        quality: float = 0.9,
    ) -> Evaluation:
        source = self.page(
            "source.md",
            "Source",
            "# Source\n\nUse the correct anchor here.\n",
        )
        target = self.page("target.md", "Target", "# Target\n\nDetails.\n")
        block = source.prose_blocks[0]
        start = source.source.index("correct anchor")
        anchor = AnchorOption(
            identifier="anchor_000",
            text="correct anchor",
            start=start,
            end=start + len("correct anchor"),
            block_start=block.start,
            block_end=block.end,
            block_hash=block.content_hash,
            score=1.0,
        )
        candidate = DestinationCandidate(target, 1.0, (block,), (anchor,))
        return Evaluation(
            target=candidate,
            reason_probability=0.9,
            destination_probability=0.9,
            anchor_choice=anchor_choice,
            anchor_probability=0.8,
            anchor_quality_probability=quality,
        )

    def write_cases(self, cases: list[dict]) -> Path:
        path = self.root / "cases.json"
        path.write_text(
            json.dumps({"schema_version": 2, "cases": cases}),
            encoding="utf-8",
        )
        return path

    def test_link_requires_reviewed_anchor_choice(self):
        message = check_evaluation(
            self.case("link"),
            self.evaluation("anchor_001"),
            "strong_review",
            Thresholds(),
        )
        self.assertEqual(message, "expected anchor_000, chose anchor_001")
        self.assertIsNone(
            check_evaluation(
                self.case("link"),
                self.evaluation(),
                "strong_review",
                Thresholds(),
            )
        )

    def test_link_must_remain_actionable(self):
        message = check_evaluation(
            self.case("link"),
            self.evaluation(),
            "rejected",
            Thresholds(),
        )
        self.assertEqual(message, "expected an actionable link, got rejected")

    def test_no_link_fails_when_forbidden_anchor_passes_quality(self):
        message = check_evaluation(
            self.case("no_link"),
            self.evaluation(quality=0.9),
            "strong_review",
            Thresholds(),
        )
        self.assertEqual(
            message,
            "forbidden anchor selected with passing anchor quality",
        )
        self.assertIsNone(
            check_evaluation(
                self.case("no_link"),
                self.evaluation(quality=0.5),
                "strong_review",
                Thresholds(),
            )
        )

    def test_context_selects_exact_occurrence_within_one_block(self):
        source_text = (
            "# Source\n\nUse the command palette first. "
            "Avoid the command palette's corner.\n"
        )
        source = self.page("source.md", "Source", source_text)
        first = exact_anchor(
            source,
            self.case(
                anchor_text="command palette",
                context_contains="Use the command palette first",
            ),
        )
        possessive = exact_anchor(
            source,
            self.case(
                outcome="no_link",
                anchor_text="command palette",
                context_contains="the command palette's corner",
            ),
        )
        self.assertIsNotNone(first)
        self.assertIsNotNone(possessive)
        self.assertEqual(first.start, source_text.index("command palette"))
        self.assertEqual(
            possessive.start,
            source_text.index("command palette", first.end),
        )

    def test_context_must_be_unique(self):
        source = self.page(
            "source.md",
            "Source",
            "# Source\n\nUse the anchor. Use the anchor.\n",
        )
        self.assertIsNone(
            exact_anchor(
                source,
                self.case(anchor_text="anchor", context_contains="Use the anchor"),
            )
        )

    def test_word_boundaries_reject_mid_word_anchor(self):
        source = self.page(
            "source.md",
            "Source",
            "# Source\n\nVim modes are listed.\n",
        )
        self.assertIsNone(exact_anchor(source, self.case(anchor_text="Vim mode")))

    def test_forced_candidate_puts_reviewed_anchor_first(self):
        source = self.page(
            "source.md",
            "Source",
            (
                "# Source\n\nUse the correct anchor with several extra words for context.\n\n"
                "Configure another useful feature from the settings page.\n\n"
                "Open the project panel to inspect workspace files.\n\n"
                "Run terminal commands while editing source code.\n"
            ),
        )
        target = self.page(
            "target.md",
            "Target",
            "# Target\n\nDetailed target guidance for readers.\n",
        )
        candidate = forced_candidate(
            source,
            target,
            self.case(),
            Index((source, target)),
            6,
        )
        self.assertIsNotNone(candidate)
        self.assertEqual(candidate.anchors[0].identifier, "anchor_000")
        self.assertEqual(candidate.anchors[0].text, "correct anchor")
        self.assertEqual(len(candidate.anchors), 6)

    def test_table_case_is_out_of_scope(self):
        source = self.page(
            "source.md",
            "Source",
            "# Source\n\n| Name | Value |\n| ---- | ----- |\n| Agent Panel | yes |\n",
        )
        self.assertIsNone(
            exact_anchor(
                source,
                self.case("out_of_scope", "Agent Panel"),
            )
        )

    def test_case_ids_must_be_unique(self):
        raw_case = {
            "id": "same",
            "outcome": "link",
            "source_path": "source.md",
            "target_path": "target.md",
            "anchor_text": "anchor",
            "reason": "Expected.",
        }
        with self.assertRaisesRegex(ValueError, "unique IDs"):
            load_cases(self.write_cases([raw_case, dict(raw_case)]))

    def test_calibration_records_every_case_without_requiring_all_pass(self):
        calibration = json.loads(
            Path("script/doc_links/evals/jev-1.13.0-calibration.json").read_text()
        )
        cases = load_cases(Path("script/doc_links/evals/pr_64481_review.json"))
        self.assertEqual(calibration["schema_version"], 2)
        self.assertEqual(calibration["thresholds"], Thresholds().to_dict())
        self.assertEqual(calibration["dataset"], "pr_64481_review.json")
        self.assertRegex(calibration["corpus_hash"], r"^[0-9a-f]{64}$")
        recorded = {item["id"]: item for item in calibration["cases"]}
        self.assertEqual(set(recorded), {case.identifier for case in cases})
        for case in cases:
            result = recorded[case.identifier]
            self.assertEqual(result["outcome"], case.outcome)
            self.assertIsInstance(result["passed"], bool)
            if case.outcome == "out_of_scope":
                self.assertIsNone(result["anchor_choice"])
                continue
            self.assertIsInstance(result["anchor_choice"], str)
            for key in (
                "reason_probability",
                "destination_probability",
                "anchor_probability",
                "anchor_quality_probability",
            ):
                self.assertGreaterEqual(result[key], 0)
                self.assertLessEqual(result[key], 1)

    def test_repository_feedback_dataset_is_valid(self):
        cases = load_cases(Path("script/doc_links/evals/pr_64481_review.json"))
        self.assertEqual(len(cases), 24)
        self.assertEqual(
            {case.outcome for case in cases},
            {"link", "no_link", "out_of_scope"},
        )


if __name__ == "__main__":
    unittest.main()
