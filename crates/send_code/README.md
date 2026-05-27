# send_code

Send code from the editor to Zed's built-in terminal.

## Actions

| Action | Behavior |
|---|---|
| `send_code::SendSelectionToTerminal` | Send the current selection to the active terminal. No-op when the selection is empty. |
| `send_code::SendEvalAtCursorToTerminal` | Find the smallest evaluable block containing the cursor (via the language's `eval.scm` tree-sitter query) and send it. Advances the cursor to the next line. No-op when the cursor is not inside an `@eval` node. |

To get "send the current line" behavior, compose actions:

```json
{
  "context": "Editor && mode == full",
  "bindings": {
    "ctrl-enter": ["workspace::ActivateNextItem"],
    "shift-enter": [
      "action::Sequence",
      ["editor::SelectLine", "send_code::SendSelectionToTerminal"]
    ]
  }
}
```

## Settings

```jsonc
{
  "send_code": {
    // Whether the SendCode actions are enabled (default: true).
    "enabled": true,

    // Wrap multi-line sends in bracketed paste (default: true). Disable for
    // REPLs whose readline does not interpret bracketed paste, e.g. R's
    // default readline.
    "bracketed_paste": true
  }
}
```

## Adding eval support to a language

Each language grammar may ship an `eval.scm` query that captures the nodes that should be sent as a unit. The query uses a single `@eval` capture:

```scheme
(function_definition) @eval
(if_statement) @eval
(assignment) @eval
```

Languages bundled in core today with `eval.scm` queries: bash, javascript, python, typescript. Other languages (including extension languages like R and Julia) can opt in by adding an `eval.scm` to their grammar; until they do, `SendEvalAtCursorToTerminal` is a no-op for cursors in those buffers.

## Architecture

```
crates/send_code/src/
  send_code.rs        Action registration and dispatch
  settings.rs         enabled / bracketed_paste
  code_getter.rs      Selection / eval-at-cursor extraction
  eval.rs             Tree-sitter eval.scm matching
  senders.rs          Write to the active terminal pane
```
