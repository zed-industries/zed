# Workspace Persistence

Zed creates local SQLite databases to persist data relating to its workspace and your projects. These databases store, for instance, the tabs and panes you have open in a project, the scroll position of each open file, the list of all projects you've opened (for the recent projects modal picker), etc. You can find and explore these databases in the following locations:

- macOS: `~/Library/Application Support/Zed`
- Linux: `~/.local/share/Zed`
- Windows: `%LOCALAPPDATA%\Zed`

The naming convention of these databases takes on the form of `0-<zed_channel>`:

- Stable: `0-stable`
- Preview: `0-preview`

**If you encounter workspace persistence issues in Zed, deleting the database and restarting Zed often resolves the problem, as the database may have been corrupted at some point.** If your issue continues after restarting Zed and regenerating a new database, please [file an issue](https://github.com/zed-industries/zed/issues/new?template=10_bug_report.yml).
