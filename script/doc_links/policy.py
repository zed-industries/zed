from dataclasses import dataclass, replace
import hashlib

from .corpus import Page, relative_link
from .jev import Evaluation
from .schema import Anchor, Decision


@dataclass(frozen=True)
class Thresholds:
    automatic_reason: float = 0.80
    direct_destination: float = 0.75
    exact_anchor: float = 0.60
    anchor_quality: float = 0.75
    near_reason: float = 0.70

    def to_dict(self) -> dict[str, float]:
        return {
            "automatic_reason": self.automatic_reason,
            "direct_destination": self.direct_destination,
            "exact_anchor": self.exact_anchor,
            "anchor_quality": self.anchor_quality,
            "near_reason": self.near_reason,
        }


def queue_for(evaluation: Evaluation, thresholds: Thresholds) -> str:
    exact_anchor = (
        evaluation.anchor is not None
        and evaluation.anchor_probability >= thresholds.exact_anchor
        and evaluation.anchor_quality_probability >= thresholds.anchor_quality
    )
    direct = evaluation.destination_probability >= thresholds.direct_destination
    if (
        evaluation.reason_probability >= thresholds.automatic_reason
        and direct
        and exact_anchor
    ):
        return "automatic"
    if evaluation.reason_probability >= thresholds.automatic_reason:
        return "strong_review"
    if evaluation.reason_probability >= thresholds.near_reason and exact_anchor:
        return "near_review"
    return "rejected"


def decision_for(
    source: Page,
    evaluation: Evaluation,
    thresholds: Thresholds,
) -> Decision:
    option = evaluation.anchor
    relative_target = relative_link(source.path, evaluation.target.target.path)
    anchor = (
        Anchor(
            text=option.text,
            start=option.start,
            end=option.end,
            block_start=option.block_start,
            block_end=option.block_end,
            block_hash=option.block_hash,
            relative_target=relative_target,
        )
        if option
        else None
    )
    identity = ":".join(
        (
            str(source.path),
            str(evaluation.target.target.path),
            source.content_hash,
            evaluation.anchor_choice,
            str(option.start if option else -1),
        )
    )
    return Decision(
        identifier=hashlib.sha256(identity.encode()).hexdigest()[:16],
        queue=queue_for(evaluation, thresholds),
        source_path=str(source.path),
        target_path=str(evaluation.target.target.path),
        target_hash=evaluation.target.target.content_hash,
        reason_probability=evaluation.reason_probability,
        destination_probability=evaluation.destination_probability,
        anchor_choice=evaluation.anchor_choice,
        anchor_probability=evaluation.anchor_probability,
        anchor_quality_probability=evaluation.anchor_quality_probability,
        anchor=anchor,
    )


def queue_rank(queue: str) -> int:
    return {
        "automatic": 0,
        "strong_review": 1,
        "near_review": 2,
        "superseded": 3,
        "rejected": 4,
    }[queue]


def overlapping_clusters(decisions: list[Decision]) -> tuple[tuple[int, ...], ...]:
    candidates = sorted(
        (
            (index, decision)
            for index, decision in enumerate(decisions)
            if decision.queue != "rejected" and decision.anchor is not None
        ),
        key=lambda item: (item[1].anchor.start, item[1].anchor.end),
    )
    clusters = []
    current = []
    current_end = -1
    for index, decision in candidates:
        if current and decision.anchor.start >= current_end:
            if len(current) > 1:
                clusters.append(tuple(current))
            current = []
            current_end = -1
        current.append(index)
        current_end = max(current_end, decision.anchor.end)
    if len(current) > 1:
        clusters.append(tuple(current))
    return tuple(clusters)


def supersede_competing(decisions: list[Decision]) -> list[Decision]:
    superseded = {}
    for indexes in overlapping_clusters(decisions):
        winner = min(
            indexes,
            key=lambda index: (
                queue_rank(decisions[index].queue),
                -decisions[index].reason_probability,
                -decisions[index].destination_probability,
                -decisions[index].anchor_quality_probability,
                -decisions[index].anchor_probability,
                decisions[index].target_path,
            ),
        )
        for index in indexes:
            if index != winner:
                superseded[index] = decisions[winner].identifier
    return [
        replace(
            decision,
            queue="superseded",
            superseded_by=superseded[index],
        )
        if index in superseded
        else decision
        for index, decision in enumerate(decisions)
    ]


def decisions_for_source(
    source: Page,
    evaluations: tuple[Evaluation, ...],
    thresholds: Thresholds,
) -> tuple[Decision, ...]:
    decisions = [decision_for(source, item, thresholds) for item in evaluations]
    return tuple(supersede_competing(decisions))
