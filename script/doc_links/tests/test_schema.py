import unittest

from doc_links.schema import parse_review_export


class SchemaTest(unittest.TestCase):
    def test_stale_label_is_rejected(self):
        with self.assertRaisesRegex(ValueError, "unknown decision"):
            parse_review_export(
                {
                    "schema_version": 1,
                    "report_hash": "hash",
                    "decisions": [],
                    "labels": {
                        "old": {"label": "pass", "notes": ""},
                    },
                }
            )

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
