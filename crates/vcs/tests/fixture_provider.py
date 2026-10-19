#!/usr/bin/env python3
"""Neutral protocol-1 stdio fixture; no VCS, index, or repository metadata needed.

Usage: python3 crates/vcs/tests/fixture_provider.py --scenario normal --events FILE
Feed Content-Length framed JSON-RPC. `--events` records test-only lifecycle events.
Scenarios: normal, reply-eof, crash, error, pending, malformed, oversized,
stderr-flood, descendant, early-exit, reordered, hang-after-exit,
exit-leaves-descendant, polling-content. `reordered` holds the first
snapshot, then sends the second response before the first, even after cancellation.
`descendant` creates one idle child in the
same process group so the host's forced-close ownership can be tested.
`hang-after-exit` acknowledges shutdown but hangs after recording exit receipt.
`exit-leaves-descendant` exits normally while its child keeps stdout open.
`polling-content` creates fresh snapshot IDs; FILE.revision containing r43 moves
later snapshots from r42 to r43 while older snapshot contents remain pinned.
"""
import argparse
import json
import os
from pathlib import Path
import signal
import subprocess
import sys

MAX_MESSAGE_BYTES = 16 * 1024 * 1024
EOF_TEXT = "baseline — r42\n" * 8192
SCENARIOS = ("normal", "reply-eof", "crash", "error", "pending", "malformed",
             "oversized", "stderr-flood", "descendant", "early-exit", "reordered",
             "hang-after-exit", "exit-leaves-descendant", "polling-content")


def read_message():
    length = None
    while True:
        line = sys.stdin.buffer.readline()
        if not line:
            return None
        if line == b"\r\n":
            break
        key, _, value = line.partition(b":")
        if key.lower() == b"content-length":
            length = int(value)
    if length is None or not 0 <= length <= MAX_MESSAGE_BYTES:
        raise ValueError("invalid fixture request length")
    body = sys.stdin.buffer.read(length)
    if len(body) != length:
        raise ValueError("truncated fixture request")
    return json.loads(body)


def send(message):
    body = json.dumps(message, ensure_ascii=False, separators=(",", ":")).encode()
    sys.stdout.buffer.write(f"Content-Length: {len(body)}\r\n\r\n".encode() + body)
    sys.stdout.buffer.flush()


def reply(identifier, result=None, error=None):
    message = {"jsonrpc": "2.0", "id": identifier}
    message.update({"error": error} if error is not None else {"result": result})
    send(message)


