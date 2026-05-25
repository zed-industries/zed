use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Arc;

use fuzzy::StringMatchCandidate;
use gpui::{
    App, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, ParentElement,
    Render, SharedString, Styled, Subscription, Task, Window, rems,
};
use picker::{Picker, PickerDelegate};
use ui::{HighlightedLabel, Icon, IconName, IconSize, ListItem, ListItemSpacing, prelude::*};
use util::ResultExt as _;
use util::paths::PathExt;
use workspace::{ModalView, Workspace};

pub type OnWorktreeSelected = Rc<dyn Fn(PathBuf, &mut Window, &mut App) + 'static>;

pub struct TerminalWorktreePicker {
    picker: Entity<Picker<TerminalWorktreePickerDelegate>>,
    _subscription: Subscription,
}

impl TerminalWorktreePicker {
    pub fn new(
        entries: Vec<WorktreeEntry>,
        on_selected: OnWorktreeSelected,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let matches = (0..entries.len())
            .map(|entry_ix| EntryMatch {
                entry_ix,
                positions: Vec::new(),
            })
            .collect();
        let delegate = TerminalWorktreePickerDelegate {
            entries,
            matches,
            selected_index: 0,
            on_selected,
        };

        let picker = cx.new(|cx| {
            Picker::uniform_list(delegate, window, cx).max_height(Some(rems(20.).into()))
        });

        let subscription = cx.subscribe(&picker, |_, _, _: &DismissEvent, cx| {
            cx.emit(DismissEvent);
        });

        Self {
            picker,
            _subscription: subscription,
        }
    }
}

impl Focusable for TerminalWorktreePicker {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl EventEmitter<DismissEvent> for TerminalWorktreePicker {}
impl ModalView for TerminalWorktreePicker {}

impl Render for TerminalWorktreePicker {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .key_context("TerminalWorktreePicker")
            .w(rems(34.))
            .elevation_3(cx)
            .child(self.picker.clone())
    }
}

#[derive(Clone)]
pub struct WorktreeEntry {
    pub name: SharedString,
    pub path: PathBuf,
}

#[derive(Clone)]
struct EntryMatch {
    entry_ix: usize,
    positions: Vec<usize>,
}

pub struct TerminalWorktreePickerDelegate {
    entries: Vec<WorktreeEntry>,
    matches: Vec<EntryMatch>,
    selected_index: usize,
    on_selected: OnWorktreeSelected,
}

pub fn collect_worktree_entries(workspace: &Workspace, cx: &App) -> Vec<WorktreeEntry> {
    workspace
        .visible_worktrees(cx)
        .map(|worktree| {
            let worktree = worktree.read(cx);
            let path = worktree.abs_path().to_path_buf();
            let name = SharedString::from(worktree.root_name_str().to_string());
            WorktreeEntry { name, path }
        })
        .collect()
}

