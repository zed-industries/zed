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


### Where does it live in the UI?

Most features need a home. Start from what your feature *does*, and that tells you where it belongs:

| If your feature... | It belongs in a... | Examples | Trait / API |
| --- | --- | --- | --- |
| Shows rich content the user works with over time | **Tab / Item** in the central pane | Editor, preview, diagnostics view | `Item` trait |
| Provides an always-available tool the user docks to the side | **Panel** in a dock (left, bottom, right) | Project panel, terminal, git panel | `Panel` trait |
| Involves a transient interaction — pick something, then dismiss | **Modal** overlay | File finder, command palette, branch picker | `ModalView` trait |
| Augments whatever item is currently active in the pane | **Toolbar** strip above the editor | Search bar, breadcrumbs | `ToolbarItemView` trait |
| Provides a persistent, glanceable indicator | **Status bar** at the bottom of the window | Diagnostics count, git branch, language server progress | `StatusItemView` trait |
| Tells the user something happened, with an optional action | **Toast / Notification** | "Copied to clipboard", "Update available" | `Workspace::show_toast()`, `show_notification()` |
| Adds an action relevant to a specific right-click context | **Context menu** entry | "Reveal in Finder", "Copy Path", "Stage Hunk" | `ContextMenu` builder |
| Appears anchored near the cursor or a specific element | **Popover** | Hover info, completions, signature help | Popover elements |

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
| **Multi-buffer** | ...the editor is showing a multi-buffer (search results, diagnostics view)? This is a common blind spot — features that work in a normal buffer often break in multi-buffer contexts. |
| **Minimap** | ...the minimap is visible? Does your feature need to be represented there? |

This list isn't exhaustive — the editor is one of the most complex parts of Zed. Use it as a starting point, not a complete checklist.

### What systems does it plug into?

These are the cross-cutting infrastructure systems that most features eventually need:

| System | Ask yourself |
| --- | --- |
| **Actions & keybindings** | What actions does your feature define? What are the default keybindings? Do they conflict with existing ones? Actions are automatically discoverable in the command palette. |
| **Settings** | Is any behavior configurable? Global vs. per-project vs. per-language? What are the defaults? Use the `Settings` + `SettingsKey` traits and don't forget to add it to the Settings UI. |
| **Themes & styling** | Does it introduce new colored elements? Use semantic theme tokens from `cx.theme()` — don't hardcode colors. Does it look right in both light and dark mode? |
| **Serialization / persistence** | Does this feature add workspace or tab state? That state probably needs to survive restarts. Tab state uses `SerializableItem`. Other state may need the workspace DB or key-value store. |
| **Search** | Is the content searchable? Implement `SearchableItem` for find/replace support within your view. |
| **Collaboration** | Does this work when sharing a project? What do guests see? If it's a tab, does it support leader/follower via `FollowableItem`? |

### Commonly forgotten cross-cutting concerns

You'll naturally think about the features that are obviously related to yours. If you're building a Git feature, you already know to look at the git panel. These are the ones people *forget* — the non-obvious intersections that tend to surface late in review:

- **Vim mode.** Does your feature introduce keybindings or interactive UI? Vim users expect modal behavior, and bindings that work great in insert mode might conflict with normal mode. If you add a new action, consider whether it needs Vim-specific bindings.
- **Remote development.** Does your feature assume the project is local? When a project is open over SSH, some things run on the remote machine and some run locally. File paths, shell commands, and environment variables all behave differently.
- **AI / Agent context.** Does your feature produce information the agent should know about? Could there be an agent tool for it? Does it provide context that would be useful in a prompt?
- **Per-language settings.** Is the behavior the same for every language, or might users want different defaults for Rust vs. Python vs. Markdown?
- **Persistence across restarts.** If the user sets something up (a panel state, a configuration, a selection), is it gone when they reopen the workspace? Should it be?

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
