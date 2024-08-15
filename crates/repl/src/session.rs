use crate::components::KernelListItem;
use crate::{
    kernels::{Kernel, KernelSpecification, RunningKernel},
    outputs::{ExecutionStatus, ExecutionView},
};
use crate::{stdio, KernelStatus};
use client::telemetry::Telemetry;
use collections::{HashMap, HashSet};
use editor::{
    display_map::{
        BlockContext, BlockDisposition, BlockId, BlockProperties, BlockStyle, CustomBlockId,
        RenderBlock,
    },
    scroll::Autoscroll,
    Anchor, AnchorRangeExt as _, Editor, MultiBuffer, ToPoint,
};
use futures::io::BufReader;
use futures::{AsyncBufReadExt as _, FutureExt as _, StreamExt as _};
use gpui::{
    div, prelude::*, EntityId, EventEmitter, Model, Render, Subscription, Task, View, ViewContext,
    WeakView,
};
use language::Point;
use project::Fs;
use runtimelib::{
    ExecuteRequest, ExecutionState, InterruptRequest, JupyterMessage, JupyterMessageContent,
    ShutdownRequest,
};
use std::{env::temp_dir, ops::Range, sync::Arc, time::Duration};
use theme::ActiveTheme;
use ui::{prelude::*, IconButtonShape, Tooltip};

pub struct Session {
    editor: WeakView<Editor>,
    pub kernel: Kernel,
    blocks: HashMap<String, EditorBlock>,
    messaging_task: Task<()>,
    pub kernel_specification: KernelSpecification,
    telemetry: Arc<Telemetry>,
    _buffer_subscription: Subscription,
}

struct EditorBlock {
    code_range: Range<Anchor>,
    invalidation_anchor: Anchor,
    block_id: CustomBlockId,
    execution_view: View<ExecutionView>,
}

type CloseBlockFn =
    Arc<dyn for<'a> Fn(CustomBlockId, &'a mut WindowContext) + Send + Sync + 'static>;

impl EditorBlock {
    fn new(
        editor: WeakView<Editor>,
        code_range: Range<Anchor>,
        status: ExecutionStatus,
        on_close: CloseBlockFn,
        cx: &mut ViewContext<Session>,
    ) -> anyhow::Result<Self> {
        let execution_view = cx.new_view(|cx| ExecutionView::new(status, cx));

        let (block_id, invalidation_anchor) = editor.update(cx, |editor, cx| {
            let buffer = editor.buffer().clone();
            let buffer_snapshot = buffer.read(cx).snapshot(cx);
            let end_point = code_range.end.to_point(&buffer_snapshot);
            let next_row_start = end_point + Point::new(1, 0);
            if next_row_start > buffer_snapshot.max_point() {
                buffer.update(cx, |buffer, cx| {
                    buffer.edit(
                        [(
                            buffer_snapshot.max_point()..buffer_snapshot.max_point(),
                            "\n",
                        )],
                        None,
                        cx,
                    )
                });
            }

            let invalidation_anchor = buffer.read(cx).read(cx).anchor_before(next_row_start);
            let block = BlockProperties {
                position: code_range.end,
                // Take up at least one height for status, allow the editor to determine the real height based on the content from render
                height: 1,
                style: BlockStyle::Sticky,
                render: Self::create_output_area_renderer(execution_view.clone(), on_close.clone()),
                disposition: BlockDisposition::Below,
                priority: 0,
            };

            let block_id = editor.insert_blocks([block], None, cx)[0];
            (block_id, invalidation_anchor)
        })?;

        anyhow::Ok(Self {
            code_range,
            invalidation_anchor,
            block_id,
            execution_view,
        })
    }

    fn handle_message(&mut self, message: &JupyterMessage, cx: &mut ViewContext<Session>) {
        self.execution_view.update(cx, |execution_view, cx| {
            execution_view.push_message(&message.content, cx);
        });
    }

