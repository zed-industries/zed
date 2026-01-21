---
description: Zed is a text editor that supports lots of Git features
title: Zed Editor Git integration documentation
---

# Git

Zed has built-in Git support for the most common workflows: committing, staging, branching, stashing, and viewing history. You can do most day-to-day Git operations without leaving the editor.

Zed doesn't yet have conflict resolution tools or line-by-line staging—for those, you'll need to use the command line or another tool.

**What you can do in Zed:**

- Commit, stage, push, pull, and fetch
- View all changes in Project Diff (an editable multibuffer)
- See diff indicators in buffers and scrollbars
- Expand inline diffs and revert changes directly in the editor
- Create, switch, and delete branches
- View Git blame and file history
- Stash, pop, apply, and drop changes
- Manage remotes

## Git Panel

The Git Panel gives you a birds-eye view of the state of your working tree and of Git's staging area.

You can open the Git Panel using {#action git_panel::ToggleFocus}, or by clicking the Git icon in the status bar.

In the panel you can see the state of your project at a glance—which repository and branch are active, what files have changed and the current staging state of each file.

Zed monitors your repository so that changes you make on the command line are instantly reflected.

### Configuration

Open the Settings Editor (`Cmd+,` on macOS, `Ctrl+,` on Linux/Windows) and search for "git" to see all available options.

**Git Panel settings** (`git_panel` in JSON):

- `dock`: Where to position the panel (`left`, `right`, or `bottom`)
- `default_width`: Panel width in pixels
- `status_style`: Show file status as `icon` or `letter`
- `sort_by_path`: Sort entries by path instead of status
- `tree_view`: Show entries as a tree instead of a flat list
- `collapse_untracked_diff`: Collapse untracked files in diffs
- `fallback_branch_name`: Default branch name when `git init` doesn't specify one
- `button`: Show/hide the Git Panel button in the status bar

**Git integration settings** (`git` in JSON):

- `git_gutter`: Show diff indicators in the gutter (`tracked_files` or `hide`)
- `gutter_debounce`: Milliseconds to wait before updating gutter indicators
- `inline_blame`: Show blame info on the current line (with `enabled`, `delay_ms`, and formatting options)
- `hunk_style`: How to display diff hunks (`staged_hollow`, `solid`, etc.)
- `path_style`: Display paths as `file_name_first` or `file_path_first`

**Commit message formatting** (under `languages."Git Commit"`):

- `preferred_line_length`: Hard wrap width (default: `72`)
- `soft_wrap`: How text wraps while typing (`editor_width`, `none`, `preferred_line_length`, `bounded`)

```json
{
  "git_panel": {
    "dock": "left",
    "status_style": "icon",
    "tree_view": true
  },
  "git": {
    "inline_blame": {
      "enabled": true,
      "delay_ms": 500
    }
  },
  "languages": {
    "Git Commit": {
      "preferred_line_length": 72
    }
  }
}
```

## Project Diff

You can see all of the changes captured by Git in Zed by opening the Project Diff ({#kb git::Diff}), accessible via the {#action git::Diff} action in the Command Palette or the Git Panel.

All of the changes displayed in the Project Diff behave exactly the same as any other multibuffer: they are all editable excerpts of files.

You can stage or unstage each hunk as well as a whole file by hitting the buttons on the tab bar or their corresponding keybindings.

<!-- Add media -->

### Word Diff Highlighting

By default, Zed highlights changed words within modified lines to make it easier to spot exactly what changed. You can disable this per-language using the `word_diff_enabled` setting:

```json
{
  "languages": {
    "Markdown": {
      "word_diff_enabled": false
    }
  }
}
```

To disable word diff highlighting globally:

```json
{
  "word_diff_enabled": false
}
```

## File History

File History shows the commit history for an individual file, letting you browse how a file has changed over time.

To open File History:

- Right-click on a file in the Project Panel and select "Open File History"
- Right-click on an editor tab and select "Open File History"
- Use the Command Palette and search for "file history"

The File History view displays a list of commits that modified the file. Select a commit to see the diff showing what changed in that commit.

## Fetch, Push, and Pull

Fetch, push, or pull from your Git repository in Zed via the buttons available on the Git Panel or via the Command Palette by looking at the respective actions: {#action git::Fetch}, {#action git::Push}, and {#action git::Pull}.

### Push Configuration

Zed respects Git's push configuration. When pushing, Zed checks the following in order:

1. `pushRemote` configured for the current branch
2. `remote.pushDefault` in your Git config
3. The branch's tracking remote

This matches Git's standard behavior, so if you've configured `pushRemote` or `pushDefault` in your `.gitconfig` or via `git config`, Zed will use those settings.

## Remotes

Zed supports managing Git remotes directly from the interface. You can view, add, and work with multiple remotes for your repository.

To manage remotes, use the Command Palette and search for remote-related actions, or use the Git Panel's remote selector when pushing or fetching.

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

## Branch Management

### Creating and Switching Branches

Create a new branch using {#action git::Branch} or switch to an existing branch using {#action git::Switch} or {#action git::CheckoutBranch}.

### Deleting Branches

To delete a branch, open the branch switcher with {#action git::Switch}, find the branch you want to delete, and use the delete option. Zed will confirm before deleting to prevent accidental data loss.

> **Note:** You cannot delete the branch you currently have checked out. Switch to a different branch first.

## Merge Conflicts

When you encounter merge conflicts, Zed displays conflict markers in your files with buttons to help resolve them. The conflict buttons show the actual branch names involved in the conflict (for example, "main" and "feature-branch") rather than generic labels, making it easier to understand which changes come from which branch.

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

### Self-Hosted Instances

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

Supported `provider` values are `github`, `gitlab`, and `bitbucket`. The `name` field is optional and used for display purposes.

### Permalinks

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
