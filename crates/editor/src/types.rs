use crate::Editor;
use client::Client;

use anyhow::Result;

use client::{proto::PeerId, Collaborator, ParticipantIndex};
use collections::{HashMap, HashSet};
use gpui::geometry::vector::Vector2F;
use gpui::WeakViewHandle;
use gpui::{AppContext, ModelHandle, Subscription, Task, ViewContext, ViewHandle};
use language::{
    Buffer, CachedLspAdapter, CodeAction, Completion, LanguageRegistry, LanguageServerName,
};
use lsp::{LanguageServer, LanguageServerId};
use project_types::ProjectPath;
use project_types::{FormatTrigger, ProjectTransaction};
use std::ops::Range;
use std::path::PathBuf;
use std::sync::Arc;
use workspace_types::{ItemId, SplitDirection, WorkspaceId};

#[async_trait::async_trait]
pub trait Db: 'static + Send + Sync {
    async fn save_scroll_position(
        &self,
        item_id: ItemId,
        workspace_id: WorkspaceId,
        top_row: u32,
        vertical_offset: f32,
        horizontal_offset: f32,
    ) -> Result<()>;
    fn get_scroll_position(
        &self,
        item_id: ItemId,
        workspace_id: WorkspaceId,
    ) -> Result<Option<(u32, f32, f32)>>;
    async fn save_path(
        &self,
        item_id: ItemId,
        workspace_id: WorkspaceId,
        path: PathBuf,
    ) -> Result<()>;
}

pub type DisableUpdateHistoryGuard = Box<dyn DisableUpdateHistory>;

pub trait DisableUpdateHistory {
    fn release(self, cx: &mut AppContext);
}

pub trait CollaborationHub {
    fn collaborators<'a>(&self, cx: &'a AppContext) -> &'a HashMap<PeerId, Collaborator>;
    fn user_participant_indices<'a>(
        &self,
        cx: &'a AppContext,
    ) -> &'a HashMap<u64, ParticipantIndex>;
}

pub trait Workspace: 'static {
    fn db(&self) -> Arc<dyn Db>;
    fn open_abs_path(&self, abs_path: PathBuf, visible: bool, cx: &mut AppContext);
    fn open_path(
        &self,
        path: ProjectPath,
        focus_item: bool,
        cx: &mut AppContext,
    ) -> Task<Result<ViewHandle<Editor>>>;
    fn active_editor(&self, cx: &mut AppContext) -> Option<ViewHandle<Editor>>;
    fn project(&self, cx: &mut AppContext) -> Arc<dyn Project>;
    fn disable_update_history_for_current_pane(
        &self,
        cx: &mut AppContext,
    ) -> Option<DisableUpdateHistoryGuard>;
    fn split_buffer(
        &self,
        buffer: ModelHandle<Buffer>,
        cx: &mut AppContext,
    ) -> ViewHandle<crate::Editor>;
    fn open_buffer(
        &self,
        buffer: ModelHandle<Buffer>,
        cx: &mut AppContext,
    ) -> ViewHandle<crate::Editor>;
    fn add_item(&self, item: Box<ViewHandle<Editor>>, cx: &mut AppContext);
    fn split_item(
        &self,
        split_direction: SplitDirection,
        item: Box<ViewHandle<Editor>>,
        cx: &mut AppContext,
    );
}

