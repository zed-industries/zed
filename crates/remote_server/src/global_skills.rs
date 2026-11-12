use std::path::{Component, Path, PathBuf};
use std::sync::Arc;

use agent_skills::{GlobalSkillFileOperation, Skill, SkillLoadWarning, SkillSource};
use anyhow::{Context as _, Result, anyhow};
use fs::Fs;
use gpui::{AsyncApp, Context, Entity, Task};
use rpc::{
    AnyProtoClient, TypedEnvelope,
    proto::{self, REMOTE_SERVER_PROJECT_ID},
};
use util::ResultExt as _;

/// Serves the remote machine's `~/.agents/skills` to the client.
pub struct GlobalSkills {
    fs: Arc<dyn Fs>,
    session: AnyProtoClient,
    watch: Option<Task<()>>,
}

impl GlobalSkills {
    pub fn new(fs: Arc<dyn Fs>, session: AnyProtoClient, cx: &mut Context<Self>) -> Self {
        session.add_request_handler(cx.weak_entity(), Self::handle_get_global_skills);
        session.add_request_handler(cx.weak_entity(), Self::handle_create_global_skill);
        session.add_request_handler(cx.weak_entity(), Self::handle_trash_global_skill);
        session.add_request_handler(cx.weak_entity(), Self::handle_resolve_global_skill_path);
        session.add_request_handler(
            cx.weak_entity(),
            Self::handle_apply_global_skill_file_operation,
        );
        Self {
            fs,
            session,
            watch: None,
        }
    }

    async fn handle_get_global_skills(
        this: Entity<Self>,
        _: TypedEnvelope<proto::GetGlobalSkills>,
        mut cx: AsyncApp,
    ) -> Result<proto::GetGlobalSkillsResponse> {
        let fs = this.read_with(&cx, |this, _| this.fs.clone());
        let skills_dir = agent_skills::global_skills_dir();
        let mut response = proto::GetGlobalSkillsResponse::default();
        if !fs.is_dir(&skills_dir).await {
            return Ok(response);
        }
        this.update(&mut cx, |this, cx| this.watch(skills_dir.clone(), cx));

        // Only the frontmatter is sent. The agent reads a skill's body when it's used.
        let results =
            agent_skills::load_skills_from_directory(&fs, &skills_dir, SkillSource::Global).await;
        response.results = results
            .into_iter()
            .map(|result| proto::GlobalSkillLoadResult {
                variant: Some(match result {
                    Ok(skill) => proto::global_skill_load_result::Variant::Skill(
                        global_skill_to_proto(skill),
                    ),
                    Err(error) => proto::global_skill_load_result::Variant::Error(
                        proto::GlobalSkillLoadError {
                            path: error.path.to_string_lossy().into_owned(),
                            message: error.message,
                        },
                    ),
                }),
            })
            .collect();
        Ok(response)
    }

    /// Tells the client when global skills change, until the skills
    /// directory is removed. The next request watches it again.
    fn watch(&mut self, skills_dir: PathBuf, cx: &mut Context<Self>) {
        if self.watch.is_some() {
            return;
        }
        let fs = self.fs.clone();
        let session = self.session.clone();
        self.watch = Some(cx.spawn(async move |this, cx| {
            agent_skills::watch_skills_dir(fs, skills_dir, || {
                session
                    .send(proto::GlobalSkillsChanged {
                        project_id: REMOTE_SERVER_PROJECT_ID,
                    })
                    .log_err();
                true
            })
            .await;
            this.update(cx, |this, _| this.watch = None).ok();
        }));
    }

