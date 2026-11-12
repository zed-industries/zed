use super::*;
use crate::branch_diff::BranchDiff;
use project::git_store::diff_buffer_list::{BranchDiffEvent, DiffBase, DiffBufferList};

/// The read-only list of files changed against a chosen branch. It is kept apart from the
/// panel's working-change entries so that staging and commit state never reflect it.
pub(super) struct BranchChanges {
    base_ref: SharedString,
    changed_files: Entity<DiffBufferList>,
    tree_state: TreeViewState,
    visible_rows: Vec<GitListEntry>,
    selected_row: Option<usize>,
    scroll_handle: UniformListScrollHandle,
    _subscriptions: [Subscription; 2],
}

impl GitPanel {
    pub(super) fn choose_branch_comparison(
        &mut self,
        _: &ChooseBranchComparison,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(repository) = self.active_repository.clone() else {
            return;
        };
        let current_base_ref = self
            .branch_changes
            .as_ref()
            .map(|branch_changes| branch_changes.base_ref.clone());
        let on_select = Arc::new({
            let git_panel = cx.weak_entity();
            move |branch: Branch, window: &mut Window, cx: &mut App| {
                git_panel
                    .update(cx, |git_panel, cx| {
                        git_panel.compare_with_branch(branch.name().to_owned().into(), window, cx);
                    })
                    .log_err();
            }
        });
        self.workspace
            .update(cx, |workspace, cx| {
                let workspace_handle = workspace.weak_handle();
                workspace.toggle_modal(window, cx, |window, cx| {
                    branch_picker::select_modal(
                        workspace_handle,
                        Some(repository),
                        current_base_ref,
                        on_select,
                        window,
                        cx,
                    )
                });
            })
            .log_err();
    }

    pub(super) fn compare_with_branch(
        &mut self,
        base_ref: SharedString,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(repository) = self.active_repository.clone() else {
            return;
        };
        let git_store = self.project.read(cx).git_store().clone();
        let changed_files = cx.new(|cx| {
            DiffBufferList::new(
                DiffBase::Merge {
                    base_ref: base_ref.clone(),
                },
                git_store,
                Some(repository),
                cx,
            )
        });
        let subscriptions = [
            cx.subscribe(&changed_files, |this, _, event, cx| {
                if matches!(event, BranchDiffEvent::FileListChanged) {
                    this.rebuild_branch_changes(cx);
                }
            }),
            cx.observe(&changed_files, |_, _, cx| cx.notify()),
        ];
        self.branch_changes = Some(BranchChanges {
            base_ref,
            changed_files,
            tree_state: TreeViewState::default(),
            visible_rows: Vec::new(),
            selected_row: None,
            scroll_handle: UniformListScrollHandle::new(),
            _subscriptions: subscriptions,
        });
        self.set_active_tab(GitPanelTab::BranchChanges, window, cx);
        cx.notify();
    }

    pub(super) fn close_branch_changes(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.branch_changes.take().is_none() {
            return;
        }
        if self.active_tab == GitPanelTab::BranchChanges {
            // A repository switch also closes the comparison, and must not pull focus
            // into the panel from wherever the user is working.
            if self.focus_handle.contains_focused(window, cx) {
                self.set_active_tab(GitPanelTab::Changes, window, cx);
            } else {
                self.active_tab = GitPanelTab::Changes;
            }
        }
        cx.notify();
    }

