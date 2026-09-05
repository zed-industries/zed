#[cfg(test)]
mod recent_files_tests;

use std::sync::Arc;

use futures::future::join_all;
use gpui::{
    Action, App, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, Modifiers,
    ModifiersChangedEvent, ParentElement, Render, SharedString, Styled, Task, TaskExt, WeakEntity,
    Window, actions, rems,
};
use picker::{Picker, PickerDelegate};
use project::ProjectPath;
use ui::{HighlightedLabel, ListItem, ListItemSpacing, prelude::*};
use util::ResultExt;
use workspace::{ModalView, Workspace};

use crate::FoundPath;

const PANEL_WIDTH_REMS: f32 = 34.;
const MAX_MATCHES: usize = 100;

actions!(
    recent_files,
    [
        /// Toggles the recently opened files palette.
        Toggle
    ]
);

pub fn init(cx: &mut App) {
    cx.observe_new(RecentFiles::register).detach();
}

pub struct RecentFiles {
    picker: Entity<Picker<RecentFilesDelegate>>,
    init_modifiers: Option<Modifiers>,
}

impl ModalView for RecentFiles {}

impl RecentFiles {
    fn register(
        workspace: &mut Workspace,
        _window: Option<&mut Window>,
        _: &mut Context<Workspace>,
    ) {
        workspace.register_action(|workspace, _: &Toggle, window, cx| {
            let Some(recent_files) = workspace.active_modal::<Self>(cx) else {
                Self::open(workspace, window, cx).detach();
                return;
            };

            recent_files.update(cx, |recent_files, cx| {
                recent_files.init_modifiers = Some(window.modifiers());
                recent_files.picker.update(cx, |picker, cx| {
                    picker.cycle_selection(window, cx);
                });
            });
        });
    }

    fn open(
        workspace: &mut Workspace,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Task<()> {
        let project = workspace.project().read(cx);
        let fs = project.fs().clone();

        let currently_opened_project_path = workspace
            .active_item(cx)
            .and_then(|item| item.project_path(cx));

        // Same resolution as `FileFinder::open`'s history_items: prefer the fast
        // in-worktree check, and fall back to an async filesystem existence check
        // for history entries whose worktree isn't currently loaded.
        let history_items = workspace
            .recent_navigation_history(None, cx)
            .into_iter()
            .filter_map(|(project_path, abs_path)| {
                if project.entry_for_path(&project_path, cx).is_some() {
                    return Some(Task::ready(Some(FoundPath::new(project_path, abs_path?))));
                }
                let abs_path = abs_path?;
                if project.is_local() {
                    let fs = fs.clone();
                    Some(cx.background_spawn(async move {
                        if fs.is_file(&abs_path).await {
                            Some(FoundPath::new(project_path, abs_path))
                        } else {
                            None
                        }
                    }))
                } else {
                    Some(Task::ready(Some(FoundPath::new(project_path, abs_path))))
                }
            })
            .collect::<Vec<_>>();

        cx.spawn_in(window, async move |workspace, cx| {
            let history_items: Vec<FoundPath> = join_all(history_items)
                .await
                .into_iter()
                .flatten()
                .collect();

            workspace
                .update_in(cx, |workspace, window, cx| {
                    let weak_workspace = cx.entity().downgrade();
                    workspace.toggle_modal(window, cx, |window, cx| {
                        let delegate = RecentFilesDelegate::new(
                            cx.entity().downgrade(),
                            weak_workspace,
                            currently_opened_project_path,
                            history_items,
                        );
                        RecentFiles::new(delegate, window, cx)
                    });
                })
                .ok();
        })
    }

    fn new(delegate: RecentFilesDelegate, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let init_modifiers = window.modifiers().modified().then_some(window.modifiers());
        Self {
            picker: cx
                .new(|cx| Picker::list(delegate, window, cx).initial_width(rems(PANEL_WIDTH_REMS))),
            init_modifiers,
        }
    }

    fn handle_modifiers_changed(
        &mut self,
        event: &ModifiersChangedEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(init_modifiers) = self.init_modifiers else {
            return;
        };
        if !event.modified() || !init_modifiers.is_subset_of(event) {
            self.init_modifiers = None;
            if self.picker.read(cx).delegate.matches.is_empty() {
                cx.emit(DismissEvent)
            } else {
                window.dispatch_action(menu::Confirm.boxed_clone(), cx);
            }
        }
    }
}

impl EventEmitter<DismissEvent> for RecentFiles {}

impl Focusable for RecentFiles {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl Render for RecentFiles {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .key_context("RecentFiles")
            .w(rems(PANEL_WIDTH_REMS))
            .on_modifiers_changed(cx.listener(Self::handle_modifiers_changed))
            .child(self.picker.clone())
    }
}

/// A single candidate in the recent-files list: a resolved path plus the
/// pre-split display strings so matching only needs to run against the file
/// name, not the whole absolute path.
#[derive(Clone)]
struct RecentFileEntry {
    path: FoundPath,
    file_name: SharedString,
    parent_path: SharedString,
}

impl RecentFileEntry {
    fn new(path: FoundPath) -> Self {
        let file_name = path
            .absolute
            .file_name()
            .map_or_else(String::new, |name| name.to_string_lossy().into_owned());
        let parent_path = path
            .absolute
            .parent()
            .map_or_else(String::new, |parent| parent.to_string_lossy().into_owned());
        Self {
            path,
            file_name: file_name.into(),
            parent_path: parent_path.into(),
        }
    }
}

struct RecentFileMatch {
    entry_index: usize,
    positions: Vec<usize>,
}

pub struct RecentFilesDelegate {
    recent_files: WeakEntity<RecentFiles>,
    workspace: WeakEntity<Workspace>,
    currently_opened_project_path: Option<ProjectPath>,
    selected_index: usize,
    entries: Vec<RecentFileEntry>,
    matches: Vec<RecentFileMatch>,
}

impl RecentFilesDelegate {
    fn new(
        recent_files: WeakEntity<RecentFiles>,
        workspace: WeakEntity<Workspace>,
        currently_opened_project_path: Option<ProjectPath>,
        history_items: Vec<FoundPath>,
    ) -> Self {
        // Most-recent-first order comes from `recent_navigation_history`; the
        // currently open file (if present in history) stays in the list so the
        // palette reads the same as the editor's tab order.
        let entries = history_items
            .into_iter()
            .map(RecentFileEntry::new)
            .collect();

        Self {
            recent_files,
            workspace,
            currently_opened_project_path,
            selected_index: 0,
            entries,
            matches: Vec::new(),
        }
    }

