use crate::{
    kernels::{Kernel, KernelSpecification, RunningKernel},
    outputs::{ExecutionStatus, ExecutionView, LineHeight as _},
};
use collections::{HashMap, HashSet};
use editor::{
    display_map::{
        BlockContext, BlockDisposition, BlockId, BlockProperties, BlockStyle, RenderBlock,
    },
    Anchor, AnchorRangeExt as _, Editor,
};
use futures::{FutureExt as _, StreamExt as _};
use gpui::{div, prelude::*, Entity, EventEmitter, Render, Task, View, ViewContext};
use project::Fs;
use runtimelib::{
    ExecuteRequest, InterruptRequest, JupyterMessage, JupyterMessageContent, KernelInfoRequest,
    ShutdownRequest,
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
    kernel_specification: KernelSpecification,
}

#[derive(Debug)]
struct EditorBlock {
    editor: View<Editor>,
    code_range: Range<Anchor>,
    block_id: BlockId,
    execution_view: View<ExecutionView>,
}

impl EditorBlock {
    fn new(
        editor: View<Editor>,
        code_range: Range<Anchor>,
        status: ExecutionStatus,
        cx: &mut ViewContext<Session>,
    ) -> Self {
        let execution_view = cx.new_view(|cx| ExecutionView::new(status, cx));

        let block_id = editor.update(cx, |editor, cx| {
            let block = BlockProperties {
                position: code_range.end,
                height: execution_view.num_lines(cx).saturating_add(1),
                style: BlockStyle::Sticky,
                render: Self::create_output_area_render(execution_view.clone()),
                disposition: BlockDisposition::Below,
            };

            editor.insert_blocks([block], None, cx)[0]
        });

        Self {
            editor,
            code_range,
            block_id,
            execution_view,
        }
    }

