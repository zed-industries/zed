from pathlib import Path
import unittest

from doc_links import markdown
from doc_links.corpus import Page
from doc_links.jev import Evaluation
from doc_links.policy import Thresholds, decisions_for_source
from doc_links.retrieval import AnchorOption, DestinationCandidate


class PolicyTest(unittest.TestCase):
    def page(self, path: str, source: str) -> Page:
        return Page(
            path=Path(path),
            title=Path(path).stem,
            source=source,
            blocks=markdown.parse(source),
            existing_links=frozenset(),
        )

    def evaluation(
        self,
        source: Page,
        target: Page,
        start: int,
        text: str,
        destination_probability: float = 0.9,
        anchor_quality_probability: float = 0.9,
    ) -> Evaluation:
        block = next(
            block
            for block in source.prose_blocks
            if block.start <= start < block.end
        )
        anchor = AnchorOption(
            identifier="anchor_000",
            text=text,
            start=start,
            end=start + len(text),
            block_start=block.start,
            block_end=block.end,
            block_hash=block.content_hash,
            score=1.0,
        )
        candidate = DestinationCandidate(
            target=target,
            similarity=1.0,
            blocks=(block,),
            anchors=(anchor,),
        )
        return Evaluation(
            target=candidate,
            reason_probability=0.9,
            destination_probability=destination_probability,
            anchor_choice="anchor_000",
            anchor_probability=0.8,
            anchor_quality_probability=anchor_quality_probability,
        )

    def test_one_occurrence_selects_best_destination(self):
        source_text = "# Source\n\nUse the command palette.\n"
        source = self.page("source.md", source_text)
        start = source_text.index("command palette")
        direct = self.page("command-palette.md", "# Direct\n\nDetails.\n")
        forwarding = self.page("navigation.md", "# Navigation\n\nSee commands.\n")
        decisions = decisions_for_source(
            source,
            (
                self.evaluation(source, direct, start, "command palette", 0.9),
                self.evaluation(source, forwarding, start, "command palette", 0.6),
            ),
            Thresholds(),
        )
        queues = {item.target_path: item.queue for item in decisions}
        self.assertEqual(queues["command-palette.md"], "automatic")
        self.assertEqual(queues["navigation.md"], "rejected")

    def test_same_target_uses_first_qualified_occurrence(self):
        source_text = (
            "# Source\n\nThe command palette runs actions.\n\n"
            "Open the command palette again.\n"
        )
        source = self.page("source.md", source_text)
        first = source_text.index("command palette")
        second = source_text.rindex("command palette")
        target = self.page("command-palette.md", "# Target\n\nDetails.\n")
        decisions = decisions_for_source(
            source,
            (
                self.evaluation(
                    source,
                    target,
                    first,
                    "command palette",
                    anchor_quality_probability=0.81,
                ),
                self.evaluation(
                    source,
                    target,
                    second,
                    "command palette",
                    anchor_quality_probability=0.99,
                ),
            ),
            Thresholds(),
        )
        automatic = [item for item in decisions if item.queue == "automatic"]
        self.assertEqual(len(automatic), 1)
        self.assertEqual(automatic[0].anchor.start, first)


if __name__ == "__main__":
    unittest.main()
