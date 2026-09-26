from dataclasses import dataclass
import re
from typing import Iterator

from .schema import content_hash

FENCE_PATTERN = re.compile(r"^\s*(`{3,}|~{3,})")
HEADING_PATTERN = re.compile(r"^(#{1,6})\s+(.+?)(?:\s+\{#[^}]+\})?\s*$")
LIST_PATTERN = re.compile(r"^\s*(?:[-+*]|\d+[.)])\s+")
TABLE_DIVIDER_PATTERN = re.compile(
    r"^\s*\|?\s*:?-{3,}:?\s*(?:\|\s*:?-{3,}:?\s*)+\|?\s*$"
)
LINK_PATTERN = re.compile(r"!?\[([^\]]*)\]\(([^)\s]+)(?:\s+[^)]*)?\)")
CODE_PATTERN = re.compile(r"(`+)(.+?)\1")
SPECIAL_PATTERN = re.compile(r"\{#[^}]+\}|https?://\S+")
EMPHASIS_PATTERN = re.compile(r"[*_~]{1,3}")


@dataclass(frozen=True)
class Span:
    start: int
    end: int
    kind: str
    destination: str | None = None


@dataclass(frozen=True)
class Block:
    kind: str
    start: int
    end: int
    source: str
    excluded: tuple[Span, ...]

    @property
    def content_hash(self) -> str:
        return content_hash(self.source)

    @property
    def eligible_for_anchor(self) -> bool:
        return self.kind in {"paragraph", "list_item"}

    @property
    def visible_text(self) -> str:
        characters = list(self.source)
        for span in self.excluded:
            local_start = span.start - self.start
            local_end = span.end - self.start
            for index in range(max(0, local_start), min(len(characters), local_end)):
                characters[index] = " "
        text = "".join(characters)
        text = re.sub(r"[\n\r\t]+", " ", text)
        text = re.sub(r"[*_>#|]", " ", text)
        return re.sub(r"\s+", " ", text).strip()

    def line_ranges(self) -> Iterator[tuple[int, int]]:
        position = self.start
        for line in self.source.splitlines(keepends=True):
            content_end = position + len(line.rstrip("\r\n"))
            yield position, content_end
            position += len(line)
        if not self.source:
            yield self.start, self.end


def inline_spans(source: str, offset: int = 0) -> tuple[Span, ...]:
    spans = []
    for match in LINK_PATTERN.finditer(source):
        spans.append(
            Span(
                start=offset + match.start(),
                end=offset + match.end(),
                kind="link",
                destination=match.group(2),
            )
        )
    for pattern, kind in (
        (CODE_PATTERN, "code"),
        (SPECIAL_PATTERN, "special"),
        (EMPHASIS_PATTERN, "emphasis"),
    ):
        for match in pattern.finditer(source):
            spans.append(
                Span(
                    start=offset + match.start(),
                    end=offset + match.end(),
                    kind=kind,
                )
            )
    spans.sort(key=lambda span: (span.start, -(span.end - span.start)))
    merged = []
    for span in spans:
        if merged and span.start < merged[-1].end:
            continue
        merged.append(span)
    return tuple(merged)


def links(source: str, offset: int = 0) -> tuple[Span, ...]:
    return tuple(span for span in inline_spans(source, offset) if span.kind == "link")


def line_offsets(source: str) -> list[tuple[int, int, str]]:
    lines = []
    position = 0
    for line in source.splitlines(keepends=True):
        lines.append((position, position + len(line), line))
        position += len(line)
    if not source or position < len(source):
        lines.append((position, len(source), source[position:]))
    return lines


def is_table_start(lines: list[tuple[int, int, str]], index: int) -> bool:
    if index + 1 >= len(lines) or "|" not in lines[index][2]:
        return False
    return TABLE_DIVIDER_PATTERN.match(lines[index + 1][2].rstrip("\r\n")) is not None


