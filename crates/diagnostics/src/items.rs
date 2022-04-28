use editor::{Editor, GoToNextDiagnostic};
use gpui::{
    elements::*, platform::CursorStyle, serde_json, Entity, ModelHandle, MutableAppContext,
    RenderContext, Subscription, View, ViewContext, ViewHandle, WeakViewHandle,
};
use language::Diagnostic;
use project::Project;
use settings::Settings;
use workspace::StatusItemView;

pub struct DiagnosticIndicator {
    summary: project::DiagnosticSummary,
    active_editor: Option<WeakViewHandle<Editor>>,
    current_diagnostic: Option<Diagnostic>,
    check_in_progress: bool,
    _observe_active_editor: Option<Subscription>,
}

pub fn init(cx: &mut MutableAppContext) {
    cx.add_action(DiagnosticIndicator::go_to_next_diagnostic);
}

impl DiagnosticIndicator {
    pub fn new(project: &ModelHandle<Project>, cx: &mut ViewContext<Self>) -> Self {
        cx.subscribe(project, |this, project, event, cx| match event {
            project::Event::DiskBasedDiagnosticsUpdated => {
                cx.notify();
            }
            project::Event::DiskBasedDiagnosticsStarted => {
                this.check_in_progress = true;
                cx.notify();
            }
            project::Event::DiskBasedDiagnosticsFinished => {
                this.summary = project.read(cx).diagnostic_summary(cx);
                this.check_in_progress = false;
                cx.notify();
            }
            _ => {}
        })
        .detach();
        Self {
            summary: project.read(cx).diagnostic_summary(cx),
            check_in_progress: project.read(cx).is_running_disk_based_diagnostics(),
            active_editor: None,
            current_diagnostic: None,
            _observe_active_editor: None,
        }
    }

    fn go_to_next_diagnostic(&mut self, _: &GoToNextDiagnostic, cx: &mut ViewContext<Self>) {
        if let Some(editor) = self.active_editor.as_ref().and_then(|e| e.upgrade(cx)) {
            editor.update(cx, |editor, cx| {
                editor.go_to_diagnostic(editor::Direction::Next, cx);
            })
        }
    }

    fn update(&mut self, editor: ViewHandle<Editor>, cx: &mut ViewContext<Self>) {
        let editor = editor.read(cx);
        let buffer = editor.buffer().read(cx);
        let cursor_position = editor
            .newest_selection_with_snapshot::<usize>(&buffer.read(cx))
            .head();
        let new_diagnostic = buffer
            .read(cx)
            .diagnostics_in_range::<_, usize>(cursor_position..cursor_position, false)
            .filter(|entry| !entry.range.is_empty())
            .min_by_key(|entry| (entry.diagnostic.severity, entry.range.len()))
            .map(|entry| entry.diagnostic);
        if new_diagnostic != self.current_diagnostic {
            self.current_diagnostic = new_diagnostic;
            cx.notify();
        }
    }
}

impl Entity for DiagnosticIndicator {
    type Event = ();
}

impl View for DiagnosticIndicator {
    fn ui_name() -> &'static str {
        "DiagnosticIndicator"
    }

    fn render(&mut self, cx: &mut RenderContext<Self>) -> ElementBox {
        enum Summary {}
        enum Message {}

        let in_progress = self.check_in_progress;
        let mut element = Flex::row().with_child(
            MouseEventHandler::new::<Summary, _, _>(0, cx, |state, cx| {
                let style = &cx
                    .global::<Settings>()
                    .theme
                    .workspace
                    .status_bar
                    .diagnostics;

                let summary_style = if self.summary.error_count > 0 {
                    &style.summary_error
                } else if self.summary.warning_count > 0 {
                    &style.summary_warning
                } else {
                    &style.summary_ok
                };

                let summary_style = if state.hovered {
                    summary_style.hover()
                } else {
                    &summary_style.default
                };

                let mut summary_row = Flex::row();
                if self.summary.error_count > 0 {
                    summary_row.add_children([
                        Svg::new("icons/error-solid-14.svg")
                            .with_color(style.icon_color_error)
                            .constrained()
                            .with_width(style.icon_width)
                            .aligned()
                            .contained()
                            .with_margin_right(style.icon_spacing)
                            .named("error-icon"),
                        Label::new(
                            self.summary.error_count.to_string(),
                            summary_style.text.clone(),
                        )
                        .aligned()
                        .boxed(),
                    ]);
                }

                if self.summary.warning_count > 0 {
                    summary_row.add_children([
                        Svg::new("icons/warning-solid-14.svg")
                            .with_color(style.icon_color_warning)
                            .constrained()
                            .with_width(style.icon_width)
                            .aligned()
                            .contained()
                            .with_margin_right(style.icon_spacing)
                            .with_margin_left(if self.summary.error_count > 0 {
                                style.summary_spacing
                            } else {
                                0.
                            })
                            .named("warning-icon"),
                        Label::new(
                            self.summary.warning_count.to_string(),
                            summary_style.text.clone(),
                        )
                        .aligned()
                        .boxed(),
                    ]);
                }

                if self.summary.error_count == 0 && self.summary.warning_count == 0 {
                    summary_row.add_child(
                        Svg::new("icons/no-error-solid-14.svg")
                            .with_color(style.icon_color_ok)
                            .constrained()
                            .with_width(style.icon_width)
                            .aligned()
                            .named("ok-icon"),
                    );
                }

                summary_row
                    .constrained()
                    .with_height(style.height)
                    .contained()
                    .with_style(summary_style.container)
                    .boxed()
            })
            .with_cursor_style(CursorStyle::PointingHand)
            .on_click(|cx| cx.dispatch_action(crate::Deploy))
            .aligned()
            .boxed(),
        );

        let style = &cx.global::<Settings>().theme.workspace.status_bar;
        let item_spacing = style.item_spacing;
        let message_style = &style.diagnostics.message;

        if in_progress {
            element.add_child(
                Label::new("checking…".into(), message_style.default.text.clone())
                    .aligned()
                    .contained()
                    .with_margin_left(item_spacing)
                    .boxed(),
            );
        } else if let Some(diagnostic) = &self.current_diagnostic {
            let message_style = message_style.clone();
            element.add_child(
                MouseEventHandler::new::<Message, _, _>(0, cx, |state, _| {
                    Label::new(
                        diagnostic.message.split('\n').next().unwrap().to_string(),
                        if state.hovered {
                            message_style.hover().text.clone()
                        } else {
                            message_style.default.text.clone()
                        },
                    )
                    .aligned()
                    .contained()
                    .with_margin_left(item_spacing)
                    .boxed()
                })
                .with_cursor_style(CursorStyle::PointingHand)
                .on_click(|cx| cx.dispatch_action(GoToNextDiagnostic))
                .boxed(),
            );
        }

        element.named("diagnostic indicator")
    }

    fn debug_json(&self, _: &gpui::AppContext) -> serde_json::Value {
        serde_json::json!({ "summary": self.summary })
    }
}

impl StatusItemView for DiagnosticIndicator {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn workspace::ItemHandle>,
        cx: &mut ViewContext<Self>,
    ) {
        if let Some(editor) = active_pane_item.and_then(|item| item.downcast::<Editor>()) {
            self.active_editor = Some(editor.downgrade());
            self._observe_active_editor = Some(cx.observe(&editor, Self::update));
            self.update(editor, cx);
        } else {
            self.active_editor = None;
            self.current_diagnostic = None;
            self._observe_active_editor = None;
        }
        cx.notify();
    }
}
