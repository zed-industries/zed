use crate::{ItemHandle, StatusItemView};
use futures::StreamExt;
use gpui::{actions, AppContext, EventContext};
use gpui::{
    elements::*, platform::CursorStyle, Entity, ModelHandle, MutableAppContext, RenderContext,
    View, ViewContext,
};
use language::{LanguageRegistry, LanguageServerBinaryStatus};
use project::{LanguageServerProgress, Project};
use settings::Settings;
use smallvec::SmallVec;
use std::cmp::Reverse;
use std::fmt::Write;
use std::sync::Arc;

actions!(lsp_status, [DismissErrorMessage]);

pub struct LspStatus {
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

    fn pending_language_server_work<'a>(
        &self,
        cx: &'a AppContext,
    ) -> impl Iterator<Item = (&'a str, &'a str, &'a LanguageServerProgress)> {
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
                        .map(|(token, progress)| (status.name.as_str(), token.as_str(), progress))
                        .collect::<SmallVec<[_; 4]>>();
                    pending_work.sort_by_key(|(_, _, progress)| Reverse(progress.last_update_at));
                    Some(pending_work)
                }
            })
            .flatten()
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
        let mut message;
        let mut icon = None;
        let mut handler = None;

        let mut pending_work = self.pending_language_server_work(cx);
        if let Some((lang_server_name, progress_token, progress)) = pending_work.next() {
            message = lang_server_name.to_string();

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
        } else {
            drop(pending_work);

            if !self.downloading.is_empty() {
                icon = Some("icons/download-solid-14.svg");
                message = format!(
                    "Downloading {} language server{}...",
                    self.downloading.join(", "),
                    if self.downloading.len() > 1 { "s" } else { "" }
                );
            } else if !self.checking_for_update.is_empty() {
                icon = Some("icons/download-solid-14.svg");
                message = format!(
                    "Checking for updates to {} language server{}...",
                    self.checking_for_update.join(", "),
                    if self.checking_for_update.len() > 1 {
                        "s"
                    } else {
                        ""
                    }
                );
            } else if !self.failed.is_empty() {
                icon = Some("icons/warning-solid-14.svg");
                message = format!(
                    "Failed to download {} language server{}. Click to dismiss.",
                    self.failed.join(", "),
                    if self.failed.len() > 1 { "s" } else { "" }
                );
                handler = Some(|_, cx: &mut EventContext| cx.dispatch_action(DismissErrorMessage));
            } else {
                return Empty::new().boxed();
            }
        }

        let mut element = MouseEventHandler::new::<Self, _, _>(0, cx, |state, cx| {
            let theme = &cx
                .global::<Settings>()
                .theme
                .workspace
                .status_bar
                .lsp_status;
            let style = if state.hovered && handler.is_some() {
                theme.hover.as_ref().unwrap_or(&theme.default)
            } else {
                &theme.default
            };
            Flex::row()
                .with_children(icon.map(|path| {
                    Svg::new(path)
                        .with_color(style.icon_color)
                        .constrained()
                        .with_width(style.icon_width)
                        .contained()
                        .with_margin_right(style.icon_spacing)
                        .aligned()
                        .named("warning-icon")
                }))
                .with_child(Label::new(message, style.message.clone()).aligned().boxed())
                .constrained()
                .with_height(style.height)
                .contained()
                .with_style(style.container)
                .aligned()
                .boxed()
        });

        if let Some(handler) = handler {
            element = element
                .with_cursor_style(CursorStyle::PointingHand)
                .on_click(handler);
        }

        element.boxed()
    }
}

impl StatusItemView for LspStatus {
    fn set_active_pane_item(&mut self, _: Option<&dyn ItemHandle>, _: &mut ViewContext<Self>) {}
}
