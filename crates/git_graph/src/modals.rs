use super::*;

pub(super) struct CreateBranchAtCommitModal {
    graph: WeakEntity<GitGraph>,
    repository: Entity<Repository>,
    commit_sha: SharedString,
    title: SharedString,
    editor: Entity<Editor>,
    checkout_after_create: bool,
}

impl CreateBranchAtCommitModal {
    pub(super) fn new(
        graph: WeakEntity<GitGraph>,
        repository: Entity<Repository>,
        commit_sha: SharedString,
        initial_name: Option<String>,
        title: SharedString,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            if let Some(initial_name) = initial_name.clone() {
                editor.set_text(initial_name, window, cx);
            } else {
                editor.set_placeholder_text("Enter branch name...", window, cx);
            }
            editor
        });

        Self {
            graph,
            repository,
            commit_sha,
            title,
            editor,
            checkout_after_create: false,
        }
    }

    fn cancel(&mut self, _: &Cancel, _window: &mut Window, cx: &mut Context<Self>) {
        cx.emit(DismissEvent);
    }

    fn confirm(&mut self, _: &Confirm, window: &mut Window, cx: &mut Context<Self>) {
        let branch_name = self.editor.read(cx).text(cx).trim().replace(' ', "-");
        if branch_name.is_empty() {
            return;
        }

        let repository = self.repository.clone();
        let graph = self.graph.clone();
        let commit_sha = self.commit_sha.to_string();
        let checkout_after_create = self.checkout_after_create;

        cx.spawn(async move |_, cx| {
            repository
                .update(cx, |repository, _| {
                    repository.create_branch_at(commit_sha, branch_name.clone())
                })
                .await??;

            if checkout_after_create {
                repository
                    .update(cx, |repository, _| repository.change_branch(branch_name))
                    .await??;
            }

            let _ = graph.update(cx, |graph, cx| {
                graph.reload_graph(cx);
            });

            Ok(())
        })
        .detach_and_prompt_err("Failed to create branch", window, cx, |error, _, _| {
            Some(error.to_string())
        });

        cx.emit(DismissEvent);
    }
}

impl EventEmitter<DismissEvent> for CreateBranchAtCommitModal {}
impl ModalView for CreateBranchAtCommitModal {}
impl Focusable for CreateBranchAtCommitModal {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.editor.focus_handle(cx)
    }
}

impl Render for CreateBranchAtCommitModal {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .key_context("CreateBranchAtCommitModal")
            .on_action(cx.listener(Self::cancel))
            .on_action(cx.listener(Self::confirm))
            .elevation_2(cx)
            .w(ui::rems(34.))
            .child(
                h_flex()
                    .px_3()
                    .pt_2()
                    .pb_1()
                    .gap_1p5()
                    .child(Icon::new(IconName::GitBranch).size(IconSize::XSmall))
                    .child(Label::new(self.title.clone())),
            )
            .child(
                v_flex()
                    .px_3()
                    .pb_3()
                    .w_full()
                    .gap_2()
                    .child(self.editor.clone())
                    .child(
                        Checkbox::new(
                            "create-branch-checkout-after-create",
                            if self.checkout_after_create {
                                ToggleState::Selected
                            } else {
                                ToggleState::Unselected
                            },
                        )
                        .label("Checkout after create")
                        .label_size(LabelSize::Small)
                        .on_click(cx.listener(
                            |this: &mut CreateBranchAtCommitModal, _, _window, cx| {
                                this.checkout_after_create = !this.checkout_after_create;
                                cx.notify();
                            },
                        )),
                    ),
            )
    }
}

pub(super) struct CherryPickModal {
    graph: WeakEntity<GitGraph>,
    repository: Entity<Repository>,
    commit_sha: SharedString,
    record_origin: bool,
    no_commit: bool,
    focus_handle: FocusHandle,
}

impl CherryPickModal {
    pub(super) fn new(
        graph: WeakEntity<GitGraph>,
        repository: Entity<Repository>,
        commit_sha: SharedString,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        Self {
            graph,
            repository,
            commit_sha,
            record_origin: false,
            no_commit: false,
            focus_handle: cx.focus_handle(),
        }
    }

    fn cancel(&mut self, _: &Cancel, _window: &mut Window, cx: &mut Context<Self>) {
        cx.emit(DismissEvent);
    }

