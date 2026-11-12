use std::path::{Path, PathBuf};
use std::sync::Arc;

use agent_skills::{
    GlobalSkillFileOperation, GlobalSkillPath, Skill, SkillLoadError, SkillLoadWarning, SkillSource,
};
use anyhow::{Context as _, Result, anyhow};
use client::{AnyProtoClient, proto};
use fs::Fs;
use futures::StreamExt as _;
use gpui::{App, AppContext as _, AsyncApp, Entity, Task};
use project::Project;
use util::{ResultExt as _, paths::PathStyle};

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
        response
            .results
            .into_iter()
            .filter_map(|result| match result.variant? {
                proto::global_skill_load_result::Variant::Skill(skill) => Some(Ok(Skill {
                    name: skill.name,
                    description: skill.description,
                    source: SkillSource::Global,
                    directory_path: PathBuf::from(skill.directory_path),
                    skill_file_path: PathBuf::from(skill.skill_file_path),
                    load_warnings: skill
                        .description_too_long
                        .map(|warning| SkillLoadWarning::DescriptionTooLong {
                            actual_len: warning.actual_len as usize,
                            max_len: warning.max_len as usize,
                        })
                        .into_iter()
                        .collect(),
                    disable_model_invocation: skill.disable_model_invocation,
                    embedded_body: None,
                })),
                proto::global_skill_load_result::Variant::Error(error) => {
                    Some(Err(SkillLoadError {
                        path: PathBuf::from(error.path),
                        message: error.message,
                    }))
                }
            })
            .collect()
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

/// The files of a project's global skills, on whichever machine they're on.
/// Agent tools reach `~/.agents/skills` through this, since it's outside any
/// worktree.
#[derive(Clone)]
pub enum GlobalSkillFiles {
    Local(Arc<dyn Fs>),
    Remote(Entity<Project>),
}

/// An entry of a directory in the global skills directory.
pub struct GlobalSkillDirectoryEntry {
    pub path: PathBuf,
    pub is_dir: bool,
}

impl GlobalSkillFiles {
    pub fn for_project(project: &Entity<Project>, cx: &App) -> Self {
        let project_ref = project.read(cx);
        if global_skills_are_remote(project_ref) {
            Self::Remote(project.clone())
        } else {
            Self::Local(project_ref.fs().clone())
        }
    }

    /// The path style of the machine the skills are on. A collab guest's
    /// skills are local while its project has the host's path style, so this
    /// isn't always the project's.
    pub fn path_style(&self, cx: &App) -> PathStyle {
        match self {
            Self::Local(_) => PathStyle::local(),
            Self::Remote(project) => project.read(cx).path_style(cx),
        }
    }

    /// The canonical path of `path`, if it names an existing file or directory
    /// in the global skills directory, or the directory itself.
    pub async fn resolve(&self, path: &Path, cx: &mut AsyncApp) -> Option<PathBuf> {
        Some(self.locate(path, false, cx).await?.path)
    }

    /// Like [`Self::resolve`], for a path that may not exist yet.
    pub async fn resolve_creatable(&self, path: &Path, cx: &mut AsyncApp) -> Option<PathBuf> {
        Some(self.locate(path, true, cx).await?.path)
    }

    /// Like [`Self::resolve`], but only for a path strictly inside the global
    /// skills directory on disk: not the directory itself, nor a path in a
    /// skill directory that links elsewhere.
    pub async fn resolve_descendant(&self, path: &Path, cx: &mut AsyncApp) -> Option<PathBuf> {
        self.locate(path, false, cx).await?.into_descendant()
    }

    /// Like [`Self::resolve_descendant`], for a path that may not exist yet.
    pub async fn resolve_creatable_descendant(
        &self,
        path: &Path,
        cx: &mut AsyncApp,
    ) -> Option<PathBuf> {
        self.locate(path, true, cx).await?.into_descendant()
    }

    /// Whether `path` names the global skills directory itself, which
    /// destructive tools refuse to act on.
    pub async fn is_skills_dir(&self, path: &Path, cx: &mut AsyncApp) -> bool {
        self.locate(path, true, cx)
            .await
            .is_some_and(|located| located.is_skills_dir)
    }

    async fn locate(
        &self,
        path: &Path,
        may_not_exist: bool,
        cx: &mut AsyncApp,
    ) -> Option<GlobalSkillPath> {
        match self {
            Self::Local(fs) => {
                agent_skills::locate_global_skill_path(path, fs.as_ref(), may_not_exist).await
            }
            Self::Remote(project) => {
                // Most tool calls name project files, so skip the round trip
                // for paths that can't be under any home's skills dir.
                if !mentions_skills_dir(path) {
                    return None;
                }
                let client = cx
                    .update(|cx| remote_server_client(project, cx))
                    .log_err()?;
                let response = client
                    .request(proto::ResolveGlobalSkillPath {
                        project_id: proto::REMOTE_SERVER_PROJECT_ID,
                        path: path.to_string_lossy().into_owned(),
                        may_not_exist,
                    })
                    .await
                    .log_err()?;
                Some(GlobalSkillPath {
                    path: PathBuf::from(response.path?),
                    is_skills_dir: response.is_skills_dir,
                    is_linked: response.is_linked,
                })
            }
        }
    }

