use crate::{Editor, EditorEvent};
use collections::HashSet;
use futures::{channel::mpsc, future::join_all};
use gpui::{AppContext, EventEmitter, FocusableView, Model, Render, Subscription, Task, View};
use language::{Buffer, BufferEvent, Capability};
use multi_buffer::{ExcerptRange, MultiBuffer};
use project::Project;
use smol::stream::StreamExt;
use std::{ops::Range, time::Duration};
use text::ToOffset;
use ui::prelude::*;
use workspace::Item;

pub struct ProposedChangesEditor {
    editor: View<Editor>,
    _subscriptions: Vec<Subscription>,
    _recalculate_diffs_task: Task<Option<()>>,
    recalculate_diffs_tx: mpsc::UnboundedSender<Model<Buffer>>,
}

pub struct ProposedChangesBuffer<T> {
    pub buffer: Model<Buffer>,
    pub ranges: Vec<Range<T>>,
}

impl ProposedChangesEditor {
    pub fn new<T: ToOffset>(
        buffers: Vec<ProposedChangesBuffer<T>>,
        project: Option<Model<Project>>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let mut subscriptions = Vec::new();
        let multibuffer = cx.new_model(|_| MultiBuffer::new(Capability::ReadWrite));

        for buffer in buffers {
            let branch_buffer = buffer.buffer.update(cx, |buffer, cx| buffer.branch(cx));
            subscriptions.push(cx.subscribe(&branch_buffer, Self::on_buffer_event));

            multibuffer.update(cx, |multibuffer, cx| {
                multibuffer.push_excerpts(
                    branch_buffer,
                    buffer.ranges.into_iter().map(|range| ExcerptRange {
                        context: range,
                        primary: None,
                    }),
                    cx,
                );
            });
        }

        let (recalculate_diffs_tx, mut recalculate_diffs_rx) = mpsc::unbounded();

        Self {
            editor: cx
                .new_view(|cx| Editor::for_multibuffer(multibuffer.clone(), project, true, cx)),
            recalculate_diffs_tx,
            _recalculate_diffs_task: cx.spawn(|_, mut cx| async move {
                let mut buffers_to_diff = HashSet::default();
                while let Some(buffer) = recalculate_diffs_rx.next().await {
                    buffers_to_diff.insert(buffer);

                    loop {
                        cx.background_executor()
                            .timer(Duration::from_millis(250))
                            .await;
                        let mut had_further_changes = false;
                        while let Ok(next_buffer) = recalculate_diffs_rx.try_next() {
                            buffers_to_diff.insert(next_buffer?);
                            had_further_changes = true;
                        }
                        if !had_further_changes {
                            break;
                        }
                    }

                    join_all(buffers_to_diff.drain().filter_map(|buffer| {
                        buffer
                            .update(&mut cx, |buffer, cx| buffer.recalculate_diff(cx))
                            .ok()?
                    }))
                    .await;
                }
                None
            }),
            _subscriptions: subscriptions,
        }
    }

    fn on_buffer_event(
        &mut self,
        buffer: Model<Buffer>,
        event: &BufferEvent,
        _cx: &mut ViewContext<Self>,
    ) {
        if let BufferEvent::Edited = event {
            self.recalculate_diffs_tx.unbounded_send(buffer).ok();
        }
    }
}

impl Render for ProposedChangesEditor {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        self.editor.clone()
    }
}

impl FocusableView for ProposedChangesEditor {
    fn focus_handle(&self, cx: &AppContext) -> gpui::FocusHandle {
        self.editor.focus_handle(cx)
    }
}

impl EventEmitter<EditorEvent> for ProposedChangesEditor {}

impl Item for ProposedChangesEditor {
    type Event = EditorEvent;

    fn tab_icon(&self, _cx: &ui::WindowContext) -> Option<Icon> {
        Some(Icon::new(IconName::Pencil))
    }

    fn tab_content_text(&self, _cx: &WindowContext) -> Option<SharedString> {
        Some("Proposed changes".into())
    }
}
