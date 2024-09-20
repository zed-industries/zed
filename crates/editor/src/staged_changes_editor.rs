use crate::{CustomBlockId, Editor, EditorEvent};
use clock::LOCAL_BRANCH_REPLICA_ID;
use collections::HashMap;
use gpui::{AppContext, EntityId, EventEmitter, FocusableView, Model, Render, Subscription, View};
use language::{Buffer, BufferEvent, Capability};
use multi_buffer::{Anchor, ExcerptRange, MultiBuffer};
use project::Project;
use std::{cmp::Ordering, ops::Range};
use text::{BufferId, ToOffset};
use ui::prelude::*;
use workspace::Item;

pub struct StagedChangesEditor {
    editor: View<Editor>,
    multibuffer: Model<MultiBuffer>,
    bases_by_buffer_id: HashMap<EntityId, Model<Buffer>>,
    base_buffers: HashMap<BufferId, Model<Buffer>>,
    diff: StagedChangesDiff,
    _subscriptions: Vec<Subscription>,
}

#[derive(Default)]
struct StagedChangesDiff {
    insertions: Vec<Insertion>,
    deletions: Vec<Deletion>,
}

struct Insertion {
    range: Range<Anchor>,
}

struct Deletion {
    block_id: CustomBlockId,
    position: Anchor,
    base_buffer_id: BufferId,
    base_buffer_range: Range<text::Anchor>,
}

pub struct StagedChangeBuffer<T> {
    pub buffer: Model<Buffer>,
    pub ranges: Vec<Range<T>>,
}