    fn handle_message(&mut self, message: &JupyterMessage, cx: &mut ViewContext<Session>) {
        self.execution_view.update(cx, |execution_view, cx| {
            execution_view.push_message(&message.content, cx);
        });

        self.editor.update(cx, |editor, cx| {
            let mut replacements = HashMap::default();
            replacements.insert(
                self.block_id,
                (
                    Some(self.execution_view.num_lines(cx).saturating_add(1)),
                    Self::create_output_area_render(self.execution_view.clone()),
                ),
            );
            editor.replace_blocks(replacements, None, cx);
        })
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
}

impl Session {
    pub fn new(
        editor: View<Editor>,
        fs: Arc<dyn Fs>,
        kernel_specification: KernelSpecification,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let entity_id = editor.entity_id();
        let kernel = RunningKernel::new(kernel_specification.clone(), entity_id, fs.clone(), cx);

        let pending_kernel = cx
            .spawn(|this, mut cx| async move {
                let kernel = kernel.await;

                match kernel {
                    Ok((kernel, mut messages_rx)) => {
                        this.update(&mut cx, |this, cx| {
                            // At this point we can create a new kind of kernel that has the process and our long running background tasks
                            this.kernel = Kernel::RunningKernel(kernel);

                            this.messaging_task = cx.spawn(|session, mut cx| async move {
                                while let Some(message) = messages_rx.next().await {
                                    session
                                        .update(&mut cx, |session, cx| {
                                            session.route(&message, cx);
                                        })
                                        .ok();
                                }
                            });

                            // For some reason sending a kernel info request will brick the ark (R) kernel.
                            // Note that Deno and Python do not have this issue.
                            if this.kernel_specification.name == "ark" {
                                return;
                            }

                            // Get kernel info after (possibly) letting the kernel start
                            cx.spawn(|this, mut cx| async move {
                                cx.background_executor()
                                    .timer(Duration::from_millis(120))
                                    .await;
                                this.update(&mut cx, |this, _cx| {
                                    this.send(KernelInfoRequest {}.into(), _cx).ok();
                                })
                                .ok();
                            })
                            .detach();
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
            kernel_specification,
        };
    }

    fn send(&mut self, message: JupyterMessage, _cx: &mut ViewContext<Self>) -> anyhow::Result<()> {
        match &mut self.kernel {
            Kernel::RunningKernel(kernel) => {
                kernel.request_tx.try_send(message).ok();
            }
            _ => {}
        }

        anyhow::Ok(())
    }

    pub fn clear_outputs(&mut self, cx: &mut ViewContext<Self>) {
        let blocks_to_remove: HashSet<BlockId> =
            self.blocks.values().map(|block| block.block_id).collect();

        self.editor.update(cx, |editor, cx| {
            editor.remove_blocks(blocks_to_remove, None, cx);
        });

        self.blocks.clear();
    }

    pub fn execute(&mut self, code: &str, anchor_range: Range<Anchor>, cx: &mut ViewContext<Self>) {
        let execute_request = ExecuteRequest {
            code: code.to_string(),
            ..ExecuteRequest::default()
        };

        let message: JupyterMessage = execute_request.into();

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

        self.editor.update(cx, |editor, cx| {
            editor.remove_blocks(blocks_to_remove, None, cx);
        });

        let status = match &self.kernel {
            Kernel::RunningKernel(_) => ExecutionStatus::Queued,
            Kernel::StartingKernel(_) => ExecutionStatus::ConnectingToKernel,
            Kernel::ErroredLaunch(error) => ExecutionStatus::KernelErrored(error.clone()),
            Kernel::ShuttingDown => ExecutionStatus::ShuttingDown,
            Kernel::Shutdown => ExecutionStatus::Shutdown,
        };

        let editor_block = EditorBlock::new(self.editor.clone(), anchor_range, status, cx);

        self.blocks
            .insert(message.header.msg_id.clone(), editor_block);

        match &self.kernel {
            Kernel::RunningKernel(_) => {
                self.send(message, cx).ok();
            }
            Kernel::StartingKernel(task) => {
                // Queue up the execution as a task to run after the kernel starts
                let task = task.clone();
                let message = message.clone();

                cx.spawn(|this, mut cx| async move {
                    task.await;
                    this.update(&mut cx, |this, cx| {
                        this.send(message, cx).ok();
                    })
                    .ok();
                })
                .detach();
            }
            _ => {}
        }
    }

    fn route(&mut self, message: &JupyterMessage, cx: &mut ViewContext<Self>) {
        let parent_message_id = match message.parent_header.as_ref() {
            Some(header) => &header.msg_id,
            None => return,
        };

        match &message.content {
            JupyterMessageContent::Status(status) => {
                self.kernel.set_execution_state(&status.execution_state);
                cx.notify();
            }
            JupyterMessageContent::KernelInfoReply(reply) => {
                self.kernel.set_kernel_info(&reply);
                cx.notify();
            }
            _ => {}
        }

        if let Some(block) = self.blocks.get_mut(parent_message_id) {
            block.handle_message(&message, cx);
            return;
        }
    }

    fn interrupt(&mut self, cx: &mut ViewContext<Self>) {
        match &mut self.kernel {
            Kernel::RunningKernel(_kernel) => {
                self.send(InterruptRequest {}.into(), cx).ok();
            }
            Kernel::StartingKernel(_task) => {
                // NOTE: If we switch to a literal queue instead of chaining on to the task, clear all queued executions
            }
            _ => {}
        }
    }

    fn shutdown(&mut self, cx: &mut ViewContext<Self>) {
        let kernel = std::mem::replace(&mut self.kernel, Kernel::ShuttingDown);
        // todo!(): emit event for the runtime panel to remove this session once in shutdown state

        match kernel {
            Kernel::RunningKernel(mut kernel) => {
                let mut request_tx = kernel.request_tx.clone();

                cx.spawn(|this, mut cx| async move {
                    let message: JupyterMessage = ShutdownRequest { restart: false }.into();
                    request_tx.try_send(message).ok();

                    // Give the kernel a bit of time to clean up
                    cx.background_executor().timer(Duration::from_secs(3)).await;

                    kernel.process.kill().ok();

                    this.update(&mut cx, |this, cx| {
                        cx.emit(SessionEvent::Shutdown(this.editor.clone()));
                        this.clear_outputs(cx);
                        this.kernel = Kernel::Shutdown;
                        cx.notify();
                    })
                    .ok();
                })
                .detach();
            }
            Kernel::StartingKernel(_kernel) => {
                self.kernel = Kernel::Shutdown;
            }
            _ => {
                self.kernel = Kernel::Shutdown;
            }
        }
        cx.notify();
    }
}

pub enum SessionEvent {
    Shutdown(View<Editor>),
}

impl EventEmitter<SessionEvent> for Session {}

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
                        .on_click(cx.listener(move |session, _, cx| {
                            session.interrupt(cx);
                        })),
                );
                let mut name = self.kernel_specification.name.clone();

                if let Some(info) = &kernel.kernel_info {
                    name.push_str(" (");
                    name.push_str(&info.language_info.name);
                    name.push_str(")");
                }
                name
            }
            Kernel::StartingKernel(_) => format!("{} (Starting)", self.kernel_specification.name),
            Kernel::ErroredLaunch(err) => {
                format!("{} (Error: {})", self.kernel_specification.name, err)
            }
            Kernel::ShuttingDown => format!("{} (Shutting Down)", self.kernel_specification.name),
            Kernel::Shutdown => format!("{} (Shutdown)", self.kernel_specification.name),
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
