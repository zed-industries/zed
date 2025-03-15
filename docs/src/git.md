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
You can see at a glance which files have changed, and which are staged for commit.
Zed monitors your repository so that changes you make on the command line are instantly reflected.

<!-- Add media and keybinding -->

### Fetch, push, and pull

You can fetch, push, or pull from your Git repository in Zed via the buttons available on the Git Panel or via the Command Palette by looking at the respective actions: `git fetch`, `git push`, and `git pull`.

## Diff View

You can see all of the changes captured by Git in Zed by opening the Diff View, accessible via the `git: diff` action in the Command Palette or the Git Panel.

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
