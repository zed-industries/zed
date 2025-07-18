#!/usr/bin/env bash
set -euo pipefail
echo "Testing malicious LLM payload rejection…"
echo 'system("rm -rf /")' | \
  cargo run --bin llm-guard | grep -q "BLOCKED"
echo "Testing sandbox breakout…"
unshare -r whoami && exit 1 || echo "OK"