impl StagedChangesEditor {
    pub fn new<T: ToOffset>(
        buffers: Vec<StagedChangeBuffer<T>>,
        project: Option<Model<Project>>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let mut subscriptions = Vec::new();
        let mut bases_by_buffer_id = HashMap::default();
        let multibuffer =
            cx.new_model(|_| MultiBuffer::new(LOCAL_BRANCH_REPLICA_ID, Capability::ReadWrite));

        for buffer in buffers {
            let branch_buffer = buffer.buffer.update(cx, |buffer, cx| buffer.branch(cx));
            subscriptions.push(cx.subscribe(&branch_buffer, Self::on_buffer_event));

            multibuffer.update(cx, |multibuffer, cx| {
                bases_by_buffer_id.insert(branch_buffer.entity_id(), buffer.buffer.clone());
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

        let base_buffers = bases_by_buffer_id
            .values()
            .map(|buffer| (buffer.read(cx).remote_id(), buffer.clone()))
            .collect();

        Self {
            editor: cx
                .new_view(|cx| Editor::for_multibuffer(multibuffer.clone(), project, true, cx)),
            multibuffer,
            base_buffers,
            bases_by_buffer_id,
            diff: Default::default(),
            _subscriptions: subscriptions,
        }
    }

    fn on_buffer_event(
        &mut self,
        buffer: Model<Buffer>,
        event: &BufferEvent,
        cx: &mut ViewContext<Self>,
    ) {
        if let BufferEvent::Edited = event {
            self.update_diff(buffer, cx);
        }
    }

    fn update_diff(&mut self, buffer: Model<Buffer>, cx: &mut ViewContext<Self>) {
        let multibuffer = self.multibuffer.read(cx);
        let excerpts = multibuffer.excerpts_for_buffer(&buffer, cx);
        if excerpts.is_empty() {
            return;
        }

        let snapshot = multibuffer.read(cx);
        let start_excerpt = excerpts.first().unwrap();
        let end_excerpt = excerpts.last().unwrap();
        let start_anchor = snapshot
            .anchor_in_excerpt(start_excerpt.0, start_excerpt.1.context.start)
            .unwrap();
        let end_anchor = snapshot
            .anchor_in_excerpt(end_excerpt.0, end_excerpt.1.context.end)
            .unwrap();

        let insertions_start_ix = self
            .diff
            .insertions
            .binary_search_by(|insertion| {
                insertion
                    .range
                    .end
                    .cmp(&start_anchor, &*snapshot)
                    .then(Ordering::Greater)
            })
            .unwrap_err();
        let insertions_end_ix = self
            .diff
            .insertions
            .binary_search_by(|insertion| {
                insertion
                    .range
                    .start
                    .cmp(&end_anchor, &*snapshot)
                    .then(Ordering::Less)
            })
            .unwrap_err();
    }
}

impl Render for StagedChangesEditor {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        self.editor.clone()
    }
}

impl FocusableView for StagedChangesEditor {
    fn focus_handle(&self, cx: &AppContext) -> gpui::FocusHandle {
        self.editor.focus_handle(cx)
    }
}

impl EventEmitter<EditorEvent> for StagedChangesEditor {}

impl Item for StagedChangesEditor {
    type Event = EditorEvent;

    fn tab_icon(&self, _cx: &ui::WindowContext) -> Option<Icon> {
        Some(Icon::new(IconName::Pencil))
    }

    fn tab_content_text(&self, _cx: &WindowContext) -> Option<SharedString> {
        Some("Proposed changes".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor_tests::init_test;
    use gpui::{TestAppContext, VisualTestContext};
    use multi_buffer::{AnchorRangeExt, ToOffset as _};
    use text::Point;
    use unindent::Unindent as _;

    #[gpui::test]
    fn test_staged_changes_editor(cx: &mut TestAppContext) {
        init_test(cx, |_| {});

        let text = "
            zero
            one
            two
            three
            four
            five
            six
        "
        .unindent();

        let buffer = cx.new_model(|cx| Buffer::local(&text, cx));
        let staged_change_buffers = vec![StagedChangeBuffer {
            buffer,
            ranges: vec![Point::new(2, 0)..Point::new(6, 0)],
        }];

        let (editor, cx) =
            cx.add_window_view(|cx| StagedChangesEditor::new(staged_change_buffers, None, cx));
        let multibuffer = editor.update(cx, |editor, _cx| editor.multibuffer.clone());

        multibuffer.update(cx, |multibuffer, cx| {
            assert_eq!(
                multibuffer.read(cx).text(),
                "
                    two
                    three
                    four
                    five
                "
                .unindent()
            );
        });

        multibuffer.update(cx, |multibuffer, cx| {
            multibuffer.edit(
                [
                    (Point::new(1, 5)..Point::new(1, 5), "!"),
                    (Point::new(2, 4)..Point::new(2, 4), "?"),
                ],
                None,
                cx,
            );
        });

        expect_diff(
            &editor,
            cx,
            "
              two
            - three
            - four
            + three!
            + four?
              five
            ",
        );
    }

    fn expect_diff(
        editor: &View<StagedChangesEditor>,
        cx: &mut VisualTestContext,
        expected_diff: &str,
    ) {
        let expected = expected_diff.unindent();
        let mut expected_text = String::new();
        let mut expected_insertions = Vec::<Range<usize>>::new();
        let mut expected_deletions = Vec::<(usize, String)>::new();

        for line in expected.lines() {
            let (prefix, content) = line.split_at(2);
            let offset = expected_text.len();
            match prefix.trim() {
                "+" => {
                    let end = offset + content.len() + 1;
                    if let Some(last_insertion) = expected_insertions
                        .last_mut()
                        .filter(|range| range.end == offset)
                    {
                        last_insertion.end = end;
                    } else {
                        expected_insertions.push(offset..end);
                    }
                    expected_text.push_str(content);
                    expected_text.push('\n');
                }
                "-" => {
                    let mut content = content.to_string();
                    content.push('\n');
                    if let Some(last_deletion) = expected_deletions
                        .last_mut()
                        .filter(|(position, _)| *position == offset)
                    {
                        last_deletion.1.push_str(&content);
                    } else {
                        expected_deletions.push((offset, content));
                    }
                }
                "" => {
                    expected_text.push_str(content);
                    expected_text.push('\n');
                }
                _ => panic!("invalid line prefix {prefix:?} in expected diff"),
            }
        }

        editor.update(cx, |editor, cx| {
            let multibuffer = editor.multibuffer.read(cx).read(cx);
            let actual_insertions = editor
                .diff
                .insertions
                .iter()
                .map(|insertion| insertion.range.to_offset(&*multibuffer))
                .collect::<Vec<_>>();
            let actual_deletions = editor
                .diff
                .deletions
                .iter()
                .map(|deletion| {
                    let base_buffer = &editor.base_buffers[&deletion.base_buffer_id];
                    let old_text = base_buffer
                        .read(cx)
                        .text_for_range(deletion.base_buffer_range.clone())
                        .collect();
                    (deletion.position.to_offset(&*multibuffer), old_text)
                })
                .collect::<Vec<_>>();

            assert_eq!(multibuffer.text(), expected_text);
            assert_eq!(actual_insertions, expected_insertions);
            assert_eq!(actual_deletions, expected_deletions);
        });
    }
}
