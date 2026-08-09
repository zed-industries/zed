use anyhow::{Context as _, Result};
use collections::HashMap;
use gpui::{App, AppContext as _, Entity, Task};
use project::ProjectEnvironment;
use serde::Deserialize;
use std::{path::PathBuf, sync::Arc};
use task::Shell;

/// The `gh` CLI could not be found on the project's PATH.
#[derive(Debug)]
pub struct GhNotInstalledError;

impl std::fmt::Display for GhNotInstalledError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "GitHub CLI (gh) not found")
    }
}

impl std::error::Error for GhNotInstalledError {}

/// The `gh` CLI is installed but not signed in to GitHub.
#[derive(Debug)]
pub struct GhNotAuthenticatedError {
    pub message: String,
}

impl std::fmt::Display for GhNotAuthenticatedError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.message)
    }
}

impl std::error::Error for GhNotAuthenticatedError {}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct IssueAuthor {
    pub login: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct IssueLabel {
    pub name: String,
    /// Hex color without a leading `#`, e.g. `d73a4a`.
    pub color: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct IssueSummary {
    pub number: u64,
    pub title: String,
    /// `None` for deleted ("ghost") users.
    pub author: Option<IssueAuthor>,
    pub state: String,
    pub created_at: String,
    pub updated_at: String,
    pub labels: Vec<IssueLabel>,
    pub url: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct IssueComment {
    pub author: Option<IssueAuthor>,
    pub body: String,
    pub created_at: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct IssueDetails {
    pub number: u64,
    pub title: String,
    pub body: String,
    pub author: Option<IssueAuthor>,
    pub state: String,
    pub created_at: String,
    pub updated_at: String,
    pub labels: Vec<IssueLabel>,
    pub url: String,
    pub comments: Vec<IssueComment>,
}

/// Runs the `gh` CLI in a repository's working directory.
///
/// Authentication is entirely `gh`'s job: the client runs with the shell
/// environment resolved for the repository directory, so whatever credentials
/// `gh auth login` stored (or a `GH_TOKEN` in the environment) apply here too.
#[derive(Clone)]
pub struct GithubIssuesClient {
    gh_binary_path: PathBuf,
    environment: Arc<HashMap<String, String>>,
    repo_workdir: PathBuf,
}

impl GithubIssuesClient {
    pub fn open(
        project_environment: Entity<ProjectEnvironment>,
        repo_workdir: PathBuf,
        cx: &mut App,
    ) -> Task<Result<Self>> {
        let environment = project_environment.update(cx, |project_environment, cx| {
            project_environment.local_directory_environment(
                &Shell::System,
                repo_workdir.as_path().into(),
                cx,
            )
        });
        cx.background_spawn(async move {
            let environment = Arc::new(environment.await.unwrap_or_else(|| {
                log::error!("no directory environment for repository {repo_workdir:?}");
                HashMap::default()
            }));
            // The app process PATH often lacks the user's shell PATH (e.g.
            // homebrew's bin on macOS), so search the resolved environment
            // first and only then the process environment.
            let gh_binary_path = which::which_in(
                "gh",
                environment.get("PATH").filter(|path| !path.is_empty()),
                &repo_workdir,
            )
            .or_else(|_| which::which("gh"))
            .map_err(|_| anyhow::Error::new(GhNotInstalledError))?;
            Ok(Self {
                gh_binary_path,
                environment,
                repo_workdir,
            })
        })
    }

    /// The open issues of the repository's GitHub remote, newest first.
    ///
    /// Capped at 100 issues; pagination is out of scope for now.
    pub fn list_open_issues(&self, cx: &App) -> Task<Result<Vec<IssueSummary>>> {
        let client = self.clone();
        cx.background_spawn(async move {
            let stdout = client
                .run_gh(&[
                    "issue",
                    "list",
                    "--state",
                    "open",
                    "--limit",
                    "100",
                    "--json",
                    "number,title,author,state,createdAt,updatedAt,labels,url",
                ])
                .await?;
            serde_json::from_slice(&stdout).context("parsing `gh issue list` output")
        })
    }

    pub fn load_issue(&self, number: u64, cx: &App) -> Task<Result<IssueDetails>> {
        let client = self.clone();
        cx.background_spawn(async move {
            let stdout = client
                .run_gh(&[
                    "issue",
                    "view",
                    &number.to_string(),
                    "--json",
                    "number,title,body,author,state,createdAt,updatedAt,labels,url,comments",
                ])
                .await?;
            serde_json::from_slice(&stdout).context("parsing `gh issue view` output")
        })
    }

    async fn run_gh(&self, args: &[&str]) -> Result<Vec<u8>> {
        let output = util::command::new_command(&self.gh_binary_path)
            .args(args)
            .envs(self.environment.iter())
            .current_dir(&self.repo_workdir)
            .output()
            .await
            .with_context(|| format!("running {:?}", self.gh_binary_path))?;
        if output.status.success() {
            Ok(output.stdout)
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
            if stderr.contains("gh auth login") || stderr.contains("not logged in") {
                Err(anyhow::Error::new(GhNotAuthenticatedError {
                    message: stderr,
                }))
            } else {
                anyhow::bail!("gh exited with {}: {stderr}", output.status)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_issue_list() {
        let json = r#"[
            {
                "number": 42,
                "title": "Crash when saving large files",
                "author": {"id": "U_abc", "is_bot": false, "login": "octocat", "name": "Octo Cat"},
                "state": "OPEN",
                "createdAt": "2026-07-01T12:00:00Z",
                "updatedAt": "2026-08-01T09:30:00Z",
                "labels": [{"id": "L_1", "name": "bug", "description": "", "color": "d73a4a"}],
                "url": "https://github.com/octocat/hello/issues/42"
            },
            {
                "number": 41,
                "title": "Issue from a deleted user",
                "author": null,
                "state": "OPEN",
                "createdAt": "2026-06-01T12:00:00Z",
                "updatedAt": "2026-06-02T12:00:00Z",
                "labels": [],
                "url": "https://github.com/octocat/hello/issues/41"
            }
        ]"#;
        let issues: Vec<IssueSummary> = serde_json::from_slice(json.as_bytes()).unwrap();
        assert_eq!(issues.len(), 2);
        assert_eq!(issues[0].number, 42);
        assert_eq!(issues[0].author.as_ref().unwrap().login, "octocat");
        assert_eq!(issues[0].labels[0].name, "bug");
        assert_eq!(issues[0].labels[0].color, "d73a4a");
        assert_eq!(issues[1].author, None);
        assert!(issues[1].labels.is_empty());
    }

    #[test]
    fn test_parse_issue_details() {
        let json = r###"{
            "number": 42,
            "title": "Crash when saving large files",
            "body": "## Steps\n\n1. Open a large file\n2. Save",
            "author": {"login": "octocat"},
            "state": "OPEN",
            "createdAt": "2026-07-01T12:00:00Z",
            "updatedAt": "2026-08-01T09:30:00Z",
            "labels": [{"name": "bug", "color": "d73a4a"}],
            "url": "https://github.com/octocat/hello/issues/42",
            "comments": [
                {
                    "id": "IC_1",
                    "author": {"login": "hubot"},
                    "authorAssociation": "MEMBER",
                    "body": "Reproduced on main.",
                    "createdAt": "2026-07-02T08:00:00Z",
                    "includesCreatedEdit": false,
                    "reactionGroups": []
                }
            ]
        }"###;
        let details: IssueDetails = serde_json::from_slice(json.as_bytes()).unwrap();
        assert_eq!(details.number, 42);
        assert!(details.body.contains("## Steps"));
        assert_eq!(details.comments.len(), 1);
        assert_eq!(details.comments[0].author.as_ref().unwrap().login, "hubot");
    }
}
