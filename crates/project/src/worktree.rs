use super::{
    fs::{self, Fs},
    ignore::IgnoreStack,
    DiagnosticSummary,
};
use ::ignore::gitignore::{Gitignore, GitignoreBuilder};
use anyhow::{anyhow, Context, Result};
use client::{proto, Client, PeerId, TypedEnvelope, UserStore};
use clock::ReplicaId;
use collections::{hash_map, HashMap};
use collections::{BTreeMap, HashSet};
use futures::{Stream, StreamExt};
use fuzzy::CharBag;
use gpui::{
    executor, AppContext, AsyncAppContext, Entity, ModelContext, ModelHandle, MutableAppContext,
    Task, UpgradeModelHandle, WeakModelHandle,
};
use language::{
    Buffer, Diagnostic, DiagnosticEntry, DiagnosticSeverity, File as _, Language, LanguageRegistry,
    Operation, PointUtf16, Rope,
};
use lazy_static::lazy_static;
use lsp::LanguageServer;
use parking_lot::Mutex;
use postage::{
    prelude::{Sink as _, Stream as _},
    watch,
};
use serde::Deserialize;
use smol::channel::{self, Sender};
use std::{
    any::Any,
    cmp::{self, Ordering},
    convert::{TryFrom, TryInto},
    ffi::{OsStr, OsString},
    fmt,
    future::Future,
    mem,
    ops::{Deref, Range},
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicUsize, Ordering::SeqCst},
        Arc,
    },
    time::{Duration, SystemTime},
};
use sum_tree::Bias;
use sum_tree::{Edit, SeekTarget, SumTree};
use util::{post_inc, ResultExt, TryFutureExt};

lazy_static! {
    static ref GITIGNORE: &'static OsStr = OsStr::new(".gitignore");
    static ref DIAGNOSTIC_PROVIDER_NAME: Arc<str> = Arc::from("diagnostic_source");
    static ref LSP_PROVIDER_NAME: Arc<str> = Arc::from("lsp");
}

#[derive(Clone, Debug)]
enum ScanState {
    Idle,
    Scanning,
    Err(Arc<anyhow::Error>),
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct WorktreeId(usize);

pub enum Worktree {
    Local(LocalWorktree),
    Remote(RemoteWorktree),
}

#[derive(Debug)]
pub enum Event {
    DiagnosticsUpdated(Arc<Path>),
}

impl Entity for Worktree {
    type Event = Event;

    fn app_will_quit(
        &mut self,
        _: &mut MutableAppContext,
    ) -> Option<std::pin::Pin<Box<dyn 'static + Future<Output = ()>>>> {
        use futures::FutureExt;

        if let Self::Local(worktree) = self {
            let shutdown_futures = worktree
                .language_servers
                .drain()
                .filter_map(|(_, server)| server.shutdown())
                .collect::<Vec<_>>();
            Some(
                async move {
                    futures::future::join_all(shutdown_futures).await;
                }
                .boxed(),
            )
        } else {
            None
        }
    }
}

impl Worktree {
    pub async fn open_local(
        client: Arc<Client>,
        user_store: ModelHandle<UserStore>,
        path: impl Into<Arc<Path>>,
        fs: Arc<dyn Fs>,
        languages: Arc<LanguageRegistry>,
        cx: &mut AsyncAppContext,
    ) -> Result<ModelHandle<Self>> {
        let (tree, scan_states_tx) =
            LocalWorktree::new(client, user_store, path, fs.clone(), languages, cx).await?;
        tree.update(cx, |tree, cx| {
            let tree = tree.as_local_mut().unwrap();
            let abs_path = tree.snapshot.abs_path.clone();
            let background_snapshot = tree.background_snapshot.clone();
            let background = cx.background().clone();
            tree._background_scanner_task = Some(cx.background().spawn(async move {
                let events = fs.watch(&abs_path, Duration::from_millis(100)).await;
                let scanner =
                    BackgroundScanner::new(background_snapshot, scan_states_tx, fs, background);
                scanner.run(events).await;
            }));
        });
        Ok(tree)
    }

    pub async fn remote(
        project_remote_id: u64,
        replica_id: ReplicaId,
        worktree: proto::Worktree,
        client: Arc<Client>,
        user_store: ModelHandle<UserStore>,
        languages: Arc<LanguageRegistry>,
        cx: &mut AsyncAppContext,
    ) -> Result<ModelHandle<Self>> {
        let remote_id = worktree.id;
        let root_char_bag: CharBag = worktree
            .root_name
            .chars()
            .map(|c| c.to_ascii_lowercase())
            .collect();
        let root_name = worktree.root_name.clone();
        let (entries_by_path, entries_by_id) = cx
            .background()
            .spawn(async move {
                let mut entries_by_path_edits = Vec::new();
                let mut entries_by_id_edits = Vec::new();
                for entry in worktree.entries {
                    match Entry::try_from((&root_char_bag, entry)) {
                        Ok(entry) => {
                            entries_by_id_edits.push(Edit::Insert(PathEntry {
                                id: entry.id,
                                path: entry.path.clone(),
                                is_ignored: entry.is_ignored,
                                scan_id: 0,
                            }));
                            entries_by_path_edits.push(Edit::Insert(entry));
                        }
                        Err(err) => log::warn!("error for remote worktree entry {:?}", err),
                    }
                }

                let mut entries_by_path = SumTree::new();
                let mut entries_by_id = SumTree::new();
                entries_by_path.edit(entries_by_path_edits, &());
                entries_by_id.edit(entries_by_id_edits, &());
                (entries_by_path, entries_by_id)
            })
            .await;

        let worktree = cx.update(|cx| {
            cx.add_model(|cx: &mut ModelContext<Worktree>| {
                let snapshot = Snapshot {
                    id: WorktreeId(remote_id as usize),
                    scan_id: 0,
                    abs_path: Path::new("").into(),
                    root_name,
                    root_char_bag,
                    ignores: Default::default(),
                    entries_by_path,
                    entries_by_id,
                    removed_entry_ids: Default::default(),
                    next_entry_id: Default::default(),
                };

                let (updates_tx, mut updates_rx) = postage::mpsc::channel(64);
                let (mut snapshot_tx, snapshot_rx) = watch::channel_with(snapshot.clone());

                cx.background()
                    .spawn(async move {
                        while let Some(update) = updates_rx.recv().await {
                            let mut snapshot = snapshot_tx.borrow().clone();
                            if let Err(error) = snapshot.apply_update(update) {
                                log::error!("error applying worktree update: {}", error);
                            }
                            *snapshot_tx.borrow_mut() = snapshot;
                        }
                    })
                    .detach();

                {
                    let mut snapshot_rx = snapshot_rx.clone();
                    cx.spawn_weak(|this, mut cx| async move {
                        while let Some(_) = snapshot_rx.recv().await {
                            if let Some(this) = cx.read(|cx| this.upgrade(cx)) {
                                this.update(&mut cx, |this, cx| this.poll_snapshot(cx));
                            } else {
                                break;
                            }
                        }
                    })
                    .detach();
                }

                Worktree::Remote(RemoteWorktree {
                    project_id: project_remote_id,
                    replica_id,
                    snapshot,
                    snapshot_rx,
                    updates_tx,
                    client: client.clone(),
                    loading_buffers: Default::default(),
                    open_buffers: Default::default(),
                    diagnostic_summaries: Default::default(),
                    queued_operations: Default::default(),
                    languages,
                    user_store,
                })
            })
        });

        Ok(worktree)
    }

    pub fn as_local(&self) -> Option<&LocalWorktree> {
        if let Worktree::Local(worktree) = self {
            Some(worktree)
        } else {
            None
        }
    }

    pub fn as_remote(&self) -> Option<&RemoteWorktree> {
        if let Worktree::Remote(worktree) = self {
            Some(worktree)
        } else {
            None
        }
    }

    pub fn as_local_mut(&mut self) -> Option<&mut LocalWorktree> {
        if let Worktree::Local(worktree) = self {
            Some(worktree)
        } else {
            None
        }
    }

    pub fn as_remote_mut(&mut self) -> Option<&mut RemoteWorktree> {
        if let Worktree::Remote(worktree) = self {
            Some(worktree)
        } else {
            None
        }
    }

    pub fn snapshot(&self) -> Snapshot {
        match self {
            Worktree::Local(worktree) => worktree.snapshot(),
            Worktree::Remote(worktree) => worktree.snapshot(),
        }
    }

    pub fn replica_id(&self) -> ReplicaId {
        match self {
            Worktree::Local(_) => 0,
            Worktree::Remote(worktree) => worktree.replica_id,
        }
    }

    pub fn remove_collaborator(
        &mut self,
        peer_id: PeerId,
        replica_id: ReplicaId,
        cx: &mut ModelContext<Self>,
    ) {
        match self {
            Worktree::Local(worktree) => worktree.remove_collaborator(peer_id, replica_id, cx),
            Worktree::Remote(worktree) => worktree.remove_collaborator(replica_id, cx),
        }
    }

    pub fn languages(&self) -> &Arc<LanguageRegistry> {
        match self {
            Worktree::Local(worktree) => &worktree.language_registry,
            Worktree::Remote(worktree) => &worktree.languages,
        }
    }

    pub fn user_store(&self) -> &ModelHandle<UserStore> {
        match self {
            Worktree::Local(worktree) => &worktree.user_store,
            Worktree::Remote(worktree) => &worktree.user_store,
        }
    }

    pub fn handle_open_buffer(
        &mut self,
        envelope: TypedEnvelope<proto::OpenBuffer>,
        rpc: Arc<Client>,
        cx: &mut ModelContext<Self>,
    ) -> anyhow::Result<()> {
        let receipt = envelope.receipt();

        let response = self
            .as_local_mut()
            .unwrap()
            .open_remote_buffer(envelope, cx);

        cx.background()
            .spawn(
                async move {
                    rpc.respond(receipt, response.await?).await?;
                    Ok(())
                }
                .log_err(),
            )
            .detach();

        Ok(())
    }

    pub fn handle_close_buffer(
        &mut self,
        envelope: TypedEnvelope<proto::CloseBuffer>,
        _: Arc<Client>,
        cx: &mut ModelContext<Self>,
    ) -> anyhow::Result<()> {
        self.as_local_mut()
            .unwrap()
            .close_remote_buffer(envelope, cx)
    }

