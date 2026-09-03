# dap_adapters

Zed's built-in debug adapters. Each module implements `DebugAdapter` and is
registered in `src/dap_adapters.rs` (`init`).

## Current adapters

| Module | Adapter name | Languages |
|--------|--------------|-----------|
| `codelldb.rs` | `CodeLLDB` | Rust, C, C++ |
| `python.rs` | `Debugpy` | Python |
| `javascript.rs` | `JavaScript` | JavaScript, TypeScript |
| `go.rs` | `Delve` | Go |
| `gdb.rs` | `GDB` | C, C++ |

## Planned adapters

Not yet implemented. Each needs an adapter module here plus a `start_session`
launch-config shape and a feature test in the sibling `zed-debugger-demo` repo
(see its `ROADMAP.md`).

- [ ] Java / Kotlin — JDT debug adapter (`org.eclipse.jdt.ls` / vscode-java)
- [ ] C# / .NET — vsdbg / OmniSharp
- [ ] Ruby — `rdbg` (debug gem) / ruby-debug-ide
- [ ] PHP — Xdebug / php-debug