    fn create_output_area_renderer(
        execution_view: View<ExecutionView>,
        on_close: CloseBlockFn,
    ) -> RenderBlock {
        let render = move |cx: &mut BlockContext| {
            let execution_view = execution_view.clone();
            let text_style = stdio::text_style(cx);

            let gutter = cx.gutter_dimensions;

            let block_id = cx.block_id;
            let on_close = on_close.clone();

            let rem_size = cx.rem_size();

            let text_line_height = text_style.line_height_in_pixels(rem_size);

            let close_button = h_flex()
                .flex_none()
                .items_center()
                .justify_center()
                .absolute()
                .top(text_line_height / 2.)
                .right(
                    // 2px is a magic number to nudge the button just a bit closer to
                    // the line number start
                    gutter.full_width() / 2.0 - text_line_height / 2.0 - px(2.),
                )
                .w(text_line_height)
                .h(text_line_height)
                .child(
                    IconButton::new(
                        ("close_output_area", EntityId::from(cx.block_id)),
                        IconName::Close,
                    )
                    .icon_size(IconSize::Small)
                    .icon_color(Color::Muted)
                    .size(ButtonSize::Compact)
                    .shape(IconButtonShape::Square)
                    .tooltip(|cx| Tooltip::text("Close output area", cx))
                    .on_click(move |_, cx| {
                        if let BlockId::Custom(block_id) = block_id {
                            (on_close)(block_id, cx)
                        }
                    }),
                );

            div()
                .flex()
                .items_start()
                .min_h(text_line_height)
                .w_full()
                .border_y_1()
                .border_color(cx.theme().colors().border)
                .bg(cx.theme().colors().background)
                .child(
                    div()
                        .relative()
                        .w(gutter.full_width())
                        .h(text_line_height * 2)
                        .child(close_button),
                )
                .child(
                    div()
                        .flex_1()
                        .size_full()
                        .py(text_line_height / 2.)
                        .mr(gutter.width)
                        .child(execution_view),
                )
                .into_any_element()
        };

        Box::new(render)
    }
}

impl Session {
    pub fn new(
        editor: WeakView<Editor>,
        fs: Arc<dyn Fs>,
        telemetry: Arc<Telemetry>,
        kernel_specification: KernelSpecification,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let kernel_language = kernel_specification.kernelspec.language.clone();

        telemetry.report_repl_event(
            kernel_language.clone(),
            KernelStatus::Starting.to_string(),
            cx.entity_id().to_string(),
        );

        let entity_id = editor.entity_id();
        let working_directory = editor
            .upgrade()
            .and_then(|editor| editor.read(cx).working_directory(cx))
            .unwrap_or_else(temp_dir);
        let kernel = RunningKernel::new(
            kernel_specification.clone(),
            entity_id,
            working_directory,
            fs.clone(),
            cx,
        );

        let pending_kernel = cx
            .spawn(|this, mut cx| async move {
                let kernel = kernel.await;

                match kernel {
                    Ok((mut kernel, mut messages_rx)) => {
                        this.update(&mut cx, |session, cx| {
                            let stderr = kernel.process.stderr.take();

                            cx.spawn(|_session, mut _cx| async move {
                                if let None = stderr {
                                    return;
                                }
                                let reader = BufReader::new(stderr.unwrap());
                                let mut lines = reader.lines();
                                while let Some(Ok(line)) = lines.next().await {
                                    log::error!("kernel: {}", line);
                                }
                            })
                            .detach();

                            let stdout = kernel.process.stdout.take();

                            cx.spawn(|_session, mut _cx| async move {
                                if let None = stdout {
                                    return;
                                }
                                let reader = BufReader::new(stdout.unwrap());
                                let mut lines = reader.lines();
                                while let Some(Ok(line)) = lines.next().await {
                                    log::info!("kernel: {}", line);
                                }
                            })
                            .detach();

                            let status = kernel.process.status();
                            session.kernel(Kernel::RunningKernel(kernel), cx);

                            cx.spawn(|session, mut cx| async move {
                                let error_message = match status.await {
                                    Ok(status) => {
                                        if status.success() {
                                            log::info!("kernel process exited successfully");
                                            return;
                                        }

                                        format!("kernel process exited with status: {:?}", status)
                                    }
                                    Err(err) => {
                                        format!("kernel process exited with error: {:?}", err)
                                    }
                                };

                                log::error!("{}", error_message);

                                session
                                    .update(&mut cx, |session, cx| {
                                        session.kernel(
                                            Kernel::ErroredLaunch(error_message.clone()),
                                            cx,
                                        );

                                        session.blocks.values().for_each(|block| {
                                            block.execution_view.update(
                                                cx,
                                                |execution_view, cx| {
                                                    match execution_view.status {
                                                        ExecutionStatus::Finished => {
                                                            // Do nothing when the output was good
                                                        }
                                                        _ => {
                                                            // All other cases, set the status to errored
                                                            execution_view.status =
                                                                ExecutionStatus::KernelErrored(
                                                                    error_message.clone(),
                                                                )
                                                        }
                                                    }
                                                    cx.notify();
                                                },
                                            );
                                        });

                                        cx.notify();
                                    })
                                    .ok();
                            })
                            .detach();

                            session.messaging_task = cx.spawn(|session, mut cx| async move {
                                while let Some(message) = messages_rx.next().await {
                                    session
                                        .update(&mut cx, |session, cx| {
                                            session.route(&message, cx);
                                        })
                                        .ok();
                                }
                            });

                            // todo!(@rgbkrk): send kernelinforequest once our shell channel read/writes are split
                            // cx.spawn(|this, mut cx| async move {
                            //     cx.background_executor()
                            //         .timer(Duration::from_millis(120))
                            //         .await;
                            //     this.update(&mut cx, |this, cx| {
                            //         this.send(KernelInfoRequest {}.into(), cx).ok();
                            //     })
                            //     .ok();
                            // })
                            // .detach();
                        })
                        .ok();
                    }
                    Err(err) => {
                        this.update(&mut cx, |session, cx| {
                            session.kernel(Kernel::ErroredLaunch(err.to_string()), cx);
                        })
                        .ok();
                    }
                }
            })
            .shared();

        let subscription = match editor.upgrade() {
            Some(editor) => {
                let buffer = editor.read(cx).buffer().clone();
                cx.subscribe(&buffer, Self::on_buffer_event)
            }
            None => Subscription::new(|| {}),
        };

        return Self {
            editor,
            kernel: Kernel::StartingKernel(pending_kernel),
            messaging_task: Task::ready(()),
            blocks: HashMap::default(),
            kernel_specification,
            _buffer_subscription: subscription,
            telemetry,
        };
    }

