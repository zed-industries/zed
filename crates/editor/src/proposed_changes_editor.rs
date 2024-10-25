use crate::{ApplyAllDiffHunks, Editor, EditorEvent, SemanticsProvider};
use collections::HashSet;
use futures::{channel::mpsc, future::join_all};
use gpui::{AppContext, EventEmitter, FocusableView, Model, Render, Subscription, Task, View};
use language::{Buffer, BufferEvent, Capability};
use multi_buffer::{ExcerptRange, MultiBuffer};
use project::Project;
use smol::stream::StreamExt;
use std::{any::TypeId, ops::Range, rc::Rc, time::Duration};
use text::ToOffset;
use ui::{prelude::*, ButtonLike, KeyBinding};
use workspace::{
    searchable::SearchableItemHandle, Item, ItemHandle as _, ToolbarItemEvent, ToolbarItemLocation,
    ToolbarItemView, Workspace,
};

pub struct ProposedChangesEditor {
    editor: View<Editor>,
    multibuffer: Model<MultiBuffer>,
    title: SharedString,
    buffer_entries: Vec<BufferEntry>,
    _recalculate_diffs_task: Task<Option<()>>,
    recalculate_diffs_tx: mpsc::UnboundedSender<RecalculateDiff>,
}

pub struct ProposedChangeLocation<T> {
    pub buffer: Model<Buffer>,
    pub ranges: Vec<Range<T>>,
}

struct BufferEntry {
    base: Model<Buffer>,
    branch: Model<Buffer>,
    _subscription: Subscription,
}

pub struct ProposedChangesEditorToolbar {
    current_editor: Option<View<ProposedChangesEditor>>,
}

struct RecalculateDiff {
    buffer: Model<Buffer>,
    debounce: bool,
}

/// A provider of code semantics for branch buffers.
///
/// Requests in edited regions will return nothing, but requests in unchanged
/// regions will be translated into the base buffer's coordinates.
struct BranchBufferSemanticsProvider(Rc<dyn SemanticsProvider>);

