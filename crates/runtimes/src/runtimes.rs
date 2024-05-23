use anyhow::{Context as _, Result};
use collections::{HashMap, HashSet};
use editor::{
    display_map::{BlockContext, BlockDisposition, BlockProperties, BlockStyle, RenderBlock},
    Anchor, Editor,
};
use futures::{
    channel::mpsc::{self, UnboundedSender},
    Future, StreamExt as _,
};
use gpui::{actions, AppContext, Context, EntityId, Global, Model, ModelContext, Task};
use gpui::{Entity, View};
use kernelspecs::{get_runtimes, RunningKernel, Runtime};
use language::Point;
use outputs::ExecutionView;
use project::Fs;
use settings::Settings as _;
use std::sync::Arc;
use ui::prelude::*;
use workspace::Workspace;

mod kernelspecs;
mod outputs;
mod stdio;
mod tokio_kernel;

use theme::{ActiveTheme, ThemeSettings};

use tokio_kernel::{ExecutionId, ExecutionRequest, ExecutionUpdate};

actions!(runtimes, [Run]);

#[derive(Clone)]
pub struct RuntimeGlobal(Model<RuntimeManager>);

impl Global for RuntimeGlobal {}

pub fn init(fs: Arc<dyn Fs>, cx: &mut AppContext) {
    let runtime_manager = cx.new_model(|cx| RuntimeManager::new(fs.clone(), cx));
    RuntimeManager::set_global(runtime_manager.clone(), cx);

    cx.spawn(|mut cx| async move {
        let fs = fs.clone();

        let runtimes = get_runtimes(fs).await?;

        runtime_manager.update(&mut cx, |this, _cx| {
            this.runtimes = runtimes;
        })?;

        anyhow::Ok(())
    })
    .detach_and_log_err(cx);

    cx.observe_new_views(
        |workspace: &mut Workspace, _: &mut ViewContext<Workspace>| {
            // Note: this will have to both start a kernel if not already running, and run code selections
            workspace.register_action(RuntimeManager::run);
        },
    )
    .detach();
}

pub struct RuntimeManager {
    fs: Arc<dyn Fs>,
    runtimes: Vec<Runtime>,
    instances: HashMap<EntityId, RunningKernel>,
}

pub struct ActiveCode {
    pub selected_text: String,
    pub language_name: Arc<str>,
    pub anchor: Anchor,
    pub editor: View<Editor>,
}

impl RuntimeManager {
    pub fn new(fs: Arc<dyn Fs>, _cx: &mut AppContext) -> Self {
        Self {
            fs,
            runtimes: Default::default(),
            instances: Default::default(),
        }
    }

