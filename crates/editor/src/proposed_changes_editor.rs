use crate::{ApplyAllDiffHunks, Editor, EditorEvent, SemanticsProvider};
use buffer_diff::BufferDiff;
use collections::HashSet;
use futures::{channel::mpsc, future::join_all};
use gpui::{App, Entity, EventEmitter, Focusable, Render, Subscription, Task};
use language::{Buffer, BufferEvent, Capability};
use multi_buffer::{ExcerptRange, MultiBuffer};
use project::{LspPullDiagnostics, Project};
use smol::stream::StreamExt;
use std::{any::TypeId, ops::Range, rc::Rc, time::Duration};
use text::ToOffset;
use ui::{ButtonLike, KeyBinding, prelude::*};
use workspace::{
    Item, ItemHandle as _, ToolbarItemEvent, ToolbarItemLocation, ToolbarItemView, Workspace,
    searchable::SearchableItemHandle,
};

pub struct ProposedChangesEditor {
    editor: Entity<Editor>,
    multibuffer: Entity<MultiBuffer>,
    title: SharedString,
    buffer_entries: Vec<BufferEntry>,
    _recalculate_diffs_task: Task<Option<()>>,
    recalculate_diffs_tx: mpsc::UnboundedSender<RecalculateDiff>,
}

pub struct ProposedChangeLocation<T> {
    pub buffer: Entity<Buffer>,
    pub ranges: Vec<Range<T>>,
}

struct BufferEntry {
    base: Entity<Buffer>,
    branch: Entity<Buffer>,
    _subscription: Subscription,
}

pub struct ProposedChangesEditorToolbar {
    current_editor: Option<Entity<ProposedChangesEditor>>,
}

struct RecalculateDiff {
    buffer: Entity<Buffer>,
    debounce: bool,
}

/// A provider of code semantics for branch buffers.
///
/// Requests in edited regions will return nothing, but requests in unchanged
/// regions will be translated into the base buffer's coordinates.
struct BranchBufferSemanticsProvider(Rc<dyn SemanticsProvider>);

impl ProposedChangesEditor {
    pub fn new<T: Clone + ToOffset>(
        title: impl Into<SharedString>,
        locations: Vec<ProposedChangeLocation<T>>,
        project: Option<Entity<Project>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let multibuffer = cx.new(|_| MultiBuffer::new(Capability::ReadWrite));
        let (recalculate_diffs_tx, mut recalculate_diffs_rx) = mpsc::unbounded();
        let mut this = Self {
            editor: cx.new(|cx| {
                let mut editor = Editor::for_multibuffer(multibuffer.clone(), project, window, cx);
                editor.set_expand_all_diff_hunks(cx);
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
            _recalculate_diffs_task: cx.spawn_in(window, async move |this, cx| {
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

                    let recalculate_diff_futures = this
                        .update(cx, |this, cx| {
                            buffers_to_diff
                                .drain()
                                .filter_map(|buffer| {
                                    let buffer = buffer.read(cx);
                                    let base_buffer = buffer.base_buffer()?;
                                    let buffer = buffer.text_snapshot();
                                    let diff =
                                        this.multibuffer.read(cx).diff_for(buffer.remote_id())?;
                                    Some(diff.update(cx, |diff, cx| {
                                        diff.set_base_text_buffer(base_buffer.clone(), buffer, cx)
                                    }))
                                })
                                .collect::<Vec<_>>()
                        })
                        .ok()?;

                    join_all(recalculate_diff_futures).await;
                }
                None
            }),
        };
        this.reset_locations(locations, window, cx);
        this
    }

    pub fn branch_buffer_for_base(&self, base_buffer: &Entity<Buffer>) -> Option<Entity<Buffer>> {
        self.buffer_entries.iter().find_map(|entry| {
            if &entry.base == base_buffer {
                Some(entry.branch.clone())
            } else {
                None
            }
        })
    }

    pub fn set_title(&mut self, title: SharedString, cx: &mut Context<Self>) {
        self.title = title;
        cx.notify();
    }

    pub fn reset_locations<T: Clone + ToOffset>(
        &mut self,
        locations: Vec<ProposedChangeLocation<T>>,
        window: &mut Window,
        cx: &mut Context<Self>,
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
        let mut new_diffs = Vec::new();
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
                new_diffs.push(cx.new(|cx| {
                    let mut diff = BufferDiff::new(&branch_buffer.read(cx).snapshot(), cx);
                    let _ = diff.set_base_text_buffer(
                        location.buffer.clone(),
                        branch_buffer.read(cx).text_snapshot(),
                        cx,
                    );
                    diff
                }));
                buffer_entries.push(BufferEntry {
                    branch: branch_buffer.clone(),
                    base: location.buffer.clone(),
                    _subscription: cx.subscribe(&branch_buffer, Self::on_buffer_event),
                });
            }

            self.multibuffer.update(cx, |multibuffer, cx| {
                multibuffer.push_excerpts(
                    branch_buffer,
                    location
                        .ranges
                        .into_iter()
                        .map(|range| ExcerptRange::new(range)),
                    cx,
                );
            });
        }

