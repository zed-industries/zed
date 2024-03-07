# Tasks

Zed supports ways to spawn (and rerun) commands using its integrated terminal to output the results.

Currently, two kinds of tasks are supported, but more will be added in the future.

All tasks are sorted in LRU order and their names can be used (with `menu::UseSelectedQuery`, `shift-enter` by default) as an input text for quicker oneshot task edit-spawn cycle.

## Static tasks

Tasks, defined in a config file (`tasks.json` in the Zed config directory) that do not depend on the current editor or its content.

Config file can be opened with `zed::OpenTasks` action ("zed: open tasks" in the command palette), it will have a configuration example with all options commented.

Every task from that file can be spawned via the task modal, that is opened with `task::Spawn` action ("tasks: spawn" in the command pane).

Last task spawned via that modal can be rerun with `task::Rerun` ("tasks: rerun" in the command palette) command.

## Oneshot tasks

Same task modal opened via `task::Spawn` supports arbitrary bash-like command execution: type a command inside the modal, and use `cmd-enter` to spawn it.

Task modal will persist list of those command for current Zed session, `task::Rerun` will also rerun such tasks if they were the last ones spawned.

## Variables

Variables allow you to pull information from the current editor and use it in your tasks.

- `ZED_COLUMN`: current line column
- `ZED_ROW`: current line row and the following, which are available for buffers with associated files:
- `ZED_WORKTREE_ROOT`: absolute path to the root of the current worktree.
- `ZED_FILE`: absolute path to the file
- `ZED_SYMBOL`: currently selected symbol; should match the last symbol shown in a symbol breadcrumb (e.g. `mod tests > fn test_task_contexts`
