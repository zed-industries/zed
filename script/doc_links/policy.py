from dataclasses import dataclass
import hashlib

from .corpus import Page, relative_link
from .jev import Evaluation
from .retrieval import AnchorOption
from .schema import Anchor, Decision


@dataclass(frozen=True)
class Thresholds:
    automatic_reason: float = 0.80
    exact_anchor: float = 0.60
    near_reason: float = 0.75

    def to_dict(self) -> dict[str, float]:
        return {
            "automatic_reason": self.automatic_reason,
            "exact_anchor": self.exact_anchor,
            "near_reason": self.near_reason,
        }


def queue_for(
    reason_probability: float,
    anchor: AnchorOption | None,
    anchor_probability: float,
    thresholds: Thresholds,
) -> str:
    exact_anchor = anchor is not None and anchor_probability >= thresholds.exact_anchor
    if reason_probability >= thresholds.automatic_reason and exact_anchor:
        return "automatic"
    if reason_probability >= thresholds.automatic_reason:
        return "strong_review"
    if reason_probability >= thresholds.near_reason and exact_anchor:
        return "near_review"
    return "rejected"


def decision_for(
    source: Page,
    evaluation: Evaluation,
    thresholds: Thresholds,
) -> Decision:
    option = next(
        (
            option
            for option in evaluation.target.anchors
            if option.identifier == evaluation.anchor_choice
        ),
        None,
    )
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
    queue = queue_for(
        evaluation.reason_probability,
        option,
        evaluation.anchor_probability,
        thresholds,
    )
    identity = ":".join(
        (
            str(source.path),
            str(evaluation.target.target.path),
            source.content_hash,
            evaluation.anchor_choice,
        )
    )
    return Decision(
        identifier=hashlib.sha256(identity.encode()).hexdigest()[:16],
        queue=queue,
        source_path=str(source.path),
        target_path=str(evaluation.target.target.path),
        target_hash=evaluation.target.target.content_hash,
        reason_probability=evaluation.reason_probability,
        anchor_choice=evaluation.anchor_choice,
        anchor_probability=evaluation.anchor_probability,
        anchor=anchor,
    )
