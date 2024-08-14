# Git

Zed currently supports the following Git features:

- Diff indicators in buffers and editor scrollbars
- Inline diff toggle and reverts in the editor for unstaged changes
- Git status in the project panel
- Branch creating and switching
- Git blame viewing

More advanced Git features—like staging and committing changes or viewing history within Zed—will be coming in the future.

<!--
## Git Hunk Navigation

TBD: Explain Git Hunks

- Navigating hunks
- Expanding hunks
- Reverting hunks
-->

## Git Integrations

Zed integrates with popular Git hosting services to ensure that git commit hashes
and references to Issues / Pull Requests / Merge Requests become clickable links.
Zed currently support links to
[GitHub.com](https://github.com),
[GitLab.com](https://gitlab.com),
[Bitbucket.org](https://bitbucket.org),
[SourceHut.org](https://sr.ht) and
[Codeberg.org](https://codeberg.org).

Zed also has a Copy Permalink feature to create a permanent link to a code snippet on your Git hosting service.
These links are useful for sharing a specific line or range of lines in a file at a specific commit.
Trigger this action via the [Command Palette](./getting-started.md#command-palette) (search for `permalink`),
by creating a [custom key bindings](key-bindings.md#custom-key-bindings) to the
`editor::CopyPermalinkToLine` or `editor::OpenPermalinkToLine` actions
or by simply right clicking and selecting `Copy Permalink` with line(s) selected in your editor.
