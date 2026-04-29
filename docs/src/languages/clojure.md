---
title: Clojure
description: "Configure Clojure language support in Zed, including language servers, formatting, and debugging."
---

# Clojure

Clojure support is available through the [Clojure extension](https://github.com/zed-extensions/clojure).

- Tree-sitter: [prcastro/tree-sitter-clojure](https://github.com/prcastro/tree-sitter-clojure)
- Language Server: [clojure-lsp/clojure-lsp](https://github.com/clojure-lsp/clojure-lsp)

## nREPL

Zed ships a built-in [nREPL](https://nrepl.org/) client so you can evaluate
forms, selections, and whole files from inside the editor and see the results
inline — the workflow Clojure developers expect from CIDER, Calva, and
Cursive.

The MVP connects to an nREPL server you start yourself (e.g. with `clj
-M:nrepl`, `lein repl`, `bb nrepl-server`, or shadow-cljs); Zed does not start
the server for you. ClojureScript via Piggieback, CIDER middleware ops
(completion, info, debug, …), and remote/SSH transports are out of scope for
v1.

### Connecting

1. Start an nREPL server in your project. Most build tools write the chosen
   port to a `.nrepl-port` file in the project root. For example:

   ```sh
   # Leiningen
   lein repl

   # tools.deps with the standard :nrepl alias
   clj -M:nrepl

   # Babashka
   bb nrepl-server

   # shadow-cljs (CLJS, not yet wired up — see notes below)
   shadow-cljs server
   ```

2. Run the `nrepl: connect` command from the command palette. Zed walks the
   workspace's local worktrees, finds the first `.nrepl-port`, and connects
   over TCP. The **nREPL Sessions** panel opens automatically and shows the
   connection's state (`Resolving…` → `Connecting` → `Connected` / `Failed`).

3. To disconnect, run `nrepl: disconnect`. Quitting Zed also tears down any
   live connections.

If you have multiple worktrees with `.nrepl-port` files, the first one in
worktree order wins. There is no manual host:port picker yet — start the
server in the project you want to connect to.

### Evaluating code

With a connection up, the following actions are available from the command
palette in any Clojure buffer:

| Action                    | What it does                                                     |
| ------------------------- | ---------------------------------------------------------------- |
| `nrepl: eval`             | Evaluates the top-level form under the cursor                    |
| `nrepl: eval selection`   | Evaluates the current selection                                  |
| `nrepl: eval buffer`      | Sends the entire buffer via `load-file` (refuses on dirty files) |
| `nrepl: load file`        | Same as `eval buffer`                                            |
| `nrepl: interrupt`        | Cancels the most recent in-flight eval                           |
| `nrepl: switch namespace` | Re-parses the buffer's `(ns ...)` form                           |
| `nrepl: clear outputs`    | Removes all result blocks and inlays for the current editor      |

Results render below the evaluated form as a block. Short single-line
values collapse into an end-of-line inlay once evaluation finishes; empty
side-effect results disappear, leaving only a green gutter highlight on
the evaluated lines. Editing the source range invalidates the result and
removes its block.

`nrepl: eval buffer` and `nrepl: load file` require the buffer to be
saved first — `:op "load-file"` needs the on-disk path. CIDER behaves the
same way; save the buffer and try again.

v1 ships with **no default keybindings**. Bind the actions to whatever you
prefer in your `keymap.json`, for example:

```json
[
  {
    "context": "Editor && nrepl",
    "bindings": {
      "ctrl-c ctrl-c": "nrepl::Eval",
      "ctrl-c ctrl-r": "nrepl::EvalSelection",
      "ctrl-c ctrl-k": "nrepl::LoadFile",
      "ctrl-c ctrl-b": "nrepl::Interrupt"
    }
  }
]
```

The `nrepl` context is added to editors automatically when nREPL is
enabled.

### Namespace handling

Each editor caches the namespace declared by its first `(ns ...)` form
and sends it as `:ns` on every eval request. If no `(ns ...)` is present,
the server-default `user` namespace is used. The cached namespace is
refreshed before each eval; tracking `(in-ns ...)` calls mid-buffer is
not supported in v1.

### Sessions

By default Zed uses one nREPL session per workspace, shared by every
editor. This matches CIDER's default and gives you the `def`/`*1`/`*2`/
`*3` continuity you'd expect — a `def` in `core.clj` is visible from
`core_test.clj`. Per-editor isolated sessions are not supported in the
MVP.

### Settings

The `nrepl` settings live at the top level of `settings.json`:

```json
{
  "nrepl": {
    // Whether the nREPL feature is enabled. Disabling removes the
    // `nrepl::*` actions from the command palette and the `nrepl` keymap
    // context from editors.
    "enabled": true,
    // Default host used when connecting to a `.nrepl-port`-discovered
    // server. Only `localhost` and IP addresses are honored in v1.
    "default_host": "localhost",
    // Reserved for a future "auto-connect on workspace open" path.
    // Currently unused; you must run `nrepl::Connect` manually.
    "auto_connect": true,
    // File name (relative to the worktree root) that auto-discovery
    // looks for. The default matches what every common Clojure build
    // tool writes.
    "port_file": ".nrepl-port"
  }
}
```

### Limitations

The MVP is intentionally narrow. The following are out of scope and
tracked for follow-up work:

- CIDER middleware ops (`info`, `complete`, `eldoc`, `test`, `debug`,
  `classpath`, `ns-list`).
- Starting an nREPL server from inside Zed.
- ClojureScript via Piggieback / shadow-cljs nREPL.
- Remote, SSH, and WSL transports. Localhost TCP only.
- TLS / authenticated nREPL.
- Server-side pretty-printing configuration (results are `pr-str`'d).
- Tracking `(in-ns ...)` mid-buffer.

<!--
TBD: Add some Clojure Docs
-->