    /// Whether `path` is a directory, or `None` if nothing is there.
    pub async fn is_dir(&self, path: &Path, cx: &mut AsyncApp) -> Result<Option<bool>> {
        match self {
            Self::Local(fs) => Ok(fs.metadata(path).await?.map(|metadata| metadata.is_dir)),
            Self::Remote(project) => {
                let client = cx.update(|cx| remote_server_client(project, cx))?;
                let response = client
                    .request(proto::GetPathMetadata {
                        project_id: proto::REMOTE_SERVER_PROJECT_ID,
                        path: path.to_string_lossy().into_owned(),
                        canonicalize: false,
                    })
                    .await?;
                Ok(response.exists.then_some(response.is_dir))
            }
        }
    }

    pub async fn load(&self, path: &Path, cx: &mut AsyncApp) -> Result<String> {
        match self {
            Self::Local(fs) => fs.load(path).await,
            Self::Remote(project) => {
                let buffer = project
                    .update(cx, |project, cx| project.open_local_buffer(path, cx))
                    .await?;
                Ok(buffer.read_with(cx, |buffer, _| buffer.text()))
            }
        }
    }

    pub async fn list_directory(
        &self,
        path: &Path,
        cx: &mut AsyncApp,
    ) -> Result<Vec<GlobalSkillDirectoryEntry>> {
        match self {
            Self::Local(fs) => {
                let mut paths = fs.read_dir(path).await?;
                let mut entries = Vec::new();
                while let Some(path) = paths.next().await {
                    let Ok(path) = path else {
                        continue;
                    };
                    // A dangling symlink has no metadata, so it's skipped.
                    let Ok(Some(metadata)) = fs.metadata(&path).await else {
                        continue;
                    };
                    entries.push(GlobalSkillDirectoryEntry {
                        path,
                        is_dir: metadata.is_dir,
                    });
                }
                Ok(entries)
            }
            Self::Remote(project) => {
                let (items, path_style) = project.update(cx, |project, cx| {
                    (
                        project.list_directory(path.to_string_lossy().into_owned(), cx),
                        project.path_style(cx),
                    )
                });
                Ok(items
                    .await?
                    .into_iter()
                    .map(|item| GlobalSkillDirectoryEntry {
                        path: path_style.join(path, item.path),
                        is_dir: item.is_dir,
                    })
                    .collect())
            }
        }
    }

    pub async fn apply(
        &self,
        operation: GlobalSkillFileOperation,
        cx: &mut AsyncApp,
    ) -> Result<()> {
        use proto::apply_global_skill_file_operation as operation_proto;

        match self {
            Self::Local(fs) => {
                agent_skills::apply_global_skill_file_operation(fs.as_ref(), operation).await
            }
            Self::Remote(project) => {
                let client = cx.update(|cx| remote_server_client(project, cx))?;
                let path = |path: PathBuf| path.to_string_lossy().into_owned();
                let operation = match operation {
                    GlobalSkillFileOperation::CreateDirectory { path: directory } => {
                        operation_proto::Operation::CreateDirectory(
                            operation_proto::CreateDirectory {
                                path: path(directory),
                            },
                        )
                    }
                    GlobalSkillFileOperation::Delete { path: deleted } => {
                        operation_proto::Operation::Delete(operation_proto::Delete {
                            path: path(deleted),
                        })
                    }
                    GlobalSkillFileOperation::Copy {
                        source,
                        destination,
                    } => operation_proto::Operation::Copy(operation_proto::Copy {
                        source: path(source),
                        destination: path(destination),
                    }),
                    GlobalSkillFileOperation::Move {
                        source,
                        destination,
                    } => operation_proto::Operation::Move(operation_proto::Move {
                        source: path(source),
                        destination: path(destination),
                    }),
                };
                client
                    .request(proto::ApplyGlobalSkillFileOperation {
                        project_id: proto::REMOTE_SERVER_PROJECT_ID,
                        operation: Some(operation),
                    })
                    .await?;
                Ok(())
            }
        }
    }
}

/// Whether `path` could be under a `.agents/skills` directory. It can't be
/// without naming both, however it spells the rest.
fn mentions_skills_dir(path: &Path) -> bool {
    let path = path.to_string_lossy();
    path.contains(agent_skills::AGENTS_DIR_NAME) && path.contains(agent_skills::SKILLS_DIR_NAME)
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::TestAppContext;

    #[gpui::test]
    fn test_local_global_skill_files_use_the_local_path_style(cx: &mut TestAppContext) {
        let global_skill_files = GlobalSkillFiles::Local(fs::FakeFs::new(cx.executor()));
        cx.update(|cx| assert_eq!(global_skill_files.path_style(cx), PathStyle::local()));
    }
}
