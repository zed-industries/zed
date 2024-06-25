use auto_update::{AutoUpdateStatus, AutoUpdater, DismissErrorMessage};
use editor::Editor;
use extension::ExtensionStore;
use futures::StreamExt;
use gpui::{
    actions, anchored, deferred, percentage, Animation, AnimationExt as _, AppContext, CursorStyle,
    DismissEvent, EventEmitter, InteractiveElement as _, Model, ParentElement as _, Render,
    SharedString, StatefulInteractiveElement, Styled, Transformation, View, ViewContext,
    VisualContext as _,
};
use language::{
    LanguageRegistry, LanguageServerBinaryStatus, LanguageServerId, LanguageServerName,
};
use project::{LanguageServerProgress, Project};
use smallvec::SmallVec;
use std::{cmp::Reverse, fmt::Write, sync::Arc, time::Duration};
use ui::{prelude::*, ContextMenu};
use workspace::{item::ItemHandle, StatusItemView, Workspace};

actions!(activity_indicator, [ShowErrorMessage]);

pub enum Event {
    ShowError { lsp_name: Arc<str>, error: String },
}

pub struct ActivityIndicator {
    statuses: Vec<LspStatus>,
    project: Model<Project>,
    auto_updater: Option<Model<AutoUpdater>>,
    context_menu: Option<View<ContextMenu>>,
}

struct LspStatus {
    name: LanguageServerName,
    status: LanguageServerBinaryStatus,
}

struct PendingWork<'a> {
    language_server_id: LanguageServerId,
    progress_token: &'a str,
    progress: &'a LanguageServerProgress,
}

#[derive(Default)]
struct Content {
    icon: Option<gpui::AnyElement>,
    message: String,
    on_click: Option<Arc<dyn Fn(&mut ActivityIndicator, &mut ViewContext<ActivityIndicator>)>>,
}

