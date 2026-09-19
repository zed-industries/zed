from datetime import datetime, timedelta, timezone
from email.message import Message
from email.utils import format_datetime
import io
import json
from pathlib import Path
import tempfile
import unittest
from types import SimpleNamespace
from unittest import mock
import urllib.error

from doc_links.corpus import Page
from doc_links import markdown
from doc_links.jev import (
    Client,
    MaxTokensError,
    build_request,
    evaluate_source,
    retry_delay,
    validate_response,
)
from doc_links.policy import Thresholds, queue_for
from doc_links.retrieval import AnchorOption, DestinationCandidate




class FakeResponse:
    def __init__(self, value):
        self.value = value

    def __enter__(self):
        return self

    def __exit__(self, *args):
        return None

    def read(self):
        return json.dumps(self.value).encode()


def fixture():
    source_text = "# Source\n\nUse the command palette.\n"
    target_text = "# Command Palette\n\nRun commands.\n"
    source = Page(
        path=Path("source.md"),
        title="Source",
        source=source_text,
        blocks=markdown.parse(source_text),
        existing_links=frozenset(),
    )
    target = Page(
        path=Path("target.md"),
        title="Command Palette",
        source=target_text,
        blocks=markdown.parse(target_text),
        existing_links=frozenset(),
    )
    block = source.prose_blocks[0]
    start = source_text.index("command palette")
    anchors = tuple(
        AnchorOption(
            identifier=f"anchor_{index:03d}",
            text="command palette" if index == 0 else f"candidate {index}",
            start=start,
            end=start + len("command palette"),
            block_start=block.start,
            block_end=block.end,
            block_hash=block.content_hash,
            score=1.0,
        )
        for index in range(6)
    )
    candidate = DestinationCandidate(
        target=target,
        similarity=1.0,
        blocks=(block,),
        anchors=anchors,
    )
    payload, target_map = build_request(source, (candidate,), "jev-1.13.0")
    probabilities = {anchor.identifier: 0.02 for anchor in anchors}
    probabilities.update({"other_anchor": 0.03, "no_anchor": 0.05})
    probabilities["anchor_000"] = 0.80
    response = {
        "model": "jev-1.13.0",
        "answers": {
            "reason_target_000": {"type": "noul", "noul": 0.9},
            "anchor_target_000": {
                "type": "choice",
                "choice": "anchor_000",
                "probabilities": probabilities,
                "confidence": 0.8,
            },
        },
        "usage": {"input_tokens": 100, "output_tokens": 10},
    }
    return payload, target_map, response


class JevTest(unittest.TestCase):
    def test_validates_complete_response(self):
        _, target_map, response = fixture()
        result = validate_response(response, target_map)
        self.assertEqual(result.evaluations[0].anchor_probability, 0.8)

    def test_missing_no_anchor_probability_is_rejected(self):
        _, target_map, response = fixture()
        del response["answers"]["anchor_target_000"]["probabilities"]["no_anchor"]
        with self.assertRaisesRegex(ValueError, "do not match choices"):
            validate_response(response, target_map)

    def test_invalid_response_is_not_cached(self):
        payload, target_map, response = fixture()
        response["answers"] = {}
        with tempfile.TemporaryDirectory() as directory:
            client = Client("key", Path(directory))
            with mock.patch(
                "urllib.request.urlopen",
                return_value=FakeResponse(response),
            ):
                with self.assertRaisesRegex(ValueError, "do not match questions"):
                    client.evaluate(
                        payload,
                        lambda raw: validate_response(raw, target_map),
                    )
            self.assertEqual(list(Path(directory).glob("*.json")), [])

    def test_valid_response_is_cached_after_validation(self):
        payload, target_map, response = fixture()
        with tempfile.TemporaryDirectory() as directory:
            client = Client("key", Path(directory))
            with mock.patch(
                "urllib.request.urlopen",
                return_value=FakeResponse(response),
            ):
                client.evaluate(
                    payload,
                    lambda raw: validate_response(raw, target_map),
                )
            self.assertEqual(len(list(Path(directory).glob("*.json"))), 1)

    def test_retry_after_http_date(self):
        headers = Message()
        headers["Retry-After"] = format_datetime(
            datetime.now(timezone.utc) + timedelta(seconds=5)
        )
        error = SimpleNamespace(headers=headers)
        self.assertGreaterEqual(retry_delay(error, 0), 0)
        self.assertLessEqual(retry_delay(error, 0), 5)

    def test_rate_limit_is_retried(self):
        payload, target_map, response = fixture()
        headers = Message()
        headers["Retry-After"] = "0"
        error = urllib.error.HTTPError(
            "https://api.typesafe.ai",
            429,
            "rate limited",
            headers,
            io.BytesIO(b"rate limited"),
        )
        with tempfile.TemporaryDirectory() as directory:
            client = Client("key", Path(directory))
            with mock.patch(
                "urllib.request.urlopen",
                side_effect=[error, FakeResponse(response)],
            ) as urlopen, mock.patch("time.sleep"):
                result = client.evaluate(
                    payload,
                    lambda raw: validate_response(raw, target_map),
                )
            error.close()
            self.assertEqual(urlopen.call_count, 2)
            self.assertEqual(result.model, "jev-1.13.0")

    def test_oversized_request_splits_targets_not_anchor_choices(self):
        _, target_map, _ = fixture()
        candidate = next(iter(target_map.values()))
        candidates = (candidate, candidate)
        source_text = "# Source\n\nUse the command palette.\n"
        source = Page(
            path=Path("source.md"),
            title="Source",
            source=source_text,
            blocks=markdown.parse(source_text),
            existing_links=frozenset(),
        )

        class SplittingClient:
            def __init__(self):
                self.target_counts = []

            def evaluate(self, payload, validator):
                target_count = len(payload["state"]["targets"])
                self.target_counts.append(target_count)
                if target_count > 1:
                    raise MaxTokensError("too large")
                return validator(fixture()[2])

        client = SplittingClient()
        result = evaluate_source(client, source, candidates, "jev-1.13.0")
        self.assertEqual(client.target_counts, [2, 1, 1])
        self.assertEqual(len(result.evaluations), 2)
        self.assertTrue(
            all(len(item.target.anchors) == 6 for item in result.evaluations)
        )

    def test_policy_ladder(self):
        thresholds = Thresholds()
        anchor = object()
        self.assertEqual(queue_for(0.9, anchor, 0.7, thresholds), "automatic")
        self.assertEqual(queue_for(0.9, anchor, 0.5, thresholds), "strong_review")
        self.assertEqual(queue_for(0.76, anchor, 0.7, thresholds), "near_review")
        self.assertEqual(queue_for(0.7, anchor, 0.7, thresholds), "rejected")


if __name__ == "__main__":
    unittest.main()
