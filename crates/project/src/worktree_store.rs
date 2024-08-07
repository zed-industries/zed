use anyhow::{anyhow, Context as _, Result};
use collections::HashMap;
use gpui::{AppContext, AsyncAppContext, EntityId, EventEmitter, Model, ModelContext, WeakModel};
use rpc::{
    proto::{self, AnyProtoClient},
    TypedEnvelope,
};
use text::ReplicaId;
use worktree::{ProjectEntryId, Worktree, WorktreeId};

pub struct WorktreeStore {
    is_shared: bool,
    worktrees: Vec<WorktreeHandle>,
    worktrees_reordered: bool,
}

pub enum WorktreeStoreEvent {
    WorktreeAdded(Model<Worktree>),
    WorktreeRemoved(EntityId, WorktreeId),
    WorktreeOrderChanged,
}

impl EventEmitter<WorktreeStoreEvent> for WorktreeStore {}

impl WorktreeStore {
    pub fn new(retain_worktrees: bool) -> Self {
        Self {
            is_shared: retain_worktrees,
            worktrees: Vec::new(),
            worktrees_reordered: false,
        }
    }

    /// Iterates through all worktrees, including ones that don't appear in the project panel
    pub fn worktrees(&self) -> impl '_ + DoubleEndedIterator<Item = Model<Worktree>> {
        self.worktrees
            .iter()
            .filter_map(move |worktree| worktree.upgrade())
    }

    /// Iterates through all user-visible worktrees, the ones that appear in the project panel.
    pub fn visible_worktrees<'a>(
        &'a self,
        cx: &'a AppContext,
    ) -> impl 'a + DoubleEndedIterator<Item = Model<Worktree>> {
        self.worktrees()
            .filter(|worktree| worktree.read(cx).is_visible())
    }

    pub fn worktree_for_id(&self, id: WorktreeId, cx: &AppContext) -> Option<Model<Worktree>> {
        self.worktrees()
            .find(|worktree| worktree.read(cx).id() == id)
    }

    pub fn worktree_for_entry(
        &self,
        entry_id: ProjectEntryId,
        cx: &AppContext,
    ) -> Option<Model<Worktree>> {
        self.worktrees()
            .find(|worktree| worktree.read(cx).contains_entry(entry_id))
    }

    pub fn add(&mut self, worktree: &Model<Worktree>, cx: &mut ModelContext<Self>) {
        let push_strong_handle = self.is_shared || worktree.read(cx).is_visible();
        let handle = if push_strong_handle {
            WorktreeHandle::Strong(worktree.clone())
        } else {
            WorktreeHandle::Weak(worktree.downgrade())
        };
        if self.worktrees_reordered {
            self.worktrees.push(handle);
        } else {
            let i = match self
                .worktrees
                .binary_search_by_key(&Some(worktree.read(cx).abs_path()), |other| {
                    other.upgrade().map(|worktree| worktree.read(cx).abs_path())
                }) {
                Ok(i) | Err(i) => i,
            };
            self.worktrees.insert(i, handle);
        }

        cx.emit(WorktreeStoreEvent::WorktreeAdded(worktree.clone()));

        let handle_id = worktree.entity_id();
        cx.observe_release(worktree, move |_, worktree, cx| {
            cx.emit(WorktreeStoreEvent::WorktreeRemoved(
                handle_id,
                worktree.id(),
            ));
        })
        .detach();
    }

    pub fn remove_worktree(&mut self, id_to_remove: WorktreeId, cx: &mut ModelContext<Self>) {
        self.worktrees.retain(|worktree| {
            if let Some(worktree) = worktree.upgrade() {
                if worktree.read(cx).id() == id_to_remove {
                    cx.emit(WorktreeStoreEvent::WorktreeRemoved(
                        worktree.entity_id(),
                        id_to_remove,
                    ));
                    false
                } else {
                    true
                }
            } else {
                false
            }
        });
    }

    pub fn set_worktrees_reordered(&mut self, worktrees_reordered: bool) {
        self.worktrees_reordered = worktrees_reordered;
    }

    pub fn set_worktrees_from_proto(
        &mut self,
        worktrees: Vec<proto::WorktreeMetadata>,
        replica_id: ReplicaId,
        remote_id: u64,
        client: AnyProtoClient,
        cx: &mut ModelContext<Self>,
    ) -> Result<()> {
        let mut old_worktrees_by_id = self
            .worktrees
            .drain(..)
            .filter_map(|worktree| {
                let worktree = worktree.upgrade()?;
                Some((worktree.read(cx).id(), worktree))
            })
            .collect::<HashMap<_, _>>();

        for worktree in worktrees {
            if let Some(old_worktree) =
                old_worktrees_by_id.remove(&WorktreeId::from_proto(worktree.id))
            {
                self.worktrees.push(WorktreeHandle::Strong(old_worktree));
            } else {
                self.add(
                    &Worktree::remote(remote_id, replica_id, worktree, client.clone(), cx),
                    cx,
                );
            }
        }

        Ok(())
    }

    pub fn move_worktree(
        &mut self,
        source: WorktreeId,
        destination: WorktreeId,
        cx: &mut ModelContext<Self>,
    ) -> Result<()> {
        if source == destination {
            return Ok(());
        }

        let mut source_index = None;
        let mut destination_index = None;
        for (i, worktree) in self.worktrees.iter().enumerate() {
            if let Some(worktree) = worktree.upgrade() {
                let worktree_id = worktree.read(cx).id();
                if worktree_id == source {
                    source_index = Some(i);
                    if destination_index.is_some() {
                        break;
                    }
                } else if worktree_id == destination {
                    destination_index = Some(i);
                    if source_index.is_some() {
                        break;
                    }
                }
            }
        }

        let source_index =
            source_index.with_context(|| format!("Missing worktree for id {source}"))?;
        let destination_index =
            destination_index.with_context(|| format!("Missing worktree for id {destination}"))?;

        if source_index == destination_index {
            return Ok(());
        }

        let worktree_to_move = self.worktrees.remove(source_index);
        self.worktrees.insert(destination_index, worktree_to_move);
        self.worktrees_reordered = true;
        cx.emit(WorktreeStoreEvent::WorktreeOrderChanged);
        cx.notify();
        Ok(())
    }

    pub fn disconnected_from_host(&mut self, cx: &mut AppContext) {
        for worktree in &self.worktrees {
            if let Some(worktree) = worktree.upgrade() {
                worktree.update(cx, |worktree, _| {
                    if let Some(worktree) = worktree.as_remote_mut() {
                        worktree.disconnected_from_host();
                    }
                });
            }
        }
    }

    pub fn set_shared(&mut self, is_shared: bool, cx: &mut ModelContext<Self>) {
        self.is_shared = is_shared;

        // When shared, retain all worktrees
        if is_shared {
            for worktree_handle in self.worktrees.iter_mut() {
                match worktree_handle {
                    WorktreeHandle::Strong(_) => {}
                    WorktreeHandle::Weak(worktree) => {
                        if let Some(worktree) = worktree.upgrade() {
                            *worktree_handle = WorktreeHandle::Strong(worktree);
                        }
                    }
                }
            }
        }
        // When not shared, only retain the visible worktrees
        else {
            for worktree_handle in self.worktrees.iter_mut() {
                if let WorktreeHandle::Strong(worktree) = worktree_handle {
                    let is_visible = worktree.update(cx, |worktree, _| {
                        worktree.stop_observing_updates();
                        worktree.is_visible()
                    });
                    if !is_visible {
                        *worktree_handle = WorktreeHandle::Weak(worktree.downgrade());
                    }
                }
            }
        }
    }

    pub async fn handle_create_project_entry(
        this: Model<Self>,
        envelope: TypedEnvelope<proto::CreateProjectEntry>,
        mut cx: AsyncAppContext,
    ) -> Result<proto::ProjectEntryResponse> {
        let worktree = this.update(&mut cx, |this, cx| {
            let worktree_id = WorktreeId::from_proto(envelope.payload.worktree_id);
            this.worktree_for_id(worktree_id, cx)
                .ok_or_else(|| anyhow!("worktree not found"))
        })??;
        Worktree::handle_create_entry(worktree, envelope.payload, cx).await
    }

    pub async fn handle_rename_project_entry(
        this: Model<Self>,
        envelope: TypedEnvelope<proto::RenameProjectEntry>,
        mut cx: AsyncAppContext,
    ) -> Result<proto::ProjectEntryResponse> {
        let entry_id = ProjectEntryId::from_proto(envelope.payload.entry_id);
        let worktree = this.update(&mut cx, |this, cx| {
            this.worktree_for_entry(entry_id, cx)
                .ok_or_else(|| anyhow!("worktree not found"))
        })??;
        Worktree::handle_rename_entry(worktree, envelope.payload, cx).await
    }

    pub async fn handle_copy_project_entry(
        this: Model<Self>,
        envelope: TypedEnvelope<proto::CopyProjectEntry>,
        mut cx: AsyncAppContext,
    ) -> Result<proto::ProjectEntryResponse> {
        let entry_id = ProjectEntryId::from_proto(envelope.payload.entry_id);
        let worktree = this.update(&mut cx, |this, cx| {
            this.worktree_for_entry(entry_id, cx)
                .ok_or_else(|| anyhow!("worktree not found"))
        })??;
        Worktree::handle_copy_entry(worktree, envelope.payload, cx).await
    }

    pub async fn handle_delete_project_entry(
        this: Model<Self>,
        envelope: TypedEnvelope<proto::DeleteProjectEntry>,
        mut cx: AsyncAppContext,
    ) -> Result<proto::ProjectEntryResponse> {
        let entry_id = ProjectEntryId::from_proto(envelope.payload.entry_id);
        let worktree = this.update(&mut cx, |this, cx| {
            this.worktree_for_entry(entry_id, cx)
                .ok_or_else(|| anyhow!("worktree not found"))
        })??;
        Worktree::handle_delete_entry(worktree, envelope.payload, cx).await
    }

    pub async fn handle_expand_project_entry(
        this: Model<Self>,
        envelope: TypedEnvelope<proto::ExpandProjectEntry>,
        mut cx: AsyncAppContext,
    ) -> Result<proto::ExpandProjectEntryResponse> {
        let entry_id = ProjectEntryId::from_proto(envelope.payload.entry_id);
        let worktree = this
            .update(&mut cx, |this, cx| this.worktree_for_entry(entry_id, cx))?
            .ok_or_else(|| anyhow!("invalid request"))?;
        Worktree::handle_expand_entry(worktree, envelope.payload, cx).await
    }
}

#[derive(Clone)]
enum WorktreeHandle {
    Strong(Model<Worktree>),
    Weak(WeakModel<Worktree>),
}

impl WorktreeHandle {
    fn upgrade(&self) -> Option<Model<Worktree>> {
        match self {
            WorktreeHandle::Strong(handle) => Some(handle.clone()),
            WorktreeHandle::Weak(handle) => handle.upgrade(),
        }
    }
}