    fn confirm(&mut self, _: &Confirm, window: &mut Window, cx: &mut Context<Self>) {
        let repository = self.repository.clone();
        let graph = self.graph.clone();
        let sha = self.commit_sha.to_string();
        let record_origin = self.record_origin;
        let no_commit = self.no_commit;

        cx.spawn(async move |_, cx| {
            repository
                .update(cx, |repository, _| {
                    repository.cherry_pick(sha, record_origin, no_commit)
                })
                .await??;

            let _ = graph.update(cx, |graph, cx| {
                graph.reload_graph(cx);
            });

            Ok(())
        })
        .detach_and_prompt_err(
            "Failed to cherry-pick commit",
            window,
            cx,
            |error, _, _| Some(error.to_string()),
        );

        cx.emit(DismissEvent);
    }
}

impl EventEmitter<DismissEvent> for CherryPickModal {}
impl ModalView for CherryPickModal {}
impl Focusable for CherryPickModal {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for CherryPickModal {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .key_context("CherryPickModal")
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(Self::cancel))
            .on_action(cx.listener(Self::confirm))
            .elevation_2(cx)
            .w(ui::rems(34.))
            .child(
                h_flex()
                    .px_3()
                    .pt_2()
                    .pb_1()
                    .gap_1p5()
                    .child(Icon::new(IconName::GitCommit).size(IconSize::XSmall))
                    .child(Label::new(format!("Cherry Pick {}", self.commit_sha))),
            )
            .child(
                v_flex()
                    .px_3()
                    .pb_2()
                    .gap_1()
                    .child(
                        Checkbox::new(
                            "cherry-pick-record-origin",
                            if self.record_origin {
                                ToggleState::Selected
                            } else {
                                ToggleState::Unselected
                            },
                        )
                        .label("Record origin (-x)")
                        .label_size(LabelSize::Small)
                        .on_click(cx.listener(|this, _, _window, cx| {
                            this.record_origin = !this.record_origin;
                            cx.notify();
                        })),
                    )
                    .child(
                        Checkbox::new(
                            "cherry-pick-no-commit",
                            if self.no_commit {
                                ToggleState::Selected
                            } else {
                                ToggleState::Unselected
                            },
                        )
                        .label("No commit (--no-commit)")
                        .label_size(LabelSize::Small)
                        .on_click(cx.listener(|this, _, _window, cx| {
                            this.no_commit = !this.no_commit;
                            cx.notify();
                        })),
                    ),
            )
            .child(
                h_flex()
                    .px_3()
                    .pb_3()
                    .gap_2()
                    .justify_end()
                    .child(
                        Button::new("cherry-pick-cancel", "Cancel")
                            .style(ButtonStyle::Subtle)
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.cancel(&Cancel, window, cx);
                            })),
                    )
                    .child(
                        Button::new("cherry-pick-confirm", "Cherry Pick")
                            .style(ButtonStyle::Filled)
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.confirm(&Confirm, window, cx);
                            })),
                    ),
            )
    }
}

pub(super) struct AddTagModal {
    graph: WeakEntity<GitGraph>,
    repository: Entity<Repository>,
    commit_sha: SharedString,
    name_editor: Entity<Editor>,
    message_editor: Entity<Editor>,
}

impl AddTagModal {
    pub(super) fn new(
        graph: WeakEntity<GitGraph>,
        repository: Entity<Repository>,
        commit_sha: SharedString,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let name_editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_placeholder_text("Enter tag name...", window, cx);
            editor
        });
        let message_editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_placeholder_text("Optional tag message...", window, cx);
            editor
        });

        Self {
            graph,
            repository,
            commit_sha,
            name_editor,
            message_editor,
        }
    }

    fn cancel(&mut self, _: &Cancel, _window: &mut Window, cx: &mut Context<Self>) {
        cx.emit(DismissEvent);
    }

    fn confirm(&mut self, _: &Confirm, window: &mut Window, cx: &mut Context<Self>) {
        let tag_name = self.name_editor.read(cx).text(cx).trim().to_string();
        if tag_name.is_empty() {
            return;
        }

        let tag_message = self.message_editor.read(cx).text(cx).trim().to_string();
        let repository = self.repository.clone();
        let graph = self.graph.clone();
        let commit_sha = self.commit_sha.to_string();

        cx.spawn(async move |_, cx| {
            repository
                .update(cx, |repository, _| {
                    repository.create_tag(
                        commit_sha,
                        tag_name,
                        (!tag_message.is_empty()).then_some(tag_message),
                    )
                })
                .await??;

            let _ = graph.update(cx, |graph, cx| {
                graph.reload_graph(cx);
            });

            Ok(())
        })
        .detach_and_prompt_err("Failed to add tag", window, cx, |error, _, _| {
            Some(error.to_string())
        });

        cx.emit(DismissEvent);
    }
}

