use crate::ProjectPath;
use anyhow::{anyhow, Result};
use collections::{hash_map, HashMap};
use futures::channel::oneshot;
use gpui::{AppContext, Context as _, Model, Task, WeakModel};
use language::{Buffer, Capability, Operation};
use rpc::{proto, ErrorExt as _, TypedEnvelope};
use std::{path::Path, sync::Arc};
use text::BufferId;
use util::{debug_panic, maybe};
use worktree::{File, ProjectEntryId};

#[derive(Default)]
pub(crate) struct BufferStore {
    opened_buffers: HashMap<BufferId, OpenBuffer>,
    remote_buffer_listeners:
        HashMap<BufferId, Vec<oneshot::Sender<Result<Model<Buffer>, anyhow::Error>>>>,
    incomplete_remote_buffers: HashMap<BufferId, Model<Buffer>>,
    local_buffer_ids_by_path: HashMap<ProjectPath, BufferId>,
    local_buffer_ids_by_entry_id: HashMap<ProjectEntryId, BufferId>,
}

enum OpenBuffer {
    Strong(Model<Buffer>),
    Weak(WeakModel<Buffer>),
    Operations(Vec<Operation>),
}

impl BufferStore {
    pub fn add_buffer(
        &mut self,
        buffer: &Model<Buffer>,
        is_strong: bool,
        cx: &mut AppContext,
    ) -> Result<()> {
        let remote_id = buffer.read(cx).remote_id();
        let is_remote = buffer.read(cx).replica_id() != 0;
        let open_buffer = if is_strong {
            OpenBuffer::Strong(buffer.clone())
        } else {
            OpenBuffer::Weak(buffer.downgrade())
        };

        match self.opened_buffers.entry(remote_id) {
            hash_map::Entry::Vacant(entry) => {
                entry.insert(open_buffer);
            }
            hash_map::Entry::Occupied(mut entry) => {
                if let OpenBuffer::Operations(operations) = entry.get_mut() {
                    buffer.update(cx, |b, cx| b.apply_ops(operations.drain(..), cx))?;
                } else if entry.get().upgrade().is_some() {
                    if is_remote {
                        return Ok(());
                    } else {
                        debug_panic!("buffer {} was already registered", remote_id);
                        Err(anyhow!("buffer {} was already registered", remote_id))?;
                    }
                }
                entry.insert(open_buffer);
            }
        }

        if let Some(senders) = self.remote_buffer_listeners.remove(&remote_id) {
            for sender in senders {
                sender.send(Ok(buffer.clone())).ok();
            }
        }

        if let Some(file) = File::from_dyn(buffer.read(cx).file()) {
            if file.is_local {
                self.local_buffer_ids_by_path.insert(
                    ProjectPath {
                        worktree_id: file.worktree_id(cx),
                        path: file.path.clone(),
                    },
                    remote_id,
                );

                if let Some(entry_id) = file.entry_id {
                    self.local_buffer_ids_by_entry_id
                        .insert(entry_id, remote_id);
                }
            }
        }

        Ok(())
    }

    pub fn add_incomplete_buffer(
        &mut self,
        buffer_id: BufferId,
        buffer_result: Result<Buffer>,
        cx: &mut AppContext,
    ) {
        match buffer_result {
            Ok(buffer) => {
                let buffer = cx.new_model(|_| buffer);
                self.incomplete_remote_buffers.insert(buffer_id, buffer);
            }
            Err(error) => {
                if let Some(listeners) = self.remote_buffer_listeners.remove(&buffer_id) {
                    for listener in listeners {
                        listener.send(Err(anyhow!(error.cloned()))).ok();
                    }
                }
            }
        };
    }

    pub fn add_incomplete_buffer_chunk(
        &mut self,
        chunk: proto::BufferChunk,
        cx: &mut AppContext,
    ) -> Result<Option<Model<Buffer>>> {
        let buffer_id = BufferId::new(chunk.buffer_id)?;
        let buffer = self
            .incomplete_remote_buffers
            .get(&buffer_id)
            .cloned()
            .ok_or_else(|| {
                anyhow!(
                    "received chunk for buffer {} without initial state",
                    chunk.buffer_id
                )
            })?;

        let result = maybe!({
            let operations = chunk
                .operations
                .into_iter()
                .map(language::proto::deserialize_operation)
                .collect::<Result<Vec<_>>>()?;
            buffer.update(cx, |buffer, cx| buffer.apply_ops(operations, cx))
        });

        if let Err(error) = result {
            self.incomplete_remote_buffers.remove(&buffer_id);
            if let Some(listeners) = self.remote_buffer_listeners.remove(&buffer_id) {
                for listener in listeners {
                    listener.send(Err(error.cloned())).ok();
                }
            }
        } else {
            if chunk.is_last {
                self.incomplete_remote_buffers.remove(&buffer_id);
                return Ok(Some(buffer));
            }
        }

        Ok(None)
    }

