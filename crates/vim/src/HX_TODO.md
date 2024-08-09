## TODOS
- [-] fix enter insert mode and exiting insert mode moving the cursor.
  - [ ] fix updating the selection in append mode
- [x] fix xc on multi line selection
  - [ ] figure out how to auto indent on xc when full line is selected
- [ ] fix xp
- [x] correctly handle xd and xc (unable to nuke the end of line character)
- [ ] d doesn't leave behind selection (so multiple ds doesnt work together)
- [ ] fix ge
- [ ] fix gd on word selection (extra space at end is selected)
- [ ] fix search (how to select the search word properly, new selections are always zero sized)
- [ ] mouse click sets cursor incorrectly
- [ ] document highlight is broken (lsp protocol only deals with bar cursor not block cursor)
- [ ] fix the selection goals
- [ ] w and soft wrap has infinite loop bugs
- [ ] w is wrong when cursor is at end of the line
- [ ] keep select even when entering insert mode and leave insert mode
- [ ] `u` undo is incorrect
- [ ] w is wrong at end of file
- [ ] b is wrong at start of file
- [ ] preserve selection after undo
- [x] handle newlines in x is horrible!
- [ ] x is not as same helix
- [ ] fix status status showing line and column is incorrect :/
- [ ] everything is broken on EOF

## Major changes

keep track of selections in undo history
selection mode (helix visual mode)

### Movement

> NOTE: Unlike Vim, `f`, `F`, `t` and `T` are not confined to the current line.

index
```
x = done
. = done but slightly incorrect
- = left for later
```


| Key                       | Description                                        | Command                     |
| -----                     | -----------                                        | -------                     |
| [x] `h`, `Left`           | Move left                                          | `move_char_left`            |
| [x] `j`, `Down`           | Move down                                          | `move_visual_line_down`     |
| [x] `k`, `Up`             | Move up                                            | `move_visual_line_up`       |
| [x] `l`, `Right`          | Move right                                         | `move_char_right`           |
| [x] `w`                   | Move next word start                               | `move_next_word_start`      |
| [x] `b`                   | Move previous word start                           | `move_prev_word_start`      |
| [x] `e`                   | Move next word end                                 | `move_next_word_end`        |
| [x] `W`                   | Move next WORD start                               | `move_next_long_word_start` |
| [x] `B`                   | Move previous WORD start                           | `move_prev_long_word_start` |
| [x] `E`                   | Move next WORD end                                 | `move_next_long_word_end`   |
| [x] `t`                   | Find 'till next char                               | `find_till_char`            |
| [x] `f`                   | Find next char                                     | `find_next_char`            |
| [x] `T`                   | Find 'till previous char                           | `till_prev_char`            |
| [x] `F`                   | Find previous char                                 | `find_prev_char`            |
| [.] `G`                   | Go to line number `<n>`                            | `goto_line`                 |
| [-] `Alt-.`               | Repeat last motion (`f`, `t`, `m`, `[` or `]`)     | `repeat_last_motion`        |
| [x] `Home`                | Move to the start of the line                      | `goto_line_start`           |
| [.] `End`                 | Move to the end of the line                        | `goto_line_end`             |
| [x] `Ctrl-b`, `PageUp`    | Move page up                                       | `page_up`                   |
| [x] `Ctrl-f`, `PageDown`  | Move page down                                     | `page_down`                 |
| [x] `Ctrl-u`              | Move cursor and page half page up                  | `page_cursor_half_up`       |
| [x] `Ctrl-d`              | Move cursor and page half page down                | `page_cursor_half_down`     |
| [x] `Ctrl-i`              | Jump forward on the jumplist                       | `jump_forward`              |
| [x] `Ctrl-o`              | Jump backward on the jumplist                      | `jump_backward`             |
| [-] `Ctrl-s`              | Save the current selection to the jumplist         | `save_selection`            |

### Changes

