from collections import Counter
from dataclasses import dataclass
import math
import re

from .corpus import Page
from .markdown import Block

TOKEN_PATTERN = re.compile(r"[a-z][a-z0-9+#.-]{1,}")
WORD_PATTERN = re.compile(r"[A-Za-z0-9][A-Za-z0-9+#.'’/-]*")
RETRIEVAL_STOP_WORDS = {
    "about", "after", "again", "also", "and", "are", "because", "before",
    "being", "can", "code", "does", "docs", "documentation", "each", "for",
    "from", "have", "how", "into", "linux", "macos", "more", "not", "only",
    "page", "section", "that", "the", "their", "then", "these", "this",
    "through", "use", "using", "when", "where", "which", "will", "windows",
    "with", "you", "your", "zed",
}
BOUNDARY_STOP_WORDS = RETRIEVAL_STOP_WORDS - {
    "zed",
    "windows",
    "linux",
    "macos",
}


@dataclass(frozen=True)
class AnchorOption:
    identifier: str
    text: str
    start: int
    end: int
    block_start: int
    block_end: int
    block_hash: str
    score: float


@dataclass(frozen=True)
class DestinationCandidate:
    target: Page
    similarity: float
    blocks: tuple[Block, ...]
    anchors: tuple[AnchorOption, ...]


def tokenize(text: str) -> Counter[str]:
    return Counter(
        token
        for token in TOKEN_PATTERN.findall(text.lower())
        if token not in RETRIEVAL_STOP_WORDS
    )


def inverse_document_frequencies(pages: tuple[Page, ...]) -> dict[str, float]:
    frequencies = Counter(
        token
        for page in pages
        for token in tokenize(page.title + " " + page.overview)
    )
    return {
        token: math.log((len(pages) + 1) / (frequency + 1)) + 1
        for token, frequency in frequencies.items()
    }


def vector(tokens: Counter[str], inverse: dict[str, float]) -> dict[str, float]:
    return {
        token: (1 + math.log(count)) * inverse.get(token, 1)
        for token, count in tokens.items()
    }


def cosine(left: dict[str, float], right: dict[str, float]) -> float:
    common = left.keys() & right.keys()
    numerator = sum(left[token] * right[token] for token in common)
    left_norm = math.sqrt(sum(value * value for value in left.values()))
    right_norm = math.sqrt(sum(value * value for value in right.values()))
    return numerator / (left_norm * right_norm) if left_norm and right_norm else 0


def page_text(page: Page) -> str:
    return " ".join(
        (page.title, *(block.visible_text for block in page.prose_blocks))
    )


def top_blocks(
    source: Page,
    target: Page,
    inverse: dict[str, float],
    count: int,
) -> tuple[Block, ...]:
    target_vector = vector(tokenize(page_text(target)), inverse)
    scored = [
        (
            cosine(vector(tokenize(block.visible_text), inverse), target_vector),
            block,
        )
        for block in source.prose_blocks
    ]
    scored.sort(key=lambda item: (-item[0], item[1].start))
    return tuple(block for _, block in scored[:count])


def overlaps_excluded(block: Block, start: int, end: int) -> bool:
    return any(start < span.end and end > span.start for span in block.excluded)


def earliest_occurrence(
    phrase: str,
    blocks: tuple[Block, ...],
) -> tuple[int, int, Block] | None:
    pattern = re.compile(
        rf"(?<![A-Za-z0-9]){re.escape(phrase)}(?![A-Za-z0-9])"
    )
    for block in blocks:
        for match in pattern.finditer(block.source):
            start = block.start + match.start()
            end = block.start + match.end()
            if not overlaps_excluded(block, start, end):
                return start, end, block
    return None


