# Troubleshooting

## Zed Log

Often, a good first place to look when troubleshooting any issue in Zed is the Zed log, which might contain clues about what's going wrong.
You can review the most recent 1000 lines of the log by running the {#action zed::OpenLog} command from the command palette (`cmd-shift-p` on macOS or `ctrl-shift-p` on Windows/Linux).
If you want to view the full file, you can find it at the respective location on each operating system:

- macOS: `~/Library/Logs/Zed/Zed.log`
- Windows: `C:\Users\YOU\AppData\Local\Zed\logs\Zed.log`
- Linux: `~/.local/share/zed/logs/Zed.log` or `$XDG_DATA_HOME`

> Note: In some cases, it might be useful to monitor the log live, such as when [developing a Zed extension](https://zed.dev/docs/extensions/developing-extensions).
> Example: `tail -f ~/Library/Logs/Zed/Zed.log`

The log may contain enough context to help you debug the issue yourself, or you may find specific errors that are useful when filing a [GitHub Issue](https://github.com/zed-industries/zed/issues/new/choose) or when talking to Zed staff in our [Discord server](https://zed.dev/community-links#forums-and-discussions).
