use gpui::SharedString;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct CloneSuggestion {
    pub(crate) title: SharedString,
    pub(crate) detail: SharedString,
    pub(crate) repo_url: SharedString,
    pub(crate) provider: Option<SharedString>,
}

pub(crate) fn for_input(input: &str) -> Vec<CloneSuggestion> {
    let input = input.trim();
    if input.is_empty() {
        Vec::new()
    } else {
        vec![entered_repository(input)]
    }
}

pub(crate) fn should_search_providers(query: &str) -> bool {
    query.len() >= 3 && !is_explicit_remote(query) && !is_explicit_local_path(query)
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

impl CloneSuggestion {
    pub(crate) fn from_search_result(
        result: git::RepositorySearchResult,
        provider_name: &str,
    ) -> Self {
        Self {
            title: result.name,
            detail: result.detail,
            repo_url: result.clone_url,
            provider: Some(provider_name.into()),
        }
    }
}

fn entered_repository(input: &str) -> CloneSuggestion {
    let repo_url = SharedString::new(input);
    CloneSuggestion {
        title: SharedString::new_static("Clone entered repository"),
        detail: repo_url.clone(),
        repo_url,
        provider: None,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preserves_entered_repository_inputs() {
        assert!(for_input("  ").is_empty());

        for input in [
            "zed-industries/zed",
            "https://github.com/zed-industries/zed",
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
    fn searches_only_provider_queries() {
        assert!(should_search_providers("zed"));
        assert!(should_search_providers("zed-industries/zed"));
        assert!(!should_search_providers("ze"));
        assert!(!should_search_providers("../zed"));
        assert!(!should_search_providers(
            "https://github.com/zed-industries/zed"
        ));
        assert!(!should_search_providers(
            "git@github.com:zed-industries/zed"
        ));
    }

    #[test]
    fn converts_and_deduplicates_provider_results() {
        let result = git::RepositorySearchResult {
            name: "zed-industries/zed".into(),
            detail: "public".into(),
            clone_url: "https://github.com/zed-industries/zed.git".into(),
        };
        let suggestion = CloneSuggestion::from_search_result(result, "GitHub");
        let mut suggestions = vec![suggestion.clone()];
        append_unique(&mut suggestions, [suggestion]);

        assert_eq!(suggestions.len(), 1);
        assert_eq!(suggestions[0].title, "zed-industries/zed");
        assert_eq!(suggestions[0].provider.as_deref(), Some("GitHub"));
    }
}
