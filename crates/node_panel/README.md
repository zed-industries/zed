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
| Quick Actions | **Install** with the project's own package manager, **Task Chain** and **Package Manager** |
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
- **The choice is per workspace.** Each window's workspace has its own:
  - **Auto** (the default): the version the project's `.nvmrc`,
    `.node-version` or `engines.node` in `package.json` asks for, when that
    version is installed; otherwise the IDE default, the first version
    installed. An SPFx project with none of those is read from its
    `.yo-rc.json`: the SharePoint generator records the Node.js it was run
    with there (`nodeVersion`), and its major version is used, so
    `22.23.0` means the newest Node 22 installed.
  - **System**: the Node.js already on the machine, however it got there.
    Clicking it hands the choice back without removing anything.
  - **A version**: clicking one pins it for this workspace, whatever the
    project asks for.
- The header says what is in use and why: `pinned`, `project file`,
  `IDE default` or `system`. A project whose request doesn't match shows
  **Use** when the matching version is installed (pinning it), or **Install**
  when it is missing. A pinned version that has been removed is reported, and
  Auto is used until it is pinned again or reinstalled.
- A Node project that asks for no version at all, or only through
  `.yo-rc.json`, shows **Create .nvmrc**, which writes a version into the
  project's `.nvmrc`: the `.yo-rc.json` major version (`22`) for an SPFx
  project, otherwise the version in use. Only this IDE reads `.yo-rc.json`;
  nvm, fnm, CI and other editors read `.nvmrc`. An existing `.nvmrc` is never
  overwritten.

Choosing changes what the IDE starts in this workspace: its terminals, the
Script Runner, workflow actions and this panel's own commands. Each workspace
has a link of its own in the store, first on their `PATH`, so a switch reaches
the workspace's terminals that are already open, at their next command, and
no other window's. A program that is already running (a dev server) keeps the
Node.js it started with. Nothing outside the IDE is switched. The choice is
remembered per workspace folder. The versions are kept in a `node_versions`
folder in the IDE's data directory; with no workspace open, the panel picks
the IDE default instead.

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

- **Install** runs the selected project's own package manager: `npm`, `yarn`,
  `pnpm` or `bun install`, decided by the `packageManager` field in
  `package.json`, else its lock file, looked for up to the workspace root (a
  pnpm or Yarn workspace keeps it there); npm when there is neither. The
  button says which, as **Install (pnpm)**. With lock files of more than one
  kind it asks which to use, for that install only. When the tool isn't on
  the `PATH` the install would get, the button shows a warning and suggests
  `corepack enable` (Yarn, pnpm); nothing is installed for you. Scripts are
  run from the **Scripts** section, which lists the ones the project has.
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
