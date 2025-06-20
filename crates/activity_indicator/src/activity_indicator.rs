use auto_update::{AutoUpdateStatus, AutoUpdater, DismissErrorMessage, VersionCheckType};
use editor::Editor;
use extension_host::ExtensionStore;

use gpui::{
    Animation, AnimationExt as _, App, Context, CursorStyle, Entity, EventEmitter,
    InteractiveElement as _, ParentElement as _, Render, SharedString, StatefulInteractiveElement,
    Styled, Transformation, Window, actions, percentage,
};
use language::{LanguageServerId, LanguageServerName};
use project::{
    EnvironmentErrorMessage, LanguageServerProgress, Project, ProjectEnvironmentEvent,
    git_store::{GitStoreEvent, Repository},
};
use smallvec::SmallVec;
use std::{
    cmp::Reverse,
    fmt::Write,
    path::Path,
    sync::Arc,
    time::{Duration, Instant},
};
use ui::{ButtonLike, ContextMenu, PopoverMenu, PopoverMenuHandle, Tooltip, prelude::*};
use util::truncate_and_trailoff;
use workspace::{StatusItemView, Workspace, item::ItemHandle};

const GIT_OPERATION_DELAY: Duration = Duration::from_millis(0);

actions!(activity_indicator, [ShowErrorMessage]);

pub enum Event {
    ShowStatus {
        server_name: LanguageServerName,
        status: SharedString,
    },
}

pub struct ActivityIndicator {
    project: Entity<Project>,
    auto_updater: Option<Entity<AutoUpdater>>,
    context_menu_handle: PopoverMenuHandle<ContextMenu>,
}

