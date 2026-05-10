# Git Graph Task Migration Todos

## Core behavior

1. **Decide public vs custom Git task variables**
   - Current implementation adds public variables:
     - `$ZED_GIT_SHA`
     - `$ZED_GIT_SHA_SHORT`
     - `$ZED_GIT_REPOSITORY_NAME`
     - `$ZED_GIT_REPOSITORY_PATH`
   - Alternative: use custom variables for now, such as:
     - `$ZED_CUSTOM_GIT_SHA`
     - `$ZED_CUSTOM_GIT_SHA_SHORT`
     - `$ZED_CUSTOM_GIT_REPOSITORY_NAME`
     - `$ZED_CUSTOM_GIT_REPOSITORY_PATH`
   - Custom variables avoid committing to a public task variable API while the feature is still settling.
   - Public variables are nicer for users and documentation, but harder to rename later.
   - The task resolver already filters out tasks when required variables are missing, so this decision should not require changing the core task resolution layer.

2. **Decide whether Git graph task context should include broader task variables**
   - Currently provided:
     - `$ZED_GIT_SHA`
     - `$ZED_GIT_SHA_SHORT`
     - `$ZED_GIT_REPOSITORY_NAME`
     - `$ZED_GIT_REPOSITORY_PATH`
     - `cwd`
   - Decide whether to also populate:
     - `$ZED_WORKTREE_ROOT`
     - `$ZED_MAIN_GIT_WORKTREE`
     - Project environment for the repository root

## Testing

3. **Add Git graph/task integration tests**
   - Task appears when tagged with `git-command`.
   - Task does not appear in Git graph when it is not tagged with `git-command`.
   - Confirm unresolved `git-command` templates stay out of normal task spawn.
   - If Git graph task runs should not appear in normal task spawn history, add coverage for that filtering.
   - Git variables resolve correctly in `label`, `command`, `args`, `cwd`, and `env`.
   - Clicking a menu entry schedules the resolved task and records it in normal task history.
   - Task `cwd` is the selected repository root.
   - Global and worktree-local tasks are both found.

4. **Validate repository-to-worktree lookup**
   - Multi-root workspaces.
   - Nested repositories.
   - Linked worktrees.
   - Repositories associated with multiple worktrees.
   - Invisible worktrees.

## UX polish

5. **Rebuild and polish command preview aside**
   - Base it on `ResolvedTask`.
   - Show command and args.
   - Show `cwd`.
   - Avoid ambiguous naive arg joining if possible.
   - Decide whether documentation aside is the right UI surface.

6. **Finalize menu presentation**
   - Keep `Git Tasks` header or choose another label.
   - Decide submenu vs inline section.
   - Decide disabled vs hidden behavior for unresolved tasks.
   - Consider showing task source information, such as global vs project-local.
   - Consider deduplicating duplicate resolved labels.

## Docs/schema/autocomplete

7. **Polish docs after behavior settles**
   - Document `tags: ["git-command"]` as the way to define Git graph context menu tasks.
   - Mention that unresolved Git graph task templates do not appear in normal task spawn.
   - Document the final decision for whether Git graph task runs appear in normal task history after being run once.
   - Double-check wording of the feature in labels and documentation

---

8. **Investigate variable autocomplete**
   - Validation currently recognizes `$ZED_GIT_*` variables.
   - Check whether task variable completions pick them up automatically.
   - If not, add completions for:
     - `$ZED_GIT_SHA`
     - `$ZED_GIT_SHA_SHORT`
     - `$ZED_GIT_REPOSITORY_NAME`
     - `$ZED_GIT_REPOSITORY_PATH`