impl EventEmitter<DismissEvent> for AddTagModal {}
impl ModalView for AddTagModal {}
impl Focusable for AddTagModal {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.name_editor.focus_handle(cx)
    }
}

impl Render for AddTagModal {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .key_context("AddTagModal")
            .on_action(cx.listener(Self::cancel))
            .on_action(cx.listener(Self::confirm))
            .elevation_2(cx)
            .w(ui::rems(34.))
            .child(
                h_flex()
                    .px_3()
                    .pt_2()
                    .pb_1()
                    .gap_1p5()
                    .child(Icon::new(IconName::GitCommit).size(IconSize::XSmall))
                    .child(Label::new(format!("Add Tag at {}", self.commit_sha))),
            )
            .child(
                v_flex()
                    .px_3()
                    .pb_3()
                    .w_full()
                    .gap_2()
                    .child(self.name_editor.clone())
                    .child(self.message_editor.clone()),
            )
    }
}

pub(super) struct RenameBranchModal {
    graph: WeakEntity<GitGraph>,
    repository: Entity<Repository>,
    branch_name: SharedString,
    editor: Entity<Editor>,
}

impl RenameBranchModal {
    pub(super) fn new(
        branch_name: String,
        repository: Entity<Repository>,
        graph: WeakEntity<GitGraph>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_text(branch_name.clone(), window, cx);
            editor
        });
        Self {
            graph,
            repository,
            branch_name: branch_name.into(),
            editor,
        }
    }

    fn cancel(&mut self, _: &Cancel, _window: &mut Window, cx: &mut Context<Self>) {
        cx.emit(DismissEvent);
    }

    fn confirm(&mut self, _: &Confirm, window: &mut Window, cx: &mut Context<Self>) {
        let new_name = self.editor.read(cx).text(cx);
        if new_name.is_empty() || new_name == self.branch_name.as_ref() {
            cx.emit(DismissEvent);
            return;
        }

        let repository = self.repository.clone();
        let graph = self.graph.clone();
        let old_name = self.branch_name.to_string();

        cx.spawn(async move |_, cx| {
            repository
                .update(cx, |repository, _| {
                    repository.rename_branch(old_name.clone(), new_name.clone())
                })
                .await??;

            let _ = graph.update(cx, |graph, cx| {
                graph.reload_graph(cx);
            });

            Ok(())
        })
        .detach_and_prompt_err("Failed to rename branch", window, cx, |error, _, _| {
            Some(error.to_string())
        });

        cx.emit(DismissEvent);
    }
}

impl EventEmitter<DismissEvent> for RenameBranchModal {}
impl ModalView for RenameBranchModal {}
impl Focusable for RenameBranchModal {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.editor.focus_handle(cx)
    }
}

impl Render for RenameBranchModal {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .key_context("RenameBranchModal")
            .on_action(cx.listener(Self::cancel))
            .on_action(cx.listener(Self::confirm))
            .elevation_2(cx)
            .w(ui::rems(34.))
            .child(
                h_flex()
                    .px_3()
                    .pt_2()
                    .pb_1()
                    .gap_1p5()
                    .child(Icon::new(IconName::GitBranch).size(IconSize::XSmall))
                    .child(Label::new(format!("Rename Branch ({})", self.branch_name))),
            )
            .child(div().px_3().pb_3().w_full().child(self.editor.clone()))
    }
}

pub(super) struct PushBranchModal {
    graph: WeakEntity<GitGraph>,
    state: PushBranchDialogState,
    focus_handle: FocusHandle,
}