    async fn handle_create_global_skill(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::CreateGlobalSkill>,
        cx: AsyncApp,
    ) -> Result<proto::CreateGlobalSkillResponse> {
        let fs = this.read_with(&cx, |this, _| this.fs.clone());
        let payload = envelope.payload;
        agent_skills::validate_name(&payload.name).map_err(|error| anyhow!(error))?;
        let skill_file_path = agent_skills::write_skill_to_disk(
            fs.as_ref(),
            &agent_skills::global_skills_dir(),
            &payload.name,
            &payload.description,
            &payload.body,
            payload.disable_model_invocation,
        )
        .await?;
        Ok(proto::CreateGlobalSkillResponse {
            skill_file_path: skill_file_path.to_string_lossy().into_owned(),
        })
    }

    async fn handle_trash_global_skill(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::TrashGlobalSkill>,
        cx: AsyncApp,
    ) -> Result<proto::Ack> {
        let fs = this.read_with(&cx, |this, _| this.fs.clone());
        let directory_name = envelope.payload.directory_name;
        // Only a direct child of the skills directory may be trashed.
        let mut components = Path::new(&directory_name).components();
        anyhow::ensure!(
            matches!(
                (components.next(), components.next()),
                (Some(Component::Normal(_)), None)
            ),
            "invalid skill directory name {directory_name:?}"
        );
        fs.trash(
            &agent_skills::global_skills_dir().join(&directory_name),
            fs::RemoveOptions {
                recursive: true,
                ignore_if_not_exists: true,
            },
        )
        .await?;
        Ok(proto::Ack {})
    }

    async fn handle_resolve_global_skill_path(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::ResolveGlobalSkillPath>,
        cx: AsyncApp,
    ) -> Result<proto::ResolveGlobalSkillPathResponse> {
        let fs = this.read_with(&cx, |this, _| this.fs.clone());
        let payload = envelope.payload;
        let located = agent_skills::locate_global_skill_path(
            Path::new(&payload.path),
            fs.as_ref(),
            payload.may_not_exist,
        )
        .await;
        Ok(match located {
            Some(located) => proto::ResolveGlobalSkillPathResponse {
                path: Some(located.path.to_string_lossy().into_owned()),
                is_skills_dir: located.is_skills_dir,
                is_linked: located.is_linked,
            },
            None => proto::ResolveGlobalSkillPathResponse::default(),
        })
    }

    async fn handle_apply_global_skill_file_operation(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::ApplyGlobalSkillFileOperation>,
        cx: AsyncApp,
    ) -> Result<proto::Ack> {
        use proto::apply_global_skill_file_operation::Operation;

        let fs = this.read_with(&cx, |this, _| this.fs.clone());
        let operation = match envelope.payload.operation.context("missing operation")? {
            Operation::CreateDirectory(create) => GlobalSkillFileOperation::CreateDirectory {
                path: PathBuf::from(create.path),
            },
            Operation::Delete(delete) => GlobalSkillFileOperation::Delete {
                path: PathBuf::from(delete.path),
            },
            Operation::Copy(copy) => GlobalSkillFileOperation::Copy {
                source: PathBuf::from(copy.source),
                destination: PathBuf::from(copy.destination),
            },
            Operation::Rename(rename) => GlobalSkillFileOperation::Move {
                source: PathBuf::from(rename.source),
                destination: PathBuf::from(rename.destination),
            },
        };
        agent_skills::apply_global_skill_file_operation(fs.as_ref(), operation).await?;
        Ok(proto::Ack {})
    }
}

fn global_skill_to_proto(skill: Skill) -> proto::GlobalSkill {
    let mut description_too_long = None;
    for warning in skill.load_warnings {
        match warning {
            SkillLoadWarning::DescriptionTooLong {
                actual_len,
                max_len,
            } => {
                description_too_long = Some(proto::SkillDescriptionTooLong {
                    actual_len: actual_len as u64,
                    max_len: max_len as u64,
                });
            }
        }
    }
    proto::GlobalSkill {
        name: skill.name,
        description: skill.description,
        skill_file_path: skill.skill_file_path.to_string_lossy().into_owned(),
        directory_path: skill.directory_path.to_string_lossy().into_owned(),
        disable_model_invocation: skill.disable_model_invocation,
        description_too_long,
    }
}
