use anyhow::anyhow;
use rpc::proto::{
    self, create_buffer_for_peer, CreateBufferForPeer, EntityMessage, OpenBufferResponse,
    RequestMessage,
};
use sea_orm::{ColumnTrait, Condition, EntityTrait, QueryFilter};

use crate::db::{worktree_entry, HostedProjectId, ProjectId};
use crate::rpc::{Response, Session};
use crate::Result;

pub(crate) trait ProjectRequest: EntityMessage + RequestMessage {
    async fn handle_hosted_project_request(
        self,
        _hosted_project_id: HostedProjectId,
        _response: Response<Self>,
        _session: Session,
    ) -> Result<()> {
        Err(anyhow!("not supported for hosted projects"))?
    }
}
impl ProjectRequest for proto::GetHover {}
impl ProjectRequest for proto::GetDefinition {}
impl ProjectRequest for proto::GetTypeDefinition {}
impl ProjectRequest for proto::GetReferences {}
impl ProjectRequest for proto::SearchProject {}
impl ProjectRequest for proto::GetDocumentHighlights {}
impl ProjectRequest for proto::GetProjectSymbols {}
impl ProjectRequest for proto::OpenBufferForSymbol {}
impl ProjectRequest for proto::OpenBufferById {}
impl ProjectRequest for proto::SynchronizeBuffers {}
impl ProjectRequest for proto::InlayHints {}

impl ProjectRequest for proto::OpenBufferByPath {
    async fn handle_hosted_project_request(
        self,
        _hosted_project_id: HostedProjectId,
        response: Response<Self>,
        session: Session,
    ) -> Result<()> {
        let project_id = ProjectId(self.project_id as i32);
        let worktree_id = self.worktree_id as i32;
        let path = self.path.clone();

        let entry = session
            .db()
            .await
            .transaction({
                let path = &path;

                move |tx| async move {
                    Ok(worktree_entry::Entity::find()
                        .filter(
                            Condition::all()
                                .add(worktree_entry::Column::ProjectId.eq(project_id))
                                .add(worktree_entry::Column::WorktreeId.eq(worktree_id))
                                .add(worktree_entry::Column::Path.eq(path.clone())),
                        )
                        .one(&*tx)
                        .await?)
                }
            })
            .await?
            .ok_or_else(|| anyhow!("no such file"))?;

        response.send(OpenBufferResponse {
            buffer_id: entry.inode as u64,
        })?;

        session.peer.send(
            session.connection_id,
            CreateBufferForPeer {
                project_id: project_id.to_proto(),
                peer_id: None,
                variant: Some(proto::create_buffer_for_peer::Variant::State(
                    proto::BufferState {
                        id: entry.inode as u64,
                        file: Some(proto::File {
                            worktree_id: entry.worktree_id as u64,
                            entry_id: Some(entry.inode as u64),
                            path,
                            mtime: Some(proto::Timestamp {
                                seconds: entry.mtime_seconds as u64,
                                nanos: entry.mtime_nanos as u32,
                            }),
                            is_deleted: entry.is_deleted,
                        }),
                        base_text: "Hello world".to_string(),
                        diff_base: Some("Hello world".to_string()),
                        line_ending: proto::LineEnding::Unix.into(),
                        saved_version: vec![],
                        saved_version_fingerprint: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".into(),
                        saved_mtime: Some(proto::Timestamp {
                            seconds: entry.mtime_seconds as u64,
                            nanos: entry.mtime_nanos as u32,
                        }),
                    },
                )),
            },
        )?;

        session.peer.send(
            session.connection_id,
            CreateBufferForPeer {
                project_id: project_id.to_proto(),
                peer_id: None,
                variant: Some(create_buffer_for_peer::Variant::Chunk(proto::BufferChunk {
                    buffer_id: entry.inode as u64,
                    operations: vec![],
                    is_last: true,
                })),
            },
        )?;

        Ok(())
    }
}
