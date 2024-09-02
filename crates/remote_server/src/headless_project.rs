use anyhow::{anyhow, Result};
use fs::Fs;
use gpui::{AppContext, AsyncAppContext, Context, Model, ModelContext};
use project::{
    buffer_store::BufferStore, search::SearchQuery, worktree_store::WorktreeStore, ProjectPath,
    WorktreeId, WorktreeSettings,
};
use remote::SshSession;
use rpc::{
    proto::{self, AnyProtoClient, SSH_PEER_ID, SSH_PROJECT_ID},
    TypedEnvelope,
};
use settings::{Settings as _, SettingsStore};
use smol::stream::StreamExt;
use std::{
    path::{Path, PathBuf},
    sync::{atomic::AtomicUsize, Arc},
};
use worktree::Worktree;

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
        let worktree_store = cx.new_model(|_| WorktreeStore::new(true, fs.clone()));
        let buffer_store = cx.new_model(|cx| {
            let mut buffer_store =
                BufferStore::new(worktree_store.clone(), Some(SSH_PROJECT_ID), cx);
            buffer_store.shared(SSH_PROJECT_ID, session.clone().into(), cx);
            buffer_store
        });

        let client: AnyProtoClient = session.clone().into();

        session.subscribe_to_entity(SSH_PROJECT_ID, &worktree_store);
        session.subscribe_to_entity(SSH_PROJECT_ID, &buffer_store);
        session.subscribe_to_entity(SSH_PROJECT_ID, &cx.handle());

        client.add_request_handler(cx.weak_model(), Self::handle_list_remote_directory);

        client.add_model_request_handler(Self::handle_add_worktree);
        client.add_model_request_handler(Self::handle_open_buffer_by_path);
        client.add_model_request_handler(Self::handle_find_search_candidates);

        client.add_model_request_handler(BufferStore::handle_update_buffer);
        client.add_model_message_handler(BufferStore::handle_close_buffer);

        BufferStore::init(&client);
        WorktreeStore::init(&client);

        HeadlessProject {
            session: client,
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
        let (buffer_store, buffer) = this.update(&mut cx, |this, cx| {
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
            anyhow::Ok((buffer_store, buffer))
        })??;

        let buffer = buffer.await?;
        let buffer_id = buffer.read_with(&cx, |b, _| b.remote_id())?;
        buffer_store.update(&mut cx, |buffer_store, cx| {
            buffer_store
                .create_buffer_for_peer(&buffer, SSH_PEER_ID, cx)
                .detach_and_log_err(cx);
        })?;

        Ok(proto::OpenBufferResponse {
            buffer_id: buffer_id.to_proto(),
        })
    }

    pub async fn handle_find_search_candidates(
        this: Model<Self>,
        envelope: TypedEnvelope<proto::FindSearchCandidates>,
        mut cx: AsyncAppContext,
    ) -> Result<proto::FindSearchCandidatesResponse> {
        let message = envelope.payload;
        let query = SearchQuery::from_proto(
            message
                .query
                .ok_or_else(|| anyhow!("missing query field"))?,
        )?;
        let mut results = this.update(&mut cx, |this, cx| {
            this.buffer_store.update(cx, |buffer_store, cx| {
                buffer_store.find_search_candidates(&query, message.limit as _, this.fs.clone(), cx)
            })
        })?;

        let mut response = proto::FindSearchCandidatesResponse {
            buffer_ids: Vec::new(),
        };

        let buffer_store = this.read_with(&cx, |this, _| this.buffer_store.clone())?;

        while let Some(buffer) = results.next().await {
            let buffer_id = buffer.update(&mut cx, |this, _| this.remote_id())?;
            response.buffer_ids.push(buffer_id.to_proto());
            buffer_store
                .update(&mut cx, |buffer_store, cx| {
                    buffer_store.create_buffer_for_peer(&buffer, SSH_PEER_ID, cx)
                })?
                .await?;
        }

        Ok(response)
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
}
