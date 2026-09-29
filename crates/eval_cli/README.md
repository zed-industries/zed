# eval-cli

Headless Rust binary for running Zed's agent in evaluation and benchmark
environments. It is designed for containerized harnesses such as
[Harbor](https://harborframework.com/) and Pier, where the repository is already
checked out and model API keys are provided via environment variables.

`eval-cli` uses the same `NativeAgent` + `AcpThread` pipeline as the production
Zed editor: a full agentic loop with tool calls, subagents, and retries, without
a GUI.

This directory also contains `zed_eval/`, the Python `zed-eval` package used to
build this binary, launch remote benchmark runs on Modal/Harbor/Pier, and fetch
results. For normal benchmark orchestration, start with
[`zed_eval/README.md`](zed_eval/README.md).

## Building

### Native, for local testing on the same OS

```sh
cargo build --release -p eval_cli
```

### Linux x86_64, for Harbor/Pier sandboxes

Harbor and Pier containers run Linux x86_64. From the repository root, use the
Docker-based build script:

```sh
crates/eval_cli/script/build-linux
```

This produces `target/eval-cli`, an x86_64 Linux ELF binary. You can also
specify a custom output path:

```sh
crates/eval_cli/script/build-linux --output ~/bin/eval-cli-linux
```

## Standalone usage

```sh
eval-cli \
  --workdir /testbed \
  --model anthropic/claude-sonnet-4-6 \
  --instruction "Fix the bug described in..." \
  --timeout 600 \
  --output-dir /logs/agent
```

`eval-cli` reads provider API keys from environment variables such as
`ANTHROPIC_API_KEY` and `OPENAI_API_KEY`. It writes `result.json`, `thread.md`,
and `thread.json` to the output directory.

### Exit codes

| Code | Meaning |
| --- | --- |
| 0 | Agent finished |
| 1 | Error, such as model/auth/runtime failure |
| 2 | Timeout |
| 3 | Interrupted by SIGTERM or SIGINT |

## Running benchmarks

Most benchmark runs should use the Python `zed-eval` CLI instead of invoking
`eval-cli` directly. From the repository root:

```sh
crates/eval_cli/script/install-zed-eval
zed-eval doctor --create-volume
zed-eval run rf --from local --n-tasks 2
```

For one-off source runs without installing the tool globally, use
`crates/eval_cli/script/zed-eval <args>`.

See [`zed_eval/README.md`](zed_eval/README.md) for supported benchmarks, remote
builds, Modal setup, reporting, rejudging, baselines, and Harbor/Pier installed
agent usage.
