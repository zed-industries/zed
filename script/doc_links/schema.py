from dataclasses import dataclass
import hashlib
import json
from typing import Any

from . import SCHEMA_VERSION

QUEUES = {"automatic", "strong_review", "near_review", "rejected"}
LABELS = {"pass", "fail", "defer"}


def content_hash(value: str) -> str:
    return hashlib.sha256(value.encode()).hexdigest()


def canonical_hash(value: Any) -> str:
    serialized = json.dumps(value, sort_keys=True, separators=(",", ":"))
    return content_hash(serialized)


def require_mapping(value: Any, name: str) -> dict[str, Any]:
    if not isinstance(value, dict):
        raise ValueError(f"{name} must be an object")
    return value


def require_string(value: Any, name: str) -> str:
    if not isinstance(value, str) or not value:
        raise ValueError(f"{name} must be a non-empty string")
    return value


def require_probability(value: Any, name: str) -> float:
    if not isinstance(value, (int, float)) or not 0 <= value <= 1:
        raise ValueError(f"{name} must be between 0 and 1")
    return float(value)


@dataclass(frozen=True)
class Anchor:
    text: str
    start: int
    end: int
    block_start: int
    block_end: int
    block_hash: str
    relative_target: str

    def to_dict(self) -> dict[str, Any]:
        return {
            "text": self.text,
            "start": self.start,
            "end": self.end,
            "block_start": self.block_start,
            "block_end": self.block_end,
            "block_hash": self.block_hash,
            "relative_target": self.relative_target,
        }

    @classmethod
    def from_dict(cls, raw: Any) -> "Anchor":
        value = require_mapping(raw, "anchor")
        integers = {}
        for key in ("start", "end", "block_start", "block_end"):
            item = value.get(key)
            if not isinstance(item, int) or item < 0:
                raise ValueError(f"anchor.{key} must be a non-negative integer")
            integers[key] = item
        if not (
            integers["block_start"]
            <= integers["start"]
            < integers["end"]
            <= integers["block_end"]
        ):
            raise ValueError("anchor offsets are inconsistent")
        return cls(
            text=require_string(value.get("text"), "anchor.text"),
            start=integers["start"],
            end=integers["end"],
            block_start=integers["block_start"],
            block_end=integers["block_end"],
            block_hash=require_string(value.get("block_hash"), "anchor.block_hash"),
            relative_target=require_string(
                value.get("relative_target"),
                "anchor.relative_target",
            ),
        )


@dataclass(frozen=True)
class Decision:
    identifier: str
    queue: str
    source_path: str
    target_path: str
    target_hash: str
    reason_probability: float
    anchor_choice: str
    anchor_probability: float
    anchor: Anchor | None

    def to_dict(self) -> dict[str, Any]:
        return {
            "id": self.identifier,
            "queue": self.queue,
            "source_path": self.source_path,
            "target_path": self.target_path,
            "target_hash": self.target_hash,
            "reason_probability": self.reason_probability,
            "anchor_choice": self.anchor_choice,
            "anchor_probability": self.anchor_probability,
            "anchor": self.anchor.to_dict() if self.anchor else None,
        }

    @classmethod
    def from_dict(cls, raw: Any) -> "Decision":
        value = require_mapping(raw, "decision")
        queue = require_string(value.get("queue"), "decision.queue")
        if queue not in QUEUES:
            raise ValueError(f"unknown decision queue: {queue}")
        anchor_raw = value.get("anchor")
        return cls(
            identifier=require_string(value.get("id"), "decision.id"),
            queue=queue,
            source_path=require_string(
                value.get("source_path"),
                "decision.source_path",
            ),
            target_path=require_string(
                value.get("target_path"),
                "decision.target_path",
            ),
            target_hash=require_string(
                value.get("target_hash"),
                "decision.target_hash",
            ),
            reason_probability=require_probability(
                value.get("reason_probability"),
                "decision.reason_probability",
            ),
            anchor_choice=require_string(
                value.get("anchor_choice"),
                "decision.anchor_choice",
            ),
            anchor_probability=require_probability(
                value.get("anchor_probability"),
                "decision.anchor_probability",
            ),
            anchor=Anchor.from_dict(anchor_raw) if anchor_raw is not None else None,
        )