    pub fn diagnostic_summaries<'a>(
        &'a self,
    ) -> impl Iterator<Item = (Arc<Path>, DiagnosticSummary)> + 'a {
        match self {
            Worktree::Local(worktree) => &worktree.diagnostic_summaries,
            Worktree::Remote(worktree) => &worktree.diagnostic_summaries,
        }
        .iter()
        .map(|(path, summary)| (path.clone(), summary.clone()))
    }

    pub fn loading_buffers<'a>(&'a mut self) -> &'a mut LoadingBuffers {
        match self {
            Worktree::Local(worktree) => &mut worktree.loading_buffers,
            Worktree::Remote(worktree) => &mut worktree.loading_buffers,
        }
    }

    pub fn open_buffer(
        &mut self,
        path: impl AsRef<Path>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<ModelHandle<Buffer>>> {
        let path = path.as_ref();

        // If there is already a buffer for the given path, then return it.
        let existing_buffer = match self {
            Worktree::Local(worktree) => worktree.get_open_buffer(path, cx),
            Worktree::Remote(worktree) => worktree.get_open_buffer(path, cx),
        };
        if let Some(existing_buffer) = existing_buffer {
            return cx.spawn(move |_, _| async move { Ok(existing_buffer) });
        }

        let path: Arc<Path> = Arc::from(path);
        let mut loading_watch = match self.loading_buffers().entry(path.clone()) {
            // If the given path is already being loaded, then wait for that existing
            // task to complete and return the same buffer.
            hash_map::Entry::Occupied(e) => e.get().clone(),

            // Otherwise, record the fact that this path is now being loaded.
            hash_map::Entry::Vacant(entry) => {
                let (mut tx, rx) = postage::watch::channel();
                entry.insert(rx.clone());

                let load_buffer = match self {
                    Worktree::Local(worktree) => worktree.open_buffer(&path, cx),
                    Worktree::Remote(worktree) => worktree.open_buffer(&path, cx),
                };
                cx.spawn(move |this, mut cx| async move {
                    let result = load_buffer.await;

                    // After the buffer loads, record the fact that it is no longer
                    // loading.
                    this.update(&mut cx, |this, _| this.loading_buffers().remove(&path));
                    *tx.borrow_mut() = Some(result.map_err(|e| Arc::new(e)));
                })
                .detach();
                rx
            }
        };

        cx.spawn(|_, _| async move {
            loop {
                if let Some(result) = loading_watch.borrow().as_ref() {
                    return result.clone().map_err(|e| anyhow!("{}", e));
                }
                loading_watch.recv().await;
            }
        })
    }

    #[cfg(feature = "test-support")]
    pub fn has_open_buffer(&self, path: impl AsRef<Path>, cx: &AppContext) -> bool {
        let mut open_buffers: Box<dyn Iterator<Item = _>> = match self {
            Worktree::Local(worktree) => Box::new(worktree.open_buffers.values()),
            Worktree::Remote(worktree) => {
                Box::new(worktree.open_buffers.values().filter_map(|buf| {
                    if let RemoteBuffer::Loaded(buf) = buf {
                        Some(buf)
                    } else {
                        None
                    }
                }))
            }
        };

        let path = path.as_ref();
        open_buffers
            .find(|buffer| {
                if let Some(file) = buffer.upgrade(cx).and_then(|buffer| buffer.read(cx).file()) {
                    file.path().as_ref() == path
                } else {
                    false
                }
            })
            .is_some()
    }

    pub fn handle_update_buffer(
        &mut self,
        envelope: TypedEnvelope<proto::UpdateBuffer>,
        cx: &mut ModelContext<Self>,
    ) -> Result<()> {
        let payload = envelope.payload.clone();
        let buffer_id = payload.buffer_id as usize;
        let ops = payload
            .operations
            .into_iter()
            .map(|op| language::proto::deserialize_operation(op))
            .collect::<Result<Vec<_>, _>>()?;

        match self {
            Worktree::Local(worktree) => {
                let buffer = worktree
                    .open_buffers
                    .get(&buffer_id)
                    .and_then(|buf| buf.upgrade(cx))
                    .ok_or_else(|| {
                        anyhow!("invalid buffer {} in update buffer message", buffer_id)
                    })?;
                buffer.update(cx, |buffer, cx| buffer.apply_ops(ops, cx))?;
            }
            Worktree::Remote(worktree) => match worktree.open_buffers.get_mut(&buffer_id) {
                Some(RemoteBuffer::Operations(pending_ops)) => pending_ops.extend(ops),
                Some(RemoteBuffer::Loaded(buffer)) => {
                    if let Some(buffer) = buffer.upgrade(cx) {
                        buffer.update(cx, |buffer, cx| buffer.apply_ops(ops, cx))?;
                    } else {
                        worktree
                            .open_buffers
                            .insert(buffer_id, RemoteBuffer::Operations(ops));
                    }
                }
                None => {
                    worktree
                        .open_buffers
                        .insert(buffer_id, RemoteBuffer::Operations(ops));
                }
            },
        }

        Ok(())
    }

    pub fn handle_save_buffer(
        &mut self,
        envelope: TypedEnvelope<proto::SaveBuffer>,
        rpc: Arc<Client>,
        cx: &mut ModelContext<Self>,
    ) -> Result<()> {
        let sender_id = envelope.original_sender_id()?;
        let this = self.as_local().unwrap();
        let project_id = this
            .share
            .as_ref()
            .ok_or_else(|| anyhow!("can't save buffer while disconnected"))?
            .project_id;

        let buffer = this
            .shared_buffers
            .get(&sender_id)
            .and_then(|shared_buffers| shared_buffers.get(&envelope.payload.buffer_id).cloned())
            .ok_or_else(|| anyhow!("unknown buffer id {}", envelope.payload.buffer_id))?;

        let receipt = envelope.receipt();
        let worktree_id = envelope.payload.worktree_id;
        let buffer_id = envelope.payload.buffer_id;
        let save = cx.spawn(|_, mut cx| async move {
            buffer.update(&mut cx, |buffer, cx| buffer.save(cx))?.await
        });

        cx.background()
            .spawn(
                async move {
                    let (version, mtime) = save.await?;

                    rpc.respond(
                        receipt,
                        proto::BufferSaved {
                            project_id,
                            worktree_id,
                            buffer_id,
                            version: (&version).into(),
                            mtime: Some(mtime.into()),
                        },
                    )
                    .await?;

                    Ok(())
                }
                .log_err(),
            )
            .detach();

        Ok(())
    }

    pub fn handle_buffer_saved(
        &mut self,
        envelope: TypedEnvelope<proto::BufferSaved>,
        cx: &mut ModelContext<Self>,
    ) -> Result<()> {
        let payload = envelope.payload.clone();
        let worktree = self.as_remote_mut().unwrap();
        if let Some(buffer) = worktree
            .open_buffers
            .get(&(payload.buffer_id as usize))
            .and_then(|buf| buf.upgrade(cx))
        {
            buffer.update(cx, |buffer, cx| {
                let version = payload.version.try_into()?;
                let mtime = payload
                    .mtime
                    .ok_or_else(|| anyhow!("missing mtime"))?
                    .into();
                buffer.did_save(version, mtime, None, cx);
                Result::<_, anyhow::Error>::Ok(())
            })?;
        }
        Ok(())
    }

    fn poll_snapshot(&mut self, cx: &mut ModelContext<Self>) {
        match self {
            Self::Local(worktree) => {
                let is_fake_fs = worktree.fs.is_fake();
                worktree.snapshot = worktree.background_snapshot.lock().clone();
                if worktree.is_scanning() {
                    if worktree.poll_task.is_none() {
                        worktree.poll_task = Some(cx.spawn(|this, mut cx| async move {
                            if is_fake_fs {
                                smol::future::yield_now().await;
                            } else {
                                smol::Timer::after(Duration::from_millis(100)).await;
                            }
                            this.update(&mut cx, |this, cx| {
                                this.as_local_mut().unwrap().poll_task = None;
                                this.poll_snapshot(cx);
                            })
                        }));
                    }
                } else {
                    worktree.poll_task.take();
                    self.update_open_buffers(cx);
                }
            }
            Self::Remote(worktree) => {
                worktree.snapshot = worktree.snapshot_rx.borrow().clone();
                self.update_open_buffers(cx);
            }
        };

        cx.notify();
    }

    fn update_open_buffers(&mut self, cx: &mut ModelContext<Self>) {
        let open_buffers: Box<dyn Iterator<Item = _>> = match &self {
            Self::Local(worktree) => Box::new(worktree.open_buffers.iter()),
            Self::Remote(worktree) => {
                Box::new(worktree.open_buffers.iter().filter_map(|(id, buf)| {
                    if let RemoteBuffer::Loaded(buf) = buf {
                        Some((id, buf))
                    } else {
                        None
                    }
                }))
            }
        };

        let local = self.as_local().is_some();
        let worktree_path = self.abs_path.clone();
        let worktree_handle = cx.handle();
        let mut buffers_to_delete = Vec::new();
        for (buffer_id, buffer) in open_buffers {
            if let Some(buffer) = buffer.upgrade(cx) {
                buffer.update(cx, |buffer, cx| {
                    if let Some(old_file) = File::from_dyn(buffer.file()) {
                        let new_file = if let Some(entry) = old_file
                            .entry_id
                            .and_then(|entry_id| self.entry_for_id(entry_id))
                        {
                            File {
                                is_local: local,
                                worktree_path: worktree_path.clone(),
                                entry_id: Some(entry.id),
                                mtime: entry.mtime,
                                path: entry.path.clone(),
                                worktree: worktree_handle.clone(),
                            }
                        } else if let Some(entry) = self.entry_for_path(old_file.path().as_ref()) {
                            File {
                                is_local: local,
                                worktree_path: worktree_path.clone(),
                                entry_id: Some(entry.id),
                                mtime: entry.mtime,
                                path: entry.path.clone(),
                                worktree: worktree_handle.clone(),
                            }
                        } else {
                            File {
                                is_local: local,
                                worktree_path: worktree_path.clone(),
                                entry_id: None,
                                path: old_file.path().clone(),
                                mtime: old_file.mtime(),
                                worktree: worktree_handle.clone(),
                            }
                        };

                        if let Some(task) = buffer.file_updated(Box::new(new_file), cx) {
                            task.detach();
                        }
                    }
                });
            } else {
                buffers_to_delete.push(*buffer_id);
            }
        }

        for buffer_id in buffers_to_delete {
            match self {
                Self::Local(worktree) => {
                    worktree.open_buffers.remove(&buffer_id);
                }
                Self::Remote(worktree) => {
                    worktree.open_buffers.remove(&buffer_id);
                }
            }
        }
    }

    pub fn update_diagnostics_from_lsp(
        &mut self,
        mut params: lsp::PublishDiagnosticsParams,
        disk_based_sources: &HashSet<String>,
        cx: &mut ModelContext<Worktree>,
    ) -> Result<()> {
        let this = self.as_local_mut().ok_or_else(|| anyhow!("not local"))?;
        let abs_path = params
            .uri
            .to_file_path()
            .map_err(|_| anyhow!("URI is not a file"))?;
        let worktree_path = Arc::from(
            abs_path
                .strip_prefix(&this.abs_path)
                .context("path is not within worktree")?,
        );

        let mut group_ids_by_diagnostic_range = HashMap::default();
        let mut diagnostics_by_group_id = HashMap::default();
        let mut next_group_id = 0;
        for diagnostic in &mut params.diagnostics {
            let source = diagnostic.source.as_ref();
            let code = diagnostic.code.as_ref();
            let group_id = diagnostic_ranges(&diagnostic, &abs_path)
                .find_map(|range| group_ids_by_diagnostic_range.get(&(source, code, range)))
                .copied()
                .unwrap_or_else(|| {
                    let group_id = post_inc(&mut next_group_id);
                    for range in diagnostic_ranges(&diagnostic, &abs_path) {
                        group_ids_by_diagnostic_range.insert((source, code, range), group_id);
                    }
                    group_id
                });

            diagnostics_by_group_id
                .entry(group_id)
                .or_insert(Vec::new())
                .push(DiagnosticEntry {
                    range: diagnostic.range.start.to_point_utf16()
                        ..diagnostic.range.end.to_point_utf16(),
                    diagnostic: Diagnostic {
                        code: diagnostic.code.clone().map(|code| match code {
                            lsp::NumberOrString::Number(code) => code.to_string(),
                            lsp::NumberOrString::String(code) => code,
                        }),
                        severity: diagnostic.severity.unwrap_or(DiagnosticSeverity::ERROR),
                        message: mem::take(&mut diagnostic.message),
                        group_id,
                        is_primary: false,
                        is_valid: true,
                        is_disk_based: diagnostic
                            .source
                            .as_ref()
                            .map_or(false, |source| disk_based_sources.contains(source)),
                    },
                });
        }

        let diagnostics = diagnostics_by_group_id
            .into_values()
            .flat_map(|mut diagnostics| {
                let primary = diagnostics
                    .iter_mut()
                    .min_by_key(|entry| entry.diagnostic.severity)
                    .unwrap();
                primary.diagnostic.is_primary = true;
                diagnostics
            })
            .collect::<Vec<_>>();

        let this = self.as_local_mut().unwrap();
        for buffer in this.open_buffers.values() {
            if let Some(buffer) = buffer.upgrade(cx) {
                if buffer
                    .read(cx)
                    .file()
                    .map_or(false, |file| *file.path() == worktree_path)
                {
                    let (remote_id, operation) = buffer.update(cx, |buffer, cx| {
                        (
                            buffer.remote_id(),
                            buffer.update_diagnostics(
                                LSP_PROVIDER_NAME.clone(),
                                params.version,
                                diagnostics.clone(),
                                cx,
                            ),
                        )
                    });
                    self.send_buffer_update(remote_id, operation?, cx);
                    break;
                }
            }
        }

        let this = self.as_local_mut().unwrap();
        this.diagnostic_summaries
            .insert(worktree_path.clone(), DiagnosticSummary::new(&diagnostics));
        this.lsp_diagnostics
            .insert(worktree_path.clone(), diagnostics);
        cx.emit(Event::DiagnosticsUpdated(worktree_path.clone()));
        Ok(())
    }

    pub fn update_diagnostics_from_provider(
        &mut self,
        path: Arc<Path>,
        diagnostics: Vec<DiagnosticEntry<usize>>,
        cx: &mut ModelContext<Worktree>,
    ) -> Result<()> {
        let this = self.as_local_mut().unwrap();
        for buffer in this.open_buffers.values() {
            if let Some(buffer) = buffer.upgrade(cx) {
                if buffer
                    .read(cx)
                    .file()
                    .map_or(false, |file| *file.path() == path)
                {
                    let (remote_id, operation) = buffer.update(cx, |buffer, cx| {
                        (
                            buffer.remote_id(),
                            buffer.update_diagnostics(
                                DIAGNOSTIC_PROVIDER_NAME.clone(),
                                None,
                                diagnostics.clone(),
                                cx,
                            ),
                        )
                    });
                    self.send_buffer_update(remote_id, operation?, cx);
                    break;
                }
            }
        }

        let this = self.as_local_mut().unwrap();
        this.diagnostic_summaries
            .insert(path.clone(), DiagnosticSummary::new(&diagnostics));
        this.provider_diagnostics.insert(path.clone(), diagnostics);
        cx.emit(Event::DiagnosticsUpdated(path.clone()));
        Ok(())
    }

    fn send_buffer_update(
        &mut self,
        buffer_id: u64,
        operation: Operation,
        cx: &mut ModelContext<Self>,
    ) {
        if let Some((project_id, worktree_id, rpc)) = match self {
            Worktree::Local(worktree) => worktree
                .share
                .as_ref()
                .map(|share| (share.project_id, worktree.id(), worktree.client.clone())),
            Worktree::Remote(worktree) => Some((
                worktree.project_id,
                worktree.snapshot.id(),
                worktree.client.clone(),
            )),
        } {
            cx.spawn(|worktree, mut cx| async move {
                if let Err(error) = rpc
                    .request(proto::UpdateBuffer {
                        project_id,
                        worktree_id: worktree_id.0 as u64,
                        buffer_id,
                        operations: vec![language::proto::serialize_operation(&operation)],
                    })
                    .await
                {
                    worktree.update(&mut cx, |worktree, _| {
                        log::error!("error sending buffer operation: {}", error);
                        match worktree {
                            Worktree::Local(t) => &mut t.queued_operations,
                            Worktree::Remote(t) => &mut t.queued_operations,
                        }
                        .push((buffer_id, operation));
                    });
                }
            })
            .detach();
        }
    }
}

impl WorktreeId {
    pub fn from_usize(handle_id: usize) -> Self {
        Self(handle_id)
    }

    pub(crate) fn from_proto(id: u64) -> Self {
        Self(id as usize)
    }

    pub fn to_proto(&self) -> u64 {
        self.0 as u64
    }

    pub fn to_usize(&self) -> usize {
        self.0
    }
}

impl fmt::Display for WorktreeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Clone)]
pub struct Snapshot {
    id: WorktreeId,
    scan_id: usize,
    abs_path: Arc<Path>,
    root_name: String,
    root_char_bag: CharBag,
    ignores: HashMap<Arc<Path>, (Arc<Gitignore>, usize)>,
    entries_by_path: SumTree<Entry>,
    entries_by_id: SumTree<PathEntry>,
    removed_entry_ids: HashMap<u64, usize>,
    next_entry_id: Arc<AtomicUsize>,
}

pub struct LocalWorktree {
    snapshot: Snapshot,
    config: WorktreeConfig,
    background_snapshot: Arc<Mutex<Snapshot>>,
    last_scan_state_rx: watch::Receiver<ScanState>,
    _background_scanner_task: Option<Task<()>>,
    poll_task: Option<Task<()>>,
    share: Option<ShareState>,
    loading_buffers: LoadingBuffers,
    open_buffers: HashMap<usize, WeakModelHandle<Buffer>>,
    shared_buffers: HashMap<PeerId, HashMap<u64, ModelHandle<Buffer>>>,
    lsp_diagnostics: HashMap<Arc<Path>, Vec<DiagnosticEntry<PointUtf16>>>,
    provider_diagnostics: HashMap<Arc<Path>, Vec<DiagnosticEntry<usize>>>,
    diagnostic_summaries: BTreeMap<Arc<Path>, DiagnosticSummary>,
    queued_operations: Vec<(u64, Operation)>,
    language_registry: Arc<LanguageRegistry>,
    client: Arc<Client>,
    user_store: ModelHandle<UserStore>,
    fs: Arc<dyn Fs>,
    languages: Vec<Arc<Language>>,
    language_servers: HashMap<String, Arc<LanguageServer>>,
}

struct ShareState {
    project_id: u64,
    snapshots_tx: Sender<Snapshot>,
}

pub struct RemoteWorktree {
    project_id: u64,
    snapshot: Snapshot,
    snapshot_rx: watch::Receiver<Snapshot>,
    client: Arc<Client>,
    updates_tx: postage::mpsc::Sender<proto::UpdateWorktree>,
    replica_id: ReplicaId,
    loading_buffers: LoadingBuffers,
    open_buffers: HashMap<usize, RemoteBuffer>,
    diagnostic_summaries: BTreeMap<Arc<Path>, DiagnosticSummary>,
    languages: Arc<LanguageRegistry>,
    user_store: ModelHandle<UserStore>,
    queued_operations: Vec<(u64, Operation)>,
}

type LoadingBuffers = HashMap<
    Arc<Path>,
    postage::watch::Receiver<Option<Result<ModelHandle<Buffer>, Arc<anyhow::Error>>>>,
>;

#[derive(Default, Deserialize)]
struct WorktreeConfig {
    collaborators: Vec<String>,
}

impl LocalWorktree {
    async fn new(
        client: Arc<Client>,
        user_store: ModelHandle<UserStore>,
        path: impl Into<Arc<Path>>,
        fs: Arc<dyn Fs>,
        languages: Arc<LanguageRegistry>,
        cx: &mut AsyncAppContext,
    ) -> Result<(ModelHandle<Worktree>, Sender<ScanState>)> {
        let abs_path = path.into();
        let path: Arc<Path> = Arc::from(Path::new(""));
        let next_entry_id = AtomicUsize::new(0);

        // After determining whether the root entry is a file or a directory, populate the
        // snapshot's "root name", which will be used for the purpose of fuzzy matching.
        let root_name = abs_path
            .file_name()
            .map_or(String::new(), |f| f.to_string_lossy().to_string());
        let root_char_bag = root_name.chars().map(|c| c.to_ascii_lowercase()).collect();
        let metadata = fs.metadata(&abs_path).await?;

        let mut config = WorktreeConfig::default();
        if let Ok(zed_toml) = fs.load(&abs_path.join(".zed.toml")).await {
            if let Ok(parsed) = toml::from_str(&zed_toml) {
                config = parsed;
            }
        }

        let (scan_states_tx, scan_states_rx) = smol::channel::unbounded();
        let (mut last_scan_state_tx, last_scan_state_rx) = watch::channel_with(ScanState::Scanning);
        let tree = cx.add_model(move |cx: &mut ModelContext<Worktree>| {
            let mut snapshot = Snapshot {
                id: WorktreeId::from_usize(cx.model_id()),
                scan_id: 0,
                abs_path,
                root_name: root_name.clone(),
                root_char_bag,
                ignores: Default::default(),
                entries_by_path: Default::default(),
                entries_by_id: Default::default(),
                removed_entry_ids: Default::default(),
                next_entry_id: Arc::new(next_entry_id),
            };
            if let Some(metadata) = metadata {
                snapshot.insert_entry(
                    Entry::new(
                        path.into(),
                        &metadata,
                        &snapshot.next_entry_id,
                        snapshot.root_char_bag,
                    ),
                    fs.as_ref(),
                );
            }

            let tree = Self {
                snapshot: snapshot.clone(),
                config,
                background_snapshot: Arc::new(Mutex::new(snapshot)),
                last_scan_state_rx,
                _background_scanner_task: None,
                share: None,
                poll_task: None,
                loading_buffers: Default::default(),
                open_buffers: Default::default(),
                shared_buffers: Default::default(),
                lsp_diagnostics: Default::default(),
                provider_diagnostics: Default::default(),
                diagnostic_summaries: Default::default(),
                queued_operations: Default::default(),
                language_registry: languages,
                client,
                user_store,
                fs,
                languages: Default::default(),
                language_servers: Default::default(),
            };

            cx.spawn_weak(|this, mut cx| async move {
                while let Ok(scan_state) = scan_states_rx.recv().await {
                    if let Some(handle) = cx.read(|cx| this.upgrade(cx)) {
                        let to_send = handle.update(&mut cx, |this, cx| {
                            last_scan_state_tx.blocking_send(scan_state).ok();
                            this.poll_snapshot(cx);
                            let tree = this.as_local_mut().unwrap();
                            if !tree.is_scanning() {
                                if let Some(share) = tree.share.as_ref() {
                                    return Some((tree.snapshot(), share.snapshots_tx.clone()));
                                }
                            }
                            None
                        });

                        if let Some((snapshot, snapshots_to_send_tx)) = to_send {
                            if let Err(err) = snapshots_to_send_tx.send(snapshot).await {
                                log::error!("error submitting snapshot to send {}", err);
                            }
                        }
                    } else {
                        break;
                    }
                }
            })
            .detach();

            Worktree::Local(tree)
        });

        Ok((tree, scan_states_tx))
    }

