from collections import defaultdict
from dataclasses import dataclass, replace
import hashlib

from .corpus import Page, relative_link
from .jev import Evaluation
from .schema import Anchor, Decision


@dataclass(frozen=True)
class Thresholds:
    automatic_reason: float = 0.80
    direct_destination: float = 0.70
    exact_anchor: float = 0.60
    anchor_quality: float = 0.80
    near_reason: float = 0.75

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
    if evaluation.reason_probability >= thresholds.automatic_reason and direct:
        return "strong_review"
    if (
        evaluation.reason_probability >= thresholds.near_reason
        and direct
        and exact_anchor
    ):
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


def decision_score(decision: Decision) -> float:
    return min(
        decision.reason_probability,
        decision.destination_probability,
        decision.anchor_probability,
        decision.anchor_quality_probability,
    )


def queue_rank(queue: str) -> int:
    return {
        "automatic": 0,
        "strong_review": 1,
        "near_review": 2,
        "rejected": 3,
    }[queue]


def reject_competing(
    decisions: list[Decision],
    key,
    prefer_first: bool = False,
) -> list[Decision]:
    groups = defaultdict(list)
    for index, decision in enumerate(decisions):
        if decision.queue != "rejected" and decision.anchor is not None:
            groups[key(decision)].append(index)
    rejected = set()
    for indexes in groups.values():
        if len(indexes) < 2:
            continue
        def rank(index: int):
            decision = decisions[index]
            if prefer_first:
                return (
                    queue_rank(decision.queue),
                    decision.anchor.start,
                    -decision_score(decision),
                )
            return (
                queue_rank(decision.queue),
                -decision_score(decision),
                decision.anchor.start,
                decision.target_path,
            )

        winner = min(indexes, key=rank)
        rejected.update(index for index in indexes if index != winner)
    return [
        replace(decision, queue="rejected") if index in rejected else decision
        for index, decision in enumerate(decisions)
    ]


def decisions_for_source(
    source: Page,
    evaluations: tuple[Evaluation, ...],
    thresholds: Thresholds,
) -> tuple[Decision, ...]:
    decisions = [decision_for(source, item, thresholds) for item in evaluations]
    decisions = reject_competing(
        decisions,
        lambda item: (item.anchor.start, item.anchor.end),
    )
    decisions = reject_competing(
        decisions,
        lambda item: item.target_path,
        prefer_first=True,
    )
    return tuple(decisions)
