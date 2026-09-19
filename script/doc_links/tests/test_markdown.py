from pathlib import Path
import tempfile
import unittest

from doc_links import markdown
from doc_links.corpus import Page
from doc_links.retrieval import anchor_options


class MarkdownTest(unittest.TestCase):
    def test_nested_shorter_fence_stays_code(self):
        source = """# Example

````md
```python
def hidden():
    return True
```
````

Visible prose.
"""
        blocks = markdown.parse(source)
        prose = [block.visible_text for block in blocks if block.eligible_for_anchor]
        self.assertEqual(prose, ["Visible prose."])
        self.assertEqual(sum(block.kind == "code" for block in blocks), 1)

    def test_ordered_list_items_are_separate_blocks(self):
        blocks = markdown.parse("1. First item\n2. Second item\n3. Third item\n")
        self.assertEqual([block.kind for block in blocks], ["list_item"] * 3)
        self.assertEqual(
            [block.visible_text for block in blocks],
            ["First item", "Second item", "Third item"],
        )

    def test_anchor_options_stay_on_one_line_and_exclude_markup(self):
        source_text = """Use the command
palette for actions.

The [command palette](./existing.md) is linked.

Run `command palette` in code.
"""
        target_text = "# Command Palette\n\nRun actions and search commands.\n"
        source_blocks = markdown.parse(source_text)
        target_blocks = markdown.parse(target_text)
        source = Page(
            path=Path("source.md"),
            title="Source",
            source=source_text,
            blocks=source_blocks,
            existing_links=frozenset(),
        )
        target = Page(
            path=Path("target.md"),
            title="Command Palette",
            source=target_text,
            blocks=target_blocks,
            existing_links=frozenset(),
        )
        anchors = anchor_options(target, source.prose_blocks, 10)
        self.assertFalse(any("\n" in anchor.text for anchor in anchors))
        self.assertFalse(any(anchor.text == "command palette" for anchor in anchors))

    def test_anchor_options_exclude_emphasis_and_list_markers(self):
        source_text = "1. In the **External Agents** view, open agent settings.\n"
        target_text = "# External Agents\n\nConfigure external agent settings.\n"
        source = Page(
            path=Path("source.md"),
            title="Source",
            source=source_text,
            blocks=markdown.parse(source_text),
            existing_links=frozenset(),
        )
        target = Page(
            path=Path("target.md"),
            title="External Agents",
            source=target_text,
            blocks=markdown.parse(target_text),
            existing_links=frozenset(),
        )
        anchors = anchor_options(target, source.prose_blocks, 20)
        self.assertIn("External Agents", {anchor.text for anchor in anchors})
        self.assertTrue(
            all(not any(marker in anchor.text for marker in ("*", "_", "~")) for anchor in anchors)
        )
        self.assertTrue(all(not anchor.text.startswith("1.") for anchor in anchors))

    def test_heading_table_html_and_code_are_not_anchor_blocks(self):
        source = """# Heading

| Name | Value |
| ---- | ----- |
| Agent Panel | Yes |

<div>Agent Panel</div>

```text
Agent Panel
```

Use the Agent Panel for agent work.
"""
        blocks = markdown.parse(source)
        eligible = [block.visible_text for block in blocks if block.eligible_for_anchor]
        self.assertEqual(eligible, ["Use the Agent Panel for agent work."])


if __name__ == "__main__":
    unittest.main()
