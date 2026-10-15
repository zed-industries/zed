# node_panel

A left-dock panel for working with Node.js projects: Node.js versions,
project discovery, scripts, packages and running processes.

## Why it exists

It puts the things you normally do in a terminal for a Node project — check
the Node version, switch it, run a script, see what is outdated — one click
away, and scoped to the project you have open. Ported from Forge.

## Using it

Open with `ctrl-k o` (`cmd-k o` on macOS), the status bar icon (tooltip
"Node"), or `node panel: toggle focus`.

The panel is a stack of collapsible sections:

| Section | What it does |
| --- | --- |
| Node Versions | Lists installed Node.js versions; switch, install or uninstall |
| Projects | Every directory with a `package.json` under the workspace; pick the active one |
| Quick Actions | Common commands for the active project, plus **Package Manager** and **Task Chain** |
| Packages | Installed dependencies |
| Outdated | Packages with newer versions, with **Update all (patch)** / **Update all (minor)** |
| Vulnerabilities | `npm audit` findings for the active project |
| Scripts | The `scripts` from `package.json`; run, add, edit or delete them |
| Processes | Running Node processes: what each is, this workspace's first, Stop |
| SPFx | SharePoint Framework actions, shown for SPFx projects |

### Node versions

Version management is built in. Nothing has to be installed first, neither
fnm nor nvm, and it works the same on Windows, macOS and Linux.

- **Install Version…** lists every Node.js release; long-term-support ones
  are marked. With nothing installed yet, **Install latest LTS** is one click.
- Click a version to make it the active one. The first version installed
  becomes active on its own.
- **System** is the Node.js already on the machine, however it got there. It
  is used whenever no built-in version is active, and clicking it hands the
  choice back without removing anything.
- The version in use is always the one picked here. A project that asks for a
  version in `.nvmrc`, `.node-version`, or the `engines.node` field of
  `package.json` never overrides the pick; the header line shows what it asks
  for, and offers **Use** when the matching version is installed but isn't
  the one picked, or **Install** when it is missing.

Choosing a version changes what the IDE starts: its terminals, the Script
Runner, workflow actions and this panel's own commands. They have the store's
link to the active version first on their `PATH`, so a switch reaches a
terminal that is already open, at its next command. A program that is already
running (a dev server) keeps the Node.js it started with. Nothing outside the
IDE is switched. The versions are kept in a `node_versions` folder in the
IDE's data directory.

### Processes

- **What** names each process from its command line: `next dev`,
  `npm run dev`, `gulp build`, `tsserver`, or a project script's file name.
  Hover it for the full command line. Anything in it that looks like a
  secret (`--token=…`, `NPM_TOKEN=…`, a password in a URL) shows as `***`.
- Processes started from this workspace come first. The rest of the
  machine's are folded under **Other**, including the IDE's own, marked
  **IDE** (language servers and the helpers they start). Both lists are
  heaviest first. Only `node` itself is listed, not every program with
  "node" in its name.
- **Stop** asks for confirmation in the row, then stops the process and
  everything it started (stopping `npm run dev` also stops the server npm
  ran), and says how it went: what is still running if something wouldn't
  stop, or that the process may belong to an administrator or another user.
  On Linux and macOS the processes are asked to finish first and given two
  seconds; Windows can't ask a console program to finish from outside it,
  so there they are ended at once. A row's process is checked by its start
  time as well as its number, so Stop never reaches a later process that
  the OS gave the same number. The IDE's own processes have no Stop.

"This workspace" means started in the workspace folder, running a file from
it, or started by a process that was: the server `npm run dev` runs belongs
to the workspace wherever its files are. A process the IDE started from a
terminal is not the IDE's own: the shell in between says so. When a project
is opened after the panel, the panel moves to it.

Useful to know:

- Running a script or quick action sends the command to the **Script Runner**
  panel, which opens and streams the output. It runs under the Node.js
  version described above.
- **Package Manager** opens the Node Package Manager tab
  ([`npm_manager_panel`](../npm_manager_panel/README.md)).
- **Task Chain** opens the Flows panel's task-chain wizard, pre-filled with
  this project ([`flows_panel`](../flows_panel/README.md)).
- Editing scripts in the panel writes back to `package.json`.

## How it is wired

- `node_panel::init(cx)` registers `node_panel::ToggleFocus`.
- `NodePanel::load(workspace, cx)` is awaited in `initialize_panels`
  (`crates/zed/src/zed.rs`). After it and its dependencies have loaded, the
  same function calls:
  - `set_script_runner(WeakEntity<ScriptRunnerPanel>)` — where runs are sent.
  - `set_flows_panel(WeakEntity<FlowsPanel>)` — for the Task Chain action.
- Implements `workspace::dock::Panel`, docked left by default (the
  `node_panel.dock` setting moves it left or right; `node_panel.button` hides
  its status bar button), `activation_priority() = 26`.

If you add a panel that this one must talk to, wire it in that same place
rather than at construction: the panels load concurrently, so none of them
can assume another exists when it is built.

### Read-only accessors

`dashboard_panel` reads this panel's state through public getters instead of
rescanning: `node_version()`, `detected_projects()`, `outdated_count()`,
`vulnerable_count()`, `vulnerable_findings()`, `vulnerable_loading()`,
`vulnerable_error()`, `active_project_label()` and
`rescan_vulnerabilities(cx)`.

## Where the logic lives

| Concern | Crate |
| --- | --- |
| Runtime detection, project scan | [`node_backend`](../node_backend/README.md) |
| Installing and selecting Node.js versions | `node_versions`, on the [fnm](https://github.com/Schniz/fnm) library |
| npm CLI and output parsing | [`npm_backend`](../npm_backend/README.md) |
| Running commands | [`script_runner_panel`](../script_runner_panel/README.md) |
| The process list | `runtime_processes`, shared with the Python panel |

The process list scans the machine's processes itself; Forge fed it from the
Cockpit's broadcast tick, which does not exist here. It runs every 3
seconds, and only while the list is on screen: hiding the panel or
collapsing **Processes** stops it, and showing it again refreshes at once.

## Development

```sh
cargo check -p node_panel -j 8
cargo test -p node_panel -j 8
```

The tests cover reading `package.json`, the bulk-update and run-script
commands (including refusing names containing shell syntax), and SPFx
detection against temporary directories.

To run the tests of every fork crate at once: `script/test-fork-crates.ps1`.
