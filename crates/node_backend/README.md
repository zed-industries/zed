# node_backend

Node.js runtime detection and project scanning, with no UI.

## Why it exists

The Node panel needs to know which Node.js and npm it is running, and where
the projects in a workspace are. Keeping that logic
here — plain functions over strings and paths, with no GPUI and no
application state — means it can be shared (`npm_manager_panel` uses the same
project scanner) and tested without a window.

Ported from Forge alongside [`node_panel`](../node_panel/README.md). It used
to drive nvm-windows as well; installing and switching Node.js versions is now
built in, in [`node_versions`](../node_versions).

## Using it

Every function takes plain values and returns a plain value or
`Result<_, String>`. They block, so call them from a background task:

```rust
let projects = cx
    .background_spawn(async move {
        node_backend::scan_node_projects(&root, node_backend::MAX_SCAN_DEPTH)
    })
    .await;
```

### API

| Area | Items |
| --- | --- |
| Runtime | `query_runtime(exe)` — run `<exe> --version` style queries |
| Project scan | `scan_node_projects(root, depth)` → `Vec<DetectedNodeProject>` |
| Files | `read_text_file`, `write_file`, `read_dir`, `reveal_in_explorer` |
| Types | `DetectedNodeProject`, `DirEntry` |
| Limits | `MAX_SCAN_DEPTH` (5), `SCAN_SKIP_DIRS` |

`scan_node_projects` walks down from `root` looking for directories that
contain a `package.json`, skipping the directories in `SCAN_SKIP_DIRS`
(`node_modules` and similar) and stopping at `MAX_SCAN_DEPTH`.

## PATH handling on Windows

`src/path_env.rs` exists because a GUI-launched app on Windows inherits a
trimmed PATH that often lacks user-installed tools. `merged_path`
combines the process PATH with the `HKCU`/`HKLM` registry values (expanding
`%VAR%` references), and `enrich_path` applies the result to a
`std::process::Command`. Without it, a `node` installed on the machine is
frequently "not found" even though it works in a terminal.

The same module is duplicated in `npm_backend`, `python_backend` and
`dotnet_backend` so each backend stays free of dependencies on the others.

## Development

```sh
cargo check -p node_backend -j 8
cargo test -p node_backend -j 8
```

The tests cover the file helpers and the project scanner (against temporary
directories). Nothing in them invokes `node`.

To run the tests of every fork crate at once: `script/test-fork-crates.ps1`.