fn render_remote_dropdown<T: 'static>(
    id: &'static str,
    selected_remote: SharedString,
    available_remotes: &[SharedString],
    window: &mut Window,
    cx: &mut Context<T>,
    select_remote: fn(&mut T, SharedString, &mut Context<T>),
) -> DropdownMenu {
    let weak = cx.weak_entity();
    let remotes = available_remotes.to_vec();
    let menu = ContextMenu::build(window, cx, move |mut menu, _, _| {
        for remote_name in remotes.clone() {
            let weak = weak.clone();
            menu = menu.entry(remote_name.clone(), None, move |_window, cx| {
                if let Some(entity) = weak.upgrade() {
                    entity.update(cx, |this, cx| {
                        select_remote(this, remote_name.clone(), cx);
                    });
                }
            });
        }
        menu
    });

    DropdownMenu::new(id, selected_remote, menu)
        .style(DropdownStyle::Outlined)
        .full_width(true)
}

impl PushBranchModal {
    pub(super) fn new(
        graph: WeakEntity<GitGraph>,
        state: PushBranchDialogState,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        Self {
            graph,
            state,
            focus_handle: cx.focus_handle(),
        }
    }

    fn cancel(&mut self, _: &Cancel, _window: &mut Window, cx: &mut Context<Self>) {
        cx.emit(DismissEvent);
    }

    fn confirm(&mut self, _: &Confirm, window: &mut Window, cx: &mut Context<Self>) {
        let target = self.state.push_target();
        if let Some(graph) = self.graph.upgrade() {
            graph.update(cx, |graph, cx| {
                graph.perform_push_branch(target, window, cx);
            });
        }

        cx.emit(DismissEvent);
    }

    fn select_remote(&mut self, remote_name: SharedString, cx: &mut Context<Self>) {
        self.state.select_remote(remote_name);
        cx.notify();
    }

    fn render_remote_dropdown(&self, window: &mut Window, cx: &mut Context<Self>) -> DropdownMenu {
        render_remote_dropdown(
            "push-branch-remote-dropdown",
            self.state.selected_remote.clone(),
            &self.state.available_remotes,
            window,
            cx,
            Self::select_remote,
        )
    }

    fn render_push_mode_option(
        &self,
        id: &'static str,
        label: &'static str,
        push_mode: PushMode,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        Checkbox::new(
            id,
            if self.state.push_mode == push_mode {
                ToggleState::Selected
            } else {
                ToggleState::Unselected
            },
        )
        .label(label)
        .label_size(LabelSize::Small)
        .on_click(
            cx.listener(move |this: &mut PushBranchModal, _, _window, cx| {
                this.state.push_mode = push_mode;
                cx.notify();
            }),
        )
    }
}

impl EventEmitter<DismissEvent> for PushBranchModal {}
impl ModalView for PushBranchModal {}
impl Focusable for PushBranchModal {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for PushBranchModal {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .key_context("PushBranchModal")
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(Self::cancel))
            .on_action(cx.listener(Self::confirm))
            .elevation_2(cx)
            .w(ui::rems(36.))
            .child(
                h_flex()
                    .px_3()
                    .pt_2()
                    .pb_1()
                    .gap_1p5()
                    .child(Icon::new(IconName::GitBranch).size(IconSize::XSmall))
                    .child(Label::new(format!(
                        "Push Branch ({})",
                        self.state.branch.name()
                    ))),
            )
            .child(
                v_flex()
                    .px_3()
                    .pb_3()
                    .w_full()
                    .gap_3()
                    .child(
                        v_flex()
                            .gap_1()
                            .child(Label::new("Push to Remote(s):").size(LabelSize::Small))
                            .child(self.render_remote_dropdown(window, cx)),
                    )
                    .child(
                        Checkbox::new(
                            "push-branch-set-upstream",
                            if self.state.set_upstream {
                                ToggleState::Selected
                            } else {
                                ToggleState::Unselected
                            },
                        )
                        .label("Set Upstream")
                        .label_size(LabelSize::Small)
                        .on_click(cx.listener(
                            |this: &mut PushBranchModal, _, _window, cx| {
                                this.state.set_upstream = !this.state.set_upstream;
                                cx.notify();
                            },
                        )),
                    )
                    .child(
                        v_flex()
                            .gap_1()
                            .child(Label::new("Push Mode:").size(LabelSize::Small))
                            .child(
                                v_flex()
                                    .gap_1()
                                    .child(self.render_push_mode_option(
                                        "push-branch-mode-normal",
                                        "Normal",
                                        PushMode::Normal,
                                        cx,
                                    ))
                                    .child(self.render_push_mode_option(
                                        "push-branch-mode-force-with-lease",
                                        "Force With Lease",
                                        PushMode::ForceWithLease,
                                        cx,
                                    ))
                                    .child(self.render_push_mode_option(
                                        "push-branch-mode-force",
                                        "Force",
                                        PushMode::Force,
                                        cx,
                                    )),
                            ),
                    )
                    .child(
                        h_flex()
                            .justify_end()
                            .gap_2()
                            .child(
                                Button::new("push-branch-cancel", "Cancel")
                                    .style(ButtonStyle::Subtle)
                                    .on_click(cx.listener(
                                        |this: &mut PushBranchModal, _, window, cx| {
                                            this.cancel(&Cancel, window, cx);
                                        },
                                    )),
                            )
                            .child(
                                Button::new("push-branch-confirm", "Push")
                                    .style(ButtonStyle::Filled)
                                    .on_click(cx.listener(
                                        |this: &mut PushBranchModal, _, window, cx| {
                                            this.confirm(&Confirm, window, cx);
                                        },
                                    )),
                            ),
                    ),
            )
    }
}

