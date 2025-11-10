use anyhow::Context as _;
use fuzzy::StringMatchCandidate;

use collections::HashSet;
use git::{Remote, RemoteUrl};
use gpui::{
    App, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, InteractiveElement,
    IntoElement, Modifiers, ModifiersChangedEvent, ParentElement, Render, SharedString, Styled,
    Subscription, Task, WeakEntity, Window, rems,
};
use picker::{Picker, PickerDelegate, PickerEditorPosition};
use project::git_store::Repository;
use project::project_settings::ProjectSettings;
use settings::Settings;
use std::str::FromStr;
use std::sync::Arc;
use time::OffsetDateTime;
use ui::{HighlightedLabel, ListItem, ListItemSpacing, Tooltip, prelude::*};
use util::ResultExt;
use workspace::notifications::DetachAndPromptErr;
use workspace::{ModalView, Workspace};

use crate::remote_modal::RemoteModal;

pub fn register(workspace: &mut Workspace) {
    workspace.register_action(select_remote);
}

pub fn select_remote(
    workspace: &mut Workspace,
    _: &zed_actions::git::SelectRemote,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    open(workspace, &zed_actions::git::SelectRemote, window, cx);
}

pub fn open(
    workspace: &mut Workspace,
    _: &zed_actions::git::SelectRemote,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    let repository = workspace.project().read(cx).active_repository(cx);
    let style = RemoteListStyle::Modal;
    let workspace_weak = workspace.weak_handle();
    workspace.toggle_modal(window, cx, move |window, cx| {
        RemoteList::new(repository, style, rems(34.), workspace_weak, window, cx)
    })
}

