use std::time::Duration;

use editor::Editor;
use gpui::{
    Context, Entity, EventEmitter, IntoElement, ParentElement, Render, Styled, Subscription, Task,
    WeakEntity, Window,
};
use language::Diagnostic;
use project::project_settings::{GoToDiagnosticSeverityFilter, ProjectSettings};
use settings::Settings;
use ui::{Button, ButtonLike, Color, Icon, IconName, Label, Tooltip, h_flex, prelude::*};
use util::ResultExt;
use workspace::{StatusItemView, ToolbarItemEvent, Workspace, item::ItemHandle};

use crate::{Deploy, IncludeWarnings, ProjectDiagnosticsEditor};

/// The status bar item that displays diagnostic counts.
pub struct DiagnosticIndicator {
    summary: project::DiagnosticSummary,
    workspace: WeakEntity<Workspace>,
    current_diagnostic: Option<Diagnostic>,
    active_editor: Option<WeakEntity<Editor>>,
    _observe_active_editor: Option<Subscription>,

    diagnostics_update: Task<()>,
    diagnostic_summary_update: Task<()>,
}

impl Render for DiagnosticIndicator {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let indicator = h_flex().gap_2();
        if !ProjectSettings::get_global(cx).diagnostics.button {
            return indicator.hidden();
        }

        let diagnostic_indicator = match (self.summary.error_count, self.summary.warning_count) {
            (0, 0) => h_flex().child(
                Icon::new(IconName::Check)
                    .size(IconSize::Small)
                    .color(Color::Default),
            ),
            (error_count, warning_count) => h_flex()
                .gap_1()
                .when(error_count > 0, |this| {
                    this.child(
                        Icon::new(IconName::XCircle)
                            .size(IconSize::Small)
                            .color(Color::Error),
                    )
                    .child(Label::new(error_count.to_string()).size(LabelSize::Small))
                })
                .when(warning_count > 0, |this| {
                    this.child(
                        Icon::new(IconName::Warning)
                            .size(IconSize::Small)
                            .color(Color::Warning),
                    )
                    .child(Label::new(warning_count.to_string()).size(LabelSize::Small))
                }),
        };

        let status = if let Some(diagnostic) = &self.current_diagnostic {
            let message = diagnostic
                .message
                .split_once('\n')
                .map_or(&*diagnostic.message, |(first, _)| first);
            Some(
                Button::new("diagnostic_message", SharedString::new(message))
                    .label_size(LabelSize::Small)
                    .tooltip(|_window, cx| {
                        Tooltip::for_action(
                            "Next Diagnostic",
                            &editor::actions::GoToDiagnostic::default(),
                            cx,
                        )
                    })
                    .on_click(
                        cx.listener(|this, _, window, cx| this.go_to_next_diagnostic(window, cx)),
                    ),
            )
        } else {
            None
        };

        indicator
            .child(
                ButtonLike::new("diagnostic-indicator")
                    .child(diagnostic_indicator)
                    .tooltip(move |_window, cx| {
                        Tooltip::for_action("Project Diagnostics", &Deploy, cx)
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
                this.diagnostic_summary_update = cx.spawn(async move |this, cx| {
                    cx.background_executor()
                        .timer(Duration::from_millis(30))
                        .await;
                    this.update(cx, |this, cx| {
                        this.summary = project.read(cx).diagnostic_summary(false, cx);
                        cx.notify();
                    })
                    .log_err();
                });
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
            diagnostic_summary_update: Task::ready(()),
        }
    }

    fn go_to_next_diagnostic(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(editor) = self.active_editor.as_ref().and_then(|e| e.upgrade()) {
            editor.update(cx, |editor, cx| {
                editor.go_to_diagnostic_impl(
                    editor::Direction::Next,
                    GoToDiagnosticSeverityFilter::default(),
                    window,
                    cx,
                );
            })
        }
    }

    fn update(&mut self, editor: Entity<Editor>, window: &mut Window, cx: &mut Context<Self>) {
        let (buffer, cursor_position) = editor.update(cx, |editor, cx| {
            let buffer = editor.buffer().read(cx).snapshot(cx);
            let cursor_position = editor
                .selections
                .newest::<usize>(&editor.display_snapshot(cx))
                .head();
            (buffer, cursor_position)
        });
        let new_diagnostic = buffer
            .diagnostics_in_range::<usize>(cursor_position..cursor_position)
            .filter(|entry| !entry.range.is_empty())
            .min_by_key(|entry| (entry.diagnostic.severity, entry.range.len()))
            .map(|entry| entry.diagnostic);
        if new_diagnostic != self.current_diagnostic.as_ref() {
            let new_diagnostic = new_diagnostic.cloned();
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
