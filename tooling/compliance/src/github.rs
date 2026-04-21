use std::{borrow::Cow, collections::HashMap, fmt, ops::Not, rc::Rc, sync::LazyLock};

use anyhow::Result;
use derive_more::Deref;
use serde::Deserialize;

use crate::git::CommitSha;

pub const PR_REVIEW_LABEL: &str = "PR state:needs review";

#[derive(Debug, Clone)]
pub struct GithubUser {
    pub login: String,
}

#[derive(Debug, Clone)]
pub struct PullRequestData {
    pub number: u64,
    pub user: Option<GithubUser>,
    pub merged_by: Option<GithubUser>,
    pub labels: Option<Vec<String>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReviewState {
    Approved,
    Other,
}

#[derive(Debug, Clone)]
pub struct PullRequestReview {
    pub user: Option<GithubUser>,
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
    pub user: GithubUser,
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

pub(crate) static ZED_ZIPPY_AUTHOR: LazyLock<CommitAuthor> = LazyLock::new(|| CommitAuthor {
    name: "Zed Zippy".to_string(),
    email: "234243425+zed-zippy[bot]@users.noreply.github.com".to_string(),
    user: Some(GithubLogin {
        login: "zed-zippy[bot]".to_string(),
    }),
});

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
    #[serde(rename = "authors", deserialize_with = "graph_ql::deserialize_nodes")]
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

#[derive(Debug, Deref)]
pub struct AuthorsForCommits(HashMap<CommitSha, CommitAuthors>);

impl AuthorsForCommits {
    const SHA_PREFIX: &'static str = "commit";
}

impl<'de> serde::Deserialize<'de> for AuthorsForCommits {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let raw = HashMap::<String, CommitAuthors>::deserialize(deserializer)?;
        let map = raw
            .into_iter()
            .map(|(key, value)| {
                let sha = key
                    .strip_prefix(AuthorsForCommits::SHA_PREFIX)
                    .unwrap_or(&key);
                (CommitSha::new(sha.to_owned()), value)
            })
            .collect();
        Ok(Self(map))
    }
}

#[derive(Clone)]
pub struct Repository<'a> {
    owner: Cow<'a, str>,
    name: Cow<'a, str>,
}

impl<'a> Repository<'a> {
    pub const ZED: Repository<'static> = Repository::new_static("zed-industries", "zed");

    pub fn new(owner: &'a str, name: &'a str) -> Self {
        Self {
            owner: Cow::Borrowed(owner),
            name: Cow::Borrowed(name),
        }
    }

    pub fn owner(&self) -> &str {
        &self.owner
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl Repository<'static> {
    pub const fn new_static(owner: &'static str, name: &'static str) -> Self {
        Self {
            owner: Cow::Borrowed(owner),
            name: Cow::Borrowed(name),
        }
    }
}

#[async_trait::async_trait(?Send)]
pub trait GithubApiClient {
    async fn get_pull_request(
        &self,
        repo: &Repository<'_>,
        pr_number: u64,
    ) -> Result<PullRequestData>;
    async fn get_pull_request_reviews(
        &self,
        repo: &Repository<'_>,
        pr_number: u64,
    ) -> Result<Vec<PullRequestReview>>;
    async fn get_pull_request_comments(
        &self,
        repo: &Repository<'_>,
        pr_number: u64,
    ) -> Result<Vec<PullRequestComment>>;
    async fn get_commit_authors(
        &self,
        repo: &Repository<'_>,
        commit_shas: &[&CommitSha],
    ) -> Result<AuthorsForCommits>;
    async fn check_repo_write_permission(
        &self,
        repo: &Repository<'_>,
        login: &GithubLogin,
    ) -> Result<bool>;
    async fn add_label_to_issue(
        &self,
        repo: &Repository<'_>,
        label: &str,
        issue_number: u64,
    ) -> Result<()>;
}

#[derive(Deref)]
pub struct GithubClient {
    api: Rc<dyn GithubApiClient>,
}

impl GithubClient {
    pub fn new(api: Rc<dyn GithubApiClient>) -> Self {
        Self { api }
    }

    #[cfg(feature = "octo-client")]
    pub async fn for_app_in_repo(app_id: u64, app_private_key: &str, org: &str) -> Result<Self> {
        let client = OctocrabClient::new(app_id, app_private_key, org).await?;
        Ok(Self::new(Rc::new(client)))
    }
}

pub mod graph_ql {
    use anyhow::{Context as _, Result};
    use itertools::Itertools as _;
    use serde::Deserialize;

    use crate::git::CommitSha;

    use super::AuthorsForCommits;

    #[derive(Debug, Deserialize)]
    pub struct GraphQLResponse<T> {
        pub data: Option<T>,
        pub errors: Option<Vec<GraphQLError>>,
    }

    impl<T> GraphQLResponse<T> {
        pub fn into_data(self) -> Result<T> {
            if let Some(errors) = &self.errors {
                if !errors.is_empty() {
                    let messages: String = errors.iter().map(|e| e.message.as_str()).join("; ");
                    anyhow::bail!("GraphQL error: {messages}");
                }
            }

            self.data.context("GraphQL response contained no data")
        }
    }

    #[derive(Debug, Deserialize)]
    pub struct GraphQLError {
        pub message: String,
    }

    #[derive(Debug, Deserialize)]
    pub struct CommitAuthorsResponse {
        pub repository: AuthorsForCommits,
    }

    pub fn deserialize_nodes<'de, T, D>(deserializer: D) -> std::result::Result<Vec<T>, D::Error>
    where
        T: Deserialize<'de>,
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Nodes<T> {
            nodes: Vec<T>,
        }
        Nodes::<T>::deserialize(deserializer).map(|wrapper| wrapper.nodes)
    }

