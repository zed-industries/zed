use super::{SlashCommand, SlashCommandOutput};
use anyhow::{anyhow, Result};
use assistant_slash_command::SlashCommandOutputSection;
use gpui::{AppContext, RenderOnce, Task, WeakView};
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
            sections.push((
                last_end..last_end + file_path.len(),
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
            PlaceholderType::Root => (
                Icon::new(IconName::CopilotDisabled),
                Label::new("Diagnostics"),
            ),
            PlaceholderType::File(file) => (Icon::new(IconName::File), Label::new(file)),
        };

        ButtonLike::new(self.id)
            .style(ButtonStyle::Filled)
            .layer(ElevationIndex::ElevatedSurface)
            .child(icon)
            .child(content)
            .on_click(move |_, cx| unfold(cx))
    }
}