        self.buffer_entries = buffer_entries;
        self.editor.update(cx, |editor, cx| {
            editor.change_selections(None, window, cx, |selections| selections.refresh());
            editor.buffer.update(cx, |buffer, cx| {
                for diff in new_diffs {
                    buffer.add_diff(diff, cx)
                }
            })
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
        buffer: Entity<Buffer>,
        event: &BufferEvent,
        _cx: &mut Context<Self>,
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
            // BufferEvent::DiffBaseChanged => {
            //     self.recalculate_diffs_tx
            //         .unbounded_send(RecalculateDiff {
            //             buffer,
            //             debounce: false,
            //         })
            //         .ok();
            // }
            _ => (),
        }
    }
}

impl Render for ProposedChangesEditor {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .key_context("ProposedChangesEditor")
            .child(self.editor.clone())
    }
}

impl Focusable for ProposedChangesEditor {
    fn focus_handle(&self, cx: &App) -> gpui::FocusHandle {
        self.editor.focus_handle(cx)
    }
}

impl EventEmitter<EditorEvent> for ProposedChangesEditor {}

impl Item for ProposedChangesEditor {
    type Event = EditorEvent;

    fn tab_icon(&self, _window: &Window, _cx: &App) -> Option<Icon> {
        Some(Icon::new(IconName::Diff))
    }

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
        self.title.clone()
    }

    fn as_searchable(&self, _: &Entity<Self>) -> Option<Box<dyn SearchableItemHandle>> {
        Some(Box::new(self.editor.clone()))
    }

    fn act_as_type<'a>(
        &'a self,
        type_id: TypeId,
        self_handle: &'a Entity<Self>,
        _: &'a App,
    ) -> Option<gpui::AnyView> {
        if type_id == TypeId::of::<Self>() {
            Some(self_handle.to_any())
        } else if type_id == TypeId::of::<Editor>() {
            Some(self.editor.to_any())
        } else {
            None
        }
    }

    fn added_to_workspace(
        &mut self,
        workspace: &mut Workspace,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.editor.update(cx, |editor, cx| {
            Item::added_to_workspace(editor, workspace, window, cx)
        });
    }

    fn deactivated(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.editor
            .update(cx, |editor, cx| editor.deactivated(window, cx));
    }

    fn navigate(
        &mut self,
        data: Box<dyn std::any::Any>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        self.editor
            .update(cx, |editor, cx| Item::navigate(editor, data, window, cx))
    }

    fn set_nav_history(
        &mut self,
        nav_history: workspace::ItemNavHistory,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.editor.update(cx, |editor, cx| {
            Item::set_nav_history(editor, nav_history, window, cx)
        });
    }

    fn can_save(&self, cx: &App) -> bool {
        self.editor.read(cx).can_save(cx)
    }

    fn save(
        &mut self,
        format: bool,
        project: Entity<Project>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<gpui::Result<()>> {
        self.editor.update(cx, |editor, cx| {
            Item::save(editor, format, project, window, cx)
        })
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
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let button_like = ButtonLike::new("apply-changes").child(Label::new("Apply All"));

        match &self.current_editor {
            Some(editor) => {
                let focus_handle = editor.focus_handle(cx);
                let keybinding =
                    KeyBinding::for_action_in(&ApplyAllDiffHunks, &focus_handle, window, cx)
                        .map(|binding| binding.into_any_element());

                button_like.children(keybinding).on_click({
                    move |_event, window, cx| {
                        focus_handle.dispatch_action(&ApplyAllDiffHunks, window, cx)
                    }
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
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> workspace::ToolbarItemLocation {
        self.current_editor =
            active_pane_item.and_then(|item| item.downcast::<ProposedChangesEditor>());
        self.get_toolbar_item_location()
    }
}

impl BranchBufferSemanticsProvider {
    fn to_base(
        &self,
        buffer: &Entity<Buffer>,
        positions: &[text::Anchor],
        cx: &App,
    ) -> Option<Entity<Buffer>> {
        let base_buffer = buffer.read(cx).base_buffer()?;
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
        buffer: &Entity<Buffer>,
        position: text::Anchor,
        cx: &mut App,
    ) -> Option<Task<Vec<project::Hover>>> {
        let buffer = self.to_base(buffer, &[position], cx)?;
        self.0.hover(&buffer, position, cx)
    }

    fn inlay_hints(
        &self,
        buffer: Entity<Buffer>,
        range: Range<text::Anchor>,
        cx: &mut App,
    ) -> Option<Task<anyhow::Result<Vec<project::InlayHint>>>> {
        let buffer = self.to_base(&buffer, &[range.start, range.end], cx)?;
        self.0.inlay_hints(buffer, range, cx)
    }

    fn inline_values(
        &self,
        _: Entity<Buffer>,
        _: Range<text::Anchor>,
        _: &mut App,
    ) -> Option<Task<anyhow::Result<Vec<project::InlayHint>>>> {
        None
    }

    fn resolve_inlay_hint(
        &self,
        hint: project::InlayHint,
        buffer: Entity<Buffer>,
        server_id: lsp::LanguageServerId,
        cx: &mut App,
    ) -> Option<Task<anyhow::Result<project::InlayHint>>> {
        let buffer = self.to_base(&buffer, &[], cx)?;
        self.0.resolve_inlay_hint(hint, buffer, server_id, cx)
    }

    fn supports_inlay_hints(&self, buffer: &Entity<Buffer>, cx: &mut App) -> bool {
        if let Some(buffer) = self.to_base(&buffer, &[], cx) {
            self.0.supports_inlay_hints(&buffer, cx)
        } else {
            false
        }
    }

    fn document_highlights(
        &self,
        buffer: &Entity<Buffer>,
        position: text::Anchor,
        cx: &mut App,
    ) -> Option<Task<gpui::Result<Vec<project::DocumentHighlight>>>> {
        let buffer = self.to_base(&buffer, &[position], cx)?;
        self.0.document_highlights(&buffer, position, cx)
    }

    fn definitions(
        &self,
        buffer: &Entity<Buffer>,
        position: text::Anchor,
        kind: crate::GotoDefinitionKind,
        cx: &mut App,
    ) -> Option<Task<gpui::Result<Vec<project::LocationLink>>>> {
        let buffer = self.to_base(&buffer, &[position], cx)?;
        self.0.definitions(&buffer, position, kind, cx)
    }

    fn range_for_rename(
        &self,
        _: &Entity<Buffer>,
        _: text::Anchor,
        _: &mut App,
    ) -> Option<Task<gpui::Result<Option<Range<text::Anchor>>>>> {
        None
    }

    fn perform_rename(
        &self,
        _: &Entity<Buffer>,
        _: text::Anchor,
        _: String,
        _: &mut App,
    ) -> Option<Task<gpui::Result<project::ProjectTransaction>>> {
        None
    }

    fn pull_diagnostics(
        &self,
        _: &Entity<Buffer>,
        _: &mut App,
    ) -> Task<anyhow::Result<Vec<LspPullDiagnostics>>> {
        Task::ready(Ok(Vec::new()))
    }

    fn update_diagnostics(
        &self,
        _: Vec<LspPullDiagnostics>,
        _: &mut App,
    ) -> Task<anyhow::Result<()>> {
        Task::ready(Ok(()))
    }
}
