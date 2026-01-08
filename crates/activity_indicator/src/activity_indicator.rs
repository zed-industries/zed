use auto_update::{AutoUpdateStatus, AutoUpdater, DismissMessage, VersionCheckType};
use editor::Editor;
use extension_host::{ExtensionOperation, ExtensionStore};
use futures::StreamExt;
use gpui::{
    App, Context, CursorStyle, Entity, EventEmitter, InteractiveElement as _, ParentElement as _,
    Render, SharedString, StatefulInteractiveElement, Styled, Window, actions,
};
use language::{
    BinaryStatus, LanguageRegistry, LanguageServerId, LanguageServerName,
    LanguageServerStatusUpdate, ServerHealth,
};
use project::{
    LanguageServerProgress, LspStoreEvent, ProgressToken, Project, ProjectEnvironmentEvent,
    git_store::{GitStoreEvent, Repository},
};
use smallvec::SmallVec;
use std::{
    cmp::Reverse,
    collections::HashSet,
    fmt::Write,
    sync::Arc,
    time::{Duration, Instant},
};
use ui::{
    ButtonLike, CommonAnimationExt, ContextMenu, PopoverMenu, PopoverMenuHandle, Tooltip,
    prelude::*,
};
use util::truncate_and_trailoff;
use workspace::{StatusItemView, Workspace, item::ItemHandle};

const GIT_OPERATION_DELAY: Duration = Duration::from_millis(0);

actions!(
    activity_indicator,
    [
        /// Displays error messages from language servers in the status bar.
        ShowErrorMessage
    ]
);

pub enum Event {
    ShowStatus {
        server_name: LanguageServerName,
        status: SharedString,
    },
}

pub struct ActivityIndicator {
    statuses: Vec<ServerStatus>,
    project: Entity<Project>,
    auto_updater: Option<Entity<AutoUpdater>>,
    context_menu_handle: PopoverMenuHandle<ContextMenu>,
    fs_jobs: Vec<fs::JobInfo>,
}

#[derive(Debug)]
struct ServerStatus {
    name: LanguageServerName,
    status: LanguageServerStatusUpdate,
}

struct PendingWork<'a> {
    language_server_id: LanguageServerId,
    progress_token: &'a ProgressToken,
    progress: &'a LanguageServerProgress,
}

struct Content {
    icon: Option<gpui::AnyElement>,
    message: String,
    on_click:
        Option<Arc<dyn Fn(&mut ActivityIndicator, &mut Window, &mut Context<ActivityIndicator>)>>,
    tooltip_message: Option<String>,
}

