use super::{SlashCommand, SlashCommandOutput};
use anyhow::{anyhow, Context as _, Result};
use assistant_slash_command::{ArgumentCompletion, SlashCommandOutputSection};
use editor::Editor;
use gpui::{Task, WeakView};
use language::LspAdapterDelegate;
use std::sync::Arc;
use std::{path::Path, sync::atomic::AtomicBool};
use ui::{IconName, WindowContext};
use workspace::Workspace;

pub(crate) struct OutlineSlashCommand;

impl SlashCommand for OutlineSlashCommand {
    fn name(&self) -> String {
        "symbols".into()
    }

    fn description(&self) -> String {
        "insert symbols for active tab".into()
    }

    fn menu_text(&self) -> String {
        "Insert Symbols for Active Tab".into()
    }

    fn complete_argument(
        self: Arc<Self>,
        _arguments: &[String],
        _cancel: Arc<AtomicBool>,
        _workspace: Option<WeakView<Workspace>>,
        _cx: &mut WindowContext,
    ) -> Task<Result<Vec<ArgumentCompletion>>> {
        Task::ready(Err(anyhow!("this command does not require argument")))
    }

    fn requires_argument(&self) -> bool {
        false
    }

    fn run(
        self: Arc<Self>,
        _arguments: &[String],
        workspace: WeakView<Workspace>,
        _delegate: Option<Arc<dyn LspAdapterDelegate>>,
        cx: &mut WindowContext,
    ) -> Task<Result<SlashCommandOutput>> {
        let output = workspace.update(cx, |workspace, cx| {
            let Some(active_item) = workspace.active_item(cx) else {
                return Task::ready(Err(anyhow!("no active tab")));
            };
            let Some(buffer) = active_item
                .downcast::<Editor>()
                .and_then(|editor| editor.read(cx).buffer().read(cx).as_singleton())
            else {
                return Task::ready(Err(anyhow!("active tab is not an editor")));
            };

            let snapshot = buffer.read(cx).snapshot();
            let path = snapshot.resolve_file_path(cx, true);

            cx.background_executor().spawn(async move {
                let outline = snapshot
                    .outline(None)
                    .context("no symbols for active tab")?;

                let path = path.as_deref().unwrap_or(Path::new("untitled"));
                let mut outline_text = format!("Symbols for {}:\n", path.display());
                for item in &outline.path_candidates {
                    outline_text.push_str("- ");
                    outline_text.push_str(&item.string);
                    outline_text.push('\n');
                }

                Ok(SlashCommandOutput {
                    sections: vec![SlashCommandOutputSection {
                        range: 0..outline_text.len(),
                        icon: IconName::ListTree,
                        label: path.to_string_lossy().to_string().into(),
                    }],
                    text: outline_text,
                    run_commands_in_text: false,
                })
            })
        });

        output.unwrap_or_else(|error| Task::ready(Err(error)))
    }
}
