# Eval

This eval assumes the working directory is the root of the repository. Run it with:

```sh
cargo run -p eval
```

The eval will optionally read a `.env` file in `crates/eval` if you need it to set environment variables, such as API keys.

## Explorer Tool

The explorer tool generates a self-contained HTML view from one or more thread
JSON file. It provides a visual interface to explore the agent thread, including
tool calls and results. See [./docs/explorer.md](./docs/explorer.md) for more details.

### Usage

```sh
cargo run -p eval --bin explorer -- --input <path-to-json-files> --output <output-html-path>
```

Example:

```sh
cargo run -p eval --bin explorer -- --input ./runs/2025-04-23_15-53-30/fastmcp_bugifx/*/last.messages.json --output /tmp/explorer.html
```
