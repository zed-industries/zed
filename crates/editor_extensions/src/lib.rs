mod items;
mod persistence;
use gpui::elements::ChildView;
use gpui::elements::ParentElement;
use gpui::keymap_matcher::KeymapContext;
use gpui::{
    AnyViewHandle, AppContext, Element, ModelHandle, Subscription, Task, ViewContext, ViewHandle,
    WeakViewHandle,
};
pub use items::*;

use anyhow::{anyhow, Result};
use client::{Client, Collaborator, ParticipantIndex};
use collections::{HashMap, HashSet};
use editor::scroll::autoscroll::Autoscroll;
use editor::{
    CollaborationHub, ConfirmRename, Editor, Event, InlayHintRefreshReason, OpenExcerpts, Project,
};
use language::{
    Buffer, CachedLspAdapter, CodeAction, Completion, LanguageRegistry, LanguageServerName,
};
use lsp::{LanguageServer, LanguageServerId};
use project_types::*;
use rpc::proto::PeerId;
use std::mem;
use std::ops::Range;
use std::path::PathBuf;
use std::sync::Arc;
use util::ResultExt;
use workspace::item::ItemHandle;
use workspace::Workspace;
use workspace_types::*;

pub fn init(cx: &mut AppContext) {
    cx.add_action(new_file);
    cx.add_action(new_file_in_direction);
    cx.add_action(open_excerpts);
    cx.add_action(confirm_rename);
    workspace::register_project_item::<FollowableEditor>(cx);
    workspace::register_followable_item::<FollowableEditor>(cx);
    workspace::register_deserializable_item::<FollowableEditor>(cx);
}

impl Project for ProjectHandle {
    fn apply_code_action(
        &self,
        buffer_handle: ModelHandle<Buffer>,
        mut action: CodeAction,
        push_to_history: bool,
        cx: &mut AppContext,
    ) -> Task<Result<ProjectTransaction>> {
        self.0.update(cx, |this, cx| {
            this.apply_code_action(buffer_handle, action, push_to_history, cx)
        })
    }

    fn inlay_hints(
        &self,
        buffer_handle: ModelHandle<Buffer>,
        range: Range<text::Anchor>,
        cx: &mut AppContext,
    ) -> Task<anyhow::Result<Vec<InlayHint>>> {
        self.0
            .update(cx, |this, cx| this.inlay_hints(buffer_handle, range, cx))
    }
    fn visible_worktrees_count(&self, cx: &AppContext) -> usize {
        self.0.read(cx).visible_worktrees(cx).count()
    }
    fn resolve_inlay_hint(
        &self,
        hint: InlayHint,
        buffer_handle: ModelHandle<Buffer>,
        server_id: LanguageServerId,
        cx: &mut AppContext,
    ) -> Task<anyhow::Result<InlayHint>> {
        self.0.update(cx, |this, cx| {
            this.resolve_inlay_hint(hint, buffer_handle, server_id, cx)
        })
    }
    fn languages(&self, cx: &AppContext) -> Arc<LanguageRegistry> {
        self.0.read(cx).languages().clone()
    }
    fn hover(
        &self,
        buffer: &ModelHandle<Buffer>,
        position: text::Anchor,
        cx: &mut AppContext,
    ) -> Task<Result<Option<Hover>>> {
        self.0
            .update(cx, |this, cx| this.hover(buffer, position, cx))
    }
    fn definition(
        &self,
        buffer: &ModelHandle<Buffer>,
        position: text::Anchor,
        cx: &mut AppContext,
    ) -> Task<Result<Vec<LocationLink>>> {
        self.0
            .update(cx, |this, cx| this.definition(buffer, position, cx))
    }

    fn type_definition(
        &self,
        buffer: &ModelHandle<Buffer>,
        position: text::Anchor,
        cx: &mut AppContext,
    ) -> Task<Result<Vec<LocationLink>>> {
        self.0
            .update(cx, |this, cx| this.type_definition(buffer, position, cx))
    }

    fn completions(
        &self,
        buffer: &ModelHandle<Buffer>,
        position: text::Anchor,
        cx: &mut AppContext,
    ) -> Task<Result<Vec<Completion>>> {
        self.0
            .update(cx, |this, cx| this.completions(buffer, position, cx))
    }