    pub fn build_co_authors_query<'a>(
        org: &str,
        repo: &str,
        shas: impl IntoIterator<Item = &'a CommitSha>,
    ) -> String {
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

        let objects = shas
            .into_iter()
            .map(|commit_sha| {
                format!(
                    "{sha_prefix}{sha}: object(oid: \"{sha}\") {{ {FRAGMENT} }}",
                    sha_prefix = AuthorsForCommits::SHA_PREFIX,
                    sha = **commit_sha,
                )
            })
            .join("\n");

        format!("{{  repository(owner: \"{org}\", name: \"{repo}\") {{ {objects}  }} }}")
            .replace("\n", "")
    }
}

#[cfg(feature = "octo-client")]
mod octo_client {
    use anyhow::{Context, Result};
    use futures::TryStreamExt as _;
    use jsonwebtoken::EncodingKey;
    use octocrab::{
        Octocrab, Page, models::pulls::ReviewState as OctocrabReviewState,
        service::middleware::cache::mem::InMemoryCache,
    };
    use serde::de::DeserializeOwned;
    use tokio::pin;

    use crate::{
        git::CommitSha,
        github::{Repository, graph_ql},
    };

    use super::{
        AuthorsForCommits, GithubApiClient, GithubLogin, GithubUser, PullRequestComment,
        PullRequestData, PullRequestReview, ReviewState,
    };

    const PAGE_SIZE: u8 = 100;

    pub struct OctocrabClient {
        client: Octocrab,
    }

    impl OctocrabClient {
        pub async fn new(app_id: u64, app_private_key: &str, org: &str) -> Result<Self> {
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
                .find(|installation| installation.account.login == org)
                .context("Could not find Zed repository in installations")?
                .id;

            let client = octocrab.installation(installation_id)?;
            Ok(Self { client })
        }

        async fn graphql<R: DeserializeOwned>(&self, query: &serde_json::Value) -> Result<R> {
            let response: serde_json::Value = self.client.graphql(query).await?;
            let parsed: graph_ql::GraphQLResponse<R> = serde_json::from_value(response)
                .context("Failed to parse GraphQL response envelope")?;
            parsed.into_data()
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
    impl GithubApiClient for OctocrabClient {
        async fn get_pull_request(
            &self,
            repo: &Repository<'_>,
            pr_number: u64,
        ) -> Result<PullRequestData> {
            let pr = self
                .client
                .pulls(repo.owner.as_ref(), repo.name.as_ref())
                .get(pr_number)
                .await?;
            Ok(PullRequestData {
                number: pr.number,
                user: pr.user.map(|user| GithubUser { login: user.login }),
                merged_by: pr.merged_by.map(|user| GithubUser { login: user.login }),
                labels: pr
                    .labels
                    .map(|labels| labels.into_iter().map(|label| label.name).collect()),
            })
        }

        async fn get_pull_request_reviews(
            &self,
            repo: &Repository<'_>,
            pr_number: u64,
        ) -> Result<Vec<PullRequestReview>> {
            let page = self
                .client
                .pulls(repo.owner.as_ref(), repo.name.as_ref())
                .list_reviews(pr_number)
                .per_page(PAGE_SIZE)
                .send()
                .await?;

            let reviews = self.get_all(page).await?;

            Ok(reviews
                .into_iter()
                .map(|review| PullRequestReview {
                    user: review.user.map(|user| GithubUser { login: user.login }),
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
            repo: &Repository<'_>,
            pr_number: u64,
        ) -> Result<Vec<PullRequestComment>> {
            let page = self
                .client
                .issues(repo.owner.as_ref(), repo.name.as_ref())
                .list_comments(pr_number)
                .per_page(PAGE_SIZE)
                .send()
                .await?;

            let comments = self.get_all(page).await?;

            Ok(comments
                .into_iter()
                .map(|comment| PullRequestComment {
                    user: GithubUser {
                        login: comment.user.login,
                    },
                    body: comment.body,
                })
                .collect())
        }

        async fn get_commit_authors(
            &self,
            repo: &Repository<'_>,
            commit_shas: &[&CommitSha],
        ) -> Result<AuthorsForCommits> {
            let query = graph_ql::build_co_authors_query(
                repo.owner.as_ref(),
                repo.name.as_ref(),
                commit_shas.iter().copied(),
            );
            let query = serde_json::json!({ "query": query });
            self.graphql::<graph_ql::CommitAuthorsResponse>(&query)
                .await
                .map(|response| response.repository)
        }

        async fn check_repo_write_permission(
            &self,
            repo: &Repository<'_>,
            login: &GithubLogin,
        ) -> Result<bool> {
            // Check org membership first - we save ourselves a few request that way
            let page = self
                .client
                .orgs(repo.owner.as_ref())
                .list_members()
                .per_page(PAGE_SIZE)
                .send()
                .await?;

            let members = self.get_all(page).await?;

            if members
                .into_iter()
                .any(|member| member.login == login.as_str())
            {
                return Ok(true);
            }

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
                        "/repos/{owner}/{repo}/collaborators/{user}/permission",
                        owner = repo.owner.as_ref(),
                        repo = repo.name.as_ref(),
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

        async fn add_label_to_issue(
            &self,
            repo: &Repository<'_>,
            label: &str,
            issue_number: u64,
        ) -> Result<()> {
            self.client
                .issues(repo.owner.as_ref(), repo.name.as_ref())
                .add_labels(issue_number, &[label.to_owned()])
                .await
                .map(|_| ())
                .map_err(Into::into)
        }
    }
}

#[cfg(feature = "octo-client")]
pub use octo_client::OctocrabClient;
