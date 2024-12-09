use std::time::Duration;

use editor::Editor;
use gpui::{
    AppContext, EventEmitter, IntoElement, ParentElement, Render, Styled, Subscription, Task, View,
    WeakView,
};
use language::Diagnostic;
use ui::{h_flex, prelude::*, Button, ButtonLike, Color, Icon, IconName, Label, Tooltip};
use workspace::{item::ItemHandle, StatusItemView, ToolbarItemEvent, Workspace};

use crate::{Deploy, ProjectDiagnosticsEditor};

pub struct DiagnosticIndicator {
    summary: project::DiagnosticSummary,
    active_editor: Option<WeakModel<Editor>>,
    workspace: WeakModel<Workspace>,
    current_diagnostic: Option<Diagnostic>,
    _observe_active_editor: Option<Subscription>,
    diagnostics_update: Task<()>,
}

impl Render for DiagnosticIndicator {
    fn render(
        &mut self,
        model: &Model<Self>,
        window: &mut gpui::Window,
        cx: &mut AppContext,
    ) -> impl IntoElement {
        let diagnostic_indicator = match (self.summary.error_count, self.summary.warning_count) {
            (0, 0) => h_flex().map(|this| {
                this.child(
                    Icon::new(IconName::Check)
                        .size(IconSize::Small)
                        .color(Color::Default),
                )
            }),
            (0, warning_count) => h_flex()
                .gap_1()
                .child(
                    Icon::new(IconName::Warning)
                        .size(IconSize::Small)
                        .color(Color::Warning),
                )
                .child(Label::new(warning_count.to_string()).size(LabelSize::Small)),
            (error_count, 0) => h_flex()
                .gap_1()
                .child(
                    Icon::new(IconName::XCircle)
                        .size(IconSize::Small)
                        .color(Color::Error),
                )
                .child(Label::new(error_count.to_string()).size(LabelSize::Small)),
            (error_count, warning_count) => h_flex()
                .gap_1()
                .child(
                    Icon::new(IconName::XCircle)
                        .size(IconSize::Small)
                        .color(Color::Error),
                )
                .child(Label::new(error_count.to_string()).size(LabelSize::Small))
                .child(
                    Icon::new(IconName::Warning)
                        .size(IconSize::Small)
                        .color(Color::Warning),
                )
                .child(Label::new(warning_count.to_string()).size(LabelSize::Small)),
        };

        let status = if let Some(diagnostic) = &self.current_diagnostic {
            let message = diagnostic.message.split('\n').next().unwrap().to_string();
            Some(
                Button::new("diagnostic_message", message)
                    .label_size(LabelSize::Small)
                    .tooltip(|window, cx| {
                        Tooltip::for_action(
                            "Next Diagnostic",
                            &editor::actions::GoToDiagnostic,
                            model,
                            cx,
                        )
                    })
                    .on_click(model.listener(|this, model, _, cx| {
                        this.go_to_next_diagnostic(cx);
                    }))
                    .into_any_element(),
            )
        } else {
            None
        };

        h_flex()
            .gap_2()
            .pl_1()
            .border_l_1()
            .border_color(cx.theme().colors().border)
            .child(
                ButtonLike::new("diagnostic-indicator")
                    .child(diagnostic_indicator)
                    .tooltip(|window, cx| {
                        Tooltip::for_action("Project Diagnostics", &Deploy, model, cx)
                    })
                    .on_click(model.listener(|this, model, _, cx| {
                        if let Some(workspace) = this.workspace.upgrade() {
                            workspace.update(cx, |workspace, model, cx| {
                                ProjectDiagnosticsEditor::deploy(
                                    workspace,
                                    &Default::default(),
                                    model,
                                    cx,
                                )
                            })
                        }
                    })),
            )
            .children(status)
    }
}

impl DiagnosticIndicator {
    pub fn new(workspace: &Workspace, model: &Model<Self>, cx: &mut AppContext) -> Self {
        let project = workspace.project();
        cx.subscribe(project, |this, project, event, cx| match event {
            project::Event::DiskBasedDiagnosticsStarted { .. } => {
                model.notify(cx);
            }

            project::Event::DiskBasedDiagnosticsFinished { .. }
            | project::Event::LanguageServerRemoved(_) => {
                this.summary = project.read(cx).diagnostic_summary(false, cx);
                model.notify(cx);
            }

            project::Event::DiagnosticsUpdated { .. } => {
                this.summary = project.read(cx).diagnostic_summary(false, cx);
                model.notify(cx);
            }

            _ => {}
        })
        .detach();

        Self {
            summary: project.read(cx).diagnostic_summary(false, cx),
            active_editor: None,
            workspace: workspace.weak_handle(),
            current_diagnostic: None,
            _observe_active_editor: None,
            diagnostics_update: Task::ready(()),
        }
    }

    fn go_to_next_diagnostic(&mut self, model: &Model<Self>, cx: &mut AppContext) {
        if let Some(editor) = self.active_editor.as_ref().and_then(|e| e.upgrade()) {
            editor.update(cx, |editor, model, cx| {
                editor.go_to_diagnostic_impl(editor::Direction::Next, cx);
            })
        }
    }

    fn update(&mut self, editor: Model<Editor>, model: &Model<Self>, cx: &mut AppContext) {
        let (buffer, cursor_position) = editor.update(cx, |editor, model, cx| {
            let buffer = editor.buffer().read(cx).snapshot(cx);
            let cursor_position = editor.selections.newest::<usize>(cx).head();
            (buffer, cursor_position)
        });
        let new_diagnostic = buffer
            .diagnostics_in_range::<_, usize>(cursor_position..cursor_position, false)
            .filter(|entry| !entry.range.is_empty())
            .min_by_key(|entry| (entry.diagnostic.severity, entry.range.len()))
            .map(|entry| entry.diagnostic);
        if new_diagnostic != self.current_diagnostic {
            self.diagnostics_update = cx.spawn(|diagnostics_indicator, mut cx| async move {
                cx.background_executor()
                    .timer(Duration::from_millis(50))
                    .await;
                diagnostics_indicator
                    .update(&mut cx, |diagnostics_indicator, cx| {
                        diagnostics_indicator.current_diagnostic = new_diagnostic;
                        model.notify(cx);
                    })
                    .ok();
            });
        }
    }
}

impl EventEmitter<ToolbarItemEvent> for DiagnosticIndicator {}

impl StatusItemView for DiagnosticIndicator {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        model: &Model<Self>,
        cx: &mut AppContext,
    ) {
        if let Some(editor) = active_pane_item.and_then(|item| item.downcast::<Editor>()) {
            self.active_editor = Some(editor.downgrade());
            self._observe_active_editor = Some(cx.observe(&editor, Self::update));
            self.update(editor, model, cx);
        } else {
            self.active_editor = None;
            self.current_diagnostic = None;
            self._observe_active_editor = None;
        }
        model.notify(cx);
    }
}
