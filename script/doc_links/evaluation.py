from dataclasses import dataclass
import json
from pathlib import Path
import re
from typing import Any

from .corpus import Page, load_pages
from .jev import Client, evaluate_source
from .policy import Thresholds, decision_for
from .retrieval import (
    BOUNDARY_STOP_WORDS,
    WORD_PATTERN,
    AnchorOption,
    DestinationCandidate,
    Index,
    anchor_options,
    overlaps_excluded,
    top_blocks,
)
from .review import load_report

OUTCOMES = {"link", "no_link", "out_of_scope"}


@dataclass(frozen=True)
class EvaluationFailure:
    identifier: str
    message: str


@dataclass(frozen=True)
class EvaluationCase:
    identifier: str
    outcome: str
    source_path: str
    target_path: str
    anchor_text: str
    reason: str
    context_contains: str | None = None


def load_cases(path: Path) -> tuple[EvaluationCase, ...]:
    try:
        raw = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as error:
        raise RuntimeError(f"Could not read evaluation cases from {path}: {error}") from error
    if not isinstance(raw, dict) or raw.get("schema_version") != 2:
        raise ValueError("unsupported evaluation case schema")
    cases = raw.get("cases")
    if not isinstance(cases, list):
        raise ValueError("evaluation cases must be an array")
    result = []
    for raw_case in cases:
        if not isinstance(raw_case, dict):
            raise ValueError("evaluation case must be an object")
        required = {
            "id",
            "outcome",
            "source_path",
            "target_path",
            "anchor_text",
            "reason",
        }
        if not required <= set(raw_case):
            raise ValueError("evaluation case is missing required fields")
        if not all(
            isinstance(raw_case[key], str) and raw_case[key]
            for key in required
        ):
            raise ValueError("evaluation case fields must be non-empty strings")
        if raw_case["outcome"] not in OUTCOMES:
            raise ValueError(f"unknown evaluation outcome: {raw_case['outcome']}")
        context = raw_case.get("context_contains")
        if context is not None and not isinstance(context, str):
            raise ValueError("context_contains must be a string")
        result.append(
            EvaluationCase(
                identifier=raw_case["id"],
                outcome=raw_case["outcome"],
                source_path=raw_case["source_path"],
                target_path=raw_case["target_path"],
                anchor_text=raw_case["anchor_text"],
                reason=raw_case["reason"],
                context_contains=context,
            )
        )
    return tuple(result)


def matching_decisions(report, case: EvaluationCase):
    return tuple(
        decision
        for decision in report.decisions
        if decision.source_path == case.source_path
        and decision.target_path == case.target_path
        and decision.anchor is not None
        and decision.anchor.text == case.anchor_text
    )


def context_matches(source: str, case: EvaluationCase, decision) -> bool:
    if case.context_contains is None:
        return True
    context = source[decision.anchor.block_start : decision.anchor.block_end]
    return case.context_contains in context


def check_decision(case: EvaluationCase, decision) -> str | None:
    if case.outcome == "link":
        if decision.queue in {"rejected", "superseded"}:
            return f"expected an actionable link, got {decision.queue}"
        return None
    if decision.queue == "automatic":
        return "forbidden link returned as automatic"
    return None


def evaluate_report(
    report_path: Path,
    cases_path: Path,
    docs_dir: Path,
) -> tuple[EvaluationFailure, ...]:
    report = load_report(report_path)
    cases = load_cases(cases_path)
    failures = []
    for case in cases:
        metadata = report.pages.get(case.source_path)
        source_path = docs_dir / case.source_path
        if metadata is None or not source_path.is_file():
            failures.append(EvaluationFailure(case.identifier, "source page was not audited"))
            continue
        source = source_path.read_text(encoding="utf-8")
        from .schema import content_hash

        if content_hash(source) != metadata["content_hash"]:
            failures.append(EvaluationFailure(case.identifier, "source page changed after audit"))
            continue
        matches = tuple(
            decision
            for decision in matching_decisions(report, case)
            if context_matches(source, case, decision)
        )
        if case.outcome == "out_of_scope":
            if matches:
                failures.append(
                    EvaluationFailure(case.identifier, "out-of-scope anchor was proposed")
                )
            continue
        if not matches:
            failures.append(EvaluationFailure(case.identifier, "case was not exercised"))
            continue
        for decision in matches:
            message = check_decision(case, decision)
            if message:
                failures.append(EvaluationFailure(case.identifier, message))
                break
    return tuple(failures)


