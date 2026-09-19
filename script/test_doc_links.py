#!/usr/bin/env python3

import json
from pathlib import Path
import runpy
import tempfile
import unittest


SCRIPT_DIR = Path(__file__).parent
AUDIT = runpy.run_path(str(SCRIPT_DIR / "audit-doc-links"))
REVIEW = runpy.run_path(str(SCRIPT_DIR / "review-doc-links"))


class AuditDocLinksTest(unittest.TestCase):
    def setUp(self):
        self.temporary_directory = tempfile.TemporaryDirectory()
        self.docs_dir = Path(self.temporary_directory.name)
        (self.docs_dir / "guides").mkdir()
        (self.docs_dir / "SUMMARY.md").write_text(
            "\n".join(
                (
                    "# Summary",
                    "",
                    "- [Source](./source.md)",
                    "- [Command Palette](./command-palette.md)",
                    "- [Settings](./guides/settings.md)",
                )
            ),
            encoding="utf-8",
        )
        (self.docs_dir / "source.md").write_text(
            "\n".join(
                (
                    "# Source",
                    "",
                    "Use the command palette to run actions.",
                    "",
                    "The [Settings Editor](./guides/settings.md) changes preferences.",
                    "",
                    "Run `zed --version` to print the version.",
                )
            ),
            encoding="utf-8",
        )
        (self.docs_dir / "command-palette.md").write_text(
            "# Command Palette\n\nRun actions and search for commands.\n",
            encoding="utf-8",
        )
        (self.docs_dir / "guides" / "settings.md").write_text(
            "# Settings\n\nConfigure Zed preferences.\n",
            encoding="utf-8",
        )

    def tearDown(self):
        self.temporary_directory.cleanup()

    def test_loads_only_published_pages_and_existing_links(self):
        (self.docs_dir / "draft.md").write_text(
            "# Draft\n\nNot published.\n",
            encoding="utf-8",
        )
        pages = AUDIT["load_pages"](self.docs_dir)
        self.assertEqual(
            {str(page.path) for page in pages},
            {"source.md", "command-palette.md", "guides/settings.md"},
        )
        source = next(page for page in pages if str(page.path) == "source.md")
        self.assertEqual(
            source.existing_links,
            frozenset({Path("guides/settings.md")}),
        )

    def test_anchor_candidates_exclude_links_and_inline_code(self):
        pages = AUDIT["load_pages"](self.docs_dir)
        source = next(page for page in pages if str(page.path) == "source.md")
        destination = next(
            page for page in pages if str(page.path) == "command-palette.md"
        )
        candidate = AUDIT["Candidate"](
            page=destination,
            similarity=1.0,
            passages=source.passages,
        )
        anchors = AUDIT["build_anchor_candidates"](candidate, 12)
        phrases = {anchor.text for anchor in anchors}
        self.assertIn("command palette", phrases)
        self.assertNotIn("Settings Editor", phrases)
        self.assertFalse(any("zed --version" in phrase for phrase in phrases))
        self.assertTrue(all(len(anchor.text.split()) <= 6 for anchor in anchors))

    def test_request_has_reason_and_anchor_decisions(self):
        pages = AUDIT["load_pages"](self.docs_dir)
        source = next(page for page in pages if str(page.path) == "source.md")
        destination = next(
            page for page in pages if str(page.path) == "command-palette.md"
        )
        candidate = AUDIT["Candidate"](
            page=destination,
            similarity=1.0,
            passages=source.passages,
        )
        payload, _, _ = AUDIT["build_request"](
            source,
            [candidate],
            "jev-latest",
            12,
        )
        self.assertEqual(payload["questions"]["reason_target_000"]["type"], "noul")
        anchor = payload["questions"]["anchor_target_000"]
        self.assertEqual(anchor["type"], "choice")
        self.assertIn("other_anchor", anchor["criteria"])
        self.assertIn("no_anchor", anchor["criteria"])

    def test_relative_doc_links(self):
        relative_doc_link = AUDIT["relative_doc_link"]
        self.assertEqual(
            relative_doc_link(
                Path("guides/source.md"),
                Path("reference/settings.md"),
            ),
            "../reference/settings.md",
        )
        self.assertEqual(
            relative_doc_link(
                Path("source.md"),
                Path("command-palette.md"),
            ),
            "./command-palette.md",
        )


class ReviewDocLinksTest(unittest.TestCase):
    def setUp(self):
        self.temporary_directory = tempfile.TemporaryDirectory()
        self.root = Path(self.temporary_directory.name)
        self.docs_dir = self.root / "docs"
        self.docs_dir.mkdir()
        (self.docs_dir / "source.md").write_text(
            "# Source\n\nUse the command palette to run actions.\n",
            encoding="utf-8",
        )
        (self.docs_dir / "target.md").write_text(
            "# Command Palette\n\nRun actions.\n",
            encoding="utf-8",
        )

    def tearDown(self):
        self.temporary_directory.cleanup()

    def exported_review(self):
        source = (self.docs_dir / "source.md").read_text(encoding="utf-8")
        item = {
            "id": "automatic:test",
            "queue": "automatic",
            "source_path": "source.md",
            "target_path": "target.md",
            "relative_target": "./target.md",
            "source_markdown": source,
            "selected_anchor": {
                "text": "command palette",
                "context": "Use the command palette to run actions.",
            },
            "context": "Use the command palette to run actions.",
        }
        return {
            "labels": {
                item["id"]: {
                    "label": "pass",
                    "notes": "",
                }
            },
            "approved": [
                {
                    "item": item,
                    "review": {
                        "label": "pass",
                        "notes": "",
                    },
                }
            ],
        }

    def test_apply_export_is_dry_run_by_default(self):
        export_path = self.root / "review.json"
        export_path.write_text(
            json.dumps(self.exported_review()),
            encoding="utf-8",
        )
        REVIEW["apply_export"](
            export_path,
            self.docs_dir,
            False,
            False,
        )
        source = (self.docs_dir / "source.md").read_text(encoding="utf-8")
        self.assertNotIn("[command palette]", source)

    def test_apply_export_writes_only_approved_anchor(self):
        export_path = self.root / "review.json"
        export_path.write_text(
            json.dumps(self.exported_review()),
            encoding="utf-8",
        )
        REVIEW["apply_export"](
            export_path,
            self.docs_dir,
            True,
            False,
        )
        source = (self.docs_dir / "source.md").read_text(encoding="utf-8")
        self.assertIn("[command palette](./target.md)", source)

    def test_apply_export_rejects_changed_source(self):
        export_path = self.root / "review.json"
        export_path.write_text(
            json.dumps(self.exported_review()),
            encoding="utf-8",
        )
        (self.docs_dir / "source.md").write_text(
            "# Source\n\nThe command palette runs actions.\n",
            encoding="utf-8",
        )
        with self.assertRaisesRegex(RuntimeError, "changed after review"):
            REVIEW["apply_export"](
                export_path,
                self.docs_dir,
                False,
                False,
            )


if __name__ == "__main__":
    unittest.main()