impl ActivityIndicator {
    pub fn new(
        workspace: &mut Workspace,
        languages: Arc<LanguageRegistry>,
        cx: &mut ViewContext<Workspace>,
    ) -> View<ActivityIndicator> {
        let project = workspace.project().clone();
        let auto_updater = AutoUpdater::get(cx);
        let this = cx.new_view(|cx: &mut ViewContext<Self>| {
            let mut status_events = languages.language_server_binary_statuses();
            cx.spawn(|this, mut cx| async move {
                while let Some((name, status)) = status_events.next().await {
                    this.update(&mut cx, |this, cx| {
                        this.statuses.retain(|s| s.name != name);
                        this.statuses.push(LspStatus { name, status });
                        cx.notify();
                    })?;
                }
                anyhow::Ok(())
            })
            .detach();
            cx.observe(&project, |_, _, cx| cx.notify()).detach();

            if let Some(auto_updater) = auto_updater.as_ref() {
                cx.observe(auto_updater, |_, _, cx| cx.notify()).detach();
            }

            Self {
                statuses: Default::default(),
                project: project.clone(),
                auto_updater,
                context_menu: None,
            }
        });

        cx.subscribe(&this, move |_, _, event, cx| match event {
            Event::ShowError { lsp_name, error } => {
                let create_buffer = project.update(cx, |project, cx| project.create_buffer(cx));
                let project = project.clone();
                let error = error.clone();
                let lsp_name = lsp_name.clone();
                cx.spawn(|workspace, mut cx| async move {
                    let buffer = create_buffer.await?;
                    buffer.update(&mut cx, |buffer, cx| {
                        buffer.edit(
                            [(
                                0..0,
                                format!("Language server error: {}\n\n{}", lsp_name, error),
                            )],
                            None,
                            cx,
                        );
                    })?;
                    workspace.update(&mut cx, |workspace, cx| {
                        workspace.add_item_to_active_pane(
                            Box::new(cx.new_view(|cx| {
                                Editor::for_buffer(buffer, Some(project.clone()), cx)
                            })),
                            None,
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

    fn show_error_message(&mut self, _: &ShowErrorMessage, cx: &mut ViewContext<Self>) {
        self.statuses.retain(|status| {
            if let LanguageServerBinaryStatus::Failed { error } = &status.status {
                cx.emit(Event::ShowError {
                    lsp_name: status.name.0.clone(),
                    error: error.clone(),
                });
                false
            } else {
                true
            }
        });

        cx.notify();
    }

    fn dismiss_error_message(&mut self, _: &DismissErrorMessage, cx: &mut ViewContext<Self>) {
        if let Some(updater) = &self.auto_updater {
            updater.update(cx, |updater, cx| {
                updater.dismiss_error(cx);
            });
        }
        cx.notify();
    }

    fn pending_language_server_work<'a>(
        &self,
        cx: &'a AppContext,
    ) -> impl Iterator<Item = PendingWork<'a>> {
        self.project
            .read(cx)
            .language_server_statuses()
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

    fn content_to_render(&mut self, cx: &mut ViewContext<Self>) -> Content {
        // Show any language server has pending activity.
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

            return Content {
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
                message,
                on_click: Some(Arc::new(Self::toggle_language_server_work_context_menu)),
            };
        }

        // Show any language server installation info.
        let mut downloading = SmallVec::<[_; 3]>::new();
        let mut checking_for_update = SmallVec::<[_; 3]>::new();
        let mut failed = SmallVec::<[_; 3]>::new();
        for status in &self.statuses {
            match status.status {
                LanguageServerBinaryStatus::CheckingForUpdate => {
                    checking_for_update.push(status.name.0.as_ref())
                }
                LanguageServerBinaryStatus::Downloading => downloading.push(status.name.0.as_ref()),
                LanguageServerBinaryStatus::Failed { .. } => failed.push(status.name.0.as_ref()),
                LanguageServerBinaryStatus::None => {}
            }
        }

        if !downloading.is_empty() {
            return Content {
                icon: Some(
                    Icon::new(IconName::Download)
                        .size(IconSize::Small)
                        .into_any_element(),
                ),
                message: format!("Downloading {}...", downloading.join(", "),),
                on_click: None,
            };
        }

        if !checking_for_update.is_empty() {
            return Content {
                icon: Some(
                    Icon::new(IconName::Download)
                        .size(IconSize::Small)
                        .into_any_element(),
                ),
                message: format!(
                    "Checking for updates to {}...",
                    checking_for_update.join(", "),
                ),
                on_click: None,
            };
        }

        if !failed.is_empty() {
            return Content {
                icon: Some(
                    Icon::new(IconName::ExclamationTriangle)
                        .size(IconSize::Small)
                        .into_any_element(),
                ),
                message: format!(
                    "Failed to download {}. Click to show error.",
                    failed.join(", "),
                ),
                on_click: Some(Arc::new(|this, cx| {
                    this.show_error_message(&Default::default(), cx)
                })),
            };
        }

        // Show any formatting failure
        if let Some(failure) = self.project.read(cx).last_formatting_failure() {
            return Content {
                icon: Some(
                    Icon::new(IconName::ExclamationTriangle)
                        .size(IconSize::Small)
                        .into_any_element(),
                ),
                message: format!("Formatting failed: {}. Click to see logs.", failure),
                on_click: Some(Arc::new(|_, cx| {
                    cx.dispatch_action(Box::new(workspace::OpenLog));
                })),
            };
        }

        // Show any application auto-update info.
        if let Some(updater) = &self.auto_updater {
            return match &updater.read(cx).status() {
                AutoUpdateStatus::Checking => Content {
                    icon: Some(
                        Icon::new(IconName::Download)
                            .size(IconSize::Small)
                            .into_any_element(),
                    ),
                    message: "Checking for Zed updates…".to_string(),
                    on_click: None,
                },
                AutoUpdateStatus::Downloading => Content {
                    icon: Some(
                        Icon::new(IconName::Download)
                            .size(IconSize::Small)
                            .into_any_element(),
                    ),
                    message: "Downloading Zed update…".to_string(),
                    on_click: None,
                },
                AutoUpdateStatus::Installing => Content {
                    icon: Some(
                        Icon::new(IconName::Download)
                            .size(IconSize::Small)
                            .into_any_element(),
                    ),
                    message: "Installing Zed update…".to_string(),
                    on_click: None,
                },
                AutoUpdateStatus::Updated { binary_path } => Content {
                    icon: None,
                    message: "Click to restart and update Zed".to_string(),
                    on_click: Some(Arc::new({
                        let reload = workspace::Reload {
                            binary_path: Some(binary_path.clone()),
                        };
                        move |_, cx| workspace::reload(&reload, cx)
                    })),
                },
                AutoUpdateStatus::Errored => Content {
                    icon: Some(
                        Icon::new(IconName::ExclamationTriangle)
                            .size(IconSize::Small)
                            .into_any_element(),
                    ),
                    message: "Auto update failed".to_string(),
                    on_click: Some(Arc::new(|this, cx| {
                        this.dismiss_error_message(&Default::default(), cx)
                    })),
                },
                AutoUpdateStatus::Idle => Default::default(),
            };
        }

        if let Some(extension_store) =
            ExtensionStore::try_global(cx).map(|extension_store| extension_store.read(cx))
        {
            if let Some(extension_id) = extension_store.outstanding_operations().keys().next() {
                return Content {
                    icon: Some(
                        Icon::new(IconName::Download)
                            .size(IconSize::Small)
                            .into_any_element(),
                    ),
                    message: format!("Updating {extension_id} extension…"),
                    on_click: None,
                };
            }
        }

        Default::default()
    }

    fn toggle_language_server_work_context_menu(&mut self, cx: &mut ViewContext<Self>) {
        if self.context_menu.take().is_some() {
            return;
        }

        self.build_lsp_work_context_menu(cx);
        cx.notify();
    }

    fn build_lsp_work_context_menu(&mut self, cx: &mut ViewContext<Self>) {
        let mut has_work = false;
        let this = cx.view().downgrade();
        let context_menu = ContextMenu::build(cx, |mut menu, cx| {
            for work in self.pending_language_server_work(cx) {
                has_work = true;

                let this = this.clone();
                let title = SharedString::from(
                    work.progress
                        .title
                        .as_deref()
                        .unwrap_or(work.progress_token)
                        .to_string(),
                );
                if work.progress.is_cancellable {
                    let language_server_id = work.language_server_id;
                    let token = work.progress_token.to_string();
                    menu = menu.custom_entry(
                        move |_| {
                            h_flex()
                                .w_full()
                                .justify_between()
                                .child(Label::new(title.clone()))
                                .child(Icon::new(IconName::XCircle))
                                .into_any_element()
                        },
                        move |cx| {
                            this.update(cx, |this, cx| {
                                this.project.update(cx, |project, cx| {
                                    project.cancel_language_server_work(
                                        language_server_id,
                                        Some(token.clone()),
                                        cx,
                                    );
                                });
                                this.context_menu.take();
                            })
                            .ok();
                        },
                    );
                } else {
                    menu = menu.label(title.clone());
                }
            }
            menu
        });

        if has_work {
            cx.subscribe(&context_menu, |this, _, _: &DismissEvent, cx| {
                this.context_menu.take();
                cx.notify();
            })
            .detach();
            cx.focus_view(&context_menu);
            self.context_menu = Some(context_menu);
            cx.notify();
        }
    }
}

impl EventEmitter<Event> for ActivityIndicator {}

impl Render for ActivityIndicator {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let content = self.content_to_render(cx);

        let mut result = h_flex()
            .id("activity-indicator")
            .on_action(cx.listener(Self::show_error_message))
            .on_action(cx.listener(Self::dismiss_error_message));

        if let Some(on_click) = content.on_click {
            result = result
                .cursor(CursorStyle::PointingHand)
                .on_click(cx.listener(move |this, _, cx| {
                    on_click(this, cx);
                }))
        }

        result
            .gap_2()
            .children(content.icon)
            .child(Label::new(SharedString::from(content.message)).size(LabelSize::Small))
            .children(self.context_menu.as_ref().map(|menu| {
                deferred(
                    anchored()
                        .anchor(gpui::AnchorCorner::BottomLeft)
                        .child(menu.clone()),
                )
                .with_priority(1)
            }))
    }
}

impl StatusItemView for ActivityIndicator {
    fn set_active_pane_item(&mut self, _: Option<&dyn ItemHandle>, _: &mut ViewContext<Self>) {}
}