impl ActivityIndicator {
    pub fn new(
        workspace: &mut Workspace,
        languages: Arc<LanguageRegistry>,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Entity<ActivityIndicator> {
        let project = workspace.project().clone();
        let auto_updater = AutoUpdater::get(cx);
        let this = cx.new(|cx| {
            let mut status_events = languages.language_server_binary_statuses();
            cx.spawn(async move |this, cx| {
                while let Some((name, binary_status)) = status_events.next().await {
                    this.update(cx, |this: &mut ActivityIndicator, cx| {
                        this.statuses.retain(|s| s.name != name);
                        this.statuses.push(ServerStatus {
                            name,
                            status: LanguageServerStatusUpdate::Binary(binary_status),
                        });
                        cx.notify();
                    })?;
                }
                anyhow::Ok(())
            })
            .detach();

            let fs = project.read(cx).fs().clone();
            let mut job_events = fs.subscribe_to_jobs();
            cx.spawn(async move |this, cx| {
                while let Some(job_event) = job_events.next().await {
                    this.update(cx, |this: &mut ActivityIndicator, cx| {
                        match job_event {
                            fs::JobEvent::Started { info } => {
                                this.fs_jobs.retain(|j| j.id != info.id);
                                this.fs_jobs.push(info);
                            }
                            fs::JobEvent::Completed { id } => {
                                this.fs_jobs.retain(|j| j.id != id);
                            }
                        }
                        cx.notify();
                    })?;
                }
                anyhow::Ok(())
            })
            .detach();

            cx.subscribe(
                &project.read(cx).lsp_store(),
                |activity_indicator, _, event, cx| {
                    if let LspStoreEvent::LanguageServerUpdate { name, message, .. } = event {
                        if let proto::update_language_server::Variant::StatusUpdate(status_update) =
                            message
                        {
                            let Some(name) = name.clone() else {
                                return;
                            };
                            let status = match &status_update.status {
                                Some(proto::status_update::Status::Binary(binary_status)) => {
                                    if let Some(binary_status) =
                                        proto::ServerBinaryStatus::from_i32(*binary_status)
                                    {
                                        let binary_status = match binary_status {
                                            proto::ServerBinaryStatus::None => BinaryStatus::None,
                                            proto::ServerBinaryStatus::CheckingForUpdate => {
                                                BinaryStatus::CheckingForUpdate
                                            }
                                            proto::ServerBinaryStatus::Downloading => {
                                                BinaryStatus::Downloading
                                            }
                                            proto::ServerBinaryStatus::Starting => {
                                                BinaryStatus::Starting
                                            }
                                            proto::ServerBinaryStatus::Stopping => {
                                                BinaryStatus::Stopping
                                            }
                                            proto::ServerBinaryStatus::Stopped => {
                                                BinaryStatus::Stopped
                                            }
                                            proto::ServerBinaryStatus::Failed => {
                                                let Some(error) = status_update.message.clone()
                                                else {
                                                    return;
                                                };
                                                BinaryStatus::Failed { error }
                                            }
                                        };
                                        LanguageServerStatusUpdate::Binary(binary_status)
                                    } else {
                                        return;
                                    }
                                }
                                Some(proto::status_update::Status::Health(health_status)) => {
                                    if let Some(health) =
                                        proto::ServerHealth::from_i32(*health_status)
                                    {
                                        let health = match health {
                                            proto::ServerHealth::Ok => ServerHealth::Ok,
                                            proto::ServerHealth::Warning => ServerHealth::Warning,
                                            proto::ServerHealth::Error => ServerHealth::Error,
                                        };
                                        LanguageServerStatusUpdate::Health(
                                            health,
                                            status_update.message.clone().map(SharedString::from),
                                        )
                                    } else {
                                        return;
                                    }
                                }
                                None => return,
                            };

                            activity_indicator.statuses.retain(|s| s.name != name);
                            activity_indicator
                                .statuses
                                .push(ServerStatus { name, status });
                        }
                        cx.notify()
                    }
                },
            )
            .detach();

            cx.subscribe(
                &project.read(cx).environment().clone(),
                |_, _, event, cx| match event {
                    ProjectEnvironmentEvent::ErrorsUpdated => cx.notify(),
                },
            )
            .detach();

            cx.subscribe(
                &project.read(cx).git_store().clone(),
                |_, _, event: &GitStoreEvent, cx| {
                    if let project::git_store::GitStoreEvent::JobsUpdated = event {
                        cx.notify()
                    }
                },
            )
            .detach();

            if let Some(auto_updater) = auto_updater.as_ref() {
                cx.observe(auto_updater, |_, _, cx| cx.notify()).detach();
            }

            Self {
                statuses: Vec::new(),
                project: project.clone(),
                auto_updater,
                context_menu_handle: PopoverMenuHandle::default(),
                fs_jobs: Vec::new(),
            }
        });

        cx.subscribe_in(&this, window, move |_, _, event, window, cx| match event {
            Event::ShowStatus {
                server_name,
                status,
            } => {
                let create_buffer =
                    project.update(cx, |project, cx| project.create_buffer(false, cx));
                let status = status.clone();
                let server_name = server_name.clone();
                cx.spawn_in(window, async move |workspace, cx| {
                    let buffer = create_buffer.await?;
                    buffer.update(cx, |buffer, cx| {
                        buffer.edit(
                            [(0..0, format!("Language server {server_name}:\n\n{status}"))],
                            None,
                            cx,
                        );
                        buffer.set_capability(language::Capability::ReadOnly, cx);
                    });
                    workspace.update_in(cx, |workspace, window, cx| {
                        workspace.add_item_to_active_pane(
                            Box::new(cx.new(|cx| {
                                let mut editor = Editor::for_buffer(buffer, None, window, cx);
                                editor.set_read_only(true);
                                editor
                            })),
                            None,
                            true,
                            window,
                            cx,
                        );
                    })?;

                    anyhow::Ok(())
                })
                .detach();
            }
        })
        .detach();
        this
    }

    fn show_error_message(&mut self, _: &ShowErrorMessage, _: &mut Window, cx: &mut Context<Self>) {
        let mut status_message_shown = false;
        self.statuses.retain(|status| match &status.status {
            LanguageServerStatusUpdate::Binary(BinaryStatus::Failed { error })
                if !status_message_shown =>
            {
                cx.emit(Event::ShowStatus {
                    server_name: status.name.clone(),
                    status: SharedString::from(error),
                });
                status_message_shown = true;
                false
            }
            LanguageServerStatusUpdate::Health(
                ServerHealth::Error | ServerHealth::Warning,
                status_string,
            ) if !status_message_shown => match status_string {
                Some(error) => {
                    cx.emit(Event::ShowStatus {
                        server_name: status.name.clone(),
                        status: error.clone(),
                    });
                    status_message_shown = true;
                    false
                }
                None => false,
            },
            _ => true,
        });
    }

    fn dismiss_message(&mut self, _: &DismissMessage, _: &mut Window, cx: &mut Context<Self>) {
        let dismissed = if let Some(updater) = &self.auto_updater {
            updater.update(cx, |updater, cx| updater.dismiss(cx))
        } else {
            false
        };
        if dismissed {
            return;
        }

        self.project.update(cx, |project, cx| {
            if project.last_formatting_failure(cx).is_some() {
                project.reset_last_formatting_failure(cx);
                true
            } else {
                false
            }
        });
    }

    fn pending_language_server_work<'a>(
        &self,
        cx: &'a App,
    ) -> impl Iterator<Item = PendingWork<'a>> {
        self.project
            .read(cx)
            .language_server_statuses(cx)
            .rev()
            .filter_map(|(server_id, status)| {
                if status.pending_work.is_empty() {
                    None
                } else {
                    let mut pending_work = status
                        .pending_work
                        .iter()
                        .map(|(progress_token, progress)| PendingWork {
                            language_server_id: server_id,
                            progress_token,
                            progress,
                        })
                        .collect::<SmallVec<[_; 4]>>();
                    pending_work.sort_by_key(|work| Reverse(work.progress.last_update_at));
                    Some(pending_work)
                }
            })
            .flatten()
    }

    fn pending_environment_error<'a>(&'a self, cx: &'a App) -> Option<&'a String> {
        self.project.read(cx).peek_environment_error(cx)
    }

    fn content_to_render(&mut self, cx: &mut Context<Self>) -> Option<Content> {
        // Show if any direnv calls failed
        if let Some(message) = self.pending_environment_error(cx) {
            return Some(Content {
                icon: Some(
                    Icon::new(IconName::Warning)
                        .size(IconSize::Small)
                        .into_any_element(),
                ),
                message: message.clone(),
                on_click: Some(Arc::new(move |this, window, cx| {
                    this.project.update(cx, |project, cx| {
                        project.pop_environment_error(cx);
                    });
                    window.dispatch_action(Box::new(workspace::OpenLog), cx);
                })),
                tooltip_message: None,
            });
        }
        // Show any language server has pending activity.
        {
            let mut pending_work = self.pending_language_server_work(cx);
            if let Some(PendingWork {
                progress_token,
                progress,
                ..
            }) = pending_work.next()
            {
                let mut message = progress.title.clone().unwrap_or(progress_token.to_string());

                if let Some(percentage) = progress.percentage {
                    write!(&mut message, " ({}%)", percentage).unwrap();
                }

                if let Some(progress_message) = progress.message.as_ref() {
                    message.push_str(": ");
                    message.push_str(progress_message);
                }

                let additional_work_count = pending_work.count();
                if additional_work_count > 0 {
                    write!(&mut message, " + {} more", additional_work_count).unwrap();
                }

                return Some(Content {
                    icon: Some(
                        Icon::new(IconName::ArrowCircle)
                            .size(IconSize::Small)
                            .with_rotate_animation(2)
                            .into_any_element(),
                    ),
                    message,
                    on_click: Some(Arc::new(Self::toggle_language_server_work_context_menu)),
                    tooltip_message: None,
                });
            }
        }

        if let Some(session) = self
            .project
            .read(cx)
            .dap_store()
            .read(cx)
            .sessions()
            .find(|s| !s.read(cx).is_started())
        {
            return Some(Content {
                icon: Some(
                    Icon::new(IconName::ArrowCircle)
                        .size(IconSize::Small)
                        .with_rotate_animation(2)
                        .into_any_element(),
                ),
                message: format!("Debug: {}", session.read(cx).adapter()),
                tooltip_message: session.read(cx).label().map(|label| label.to_string()),
                on_click: None,
            });
        }

        let current_job = self
            .project
            .read(cx)
            .active_repository(cx)
            .map(|r| r.read(cx))
            .and_then(Repository::current_job);
        // Show any long-running git command
        if let Some(job_info) = current_job
            && Instant::now() - job_info.start >= GIT_OPERATION_DELAY
        {
            return Some(Content {
                icon: Some(
                    Icon::new(IconName::ArrowCircle)
                        .size(IconSize::Small)
                        .with_rotate_animation(2)
                        .into_any_element(),
                ),
                message: job_info.message.into(),
                on_click: None,
                tooltip_message: None,
            });
        }

        // Show any long-running fs command
        for fs_job in &self.fs_jobs {
            if Instant::now().duration_since(fs_job.start) >= GIT_OPERATION_DELAY {
                return Some(Content {
                    icon: Some(
                        Icon::new(IconName::ArrowCircle)
                            .size(IconSize::Small)
                            .with_rotate_animation(2)
                            .into_any_element(),
                    ),
                    message: fs_job.message.clone().into(),
                    on_click: None,
                    tooltip_message: None,
                });
            }
        }

        // Show any language server installation info.
        let mut downloading = SmallVec::<[_; 3]>::new();
        let mut checking_for_update = SmallVec::<[_; 3]>::new();
        let mut failed = SmallVec::<[_; 3]>::new();
        let mut health_messages = SmallVec::<[_; 3]>::new();
        let mut servers_to_clear_statuses = HashSet::<LanguageServerName>::default();
        for status in &self.statuses {
            match &status.status {
                LanguageServerStatusUpdate::Binary(
                    BinaryStatus::Starting | BinaryStatus::Stopping,
                ) => {}
                LanguageServerStatusUpdate::Binary(BinaryStatus::Stopped) => {
                    servers_to_clear_statuses.insert(status.name.clone());
                }
                LanguageServerStatusUpdate::Binary(BinaryStatus::CheckingForUpdate) => {
                    checking_for_update.push(status.name.clone());
                }
                LanguageServerStatusUpdate::Binary(BinaryStatus::Downloading) => {
                    downloading.push(status.name.clone());
                }
                LanguageServerStatusUpdate::Binary(BinaryStatus::Failed { .. }) => {
                    failed.push(status.name.clone());
                }
                LanguageServerStatusUpdate::Binary(BinaryStatus::None) => {}
                LanguageServerStatusUpdate::Health(health, server_status) => match server_status {
                    Some(server_status) => {
                        health_messages.push((status.name.clone(), *health, server_status.clone()));
                    }
                    None => {
                        servers_to_clear_statuses.insert(status.name.clone());
                    }
                },
            }
        }
        self.statuses
            .retain(|status| !servers_to_clear_statuses.contains(&status.name));

        health_messages.sort_by_key(|(_, health, _)| match health {
            ServerHealth::Error => 2,
            ServerHealth::Warning => 1,
            ServerHealth::Ok => 0,
        });

        if !downloading.is_empty() {
            return Some(Content {
                icon: Some(
                    Icon::new(IconName::Download)
                        .size(IconSize::Small)
                        .into_any_element(),
                ),
                message: format!(
                    "Downloading {}...",
                    downloading.iter().map(|name| name.as_ref()).fold(
                        String::new(),
                        |mut acc, s| {
                            if !acc.is_empty() {
                                acc.push_str(", ");
                            }
                            acc.push_str(s);
                            acc
                        }
                    )
                ),
                on_click: Some(Arc::new(move |this, window, cx| {
                    this.statuses
                        .retain(|status| !downloading.contains(&status.name));
                    this.dismiss_message(&DismissMessage, window, cx)
                })),
                tooltip_message: None,
            });
        }

        if !checking_for_update.is_empty() {
            return Some(Content {
                icon: Some(
                    Icon::new(IconName::Download)
                        .size(IconSize::Small)
                        .into_any_element(),
                ),
                message: format!(
                    "Checking for updates to {}...",
                    checking_for_update.iter().map(|name| name.as_ref()).fold(
                        String::new(),
                        |mut acc, s| {
                            if !acc.is_empty() {
                                acc.push_str(", ");
                            }
                            acc.push_str(s);
                            acc
                        }
                    ),
                ),
                on_click: Some(Arc::new(move |this, window, cx| {
                    this.statuses
                        .retain(|status| !checking_for_update.contains(&status.name));
                    this.dismiss_message(&DismissMessage, window, cx)
                })),
                tooltip_message: None,
            });
        }

        if !failed.is_empty() {
            return Some(Content {
                icon: Some(
                    Icon::new(IconName::Warning)
                        .size(IconSize::Small)
                        .into_any_element(),
                ),
                message: format!(
                    "Failed to run {}. Click to show error.",
                    failed
                        .iter()
                        .map(|name| name.as_ref())
                        .fold(String::new(), |mut acc, s| {
                            if !acc.is_empty() {
                                acc.push_str(", ");
                            }
                            acc.push_str(s);
                            acc
                        }),
                ),
                on_click: Some(Arc::new(|this, window, cx| {
                    this.show_error_message(&ShowErrorMessage, window, cx)
                })),
                tooltip_message: None,
            });
        }

        // Show any formatting failure
        if let Some(failure) = self.project.read(cx).last_formatting_failure(cx) {
            return Some(Content {
                icon: Some(
                    Icon::new(IconName::Warning)
                        .size(IconSize::Small)
                        .into_any_element(),
                ),
                message: format!("Formatting failed: {failure}. Click to see logs."),
                on_click: Some(Arc::new(|indicator, window, cx| {
                    indicator.project.update(cx, |project, cx| {
                        project.reset_last_formatting_failure(cx);
                    });
                    window.dispatch_action(Box::new(workspace::OpenLog), cx);
                })),
                tooltip_message: None,
            });
        }

        // Show any health messages for the language servers
        if let Some((server_name, health, message)) = health_messages.pop() {
            let health_str = match health {
                ServerHealth::Ok => format!("({server_name}) "),
                ServerHealth::Warning => format!("({server_name}) Warning: "),
                ServerHealth::Error => format!("({server_name}) Error: "),
            };
            let single_line_message = message
                .lines()
                .filter_map(|line| {
                    let line = line.trim();
                    if line.is_empty() { None } else { Some(line) }
                })
                .collect::<Vec<_>>()
                .join(" ");
            let mut altered_message = single_line_message != message;
            let truncated_message = truncate_and_trailoff(
                &single_line_message,
                MAX_MESSAGE_LEN.saturating_sub(health_str.len()),
            );
            altered_message |= truncated_message != single_line_message;
            let final_message = format!("{health_str}{truncated_message}");

            let tooltip_message = if altered_message {
                Some(format!("{health_str}{message}"))
            } else {
                None
            };

            return Some(Content {
                icon: Some(
                    Icon::new(IconName::Warning)
                        .size(IconSize::Small)
                        .into_any_element(),
                ),
                message: final_message,
                tooltip_message,
                on_click: Some(Arc::new(move |activity_indicator, window, cx| {
                    if altered_message {
                        activity_indicator.show_error_message(&ShowErrorMessage, window, cx)
                    } else {
                        activity_indicator
                            .statuses
                            .retain(|status| status.name != server_name);
                        cx.notify();
                    }
                })),
            });
        }

        // Show any application auto-update info.
        self.auto_updater
            .as_ref()
            .and_then(|updater| match &updater.read(cx).status() {
                AutoUpdateStatus::Checking => Some(Content {
                    icon: Some(
                        Icon::new(IconName::LoadCircle)
                            .size(IconSize::Small)
                            .with_rotate_animation(3)
                            .into_any_element(),
                    ),
                    message: "Checking for Zed updates…".to_string(),
                    on_click: Some(Arc::new(|this, window, cx| {
                        this.dismiss_message(&DismissMessage, window, cx)
                    })),
                    tooltip_message: None,
                }),
                AutoUpdateStatus::Downloading { version } => Some(Content {
                    icon: Some(
                        Icon::new(IconName::Download)
                            .size(IconSize::Small)
                            .into_any_element(),
                    ),
                    message: "Downloading Zed update…".to_string(),
                    on_click: Some(Arc::new(|this, window, cx| {
                        this.dismiss_message(&DismissMessage, window, cx)
                    })),
                    tooltip_message: Some(Self::version_tooltip_message(version)),
                }),
                AutoUpdateStatus::Installing { version } => Some(Content {
                    icon: Some(
                        Icon::new(IconName::LoadCircle)
                            .size(IconSize::Small)
                            .with_rotate_animation(3)
                            .into_any_element(),
                    ),
                    message: "Installing Zed update…".to_string(),
                    on_click: Some(Arc::new(|this, window, cx| {
                        this.dismiss_message(&DismissMessage, window, cx)
                    })),
                    tooltip_message: Some(Self::version_tooltip_message(version)),
                }),
                AutoUpdateStatus::Updated { version } => Some(Content {
                    icon: None,
                    message: "Click to restart and update Zed".to_string(),
                    on_click: Some(Arc::new(move |_, _, cx| workspace::reload(cx))),
                    tooltip_message: Some(Self::version_tooltip_message(version)),
                }),
                AutoUpdateStatus::Errored { error } => Some(Content {
                    icon: Some(
                        Icon::new(IconName::Warning)
                            .size(IconSize::Small)
                            .into_any_element(),
                    ),
                    message: "Failed to update Zed".to_string(),
                    on_click: Some(Arc::new(|this, window, cx| {
                        window.dispatch_action(Box::new(workspace::OpenLog), cx);
                        this.dismiss_message(&DismissMessage, window, cx);
                    })),
                    tooltip_message: Some(format!("{error}")),
                }),
                AutoUpdateStatus::Idle => None,
            })
            .or_else(|| {
                if let Some(extension_store) =
                    ExtensionStore::try_global(cx).map(|extension_store| extension_store.read(cx))
                    && let Some((extension_id, operation)) =
                        extension_store.outstanding_operations().iter().next()
                {
                    let (message, icon, rotate) = match operation {
                        ExtensionOperation::Install => (
                            format!("Installing {extension_id} extension…"),
                            IconName::LoadCircle,
                            true,
                        ),
                        ExtensionOperation::Upgrade => (
                            format!("Updating {extension_id} extension…"),
                            IconName::Download,
                            false,
                        ),
                        ExtensionOperation::Remove => (
                            format!("Removing {extension_id} extension…"),
                            IconName::LoadCircle,
                            true,
                        ),
                    };

                    Some(Content {
                        icon: Some(Icon::new(icon).size(IconSize::Small).map(|this| {
                            if rotate {
                                this.with_rotate_animation(3).into_any_element()
                            } else {
                                this.into_any_element()
                            }
                        })),
                        message,
                        on_click: Some(Arc::new(|this, window, cx| {
                            this.dismiss_message(&Default::default(), window, cx)
                        })),
                        tooltip_message: None,
                    })
                } else {
                    None
                }
            })
    }

    fn version_tooltip_message(version: &VersionCheckType) -> String {
        format!("Version: {}", {
            match version {
                auto_update::VersionCheckType::Sha(sha) => format!("{}…", sha.short()),
                auto_update::VersionCheckType::Semantic(semantic_version) => {
                    semantic_version.to_string()
                }
            }
        })
    }

    fn toggle_language_server_work_context_menu(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.context_menu_handle.toggle(window, cx);
    }
}

