from collections import defaultdict
from dataclasses import dataclass
from pathlib import Path

from . import markdown
from .corpus import relative_link
from .io import read_json, write_text_atomic
from .schema import Decision, content_hash, parse_review_export


@dataclass(frozen=True)
class PlannedEdit:
    decision: Decision
    path: Path
    start: int
    end: int
    replacement: str


@dataclass(frozen=True)
class ApplyPlan:
    edits: tuple[PlannedEdit, ...]
    files: dict[Path, str]


def safe_path(docs_dir: Path, relative_path: str) -> Path:
    relative = Path(relative_path)
    if relative.is_absolute() or relative.suffix != ".md":
        raise ValueError(f"invalid documentation path: {relative_path}")
    path = (docs_dir / relative).resolve()
    try:
        path.relative_to(docs_dir.resolve())
    except ValueError as error:
        raise ValueError(f"path escapes documentation directory: {relative_path}") from error
    if not path.is_file():
        raise ValueError(f"documentation page does not exist: {relative_path}")
    return path


def locate_anchor(source: str, decision: Decision) -> tuple[int, int]:
    if decision.anchor is None:
        raise ValueError(f"approved decision {decision.identifier} has no exact anchor")
    anchor = decision.anchor
    relative_start = anchor.start - anchor.block_start
    relative_end = anchor.end - anchor.block_start
    matches = []
    for block in markdown.parse(source):
        if block.content_hash != anchor.block_hash or not block.eligible_for_anchor:
            continue
        start = block.start + relative_start
        end = block.start + relative_end
        excluded = any(
            start < span.end and end > span.start for span in block.excluded
        )
        if (
            not excluded
            and 0 <= relative_start < relative_end <= len(block.source)
            and block.source[relative_start:relative_end] == anchor.text
        ):
            matches.append((start, end))
    if not matches:
        raise ValueError(
            f"source block changed for {decision.source_path}: {anchor.text!r}"
        )
    if len(matches) > 1:
        raise ValueError(
            f"source block is ambiguous for {decision.source_path}: {anchor.text!r}"
        )
    return matches[0]


def build_plan(export_path: Path, docs_dir: Path) -> ApplyPlan:
    _, decisions, labels = parse_review_export(read_json(export_path))
    approved = [
        decision
        for decision in decisions
        if labels.get(decision.identifier, {}).get("label") == "pass"
    ]
    invalid = [
        decision.identifier
        for decision in approved
        if decision.queue not in {"automatic", "strong_review", "near_review"}
    ]
    if invalid:
        raise ValueError(
            f"non-actionable decisions were approved: {sorted(invalid)}"
        )
    by_source = defaultdict(list)
    for decision in approved:
        by_source[decision.source_path].append(decision)

    edits = []
    original_files = {}
    for source_path, source_decisions in by_source.items():
        path = safe_path(docs_dir, source_path)
        source = path.read_text(encoding="utf-8")
        original_files[path] = source
        for decision in source_decisions:
            anchor = decision.anchor
            if anchor is None:
                raise ValueError(
                    f"approved decision {decision.identifier} has no exact anchor"
                )
            target_path = safe_path(docs_dir, decision.target_path)
            if content_hash(target_path.read_text(encoding="utf-8")) != decision.target_hash:
                raise ValueError(
                    f"target page changed after review: {decision.target_path}"
                )
            expected_target = relative_link(
                Path(source_path),
                target_path.relative_to(docs_dir.resolve()),
            )
            if anchor.relative_target != expected_target:
                raise ValueError(
                    f"invalid target for {decision.identifier}: "
                    f"{anchor.relative_target!r} != {expected_target!r}"
                )
            start, end = locate_anchor(source, decision)
            edits.append(
                PlannedEdit(
                    decision=decision,
                    path=path,
                    start=start,
                    end=end,
                    replacement=f"[{anchor.text}]({anchor.relative_target})",
                )
            )

    edits_by_file = defaultdict(list)
    for edit in edits:
        edits_by_file[edit.path].append(edit)
    updated_files = {}
    for path, file_edits in edits_by_file.items():
        ordered = sorted(file_edits, key=lambda edit: edit.start)
        for left, right in zip(ordered, ordered[1:]):
            if left.end > right.start:
                raise ValueError(
                    f"approved anchors overlap in {left.decision.source_path}"
                )
        text = original_files[path]
        for edit in reversed(ordered):
            if text[edit.start:edit.end] != edit.decision.anchor.text:
                raise ValueError(
                    f"anchor text changed in {edit.decision.source_path}"
                )
            text = text[:edit.start] + edit.replacement + text[edit.end:]
        updated_files[path] = text

    return ApplyPlan(
        edits=tuple(edits),
        files=updated_files,
    )


def apply_plan(plan: ApplyPlan) -> None:
    for path, text in plan.files.items():
        write_text_atomic(path, text)