    fn unfiltered_matches(entry_count: usize) -> Vec<RecentFileMatch> {
        (0..entry_count)
            .map(|entry_index| RecentFileMatch {
                entry_index,
                positions: Vec::new(),
            })
            .collect()
    }

    /// Index of the first entry that isn't the currently open file, so that
    /// pressing Enter with no query immediately switches to a different file
    /// rather than reopening the one already on screen.
    fn default_selected_index(&self) -> usize {
        if self.matches.len() > 1
            && let Some(currently_opened_project_path) = &self.currently_opened_project_path
            && self
                .entries
                .first()
                .is_some_and(|entry| &entry.path.project == currently_opened_project_path)
        {
            1
        } else {
            0
        }
    }
}

impl PickerDelegate for RecentFilesDelegate {
    type ListItem = ListItem;

    fn name() -> &'static str {
        "recent files"
    }

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        "Search recently opened files…".into()
    }

    fn no_matches_text(&self, _window: &mut Window, _cx: &mut App) -> Option<SharedString> {
        Some("No recently opened files".into())
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
        cx: &mut Context<Picker<Self>>,
    ) {
        self.selected_index = ix;
        cx.notify();
    }

    fn separators_after_indices(&self) -> Vec<usize> {
        Vec::new()
    }

    fn update_matches(
        &mut self,
        query: String,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        if query.is_empty() {
            self.matches = Self::unfiltered_matches(self.entries.len());
            self.selected_index = self.default_selected_index();
        } else {
            let candidates: Vec<_> = self
                .entries
                .iter()
                .enumerate()
                .map(|(id, entry)| {
                    fuzzy_nucleo::StringMatchCandidate::from_shared(id, entry.file_name.clone())
                })
                .collect();
            self.matches = fuzzy_nucleo::match_strings(
                &candidates,
                &query,
                fuzzy_nucleo::Case::Smart,
                fuzzy_nucleo::LengthPenalty::On,
                MAX_MATCHES,
            )
            .into_iter()
            .map(|string_match| RecentFileMatch {
                entry_index: string_match.candidate_id,
                positions: string_match.positions,
            })
            .collect();
            self.selected_index = 0;
        }
        Task::ready(())
    }

    fn confirm(&mut self, _secondary: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        let Some(entry_match) = self.matches.get(self.selected_index) else {
            return;
        };
        let Some(entry) = self.entries.get(entry_match.entry_index) else {
            return;
        };
        let Some(workspace) = self.workspace.upgrade() else {
            return;
        };

        let project_path = entry.path.project.clone();
        workspace.update(cx, |workspace, cx| {
            workspace
                .open_path(project_path, None, true, window, cx)
                .detach_and_log_err(cx);
        });

        self.recent_files
            .update(cx, |_, cx| cx.emit(DismissEvent))
            .log_err();
    }

    fn dismissed(&mut self, _window: &mut Window, cx: &mut Context<Picker<Self>>) {
        self.recent_files
            .update(cx, |_, cx| cx.emit(DismissEvent))
            .log_err();
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let entry_match = self.matches.get(ix)?;
        let entry = self.entries.get(entry_match.entry_index)?;

        Some(
            ListItem::new(ix)
                .spacing(ListItemSpacing::Sparse)
                .inset(true)
                .toggle_state(selected)
                .child(
                    v_flex()
                        .w_full()
                        .min_w_0()
                        .overflow_hidden()
                        .child(HighlightedLabel::new(
                            entry.file_name.clone(),
                            entry_match.positions.clone(),
                        ))
                        .child(
                            Label::new(entry.parent_path.clone())
                                .size(LabelSize::Small)
                                .color(Color::Muted),
                        ),
                ),
        )
    }
}
