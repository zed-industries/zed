use crate::{
    outputs::{ExecutionStatus, ExecutionView, LineHeight as _},
    runtimes::{Kernel, RunningKernel, RuntimeSpecification},
};
use collections::{HashMap, HashSet};
use editor::{
    display_map::{
        BlockContext, BlockDisposition, BlockId, BlockProperties, BlockStyle, RenderBlock,
    },
    Anchor, AnchorRangeExt as _, Editor,
};
use futures::{FutureExt as _, StreamExt as _};
use gpui::{prelude::*, Entity, Render, Task, View, ViewContext};
use project::Fs;
use runtimelib::{ExecuteRequest, JupyterMessage};
use settings::Settings as _;
use std::{ops::Range, sync::Arc};
use theme::{ActiveTheme, ThemeSettings};
use ui::{div, h_flex, v_flex};

pub struct Session {
    editor: View<Editor>,
    kernel: Kernel,
    blocks: HashMap<String, EditorBlock>,
    messaging_task: Task<()>,
}

#[derive(Debug)]
struct EditorBlock {
    code_range: Range<Anchor>,
    block_id: BlockId,
    execution_view: View<ExecutionView>,
}

impl Session {
    pub fn new(
        editor: View<Editor>,
        fs: Arc<dyn Fs>,
        runtime_specification: RuntimeSpecification,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let entity_id = editor.entity_id();
        let kernel = RunningKernel::new(runtime_specification, entity_id, fs.clone(), cx);

        // todo!(): Add in a kernel info request on startup until ready with a duration check

        let pending_kernel = cx
            .spawn(|this, mut cx| async move {
                let kernel = kernel.await;
                // In reality, this is like the "starting" kernel

                match kernel {
                    Ok((kernel, mut messages_rx)) => {
                        this.update(&mut cx, |this, cx| {
                            // At this point we can create a new kind of kernel that has the process and our long running background tasks
                            this.kernel = Kernel::RunningKernel(kernel);
                            // todo!(): Clear queue of pending executions
                            this.messaging_task = cx.spawn(|session, mut cx| async move {
                                while let Some(message) = messages_rx.next().await {
                                    session
                                        .update(&mut cx, |session, cx| {
                                            session.route(message, cx);
                                        })
                                        .ok();
                                }
                            });
                        })
                        .ok();
                    }
                    Err(err) => {
                        this.update(&mut cx, |this, _cx| {
                            this.kernel = Kernel::ErroredLaunch(err.to_string());
                        })
                        .ok();
                    }
                }
            })
            .shared();

        return Self {
            editor,
            kernel: Kernel::StartingKernel(pending_kernel),
            messaging_task: Task::ready(()),
            blocks: HashMap::default(),
        };
    }

    pub fn send(
        &mut self,
        message: &JupyterMessage,
        _cx: &mut ViewContext<Self>,
    ) -> anyhow::Result<()> {
        match &mut self.kernel {
            Kernel::RunningKernel(kernel) => {
                kernel.request_tx.try_send(message.clone()).ok();
            }
            Kernel::StartingKernel(_kernel_task) => {
                // todo!(): Queue up the execution
            }
            Kernel::ErroredLaunch(_) => {
                // todo!(): Show error message for this run
            }
        }

        anyhow::Ok(())
    }

    pub fn execute(&mut self, code: &str, anchor_range: Range<Anchor>, cx: &mut ViewContext<Self>) {
        let execute_request = ExecuteRequest {
            code: code.to_string(),
            ..ExecuteRequest::default()
        };

        let message: JupyterMessage = execute_request.into();

        let status = match &self.kernel {
            // Technically this is probably more like queued. Later Status messages will update it
            Kernel::RunningKernel(_) => ExecutionStatus::Executing,
            Kernel::StartingKernel(_) => ExecutionStatus::ConnectingToKernel,
            // todo!(): Be more fine grained
            Kernel::ErroredLaunch(_) => ExecutionStatus::Unknown,
        };

        let execution_view = cx.new_view(|cx| {
            let mut execution_view = ExecutionView::new(cx);
            execution_view.set_status(status, cx);
            execution_view
        });

        let mut blocks_to_remove: HashSet<BlockId> = HashSet::default();

        let buffer = self.editor.read(cx).buffer().read(cx).snapshot(cx);

        self.blocks.retain(|_key, block| {
            if anchor_range.overlaps(&block.code_range, &buffer) {
                blocks_to_remove.insert(block.block_id);
                false
            } else {
                true
            }
        });

        let block_id = self.editor.update(cx, |editor, cx| {
            editor.remove_blocks(blocks_to_remove, None, cx);
            let block = BlockProperties {
                position: anchor_range.end,
                height: execution_view.num_lines(cx).saturating_add(1),
                style: BlockStyle::Sticky,
                render: create_output_area_render(execution_view.clone()),
                disposition: BlockDisposition::Below,
            };

            editor.insert_blocks([block], None, cx)[0]
        });

        let editor_block = EditorBlock {
            code_range: anchor_range,
            block_id,
            execution_view,
        };

        self.blocks
            .insert(message.header.msg_id.clone(), editor_block);

        self.send(&message, cx).ok();
    }

    fn route(&mut self, message: JupyterMessage, cx: &mut ViewContext<Self>) {
        let parent_message_id = match message.parent_header {
            Some(header) => header.msg_id,
            None => return,
        };

        if let Some(block) = self.blocks.get_mut(&parent_message_id) {
            block.execution_view.update(cx, |execution_view, cx| {
                execution_view.push_message(&message.content, cx);
            });
            self.editor.update(cx, |editor, cx| {
                let mut replacements = HashMap::default();
                replacements.insert(
                    block.block_id,
                    (
                        Some(block.execution_view.num_lines(cx).saturating_add(1)),
                        create_output_area_render(block.execution_view.clone()),
                    ),
                );
                editor.replace_blocks(replacements, None, cx);
            });
        }
    }
}

impl Render for Session {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl ui::IntoElement {
        let kernel_text = match &self.kernel {
            Kernel::RunningKernel(_) => "Running".to_string(),
            Kernel::StartingKernel(_) => "Starting".to_string(),
            Kernel::ErroredLaunch(error) => format!("Error: {}", error),
        };

        v_flex()
            .child("Session")
            .child(kernel_text)
            .into_any_element()
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