    pub(super) fn rebuild_branch_changes(&mut self, cx: &mut Context<Self>) {
        let Some(branch_changes) = self.branch_changes.as_mut() else {
            return;
        };
        let selected_path = branch_changes
            .selected_row
            .and_then(|row| branch_changes.visible_rows.get(row))
            .and_then(GitListEntry::repo_path)
            .cloned();
        let mut changed_files = branch_changes
            .changed_files
            .read(cx)
            .statuses_by_path()
            .map(|statuses| {
                statuses
                    .iter()
                    .map(|entry| GitStatusEntry {
                        repo_path: entry.repo_path.clone(),
                        status: entry.status,
                        staging: entry.status.staging(),
                        diff_stat: entry.diff_stat,
                    })
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        let settings = GitPanelSettings::get_global(cx);
        branch_changes.visible_rows = if settings.tree_view {
            let mut seen_directories = HashSet::default();
            branch_changes.tree_state.directory_descendants.clear();
            let rows = branch_changes.tree_state.build_tree_entries(
                Section::Tracked,
                changed_files,
                &mut seen_directories,
            );
            branch_changes
                .tree_state
                .expanded_dirs
                .retain(|key, _| seen_directories.contains(key));
            rows.into_iter()
                .filter_map(|(row, is_visible)| is_visible.then_some(row))
                .collect()
        } else {
            match settings.sort_by {
                GitPanelSortBy::Path => {
                    changed_files.sort_by(|a, b| a.repo_path.cmp(&b.repo_path));
                }
                GitPanelSortBy::Name => changed_files.sort_by(|a, b| {
                    a.repo_path
                        .file_name()
                        .cmp(&b.repo_path.file_name())
                        .then_with(|| a.repo_path.cmp(&b.repo_path))
                }),
            }
            changed_files
                .into_iter()
                .map(GitListEntry::Status)
                .collect()
        };
        branch_changes.selected_row = selected_path.and_then(|selected_path| {
            branch_changes
                .visible_rows
                .iter()
                .position(|row| row.repo_path() == Some(&selected_path))
        });
        cx.notify();
    }

    /// Selects the row returned by `row`, which receives the current selection and the last row.
    pub(super) fn select_branch_changes_row(
        &mut self,
        row: impl FnOnce(Option<usize>, usize) -> usize,
        cx: &mut Context<Self>,
    ) {
        let Some(branch_changes) = self.branch_changes.as_mut() else {
            return;
        };
        let Some(last_row) = branch_changes.visible_rows.len().checked_sub(1) else {
            return;
        };
        let row = row(branch_changes.selected_row, last_row).min(last_row);
        branch_changes.selected_row = Some(row);
        branch_changes
            .scroll_handle
            .scroll_to_item(row, ScrollStrategy::Center);
        cx.notify();
    }

    pub(super) fn open_selected_branch_change(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(branch_changes) = self.branch_changes.as_mut() else {
            return;
        };
        let Some(row) = branch_changes
            .selected_row
            .and_then(|row| branch_changes.visible_rows.get(row))
        else {
            return;
        };
        if let GitListEntry::Directory(directory) = row {
            let expanded = branch_changes
                .tree_state
                .expanded_dirs
                .entry(directory.key.clone())
                .or_insert(true);
            *expanded = !*expanded;
            self.rebuild_branch_changes(cx);
        } else if let Some(entry) = row.status_entry().cloned() {
            self.open_branch_diff(Some(entry), window, cx);
        }
    }

    fn open_branch_diff(
        &mut self,
        entry: Option<GitStatusEntry>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(branch_changes) = self.branch_changes.as_ref() else {
            return;
        };
        let Some(repository) = branch_changes.changed_files.read(cx).repo().cloned() else {
            return;
        };
        let base_ref = branch_changes.base_ref.clone();
        let project = self.project.clone();
        self.workspace
            .update(cx, |workspace, cx| {
                BranchDiff::deploy_branch_diff_with_base_ref(
                    workspace, project, repository, base_ref, None, entry, window, cx,
                );
            })
            .log_err();
    }

    pub(super) fn render_branch_changes(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        v_flex()
            .flex_1()
            .size_full()
            .overflow_hidden()
            .children(self.render_branch_changes_header(cx))
            .children(self.render_branch_changes_list(window, cx))
    }

    fn render_branch_changes_header(&self, cx: &mut Context<Self>) -> Option<impl IntoElement> {
        let branch_changes = self.branch_changes.as_ref()?;
        Some(
            h_flex()
                .min_h(Tab::container_height(cx))
                .w_full()
                .pl_1()
                .pr_2()
                .flex_none()
                .flex_wrap()
                .gap_1()
                .justify_between()
                .child(
                    ButtonLike::new("branch-diff-button")
                        .child(
                            h_flex()
                                .gap_1()
                                .child(
                                    Icon::new(IconName::Diff)
                                        .size(IconSize::Small)
                                        .color(Color::Muted),
                                )
                                .child(
                                    Label::new("View Diff")
                                        .size(LabelSize::Small)
                                        .color(Color::Muted),
                                ),
                        )
                        .tooltip(Tooltip::text("View Branch Diff"))
                        .on_click(cx.listener(|this, _, window, cx| {
                            this.open_branch_diff(None, window, cx);
                        })),
                )
                .child(
                    h_flex()
                        .gap_1()
                        .child(
                            Button::new(
                                "branch-changes-base",
                                format!("Base: {}", branch_changes.base_ref),
                            )
                            .label_size(LabelSize::Small)
                            .truncate(true)
                            .tooltip(Tooltip::text("Select Base Branch"))
                            .on_click(cx.listener(
                                |this, _, window, cx| {
                                    this.choose_branch_comparison(
                                        &ChooseBranchComparison,
                                        window,
                                        cx,
                                    );
                                },
                            )),
                        )
                        .child(self.render_view_options_menu("branch_changes_view_options_menu"))
                        .child(
                            IconButton::new("close-branch-changes", IconName::Close)
                                .icon_size(IconSize::Small)
                                .tooltip(Tooltip::text("Close Branch Comparison"))
                                .on_click(cx.listener(|this, _, window, cx| {
                                    this.close_branch_changes(window, cx);
                                })),
                        ),
                ),
        )
    }

    fn render_branch_changes_list(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let branch_changes = self.branch_changes.as_ref()?;
        if branch_changes.visible_rows.is_empty() {
            let changed_files = branch_changes.changed_files.read(cx);
            let base_ref = &branch_changes.base_ref;
            let message = if changed_files.is_tree_base_loading() {
                "Loading changes…".to_string()
            } else if changed_files.statuses_by_path().is_none() {
                format!("Could not load changes against {base_ref}")
            } else {
                format!("No changes against {base_ref}")
            };
            return Some(
                h_flex()
                    .flex_1()
                    .justify_center()
                    .child(Label::new(message).color(Color::Muted))
                    .into_any_element(),
            );
        }

        let scroll_handle = branch_changes.scroll_handle.clone();
        Some(
            v_flex()
                .flex_1()
                .size_full()
                .overflow_hidden()
                .child(
                    uniform_list(
                        "branch_changes_list",
                        branch_changes.visible_rows.len(),
                        cx.processor(|this, range: Range<usize>, window, cx| {
                            let Some(branch_changes) = this.branch_changes.as_ref() else {
                                return Vec::new();
                            };
                            range
                                .filter_map(|row_index| {
                                    let row = branch_changes.visible_rows.get(row_index)?;
                                    let selected = branch_changes.selected_row == Some(row_index);
                                    this.render_branch_changes_row(
                                        row_index, row, selected, window, cx,
                                    )
                                })
                                .collect()
                        }),
                    )
                    .size_full()
                    .track_scroll(&scroll_handle),
                )
                .vertical_scrollbar_for(&scroll_handle, window, cx)
                .into_any_element(),
        )
    }

    fn render_branch_changes_row(
        &self,
        row_index: usize,
        row: &GitListEntry,
        selected: bool,
        window: &Window,
        cx: &Context<Self>,
    ) -> Option<AnyElement> {
        let name_row = match row {
            GitListEntry::Status(entry) => self.status_entry_name_row(entry, 0, cx),
            GitListEntry::TreeStatus(entry) => {
                self.status_entry_name_row(&entry.entry, entry.depth, cx)
            }
            GitListEntry::Directory(directory) => self.directory_entry_name_row(directory, cx),
            GitListEntry::Header(_) | GitListEntry::EmptySection(_) => return None,
        };
        let colors = cx.theme().colors();
        let background = if selected {
            cx.theme().status().info.alpha(0.08)
        } else {
            colors.ghost_element_background
        };
        let hover_background = colors.ghost_element_hover;
        Some(
            h_flex()
                .id(("branch-change", row_index))
                .h(self.list_item_height())
                .w_full()
                .pl_2p5()
                .pr_1()
                .border_1()
                .border_r_2()
                .when(selected && self.focus_handle.is_focused(window), |row| {
                    row.border_color(colors.panel_focused_border)
                })
                .bg(background)
                .hover(|style| style.bg(hover_background))
                .cursor_pointer()
                .child(name_row)
                .on_click(cx.listener(move |this, _: &ClickEvent, window, cx| {
                    if let Some(branch_changes) = this.branch_changes.as_mut() {
                        branch_changes.selected_row = Some(row_index);
                    }
                    this.open_selected_branch_change(window, cx);
                }))
                .into_any_element(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::{init_test, setup_git_panel_with_changes};
    use super::*;
    use git::repository::repo_path;
    use gpui::{TestAppContext, UpdateGlobal, VisualTestContext};
    use project::FakeFs;
    use serde_json::json;
    use util::path;

    async fn panel_on_branch_with_committed_changes(
        working_file_content: &str,
        cx: &mut TestAppContext,
    ) -> (
        Arc<FakeFs>,
        Entity<Workspace>,
        Entity<GitPanel>,
        VisualTestContext,
    ) {
        init_test(cx);
        let (fs, _project, workspace, panel, mut cx) = setup_git_panel_with_changes(
            cx,
            json!({
                ".git": {},
                "src": { "a.txt": "branch a", "b.txt": "branch b" },
                "same.txt": "same",
                "working.txt": working_file_content,
            }),
            &[],
        )
        .await;
        fs.set_head_and_index_for_repo(
            path!("/project/.git").as_ref(),
            &[
                ("src/a.txt", "branch a".into()),
                ("src/b.txt", "branch b".into()),
                ("same.txt", "same".into()),
                ("working.txt", "committed".into()),
            ],
        );
        fs.set_merge_base_content_for_repo(
            path!("/project/.git").as_ref(),
            &[
                ("src/a.txt", "base a".into()),
                ("same.txt", "same".into()),
                ("working.txt", "committed".into()),
                ("gone.txt", "gone".into()),
            ],
        );
        cx.run_until_parked();
        cx.executor().advance_clock(2 * UPDATE_DEBOUNCE);
        cx.run_until_parked();
        workspace.update_in(&mut cx, |workspace, window, cx| {
            workspace.add_panel(panel.clone(), window, cx);
            workspace.open_panel::<GitPanel>(window, cx);
        });
        (fs, workspace, panel, cx)
    }

    fn compare_with_main(panel: &Entity<GitPanel>, cx: &mut VisualTestContext) {
        panel.update_in(cx, |panel, window, cx| {
            panel.compare_with_branch("main".into(), window, cx);
            panel.focus_handle.focus(window, cx);
        });
        cx.run_until_parked();
        cx.update(|window, cx| window.draw(cx).clear(cx));
    }

    fn visible_rows(panel: &Entity<GitPanel>, cx: &mut VisualTestContext) -> Vec<String> {
        panel.read_with(cx, |panel, _| {
            panel
                .branch_changes
                .as_ref()
                .unwrap()
                .visible_rows
                .iter()
                .map(|row| row.repo_path().unwrap().as_unix_str().to_owned())
                .collect()
        })
    }

    fn working_paths(panel: &Entity<GitPanel>, cx: &mut VisualTestContext) -> Vec<String> {
        panel.read_with(cx, |panel, _| {
            panel
                .entries
                .iter()
                .filter_map(|entry| Some(entry.status_entry()?.repo_path.as_unix_str().to_owned()))
                .collect()
        })
    }

    #[gpui::test]
    async fn committed_branch_changes_are_listed_and_open_in_the_branch_diff(
        cx: &mut TestAppContext,
    ) {
        let (_fs, workspace, panel, mut cx) =
            panel_on_branch_with_committed_changes("committed", cx).await;
        assert_eq!(working_paths(&panel, &mut cx), Vec::<String>::new());

        compare_with_main(&panel, &mut cx);
        assert_eq!(
            visible_rows(&panel, &mut cx),
            ["gone.txt", "src/a.txt", "src/b.txt"]
        );
        assert_eq!(working_paths(&panel, &mut cx), Vec::<String>::new());

        cx.dispatch_action(menu::SelectFirst);
        cx.dispatch_action(menu::SelectNext);
        cx.dispatch_action(menu::SelectNext);
        cx.dispatch_action(menu::Confirm);
        cx.run_until_parked();
        workspace.read_with(&cx, |workspace, cx| {
            let branch_diff = workspace.active_item_as::<BranchDiff>(cx).unwrap();
            let branch_diff = branch_diff.read(cx);
            assert_eq!(
                branch_diff.diff_base(cx),
                &DiffBase::Merge {
                    base_ref: "main".into()
                }
            );
            assert_eq!(
                branch_diff.active_project_path(cx).unwrap().path.as_ref(),
                util::rel_path::rel_path("src/b.txt")
            );
        });

        cx.update(|_, cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings.git_panel.get_or_insert_default().tree_view = Some(true);
                })
            });
        });
        cx.run_until_parked();
        assert_eq!(
            visible_rows(&panel, &mut cx),
            ["src", "src/a.txt", "src/b.txt", "gone.txt"]
        );
        panel.update_in(&mut cx, |panel, window, cx| {
            panel.focus_handle.focus(window, cx);
        });
        cx.update(|window, cx| window.draw(cx).clear(cx));
        cx.dispatch_action(menu::SelectFirst);
        cx.dispatch_action(menu::Confirm);
        assert_eq!(visible_rows(&panel, &mut cx), ["src", "gone.txt"]);
    }