def exact_anchor(source: Page, case: EvaluationCase) -> AnchorOption | None:
    pattern = re.compile(
        rf"(?<![A-Za-z0-9]){re.escape(case.anchor_text)}(?![A-Za-z0-9])"
    )
    for block in source.prose_blocks:
        if case.context_contains and case.context_contains not in block.source:
            continue
        for match in pattern.finditer(block.source):
            start = block.start + match.start()
            end = block.start + match.end()
            if any(start < span.end and end > span.start for span in block.excluded):
                continue
            return AnchorOption(
                identifier="anchor_000",
                text=case.anchor_text,
                start=start,
                end=end,
                block_start=block.start,
                block_end=block.end,
                block_hash=block.content_hash,
                score=100.0,
            )
    return None




def filler_anchors(
    source: Page,
    selected: list[AnchorOption],
    count: int,
) -> None:
    seen = {anchor.text.casefold() for anchor in selected}
    for block in source.prose_blocks:
        for line_start, line_end in block.line_ranges():
            line = block.source[
                line_start - block.start : line_end - block.start
            ]
            words = list(WORD_PATTERN.finditer(line))
            for first_index, first_word in enumerate(words):
                for word_count in range(1, 5):
                    last_index = first_index + word_count
                    if last_index > len(words):
                        break
                    last_word = words[last_index - 1]
                    start = line_start + first_word.start()
                    end = line_start + last_word.end()
                    phrase = source.source[start:end]
                    first = first_word.group().lower().strip(".'’/-")
                    last = last_word.group().lower().strip(".'’/-")
                    if (
                        not first
                        or not last
                        or first in BOUNDARY_STOP_WORDS
                        or last in BOUNDARY_STOP_WORDS
                        or phrase.casefold() in seen
                        or overlaps_excluded(block, start, end)
                        or any(
                            start < anchor.end and end > anchor.start
                            for anchor in selected
                        )
                    ):
                        continue
                    selected.append(
                        AnchorOption(
                            identifier="",
                            text=phrase,
                            start=start,
                            end=end,
                            block_start=block.start,
                            block_end=block.end,
                            block_hash=block.content_hash,
                            score=0.0,
                        )
                    )
                    seen.add(phrase.casefold())
                    if len(selected) == count:
                        return

def forced_candidate(
    source: Page,
    target: Page,
    case: EvaluationCase,
    index: Index,
    anchor_count: int,
) -> DestinationCandidate | None:
    required = exact_anchor(source, case)
    if required is None:
        return None
    blocks = top_blocks(source, target, index.inverse, 4)
    generated = anchor_options(target, blocks, anchor_count, source.prose_blocks)
    anchors = [required]
    for anchor in generated:
        if anchor.text.casefold() == required.text.casefold():
            continue
        if any(
            anchor.start < selected.end and anchor.end > selected.start
            for selected in anchors
        ):
            continue
        anchors.append(anchor)
        if len(anchors) == anchor_count:
            break
    if len(anchors) < anchor_count:
        filler_anchors(source, anchors, anchor_count)
    if len(anchors) != anchor_count:
        return None
    anchors = [
        AnchorOption(
            identifier=f"anchor_{offset:03d}",
            text=anchor.text,
            start=anchor.start,
            end=anchor.end,
            block_start=anchor.block_start,
            block_end=anchor.block_end,
            block_hash=anchor.block_hash,
            score=anchor.score,
        )
        for offset, anchor in enumerate(anchors)
    ]
    block_by_start = {block.start: block for block in source.prose_blocks}
    supplied = tuple(
        sorted(
            {
                block.start: block
                for block in (
                    *blocks,
                    *(block_by_start[item.block_start] for item in anchors),
                )
            }.values(),
            key=lambda block: block.start,
        )
    )
    return DestinationCandidate(
        target=target,
        similarity=1.0,
        blocks=supplied,
        anchors=tuple(anchors),
    )


def evaluate_live(
    client: Client,
    cases_path: Path,
    docs_dir: Path,
    model: str,
    thresholds: Thresholds,
    anchor_count: int = 6,
) -> tuple[EvaluationFailure, ...]:
    cases = load_cases(cases_path)
    pages = load_pages(docs_dir)
    by_path = {str(page.path): page for page in pages}
    index = Index(pages)
    failures = []
    for case in cases:
        source = by_path.get(case.source_path)
        target = by_path.get(case.target_path)
        if source is None or target is None:
            failures.append(EvaluationFailure(case.identifier, "page does not exist"))
            continue
        candidate = forced_candidate(source, target, case, index, anchor_count)
        if case.outcome == "out_of_scope":
            if candidate is not None:
                failures.append(
                    EvaluationFailure(case.identifier, "out-of-scope anchor is eligible")
                )
            continue
        if candidate is None:
            failures.append(EvaluationFailure(case.identifier, "case was not exercised"))
            continue
        result = evaluate_source(client, source, (candidate,), model)
        from .policy import decisions_for_source

        decision = decisions_for_source(source, result.evaluations, thresholds)[0]
        message = check_decision(case, decision)
        if message:
            failures.append(EvaluationFailure(case.identifier, message))
    return tuple(failures)