| Key             | Description                                                          | Command                   |
| -----           | -----------                                                          | -------                   |
| [x] `r`         | Replace with a character                                             | `replace`                 |
| [ ] `R`         | Replace with yanked text                                             | `replace_with_yanked`     |
| [x] `~`         | Switch case of the selected text                                     | `switch_case`             |
| [x] `` ` ``     | Set the selected text to lower case                                  | `switch_to_lowercase`     |
| [x] `` Alt-` `` | Set the selected text to upper case                                  | `switch_to_uppercase`     |
| [x] `i`         | Insert before selection                                              | `insert_mode`             |
| [x] `a`         | Insert after selection (append)                                      | `append_mode`             |
| [x] `I`         | Insert at the start of the line                                      | `insert_at_line_start`    |
| [x] `A`         | Insert at the end of the line                                        | `insert_at_line_end`      |
| [x] `o`         | Open new line below selection                                        | `open_below`              |
| [x] `O`         | Open new line above selection                                        | `open_above`              |
| [ ] `.`         | Repeat last insert                                                   | N/A                       |
| [x] `u`         | Undo change                                                          | `undo`                    |
| [x] `U`         | Redo change                                                          | `redo`                    |
| [ ] `Alt-u`     | Move backward in history                                             | `earlier`                 |
| [ ] `Alt-U`     | Move forward in history                                              | `later`                   |
| [ ] `y`         | Yank selection                                                       | `yank`                    |
| [ ] `p`         | Paste after selection                                                | `paste_after`             |
| [ ] `P`         | Paste before selection                                               | `paste_before`            |
| [ ] `"` `<reg>` | Select a register to yank to or paste from                           | `select_register`         |
| [ ] `>`         | Indent selection                                                     | `indent`                  |
| [ ] `<`         | Unindent selection                                                   | `unindent`                |
| [ ] `=`         | Format selection (**LSP**)                                           | `format_selections`       |
| [x] `d`         | Delete selection                                                     | `delete_selection`        |
| [ ] `Alt-d`     | Delete selection, without yanking                                    | `delete_selection_noyank` |
| [x] `c`         | Change selection (delete and enter insert mode)                      | `change_selection`        |
| [ ] `Alt-c`     | Change selection (delete and enter insert mode, without yanking)     | `change_selection_noyank` |
| [ ] `Ctrl-a`    | Increment object (number) under cursor                               | `increment`               |
| [ ] `Ctrl-x`    | Decrement object (number) under cursor                               | `decrement`               |
| [ ] `Q`         | Start/stop macro recording to the selected register (experimental)   | `record_macro`            |
| [ ] `q`         | Play back a recorded macro from the selected register (experimental) | `replay_macro`            |

#### Shell

| Key     | Description                                                                      | Command               |
| ------  | -----------                                                                      | -------               |
| <code>&#124;</code>     | Pipe each selection through shell command, replacing with output                 | `shell_pipe`          |
| <code>Alt-&#124;</code> | Pipe each selection into shell command, ignoring output                          | `shell_pipe_to`       |
| `!`     | Run shell command, inserting output before each selection                        | `shell_insert_output` |
| `Alt-!` | Run shell command, appending output after each selection                         | `shell_append_output` |
| `$`     | Pipe each selection into shell command, keep selections where command returned 0 | `shell_keep_pipe`     |


### Selection manipulation