    pub fn authorized_logins(&self) -> Vec<String> {
        self.config.collaborators.clone()
    }

    pub fn language_registry(&self) -> &LanguageRegistry {
        &self.language_registry
    }

    pub fn languages(&self) -> &[Arc<Language>] {
        &self.languages
    }

    pub fn register_language(
        &mut self,
        language: &Arc<Language>,
        cx: &mut ModelContext<Worktree>,
    ) -> Option<Arc<LanguageServer>> {
        if !self.languages.iter().any(|l| Arc::ptr_eq(l, language)) {
            self.languages.push(language.clone());
        }

        if let Some(server) = self.language_servers.get(language.name()) {
            return Some(server.clone());
        }

        if let Some(language_server) = language
            .start_server(self.abs_path(), cx)
            .log_err()
            .flatten()
        {
            let disk_based_sources = language
                .disk_based_diagnostic_sources()
                .cloned()
                .unwrap_or_default();
            let (diagnostics_tx, diagnostics_rx) = smol::channel::unbounded();
            language_server
                .on_notification::<lsp::notification::PublishDiagnostics, _>(move |params| {
                    smol::block_on(diagnostics_tx.send(params)).ok();
                })
                .detach();
            cx.spawn_weak(|this, mut cx| async move {
                while let Ok(diagnostics) = diagnostics_rx.recv().await {
                    if let Some(handle) = cx.read(|cx| this.upgrade(cx)) {
                        handle.update(&mut cx, |this, cx| {
                            this.update_diagnostics_from_lsp(diagnostics, &disk_based_sources, cx)
                                .log_err();
                        });
                    } else {
                        break;
                    }
                }
            })
            .detach();

            self.language_servers
                .insert(language.name().to_string(), language_server.clone());
            Some(language_server.clone())
        } else {
            None
        }
    }

    fn get_open_buffer(
        &mut self,
        path: &Path,
        cx: &mut ModelContext<Worktree>,
    ) -> Option<ModelHandle<Buffer>> {
        let handle = cx.handle();
        let mut result = None;
        self.open_buffers.retain(|_buffer_id, buffer| {
            if let Some(buffer) = buffer.upgrade(cx) {
                if let Some(file) = File::from_dyn(buffer.read(cx).file()) {
                    if file.worktree == handle && file.path().as_ref() == path {
                        result = Some(buffer);
                    }
                }
                true
            } else {
                false
            }
        });
        result
    }

    fn open_buffer(
        &mut self,
        path: &Path,
        cx: &mut ModelContext<Worktree>,
    ) -> Task<Result<ModelHandle<Buffer>>> {
        let path = Arc::from(path);
        cx.spawn(move |this, mut cx| async move {
            let (file, contents) = this
                .update(&mut cx, |t, cx| t.as_local().unwrap().load(&path, cx))
                .await?;

            let (lsp_diagnostics, provider_diagnostics, language, language_server) =
                this.update(&mut cx, |this, cx| {
                    let this = this.as_local_mut().unwrap();
                    let lsp_diagnostics = this.lsp_diagnostics.remove(&path);
                    let provider_diagnostics = this.provider_diagnostics.remove(&path);
                    let language = this
                        .language_registry
                        .select_language(file.full_path())
                        .cloned();
                    let server = language
                        .as_ref()
                        .and_then(|language| this.register_language(language, cx));
                    (lsp_diagnostics, provider_diagnostics, language, server)
                });

            let mut buffer_operations = Vec::new();
            let buffer = cx.add_model(|cx| {
                let mut buffer = Buffer::from_file(0, contents, Box::new(file), cx);
                buffer.set_language(language, language_server, cx);
                if let Some(diagnostics) = lsp_diagnostics {
                    let op = buffer
                        .update_diagnostics(LSP_PROVIDER_NAME.clone(), None, diagnostics, cx)
                        .unwrap();
                    buffer_operations.push(op);
                }
                if let Some(diagnostics) = provider_diagnostics {
                    let op = buffer
                        .update_diagnostics(DIAGNOSTIC_PROVIDER_NAME.clone(), None, diagnostics, cx)
                        .unwrap();
                    buffer_operations.push(op);
                }
                buffer
            });

            this.update(&mut cx, |this, cx| {
                for op in buffer_operations {
                    this.send_buffer_update(buffer.read(cx).remote_id(), op, cx);
                }
                let this = this.as_local_mut().unwrap();
                this.open_buffers.insert(buffer.id(), buffer.downgrade());
            });

            Ok(buffer)
        })
    }

    pub fn open_remote_buffer(
        &mut self,
        envelope: TypedEnvelope<proto::OpenBuffer>,
        cx: &mut ModelContext<Worktree>,
    ) -> Task<Result<proto::OpenBufferResponse>> {
        cx.spawn(|this, mut cx| async move {
            let peer_id = envelope.original_sender_id();
            let path = Path::new(&envelope.payload.path);
            let buffer = this
                .update(&mut cx, |this, cx| this.open_buffer(path, cx))
                .await?;
            this.update(&mut cx, |this, cx| {
                this.as_local_mut()
                    .unwrap()
                    .shared_buffers
                    .entry(peer_id?)
                    .or_default()
                    .insert(buffer.id() as u64, buffer.clone());

                Ok(proto::OpenBufferResponse {
                    buffer: Some(buffer.update(cx.as_mut(), |buffer, _| buffer.to_proto())),
                })
            })
        })
    }

    pub fn close_remote_buffer(
        &mut self,
        envelope: TypedEnvelope<proto::CloseBuffer>,
        cx: &mut ModelContext<Worktree>,
    ) -> Result<()> {
        if let Some(shared_buffers) = self.shared_buffers.get_mut(&envelope.original_sender_id()?) {
            shared_buffers.remove(&envelope.payload.buffer_id);
            cx.notify();
        }

        Ok(())
    }

    pub fn remove_collaborator(
        &mut self,
        peer_id: PeerId,
        replica_id: ReplicaId,
        cx: &mut ModelContext<Worktree>,
    ) {
        self.shared_buffers.remove(&peer_id);
        for (_, buffer) in &self.open_buffers {
            if let Some(buffer) = buffer.upgrade(cx) {
                buffer.update(cx, |buffer, cx| buffer.remove_peer(replica_id, cx));
            }
        }
        cx.notify();
    }

    pub fn scan_complete(&self) -> impl Future<Output = ()> {
        let mut scan_state_rx = self.last_scan_state_rx.clone();
        async move {
            let mut scan_state = Some(scan_state_rx.borrow().clone());
            while let Some(ScanState::Scanning) = scan_state {
                scan_state = scan_state_rx.recv().await;
            }
        }
    }

    fn is_scanning(&self) -> bool {
        if let ScanState::Scanning = *self.last_scan_state_rx.borrow() {
            true
        } else {
            false
        }
    }

    pub fn snapshot(&self) -> Snapshot {
        self.snapshot.clone()
    }

    pub fn abs_path(&self) -> &Arc<Path> {
        &self.snapshot.abs_path
    }

    pub fn contains_abs_path(&self, path: &Path) -> bool {
        path.starts_with(&self.snapshot.abs_path)
    }

    fn absolutize(&self, path: &Path) -> PathBuf {
        if path.file_name().is_some() {
            self.snapshot.abs_path.join(path)
        } else {
            self.snapshot.abs_path.to_path_buf()
        }
    }

    fn load(&self, path: &Path, cx: &mut ModelContext<Worktree>) -> Task<Result<(File, String)>> {
        let handle = cx.handle();
        let path = Arc::from(path);
        let worktree_path = self.abs_path.clone();
        let abs_path = self.absolutize(&path);
        let background_snapshot = self.background_snapshot.clone();
        let fs = self.fs.clone();
        cx.spawn(|this, mut cx| async move {
            let text = fs.load(&abs_path).await?;
            // Eagerly populate the snapshot with an updated entry for the loaded file
            let entry = refresh_entry(fs.as_ref(), &background_snapshot, path, &abs_path).await?;
            this.update(&mut cx, |this, cx| this.poll_snapshot(cx));
            Ok((
                File {
                    entry_id: Some(entry.id),
                    worktree: handle,
                    worktree_path,
                    path: entry.path,
                    mtime: entry.mtime,
                    is_local: true,
                },
                text,
            ))
        })
    }

    pub fn save_buffer_as(
        &self,
        buffer: ModelHandle<Buffer>,
        path: impl Into<Arc<Path>>,
        text: Rope,
        cx: &mut ModelContext<Worktree>,
    ) -> Task<Result<File>> {
        let save = self.save(path, text, cx);
        cx.spawn(|this, mut cx| async move {
            let entry = save.await?;
            this.update(&mut cx, |this, cx| {
                let this = this.as_local_mut().unwrap();
                this.open_buffers.insert(buffer.id(), buffer.downgrade());
                Ok(File {
                    entry_id: Some(entry.id),
                    worktree: cx.handle(),
                    worktree_path: this.abs_path.clone(),
                    path: entry.path,
                    mtime: entry.mtime,
                    is_local: true,
                })
            })
        })
    }

    fn save(
        &self,
        path: impl Into<Arc<Path>>,
        text: Rope,
        cx: &mut ModelContext<Worktree>,
    ) -> Task<Result<Entry>> {
        let path = path.into();
        let abs_path = self.absolutize(&path);
        let background_snapshot = self.background_snapshot.clone();
        let fs = self.fs.clone();
        let save = cx.background().spawn(async move {
            fs.save(&abs_path, &text).await?;
            refresh_entry(fs.as_ref(), &background_snapshot, path.clone(), &abs_path).await
        });

        cx.spawn(|this, mut cx| async move {
            let entry = save.await?;
            this.update(&mut cx, |this, cx| this.poll_snapshot(cx));
            Ok(entry)
        })
    }

    pub fn share(
        &mut self,
        project_id: u64,
        cx: &mut ModelContext<Worktree>,
    ) -> Task<anyhow::Result<()>> {
        if self.share.is_some() {
            return Task::ready(Ok(()));
        }

        let snapshot = self.snapshot();
        let rpc = self.client.clone();
        let worktree_id = cx.model_id() as u64;
        let (snapshots_to_send_tx, snapshots_to_send_rx) = smol::channel::unbounded::<Snapshot>();
        self.share = Some(ShareState {
            project_id,
            snapshots_tx: snapshots_to_send_tx,
        });

        cx.background()
            .spawn({
                let rpc = rpc.clone();
                let snapshot = snapshot.clone();
                async move {
                    let mut prev_snapshot = snapshot;
                    while let Ok(snapshot) = snapshots_to_send_rx.recv().await {
                        let message =
                            snapshot.build_update(&prev_snapshot, project_id, worktree_id, false);
                        match rpc.send(message).await {
                            Ok(()) => prev_snapshot = snapshot,
                            Err(err) => log::error!("error sending snapshot diff {}", err),
                        }
                    }
                }
            })
            .detach();

        let share_message = cx.background().spawn(async move {
            proto::ShareWorktree {
                project_id,
                worktree: Some(snapshot.to_proto()),
            }
        });

        cx.foreground().spawn(async move {
            rpc.request(share_message.await).await?;
            Ok(())
        })
    }
}

fn build_gitignore(abs_path: &Path, fs: &dyn Fs) -> Result<Gitignore> {
    let contents = smol::block_on(fs.load(&abs_path))?;
    let parent = abs_path.parent().unwrap_or(Path::new("/"));
    let mut builder = GitignoreBuilder::new(parent);
    for line in contents.lines() {
        builder.add_line(Some(abs_path.into()), line)?;
    }
    Ok(builder.build()?)
}

impl Deref for Worktree {
    type Target = Snapshot;

    fn deref(&self) -> &Self::Target {
        match self {
            Worktree::Local(worktree) => &worktree.snapshot,
            Worktree::Remote(worktree) => &worktree.snapshot,
        }
    }
}

impl Deref for LocalWorktree {
    type Target = Snapshot;

    fn deref(&self) -> &Self::Target {
        &self.snapshot
    }
}

impl Deref for RemoteWorktree {
    type Target = Snapshot;

    fn deref(&self) -> &Self::Target {
        &self.snapshot
    }
}

impl fmt::Debug for LocalWorktree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.snapshot.fmt(f)
    }
}

impl RemoteWorktree {
    fn get_open_buffer(
        &mut self,
        path: &Path,
        cx: &mut ModelContext<Worktree>,
    ) -> Option<ModelHandle<Buffer>> {
        let handle = cx.handle();
        let mut existing_buffer = None;
        self.open_buffers.retain(|_buffer_id, buffer| {
            if let Some(buffer) = buffer.upgrade(cx.as_ref()) {
                if let Some(file) = File::from_dyn(buffer.read(cx).file()) {
                    if file.worktree == handle && file.path().as_ref() == path {
                        existing_buffer = Some(buffer);
                    }
                }
                true
            } else {
                false
            }
        });
        existing_buffer
    }

    fn open_buffer(
        &mut self,
        path: &Path,
        cx: &mut ModelContext<Worktree>,
    ) -> Task<Result<ModelHandle<Buffer>>> {
        let rpc = self.client.clone();
        let replica_id = self.replica_id;
        let project_id = self.project_id;
        let remote_worktree_id = self.id();
        let root_path = self.snapshot.abs_path.clone();
        let path: Arc<Path> = Arc::from(path);
        let path_string = path.to_string_lossy().to_string();
        cx.spawn_weak(move |this, mut cx| async move {
            let entry = this
                .upgrade(&cx)
                .ok_or_else(|| anyhow!("worktree was closed"))?
                .read_with(&cx, |tree, _| tree.entry_for_path(&path).cloned())
                .ok_or_else(|| anyhow!("file does not exist"))?;
            let response = rpc
                .request(proto::OpenBuffer {
                    project_id,
                    worktree_id: remote_worktree_id.to_proto(),
                    path: path_string,
                })
                .await?;

            let this = this
                .upgrade(&cx)
                .ok_or_else(|| anyhow!("worktree was closed"))?;
            let file = File {
                entry_id: Some(entry.id),
                worktree: this.clone(),
                worktree_path: root_path,
                path: entry.path,
                mtime: entry.mtime,
                is_local: false,
            };
            let language = this.read_with(&cx, |this, _| {
                use language::File;
                this.languages().select_language(file.full_path()).cloned()
            });
            let remote_buffer = response.buffer.ok_or_else(|| anyhow!("empty buffer"))?;
            let buffer_id = remote_buffer.id as usize;
            let buffer = cx.add_model(|cx| {
                Buffer::from_proto(replica_id, remote_buffer, Some(Box::new(file)), cx)
                    .unwrap()
                    .with_language(language, None, cx)
            });
            this.update(&mut cx, move |this, cx| {
                let this = this.as_remote_mut().unwrap();
                if let Some(RemoteBuffer::Operations(pending_ops)) = this
                    .open_buffers
                    .insert(buffer_id, RemoteBuffer::Loaded(buffer.downgrade()))
                {
                    buffer.update(cx, |buf, cx| buf.apply_ops(pending_ops, cx))?;
                }
                Result::<_, anyhow::Error>::Ok(buffer)
            })
        })
    }

    pub fn close_all_buffers(&mut self, cx: &mut MutableAppContext) {
        for (_, buffer) in self.open_buffers.drain() {
            if let RemoteBuffer::Loaded(buffer) = buffer {
                if let Some(buffer) = buffer.upgrade(cx) {
                    buffer.update(cx, |buffer, cx| buffer.close(cx))
                }
            }
        }
    }

    fn snapshot(&self) -> Snapshot {
        self.snapshot.clone()
    }

    pub fn update_from_remote(
        &mut self,
        envelope: TypedEnvelope<proto::UpdateWorktree>,
        cx: &mut ModelContext<Worktree>,
    ) -> Result<()> {
        let mut tx = self.updates_tx.clone();
        let payload = envelope.payload.clone();
        cx.background()
            .spawn(async move {
                tx.send(payload).await.expect("receiver runs to completion");
            })
            .detach();

        Ok(())
    }

    pub fn remove_collaborator(&mut self, replica_id: ReplicaId, cx: &mut ModelContext<Worktree>) {
        for (_, buffer) in &self.open_buffers {
            if let Some(buffer) = buffer.upgrade(cx) {
                buffer.update(cx, |buffer, cx| buffer.remove_peer(replica_id, cx));
            }
        }
        cx.notify();
    }
}

enum RemoteBuffer {
    Operations(Vec<Operation>),
    Loaded(WeakModelHandle<Buffer>),
}

impl RemoteBuffer {
    fn upgrade(&self, cx: &impl UpgradeModelHandle) -> Option<ModelHandle<Buffer>> {
        match self {
            Self::Operations(_) => None,
            Self::Loaded(buffer) => buffer.upgrade(cx),
        }
    }
}

impl Snapshot {
    pub fn id(&self) -> WorktreeId {
        self.id
    }

