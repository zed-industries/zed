from collections import Counter
from dataclasses import dataclass
import json
from pathlib import Path
import re

from .corpus import Page, load_pages
from .jev import Client, Evaluation, evaluate_source
from .policy import Thresholds, decisions_for_source
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
from .schema import canonical_hash

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


@dataclass(frozen=True)
class EvaluationObservation:
    case: EvaluationCase
    passed: bool
    message: str | None
    queue: str | None = None
    selected_anchor: str | None = None
    anchor_choice: str | None = None
    reason_probability: float | None = None
    destination_probability: float | None = None
    anchor_probability: float | None = None
    anchor_quality_probability: float | None = None

    def to_dict(self) -> dict:
        return {
            "id": self.case.identifier,
            "outcome": self.case.outcome,
            "passed": self.passed,
            "message": self.message,
            "queue": self.queue,
            "selected_anchor": self.selected_anchor,
            "anchor_choice": self.anchor_choice,
            "reason_probability": self.reason_probability,
            "destination_probability": self.destination_probability,
            "anchor_probability": self.anchor_probability,
            "anchor_quality_probability": self.anchor_quality_probability,
        }


@dataclass(frozen=True)
class EvaluationRun:
    model: str
    dataset: str
    corpus_hash: str
    thresholds: Thresholds
    observations: tuple[EvaluationObservation, ...]
    input_tokens: int
    output_tokens: int

    @property
    def failures(self) -> tuple[EvaluationFailure, ...]:
        return tuple(
            EvaluationFailure(item.case.identifier, item.message or "case failed")
            for item in self.observations
            if not item.passed
        )

    def to_dict(self) -> dict:
        counts = Counter(item.case.outcome for item in self.observations)
        passed = Counter(
            item.case.outcome for item in self.observations if item.passed
        )
        return {
            "schema_version": 2,
            "model": self.model,
            "dataset": self.dataset,
            "corpus_hash": self.corpus_hash,
            "thresholds": self.thresholds.to_dict(),
            "usage": {
                "input_tokens": self.input_tokens,
                "output_tokens": self.output_tokens,
            },
            "summary": {
                outcome: {"passed": passed[outcome], "total": counts[outcome]}
                for outcome in sorted(OUTCOMES)
            },
            "cases": [item.to_dict() for item in self.observations],
        }


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
            isinstance(raw_case[key], str) and raw_case[key] for key in required
        ):
            raise ValueError("evaluation case fields must be non-empty strings")
        if raw_case["outcome"] not in OUTCOMES:
            raise ValueError(f"unknown evaluation outcome: {raw_case['outcome']}")
        context = raw_case.get("context_contains")
        if context is not None and (not isinstance(context, str) or not context):
            raise ValueError("context_contains must be a non-empty string")
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
    identifiers = {case.identifier for case in result}
    if len(identifiers) != len(result):
        raise ValueError("evaluation cases must have unique IDs")
    return tuple(result)


def context_span(source: str, context: str | None) -> tuple[int, int] | None:
    if context is None:
        return None
    starts = [match.start() for match in re.finditer(re.escape(context), source)]
    if len(starts) != 1:
        return None
    return starts[0], starts[0] + len(context)


def exact_anchor(source: Page, case: EvaluationCase) -> AnchorOption | None:
    required_context = context_span(source.source, case.context_contains)
    if case.context_contains is not None and required_context is None:
        return None
    pattern = re.compile(
        rf"(?<![A-Za-z0-9]){re.escape(case.anchor_text)}(?![A-Za-z0-9])"
    )
    for block in source.prose_blocks:
        for match in pattern.finditer(block.source):
            start = block.start + match.start()
            end = block.start + match.end()
            if required_context and not (
                required_context[0] <= start and end <= required_context[1]
            ):
                continue
            if overlaps_excluded(block, start, end):
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
            line = block.source[line_start - block.start : line_end - block.start]
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


def check_evaluation(
    case: EvaluationCase,
    evaluation: Evaluation,
    queue: str,
    thresholds: Thresholds,
) -> str | None:
    if case.outcome == "link":
        if evaluation.anchor_choice != "anchor_000":
            return f"expected anchor_000, chose {evaluation.anchor_choice}"
        if queue in {"rejected", "superseded"}:
            return f"expected an actionable link, got {queue}"
        return None
    if (
        evaluation.anchor_choice == "anchor_000"
        and evaluation.anchor_quality_probability is not None
        and evaluation.anchor_quality_probability >= thresholds.anchor_quality
    ):
        return "forbidden anchor selected with passing anchor quality"
    return None


def observation_for_failure(
    case: EvaluationCase,
    message: str,
) -> EvaluationObservation:
    return EvaluationObservation(case=case, passed=False, message=message)


def evaluation_corpus_hash(
    cases: tuple[EvaluationCase, ...],
    pages: dict[str, Page],
) -> str:
    return canonical_hash(
        {
            path: pages[path].content_hash
            for case in cases
            for path in (case.source_path, case.target_path)
            if path in pages
        }
    )


def evaluate_live(
    client: Client,
    cases_path: Path,
    docs_dir: Path,
    model: str,
    thresholds: Thresholds,
    anchor_count: int = 6,
) -> EvaluationRun:
    cases = load_cases(cases_path)
    pages = load_pages(docs_dir)
    by_path = {str(page.path): page for page in pages}
    index = Index(pages)
    observations = []
    models = set()
    input_tokens = 0
    output_tokens = 0
    for case in cases:
        source = by_path.get(case.source_path)
        target = by_path.get(case.target_path)
        if source is None or target is None:
            observations.append(observation_for_failure(case, "page does not exist"))
            continue
        candidate = forced_candidate(source, target, case, index, anchor_count)
        if case.outcome == "out_of_scope":
            message = "out-of-scope anchor is eligible" if candidate else None
            observations.append(
                EvaluationObservation(
                    case=case,
                    passed=message is None,
                    message=message,
                )
            )
            continue
        if candidate is None:
            observations.append(
                observation_for_failure(case, "case anchor is not eligible")
            )
            continue
        result = evaluate_source(client, source, (candidate,), model)
        models.add(result.model)
        input_tokens += result.input_tokens
        output_tokens += result.output_tokens
        evaluation = result.evaluations[0]
        decision = decisions_for_source(source, (evaluation,), thresholds)[0]
        message = check_evaluation(case, evaluation, decision.queue, thresholds)
        observations.append(
            EvaluationObservation(
                case=case,
                passed=message is None,
                message=message,
                queue=decision.queue,
                selected_anchor=(
                    evaluation.anchor.text if evaluation.anchor is not None else None
                ),
                anchor_choice=evaluation.anchor_choice,
                reason_probability=evaluation.reason_probability,
                destination_probability=evaluation.destination_probability,
                anchor_probability=evaluation.anchor_probability,
                anchor_quality_probability=evaluation.anchor_quality_probability,
            )
        )
    if len(models) > 1:
        raise RuntimeError(f"TypeSafe returned multiple models: {sorted(models)}")
    return EvaluationRun(
        model=models.pop() if models else model,
        dataset=cases_path.name,
        corpus_hash=evaluation_corpus_hash(cases, by_path),
        thresholds=thresholds,
        observations=tuple(observations),
        input_tokens=input_tokens,
        output_tokens=output_tokens,
    )
