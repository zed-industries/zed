# Zed Feature Process

This document is primarily for external contributors proposing or building a moderate-to-large feature. Something that introduces new UI, changes existing behavior, or cuts across multiple parts of the editor. If you're adding a keybinding, a small settings option, or a context menu entry, you probably don't need all of this. Use your judgment on which steps apply.

> **Before you start:** If you're an external contributor, make sure the feature is something the team wants before investing significant effort. That said, coming prepared with the background prep already done makes it much easier for the team to understand and approve the proposal. Read the [Contributing guide](../../../CONTRIBUTING.md#sending-changes) — if there isn't already a GitHub issue with staff confirmation, start with a GitHub Discussion or a discord message rather than a PR.

## 1. Start with the Why

Every feature begins as an idea. Before writing any code, ground that idea in context.

Ask yourself:

- **What problem does this solve?** Who hits this problem, and how often?
- **What social context supports this?** Are people asking for this in Discord? Are blog posts or tweets calling it out? Is there a lot of thumbs up on the github issue?
- **Is there prior art in other editors?** If it's in VS Code (or derivatives), JetBrains, Neovim, or there's a wildly popular plugin that implements this feature, that's a strong signal.
- **Is there a conceptual lineage?** Even if your idea is novel, it probably relates to something that already exists. "This is a reformulation of X, adapted to work with Zed's multi-buffers" is far more useful than "I think this would be cool."

You don't need all of these, but the more you have, the more confidence we and the team can have in the direction.

## 2. Define the What

Now that you know *why* this should exist, describe *what* it is. Write a short, concrete feature statement:

> **Feature:** Inline Git Blame
> **Purpose:** Show the last commit author and message for each line directly after the editor text, so developers can understand the code history without opening the git blame.

Keep this tight. If you can't describe the feature in a few sentences, it might be too big or too vague.

Then include any relevant background or context that helps the team understand why this is important:

> **Background:**
> Inline git blame is used across all major code editors:
> [screenshot of VSCode]
> [screenshot of Intellij]
> [screenshot of Neovim]
> And has 146 thumbs up on the [github issue](link-to-issue).

## 3. Map the Integration Surface

Zed has a lot of features, and many of them interact with each other. Before you start building, walk through the sections below and ask: "Does my feature involve this system or could use this UI?"

### Cross-cutting concerns

These are the systems and features that tend to come up across many different kinds of changes:

- **Actions & keybindings.** What actions does your feature define? What are the default keybindings? Do they conflict with existing ones?
- **Settings.** Is any behavior configurable? Global vs. per-project vs. per-language? What are the defaults? Don't forget to add any new settings to the Settings UI.
- **Themes & styling.** Does this need a new semantic token? Does it look right in both light and dark mode?
- **Vim mode.** Does your feature introduce keybindings or interactive UI, particularly to the editor? Vim users might need specific consideration.
- **Remote development.** Does your feature assume the project is local? When a project is open over SSH, some things run on the remote machine and some run locally. File paths, shell commands, and environment variables all behave differently.
- **Persistence across restarts.** If the user sets up some UI state (a panel, several editor tabs, the edits in those buffers), should it be preserved across restarts?

### If it touches the editor, what needs to keep working?

The editor already has a lot going on. If your feature interacts with the editing surface, these are the existing behaviors it needs to coexist with — things that should still work correctly after your change lands:

| Existing behavior | Does your feature still work when... |
| --- | --- |
| **Gutter elements** | ...the gutter already has line numbers, diff markers, breakpoints, code action lightbulbs, and runnable play buttons? If you're adding to the gutter, how does it share space? |
| **Inline / block elements** | ...there are diagnostic blocks, diff hunks, inline blame, or inlay hints injected between or within lines? |
| **Selections & cursors** | ...the user has multiple cursors? A column selection? A zero-width selection vs. a range? |
| **Scrolling** | ...the user scrolls, or something else auto-scrolls to a location? Does your feature survive scroll position changes? |
| **Code intelligence** | ...completions, hover info, code actions, or signature help are active? Do these features still work with your change in place? |
| **Edit predictions** | ...AI edit predictions (ghost text) are visible? Does your feature conflict with or obscure them? |
| **Folding** | ...code is folded? Does your feature handle folded ranges correctly, or does it break when lines are hidden? |
| **Multi-buffer** | ...the editor is showing a multi-buffer (search results, diagnostics view)? This feature might need to be adapted or deactivated when in a multi-buffer. |
| **Minimap** | ...the minimap is visible? Does your feature need to be represented there? |

### Platform & quality

| Concern | Ask yourself |
| --- | --- |
| **Accessibility** | Is it keyboard-navigable? Does it work with screen readers? Are focus states clear? |
| **Platform differences** | Does behavior differ on macOS, Linux, or Windows? Does this involve system settings (fonts, scaling, input methods)? |
| **Performance** | What's the computational complexity of this operation? How does it behave with large files or big projects? Are interactions instant? |
| **Responsive layout** | Does it work in narrow panes, small windows, split views? |

## 4. Ship It

At this point you have:

1. A clear **why** backed by evidence
2. A concise **what** — the feature and its purpose
3. A map of **cross-cutting concerns** across Zed's feature surface

What you do with this depends on where you are. If you're filing a feature proposal, this thinking becomes the body of your issue or Discussion. If you're building the feature, this is the context you bring to your PR description. If you're in a design conversation, these are the things to talk through. The format matters less than the thinking — the goal is to make review faster, scope clearer, and the final result more cohesive.

## Summary

| Step | Question to answer            | Output                           |
| ---- | ----------------------------- | -------------------------------- |
| 1    | Why does this matter?         | Problem statement and motivation |
| 2    | What is the feature?          | Feature definition with context  |
| 3    | What else does this touch?    | Integration surface map          |
| 4    | Ship it                       | PR, review, merge                |
