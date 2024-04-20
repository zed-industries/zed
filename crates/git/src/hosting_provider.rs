use core::fmt;
use std::{ops::Range, sync::Arc};

use anyhow::Result;
use url::Url;
use util::{github, http::HttpClient};

use crate::Oid;

#[derive(Clone, Debug, Hash)]
pub enum HostingProvider {
    Github,
    Gitlab,
    Gitee,
    Bitbucket,
    Sourcehut,
    Codeberg,
}

impl HostingProvider {
    pub(crate) fn base_url(&self) -> Url {
        let base_url = match self {
            Self::Github => "https://github.com",
            Self::Gitlab => "https://gitlab.com",
            Self::Gitee => "https://gitee.com",
            Self::Bitbucket => "https://bitbucket.org",
            Self::Sourcehut => "https://git.sr.ht",
            Self::Codeberg => "https://codeberg.org",
        };

        Url::parse(&base_url).unwrap()
    }

    /// Returns the fragment portion of the URL for the selected lines in
    /// the representation the [`GitHostingProvider`] expects.
    pub(crate) fn line_fragment(&self, selection: &Range<u32>) -> String {
        if selection.start == selection.end {
            let line = selection.start + 1;

            match self {
                Self::Github | Self::Gitlab | Self::Gitee | Self::Sourcehut | Self::Codeberg => {
                    format!("L{}", line)
                }
                Self::Bitbucket => format!("lines-{}", line),
            }
        } else {
            let start_line = selection.start + 1;
            let end_line = selection.end + 1;

            match self {
                Self::Github | Self::Codeberg => format!("L{}-L{}", start_line, end_line),
                Self::Gitlab | Self::Gitee | Self::Sourcehut => {
                    format!("L{}-{}", start_line, end_line)
                }
                Self::Bitbucket => format!("lines-{}:{}", start_line, end_line),
            }
        }
    }

    pub fn supports_avatars(&self) -> bool {
        match self {
            HostingProvider::Github => true,
            _ => false,
        }
    }

    pub async fn commit_author_avatar_url(
        &self,
        repo_owner: &str,
        repo: &str,
        commit: Oid,
        client: Arc<dyn HttpClient>,
    ) -> Result<Option<Url>> {
        match self {
            HostingProvider::Github => {
                let commit = commit.to_string();

                let author =
                    github::fetch_github_commit_author(repo_owner, repo, &commit, &client).await?;

                let url = if let Some(author) = author {
                    let mut url = Url::parse(&author.avatar_url)?;
                    url.set_query(Some("size=128"));
                    Some(url)
                } else {
                    None
                };
                Ok(url)
            }
            _ => Ok(None),
        }
    }
}

impl fmt::Display for HostingProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            HostingProvider::Github => "GitHub",
            HostingProvider::Gitlab => "GitLab",
            HostingProvider::Gitee => "Gitee",
            HostingProvider::Bitbucket => "Bitbucket",
            HostingProvider::Sourcehut => "Sourcehut",
            HostingProvider::Codeberg => "Codeberg",
        };
        write!(f, "{}", name)
    }
}