pub(super) struct PushTagModal {
    graph: WeakEntity<GitGraph>,
    state: PushTagDialogState,
    focus_handle: FocusHandle,
}

impl PushTagModal {
    pub(super) fn new(
        graph: WeakEntity<GitGraph>,
        state: PushTagDialogState,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        Self {
            graph,
            state,
            focus_handle: cx.focus_handle(),
        }
    }

    fn cancel(&mut self, _: &Cancel, _window: &mut Window, cx: &mut Context<Self>) {
        cx.emit(DismissEvent);
    }

    fn confirm(&mut self, _: &Confirm, window: &mut Window, cx: &mut Context<Self>) {
        let target = self.state.push_target();
        if let Some(graph) = self.graph.upgrade() {
            graph.update(cx, |graph, cx| {
                graph.perform_push_tag(target, window, cx);
            });
        }

        cx.emit(DismissEvent);
    }

    fn select_remote(&mut self, remote_name: SharedString, cx: &mut Context<Self>) {
        self.state.select_remote(remote_name);
        cx.notify();
    }

    fn render_remote_dropdown(&self, window: &mut Window, cx: &mut Context<Self>) -> DropdownMenu {
        render_remote_dropdown(
            "push-tag-remote-dropdown",
            self.state.selected_remote.clone(),
            &self.state.available_remotes,
            window,
            cx,
            Self::select_remote,
        )
    }
}

impl EventEmitter<DismissEvent> for PushTagModal {}
impl ModalView for PushTagModal {}
impl Focusable for PushTagModal {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for PushTagModal {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .key_context("PushTagModal")
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(Self::cancel))
            .on_action(cx.listener(Self::confirm))
            .elevation_2(cx)
            .w(ui::rems(34.))
            .child(
                h_flex()
                    .px_3()
                    .pt_2()
                    .pb_1()
                    .gap_1p5()
                    .child(Icon::new(IconName::GitCommit).size(IconSize::XSmall))
                    .child(Label::new(format!("Push Tag ({})", self.state.tag_name))),
            )
            .child(
                v_flex()
                    .px_3()
                    .pb_3()
                    .w_full()
                    .gap_3()
                    .child(
                        v_flex()
                            .gap_1()
                            .child(Label::new("Push to Remote:").size(LabelSize::Small))
                            .child(self.render_remote_dropdown(window, cx)),
                    )
                    .child(
                        h_flex()
                            .justify_end()
                            .gap_2()
                            .child(
                                Button::new("push-tag-cancel", "Cancel")
                                    .style(ButtonStyle::Subtle)
                                    .on_click(cx.listener(
                                        |this: &mut PushTagModal, _, window, cx| {
                                            this.cancel(&Cancel, window, cx);
                                        },
                                    )),
                            )
                            .child(
                                Button::new("push-tag-confirm", "Push")
                                    .style(ButtonStyle::Filled)
                                    .on_click(cx.listener(
                                        |this: &mut PushTagModal, _, window, cx| {
                                            this.confirm(&Confirm, window, cx);
                                        },
                                    )),
                            ),
                    ),
            )
    }
}

pub(super) struct DeleteBranchModal {
    graph: WeakEntity<GitGraph>,
    branch_name: SharedString,
    is_remote: bool,
    force_delete: bool,
    focus_handle: FocusHandle,
}

