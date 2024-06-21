use std::sync::{atomic::AtomicBool, Arc};

use anyhow::{anyhow, Result};
use assistant_slash_command::{SlashCommand, SlashCommandOutput, SlashCommandOutputSection};
use futures::FutureExt;
use gpui::{AppContext, Task, WeakView, WindowContext};
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
        query: String,
        _cancel: Arc<AtomicBool>,
        _workspace: Option<WeakView<Workspace>>,
        cx: &mut AppContext,
    ) -> Task<Result<Vec<String>>> {
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
                                    query.as_ref(),
                                )
                                .await?
                                .map_err(|e| anyhow!("{}", e))?;

                            anyhow::Ok(completions)
                        }
                        .boxed()
                    }
                })
                .await
        })
    }

    fn run(
        self: Arc<Self>,
        argument: Option<&str>,
        _workspace: WeakView<Workspace>,
        delegate: Arc<dyn LspAdapterDelegate>,
        cx: &mut WindowContext,
    ) -> Task<Result<SlashCommandOutput>> {
        let argument = argument.map(|arg| arg.to_string());
        let output = cx.background_executor().spawn(async move {
            self.extension
                .call({
                    let this = self.clone();
                    move |extension, store| {
                        async move {
                            let resource = store.data_mut().table().push(delegate)?;
                            let output = extension
                                .call_run_slash_command(
                                    store,
                                    &this.command,
                                    argument.as_deref(),
                                    resource,
                                )
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
