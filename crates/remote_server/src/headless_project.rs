use anyhow::Result;
use fs::Fs;
use gpui::{AppContext, AsyncAppContext, Context, Model, ModelContext};
use project::{
    buffer_store::{BufferStore, BufferStoreEvent},
    worktree_store::WorktreeStore,
    ProjectPath, WorktreeId, WorktreeSettings,
};
use remote::SshSession;
use rpc::{
    proto::{self, AnyProtoClient, PeerId},
    TypedEnvelope,
};
use settings::{Settings as _, SettingsStore};
use smol::stream::StreamExt;
use std::{
    path::{Path, PathBuf},
    sync::{atomic::AtomicUsize, Arc},
};
use util::ResultExt as _;
use worktree::Worktree;

const PEER_ID: PeerId = PeerId { owner_id: 0, id: 0 };
const PROJECT_ID: u64 = 0;

pub struct HeadlessProject {
    pub fs: Arc<dyn Fs>,
    pub session: AnyProtoClient,
    pub worktree_store: Model<WorktreeStore>,
    pub buffer_store: Model<BufferStore>,
    pub next_entry_id: Arc<AtomicUsize>,
}

impl HeadlessProject {
    pub fn init(cx: &mut AppContext) {
        cx.set_global(SettingsStore::new(cx));
        WorktreeSettings::register(cx);
    }

    pub fn new(session: Arc<SshSession>, fs: Arc<dyn Fs>, cx: &mut ModelContext<Self>) -> Self {
        let this = cx.weak_model();

        let worktree_store = cx.new_model(|_| WorktreeStore::new(true));
        let buffer_store =
            cx.new_model(|cx| BufferStore::new(worktree_store.clone(), Some(PROJECT_ID), cx));
        cx.subscribe(&buffer_store, Self::on_buffer_store_event)
            .detach();

        session.add_request_handler(this.clone(), Self::handle_list_remote_directory);
        session.add_request_handler(this.clone(), Self::handle_add_worktree);
        session.add_request_handler(this.clone(), Self::handle_open_buffer_by_path);

        session.add_request_handler(buffer_store.downgrade(), BufferStore::handle_blame_buffer);
        session.add_request_handler(buffer_store.downgrade(), BufferStore::handle_update_buffer);
        session.add_request_handler(buffer_store.downgrade(), BufferStore::handle_save_buffer);

        session.add_request_handler(
            worktree_store.downgrade(),
            WorktreeStore::handle_create_project_entry,
        );
        session.add_request_handler(
            worktree_store.downgrade(),
            WorktreeStore::handle_rename_project_entry,
        );
        session.add_request_handler(
            worktree_store.downgrade(),
            WorktreeStore::handle_copy_project_entry,
        );
        session.add_request_handler(
            worktree_store.downgrade(),
            WorktreeStore::handle_delete_project_entry,
        );
        session.add_request_handler(
            worktree_store.downgrade(),
            WorktreeStore::handle_expand_project_entry,
        );

        HeadlessProject {
            session: session.into(),
            fs,
            worktree_store,
            buffer_store,
            next_entry_id: Default::default(),
        }
    }

    pub async fn handle_add_worktree(
        this: Model<Self>,
        message: TypedEnvelope<proto::AddWorktree>,
        mut cx: AsyncAppContext,
    ) -> Result<proto::AddWorktreeResponse> {
        let path = shellexpand::tilde(&message.payload.path).to_string();
        let worktree = this
            .update(&mut cx.clone(), |this, _| {
                Worktree::local(
                    Path::new(&path),
                    true,
                    this.fs.clone(),
                    this.next_entry_id.clone(),
                    &mut cx,
                )
            })?
            .await?;

        this.update(&mut cx, |this, cx| {
            let session = this.session.clone();
            this.worktree_store.update(cx, |worktree_store, cx| {
                worktree_store.add(&worktree, cx);
            });
            worktree.update(cx, |worktree, cx| {
                worktree.observe_updates(0, cx, move |update| {
                    session.send(update).ok();
                    futures::future::ready(true)
                });
                proto::AddWorktreeResponse {
                    worktree_id: worktree.id().to_proto(),
                }
            })
        })
    }

    pub async fn handle_open_buffer_by_path(
        this: Model<Self>,
        message: TypedEnvelope<proto::OpenBufferByPath>,
        mut cx: AsyncAppContext,
    ) -> Result<proto::OpenBufferResponse> {
        let worktree_id = WorktreeId::from_proto(message.payload.worktree_id);
        let (buffer_store, buffer, session) = this.update(&mut cx, |this, cx| {
            let buffer_store = this.buffer_store.clone();
            let buffer = this.buffer_store.update(cx, |buffer_store, cx| {
                buffer_store.open_buffer(
                    ProjectPath {
                        worktree_id,
                        path: PathBuf::from(message.payload.path).into(),
                    },
                    cx,
                )
            });
            anyhow::Ok((buffer_store, buffer, this.session.clone()))
        })??;

        let buffer = buffer.await?;
        let buffer_id = buffer.read_with(&cx, |b, _| b.remote_id())?;

        cx.spawn(|mut cx| async move {
            BufferStore::create_buffer_for_peer(
                buffer_store,
                PEER_ID,
                buffer_id,
                PROJECT_ID,
                session,
                &mut cx,
            )
            .await
        })
        .detach();

        Ok(proto::OpenBufferResponse {
            buffer_id: buffer_id.to_proto(),
        })
    }

    pub async fn handle_list_remote_directory(
        this: Model<Self>,
        envelope: TypedEnvelope<proto::ListRemoteDirectory>,
        cx: AsyncAppContext,
    ) -> Result<proto::ListRemoteDirectoryResponse> {
        let expanded = shellexpand::tilde(&envelope.payload.path).to_string();
        let fs = cx.read_model(&this, |this, _| this.fs.clone())?;

        let mut entries = Vec::new();
        let mut response = fs.read_dir(Path::new(&expanded)).await?;
        while let Some(path) = response.next().await {
            if let Some(file_name) = path?.file_name() {
                entries.push(file_name.to_string_lossy().to_string());
            }
        }
        Ok(proto::ListRemoteDirectoryResponse { entries })
    }

    pub fn on_buffer_store_event(
        &mut self,
        _: Model<BufferStore>,
        event: &BufferStoreEvent,
        _: &mut ModelContext<Self>,
    ) {
        match event {
            BufferStoreEvent::MessageToReplicas(message) => {
                self.session
                    .send_dynamic(message.as_ref().clone())
                    .log_err();
            }
            _ => {}
        }
    }
}
