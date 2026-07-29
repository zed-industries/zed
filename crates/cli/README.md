# Cli

This crate ships two things:

- The `cli` library: the IPC protocol (`CliRequest`/`CliResponse`) spoken
  between the terminal client and the Zed editor.
- The `cli` binary: a shim that locates the sibling `zed` binary and
  forwards the invocation to it verbatim.

All actual CLI logic (argument handling, launching/contacting the editor,
relaying output, `--wait`, etc.) lives in the `zed` binary itself, in
`crates/zed/src/cli_client.rs`.

## Testing

Build the main zed binary and invoke it with CLI arguments directly:

```
cargo build -p zed
./target/debug/zed --wait path/to/file
```

To test the shim itself, build both binaries and run the shim; it will find
`./zed` next to itself in the target directory:

```
cargo build -p zed -p cli
./target/debug/cli --version
```
