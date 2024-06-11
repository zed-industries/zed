use super::{SlashCommand, SlashCommandOutput};
use anyhow::{anyhow, Result};
use assistant_slash_command::SlashCommandOutputSection;
use fuzzy::PathMatch;
use gpui::{AppContext, RenderOnce, SharedString, Task, View, WeakView};
use language::{LineEnding, LspAdapterDelegate};
use project::PathMatchCandidateSet;
use std::{
    ops::Range,
    path::{Path, PathBuf},
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
        query: String,
        cancellation_flag: Arc<AtomicBool>,
        workspace: Option<WeakView<Workspace>>,
        cx: &mut AppContext,
    ) -> Task<Result<Vec<String>>> {
        Task::ready(Err(anyhow!("this command does not require argument")))
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

        let text = "Diagnostics".to_string();
        let range = 0..text.len();
        Task::Ready(Some(Ok(SlashCommandOutput {
            text,
            sections: vec![SlashCommandOutputSection {
                range,
                render_placeholder: Arc::new(move |id, unfold, _cx| {
                    DiagnosticsPlaceholder { id, unfold }.into_any_element()
                }),
            }],
            run_commands_in_text: false,
        })))
    }
}

#[derive(IntoElement)]
pub struct DiagnosticsPlaceholder {
    pub id: ElementId,
    pub unfold: Arc<dyn Fn(&mut WindowContext)>,
}

impl RenderOnce for DiagnosticsPlaceholder {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        let unfold = self.unfold;

        ButtonLike::new(self.id)
            .style(ButtonStyle::Filled)
            .layer(ElevationIndex::ElevatedSurface)
            .child(Icon::new(IconName::File))
            .child(Label::new("Some file"))
            .on_click(move |_, cx| unfold(cx))
    }
}
