use std::{collections::HashMap, fmt, ops::Not, rc::Rc};

use anyhow::Result;
use derive_more::Deref;
use serde::Deserialize;

use crate::git::CommitSha;

pub const PR_REVIEW_LABEL: &str = "PR state:needs review";

#[derive(Debug, Clone)]
pub struct GitHubUser {
    pub login: String,
}

#[derive(Debug, Clone)]
pub struct PullRequestData {
    pub number: u64,
    pub user: Option<GitHubUser>,
    pub merged_by: Option<GitHubUser>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReviewState {
    Approved,
    Other,
}

#[derive(Debug, Clone)]
pub struct PullRequestReview {
    pub user: Option<GitHubUser>,
    pub state: Option<ReviewState>,
    pub body: Option<String>,
}

impl PullRequestReview {
    pub fn with_body(self, body: impl ToString) -> Self {
        Self {
            body: Some(body.to_string()),
            ..self
        }
    }
}

#[derive(Debug, Clone)]
pub struct PullRequestComment {
    pub user: GitHubUser,
    pub body: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Deref, PartialEq, Eq)]
pub struct GithubLogin {
    login: String,
}

impl GithubLogin {
    pub fn new(login: String) -> Self {
        Self { login }
    }
}

impl fmt::Display for GithubLogin {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "@{}", self.login)
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct CommitAuthor {
    name: String,
    email: String,
    user: Option<GithubLogin>,
}

impl CommitAuthor {
    pub(crate) fn user(&self) -> Option<&GithubLogin> {
        self.user.as_ref()
    }
}

impl PartialEq for CommitAuthor {
    fn eq(&self, other: &Self) -> bool {
        self.user.as_ref().zip(other.user.as_ref()).map_or_else(
            || self.email == other.email || self.name == other.name,
            |(l, r)| l == r,
        )
    }
}

impl fmt::Display for CommitAuthor {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.user.as_ref() {
            Some(user) => write!(formatter, "{} ({user})", self.name),
            None => write!(formatter, "{} ({})", self.name, self.email),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CommitAuthors {
    #[serde(rename = "author")]
    primary_author: CommitAuthor,
    #[serde(rename = "authors")]
    co_authors: Vec<CommitAuthor>,
}

impl CommitAuthors {
    pub fn co_authors(&self) -> Option<impl Iterator<Item = &CommitAuthor>> {
        self.co_authors.is_empty().not().then(|| {
            self.co_authors
                .iter()
                .filter(|co_author| *co_author != &self.primary_author)
        })
    }
}

#[derive(Debug, Deserialize, Deref)]
pub struct AuthorsForCommits(HashMap<CommitSha, CommitAuthors>);

#[async_trait::async_trait(?Send)]
pub trait GitHubApiClient {
    async fn get_pull_request(&self, pr_number: u64) -> Result<PullRequestData>;
    async fn get_pull_request_reviews(&self, pr_number: u64) -> Result<Vec<PullRequestReview>>;
    async fn get_pull_request_comments(&self, pr_number: u64) -> Result<Vec<PullRequestComment>>;
    async fn get_commit_authors(&self, commit_shas: &[&CommitSha]) -> Result<AuthorsForCommits>;
    async fn check_org_membership(&self, login: &GithubLogin) -> Result<bool>;
    async fn check_repo_write_permission(&self, login: &GithubLogin) -> Result<bool>;
    async fn actor_has_repository_write_permission(
        &self,
        login: &GithubLogin,
    ) -> anyhow::Result<bool> {
        Ok(self.check_org_membership(login).await?
            || self.check_repo_write_permission(login).await?)
    }
    async fn ensure_pull_request_has_label(&self, label: &str, pr_number: u64) -> Result<()>;
}

#[derive(Deref)]
pub struct GitHubClient {
    api: Rc<dyn GitHubApiClient>,
}

impl GitHubClient {
    pub fn new(api: Rc<dyn GitHubApiClient>) -> Self {
        Self { api }
    }

