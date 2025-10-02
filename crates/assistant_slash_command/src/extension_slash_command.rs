use anyhow::Result;
use async_trait::async_trait;
use extension::{Extension, ExtensionHostProxy, ExtensionSlashCommandProxy, WorktreeDelegate};
use gpui::{App, Task, WeakEntity, Window};
use language::{BufferSnapshot, LspAdapterDelegate};
use std::sync::{Arc, atomic::AtomicBool};
use ui::prelude::*;
use util::rel_path::RelPath;
use workspace::Workspace;

use crate::{
    ArgumentCompletion, SlashCommand, SlashCommandOutput, SlashCommandOutputSection,
    SlashCommandRegistry, SlashCommandResult,
};

pub fn init(cx: &mut App) {
    let proxy = ExtensionHostProxy::default_global(cx);
    proxy.register_slash_command_proxy(SlashCommandRegistryProxy {
        slash_command_registry: SlashCommandRegistry::global(cx),
    });
}

struct SlashCommandRegistryProxy {
    slash_command_registry: Arc<SlashCommandRegistry>,
}

impl ExtensionSlashCommandProxy for SlashCommandRegistryProxy {
    fn register_slash_command(
        &self,
        extension: Arc<dyn Extension>,
        command: extension::SlashCommand,
    ) {
        self.slash_command_registry
            .register_command(ExtensionSlashCommand::new(extension, command), false)
    }

    fn unregister_slash_command(&self, command_name: Arc<str>) {
        self.slash_command_registry
            .unregister_command_by_name(&command_name)
    }
}

/// An adapter that allows an [`LspAdapterDelegate`] to be used as a [`WorktreeDelegate`].
struct WorktreeDelegateAdapter(Arc<dyn LspAdapterDelegate>);

#[async_trait]
impl WorktreeDelegate for WorktreeDelegateAdapter {
    fn id(&self) -> u64 {
        self.0.worktree_id().to_proto()
    }

    fn root_path(&self) -> String {
        self.0.worktree_root_path().to_string_lossy().into_owned()
    }

    async fn read_text_file(&self, path: &RelPath) -> Result<String> {
        self.0.read_text_file(path).await
    }

    async fn which(&self, binary_name: String) -> Option<String> {
        self.0
            .which(binary_name.as_ref())
            .await
            .map(|path| path.to_string_lossy().into_owned())
    }

    async fn shell_env(&self) -> Vec<(String, String)> {
        self.0.shell_env().await.into_iter().collect()
    }
}

pub struct ExtensionSlashCommand {
    extension: Arc<dyn Extension>,
    command: extension::SlashCommand,
}

impl ExtensionSlashCommand {
    pub fn new(extension: Arc<dyn Extension>, command: extension::SlashCommand) -> Self {
        Self { extension, command }
    }
}

impl SlashCommand for ExtensionSlashCommand {
    fn name(&self) -> String {
        self.command.name.clone()
    }

    fn description(&self) -> String {
        self.command.description.clone()
    }

    fn menu_text(&self) -> String {
        self.command.tooltip_text.clone()
    }

    fn requires_argument(&self) -> bool {
        self.command.requires_argument
    }

    fn complete_argument(
        self: Arc<Self>,
        arguments: &[String],
        _cancel: Arc<AtomicBool>,
        _workspace: Option<WeakEntity<Workspace>>,
        _window: &mut Window,
        cx: &mut App,
    ) -> Task<Result<Vec<ArgumentCompletion>>> {
        let command = self.command.clone();
        let arguments = arguments.to_owned();
        cx.background_spawn(async move {
            let completions = self
                .extension
                .complete_slash_command_argument(command, arguments)
                .await?;

            anyhow::Ok(
                completions
                    .into_iter()
                    .map(|completion| ArgumentCompletion {
                        label: completion.label.into(),
                        new_text: completion.new_text,
                        replace_previous_arguments: false,
                        after_completion: completion.run_command.into(),
                    })
                    .collect(),
            )
        })
    }

    fn run(
        self: Arc<Self>,
        arguments: &[String],
        _context_slash_command_output_sections: &[SlashCommandOutputSection<language::Anchor>],
        _context_buffer: BufferSnapshot,
        _workspace: WeakEntity<Workspace>,
        delegate: Option<Arc<dyn LspAdapterDelegate>>,
        _window: &mut Window,
        cx: &mut App,
    ) -> Task<SlashCommandResult> {
        let command = self.command.clone();
        let arguments = arguments.to_owned();
        let output = cx.background_spawn(async move {
            let delegate =
                delegate.map(|delegate| Arc::new(WorktreeDelegateAdapter(delegate.clone())) as _);
            let output = self
                .extension
                .run_slash_command(command, arguments, delegate)
                .await?;

            anyhow::Ok(output)
        });
        cx.foreground_executor().spawn(async move {
            let output = output.await?;
            Ok(SlashCommandOutput {
                text: output.text,
                sections: output
                    .sections
                    .into_iter()
                    .map(|section| SlashCommandOutputSection {
                        range: section.range,
                        icon: IconName::Code,
                        label: section.label.into(),
                        metadata: None,
                    })
                    .collect(),
                run_commands_in_text: false,
            }
            .into_event_stream())
        })
    }
}
