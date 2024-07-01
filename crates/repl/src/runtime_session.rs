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
use gpui::{div, prelude::*, Entity, Render, Task, View, ViewContext};
use project::Fs;
use runtimelib::{
    ExecuteRequest, JupyterMessage, JupyterMessageContent, KernelInfoRequest, ShutdownRequest,
};
use settings::Settings as _;
use std::{ops::Range, sync::Arc, time::Duration};
use theme::{ActiveTheme, ThemeSettings};
use ui::{h_flex, prelude::*, v_flex, ButtonLike, ButtonStyle, Label};

pub struct Session {
    editor: View<Editor>,
    kernel: Kernel,
    blocks: HashMap<String, EditorBlock>,
    messaging_task: Task<()>,
    runtime_specification: RuntimeSpecification,
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
        let kernel = RunningKernel::new(runtime_specification.clone(), entity_id, fs.clone(), cx);

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

                            // todo!(): await the kernel info reply, with a timeout duration
                            this.send(&KernelInfoRequest {}.into(), cx).ok();

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
            runtime_specification,
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
            _ => {}
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
            Kernel::RunningKernel(_) => ExecutionStatus::Queued,
            Kernel::StartingKernel(_) => ExecutionStatus::ConnectingToKernel,
            // todo!(): Be more fine grained
            Kernel::ErroredLaunch(_) => ExecutionStatus::Unknown,
            Kernel::ShuttingDown => ExecutionStatus::ShuttingDown,
            Kernel::Shutdown => ExecutionStatus::Shutdown,
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

        match &self.kernel {
            Kernel::RunningKernel(_) => {
                self.send(&message, cx).ok();
            }
            Kernel::StartingKernel(task) => {
                let task = task.clone();
                let message = message.clone();

                cx.spawn(|this, mut cx| async move {
                    task.await;
                    this.update(&mut cx, |this, cx| {
                        this.send(&message, cx).ok();
                    })
                    .ok();
                })
                .detach();
            }
            Kernel::ErroredLaunch(_) => {
                // todo!(): Show error message for this run
            }
            _ => {}
        }
    }

    fn route(&mut self, message: JupyterMessage, cx: &mut ViewContext<Self>) {
        let parent_message_id = match message.parent_header {
            Some(header) => header.msg_id,
            None => return,
        };

        match &message.content {
            runtimelib::JupyterMessageContent::Status(status) => {
                self.kernel.set_execution_state(&status.execution_state);
                cx.notify();
            }
            runtimelib::JupyterMessageContent::KernelInfoReply(reply) => {
                self.kernel.set_kernel_info(&reply);
                cx.notify();
            }
            _ => {}
        }

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
            return;
        }
    }

    pub fn shutdown(&mut self, cx: &mut ViewContext<Self>) {
        let kernel = std::mem::replace(&mut self.kernel, Kernel::ShuttingDown);
        // todo!(): emit event for the runtime panel to remove this session once in shutdown state

        match kernel {
            Kernel::RunningKernel(mut kernel) => {
                let mut request_tx = kernel.request_tx.clone();

                cx.spawn(|this, mut cx| async move {
                    let request = ShutdownRequest { restart: false };
                    let message =
                        JupyterMessage::new(JupyterMessageContent::ShutdownRequest(request), None);

                    request_tx.try_send(message).ok();

                    // Give the kernel a bit of time to clean up
                    cx.background_executor().timer(Duration::from_secs(3)).await;

                    kernel.process.kill().ok();

                    this.update(&mut cx, |this, cx| {
                        this.kernel = Kernel::Shutdown;
                        cx.notify();
                    })
                    .ok();
                })
                .detach();
            }
            Kernel::StartingKernel(_kernel) => {
                // todo!(): cancel the task
                self.kernel = Kernel::Shutdown;
            }
            _ => {
                self.kernel = Kernel::Shutdown;
            }
        }
        cx.notify();
    }
}

impl Render for Session {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let mut buttons = vec![];

        buttons.push(
            ButtonLike::new("shutdown")
                .child(Label::new("Shutdown"))
                .style(ButtonStyle::Subtle)
                .on_click(cx.listener(move |session, _, cx| {
                    session.shutdown(cx);
                })),
        );

        let status_text = match &self.kernel {
            Kernel::RunningKernel(kernel) => {
                buttons.push(
                    ButtonLike::new("interrupt")
                        .child(Label::new("Interrupt"))
                        .style(ButtonStyle::Subtle)
                        .on_click(cx.listener(move |_this, _, _cx| {
                            // todo!(): Implement interrupt
                        })),
                );
                let mut name = self.runtime_specification.name.clone();

                if let Some(info) = &kernel.kernel_info {
                    name.push_str(" (");
                    name.push_str(&info.language_info.name);
                    name.push_str(")");
                }
                name
            }
            Kernel::StartingKernel(_) => format!("{} (Starting)", self.runtime_specification.name),
            Kernel::ErroredLaunch(err) => {
                format!("{} (Error: {})", self.runtime_specification.name, err)
            }
            Kernel::ShuttingDown => format!("{} (Shutting Down)", self.runtime_specification.name),
            Kernel::Shutdown => format!("{} (Shutdown)", self.runtime_specification.name),
        };

        return v_flex()
            .gap_1()
            .child(
                h_flex()
                    .gap_2()
                    .child(self.kernel.dot())
                    .child(Label::new(status_text)),
            )
            .child(h_flex().gap_2().children(buttons));
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