    #[cfg(feature = "octo-client")]
    pub async fn for_app(app_id: u64, app_private_key: &str) -> Result<Self> {
        let client = OctocrabClient::new(app_id, app_private_key).await?;
        Ok(Self::new(Rc::new(client)))
    }
}

#[cfg(feature = "octo-client")]
mod octo_client {
    use anyhow::{Context, Result};
    use futures::TryStreamExt as _;
    use itertools::Itertools;
    use jsonwebtoken::EncodingKey;
    use octocrab::{
        Octocrab, Page, models::pulls::ReviewState as OctocrabReviewState,
        service::middleware::cache::mem::InMemoryCache,
    };
    use serde::de::DeserializeOwned;
    use tokio::pin;

    use crate::git::CommitSha;

    use super::{
        AuthorsForCommits, GitHubApiClient, GitHubUser, GithubLogin, PullRequestComment,
        PullRequestData, PullRequestReview, ReviewState,
    };

    const PAGE_SIZE: u8 = 100;
    const ORG: &str = "zed-industries";
    const REPO: &str = "zed";

    pub struct OctocrabClient {
        client: Octocrab,
    }

    impl OctocrabClient {
        pub async fn new(app_id: u64, app_private_key: &str) -> Result<Self> {
            let octocrab = Octocrab::builder()
                .cache(InMemoryCache::new())
                .app(
                    app_id.into(),
                    EncodingKey::from_rsa_pem(app_private_key.as_bytes())?,
                )
                .build()?;

            let installations = octocrab
                .apps()
                .installations()
                .send()
                .await
                .context("Failed to fetch installations")?
                .take_items();

            let installation_id = installations
                .into_iter()
                .find(|installation| installation.account.login == ORG)
                .context("Could not find Zed repository in installations")?
                .id;

            let client = octocrab.installation(installation_id)?;
            Ok(Self { client })
        }

        fn build_co_authors_query<'a>(shas: impl IntoIterator<Item = &'a CommitSha>) -> String {
            const FRAGMENT: &str = r#"
                ... on Commit {
                    author {
                        name
                        email
                        user { login }
                    }
                    authors(first: 10) {
                        nodes {
                            name
                            email
                            user { login }
                        }
                    }
                }
            "#;

            let objects: String = shas
                .into_iter()
                .map(|commit_sha| {
                    format!(
                        "commit{sha}: object(oid: \"{sha}\") {{ {FRAGMENT} }}",
                        sha = **commit_sha
                    )
                })
                .join("\n");

            format!("{{  repository(owner: \"{ORG}\", name: \"{REPO}\") {{ {objects}  }} }}")
                .replace("\n", "")
        }

        async fn graphql<R: octocrab::FromResponse>(
            &self,
            query: &serde_json::Value,
        ) -> octocrab::Result<R> {
            self.client.graphql(query).await
        }

        async fn get_all<T: DeserializeOwned + 'static>(
            &self,
            page: Page<T>,
        ) -> octocrab::Result<Vec<T>> {
            self.get_filtered(page, |_| true).await
        }