    fn as_hub(&self) -> Box<dyn CollaborationHub> {
        Box::new(self.clone())
    }
    fn is_remote(&self, cx: &AppContext) -> bool {
        self.0.read(cx).is_remote()
    }
    fn remote_id(&self, cx: &AppContext) -> Option<u64> {
        self.0.read(cx).remote_id()
    }
    fn language_servers_for_buffer(
        &self,
        buffer: &Buffer,
        cx: &AppContext,
    ) -> Vec<(Arc<CachedLspAdapter>, Arc<LanguageServer>)> {
        self.0
            .read(cx)
            .language_servers_for_buffer(buffer, cx)
            .map(|(adapter, server)| (adapter.clone(), server.clone()))
            .collect()
    }

    fn on_type_format(
        &self,
        buffer: ModelHandle<Buffer>,
        position: text::Anchor,
        trigger: String,
        push_to_history: bool,
        cx: &mut AppContext,
    ) -> Task<Result<Option<text::Transaction>>> {
        self.0.update(cx, |this, cx| {
            this.on_type_format(buffer, position, trigger, push_to_history, cx)
        })
    }
    fn client(&self, cx: &AppContext) -> Arc<Client> {
        self.0.read(cx).client().clone()
    }

    fn language_server_for_id(
        &self,
        id: LanguageServerId,
        cx: &AppContext,
    ) -> Option<Arc<LanguageServer>> {
        self.0.read(cx).language_server_for_id(id).clone()
    }

    fn code_actions(
        &self,
        buffer_handle: &ModelHandle<Buffer>,
        range: Range<text::Anchor>,
        cx: &mut AppContext,
    ) -> Task<Result<Vec<CodeAction>>> {
        self.0
            .update(cx, |this, cx| this.code_actions(buffer_handle, range, cx))
    }
    fn document_highlights(
        &self,
        buffer: &ModelHandle<Buffer>,
        position: text::Anchor,
        cx: &mut AppContext,
    ) -> Task<Result<Vec<DocumentHighlight>>> {
        self.0.update(cx, |this, cx| {
            this.document_highlights(buffer, position, cx)
        })
    }

    fn format(
        &self,
        buffers: HashSet<ModelHandle<Buffer>>,
        push_to_history: bool,
        trigger: FormatTrigger,
        cx: &mut AppContext,
    ) -> Task<anyhow::Result<ProjectTransaction>> {
        self.0.update(cx, |this, cx| {
            this.format(buffers, push_to_history, trigger, cx)
        })
    }

    fn restart_language_servers_for_buffers(
        &self,
        buffers: HashSet<ModelHandle<Buffer>>,
        cx: &mut AppContext,
    ) -> Option<()> {
        self.0.update(cx, |this, cx| {
            this.restart_language_servers_for_buffers(buffers, cx)
        })
    }

    fn prepare_rename(
        &self,
        buffer: ModelHandle<Buffer>,
        position: usize,
        cx: &mut AppContext,
    ) -> Task<Result<Option<Range<text::Anchor>>>> {
        self.0
            .update(cx, |this, cx| this.prepare_rename(buffer, position, cx))
    }

    fn references(
        &self,
        buffer: &ModelHandle<Buffer>,
        position: text::Anchor,
        cx: &mut AppContext,
    ) -> Task<Result<Vec<Location>>> {
        self.0
            .update(cx, |this, cx| this.references(buffer, position, cx))
    }

    fn apply_additional_edits_for_completion(
        &self,
        buffer_handle: ModelHandle<Buffer>,
        completion: Completion,
        push_to_history: bool,
        cx: &mut AppContext,
    ) -> Task<Result<Option<text::Transaction>>> {
        self.0.update(cx, |this, cx| {
            this.apply_additional_edits_for_completion(
                buffer_handle,
                completion,
                push_to_history,
                cx,
            )
        })
    }