    pub fn to_proto(&self) -> proto::Worktree {
        let root_name = self.root_name.clone();
        proto::Worktree {
            id: self.id.0 as u64,
            root_name,
            entries: self
                .entries_by_path
                .cursor::<()>()
                .filter(|e| !e.is_ignored)
                .map(Into::into)
                .collect(),
        }
    }

    pub fn build_update(
        &self,
        other: &Self,
        project_id: u64,
        worktree_id: u64,
        include_ignored: bool,
    ) -> proto::UpdateWorktree {
        let mut updated_entries = Vec::new();
        let mut removed_entries = Vec::new();
        let mut self_entries = self
            .entries_by_id
            .cursor::<()>()
            .filter(|e| include_ignored || !e.is_ignored)
            .peekable();
        let mut other_entries = other
            .entries_by_id
            .cursor::<()>()
            .filter(|e| include_ignored || !e.is_ignored)
            .peekable();
        loop {
            match (self_entries.peek(), other_entries.peek()) {
                (Some(self_entry), Some(other_entry)) => {
                    match Ord::cmp(&self_entry.id, &other_entry.id) {
                        Ordering::Less => {
                            let entry = self.entry_for_id(self_entry.id).unwrap().into();
                            updated_entries.push(entry);
                            self_entries.next();
                        }
                        Ordering::Equal => {
                            if self_entry.scan_id != other_entry.scan_id {
                                let entry = self.entry_for_id(self_entry.id).unwrap().into();
                                updated_entries.push(entry);
                            }

                            self_entries.next();
                            other_entries.next();
                        }
                        Ordering::Greater => {
                            removed_entries.push(other_entry.id as u64);
                            other_entries.next();
                        }
                    }
                }
                (Some(self_entry), None) => {
                    let entry = self.entry_for_id(self_entry.id).unwrap().into();
                    updated_entries.push(entry);
                    self_entries.next();
                }
                (None, Some(other_entry)) => {
                    removed_entries.push(other_entry.id as u64);
                    other_entries.next();
                }
                (None, None) => break,
            }
        }

        proto::UpdateWorktree {
            project_id,
            worktree_id,
            root_name: self.root_name().to_string(),
            updated_entries,
            removed_entries,
        }
    }

    fn apply_update(&mut self, update: proto::UpdateWorktree) -> Result<()> {
        self.scan_id += 1;
        let scan_id = self.scan_id;

        let mut entries_by_path_edits = Vec::new();
        let mut entries_by_id_edits = Vec::new();
        for entry_id in update.removed_entries {
            let entry_id = entry_id as usize;
            let entry = self
                .entry_for_id(entry_id)
                .ok_or_else(|| anyhow!("unknown entry"))?;
            entries_by_path_edits.push(Edit::Remove(PathKey(entry.path.clone())));
            entries_by_id_edits.push(Edit::Remove(entry.id));
        }

        for entry in update.updated_entries {
            let entry = Entry::try_from((&self.root_char_bag, entry))?;
            if let Some(PathEntry { path, .. }) = self.entries_by_id.get(&entry.id, &()) {
                entries_by_path_edits.push(Edit::Remove(PathKey(path.clone())));
            }
            entries_by_id_edits.push(Edit::Insert(PathEntry {
                id: entry.id,
                path: entry.path.clone(),
                is_ignored: entry.is_ignored,
                scan_id,
            }));
            entries_by_path_edits.push(Edit::Insert(entry));
        }

        self.entries_by_path.edit(entries_by_path_edits, &());
        self.entries_by_id.edit(entries_by_id_edits, &());

        Ok(())
    }

    pub fn file_count(&self) -> usize {
        self.entries_by_path.summary().file_count
    }

    pub fn visible_file_count(&self) -> usize {
        self.entries_by_path.summary().visible_file_count
    }

    fn traverse_from_offset(
        &self,
        include_dirs: bool,
        include_ignored: bool,
        start_offset: usize,
    ) -> Traversal {
        let mut cursor = self.entries_by_path.cursor();
        cursor.seek(
            &TraversalTarget::Count {
                count: start_offset,
                include_dirs,
                include_ignored,
            },
            Bias::Right,
            &(),
        );
        Traversal {
            cursor,
            include_dirs,
            include_ignored,
        }
    }

    fn traverse_from_path(
        &self,
        include_dirs: bool,
        include_ignored: bool,
        path: &Path,
    ) -> Traversal {
        let mut cursor = self.entries_by_path.cursor();
        cursor.seek(&TraversalTarget::Path(path), Bias::Left, &());
        Traversal {
            cursor,
            include_dirs,
            include_ignored,
        }
    }

    pub fn files(&self, include_ignored: bool, start: usize) -> Traversal {
        self.traverse_from_offset(false, include_ignored, start)
    }

    pub fn entries(&self, include_ignored: bool) -> Traversal {
        self.traverse_from_offset(true, include_ignored, 0)
    }

    pub fn paths(&self) -> impl Iterator<Item = &Arc<Path>> {
        let empty_path = Path::new("");
        self.entries_by_path
            .cursor::<()>()
            .filter(move |entry| entry.path.as_ref() != empty_path)
            .map(|entry| &entry.path)
    }

    fn child_entries<'a>(&'a self, parent_path: &'a Path) -> ChildEntriesIter<'a> {
        let mut cursor = self.entries_by_path.cursor();
        cursor.seek(&TraversalTarget::Path(parent_path), Bias::Right, &());
        let traversal = Traversal {
            cursor,
            include_dirs: true,
            include_ignored: true,
        };
        ChildEntriesIter {
            traversal,
            parent_path,
        }
    }

    pub fn root_entry(&self) -> Option<&Entry> {
        self.entry_for_path("")
    }

    pub fn root_name(&self) -> &str {
        &self.root_name
    }

    pub fn entry_for_path(&self, path: impl AsRef<Path>) -> Option<&Entry> {
        let path = path.as_ref();
        self.traverse_from_path(true, true, path)
            .entry()
            .and_then(|entry| {
                if entry.path.as_ref() == path {
                    Some(entry)
                } else {
                    None
                }
            })
    }

    pub fn entry_for_id(&self, id: usize) -> Option<&Entry> {
        let entry = self.entries_by_id.get(&id, &())?;
        self.entry_for_path(&entry.path)
    }

    pub fn inode_for_path(&self, path: impl AsRef<Path>) -> Option<u64> {
        self.entry_for_path(path.as_ref()).map(|e| e.inode)
    }

    fn insert_entry(&mut self, mut entry: Entry, fs: &dyn Fs) -> Entry {
        if !entry.is_dir() && entry.path.file_name() == Some(&GITIGNORE) {
            let abs_path = self.abs_path.join(&entry.path);
            match build_gitignore(&abs_path, fs) {
                Ok(ignore) => {
                    let ignore_dir_path = entry.path.parent().unwrap();
                    self.ignores
                        .insert(ignore_dir_path.into(), (Arc::new(ignore), self.scan_id));
                }
                Err(error) => {
                    log::error!(
                        "error loading .gitignore file {:?} - {:?}",
                        &entry.path,
                        error
                    );
                }
            }
        }

        self.reuse_entry_id(&mut entry);
        self.entries_by_path.insert_or_replace(entry.clone(), &());
        self.entries_by_id.insert_or_replace(
            PathEntry {
                id: entry.id,
                path: entry.path.clone(),
                is_ignored: entry.is_ignored,
                scan_id: self.scan_id,
            },
            &(),
        );
        entry
    }

    fn populate_dir(
        &mut self,
        parent_path: Arc<Path>,
        entries: impl IntoIterator<Item = Entry>,
        ignore: Option<Arc<Gitignore>>,
    ) {
        let mut parent_entry = self
            .entries_by_path
            .get(&PathKey(parent_path.clone()), &())
            .unwrap()
            .clone();
        if let Some(ignore) = ignore {
            self.ignores.insert(parent_path, (ignore, self.scan_id));
        }
        if matches!(parent_entry.kind, EntryKind::PendingDir) {
            parent_entry.kind = EntryKind::Dir;
        } else {
            unreachable!();
        }

        let mut entries_by_path_edits = vec![Edit::Insert(parent_entry)];
        let mut entries_by_id_edits = Vec::new();

        for mut entry in entries {
            self.reuse_entry_id(&mut entry);
            entries_by_id_edits.push(Edit::Insert(PathEntry {
                id: entry.id,
                path: entry.path.clone(),
                is_ignored: entry.is_ignored,
                scan_id: self.scan_id,
            }));
            entries_by_path_edits.push(Edit::Insert(entry));
        }

        self.entries_by_path.edit(entries_by_path_edits, &());
        self.entries_by_id.edit(entries_by_id_edits, &());
    }

    fn reuse_entry_id(&mut self, entry: &mut Entry) {
        if let Some(removed_entry_id) = self.removed_entry_ids.remove(&entry.inode) {
            entry.id = removed_entry_id;
        } else if let Some(existing_entry) = self.entry_for_path(&entry.path) {
            entry.id = existing_entry.id;
        }
    }

    fn remove_path(&mut self, path: &Path) {
        let mut new_entries;
        let removed_entries;
        {
            let mut cursor = self.entries_by_path.cursor::<TraversalProgress>();
            new_entries = cursor.slice(&TraversalTarget::Path(path), Bias::Left, &());
            removed_entries = cursor.slice(&TraversalTarget::PathSuccessor(path), Bias::Left, &());
            new_entries.push_tree(cursor.suffix(&()), &());
        }
        self.entries_by_path = new_entries;

        let mut entries_by_id_edits = Vec::new();
        for entry in removed_entries.cursor::<()>() {
            let removed_entry_id = self
                .removed_entry_ids
                .entry(entry.inode)
                .or_insert(entry.id);
            *removed_entry_id = cmp::max(*removed_entry_id, entry.id);
            entries_by_id_edits.push(Edit::Remove(entry.id));
        }
        self.entries_by_id.edit(entries_by_id_edits, &());

        if path.file_name() == Some(&GITIGNORE) {
            if let Some((_, scan_id)) = self.ignores.get_mut(path.parent().unwrap()) {
                *scan_id = self.scan_id;
            }
        }
    }

    fn ignore_stack_for_path(&self, path: &Path, is_dir: bool) -> Arc<IgnoreStack> {
        let mut new_ignores = Vec::new();
        for ancestor in path.ancestors().skip(1) {
            if let Some((ignore, _)) = self.ignores.get(ancestor) {
                new_ignores.push((ancestor, Some(ignore.clone())));
            } else {
                new_ignores.push((ancestor, None));
            }
        }

        let mut ignore_stack = IgnoreStack::none();
        for (parent_path, ignore) in new_ignores.into_iter().rev() {
            if ignore_stack.is_path_ignored(&parent_path, true) {
                ignore_stack = IgnoreStack::all();
                break;
            } else if let Some(ignore) = ignore {
                ignore_stack = ignore_stack.append(Arc::from(parent_path), ignore);
            }
        }

        if ignore_stack.is_path_ignored(path, is_dir) {
            ignore_stack = IgnoreStack::all();
        }

        ignore_stack
    }
}

impl fmt::Debug for Snapshot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for entry in self.entries_by_path.cursor::<()>() {
            for _ in entry.path.ancestors().skip(1) {
                write!(f, " ")?;
            }
            writeln!(f, "{:?} (inode: {})", entry.path, entry.inode)?;
        }
        Ok(())
    }
}

#[derive(Clone, PartialEq)]
pub struct File {
    entry_id: Option<usize>,
    worktree: ModelHandle<Worktree>,
    worktree_path: Arc<Path>,
    pub path: Arc<Path>,
    pub mtime: SystemTime,
    is_local: bool,
}

impl language::File for File {
    fn mtime(&self) -> SystemTime {
        self.mtime
    }

    fn path(&self) -> &Arc<Path> {
        &self.path
    }

    fn abs_path(&self) -> Option<PathBuf> {
        if self.is_local {
            Some(self.worktree_path.join(&self.path))
        } else {
            None
        }
    }

    fn full_path(&self) -> PathBuf {
        let mut full_path = PathBuf::new();
        if let Some(worktree_name) = self.worktree_path.file_name() {
            full_path.push(worktree_name);
        }
        full_path.push(&self.path);
        full_path
    }

