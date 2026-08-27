use anyhow::Context as _;
use futures::AsyncReadExt;
use gpui::SharedString;
use http_client::{AsyncBody, HttpClient, HttpClientWithUrl, HttpRequestExt, Request};
use serde::Deserialize;
use std::{sync::Arc, time::Duration};

const GITHUB_SEARCH_TIMEOUT: Duration = Duration::from_secs(10);
const MAX_GITHUB_SUGGESTIONS: usize = 8;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct CloneSuggestion {
    pub(crate) title: SharedString,
    pub(crate) detail: SharedString,
    pub(crate) repo_url: SharedString,
}

pub(crate) fn for_input(input: &str) -> Vec<CloneSuggestion> {
    let input = input.trim();
    if input.is_empty() {
        return Vec::new();
    }

    if let Some(repo_url) = normalize_github_url(input) {
        let repo_url: SharedString = repo_url.into();
        return vec![CloneSuggestion {
            title: SharedString::new_static("Clone GitHub repository"),
            detail: repo_url.clone(),
            repo_url,
        }];
    }

    if is_explicit_local_path(input) || is_explicit_remote(input) {
        return vec![entered_repository(input)];
    }

    if is_bare_owner_repo(input) {
        let github_url: SharedString =
            format!("https://github.com/{}.git", input.trim_end_matches(".git")).into();
        let entered_location = SharedString::new(input);
        return vec![
            CloneSuggestion {
                title: SharedString::new_static("Clone GitHub repository"),
                detail: github_url.clone(),
                repo_url: github_url,
            },
            CloneSuggestion {
                title: SharedString::new_static("Clone entered location"),
                detail: entered_location.clone(),
                repo_url: entered_location,
            },
        ];
    }

    vec![entered_repository(input)]
}

pub(crate) fn should_search_github(query: &str) -> bool {
    query.len() >= 3
        && !is_explicit_remote(query)
        && !is_explicit_local_path(query)
        && normalize_github_url(query).is_none()
}

pub(crate) fn append_unique(
    suggestions: &mut Vec<CloneSuggestion>,
    new_suggestions: impl IntoIterator<Item = CloneSuggestion>,
) {
    for suggestion in new_suggestions {
        if suggestions
            .iter()
            .all(|existing| existing.repo_url != suggestion.repo_url)
        {
            suggestions.push(suggestion);
        }
    }
}

pub(crate) async fn search_github(
    http_client: Arc<HttpClientWithUrl>,
    query: &str,
) -> anyhow::Result<Vec<CloneSuggestion>> {
    let request = github_search_request(query, github_token().as_deref())?;
    let mut response = http_client
        .send(request)
        .await
        .context("requesting GitHub repository suggestions")?;
    let status = response.status();
    anyhow::ensure!(
        status.is_success(),
        "GitHub repository search returned HTTP {}",
        status,
    );

    let mut body = Vec::new();
    response
        .body_mut()
        .read_to_end(&mut body)
        .await
        .context("reading GitHub repository suggestions")?;
    let response: GithubSearchResponse =
        serde_json::from_slice(&body).context("parsing GitHub repository suggestions")?;

    Ok(response
        .items
        .into_iter()
        .take(MAX_GITHUB_SUGGESTIONS)
        .map(CloneSuggestion::from)
        .collect())
}

fn entered_repository(input: &str) -> CloneSuggestion {
    let repo_url = SharedString::new(input);
    CloneSuggestion {
        title: SharedString::new_static("Clone entered repository"),
        detail: repo_url.clone(),
        repo_url,
    }
}

fn is_explicit_local_path(input: &str) -> bool {
    input.starts_with("./")
        || input.starts_with("../")
        || input.starts_with('~')
        || input.starts_with('/')
        || input.contains('\\')
        || input
            .get(1..3)
            .is_some_and(|prefix| matches!(prefix, ":/" | ":\\"))
}

fn is_explicit_remote(input: &str) -> bool {
    input.contains("://")
        || input
            .split_once(':')
            .is_some_and(|(host, path)| host.len() > 1 && !host.contains('/') && !path.is_empty())
}

fn normalize_github_url(input: &str) -> Option<String> {
    let url = if input.contains("://") {
        url::Url::parse(input).ok()?
    } else {
        url::Url::parse(&format!("https://{input}")).ok()?
    };
    if !matches!(url.scheme(), "http" | "https")
        || !matches!(url.host_str(), Some("github.com" | "www.github.com"))
    {
        return None;
    }

    let mut segments = url.path_segments()?.filter(|segment| !segment.is_empty());
    let owner = segments.next()?;
    let repository = segments.next()?.trim_end_matches(".git");
    (segments.next().is_none() && is_valid_owner_repo(owner, repository))
        .then(|| format!("https://github.com/{owner}/{repository}.git"))
}

fn is_bare_owner_repo(input: &str) -> bool {
    let input = input.trim_end_matches(".git");
    let mut segments = input.split('/');
    let Some(owner) = segments.next() else {
        return false;
    };
    let Some(repository) = segments.next() else {
        return false;
    };

    segments.next().is_none() && is_valid_owner_repo(owner, repository)
}

fn is_valid_owner_repo(owner: &str, repository: &str) -> bool {
    !matches!(owner, "" | "." | "..")
        && !matches!(repository, "" | "." | "..")
        && !owner.chars().any(char::is_whitespace)
        && !repository.chars().any(char::is_whitespace)
}

