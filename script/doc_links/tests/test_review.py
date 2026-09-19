import json
from pathlib import Path
import tempfile
import unittest

from doc_links import SCHEMA_VERSION
from doc_links.review import generate_html, review_data
from doc_links.schema import Anchor, Decision, Report, content_hash


class ReviewTest(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory()
        self.root = Path(self.temporary.name)
        self.docs = self.root / "docs"
        self.docs.mkdir()
        self.source = "# Source\n\nUnique source sentence with command palette.\n"
        self.target = "# Target\n\nUnique target sentence.\n"
        (self.docs / "source.md").write_text(self.source, encoding="utf-8")
        (self.docs / "target.md").write_text(self.target, encoding="utf-8")

    def tearDown(self):
        self.temporary.cleanup()

    def report(self) -> Report:
        start = self.source.index("command palette")
        block_start = self.source.index("Unique source")
        block_end = len(self.source)
        anchor = Anchor(
            text="command palette",
            start=start,
            end=start + len("command palette"),
            block_start=block_start,
            block_end=block_end,
            block_hash=content_hash(self.source[block_start:block_end]),
            relative_target="./target.md",
        )
        decisions = tuple(
            Decision(
                identifier=f"decision-{index}",
                queue="automatic",
                source_path="source.md",
                target_path="target.md",
                target_hash=content_hash(self.target),
                reason_probability=0.9,
                destination_probability=0.9,
                anchor_choice="anchor_000",
                anchor_probability=0.8,
                anchor_quality_probability=0.9,
                anchor=anchor,
            )
            for index in range(2)
        )
        return Report(
            model="jev-1.13.0",
            thresholds={
                "automatic_reason": 0.8,
                "direct_destination": 0.8,
                "exact_anchor": 0.6,
                "anchor_quality": 0.8,
                "near_reason": 0.75,
            },
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

    def test_pages_are_stored_once_in_review_data(self):
        data = review_data(self.report(), self.docs)
        self.assertEqual(set(data["pages"]), {"source.md", "target.md"})
        self.assertEqual(len(data["decisions"]), 2)

    def test_changed_page_blocks_review_generation(self):
        report = self.report()
        (self.docs / "source.md").write_text("Changed.\n", encoding="utf-8")
        with self.assertRaisesRegex(RuntimeError, "changed after the audit"):
            review_data(report, self.docs)

    def test_generated_html_uses_report_scoped_storage(self):
        report = self.report()
        report_path = self.root / "audit.json"
        report_path.write_text(json.dumps(report.to_dict()))
        output = self.root / "review.html"
        generate_html(report_path, self.docs, output)
        html = output.read_text()
        self.assertIn(report.report_hash, html)
        self.assertIn("zed-doc-links:", html)
        self.assertEqual(html.count("Unique source sentence"), 1)
        self.assertNotIn("/__api", html)


if __name__ == "__main__":
    unittest.main()
