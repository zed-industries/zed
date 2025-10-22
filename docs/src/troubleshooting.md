# Troubleshooting

This guide covers common troubleshooting techniques for Zed.
Sometimes you'll be able to identify and resolve issues on your own using this information.
Other times, troubleshooting means gathering the right information—logs, profiles, or reproduction steps—to help us diagnose and fix the problem.

> **Note**: To open the command palette, use `cmd-shift-p` on macOS or `ctrl-shift-p` on Windows / Linux.

## Retrieve Zed and System Information

When reporting issues or seeking help, it's useful to know your Zed version and system specifications. You can retrieve this information using the following actions from the command palette:

- {#action zed::About}: Find your Zed version number
- {#action zed::CopySystemSpecsIntoClipboard}: Populate your clipboard with Zed version number, operating system version, and hardware specs

## Zed Log

Often, a good first place to look when troubleshooting any issue in Zed is the Zed log, which might contain clues about what's going wrong.
You can review the most recent 1000 lines of the log by running the {#action zed::OpenLog} action from the command palette.
If you want to view the full file, you can reveal it in your operating system's native file manager via {#action zed::RevealLogInFileManager} from the command palette.

You'll find the Zed log in the respective location on each operating system:

- macOS: `~/Library/Logs/Zed/Zed.log`
- Windows: `C:\Users\YOU\AppData\Local\Zed\logs\Zed.log`
- Linux: `~/.local/share/zed/logs/Zed.log` or `$XDG_DATA_HOME`

> Note: In some cases, it might be useful to monitor the log live, such as when [developing a Zed extension](https://zed.dev/docs/extensions/developing-extensions).
> Example: `tail -f ~/Library/Logs/Zed/Zed.log`

The log may contain enough context to help you debug the issue yourself, or you may find specific errors that are useful when filing a [GitHub issue](https://github.com/zed-industries/zed/issues/new/choose) or when talking to Zed staff in our [Discord server](https://zed.dev/community-links#forums-and-discussions).

## Performance Issues (Profiling)

If you're running into performance issues in Zed—such as hitches, hangs, or general unresponsiveness—having a performance profile attached to your issue will help us zero in on what is getting stuck, so we can fix it.

### macOS

Xcode Instruments (which comes bundled with your [Xcode](https://apps.apple.com/us/app/xcode/id497799835) download) is the standard tool for profiling on macOS.

1. With Zed running, open Instruments
1. Select `Time Profiler` as the profiling template
1. In the `Time Profiler` configuration, set the target to the running Zed process
1. Start recording
1. If the performance issue occurs when performing a specific action in Zed, perform that action now
1. Stop recording
1. Save the trace file
1. Compress the trace file into a zip archive
1. File a [GitHub issue](https://github.com/zed-industries/zed/issues/new/choose) with the trace zip attached

<!--### Windows-->

<!--### Linux-->

## Startup and Workspace Issues

Zed creates local SQLite databases to persist data relating to its workspace and your projects. These databases store, for instance, the tabs and panes you have open in a project, the scroll position of each open file, the list of all projects you've opened (for the recent projects modal picker), etc. You can find and explore these databases in the following locations:

- macOS: `~/Library/Application Support/Zed/db`
- Linux and FreeBSD: `~/.local/share/zed/db` (or within `XDG_DATA_HOME` or `FLATPAK_XDG_DATA_HOME`)
- Windows: `%LOCALAPPDATA%\Zed\db`

The naming convention of these databases takes on the form of `0-<zed_channel>`:

- Stable: `0-stable`
- Preview: `0-preview`
- Nightly: `0-nightly`
- Dev: `0-dev`

While rare, we've seen a few cases where workspace databases became corrupted, which prevented Zed from starting.
If you're experiencing startup issues, you can test whether it's workspace-related by temporarily moving the database from its location, then trying to start Zed again.

> **Note**: Moving the workspace database will cause Zed to create a fresh one.
> Your recent projects, open tabs, etc. will be reset to "factory".

If your issue persists after regenerating the database, please [file an issue](https://github.com/zed-industries/zed/issues/new/choose).

## Language Server Issues

If you're experiencing language-server related issues, such as stale diagnostics or issues jumping to definitions, restarting the language server via {#action editor::RestartLanguageServer} from the command palette will often resolve the issue.
