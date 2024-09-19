use crate::{Editor, EditorEvent};
use clock::LOCAL_BRANCH_REPLICA_ID;
use collections::HashMap;
use gpui::{AppContext, EntityId, EventEmitter, FocusableView, Model, Render, View};
use language::{Buffer, Capability};
use multi_buffer::{ExcerptRange, MultiBuffer};
use project::Project;
use std::ops::Range;
use ui::prelude::*;
use workspace::Item;

pub struct StagedChangesEditor {
    editor: View<Editor>,
    bases_by_buffer_id: HashMap<EntityId, Model<Buffer>>,
}

pub struct StagedChangeBuffer {
    pub buffer: Model<Buffer>,
    pub ranges: Vec<Range<usize>>,
}

impl StagedChangesEditor {
    pub fn new(
        buffers: &[StagedChangeBuffer],
        project: Option<Model<Project>>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let mut bases_by_buffer_id = HashMap::default();
        let multibuffer = cx.new_model(|cx| {
            let mut multibuffer = MultiBuffer::new(LOCAL_BRANCH_REPLICA_ID, Capability::ReadWrite);
            for buffer in buffers {
                let branch_buffer = buffer.buffer.update(cx, |buffer, cx| buffer.branch(cx));
                bases_by_buffer_id.insert(branch_buffer.entity_id(), buffer.buffer.clone());
                multibuffer.push_excerpts(
                    branch_buffer,
                    buffer.ranges.iter().map(|range| ExcerptRange {
                        context: range.clone(),
                        primary: None,
                    }),
                    cx,
                );
            }
            multibuffer
        });

        Self {
            editor: cx.new_view(|cx| Editor::for_multibuffer(multibuffer, project, true, cx)),
            bases_by_buffer_id,
        }
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
