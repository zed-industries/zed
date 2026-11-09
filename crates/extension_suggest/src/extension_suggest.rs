mod suggestions_by_path_suffix;

use std::sync::Arc;

use path::rel_path::RelPath;

use suggestions_by_path_suffix::SUGGESTIONS_BY_PATH_SUFFIX;

pub struct LanguageAdditionSuggestion {
    pub extension_id: &'static str,
    pub languages: &'static [&'static str],
    pub title: &'static str,
    pub description: &'static str,
    pub docs_url: &'static str,
    pub install_message: &'static str,
}

const SUGGESTIONS_BY_LANGUAGE: &[LanguageAdditionSuggestion] = &[LanguageAdditionSuggestion {
    extension_id: "emmet",
    languages: &[
        "Angular",
        "Blade",
        "CSS",
        "Django",
        "ERB",
        "Elixir",
        "HEEx",
        "HTML",
        "HTML+ERB",
        "JavaScript",
        "Jinja2",
        "LESS",
        "Liquid",
        "Nunjucks",
        "PHP",
        "SCSS",
        "Statamic Antlers",
        "TSX",
        "Twig",
        "Vue.js",
    ],
    title: "Emmet is available for this file",
    description: "Emmet expands abbreviations such as `ul>li*3` into HTML and `m10` into CSS.",
    docs_url: "https://zed.dev/docs/languages/emmet",
    install_message: "Install Emmet",
}];

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SuggestedExtension {
    pub extension_id: Arc<str>,
    pub file_name_or_extension: Arc<str>,
}

/// Returns the suggested extension for the given [`Path`].
pub fn suggest_extension(path: &RelPath) -> Option<SuggestedExtension> {
    let (file_name_or_extension, extension_id) = path
        .file_name()
        .and_then(|file_name| {
            // We suggest against file names first, as these suggestions will be more
            // specific than ones based on the file extension.
            SUGGESTIONS_BY_PATH_SUFFIX
                .get(file_name)
                .map(|extension_id| (Arc::from(file_name), *extension_id))
        })
        .or_else(|| {
            path.extension().and_then(|file_extension| {
                SUGGESTIONS_BY_PATH_SUFFIX
                    .get(file_extension)
                    .map(|extension_id| (Arc::from(file_extension), *extension_id))
            })
        })?;

    Some(SuggestedExtension {
        extension_id: Arc::from(extension_id),
        file_name_or_extension,
    })
}

pub fn additional_suggestion_for_language(
    language_name: &str,
) -> Option<&LanguageAdditionSuggestion> {
    SUGGESTIONS_BY_LANGUAGE
        .iter()
        .find(|suggestion| suggestion.languages.contains(&language_name))
}

#[cfg(test)]
mod tests {
    use super::*;
    use path::rel_path::rel_path;

    #[test]
    pub fn test_suggested_extension() {
        assert_eq!(
            suggest_extension(rel_path("Cargo.toml")),
            Some(SuggestedExtension {
                extension_id: "toml".into(),
                file_name_or_extension: "toml".into()
            })
        );
        assert_eq!(
            suggest_extension(rel_path("Cargo.lock")),
            Some(SuggestedExtension {
                extension_id: "toml".into(),
                file_name_or_extension: "Cargo.lock".into()
            })
        );
        assert_eq!(
            suggest_extension(rel_path("Dockerfile")),
            Some(SuggestedExtension {
                extension_id: "dockerfile".into(),
                file_name_or_extension: "Dockerfile".into()
            })
        );
        assert_eq!(
            suggest_extension(rel_path("a/b/c/d/.gitignore")),
            Some(SuggestedExtension {
                extension_id: "git-firefly".into(),
                file_name_or_extension: ".gitignore".into()
            })
        );
        assert_eq!(
            suggest_extension(rel_path("a/b/c/d/test.gleam")),
            Some(SuggestedExtension {
                extension_id: "gleam".into(),
                file_name_or_extension: "gleam".into()
            })
        );
        assert_eq!(
            suggest_extension(rel_path("a/b/c/d/test.sol")),
            Some(SuggestedExtension {
                extension_id: "solidity".into(),
                file_name_or_extension: "sol".into()
            })
        );
        assert_eq!(
            suggest_extension(rel_path("a/b/c/d/test.jl")),
            Some(SuggestedExtension {
                extension_id: "julia".into(),
                file_name_or_extension: "jl".into()
            })
        );
        assert_eq!(
            suggest_extension(rel_path("script.pl")),
            Some(SuggestedExtension {
                extension_id: "perl".into(),
                file_name_or_extension: "pl".into()
            })
        );
        assert_eq!(
            suggest_extension(rel_path("app/uv.lock")),
            Some(SuggestedExtension {
                extension_id: "toml".into(),
                file_name_or_extension: "uv.lock".into()
            })
        );
        // Dotfiles have no `Path::extension`, so they match by name.
        assert_eq!(
            suggest_extension(rel_path(".helmignore")),
            Some(SuggestedExtension {
                extension_id: "helm".into(),
                file_name_or_extension: ".helmignore".into()
            })
        );
        assert_eq!(
            suggest_extension(rel_path(".gitattributes")),
            Some(SuggestedExtension {
                extension_id: "git-firefly".into(),
                file_name_or_extension: ".gitattributes".into()
            })
        );
    }
}
