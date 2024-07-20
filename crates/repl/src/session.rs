use crate::{
    kernels::{Kernel, KernelSpecification, RunningKernel},
    outputs::{ExecutionStatus, ExecutionView, LineHeight as _},
};
use collections::{HashMap, HashSet};
use editor::{
    display_map::{
        BlockContext, BlockDisposition, BlockId, BlockProperties, BlockStyle, CustomBlockId,
        RenderBlock,
    },
    scroll::Autoscroll,
    Anchor, AnchorRangeExt as _, Editor, MultiBuffer, ToPoint,
};
use futures::{FutureExt as _, StreamExt as _};
use gpui::{
    div, prelude::*, EntityId, EventEmitter, Model, Render, Subscription, Task, View, ViewContext,
    WeakView,
};
use language::Point;
use project::Fs;
use runtimelib::{
    ExecuteRequest, InterruptRequest, JupyterMessage, JupyterMessageContent, ShutdownRequest,
};
use settings::Settings as _;
use std::{env::temp_dir, ops::Range, path::PathBuf, sync::Arc, time::Duration};
use theme::{ActiveTheme, ThemeSettings};
use ui::{h_flex, prelude::*, v_flex, ButtonLike, ButtonStyle, IconButtonShape, Label, Tooltip};

pub struct Session {
    pub editor: WeakView<Editor>,
    pub kernel: Kernel,
    blocks: HashMap<String, EditorBlock>,
    pub messaging_task: Task<()>,
    pub kernel_specification: KernelSpecification,
    _buffer_subscription: Subscription,
}

struct EditorBlock {
    editor: WeakView<Editor>,
    code_range: Range<Anchor>,
    invalidation_anchor: Anchor,
    block_id: CustomBlockId,
    execution_view: View<ExecutionView>,
    on_close: CloseBlockFn,
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
                height: execution_view.num_lines(cx).saturating_add(1),
                style: BlockStyle::Sticky,
                render: Self::create_output_area_render(execution_view.clone(), on_close.clone()),
                disposition: BlockDisposition::Below,
            };

            let block_id = editor.insert_blocks([block], None, cx)[0];
            (block_id, invalidation_anchor)
        })?;

        anyhow::Ok(Self {
            editor,
            code_range,
            invalidation_anchor,
            block_id,
            execution_view,
            on_close,
        })
    }

    fn handle_message(&mut self, message: &JupyterMessage, cx: &mut ViewContext<Session>) {
        self.execution_view.update(cx, |execution_view, cx| {
            execution_view.push_message(&message.content, cx);
        });

        self.editor
            .update(cx, |editor, cx| {
                let mut replacements = HashMap::default();

                replacements.insert(
                    self.block_id,
                    (
                        Some(self.execution_view.num_lines(cx).saturating_add(1)),
                        Self::create_output_area_render(
                            self.execution_view.clone(),
                            self.on_close.clone(),
                        ),
                    ),
                );
                editor.replace_blocks(replacements, None, cx);
            })
            .ok();
    }

    fn create_output_area_render(
        execution_view: View<ExecutionView>,
        on_close: CloseBlockFn,
    ) -> RenderBlock {
        let render = move |cx: &mut BlockContext| {
            let execution_view = execution_view.clone();
            let text_font = ThemeSettings::get_global(cx).buffer_font.family.clone();
            let text_font_size = ThemeSettings::get_global(cx).buffer_font_size;

            let gutter = cx.gutter_dimensions;
            let close_button_size = IconSize::XSmall;

            let block_id = cx.block_id;
            let on_close = on_close.clone();

            let rem_size = cx.rem_size();
            let line_height = cx.text_style().line_height_in_pixels(rem_size);

            let (close_button_width, close_button_padding) =
                close_button_size.square_components(cx);

            div()
                .min_h(line_height)
                .flex()
                .flex_row()
                .items_start()
                .w_full()
                .bg(cx.theme().colors().background)
                .border_y_1()
                .border_color(cx.theme().colors().border)
                .child(
                    v_flex().min_h(cx.line_height()).justify_center().child(
                        h_flex()
                            .w(gutter.full_width())
                            .justify_end()
                            .pt(line_height / 2.)
                            .child(
                                h_flex()
                                    .pr(gutter.width / 2. - close_button_width
                                        + close_button_padding / 2.)
                                    .child(
                                        IconButton::new(
                                            ("close_output_area", EntityId::from(cx.block_id)),
                                            IconName::Close,
                                        )
                                        .shape(IconButtonShape::Square)
                                        .icon_size(close_button_size)
                                        .icon_color(Color::Muted)
                                        .tooltip(|cx| Tooltip::text("Close output area", cx))
                                        .on_click(
                                            move |_, cx| {
                                                if let BlockId::Custom(block_id) = block_id {
                                                    (on_close)(block_id, cx)
                                                }
                                            },
                                        ),
                                    ),
                            ),
                    ),
                )
                .child(
                    div()
                        .flex_1()
                        .size_full()
                        .my_2()
                        .mr(gutter.width)
                        .text_size(text_font_size)
                        .font_family(text_font)
                        .child(execution_view),
                )
                .into_any_element()
        };

        Box::new(render)
    }
}

impl Session {
    pub fn working_directory(editor: WeakView<Editor>, cx: &WindowContext) -> PathBuf {
        if let Some(working_directory) = editor
            .upgrade()
            .and_then(|editor| editor.read(cx).working_directory(cx))
        {
            working_directory
        } else {
            temp_dir()
        }
    }

    pub fn new(
        editor: WeakView<Editor>,
        fs: Arc<dyn Fs>,
        kernel_specification: KernelSpecification,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let entity_id = editor.entity_id();

        let kernel = RunningKernel::new(
            kernel_specification.clone(),
            entity_id,
            Self::working_directory(editor.clone(), cx),
            fs.clone(),
            cx,
        );

        let pending_kernel = cx
            .spawn(|this, mut cx| async move {
                let kernel = kernel.await;

                match kernel {
                    Ok((mut kernel, mut messages_rx)) => {
                        this.update(&mut cx, |this, cx| {
                            // At this point we can create a new kind of kernel that has the process and our long running background tasks

                            let status = kernel.process.status();
                            this.kernel = Kernel::RunningKernel(kernel);

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
                                        session.kernel =
                                            Kernel::ErroredLaunch(error_message.clone());

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

                            this.messaging_task = cx.spawn(|session, mut cx| async move {
                                while let Some(message) = messages_rx.next().await {
                                    session
                                        .update(&mut cx, |session, cx| {
                                            session.route(&message, cx);
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

    pub fn execute(&mut self, code: &str, anchor_range: Range<Anchor>, cx: &mut ViewContext<Self>) {
        let editor = if let Some(editor) = self.editor.upgrade() {
            editor
        } else {
            return;
        };

        if code.is_empty() {
            return;
        }

        let execute_request = ExecuteRequest {
            code: code.to_string(),
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

        let editor_block = if let Ok(editor_block) =
            EditorBlock::new(self.editor.clone(), anchor_range, status, on_close, cx)
        {
            editor_block
        } else {
            return;
        };

        let new_cursor_pos = editor_block.invalidation_anchor;

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

        // Now move the cursor to after the block
        editor.update(cx, move |editor, cx| {
            editor.change_selections(Some(Autoscroll::top_relative(8)), cx, |selections| {
                selections.select_ranges([new_cursor_pos..new_cursor_pos]);
            });
        });
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
    Shutdown(WeakView<Editor>),
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
