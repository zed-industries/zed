use std::time::Duration;

use collections::HashSet;
use editor::Editor;
use gpui::{
    percentage, rems, Animation, AnimationExt, EventEmitter, IntoElement, ParentElement, Render,
    Styled, Subscription, Transformation, View, ViewContext, WeakView,
};
use language::Diagnostic;
use lsp::LanguageServerId;
use ui::{h_flex, prelude::*, Button, ButtonLike, Color, Icon, IconName, Label, Tooltip};
use workspace::{item::ItemHandle, StatusItemView, ToolbarItemEvent, Workspace};

use crate::{Deploy, ProjectDiagnosticsEditor};

pub struct DiagnosticIndicator {
    summary: project::DiagnosticSummary,
    active_editor: Option<WeakView<Editor>>,
    workspace: WeakView<Workspace>,
    current_diagnostic: Option<Diagnostic>,
    in_progress_checks: HashSet<LanguageServerId>,
    _observe_active_editor: Option<Subscription>,
}

impl Render for DiagnosticIndicator {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
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
                    Icon::new(IconName::ExclamationTriangle)
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
                    Icon::new(IconName::ExclamationTriangle)
                        .size(IconSize::Small)
                        .color(Color::Warning),
                )
                .child(Label::new(warning_count.to_string()).size(LabelSize::Small)),
        };

        let status = if !self.in_progress_checks.is_empty() {
            Some(
                h_flex()
                    .gap_2()
                    .child(
                        Icon::new(IconName::ArrowCircle)
                            .size(IconSize::Small)
                            .with_animation(
                                "arrow-circle",
                                Animation::new(Duration::from_secs(2)).repeat(),
                                |icon, delta| {
                                    icon.transform(Transformation::rotate(percentage(delta)))
                                },
                            ),
                    )
                    .child(
                        Label::new("Checking…")
                            .size(LabelSize::Small)
                            .into_any_element(),
                    )
                    .into_any_element(),
            )
        } else if let Some(diagnostic) = &self.current_diagnostic {
            let message = diagnostic.message.split('\n').next().unwrap().to_string();
            Some(
                Button::new("diagnostic_message", message)
                    .label_size(LabelSize::Small)
                    .tooltip(|cx| {
                        Tooltip::for_action("Next Diagnostic", &editor::actions::GoToDiagnostic, cx)
                    })
                    .on_click(cx.listener(|this, _, cx| {
                        this.go_to_next_diagnostic(cx);
                    }))
                    .into_any_element(),
            )
        } else {
            None
        };

        h_flex()
            .h(rems(1.375))
            .gap_2()
            .child(
                ButtonLike::new("diagnostic-indicator")
                    .child(diagnostic_indicator)
                    .tooltip(|cx| Tooltip::for_action("Project Diagnostics", &Deploy, cx))
                    .on_click(cx.listener(|this, _, cx| {
                        if let Some(workspace) = this.workspace.upgrade() {
                            workspace.update(cx, |workspace, cx| {
                                ProjectDiagnosticsEditor::deploy(workspace, &Default::default(), cx)
                            })
                        }
                    })),
            )
            .children(status)
    }
}

impl DiagnosticIndicator {
    pub fn new(workspace: &Workspace, cx: &mut ViewContext<Self>) -> Self {
        let project = workspace.project();
        cx.subscribe(project, |this, project, event, cx| match event {
            project::Event::DiskBasedDiagnosticsStarted { language_server_id } => {
                this.in_progress_checks.insert(*language_server_id);
                cx.notify();
            }

            project::Event::DiskBasedDiagnosticsFinished { language_server_id }
            | project::Event::LanguageServerRemoved(language_server_id) => {
                this.summary = project.read(cx).diagnostic_summary(false, cx);
                this.in_progress_checks.remove(language_server_id);
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

    fn go_to_next_diagnostic(&mut self, cx: &mut ViewContext<Self>) {
        if let Some(editor) = self.active_editor.as_ref().and_then(|e| e.upgrade()) {
            editor.update(cx, |editor, cx| {
                editor.go_to_diagnostic_impl(editor::Direction::Next, cx);
            })
        }
    }

    fn update(&mut self, editor: View<Editor>, cx: &mut ViewContext<Self>) {
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

impl EventEmitter<ToolbarItemEvent> for DiagnosticIndicator {}

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