impl ProposedChangesEditor {
    pub fn new<T: ToOffset>(
        title: impl Into<SharedString>,
        locations: Vec<ProposedChangeLocation<T>>,
        project: Option<Model<Project>>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let multibuffer = cx.new_model(|_| MultiBuffer::new(Capability::ReadWrite));
        let (recalculate_diffs_tx, mut recalculate_diffs_rx) = mpsc::unbounded();
        let mut this = Self {
            editor: cx.new_view(|cx| {
                let mut editor = Editor::for_multibuffer(multibuffer.clone(), project, true, cx);
                editor.set_expand_all_diff_hunks();
                editor.set_completion_provider(None);
                editor.clear_code_action_providers();
                editor.set_semantics_provider(
                    editor
                        .semantics_provider()
                        .map(|provider| Rc::new(BranchBufferSemanticsProvider(provider)) as _),
                );
                editor
            }),
            multibuffer,
            title: title.into(),
            buffer_entries: Vec::new(),
            recalculate_diffs_tx,
            _recalculate_diffs_task: cx.spawn(|_, mut cx| async move {
                let mut buffers_to_diff = HashSet::default();
                while let Some(mut recalculate_diff) = recalculate_diffs_rx.next().await {
                    buffers_to_diff.insert(recalculate_diff.buffer);

                    while recalculate_diff.debounce {
                        cx.background_executor()
                            .timer(Duration::from_millis(50))
                            .await;
                        let mut had_further_changes = false;
                        while let Ok(next_recalculate_diff) = recalculate_diffs_rx.try_next() {
                            let next_recalculate_diff = next_recalculate_diff?;
                            recalculate_diff.debounce &= next_recalculate_diff.debounce;
                            buffers_to_diff.insert(next_recalculate_diff.buffer);
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
        };
        this.reset_locations(locations, cx);
        this
    }

    pub fn branch_buffer_for_base(&self, base_buffer: &Model<Buffer>) -> Option<Model<Buffer>> {
        self.buffer_entries.iter().find_map(|entry| {
            if &entry.base == base_buffer {
                Some(entry.branch.clone())
            } else {
                None
            }
        })
    }

    pub fn set_title(&mut self, title: SharedString, cx: &mut ViewContext<Self>) {
        self.title = title;
        cx.notify();
    }

    pub fn reset_locations<T: ToOffset>(
        &mut self,
        locations: Vec<ProposedChangeLocation<T>>,
        cx: &mut ViewContext<Self>,
    ) {
        // Undo all branch changes
        for entry in &self.buffer_entries {
            let base_version = entry.base.read(cx).version();
            entry.branch.update(cx, |buffer, cx| {
                let undo_counts = buffer
                    .operations()
                    .iter()
                    .filter_map(|(timestamp, _)| {
                        if !base_version.observed(*timestamp) {
                            Some((*timestamp, u32::MAX))
                        } else {
                            None
                        }
                    })
                    .collect();
                buffer.undo_operations(undo_counts, cx);
            });
        }

        self.multibuffer.update(cx, |multibuffer, cx| {
            multibuffer.clear(cx);
        });

        let mut buffer_entries = Vec::new();
        for location in locations {
            let branch_buffer;
            if let Some(ix) = self
                .buffer_entries
                .iter()
                .position(|entry| entry.base == location.buffer)
            {
                let entry = self.buffer_entries.remove(ix);
                branch_buffer = entry.branch.clone();
                buffer_entries.push(entry);
            } else {
                branch_buffer = location.buffer.update(cx, |buffer, cx| buffer.branch(cx));
                buffer_entries.push(BufferEntry {
                    branch: branch_buffer.clone(),
                    base: location.buffer.clone(),
                    _subscription: cx.subscribe(&branch_buffer, Self::on_buffer_event),
                });
            }

            self.multibuffer.update(cx, |multibuffer, cx| {
                multibuffer.push_excerpts(
                    branch_buffer,
                    location.ranges.into_iter().map(|range| ExcerptRange {
                        context: range,
                        primary: None,
                    }),
                    cx,
                );
            });
        }

        self.buffer_entries = buffer_entries;
        self.editor.update(cx, |editor, cx| {
            editor.change_selections(None, cx, |selections| selections.refresh())
        });
    }

    pub fn recalculate_all_buffer_diffs(&self) {
        for (ix, entry) in self.buffer_entries.iter().enumerate().rev() {
            self.recalculate_diffs_tx
                .unbounded_send(RecalculateDiff {
                    buffer: entry.branch.clone(),
                    debounce: ix > 0,
                })
                .ok();
        }
    }

    fn on_buffer_event(
        &mut self,
        buffer: Model<Buffer>,
        event: &BufferEvent,
        _cx: &mut ViewContext<Self>,
    ) {
        match event {
            BufferEvent::Operation { .. } => {
                self.recalculate_diffs_tx
                    .unbounded_send(RecalculateDiff {
                        buffer,
                        debounce: true,
                    })
                    .ok();
            }
            BufferEvent::DiffBaseChanged => {
                self.recalculate_diffs_tx
                    .unbounded_send(RecalculateDiff {
                        buffer,
                        debounce: false,
                    })
                    .ok();
            }
            _ => (),
        }
    }
}

impl Render for ProposedChangesEditor {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .size_full()
            .key_context("ProposedChangesEditor")
            .child(self.editor.clone())
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
        Some(Icon::new(IconName::Diff))
    }

    fn tab_content_text(&self, _cx: &WindowContext) -> Option<SharedString> {
        Some(self.title.clone())
    }

    fn as_searchable(&self, _: &View<Self>) -> Option<Box<dyn SearchableItemHandle>> {
        Some(Box::new(self.editor.clone()))
    }

    fn act_as_type<'a>(
        &'a self,
        type_id: TypeId,
        self_handle: &'a View<Self>,
        _: &'a AppContext,
    ) -> Option<gpui::AnyView> {
        if type_id == TypeId::of::<Self>() {
            Some(self_handle.to_any())
        } else if type_id == TypeId::of::<Editor>() {
            Some(self.editor.to_any())
        } else {
            None
        }
    }

    fn added_to_workspace(&mut self, workspace: &mut Workspace, cx: &mut ViewContext<Self>) {
        self.editor.update(cx, |editor, cx| {
            Item::added_to_workspace(editor, workspace, cx)
        });
    }

    fn deactivated(&mut self, cx: &mut ViewContext<Self>) {
        self.editor.update(cx, Item::deactivated);
    }

    fn navigate(&mut self, data: Box<dyn std::any::Any>, cx: &mut ViewContext<Self>) -> bool {
        self.editor
            .update(cx, |editor, cx| Item::navigate(editor, data, cx))
    }

    fn set_nav_history(
        &mut self,
        nav_history: workspace::ItemNavHistory,
        cx: &mut ViewContext<Self>,
    ) {
        self.editor.update(cx, |editor, cx| {
            Item::set_nav_history(editor, nav_history, cx)
        });
    }

    fn can_save(&self, cx: &AppContext) -> bool {
        self.editor.read(cx).can_save(cx)
    }

    fn save(
        &mut self,
        format: bool,
        project: Model<Project>,
        cx: &mut ViewContext<Self>,
    ) -> Task<gpui::Result<()>> {
        self.editor
            .update(cx, |editor, cx| Item::save(editor, format, project, cx))
    }
}

impl ProposedChangesEditorToolbar {
    pub fn new() -> Self {
        Self {
            current_editor: None,
        }
    }

    fn get_toolbar_item_location(&self) -> ToolbarItemLocation {
        if self.current_editor.is_some() {
            ToolbarItemLocation::PrimaryRight
        } else {
            ToolbarItemLocation::Hidden
        }
    }
}

impl Render for ProposedChangesEditorToolbar {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let button_like = ButtonLike::new("apply-changes").child(Label::new("Apply All"));

        match &self.current_editor {
            Some(editor) => {
                let focus_handle = editor.focus_handle(cx);
                let keybinding = KeyBinding::for_action_in(&ApplyAllDiffHunks, &focus_handle, cx)
                    .map(|binding| binding.into_any_element());

                button_like.children(keybinding).on_click({
                    move |_event, cx| focus_handle.dispatch_action(&ApplyAllDiffHunks, cx)
                })
            }
            None => button_like.disabled(true),
        }
    }
}

impl EventEmitter<ToolbarItemEvent> for ProposedChangesEditorToolbar {}

impl ToolbarItemView for ProposedChangesEditorToolbar {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn workspace::ItemHandle>,
        _cx: &mut ViewContext<Self>,
    ) -> workspace::ToolbarItemLocation {
        self.current_editor =
            active_pane_item.and_then(|item| item.downcast::<ProposedChangesEditor>());
        self.get_toolbar_item_location()
    }
}

impl BranchBufferSemanticsProvider {
    fn to_base(
        &self,
        buffer: &Model<Buffer>,
        positions: &[text::Anchor],
        cx: &AppContext,
    ) -> Option<Model<Buffer>> {
        let base_buffer = buffer.read(cx).diff_base_buffer()?;
        let version = base_buffer.read(cx).version();
        if positions
            .iter()
            .any(|position| !version.observed(position.timestamp))
        {
            return None;
        }
        Some(base_buffer)
    }
}

impl SemanticsProvider for BranchBufferSemanticsProvider {
    fn hover(
        &self,
        buffer: &Model<Buffer>,
        position: text::Anchor,
        cx: &mut AppContext,
    ) -> Option<Task<Vec<project::Hover>>> {
        let buffer = self.to_base(buffer, &[position], cx)?;
        self.0.hover(&buffer, position, cx)
    }

