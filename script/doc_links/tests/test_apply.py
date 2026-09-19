from dataclasses import replace
import json
from pathlib import Path
import tempfile
import unittest

from doc_links import SCHEMA_VERSION, markdown
from doc_links.apply import apply_plan, build_plan
from doc_links.schema import Anchor, Decision, content_hash


class ApplyTest(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory()
        self.docs = Path(self.temporary.name)
        (self.docs / "target.md").write_text(
            "# Target\n\nTarget details.\n",
            encoding="utf-8",
        )

    def tearDown(self):
        self.temporary.cleanup()

    def decision(self, source: str, phrase: str, occurrence: int = 0) -> Decision:
        blocks = markdown.parse(source)
        block = next(
            block
            for block in blocks
            if block.eligible_for_anchor and phrase in block.source
        )
        positions = []
        start = 0
        while True:
            found = block.source.find(phrase, start)
            if found < 0:
                break
            positions.append(found)
            start = found + 1
        local_start = positions[occurrence]
        absolute_start = block.start + local_start
        anchor = Anchor(
            text=phrase,
            start=absolute_start,
            end=absolute_start + len(phrase),
            block_start=block.start,
            block_end=block.end,
            block_hash=block.content_hash,
            relative_target="./target.md",
        )
        return Decision(
            identifier=f"decision-{occurrence}",
            queue="automatic",
            source_path="source.md",
            target_path="target.md",
            target_hash=content_hash(
                (self.docs / "target.md").read_text(encoding="utf-8")
            ),
            reason_probability=0.9,
            anchor_choice="anchor_000",
            anchor_probability=0.8,
            anchor=anchor,
        )

    def write_export(self, decisions: list[Decision]) -> Path:
        path = self.docs / "review.json"
        path.write_text(
            json.dumps(
                {
                    "schema_version": SCHEMA_VERSION,
                    "report_hash": "report-hash",
                    "decisions": [decision.to_dict() for decision in decisions],
                    "labels": {
                        decision.identifier: {"label": "pass", "notes": ""}
                        for decision in decisions
                    },
                }
            ),
            encoding="utf-8",
        )
        return path

    def test_duplicate_phrase_uses_reviewed_occurrence(self):
        source = (
            "# Source\n\nUse the command palette for actions. "
            "The command palette lists commands.\n"
        )
        (self.docs / "source.md").write_text(source, encoding="utf-8")
        decision = self.decision(source, "command palette", occurrence=0)
        plan = build_plan(self.write_export([decision]), self.docs)
        apply_plan(plan)
        updated = (self.docs / "source.md").read_text(encoding="utf-8")
        self.assertIn("the [command palette](./target.md) for actions", updated)
        self.assertIn("The command palette lists commands", updated)

    def test_unrelated_edit_outside_block_is_allowed(self):
        source = "# Source\n\nUse the command palette for actions.\n"
        decision = self.decision(source, "command palette")
        updated = "Intro added later.\n\n" + source
        (self.docs / "source.md").write_text(updated, encoding="utf-8")
        plan = build_plan(self.write_export([decision]), self.docs)
        apply_plan(plan)
        result = (self.docs / "source.md").read_text(encoding="utf-8")
        self.assertTrue(result.startswith("Intro added later."))
        self.assertIn("[command palette](./target.md)", result)

    def test_changed_block_aborts_complete_batch(self):
        first = "# Source\n\nUse the command palette for actions.\n"
        second = "# Other\n\nUse the command palette for navigation.\n"
        (self.docs / "source.md").write_text(first, encoding="utf-8")
        (self.docs / "other.md").write_text(second, encoding="utf-8")
        first_decision = self.decision(first, "command palette")
        second_decision = replace(
            self.decision(second, "command palette"),
            identifier="other",
            source_path="other.md",
        )
        (self.docs / "source.md").write_text(
            first.replace("for actions", "to run actions"),
            encoding="utf-8",
        )
        export = self.write_export([first_decision, second_decision])
        with self.assertRaisesRegex(ValueError, "source block changed"):
            build_plan(export, self.docs)
        self.assertNotIn(
            "[command palette]",
            (self.docs / "other.md").read_text(encoding="utf-8"),
        )

    def test_multiple_items_in_one_file_apply_by_offset(self):
        source = "# Source\n\nUse the command palette and Agent Panel together.\n"
        (self.docs / "source.md").write_text(source, encoding="utf-8")
        (self.docs / "agent.md").write_text("# Agent Panel\n", encoding="utf-8")
        command = self.decision(source, "command palette")
        agent = replace(
            self.decision(source, "Agent Panel"),
            identifier="agent",
            target_path="agent.md",
            target_hash=content_hash(
                (self.docs / "agent.md").read_text(encoding="utf-8")
            ),
            anchor=replace(
                self.decision(source, "Agent Panel").anchor,
                relative_target="./agent.md",
            ),
        )
        plan = build_plan(self.write_export([command, agent]), self.docs)
        apply_plan(plan)
        result = (self.docs / "source.md").read_text(encoding="utf-8")
        self.assertIn("[command palette](./target.md)", result)
        self.assertIn("[Agent Panel](./agent.md)", result)

    def test_overlapping_anchors_are_rejected(self):
        source = "# Source\n\nUse the command palette.\n"
        (self.docs / "source.md").write_text(source, encoding="utf-8")
        command = self.decision(source, "command palette")
        palette = replace(
            self.decision(source, "palette"),
            identifier="palette",
        )
        with self.assertRaisesRegex(ValueError, "overlap"):
            build_plan(self.write_export([command, palette]), self.docs)

    def test_relative_target_mismatch_is_rejected(self):
        source = "# Source\n\nUse the command palette.\n"
        (self.docs / "source.md").write_text(source, encoding="utf-8")
        decision = self.decision(source, "command palette")
        decision = replace(
            decision,
            anchor=replace(decision.anchor, relative_target="./wrong.md"),
        )
        with self.assertRaisesRegex(ValueError, "invalid target"):
            build_plan(self.write_export([decision]), self.docs)

    def test_identical_source_blocks_are_ambiguous(self):
        paragraph = "Use the command palette."
        source = f"# Source\n\n{paragraph}\n\n{paragraph}\n"
        (self.docs / "source.md").write_text(source, encoding="utf-8")
        decision = self.decision(source, "command palette")
        with self.assertRaisesRegex(ValueError, "ambiguous"):
            build_plan(self.write_export([decision]), self.docs)

    def test_linked_occurrence_is_rejected(self):
        source = "# Source\n\nUse the [command palette](./old.md).\n"
        (self.docs / "source.md").write_text(source, encoding="utf-8")
        decision = self.decision(source, "command palette")
        with self.assertRaisesRegex(ValueError, "source block changed"):
            build_plan(self.write_export([decision]), self.docs)

    def test_heading_occurrence_is_rejected(self):
        source = "# Command Palette\n\nOther prose.\n"
        (self.docs / "source.md").write_text(source, encoding="utf-8")
        block = markdown.parse(source)[0]
        start = source.index("Command Palette")
        anchor = Anchor(
            text="Command Palette",
            start=start,
            end=start + len("Command Palette"),
            block_start=block.start,
            block_end=block.end,
            block_hash=block.content_hash,
            relative_target="./target.md",
        )
        decision = Decision(
            identifier="heading",
            queue="automatic",
            source_path="source.md",
            target_path="target.md",
            target_hash=content_hash(
                (self.docs / "target.md").read_text(encoding="utf-8")
            ),
            reason_probability=0.9,
            anchor_choice="anchor_000",
            anchor_probability=0.8,
            anchor=anchor,
        )
        with self.assertRaisesRegex(ValueError, "source block changed"):
            build_plan(self.write_export([decision]), self.docs)

    def test_path_traversal_is_rejected(self):
        source = "# Source\n\nUse the command palette.\n"
        (self.docs / "source.md").write_text(source, encoding="utf-8")
        decision = replace(
            self.decision(source, "command palette"),
            source_path="../outside.md",
        )
        with self.assertRaisesRegex(ValueError, "escapes documentation directory"):
            build_plan(self.write_export([decision]), self.docs)

    def test_changed_target_is_rejected(self):
        source = "# Source\n\nUse the command palette.\n"
        (self.docs / "source.md").write_text(source, encoding="utf-8")
        decision = self.decision(source, "command palette")
        (self.docs / "target.md").write_text(
            "# Target\n\nChanged details.\n",
            encoding="utf-8",
        )
        with self.assertRaisesRegex(ValueError, "target page changed"):
            build_plan(self.write_export([decision]), self.docs)

    def test_missing_target_is_rejected(self):
        source = "# Source\n\nUse the command palette.\n"
        (self.docs / "source.md").write_text(source, encoding="utf-8")
        decision = replace(
            self.decision(source, "command palette"),
            target_path="missing.md",
        )
        with self.assertRaisesRegex(ValueError, "does not exist"):
            build_plan(self.write_export([decision]), self.docs)


if __name__ == "__main__":
    unittest.main()
