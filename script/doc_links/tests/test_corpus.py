from pathlib import Path
import unittest

from doc_links.corpus import Page, select_pages
from doc_links import markdown


class CorpusTest(unittest.TestCase):
    def page(self, path: str) -> Page:
        source = "# Page\n\nProse.\n"
        return Page(
            path=Path(path),
            title="Page",
            source=source,
            blocks=markdown.parse(source),
            existing_links=frozenset(),
        )

    def test_source_glob_does_not_cross_directories(self):
        pages = (
            self.page("ai/direct.md"),
            self.page("ai/nested/deep.md"),
        )
        selected = select_pages(pages, ("ai/*.md",))
        self.assertEqual([str(page.path) for page in selected], ["ai/direct.md"])


if __name__ == "__main__":
    unittest.main()
