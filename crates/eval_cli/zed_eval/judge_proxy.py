"""The in-sandbox judge proxy shim, shared by the Harbor verifier and the
offline rejudge path so judge calls go through one identical code path
everywhere.

Stdlib-only and free of any `harbor`/`modal` imports on purpose: `verifier.py`
imports these constants for the live in-sandbox install, and `rejudge.py` runs
the very same proxy in a plain container to re-grade stored runs. Keeping the
shim here means there is exactly one implementation of the upstream fixups
(Anthropic `response_format` stripping, `max_tokens` -> `max_completion_tokens`,
the `ZED_JUDGE_MAX_TOKENS` floor) rather than a copy per call site.
"""

from __future__ import annotations

# Stdlib-only OpenAI-protocol shim. The SWE-Atlas RF judge hardcodes
# `response_format={"type": "json_object"}`, which Anthropic's OpenAI-compatible
# endpoint rejects (400: "response_format.type: Input should be 'json_schema'").
# The shim listens on 127.0.0.1:PORT and forwards to ZED_JUDGE_UPSTREAM, applying
# upstream-specific fixups:
#   - `response_format` is stripped ONLY for Anthropic (Baseten/OpenAI accept
#     json_object, and the Kimi/DeepSeek judges rely on it);
#   - `max_tokens` is renamed to `max_completion_tokens` (accepted by both
#     Anthropic-compat and OpenAI, including GPT-5.x reasoning models that reject
#     `max_tokens`) and raised to the ZED_JUDGE_MAX_TOKENS floor so the runtime
#     judge matches the offline-calibrated token budget.
# When ZED_JUDGE_AUTH_ENV is set, it replaces verifier auth with the named
# sandbox env var; otherwise it passes auth headers through unchanged.
JUDGE_PROXY_SCRIPT = """\
import json, os, urllib.request, urllib.error
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer

UPSTREAM = os.environ.get("ZED_JUDGE_UPSTREAM", "https://api.anthropic.com/v1").rstrip("/")
PORT = int(os.environ.get("ZED_JUDGE_PROXY_PORT", "8089"))
AUTH_ENV = os.environ.get("ZED_JUDGE_AUTH_ENV")
MAX_TOKENS_FLOOR = int(os.environ.get("ZED_JUDGE_MAX_TOKENS", "0"))

class Handler(BaseHTTPRequestHandler):
    def do_POST(self):
        length = int(self.headers.get("Content-Length", 0))
        body = self.rfile.read(length)
        try:
            payload = json.loads(body)
            if "anthropic" in UPSTREAM:
                payload.pop("response_format", None)
            sent = payload.pop("max_tokens", None)
            current = sent if sent is not None else payload.get("max_completion_tokens")
            if current is not None or MAX_TOKENS_FLOOR:
                payload["max_completion_tokens"] = max(current or 0, MAX_TOKENS_FLOOR)
            body = json.dumps(payload).encode()
        except Exception as error:
            print(f"proxy: passing body through unmodified: {error}", flush=True)
        path = self.path
        if path.startswith("/v1/"):
            path = path[len("/v1"):]
        request = urllib.request.Request(UPSTREAM + path, data=body, method="POST")
        request.add_header("Content-Type", "application/json")
        auth_value = os.environ.get(AUTH_ENV, "") if AUTH_ENV else ""
        if auth_value:
            request.add_header("Authorization", f"Bearer {auth_value}")
        else:
            for header in ("Authorization", "x-api-key"):
                value = self.headers.get(header)
                if value:
                    request.add_header(header, value)
        for header in ("anthropic-version",):
            value = self.headers.get(header)
            if value:
                request.add_header(header, value)
        try:
            with urllib.request.urlopen(request, timeout=900) as response:
                data = response.read()
                status = response.status
        except urllib.error.HTTPError as error:
            data = error.read()
            status = error.code
        except Exception as error:
            data = json.dumps({"error": {"message": str(error)}}).encode()
            status = 502
        self.send_response(status)
        self.send_header("Content-Type", "application/json")
        self.send_header("Content-Length", str(len(data)))
        self.end_headers()
        self.wfile.write(data)

    def log_message(self, format, *args):
        print("proxy: " + format % args, flush=True)

if __name__ == "__main__":
    print(f"judge proxy listening on 127.0.0.1:{PORT} -> {UPSTREAM}", flush=True)
    ThreadingHTTPServer(("127.0.0.1", PORT), Handler).serve_forever()
"""

# Idempotent starter. Uses bash's /dev/tcp (pgrep isn't in every image) to check
# the port; the daemon inherits ZED_JUDGE_UPSTREAM from the verifier setup exec
# env. stdout/stderr/stdin are fully detached so the spawning exec doesn't hang
# on open pipes.
JUDGE_PROXY_ENSURE_SCRIPT = """\
#!/bin/bash
if ! (exec 3<>/dev/tcp/127.0.0.1/${ZED_JUDGE_PROXY_PORT:-8089}) 2>/dev/null; then
    nohup python3 /usr/local/lib/zed_judge_proxy.py \\
        >>/tmp/zed-judge-proxy.log 2>&1 </dev/null &
    sleep 1
fi
"""
