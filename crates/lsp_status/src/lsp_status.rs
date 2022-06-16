use editor::Editor;
use futures::StreamExt;
use gpui::{
    actions, elements::*, platform::CursorStyle, AppContext, Entity, EventContext, ModelHandle,
    MutableAppContext, RenderContext, View, ViewContext, ViewHandle,
};
use language::{LanguageRegistry, LanguageServerBinaryStatus};
use project::{LanguageServerProgress, Project};
use settings::Settings;
use smallvec::SmallVec;
use std::{cmp::Reverse, fmt::Write, sync::Arc};
use util::ResultExt;
use workspace::{ItemHandle, StatusItemView, Workspace};

actions!(lsp_status, [ShowErrorMessage]);

pub enum Event {
    ShowError { lsp_name: Arc<str>, error: String },
}

pub struct LspStatusItem {
    statuses: Vec<LspStatus>,
    project: ModelHandle<Project>,
}

struct LspStatus {
    name: Arc<str>,
    status: LanguageServerBinaryStatus,
}

pub fn init(cx: &mut MutableAppContext) {
    cx.add_action(LspStatusItem::show_error_message);
}

impl LspStatusItem {
    pub fn new(
        workspace: &mut Workspace,
        languages: Arc<LanguageRegistry>,
        cx: &mut ViewContext<Workspace>,
    ) -> ViewHandle<LspStatusItem> {
        let project = workspace.project().clone();
        let this = cx.add_view(|cx: &mut ViewContext<Self>| {
            let mut status_events = languages.language_server_binary_statuses();
            cx.spawn_weak(|this, mut cx| async move {
                while let Some((language, event)) = status_events.next().await {
                    if let Some(this) = this.upgrade(&cx) {
                        this.update(&mut cx, |this, cx| {
                            this.statuses.retain(|s| s.name != language.name());
                            this.statuses.push(LspStatus {
                                name: language.name(),
                                status: event,
                            });
                            cx.notify();
                        });
                    } else {
                        break;
                    }
                }
            })
            .detach();
            cx.observe(&project, |_, _, cx| cx.notify()).detach();

            Self {
                statuses: Default::default(),
                project: project.clone(),
            }
        });
        cx.subscribe(&this, move |workspace, _, event, cx| match event {
            Event::ShowError { lsp_name, error } => {
                if let Some(buffer) = project
                    .update(cx, |project, cx| project.create_buffer(&error, None, cx))
                    .log_err()
                {
                    buffer.update(cx, |buffer, cx| {
                        buffer.edit(
                            [(0..0, format!("Language server error: {}\n\n", lsp_name))],
                            cx,
                        );
                    });
                    workspace.add_item(
                        Box::new(
                            cx.add_view(|cx| Editor::for_buffer(buffer, Some(project.clone()), cx)),
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

impl Entity for LspStatusItem {
    type Event = Event;
}

impl View for LspStatusItem {
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

            let mut downloading = SmallVec::<[_; 3]>::new();
            let mut checking_for_update = SmallVec::<[_; 3]>::new();
            let mut failed = SmallVec::<[_; 3]>::new();
            for status in &self.statuses {
                match status.status {
                    LanguageServerBinaryStatus::CheckingForUpdate => {
                        checking_for_update.push(status.name.clone());
                    }
                    LanguageServerBinaryStatus::Downloading => {
                        downloading.push(status.name.clone());
                    }
                    LanguageServerBinaryStatus::Failed { .. } => {
                        failed.push(status.name.clone());
                    }
                    LanguageServerBinaryStatus::Downloaded | LanguageServerBinaryStatus::Cached => {
                    }
                }
            }

            if !downloading.is_empty() {
                icon = Some("icons/download-solid-14.svg");
                message = format!(
                    "Downloading {} language server{}...",
                    downloading.join(", "),
                    if downloading.len() > 1 { "s" } else { "" }
                );
            } else if !checking_for_update.is_empty() {
                icon = Some("icons/download-solid-14.svg");
                message = format!(
                    "Checking for updates to {} language server{}...",
                    checking_for_update.join(", "),
                    if checking_for_update.len() > 1 {
                        "s"
                    } else {
                        ""
                    }
                );
            } else if !failed.is_empty() {
                icon = Some("icons/warning-solid-14.svg");
                message = format!(
                    "Failed to download {} language server{}. Click to show error.",
                    failed.join(", "),
                    if failed.len() > 1 { "s" } else { "" }
                );
                handler = Some(|_, _, cx: &mut EventContext| cx.dispatch_action(ShowErrorMessage));
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

impl StatusItemView for LspStatusItem {
    fn set_active_pane_item(&mut self, _: Option<&dyn ItemHandle>, _: &mut ViewContext<Self>) {}
}