    /// Returns the last component of this handle's absolute path. If this handle refers to the root
    /// of its worktree, then this method will return the name of the worktree itself.
    fn file_name<'a>(&'a self) -> Option<OsString> {
        self.path
            .file_name()
            .or_else(|| self.worktree_path.file_name())
            .map(Into::into)
    }

    fn is_deleted(&self) -> bool {
        self.entry_id.is_none()
    }

    fn save(
        &self,
        buffer_id: u64,
        text: Rope,
        version: clock::Global,
        cx: &mut MutableAppContext,
    ) -> Task<Result<(clock::Global, SystemTime)>> {
        let worktree_id = self.worktree.read(cx).id().to_proto();
        self.worktree.update(cx, |worktree, cx| match worktree {
            Worktree::Local(worktree) => {
                let rpc = worktree.client.clone();
                let project_id = worktree.share.as_ref().map(|share| share.project_id);
                let save = worktree.save(self.path.clone(), text, cx);
                cx.background().spawn(async move {
                    let entry = save.await?;
                    if let Some(project_id) = project_id {
                        rpc.send(proto::BufferSaved {
                            project_id,
                            worktree_id,
                            buffer_id,
                            version: (&version).into(),
                            mtime: Some(entry.mtime.into()),
                        })
                        .await?;
                    }
                    Ok((version, entry.mtime))
                })
            }
            Worktree::Remote(worktree) => {
                let rpc = worktree.client.clone();
                let project_id = worktree.project_id;
                cx.foreground().spawn(async move {
                    let response = rpc
                        .request(proto::SaveBuffer {
                            project_id,
                            worktree_id,
                            buffer_id,
                        })
                        .await?;
                    let version = response.version.try_into()?;
                    let mtime = response
                        .mtime
                        .ok_or_else(|| anyhow!("missing mtime"))?
                        .into();
                    Ok((version, mtime))
                })
            }
        })
    }

    fn load_local(&self, cx: &AppContext) -> Option<Task<Result<String>>> {
        let worktree = self.worktree.read(cx).as_local()?;
        let abs_path = worktree.absolutize(&self.path);
        let fs = worktree.fs.clone();
        Some(
            cx.background()
                .spawn(async move { fs.load(&abs_path).await }),
        )
    }

    fn buffer_updated(&self, buffer_id: u64, operation: Operation, cx: &mut MutableAppContext) {
        self.worktree.update(cx, |worktree, cx| {
            worktree.send_buffer_update(buffer_id, operation, cx);
        });
    }

    fn buffer_removed(&self, buffer_id: u64, cx: &mut MutableAppContext) {
        self.worktree.update(cx, |worktree, cx| {
            if let Worktree::Remote(worktree) = worktree {
                let project_id = worktree.project_id;
                let worktree_id = worktree.id().to_proto();
                let rpc = worktree.client.clone();
                cx.background()
                    .spawn(async move {
                        if let Err(error) = rpc
                            .send(proto::CloseBuffer {
                                project_id,
                                worktree_id,
                                buffer_id,
                            })
                            .await
                        {
                            log::error!("error closing remote buffer: {}", error);
                        }
                    })
                    .detach();
            }
        });
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl File {
    pub fn from_dyn(file: Option<&dyn language::File>) -> Option<&Self> {
        file.and_then(|f| f.as_any().downcast_ref())
    }

    pub fn worktree_id(&self, cx: &AppContext) -> WorktreeId {
        self.worktree.read(cx).id()
    }
}

#[derive(Clone, Debug)]
pub struct Entry {
    pub id: usize,
    pub kind: EntryKind,
    pub path: Arc<Path>,
    pub inode: u64,
    pub mtime: SystemTime,
    pub is_symlink: bool,
    pub is_ignored: bool,
}

#[derive(Clone, Debug)]
pub enum EntryKind {
    PendingDir,
    Dir,
    File(CharBag),
}

impl Entry {
    fn new(
        path: Arc<Path>,
        metadata: &fs::Metadata,
        next_entry_id: &AtomicUsize,
        root_char_bag: CharBag,
    ) -> Self {
        Self {
            id: next_entry_id.fetch_add(1, SeqCst),
            kind: if metadata.is_dir {
                EntryKind::PendingDir
            } else {
                EntryKind::File(char_bag_for_path(root_char_bag, &path))
            },
            path,
            inode: metadata.inode,
            mtime: metadata.mtime,
            is_symlink: metadata.is_symlink,
            is_ignored: false,
        }
    }

    pub fn is_dir(&self) -> bool {
        matches!(self.kind, EntryKind::Dir | EntryKind::PendingDir)
    }

    pub fn is_file(&self) -> bool {
        matches!(self.kind, EntryKind::File(_))
    }
}

impl sum_tree::Item for Entry {
    type Summary = EntrySummary;

    fn summary(&self) -> Self::Summary {
        let visible_count = if self.is_ignored { 0 } else { 1 };
        let file_count;
        let visible_file_count;
        if self.is_file() {
            file_count = 1;
            visible_file_count = visible_count;
        } else {
            file_count = 0;
            visible_file_count = 0;
        }

        EntrySummary {
            max_path: self.path.clone(),
            count: 1,
            visible_count,
            file_count,
            visible_file_count,
        }
    }
}

impl sum_tree::KeyedItem for Entry {
    type Key = PathKey;

    fn key(&self) -> Self::Key {
        PathKey(self.path.clone())
    }
}

#[derive(Clone, Debug)]
pub struct EntrySummary {
    max_path: Arc<Path>,
    count: usize,
    visible_count: usize,
    file_count: usize,
    visible_file_count: usize,
}

impl Default for EntrySummary {
    fn default() -> Self {
        Self {
            max_path: Arc::from(Path::new("")),
            count: 0,
            visible_count: 0,
            file_count: 0,
            visible_file_count: 0,
        }
    }
}

impl sum_tree::Summary for EntrySummary {
    type Context = ();

    fn add_summary(&mut self, rhs: &Self, _: &()) {
        self.max_path = rhs.max_path.clone();
        self.visible_count += rhs.visible_count;
        self.file_count += rhs.file_count;
        self.visible_file_count += rhs.visible_file_count;
    }
}

#[derive(Clone, Debug)]
struct PathEntry {
    id: usize,
    path: Arc<Path>,
    is_ignored: bool,
    scan_id: usize,
}

impl sum_tree::Item for PathEntry {
    type Summary = PathEntrySummary;

    fn summary(&self) -> Self::Summary {
        PathEntrySummary { max_id: self.id }
    }
}

impl sum_tree::KeyedItem for PathEntry {
    type Key = usize;

    fn key(&self) -> Self::Key {
        self.id
    }
}

#[derive(Clone, Debug, Default)]
struct PathEntrySummary {
    max_id: usize,
}

impl sum_tree::Summary for PathEntrySummary {
    type Context = ();

    fn add_summary(&mut self, summary: &Self, _: &Self::Context) {
        self.max_id = summary.max_id;
    }
}

impl<'a> sum_tree::Dimension<'a, PathEntrySummary> for usize {
    fn add_summary(&mut self, summary: &'a PathEntrySummary, _: &()) {
        *self = summary.max_id;
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct PathKey(Arc<Path>);

impl Default for PathKey {
    fn default() -> Self {
        Self(Path::new("").into())
    }
}

impl<'a> sum_tree::Dimension<'a, EntrySummary> for PathKey {
    fn add_summary(&mut self, summary: &'a EntrySummary, _: &()) {
        self.0 = summary.max_path.clone();
    }
}

struct BackgroundScanner {
    fs: Arc<dyn Fs>,
    snapshot: Arc<Mutex<Snapshot>>,
    notify: Sender<ScanState>,
    executor: Arc<executor::Background>,
}

impl BackgroundScanner {
    fn new(
        snapshot: Arc<Mutex<Snapshot>>,
        notify: Sender<ScanState>,
        fs: Arc<dyn Fs>,
        executor: Arc<executor::Background>,
    ) -> Self {
        Self {
            fs,
            snapshot,
            notify,
            executor,
        }
    }

    fn abs_path(&self) -> Arc<Path> {
        self.snapshot.lock().abs_path.clone()
    }

    fn snapshot(&self) -> Snapshot {
        self.snapshot.lock().clone()
    }

    async fn run(mut self, events_rx: impl Stream<Item = Vec<fsevent::Event>>) {
        if self.notify.send(ScanState::Scanning).await.is_err() {
            return;
        }

        if let Err(err) = self.scan_dirs().await {
            if self
                .notify
                .send(ScanState::Err(Arc::new(err)))
                .await
                .is_err()
            {
                return;
            }
        }

        if self.notify.send(ScanState::Idle).await.is_err() {
            return;
        }

        futures::pin_mut!(events_rx);
        while let Some(events) = events_rx.next().await {
            if self.notify.send(ScanState::Scanning).await.is_err() {
                break;
            }

            if !self.process_events(events).await {
                break;
            }

            if self.notify.send(ScanState::Idle).await.is_err() {
                break;
            }
        }
    }

    async fn scan_dirs(&mut self) -> Result<()> {
        let root_char_bag;
        let next_entry_id;
        let is_dir;
        {
            let snapshot = self.snapshot.lock();
            root_char_bag = snapshot.root_char_bag;
            next_entry_id = snapshot.next_entry_id.clone();
            is_dir = snapshot.root_entry().map_or(false, |e| e.is_dir())
        };

        if is_dir {
            let path: Arc<Path> = Arc::from(Path::new(""));
            let abs_path = self.abs_path();
            let (tx, rx) = channel::unbounded();
            tx.send(ScanJob {
                abs_path: abs_path.to_path_buf(),
                path,
                ignore_stack: IgnoreStack::none(),
                scan_queue: tx.clone(),
            })
            .await
            .unwrap();
            drop(tx);

            self.executor
                .scoped(|scope| {
                    for _ in 0..self.executor.num_cpus() {
                        scope.spawn(async {
                            while let Ok(job) = rx.recv().await {
                                if let Err(err) = self
                                    .scan_dir(root_char_bag, next_entry_id.clone(), &job)
                                    .await
                                {
                                    log::error!("error scanning {:?}: {}", job.abs_path, err);
                                }
                            }
                        });
                    }
                })
                .await;
        }

        Ok(())
    }

    async fn scan_dir(
        &self,
        root_char_bag: CharBag,
        next_entry_id: Arc<AtomicUsize>,
        job: &ScanJob,
    ) -> Result<()> {
        let mut new_entries: Vec<Entry> = Vec::new();
        let mut new_jobs: Vec<ScanJob> = Vec::new();
        let mut ignore_stack = job.ignore_stack.clone();
        let mut new_ignore = None;

        let mut child_paths = self.fs.read_dir(&job.abs_path).await?;
        while let Some(child_abs_path) = child_paths.next().await {
            let child_abs_path = match child_abs_path {
                Ok(child_abs_path) => child_abs_path,
                Err(error) => {
                    log::error!("error processing entry {:?}", error);
                    continue;
                }
            };
            let child_name = child_abs_path.file_name().unwrap();
            let child_path: Arc<Path> = job.path.join(child_name).into();
            let child_metadata = match self.fs.metadata(&child_abs_path).await? {
                Some(metadata) => metadata,
                None => continue,
            };

            // If we find a .gitignore, add it to the stack of ignores used to determine which paths are ignored
            if child_name == *GITIGNORE {
                match build_gitignore(&child_abs_path, self.fs.as_ref()) {
                    Ok(ignore) => {
                        let ignore = Arc::new(ignore);
                        ignore_stack = ignore_stack.append(job.path.clone(), ignore.clone());
                        new_ignore = Some(ignore);
                    }
                    Err(error) => {
                        log::error!(
                            "error loading .gitignore file {:?} - {:?}",
                            child_name,
                            error
                        );
                    }
                }

                // Update ignore status of any child entries we've already processed to reflect the
                // ignore file in the current directory. Because `.gitignore` starts with a `.`,
                // there should rarely be too numerous. Update the ignore stack associated with any
                // new jobs as well.
                let mut new_jobs = new_jobs.iter_mut();
                for entry in &mut new_entries {
                    entry.is_ignored = ignore_stack.is_path_ignored(&entry.path, entry.is_dir());
                    if entry.is_dir() {
                        new_jobs.next().unwrap().ignore_stack = if entry.is_ignored {
                            IgnoreStack::all()
                        } else {
                            ignore_stack.clone()
                        };
                    }
                }
            }

            let mut child_entry = Entry::new(
                child_path.clone(),
                &child_metadata,
                &next_entry_id,
                root_char_bag,
            );

            if child_metadata.is_dir {
                let is_ignored = ignore_stack.is_path_ignored(&child_path, true);
                child_entry.is_ignored = is_ignored;
                new_entries.push(child_entry);
                new_jobs.push(ScanJob {
                    abs_path: child_abs_path,
                    path: child_path,
                    ignore_stack: if is_ignored {
                        IgnoreStack::all()
                    } else {
                        ignore_stack.clone()
                    },
                    scan_queue: job.scan_queue.clone(),
                });
            } else {
                child_entry.is_ignored = ignore_stack.is_path_ignored(&child_path, false);
                new_entries.push(child_entry);
            };
        }

        self.snapshot
            .lock()
            .populate_dir(job.path.clone(), new_entries, new_ignore);
        for new_job in new_jobs {
            job.scan_queue.send(new_job).await.unwrap();
        }

        Ok(())
    }

    async fn process_events(&mut self, mut events: Vec<fsevent::Event>) -> bool {
        let mut snapshot = self.snapshot();
        snapshot.scan_id += 1;

        let root_abs_path = if let Ok(abs_path) = self.fs.canonicalize(&snapshot.abs_path).await {
            abs_path
        } else {
            return false;
        };
        let root_char_bag = snapshot.root_char_bag;
        let next_entry_id = snapshot.next_entry_id.clone();

        events.sort_unstable_by(|a, b| a.path.cmp(&b.path));
        events.dedup_by(|a, b| a.path.starts_with(&b.path));

        for event in &events {
            match event.path.strip_prefix(&root_abs_path) {
                Ok(path) => snapshot.remove_path(&path),
                Err(_) => {
                    log::error!(
                        "unexpected event {:?} for root path {:?}",
                        event.path,
                        root_abs_path
                    );
                    continue;
                }
            }
        }

        let (scan_queue_tx, scan_queue_rx) = channel::unbounded();
        for event in events {
            let path: Arc<Path> = match event.path.strip_prefix(&root_abs_path) {
                Ok(path) => Arc::from(path.to_path_buf()),
                Err(_) => {
                    log::error!(
                        "unexpected event {:?} for root path {:?}",
                        event.path,
                        root_abs_path
                    );
                    continue;
                }
            };

            match self.fs.metadata(&event.path).await {
                Ok(Some(metadata)) => {
                    let ignore_stack = snapshot.ignore_stack_for_path(&path, metadata.is_dir);
                    let mut fs_entry = Entry::new(
                        path.clone(),
                        &metadata,
                        snapshot.next_entry_id.as_ref(),
                        snapshot.root_char_bag,
                    );
                    fs_entry.is_ignored = ignore_stack.is_all();
                    snapshot.insert_entry(fs_entry, self.fs.as_ref());
                    if metadata.is_dir {
                        scan_queue_tx
                            .send(ScanJob {
                                abs_path: event.path,
                                path,
                                ignore_stack,
                                scan_queue: scan_queue_tx.clone(),
                            })
                            .await
                            .unwrap();
                    }
                }
                Ok(None) => {}
                Err(err) => {
                    // TODO - create a special 'error' entry in the entries tree to mark this
                    log::error!("error reading file on event {:?}", err);
                }
            }
        }

        *self.snapshot.lock() = snapshot;

        // Scan any directories that were created as part of this event batch.
        drop(scan_queue_tx);
        self.executor
            .scoped(|scope| {
                for _ in 0..self.executor.num_cpus() {
                    scope.spawn(async {
                        while let Ok(job) = scan_queue_rx.recv().await {
                            if let Err(err) = self
                                .scan_dir(root_char_bag, next_entry_id.clone(), &job)
                                .await
                            {
                                log::error!("error scanning {:?}: {}", job.abs_path, err);
                            }
                        }
                    });
                }
            })
            .await;

        // Attempt to detect renames only over a single batch of file-system events.
        self.snapshot.lock().removed_entry_ids.clear();

        self.update_ignore_statuses().await;
        true
    }

    async fn update_ignore_statuses(&self) {
        let mut snapshot = self.snapshot();

        let mut ignores_to_update = Vec::new();
        let mut ignores_to_delete = Vec::new();
        for (parent_path, (_, scan_id)) in &snapshot.ignores {
            if *scan_id == snapshot.scan_id && snapshot.entry_for_path(parent_path).is_some() {
                ignores_to_update.push(parent_path.clone());
            }

            let ignore_path = parent_path.join(&*GITIGNORE);
            if snapshot.entry_for_path(ignore_path).is_none() {
                ignores_to_delete.push(parent_path.clone());
            }
        }

        for parent_path in ignores_to_delete {
            snapshot.ignores.remove(&parent_path);
            self.snapshot.lock().ignores.remove(&parent_path);
        }

        let (ignore_queue_tx, ignore_queue_rx) = channel::unbounded();
        ignores_to_update.sort_unstable();
        let mut ignores_to_update = ignores_to_update.into_iter().peekable();
        while let Some(parent_path) = ignores_to_update.next() {
            while ignores_to_update
                .peek()
                .map_or(false, |p| p.starts_with(&parent_path))
            {
                ignores_to_update.next().unwrap();
            }

            let ignore_stack = snapshot.ignore_stack_for_path(&parent_path, true);
            ignore_queue_tx
                .send(UpdateIgnoreStatusJob {
                    path: parent_path,
                    ignore_stack,
                    ignore_queue: ignore_queue_tx.clone(),
                })
                .await
                .unwrap();
        }
        drop(ignore_queue_tx);

        self.executor
            .scoped(|scope| {
                for _ in 0..self.executor.num_cpus() {
                    scope.spawn(async {
                        while let Ok(job) = ignore_queue_rx.recv().await {
                            self.update_ignore_status(job, &snapshot).await;
                        }
                    });
                }
            })
            .await;
    }

    async fn update_ignore_status(&self, job: UpdateIgnoreStatusJob, snapshot: &Snapshot) {
        let mut ignore_stack = job.ignore_stack;
        if let Some((ignore, _)) = snapshot.ignores.get(&job.path) {
            ignore_stack = ignore_stack.append(job.path.clone(), ignore.clone());
        }

        let mut entries_by_id_edits = Vec::new();
        let mut entries_by_path_edits = Vec::new();
        for mut entry in snapshot.child_entries(&job.path).cloned() {
            let was_ignored = entry.is_ignored;
            entry.is_ignored = ignore_stack.is_path_ignored(&entry.path, entry.is_dir());
            if entry.is_dir() {
                let child_ignore_stack = if entry.is_ignored {
                    IgnoreStack::all()
                } else {
                    ignore_stack.clone()
                };
                job.ignore_queue
                    .send(UpdateIgnoreStatusJob {
                        path: entry.path.clone(),
                        ignore_stack: child_ignore_stack,
                        ignore_queue: job.ignore_queue.clone(),
                    })
                    .await
                    .unwrap();
            }

            if entry.is_ignored != was_ignored {
                let mut path_entry = snapshot.entries_by_id.get(&entry.id, &()).unwrap().clone();
                path_entry.scan_id = snapshot.scan_id;
                path_entry.is_ignored = entry.is_ignored;
                entries_by_id_edits.push(Edit::Insert(path_entry));
                entries_by_path_edits.push(Edit::Insert(entry));
            }
        }

        let mut snapshot = self.snapshot.lock();
        snapshot.entries_by_path.edit(entries_by_path_edits, &());
        snapshot.entries_by_id.edit(entries_by_id_edits, &());
    }
}

async fn refresh_entry(
    fs: &dyn Fs,
    snapshot: &Mutex<Snapshot>,
    path: Arc<Path>,
    abs_path: &Path,
) -> Result<Entry> {
    let root_char_bag;
    let next_entry_id;
    {
        let snapshot = snapshot.lock();
        root_char_bag = snapshot.root_char_bag;
        next_entry_id = snapshot.next_entry_id.clone();
    }
    let entry = Entry::new(
        path,
        &fs.metadata(abs_path)
            .await?
            .ok_or_else(|| anyhow!("could not read saved file metadata"))?,
        &next_entry_id,
        root_char_bag,
    );
    Ok(snapshot.lock().insert_entry(entry, fs))
}

fn char_bag_for_path(root_char_bag: CharBag, path: &Path) -> CharBag {
    let mut result = root_char_bag;
    result.extend(
        path.to_string_lossy()
            .chars()
            .map(|c| c.to_ascii_lowercase()),
    );
    result
}

struct ScanJob {
    abs_path: PathBuf,
    path: Arc<Path>,
    ignore_stack: Arc<IgnoreStack>,
    scan_queue: Sender<ScanJob>,
}

struct UpdateIgnoreStatusJob {
    path: Arc<Path>,
    ignore_stack: Arc<IgnoreStack>,
    ignore_queue: Sender<UpdateIgnoreStatusJob>,
}

pub trait WorktreeHandle {
    #[cfg(test)]
    fn flush_fs_events<'a>(
        &self,
        cx: &'a gpui::TestAppContext,
    ) -> futures::future::LocalBoxFuture<'a, ()>;
}

impl WorktreeHandle for ModelHandle<Worktree> {
    // When the worktree's FS event stream sometimes delivers "redundant" events for FS changes that
    // occurred before the worktree was constructed. These events can cause the worktree to perfrom
    // extra directory scans, and emit extra scan-state notifications.
    //
    // This function mutates the worktree's directory and waits for those mutations to be picked up,
    // to ensure that all redundant FS events have already been processed.
    #[cfg(test)]
    fn flush_fs_events<'a>(
        &self,
        cx: &'a gpui::TestAppContext,
    ) -> futures::future::LocalBoxFuture<'a, ()> {
        use smol::future::FutureExt;

        let filename = "fs-event-sentinel";
        let root_path = cx.read(|cx| self.read(cx).abs_path.clone());
        let tree = self.clone();
        async move {
            std::fs::write(root_path.join(filename), "").unwrap();
            tree.condition(&cx, |tree, _| tree.entry_for_path(filename).is_some())
                .await;

            std::fs::remove_file(root_path.join(filename)).unwrap();
            tree.condition(&cx, |tree, _| tree.entry_for_path(filename).is_none())
                .await;

            cx.read(|cx| tree.read(cx).as_local().unwrap().scan_complete())
                .await;
        }
        .boxed_local()
    }
}

