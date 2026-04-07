use std::path::PathBuf;
use std::sync::Arc;

use agent_settings::AgentSettings;
use fs::Fs;
use fuzzy::StringMatchCandidate;
use git::repository::Worktree as GitWorktree;
use gpui::{
    App, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, IntoElement,
    ParentElement, Render, SharedString, Styled, Task, Window, rems,
};
use picker::{Picker, PickerDelegate, PickerEditorPosition};
use project::{Project, git_store::RepositoryId};
use settings::{NewThreadLocation, Settings, update_settings_file};
use ui::{
    HighlightedLabel, Icon, IconName, Label, LabelCommon, ListItem, ListItemSpacing, Tooltip,
    prelude::*,
};
use util::ResultExt as _;

use crate::ui::HoldForDefault;
use crate::{NewWorktreeBranchTarget, StartThreadIn};

pub(crate) struct ThreadWorktreePicker {
    picker: Entity<Picker<ThreadWorktreePickerDelegate>>,
    focus_handle: FocusHandle,
    _subscription: gpui::Subscription,
}

impl ThreadWorktreePicker {
    pub fn new(
        project: Entity<Project>,
        current_target: &StartThreadIn,
        fs: Arc<dyn Fs>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let project_worktree_paths: Vec<PathBuf> = project
            .read(cx)
            .visible_worktrees(cx)
            .map(|wt| wt.read(cx).abs_path().to_path_buf())
            .collect();

        let preserved_branch_target = match current_target {
            StartThreadIn::NewWorktree { branch_target, .. } => branch_target.clone(),
            _ => NewWorktreeBranchTarget::default(),
        };

        let delegate = ThreadWorktreePickerDelegate {
            matches: vec![
                ThreadWorktreeEntry::CurrentWorktree,
                ThreadWorktreeEntry::NewWorktree,
            ],
            all_worktrees: project
                .read(cx)
                .repositories(cx)
                .iter()
                .map(|(repo_id, repo)| (*repo_id, repo.read(cx).linked_worktrees.clone()))
                .collect(),
            project_worktree_paths,
            selected_index: match current_target {
                StartThreadIn::LocalProject => 0,
                StartThreadIn::NewWorktree { .. } => 1,
                _ => 0,
            },
            project: project.clone(),
            preserved_branch_target,
            fs,
        };

        let picker = cx.new(|cx| {
            Picker::list(delegate, window, cx)
                .list_measure_all()
                .modal(false)
                .max_height(Some(rems(20.).into()))
        });

        let subscription = cx.subscribe(&picker, |_, _, _, cx| {
            cx.emit(DismissEvent);
        });

        Self {
            focus_handle: picker.focus_handle(cx),
            picker,
            _subscription: subscription,
        }
    }
}

impl Focusable for ThreadWorktreePicker {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<DismissEvent> for ThreadWorktreePicker {}

impl Render for ThreadWorktreePicker {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .w(rems(20.))
            .elevation_3(cx)
            .child(self.picker.clone())
            .on_mouse_down_out(cx.listener(|_, _, _, cx| {
                cx.emit(DismissEvent);
            }))
    }
}

#[derive(Clone)]
enum ThreadWorktreeEntry {
    CurrentWorktree,
    NewWorktree,
    LinkedWorktree {
        worktree: GitWorktree,
        positions: Vec<usize>,
    },
    CreateNamed {
        name: String,
        disabled_reason: Option<String>,
    },
}

pub(crate) struct ThreadWorktreePickerDelegate {
    matches: Vec<ThreadWorktreeEntry>,
    all_worktrees: Vec<(RepositoryId, Arc<[GitWorktree]>)>,
    project_worktree_paths: Vec<PathBuf>,
    selected_index: usize,
    preserved_branch_target: NewWorktreeBranchTarget,
    project: Entity<Project>,
    fs: Arc<dyn Fs>,
}

impl ThreadWorktreePickerDelegate {
    fn new_worktree_action(&self, worktree_name: Option<String>) -> StartThreadIn {
        StartThreadIn::NewWorktree {
            worktree_name,
            branch_target: self.preserved_branch_target.clone(),
        }
    }

    fn sync_selected_index(&mut self) {
        if let Some(index) = self
            .matches
            .iter()
            .position(|entry| matches!(entry, ThreadWorktreeEntry::LinkedWorktree { .. }))
        {
            self.selected_index = index;
        } else if let Some(index) = self
            .matches
            .iter()
            .position(|entry| matches!(entry, ThreadWorktreeEntry::CreateNamed { .. }))
        {
            self.selected_index = index;
        } else {
            self.selected_index = 0;
        }
    }
}