    fn language_server_for_buffer<'a>(
        &self,
        buffer: &Buffer,
        server_id: LanguageServerId,
        cx: &'a AppContext,
    ) -> Option<(&'a Arc<CachedLspAdapter>, &'a Arc<LanguageServer>)> {
        self.0
            .read(cx)
            .language_server_for_buffer(buffer, server_id, cx)
    }

    fn open_local_buffer_via_lsp(
        &self,
        abs_path: lsp::Url,
        language_server_id: LanguageServerId,
        language_server_name: LanguageServerName,
        cx: &mut AppContext,
    ) -> Task<Result<ModelHandle<Buffer>>> {
        self.0.update(cx, |this, cx| {
            this.open_local_buffer_via_lsp(abs_path, language_server_id, language_server_name, cx)
        })
    }

    fn subscribe(&self, is_singleton: bool, cx: &mut ViewContext<Editor>) -> Vec<Subscription> {
        let mut project_subscriptions = Vec::with_capacity(2);
        if is_singleton {
            project_subscriptions.push(cx.observe(&self.0, |_, _, cx| {
                cx.emit(Event::TitleChanged);
            }));
        }
        project_subscriptions.push(cx.subscribe(&self.0, |editor, _, event, cx| {
            if let project::Event::RefreshInlayHints = event {
                editor.refresh_inlay_hints(InlayHintRefreshReason::RefreshRequested, cx);
            };
        }));
        project_subscriptions
    }

    fn project_file(&self, file: &dyn language::File) -> ProjectPath {
        todo!()
        //project::File::from_dyn(file).map
    }
}

#[derive(Clone, Debug, PartialEq)]
struct ProjectHandle(pub ModelHandle<project::Project>);
#[derive(Clone, Debug, PartialEq)]
struct WeakWorkspaceHandle(pub WeakViewHandle<workspace::Workspace>);

impl CollaborationHub for ProjectHandle {
    fn collaborators<'a>(&self, cx: &'a AppContext) -> &'a HashMap<PeerId, Collaborator> {
        self.0.read(cx).collaborators()
    }

    fn user_participant_indices<'a>(
        &self,
        cx: &'a AppContext,
    ) -> &'a HashMap<u64, ParticipantIndex> {
        self.0.read(cx).user_store().read(cx).participant_indices()
    }
}

impl editor::Workspace for WeakWorkspaceHandle {
    fn open_abs_path(
        &self,
        abs_path: PathBuf,
        visible: bool,
        cx: &mut AppContext,
    ) -> Task<anyhow::Result<ViewHandle<Editor>>> {
        self.0
            .update(cx, |workspace, cx| {
                workspace.open_abs_path(abs_path, visible, cx)
            })
            .map_or_else(|err| Task::ready(Err(err)), |ok| ok)
    }
    fn open_path(
        &self,
        path: ProjectPath,
        focus_item: bool,
        cx: &mut AppContext,
    ) -> Task<Result<Box<dyn ItemHandle>, anyhow::Error>> {
        self.0
            .update(cx, |this, cx| this.open_path(path, None, focus_item, cx))
            .map_or_else(|err| Task::ready(Err(err)), |ok| ok)
    }

    fn active_editor(&self, cx: &mut AppContext) -> Option<ViewHandle<Editor>> {
        self.0
            .update(cx, |workspace, cx| {
                workspace
                    .active_item(cx)
                    .and_then(|item| item.act_as::<Editor>(cx))
            })
            .log_err()
            .flatten()
    }

    fn project(&self, cx: &mut AppContext) -> Arc<dyn Project> {
        Arc::new(ProjectHandle(
            self.0
                .update(cx, |this, cx| this.project().clone())
                .unwrap(),
        ))
    }
    fn disable_update_history_for_current_pane(
        &self,
        cx: &mut AppContext,
    ) -> Option<editor::DisableUpdateHistoryGuard> {
        let pane = self
            .0
            .update(cx, |this, cx| this.active_pane().clone())
            .log_err()?;
        pane.update(cx, |pane, cx| {
            pane.disable_history();
            pane.enable_history()
        });
        let pane = pane.downgrade();
        struct Guard {
            pane: WeakViewHandle<workspace::Pane>,
        }
        impl editor::DisableUpdateHistory for Guard {
            fn release(self, cx: &mut AppContext) {
                self.pane.update(cx, |this, _| this.enable_history());
            }
        }
        Some(Box::new(Guard { pane }))
    }
    fn split_buffer(&self, buffer: ModelHandle<Buffer>, cx: &mut AppContext) -> ViewHandle<Editor> {
        self.0
            .update(cx, |this, cx| {
                this.split_project_item::<FollowableEditor>(buffer, cx)
            })
            .unwrap()
            .read(cx)
            .0
            .clone()
    }