impl DeleteBranchModal {
    pub(super) fn new(
        graph: WeakEntity<GitGraph>,
        branch_name: String,
        is_remote: bool,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        Self {
            graph,
            branch_name: branch_name.into(),
            is_remote,
            force_delete: false,
            focus_handle: cx.focus_handle(),
        }
    }

    fn cancel(&mut self, _: &Cancel, _window: &mut Window, cx: &mut Context<Self>) {
        cx.emit(DismissEvent);
    }

    fn confirm(&mut self, _: &Confirm, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(graph) = self.graph.upgrade() {
            let branch_name = self.branch_name.to_string();
            let is_remote = self.is_remote;
            let force_delete = self.force_delete;
            graph.update(cx, |graph, cx| {
                graph.perform_delete_branch(branch_name, is_remote, force_delete, window, cx);
            });
        }

        cx.emit(DismissEvent);
    }
}

impl EventEmitter<DismissEvent> for DeleteBranchModal {}
impl ModalView for DeleteBranchModal {}
impl Focusable for DeleteBranchModal {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for DeleteBranchModal {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .key_context("DeleteBranchModal")
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(Self::cancel))
            .on_action(cx.listener(Self::confirm))
            .elevation_2(cx)
            .w(ui::rems(34.))
            .child(
                h_flex()
                    .px_3()
                    .pt_2()
                    .pb_1()
                    .gap_1p5()
                    .child(Icon::new(IconName::Trash).size(IconSize::XSmall))
                    .child(Label::new(if self.is_remote {
                        format!("Delete Remote-Tracking Branch ({})", self.branch_name)
                    } else {
                        format!("Delete Branch ({})", self.branch_name)
                    })),
            )
            .child(
                v_flex()
                    .px_3()
                    .pb_3()
                    .w_full()
                    .gap_2()
                    .child(Label::new("This cannot be undone."))
                    .when(!self.is_remote, |this| {
                        this.child(
                            Checkbox::new(
                                "delete-branch-force-delete",
                                if self.force_delete {
                                    ToggleState::Selected
                                } else {
                                    ToggleState::Unselected
                                },
                            )
                            .label("Force delete")
                            .label_size(LabelSize::Small)
                            .on_click(cx.listener(
                                |this: &mut DeleteBranchModal, _, _window, cx| {
                                    this.force_delete = !this.force_delete;
                                    cx.notify();
                                },
                            )),
                        )
                    })
                    .child(
                        h_flex()
                            .justify_end()
                            .gap_2()
                            .child(
                                Button::new("delete-branch-cancel", "Cancel")
                                    .style(ButtonStyle::Subtle)
                                    .on_click(cx.listener(
                                        |this: &mut DeleteBranchModal, _, window, cx| {
                                            this.cancel(&Cancel, window, cx);
                                        },
                                    )),
                            )
                            .child(
                                Button::new("delete-branch-confirm", "Delete")
                                    .style(ButtonStyle::Filled)
                                    .on_click(cx.listener(
                                        |this: &mut DeleteBranchModal, _, window, cx| {
                                            this.confirm(&Confirm, window, cx);
                                        },
                                    )),
                            ),
                    ),
            )
    }
}

pub(super) struct RevertCommitModal {
    graph: WeakEntity<GitGraph>,
    repository: Entity<Repository>,
    commit_sha: SharedString,
    no_commit: bool,
    focus_handle: FocusHandle,
}

impl RevertCommitModal {
    pub(super) fn new(
        graph: WeakEntity<GitGraph>,
        repository: Entity<Repository>,
        commit_sha: SharedString,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        Self {
            graph,
            repository,
            commit_sha,
            no_commit: false,
            focus_handle: cx.focus_handle(),
        }
    }

    fn cancel(&mut self, _: &Cancel, _window: &mut Window, cx: &mut Context<Self>) {
        cx.emit(DismissEvent);
    }

    fn confirm(&mut self, _: &Confirm, window: &mut Window, cx: &mut Context<Self>) {
        let repository = self.repository.clone();
        let graph = self.graph.clone();
        let sha = self.commit_sha.to_string();
        let no_commit = self.no_commit;

        cx.spawn(async move |_, cx| {
            repository
                .update(cx, |repository, _| repository.revert_commit(sha, no_commit))
                .await??;

            let _ = graph.update(cx, |graph, cx| {
                graph.reload_graph(cx);
            });

            Ok(())
        })
        .detach_and_prompt_err("Failed to revert commit", window, cx, |error, _, _| {
            Some(error.to_string())
        });

        cx.emit(DismissEvent);
    }
}