| Key                   | Description                                                       | Command                              |
| -----                 | -----------                                                       | -------                              |
| `s`                   | Select all regex matches inside selections                        | `select_regex`                       |
| `S`                   | Split selection into sub selections on regex matches              | `split_selection`                    |
| `Alt-s`               | Split selection on newlines                                       | `split_selection_on_newline`         |
| `Alt-minus`           | Merge selections                                                  | `merge_selections`                   |
| `Alt-_`               | Merge consecutive selections                                      | `merge_consecutive_selections`       |
| `&`                   | Align selection in columns                                        | `align_selections`                   |
| `_`                   | Trim whitespace from the selection                                | `trim_selections`                    |
| `;`                   | Collapse selection onto a single cursor                           | `collapse_selection`                 |
| `Alt-;`               | Flip selection cursor and anchor                                  | `flip_selections`                    |
| `Alt-:`               | Ensures the selection is in forward direction                     | `ensure_selections_forward`          |
| `,`                   | Keep only the primary selection                                   | `keep_primary_selection`             |
| `Alt-,`               | Remove the primary selection                                      | `remove_primary_selection`           |
| `C`                   | Copy selection onto the next line (Add cursor below)              | `copy_selection_on_next_line`        |
| `Alt-C`               | Copy selection onto the previous line (Add cursor above)          | `copy_selection_on_prev_line`        |
| `(`                   | Rotate main selection backward                                    | `rotate_selections_backward`         |
| `)`                   | Rotate main selection forward                                     | `rotate_selections_forward`          |
| `Alt-(`               | Rotate selection contents backward                                | `rotate_selection_contents_backward` |
| `Alt-)`               | Rotate selection contents forward                                 | `rotate_selection_contents_forward`  |
| `%`                   | Select entire file                                                | `select_all`                         |
| `x`                   | Select current line, if already selected, extend to next line     | `extend_line_below`                  |
| `X`                   | Extend selection to line bounds (line-wise selection)             | `extend_to_line_bounds`              |
| `Alt-x`               | Shrink selection to line bounds (line-wise selection)             | `shrink_to_line_bounds`              |
| `J`                   | Join lines inside selection                                       | `join_selections`                    |
| `Alt-J`               | Join lines inside selection and select the inserted space         | `join_selections_space`              |
| `K`                   | Keep selections matching the regex                                | `keep_selections`                    |
| `Alt-K`               | Remove selections matching the regex                              | `remove_selections`                  |
| `Ctrl-c`              | Comment/uncomment the selections                                  | `toggle_comments`                    |
| `Alt-o`, `Alt-up`     | Expand selection to parent syntax node (**TS**)                   | `expand_selection`                   |
| `Alt-i`, `Alt-down`   | Shrink syntax tree object selection (**TS**)                      | `shrink_selection`                   |
| `Alt-p`, `Alt-left`   | Select previous sibling node in syntax tree (**TS**)              | `select_prev_sibling`                |
| `Alt-n`, `Alt-right`  | Select next sibling node in syntax tree (**TS**)                  | `select_next_sibling`                |

### Search

Search commands all operate on the `/` register by default. To use a different register, use `"<char>`.

| Key   | Description                                 | Command              |
| ----- | -----------                                 | -------              |
| `/`   | Search for regex pattern                    | `search`             |
| `?`   | Search for previous pattern                 | `rsearch`            |
| `n`   | Select next search match                    | `search_next`        |
| `N`   | Select previous search match                | `search_prev`        |
| `*`   | Use current selection as the search pattern | `search_selection`   |

### Minor modes

These sub-modes are accessible from normal mode and typically switch back to normal mode after a command.

