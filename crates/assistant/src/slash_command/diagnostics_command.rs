use super::{SlashCommand, SlashCommandOutput};
use anyhow::{anyhow, Result};
use assistant_slash_command::SlashCommandOutputSection;
use gpui::{svg, AppContext, Model, RenderOnce, Task, WeakView};
use language::{BufferSnapshot, DiagnosticSeverity, LspAdapterDelegate};
use project::{DiagnosticSummary, Project};
use std::{
    ops::Range,
    sync::{atomic::AtomicBool, Arc},
};
use ui::{prelude::*, ButtonLike, ElevationIndex};
use util::ResultExt;
use workspace::Workspace;

pub(crate) struct DiagnosticsCommand;

impl SlashCommand for DiagnosticsCommand {
    fn name(&self) -> String {
        "diagnostics".into()
    }

    fn description(&self) -> String {
        "Insert diagnostics".into()
    }

    fn menu_text(&self) -> String {
        "Insert Diagnostics".into()
    }

    fn requires_argument(&self) -> bool {
        false
    }

    fn complete_argument(
        &self,
        _query: String,
        _cancellation_flag: Arc<AtomicBool>,
        _workspace: Option<WeakView<Workspace>>,
        _cx: &mut AppContext,
    ) -> Task<Result<Vec<String>>> {
        Task::ready(Err(anyhow!("this command does not require argument")))
    }

    fn run(
        self: Arc<Self>,
        _argument: Option<&str>,
        workspace: WeakView<Workspace>,
        _delegate: Arc<dyn LspAdapterDelegate>,
        cx: &mut WindowContext,
    ) -> Task<Result<SlashCommandOutput>> {
        let Some(workspace) = workspace.upgrade() else {
            return Task::ready(Err(anyhow!("workspace was dropped")));
        };

        let task = collect_diagnostics(workspace.read(cx).project().clone(), cx);
        cx.spawn(move |_| async move {
            let (text, sections) = task.await?;
            Ok(SlashCommandOutput {
                text,
                sections: sections
                    .into_iter()
                    .map(|(range, placeholder_type)| SlashCommandOutputSection {
                        range,
                        render_placeholder: Arc::new(move |id, unfold, _cx| {
                            DiagnosticsPlaceholder {
                                id,
                                unfold,
                                placeholder_type: placeholder_type.clone(),
                            }
                            .into_any_element()
                        }),
                    })
                    .collect(),
                run_commands_in_text: false,
            })
        })
    }
}

fn collect_diagnostics(
    project: Model<Project>,
    cx: &mut AppContext,
) -> Task<Result<(String, Vec<(Range<usize>, PlaceholderType)>)>> {
    let project_handle = project.downgrade();
    let diagnostic_summaries: Vec<_> = project.read(cx).diagnostic_summaries(false, cx).collect();

    cx.spawn(|mut cx| async move {
        let mut text = String::new();
        text.push_str("Diagnostics:\n");
        let mut sections: Vec<(Range<usize>, PlaceholderType)> = Vec::new();

        let mut project_summary = DiagnosticSummary::default();
        for (project_path, _, summary) in diagnostic_summaries {
            project_summary.error_count += summary.error_count;
            project_summary.warning_count += summary.warning_count;

            let last_end = text.len();
            let file_path = project_path.path.to_string_lossy().to_string();
            text.push_str(&file_path);
            text.push('\n');

            if let Some(buffer) = project_handle
                .update(&mut cx, |project, cx| project.open_buffer(project_path, cx))?
                .await
                .log_err()
            {
                collect_buffer_diagnostics(
                    &mut text,
                    &mut sections,
                    cx.read_model(&buffer, |buffer, _| buffer.snapshot())?,
                );
            }

            sections.push((
                last_end..text.len().saturating_sub(1),
                PlaceholderType::File(file_path),
            ))
        }
        sections.push((0..text.len(), PlaceholderType::Root(project_summary)));

        Ok((text, sections))
    })
}

fn collect_buffer_diagnostics(
    text: &mut String,
    sections: &mut Vec<(Range<usize>, PlaceholderType)>,
    snapshot: BufferSnapshot,
) {
    for (_, group) in snapshot.diagnostic_groups(None) {
        //TODO Find to link related diagnostics together (primary diagnostic)
        for entry in group.entries {
            let ty = match entry.diagnostic.severity {
                DiagnosticSeverity::WARNING => DiagnosticType::Warning,
                DiagnosticSeverity::ERROR => DiagnosticType::Error,
                _ => continue,
            };
            let prev_len = text.len();
            text.push_str(&entry.diagnostic.message);
            text.push('\n');
            sections.push((
                prev_len..text.len().saturating_sub(1),
                PlaceholderType::Diagnostic(
                    ty,
                    util::truncate_and_trailoff(&entry.diagnostic.message, 50).replace('\n', " "),
                ),
            ))
        }
    }
}

#[derive(Clone)]
pub enum PlaceholderType {
    Root(DiagnosticSummary),
    File(String),
    Diagnostic(DiagnosticType, String),
}

#[derive(Copy, Clone, IntoElement)]
pub enum DiagnosticType {
    Warning,
    Error,
}

#[derive(IntoElement)]
pub struct DiagnosticsPlaceholder {
    pub id: ElementId,
    pub placeholder_type: PlaceholderType,
    pub unfold: Arc<dyn Fn(&mut WindowContext)>,
}

impl RenderOnce for DiagnosticsPlaceholder {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        let unfold = self.unfold;
        let (icon, content) = match self.placeholder_type {
            PlaceholderType::Root(summary) => (
                h_flex()
                    .gap_0p5()
                    .when(summary.error_count > 0, |this| {
                        this.child(DiagnosticType::Error)
                            .child(Label::new(summary.error_count.to_string()))
                    })
                    .when(summary.warning_count > 0, |this| {
                        this.child(DiagnosticType::Warning)
                            .child(Label::new(summary.warning_count.to_string()))
                    })
                    .into_any_element(),
                Label::new("Diagnostics"),
            ),
            PlaceholderType::File(file) => (
                Icon::new(IconName::File).into_any_element(),
                Label::new(file),
            ),
            PlaceholderType::Diagnostic(diagnostic_type, message) => {
                (diagnostic_type.into_any_element(), Label::new(message))
            }
        };

        ButtonLike::new(self.id)
            .style(ButtonStyle::Filled)
            .layer(ElevationIndex::ElevatedSurface)
            .child(icon)
            .child(content)
            .on_click(move |_, cx| unfold(cx))
    }
}

impl RenderOnce for DiagnosticType {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        svg()
            .size(cx.text_style().font_size)
            .flex_none()
            .map(|icon| match self {
                DiagnosticType::Warning => icon
                    .path(IconName::XCircle.path())
                    .text_color(Color::Error.color(cx)),
                DiagnosticType::Error => icon
                    .path(IconName::ExclamationTriangle.path())
                    .text_color(Color::Warning.color(cx)),
            })
    }
}
