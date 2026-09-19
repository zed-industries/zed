from dataclasses import dataclass
from datetime import datetime, timezone
from email.utils import parsedate_to_datetime
import hashlib
import json
import os
from pathlib import Path
import time
from typing import Any, Callable, TypeVar
import urllib.error
import urllib.request

from .corpus import Page, page_kind
from .io import read_json, write_json_atomic
from .retrieval import AnchorOption, DestinationCandidate

API_URL = "https://api.typesafe.ai/v1/systemone"
T = TypeVar("T")


class MaxTokensError(RuntimeError):
    pass


@dataclass(frozen=True)
class PreliminaryEvaluation:
    target: DestinationCandidate
    reason_probability: float
    destination_probability: float
    anchor_choice: str
    anchor_probability: float

    @property
    def anchor(self) -> AnchorOption | None:
        return next(
            (
                option
                for option in self.target.anchors
                if option.identifier == self.anchor_choice
            ),
            None,
        )


@dataclass(frozen=True)
class Evaluation:
    target: DestinationCandidate
    reason_probability: float
    destination_probability: float
    anchor_choice: str
    anchor_probability: float
    anchor_quality_probability: float

    @property
    def anchor(self) -> AnchorOption | None:
        return next(
            (
                option
                for option in self.target.anchors
                if option.identifier == self.anchor_choice
            ),
            None,
        )


@dataclass(frozen=True)
class EvaluationResult:
    model: str
    evaluations: tuple[Evaluation, ...]
    input_tokens: int
    output_tokens: int


@dataclass(frozen=True)
class PreliminaryResult:
    model: str
    evaluations: tuple[PreliminaryEvaluation, ...]
    input_tokens: int
    output_tokens: int


@dataclass(frozen=True)
class QualityResult:
    model: str
    probabilities: dict[PreliminaryEvaluation, float]
    input_tokens: int
    output_tokens: int


def target_state(candidate: DestinationCandidate) -> dict[str, Any]:
    return {
        "path": str(candidate.target.path),
        "title": candidate.target.title,
        "page_kind": page_kind(candidate.target.path),
        "overview": candidate.target.overview,
        "outbound_links": sorted(str(path) for path in candidate.target.existing_links),
        "anchor_candidates": {
            anchor.identifier: {
                "text": anchor.text,
                "block": f"block_{anchor.block_start}",
                "start": anchor.start,
            }
            for anchor in candidate.anchors
        },
    }


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
        targets[identifier] = target_state(candidate)
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
        questions[f"destination_{identifier}"] = {
            "type": "noul",
            "instructions": {
                "judgment": (
                    "Is this the direct destination for the source reader's task, "
                    "rather than an indirect, circular, or wrong-audience page?"
                ),
                "source": "`source`",
                "target": f"`targets.{identifier}`",
            },
            "criteria": {
                "true": (
                    "The target itself contains the useful details and its page kind "
                    "matches the reader's intent."
                ),
                "false": (
                    "The target mainly forwards elsewhere or back to the source, or "
                    "serves a different audience such as extension authors instead "
                    "of readers installing an extension."
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
                    "The phrase accurately describes the target and reads naturally "
                    "as a link in its existing sentence."
                ),
            }
            for anchor in candidate.anchors
        }
        criteria["other_anchor"] = (
            "A suitable exact unlinked phrase exists in the supplied source "
            "blocks, "
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
                    "Keep proper nouns together and avoid multi-concept lists.",
                    "When the same phrase appears more than once, choose the "
                    "earliest clear introduction.",
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
                    "page_kind": page_kind(source.path),
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


def usage(raw: Any) -> tuple[int, int]:
    if not isinstance(raw, dict):
        raise ValueError("TypeSafe response is missing usage")
    values = []
    for key in ("input_tokens", "output_tokens"):
        value = raw.get(key, 0)
        if not isinstance(value, int) or value < 0:
            raise ValueError(f"usage.{key} must be a non-negative integer")
        values.append(value)
    return values[0], values[1]


def response_parts(raw: Any) -> tuple[str, dict[str, Any], int, int]:
    if not isinstance(raw, dict):
        raise ValueError("TypeSafe response must be an object")
    model = raw.get("model")
    answers = raw.get("answers")
    if not isinstance(model, str) or not model:
        raise ValueError("TypeSafe response is missing model")
    if not isinstance(answers, dict):
        raise ValueError("TypeSafe response is missing answers")
    input_tokens, output_tokens = usage(raw.get("usage"))
    return model, answers, input_tokens, output_tokens


def noul_answer(answers: dict[str, Any], key: str) -> float:
    answer = answers.get(key)
    if not isinstance(answer, dict) or answer.get("type") != "noul":
        raise ValueError(f"{key} must be a Noul answer")
    return probability(answer.get("noul"), f"{key}.noul")


def validate_response(
    raw: Any,
    target_map: dict[str, DestinationCandidate],
) -> PreliminaryResult:
    model, answers, input_tokens, output_tokens = response_parts(raw)
    expected = {
        f"{kind}_{identifier}"
        for identifier in target_map
        for kind in ("reason", "destination", "anchor")
    }
    if set(answers) != expected:
        raise ValueError(
            "TypeSafe answers do not match questions: "
            f"missing={sorted(expected - set(answers))}, "
            f"unexpected={sorted(set(answers) - expected)}"
        )

    evaluations = []
    for identifier, candidate in target_map.items():
        anchor = answers[f"anchor_{identifier}"]
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
            PreliminaryEvaluation(
                target=candidate,
                reason_probability=noul_answer(answers, f"reason_{identifier}"),
                destination_probability=noul_answer(
                    answers, f"destination_{identifier}"
                ),
                anchor_choice=choice,
                anchor_probability=parsed_probabilities[choice],
            )
        )
    return PreliminaryResult(
        model=model,
        evaluations=tuple(evaluations),
        input_tokens=input_tokens,
        output_tokens=output_tokens,
    )


