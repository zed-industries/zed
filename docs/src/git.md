# Git

Zed currently offers a set of fundamental Git features, with support for more advanced ones—like conflict resolution tools, line by line staging, and more—coming in the future.

Here's an overview of all currently supported features:

- Committing
- Staging, pushing, pulling, and fetching
- Changeset diff view
- Diff indicators in buffers and editor scrollbars
- Inline diff toggle and reverts in the editor for unstaged changes
- Git status in the project panel
- Branch creating and switching
- Git blame viewing

## Git Panel

The Git Panel gives you a birds-eye view of the state of your working tree and of Git's staging area.

You can open the Git Panel using {#action git_panel::ToggleFocus}, or by clicking the git icon in the status bar.

In the panel you can see  the state of your project at a glance – which repository and branch are active, what files have changed and the current staging state of each file.

Zed monitors your repository so that changes you make on the command line are instantly reflected.

<!-- Add media and keybinding -->

### Fetch, push, and pull

You can fetch, push, or pull from your Git repository in Zed via the buttons available on the Git Panel or via the Command Palette by looking at the respective actions: {#action git::Fetch}, {#action git::Push}, and {#action git::Pull}.

### Staging Workflow

Zed has two primary staging workflows: Staging using the Project Diff, and using the panel directly.

#### Using the Project Diff

#### Using the Panel

From the panel you can simply type a commit message and hit the commit button, or {#action git::Commit}. This will automatically all tracked files (indicated by a `[·]` in the entry's checkbox) and commit them.

<!-- Show a set of changes with default staged -->

Entries can be staged using each individual entry's checkbox.


## Diff View

You can see all of the changes captured by Git in Zed by opening the Diff View, accessible via the {#action git::Diff} action in the Command Palette or the Git Panel.

All of the changes displayed in the Diff View behave exactly the same as any other multibuffer: they are all editable excerpts of files.

You can stage or unstage each hunk as well as a whole file by hitting the buttons on the tab bar or their corresponding keybindings.

<!-- Add media and keybinding -->

## Git with AI

Zed currently supports LLM-powered commit message generation. This can be done when focused on the commit message editor in the Git Panel.

> Note that you need to have an LLM provider configured.

<!-- Add media and keybinding -->

More advanced AI integration with Git features may come in the future.

<!--
## Git Hunk Navigation

TBD: Explain Git Hunks

- Navigating hunks
- Expanding hunks
- Reverting hunks
-->

## Git Integrations

Zed integrates with popular Git hosting services to ensure that Git commit hashes and references to Issues / Pull Requests / Merge Requests become clickable links.

Zed currently support links to the hosted versions of
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

## All Commands

Not all actions have default keybindings, but can be bound by [customizing your keymap](/key-bindings.md#user-keymaps).

| Action | Keybinding |
|--------|-------------|
| {#action git::Add} | {#kb git::Add} |
| {#action git::Push} | {#kb git::Push} |
| {#action git::Pull} | {#kb git::Pull} |
| {#action git::Diff} | {#kb git::Diff} |
| {#action git::Fetch} | {#kb git::Fetch} |
| {#action git::Switch} | {#kb git::Switch} |
| {#action git::Commit} | {#kb git::Commit} |
| {#action git::Branch} | {#kb git::Branch} |
| {#action git::Restore} | {#kb git::Restore} |
| {#action git::StageAll} | {#kb git::StageAll} |
| {#action git::ForcePush} | {#kb git::ForcePush} |
| {#action git::UnstageAll} | {#kb git::UnstageAll} |
| {#action git::RestoreFile} | {#kb git::RestoreFile} |
| {#action git::ToggleStaged} | {#kb git::ToggleStaged} |
| {#action git::StageAndNext} | {#kb git::StageAndNext} |
| {#action git::CheckoutBranch} | {#kb git::CheckoutBranch} |
| {#action git::UnstageAndNext} | {#kb git::UnstageAndNext} |
| {#action git::ExpandCommitEditor} | {#kb git::ExpandCommitEditor} |
| {#action editor::ToggleGitBlame} | {#kb editor::ToggleGitBlame} |
| {#action editor::ToggleGitBlameInline} | {#kb editor::ToggleGitBlameInline} |
