use anyhow::{Context as _, Result};
use fs::Fs;
use gpui::{AppContext, AsyncAppContext, Context, Model, ModelContext};
use project::{buffer_store::BufferStore, ProjectPath, WorktreeId, WorktreeSettings};
use remote::SshSession;
use rpc::{
    proto::{self, AnyProtoClient, PeerId},
    TypedEnvelope,
};
use settings::{Settings as _, SettingsStore};
use std::{
    path::{Path, PathBuf},
    sync::{atomic::AtomicUsize, Arc},
};
use worktree::Worktree;

const PEER_ID: PeerId = PeerId { owner_id: 0, id: 0 };
const PROJECT_ID: u64 = 0;

pub struct HeadlessProject {
    pub fs: Arc<dyn Fs>,
    pub session: AnyProtoClient,
    pub worktrees: Vec<Model<Worktree>>,
    pub buffer_store: Model<BufferStore>,
    pub next_entry_id: Arc<AtomicUsize>,
}

impl HeadlessProject {
    pub fn init(cx: &mut AppContext) {
        cx.set_global(SettingsStore::default());
        WorktreeSettings::register(cx);
    }

    pub fn new(session: Arc<SshSession>, fs: Arc<dyn Fs>, cx: &mut ModelContext<Self>) -> Self {
        let this = cx.weak_model();

        session.add_request_handler(this.clone(), Self::handle_add_worktree);
        session.add_request_handler(this.clone(), Self::handle_open_buffer_by_path);
        session.add_request_handler(this.clone(), Self::handle_update_buffer);
        session.add_request_handler(this.clone(), Self::handle_save_buffer);

        HeadlessProject {
            session: session.into(),
            fs,
            worktrees: Vec::new(),
            buffer_store: cx.new_model(|_| BufferStore::new(true)),
            next_entry_id: Default::default(),
        }
    }

    fn worktree_for_id(&self, id: WorktreeId, cx: &AppContext) -> Option<Model<Worktree>> {
        self.worktrees
            .iter()
            .find(|worktree| worktree.read(cx).id() == id)
            .cloned()
    }

    pub async fn handle_add_worktree(
        this: Model<Self>,
        message: TypedEnvelope<proto::AddWorktree>,
        mut cx: AsyncAppContext,
    ) -> Result<proto::AddWorktreeResponse> {
        let worktree = this
            .update(&mut cx.clone(), |this, _| {
                Worktree::local(
                    Path::new(&message.payload.path),
                    true,
                    this.fs.clone(),
                    this.next_entry_id.clone(),
                    &mut cx,
                )
            })?
            .await?;

        this.update(&mut cx, |this, cx| {
            let session = this.session.clone();
            this.worktrees.push(worktree.clone());
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

    pub async fn handle_update_buffer(
        this: Model<Self>,
        envelope: TypedEnvelope<proto::UpdateBuffer>,
        mut cx: AsyncAppContext,
    ) -> Result<proto::Ack> {
        this.update(&mut cx, |this, cx| {
            this.buffer_store.update(cx, |buffer_store, cx| {
                buffer_store.handle_update_buffer(envelope, false, cx)
            })
        })?
    }

    pub async fn handle_save_buffer(
        this: Model<Self>,
        envelope: TypedEnvelope<proto::SaveBuffer>,
        mut cx: AsyncAppContext,
    ) -> Result<proto::BufferSaved> {
        let (buffer_store, worktree) = this.update(&mut cx, |this, cx| {
            let buffer_store = this.buffer_store.clone();
            let worktree = if let Some(path) = &envelope.payload.new_path {
                Some(
                    this.worktree_for_id(WorktreeId::from_proto(path.worktree_id), cx)
                        .context("worktree does not exist")?,
                )
            } else {
                None
            };
            anyhow::Ok((buffer_store, worktree))
        })??;
        BufferStore::handle_save_buffer(buffer_store, PROJECT_ID, worktree, envelope, cx).await
    }

    pub async fn handle_open_buffer_by_path(
        this: Model<Self>,
        message: TypedEnvelope<proto::OpenBufferByPath>,
        mut cx: AsyncAppContext,
    ) -> Result<proto::OpenBufferResponse> {
        let worktree_id = WorktreeId::from_proto(message.payload.worktree_id);
        let (buffer_store, buffer, session) = this.update(&mut cx, |this, cx| {
            let worktree = this
                .worktree_for_id(worktree_id, cx)
                .context("no such worktree")?;
            let buffer_store = this.buffer_store.clone();
            let buffer = this.buffer_store.update(cx, |buffer_store, cx| {
                buffer_store.open_buffer(
                    ProjectPath {
                        worktree_id,
                        path: PathBuf::from(message.payload.path).into(),
                    },
                    worktree,
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
}
