use std::path::{Path, PathBuf};

use agent_skills::{Skill, SkillLoadError, SkillSource};
use anyhow::{Context as _, Result, anyhow};
use client::{AnyProtoClient, proto};
use gpui::{App, AppContext as _, Entity, Task};
use project::Project;

/// In a remote project, global skills come from the remote machine's
/// `~/.agents/skills`, not the local one. The remote server loads and watches
/// them, and tells the project when they change.
pub fn global_skills_are_remote(project: &Project) -> bool {
    project.is_via_remote_server()
}

fn remote_server_client(project: &Entity<Project>, cx: &App) -> Result<AnyProtoClient> {
    let remote_client = project
        .read(cx)
        .remote_client()
        .context("project has no remote server")?;
    Ok(remote_client.read(cx).proto_client())
}

pub fn load_remote_global_skills(
    project: &Entity<Project>,
    cx: &App,
) -> Task<Vec<Result<Skill, SkillLoadError>>> {
    let request = match remote_server_client(project, cx) {
        Ok(client) => client.request(proto::GetGlobalSkills {
            project_id: proto::REMOTE_SERVER_PROJECT_ID,
        }),
        Err(error) => return Task::ready(vec![Err(remote_skills_load_error(error))]),
    };
    cx.background_spawn(async move {
        let response = match request.await {
            Ok(response) => response,
            Err(error) => return vec![Err(remote_skills_load_error(error))],
        };
        let skills = response.files.into_iter().map(|file| {
            let skill_file_path = PathBuf::from(file.path);
            agent_skills::parse_skill_frontmatter(
                &skill_file_path,
                &file.content,
                SkillSource::Global,
            )
            .map_err(|error| SkillLoadError {
                path: skill_file_path,
                message: error.to_string(),
            })
        });
        let errors = response.errors.into_iter().map(|error| {
            Err(SkillLoadError {
                path: PathBuf::from(error.path),
                message: error.message,
            })
        });
        skills.chain(errors).collect()
    })
}

fn remote_skills_load_error(error: anyhow::Error) -> SkillLoadError {
    SkillLoadError {
        path: PathBuf::from(format!(
            "~/{}/{}",
            agent_skills::AGENTS_DIR_NAME,
            agent_skills::SKILLS_DIR_NAME
        )),
        message: format!("Failed to load global skills from the remote machine: {error:#}"),
    }
}

/// Writes a new skill to the remote's `~/.agents/skills`, creating the
/// directory if needed. Returns the new `SKILL.md` path.
pub fn create_remote_global_skill(
    project: &Entity<Project>,
    name: &str,
    description: &str,
    body: &str,
    disable_model_invocation: bool,
    cx: &App,
) -> Task<Result<PathBuf>> {
    let client = match remote_server_client(project, cx) {
        Ok(client) => client,
        Err(error) => return Task::ready(Err(error)),
    };
    let request = client.request(proto::CreateGlobalSkill {
        project_id: proto::REMOTE_SERVER_PROJECT_ID,
        name: name.to_string(),
        description: description.to_string(),
        body: body.to_string(),
        disable_model_invocation,
    });
    cx.background_spawn(async move { Ok(PathBuf::from(request.await?.skill_file_path)) })
}

/// Moves a skill directory in the remote's `~/.agents/skills` to the trash.
pub fn trash_remote_global_skill(
    project: &Entity<Project>,
    directory_path: &Path,
    cx: &App,
) -> Task<Result<()>> {
    let path_style = project.read(cx).path_style(cx);
    let Some(directory_name) = path_style
        .file_name(directory_path)
        .map(|name| name.to_string_lossy().into_owned())
    else {
        return Task::ready(Err(anyhow!(
            "{} is not a skill directory",
            directory_path.display()
        )));
    };
    let client = match remote_server_client(project, cx) {
        Ok(client) => client,
        Err(error) => return Task::ready(Err(error)),
    };
    let request = client.request(proto::TrashGlobalSkill {
        project_id: proto::REMOTE_SERVER_PROJECT_ID,
        directory_name,
    });
    cx.background_spawn(async move {
        request.await?;
        Ok(())
    })
}
