use crate::{ItemViewHandle, Settings, StatusItemView};
use futures::StreamExt;
use gpui::{
    action, elements::*, platform::CursorStyle, Entity, ModelHandle, MutableAppContext,
    RenderContext, View, ViewContext,
};
use language::{LanguageRegistry, LanguageServerBinaryStatus};
use postage::watch;
use project::Project;
use std::fmt::Write;
use std::sync::Arc;

action!(DismissErrorMessage);

pub struct LspStatus {
    settings_rx: watch::Receiver<Settings>,
    checking_for_update: Vec<String>,
    downloading: Vec<String>,
    failed: Vec<String>,
    project: ModelHandle<Project>,
}

pub fn init(cx: &mut MutableAppContext) {
    cx.add_action(LspStatus::dismiss_error_message);
}

impl LspStatus {
    pub fn new(
        project: &ModelHandle<Project>,
        languages: Arc<LanguageRegistry>,
        settings_rx: watch::Receiver<Settings>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let mut status_events = languages.language_server_binary_statuses();
        cx.spawn_weak(|this, mut cx| async move {
            while let Some((language, event)) = status_events.next().await {
                if let Some(this) = this.upgrade(&cx) {
                    this.update(&mut cx, |this, cx| {
                        for vector in [
                            &mut this.checking_for_update,
                            &mut this.downloading,
                            &mut this.failed,
                        ] {
                            vector.retain(|name| name != language.name().as_ref());
                        }

                        match event {
                            LanguageServerBinaryStatus::CheckingForUpdate => {
                                this.checking_for_update.push(language.name().to_string());
                            }
                            LanguageServerBinaryStatus::Downloading => {
                                this.downloading.push(language.name().to_string());
                            }
                            LanguageServerBinaryStatus::Failed => {
                                this.failed.push(language.name().to_string());
                            }
                            LanguageServerBinaryStatus::Downloaded
                            | LanguageServerBinaryStatus::Cached => {}
                        }

                        cx.notify();
                    });
                } else {
                    break;
                }
            }
        })
        .detach();
        cx.observe(project, |_, _, cx| cx.notify()).detach();

        Self {
            settings_rx,
            checking_for_update: Default::default(),
            downloading: Default::default(),
            failed: Default::default(),
            project: project.clone(),
        }
    }

    fn dismiss_error_message(&mut self, _: &DismissErrorMessage, cx: &mut ViewContext<Self>) {
        self.failed.clear();
        cx.notify();
    }
}

impl Entity for LspStatus {
    type Event = ();
}

impl View for LspStatus {
    fn ui_name() -> &'static str {
        "LspStatus"
    }

    fn render(&mut self, cx: &mut RenderContext<Self>) -> ElementBox {
        let theme = &self.settings_rx.borrow().theme;

        let mut pending_work = self.project.read(cx).pending_language_server_work();
        if let Some((lang_server_name, progress_token, progress)) = pending_work.next() {
            let mut message = lang_server_name.to_string();

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

            Label::new(message, theme.workspace.status_bar.lsp_message.clone()).boxed()
        } else if !self.downloading.is_empty() {
            Label::new(
                format!(
                    "Downloading {} language server{}...",
                    self.downloading.join(", "),
                    if self.downloading.len() > 1 { "s" } else { "" }
                ),
                theme.workspace.status_bar.lsp_message.clone(),
            )
            .boxed()
        } else if !self.checking_for_update.is_empty() {
            Label::new(
                format!(
                    "Checking for updates to {} language server{}...",
                    self.checking_for_update.join(", "),
                    if self.checking_for_update.len() > 1 {
                        "s"
                    } else {
                        ""
                    }
                ),
                theme.workspace.status_bar.lsp_message.clone(),
            )
            .boxed()
        } else if !self.failed.is_empty() {
            drop(pending_work);
            MouseEventHandler::new::<Self, _, _>(0, cx, |_, _| {
                Label::new(
                    format!(
                        "Failed to download {} language server{}. Click to dismiss.",
                        self.failed.join(", "),
                        if self.failed.len() > 1 { "s" } else { "" }
                    ),
                    theme.workspace.status_bar.lsp_message.clone(),
                )
                .boxed()
            })
            .with_cursor_style(CursorStyle::PointingHand)
            .on_click(|cx| cx.dispatch_action(DismissErrorMessage))
            .boxed()
        } else {
            Empty::new().boxed()
        }
    }
}

impl StatusItemView for LspStatus {
    fn set_active_pane_item(&mut self, _: Option<&dyn ItemViewHandle>, _: &mut ViewContext<Self>) {}
}
