use std::path::{Component, Path, PathBuf};
use std::sync::Arc;

use anyhow::{Result, anyhow};
use fs::Fs;
use gpui::{AppContext as _, AsyncApp, Context, Entity, Task};
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
    pub fn new(fs: Arc<dyn Fs>, session: AnyProtoClient) -> Self {
        Self {
            fs,
            session,
            watch: None,
        }
    }

    pub fn init(this: &Entity<Self>, session: &AnyProtoClient) {
        session.add_request_handler(this.downgrade(), Self::handle_get_global_skills);
        session.add_request_handler(this.downgrade(), Self::handle_create_global_skill);
        session.add_request_handler(this.downgrade(), Self::handle_trash_global_skill);
    }

    async fn handle_get_global_skills(
        this: Entity<Self>,
        _: TypedEnvelope<proto::GetGlobalSkills>,
        mut cx: AsyncApp,
    ) -> Result<proto::GetGlobalSkillsResponse> {
        let fs = cx.read_entity(&this, |this, _| this.fs.clone());
        let skills_dir = agent_skills::global_skills_dir();
        let mut response = proto::GetGlobalSkillsResponse::default();
        if !fs.is_dir(&skills_dir).await {
            return Ok(response);
        }
        this.update(&mut cx, |this, cx| this.watch(skills_dir.clone(), cx));

        // The client parses these, so remote skills load like local ones.
        let mut skill_file_paths = agent_skills::find_skill_files(&fs, &skills_dir).await;
        skill_file_paths.sort();
        for skill_file_path in skill_file_paths {
            match agent_skills::read_skill_file(fs.as_ref(), &skill_file_path).await {
                Ok(content) => response.files.push(proto::GlobalSkillFile {
                    path: skill_file_path.to_string_lossy().into_owned(),
                    content,
                }),
                Err(error) => response.errors.push(proto::GlobalSkillLoadError {
                    path: error.path.to_string_lossy().into_owned(),
                    message: error.message,
                }),
            }
        }
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
        let fs = cx.read_entity(&this, |this, _| this.fs.clone());
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
        let fs = cx.read_entity(&this, |this, _| this.fs.clone());
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
}