    fn on_buffer_event(
        &mut self,
        buffer: Model<MultiBuffer>,
        event: &multi_buffer::Event,
        cx: &mut ViewContext<Self>,
    ) {
        if let multi_buffer::Event::Edited { .. } = event {
            let snapshot = buffer.read(cx).snapshot(cx);

            let mut blocks_to_remove: HashSet<CustomBlockId> = HashSet::default();

            self.blocks.retain(|_id, block| {
                if block.invalidation_anchor.is_valid(&snapshot) {
                    true
                } else {
                    blocks_to_remove.insert(block.block_id);
                    false
                }
            });

            if !blocks_to_remove.is_empty() {
                self.editor
                    .update(cx, |editor, cx| {
                        editor.remove_blocks(blocks_to_remove, None, cx);
                    })
                    .ok();
                cx.notify();
            }
        }
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
        let blocks_to_remove: HashSet<CustomBlockId> =
            self.blocks.values().map(|block| block.block_id).collect();

        self.editor
            .update(cx, |editor, cx| {
                editor.remove_blocks(blocks_to_remove, None, cx);
            })
            .ok();

        self.blocks.clear();
    }

    pub fn execute(
        &mut self,
        code: String,
        anchor_range: Range<Anchor>,
        next_cell: Option<Anchor>,
        move_down: bool,
        cx: &mut ViewContext<Self>,
    ) {
        let Some(editor) = self.editor.upgrade() else {
            return;
        };

        if code.is_empty() {
            return;
        }

        let execute_request = ExecuteRequest {
            code,
            ..ExecuteRequest::default()
        };

        let message: JupyterMessage = execute_request.into();

        let mut blocks_to_remove: HashSet<CustomBlockId> = HashSet::default();

        let buffer = editor.read(cx).buffer().read(cx).snapshot(cx);

        self.blocks.retain(|_key, block| {
            if anchor_range.overlaps(&block.code_range, &buffer) {
                blocks_to_remove.insert(block.block_id);
                false
            } else {
                true
            }
        });

        self.editor
            .update(cx, |editor, cx| {
                editor.remove_blocks(blocks_to_remove, None, cx);
            })
            .ok();

        let status = match &self.kernel {
            Kernel::RunningKernel(_) => ExecutionStatus::Queued,
            Kernel::StartingKernel(_) => ExecutionStatus::ConnectingToKernel,
            Kernel::ErroredLaunch(error) => ExecutionStatus::KernelErrored(error.clone()),
            Kernel::ShuttingDown => ExecutionStatus::ShuttingDown,
            Kernel::Shutdown => ExecutionStatus::Shutdown,
        };

        let parent_message_id = message.header.msg_id.clone();
        let session_view = cx.view().downgrade();
        let weak_editor = self.editor.clone();

        let on_close: CloseBlockFn =
            Arc::new(move |block_id: CustomBlockId, cx: &mut WindowContext| {
                if let Some(session) = session_view.upgrade() {
                    session.update(cx, |session, cx| {
                        session.blocks.remove(&parent_message_id);
                        cx.notify();
                    });
                }

                if let Some(editor) = weak_editor.upgrade() {
                    editor.update(cx, |editor, cx| {
                        let mut block_ids = HashSet::default();
                        block_ids.insert(block_id);
                        editor.remove_blocks(block_ids, None, cx);
                    });
                }
            });

        let Ok(editor_block) =
            EditorBlock::new(self.editor.clone(), anchor_range, status, on_close, cx)
        else {
            return;
        };

        let new_cursor_pos = if let Some(next_cursor) = next_cell {
            next_cursor
        } else {
            editor_block.invalidation_anchor
        };

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
                    this.update(&mut cx, |session, cx| {
                        session.send(message, cx).ok();
                    })
                    .ok();
                })
                .detach();
            }
            _ => {}
        }

        if move_down {
            editor.update(cx, move |editor, cx| {
                editor.change_selections(Some(Autoscroll::top_relative(8)), cx, |selections| {
                    selections.select_ranges([new_cursor_pos..new_cursor_pos]);
                });
            });
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

                self.telemetry.report_repl_event(
                    self.kernel_specification.kernelspec.language.clone(),
                    KernelStatus::from(&self.kernel).to_string(),
                    cx.entity_id().to_string(),
                );

                cx.notify();
            }
            JupyterMessageContent::KernelInfoReply(reply) => {
                self.kernel.set_kernel_info(&reply);
                cx.notify();
            }
            JupyterMessageContent::UpdateDisplayData(update) => {
                let display_id = if let Some(display_id) = update.transient.display_id.clone() {
                    display_id
                } else {
                    return;
                };

                self.blocks.iter_mut().for_each(|(_, block)| {
                    block.execution_view.update(cx, |execution_view, cx| {
                        execution_view.update_display_data(&update.data, &display_id, cx);
                    });
                });
                return;
            }
            _ => {}
        }

        if let Some(block) = self.blocks.get_mut(parent_message_id) {
            block.handle_message(&message, cx);
            return;
        }
    }

    pub fn interrupt(&mut self, cx: &mut ViewContext<Self>) {
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

    pub fn kernel(&mut self, kernel: Kernel, cx: &mut ViewContext<Self>) {
        if let Kernel::Shutdown = kernel {
            cx.emit(SessionEvent::Shutdown(self.editor.clone()));
        }

        let kernel_status = KernelStatus::from(&kernel).to_string();
        let kernel_language = self.kernel_specification.kernelspec.language.clone();

        self.telemetry.report_repl_event(
            kernel_language,
            kernel_status,
            cx.entity_id().to_string(),
        );

        self.kernel = kernel;
    }

    pub fn shutdown(&mut self, cx: &mut ViewContext<Self>) {
        let kernel = std::mem::replace(&mut self.kernel, Kernel::ShuttingDown);

        match kernel {
            Kernel::RunningKernel(mut kernel) => {
                let mut request_tx = kernel.request_tx.clone();

                cx.spawn(|this, mut cx| async move {
                    let message: JupyterMessage = ShutdownRequest { restart: false }.into();
                    request_tx.try_send(message).ok();

                    // Give the kernel a bit of time to clean up
                    cx.background_executor().timer(Duration::from_secs(3)).await;

                    kernel.process.kill().ok();

                    this.update(&mut cx, |session, cx| {
                        session.clear_outputs(cx);
                        session.kernel(Kernel::Shutdown, cx);
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
    Shutdown(WeakView<Editor>),
}

impl EventEmitter<SessionEvent> for Session {}

impl Render for Session {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let (status_text, interrupt_button) = match &self.kernel {
            Kernel::RunningKernel(kernel) => (
                kernel
                    .kernel_info
                    .as_ref()
                    .map(|info| info.language_info.name.clone()),
                Some(
                    Button::new("interrupt", "Interrupt")
                        .style(ButtonStyle::Subtle)
                        .on_click(cx.listener(move |session, _, cx| {
                            session.interrupt(cx);
                        })),
                ),
            ),
            Kernel::StartingKernel(_) => (Some("Starting".into()), None),
            Kernel::ErroredLaunch(err) => (Some(format!("Error: {err}")), None),
            Kernel::ShuttingDown => (Some("Shutting Down".into()), None),
            Kernel::Shutdown => (Some("Shutdown".into()), None),
        };

        KernelListItem::new(self.kernel_specification.clone())
            .status_color(match &self.kernel {
                Kernel::RunningKernel(kernel) => match kernel.execution_state {
                    ExecutionState::Idle => Color::Success,
                    ExecutionState::Busy => Color::Modified,
                },
                Kernel::StartingKernel(_) => Color::Modified,
                Kernel::ErroredLaunch(_) => Color::Error,
                Kernel::ShuttingDown => Color::Modified,
                Kernel::Shutdown => Color::Disabled,
            })
            .child(Label::new(self.kernel_specification.name.clone()))
            .children(status_text.map(|status_text| Label::new(format!("({status_text})"))))
            .button(
                Button::new("shutdown", "Shutdown")
                    .style(ButtonStyle::Subtle)
                    .disabled(self.kernel.is_shutting_down())
                    .on_click(cx.listener(move |session, _, cx| {
                        session.shutdown(cx);
                    })),
            )
            .buttons(interrupt_button)
    }
}
