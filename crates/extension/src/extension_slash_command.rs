use std::sync::{atomic::AtomicBool, Arc};

use anyhow::{anyhow, Result};
use assistant_slash_command::{SlashCommand, SlashCommandOutput, SlashCommandOutputSection};
use futures::FutureExt;
use gpui::{AppContext, IntoElement, Task, WeakView, WindowContext};
use language::LspAdapterDelegate;
use ui::{prelude::*, ButtonLike, ElevationIndex};
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
        &self,
        _query: String,
        _cancel: Arc<AtomicBool>,
        _workspace: WeakView<Workspace>,
        _cx: &mut AppContext,
    ) -> Task<Result<Vec<String>>> {
        Task::ready(Ok(Vec::new()))
    }

    fn run(
        self: Arc<Self>,
        argument: Option<&str>,
        _workspace: WeakView<Workspace>,
        delegate: Arc<dyn LspAdapterDelegate>,
        cx: &mut WindowContext,
    ) -> Task<Result<SlashCommandOutput>> {
        let command_name = SharedString::from(self.command.name.clone());
        let argument = argument.map(|arg| arg.to_string());
        let text = cx.background_executor().spawn(async move {
            let output = self
                .extension
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
                .await?;
            output.ok_or_else(|| anyhow!("no output from command: {}", self.command.name))
        });
        cx.foreground_executor().spawn(async move {
            let text = text.await?;
            let range = 0..text.len();
            Ok(SlashCommandOutput {
                text,
                sections: vec![SlashCommandOutputSection {
                    range,
                    render_placeholder: Arc::new({
                        let command_name = command_name.clone();
                        move |id, unfold, _cx| {
                            ButtonLike::new(id)
                                .style(ButtonStyle::Filled)
                                .layer(ElevationIndex::ElevatedSurface)
                                .child(Icon::new(IconName::Code))
                                .child(Label::new(command_name.clone()))
                                .on_click(move |_event, cx| unfold(cx))
                                .into_any_element()
                        }
                    }),
                }],
            })
        })
    }
}