def starts_special_block(lines: list[tuple[int, int, str]], index: int) -> bool:
    text = lines[index][2].rstrip("\r\n")
    stripped = text.strip()
    return bool(
        not stripped
        or FENCE_PATTERN.match(text)
        or HEADING_PATTERN.match(text)
        or LIST_PATTERN.match(text)
        or stripped.startswith("<")
        or is_table_start(lines, index)
    )


def make_block(kind: str, source: str, start: int, end: int) -> Block:
    block_source = source[start:end]
    excluded = (
        ()
        if kind in {"code", "front_matter", "html", "table"}
        else inline_spans(block_source, start)
    )
    if kind == "list_item":
        marker = LIST_PATTERN.match(block_source)
        if marker:
            excluded = tuple(
                sorted(
                    (*excluded, Span(start, start + marker.end(), "list_marker")),
                    key=lambda span: (span.start, span.end),
                )
            )
    return Block(
        kind=kind,
        start=start,
        end=end,
        source=block_source,
        excluded=excluded,
    )


def parse(source: str) -> tuple[Block, ...]:
    lines = line_offsets(source)
    blocks = []
    index = 0

    if lines and lines[0][2].rstrip("\r\n") == "---":
        end_index = 1
        while end_index < len(lines):
            if lines[end_index][2].rstrip("\r\n") == "---":
                end_index += 1
                break
            end_index += 1
        end = lines[end_index - 1][1] if end_index else 0
        blocks.append(make_block("front_matter", source, 0, end))
        index = end_index

    while index < len(lines):
        start, end, line = lines[index]
        text = line.rstrip("\r\n")
        stripped = text.strip()
        if not stripped:
            index += 1
            continue

        fence = FENCE_PATTERN.match(text)
        if fence:
            marker = fence.group(1)
            character = marker[0]
            minimum_length = len(marker)
            end_index = index + 1
            while end_index < len(lines):
                closing = FENCE_PATTERN.match(lines[end_index][2].rstrip("\r\n"))
                if (
                    closing
                    and closing.group(1)[0] == character
                    and len(closing.group(1)) >= minimum_length
                ):
                    end_index += 1
                    break
                end_index += 1
            block_end = lines[end_index - 1][1]
            blocks.append(make_block("code", source, start, block_end))
            index = end_index
            continue

        if HEADING_PATTERN.match(text):
            blocks.append(make_block("heading", source, start, end))
            index += 1
            continue

        if is_table_start(lines, index):
            end_index = index + 2
            while end_index < len(lines):
                row = lines[end_index][2].strip()
                if not row or "|" not in row:
                    break
                end_index += 1
            blocks.append(
                make_block("table", source, start, lines[end_index - 1][1])
            )
            index = end_index
            continue

        if stripped.startswith("<"):
            end_index = index + 1
            while end_index < len(lines) and lines[end_index][2].strip():
                end_index += 1
            blocks.append(
                make_block("html", source, start, lines[end_index - 1][1])
            )
            index = end_index
            continue

        if LIST_PATTERN.match(text):
            end_index = index + 1
            while end_index < len(lines):
                next_text = lines[end_index][2].rstrip("\r\n")
                if not next_text.strip() or LIST_PATTERN.match(next_text):
                    break
                if starts_special_block(lines, end_index):
                    break
                end_index += 1
            blocks.append(
                make_block("list_item", source, start, lines[end_index - 1][1])
            )
            index = end_index
            continue

        end_index = index + 1
        while end_index < len(lines) and not starts_special_block(lines, end_index):
            end_index += 1
        blocks.append(
            make_block("paragraph", source, start, lines[end_index - 1][1])
        )
        index = end_index

    return tuple(blocks)


def title(blocks: tuple[Block, ...]) -> str | None:
    for block in blocks:
        if block.kind == "front_matter":
            match = re.search(r"^title:\s*(.+?)\s*$", block.source, re.MULTILINE)
            if match:
                return match.group(1).strip("\"'")
        if block.kind == "heading":
            match = HEADING_PATTERN.match(block.source.rstrip("\r\n"))
            if match and len(match.group(1)) == 1:
                return re.sub(r"[`*_]", "", match.group(2)).strip()
    return None