    fn open_buffer(&self, buffer: ModelHandle<Buffer>, cx: &mut AppContext) -> ViewHandle<Editor> {
        self.0
            .update(cx, |this, cx| {
                this.open_project_item::<FollowableEditor>(buffer, cx)
            })
            .unwrap()
            .read(cx)
            .0
            .clone()
    }

    fn add_item(&self, item: Box<dyn ItemHandle>, cx: &mut AppContext) {
        self.0.update(cx, |this, cx| this.add_item(item, cx));
    }

    fn split_item(
        &self,
        split_direction: SplitDirection,
        item: Box<dyn ItemHandle>,
        cx: &mut AppContext,
    ) {
        self.0
            .update(cx, |this, cx| this.split_item(split_direction, item, cx));
    }
}

pub struct FollowableEditor(pub ViewHandle<editor::Editor>);

impl gpui::Entity for FollowableEditor {
    type Event = <Editor as gpui::Entity>::Event;
}

impl FollowableEditor {
    pub fn clone(&self, cx: &AppContext) -> Self {
        Self(self.0.read(cx).clone(cx))
    }
}

impl gpui::View for FollowableEditor {
    fn render(&mut self, cx: &mut ViewContext<'_, '_, Self>) -> gpui::AnyElement<Self> {
        ChildView::new(&self.0, cx)
    }
    fn ui_name() -> &'static str {
        Editor::ui_name()
    }

    fn focus_in(&mut self, focused: AnyViewHandle, cx: &mut ViewContext<Self>) {
        self.0.update(cx, |this, cx| this.focus_in(focused, cx))
    }

    fn focus_out(&mut self, handle: AnyViewHandle, cx: &mut ViewContext<Self>) {
        self.0.update(cx, |this, cx| this.focus_out(handle, cx))
    }

    fn modifiers_changed(
        &mut self,
        event: &gpui::platform::ModifiersChangedEvent,
        cx: &mut ViewContext<Self>,
    ) -> bool {
        self.0
            .update(cx, |this, cx| this.modifiers_changed(event, cx))
    }

    fn update_keymap_context(&self, keymap: &mut KeymapContext, cx: &AppContext) {
        self.0
            .read_with(cx, |this, cx| this.update_keymap_context(keymap, cx));
    }

    fn text_for_range(&self, range_utf16: Range<usize>, cx: &AppContext) -> Option<String> {
        self.0
            .read_with(cx, |this, cx| this.text_for_range(range_utf16, cx))
            .flatten()
    }

    fn selected_text_range(&self, cx: &AppContext) -> Option<Range<usize>> {
        self.0
            .read_with(cx, |this, cx| this.selected_text_range(cx))
            .flatten()
    }

    fn marked_text_range(&self, cx: &AppContext) -> Option<Range<usize>> {
        self.0
            .read_with(cx, |this, cx| this.marked_text_range(cx))
            .flatten()
    }

    fn unmark_text(&mut self, cx: &mut ViewContext<Self>) {
        self.0.update(cx, |this, cx| this.unmark_text(cx));
    }

    fn replace_text_in_range(
        &mut self,
        range_utf16: Option<Range<usize>>,
        text: &str,
        cx: &mut ViewContext<Self>,
    ) {
        self.0.update(cx, |this, cx| {
            this.replace_text_in_range(range_utf16, text, cx)
        })
    }

    fn replace_and_mark_text_in_range(
        &mut self,
        range_utf16: Option<Range<usize>>,
        text: &str,
        new_selected_range_utf16: Option<Range<usize>>,
        cx: &mut ViewContext<Self>,
    ) {
        self.0.update(cx, |this, cx| {
            this.replace_and_mark_text_in_range(range_utf16, text, new_selected_range_utf16, cx)
        })
    }
}

pub fn new_file(
    workspace: &mut Workspace,
    _: &workspace_types::NewFile,
    cx: &mut ViewContext<Workspace>,
) {
    let project = workspace.project().clone();
    if project.read(cx).is_remote() {
        cx.propagate_action();
    } else if let Some(buffer) = project
        .update(cx, |project, cx| project.create_buffer("", None, cx))
        .log_err()
    {
        workspace.add_item(
            Box::new(
                cx.add_view(|cx| Editor::for_buffer(buffer, Some(Arc::new(project.clone())), cx)),
            ),
            cx,
        );
    }
}