impl PickerDelegate for TerminalWorktreePickerDelegate {
    type ListItem = ListItem;

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        "Select a directory for the new terminal…".into()
    }

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(
        &mut self,
        ix: usize,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) {
        self.selected_index = ix;
    }

    fn update_matches(
        &mut self,
        query: String,
        _window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        if query.is_empty() {
            self.matches = (0..self.entries.len())
                .map(|entry_ix| EntryMatch {
                    entry_ix,
                    positions: Vec::new(),
                })
                .collect();
            self.selected_index = self.selected_index.min(self.matches.len().saturating_sub(1));
            return Task::ready(());
        }

        let candidates: Vec<_> = self
            .entries
            .iter()
            .enumerate()
            .map(|(ix, entry)| StringMatchCandidate::new(ix, entry.name.as_ref()))
            .collect();
        let executor = cx.background_executor().clone();
        let task = cx.background_executor().spawn(async move {
            fuzzy::match_strings(
                &candidates,
                &query,
                false,
                true,
                100,
                &Default::default(),
                executor,
            )
            .await
        });

        cx.spawn(async move |picker, cx| {
            let matches = task.await;
            picker
                .update(cx, |picker, cx| {
                    picker.delegate.matches = matches
                        .into_iter()
                        .map(|m| EntryMatch {
                            entry_ix: m.candidate_id,
                            positions: m.positions,
                        })
                        .collect();
                    picker.delegate.selected_index = 0;
                    cx.notify();
                })
                .log_err();
        })
    }

    fn confirm(&mut self, _secondary: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        let Some(entry_match) = self.matches.get(self.selected_index) else {
            return;
        };
        let Some(entry) = self.entries.get(entry_match.entry_ix) else {
            return;
        };
        let path = entry.path.clone();
        let on_selected = self.on_selected.clone();
        window.defer(cx, move |window, cx| {
            on_selected(path, window, cx);
        });
        cx.emit(DismissEvent);
    }

    fn dismissed(&mut self, _window: &mut Window, cx: &mut Context<Picker<Self>>) {
        cx.emit(DismissEvent);
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let entry_match = self.matches.get(ix)?;
        let entry = self.entries.get(entry_match.entry_ix)?;
        let display_path = entry.path.compact().to_string_lossy().to_string();

        Some(
            ListItem::new(SharedString::from(format!("terminal-worktree-{ix}")))
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .toggle_state(selected)
                .child(
                    h_flex()
                        .w_full()
                        .gap_2p5()
                        .child(
                            Icon::new(IconName::Folder)
                                .color(Color::Muted)
                                .size(IconSize::Small),
                        )
                        .child(
                            v_flex()
                                .w_full()
                                .min_w_0()
                                .child(
                                    HighlightedLabel::new(
                                        entry.name.to_string(),
                                        entry_match.positions.clone(),
                                    )
                                    .single_line()
                                    .truncate(),
                                )
                                .child(
                                    Label::new(display_path)
                                        .size(LabelSize::Small)
                                        .color(Color::Muted)
                                        .single_line()
                                        .truncate(),
                                ),
                        ),
                ),
        )
    }
}

