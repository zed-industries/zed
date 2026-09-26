from pathlib import Path
import unittest

from doc_links import markdown
from doc_links.corpus import Page
from doc_links.retrieval import Index, anchor_options, earliest_occurrence


class RetrievalTest(unittest.TestCase):
    def page(self, path: str, title: str, source: str) -> Page:
        return Page(
            path=Path(path),
            title=title,
            source=source,
            blocks=markdown.parse(source),
            existing_links=frozenset(),
        )

    def test_earliest_occurrence_respects_word_boundaries(self):
        source = self.page(
            "source.md",
            "Source",
            "# Source\n\nVim modes are listed.\n\nEnable Vim mode here.\n",
        )
        occurrence = earliest_occurrence("Vim mode", source.prose_blocks)
        self.assertIsNotNone(occurrence)
        start, _, _ = occurrence
        self.assertEqual(start, source.source.index("Vim mode here"))

    def test_product_name_is_valid_anchor_boundary(self):
        source = self.page(
            "source.md",
            "Source",
            "# Source\n\nZed Agent profiles control agent behavior.\n",
        )
        target = self.page(
            "agent-profiles.md",
            "Agent Profiles",
            "# Agent Profiles\n\nConfigure profiles for Zed Agent.\n",
        )
        anchors = anchor_options(
            target,
            source.prose_blocks,
            6,
            source.prose_blocks,
        )
        self.assertIn("Zed Agent profiles", {anchor.text for anchor in anchors})

    def test_case_variants_share_one_choice(self):
        source = self.page(
            "source.md",
            "Source",
            "# Source\n\nAgent Panel opens here.\n\nThe agent panel is visible.\n",
        )
        target = self.page(
            "agent-panel.md",
            "Agent Panel",
            "# Agent Panel\n\nUse the Agent Panel.\n",
        )
        anchors = anchor_options(
            target,
            source.prose_blocks,
            10,
            source.prose_blocks,
        )
        matching = [
            anchor for anchor in anchors if anchor.text.casefold() == "agent panel"
        ]
        self.assertEqual(len(matching), 1)

    def test_relocated_anchor_block_is_supplied_to_jev(self):
        source = self.page(
            "source.md",
            "Source",
            (
                "# Source\n\nAgent Panel appears briefly.\n\n"
                "The Agent Panel contains agent tools, agent threads, and agent context.\n"
            ),
        )
        target = self.page(
            "agent-panel.md",
            "Agent Panel",
            "# Agent Panel\n\nAgent tools, threads, and context.\n",
        )
        index = Index((source, target))
        candidates = index.candidates(source, 1, 1, 1)
        self.assertEqual(len(candidates), 1)
        candidate = candidates[0]
        anchor = candidate.anchors[0]
        self.assertEqual(anchor.start, source.source.index("Agent Panel"))
        self.assertIn(anchor.block_start, {block.start for block in candidate.blocks})

    def test_anchor_options_do_not_include_trailing_period(self):
        source = self.page(
            "source.md",
            "Source",
            "# Source\n\nOpen the command palette. Save the file.\n",
        )
        target = self.page(
            "command-palette.md",
            "Command Palette",
            "# Command Palette\n\nRun commands.\n",
        )
        anchors = anchor_options(target, source.prose_blocks, 6)
        self.assertTrue(anchors)
        self.assertFalse(any(anchor.text.endswith(".") for anchor in anchors))

    def test_shortest_complete_anchor_is_preferred(self):
        source = self.page(
            "source.md",
            "Source",
            "# Source\n\nStart a review on desktop or web.\n",
        )
        target = self.page(
            "web.md",
            "Delta on the Web",
            "# Delta on the Web\n\nOpen and review threads in a browser.\n",
        )
        anchors = anchor_options(target, source.prose_blocks, 6)
        texts = {anchor.text for anchor in anchors}
        self.assertIn("web", texts)
        self.assertNotIn("desktop or web", texts)

    def test_singular_anchor_matches_plural_title(self):
        source = self.page(
            "source.md",
            "Source",
            "# Source\n\nLeave a comment on the change.\n",
        )
        target = self.page(
            "comments.md",
            "Comments",
            "# Comments\n\nComment on a specific passage.\n",
        )
        anchors = anchor_options(target, source.prose_blocks, 6)
        self.assertIn("comment", {anchor.text for anchor in anchors})

    def test_anchor_does_not_cross_sentence_punctuation(self):
        source = self.page(
            "source.md",
            "Source",
            "# Source\n\nUse a git repository. When the agent commits, review it.\n",
        )
        target = self.page(
            "git.md",
            "Delta & Git",
            "# Delta & Git\n\nConnect a git repository to Delta.\n",
        )
        anchors = anchor_options(target, source.prose_blocks, 6)
        self.assertIn("git repository", {anchor.text for anchor in anchors})
        self.assertFalse(any(". " in anchor.text for anchor in anchors))

    def test_product_name_alone_is_not_preferred(self):
        source = self.page(
            "source.md",
            "Source",
            "# Source\n\nA Delta worktree keeps the project separate in Delta.\n",
        )
        target = self.page(
            "worktrees.md",
            "Delta Worktrees",
            "# Delta Worktrees\n\nA worktree contains project files.\n",
        )
        anchors = anchor_options(target, source.prose_blocks, 6)
        self.assertEqual(anchors[0].text, "Delta worktree")
        self.assertNotIn("Delta", {anchor.text for anchor in anchors})


if __name__ == "__main__":
    unittest.main()