#[derive(Clone, Debug)]
struct TraversalProgress<'a> {
    max_path: &'a Path,
    count: usize,
    visible_count: usize,
    file_count: usize,
    visible_file_count: usize,
}

impl<'a> TraversalProgress<'a> {
    fn count(&self, include_dirs: bool, include_ignored: bool) -> usize {
        match (include_ignored, include_dirs) {
            (true, true) => self.count,
            (true, false) => self.file_count,
            (false, true) => self.visible_count,
            (false, false) => self.visible_file_count,
        }
    }
}

impl<'a> sum_tree::Dimension<'a, EntrySummary> for TraversalProgress<'a> {
    fn add_summary(&mut self, summary: &'a EntrySummary, _: &()) {
        self.max_path = summary.max_path.as_ref();
        self.count += summary.count;
        self.visible_count += summary.visible_count;
        self.file_count += summary.file_count;
        self.visible_file_count += summary.visible_file_count;
    }
}

impl<'a> Default for TraversalProgress<'a> {
    fn default() -> Self {
        Self {
            max_path: Path::new(""),
            count: 0,
            visible_count: 0,
            file_count: 0,
            visible_file_count: 0,
        }
    }
}

pub struct Traversal<'a> {
    cursor: sum_tree::Cursor<'a, Entry, TraversalProgress<'a>>,
    include_ignored: bool,
    include_dirs: bool,
}

impl<'a> Traversal<'a> {
    pub fn advance(&mut self) -> bool {
        self.advance_to_offset(self.offset() + 1)
    }

    pub fn advance_to_offset(&mut self, offset: usize) -> bool {
        self.cursor.seek_forward(
            &TraversalTarget::Count {
                count: offset,
                include_dirs: self.include_dirs,
                include_ignored: self.include_ignored,
            },
            Bias::Right,
            &(),
        )
    }

    pub fn advance_to_sibling(&mut self) -> bool {
        while let Some(entry) = self.cursor.item() {
            self.cursor.seek_forward(
                &TraversalTarget::PathSuccessor(&entry.path),
                Bias::Left,
                &(),
            );
            if let Some(entry) = self.cursor.item() {
                if (self.include_dirs || !entry.is_dir())
                    && (self.include_ignored || !entry.is_ignored)
                {
                    return true;
                }
            }
        }
        false
    }

    pub fn entry(&self) -> Option<&'a Entry> {
        self.cursor.item()
    }

    pub fn offset(&self) -> usize {
        self.cursor
            .start()
            .count(self.include_dirs, self.include_ignored)
    }
}

impl<'a> Iterator for Traversal<'a> {
    type Item = &'a Entry;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.entry() {
            self.advance();
            Some(item)
        } else {
            None
        }
    }
}

#[derive(Debug)]
enum TraversalTarget<'a> {
    Path(&'a Path),
    PathSuccessor(&'a Path),
    Count {
        count: usize,
        include_ignored: bool,
        include_dirs: bool,
    },
}

impl<'a, 'b> SeekTarget<'a, EntrySummary, TraversalProgress<'a>> for TraversalTarget<'b> {
    fn cmp(&self, cursor_location: &TraversalProgress<'a>, _: &()) -> Ordering {
        match self {
            TraversalTarget::Path(path) => path.cmp(&cursor_location.max_path),
            TraversalTarget::PathSuccessor(path) => {
                if !cursor_location.max_path.starts_with(path) {
                    Ordering::Equal
                } else {
                    Ordering::Greater
                }
            }
            TraversalTarget::Count {
                count,
                include_dirs,
                include_ignored,
            } => Ord::cmp(
                count,
                &cursor_location.count(*include_dirs, *include_ignored),
            ),
        }
    }
}

struct ChildEntriesIter<'a> {
    parent_path: &'a Path,
    traversal: Traversal<'a>,
}

impl<'a> Iterator for ChildEntriesIter<'a> {
    type Item = &'a Entry;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.traversal.entry() {
            if item.path.starts_with(&self.parent_path) {
                self.traversal.advance_to_sibling();
                return Some(item);
            }
        }
        None
    }
}

impl<'a> From<&'a Entry> for proto::Entry {
    fn from(entry: &'a Entry) -> Self {
        Self {
            id: entry.id as u64,
            is_dir: entry.is_dir(),
            path: entry.path.to_string_lossy().to_string(),
            inode: entry.inode,
            mtime: Some(entry.mtime.into()),
            is_symlink: entry.is_symlink,
            is_ignored: entry.is_ignored,
        }
    }
}

impl<'a> TryFrom<(&'a CharBag, proto::Entry)> for Entry {
    type Error = anyhow::Error;

