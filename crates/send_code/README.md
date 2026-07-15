# send_code

Send code from the editor to Zed's built-in terminal.

## Actions

| Action | Behavior |
|---|---|
| `send_code::SendToTerminal` | Send the current selection when non-empty; otherwise send the smallest evaluable block at the cursor. Advances the cursor when using cursor-based eval. |
| `send_code::SendSelectionToTerminal` | Send the current selection to the active terminal. No-op when the selection is empty. |
| `send_code::SendEvalAtCursorToTerminal` | Find the smallest evaluable block containing the cursor via the language's `eval.scm` tree-sitter query and send it. Advances the cursor to the next line. Blank lines send a return and advance; other non-`@eval` locations no-op. |

For a Positron/RStudio-style send key:

```json
{
  "context": "Editor && mode == full",
  "bindings": {
    "shift-enter": "send_code::SendToTerminal"
  }
}
```

To get explicit "send the current line" behavior, compose actions:

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

    // Wrap multi-line sends in bracketed paste (default: true).
    "bracketed_paste": true
  }
}
```

## Adding Eval Support To A Language

Each language grammar may ship an `eval.scm` query that captures the nodes that should be sent as a unit. The query uses a single `@eval` capture:

```scheme
(function_definition) @eval
(if_statement) @eval
(assignment) @eval
```

Languages bundled in core today with `eval.scm` queries: Bash, Python, and R. Other languages can opt in by adding an `eval.scm` to their grammar; until they do, `SendEvalAtCursorToTerminal` only handles blank lines in those buffers.
