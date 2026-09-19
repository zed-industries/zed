from dataclasses import dataclass
import json
from pathlib import Path
from typing import Any

from .review import load_report


@dataclass(frozen=True)
class EvaluationFailure:
    identifier: str
    message: str


def load_cases(path: Path) -> tuple[dict[str, Any], ...]:
    try:
        raw = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as error:
        raise RuntimeError(f"Could not read evaluation cases from {path}: {error}") from error
    if not isinstance(raw, dict) or raw.get("schema_version") != 1:
        raise ValueError("unsupported evaluation case schema")
    cases = raw.get("cases")
    if not isinstance(cases, list):
        raise ValueError("evaluation cases must be an array")
    required = {"id", "source_path", "target_path", "forbidden_anchor", "reason"}
    for case in cases:
        if not isinstance(case, dict) or not required <= set(case):
            raise ValueError("invalid evaluation case")
        if not all(isinstance(case[key], str) and case[key] for key in required):
            raise ValueError("evaluation case fields must be non-empty strings")
        context = case.get("context_contains")
        if context is not None and not isinstance(context, str):
            raise ValueError("context_contains must be a string")
    return tuple(cases)


def evaluate_report(
    report_path: Path,
    cases_path: Path,
    docs_dir: Path,
) -> tuple[EvaluationFailure, ...]:
    report = load_report(report_path)
    cases = load_cases(cases_path)
    failures = []
    for case in cases:
        source_path = docs_dir / case["source_path"]
        source = source_path.read_text(encoding="utf-8") if source_path.is_file() else ""
        matches = []
        for decision in report.decisions:
            if (
                decision.queue != "automatic"
                or decision.source_path != case["source_path"]
                or decision.target_path != case["target_path"]
                or decision.anchor is None
                or decision.anchor.text != case["forbidden_anchor"]
            ):
                continue
            context = source[
                decision.anchor.block_start : decision.anchor.block_end
            ]
            expected_context = case.get("context_contains")
            if expected_context is None or expected_context in context:
                matches.append(decision)
        if matches:
            failures.append(
                EvaluationFailure(
                    identifier=case["id"],
                    message=(
                        f"forbidden automatic link returned: {case['source_path']} → "
                        f"{case['target_path']} using {case['forbidden_anchor']!r}"
                    ),
                )
            )
    return tuple(failures)
