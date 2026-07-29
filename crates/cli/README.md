# Cli

This crate contains the IPC protocol (`CliRequest`/`CliResponse`) spoken
between the terminal client mode and the GUI mode of the `zed` binary.

There is no separate CLI binary: all CLI logic (argument handling,
launching/contacting the editor, relaying output, `--wait`, etc.) lives in the
`zed` binary itself, in `crates/zed/src/cli_client.rs`. Installed `zed`
commands are symlinks (macOS/Linux) or thin `zed.cmd`/`zed` wrapper scripts
(Windows) pointing at that binary.

## Testing

Build the main zed binary and invoke it with CLI arguments directly:

```
cargo build -p zed
./target/debug/zed --wait path/to/file
```