| Key      | Description                                        | Command        |
| -----    | -----------                                        | -------        |
| `v`      | Enter [select (extend) mode](#select--extend-mode) | `select_mode`  |
| `g`      | Enter [goto mode](#goto-mode)                      | N/A            |
| `m`      | Enter [match mode](#match-mode)                    | N/A            |
| `:`      | Enter command mode                                 | `command_mode` |
| `z`      | Enter [view mode](#view-mode)                      | N/A            |
| `Z`      | Enter sticky [view mode](#view-mode)               | N/A            |
| `Ctrl-w` | Enter [window mode](#window-mode)                  | N/A            |
| `Space`  | Enter [space mode](#space-mode)                    | N/A            |

These modes (except command mode) can be configured by
[remapping keys](https://docs.helix-editor.com/remapping.html#minor-modes).

#### View mode

Accessed by typing `z` in [normal mode](#normal-mode).

View mode is intended for scrolling and manipulating the view without changing
the selection. The "sticky" variant of this mode (accessed by typing `Z` in
normal mode) is persistent and can be exited using the escape key. This is
useful when you're simply looking over text and not actively editing it.


| Key                  | Description                                               | Command                 |
| -----                | -----------                                               | -------                 |
| `z`, `c`             | Vertically center the line                                | `align_view_center`     |
| `t`                  | Align the line to the top of the screen                   | `align_view_top`        |
| `b`                  | Align the line to the bottom of the screen                | `align_view_bottom`     |
| `m`                  | Align the line to the middle of the screen (horizontally) | `align_view_middle`     |
| `j`, `down`          | Scroll the view downwards                                 | `scroll_down`           |
| `k`, `up`            | Scroll the view upwards                                   | `scroll_up`             |
| `Ctrl-f`, `PageDown` | Move page down                                            | `page_down`             |
| `Ctrl-b`, `PageUp`   | Move page up                                              | `page_up`               |
| `Ctrl-u`             | Move cursor and page half page up                         | `page_cursor_half_up`   |
| `Ctrl-d`             | Move cursor and page half page down                       | `page_cursor_half_down` |

#### Goto mode

Accessed by typing `g` in [normal mode](#normal-mode).

Jumps to various locations.

| Key   | Description                                      | Command                    |
| ----- | -----------                                      | -------                    |
| [x] `g`   | Go to line number `<n>` else start of file       | `goto_file_start`          |
| [x] `e`   | Go to the end of the file                        | `goto_last_line`           |
| [-] `f`   | Go to files in the selections                    | `goto_file`                |
| [x] `h`   | Go to the start of the line                      | `goto_line_start`          |
| [.] `l`   | Go to the end of the line                        | `goto_line_end`            |
| [x] `s`   | Go to first non-whitespace character of the line | `goto_first_nonwhitespace` |
| [x] `t`   | Go to the top of the screen                      | `goto_window_top`          |
| [x] `c`   | Go to the middle of the screen                   | `goto_window_center`       |
| [x] `b`   | Go to the bottom of the screen                   | `goto_window_bottom`       |
| [x] `d`   | Go to definition (**LSP**)                       | `goto_definition`          |
| [x] `y`   | Go to type definition (**LSP**)                  | `goto_type_definition`     |
| [x] `r`   | Go to references (**LSP**)                       | `goto_reference`           |
| [x] `i`   | Go to implementation (**LSP**)                   | `goto_implementation`      |
| [x] `a`   | Go to the last accessed/alternate file           | `goto_last_accessed_file`  |
| [ ] `m`   | Go to the last modified/alternate file           | `goto_last_modified_file`  |
| [x] `n`   | Go to next buffer                                | `goto_next_buffer`         |
| [x] `p`   | Go to previous buffer                            | `goto_previous_buffer`     |
| [x] `.`   | Go to last modification in current file          | `goto_last_modification`   |
| [x] `j`   | Move down textual (instead of visual) line       | `move_line_down`           |
| [x] `k`   | Move up textual (instead of visual) line         | `move_line_up`             |
| [ ] `w`   | Show labels at each word and select the word that belongs to the entered labels | `goto_word` |

#### Match mode

Accessed by typing `m` in [normal mode](#normal-mode).

See the relevant section in [Usage](./usage.md) for an explanation about
[surround](./usage.md#surround) and [textobject](./usage.md#navigating-using-tree-sitter-textobjects) usage.

| Key              | Description                                     | Command                    |
| -----            | -----------                                     | -------                    |
| `m`              | Goto matching bracket (**TS**)                  | `match_brackets`           |
| `s` `<char>`     | Surround current selection with `<char>`        | `surround_add`             |
| `r` `<from><to>` | Replace surround character `<from>` with `<to>` | `surround_replace`         |
| `d` `<char>`     | Delete surround character `<char>`              | `surround_delete`          |
| `a` `<object>`   | Select around textobject                        | `select_textobject_around` |
| `i` `<object>`   | Select inside textobject                        | `select_textobject_inner`  |

TODO: Mappings for selecting syntax nodes (a superset of `[`).

#### Window mode

Accessed by typing `Ctrl-w` in [normal mode](#normal-mode).

This layer is similar to Vim keybindings as Kakoune does not support windows.

| Key                    | Description                                          | Command           |
| -----                  | -------------                                        | -------           |
| `w`, `Ctrl-w`          | Switch to next window                                | `rotate_view`     |
| `v`, `Ctrl-v`          | Vertical right split                                 | `vsplit`          |
| `s`, `Ctrl-s`          | Horizontal bottom split                              | `hsplit`          |
| `f`                    | Go to files in the selections in horizontal splits   | `goto_file`       |
| `F`                    | Go to files in the selections in vertical splits     | `goto_file`       |
| `h`, `Ctrl-h`, `Left`  | Move to left split                                   | `jump_view_left`  |
| `j`, `Ctrl-j`, `Down`  | Move to split below                                  | `jump_view_down`  |
| `k`, `Ctrl-k`, `Up`    | Move to split above                                  | `jump_view_up`    |
| `l`, `Ctrl-l`, `Right` | Move to right split                                  | `jump_view_right` |
| `q`, `Ctrl-q`          | Close current window                                 | `wclose`          |
| `o`, `Ctrl-o`          | Only keep the current window, closing all the others | `wonly`           |
| `H`                    | Swap window to the left                              | `swap_view_left`  |
| `J`                    | Swap window downwards                                | `swap_view_down`  |
| `K`                    | Swap window upwards                                  | `swap_view_up`    |
| `L`                    | Swap window to the right                             | `swap_view_right` |

#### Space mode

Accessed by typing `Space` in [normal mode](#normal-mode).

This layer is a kludge of mappings, mostly pickers.

| Key     | Description                                                             | Command                                    |
| -----   | -----------                                                             | -------                                    |
| `f`     | Open file picker                                                        | `file_picker`                              |
| `F`     | Open file picker at current working directory                           | `file_picker_in_current_directory`         |
| `b`     | Open buffer picker                                                      | `buffer_picker`                            |
| `j`     | Open jumplist picker                                                    | `jumplist_picker`                          |
| `g`     | Open changed file picker                                                | `changed_file_picker`                      |
| `G`     | Debug (experimental)                                                    | N/A                                        |
| `k`     | Show documentation for item under cursor in a [popup](#popup) (**LSP**) | `hover`                                    |
| `s`     | Open document symbol picker (**LSP**)                                   | `symbol_picker`                            |
| `S`     | Open workspace symbol picker (**LSP**)                                  | `workspace_symbol_picker`                  |
| `d`     | Open document diagnostics picker (**LSP**)                              | `diagnostics_picker`                       |
| `D`     | Open workspace diagnostics picker (**LSP**)                             | `workspace_diagnostics_picker`             |
| `r`     | Rename symbol (**LSP**)                                                 | `rename_symbol`                            |
| `a`     | Apply code action (**LSP**)                                             | `code_action`                              |
| `h`     | Select symbol references (**LSP**)                                      | `select_references_to_symbol_under_cursor` |
| `'`     | Open last fuzzy picker                                                  | `last_picker`                              |
| `w`     | Enter [window mode](#window-mode)                                       | N/A                                        |
| `c`     | Comment/uncomment selections                                            | `toggle_comments`                          |
| `C`     | Block comment/uncomment selections                                      | `toggle_block_comments`                    |
| `Alt-c` | Line comment/uncomment selections                                       | `toggle_line_comments`                     |
| `p`     | Paste system clipboard after selections                                 | `paste_clipboard_after`                    |
| `P`     | Paste system clipboard before selections                                | `paste_clipboard_before`                   |
| `y`     | Yank selections to clipboard                                            | `yank_to_clipboard`                        |
| `Y`     | Yank main selection to clipboard                                        | `yank_main_selection_to_clipboard`         |
| `R`     | Replace selections by clipboard contents                                | `replace_selections_with_clipboard`        |
| `/`     | Global search in workspace folder                                       | `global_search`                            |
| `?`     | Open command palette                                                    | `command_palette`                          |

> 💡 Global search displays results in a fuzzy picker, use `Space + '` to bring it back up after opening a file.

##### Popup

Displays documentation for item under cursor. Remapping currently not supported.

| Key      | Description |
| ----     | ----------- |
| `Ctrl-u` | Scroll up   |
| `Ctrl-d` | Scroll down |

##### Completion Menu

Displays documentation for the selected completion item. Remapping currently not supported.

| Key                         | Description    |
| ----                        | -----------    |
| `Shift-Tab`, `Ctrl-p`, `Up` | Previous entry |
| `Tab`, `Ctrl-n`, `Down`     | Next entry     |

##### Signature-help Popup

Displays the signature of the selected completion item. Remapping currently not supported.

| Key     | Description        |
| ----    | -----------        |
| `Alt-p` | Previous signature |
| `Alt-n` | Next signature     |

#### Unimpaired

These mappings are in the style of [vim-unimpaired](https://github.com/tpope/vim-unimpaired).

| Key      | Description                                  | Command               |
| -----    | -----------                                  | -------               |
| `]d`     | Go to next diagnostic (**LSP**)              | `goto_next_diag`      |
| `[d`     | Go to previous diagnostic (**LSP**)          | `goto_prev_diag`      |
| `]D`     | Go to last diagnostic in document (**LSP**)  | `goto_last_diag`      |
| `[D`     | Go to first diagnostic in document (**LSP**) | `goto_first_diag`     |
| `]f`     | Go to next function (**TS**)                 | `goto_next_function`  |
| `[f`     | Go to previous function (**TS**)             | `goto_prev_function`  |
| `]t`     | Go to next type definition (**TS**)          | `goto_next_class`     |
| `[t`     | Go to previous type definition (**TS**)      | `goto_prev_class`     |
| `]a`     | Go to next argument/parameter (**TS**)       | `goto_next_parameter` |
| `[a`     | Go to previous argument/parameter (**TS**)   | `goto_prev_parameter` |
| `]c`     | Go to next comment (**TS**)                  | `goto_next_comment`   |
| `[c`     | Go to previous comment (**TS**)              | `goto_prev_comment`   |
| `]T`     | Go to next test (**TS**)                     | `goto_next_test`      |
| `[T`     | Go to previous test (**TS**)                 | `goto_prev_test`      |
| `]p`     | Go to next paragraph                         | `goto_next_paragraph` |
| `[p`     | Go to previous paragraph                     | `goto_prev_paragraph` |
| `]g`     | Go to next change                            | `goto_next_change`    |
| `[g`     | Go to previous change                        | `goto_prev_change`    |
| `]G`     | Go to last change                            | `goto_last_change`    |
| `[G`     | Go to first change                           | `goto_first_change`   |
| `]Space` | Add newline below                            | `add_newline_below`   |
| `[Space` | Add newline above                            | `add_newline_above`   |

## Insert mode

Accessed by typing `i` in [normal mode](#normal-mode).

Insert mode bindings are minimal by default. Helix is designed to
be a modal editor, and this is reflected in the user experience and internal
mechanics. Changes to the text are only saved for undos when
escaping from insert mode to normal mode.

> 💡 New users are strongly encouraged to learn the modal editing paradigm
> to get the smoothest experience.

| Key                                         | Description                 | Command                  |
| -----                                       | -----------                 | -------                  |
| `Escape`                                    | Switch to normal mode       | `normal_mode`            |
| `Ctrl-s`                                    | Commit undo checkpoint      | `commit_undo_checkpoint` |
| `Ctrl-x`                                    | Autocomplete                | `completion`             |
| `Ctrl-r`                                    | Insert a register content   | `insert_register`        |
| `Ctrl-w`, `Alt-Backspace`                   | Delete previous word        | `delete_word_backward`   |
| `Alt-d`, `Alt-Delete`                       | Delete next word            | `delete_word_forward`    |
| `Ctrl-u`                                    | Delete to start of line     | `kill_to_line_start`     |
| `Ctrl-k`                                    | Delete to end of line       | `kill_to_line_end`       |
| `Ctrl-h`, `Backspace`, `Shift-Backspace`    | Delete previous char        | `delete_char_backward`   |
| `Ctrl-d`, `Delete`                          | Delete next char            | `delete_char_forward`    |
| `Ctrl-j`, `Enter`                           | Insert new line             | `insert_newline`         |

These keys are not recommended, but are included for new users less familiar
with modal editors.

| Key                                         | Description                 | Command                  |
| -----                                       | -----------                 | -------                  |
| `Up`                                        | Move to previous line       | `move_line_up`           |
| `Down`                                      | Move to next line           | `move_line_down`         |
| `Left`                                      | Backward a char             | `move_char_left`         |
| `Right`                                     | Forward a char              | `move_char_right`        |
| `PageUp`                                    | Move one page up            | `page_up`                |
| `PageDown`                                  | Move one page down          | `page_down`              |
| `Home`                                      | Move to line start          | `goto_line_start`        |
| `End`                                       | Move to line end            | `goto_line_end_newline`  |

As you become more comfortable with modal editing, you may want to disable some
insert mode bindings. You can do this by editing your `config.toml` file.

```toml
[keys.insert]
up = "no_op"
down = "no_op"
left = "no_op"
right = "no_op"
pageup = "no_op"
pagedown = "no_op"
home = "no_op"
end = "no_op"
```

## Select / extend mode

Accessed by typing `v` in [normal mode](#normal-mode).

Select mode echoes Normal mode, but changes any movements to extend
selections rather than replace them. Goto motions are also changed to
extend, so that `vgl`, for example, extends the selection to the end of
the line.

Search is also affected. By default, `n` and `N` will remove the current
selection and select the next instance of the search term. Toggling this
mode before pressing `n` or `N` makes it possible to keep the current
selection. Toggling it on and off during your iterative searching allows
you to selectively add search terms to your selections.

## Picker

Keys to use within picker. Remapping currently not supported.

| Key                          | Description                                                |
| -----                        | -------------                                              |
| `Shift-Tab`, `Up`, `Ctrl-p`  | Previous entry                                             |
| `Tab`, `Down`, `Ctrl-n`      | Next entry                                                 |
| `PageUp`, `Ctrl-u`           | Page up                                                    |
| `PageDown`, `Ctrl-d`         | Page down                                                  |
| `Home`                       | Go to first entry                                          |
| `End`                        | Go to last entry                                           |
| `Enter`                      | Open selected                                              |
| `Alt-Enter`                  | Open selected in the background without closing the picker |
| `Ctrl-s`                     | Open horizontally                                          |
| `Ctrl-v`                     | Open vertically                                            |
| `Ctrl-t`                     | Toggle preview                                             |
| `Escape`, `Ctrl-c`           | Close picker                                               |

## Prompt

Keys to use within prompt, Remapping currently not supported.

| Key                                         | Description                                                             |
| -----                                       | -------------                                                           |
| `Escape`, `Ctrl-c`                          | Close prompt                                                            |
| `Alt-b`, `Ctrl-Left`                        | Backward a word                                                         |
| `Ctrl-b`, `Left`                            | Backward a char                                                         |
| `Alt-f`, `Ctrl-Right`                       | Forward a word                                                          |
| `Ctrl-f`, `Right`                           | Forward a char                                                          |
| `Ctrl-e`, `End`                             | Move prompt end                                                         |
| `Ctrl-a`, `Home`                            | Move prompt start                                                       |
| `Ctrl-w`, `Alt-Backspace`, `Ctrl-Backspace` | Delete previous word                                                    |
| `Alt-d`, `Alt-Delete`, `Ctrl-Delete`        | Delete next word                                                        |
| `Ctrl-u`                                    | Delete to start of line                                                 |
| `Ctrl-k`                                    | Delete to end of line                                                   |
| `Backspace`, `Ctrl-h`, `Shift-Backspace`    | Delete previous char                                                    |
| `Delete`, `Ctrl-d`                          | Delete next char                                                        |
| `Ctrl-s`                                    | Insert a word under doc cursor, may be changed to Ctrl-r Ctrl-w later   |
| `Ctrl-p`, `Up`                              | Select previous history                                                 |
| `Ctrl-n`, `Down`                            | Select next history                                                     |
| `Ctrl-r`                                    | Insert the content of the register selected by following input char     |
| `Tab`                                       | Select next completion item                                             |
| `BackTab`                                   | Select previous completion item                                         |
| `Enter`                                     | Open selected                                                           |