        async fn get_filtered<T: DeserializeOwned + 'static>(
            &self,
            page: Page<T>,
            predicate: impl Fn(&T) -> bool,
        ) -> octocrab::Result<Vec<T>> {
            let stream = page.into_stream(&self.client);
            pin!(stream);

            let mut results = Vec::new();

            while let Some(item) = stream.try_next().await?
                && predicate(&item)
            {
                results.push(item);
            }

            Ok(results)
        }
    }

    #[async_trait::async_trait(?Send)]
    impl GitHubApiClient for OctocrabClient {
        async fn get_pull_request(&self, pr_number: u64) -> Result<PullRequestData> {
            let pr = self.client.pulls(ORG, REPO).get(pr_number).await?;
            Ok(PullRequestData {
                number: pr.number,
                user: pr.user.map(|user| GitHubUser { login: user.login }),
                merged_by: pr.merged_by.map(|user| GitHubUser { login: user.login }),
            })
        }

        async fn get_pull_request_reviews(&self, pr_number: u64) -> Result<Vec<PullRequestReview>> {
            let page = self
                .client
                .pulls(ORG, REPO)
                .list_reviews(pr_number)
                .per_page(PAGE_SIZE)
                .send()
                .await?;

            let reviews = self.get_all(page).await?;

            Ok(reviews
                .into_iter()
                .map(|review| PullRequestReview {
                    user: review.user.map(|user| GitHubUser { login: user.login }),
                    state: review.state.map(|state| match state {
                        OctocrabReviewState::Approved => ReviewState::Approved,
                        _ => ReviewState::Other,
                    }),
                    body: review.body,
                })
                .collect())
        }

        async fn get_pull_request_comments(
            &self,
            pr_number: u64,
        ) -> Result<Vec<PullRequestComment>> {
            let page = self
                .client
                .issues(ORG, REPO)
                .list_comments(pr_number)
                .per_page(PAGE_SIZE)
                .send()
                .await?;

            let comments = self.get_all(page).await?;

            Ok(comments
                .into_iter()
                .map(|comment| PullRequestComment {
                    user: GitHubUser {
                        login: comment.user.login,
                    },
                    body: comment.body,
                })
                .collect())
        }

        async fn get_commit_authors(
            &self,
            commit_shas: &[&CommitSha],
        ) -> Result<AuthorsForCommits> {
            let query = Self::build_co_authors_query(commit_shas.iter().copied());
            let query = serde_json::json!({ "query": query });
            let mut response = self.graphql::<serde_json::Value>(&query).await?;

            response
                .get_mut("data")
                .and_then(|data| data.get_mut("repository"))
                .and_then(|repo| repo.as_object_mut())
                .ok_or_else(|| anyhow::anyhow!("Unexpected response format!"))
                .and_then(|commit_data| {
                    let mut response_map = serde_json::Map::with_capacity(commit_data.len());

                    for (key, value) in commit_data.iter_mut() {
                        let key_without_prefix = key.strip_prefix("commit").unwrap_or(key);
                        if let Some(authors) = value.get_mut("authors") {
                            if let Some(nodes) = authors.get("nodes") {
                                *authors = nodes.clone();
                            }
                        }

                        response_map.insert(key_without_prefix.to_owned(), value.clone());
                    }

                    serde_json::from_value(serde_json::Value::Object(response_map))
                        .context("Failed to deserialize commit authors")
                })
        }

        async fn check_org_membership(&self, login: &GithubLogin) -> Result<bool> {
            let page = self
                .client
                .orgs(ORG)
                .list_members()
                .per_page(PAGE_SIZE)
                .send()
                .await?;

            let members = self.get_all(page).await?;

            Ok(members
                .into_iter()
                .any(|member| member.login == login.as_str()))
        }

        async fn check_repo_write_permission(&self, login: &GithubLogin) -> Result<bool> {
            // TODO: octocrab fails to deserialize the permission response and
            // does not adhere to the scheme laid out at
            // https://docs.github.com/en/rest/collaborators/collaborators?apiVersion=2026-03-10#get-repository-permissions-for-a-user

            #[derive(serde::Deserialize)]
            #[serde(rename_all = "lowercase")]
            enum RepoPermission {
                Admin,
                Write,
                Read,
                #[serde(other)]
                Other,
            }

            #[derive(serde::Deserialize)]
            struct RepositoryPermissions {
                permission: RepoPermission,
            }

            self.client
                .get::<RepositoryPermissions, _, _>(
                    format!(
                        "/repos/{ORG}/{REPO}/collaborators/{user}/permission",
                        user = login.as_str()
                    ),
                    None::<&()>,
                )
                .await
                .map(|response| {
                    matches!(
                        response.permission,
                        RepoPermission::Write | RepoPermission::Admin
                    )
                })
                .map_err(Into::into)
        }

        async fn ensure_pull_request_has_label(&self, label: &str, pr_number: u64) -> Result<()> {
            if self
                .get_filtered(
                    self.client
                        .issues(ORG, REPO)
                        .list_labels_for_issue(pr_number)
                        .per_page(PAGE_SIZE)
                        .send()
                        .await?,
                    |pr_label| pr_label.name == label,
                )
                .await
                .is_ok_and(|l| l.is_empty())
            {
                self.client
                    .issues(ORG, REPO)
                    .add_labels(pr_number, &[label.to_owned()])
                    .await?;
            }

            Ok(())
        }
    }
}

#[cfg(feature = "octo-client")]
pub use octo_client::OctocrabClient;