pub fn popover(
    repository: Option<Entity<Repository>>,
    workspace: WeakEntity<Workspace>,
    window: &mut Window,
    cx: &mut App,
) -> Entity<RemoteList> {
    cx.new(move |cx| {
        let list = RemoteList::new(
            repository,
            RemoteListStyle::Popover,
            rems(20.),
            workspace,
            window,
            cx,
        );
        list.focus_handle(cx).focus(window);
        list
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum RemoteListStyle {
    Modal,
    Popover,
}

pub struct RemoteList {
    width: Rems,
    pub picker: Entity<Picker<RemoteListDelegate>>,
    _subscription: Subscription,
}

impl RemoteList {
    fn new(
        repository: Option<Entity<Repository>>,
        style: RemoteListStyle,
        width: Rems,
        workspace: WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let all_remotes_request = repository
            .clone()
            .map(|repository| repository.update(cx, |repository, _| repository.get_remotes(None)));

        cx.spawn_in(window, async move |this, cx| {
            let all_remotes = all_remotes_request
                .context("No active repository")?
                .await??;

            let _ = this.update_in(cx, |this, window, cx| {
                this.picker.update(cx, |picker, cx| {
                    let default_remote =
                        all_remotes.iter().enumerate().find_map(|(idx, remote)| {
                            (remote.name.as_str() == "origin").then(|| (idx, remote.name.clone()))
                        });
                    match default_remote {
                        Some((default_remote_idx, default_remote_name)) => {
                            picker.delegate.default_remote = Some(default_remote_name);
                            picker.delegate.selected_index = default_remote_idx;
                        }
                        None => {
                            picker.delegate.default_remote = None;
                        }
                    }

                    picker.delegate.all_remotes = Some(all_remotes);
                    picker.refresh(window, cx);
                })
            });

            anyhow::Ok(())
        })
        .detach_and_log_err(cx);

        let default_remote = repository
            .as_ref()
            .and_then(|repo| repo.read(cx).remote.as_ref())
            .map(|remote| remote.name.clone());
        let delegate = RemoteListDelegate::new(workspace, repository, default_remote, style);
        let picker = cx.new(|cx| Picker::uniform_list(delegate, window, cx));

        let _subscription = cx.subscribe(&picker, |_, _, _, cx| {
            cx.emit(DismissEvent);
        });

        Self {
            picker,
            width,
            _subscription,
        }
    }

    fn handle_modifiers_changed(
        &mut self,
        ev: &ModifiersChangedEvent,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.picker
            .update(cx, |picker, _| picker.delegate.modifiers = ev.modifiers)
    }
}
impl ModalView for RemoteList {}
impl EventEmitter<DismissEvent> for RemoteList {}

impl Focusable for RemoteList {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl Render for RemoteList {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .key_context("GitRemoteSelector")
            .w(self.width)
            .on_modifiers_changed(cx.listener(Self::handle_modifiers_changed))
            .child(self.picker.clone())
            .on_mouse_down_out({
                cx.listener(move |this, _, window, cx| {
                    this.picker.update(cx, |this, cx| {
                        this.cancel(&Default::default(), window, cx);
                    })
                })
            })
    }
}

#[derive(Debug, Clone)]
struct RemoteEntry {
    remote: Remote,
    positions: Vec<usize>,
    is_new: bool,
}

pub struct RemoteListDelegate {
    workspace: WeakEntity<Workspace>,
    matches: Vec<RemoteEntry>,
    all_remotes: Option<Vec<Remote>>,
    default_remote: Option<SharedString>,
    repo: Option<Entity<Repository>>,
    style: RemoteListStyle,
    selected_index: usize,
    last_query: String,
    modifiers: Modifiers,
}

impl RemoteListDelegate {
    fn new(
        workspace: WeakEntity<Workspace>,
        repo: Option<Entity<Repository>>,
        default_remote: Option<SharedString>,
        style: RemoteListStyle,
    ) -> Self {
        Self {
            workspace,
            matches: vec![],
            repo,
            style,
            all_remotes: None,
            default_remote,
            selected_index: 0,
            last_query: Default::default(),
            modifiers: Default::default(),
        }
    }

    fn open_create_remote(
        &self,
        new_remote_name: SharedString,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) {
        dbg!("open_create-remote");
        let Some(repository) = self.repo.clone() else {
            dbg!("return");
            return;
        };
        let this = cx.weak_entity();
        let window = window.window_handle();
        // FIXME it doesn't work
        window
            .update(cx, |_, window, cx| {
                dbg!("update");
                this.update(cx, |this, cx| {
                    dbg!("laaa");
                    this.delegate.workspace.update(cx, |workspace, cx| {
                        dbg!("maaaa");

                        workspace.toggle_modal(window, cx, |window, cx| {
                            dbg!("iciii");
                            RemoteModal::new(repository, new_remote_name.clone(), window, cx)
                        });
                    })
                })
            })
            .ok();
        cx.notify();
    }

    fn remove_remote(
        &self,
        remote_name: SharedString,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) {
        let Some(repo) = self.repo.clone() else {
            return;
        };

        cx.spawn(async move |_, cx| {
            repo.update(cx, |repo, _| repo.remove_remote(remote_name.to_string()))?
                .await??;

            anyhow::Ok(())
        })
        .detach_and_prompt_err("Failed to change remote", window, cx, |_, _, _| None);

        cx.notify();
        cx.emit(DismissEvent);
    }
}

impl PickerDelegate for RemoteListDelegate {
    type ListItem = ListItem;

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        "Select remote…".into()
    }

    fn editor_position(&self) -> PickerEditorPosition {
        match self.style {
            RemoteListStyle::Modal => PickerEditorPosition::Start,
            RemoteListStyle::Popover => PickerEditorPosition::End,
        }
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
        _: &mut Context<Picker<Self>>,
    ) {
        dbg!(ix);
        self.selected_index = ix;
    }

    fn update_matches(
        &mut self,
        query: String,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        let Some(all_remotes) = self.all_remotes.clone() else {
            return Task::ready(());
        };

        const RECENT_REMOTES_COUNT: usize = 10;
        cx.spawn_in(window, async move |picker, cx| {
            let mut matches: Vec<RemoteEntry> = if query.is_empty() {
                all_remotes
                    .into_iter()
                    .take(RECENT_REMOTES_COUNT)
                    .map(|remote| RemoteEntry {
                        remote,
                        positions: Vec::new(),
                        is_new: false,
                    })
                    .collect()
            } else {
                let candidates = all_remotes
                    .iter()
                    .enumerate()
                    .map(|(ix, remote)| StringMatchCandidate::new(ix, &remote.name))
                    .collect::<Vec<StringMatchCandidate>>();
                fuzzy::match_strings(
                    &candidates,
                    &query,
                    true,
                    true,
                    10000,
                    &Default::default(),
                    cx.background_executor().clone(),
                )
                .await
                .into_iter()
                .map(|candidate| RemoteEntry {
                    remote: all_remotes[candidate.candidate_id].clone(),
                    positions: candidate.positions,
                    is_new: false,
                })
                .collect()
            };
            picker
                .update(cx, |picker, _| {
                    if !query.is_empty()
                        && !matches
                            .first()
                            .is_some_and(|entry| entry.remote.name.as_str() == &query)
                    {
                        let query = query.replace(' ', "-");
                        matches.push(RemoteEntry {
                            remote: Remote {
                                name: query.into(),
                                url: RemoteUrl::default(),
                            },
                            positions: Vec::new(),
                            is_new: true,
                        })
                    }
                    let delegate = &mut picker.delegate;
                    delegate.matches = matches;
                    if delegate.matches.is_empty() {
                        delegate.selected_index = 0;
                    } else {
                        delegate.selected_index =
                            core::cmp::min(delegate.selected_index, delegate.matches.len() - 1);
                    }
                    delegate.last_query = query;
                })
                .log_err();
        })
    }

    fn confirm(&mut self, _secondary: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        let Some(entry) = self.matches.get(self.selected_index()) else {
            return;
        };

        if entry.is_new {
            dbg!("confirm");
            self.open_create_remote(entry.remote.name.to_owned(), window, cx);
            return;
        }

        // TODO: get it from local storage
        // let current_remote = self.repo.as_ref().map(|repo| {
        //     repo.read_with(cx, |repo, _| {
        //         repo.branch.as_ref().map(|branch| branch.ref_name.clone())
        //     })
        // });

        // if current_remote
        //     .flatten()
        //     .is_some_and(|current_branch| current_branch == entry.remote.ref_name)
        // {
        //     cx.emit(DismissEvent);
        //     return;
        // }
        dbg!(&self.repo.is_some());
        dbg!(&entry.remote);
        let Some(repo) = self.repo.clone() else {
            return;
        };
        let remote_name = entry.remote.name.to_string();
        let Some(repo) = self.repo.clone() else {
            return;
        };
        cx.spawn(async move |_, cx| {
            dbg!("CHANGE REMOTE !!!!!!!!!!!!");

            repo.update(cx, |repo, _| repo.change_remote(remote_name))?
                .await??;

            anyhow::Ok(())
        })
        .detach_and_prompt_err("Failed to change remote", window, cx, |_, _, _| None);
        // self.workspace.update(cx, |workspace, cx| {
        //     workspace.project().update(cx, |project, cx| {
        //         let Some(active_repo) = project.active_repository(cx) else {
        //             return;
        //         };
        //         active_repo.update(cx, |repo, cx| {
        //             repo.change_remote(&entry.remote.name);
        //         });
        //     });
        // });

        cx.emit(DismissEvent);
    }

    fn dismissed(&mut self, _: &mut Window, cx: &mut Context<Picker<Self>>) {
        cx.emit(DismissEvent);
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let entry = self.matches.get(ix)?;
        let is_selected_remote = self
            .repo
            .as_ref()
            .and_then(|repo| repo.read(cx).remote.as_ref())
            .map(|remote| remote.name == entry.remote.name)
            .unwrap_or_default();

        let icon = if let Some(default_remote) = self.default_remote.clone()
            && entry.is_new
        {
            Some(
                IconButton::new("remote-from-default", IconName::GitBranchAlt)
                    .on_click(cx.listener(move |this, _, window, cx| {
                        this.delegate.set_selected_index(ix, window, cx);
                        this.delegate.confirm(true, window, cx);
                    }))
                    .tooltip(move |_window, cx| {
                        Tooltip::for_action(
                            "Create remote".to_string(),
                            &menu::SecondaryConfirm,
                            cx,
                        )
                    }),
            )
        } else {
            None
        };

        let remote_name = if entry.is_new {
            h_flex()
                .gap_1()
                .child(
                    Icon::new(IconName::Plus)
                        .size(IconSize::Small)
                        .color(Color::Muted),
                )
                .child(
                    Label::new(format!("Create remote \"{}\"…", entry.remote.name))
                        .single_line()
                        .truncate(),
                )
                .into_any_element()
        } else {
            h_flex()
                .max_w_48()
                .child(
                    HighlightedLabel::new(entry.remote.name.clone(), entry.positions.clone())
                        .truncate(),
                )
                .into_any_element()
        };

        Some(
            ListItem::new(SharedString::from(format!("remote-menu-{ix}")))
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .toggle_state(selected)
                .tooltip({
                    let remote_name = entry.remote.name.to_string();
                    if entry.is_new {
                        Tooltip::text(format!("Create remote \"{}\"", remote_name))
                    } else {
                        Tooltip::text(remote_name)
                    }
                })
                .child(
                    v_flex()
                        .w_full()
                        .overflow_hidden()
                        .child(
                            h_flex()
                                .gap_6()
                                .justify_between()
                                .overflow_x_hidden()
                                .child(remote_name)
                                .when(!entry.is_new && !is_selected_remote, |label| {
                                    let remote_name = entry.remote.name.clone();
                                    label.child(
                                        IconButton::new("remote-remove", IconName::ToolDeleteFile)
                                            .on_click(cx.listener(move |this, _, window, cx| {
                                                this.delegate.remove_remote(
                                                    remote_name.clone(),
                                                    window,
                                                    cx,
                                                );
                                            }))
                                            .tooltip(move |_window, cx| {
                                                Tooltip::for_action(
                                                    "Remove remote".to_string(),
                                                    &menu::SecondaryConfirm,
                                                    cx,
                                                )
                                            }),
                                    )
                                }),
                        )
                        .when(self.style == RemoteListStyle::Modal, |el| {
                            el.child(div().max_w_96().child({
                                let message = if entry.is_new {
                                    "Create remote".to_string()
                                } else {
                                    entry.remote.name.to_string()
                                };
                                Label::new(message)
                                    .size(LabelSize::Small)
                                    .truncate()
                                    .color(Color::Muted)
                            }))
                        }),
                )
                .end_slot::<IconButton>(icon),
        )
    }

    fn no_matches_text(&self, _window: &mut Window, _cx: &mut App) -> Option<SharedString> {
        None
    }
}