struct PendingWork<'a> {
    language_server_id: LanguageServerId,
    progress_token: &'a str,
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
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Entity<ActivityIndicator> {
        let project = workspace.project().clone();
        let auto_updater = AutoUpdater::get(cx);
        let workspace_handle = cx.entity();
        let this = cx.new(|cx| {
            cx.subscribe_in(
                &workspace_handle,
                window,
                |activity_indicator: &mut Self, _, event, window, cx| match event {
                    workspace::Event::ClearActivityIndicator => {
                        activity_indicator.dismiss_error_message(&DismissErrorMessage, window, cx);
                    }
                    _ => {}
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
                |_, _, event: &GitStoreEvent, cx| match event {
                    project::git_store::GitStoreEvent::JobsUpdated => cx.notify(),
                    _ => {}
                },
            )
            .detach();

            if let Some(auto_updater) = auto_updater.as_ref() {
                cx.observe(auto_updater, |_, _, cx| cx.notify()).detach();
            }

            Self {
                project: project.clone(),
                auto_updater,
                context_menu_handle: Default::default(),
            }
        });

        cx.subscribe_in(&this, window, move |_, _, event, window, cx| match event {
            Event::ShowStatus {
                server_name,
                status,
            } => {
                let create_buffer = project.update(cx, |project, cx| project.create_buffer(cx));
                let project = project.clone();
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
                    })?;
                    workspace.update_in(cx, |workspace, window, cx| {
                        workspace.add_item_to_active_pane(
                            Box::new(cx.new(|cx| {
                                let mut editor =
                                    Editor::for_buffer(buffer, Some(project.clone()), window, cx);
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

    fn dismiss_error_message(
        &mut self,
        _: &DismissErrorMessage,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let cleared = self.project.update(cx, |project, cx| {
            if project.last_formatting_failure(cx).is_some() {
                project.reset_last_formatting_failure(cx);
                true
            } else {
                false
            }
        });
        if !cleared {
            if let Some(updater) = &self.auto_updater {
                updater.update(cx, |updater, cx| updater.dismiss_error(cx));
            }
        }
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
                        .map(|(token, progress)| PendingWork {
                            language_server_id: server_id,
                            progress_token: token.as_str(),
                            progress,
                        })
                        .collect::<SmallVec<[_; 4]>>();
                    pending_work.sort_by_key(|work| Reverse(work.progress.last_update_at));
                    Some(pending_work)
                }
            })
            .flatten()
    }

    fn pending_environment_errors<'a>(
        &'a self,
        cx: &'a App,
    ) -> impl Iterator<Item = (&'a Arc<Path>, &'a EnvironmentErrorMessage)> {
        self.project.read(cx).shell_environment_errors(cx)
    }

    fn content_to_render(&mut self, cx: &mut Context<Self>) -> Option<Content> {
        // Show if any direnv calls failed
        if let Some((abs_path, error)) = self.pending_environment_errors(cx).next() {
            let abs_path = abs_path.clone();
            return Some(Content {
                icon: Some(
                    Icon::new(IconName::Warning)
                        .size(IconSize::Small)
                        .into_any_element(),
                ),
                message: error.0.clone(),
                on_click: Some(Arc::new(move |this, window, cx| {
                    this.project.update(cx, |project, cx| {
                        project.remove_environment_error(&abs_path, cx);
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
                let mut message = progress
                    .title
                    .as_deref()
                    .unwrap_or(progress_token)
                    .to_string();

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
                            .with_animation(
                                "arrow-circle",
                                Animation::new(Duration::from_secs(2)).repeat(),
                                |icon, delta| {
                                    icon.transform(Transformation::rotate(percentage(delta)))
                                },
                            )
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
                        .with_animation(
                            "arrow-circle",
                            Animation::new(Duration::from_secs(2)).repeat(),
                            |icon, delta| icon.transform(Transformation::rotate(percentage(delta))),
                        )
                        .into_any_element(),
                ),
                message: format!("Debug: {}", session.read(cx).adapter()),
                tooltip_message: Some(session.read(cx).label().to_string()),
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
        if let Some(job_info) = current_job {
            if Instant::now() - job_info.start >= GIT_OPERATION_DELAY {
                return Some(Content {
                    icon: Some(
                        Icon::new(IconName::ArrowCircle)
                            .size(IconSize::Small)
                            .with_animation(
                                "arrow-circle",
                                Animation::new(Duration::from_secs(2)).repeat(),
                                |icon, delta| {
                                    icon.transform(Transformation::rotate(percentage(delta)))
                                },
                            )
                            .into_any_element(),
                    ),
                    message: job_info.message.into(),
                    on_click: None,
                    tooltip_message: None,
                });
            }
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

        // Show any application auto-update info.
        if let Some(updater) = &self.auto_updater {
            return match &updater.read(cx).status() {
                AutoUpdateStatus::Checking => Some(Content {
                    icon: Some(
                        Icon::new(IconName::Download)
                            .size(IconSize::Small)
                            .into_any_element(),
                    ),
                    message: "Checking for Zed updates…".to_string(),
                    on_click: Some(Arc::new(|this, window, cx| {
                        this.dismiss_error_message(&DismissErrorMessage, window, cx)
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
                        this.dismiss_error_message(&DismissErrorMessage, window, cx)
                    })),
                    tooltip_message: Some(Self::version_tooltip_message(&version)),
                }),
                AutoUpdateStatus::Installing { version } => Some(Content {
                    icon: Some(
                        Icon::new(IconName::Download)
                            .size(IconSize::Small)
                            .into_any_element(),
                    ),
                    message: "Installing Zed update…".to_string(),
                    on_click: Some(Arc::new(|this, window, cx| {
                        this.dismiss_error_message(&DismissErrorMessage, window, cx)
                    })),
                    tooltip_message: Some(Self::version_tooltip_message(&version)),
                }),
                AutoUpdateStatus::Updated {
                    binary_path,
                    version,
                } => Some(Content {
                    icon: None,
                    message: "Click to restart and update Zed".to_string(),
                    on_click: Some(Arc::new({
                        let reload = workspace::Reload {
                            binary_path: Some(binary_path.clone()),
                        };
                        move |_, _, cx| workspace::reload(&reload, cx)
                    })),
                    tooltip_message: Some(Self::version_tooltip_message(&version)),
                }),
                AutoUpdateStatus::Errored => Some(Content {
                    icon: Some(
                        Icon::new(IconName::Warning)
                            .size(IconSize::Small)
                            .into_any_element(),
                    ),
                    message: "Auto update failed".to_string(),
                    on_click: Some(Arc::new(|this, window, cx| {
                        this.dismiss_error_message(&DismissErrorMessage, window, cx)
                    })),
                    tooltip_message: None,
                }),
                AutoUpdateStatus::Idle => None,
            };
        }

        if let Some(extension_store) =
            ExtensionStore::try_global(cx).map(|extension_store| extension_store.read(cx))
        {
            if let Some(extension_id) = extension_store.outstanding_operations().keys().next() {
                return Some(Content {
                    icon: Some(
                        Icon::new(IconName::Download)
                            .size(IconSize::Small)
                            .into_any_element(),
                    ),
                    message: format!("Updating {extension_id} extension…"),
                    on_click: Some(Arc::new(|this, window, cx| {
                        this.dismiss_error_message(&DismissErrorMessage, window, cx)
                    })),
                    tooltip_message: None,
                });
            }
        }

        None
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
            .on_action(cx.listener(Self::dismiss_error_message));
        let Some(content) = self.content_to_render(cx) else {
            return result;
        };
        let this = cx.entity().downgrade();
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
                    let strong_this = this.upgrade()?;
                    let mut has_work = false;
                    let menu = ContextMenu::build(window, cx, |mut menu, _, cx| {
                        for work in strong_this.read(cx).pending_language_server_work(cx) {
                            has_work = true;
                            let this = this.clone();
                            let mut title = work
                                .progress
                                .title
                                .as_deref()
                                .unwrap_or(work.progress_token)
                                .to_owned();

                            if work.progress.is_cancellable {
                                let language_server_id = work.language_server_id;
                                let token = work.progress_token.to_string();
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
                                        this.update(cx, |this, cx| {
                                            this.project.update(cx, |project, cx| {
                                                project.cancel_language_server_work(
                                                    language_server_id,
                                                    Some(token.clone()),
                                                    cx,
                                                );
                                            });
                                            this.context_menu_handle.hide(cx);
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
    use gpui::SemanticVersion;
    use release_channel::AppCommitSha;

    use super::*;

    #[test]
    fn test_version_tooltip_message() {
        let message = ActivityIndicator::version_tooltip_message(&VersionCheckType::Semantic(
            SemanticVersion::new(1, 0, 0),
        ));

        assert_eq!(message, "Version: 1.0.0");

        let message = ActivityIndicator::version_tooltip_message(&VersionCheckType::Sha(
            AppCommitSha::new("14d9a4189f058d8736339b06ff2340101eaea5af".to_string()),
        ));

        assert_eq!(message, "Version: 14d9a41…");
    }
}