impl EventEmitter<DismissEvent> for RevertCommitModal {}
impl ModalView for RevertCommitModal {}
impl Focusable for RevertCommitModal {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for RevertCommitModal {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .key_context("RevertCommitModal")
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(Self::cancel))
            .on_action(cx.listener(Self::confirm))
            .elevation_2(cx)
            .w(ui::rems(34.))
            .child(
                h_flex()
                    .px_3()
                    .pt_2()
                    .pb_1()
                    .gap_1p5()
                    .child(Icon::new(IconName::GitCommit).size(IconSize::XSmall))
                    .child(Label::new(format!("Revert Commit {}", self.commit_sha))),
            )
            .child(
                v_flex()
                    .px_3()
                    .pb_3()
                    .w_full()
                    .gap_2()
                    .child(
                        Checkbox::new(
                            "revert-commit-no-commit",
                            if self.no_commit {
                                ToggleState::Selected
                            } else {
                                ToggleState::Unselected
                            },
                        )
                        .label("Do not commit (--no-commit)")
                        .label_size(LabelSize::Small)
                        .on_click(cx.listener(
                            |this: &mut RevertCommitModal, _, _window, cx| {
                                this.no_commit = !this.no_commit;
                                cx.notify();
                            },
                        )),
                    )
                    .child(
                        h_flex()
                            .justify_end()
                            .gap_2()
                            .child(
                                Button::new("revert-commit-cancel", "Cancel")
                                    .style(ButtonStyle::Subtle)
                                    .on_click(cx.listener(
                                        |this: &mut RevertCommitModal, _, window, cx| {
                                            this.cancel(&Cancel, window, cx);
                                        },
                                    )),
                            )
                            .child(
                                Button::new("revert-commit-confirm", "Revert")
                                    .style(ButtonStyle::Filled)
                                    .on_click(cx.listener(
                                        |this: &mut RevertCommitModal, _, window, cx| {
                                            this.confirm(&Confirm, window, cx);
                                        },
                                    )),
                            ),
                    ),
            )
    }
}

pub(super) struct GitGraphAskPassModal {
    operation: SharedString,
    prompt: SharedString,
    editor: Entity<Editor>,
    tx: Option<oneshot::Sender<EncryptedPassword>>,
}

impl GitGraphAskPassModal {
    pub(super) fn new(
        operation: SharedString,
        prompt: SharedString,
        tx: oneshot::Sender<EncryptedPassword>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            if prompt.contains("yes/no") || prompt.contains("Username") {
                editor.set_masked(false, cx);
            } else {
                editor.set_masked(true, cx);
            }
            editor
        });

        Self {
            operation,
            prompt,
            editor,
            tx: Some(tx),
        }
    }

    fn cancel(&mut self, _: &Cancel, _window: &mut Window, cx: &mut Context<Self>) {
        cx.emit(DismissEvent);
    }

    fn confirm(&mut self, _: &Confirm, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(tx) = self.tx.take() {
            let mut text = self.editor.update(cx, |editor, cx| {
                let text = editor.text(cx);
                editor.clear(window, cx);
                text
            });
            if let Ok(password) = EncryptedPassword::try_from(text.as_ref()) {
                tx.send(password).ok();
            }
            text.zeroize();
        }

        cx.emit(DismissEvent);
    }
}

impl EventEmitter<DismissEvent> for GitGraphAskPassModal {}
impl ModalView for GitGraphAskPassModal {}
impl Focusable for GitGraphAskPassModal {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.editor.focus_handle(cx)
    }
}

impl Render for GitGraphAskPassModal {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .key_context("GitGraphAskPassModal")
            .on_action(cx.listener(Self::cancel))
            .on_action(cx.listener(Self::confirm))
            .elevation_2(cx)
            .w(ui::rems(34.))
            .child(
                h_flex()
                    .px_3()
                    .pt_2()
                    .pb_1()
                    .gap_1p5()
                    .child(Icon::new(IconName::GitBranch).size(IconSize::XSmall))
                    .child(Label::new(self.operation.clone())),
            )
            .child(
                v_flex()
                    .px_3()
                    .pb_3()
                    .w_full()
                    .gap_2()
                    .child(Label::new(self.prompt.clone()))
                    .child(self.editor.clone()),
            )
    }
}