pub trait Project: 'static + std::fmt::Debug {
    fn apply_code_action(
        &self,
        buffer_handle: ModelHandle<Buffer>,
        action: CodeAction,
        push_to_history: bool,
        cx: &mut AppContext,
    ) -> Task<Result<ProjectTransaction>>;
    fn inlay_hints(
        &self,
        buffer_handle: ModelHandle<Buffer>,
        range: Range<text::Anchor>,
        cx: &mut AppContext,
    ) -> Task<anyhow::Result<Vec<project_types::InlayHint>>>;
    fn visible_worktrees_count(&self, cx: &AppContext) -> usize;
    fn resolve_inlay_hint(
        &self,
        hint: project_types::InlayHint,
        buffer_handle: ModelHandle<Buffer>,
        server_id: LanguageServerId,
        cx: &mut AppContext,
    ) -> Task<anyhow::Result<project_types::InlayHint>>;
    fn languages(&self, cx: &AppContext) -> Arc<LanguageRegistry>;
    fn hover(
        &self,
        buffer: &ModelHandle<Buffer>,
        position: text::Anchor,
        cx: &mut AppContext,
    ) -> Task<Result<Option<project_types::Hover>>>;
    fn definition(
        &self,
        buffer: &ModelHandle<Buffer>,
        position: text::Anchor,
        cx: &mut AppContext,
    ) -> Task<Result<Vec<project_types::LocationLink>>>;

    fn type_definition(
        &self,
        buffer: &ModelHandle<Buffer>,
        position: text::Anchor,
        cx: &mut AppContext,
    ) -> Task<Result<Vec<project_types::LocationLink>>>;
    fn completions(
        &self,
        buffer: &ModelHandle<Buffer>,
        position: text::Anchor,
        cx: &mut AppContext,
    ) -> Task<Result<Vec<Completion>>>;
    fn as_hub(&self) -> Box<dyn CollaborationHub>;
    fn is_remote(&self, cx: &AppContext) -> bool;
    fn is_local(&self, cx: &AppContext) -> bool {
        !self.is_remote(cx)
    }
    fn remote_id(&self, cx: &AppContext) -> Option<u64>;
    fn language_servers_for_buffer(
        &self,
        buffer: &Buffer,
        cx: &AppContext,
    ) -> Vec<(Arc<CachedLspAdapter>, Arc<LanguageServer>)>;
    fn on_type_format(
        &self,
        buffer: ModelHandle<Buffer>,
        position: text::Anchor,
        trigger: String,
        push_to_history: bool,
        cx: &mut AppContext,
    ) -> Task<Result<Option<text::Transaction>>>;
    fn client(&self, cx: &AppContext) -> Arc<Client>;
    fn language_server_for_id(
        &self,
        id: LanguageServerId,
        cx: &AppContext,
    ) -> Option<Arc<LanguageServer>>;

    fn code_actions(
        &self,
        buffer_handle: &ModelHandle<Buffer>,
        range: Range<text::Anchor>,
        cx: &mut AppContext,
    ) -> Task<Result<Vec<CodeAction>>>;
    fn document_highlights(
        &self,
        buffer: &ModelHandle<Buffer>,
        position: text::Anchor,
        cx: &mut AppContext,
    ) -> Task<Result<Vec<project_types::DocumentHighlight>>>;
    fn format(
        &self,
        buffers: HashSet<ModelHandle<Buffer>>,
        push_to_history: bool,
        trigger: FormatTrigger,
        cx: &mut AppContext,
    ) -> Task<anyhow::Result<ProjectTransaction>>;
    fn restart_language_servers_for_buffers(
        &self,
        buffers: HashSet<ModelHandle<Buffer>>,
        cx: &mut AppContext,
    ) -> Option<()>;
    fn prepare_rename(
        &self,
        buffer: ModelHandle<Buffer>,
        position: usize,
        cx: &mut AppContext,
    ) -> Task<Result<Option<Range<text::Anchor>>>>;
    fn references(
        &self,
        buffer: &ModelHandle<Buffer>,
        position: text::Anchor,
        cx: &mut AppContext,
    ) -> Task<Result<Vec<project_types::Location>>>;
    fn apply_additional_edits_for_completion(
        &self,
        buffer_handle: ModelHandle<Buffer>,
        completion: Completion,
        push_to_history: bool,
        cx: &mut AppContext,
    ) -> Task<Result<Option<text::Transaction>>>;
    fn language_server_for_buffer<'a>(
        &self,
        buffer: &Buffer,
        server_id: LanguageServerId,
        cx: &'a AppContext,
    ) -> Option<(&'a Arc<CachedLspAdapter>, &'a Arc<LanguageServer>)>;
    fn open_local_buffer_via_lsp(
        &self,
        abs_path: lsp::Url,
        language_server_id: LanguageServerId,
        language_server_name: LanguageServerName,
        cx: &mut AppContext,
    ) -> Task<Result<ModelHandle<Buffer>>>;
    fn subscribe(
        &self,
        is_singleton: bool,
        cx: &mut ViewContext<crate::Editor>,
    ) -> Vec<Subscription>;
    fn project_file(&self, file: &dyn language::File) -> ProjectPath;
}
