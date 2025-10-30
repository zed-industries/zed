# Debugging Crashes

When Zed panics or otherwise crashes, Zed sends a message to a sidecar process which inspects the memory of the crashing editor to create a [minidump](https://chromium.googlesource.com/breakpad/breakpad/+/master/docs/getting_started_with_breakpad.md#the-minidump-file-format) file in `~/Library/Logs/Zed` or `$XDG_DATA_HOME/zed/logs`. This minidump can be used to generate backtraces for the stacks of all threads.

If you have enabled Zed's telemetry these will be uploaded to us when you restart the app. They end up in a [Slack channel](https://zed-industries.slack.com/archives/C0977J9MA1Y) and in [Sentry](https://zed-dev.sentry.io/issues) (both of which are Zed-staff-only).

These crash reports contain rich information; but they are hard to read because they don't contain spans or symbol information. You can still work with them locally by downloading sources and an unstripped binary (or separate symbols file) for your Zed release and running:

```sh
zstd -d ~/.local/share/zed/<uuid>.dmp -o minidump.dmp
minidump-stackwalk minidump.dmp
```

Alongside the minidump file in your logs dir, there should be a `<uuid>.json` which contains additional metadata like the panic message, span, and system specs.

## Using a Debugger

If you can reproduce the crash consistently, a debugger can be used to inspect the state of the program at the time of the crash, often providing useful insights into the cause of the crash.

You can read more about setting up and using a debugger with Zed, and specifically for debugging crashes [here](./debuggers.md#debugging-panics-and-crashes)
