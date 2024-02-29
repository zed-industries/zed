use auto_update::{AutoUpdateStatus, AutoUpdater, DismissErrorMessage};
use editor::Editor;
use futures::StreamExt;
use gpui::{
    actions, svg, AppContext, CursorStyle, EventEmitter, InteractiveElement as _, Model,
    ParentElement as _, Render, SharedString, StatefulInteractiveElement, Styled, View,
    ViewContext, VisualContext as _,
};
use language::{LanguageRegistry, LanguageServerBinaryStatus};
use project::{LanguageServerProgress, Project};
use smallvec::SmallVec;
use std::{cmp::Reverse, fmt::Write, sync::Arc};
use ui::prelude::*;
use util::ResultExt;
use workspace::{item::ItemHandle, StatusItemView, Workspace};

actions!(activity_indicator, [ShowErrorMessage]);

const DOWNLOAD_ICON: &str = "icons/download.svg";
const WARNING_ICON: &str = "icons/warning.svg";

pub enum Event {
    ShowError { lsp_name: Arc<str>, error: String },
}

pub struct ActivityIndicator {
    statuses: Vec<LspStatus>,
    project: Model<Project>,
    auto_updater: Option<Model<AutoUpdater>>,
}

struct LspStatus {
    name: Arc<str>,
    status: LanguageServerBinaryStatus,
}

struct PendingWork<'a> {
    language_server_name: &'a str,
    progress_token: &'a str,
    progress: &'a LanguageServerProgress,
}

#[derive(Default)]
struct Content {
    icon: Option<&'static str>,
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
                while let Some((language, event)) = status_events.next().await {
                    this.update(&mut cx, |this, cx| {
                        this.statuses.retain(|s| s.name != language.name());
                        this.statuses.push(LspStatus {
                            name: language.name(),
                            status: event,
                        });
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
            }
        });

        cx.subscribe(&this, move |workspace, _, event, cx| match event {
            Event::ShowError { lsp_name, error } => {
                if let Some(buffer) = project
                    .update(cx, |project, cx| project.create_buffer(error, None, cx))
                    .log_err()
                {
                    buffer.update(cx, |buffer, cx| {
                        buffer.edit(
                            [(0..0, format!("Language server error: {}\n\n", lsp_name))],
                            None,
                            cx,
                        );
                    });
                    workspace.add_item_to_active_pane(
                        Box::new(
                            cx.new_view(|cx| Editor::for_buffer(buffer, Some(project.clone()), cx)),
                        ),
                        cx,
                    );
                }
            }
        })
        .detach();
        this
    }

    fn show_error_message(&mut self, _: &ShowErrorMessage, cx: &mut ViewContext<Self>) {
        self.statuses.retain(|status| {
            if let LanguageServerBinaryStatus::Failed { error } = &status.status {
                cx.emit(Event::ShowError {
                    lsp_name: status.name.clone(),
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
            .filter_map(|status| {
                if status.pending_work.is_empty() {
                    None
                } else {
                    let mut pending_work = status
                        .pending_work
                        .iter()
                        .map(|(token, progress)| PendingWork {
                            language_server_name: status.name.as_str(),
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
            language_server_name,
            progress_token,
            progress,
        }) = pending_work.next()
        {
            let mut message = language_server_name.to_string();

            message.push_str(": ");
            if let Some(progress_message) = progress.message.as_ref() {
                message.push_str(progress_message);
            } else {
                message.push_str(progress_token);
            }

            if let Some(percentage) = progress.percentage {
                write!(&mut message, " ({}%)", percentage).unwrap();
            }

            let additional_work_count = pending_work.count();
            if additional_work_count > 0 {
                write!(&mut message, " + {} more", additional_work_count).unwrap();
            }

            return Content {
                icon: None,
                message,
                on_click: None,
            };
        }

        // Show any language server installation info.
        let mut downloading = SmallVec::<[_; 3]>::new();
        let mut checking_for_update = SmallVec::<[_; 3]>::new();
        let mut failed = SmallVec::<[_; 3]>::new();
        for status in &self.statuses {
            let name = status.name.clone();
            match status.status {
                LanguageServerBinaryStatus::CheckingForUpdate => checking_for_update.push(name),
                LanguageServerBinaryStatus::Downloading => downloading.push(name),
                LanguageServerBinaryStatus::Failed { .. } => failed.push(name),
                LanguageServerBinaryStatus::Downloaded | LanguageServerBinaryStatus::Cached => {}
            }
        }

        if !downloading.is_empty() {
            return Content {
                icon: Some(DOWNLOAD_ICON),
                message: format!(
                    "Downloading {} language server{}...",
                    downloading.join(", "),
                    if downloading.len() > 1 { "s" } else { "" }
                ),
                on_click: None,
            };
        } else if !checking_for_update.is_empty() {
            return Content {
                icon: Some(DOWNLOAD_ICON),
                message: format!(
                    "Checking for updates to {} language server{}...",
                    checking_for_update.join(", "),
                    if checking_for_update.len() > 1 {
                        "s"
                    } else {
                        ""
                    }
                ),
                on_click: None,
            };
        } else if !failed.is_empty() {
            return Content {
                icon: Some(WARNING_ICON),
                message: format!(
                    "Failed to download {} language server{}. Click to show error.",
                    failed.join(", "),
                    if failed.len() > 1 { "s" } else { "" }
                ),
                on_click: Some(Arc::new(|this, cx| {
                    this.show_error_message(&Default::default(), cx)
                })),
            };
        }

        // Show any application auto-update info.
        if let Some(updater) = &self.auto_updater {
            return match &updater.read(cx).status() {
                AutoUpdateStatus::Checking => Content {
                    icon: Some(DOWNLOAD_ICON),
                    message: "Checking for Zed updates…".to_string(),
                    on_click: None,
                },
                AutoUpdateStatus::Downloading => Content {
                    icon: Some(DOWNLOAD_ICON),
                    message: "Downloading Zed update…".to_string(),
                    on_click: None,
                },
                AutoUpdateStatus::Installing => Content {
                    icon: Some(DOWNLOAD_ICON),
                    message: "Installing Zed update…".to_string(),
                    on_click: None,
                },
                AutoUpdateStatus::Updated => Content {
                    icon: None,
                    message: "Click to restart and update Zed".to_string(),
                    on_click: Some(Arc::new(|_, cx| {
                        workspace::restart(&Default::default(), cx)
                    })),
                },
                AutoUpdateStatus::Errored => Content {
                    icon: Some(WARNING_ICON),
                    message: "Auto update failed".to_string(),
                    on_click: Some(Arc::new(|this, cx| {
                        this.dismiss_error_message(&Default::default(), cx)
                    })),
                },
                AutoUpdateStatus::Idle => Default::default(),
            };
        }

        Default::default()
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
            .children(content.icon.map(|icon| svg().path(icon)))
            .child(Label::new(SharedString::from(content.message)).size(LabelSize::Small))
    }
}

impl StatusItemView for ActivityIndicator {
    fn set_active_pane_item(&mut self, _: Option<&dyn ItemHandle>, _: &mut ViewContext<Self>) {}
}