    #[gpui::test]
    async fn branch_changes_cannot_stage_or_discard_and_preserve_working_state(
        cx: &mut TestAppContext,
    ) {
        let (fs, _workspace, panel, mut cx) =
            panel_on_branch_with_committed_changes("uncommitted", cx).await;
        let staging = |panel: &Entity<GitPanel>, cx: &mut VisualTestContext| {
            panel.read_with(cx, |panel, cx| {
                let repository = panel.active_repository.as_ref().unwrap().read(cx);
                repository
                    .status_for_path(&repo_path("working.txt"))
                    .unwrap()
                    .status
                    .staging()
            })
        };
        assert_eq!(working_paths(&panel, &mut cx), ["working.txt"]);
        assert_eq!(staging(&panel, &mut cx), StageStatus::Unstaged);
        panel.update(&mut cx, |panel, cx| {
            panel
                .commit_message_buffer(cx)
                .update(cx, |buffer, cx| buffer.set_text("Draft message", cx));
        });

        compare_with_main(&panel, &mut cx);
        assert_eq!(
            visible_rows(&panel, &mut cx),
            ["gone.txt", "src/a.txt", "src/b.txt", "working.txt"]
        );
        cx.dispatch_action(menu::SelectLast);
        cx.dispatch_action(ToggleStaged);
        cx.dispatch_action(git::StageFile);
        cx.dispatch_action(RestoreFile { skip_prompt: true });
        cx.dispatch_action(RestoreTrackedFiles);
        cx.run_until_parked();
        cx.executor().advance_clock(2 * UPDATE_DEBOUNCE);
        cx.run_until_parked();
        assert_eq!(staging(&panel, &mut cx), StageStatus::Unstaged);
        assert_eq!(
            fs.load(path!("/project/working.txt").as_ref())
                .await
                .unwrap(),
            "uncommitted"
        );
        assert_eq!(working_paths(&panel, &mut cx), ["working.txt"]);

        cx.dispatch_action(ActivateChangesTab);
        cx.update(|window, cx| window.draw(cx).clear(cx));
        panel.read_with(&cx, |panel, cx| {
            assert_eq!(panel.active_tab, GitPanelTab::Changes);
            assert!(panel.branch_changes.is_some());
            assert_eq!(
                panel.commit_message_buffer(cx).read(cx).text(),
                "Draft message"
            );
        });
        cx.dispatch_action(menu::SelectFirst);
        cx.dispatch_action(ToggleStaged);
        cx.run_until_parked();
        cx.executor().advance_clock(2 * UPDATE_DEBOUNCE);
        cx.run_until_parked();
        assert_eq!(staging(&panel, &mut cx), StageStatus::Staged);

        panel.update_in(&mut cx, |panel, window, cx| {
            panel.set_active_tab(GitPanelTab::BranchChanges, window, cx);
            panel.close_branch_changes(window, cx);
            assert_eq!(panel.active_tab, GitPanelTab::Changes);
            assert!(panel.branch_changes.is_none());
        });
    }
}
