use super::{SlashCommand, SlashCommandOutput};
use anyhow::{anyhow, Result};
use assistant_slash_command::SlashCommandOutputSection;
use gpui::{svg, AppContext, Model, RenderOnce, Task, WeakView};
use language::{
    Anchor, BufferSnapshot, DiagnosticEntry, DiagnosticSeverity, LspAdapterDelegate,
    OffsetRangeExt, ToOffset,
};
use project::{DiagnosticSummary, Project};
use rope::Point;
use std::{
    ops::Range,
    sync::{atomic::AtomicBool, Arc},
};
use ui::{prelude::*, ButtonLike, ElevationIndex};
use util::ResultExt;
use workspace::Workspace;

pub(crate) struct DiagnosticsCommand;

const INCLUDE_WARNINGS_ARGUMENT: &str = "--include-warnings";

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
        Task::ready(Ok(vec![INCLUDE_WARNINGS_ARGUMENT.to_string()]))
    }

    fn run(
        self: Arc<Self>,
        argument: Option<&str>,
        workspace: WeakView<Workspace>,
        _delegate: Arc<dyn LspAdapterDelegate>,
        cx: &mut WindowContext,
    ) -> Task<Result<SlashCommandOutput>> {
        let Some(workspace) = workspace.upgrade() else {
            return Task::ready(Err(anyhow!("workspace was dropped")));
        };

        let include_warnings = argument
            .map(|argument| argument == INCLUDE_WARNINGS_ARGUMENT)
            .unwrap_or(false);

        let task = collect_diagnostics(workspace.read(cx).project().clone(), include_warnings, cx);
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
    include_warnings: bool,
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
            if include_warnings {
                project_summary.warning_count += summary.warning_count;
            } else if summary.error_count == 0 {
                continue;
            }

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
                    include_warnings,
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
    include_warnings: bool,
) {
    for (_, group) in snapshot.diagnostic_groups(None) {
        let entry = &group.entries[group.primary_ix];
        collect_diagnostic(text, sections, entry, &snapshot, include_warnings)
    }
}

fn collect_diagnostic(
    text: &mut String,
    sections: &mut Vec<(Range<usize>, PlaceholderType)>,
    entry: &DiagnosticEntry<Anchor>,
    snapshot: &BufferSnapshot,
    include_warnings: bool,
) {
    const EXCERPT_EXPANSION_SIZE: u32 = 2;

    let ty = match entry.diagnostic.severity {
        DiagnosticSeverity::WARNING => {
            if !include_warnings {
                return;
            }
            DiagnosticType::Warning
        }
        DiagnosticSeverity::ERROR => DiagnosticType::Error,
        _ => return,
    };
    let prev_len = text.len();

    let range = entry.range.to_point(snapshot);
    let diagnostic_row_number = range.start.row + 1;

    let start_row = range.start.row.saturating_sub(EXCERPT_EXPANSION_SIZE);
    let end_row = (range.end.row + EXCERPT_EXPANSION_SIZE).min(snapshot.max_point().row) + 1;
    let excerpt_range =
        Point::new(start_row, 0).to_offset(&snapshot)..Point::new(end_row, 0).to_offset(&snapshot);

    text.push_str("```");
    if let Some(language_name) = snapshot.language().map(|l| l.code_fence_block_name()) {
        text.push_str(&language_name);
    }
    text.push('\n');

    let mut buffer_text = String::new();
    for chunk in snapshot.text_for_range(excerpt_range) {
        buffer_text.push_str(chunk);
    }

    let line_number_width = end_row.to_string().len();
    for (i, line) in buffer_text.lines().enumerate() {
        let line_number = start_row + i as u32 + 1;
        text.push_str(format!("{line_number:>line_number_width$} ",).as_str());
        text.push_str(line);
        text.push('\n');

        if line_number == diagnostic_row_number {
            let prev_len = text.len();
            text.push_str("--> ");
            text.push_str(ty.as_str());
            text.push_str(": ");

            let padding = text.len() - prev_len;
            let message = entry
                .diagnostic
                .message
                .replace('\n', format!("\n{:padding$}", "").as_str());

            text.push_str(&message);
            text.push('\n');
        }
    }

    text.push_str("```");

    text.push('\n');
    sections.push((
        prev_len..text.len().saturating_sub(1),
        PlaceholderType::Diagnostic(ty, entry.diagnostic.message.clone()),
    ))
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

impl DiagnosticType {
    pub fn as_str(&self) -> &'static str {
        match self {
            DiagnosticType::Warning => "warning",
            DiagnosticType::Error => "error",
        }
    }
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
                    .w_full()
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
            PlaceholderType::Diagnostic(diagnostic_type, message) => (
                diagnostic_type.into_any_element(),
                Label::new(message).single_line(),
            ),
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
                DiagnosticType::Error => icon
                    .path(IconName::XCircle.path())
                    .text_color(Color::Error.color(cx)),
                DiagnosticType::Warning => icon
                    .path(IconName::ExclamationTriangle.path())
                    .text_color(Color::Warning.color(cx)),
            })
    }
}
