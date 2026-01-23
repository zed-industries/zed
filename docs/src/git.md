---
description: Zed is a text editor that supports lots of Git features
title: Zed Editor Git integration documentation
---

# Git

Zed currently offers a set of fundamental Git features, with support coming in the future for more advanced ones, like conflict resolution tools, line by line staging, and more.

Here's an overview of all currently supported features:

- Committing
- Staging, pushing, pulling, and fetching
- Project Diff: A multibuffer view of all changes
- Diff indicators in buffers and editor scrollbars
- Inline diff toggle and reverts in the editor for unstaged changes
- Git status in the Project Panel
- Branch creating and switching
- Git blame viewing
- Git stash pop, apply, drop and view

## Git Panel

The Git Panel gives you a birds-eye view of the state of your working tree and of Git's staging area.

You can open the Git Panel using {#action git_panel::ToggleFocus}, or by clicking the Git icon in the status bar.

In the panel you can see the state of your project at a glance—which repository and branch are active, what files have changed and the current staging state of each file.

Zed monitors your repository so that changes you make on the command line are instantly reflected.

### Configuration

You can configure how Zed hard wraps commit messages with the `preferred-line-length` setting of the "Git Commit" language. The default is `72`, but it can be set to any number of characters `0` or more.

The Git Panel also allows configuring the `soft_wrap` setting to adjust how commit messages display while you are typing them in the Git Panel. The default setting is `editor_width`, however, `none`, `preferred_line_length`, and `bounded` are also options.

#### Example

```json
"languages": {
  "Git Commit": {
    "soft_wrap": "editor_width",
    "preferred_line_length": 72
  },
}
```

## Project Diff

You can see all of the changes captured by Git in Zed by opening the Project Diff ({#kb git::Diff}), accessible via the {#action git::Diff} action in the Command Palette or the Git Panel.

All of the changes displayed in the Project Diff behave exactly the same as any other multibuffer: they are all editable excerpts of files.

You can stage or unstage each hunk as well as a whole file by hitting the buttons on the tab bar or their corresponding keybindings.

<!-- Add media -->

## Fetch, push, and pull

Fetch, push, or pull from your Git repository in Zed via the buttons available on the Git Panel or via the Command Palette by looking at the respective actions: {#action git::Fetch}, {#action git::Push}, and {#action git::Pull}.

## Staging Workflow

Zed has two primary staging workflows, using either the Project Diff or the panel directly.

### Using the Project Diff

In the Project Diff view, you can focus on each hunk and stage them individually by clicking on the tab bar buttons or via the keybindings {#action git::StageAndNext} ({#kb git::StageAndNext}).

Similarly, stage all hunks at the same time with the {#action git::StageAll} ({#kb git::StageAll}) keybinding and then immediately commit with {#action git::Commit} ({#kb git::Commit}).

### Using the Git Panel

From the panel, you can simply type a commit message and hit the commit button, or {#action git::Commit}. This will automatically stage all tracked files (indicated by a `[·]` in the entry's checkbox) and commit them.

<!-- Show a set of changes with default staged -->

Entries can be staged using each individual entry's checkbox. All changes can be staged using the button at the top of the panel, or {#action git::StageAll}.

<!-- Add media -->

## Committing

Zed offers two commit textareas:

1. The first one is available right at the bottom of the Git Panel. Hitting {#kb git::Commit} immediately commits all of your staged changes.
2. The second is available via the action {#action git::ExpandCommitEditor} or via hitting the {#kb git::ExpandCommitEditor} while focused in the Git Panel commit textarea.

### Undoing a Commit

As soon as you commit in Zed, in the Git Panel, you'll see a bar right under the commit textarea, which will show the recently submitted commit.
In there, you can use the "Uncommit" button, which performs the `git reset HEADˆ--soft` command.

### Configuring Commit Line Length

By default, Zed sets the commit line length to `72` but it can be configured in your local `settings.json` file.

Find more information about setting the `preferred-line-length` in the [Configuration](#configuration) section.

## Stashing

Git stash allows you to temporarily save your uncommitted changes and revert your working directory to a clean state. This is particularly useful when you need to quickly switch branches or pull updates without committing incomplete work.

### Creating Stashes

To stash all your current changes, use the {#action git::StashAll} action. This will save both staged and unstaged changes to a new stash entry and clean your working directory.

### Managing Stashes

Zed provides a comprehensive stash picker accessible via {#action git::ViewStash}. From the stash picker, you can:

- **View stash list**: Browse all your saved stashes with their descriptions and timestamps
- **Open diffs**: See exactly what changes are stored in each stash
- **Apply stashes**: Apply stash changes to your working directory while keeping the stash entry
- **Pop stashes**: Apply stash changes and remove the stash entry from the list
- **Drop stashes**: Delete unwanted stash entries without applying them

### Quick Stash Operations

For faster workflows, Zed provides direct actions to work with the most recent stash:

- **Apply latest stash**: Use {#action git::StashApply} to apply the most recent stash without removing it
- **Pop latest stash**: Use {#action git::StashPop} to apply and remove the most recent stash

### Stash Diff View

When viewing a specific stash in the diff view, you have additional options available through the interface:

- Apply the current stash to your working directory
- Pop the current stash (apply and remove)
- Remove the stash without applying changes

To open the stash diff view, select a stash from the stash picker and use the {#action stash_picker::ShowStashItem} ({#kb stash_picker::ShowStashItem}) keybinding.

## AI Support in Git

Zed currently supports LLM-powered commit message generation.
You can ask AI to generate a commit message by focusing on the message editor within the Git Panel and either clicking on the pencil icon in the bottom left, or reaching for the {#action git::GenerateCommitMessage} ({#kb git::GenerateCommitMessage}) keybinding.

> Note that you need to have an LLM provider configured either via your own API keys or through Zed's hosted AI models.
> Visit [the AI configuration page](./ai/configuration.md) to learn how to do so.

You can specify your preferred model to use by providing a `commit_message_model` agent setting.
See [Feature-specific models](./ai/agent-settings.md#feature-specific-models) for more information.

```json [settings]
{
  "agent": {
    "commit_message_model": {
      "provider": "anthropic",
      "model": "claude-3-5-haiku"
    }
  }
}
```

To customize the format of generated commit messages, run {#action agent::OpenRulesLibrary} and select the "Commit message" rule on the left side.
From there, you can modify the prompt to match your desired format.

<!-- Add media -->

Any specific instructions for commit messages added to [Rules files](./ai/rules.md) are also picked up by the model tasked with writing your commit message.

## Git Integrations

Zed integrates with popular Git hosting services to ensure that Git commit hashes and references to Issues, Pull Requests, and Merge Requests become clickable links.

Zed currently supports links to the hosted versions of
[GitHub](https://github.com),
[GitLab](https://gitlab.com),
[Bitbucket](https://bitbucket.org),
[SourceHut](https://sr.ht) and
[Codeberg](https://codeberg.org).

For self-hosted GitHub, GitLab, or Bitbucket instances, add them to the `git_hosting_providers` setting so commit hashes and permalinks resolve to your domain:

```json [settings]
{
  "git_hosting_providers": [
    {
      "provider": "gitlab",
      "name": "Corp GitLab",
      "base_url": "https://git.example.corp"
    }
  ]
}
```

Zed also has a Copy Permalink feature to create a permanent link to a code snippet on your Git hosting service.
These links are useful for sharing a specific line or range of lines in a file at a specific commit.
Trigger this action via the [Command Palette](./getting-started.md#command-palette) (search for `permalink`),
by creating a [custom key bindings](key-bindings.md#custom-key-bindings) to the
`editor::CopyPermalinkToLine` or `editor::OpenPermalinkToLine` actions
or by simply right clicking and selecting `Copy Permalink` with line(s) selected in your editor.

## Diff Hunk Keyboard Shortcuts

When viewing files with changes, Zed displays diff hunks that can be expanded or collapsed for detailed review:

- **Expand all diff hunks**: {#action editor::ExpandAllDiffHunks} ({#kb editor::ExpandAllDiffHunks})
- **Collapse all diff hunks**: Press `Escape` (bound to {#action editor::Cancel})
- **Toggle selected diff hunks**: {#action editor::ToggleSelectedDiffHunks} ({#kb editor::ToggleSelectedDiffHunks})
- **Navigate between hunks**: {#action editor::GoToHunk} and {#action editor::GoToPreviousHunk}

> **Tip:** The `Escape` key is the quickest way to collapse all expanded diff hunks and return to an overview of your changes.

## Action Reference

| Action                                    | Keybinding                            |
| ----------------------------------------- | ------------------------------------- |
| {#action git::Add}                        | {#kb git::Add}                        |
| {#action git::StageAll}                   | {#kb git::StageAll}                   |
| {#action git::UnstageAll}                 | {#kb git::UnstageAll}                 |
| {#action git::ToggleStaged}               | {#kb git::ToggleStaged}               |
| {#action git::StageAndNext}               | {#kb git::StageAndNext}               |
| {#action git::UnstageAndNext}             | {#kb git::UnstageAndNext}             |
| {#action git::Commit}                     | {#kb git::Commit}                     |
| {#action git::ExpandCommitEditor}         | {#kb git::ExpandCommitEditor}         |
| {#action git::Push}                       | {#kb git::Push}                       |
| {#action git::ForcePush}                  | {#kb git::ForcePush}                  |
| {#action git::Pull}                       | {#kb git::Pull}                       |
| {#action git::PullRebase}                 | {#kb git::PullRebase}                 |
| {#action git::Fetch}                      | {#kb git::Fetch}                      |
| {#action git::Diff}                       | {#kb git::Diff}                       |
| {#action git::Restore}                    | {#kb git::Restore}                    |
| {#action git::RestoreFile}                | {#kb git::RestoreFile}                |
| {#action git::Branch}                     | {#kb git::Branch}                     |
| {#action git::Switch}                     | {#kb git::Switch}                     |
| {#action git::CheckoutBranch}             | {#kb git::CheckoutBranch}             |
| {#action git::Blame}                      | {#kb git::Blame}                      |
| {#action git::StashAll}                   | {#kb git::StashAll}                   |
| {#action git::StashPop}                   | {#kb git::StashPop}                   |
| {#action git::StashApply}                 | {#kb git::StashApply}                 |
| {#action git::ViewStash}                  | {#kb git::ViewStash}                  |
| {#action editor::ToggleGitBlameInline}    | {#kb editor::ToggleGitBlameInline}    |
| {#action editor::ExpandAllDiffHunks}      | {#kb editor::ExpandAllDiffHunks}      |
| {#action editor::ToggleSelectedDiffHunks} | {#kb editor::ToggleSelectedDiffHunks} |

> Not all actions have default keybindings, but can be bound by [customizing your keymap](./key-bindings.md#user-keymaps).

## Git CLI Configuration

If you would like to also use Zed for your [git commit message editor](https://git-scm.com/book/en/v2/Customizing-Git-Git-Configuration#_core_editor) when committing from the command line you can use `zed --wait`:

```sh
git config --global core.editor "zed --wait"
```

Or add the following to your shell environment (in `~/.zshrc`, `~/.bashrc`, etc):

```sh
export GIT_EDITOR="zed --wait"
```
