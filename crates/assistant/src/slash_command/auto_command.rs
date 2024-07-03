use super::{SlashCommand, SlashCommandOutput};
use crate::slash_command::create_label_for_command;
use anyhow::Result;
use gpui::{AppContext, Task, WeakView};
use language::{CodeLabel, LspAdapterDelegate};
use std::sync::{atomic::AtomicBool, Arc};
use ui::WindowContext;
use workspace::Workspace;

pub(crate) struct AutoCommand;

impl SlashCommand for AutoCommand {
    fn name(&self) -> String {
        "action".into()
    }

    fn label(&self, cx: &AppContext) -> CodeLabel {
        create_label_for_command("action", &["--action"], cx)
    }

    fn description(&self) -> String {
        "Run an editor action".into()
    }

    fn menu_text(&self) -> String {
        "Run Editor Action".into()
    }

    fn requires_argument(&self) -> bool {
        true
    }

    fn complete_argument(
        self: Arc<Self>,
        query: String,
        _cancellation_flag: Arc<AtomicBool>,
        _workspace: Option<WeakView<Workspace>>,
        _cx: &mut AppContext,
    ) -> Task<Result<Vec<String>>> {
        let actions = vec![
            "workspace:search".to_string(),
            "workspace:newfile".to_string(),
            // Add more actions as needed
        ];

        let completions = actions
            .into_iter()
            .filter(|action| action.contains(&query))
            .collect::<Vec<_>>();

        Task::ready(Ok(completions))
    }

    fn run(
        self: Arc<Self>,
        argument: Option<&str>,
        _workspace: WeakView<Workspace>,
        _delegate: Arc<dyn LspAdapterDelegate>,
        cx: &mut WindowContext,
    ) -> Task<Result<SlashCommandOutput>> {
        use anyhow::anyhow;

        let Some(argument) = argument else {
            return Task::ready(Err(anyhow!("missing action argument")));
        };

        match argument {
            "workspace:search" => {
                cx.dispatch_action(Box::new(workspace::DeploySearch::find()));
            }
            "workspace:newfile" => {
                cx.dispatch_action(Box::new(workspace::NewFile::default()));
            }
            _ => return Task::ready(Err(anyhow!("unknown action: {}", argument))),
        }

        Task::ready(Ok(SlashCommandOutput::default()))
    }
}
