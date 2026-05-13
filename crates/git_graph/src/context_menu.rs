use super::*;

#[derive(Clone)]
pub(super) struct SelectedCommitInfo {
    pub(super) index: usize,
    pub(super) sha: SharedString,
    pub(super) subject: Option<SharedString>,
}

#[derive(Clone, Debug)]
pub(super) enum RefNameKind {
    Branch(SharedString),
    Tag(SharedString),
    Stash(SharedString),
}

impl RefNameKind {
    pub(super) fn classify(ref_name: &SharedString) -> Self {
        let name = ref_name.as_ref();
        if name == "refs/stash"
            || name == "stash"
            || name.starts_with("stash@{")
            || name.contains("refs/stash")
        {
            Self::Stash(ref_name.clone())
        } else if name.starts_with("tag: ") || name.starts_with("refs/tags/") {
            Self::Tag(ref_name.clone())
        } else {
            Self::Branch(ref_name.clone())
        }
    }

    pub(super) fn display_name(&self) -> SharedString {
        match self {
            Self::Branch(name) => {
                let name = name.as_ref();
                name.strip_prefix("HEAD -> ")
                    .unwrap_or(name)
                    .to_string()
                    .into()
            }
            Self::Tag(name) => {
                let name = name.as_ref();
                name.strip_prefix("tag: ")
                    .or_else(|| name.strip_prefix("refs/tags/"))
                    .unwrap_or(name)
                    .to_string()
                    .into()
            }
            Self::Stash(name) => name.clone(),
        }
    }

    pub(super) fn branch_lookup_name(&self) -> Option<SharedString> {
        match self {
            Self::Branch(name) => {
                let name = name.as_ref();
                Some(
                    name.strip_prefix("HEAD -> ")
                        .unwrap_or(name)
                        .to_string()
                        .into(),
                )
            }
            _ => None,
        }
    }

    pub(super) fn stash_index(&self) -> Option<usize> {
        match self {
            Self::Stash(name) => {
                let name = name.as_ref();
                if let Some(start) = name.find("stash@{") {
                    let rest = &name[start + 7..];
                    rest.strip_suffix('}')?.parse::<usize>().ok()
                } else {
                    Some(0)
                }
            }
            _ => None,
        }
    }
}

#[derive(Clone)]
pub(super) struct CommitContextMenuState {
    pub(super) row_index: usize,
}

#[derive(Clone, Copy)]
pub(super) enum ResetPromptMode {
    Soft,
    Mixed,
    Hard,
}

impl ResetPromptMode {
    pub(super) const ALL: [Self; 3] = [Self::Soft, Self::Mixed, Self::Hard];

    pub(super) fn to_reset_mode(self) -> ResetMode {
        match self {
            Self::Soft => ResetMode::Soft,
            Self::Mixed => ResetMode::Mixed,
            Self::Hard => ResetMode::Hard,
        }
    }