#[cfg(test)]
impl TerminalWorktreePicker {
    pub(crate) fn inner_picker(&self) -> Entity<Picker<TerminalWorktreePickerDelegate>> {
        self.picker.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::terminal_panel::should_prompt_for_worktree;
    use gpui::{TestAppContext, VisualTestContext};
    use project::Project;
    use std::cell::RefCell;
    use std::path::Path;
    use std::rc::Rc;
    use workspace::{AppState, MultiWorkspace};

    async fn init_picker_test(
        cx: &mut TestAppContext,
        worktree_paths: &[&Path],
    ) -> (Entity<Workspace>, VisualTestContext) {
        let params = cx.update(AppState::test);
        cx.update(editor::init);
        let fs = params.fs.as_fake();
        for path in worktree_paths.iter().copied() {
            fs.insert_tree(path, serde_json::json!({})).await;
        }
        let project = Project::test(params.fs.clone(), worktree_paths.iter().copied(), cx).await;
        let window_handle =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = window_handle
            .read_with(cx, |mw, _| mw.workspace().clone())
            .unwrap();
        let visual_cx = VisualTestContext::from_window(window_handle.into(), cx);
        (workspace, visual_cx)
    }

    #[gpui::test]
    async fn picker_lists_visible_worktrees(cx: &mut TestAppContext) {
        let (workspace, mut cx) = init_picker_test(
            cx,
            &[
                Path::new(util::path!("/alpha")),
                Path::new(util::path!("/beta")),
            ],
        )
        .await;

        let entries =
            workspace.read_with(&cx, |ws, app| collect_worktree_entries(ws, app));
        let picker = cx.update(|window, cx| {
            cx.new(|cx| {
                TerminalWorktreePicker::new(entries.clone(), Rc::new(|_, _, _| {}), window, cx)
            })
        });

        let inner = picker.read_with(&cx, |p, _| p.inner_picker());
        let entries = inner.read_with(&cx, |picker, _| {
            picker
                .delegate
                .entries
                .iter()
                .map(|e| (e.name.to_string(), e.path.clone()))
                .collect::<Vec<_>>()
        });
        assert_eq!(entries.len(), 2);
        let names: Vec<String> = entries.iter().map(|(n, _)| n.clone()).collect();
        assert!(names.iter().any(|n| n == "alpha"), "missing alpha: {names:?}");
        assert!(names.iter().any(|n| n == "beta"), "missing beta: {names:?}");
    }

    #[gpui::test]
    async fn picker_confirm_invokes_callback_with_selected_path(cx: &mut TestAppContext) {
        let (workspace, mut cx) = init_picker_test(
            cx,
            &[
                Path::new(util::path!("/alpha")),
                Path::new(util::path!("/beta")),
            ],
        )
        .await;

        let chosen: Rc<RefCell<Option<PathBuf>>> = Rc::new(RefCell::new(None));
        let chosen_for_cb = chosen.clone();
        let entries =
            workspace.read_with(&cx, |ws, app| collect_worktree_entries(ws, app));
        let picker = cx.update(|window, cx| {
            cx.new(|cx| {
                TerminalWorktreePicker::new(
                    entries,
                    Rc::new(move |path, _, _| {
                        *chosen_for_cb.borrow_mut() = Some(path);
                    }),
                    window,
                    cx,
                )
            })
        });

        let inner = picker.read_with(&cx, |p, _| p.inner_picker());
        let entries = inner.read_with(&cx, |picker, _| {
            picker
                .delegate
                .entries
                .iter()
                .map(|e| (e.name.to_string(), e.path.clone()))
                .collect::<Vec<_>>()
        });
        let target_path = entries[1].1.clone();

        cx.update(|window, cx| {
            inner.update(cx, |picker, cx| {
                picker.delegate.selected_index = 1;
                picker.delegate.confirm(false, window, cx);
            });
        });
        cx.run_until_parked();

        let chosen = chosen.borrow().clone();
        assert_eq!(chosen.as_deref(), Some(target_path.as_path()));
    }

    #[gpui::test]
    async fn picker_filters_by_query(cx: &mut TestAppContext) {
        let (workspace, mut cx) = init_picker_test(
            cx,
            &[
                Path::new(util::path!("/alpha")),
                Path::new(util::path!("/beta")),
                Path::new(util::path!("/gamma")),
            ],
        )
        .await;

        let entries =
            workspace.read_with(&cx, |ws, app| collect_worktree_entries(ws, app));
        let picker = cx.update(|window, cx| {
            cx.new(|cx| {
                TerminalWorktreePicker::new(entries.clone(), Rc::new(|_, _, _| {}), window, cx)
            })
        });

        let inner = picker.read_with(&cx, |p, _| p.inner_picker());
        let task = cx.update(|window, cx| {
            inner.update(cx, |picker, cx| {
                picker.delegate.update_matches("bet".to_string(), window, cx)
            })
        });
        task.await;
        cx.run_until_parked();

        let match_count = inner.read_with(&cx, |picker, _| picker.delegate.matches.len());
        assert_eq!(match_count, 1, "expected only 'beta' to match");
    }

    fn set_prompt_setting(cx: &mut VisualTestContext, enabled: bool) {
        use gpui::UpdateGlobal;
        cx.update(|_, cx| {
            settings::SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings
                        .terminal
                        .get_or_insert_default()
                        .prompt_directory_for_new_terminals = Some(enabled);
                });
            });
        });
    }

    #[gpui::test]
    async fn does_not_prompt_with_zero_worktrees(cx: &mut TestAppContext) {
        let (workspace, mut cx) = init_picker_test(cx, &[]).await;
        set_prompt_setting(&mut cx, true);
        cx.read(|cx| {
            assert!(!should_prompt_for_worktree(workspace.read(cx), cx));
        });
    }

    #[gpui::test]
    async fn does_not_prompt_with_one_worktree(cx: &mut TestAppContext) {
        let (workspace, mut cx) =
            init_picker_test(cx, &[Path::new(util::path!("/only"))]).await;
        set_prompt_setting(&mut cx, true);
        cx.read(|cx| {
            assert!(!should_prompt_for_worktree(workspace.read(cx), cx));
        });
    }

    #[gpui::test]
    async fn prompts_with_multiple_worktrees_when_setting_enabled(cx: &mut TestAppContext) {
        let (workspace, mut cx) = init_picker_test(
            cx,
            &[Path::new(util::path!("/a")), Path::new(util::path!("/b"))],
        )
        .await;
        set_prompt_setting(&mut cx, true);
        cx.read(|cx| {
            assert!(should_prompt_for_worktree(workspace.read(cx), cx));
        });
    }

    #[gpui::test]
    async fn does_not_prompt_when_setting_disabled(cx: &mut TestAppContext) {
        let (workspace, _vc) = init_picker_test(
            cx,
            &[Path::new(util::path!("/a")), Path::new(util::path!("/b"))],
        )
        .await;
        cx.read(|cx| {
            assert!(!should_prompt_for_worktree(workspace.read(cx), cx));
        });
    }
}