def anchor_options(
    target: Page,
    blocks: tuple[Block, ...],
    count: int,
    all_blocks: tuple[Block, ...] | None = None,
) -> tuple[AnchorOption, ...]:
    primary_tokens = set(tokenize(target.title))
    secondary_tokens = set(tokenize(target.overview))
    candidates = []

    for block_rank, block in enumerate(blocks):
        for line_start, line_end in block.line_ranges():
            line = block.source[
                line_start - block.start : line_end - block.start
            ]
            words = list(WORD_PATTERN.finditer(line))
            for first_index, first_word in enumerate(words):
                for word_count in range(1, 7):
                    last_index = first_index + word_count
                    if last_index > len(words):
                        break
                    last_word = words[last_index - 1]
                    start = line_start + first_word.start()
                    end = line_start + last_word.end()
                    if overlaps_excluded(block, start, end):
                        continue
                    phrase = block.source[
                        start - block.start : end - block.start
                    ]
                    if "," in phrase or ";" in phrase:
                        continue
                    phrase_tokens = set(tokenize(phrase))
                    primary_overlap = phrase_tokens & primary_tokens
                    secondary_overlap = phrase_tokens & secondary_tokens
                    if not primary_overlap and len(secondary_overlap) < 2:
                        continue
                    first_token = first_word.group().lower().strip(".'’/-")
                    last_token = last_word.group().lower().strip(".'’/-")
                    if (
                        not first_token
                        or not last_token
                        or first_token in BOUNDARY_STOP_WORDS
                        or last_token in BOUNDARY_STOP_WORDS
                    ):
                        continue
                    if word_count == 1 and (
                        first_token not in primary_tokens or len(first_token) < 4
                    ):
                        continue
                    score = (
                        len(primary_overlap) * 8
                        + len(secondary_overlap)
                        - abs(word_count - 3) * 0.1
                        - block_rank * 0.05
                    )
                    if phrase.casefold() == target.title.casefold():
                        score += 8
                    if (
                        phrase.casefold().startswith("zed ")
                        and target.title.casefold() in phrase.casefold()
                    ):
                        score += 12
                    candidates.append((score, start, end, phrase, block))

    candidates.sort(
        key=lambda item: (-item[0], len(item[3].split()), item[1])
    )
    selected = []
    seen_phrases = set()
    search_blocks = all_blocks or blocks
    for score, start, end, phrase, block in candidates:
        phrase_key = phrase.casefold()
        if phrase_key in seen_phrases:
            continue
        earliest = earliest_occurrence(phrase, search_blocks)
        if earliest:
            start, end, block = earliest
        overlaps_selected = any(
            start < option.end
            and end > option.start
            and block.start == option.block_start
            for option in selected
        )
        if overlaps_selected:
            continue
        seen_phrases.add(phrase_key)
        selected.append(
            AnchorOption(
                identifier=f"anchor_{len(selected):03d}",
                text=phrase,
                start=start,
                end=end,
                block_start=block.start,
                block_end=block.end,
                block_hash=block.content_hash,
                score=score,
            )
        )
        if len(selected) == count:
            break
    return tuple(selected)


class Index:
    def __init__(self, pages: tuple[Page, ...]):
        self.pages = pages
        self.inverse = inverse_document_frequencies(pages)
        self.vectors = {
            page.path: vector(tokenize(page_text(page)), self.inverse)
            for page in pages
        }

    def candidates(
        self,
        source: Page,
        candidate_count: int,
        block_count: int,
        anchor_count: int,
    ) -> tuple[DestinationCandidate, ...]:
        destinations = []
        for target in self.pages:
            if target.path == source.path or target.path in source.existing_links:
                continue
            similarity = cosine(
                self.vectors[source.path],
                self.vectors[target.path],
            )
            destinations.append((similarity, target))
        destinations.sort(key=lambda item: (-item[0], str(item[1].path)))

        result = []
        for similarity, target in destinations:
            blocks = top_blocks(source, target, self.inverse, block_count)
            anchors = anchor_options(
                target,
                blocks,
                anchor_count,
                source.prose_blocks,
            )
            if len(anchors) != anchor_count:
                continue
            block_by_start = {block.start: block for block in source.prose_blocks}
            anchor_blocks = {
                anchor.block_start: block_by_start[anchor.block_start]
                for anchor in anchors
            }
            supplied_blocks = tuple(
                sorted(
                    {block.start: block for block in (*blocks, *anchor_blocks.values())}.values(),
                    key=lambda block: block.start,
                )
            )
            result.append(
                DestinationCandidate(
                    target=target,
                    similarity=similarity,
                    blocks=supplied_blocks,
                    anchors=anchors,
                )
            )
            if len(result) == candidate_count:
                break
        return tuple(result)
