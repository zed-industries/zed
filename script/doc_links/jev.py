from dataclasses import dataclass
from datetime import datetime, timezone
from email.utils import parsedate_to_datetime
import hashlib
import json
import os
from pathlib import Path
import time
from typing import Any, Callable
import urllib.error
import urllib.request

from .corpus import Page
from .io import read_json, write_json_atomic
from .retrieval import DestinationCandidate

API_URL = "https://api.typesafe.ai/v1/systemone"


class MaxTokensError(RuntimeError):
    pass


@dataclass(frozen=True)
class Evaluation:
    target: DestinationCandidate
    reason_probability: float
    anchor_choice: str
    anchor_probability: float


@dataclass(frozen=True)
class EvaluationResult:
    model: str
    evaluations: tuple[Evaluation, ...]
    input_tokens: int
    output_tokens: int


def build_request(
    source: Page,
    candidates: tuple[DestinationCandidate, ...],
    model: str,
) -> tuple[dict[str, Any], dict[str, DestinationCandidate]]:
    targets = {}
    questions = {}
    target_map = {}
    source_blocks = {
        f"block_{block.start}": block.source
        for candidate in candidates
        for block in candidate.blocks
    }
    for index, candidate in enumerate(candidates):
        identifier = f"target_{index:03d}"
        target_map[identifier] = candidate
        targets[identifier] = {
            "path": str(candidate.target.path),
            "title": candidate.target.title,
            "overview": candidate.target.overview,
            "anchor_candidates": {
                anchor.identifier: {
                    "text": anchor.text,
                    "block": f"block_{anchor.block_start}",
                }
                for anchor in candidate.anchors
            },
        }
        questions[f"reason_{identifier}"] = {
            "type": "noul",
            "instructions": {
                "judgment": (
                    "Does this source page have a real reader-serving reason to "
                    f"link to `targets.{identifier}`?"
                ),
                "source": "`source`",
                "target": f"`targets.{identifier}`",
            },
            "criteria": {
                "true": (
                    "The target gives materially useful detail, prerequisites, "
                    "next steps, or context for a concept the source discusses."
                ),
                "false": (
                    "The pages only share a broad topic, repeat each other, or the "
                    "link would not help the reader's current task."
                ),
            },
        }
        criteria = {
            anchor.identifier: {
                "selection": (
                    "Use the exact text in "
                    f"`targets.{identifier}.anchor_candidates.{anchor.identifier}`."
                ),
                "requirement": (
                    "The phrase accurately describes the target and reads "
                    "naturally as a link in its existing sentence."
                ),
            }
            for anchor in candidate.anchors
        }
        criteria["other_anchor"] = (
            "A suitable exact unlinked phrase exists in the supplied source blocks, "
            "but it is missing from the options."
        )
        criteria["no_anchor"] = (
            "No supplied phrase points to this target naturally and accurately."
        )
        questions[f"anchor_{identifier}"] = {
            "type": "choice",
            "instructions": {
                "task": (
                    "Choose the best exact existing anchor phrase for "
                    f"`targets.{identifier}`."
                ),
                "rules": [
                    "Select only a supplied phrase.",
                    "Prefer a concise descriptive noun phrase.",
                    "Reject sentence fragments, vague text, and misleading text.",
                ],
            },
            "criteria": criteria,
        }
    return (
        {
            "state": {
                "source": {
                    "path": str(source.path),
                    "title": source.title,
                    "blocks": source_blocks,
                },
                "targets": targets,
            },
            "model": model,
            "questions": questions,
        },
        target_map,
    )


def probability(value: Any, name: str) -> float:
    if not isinstance(value, (int, float)) or not 0 <= value <= 1:
        raise ValueError(f"{name} must be between 0 and 1")
    return float(value)