impl EventEmitter<Event> for ActivityIndicator {}

const MAX_MESSAGE_LEN: usize = 50;

impl Render for ActivityIndicator {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let result = h_flex()
            .id("activity-indicator")
            .on_action(cx.listener(Self::show_error_message))
            .on_action(cx.listener(Self::dismiss_message));
        let Some(content) = self.content_to_render(cx) else {
            return result;
        };
        let activity_indicator = cx.entity().downgrade();
        let truncate_content = content.message.len() > MAX_MESSAGE_LEN;
        result.gap_2().child(
            PopoverMenu::new("activity-indicator-popover")
                .trigger(
                    ButtonLike::new("activity-indicator-trigger").child(
                        h_flex()
                            .id("activity-indicator-status")
                            .gap_2()
                            .children(content.icon)
                            .map(|button| {
                                if truncate_content {
                                    button
                                        .child(
                                            Label::new(truncate_and_trailoff(
                                                &content.message,
                                                MAX_MESSAGE_LEN,
                                            ))
                                            .size(LabelSize::Small),
                                        )
                                        .tooltip(Tooltip::text(content.message))
                                } else {
                                    button
                                        .child(Label::new(content.message).size(LabelSize::Small))
                                        .when_some(
                                            content.tooltip_message,
                                            |this, tooltip_message| {
                                                this.tooltip(Tooltip::text(tooltip_message))
                                            },
                                        )
                                }
                            })
                            .when_some(content.on_click, |this, handler| {
                                this.on_click(cx.listener(move |this, _, window, cx| {
                                    handler(this, window, cx);
                                }))
                                .cursor(CursorStyle::PointingHand)
                            }),
                    ),
                )
                .anchor(gpui::Corner::BottomLeft)
                .menu(move |window, cx| {
                    let strong_this = activity_indicator.upgrade()?;
                    let mut has_work = false;
                    let menu = ContextMenu::build(window, cx, |mut menu, _, cx| {
                        for work in strong_this.read(cx).pending_language_server_work(cx) {
                            has_work = true;
                            let activity_indicator = activity_indicator.clone();
                            let mut title = work
                                .progress
                                .title
                                .clone()
                                .unwrap_or(work.progress_token.to_string());

                            if work.progress.is_cancellable {
                                let language_server_id = work.language_server_id;
                                let token = work.progress_token.clone();
                                let title = SharedString::from(title);
                                menu = menu.custom_entry(
                                    move |_, _| {
                                        h_flex()
                                            .w_full()
                                            .justify_between()
                                            .child(Label::new(title.clone()))
                                            .child(Icon::new(IconName::XCircle))
                                            .into_any_element()
                                    },
                                    move |_, cx| {
                                        let token = token.clone();
                                        activity_indicator
                                            .update(cx, |activity_indicator, cx| {
                                                activity_indicator.project.update(
                                                    cx,
                                                    |project, cx| {
                                                        project.cancel_language_server_work(
                                                            language_server_id,
                                                            Some(token),
                                                            cx,
                                                        );
                                                    },
                                                );
                                                activity_indicator.context_menu_handle.hide(cx);
                                                cx.notify();
                                            })
                                            .ok();
                                    },
                                );
                            } else {
                                if let Some(progress_message) = work.progress.message.as_ref() {
                                    title.push_str(": ");
                                    title.push_str(progress_message);
                                }

                                menu = menu.label(title);
                            }
                        }
                        menu
                    });
                    has_work.then_some(menu)
                }),
        )
    }
}

impl StatusItemView for ActivityIndicator {
    fn set_active_pane_item(
        &mut self,
        _: Option<&dyn ItemHandle>,
        _window: &mut Window,
        _: &mut Context<Self>,
    ) {
    }
}

#[cfg(test)]
mod tests {
    use release_channel::AppCommitSha;
    use semver::Version;

    use super::*;

    #[test]
    fn test_version_tooltip_message() {
        let message = ActivityIndicator::version_tooltip_message(&VersionCheckType::Semantic(
            Version::new(1, 0, 0),
        ));

        assert_eq!(message, "Version: 1.0.0");

        let message = ActivityIndicator::version_tooltip_message(&VersionCheckType::Sha(
            AppCommitSha::new("14d9a4189f058d8736339b06ff2340101eaea5af".to_string()),
        ));

        assert_eq!(message, "Version: 14d9a41…");
    }
}
