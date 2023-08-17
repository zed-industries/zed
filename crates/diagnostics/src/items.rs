use collections::HashSet;
use editor::{Editor, GoToDiagnostic};
use gpui::{
    elements::*,
    platform::{CursorStyle, MouseButton},
    serde_json, AppContext, Entity, Subscription, View, ViewContext, ViewHandle, WeakViewHandle,
};
use language::Diagnostic;
use lsp::LanguageServerId;
use workspace::{item::ItemHandle, StatusItemView, Workspace};

use crate::ProjectDiagnosticsEditor;

pub struct DiagnosticIndicator {
    summary: project::DiagnosticSummary,
    active_editor: Option<WeakViewHandle<Editor>>,
    workspace: WeakViewHandle<Workspace>,
    current_diagnostic: Option<Diagnostic>,
    in_progress_checks: HashSet<LanguageServerId>,
    _observe_active_editor: Option<Subscription>,
}

pub fn init(cx: &mut AppContext) {
    cx.add_action(DiagnosticIndicator::go_to_next_diagnostic);
}

impl DiagnosticIndicator {
    pub fn new(workspace: &Workspace, cx: &mut ViewContext<Self>) -> Self {
        let project = workspace.project();
        cx.subscribe(project, |this, project, event, cx| match event {
            project::Event::DiskBasedDiagnosticsStarted { language_server_id } => {
                this.in_progress_checks.insert(*language_server_id);
                cx.notify();
            }
            project::Event::DiskBasedDiagnosticsFinished { language_server_id } => {
                this.summary = project.read(cx).diagnostic_summary(cx);
                this.in_progress_checks.remove(language_server_id);
                cx.notify();
            }
            _ => {}
        })
        .detach();
        Self {
            summary: project.read(cx).diagnostic_summary(cx),
            in_progress_checks: project
                .read(cx)
                .language_servers_running_disk_based_diagnostics()
                .collect(),
            active_editor: None,
            workspace: workspace.weak_handle(),
            current_diagnostic: None,
            _observe_active_editor: None,
        }
    }

    fn go_to_next_diagnostic(&mut self, _: &GoToDiagnostic, cx: &mut ViewContext<Self>) {
        if let Some(editor) = self.active_editor.as_ref().and_then(|e| e.upgrade(cx)) {
            editor.update(cx, |editor, cx| {
                editor.go_to_diagnostic_impl(editor::Direction::Next, cx);
            })
        }
    }

    fn update(&mut self, editor: ViewHandle<Editor>, cx: &mut ViewContext<Self>) {
        let editor = editor.read(cx);
        let buffer = editor.buffer().read(cx);
        let cursor_position = editor.selections.newest::<usize>(cx).head();
        let new_diagnostic = buffer
            .snapshot(cx)
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

    fn render(&mut self, cx: &mut ViewContext<Self>) -> AnyElement<Self> {
        enum Summary {}
        enum Message {}

        let tooltip_style = theme::current(cx).tooltip.clone();
        let in_progress = !self.in_progress_checks.is_empty();
        let mut element = Flex::row().with_child(
            MouseEventHandler::new::<Summary, _>(0, cx, |state, cx| {
                let theme = theme::current(cx);
                let style = theme
                    .workspace
                    .status_bar
                    .diagnostic_summary
                    .style_for(state);

                let mut summary_row = Flex::row();
                if self.summary.error_count > 0 {
                    summary_row.add_child(
                        Svg::new("icons/error.svg")
                            .with_color(style.icon_color_error)
                            .constrained()
                            .with_width(style.icon_width)
                            .aligned()
                            .contained()
                            .with_margin_right(style.icon_spacing),
                    );
                    summary_row.add_child(
                        Label::new(self.summary.error_count.to_string(), style.text.clone())
                            .aligned(),
                    );
                }

                if self.summary.warning_count > 0 {
                    summary_row.add_child(
                        Svg::new("icons/warning.svg")
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
                            }),
                    );
                    summary_row.add_child(
                        Label::new(self.summary.warning_count.to_string(), style.text.clone())
                            .aligned(),
                    );
                }

                if self.summary.error_count == 0 && self.summary.warning_count == 0 {
                    summary_row.add_child(
                        Svg::new("icons/check_circle.svg")
                            .with_color(style.icon_color_ok)
                            .constrained()
                            .with_width(style.icon_width)
                            .aligned()
                            .into_any_named("ok-icon"),
                    );
                }

                summary_row
                    .constrained()
                    .with_height(style.height)
                    .contained()
                    .with_style(if self.summary.error_count > 0 {
                        style.container_error
                    } else if self.summary.warning_count > 0 {
                        style.container_warning
                    } else {
                        style.container_ok
                    })
            })
            .with_cursor_style(CursorStyle::PointingHand)
            .on_click(MouseButton::Left, |_, this, cx| {
                if let Some(workspace) = this.workspace.upgrade(cx) {
                    workspace.update(cx, |workspace, cx| {
                        ProjectDiagnosticsEditor::deploy(workspace, &Default::default(), cx)
                    })
                }
            })
            .with_tooltip::<Summary>(
                0,
                "Project Diagnostics",
                Some(Box::new(crate::Deploy)),
                tooltip_style,
                cx,
            )
            .aligned()
            .into_any(),
        );

        let style = &theme::current(cx).workspace.status_bar;
        let item_spacing = style.item_spacing;

        if in_progress {
            element.add_child(
                Label::new("Checking…", style.diagnostic_message.default.text.clone())
                    .aligned()
                    .contained()
                    .with_margin_left(item_spacing),
            );
        } else if let Some(diagnostic) = &self.current_diagnostic {
            let message_style = style.diagnostic_message.clone();
            element.add_child(
                MouseEventHandler::new::<Message, _>(1, cx, |state, _| {
                    Label::new(
                        diagnostic.message.split('\n').next().unwrap().to_string(),
                        message_style.style_for(state).text.clone(),
                    )
                    .aligned()
                    .contained()
                    .with_margin_left(item_spacing)
                })
                .with_cursor_style(CursorStyle::PointingHand)
                .on_click(MouseButton::Left, |_, this, cx| {
                    this.go_to_next_diagnostic(&Default::default(), cx)
                }),
            );
        }

        element.into_any_named("diagnostic indicator")
    }

    fn debug_json(&self, _: &gpui::AppContext) -> serde_json::Value {
        serde_json::json!({ "summary": self.summary })
    }
}

impl StatusItemView for DiagnosticIndicator {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
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
