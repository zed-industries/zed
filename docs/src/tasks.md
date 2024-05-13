# Tasks

Zed supports ways to spawn (and rerun) commands using its integrated terminal to output the results. These commands can read a limited subset of Zed state (such as a path to the file currently being edited or selected text).

```json
[
  {
    "label": "Example task",
    "command": "for i in {1..5}; do echo \"Hello $i/5\"; sleep 1; done",
    //"args": [],
    // Env overrides for the command, will be appended to the terminal's environment from the settings.
    "env": { "foo": "bar" },
    // Current working directory to spawn the command into, defaults to current project root.
    //"cwd": "/path/to/working/directory",
    // Whether to use a new terminal tab or reuse the existing one to spawn the process, defaults to `false`.
    "use_new_terminal": false,
    // Whether to allow multiple instances of the same task to be run, or rather wait for the existing ones to finish, defaults to `false`.
    "allow_concurrent_runs": false,
    // What to do with the terminal pane and tab, after the command was started:
    // * `always` — always show the terminal pane, add and focus the corresponding task's tab in it (default)
    // * `never` — avoid changing current terminal pane focus, but still add/reuse the task's tab there
    "reveal": "always"
  }
]
```

There are two actions that drive the workflow of using tasks: `task: spawn` and `task: rerun`
`task: spawn` opens a modal with all available tasks in the current file.
`task: rerun` reruns the most-recently spawned task. You can also rerun tasks from task modal.

## Task templates

Tasks, defined in a config file (`tasks.json` in the Zed config directory).
Zed supports both global task templates (available in all projects) or workspace-local task templates (available only in the current workspace).

To edit global task templates, use `zed: open tasks` actions from command palette; to edit workspace-local task templates, use `zed: open local tasks` action.

## Variables

Variables allow you to pull information from the current editor and use it in your tasks. The following variables are available:

- `ZED_COLUMN`: current line column
- `ZED_ROW`: current line row
- `ZED_FILE`: absolute path to the file
- `ZED_SYMBOL`: currently selected symbol; should match the last symbol shown in a symbol breadcrumb (e.g. `mod tests > fn test_task_contexts`)
- `ZED_SELECTED_TEXT`: currently selected text
- `ZED_WORKTREE_ROOT`: absolute path to the root of the current worktree.
- `ZED_CUSTOM_RUST_PACKAGE`: (Rust-specific) name of the parent package of $ZED_FILE source file.

To use a variable in a task, prefix it with a dollar sign (`$`):

```json
{
  "label": "echo current file's path",
  "command": "echo $ZED_FILE"
}
```

You can also use verbose syntax that allows specifying a default if a given variable is not available: `${ZED_FILE:default_value}`

These environmental variables can also be used in tasks `cwd`, `args` and `label` fields.

## Oneshot tasks

The same task modal opened via `task: spawn` supports arbitrary bash-like command execution: type a command inside the modal text field, and use `opt-enter` to spawn it.

Task modal will persist list of those command for current Zed session, `task: rerun` will also rerun such tasks if they were the last ones spawned.

### Ephemeral tasks

You can use cmd modifier when spawning a task via a modal; tasks spawned this way will not have their usage count increased (thus, they will not be respawned with `task: rerun` and they won't be have a high rank in task modal).
The intended use of ephemeral tasks is to stay in the flow with continuous `task: rerun` usage.

## Custom keybindings for tasks

You can define your own keybindings for your tasks via additional argument to `task::Spawn`. If you wanted to bind the aforementioned `echo current file's path` task to `alt-g`, you would add the following snippet in your [`keymap.json`](./key-bindings/) file:

```json
{
  "context": "Workspace",
  "bindings": {
    "alt-g": ["task::Spawn", { "task_name": "echo current file's path" }]
  }
}
```

## Binding runnable tags to task templates

Zed supports overriding default action for inline runnable tags via workspace-local and global `tasks.json` file with the following precedence hierarchy:

1. Workspace `tasks.json`
2. Global `tasks.json`
3. Language-provided tag bindings (default).

To tag a task, add the runnable tag name to `tags` field on task template:

```json
{
  "label": "echo current file's path",
  "command": "echo $ZED_FILE",
  "tags": ["rust-test"]
}
```
