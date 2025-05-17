use std::time::Duration;

use editor::Editor;
use gpui::{
    Context, Entity, EventEmitter, IntoElement, ParentElement, Render, Styled, Subscription, Task,
    WeakEntity, Window,
};
use language::Diagnostic;
use project::project_settings::ProjectSettings;
use settings::Settings;
use ui::{Button, ButtonLike, Color, Icon, IconName, Label, Tooltip, h_flex, prelude::*};
use workspace::{StatusItemView, ToolbarItemEvent, Workspace, item::ItemHandle};

use crate::{Deploy, IncludeWarnings, ProjectDiagnosticsEditor};

pub struct DiagnosticIndicator {
    summary: project::DiagnosticSummary,
    active_editor: Option<WeakEntity<Editor>>,
    workspace: WeakEntity<Workspace>,
    current_diagnostic: Option<Diagnostic>,
    _observe_active_editor: Option<Subscription>,
    diagnostics_update: Task<()>,
}

impl Render for DiagnosticIndicator {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let indicator = h_flex().gap_2();
        if !ProjectSettings::get_global(cx).diagnostics.button {
            return indicator;
        }

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
                            window,
                            cx,
                        )
                    })
                    .on_click(cx.listener(|this, _, window, cx| {
                        this.go_to_next_diagnostic(window, cx);
                    }))
                    .into_any_element(),
            )
        } else {
            None
        };

        indicator
            .child(
                ButtonLike::new("diagnostic-indicator")
                    .child(diagnostic_indicator)
                    .tooltip(|window, cx| {
                        Tooltip::for_action("Project Diagnostics", &Deploy, window, cx)
                    })
                    .on_click(cx.listener(|this, _, window, cx| {
                        if let Some(workspace) = this.workspace.upgrade() {
                            if this.summary.error_count == 0 && this.summary.warning_count > 0 {
                                cx.update_default_global(
                                    |show_warnings: &mut IncludeWarnings, _| show_warnings.0 = true,
                                );
                            }
                            workspace.update(cx, |workspace, cx| {
                                ProjectDiagnosticsEditor::deploy(
                                    workspace,
                                    &Default::default(),
                                    window,
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
    pub fn new(workspace: &Workspace, cx: &mut Context<Self>) -> Self {
        let project = workspace.project();
        cx.subscribe(project, |this, project, event, cx| match event {
            project::Event::DiskBasedDiagnosticsStarted { .. } => {
                cx.notify();
            }

            project::Event::DiskBasedDiagnosticsFinished { .. }
            | project::Event::LanguageServerRemoved(_) => {
                this.summary = project.read(cx).diagnostic_summary(false, cx);
                cx.notify();
            }

            project::Event::DiagnosticsUpdated { .. } => {
                this.summary = project.read(cx).diagnostic_summary(false, cx);
                cx.notify();
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

    fn go_to_next_diagnostic(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(editor) = self.active_editor.as_ref().and_then(|e| e.upgrade()) {
            editor.update(cx, |editor, cx| {
                editor.go_to_diagnostic_impl(editor::Direction::Next, window, cx);
            })
        }
    }

    fn update(&mut self, editor: Entity<Editor>, window: &mut Window, cx: &mut Context<Self>) {
        let (buffer, cursor_position) = editor.update(cx, |editor, cx| {
            let buffer = editor.buffer().read(cx).snapshot(cx);
            let cursor_position = editor
                .selections
                .newest::<usize>(&editor.selections.display_map(cx))
                .head();
            (buffer, cursor_position)
        });
        let new_diagnostic = buffer
            .diagnostics_in_range::<usize>(cursor_position..cursor_position)
            .filter(|entry| !entry.range.is_empty())
            .min_by_key(|entry| (entry.diagnostic.severity, entry.range.len()))
            .map(|entry| entry.diagnostic);
        if new_diagnostic != self.current_diagnostic {
            self.diagnostics_update =
                cx.spawn_in(window, async move |diagnostics_indicator, cx| {
                    cx.background_executor()
                        .timer(Duration::from_millis(50))
                        .await;
                    diagnostics_indicator
                        .update(cx, |diagnostics_indicator, cx| {
                            diagnostics_indicator.current_diagnostic = new_diagnostic;
                            cx.notify();
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
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(editor) = active_pane_item.and_then(|item| item.downcast::<Editor>()) {
            self.active_editor = Some(editor.downgrade());
            self._observe_active_editor = Some(cx.observe_in(&editor, window, Self::update));
            self.update(editor, window, cx);
        } else {
            self.active_editor = None;
            self.current_diagnostic = None;
            self._observe_active_editor = None;
        }
        cx.notify();
    }
}