def build_quality_request(
    source: Page,
    evaluations: tuple[PreliminaryEvaluation, ...],
    model: str,
) -> tuple[dict[str, Any], dict[str, PreliminaryEvaluation]]:
    proposals = {}
    questions = {}
    proposal_map = {}
    for index, evaluation in enumerate(item for item in evaluations if item.anchor):
        identifier = f"proposal_{index:03d}"
        anchor = evaluation.anchor
        if anchor is None:
            continue
        proposal_map[identifier] = evaluation
        block = source.source[anchor.block_start : anchor.block_end]
        proposals[identifier] = {
            "anchor": anchor.text,
            "source_block": block,
            "target": target_state(evaluation.target),
        }
        questions[f"quality_{identifier}"] = {
            "type": "noul",
            "instructions": {
                "judgment": (
                    "Is this exact anchor phrase natural, specific, and correctly "
                    "scoped for the target page?"
                ),
                "proposal": f"`proposals.{identifier}`",
            },
            "criteria": {
                "true": (
                    "The anchor is one coherent noun phrase, keeps proper nouns "
                    "together, and names the target directly."
                ),
                "false": (
                    "The anchor spans multiple concepts, splits a proper noun, ends "
                    "awkwardly, uses a possessive or deictic phrase unnaturally, or "
                    "links wording such as 'below' that refers to local content."
                ),
            },
        }
    return (
        {"state": {"proposals": proposals}, "model": model, "questions": questions},
        proposal_map,
    )


def validate_quality_response(
    raw: Any,
    proposal_map: dict[str, PreliminaryEvaluation],
) -> QualityResult:
    model, answers, input_tokens, output_tokens = response_parts(raw)
    expected = {f"quality_{identifier}" for identifier in proposal_map}
    if set(answers) != expected:
        raise ValueError(
            "TypeSafe quality answers do not match questions: "
            f"missing={sorted(expected - set(answers))}, "
            f"unexpected={sorted(set(answers) - expected)}"
        )
    return QualityResult(
        model=model,
        probabilities={
            evaluation: noul_answer(answers, f"quality_{identifier}")
            for identifier, evaluation in proposal_map.items()
        },
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
        validator: Callable[[Any], T],
    ) -> T:
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


def evaluate_quality(
    client: Client,
    source: Page,
    evaluations: tuple[PreliminaryEvaluation, ...],
    model: str,
) -> QualityResult:
    exact = tuple(item for item in evaluations if item.anchor)
    if not exact:
        return QualityResult(model, {}, 0, 0)
    payload, proposal_map = build_quality_request(source, exact, model)
    try:
        return client.evaluate(
            payload,
            lambda raw: validate_quality_response(raw, proposal_map),
        )
    except MaxTokensError:
        if len(exact) == 1:
            raise
        midpoint = len(exact) // 2
        left = evaluate_quality(client, source, exact[:midpoint], model)
        right = evaluate_quality(client, source, exact[midpoint:], model)
        if left.model != right.model:
            raise RuntimeError("TypeSafe returned different models for quality checks")
        return QualityResult(
            model=left.model,
            probabilities=left.probabilities | right.probabilities,
            input_tokens=left.input_tokens + right.input_tokens,
            output_tokens=left.output_tokens + right.output_tokens,
        )


def evaluate_batch(
    client: Client,
    source: Page,
    candidates: tuple[DestinationCandidate, ...],
    model: str,
) -> EvaluationResult:
    payload, target_map = build_request(source, candidates, model)
    preliminary = client.evaluate(
        payload,
        lambda raw: validate_response(raw, target_map),
    )
    quality = evaluate_quality(client, source, preliminary.evaluations, model)
    if preliminary.model != quality.model:
        raise RuntimeError("TypeSafe returned different models for one source page")
    return EvaluationResult(
        model=preliminary.model,
        evaluations=tuple(
            Evaluation(
                target=item.target,
                reason_probability=item.reason_probability,
                destination_probability=item.destination_probability,
                anchor_choice=item.anchor_choice,
                anchor_probability=item.anchor_probability,
                anchor_quality_probability=quality.probabilities.get(item, 0.0),
            )
            for item in preliminary.evaluations
        ),
        input_tokens=preliminary.input_tokens + quality.input_tokens,
        output_tokens=preliminary.output_tokens + quality.output_tokens,
    )


def evaluate_source(
    client: Client,
    source: Page,
    candidates: tuple[DestinationCandidate, ...],
    model: str,
) -> EvaluationResult:
    try:
        return evaluate_batch(client, source, candidates, model)
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
