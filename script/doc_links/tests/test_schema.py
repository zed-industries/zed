import unittest

from doc_links import SCHEMA_VERSION
from doc_links.schema import Report, parse_review_export


class SchemaTest(unittest.TestCase):
    def test_stale_label_is_rejected(self):
        with self.assertRaisesRegex(ValueError, "unknown decision"):
            parse_review_export(
                {
                    "schema_version": SCHEMA_VERSION,
                    "report_hash": "hash",
                    "decisions": [],
                    "labels": {
                        "old": {"label": "pass", "notes": ""},
                    },
                }
            )

    def test_v3_report_round_trip_with_nullable_quality(self):
        raw = {
            "schema_version": SCHEMA_VERSION,
            "model": "jev-1.13.0",
            "thresholds": {"automatic_reason": 0.8},
            "pages": {
                "source.md": {"title": "Source", "content_hash": "source-hash"},
                "target.md": {"title": "Target", "content_hash": "target-hash"},
            },
            "decisions": [
                {
                    "id": "decision",
                    "queue": "strong_review",
                    "source_path": "source.md",
                    "target_path": "target.md",
                    "target_hash": "target-hash",
                    "reason_probability": 0.9,
                    "destination_probability": 0.4,
                    "anchor_choice": "no_anchor",
                    "anchor_probability": 0.7,
                    "anchor_quality_probability": None,
                    "anchor": None,
                    "superseded_by": None,
                }
            ],
            "usage": {"input_tokens": 1, "output_tokens": 1},
        }
        report = Report.from_dict(raw)
        self.assertEqual(report.to_dict(), raw)

    def test_v2_report_is_rejected(self):
        with self.assertRaisesRegex(ValueError, "unsupported"):
            Report.from_dict({"schema_version": 2})

    def test_unknown_schema_is_rejected(self):
        with self.assertRaisesRegex(ValueError, "unsupported"):
            parse_review_export(
                {
                    "schema_version": 99,
                    "report_hash": "hash",
                    "decisions": [],
                    "labels": {},
                }
            )


if __name__ == "__main__":
    unittest.main()
