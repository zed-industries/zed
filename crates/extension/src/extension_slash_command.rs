use std::sync::{atomic::AtomicBool, Arc};

use anyhow::{anyhow, Result};
use assistant_slash_command::{
    ArgumentCompletion, SlashCommand, SlashCommandOutput, SlashCommandOutputSection,
};
use futures::FutureExt;
use gpui::{Task, WeakView, WindowContext};
use language::LspAdapterDelegate;
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
        _workspace: WeakView<Workspace>,
        delegate: Option<Arc<dyn LspAdapterDelegate>>,
        cx: &mut WindowContext,
    ) -> Task<Result<SlashCommandOutput>> {
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
            let output = output.await?;
            Ok(SlashCommandOutput {
                text: output.text,
                sections: output
                    .sections
                    .into_iter()
                    .map(|section| SlashCommandOutputSection {
                        range: section.range.into(),
                        icon: IconName::Code,
                        label: section.label.into(),
                    })
                    .collect(),
                run_commands_in_text: false,
            })
        })
    }
}