    fn try_from((root_char_bag, entry): (&'a CharBag, proto::Entry)) -> Result<Self> {
        if let Some(mtime) = entry.mtime {
            let kind = if entry.is_dir {
                EntryKind::Dir
            } else {
                let mut char_bag = root_char_bag.clone();
                char_bag.extend(entry.path.chars().map(|c| c.to_ascii_lowercase()));
                EntryKind::File(char_bag)
            };
            let path: Arc<Path> = Arc::from(Path::new(&entry.path));
            Ok(Entry {
                id: entry.id as usize,
                kind,
                path: path.clone(),
                inode: entry.inode,
                mtime: mtime.into(),
                is_symlink: entry.is_symlink,
                is_ignored: entry.is_ignored,
            })
        } else {
            Err(anyhow!(
                "missing mtime in remote worktree entry {:?}",
                entry.path
            ))
        }
    }
}

trait ToPointUtf16 {
    fn to_point_utf16(self) -> PointUtf16;
}

impl ToPointUtf16 for lsp::Position {
    fn to_point_utf16(self) -> PointUtf16 {
        PointUtf16::new(self.line, self.character)
    }
}

fn diagnostic_ranges<'a>(
    diagnostic: &'a lsp::Diagnostic,
    abs_path: &'a Path,
) -> impl 'a + Iterator<Item = Range<PointUtf16>> {
    diagnostic
        .related_information
        .iter()
        .flatten()
        .filter_map(move |info| {
            if info.location.uri.to_file_path().ok()? == abs_path {
                let info_start = PointUtf16::new(
                    info.location.range.start.line,
                    info.location.range.start.character,
                );
                let info_end = PointUtf16::new(
                    info.location.range.end.line,
                    info.location.range.end.character,
                );
                Some(info_start..info_end)
            } else {
                None
            }
        })
        .chain(Some(
            diagnostic.range.start.to_point_utf16()..diagnostic.range.end.to_point_utf16(),
        ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fs::FakeFs;
    use anyhow::Result;
    use client::test::{FakeHttpClient, FakeServer};
    use fs::RealFs;
    use language::{tree_sitter_rust, DiagnosticEntry, LanguageServerConfig};
    use language::{Diagnostic, LanguageConfig};
    use lsp::Url;
    use rand::prelude::*;
    use serde_json::json;
    use std::{cell::RefCell, rc::Rc};
    use std::{
        env,
        fmt::Write,
        time::{SystemTime, UNIX_EPOCH},
    };
    use text::Point;
    use unindent::Unindent as _;
    use util::test::temp_tree;

    #[gpui::test]
    async fn test_traversal(mut cx: gpui::TestAppContext) {
        let fs = FakeFs::new();
        fs.insert_tree(
            "/root",
            json!({
               ".gitignore": "a/b\n",
               "a": {
                   "b": "",
                   "c": "",
               }
            }),
        )
        .await;

        let http_client = FakeHttpClient::with_404_response();
        let client = Client::new(http_client.clone());
        let user_store = cx.add_model(|cx| UserStore::new(client.clone(), http_client, cx));

        let tree = Worktree::open_local(
            client,
            user_store,
            Arc::from(Path::new("/root")),
            Arc::new(fs),
            Default::default(),
            &mut cx.to_async(),
        )
        .await
        .unwrap();
        cx.read(|cx| tree.read(cx).as_local().unwrap().scan_complete())
            .await;

        tree.read_with(&cx, |tree, _| {
            assert_eq!(
                tree.entries(false)
                    .map(|entry| entry.path.as_ref())
                    .collect::<Vec<_>>(),
                vec![
                    Path::new(""),
                    Path::new(".gitignore"),
                    Path::new("a"),
                    Path::new("a/c"),
                ]
            );
        })
    }

    #[gpui::test]
    async fn test_save_file(mut cx: gpui::TestAppContext) {
        let dir = temp_tree(json!({
            "file1": "the old contents",
        }));

        let http_client = FakeHttpClient::with_404_response();
        let client = Client::new(http_client.clone());
        let user_store = cx.add_model(|cx| UserStore::new(client.clone(), http_client, cx));

        let tree = Worktree::open_local(
            client,
            user_store,
            dir.path(),
            Arc::new(RealFs),
            Default::default(),
            &mut cx.to_async(),
        )
        .await
        .unwrap();
        let buffer = tree
            .update(&mut cx, |tree, cx| tree.open_buffer("file1", cx))
            .await
            .unwrap();
        let save = buffer.update(&mut cx, |buffer, cx| {
            buffer.edit(Some(0..0), "a line of text.\n".repeat(10 * 1024), cx);
            buffer.save(cx).unwrap()
        });
        save.await.unwrap();

        let new_text = std::fs::read_to_string(dir.path().join("file1")).unwrap();
        assert_eq!(new_text, buffer.read_with(&cx, |buffer, _| buffer.text()));
    }

    #[gpui::test]
    async fn test_save_in_single_file_worktree(mut cx: gpui::TestAppContext) {
        let dir = temp_tree(json!({
            "file1": "the old contents",
        }));
        let file_path = dir.path().join("file1");

        let http_client = FakeHttpClient::with_404_response();
        let client = Client::new(http_client.clone());
        let user_store = cx.add_model(|cx| UserStore::new(client.clone(), http_client, cx));

        let tree = Worktree::open_local(
            client,
            user_store,
            file_path.clone(),
            Arc::new(RealFs),
            Default::default(),
            &mut cx.to_async(),
        )
        .await
        .unwrap();
        cx.read(|cx| tree.read(cx).as_local().unwrap().scan_complete())
            .await;
        cx.read(|cx| assert_eq!(tree.read(cx).file_count(), 1));

        let buffer = tree
            .update(&mut cx, |tree, cx| tree.open_buffer("", cx))
            .await
            .unwrap();
        let save = buffer.update(&mut cx, |buffer, cx| {
            buffer.edit(Some(0..0), "a line of text.\n".repeat(10 * 1024), cx);
            buffer.save(cx).unwrap()
        });
        save.await.unwrap();

        let new_text = std::fs::read_to_string(file_path).unwrap();
        assert_eq!(new_text, buffer.read_with(&cx, |buffer, _| buffer.text()));
    }

    #[gpui::test]
    async fn test_rescan_and_remote_updates(mut cx: gpui::TestAppContext) {
        let dir = temp_tree(json!({
            "a": {
                "file1": "",
                "file2": "",
                "file3": "",
            },
            "b": {
                "c": {
                    "file4": "",
                    "file5": "",
                }
            }
        }));

        let user_id = 5;
        let http_client = FakeHttpClient::with_404_response();
        let mut client = Client::new(http_client.clone());
        let server = FakeServer::for_client(user_id, &mut client, &cx).await;
        let user_store = server.build_user_store(client.clone(), &mut cx).await;
        let tree = Worktree::open_local(
            client,
            user_store.clone(),
            dir.path(),
            Arc::new(RealFs),
            Default::default(),
            &mut cx.to_async(),
        )
        .await
        .unwrap();

        let buffer_for_path = |path: &'static str, cx: &mut gpui::TestAppContext| {
            let buffer = tree.update(cx, |tree, cx| tree.open_buffer(path, cx));
            async move { buffer.await.unwrap() }
        };
        let id_for_path = |path: &'static str, cx: &gpui::TestAppContext| {
            tree.read_with(cx, |tree, _| {
                tree.entry_for_path(path)
                    .expect(&format!("no entry for path {}", path))
                    .id
            })
        };

        let buffer2 = buffer_for_path("a/file2", &mut cx).await;
        let buffer3 = buffer_for_path("a/file3", &mut cx).await;
        let buffer4 = buffer_for_path("b/c/file4", &mut cx).await;
        let buffer5 = buffer_for_path("b/c/file5", &mut cx).await;

        let file2_id = id_for_path("a/file2", &cx);
        let file3_id = id_for_path("a/file3", &cx);
        let file4_id = id_for_path("b/c/file4", &cx);

        // Wait for the initial scan.
        cx.read(|cx| tree.read(cx).as_local().unwrap().scan_complete())
            .await;

        // Create a remote copy of this worktree.
        let initial_snapshot = tree.read_with(&cx, |tree, _| tree.snapshot());
        let remote = Worktree::remote(
            1,
            1,
            initial_snapshot.to_proto(),
            Client::new(http_client.clone()),
            user_store,
            Default::default(),
            &mut cx.to_async(),
        )
        .await
        .unwrap();

        cx.read(|cx| {
            assert!(!buffer2.read(cx).is_dirty());
            assert!(!buffer3.read(cx).is_dirty());
            assert!(!buffer4.read(cx).is_dirty());
            assert!(!buffer5.read(cx).is_dirty());
        });

        // Rename and delete files and directories.
        tree.flush_fs_events(&cx).await;
        std::fs::rename(dir.path().join("a/file3"), dir.path().join("b/c/file3")).unwrap();
        std::fs::remove_file(dir.path().join("b/c/file5")).unwrap();
        std::fs::rename(dir.path().join("b/c"), dir.path().join("d")).unwrap();
        std::fs::rename(dir.path().join("a/file2"), dir.path().join("a/file2.new")).unwrap();
        tree.flush_fs_events(&cx).await;

        let expected_paths = vec![
            "a",
            "a/file1",
            "a/file2.new",
            "b",
            "d",
            "d/file3",
            "d/file4",
        ];

        cx.read(|app| {
            assert_eq!(
                tree.read(app)
                    .paths()
                    .map(|p| p.to_str().unwrap())
                    .collect::<Vec<_>>(),
                expected_paths
            );

            assert_eq!(id_for_path("a/file2.new", &cx), file2_id);
            assert_eq!(id_for_path("d/file3", &cx), file3_id);
            assert_eq!(id_for_path("d/file4", &cx), file4_id);

            assert_eq!(
                buffer2.read(app).file().unwrap().path().as_ref(),
                Path::new("a/file2.new")
            );
            assert_eq!(
                buffer3.read(app).file().unwrap().path().as_ref(),
                Path::new("d/file3")
            );
            assert_eq!(
                buffer4.read(app).file().unwrap().path().as_ref(),
                Path::new("d/file4")
            );
            assert_eq!(
                buffer5.read(app).file().unwrap().path().as_ref(),
                Path::new("b/c/file5")
            );

            assert!(!buffer2.read(app).file().unwrap().is_deleted());
            assert!(!buffer3.read(app).file().unwrap().is_deleted());
            assert!(!buffer4.read(app).file().unwrap().is_deleted());
            assert!(buffer5.read(app).file().unwrap().is_deleted());
        });

        // Update the remote worktree. Check that it becomes consistent with the
        // local worktree.
        remote.update(&mut cx, |remote, cx| {
            let update_message =
                tree.read(cx)
                    .snapshot()
                    .build_update(&initial_snapshot, 1, 1, true);
            remote
                .as_remote_mut()
                .unwrap()
                .snapshot
                .apply_update(update_message)
                .unwrap();

            assert_eq!(
                remote
                    .paths()
                    .map(|p| p.to_str().unwrap())
                    .collect::<Vec<_>>(),
                expected_paths
            );
        });
    }

    #[gpui::test]
    async fn test_rescan_with_gitignore(mut cx: gpui::TestAppContext) {
        let dir = temp_tree(json!({
            ".git": {},
            ".gitignore": "ignored-dir\n",
            "tracked-dir": {
                "tracked-file1": "tracked contents",
            },
            "ignored-dir": {
                "ignored-file1": "ignored contents",
            }
        }));

        let http_client = FakeHttpClient::with_404_response();
        let client = Client::new(http_client.clone());
        let user_store = cx.add_model(|cx| UserStore::new(client.clone(), http_client, cx));

        let tree = Worktree::open_local(
            client,
            user_store,
            dir.path(),
            Arc::new(RealFs),
            Default::default(),
            &mut cx.to_async(),
        )
        .await
        .unwrap();
        cx.read(|cx| tree.read(cx).as_local().unwrap().scan_complete())
            .await;
        tree.flush_fs_events(&cx).await;
        cx.read(|cx| {
            let tree = tree.read(cx);
            let tracked = tree.entry_for_path("tracked-dir/tracked-file1").unwrap();
            let ignored = tree.entry_for_path("ignored-dir/ignored-file1").unwrap();
            assert_eq!(tracked.is_ignored, false);
            assert_eq!(ignored.is_ignored, true);
        });

        std::fs::write(dir.path().join("tracked-dir/tracked-file2"), "").unwrap();
        std::fs::write(dir.path().join("ignored-dir/ignored-file2"), "").unwrap();
        tree.flush_fs_events(&cx).await;
        cx.read(|cx| {
            let tree = tree.read(cx);
            let dot_git = tree.entry_for_path(".git").unwrap();
            let tracked = tree.entry_for_path("tracked-dir/tracked-file2").unwrap();
            let ignored = tree.entry_for_path("ignored-dir/ignored-file2").unwrap();
            assert_eq!(tracked.is_ignored, false);
            assert_eq!(ignored.is_ignored, true);
            assert_eq!(dot_git.is_ignored, true);
        });
    }

    #[gpui::test]
    async fn test_buffer_deduping(mut cx: gpui::TestAppContext) {
        let user_id = 100;
        let http_client = FakeHttpClient::with_404_response();
        let mut client = Client::new(http_client);
        let server = FakeServer::for_client(user_id, &mut client, &cx).await;
        let user_store = server.build_user_store(client.clone(), &mut cx).await;

        let fs = Arc::new(FakeFs::new());
        fs.insert_tree(
            "/the-dir",
            json!({
                "a.txt": "a-contents",
                "b.txt": "b-contents",
            }),
        )
        .await;

        let worktree = Worktree::open_local(
            client.clone(),
            user_store,
            "/the-dir".as_ref(),
            fs,
            Default::default(),
            &mut cx.to_async(),
        )
        .await
        .unwrap();

        // Spawn multiple tasks to open paths, repeating some paths.
        let (buffer_a_1, buffer_b, buffer_a_2) = worktree.update(&mut cx, |worktree, cx| {
            (
                worktree.open_buffer("a.txt", cx),
                worktree.open_buffer("b.txt", cx),
                worktree.open_buffer("a.txt", cx),
            )
        });

        let buffer_a_1 = buffer_a_1.await.unwrap();
        let buffer_a_2 = buffer_a_2.await.unwrap();
        let buffer_b = buffer_b.await.unwrap();
        assert_eq!(buffer_a_1.read_with(&cx, |b, _| b.text()), "a-contents");
        assert_eq!(buffer_b.read_with(&cx, |b, _| b.text()), "b-contents");

        // There is only one buffer per path.
        let buffer_a_id = buffer_a_1.id();
        assert_eq!(buffer_a_2.id(), buffer_a_id);

        // Open the same path again while it is still open.
        drop(buffer_a_1);
        let buffer_a_3 = worktree
            .update(&mut cx, |worktree, cx| worktree.open_buffer("a.txt", cx))
            .await
            .unwrap();

        // There's still only one buffer per path.
        assert_eq!(buffer_a_3.id(), buffer_a_id);
    }

    #[gpui::test]
    async fn test_buffer_is_dirty(mut cx: gpui::TestAppContext) {
        use std::fs;

        let dir = temp_tree(json!({
            "file1": "abc",
            "file2": "def",
            "file3": "ghi",
        }));
        let http_client = FakeHttpClient::with_404_response();
        let client = Client::new(http_client.clone());
        let user_store = cx.add_model(|cx| UserStore::new(client.clone(), http_client, cx));

        let tree = Worktree::open_local(
            client,
            user_store,
            dir.path(),
            Arc::new(RealFs),
            Default::default(),
            &mut cx.to_async(),
        )
        .await
        .unwrap();
        tree.flush_fs_events(&cx).await;
        cx.read(|cx| tree.read(cx).as_local().unwrap().scan_complete())
            .await;

        let buffer1 = tree
            .update(&mut cx, |tree, cx| tree.open_buffer("file1", cx))
            .await
            .unwrap();
        let events = Rc::new(RefCell::new(Vec::new()));

        // initially, the buffer isn't dirty.
        buffer1.update(&mut cx, |buffer, cx| {
            cx.subscribe(&buffer1, {
                let events = events.clone();
                move |_, _, event, _| events.borrow_mut().push(event.clone())
            })
            .detach();

            assert!(!buffer.is_dirty());
            assert!(events.borrow().is_empty());

            buffer.edit(vec![1..2], "", cx);
        });

        // after the first edit, the buffer is dirty, and emits a dirtied event.
        buffer1.update(&mut cx, |buffer, cx| {
            assert!(buffer.text() == "ac");
            assert!(buffer.is_dirty());
            assert_eq!(
                *events.borrow(),
                &[language::Event::Edited, language::Event::Dirtied]
            );
            events.borrow_mut().clear();
            buffer.did_save(buffer.version(), buffer.file().unwrap().mtime(), None, cx);
        });

        // after saving, the buffer is not dirty, and emits a saved event.
        buffer1.update(&mut cx, |buffer, cx| {
            assert!(!buffer.is_dirty());
            assert_eq!(*events.borrow(), &[language::Event::Saved]);
            events.borrow_mut().clear();

            buffer.edit(vec![1..1], "B", cx);
            buffer.edit(vec![2..2], "D", cx);
        });

        // after editing again, the buffer is dirty, and emits another dirty event.
        buffer1.update(&mut cx, |buffer, cx| {
            assert!(buffer.text() == "aBDc");
            assert!(buffer.is_dirty());
            assert_eq!(
                *events.borrow(),
                &[
                    language::Event::Edited,
                    language::Event::Dirtied,
                    language::Event::Edited,
                ],
            );
            events.borrow_mut().clear();

            // TODO - currently, after restoring the buffer to its
            // previously-saved state, the is still considered dirty.
            buffer.edit([1..3], "", cx);
            assert!(buffer.text() == "ac");
            assert!(buffer.is_dirty());
        });

        assert_eq!(*events.borrow(), &[language::Event::Edited]);

        // When a file is deleted, the buffer is considered dirty.
        let events = Rc::new(RefCell::new(Vec::new()));
        let buffer2 = tree
            .update(&mut cx, |tree, cx| tree.open_buffer("file2", cx))
            .await
            .unwrap();
        buffer2.update(&mut cx, |_, cx| {
            cx.subscribe(&buffer2, {
                let events = events.clone();
                move |_, _, event, _| events.borrow_mut().push(event.clone())
            })
            .detach();
        });

        fs::remove_file(dir.path().join("file2")).unwrap();
        buffer2.condition(&cx, |b, _| b.is_dirty()).await;
        assert_eq!(
            *events.borrow(),
            &[language::Event::Dirtied, language::Event::FileHandleChanged]
        );

        // When a file is already dirty when deleted, we don't emit a Dirtied event.
        let events = Rc::new(RefCell::new(Vec::new()));
        let buffer3 = tree
            .update(&mut cx, |tree, cx| tree.open_buffer("file3", cx))
            .await
            .unwrap();
        buffer3.update(&mut cx, |_, cx| {
            cx.subscribe(&buffer3, {
                let events = events.clone();
                move |_, _, event, _| events.borrow_mut().push(event.clone())
            })
            .detach();
        });

        tree.flush_fs_events(&cx).await;
        buffer3.update(&mut cx, |buffer, cx| {
            buffer.edit(Some(0..0), "x", cx);
        });
        events.borrow_mut().clear();
        fs::remove_file(dir.path().join("file3")).unwrap();
        buffer3
            .condition(&cx, |_, _| !events.borrow().is_empty())
            .await;
        assert_eq!(*events.borrow(), &[language::Event::FileHandleChanged]);
        cx.read(|cx| assert!(buffer3.read(cx).is_dirty()));
    }

    #[gpui::test]
    async fn test_buffer_file_changes_on_disk(mut cx: gpui::TestAppContext) {
        use std::fs;

        let initial_contents = "aaa\nbbbbb\nc\n";
        let dir = temp_tree(json!({ "the-file": initial_contents }));
        let http_client = FakeHttpClient::with_404_response();
        let client = Client::new(http_client.clone());
        let user_store = cx.add_model(|cx| UserStore::new(client.clone(), http_client, cx));

        let tree = Worktree::open_local(
            client,
            user_store,
            dir.path(),
            Arc::new(RealFs),
            Default::default(),
            &mut cx.to_async(),
        )
        .await
        .unwrap();
        cx.read(|cx| tree.read(cx).as_local().unwrap().scan_complete())
            .await;

        let abs_path = dir.path().join("the-file");
        let buffer = tree
            .update(&mut cx, |tree, cx| {
                tree.open_buffer(Path::new("the-file"), cx)
            })
            .await
            .unwrap();

        // TODO
        // Add a cursor on each row.
        // let selection_set_id = buffer.update(&mut cx, |buffer, cx| {
        //     assert!(!buffer.is_dirty());
        //     buffer.add_selection_set(
        //         &(0..3)
        //             .map(|row| Selection {
        //                 id: row as usize,
        //                 start: Point::new(row, 1),
        //                 end: Point::new(row, 1),
        //                 reversed: false,
        //                 goal: SelectionGoal::None,
        //             })
        //             .collect::<Vec<_>>(),
        //         cx,
        //     )
        // });

        // Change the file on disk, adding two new lines of text, and removing
        // one line.
        buffer.read_with(&cx, |buffer, _| {
            assert!(!buffer.is_dirty());
            assert!(!buffer.has_conflict());
        });
        let new_contents = "AAAA\naaa\nBB\nbbbbb\n";
        fs::write(&abs_path, new_contents).unwrap();

        // Because the buffer was not modified, it is reloaded from disk. Its
        // contents are edited according to the diff between the old and new
        // file contents.
        buffer
            .condition(&cx, |buffer, _| buffer.text() == new_contents)
            .await;

        buffer.update(&mut cx, |buffer, _| {
            assert_eq!(buffer.text(), new_contents);
            assert!(!buffer.is_dirty());
            assert!(!buffer.has_conflict());

            // TODO
            // let cursor_positions = buffer
            //     .selection_set(selection_set_id)
            //     .unwrap()
            //     .selections::<Point>(&*buffer)
            //     .map(|selection| {
            //         assert_eq!(selection.start, selection.end);
            //         selection.start
            //     })
            //     .collect::<Vec<_>>();
            // assert_eq!(
            //     cursor_positions,
            //     [Point::new(1, 1), Point::new(3, 1), Point::new(4, 0)]
            // );
        });

        // Modify the buffer
        buffer.update(&mut cx, |buffer, cx| {
            buffer.edit(vec![0..0], " ", cx);
            assert!(buffer.is_dirty());
            assert!(!buffer.has_conflict());
        });

        // Change the file on disk again, adding blank lines to the beginning.
        fs::write(&abs_path, "\n\n\nAAAA\naaa\nBB\nbbbbb\n").unwrap();

        // Because the buffer is modified, it doesn't reload from disk, but is
        // marked as having a conflict.
        buffer
            .condition(&cx, |buffer, _| buffer.has_conflict())
            .await;
    }

    #[gpui::test]
    async fn test_language_server_diagnostics(mut cx: gpui::TestAppContext) {
        let (language_server_config, mut fake_server) =
            LanguageServerConfig::fake(cx.background()).await;
        let mut languages = LanguageRegistry::new();
        languages.add(Arc::new(Language::new(
            LanguageConfig {
                name: "Rust".to_string(),
                path_suffixes: vec!["rs".to_string()],
                language_server: Some(language_server_config),
                ..Default::default()
            },
            Some(tree_sitter_rust::language()),
        )));

        let dir = temp_tree(json!({
            "a.rs": "fn a() { A }",
            "b.rs": "const y: i32 = 1",
        }));

        let http_client = FakeHttpClient::with_404_response();
        let client = Client::new(http_client.clone());
        let user_store = cx.add_model(|cx| UserStore::new(client.clone(), http_client, cx));

        let tree = Worktree::open_local(
            client,
            user_store,
            dir.path(),
            Arc::new(RealFs),
            Arc::new(languages),
            &mut cx.to_async(),
        )
        .await
        .unwrap();
        cx.read(|cx| tree.read(cx).as_local().unwrap().scan_complete())
            .await;

        // Cause worktree to start the fake language server
        let _buffer = tree
            .update(&mut cx, |tree, cx| tree.open_buffer("b.rs", cx))
            .await
            .unwrap();

        fake_server
            .notify::<lsp::notification::PublishDiagnostics>(lsp::PublishDiagnosticsParams {
                uri: Url::from_file_path(dir.path().join("a.rs")).unwrap(),
                version: None,
                diagnostics: vec![lsp::Diagnostic {
                    range: lsp::Range::new(lsp::Position::new(0, 9), lsp::Position::new(0, 10)),
                    severity: Some(lsp::DiagnosticSeverity::ERROR),
                    message: "undefined variable 'A'".to_string(),
                    ..Default::default()
                }],
            })
            .await;

        let buffer = tree
            .update(&mut cx, |tree, cx| tree.open_buffer("a.rs", cx))
            .await
            .unwrap();

        buffer.read_with(&cx, |buffer, _| {
            let snapshot = buffer.snapshot();
            let diagnostics = snapshot
                .diagnostics_in_range::<_, Point>(0..buffer.len())
                .collect::<Vec<_>>();
            assert_eq!(
                diagnostics,
                &[(
                    LSP_PROVIDER_NAME.as_ref(),
                    DiagnosticEntry {
                        range: Point::new(0, 9)..Point::new(0, 10),
                        diagnostic: Diagnostic {
                            severity: lsp::DiagnosticSeverity::ERROR,
                            message: "undefined variable 'A'".to_string(),
                            group_id: 0,
                            is_primary: true,
                            ..Default::default()
                        }
                    }
                )]
            )
        });
    }

    #[gpui::test]
    async fn test_grouped_diagnostics(mut cx: gpui::TestAppContext) {
        let fs = Arc::new(FakeFs::new());
        let http_client = FakeHttpClient::with_404_response();
        let client = Client::new(http_client.clone());
        let user_store = cx.add_model(|cx| UserStore::new(client.clone(), http_client, cx));

        fs.insert_tree(
            "/the-dir",
            json!({
                "a.rs": "
                    fn foo(mut v: Vec<usize>) {
                        for x in &v {
                            v.push(1);
                        }
                    }
                "
                .unindent(),
            }),
        )
        .await;

        let worktree = Worktree::open_local(
            client.clone(),
            user_store,
            "/the-dir".as_ref(),
            fs,
            Default::default(),
            &mut cx.to_async(),
        )
        .await
        .unwrap();

        let buffer = worktree
            .update(&mut cx, |tree, cx| tree.open_buffer("a.rs", cx))
            .await
            .unwrap();

        let buffer_uri = Url::from_file_path("/the-dir/a.rs").unwrap();
        let message = lsp::PublishDiagnosticsParams {
            uri: buffer_uri.clone(),
            diagnostics: vec![
                lsp::Diagnostic {
                    range: lsp::Range::new(lsp::Position::new(1, 8), lsp::Position::new(1, 9)),
                    severity: Some(DiagnosticSeverity::WARNING),
                    message: "error 1".to_string(),
                    related_information: Some(vec![lsp::DiagnosticRelatedInformation {
                        location: lsp::Location {
                            uri: buffer_uri.clone(),
                            range: lsp::Range::new(
                                lsp::Position::new(1, 8),
                                lsp::Position::new(1, 9),
                            ),
                        },
                        message: "error 1 hint 1".to_string(),
                    }]),
                    ..Default::default()
                },
                lsp::Diagnostic {
                    range: lsp::Range::new(lsp::Position::new(1, 8), lsp::Position::new(1, 9)),
                    severity: Some(DiagnosticSeverity::HINT),
                    message: "error 1 hint 1".to_string(),
                    related_information: Some(vec![lsp::DiagnosticRelatedInformation {
                        location: lsp::Location {
                            uri: buffer_uri.clone(),
                            range: lsp::Range::new(
                                lsp::Position::new(1, 8),
                                lsp::Position::new(1, 9),
                            ),
                        },
                        message: "original diagnostic".to_string(),
                    }]),
                    ..Default::default()
                },
                lsp::Diagnostic {
                    range: lsp::Range::new(lsp::Position::new(2, 8), lsp::Position::new(2, 17)),
                    severity: Some(DiagnosticSeverity::ERROR),
                    message: "error 2".to_string(),
                    related_information: Some(vec![
                        lsp::DiagnosticRelatedInformation {
                            location: lsp::Location {
                                uri: buffer_uri.clone(),
                                range: lsp::Range::new(
                                    lsp::Position::new(1, 13),
                                    lsp::Position::new(1, 15),
                                ),
                            },
                            message: "error 2 hint 1".to_string(),
                        },
                        lsp::DiagnosticRelatedInformation {
                            location: lsp::Location {
                                uri: buffer_uri.clone(),
                                range: lsp::Range::new(
                                    lsp::Position::new(1, 13),
                                    lsp::Position::new(1, 15),
                                ),
                            },
                            message: "error 2 hint 2".to_string(),
                        },
                    ]),
                    ..Default::default()
                },
                lsp::Diagnostic {
                    range: lsp::Range::new(lsp::Position::new(1, 13), lsp::Position::new(1, 15)),
                    severity: Some(DiagnosticSeverity::HINT),
                    message: "error 2 hint 1".to_string(),
                    related_information: Some(vec![lsp::DiagnosticRelatedInformation {
                        location: lsp::Location {
                            uri: buffer_uri.clone(),
                            range: lsp::Range::new(
                                lsp::Position::new(2, 8),
                                lsp::Position::new(2, 17),
                            ),
                        },
                        message: "original diagnostic".to_string(),
                    }]),
                    ..Default::default()
                },
                lsp::Diagnostic {
                    range: lsp::Range::new(lsp::Position::new(1, 13), lsp::Position::new(1, 15)),
                    severity: Some(DiagnosticSeverity::HINT),
                    message: "error 2 hint 2".to_string(),
                    related_information: Some(vec![lsp::DiagnosticRelatedInformation {
                        location: lsp::Location {
                            uri: buffer_uri.clone(),
                            range: lsp::Range::new(
                                lsp::Position::new(2, 8),
                                lsp::Position::new(2, 17),
                            ),
                        },
                        message: "original diagnostic".to_string(),
                    }]),
                    ..Default::default()
                },
            ],
            version: None,
        };

        worktree
            .update(&mut cx, |tree, cx| {
                tree.update_diagnostics_from_lsp(message, &Default::default(), cx)
            })
            .unwrap();
        let buffer = buffer.read_with(&cx, |buffer, _| buffer.snapshot());

        assert_eq!(
            buffer
                .diagnostics_in_range::<_, Point>(0..buffer.len())
                .collect::<Vec<_>>(),
            &[
                (
                    LSP_PROVIDER_NAME.as_ref(),
                    DiagnosticEntry {
                        range: Point::new(1, 8)..Point::new(1, 9),
                        diagnostic: Diagnostic {
                            severity: DiagnosticSeverity::WARNING,
                            message: "error 1".to_string(),
                            group_id: 0,
                            is_primary: true,
                            ..Default::default()
                        }
                    }
                ),
                (
                    LSP_PROVIDER_NAME.as_ref(),
                    DiagnosticEntry {
                        range: Point::new(1, 8)..Point::new(1, 9),
                        diagnostic: Diagnostic {
                            severity: DiagnosticSeverity::HINT,
                            message: "error 1 hint 1".to_string(),
                            group_id: 0,
                            is_primary: false,
                            ..Default::default()
                        }
                    }
                ),
                (
                    LSP_PROVIDER_NAME.as_ref(),
                    DiagnosticEntry {
                        range: Point::new(1, 13)..Point::new(1, 15),
                        diagnostic: Diagnostic {
                            severity: DiagnosticSeverity::HINT,
                            message: "error 2 hint 1".to_string(),
                            group_id: 1,
                            is_primary: false,
                            ..Default::default()
                        }
                    }
                ),
                (
                    LSP_PROVIDER_NAME.as_ref(),
                    DiagnosticEntry {
                        range: Point::new(1, 13)..Point::new(1, 15),
                        diagnostic: Diagnostic {
                            severity: DiagnosticSeverity::HINT,
                            message: "error 2 hint 2".to_string(),
                            group_id: 1,
                            is_primary: false,
                            ..Default::default()
                        }
                    }
                ),
                (
                    LSP_PROVIDER_NAME.as_ref(),
                    DiagnosticEntry {
                        range: Point::new(2, 8)..Point::new(2, 17),
                        diagnostic: Diagnostic {
                            severity: DiagnosticSeverity::ERROR,
                            message: "error 2".to_string(),
                            group_id: 1,
                            is_primary: true,
                            ..Default::default()
                        }
                    }
                )
            ]
        );

        assert_eq!(
            buffer
                .diagnostic_group::<Point>(&LSP_PROVIDER_NAME, 0)
                .collect::<Vec<_>>(),
            &[
                DiagnosticEntry {
                    range: Point::new(1, 8)..Point::new(1, 9),
                    diagnostic: Diagnostic {
                        severity: DiagnosticSeverity::WARNING,
                        message: "error 1".to_string(),
                        group_id: 0,
                        is_primary: true,
                        ..Default::default()
                    }
                },
                DiagnosticEntry {
                    range: Point::new(1, 8)..Point::new(1, 9),
                    diagnostic: Diagnostic {
                        severity: DiagnosticSeverity::HINT,
                        message: "error 1 hint 1".to_string(),
                        group_id: 0,
                        is_primary: false,
                        ..Default::default()
                    }
                },
            ]
        );
        assert_eq!(
            buffer
                .diagnostic_group::<Point>(&LSP_PROVIDER_NAME, 1)
                .collect::<Vec<_>>(),
            &[
                DiagnosticEntry {
                    range: Point::new(1, 13)..Point::new(1, 15),
                    diagnostic: Diagnostic {
                        severity: DiagnosticSeverity::HINT,
                        message: "error 2 hint 1".to_string(),
                        group_id: 1,
                        is_primary: false,
                        ..Default::default()
                    }
                },
                DiagnosticEntry {
                    range: Point::new(1, 13)..Point::new(1, 15),
                    diagnostic: Diagnostic {
                        severity: DiagnosticSeverity::HINT,
                        message: "error 2 hint 2".to_string(),
                        group_id: 1,
                        is_primary: false,
                        ..Default::default()
                    }
                },
                DiagnosticEntry {
                    range: Point::new(2, 8)..Point::new(2, 17),
                    diagnostic: Diagnostic {
                        severity: DiagnosticSeverity::ERROR,
                        message: "error 2".to_string(),
                        group_id: 1,
                        is_primary: true,
                        ..Default::default()
                    }
                }
            ]
        );
    }

    #[gpui::test(iterations = 100)]
    fn test_random(mut rng: StdRng) {
        let operations = env::var("OPERATIONS")
            .map(|o| o.parse().unwrap())
            .unwrap_or(40);
        let initial_entries = env::var("INITIAL_ENTRIES")
            .map(|o| o.parse().unwrap())
            .unwrap_or(20);

        let root_dir = tempdir::TempDir::new("worktree-test").unwrap();
        for _ in 0..initial_entries {
            randomly_mutate_tree(root_dir.path(), 1.0, &mut rng).unwrap();
        }
        log::info!("Generated initial tree");

        let (notify_tx, _notify_rx) = smol::channel::unbounded();
        let fs = Arc::new(RealFs);
        let next_entry_id = Arc::new(AtomicUsize::new(0));
        let mut initial_snapshot = Snapshot {
            id: WorktreeId::from_usize(0),
            scan_id: 0,
            abs_path: root_dir.path().into(),
            entries_by_path: Default::default(),
            entries_by_id: Default::default(),
            removed_entry_ids: Default::default(),
            ignores: Default::default(),
            root_name: Default::default(),
            root_char_bag: Default::default(),
            next_entry_id: next_entry_id.clone(),
        };
        initial_snapshot.insert_entry(
            Entry::new(
                Path::new("").into(),
                &smol::block_on(fs.metadata(root_dir.path()))
                    .unwrap()
                    .unwrap(),
                &next_entry_id,
                Default::default(),
            ),
            fs.as_ref(),
        );
        let mut scanner = BackgroundScanner::new(
            Arc::new(Mutex::new(initial_snapshot.clone())),
            notify_tx,
            fs.clone(),
            Arc::new(gpui::executor::Background::new()),
        );
        smol::block_on(scanner.scan_dirs()).unwrap();
        scanner.snapshot().check_invariants();

        let mut events = Vec::new();
        let mut snapshots = Vec::new();
        let mut mutations_len = operations;
        while mutations_len > 1 {
            if !events.is_empty() && rng.gen_bool(0.4) {
                let len = rng.gen_range(0..=events.len());
                let to_deliver = events.drain(0..len).collect::<Vec<_>>();
                log::info!("Delivering events: {:#?}", to_deliver);
                smol::block_on(scanner.process_events(to_deliver));
                scanner.snapshot().check_invariants();
            } else {
                events.extend(randomly_mutate_tree(root_dir.path(), 0.6, &mut rng).unwrap());
                mutations_len -= 1;
            }

            if rng.gen_bool(0.2) {
                snapshots.push(scanner.snapshot());
            }
        }
        log::info!("Quiescing: {:#?}", events);
        smol::block_on(scanner.process_events(events));
        scanner.snapshot().check_invariants();

        let (notify_tx, _notify_rx) = smol::channel::unbounded();
        let mut new_scanner = BackgroundScanner::new(
            Arc::new(Mutex::new(initial_snapshot)),
            notify_tx,
            scanner.fs.clone(),
            scanner.executor.clone(),
        );
        smol::block_on(new_scanner.scan_dirs()).unwrap();
        assert_eq!(
            scanner.snapshot().to_vec(true),
            new_scanner.snapshot().to_vec(true)
        );

        for mut prev_snapshot in snapshots {
            let include_ignored = rng.gen::<bool>();
            if !include_ignored {
                let mut entries_by_path_edits = Vec::new();
                let mut entries_by_id_edits = Vec::new();
                for entry in prev_snapshot
                    .entries_by_id
                    .cursor::<()>()
                    .filter(|e| e.is_ignored)
                {
                    entries_by_path_edits.push(Edit::Remove(PathKey(entry.path.clone())));
                    entries_by_id_edits.push(Edit::Remove(entry.id));
                }

                prev_snapshot
                    .entries_by_path
                    .edit(entries_by_path_edits, &());
                prev_snapshot.entries_by_id.edit(entries_by_id_edits, &());
            }

            let update = scanner
                .snapshot()
                .build_update(&prev_snapshot, 0, 0, include_ignored);
            prev_snapshot.apply_update(update).unwrap();
            assert_eq!(
                prev_snapshot.to_vec(true),
                scanner.snapshot().to_vec(include_ignored)
            );
        }
    }

    fn randomly_mutate_tree(
        root_path: &Path,
        insertion_probability: f64,
        rng: &mut impl Rng,
    ) -> Result<Vec<fsevent::Event>> {
        let root_path = root_path.canonicalize().unwrap();
        let (dirs, files) = read_dir_recursive(root_path.clone());

        let mut events = Vec::new();
        let mut record_event = |path: PathBuf| {
            events.push(fsevent::Event {
                event_id: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                flags: fsevent::StreamFlags::empty(),
                path,
            });
        };

        if (files.is_empty() && dirs.len() == 1) || rng.gen_bool(insertion_probability) {
            let path = dirs.choose(rng).unwrap();
            let new_path = path.join(gen_name(rng));

            if rng.gen() {
                log::info!("Creating dir {:?}", new_path.strip_prefix(root_path)?);
                std::fs::create_dir(&new_path)?;
            } else {
                log::info!("Creating file {:?}", new_path.strip_prefix(root_path)?);
                std::fs::write(&new_path, "")?;
            }
            record_event(new_path);
        } else if rng.gen_bool(0.05) {
            let ignore_dir_path = dirs.choose(rng).unwrap();
            let ignore_path = ignore_dir_path.join(&*GITIGNORE);

            let (subdirs, subfiles) = read_dir_recursive(ignore_dir_path.clone());
            let files_to_ignore = {
                let len = rng.gen_range(0..=subfiles.len());
                subfiles.choose_multiple(rng, len)
            };
            let dirs_to_ignore = {
                let len = rng.gen_range(0..subdirs.len());
                subdirs.choose_multiple(rng, len)
            };

            let mut ignore_contents = String::new();
            for path_to_ignore in files_to_ignore.chain(dirs_to_ignore) {
                write!(
                    ignore_contents,
                    "{}\n",
                    path_to_ignore
                        .strip_prefix(&ignore_dir_path)?
                        .to_str()
                        .unwrap()
                )
                .unwrap();
            }
            log::info!(
                "Creating {:?} with contents:\n{}",
                ignore_path.strip_prefix(&root_path)?,
                ignore_contents
            );
            std::fs::write(&ignore_path, ignore_contents).unwrap();
            record_event(ignore_path);
        } else {
            let old_path = {
                let file_path = files.choose(rng);
                let dir_path = dirs[1..].choose(rng);
                file_path.into_iter().chain(dir_path).choose(rng).unwrap()
            };

            let is_rename = rng.gen();
            if is_rename {
                let new_path_parent = dirs
                    .iter()
                    .filter(|d| !d.starts_with(old_path))
                    .choose(rng)
                    .unwrap();

                let overwrite_existing_dir =
                    !old_path.starts_with(&new_path_parent) && rng.gen_bool(0.3);
                let new_path = if overwrite_existing_dir {
                    std::fs::remove_dir_all(&new_path_parent).ok();
                    new_path_parent.to_path_buf()
                } else {
                    new_path_parent.join(gen_name(rng))
                };

                log::info!(
                    "Renaming {:?} to {}{:?}",
                    old_path.strip_prefix(&root_path)?,
                    if overwrite_existing_dir {
                        "overwrite "
                    } else {
                        ""
                    },
                    new_path.strip_prefix(&root_path)?
                );
                std::fs::rename(&old_path, &new_path)?;
                record_event(old_path.clone());
                record_event(new_path);
            } else if old_path.is_dir() {
                let (dirs, files) = read_dir_recursive(old_path.clone());

                log::info!("Deleting dir {:?}", old_path.strip_prefix(&root_path)?);
                std::fs::remove_dir_all(&old_path).unwrap();
                for file in files {
                    record_event(file);
                }
                for dir in dirs {
                    record_event(dir);
                }
            } else {
                log::info!("Deleting file {:?}", old_path.strip_prefix(&root_path)?);
                std::fs::remove_file(old_path).unwrap();
                record_event(old_path.clone());
            }
        }

        Ok(events)
    }

    fn read_dir_recursive(path: PathBuf) -> (Vec<PathBuf>, Vec<PathBuf>) {
        let child_entries = std::fs::read_dir(&path).unwrap();
        let mut dirs = vec![path];
        let mut files = Vec::new();
        for child_entry in child_entries {
            let child_path = child_entry.unwrap().path();
            if child_path.is_dir() {
                let (child_dirs, child_files) = read_dir_recursive(child_path);
                dirs.extend(child_dirs);
                files.extend(child_files);
            } else {
                files.push(child_path);
            }
        }
        (dirs, files)
    }

    fn gen_name(rng: &mut impl Rng) -> String {
        (0..6)
            .map(|_| rng.sample(rand::distributions::Alphanumeric))
            .map(char::from)
            .collect()
    }

    impl Snapshot {
        fn check_invariants(&self) {
            let mut files = self.files(true, 0);
            let mut visible_files = self.files(false, 0);
            for entry in self.entries_by_path.cursor::<()>() {
                if entry.is_file() {
                    assert_eq!(files.next().unwrap().inode, entry.inode);
                    if !entry.is_ignored {
                        assert_eq!(visible_files.next().unwrap().inode, entry.inode);
                    }
                }
            }
            assert!(files.next().is_none());
            assert!(visible_files.next().is_none());

            let mut bfs_paths = Vec::new();
            let mut stack = vec![Path::new("")];
            while let Some(path) = stack.pop() {
                bfs_paths.push(path);
                let ix = stack.len();
                for child_entry in self.child_entries(path) {
                    stack.insert(ix, &child_entry.path);
                }
            }

            let dfs_paths = self
                .entries_by_path
                .cursor::<()>()
                .map(|e| e.path.as_ref())
                .collect::<Vec<_>>();
            assert_eq!(bfs_paths, dfs_paths);

            for (ignore_parent_path, _) in &self.ignores {
                assert!(self.entry_for_path(ignore_parent_path).is_some());
                assert!(self
                    .entry_for_path(ignore_parent_path.join(&*GITIGNORE))
                    .is_some());
            }
        }

        fn to_vec(&self, include_ignored: bool) -> Vec<(&Path, u64, bool)> {
            let mut paths = Vec::new();
            for entry in self.entries_by_path.cursor::<()>() {
                if include_ignored || !entry.is_ignored {
                    paths.push((entry.path.as_ref(), entry.inode, entry.is_ignored));
                }
            }
            paths.sort_by(|a, b| a.0.cmp(&b.0));
            paths
        }
    }
}
