---
title: Clojure
description: "Configure Clojure language support in Zed, including language servers, formatting, and debugging."
---

# Clojure

Clojure support is available through the [Clojure extension](https://github.com/zed-extensions/clojure).

- Tree-sitter: [prcastro/tree-sitter-clojure](https://github.com/prcastro/tree-sitter-clojure)
- Language Server: [clojure-lsp/clojure-lsp](https://github.com/clojure-lsp/clojure-lsp)

## nREPL {#nrepl}

Zed ships a built-in [nREPL](https://nrepl.org/) client. You can start a
REPL for your project, evaluate forms inline, work in an interactive REPL
console, and get completions, documentation and go-to-definition from the
running program.

Features that use CIDER ops (completion, documentation, definitions,
stacktraces) need the [cider-nrepl](https://github.com/clojure-emacs/cider-nrepl)
middleware. Jack-in adds it for you. If you start the server yourself without
it, evaluation still works and those features are skipped.

### Jack-in {#nrepl-jack-in}

Run {#action nrepl::JackIn} to start an nREPL server for the project and
connect to it. Zed looks at the first local folder of the project and picks
the build tool from the files it finds, in this order:

| File              | Command Zed runs                                       |
| ----------------- | ------------------------------------------------------ |
| `deps.edn`        | `clojure -Sdeps ... -M -m nrepl.cmdline --port <port>` |
| `project.clj`     | `lein update-in :plugins conj ... -- repl :headless`   |
| `shadow-cljs.edn` | `npx shadow-cljs -d ... server`                        |
| `bb.edn`          | `bb nrepl-server localhost:<port>`                     |

Zed injects `nrepl` and `cider-nrepl` into the command. The **nREPL
Sessions** panel shows the command and the server's latest output while it
starts. The first jack-in of a project can take a while because the build
tool may download dependencies.

The server belongs to the connection: {#action nrepl::Disconnect}, a new
jack-in, or quitting Zed stops it.

### Connecting to a running server {#nrepl-connecting}

1. Start an nREPL server in your project. Most build tools write the chosen
   port to a `.nrepl-port` file in the project root. For example:

   ```sh
   # Leiningen
   lein repl

   # tools.deps with the standard :nrepl alias
   clj -M:nrepl

   # Babashka
   bb nrepl-server
   ```

2. Run {#action nrepl::Connect} from the command palette. Zed walks the
   project's local folders, finds the first `.nrepl-port`, and connects
   over TCP. The **nREPL Sessions** panel opens and shows the connection's
   state (`Resolving…` → `Connecting` → `Connected` / `Failed`).

3. To disconnect, run {#action nrepl::Disconnect}.

If several folders have `.nrepl-port` files, the first one in folder order
wins. There is no manual host:port picker yet.

### Evaluating code {#nrepl-evaluating-code}

| Action                           | Default keybinding       | What it does                                                       |
| -------------------------------- | ------------------------ | ------------------------------------------------------------------ |
| {#action nrepl::EvalCurrentForm} | `Ctrl+Enter`             | Evaluates the form at, or right before, the cursor                 |
| {#action nrepl::Eval}            | `Alt+Enter`              | Evaluates the top-level form under the cursor                      |
| {#action nrepl::EvalToCursor}    | `Ctrl+Alt+C Shift+Enter` | Evaluates from the start of the top-level form to the cursor       |
| {#action nrepl::EvalSelection}   |                          | Evaluates the current selection                                    |
| {#action nrepl::EvalAndComment}  | `Ctrl+Alt+C C`           | Evaluates the top-level form and inserts the result as `;; =>`     |
| {#action nrepl::EvalBuffer}      |                          | Evaluates the whole buffer as it is in the editor, saved or not    |
| {#action nrepl::LoadFile}        | `Ctrl+Alt+C Enter`       | Loads the saved file with `load-file` (refuses on unsaved changes) |
| {#action nrepl::Interrupt}       | `Ctrl+Alt+C Ctrl+Alt+D`  | Cancels the most recent running evaluation                         |
| {#action nrepl::ClearOutputs}    | `Ctrl+Alt+C L`           | Removes all result blocks and inlays from the editor               |

Results render below the evaluated form as a block. Short single-line values
collapse into an end-of-line inlay once evaluation finishes; empty results
disappear, leaving a green gutter highlight on the evaluated lines. Editing
the source range removes its result.

When an evaluation throws and cider-nrepl is available, the result block
shows the analyzed stacktrace. Frames from your project are listed first;
click a frame that points to a local file to open it at that line.

The default keybindings are active in `.clj`, `.cljs`, `.cljc`, `.edn` and
`.bb` files and follow Calva's `Ctrl+Alt+C` prefix. To change them, rebind
the actions in your `keymap.json`, for example:

```json [keymap]
[
  {
    "context": "Editor && extension == clj",
    "bindings": {
      "ctrl-c ctrl-c": "nrepl::Eval",
      "ctrl-c ctrl-e": "nrepl::EvalCurrentForm",
      "ctrl-c ctrl-k": "nrepl::LoadFile"
    }
  }
]
```

### REPL console {#nrepl-console}

Run {#action nrepl::OpenRepl} (`Ctrl+Alt+C O`) to open an interactive REPL
in a tab. Type an expression and press `Enter` to evaluate it. `Up` and
`Down` browse the input history. The prompt follows the namespace the server
reports, so `(in-ns 'my.app)` behaves as in a terminal REPL. Errors are
followed by the analyzed stacktrace.

{#action nrepl::SendToRepl} (`Ctrl+Alt+C Ctrl+Alt+E`) sends the selection, or
the top-level form under the cursor, to the console in the file's namespace.

The console and inline evaluation share one nREPL session, so `*1`, `*2`,
`*3` and `*e` refer to the latest results wherever they came from.

### Namespaces and reloading {#nrepl-namespace-handling}

Each editor sends the namespace declared by the buffer's first `(ns ...)`
form with every evaluation. Without an `(ns ...)` form, `user` is used.

| Action                            | Default keybinding      | What it does                                            |
| --------------------------------- | ----------------------- | ------------------------------------------------------- |
| {#action nrepl::SwitchNamespace}  | `Ctrl+Alt+C Ctrl+Alt+N` | Runs `(in-ns 'file.ns)` and switches the console prompt |
| {#action nrepl::RequireReload}    | `Ctrl+Alt+C R`          | Runs `(require 'file.ns :reload)`                       |
| {#action nrepl::RequireReloadAll} | `Ctrl+Alt+C Shift+R`    | Runs `(require 'file.ns :reload-all)`                   |

### Completion, documentation and definitions {#nrepl-ide-features}

While connected, completions in Clojure buffers and in the REPL console
include candidates from the running program (cider-nrepl's `complete` op)
next to the ones from clojure-lsp. They cover vars defined at the REPL, Java
classes and members, and keywords.

| Action                             | Default keybinding | What it does                                                 |
| ---------------------------------- | ------------------ | ------------------------------------------------------------ |
| {#action nrepl::ShowDocumentation} | `Ctrl+Alt+C D`     | Shows arglists and docstring for the symbol under the cursor |
| {#action nrepl::GoToDefinition}    | `Ctrl+Alt+C .`     | Opens the definition the REPL knows about                    |

Go-to-definition works for local files. Definitions inside JAR archives are
reported but not opened yet.

### Sessions {#nrepl-sessions}

Zed uses one nREPL session per project, shared by every editor and the REPL
console. A `def` evaluated in `core.clj` is visible from `core_test.clj`.

### Settings {#nrepl-settings}

Open the Settings Editor and search for `nrepl` to configure the integration.
Or add the settings to your `settings.json`:

```json [settings]
{
  "nrepl": {
    // Whether the nREPL feature is enabled. Disabling removes the
    // `nrepl::*` actions from the command palette and the `nrepl` keymap
    // context from editors.
    "enabled": true,
    // Default host used when connecting to a `.nrepl-port`-discovered
    // server. Only `localhost` and IP addresses are honored in v1.
    "default_host": "localhost",
    // Connect automatically when the project opens. Disabled by default.
    "auto_connect": false,
    // File name (relative to the worktree root) that auto-discovery
    // looks for. The default matches what every common Clojure build
    // tool writes.
    "port_file": ".nrepl-port"
  }
}
```

### Limitations {#nrepl-limitations}

The following are not supported yet:

- Choosing the build tool or aliases for jack-in. Zed picks the first match
  in the table above and runs it without extra aliases.
- ClojureScript REPLs: a shadow-cljs jack-in connects to the Clojure side of
  the server. Run `(shadow/repl :your-build)` in the console to switch.
- Running tests, the debugger, and the value inspector.
- Opening definitions that live inside JAR archives.
- Remote, SSH, and WSL transports. Localhost TCP only.
- TLS / authenticated nREPL.