    pub(super) fn label(self) -> &'static str {
        match self {
            Self::Soft => "Soft",
            Self::Mixed => "Mixed",
            Self::Hard => "Hard",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct BranchPushTarget {
    pub(super) branch: Branch,
    pub(super) remote: Remote,
    pub(super) remote_branch_name: SharedString,
    pub(super) options: Option<PushOptions>,
}

#[derive(Clone, Debug)]
pub(super) struct PushBranchDialogState {
    pub(super) branch: Branch,
    pub(super) available_remotes: Vec<SharedString>,
    pub(super) selected_remote: SharedString,
    pub(super) set_upstream: bool,
    pub(super) push_mode: PushMode,
}

impl PushBranchDialogState {
    pub(super) fn new(
        branch: Branch,
        available_remotes: Vec<SharedString>,
    ) -> anyhow::Result<Self> {
        let selected_remote = Self::default_remote_name(&branch, &available_remotes)?;
        let set_upstream = Self::default_set_upstream(&branch, selected_remote.as_ref());

        Ok(Self {
            branch,
            available_remotes,
            selected_remote,
            set_upstream,
            push_mode: PushMode::Normal,
        })
    }

    fn default_remote_name(
        branch: &Branch,
        available_remotes: &[SharedString],
    ) -> anyhow::Result<SharedString> {
        if let Some(remote_name) = Self::tracked_upstream_remote_name(branch)
            && let Some(remote) = available_remotes
                .iter()
                .find(|remote| remote.as_ref() == remote_name)
        {
            return Ok(remote.clone());
        }

        available_remotes
            .first()
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("No remote configured for repository"))
    }

    fn tracked_upstream_remote_name(branch: &Branch) -> Option<&str> {
        branch
            .upstream
            .as_ref()
            .filter(|upstream| matches!(upstream.tracking, UpstreamTracking::Tracked(_)))
            .and_then(|upstream| upstream.remote_name())
    }

    fn tracked_upstream_branch_name(branch: &Branch) -> Option<&str> {
        branch
            .upstream
            .as_ref()
            .filter(|upstream| matches!(upstream.tracking, UpstreamTracking::Tracked(_)))
            .and_then(|upstream| upstream.branch_name())
    }

    fn default_set_upstream(branch: &Branch, selected_remote: &str) -> bool {
        Self::tracked_upstream_remote_name(branch) != Some(selected_remote)
    }

    pub(super) fn select_remote(&mut self, remote_name: SharedString) {
        self.selected_remote = remote_name;
        self.set_upstream = Self::default_set_upstream(&self.branch, self.selected_remote.as_ref());
    }

    pub(super) fn push_target(&self) -> BranchPushTarget {
        let remote_branch_name = if Self::tracked_upstream_remote_name(&self.branch)
            == Some(self.selected_remote.as_ref())
        {
            Self::tracked_upstream_branch_name(&self.branch)
                .unwrap_or_else(|| self.branch.name())
                .to_string()
                .into()
        } else {
            self.branch.name().to_string().into()
        };

        let options = match (self.set_upstream, self.push_mode) {
            (false, PushMode::Normal) => None,
            (set_upstream, push_mode) => Some(PushOptions {
                set_upstream,
                push_mode,
            }),
        };

        BranchPushTarget {
            branch: self.branch.clone(),
            remote: Remote {
                name: self.selected_remote.clone(),
            },
            remote_branch_name,
            options,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct TagPushTarget {
    pub(super) tag_name: SharedString,
    pub(super) remote: Remote,
}

#[derive(Clone, Debug)]
pub(super) struct PushTagDialogState {
    pub(super) tag_name: SharedString,
    pub(super) available_remotes: Vec<SharedString>,
    pub(super) selected_remote: SharedString,
}

impl PushTagDialogState {
    pub(super) fn new(
        tag_name: SharedString,
        available_remotes: Vec<SharedString>,
    ) -> anyhow::Result<Self> {
        let selected_remote = available_remotes
            .first()
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("No remote configured for repository"))?;

        Ok(Self {
            tag_name,
            available_remotes,
            selected_remote,
        })
    }

    pub(super) fn select_remote(&mut self, remote_name: SharedString) {
        self.selected_remote = remote_name;
    }

    pub(super) fn push_target(&self) -> TagPushTarget {
        TagPushTarget {
            tag_name: self.tag_name.clone(),
            remote: Remote {
                name: self.selected_remote.clone(),
            },
        }
    }
}

fn update_git_graph(
    weak: &WeakEntity<GitGraph>,
    window: &mut Window,
    cx: &mut App,
    update: impl FnOnce(&mut GitGraph, &mut Window, &mut Context<GitGraph>),
) {
    if let Some(entity) = weak.upgrade() {
        entity.update(cx, |this, cx| update(this, window, cx));
    }
}

fn write_text_to_clipboard(text: SharedString, cx: &mut App) {
    cx.write_to_clipboard(ClipboardItem::new_string(text.to_string()));
}

impl GitGraph {
    fn git_task_context(&self, commit_sha: Oid, cx: &App) -> Option<TaskContext> {
        let repository_path = self
            .get_repository(cx)?
            .read(cx)
            .work_directory_abs_path
            .to_path_buf();

        let repository_name = repository_path
            .file_name()
            .and_then(|name| name.to_str())
            .map(ToString::to_string);

        let mut task_variables = TaskVariables::from_iter([
            (VariableName::GitSha, commit_sha.to_string()),
            (VariableName::GitShaShort, commit_sha.display_short()),
            (
                VariableName::GitRepositoryPath,
                repository_path.to_string_lossy().into_owned(),
            ),
        ]);

        if let Some(repository_name) = repository_name {
            task_variables.insert(VariableName::GitRepositoryName, repository_name);
        }

        Some(TaskContext {
            cwd: Some(repository_path),
            task_variables,
            ..TaskContext::default()
        })
    }

    fn git_context_menu_tasks(
        &self,
        task_context: &TaskContext,
        cx: &App,
    ) -> Vec<(TaskSourceKind, ResolvedTask)> {
        let Some(workspace) = self.workspace.upgrade() else {
            return Vec::new();
        };

        let project = workspace.read(cx).project().clone();

        let task_inventory = project.read_with(cx, |project, cx| {
            project.task_store().read(cx).task_inventory().cloned()
        });

        let Some(task_inventory) = task_inventory else {
            return Vec::new();
        };

        task_inventory
            .read(cx)
            .resolve_global_tasks_with_tag(GIT_COMMAND_TASK_TAG, task_context)
    }

    fn schedule_git_task(
        &mut self,
        task_source_kind: TaskSourceKind,
        resolved_task: ResolvedTask,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.workspace
            .update(cx, |workspace, cx| {
                workspace.schedule_resolved_task(
                    task_source_kind,
                    resolved_task,
                    false,
                    window,
                    cx,
                );
            })
            .ok();
    }

    pub(super) fn deploy_entry_context_menu(
        &mut self,
        position: Point<Pixels>,
        entry_idx: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.commit_context_menu_state = Some(CommitContextMenuState {
            row_index: entry_idx,
        });
        if let Some(context_menu) = self.build_commit_context_menu(entry_idx, window, cx) {
            self.set_context_menu(context_menu, position, entry_idx, window, cx);
        }
    }

    fn build_commit_context_menu(
        &self,
        entry_idx: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<Entity<ContextMenu>> {
        let selected_commit = self.commit_info_for_entry(entry_idx, cx)?;
        let context_state = self.commit_context_menu_state.as_ref()?;
        if context_state.row_index != selected_commit.index {
            return None;
        }

        let copy_subject_disabled = selected_commit.subject.is_none();
        let commit = self.graph_data.commits.get(entry_idx)?;
        let sha = commit.data.sha;
        let tag_names = commit.data.tag_names();
        let copy_tag_label = "Copy Tag";
        let copy_tag_label: SharedString = match tag_names.as_slice() {
            [] => copy_tag_label.into(),
            [tag_name] => format!("{copy_tag_label}: {tag_name}").into(),
            _ => format!("{copy_tag_label}...").into(),
        };
        let copy_tag_disabled = tag_names.is_empty();
        let git_tasks = self
            .git_task_context(sha, cx)
            .map(|task_context| self.git_context_menu_tasks(&task_context, cx))
            .unwrap_or_default();

        let focus_handle = self.focus_handle.clone();
        let git_graph = cx.entity();

        Some(ContextMenu::build(
            window,
            cx,
            move |context_menu, window, _| {
                context_menu
                    .context(focus_handle)
                    .header(format!("Commit {}", selected_commit.sha))
                    .entry(
                        "View Commit",
                        Some(OpenCommitView.boxed_clone()),
                        window.handler_for(&git_graph, move |this, window, cx| {
                            this.open_commit_view(entry_idx, window, cx);
                        }),
                    )
                    .separator()
                    .action("Create Tag...", AddTag.boxed_clone())
                    .action("Create Branch...", CreateBranchAtCommit.boxed_clone())
                    .separator()
                    .action("Checkout Commit...", CheckoutCommit.boxed_clone())
                    .action("Cherry-Pick Commit...", CherryPickCommit.boxed_clone())
                    .action("Revert Commit...", RevertCommit.boxed_clone())
                    .action("Drop Commit...", DropCommit.boxed_clone())
                    .action(
                        "Merge Commit into Current Branch...",
                        MergeCommit.boxed_clone(),
                    )
                    .action(
                        "Rebase Current Branch onto Commit...",
                        RebaseOntoCommit.boxed_clone(),
                    )
                    .action(
                        "Reset Current Branch to This Commit...",
                        ResetCommit.boxed_clone(),
                    )
                    .separator()
                    .action("Copy Commit Hash", CopyCommitHash.boxed_clone())
                    .item(
                        ContextMenuEntry::new(copy_tag_label)
                            .action(CopyCommitTag.boxed_clone())
                            .disabled(copy_tag_disabled)
                            .handler(window.handler_for(&git_graph, move |this, window, cx| {
                                this.copy_commit_tag(entry_idx, window, cx);
                            })),
                    )
                    .action_disabled_when(
                        copy_subject_disabled,
                        "Copy Commit Subject",
                        CopyCommitSubject.boxed_clone(),
                    )
                    .when(!git_tasks.is_empty(), |mut menu| {
                        menu = menu.separator().header("Custom Git Commands");

                        for (task_source_kind, resolved_task) in git_tasks {
                            let label = resolved_task.display_label().to_string();

                            menu = menu.entry(
                                label,
                                None,
                                window.handler_for(&git_graph, move |this, window, cx| {
                                    this.schedule_git_task(
                                        task_source_kind.clone(),
                                        resolved_task.clone(),
                                        window,
                                        cx,
                                    );
                                }),
                            );
                        }

                        menu
                    })
            },
        ))
    }

    pub(super) fn set_context_menu(
        &mut self,
        context_menu: Entity<ContextMenu>,
        position: Point<Pixels>,
        entry_idx: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        window.focus(&context_menu.focus_handle(cx), cx);

        let subscription = cx.subscribe_in(
            &context_menu,
            window,
            |this, _, _: &DismissEvent, window, cx| {
                if this.context_menu.as_ref().is_some_and(|context_menu| {
                    context_menu
                        .menu
                        .focus_handle(cx)
                        .contains_focused(window, cx)
                }) {
                    cx.focus_self(window);
                }
                this.context_menu.take();
                this.commit_context_menu_state = None;
                cx.notify();
            },
        );
        self.context_menu = Some(GitGraphContextMenu {
            menu: context_menu,
            position,
            entry_idx,
            _subscription: subscription,
        });
        cx.notify();
    }

    pub(super) fn deploy_ref_context_menu(
        &mut self,
        position: Point<Pixels>,
        row_index: usize,
        ref_kind: RefNameKind,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.commit_context_menu_state = None;
        match &ref_kind {
            RefNameKind::Branch(_) => {
                self.deploy_branch_context_menu(position, row_index, ref_kind, window, cx);
            }
            RefNameKind::Tag(_) => {
                if let Some(context_menu) = self.build_tag_context_menu(&ref_kind, window, cx) {
                    self.set_context_menu(context_menu, position, row_index, window, cx);
                }
            }
            RefNameKind::Stash(_) => {
                if let Some(context_menu) = self.build_stash_context_menu(&ref_kind, window, cx) {
                    self.set_context_menu(context_menu, position, row_index, window, cx);
                }
            }
        }
    }

    fn deploy_branch_context_menu(
        &mut self,
        position: Point<Pixels>,
        row_index: usize,
        ref_kind: RefNameKind,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(context_menu) = self.build_branch_context_menu(ref_kind, window, cx) {
            self.set_context_menu(context_menu, position, row_index, window, cx);
        }
    }

    fn build_branch_context_menu(
        &self,
        ref_kind: RefNameKind,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<Entity<ContextMenu>> {
        let branch_name = ref_kind
            .branch_lookup_name()
            .unwrap_or_else(|| ref_kind.display_name());
        let branch = self.resolve_branch_from_snapshot(&ref_kind, cx);
        let focus_handle = self.focus_handle.clone();
        let weak = cx.weak_entity();
        let is_remote = branch.as_ref().is_some_and(Branch::is_remote);
        let is_cached_branch = branch.is_some();

        Some(ContextMenu::build(window, cx, {
            let branch_name_for_checkout = branch_name.clone();
            let branch_name_for_copy = branch_name.clone();
            let branch_name_for_rename = branch_name.clone();
            let branch_name_for_delete = branch_name.clone();
            let branch_name_for_push = branch_name;
            move |context_menu, _, _| {
                let context_menu =
                    context_menu
                        .context(focus_handle)
                        .entry("Checkout Branch", None, {
                            let branch_name = branch_name_for_checkout.clone();
                            let weak = weak.clone();
                            move |window, cx| {
                                update_git_graph(&weak, window, cx, |this, window, cx| {
                                    this.checkout_branch(branch_name.to_string(), window, cx);
                                });
                            }
                        });

                let context_menu = if is_remote || !is_cached_branch {
                    context_menu
                } else {
                    context_menu.entry("Rename Branch...", None, {
                        let branch_name = branch_name_for_rename.clone();
                        let weak = weak.clone();
                        move |window, cx| {
                            update_git_graph(&weak, window, cx, |this, window, cx| {
                                this.rename_branch(branch_name.to_string(), window, cx);
                            });
                        }
                    })
                };

                let context_menu = if is_cached_branch {
                    context_menu.entry(
                        if is_remote {
                            "Delete Remote-Tracking Branch..."
                        } else {
                            "Delete Branch..."
                        },
                        None,
                        {
                            let branch_name = branch_name_for_delete.clone();
                            let weak = weak.clone();
                            move |window, cx| {
                                update_git_graph(&weak, window, cx, |this, window, cx| {
                                    this.delete_branch(
                                        branch_name.to_string(),
                                        is_remote,
                                        window,
                                        cx,
                                    );
                                });
                            }
                        },
                    )
                } else {
                    context_menu
                };

                let context_menu = if is_remote || !is_cached_branch {
                    context_menu
                } else {
                    context_menu.entry("Push Branch...", None, {
                        let branch_name = branch_name_for_push;
                        let weak = weak.clone();
                        move |window, cx| {
                            update_git_graph(&weak, window, cx, |this, window, cx| {
                                this.push_branch(branch_name.to_string(), window, cx);
                            });
                        }
                    })
                };

                context_menu
                    .separator()
                    .entry("Merge Branch into Current Branch...", None, {
                        let weak = weak.clone();
                        move |window, cx| {
                            update_git_graph(&weak, window, cx, |this, window, cx| {
                                this.merge_context_menu_commit(window, cx);
                            });
                        }
                    })
                    .entry("Rebase Current Branch onto Branch...", None, {
                        let weak = weak.clone();
                        move |window, cx| {
                            update_git_graph(&weak, window, cx, |this, window, cx| {
                                this.rebase_context_menu_commit(window, cx);
                            });
                        }
                    })
                    .separator()
                    .action("Copy Branch HEAD Hash", CopyCommitHash.boxed_clone())
                    .entry("Copy Branch Name", None, {
                        let name = branch_name_for_copy;
                        move |_window, cx| {
                            write_text_to_clipboard(name.clone(), cx);
                        }
                    })
            }
        }))
    }

    fn build_tag_context_menu(
        &self,
        ref_kind: &RefNameKind,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<Entity<ContextMenu>> {
        let tag_name = ref_kind.display_name();
        let focus_handle = self.focus_handle.clone();
        let weak = cx.weak_entity();

        Some(ContextMenu::build(window, cx, {
            let tag_name_for_delete = tag_name.clone();
            let tag_name_for_copy = tag_name.clone();
            let tag_name_for_push = tag_name;
            move |context_menu, _, _| {
                context_menu
                    .context(focus_handle)
                    .action("Checkout Tag...", CheckoutCommit.boxed_clone())
                    .separator()
                    .entry("Delete Tag...", None, {
                        let tag_name = tag_name_for_delete.clone();
                        let weak = weak.clone();
                        move |window, cx| {
                            update_git_graph(&weak, window, cx, |this, window, cx| {
                                this.delete_tag(tag_name.to_string(), window, cx);
                            });
                        }
                    })
                    .entry("Push Tag", None, {
                        let tag_name = tag_name_for_push;
                        let weak = weak.clone();
                        move |window, cx| {
                            update_git_graph(&weak, window, cx, |this, window, cx| {
                                this.push_tag(tag_name.to_string(), window, cx);
                            });
                        }
                    })
                    .entry("Create Branch from Tag...", None, {
                        let weak = weak.clone();
                        move |window, cx| {
                            update_git_graph(&weak, window, cx, |this, window, cx| {
                                this.show_create_branch_from_tag_modal(window, cx);
                            });
                        }
                    })
                    .separator()
                    .action("Copy Tagged Commit Hash", CopyCommitHash.boxed_clone())
                    .entry("Copy Tag Name", None, {
                        let name = tag_name_for_copy;
                        move |_window, cx| {
                            write_text_to_clipboard(name.clone(), cx);
                        }
                    })
            }
        }))
    }

    fn build_stash_context_menu(
        &self,
        ref_kind: &RefNameKind,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<Entity<ContextMenu>> {
        let stash_name = ref_kind.display_name();
        let stash_index = ref_kind.stash_index();
        let focus_handle = self.focus_handle.clone();
        let weak = cx.weak_entity();

        Some(ContextMenu::build(window, cx, {
            let stash_name_for_copy = stash_name.clone();
            let stash_name_for_branch = stash_name;
            move |context_menu, _, _| {
                context_menu
                    .context(focus_handle)
                    .entry("Apply Stash", None, {
                        let weak = weak.clone();
                        move |window, cx| {
                            update_git_graph(&weak, window, cx, |this, window, cx| {
                                this.apply_stash(stash_index, window, cx);
                            });
                        }
                    })
                    .entry("Pop Stash...", None, {
                        let weak = weak.clone();
                        move |window, cx| {
                            update_git_graph(&weak, window, cx, |this, window, cx| {
                                this.pop_stash(stash_index, window, cx);
                            });
                        }
                    })
                    .entry("Drop Stash...", None, {
                        let weak = weak.clone();
                        move |window, cx| {
                            update_git_graph(&weak, window, cx, |this, window, cx| {
                                this.drop_stash(stash_index, window, cx);
                            });
                        }
                    })
                    .separator()
                    .entry("Create Branch from Stash...", None, {
                        let weak = weak.clone();
                        move |window, cx| {
                            let stash_name = stash_name_for_branch.to_string();
                            update_git_graph(&weak, window, cx, |this, window, cx| {
                                this.show_create_branch_from_stash_modal(stash_name, window, cx);
                            });
                        }
                    })
                    .separator()
                    .action("Copy Stash Commit Hash", CopyCommitHash.boxed_clone())
                    .entry("Copy Stash Name", None, {
                        let name = stash_name_for_copy;
                        move |_window, cx| {
                            write_text_to_clipboard(name.clone(), cx);
                        }
                    })
            }
        }))
    }
}