    fn inlay_hints(
        &self,
        buffer: Model<Buffer>,
        range: Range<text::Anchor>,
        cx: &mut AppContext,
    ) -> Option<Task<anyhow::Result<Vec<project::InlayHint>>>> {
        let buffer = self.to_base(&buffer, &[range.start, range.end], cx)?;
        self.0.inlay_hints(buffer, range, cx)
    }

    fn resolve_inlay_hint(
        &self,
        hint: project::InlayHint,
        buffer: Model<Buffer>,
        server_id: lsp::LanguageServerId,
        cx: &mut AppContext,
    ) -> Option<Task<anyhow::Result<project::InlayHint>>> {
        let buffer = self.to_base(&buffer, &[], cx)?;
        self.0.resolve_inlay_hint(hint, buffer, server_id, cx)
    }

    fn supports_inlay_hints(&self, buffer: &Model<Buffer>, cx: &AppContext) -> bool {
        if let Some(buffer) = self.to_base(&buffer, &[], cx) {
            self.0.supports_inlay_hints(&buffer, cx)
        } else {
            false
        }
    }

    fn document_highlights(
        &self,
        buffer: &Model<Buffer>,
        position: text::Anchor,
        cx: &mut AppContext,
    ) -> Option<Task<gpui::Result<Vec<project::DocumentHighlight>>>> {
        let buffer = self.to_base(&buffer, &[position], cx)?;
        self.0.document_highlights(&buffer, position, cx)
    }

    fn definitions(
        &self,
        buffer: &Model<Buffer>,
        position: text::Anchor,
        kind: crate::GotoDefinitionKind,
        cx: &mut AppContext,
    ) -> Option<Task<gpui::Result<Vec<project::LocationLink>>>> {
        let buffer = self.to_base(&buffer, &[position], cx)?;
        self.0.definitions(&buffer, position, kind, cx)
    }

    fn range_for_rename(
        &self,
        _: &Model<Buffer>,
        _: text::Anchor,
        _: &mut AppContext,
    ) -> Option<Task<gpui::Result<Option<Range<text::Anchor>>>>> {
        None
    }

    fn perform_rename(
        &self,
        _: &Model<Buffer>,
        _: text::Anchor,
        _: String,
        _: &mut AppContext,
    ) -> Option<Task<gpui::Result<project::ProjectTransaction>>> {
        None
    }
}