pub fn new_file_in_direction(
    workspace: &mut Workspace,
    action: &workspace::NewFileInDirection,
    cx: &mut ViewContext<Workspace>,
) {
    let project = workspace.project().clone();
    if project.read(cx).is_remote() {
        cx.propagate_action();
    } else if let Some(buffer) = project
        .update(cx, |project, cx| project.create_buffer("", None, cx))
        .log_err()
    {
        workspace.split_item(
            action.0,
            Box::new(
                cx.add_view(|cx| Editor::for_buffer(buffer, Some(Arc::new(project.clone())), cx)),
            ),
            cx,
        );
    }
}

fn open_excerpts(workspace: &mut Workspace, _: &OpenExcerpts, cx: &mut ViewContext<Workspace>) {
    let active_item = workspace.active_item(cx);
    let editor_handle = if let Some(editor) = active_item
        .as_ref()
        .and_then(|item| item.act_as::<Editor>(cx))
    {
        editor
    } else {
        cx.propagate_action();
        return;
    };

    let editor = editor_handle.read(cx);
    let buffer = editor.buffer().read(cx);
    if buffer.is_singleton() {
        cx.propagate_action();
        return;
    }

    let mut new_selections_by_buffer = HashMap::default();
    for selection in editor.selections.all::<usize>(cx) {
        for (buffer, mut range, _) in
            buffer.range_to_buffer_ranges(selection.start..selection.end, cx)
        {
            if selection.reversed {
                mem::swap(&mut range.start, &mut range.end);
            }
            new_selections_by_buffer
                .entry(buffer)
                .or_insert(Vec::new())
                .push(range)
        }
    }

    editor_handle.update(cx, |editor, cx| {
        editor.push_to_nav_history(editor.selections.newest_anchor().head(), None, cx);
    });
    let pane = workspace.active_pane().clone();
    pane.update(cx, |pane, _| pane.disable_history());

    // We defer the pane interaction because we ourselves are a workspace item
    // and activating a new item causes the pane to call a method on us reentrantly,
    // which panics if we're on the stack.
    cx.defer(move |workspace, cx| {
        for (buffer, ranges) in new_selections_by_buffer.into_iter() {
            let editor = workspace.open_project_item::<FollowableEditor>(buffer, cx);
            editor.read(cx).0.update(cx, |editor, cx| {
                editor.change_selections(Some(Autoscroll::newest()), cx, |s| {
                    s.select_ranges(ranges);
                });
            });
        }

        pane.update(cx, |pane, _| pane.enable_history());
    });
}

pub fn confirm_rename(
    workspace: &mut Workspace,
    _: &ConfirmRename,
    cx: &mut ViewContext<Workspace>,
) -> Option<Task<Result<()>>> {
    let editor = workspace.active_item(cx)?.act_as::<Editor>(cx)?;

    let (buffer, range, old_name, new_name) = editor.update(cx, |editor, cx| {
        let rename = editor.take_rename(false, cx)?;
        let buffer = editor.buffer().read(cx);
        let (start_buffer, start) =
            buffer.text_anchor_for_position(rename.range.start.clone(), cx)?;
        let (end_buffer, end) = buffer.text_anchor_for_position(rename.range.end.clone(), cx)?;
        if start_buffer == end_buffer {
            let new_name = rename.editor.read(cx).text(cx);
            Some((start_buffer, start..end, rename.old_name, new_name))
        } else {
            None
        }
    })?;

    let rename = workspace.project().clone().update(cx, |project, cx| {
        project.perform_rename(buffer.clone(), range.start, new_name.clone(), true, cx)
    });

    let editor = editor.downgrade();
    Some(cx.spawn(|workspace, mut cx| async move {
        let project_transaction = rename.await?;
        Editor::open_project_transaction(
            &editor,
            &workspace,
            project_transaction,
            format!("Rename: {old_name} → {new_name}"),
            cx.clone(),
        )
        .await?;

        editor.update(&mut cx, |editor, cx| {
            editor.refresh_document_highlights(cx);
        })?;
        Ok(())
    }))
}
