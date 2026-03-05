# Zed's Feature Development Process

This is for moderate-to-large features — new UI, behavior changes, or work that cuts across multiple parts of Zed. Small keybindings or settings tweaks don't need all of this.

> **Before you start:** If you're an external contributor, make sure the feature is something the team wants before investing significant effort. That said, coming prepared with background research makes it much easier for the team to understand and approve the proposal. Read the [Contributing guide](../../../CONTRIBUTING.md#sending-changes) — if there isn't already a GitHub issue with staff confirmation, start with a GitHub Discussion or a Discord message rather than a PR.

## 1. Why does this matter?

Every feature starts as an idea. Before writing any code, ground it:

- **What problem does this solve?**
- **What's the evidence?** GitHub issues, Discord requests, thumbs-up counts, blog posts.
- **Is there prior art?** If it's in VS Code, JetBrains, Neovim, or a wildly popular plugin, that's a strong signal. If the idea is more novel, name what it's based on — "This is X, adapted for Zed's multi-buffers" is far more useful than "I think this would be cool."

## 2. What is it?

Write a short, concrete feature statement, then back it up with the context gathered above. If you can't describe the feature in a few sentences, it might be too big or too vague.

Here's an example format, though adapt it to whatever your feature needs:

> **Feature:** Inline Git Blame
> **Purpose:** Show the last commit author and message for each line directly after the editor text, so developers can understand code history without opening the git blame.
> **Background:**
> This is standard across all major code editors
> \[screenshot of VSCode]
> \[screenshot of Intellij]
> \[screenshot of Neovim]
> and has 146 thumbs up on the [github issue](https://github.com).
> **Decisions:**
> We have to decide whether to use the git CLI or a git library. Zed uses a git library but its blame implementation is too slow for a code editor, so we should use the CLI's porcelain interface.

## 3. What else does this affect?

Walk through this list before you start building. Not everything will apply:

- **Actions & keybindings.** What actions does your feature define? Do the default keybindings conflict with existing ones?
- **Settings.** Is any behavior configurable? Per-user vs. per-project vs. per-language? Don't forget to add new settings to the Settings UI.
- **Themes & styling.** Does this need a new semantic token? Does it look right in both light and dark mode?
- **Vim mode.** Vim users might have different expectations for this feature.
- **Remote development.** Does your feature work with remote projects? File paths, shell commands, and environment variables all might behave differently.
- **Persistence across restarts.** Should your feature's state persist across restarts?
- **Accessibility.** Is it keyboard-navigable? Are focus states clear?
- **Platform differences.** Does behavior differ on macOS, Linux, or Windows?
- **Performance.** How does it behave with large files or big projects? Are interactions instant?
- **Security.** How does this feature interact with Workspace Trust? Does it open new attack surfaces in Zed?

If your feature touches the **editor** specifically: the editor has a lot of coexisting features — gutter elements, inline blocks, multiple cursors, folding, edit predictions, code intelligence popovers, the minimap. Test your changes with different combinations of them active. Features that work in a normal buffer might need to be disabled in a multi-buffer.

## 4. Ship it

Use this as the basis for your GitHub Discussion, issue, or PR description. Good product research gets everyone aligned on goals, the state of the art, and any tradeoffs we might need to consider.
