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

## Git Panel

The Git Panel gives you a birds-eye view of the state of your working tree and of Git's staging area.

You can open the Git Panel using {#action git_panel::ToggleFocus}, or by clicking the Git icon in the status bar.

In the panel you can see the state of your project at a glance—which repository and branch are active, what files have changed and the current staging state of each file.

Zed monitors your repository so that changes you make on the command line are instantly reflected.

### Git Panel Settings

The Git Panel can be customized through your settings file. You can configure its appearance, behavior, and default states.
To customize the Git Panel, add a git_panel object to your settings.json:

```
{
  "git_panel": {
    "collapse_untracked_files_section": true
  }
}
```

The `collapse_untracked_files_section` setting is particularly useful for projects with many generated or temporary files. When enabled, the untracked files section shows only a header with a chevron icon that you can click to expand and view the files.
You can also toggle the untracked section visibility using the `git_panel::ToggleUntrackedSection` action, which can be bound to a custom keybinding in your keymap.

```
{
  "bindings": {
    "cmd-alt-u": "git_panel::ToggleUntrackedSection"
  }
}
```

Important: This action works when the Git Panel is focused! If you want to toggle the untracked section from anywhere in Zed, you can use:

```
{
  "bindings": {
      "cmd-alt-shift-f": "git_panel::ToggleFocus",
      "cmd-alt-shift-u": "git_panel::ToggleUntrackedSection"
  }
}
```

This way you can ensure the Git Panel is focused before toggling the untracked section.

Available Git Panel settings include:

- `button`: Whether to show the Git Panel button in the status bar (default: true)
- `dock`: Where to dock the panel, either "left" or "right" (default: "left")
- `default_width`: Width of the panel in pixels (default: 360)
- `status_style`: How to display file status indicators, either "icon" or "label_color" (default: "icon")
- `fallback_branch_name`: Default branch name when Git's init.defaultBranch is not set (default: "main")
- `sort_by_path`: Sort entries by file path instead of by status (default: false)
- `collapse_untracked_files_section`: Whether untracked files should be collapsed by default (default: false)

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

## AI Support in Git

Zed currently supports LLM-powered commit message generation.
You can ask AI to generate a commit message by focusing on the message editor within the Git Panel and either clicking on the pencil icon in the bottom left, or reaching for the {#action git::GenerateCommitMessage} ({#kb git::GenerateCommitMessage}) keybinding.

> Note that you need to have an LLM provider configured. Visit [the AI configuration page](./ai/configuration.md) to learn how to do so.

<!-- Add media -->

More advanced AI integration with Git features may come in the future.

## Git Integrations

Zed integrates with popular Git hosting services to ensure that Git commit hashes and references to Issues, Pull Requests, and Merge Requests become clickable links.

Zed currently supports links to the hosted versions of
[GitHub](https://github.com),
[GitLab](https://gitlab.com),
[Bitbucket](https://bitbucket.org),
[SourceHut](https://sr.ht) and
[Codeberg](https://codeberg.org).

Zed also has a Copy Permalink feature to create a permanent link to a code snippet on your Git hosting service.
These links are useful for sharing a specific line or range of lines in a file at a specific commit.
Trigger this action via the [Command Palette](./getting-started.md#command-palette) (search for `permalink`),
by creating a [custom key bindings](key-bindings.md#custom-key-bindings) to the
`editor::CopyPermalinkToLine` or `editor::OpenPermalinkToLine` actions
or by simply right clicking and selecting `Copy Permalink` with line(s) selected in your editor.

## Action Reference

| Action                                 | Keybinding                         |
| -------------------------------------- | ---------------------------------- |
| {#action git::Add}                     | {#kb git::Add}                     |
| {#action git::StageAll}                | {#kb git::StageAll}                |
| {#action git::UnstageAll}              | {#kb git::UnstageAll}              |
| {#action git::ToggleStaged}            | {#kb git::ToggleStaged}            |
| {#action git::StageAndNext}            | {#kb git::StageAndNext}            |
| {#action git::UnstageAndNext}          | {#kb git::UnstageAndNext}          |
| {#action git::Commit}                  | {#kb git::Commit}                  |
| {#action git::ExpandCommitEditor}      | {#kb git::ExpandCommitEditor}      |
| {#action git::Push}                    | {#kb git::Push}                    |
| {#action git::ForcePush}               | {#kb git::ForcePush}               |
| {#action git::Pull}                    | {#kb git::Pull}                    |
| {#action git::Fetch}                   | {#kb git::Fetch}                   |
| {#action git::Diff}                    | {#kb git::Diff}                    |
| {#action git::Restore}                 | {#kb git::Restore}                 |
| {#action git::RestoreFile}             | {#kb git::RestoreFile}             |
| {#action git::Branch}                  | {#kb git::Branch}                  |
| {#action git::Switch}                  | {#kb git::Switch}                  |
| {#action git::CheckoutBranch}          | {#kb git::CheckoutBranch}          |
| {#action editor::ToggleGitBlame}       | {#kb editor::ToggleGitBlame}       |
| {#action editor::ToggleGitBlameInline} | {#kb editor::ToggleGitBlameInline} |

> Not all actions have default keybindings, but can be bound by [customizing your keymap](./key-bindings.md#user-keymaps).
