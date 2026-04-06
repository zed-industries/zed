use std::{collections::HashMap, fmt, ops::Not};

use anyhow::{Context, Result};
use derive_more::Deref;
use futures_util::TryStreamExt as _;
use itertools::Itertools;
use jsonwebtoken::EncodingKey;
use octocrab::{
    Octocrab, Page,
    models::{
        issues,
        pulls::{PullRequest, Review},
    },
    service::middleware::cache::mem::InMemoryCache,
};
use serde::{Deserialize, de::DeserializeOwned};
use tokio::pin;

use crate::git::CommitSha;

const PAGE_SIZE: u8 = 100;
const ORG: &str = "zed-industries";
const REPO: &str = "zed";

pub struct GitHubClient {
    client: Octocrab,
}

#[derive(Debug, Deserialize, Clone, Deref, PartialEq, Eq)]
pub struct GithubLogin {
    login: String,
}

impl GithubLogin {
    pub(crate) fn new(login: String) -> Self {
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

impl GitHubClient {
    pub async fn for_app(app_id: u64, app_private_key: &str) -> Result<Self> {
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

        octocrab
            .installation(installation_id)
            .map(Self::new)
            .map_err(Into::into)
    }

    fn new(client: Octocrab) -> Self {
        Self { client }
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

    pub(crate) async fn get_commit_co_authors(
        &self,
        commit_shas: impl IntoIterator<Item = &CommitSha>,
    ) -> Result<AuthorsForCommits> {
        let query = Self::build_co_authors_query(commit_shas);

        let query = serde_json::json!({ "query": query });

        let mut response = self.graphql::<serde_json::Value>(&query).await?;

        // TODO speaks for itself
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

    pub(crate) async fn graphql<R: octocrab::FromResponse>(
        &self,
        query: &serde_json::Value,
    ) -> octocrab::Result<R> {
        self.client.graphql(query).await
    }

    pub async fn get_pull_request(&self, pr_number: u64) -> octocrab::Result<PullRequest> {
        self.client.pulls(ORG, REPO).get(pr_number).await
    }

    pub async fn get_pr_reviews(
        &self,
        pr_number: u64,
    ) -> octocrab::Result<impl Iterator<Item = Review>> {
        self.get_all(
            self.client
                .pulls(ORG, REPO)
                .list_reviews(pr_number)
                .per_page(PAGE_SIZE)
                .send()
                .await?,
        )
        .await
    }

    pub async fn get_pr_comments(
        &self,
        pr_number: u64,
    ) -> octocrab::Result<impl Iterator<Item = issues::Comment>> {
        self.get_all(
            self.client
                .issues(ORG, REPO)
                .list_comments(pr_number)
                .per_page(PAGE_SIZE)
                .send()
                .await?,
        )
        .await
    }

    pub async fn add_label_to_pr(&self, label: &str, pr_number: u64) -> octocrab::Result<()> {
        self.client
            .issues(ORG, REPO)
            .add_labels(pr_number, &[label.to_owned()])
            .await
            .map(|_| ())
    }

    pub async fn check_org_membership(&self, login: &GithubLogin) -> octocrab::Result<bool> {
        self.get_all(
            self.client
                .orgs(ORG)
                .list_members()
                .per_page(PAGE_SIZE)
                .send()
                .await?,
        )
        .await
        .map(|members| {
            members
                .map(|member| member.login)
                .any(|member_login| member_login == login.as_str())
        })
    }

    async fn get_all<T: DeserializeOwned + 'static>(
        &self,
        page: Page<T>,
    ) -> octocrab::Result<impl Iterator<Item = T>> {
        self.get_filtered(page, |_| true).await
    }

    async fn get_filtered<T: DeserializeOwned + 'static>(
        &self,
        page: Page<T>,
        predicate: fn(&T) -> bool,
    ) -> octocrab::Result<impl Iterator<Item = T>> {
        let stream = page.into_stream(&self.client);
        pin!(stream);

        let mut results = Vec::new();

        while let Some(item) = stream.try_next().await?
            && predicate(&item)
        {
            results.push(item);
        }

        Ok(results.into_iter())
    }
}
