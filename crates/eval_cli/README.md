# eval-cli

Headless CLI binary for running Zed's agent in evaluation/benchmark
environments. Designed to work inside containerized environments like
[Harbor](https://harborframework.com/) where the repository is already
checked out and API keys are provided via environment variables.

Uses the same `NativeAgent` + `AcpThread` pipeline as the production Zed
editor — full agentic loop with tool calls, subagents, and retries, just
without a GUI.

## Building

### Native (for local testing on the same OS)

```
cargo build --release -p eval_cli
```

### Cross-compile for Linux x86_64 (from macOS or other hosts)

Harbor containers run Linux x86_64. Use the Docker-based build script:

```
crates/eval_cli/script/build-linux
```

This produces `target/eval-cli` (an x86_64 Linux ELF binary). You can
also specify a custom output path:

```
crates/eval_cli/script/build-linux --output ~/bin/eval-cli-linux
```

## Standalone usage

```
eval-cli \
  --workdir /testbed \
  --model anthropic/claude-sonnet-4-6-latest \
  --instruction "Fix the bug described in..." \
  --timeout 600 \
  --output-dir /logs/agent
```

Reads API keys from environment variables (`ANTHROPIC_API_KEY`,
`OPENAI_API_KEY`, etc.). Writes `result.json`, `thread.md`, and
`thread.json` to the output directory.

### Exit codes

| Code | Meaning                            |
| ---- | ---------------------------------- |
| 0    | Agent finished                     |
| 1    | Error (model/auth/runtime failure) |
| 2    | Timeout                            |
| 3    | Interrupted (SIGTERM/SIGINT)       |

## Harbor integration

The `zed_eval/` directory contains a Python package that
implements Harbor's `BaseInstalledAgent` interface, allowing eval-cli to
be used with `--agent-import-path` without modifying Harbor's source code.

### Setup

```
pip install -e crates/eval_cli/harbor/
```

### Running with a local binary

Build for Linux first, then pass the binary path:

```
crates/eval_cli/script/build-linux

harbor run -d "swebench_verified@latest" \
  --agent-import-path zed_eval.agent:ZedAgent \
  --ae binary_path=target/eval-cli \
  -m anthropic/claude-sonnet-4-6-latest
```

The agent uploads the binary into the container during setup — no
download URL needed during local iteration.

### Running with a download URL

For CI or when the binary is hosted somewhere:

```
harbor run -d "swebench_verified@latest" \
  --agent-import-path zed_eval.agent:ZedAgent \
  --ak download_url=https://example.com/eval-cli \
  -m anthropic/claude-sonnet-4-6-latest
```

### Setting a timeout

Pass `EVAL_CLI_TIMEOUT` via `--ae`:

```
harbor run -d "swebench_verified@latest" \
  --agent-import-path zed_eval.agent:ZedAgent \
  --ak binary_path=target/eval-cli \
  --ae EVAL_CLI_TIMEOUT=600 \
  -m anthropic/claude-sonnet-4-6-latest
```
