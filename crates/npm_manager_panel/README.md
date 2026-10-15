# npm_manager_panel

The Node Package Manager: a workspace tab for searching npmjs.com and
installing, updating and removing packages across every Node project in the
workspace.

## Why it exists

The Node panel's sidebar is too narrow for browsing a registry or reading a
README. This tab gives package management a full-width home with search,
per-package details and audit results.

It also has history worth knowing. The first version of this port shipped as
a hand-rolled three-tab list with no `gpui_component::setting` layout, no
resizable split and no registry search, and was judged a failed port. It was
rebuilt on the real widgets. Read
[`NPM_MANAGER_PANEL_UI_RESTORE_PLAN.md`](../gpui_component/NPM_MANAGER_PANEL_UI_RESTORE_PLAN.md)
before changing this crate, and keep the UI composed from `gpui_component`
widgets rather than `div`s that resemble them.

## Using it

Open with `ctrl-k j` (`cmd-k j` on macOS), the **Package Manager** quick
action in the Node panel, or the `npm_manager::OpenNpmManager` action from the
command palette.

On opening, the tab scans the workspace for directories containing a
`package.json` and shows one project tab per result. If nothing is found it
falls back to the workspace root, so it never opens empty.

Each project has five pages in the left sidebar:

| Page | What it does |
| --- | --- |
| General | Detected package manager, and whether your active Node/npm fits what the project requires (`.nvmrc`, `.node-version` or `engines`, naming the file). A mismatch is a warning here, and installs ask before running |
| Search | Search npmjs.com; **Install** from results, **Load more** to page |
| Installed | Installed packages; **Remove** |
| Updates | Outdated packages; **Update** |
| Vulnerabilities | `npm audit` findings; **Fix** when a fix is offered |

Selecting a package opens a resizable details pane on the right with
versions, dependencies and the **README**.

| Key (package list focused) | Action |
| --- | --- |
| `up` / `down` | Move the selection |
| `enter` | Open details for the selected package |
| `space` | Run the page's primary action (Install / Remove / Update / Fix) |

Install, update and remove commands run in the **Script Runner** panel, which
opens so you can watch the output. The lists reload when the command
finishes.

If a package name or version contains anything other than letters, digits and
a few punctuation characters, the command is not run and the reason is shown
in the tab instead. Real registry names never trip this; it exists so that a
malformed or hostile value cannot be run as part of a shell command. See
[`script_runner_panel`](../script_runner_panel/README.md#building-commands-safely).

## How it is wired

- `npm_manager_panel::init(cx)` registers the tab as a serializable item
  (restored with the workspace), the actions, and the list key bindings
  (context `NpmPackageList`).
- `npm_manager_panel::open(workspace, window, cx)` is the programmatic entry
  point.
- `NpmManagerPanel` is a `workspace::Item` — a tab in the active pane, not a
  dock panel.

### Actions

All in the `npm_manager` namespace: `OpenNpmManager`, `ReloadNpmManager`,
`SelectNextPackage`, `SelectPrevPackage`, `OpenSelectedPackage`,
`ActSelectedPackage`.

## Layout

| File | Contents |
| --- | --- |
| `src/npm_manager_panel.rs` | The `Item`, project tabs, actions, running commands |
| `src/pages.rs` | The five `SettingPage`s |
| `src/search.rs` | HTTP calls to `registry.npmjs.org` and the downloads API |
| `src/details.rs` | The right-hand details pane |

Parsing and CLI work live in [`npm_backend`](../npm_backend/README.md);
project discovery is `node_backend::scan_node_projects`.

## Development

```sh
cargo check -p npm_manager_panel -j 8
cargo test -p npm_manager_panel -j 8
```

The tests cover the install, remove and "Update all" commands (including
refusing names or versions containing shell syntax), list navigation and
number formatting.

To run the tests of every fork crate at once: `script/test-fork-crates.ps1`.