impl PickerDelegate for ThreadWorktreePickerDelegate {
    type ListItem = ListItem;

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        "Search or create worktrees…".into()
    }

    fn editor_position(&self) -> PickerEditorPosition {
        PickerEditorPosition::Start
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

    fn separators_after_indices(&self) -> Vec<usize> {
        if self.matches.len() > 2 {
            vec![1]
        } else {
            Vec::new()
        }
    }

    fn update_matches(
        &mut self,
        query: String,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        let has_multiple_repositories = self.all_worktrees.len() > 1;

        let linked_worktrees: Vec<_> = if has_multiple_repositories {
            Vec::new()
        } else {
            self.all_worktrees
                .iter()
                .flat_map(|(_, worktrees)| worktrees.iter())
                .filter(|worktree| {
                    !self
                        .project_worktree_paths
                        .iter()
                        .any(|project_path| project_path == &worktree.path)
                })
                .cloned()
                .collect()
        };

        let normalized_query = query.replace(' ', "-");
        let has_named_worktree = self.all_worktrees.iter().any(|(_, worktrees)| {
            worktrees
                .iter()
                .any(|worktree| worktree.display_name() == normalized_query)
        });
        let create_named_disabled_reason = if has_multiple_repositories {
            Some("Cannot create a named worktree in a project with multiple repositories".into())
        } else if has_named_worktree {
            Some("A worktree with this name already exists".into())
        } else {
            None
        };

        let mut matches = vec![
            ThreadWorktreeEntry::CurrentWorktree,
            ThreadWorktreeEntry::NewWorktree,
        ];

        if query.is_empty() {
            for worktree in &linked_worktrees {
                matches.push(ThreadWorktreeEntry::LinkedWorktree {
                    worktree: worktree.clone(),
                    positions: Vec::new(),
                });
            }
        } else if linked_worktrees.is_empty() {
            matches.push(ThreadWorktreeEntry::CreateNamed {
                name: normalized_query,
                disabled_reason: create_named_disabled_reason,
            });
        } else {
            let candidates: Vec<_> = linked_worktrees
                .iter()
                .enumerate()
                .map(|(ix, worktree)| StringMatchCandidate::new(ix, worktree.display_name()))
                .collect();

            let executor = cx.background_executor().clone();
            let query_clone = query.clone();

            let task = cx.background_executor().spawn(async move {
                fuzzy::match_strings(
                    &candidates,
                    &query_clone,
                    true,
                    true,
                    10000,
                    &Default::default(),
                    executor,
                )
                .await
            });

            let linked_worktrees_clone = linked_worktrees;
            return cx.spawn_in(window, async move |picker, cx| {
                let fuzzy_matches = task.await;

                picker
                    .update_in(cx, |picker, _window, cx| {
                        let mut new_matches = vec![
                            ThreadWorktreeEntry::CurrentWorktree,
                            ThreadWorktreeEntry::NewWorktree,
                        ];

                        for candidate in &fuzzy_matches {
                            new_matches.push(ThreadWorktreeEntry::LinkedWorktree {
                                worktree: linked_worktrees_clone[candidate.candidate_id].clone(),
                                positions: candidate.positions.clone(),
                            });
                        }

                        let has_exact_match = linked_worktrees_clone
                            .iter()
                            .any(|worktree| worktree.display_name() == query);

                        if !has_exact_match {
                            new_matches.push(ThreadWorktreeEntry::CreateNamed {
                                name: normalized_query.clone(),
                                disabled_reason: create_named_disabled_reason.clone(),
                            });
                        }

                        picker.delegate.matches = new_matches;
                        picker.delegate.sync_selected_index();

                        cx.notify();
                    })
                    .log_err();
            });
        }

        self.matches = matches;
        self.sync_selected_index();

        Task::ready(())
    }

    fn confirm(&mut self, secondary: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        let Some(entry) = self.matches.get(self.selected_index) else {
            return;
        };

        match entry {
            ThreadWorktreeEntry::CurrentWorktree => {
                if secondary {
                    update_settings_file(self.fs.clone(), cx, |settings, _| {
                        settings
                            .agent
                            .get_or_insert_default()
                            .set_new_thread_location(NewThreadLocation::LocalProject);
                    });
                }
                window.dispatch_action(Box::new(StartThreadIn::LocalProject), cx);
            }
            ThreadWorktreeEntry::NewWorktree => {
                if secondary {
                    update_settings_file(self.fs.clone(), cx, |settings, _| {
                        settings
                            .agent
                            .get_or_insert_default()
                            .set_new_thread_location(NewThreadLocation::NewWorktree);
                    });
                }
                window.dispatch_action(Box::new(self.new_worktree_action(None)), cx);
            }
            ThreadWorktreeEntry::LinkedWorktree { worktree, .. } => {
                window.dispatch_action(
                    Box::new(StartThreadIn::LinkedWorktree {
                        path: worktree.path.clone(),
                        display_name: worktree.display_name().to_string(),
                    }),
                    cx,
                );
            }
            ThreadWorktreeEntry::CreateNamed {
                name,
                disabled_reason: None,
            } => {
                window.dispatch_action(Box::new(self.new_worktree_action(Some(name.clone()))), cx);
            }
            ThreadWorktreeEntry::CreateNamed {
                disabled_reason: Some(_),
                ..
            } => {
                return;
            }
        }

        cx.emit(DismissEvent);
    }

    fn dismissed(&mut self, _window: &mut Window, _cx: &mut Context<Picker<Self>>) {}

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let entry = self.matches.get(ix)?;
        let project = self.project.read(cx);
        let is_new_worktree_disabled =
            project.repositories(cx).is_empty() || project.is_via_collab();
        let new_thread_location = AgentSettings::get_global(cx).new_thread_location;
        let is_local_default = new_thread_location == NewThreadLocation::LocalProject;
        let is_new_worktree_default = new_thread_location == NewThreadLocation::NewWorktree;

        match entry {
            ThreadWorktreeEntry::CurrentWorktree => Some(
                ListItem::new("current-worktree")
                    .inset(true)
                    .spacing(ListItemSpacing::Sparse)
                    .toggle_state(selected)
                    .start_slot(Icon::new(IconName::Folder).color(Color::Muted))
                    .child(Label::new("Current Worktree"))
                    .end_slot(HoldForDefault::new(is_local_default).more_content(false))
                    .tooltip(Tooltip::text("Use the current project worktree")),
            ),
            ThreadWorktreeEntry::NewWorktree => {
                let item = ListItem::new("new-worktree")
                    .inset(true)
                    .spacing(ListItemSpacing::Sparse)
                    .toggle_state(selected)
                    .disabled(is_new_worktree_disabled)
                    .start_slot(
                        Icon::new(IconName::Plus).color(if is_new_worktree_disabled {
                            Color::Disabled
                        } else {
                            Color::Muted
                        }),
                    )
                    .child(
                        Label::new("New Git Worktree").color(if is_new_worktree_disabled {
                            Color::Disabled
                        } else {
                            Color::Default
                        }),
                    );

                Some(if is_new_worktree_disabled {
                    item.tooltip(Tooltip::text("Requires a Git repository in the project"))
                } else {
                    item.end_slot(HoldForDefault::new(is_new_worktree_default).more_content(false))
                        .tooltip(Tooltip::text("Start a thread in a new Git worktree"))
                })
            }
            ThreadWorktreeEntry::LinkedWorktree {
                worktree,
                positions,
            } => {
                let display_name = worktree.display_name();
                let first_line = display_name.lines().next().unwrap_or(display_name);
                let positions: Vec<_> = positions
                    .iter()
                    .copied()
                    .filter(|&pos| pos < first_line.len())
                    .collect();

                Some(
                    ListItem::new(SharedString::from(format!("linked-worktree-{ix}")))
                        .inset(true)
                        .spacing(ListItemSpacing::Sparse)
                        .toggle_state(selected)
                        .start_slot(Icon::new(IconName::GitWorktree).color(Color::Muted))
                        .child(HighlightedLabel::new(first_line.to_owned(), positions).truncate()),
                )
            }
            ThreadWorktreeEntry::CreateNamed {
                name,
                disabled_reason,
            } => {
                let is_disabled = disabled_reason.is_some();
                let item = ListItem::new("create-named-worktree")
                    .inset(true)
                    .spacing(ListItemSpacing::Sparse)
                    .toggle_state(selected)
                    .disabled(is_disabled)
                    .start_slot(Icon::new(IconName::Plus).color(if is_disabled {
                        Color::Disabled
                    } else {
                        Color::Accent
                    }))
                    .child(Label::new(format!("Create Worktree: \"{name}\"…")).color(
                        if is_disabled {
                            Color::Disabled
                        } else {
                            Color::Default
                        },
                    ));

                Some(if let Some(reason) = disabled_reason.clone() {
                    item.tooltip(Tooltip::text(reason))
                } else {
                    item
                })
            }
        }
    }

    fn no_matches_text(&self, _window: &mut Window, _cx: &mut App) -> Option<SharedString> {
        None
    }
}
