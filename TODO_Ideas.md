after next main merge, test this:
- Arriving this Zednesday: a dedicated action to toggle block comments. 
`cmd-k, cmd-/` (or `ctrl-k, ctrl-/`)
- try out bookmarks: <https://bsky.app/profile/zed.dev/post/3mjxczuagbc22>
- The file finder now matches search terms in any order.
- With "Focus Follows Mouse" enabled, focus is automatically transferred to whichever panel your mouse is hovering over.
- https://github.com/zed-industries/zed/pull/49802 (project_panel: Add diagnostic count badges and color priority)
- outline UI ("document_symbols": "on"). (enable this and test) (https://github.com/zed-industries/zed/pull/48780)
- https://github.com/zed-industries/zed/pull/49624 (git_ui: Show uncommitted change count badge on git panel icon)
- https://github.com/zed-industries/zed/pull/51000 (git_ui: Add file and folder icons to the Git panel)
- https://github.com/zed-industries/zed/pull/50777 (editor: Go to previous and next symbol actions)
- Added scroll-to-top and scroll-to-bottom keybindings for markdown preview (gg /G in Vim mode,cmd-up /cmd-down on macOS,ctrl-home /ctrl-end on Linux/Windows). ([#50460](https://github.com/zed-industries/zed/pull/50460) ; thanks [dremnik](https://github.com/dremnik) )
- remove all qwen AI code since the free tier got wiped
- WHAT HAPPENS IF IT IS OFF? IS IT WRAPPED? Added project_panel.scrollbar.horizontal_scroll setting to toggle horizontal scrolling in the project panel. ([#51143](https://github.com/zed-industries/zed/pull/51143) ; thanks [k4yt3x](https://github.com/k4yt3x) )


try out Sweep AI edit predictions since they have 1k free each month
once Mercury free 1 month runs out!

on next main merge, TEST and then remove this mention from my README
https://github.com/zed-industries/zed/pull/49102 (ep: Fix edit predictions not showing in new buffer)
does my diff modify the code exactly the same?

on next main merge
remove this mention from my README about the feature flag being always enabled
- Added support for viewing diffs in split ("side by side") mode. ([#48912](https://github.com/zed-industries/zed/pull/48912) )

Remove this note from README since https://github.com/zed-industries/zed/pull/49102 fixed it:
Is my code diff the same or not?
- buffers without files like ones from `workspace: new file`

WAIT until officially released, then merge and try out
https://github.com/zed-industries/zed/pull/42889
https://github.com/zed-industries/zed/pull/49150
(Add search_on_input setting to Project Search)
since I already have custom code to it with an eye icon

Try out
git_graph: Add basic keyboard navigation
https://github.com/zed-industries/zed/pull/49051


# >>> Investigations

## Fix that edit predictions do not work for buffers without files, like ones started from workspace: new file

I fixed this in my own fork already, but let's see what Zed team says:

I created this bug report issue:
https://github.com/zed-industries/zed/issues/45631 (issue open)

## Smooth caret/cursor

### editor: Add smooth cursor animation (PR open)

I tested this and it has visual glitches, apparently which I documented in GitHub, so I do not use this.
It also does not support jumping the cursor across multiple panes.

https://github.com/zed-industries/zed/pull/44770

### Add smooth cursor animation (PR closed)

This has a very small diff, I checked out the branch, but `cargo run` does not start properly and is unable to open a window:

```
Zed failed to open a window: select toolchains

Caused by:
    0: Prepare call failed for query:
       SELECT
         name,
         path,
         worktree_id,
         relative_worktree_path,
         language_name,
         raw_json
       FROM
         toolchains
       WHERE
         workspace_id = ?
    1: Sqlite call failed with code 1 and message: Some("no such column: worktree_id"). See https://zed.dev/docs/linux for troubleshooting steps.
```

I then let AI apply the diff directly on my `dima` branch, and it correctly starts up and shows the smooth cursor.
But it has the same annoying character misplaced bug as the other diff, but in this PR it instantly jumps to the character of where the cursor will be, which also looks bad.

https://github.com/zed-industries/zed/pull/43826

## Add file explorer modal v2 (PR open)

I already have his v1 (https://github.com/zed-industries/zed/pull/43961 (PR closed)) integrated. It is bound at `file_explorer::Toggle`.
I only see the v2 improvement that it has a full text field at the top, which can go outside the project root directory, but that is just a minor thing. I do not think I need it, since I can just do it via the `neovim` task.

v2 does not have the ignore files button/functionality anymore which sucks.

I tried out the branch and I really don't think I need it. I think it also mixes sorting of files and directories?

https://github.com/zed-industries/zed/pull/45307

## telescope/quick search

Not so important with `buffer_search_modal::ToggleBufferSearch` and `editor::SearchInCurrentFileViaMultiBuffer`.

### Add Search modal for project-wide text search

I have not checked this out yet, but goes in the same direction as my implementation. Just no line search.

https://github.com/zed-industries/zed/pull/46478 (PR open)

###  Add telescope style search (PR closed)

This was closed by Zed team in favor of the PR below.

I tested it, the file search only shows `...` which is not good. Text search seems very nice, otherwise, but the dialog is just too small designed for my resolution.

https://github.com/zed-industries/zed/pull/44942

### Add quick search modal (PR open)

I don't think it is ready yet, when a file has many search results, you do not see the file name anymore, it needs sticky scroll.
Otherwise, UI works great on my smaller resolution.

https://github.com/zed-industries/zed/pull/44530

## Filter for code actions

Absolutely not important since I rarely, if ever, need to search. It depends on LSP server and programming language.

### Add filter for code actions (PR open)

Has merge conflicts and I do not have a clue how to merge.

https://github.com/zed-industries/zed/pull/44534

### Add fuzzy code actions picker (PR open)

This is a bit weird with a new action and numbers. Will not use it.

https://github.com/zed-industries/zed/pull/44802

## Git side by side diffs

Not so important.

### Basic side-by-side diff implementation (PR merged)

This is kinda difficult to enable, I stopped researching it.

https://github.com/zed-industries/zed/pull/43586

### Implement initial side-by-side Git diffs (PR closed)

PR was apparently closed, only has 3k changes.
Does it have merge conflicts?

https://github.com/zed-industries/zed/pull/40014

## Support external agent history

I merged this in.
PR closed because ACP does not support history yet.

https://github.com/zed-industries/zed/pull/45734

# agent: History and recent conversations persistence per workspace

I did not check this out.

https://github.com/zed-industries/zed/pull/41874 (PR closed)

## Jump hint implementations

### The branch where my implementation is based on (no PR)

https://github.com/tebben/zed/tree/feature/jump

### Beam Jump - Lightning Fast Vim style navigation (PR open)

Has no screenshots, according to https://github.com/zed-industries/zed/pull/43733#issuecomment-3706155542, this PR is using the nice looking collab labels.

https://github.com/zed-industries/zed/pull/45387

### helix: Add Helix's "Amp Jump" Feature (PR open)

This shows 2 character hints at the start of each word.

https://github.com/zed-industries/zed/pull/43733

## Git Commit Graph View

I merged into my own fork without any issues, I just don't really need it. For now, I'll wait on Zed's team to merge it, and use `lazygit` in the meantime.

git_ui: Implement interactive Git commit graph view
https://github.com/zed-industries/zed/pull/45884 (PR closed)



# >>> Impossible to fix from my side

## Fix that the git: blame action inside a git blame commit tab is not working and only showing an error notification

I tried to fix with my yek file merger through Gemini and via auggie, but both failed.

There is this in the console:

2026-01-03T19:28:18+01:00 ERROR [editor::git::blame] failed to get git blame data: [failed to find a git repository for buffer, failed to find a git repository for buffer]


I created an issue for this:
https://github.com/zed-industries/zed/issues/45532

## improve `buffer_search_modal::ToggleBufferSearch` in `crates/search/src/buffer_search_modal.rs`

### Center top candidates list always

Can the top candidate list be centered, currently is always at either top or bottom when holding arrow up/down? I mean the selected row should be centered. Real centered movement is not implemented anywhere else in Zed, so too difficult to implement. I tried with Windsurf Penguin Alpha and it was not able to.

This is not implemented anywhere else in Zed, so probably too difficult to implement.

### Incorrect bottom padding in no line mode

Fix that in no line mode the candidate item list lines have incorrect bottom padding, They look weird, the ones for the line mode are fine weirdly, when no character is typed in, then in no line mode, the candidate rows have correct paddinge only as soon as anything is typed in.

I have no idea why this is happening, and it would be amazing to fix, but I can not figure it out.

# Too tough to implement

## Editor File Explorer

Can one enable preview tabs like in project panel for the editor file explorer in `editor.rs`? like every time I move cursor, it should update preview. Check how Markdown does it? Maybe see `markdown::OpenPreviewToTheSide`.

This is insanely useful for previewing images.