    fn acquire_execution_request_tx(
        &mut self,
        entity_id: EntityId,
        language_name: Arc<str>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<UnboundedSender<ExecutionRequest>>> {
        let running_kernel = self.instances.get(&entity_id);
        if let Some(running_kernel) = running_kernel {
            return Task::ready(anyhow::Ok(running_kernel.execution_request_tx.clone()));
        }
        // TODO: Track that a kernel is (possibly) starting up so we don't relaunch without tearing down the old one

        // Get first runtime that matches the language name (for now)
        let runtime = self
            .runtimes
            .iter()
            .find(|runtime| runtime.spec.language == language_name.to_string());

        let runtime = match runtime {
            Some(runtime) => runtime,
            None => {
                return Task::ready(Err(anyhow::anyhow!(
                    "No runtime found for language {}",
                    language_name
                )));
            }
        };

        let runtime = runtime.clone();

        let fs = self.fs.clone();

        cx.spawn(|this, mut cx| async move {
            let running_kernel = RunningKernel::new(runtime, &entity_id, fs.clone()).await?;

            let execution_request_tx = running_kernel.execution_request_tx.clone();
            this.update(&mut cx, |this, _cx| {
                this.instances
                    .insert(entity_id, running_kernel)
                    .ok_or(anyhow::anyhow!("Failed to insert runtime"))?;
                anyhow::Ok(())
            })??;
            anyhow::Ok(execution_request_tx)
        })
    }

    fn execute_code(
        &mut self,
        entity_id: EntityId,
        language_name: Arc<str>,
        execution_id: ExecutionId,
        code: String,
        cx: &mut ModelContext<Self>,
    ) -> impl Future<Output = Result<mpsc::UnboundedReceiver<ExecutionUpdate>>> {
        let (tx, rx) = mpsc::unbounded();

        let execution_request_tx = self.acquire_execution_request_tx(entity_id, language_name, cx);

        async move {
            let execution_request_tx = execution_request_tx.await?;

            execution_request_tx
                .unbounded_send(ExecutionRequest {
                    execution_id,
                    request: runtimelib::ExecuteRequest {
                        code,
                        allow_stdin: false,
                        silent: false,
                        store_history: true,
                        user_expressions: None,
                        stop_on_error: false,
                        // TODO(runtimelib): set up Default::default() for the rest of the fields
                        // ..Default::default()
                    },
                    response_sender: tx,
                })
                .context("Failed to send execution request")?;

            Ok(rx)
        }
    }

    pub fn global(cx: &AppContext) -> Option<Model<Self>> {
        cx.try_global::<RuntimeGlobal>()
            .map(|model| model.0.clone())
    }

    pub fn set_global(runtime: Model<Self>, cx: &mut AppContext) {
        cx.set_global(RuntimeGlobal(runtime));
    }

    // Gets the active selection in the editor or the current line
    pub fn get_active_code(
        workspace: &mut Workspace,
        cx: &mut ViewContext<Workspace>,
    ) -> Option<ActiveCode> {
        workspace
            .active_item(cx)
            .and_then(|item| item.act_as::<Editor>(cx))
            .and_then(|editor_view| {
                let editor = editor_view.read(cx);
                let selection = editor.selections.newest::<usize>(cx);
                let buffer = editor.buffer().read(cx).snapshot(cx);

                let range = if selection.is_empty() {
                    let cursor = selection.head();

                    let line_start = buffer.offset_to_point(cursor).row;
                    let mut start_offset = buffer.point_to_offset(Point::new(line_start, 0));

                    // Iterate backwards to find the start of the line
                    while start_offset > 0 {
                        let ch = buffer.chars_at(start_offset - 1).next().unwrap_or('\0');
                        if ch == '\n' {
                            break;
                        }
                        start_offset -= 1;
                    }

                    let mut end_offset = cursor;

                    // Iterate forwards to find the end of the line
                    while end_offset < buffer.len() {
                        let ch = buffer.chars_at(end_offset).next().unwrap_or('\0');
                        if ch == '\n' {
                            break;
                        }
                        end_offset += 1;
                    }

                    // Create a range from the start to the end of the line
                    start_offset..end_offset
                } else {
                    selection.range()
                };

                let anchor = buffer.anchor_after(range.end);

                let selected_text = buffer.text_for_range(range.clone()).collect::<String>();

                let start_language = buffer.language_at(range.start);
                let end_language = buffer.language_at(range.end);

                let language_name = if start_language == end_language {
                    start_language
                        .map(|language| language.code_fence_block_name())
                        .filter(|lang| **lang != *"markdown")
                } else {
                    None
                };

                language_name.map(|language_name| ActiveCode {
                    selected_text,
                    language_name,
                    anchor,
                    editor: editor_view,
                })
            })
    }

    pub fn run(workspace: &mut Workspace, _: &Run, cx: &mut ViewContext<Workspace>) {
        let active_code = Self::get_active_code(workspace, cx);

        let active_code = if let Some(active_code) = active_code {
            active_code
        } else {
            return;
        };

        // let (code, language_name, anchor, editor) = code_snippet;

        let runtime_manager = if let Some(runtime_manager) = RuntimeManager::global(cx) {
            runtime_manager
        } else {
            log::error!("No runtime manager found");
            return;
        };

        let entity_id = active_code.editor.entity_id();
        let execution_id = ExecutionId::new();

        // Since we don't know the height, in editor terms, we have to calculate it over time
        // and just create a new block, replacing the old. It would be better if we could
        // just rely on the view updating and for the height to be calculated automatically.
        //
        // We will just handle text for the moment to keep this accurate.
        // Plots and other images will have to wait.
        let execution_view = cx.new_view(|cx| ExecutionView::new(execution_id.clone(), cx));

        let position = active_code.anchor;

        let mut block_id = active_code.editor.update(cx, |editor, cx| {
            let block = BlockProperties {
                position,
                height: 1,
                style: BlockStyle::Sticky,
                render: create_output_area_render(execution_view.clone()),
                disposition: BlockDisposition::Below,
            };

            editor.insert_blocks([block], None, cx)[0]
        });

        let receiver = runtime_manager.update(cx, |runtime_manager, cx| {
            runtime_manager.execute_code(
                entity_id,
                active_code.language_name,
                execution_id.clone(),
                active_code.selected_text.clone(),
                cx,
            )
        });

        let editor = active_code.editor.clone();

        cx.spawn(|_this, mut cx| async move {
            let mut receiver = receiver.await?;

            let execution_view = execution_view.clone();
            while let Some(update) = receiver.next().await {
                execution_view.update(&mut cx, |execution_view, cx| {
                    execution_view.push_message(&update.update, cx)
                })?;

                editor.update(&mut cx, |editor, cx| {
                    let mut blocks_to_remove = HashSet::default();
                    blocks_to_remove.insert(block_id);

                    editor.remove_blocks(blocks_to_remove, None, cx);

                    let block = BlockProperties {
                        position,
                        height: 1 + execution_view.read(cx).execution.read(cx).num_lines(),
                        style: BlockStyle::Sticky,
                        render: create_output_area_render(execution_view.clone()),
                        disposition: BlockDisposition::Below,
                    };

                    block_id = editor.insert_blocks([block], None, cx)[0];
                })?;
            }
            anyhow::Ok(())
        })
        .detach();
    }
}

fn create_output_area_render(execution_view: View<ExecutionView>) -> RenderBlock {
    let render = move |cx: &mut BlockContext| {
        let execution_view = execution_view.clone();
        let text_font = ThemeSettings::get_global(cx).buffer_font.family.clone();
        // Note: we'll want to use `cx.anchor_x` when someone runs something with no output -- just show a checkmark and not make the full block below the line

        let gutter_width = cx.gutter_dimensions.width;

        h_flex()
            .w_full()
            .bg(cx.theme().colors().background)
            .border_y_1()
            .border_color(cx.theme().colors().border)
            .pl(gutter_width)
            .child(
                div()
                    .font_family(text_font)
                    // .ml(gutter_width)
                    .mx_1()
                    .my_2()
                    .h_full()
                    .w_full()
                    .mr(gutter_width)
                    .child(execution_view),
            )
            .into_any_element()
    };

    Box::new(render)
}
