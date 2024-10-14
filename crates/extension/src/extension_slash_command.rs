use anyhow::{anyhow, Result};
use assistant_slash_command::{
    as_stream_vec, ArgumentCompletion, SlashCommand, SlashCommandOutputSection, SlashCommandResult,
};
use assistant_slash_command::{Role, SlashCommandEvent};
use futures::FutureExt;
use gpui::{Task, WeakView, WindowContext};
use language::{BufferSnapshot, LspAdapterDelegate};
use std::sync::{atomic::AtomicBool, Arc};
use ui::prelude::*;
use wasmtime_wasi::WasiView;
use workspace::Workspace;

use crate::wasm_host::{WasmExtension, WasmHost};

pub struct ExtensionSlashCommand {
    pub(crate) extension: WasmExtension,
    #[allow(unused)]
    pub(crate) host: Arc<WasmHost>,
    pub(crate) command: crate::wit::SlashCommand,
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
        _workspace: Option<WeakView<Workspace>>,
        cx: &mut WindowContext,
    ) -> Task<Result<Vec<ArgumentCompletion>>> {
        let arguments = arguments.to_owned();
        cx.background_executor().spawn(async move {
            self.extension
                .call({
                    let this = self.clone();
                    move |extension, store| {
                        async move {
                            let completions = extension
                                .call_complete_slash_command_argument(
                                    store,
                                    &this.command,
                                    &arguments,
                                )
                                .await?
                                .map_err(|e| anyhow!("{}", e))?;

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
                        }
                        .boxed()
                    }
                })
                .await
        })
    }

    fn run(
        self: Arc<Self>,
        arguments: &[String],
        _context_slash_command_output_sections: &[SlashCommandOutputSection<language::Anchor>],
        _context_buffer: BufferSnapshot,
        _workspace: WeakView<Workspace>,
        delegate: Option<Arc<dyn LspAdapterDelegate>>,
        cx: &mut WindowContext,
    ) -> Task<SlashCommandResult> {
        let arguments = arguments.to_owned();
        let output = cx.background_executor().spawn(async move {
            self.extension
                .call({
                    let this = self.clone();
                    move |extension, store| {
                        async move {
                            let resource = if let Some(delegate) = delegate {
                                Some(store.data_mut().table().push(delegate)?)
                            } else {
                                None
                            };
                            let output = extension
                                .call_run_slash_command(store, &this.command, &arguments, resource)
                                .await?
                                .map_err(|e| anyhow!("{}", e))?;

                            anyhow::Ok(output)
                        }
                        .boxed()
                    }
                })
                .await
        });
        cx.foreground_executor().spawn(async move {
            let _output = output.await?;

            let events = vec![
                SlashCommandEvent::StartMessage {
                    role: Role::Assistant,
                },
                SlashCommandEvent::Content {
                    run_commands_in_text: false,
                    text: "Here is some fake output from the extension slash command:".to_string(),
                },
                SlashCommandEvent::StartSection {
                    icon: IconName::Code,
                    label: "Code Output".into(),
                    metadata: None,
                    ensure_newline: true,
                },
                SlashCommandEvent::Content {
                    run_commands_in_text: false,
                    text: "let x = 42;\nprintln!(\"The answer is {}\", x);".to_string(),
                },
                SlashCommandEvent::EndSection { metadata: None },
                SlashCommandEvent::Content {
                    run_commands_in_text: false,
                    text: "\nThis concludes the fake output.".to_string(),
                },
            ];

            return Ok(as_stream_vec(events));
        })
    }
}
