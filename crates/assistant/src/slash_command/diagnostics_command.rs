use super::{SlashCommand, SlashCommandOutput};
use anyhow::{anyhow, Result};
use assistant_slash_command::SlashCommandOutputSection;
use gpui::{svg, AppContext, RenderOnce, Task, WeakView};
use language::LspAdapterDelegate;
use std::{
    ops::Range,
    sync::{atomic::AtomicBool, Arc},
};
use ui::{prelude::*, ButtonLike, ElevationIndex};
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

        let project = workspace.read(cx).project().read(cx);
        let mut text = String::new();
        text.push_str("Diagnostics:\n");
        let mut sections: Vec<(Range<usize>, PlaceholderType)> = Vec::new();
        for (file, _, diagnostics) in project.diagnostic_summaries(false, cx) {
            let last_end = text.len();
            let file_path = file.path.to_string_lossy().to_string();
            text.push_str(&file_path);
            text.push('\n');

            for error in 0..diagnostics.error_count {
                let prev_len = text.len();
                text.push_str("Error ");
                text.push_str(&error.to_string());
                text.push('\n');
                sections.push((
                    prev_len..text.len().saturating_sub(1),
                    PlaceholderType::Diagnostic(DiagnosticType::Error, "Error".to_string()),
                ))
            }

            for warning in 0..diagnostics.warning_count {
                let prev_len = text.len();
                text.push_str("Warning ");
                text.push_str(&warning.to_string());
                text.push('\n');
                sections.push((
                    prev_len..text.len().saturating_sub(1),
                    PlaceholderType::Diagnostic(DiagnosticType::Warning, "Warning".to_string()),
                ))
            }

            sections.push((
                last_end..text.len().saturating_sub(1),
                PlaceholderType::File(file_path),
            ))
        }
        sections.push((0..text.len(), PlaceholderType::Root));

        //TODO move to background thread

        Task::Ready(Some(Ok(SlashCommandOutput {
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
        })))
    }
}

#[derive(Clone)]
pub enum PlaceholderType {
    Root,
    File(String),
    Diagnostic(DiagnosticType, String),
}

#[derive(Copy, Clone)]
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
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let unfold = self.unfold;

        let (icon, content) = match self.placeholder_type {
            PlaceholderType::Root => (
                Icon::new(IconName::CopilotDisabled).into_any_element(),
                Label::new("Diagnostics"),
            ),
            PlaceholderType::File(file) => (
                Icon::new(IconName::File).into_any_element(),
                Label::new(file),
            ),
            PlaceholderType::Diagnostic(diagnostic_type, message) => (
                svg()
                    .size(cx.text_style().font_size)
                    .flex_none()
                    .map(|icon| match diagnostic_type {
                        DiagnosticType::Warning => icon
                            .path(IconName::XCircle.path())
                            .text_color(Color::Error.color(cx)),
                        DiagnosticType::Error => icon
                            .path(IconName::ExclamationTriangle.path())
                            .text_color(Color::Warning.color(cx)),
                    })
                    .into_any_element(),
                Label::new(message),
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
