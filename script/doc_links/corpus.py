from dataclasses import dataclass
import os
from pathlib import Path
from typing import Iterable

from . import markdown
from .schema import content_hash


@dataclass(frozen=True)
class Page:
    path: Path
    title: str
    source: str
    blocks: tuple[markdown.Block, ...]
    existing_links: frozenset[Path]

    @property
    def content_hash(self) -> str:
        return content_hash(self.source)

    @property
    def prose_blocks(self) -> tuple[markdown.Block, ...]:
        return tuple(block for block in self.blocks if block.eligible_for_anchor)

    @property
    def overview(self) -> str:
        return " ".join(block.visible_text for block in self.prose_blocks[:2])[:1000]




def page_kind(path: Path) -> str:
    value = str(path)
    if value.startswith("reference/"):
        return "reference"
    if value.startswith("migrate/"):
        return "migration guide"
    if value == "extensions/installing-extensions.md" or "install" in path.stem:
        return "installation guide"
    if value.startswith("extensions/"):
        return "extension authoring guide"
    if path.stem in {"overview", "languages", "getting-started"}:
        return "overview"
    if value.startswith("languages/"):
        return "language guide"
    return "guide"

def resolve_link(source_path: Path, destination: str, docs_dir: Path) -> Path | None:
    link = destination.strip("<>").split("#", 1)[0].split("?", 1)[0]
    if not link or link.startswith(("mailto:", "tel:")):
        return None
    if link.startswith(("https://zed.dev/docs/", "http://zed.dev/docs/")):
        candidate = docs_dir / link.split("zed.dev/docs/", 1)[1].rstrip("/")
    elif "://" in link:
        return None
    elif link.startswith("/docs/"):
        candidate = docs_dir / link[len("/docs/") :].rstrip("/")
    elif link.startswith("/"):
        return None
    else:
        candidate = docs_dir / source_path.parent / link
    if candidate.suffix == ".html":
        candidate = candidate.with_suffix(".md")
    elif not candidate.suffix:
        candidate = candidate.with_suffix(".md")
    try:
        return candidate.resolve().relative_to(docs_dir.resolve())
    except ValueError:
        return None


def published_paths(docs_dir: Path) -> tuple[Path, ...]:
    summary_path = docs_dir / "SUMMARY.md"
    if not summary_path.is_file():
        return tuple(
            sorted(
                path.relative_to(docs_dir)
                for path in docs_dir.rglob("*.md")
                if path.name != "SUMMARY.md"
            )
        )
    summary = summary_path.read_text(encoding="utf-8")
    paths = {
        resolved
        for span in markdown.links(summary)
        if span.destination
        and (resolved := resolve_link(Path("SUMMARY.md"), span.destination, docs_dir))
        and (docs_dir / resolved).is_file()
    }
    return tuple(sorted(paths))


def load_pages(docs_dir: Path) -> tuple[Page, ...]:
    paths = published_paths(docs_dir)
    known = set(paths)
    pages = []
    for relative_path in paths:
        source = (docs_dir / relative_path).read_text(encoding="utf-8")
        blocks = markdown.parse(source)
        existing = {
            resolved
            for span in markdown.links(source)
            if span.destination
            and (resolved := resolve_link(relative_path, span.destination, docs_dir))
            in known
        }
        pages.append(
            Page(
                path=relative_path,
                title=markdown.title(blocks)
                or relative_path.stem.replace("-", " ").title(),
                source=source,
                blocks=blocks,
                existing_links=frozenset(existing),
            )
        )
    return tuple(pages)


def relative_link(source_path: Path, target_path: Path) -> str:
    relative = os.path.relpath(target_path, source_path.parent)
    return relative if relative.startswith(".") else f"./{relative}"


def select_pages(
    pages: Iterable[Page], patterns: tuple[str, ...]
) -> tuple[Page, ...]:
    def matches(path: Path, pattern: str) -> bool:
        return str(path) == pattern if "/" not in pattern else path.match(pattern)

    selected = []
    for page in pages:
        if page.prose_blocks and (
            not patterns or any(matches(page.path, pattern) for pattern in patterns)
        ):
            selected.append(page)
    return tuple(selected)