@dataclass(frozen=True)
class Report:
    model: str
    thresholds: dict[str, float]
    pages: dict[str, dict[str, str]]
    decisions: tuple[Decision, ...]
    usage: dict[str, int]

    def to_dict(self) -> dict[str, Any]:
        return {
            "schema_version": SCHEMA_VERSION,
            "model": self.model,
            "thresholds": self.thresholds,
            "pages": self.pages,
            "decisions": [decision.to_dict() for decision in self.decisions],
            "usage": self.usage,
        }

    @property
    def report_hash(self) -> str:
        return canonical_hash(self.to_dict())

    @classmethod
    def from_dict(cls, raw: Any) -> "Report":
        value = require_mapping(raw, "report")
        if value.get("schema_version") != SCHEMA_VERSION:
            raise ValueError("unsupported report schema version")
        thresholds_raw = require_mapping(value.get("thresholds"), "thresholds")
        thresholds = {
            str(key): require_probability(item, f"thresholds.{key}")
            for key, item in thresholds_raw.items()
        }
        pages_raw = require_mapping(value.get("pages"), "pages")
        pages = {}
        for path, page_raw in pages_raw.items():
            page = require_mapping(page_raw, f"pages.{path}")
            pages[path] = {
                "title": require_string(page.get("title"), f"pages.{path}.title"),
                "content_hash": require_string(
                    page.get("content_hash"),
                    f"pages.{path}.content_hash",
                ),
            }
        decisions_raw = value.get("decisions")
        if not isinstance(decisions_raw, list):
            raise ValueError("decisions must be an array")
        usage_raw = require_mapping(value.get("usage"), "usage")
        usage = {}
        for key in ("input_tokens", "output_tokens"):
            item = usage_raw.get(key, 0)
            if not isinstance(item, int) or item < 0:
                raise ValueError(f"usage.{key} must be a non-negative integer")
            usage[key] = item
        decisions = tuple(Decision.from_dict(item) for item in decisions_raw)
        identifiers = {decision.identifier for decision in decisions}
        if len(identifiers) != len(decisions):
            raise ValueError("report contains duplicate decision IDs")
        for decision in decisions:
            if decision.source_path not in pages or decision.target_path not in pages:
                raise ValueError(
                    f"decision {decision.identifier} references unknown page"
                )
        return cls(
            model=require_string(value.get("model"), "model"),
            thresholds=thresholds,
            pages=pages,
            decisions=decisions,
            usage=usage,
        )


def parse_review_export(raw: Any) -> tuple[str, tuple[Decision, ...], dict[str, dict[str, str]]]:
    value = require_mapping(raw, "review export")
    if value.get("schema_version") != SCHEMA_VERSION:
        raise ValueError("unsupported review export schema version")
    report_hash = require_string(value.get("report_hash"), "report_hash")
    decisions_raw = value.get("decisions")
    if not isinstance(decisions_raw, list):
        raise ValueError("decisions must be an array")
    decisions = tuple(Decision.from_dict(item) for item in decisions_raw)
    decision_ids = {decision.identifier for decision in decisions}
    if len(decision_ids) != len(decisions):
        raise ValueError("review export contains duplicate decision IDs")
    labels_raw = require_mapping(value.get("labels"), "labels")
    labels = {}
    for identifier, label_raw in labels_raw.items():
        label = require_mapping(label_raw, f"labels.{identifier}")
        name = require_string(label.get("label"), f"labels.{identifier}.label")
        if name not in LABELS:
            raise ValueError(f"unknown review label: {name}")
        notes = label.get("notes", "")
        if not isinstance(notes, str):
            raise ValueError(f"labels.{identifier}.notes must be a string")
        if identifier not in decision_ids:
            raise ValueError(f"label references unknown decision: {identifier}")
        labels[identifier] = {"label": name, "notes": notes}
    return report_hash, decisions, labels