def validate_response(
    raw: Any,
    target_map: dict[str, DestinationCandidate],
) -> EvaluationResult:
    if not isinstance(raw, dict):
        raise ValueError("TypeSafe response must be an object")
    model = raw.get("model")
    answers = raw.get("answers")
    usage = raw.get("usage")
    if not isinstance(model, str) or not model:
        raise ValueError("TypeSafe response is missing model")
    if not isinstance(answers, dict):
        raise ValueError("TypeSafe response is missing answers")
    if not isinstance(usage, dict):
        raise ValueError("TypeSafe response is missing usage")
    expected = {
        f"{kind}_{identifier}"
        for identifier in target_map
        for kind in ("reason", "anchor")
    }
    if set(answers) != expected:
        raise ValueError(
            "TypeSafe answers do not match questions: "
            f"missing={sorted(expected - set(answers))}, "
            f"unexpected={sorted(set(answers) - expected)}"
        )

    evaluations = []
    for identifier, candidate in target_map.items():
        reason = answers[f"reason_{identifier}"]
        anchor = answers[f"anchor_{identifier}"]
        if not isinstance(reason, dict) or reason.get("type") != "noul":
            raise ValueError(f"reason_{identifier} must be a Noul answer")
        if not isinstance(anchor, dict) or anchor.get("type") != "choice":
            raise ValueError(f"anchor_{identifier} must be a Choice answer")
        choices = {
            option.identifier for option in candidate.anchors
        } | {"other_anchor", "no_anchor"}
        choice = anchor.get("choice")
        probabilities = anchor.get("probabilities")
        if choice not in choices:
            raise ValueError(f"anchor_{identifier} selected unknown choice {choice}")
        if not isinstance(probabilities, dict) or set(probabilities) != choices:
            raise ValueError(f"anchor_{identifier} probabilities do not match choices")
        parsed_probabilities = {
            key: probability(value, f"anchor_{identifier}.{key}")
            for key, value in probabilities.items()
        }
        if not 0.98 <= sum(parsed_probabilities.values()) <= 1.02:
            raise ValueError(f"anchor_{identifier} probabilities do not sum to 1")
        evaluations.append(
            Evaluation(
                target=candidate,
                reason_probability=probability(
                    reason.get("noul"),
                    f"reason_{identifier}.noul",
                ),
                anchor_choice=choice,
                anchor_probability=parsed_probabilities[choice],
            )
        )

    input_tokens = usage.get("input_tokens", 0)
    output_tokens = usage.get("output_tokens", 0)
    if not isinstance(input_tokens, int) or input_tokens < 0:
        raise ValueError("usage.input_tokens must be a non-negative integer")
    if not isinstance(output_tokens, int) or output_tokens < 0:
        raise ValueError("usage.output_tokens must be a non-negative integer")
    return EvaluationResult(
        model=model,
        evaluations=tuple(evaluations),
        input_tokens=input_tokens,
        output_tokens=output_tokens,
    )


def retry_delay(error: urllib.error.HTTPError, attempt: int) -> float:
    value = error.headers.get("Retry-After")
    if not value:
        return float(2**attempt)
    try:
        return max(0.0, float(value))
    except ValueError:
        try:
            retry_at = parsedate_to_datetime(value)
        except (TypeError, ValueError, OverflowError):
            return float(2**attempt)
        if retry_at.tzinfo is None:
            retry_at = retry_at.replace(tzinfo=timezone.utc)
        return max(0.0, (retry_at - datetime.now(timezone.utc)).total_seconds())


class Client:
    def __init__(self, api_key: str, cache_dir: Path):
        self.api_key = api_key
        self.cache_dir = cache_dir

    def evaluate(
        self,
        payload: dict[str, Any],
        validator: Callable[[Any], EvaluationResult],
    ) -> EvaluationResult:
        serialized = json.dumps(
            payload,
            sort_keys=True,
            separators=(",", ":"),
        ).encode()
        cache_path = self.cache_dir / f"{hashlib.sha256(serialized).hexdigest()}.json"
        if cache_path.is_file():
            try:
                return validator(read_json(cache_path))
            except (RuntimeError, ValueError) as error:
                raise RuntimeError(f"Invalid cached response in {cache_path}: {error}") from error

        request = urllib.request.Request(
            API_URL,
            data=json.dumps(payload).encode(),
            headers={
                "Authorization": f"Bearer {self.api_key}",
                "Content-Type": "application/json",
            },
            method="POST",
        )
        for attempt in range(4):
            try:
                with urllib.request.urlopen(request, timeout=120) as response:
                    raw = json.loads(response.read())
                result = validator(raw)
                write_json_atomic(cache_path, raw)
                return result
            except urllib.error.HTTPError as error:
                body = error.read().decode(errors="replace")
                if "max_tokens_exceeded" in body:
                    raise MaxTokensError(body) from error
                if error.code not in (429, 529) or attempt == 3:
                    raise RuntimeError(
                        f"TypeSafe returned HTTP {error.code}: {body}"
                    ) from error
                time.sleep(retry_delay(error, attempt))
            except urllib.error.URLError as error:
                if attempt == 3:
                    raise RuntimeError(f"TypeSafe request failed: {error}") from error
                time.sleep(2**attempt)
        raise RuntimeError("TypeSafe request failed without an error")


def evaluate_source(
    client: Client,
    source: Page,
    candidates: tuple[DestinationCandidate, ...],
    model: str,
) -> EvaluationResult:
    payload, target_map = build_request(source, candidates, model)
    try:
        return client.evaluate(
            payload,
            lambda raw: validate_response(raw, target_map),
        )
    except MaxTokensError:
        if len(candidates) == 1:
            raise
        midpoint = len(candidates) // 2
        left = evaluate_source(client, source, candidates[:midpoint], model)
        right = evaluate_source(client, source, candidates[midpoint:], model)
        if left.model != right.model:
            raise RuntimeError("TypeSafe returned different models for one source page")
        return EvaluationResult(
            model=left.model,
            evaluations=left.evaluations + right.evaluations,
            input_tokens=left.input_tokens + right.input_tokens,
            output_tokens=left.output_tokens + right.output_tokens,
        )


def api_key_from_environment() -> str:
    value = os.environ.get("TYPESAFE_API_KEY")
    if not value:
        raise RuntimeError("TYPESAFE_API_KEY is not set")
    return value