#[derive(Deserialize)]
struct GithubSearchResponse {
    items: Vec<GithubRepository>,
}

#[derive(Deserialize)]
struct GithubRepository {
    full_name: String,
    clone_url: String,
    private: bool,
    description: Option<String>,
}

impl From<GithubRepository> for CloneSuggestion {
    fn from(repository: GithubRepository) -> Self {
        let visibility = if repository.private {
            "private"
        } else {
            "public"
        };
        let detail = repository
            .description
            .map(|description| format!("{visibility} - {description}"))
            .unwrap_or_else(|| visibility.to_owned());
        Self {
            title: repository.full_name.into(),
            detail: detail.into(),
            repo_url: repository.clone_url.into(),
        }
    }
}

fn github_search_request(query: &str, token: Option<&str>) -> anyhow::Result<Request<AsyncBody>> {
    let mut url = url::Url::parse("https://api.github.com/search/repositories")?;
    url.query_pairs_mut()
        .append_pair("q", query)
        .append_pair("sort", "updated")
        .append_pair("per_page", &MAX_GITHUB_SUGGESTIONS.to_string());

    let mut request = Request::get(url.as_str())
        .header("Accept", "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .follow_redirects(http_client::RedirectPolicy::FollowAll)
        .timeout(GITHUB_SEARCH_TIMEOUT);
    if let Some(token) = token {
        request = request.header("Authorization", format!("Bearer {token}"));
    }
    request.body(AsyncBody::default()).map_err(Into::into)
}

fn github_token() -> Option<String> {
    std::env::var("GITHUB_TOKEN")
        .ok()
        .filter(|token| !token.is_empty())
        .or_else(|| {
            std::env::var("GH_TOKEN")
                .ok()
                .filter(|token| !token.is_empty())
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_suggestions_for_repository_inputs() {
        assert!(for_input("  ").is_empty());
        assert_eq!(
            for_input("zed-industries/zed")
                .iter()
                .map(|suggestion| suggestion.repo_url.as_ref())
                .collect::<Vec<_>>(),
            [
                "https://github.com/zed-industries/zed.git",
                "zed-industries/zed"
            ]
        );

        for input in [
            "../zed",
            "./zed",
            "/tmp/zed",
            "~/zed",
            r"C:\src\zed",
            "C:/zed",
            "git@github.com:zed-industries/zed.git",
            "deploy@code.example.com:team/repository.git",
            "ssh://git@github.com/zed-industries/zed.git",
        ] {
            assert_eq!(
                for_input(input)
                    .first()
                    .map(|suggestion| suggestion.repo_url.as_ref()),
                Some(input),
                "{input}"
            );
        }
    }

    #[test]
    fn normalizes_github_urls() {
        for input in [
            "github.com/zed-industries/zed",
            "www.github.com/zed-industries/zed/",
            "http://github.com/zed-industries/zed.git",
            "https://github.com/zed-industries/zed",
            "https://github.com/zed-industries/zed?tab=readme",
        ] {
            assert_eq!(
                for_input(input)
                    .first()
                    .map(|suggestion| suggestion.repo_url.as_ref()),
                Some("https://github.com/zed-industries/zed.git"),
                "{input}"
            );
        }
    }

    #[test]
    fn searches_only_searchable_input() {
        assert!(should_search_github("zed"));
        assert!(should_search_github("zed-industries/zed"));
        assert!(!should_search_github("ze"));
        assert!(!should_search_github("../zed"));
        assert!(!should_search_github(
            "https://github.com/zed-industries/zed"
        ));
        assert!(!should_search_github("github.com/zed-industries/zed"));
        assert!(!should_search_github("git@github.com:zed-industries/zed"));
    }

    #[test]
    fn deduplicates_clone_urls() {
        let suggestion = CloneSuggestion {
            title: SharedString::new_static("zed-industries/zed"),
            detail: SharedString::new_static("public"),
            repo_url: SharedString::new_static("https://github.com/zed-industries/zed.git"),
        };
        let mut suggestions = vec![suggestion.clone()];
        append_unique(&mut suggestions, [suggestion]);
        assert_eq!(suggestions.len(), 1);
    }

    #[test]
    fn parses_github_search_response() -> anyhow::Result<()> {
        let response: GithubSearchResponse = serde_json::from_str(
            r#"{"items":[{"full_name":"zed-industries/zed","clone_url":"https://github.com/zed-industries/zed.git","private":false,"description":"Code at the speed of thought"}]}"#,
        )?;
        assert_eq!(
            response
                .items
                .first()
                .map(|repository| repository.full_name.as_str()),
            Some("zed-industries/zed")
        );
        Ok(())
    }

    #[test]
    fn builds_encoded_authenticated_search_request() -> anyhow::Result<()> {
        let request = github_search_request("zed editor", Some("test-token"))?;
        assert_eq!(
            request.uri().to_string(),
            "https://api.github.com/search/repositories?q=zed+editor&sort=updated&per_page=8"
        );
        assert_eq!(
            request
                .headers()
                .get("Authorization")
                .and_then(|value| value.to_str().ok()),
            Some("Bearer test-token")
        );
        Ok(())
    }
}
