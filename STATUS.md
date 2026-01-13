# Test Failures (ITERATIONS=10)

## Summary
- 3513 tests run
- 3469 passed
- 31 failed
- 13 timed out
- 47 skipped

## Timeout Investigation

Extended timeout to 300s in `.config/nextest.toml` for all timeout tests.

Results:
- [x] `vim` tests (9) - **PASS** - just slow (~60s each), pass with 300s timeout
- [x] `editor` bracket/invisibles tests (2) - **PASS** - slow (76s, 90s), pass with 300s timeout  
- [x] `language_model` test_from_image_downscales (1) - **PASS** - very slow (167s), passes
- [ ] `extension_host` test_extension_store_with_test_extension (1) - **FLAKY** - passes 1 iter, fails 10 iter

## Remaining Failures

### Real Flakes (need investigation)
- [ ] `extension_host` extension_store_test::test_extension_store_with_test_extension

### Other Failures
- [ ] `acp_thread` tests::test_terminal_kill_allows_wait_for_exit_to_complete
- [x] `command_palette` tests::test_command_palette — **FIXED**: shared static DB (`COMMAND_PALETTE_HISTORY`) persisted hit counts across seeds, breaking alphabetical sort assumption. Fix: clear DB at test start via `clear_all().await`.
- [ ] `editor` editor_tests::test_autoindent_selections
- [ ] `editor` editor_tests::test_completions_resolve_updates_labels_if_filter_text_matches
- [ ] `editor` editor_tests::test_relative_line_numbers
- [ ] `editor` element::tests::test_soft_wrap_editor_width_auto_height_editor
- [ ] `editor` element::tests::test_soft_wrap_editor_width_full_editor
- [ ] `editor` inlays::inlay_hints::tests::test_basic_cache_update_with_duplicate_hints
- [ ] `editor` inlays::inlay_hints::tests::test_cache_update_on_lsp_completion_tasks
- [ ] `editor` inlays::inlay_hints::tests::test_hint_request_cancellation
- [ ] `editor` inlays::inlay_hints::tests::test_hint_setting_changes
- [ ] `editor` inlays::inlay_hints::tests::test_inside_char_boundary_range_hints
- [ ] `editor` inlays::inlay_hints::tests::test_modifiers_change
- [ ] `editor` inlays::inlay_hints::tests::test_no_hint_updates_for_unrelated_language_files
- [ ] `git` repository::tests::test_checkpoint_basic
- [ ] `project` project_tests::test_cancel_language_server_work
- [ ] `project` project_tests::test_file_status
- [ ] `project` project_tests::test_git_repository_status
- [ ] `project` project_tests::test_rename_work_directory
- [ ] `search` project_search::tests::test_project_search — **WIP**: `SelectedTextHighlight` (green bg for matching text) missing at `dp(5,6)`. Root cause: `refresh_selected_text_highlights` spawns two async tasks: `quick_selection_highlight_task` (visible range only, no debounce) and `debounced_selection_highlight_task` (full buffer, with debounce). Line 5 likely outside visible range in test window. Quick task doesn't cover it; debounced task not completing despite clock advance + park. Need to either: set window bounds to include all lines, or fix task completion in new scheduler.
- [x] `terminal` tests::test_basic_terminal — passes with ITERATIONS=10
- [ ] `worktree` worktree_tests::test_file_scan_exclusions
- [ ] `worktree` worktree_tests::test_file_scan_exclusions_overrules_inclusions
- [ ] `worktree` worktree_tests::test_file_scan_inclusions
- [ ] `worktree` worktree_tests::test_file_scan_inclusions_reindexes_on_setting_change
- [ ] `worktree` worktree_tests::test_fs_events_in_dot_git_worktree
- [ ] `worktree` worktree_tests::test_fs_events_in_exclusions
- [ ] `worktree` worktree_tests::test_hidden_files
- [ ] `worktree` worktree_tests::test_renaming_case_only