def changed():
    send({"jsonrpc": "2.0", "method": "vcs/didChange", "params": {"repositoryId": "fixture-repository"}})


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--scenario", choices=SCENARIOS, default="normal")
    parser.add_argument("--events", type=Path)
    parser.add_argument("--child", action="store_true", help=argparse.SUPPRESS)
    parser.add_argument("--ready-fd", type=int, help=argparse.SUPPRESS)
    args = parser.parse_args()
    if args.child:
        os.write(args.ready_fd, b"ready\n")
        os.close(args.ready_fd)
        while True:
            signal.pause()

    def record(event, **details):
        if args.events:
            with args.events.open("a", encoding="utf-8") as output:
                output.write(json.dumps({"event": event, **details}) + "\n")

    record("provider-started", pid=os.getpid())
    if args.scenario == "early-exit":
        return 17
    descendant = None
    if args.scenario in ("descendant", "hang-after-exit", "exit-leaves-descendant"):
        read_ready, write_ready = os.pipe()
        try:
            descendant = subprocess.Popen([sys.executable, __file__, "--child", "--ready-fd", str(write_ready)],
                                          stdin=subprocess.DEVNULL, pass_fds=(write_ready,))
        finally:
            os.close(write_ready)
        with os.fdopen(read_ready, "rb") as ready:
            assert ready.readline() == b"ready\n"
        record("descendant-started", pid=descendant.pid)
    snapshots = 0
    held_snapshot = None
    snapshot_revisions = {}
    while (message := read_message()) is not None:
        method, identifier = message.get("method"), message.get("id")
        params = message.get("params") or {}
        record("message", method=method, id=identifier)
        if method == "exit":
            record("exit-received")
            if args.scenario == "hang-after-exit":
                while True:
                    signal.pause()
            if args.scenario == "exit-leaves-descendant":
                os._exit(0)
            if descendant is not None:
                descendant.terminate()
                descendant.wait(timeout=3)
            return 0
        if method == "$/cancelRequest":
            record("cancel-received", id=params["id"])
            if args.scenario != "reordered":
                changed()
            continue
        if method == "initialize":
            if args.scenario == "stderr-flood":
                sys.stderr.buffer.write(b"discarded-prefix\n" + ("diagnostic ✓\n" * 40000).encode() + b"fixture-stderr-tail\n")
                sys.stderr.buffer.flush()
            reply(identifier, {"protocolVersion": 1, "serverInfo": {"name": "fixture", "version": "1"}, "capabilities": {"readOnly": True}})
        elif method == "vcs/discover":
            reply(identifier, [{"id": "fixture-repository", "rootUri": params["scopeUri"], "scopeUri": params["scopeUri"],
                                "displayName": "Fixture", "baselines": [{"id": "submitted", "label": "Submitted changes"},
                                                                          {"id": "previous", "label": "Previous changes"}]}])
        elif method == "vcs/snapshot":
            snapshots += 1
            if args.scenario == "crash":
                if snapshots == 2:
                    sys.stderr.write("fixture crash\n")
                    sys.stderr.flush()
                    return 19
                continue
            if args.scenario == "pending":
                record("request-pending", id=identifier)
                changed()
                continue
            if args.scenario == "error":
                reply(identifier, error={"code": -32044, "message": "Snapshot needs refresh",
                                          "data": {"kind": "stale", "revision": "r42", "retryable": True}})
                continue
            if args.scenario in ("malformed", "oversized"):
                header = b"Content-Length: invalid\r\n\r\n" if args.scenario == "malformed" else f"Content-Length: {MAX_MESSAGE_BYTES + 1}\r\n\r\n".encode()
                sys.stdout.buffer.write(header)
                sys.stdout.buffer.flush()
                continue
            previous = params["baselineId"] == "previous"
            snapshot = {"id": "snapshot:previous" if previous else "snapshot:7", "repositoryId": params["repositoryId"], "baselineId": params["baselineId"],
                        "revisionId": "r41" if previous else "r42", "changes": [{"path": "src/example.txt", "status": "modified", "kind": "file", "oldMode": 33188, "newMode": 33188}]}
            if args.scenario == "polling-content":
                marker = Path(str(args.events) + ".revision") if args.events else None
                revision = "r43" if marker and marker.is_file() and marker.read_text().strip() == "r43" else "r42"
                snapshot["id"] = f"snapshot:polling:{snapshots}"
                snapshot["revisionId"] = revision
                snapshot_revisions[snapshot["id"]] = revision
            if args.scenario == "reordered" and snapshots == 1:
                held_snapshot = (identifier, snapshot)
                record("snapshot-held", id=identifier)
                continue
            reply(identifier, snapshot)
            if args.scenario == "reordered" and held_snapshot is not None:
                reply(*held_snapshot)
                held_snapshot = None
        elif method == "vcs/readContent":
            path = params["path"]
            text = EOF_TEXT if args.scenario == "reply-eof" else "previous — r41\n" if params["baselineId"] == "previous" else "baseline — r42\n"
            if args.scenario == "polling-content":
                revision = snapshot_revisions.get(params.get("snapshotId"))
                if revision is None:
                    reply(identifier, error={"code": -32013, "message": "Unknown fixture snapshot", "data": {"kind": "snapshotExpired"}})
                    continue
                text = f"baseline — {revision}\n"
            content = ({"kind": "absent"} if path == "absent.txt" else
                       {"kind": "binary"} if path == "binary.bin" else
                       {"kind": "symlink", "target": "../target"} if path == "link" else
                       {"kind": "text", "text": text})
            reply(identifier, content)
            if args.scenario == "reply-eof":
                return 0
        elif method == "shutdown":
            record("shutdown-received")
            reply(identifier)
        else:
            reply(identifier, error={"code": -32601, "message": "Unknown fixture method"})
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