    pub fn buffers(&self) -> impl '_ + Iterator<Item = Model<Buffer>> {
        self.opened_buffers
            .values()
            .filter_map(|buffer| buffer.upgrade())
    }

    pub fn get(&self, buffer_id: BufferId) -> Option<Model<Buffer>> {
        self.opened_buffers
            .get(&buffer_id)
            .and_then(|buffer| buffer.upgrade())
    }

    pub fn get_existing(&self, buffer_id: BufferId) -> Result<Model<Buffer>> {
        self.get(buffer_id)
            .ok_or_else(|| anyhow!("unknown buffer id {}", buffer_id))
    }

    pub fn get_possibly_incomplete(&self, buffer_id: BufferId) -> Option<Model<Buffer>> {
        self.get(buffer_id)
            .or_else(|| self.incomplete_remote_buffers.get(&buffer_id).cloned())
    }

    fn get_or_remove_by_file(
        &mut self,
        entry_id: ProjectEntryId,
        project_path: &ProjectPath,
    ) -> Option<(BufferId, Model<Buffer>)> {
        let buffer_id = match self.local_buffer_ids_by_entry_id.get(&entry_id) {
            Some(&buffer_id) => buffer_id,
            None => match self.local_buffer_ids_by_path.get(project_path) {
                Some(&buffer_id) => buffer_id,
                None => {
                    return None;
                }
            },
        };
        let buffer = if let Some(buffer) = self.get(buffer_id) {
            buffer
        } else {
            self.opened_buffers.remove(&buffer_id);
            self.local_buffer_ids_by_path.remove(project_path);
            self.local_buffer_ids_by_entry_id.remove(&entry_id);
            return None;
        };
        Some((buffer_id, buffer))
    }

    pub fn wait_for_remote_buffer(
        &mut self,
        id: BufferId,
        cx: &mut AppContext,
    ) -> Task<Result<Model<Buffer>>> {
        let buffer = self.get(id);
        if let Some(buffer) = buffer {
            return Task::ready(Ok(buffer));
        }
        let (tx, rx) = oneshot::channel();
        self.remote_buffer_listeners.entry(id).or_default().push(tx);
        cx.background_executor().spawn(async move { rx.await? })
    }

    pub fn buffer_version_info(
        &self,
        cx: &AppContext,
    ) -> (Vec<proto::BufferVersion>, Vec<BufferId>) {
        let buffers = self
            .buffers()
            .map(|buffer| {
                let buffer = buffer.read(cx);
                proto::BufferVersion {
                    id: buffer.remote_id().into(),
                    version: language::proto::serialize_version(&buffer.version),
                }
            })
            .collect();
        let incomplete_buffer_ids = self
            .incomplete_remote_buffers
            .keys()
            .copied()
            .collect::<Vec<_>>();
        (buffers, incomplete_buffer_ids)
    }

    pub fn make_all_strong(&mut self) {
        for open_buffer in self.opened_buffers.values_mut() {
            match open_buffer {
                OpenBuffer::Strong(_) => {}
                OpenBuffer::Weak(buffer) => {
                    if let Some(buffer) = buffer.upgrade() {
                        *open_buffer = OpenBuffer::Strong(buffer);
                    }
                }
                OpenBuffer::Operations(_) => unreachable!(),
            }
        }
    }

    pub fn disconnected_from_host(&mut self, cx: &mut AppContext) {
        self.make_all_weak(cx);

        for buffer in self.buffers() {
            buffer.update(cx, |buffer, cx| {
                buffer.set_capability(Capability::ReadOnly, cx)
            });
        }

        // Wake up all futures currently waiting on a buffer to get opened,
        // to give them a chance to fail now that we've disconnected.
        self.remote_buffer_listeners.clear();
    }

    pub fn make_all_weak(&mut self, cx: &mut AppContext) {
        for open_buffer in self.opened_buffers.values_mut() {
            // Wake up any tasks waiting for peers' edits to this buffer.
            if let Some(buffer) = open_buffer.upgrade() {
                buffer.update(cx, |buffer, _| buffer.give_up_waiting());
            }
            if let OpenBuffer::Strong(buffer) = open_buffer {
                *open_buffer = OpenBuffer::Weak(buffer.downgrade());
            }
        }
    }

    pub fn discard_incomplete(&mut self) {
        self.opened_buffers
            .retain(|_, buffer| !matches!(buffer, OpenBuffer::Operations(_)));
    }

    pub fn file_changed(
        &mut self,
        path: Arc<Path>,
        entry_id: ProjectEntryId,
        worktree_handle: &Model<worktree::Worktree>,
        snapshot: &worktree::Snapshot,
        cx: &mut AppContext,
    ) -> Option<(Model<Buffer>, File, Arc<File>)> {
        let (buffer_id, buffer) = self.get_or_remove_by_file(
            entry_id,
            &ProjectPath {
                worktree_id: snapshot.id(),
                path,
            },
        )?;

        buffer.update(cx, |buffer, cx| {
            let old_file = File::from_dyn(buffer.file())?;
            if old_file.worktree != *worktree_handle {
                return None;
            }

            let new_file = if let Some(entry) = old_file
                .entry_id
                .and_then(|entry_id| snapshot.entry_for_id(entry_id))
            {
                File {
                    is_local: true,
                    entry_id: Some(entry.id),
                    mtime: entry.mtime,
                    path: entry.path.clone(),
                    worktree: worktree_handle.clone(),
                    is_deleted: false,
                    is_private: entry.is_private,
                }
            } else if let Some(entry) = snapshot.entry_for_path(old_file.path.as_ref()) {
                File {
                    is_local: true,
                    entry_id: Some(entry.id),
                    mtime: entry.mtime,
                    path: entry.path.clone(),
                    worktree: worktree_handle.clone(),
                    is_deleted: false,
                    is_private: entry.is_private,
                }
            } else {
                File {
                    is_local: true,
                    entry_id: old_file.entry_id,
                    path: old_file.path.clone(),
                    mtime: old_file.mtime,
                    worktree: worktree_handle.clone(),
                    is_deleted: true,
                    is_private: old_file.is_private,
                }
            };

            if new_file != *old_file {
                if new_file.path != old_file.path {
                    self.local_buffer_ids_by_path.remove(&ProjectPath {
                        path: old_file.path.clone(),
                        worktree_id: old_file.worktree_id(cx),
                    });
                    self.local_buffer_ids_by_path.insert(
                        ProjectPath {
                            worktree_id: new_file.worktree_id(cx),
                            path: new_file.path.clone(),
                        },
                        buffer_id,
                    );
                }

                if new_file.entry_id != old_file.entry_id {
                    if let Some(entry_id) = old_file.entry_id {
                        self.local_buffer_ids_by_entry_id.remove(&entry_id);
                    }
                    if let Some(entry_id) = new_file.entry_id {
                        self.local_buffer_ids_by_entry_id
                            .insert(entry_id, buffer_id);
                    }
                }

                let old_file = old_file.clone();
                let new_file = Arc::new(new_file);
                buffer.file_updated(new_file.clone(), cx);
                return Some((cx.handle(), old_file, new_file));
            }

            None
        })
    }

    pub fn buffer_changed_file(
        &mut self,
        buffer: Model<Buffer>,
        cx: &mut AppContext,
    ) -> Option<()> {
        let file = File::from_dyn(buffer.read(cx).file())?;

        let remote_id = buffer.read(cx).remote_id();
        if let Some(entry_id) = file.entry_id {
            match self.local_buffer_ids_by_entry_id.get(&entry_id) {
                Some(_) => {
                    return None;
                }
                None => {
                    self.local_buffer_ids_by_entry_id
                        .insert(entry_id, remote_id);
                }
            }
        };
        self.local_buffer_ids_by_path.insert(
            ProjectPath {
                worktree_id: file.worktree_id(cx),
                path: file.path.clone(),
            },
            remote_id,
        );

        Some(())
    }

    pub fn handle_update_buffer(
        &mut self,
        envelope: TypedEnvelope<proto::UpdateBuffer>,
        is_remote: bool,
        cx: &mut AppContext,
    ) -> Result<proto::Ack> {
        let payload = envelope.payload.clone();
        let buffer_id = BufferId::new(payload.buffer_id)?;
        let ops = payload
            .operations
            .into_iter()
            .map(language::proto::deserialize_operation)
            .collect::<Result<Vec<_>, _>>()?;
        match self.opened_buffers.entry(buffer_id) {
            hash_map::Entry::Occupied(mut e) => match e.get_mut() {
                OpenBuffer::Strong(buffer) => {
                    buffer.update(cx, |buffer, cx| buffer.apply_ops(ops, cx))?;
                }
                OpenBuffer::Operations(operations) => operations.extend_from_slice(&ops),
                OpenBuffer::Weak(_) => {}
            },
            hash_map::Entry::Vacant(e) => {
                if !is_remote {
                    debug_panic!(
                        "received buffer update from {:?}",
                        envelope.original_sender_id
                    );
                    return Err(anyhow!("received buffer update for non-remote project"));
                }
                e.insert(OpenBuffer::Operations(ops));
            }
        }
        Ok(proto::Ack {})
    }
}

impl OpenBuffer {
    fn upgrade(&self) -> Option<Model<Buffer>> {
        match self {
            OpenBuffer::Strong(handle) => Some(handle.clone()),
            OpenBuffer::Weak(handle) => handle.upgrade(),
            OpenBuffer::Operations(_) => None,
        }
    }
}
