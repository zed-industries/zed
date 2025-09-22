//! A language for tagged comments.
//!
//! Uses [tree-sitter-comment] as grammar and dynamically created Tree-sitter queries for highlighting
//! and outline.
//!
//! These queries are derived from information in the `comment.json` configuration file.
//! Users may opt-in to tagged comment support by creating this file.
//!
//! The Zed action "open tagged comment configuration" creates a default configuration file
//! and opens it for editing.
//!
//! Users may configure which tags to be recognized, the styles to be used for highlighting and
//! what to show in outline views by editing the configuration file.
//! The styles must be syntax styles defined by the used theme or custom definitions in the
//! user's settings file.
//!
//! The configuration file is monitored for changes.
//!
//! Users may disable tagged comment support again by removing the configuration file (effective only
//! after restarting Zed), completely clearing its contents or just emptying the array of contained
//! highlighting definitions.
//!
//! If users decide to disable tagged comment support, the "tagged comment language" is removed from
//! the language registry.
//!
//! Language developers may decide to support tagged comments by including the needed injection
//! clause in their language's `injections.scm`.
//!
//! [tree-sitter-comment]: https://github.com/stsewd/tree-sitter-comment

// NOTE: This module uses crate `tree_sitter_comment` which is only available if feature "load-grammars" is enabled.

// As the "tagged comment language" uses dynamically created queries based on user configuration, the approach
// used for other languages built into Zed or shipped as extensions with Zed does not work.

use {
    language::{Language, LanguageConfig, LanguageMatcher, LanguageName, LanguageRegistry},
    std::{
        fmt::Write,
        path::PathBuf,
        sync::{Arc, OnceLock},
    },
};

/// Tagged Comment language.
pub struct Comment;

impl Comment {
    // Name used for grammar, directory, ... (this is not the `LanguageName`).
    const NAME: &str = "comment";
    const LANGUAGE_NAME: LanguageName =
        LanguageName(gpui::SharedString::new_static("Tagged Comment"));

    /// Returns the path to the configuration file.
    pub fn config_path() -> &'static PathBuf {
        static PATH: OnceLock<PathBuf> = OnceLock::new();
        PATH.get_or_init(|| paths::config_dir().join("comment.json"))
    }

    /// Returns the default configuration.
    pub fn default_config() -> std::borrow::Cow<'static, str> {
        util::asset_str::<settings::SettingsAssets>("settings/initial_tagged_comment.json")
    }

    /// Unconditionally disables tagged comment support.
    pub fn disable(registry: &LanguageRegistry) {
        Self::configure("", registry);
    }

    /// Configures the Tree-sitter highlighting and outline queries used for tagged comments.
    ///
    /// Disables tagged comment support if the configuration contains no comment groups.
    /// Otherwise registers the tagged comment language with dynamically created queries derived
    /// from the configuration.
    ///
    /// This function is intended to be called on Zed startup too.
    pub fn configure(config: &str, registry: &LanguageRegistry) {
        // There seems to be no way to update a single language in the registry (Zed v0.199).
        // So just remove any old tagged comment language entry and eventually add new one using new queries.
        // This also disables tagged comment support, if the user decides to remove/clear the tagged comment configuration.
        registry.remove_languages(&[Self::LANGUAGE_NAME], &[]);
        let queries = Self::queries_from_config(config);

        let state = if let Some((highlights, outline)) = queries {
            let language = Language::new(
                Self::language_config(),
                Some(tree_sitter_comment::LANGUAGE.into()),
            )
            .with_highlights_query(&highlights)
            .unwrap();
            let language = match outline {
                Some(outline) => language.with_outline_query(&outline).unwrap(),
                None => language,
            };
            registry.add(Arc::new(language));
            "en"
        } else {
            "dis"
        };

        log::info!("tagged comment support {state}abled");
    }

    fn grammar_name() -> &'static Arc<str> {
        // The grammar name is needed multiple times, so use a static value.
        static NAME: OnceLock<Arc<str>> = OnceLock::new();
        NAME.get_or_init(|| Self::NAME.into())
    }

    fn language_config() -> LanguageConfig {
        LanguageConfig {
            name: Self::LANGUAGE_NAME,
            grammar: Some(Self::grammar_name().clone()),
            hidden: true,
            // Can't just use LanguageMatcher::default().
            matcher: Self::language_matcher(),
            ..Default::default()
        }
    }

    // The language matcher is needed multiple times.
    fn language_matcher() -> LanguageMatcher {
        LanguageMatcher {
            // Without `path_suffixes` set, the comment language will just not be loaded (Zed v0.199).
            // It must not be empty and it must not contain a single empty string.
            path_suffixes: vec![Self::NAME.into()],
            first_line_pattern: None,
        }
    }

    // Parses the given tagged comment configuration and creates the Tree-sitter queries based on it.
    // Returns strings containing the queries similar to the contents of a "highlights.scm" and "outline.scm" file.
    // Returns `None` if tagged comment support is disabled (config is empty or containing an empty array of groups).
    fn queries_from_config(config: &str) -> Option<(String, Option<String>)> {
        let config = config.trim();
        if config.is_empty() {
            return None;
        }

        match serde_json_lenient::from_str::<Vec<Group>>(config) {
            Ok(groups) if groups.is_empty() => None,
            Ok(groups) => {
                let mut highlights = String::new();
                let mut outline = String::new();
                for group in groups {
                    group.write_highlighting_query(&mut highlights).unwrap();
                    group.write_outline_query(&mut outline).unwrap();
                }
                let outline = if outline.trim().is_empty() {
                    None
                } else {
                    Some(outline)
                };
                Some((highlights, outline))
            }
            Err(error) => {
                log::error!("invalid tagged comment language configuration: {error}");
                None
            }
        }
    }
}

// Tagged Comment configuration group.
// Stores a group of tags and related properties.
// The configuration file may contain multiple groups.
#[derive(Debug, serde::Deserialize)]
struct Group<'a> {
    // Tag names assigned to this group.
    // Uses a set to avoid duplicates (configuration is editable by users).
    tags: std::collections::HashSet<&'a str>,
    // Optional style used for the whole comment.
    // If not set, the default "comment" style is used.
    // If set to a style Zed can't match, no styling is used.
    style: Option<&'a str>,
    // Optional style used for the tag name.
    // If not set, the style set in `style` is used.
    // If set to a style Zed can't match, no styling is used.
    tag_name_style: Option<&'a str>,
    // Optional style used for the tag user.
    // If not set, the style set in `style` is used.
    // If set to a style Zed can't match, no styling is used.
    tag_user_style: Option<&'a str>,
    outline: Option<Outline>,
}

impl Group<'_> {
    const DEFAULT_STYLE: &'static str = "comment";

    fn write_highlighting_query(&self, f: &mut impl Write) -> std::fmt::Result {
        // It seems when capturing the tag name, the corresponding style must always be set.
        // Otherwise the tag name will not be styled.
        let tag_name_style = self
            .tag_name_style
            .unwrap_or_else(|| self.style.unwrap_or(Self::DEFAULT_STYLE));

        write!(
            f,
            "(_ . (tag (name) @name @{tag_name_style} ( [\"(\" \")\"] @punctuation.bracket"
        )?;

        if let Some(style) = self.tag_user_style {
            write!(f, " (user) @{style}")?;
        }

        writeln!(f, ")? \":\" @punctuation.delimiter)\n  (uri)? @link_uri")?;

        self.write_tags_predicate(f)?;

        write!(f, ")")?;
        if let Some(style) = self.style {
            write!(f, " @{style}")?;
        }
        writeln!(f)
    }

    fn write_outline_query(&self, f: &mut impl Write) -> std::fmt::Result {
        if let Some(outline) = &self.outline {
            write!(f, "(_ . (tag (name) @name")?;
            if let Outline::User = outline {
                write!(f, " (user)? @context")?;
            }
            write!(f, ")")?;
            self.write_tags_predicate(f)?;
            write!(f, ") @item")?;
            if let Outline::Comment = outline {
                write!(f, " @context")?;
            }
            writeln!(f)
        } else {
            Ok(())
        }
    }

    fn write_tags_predicate(&self, f: &mut impl Write) -> std::fmt::Result {
        let tags = &self.tags;
        write!(
            f,
            "  (#{predicate}? @name",
            predicate = if tags.len() == 1 { "eq" } else { "any-of" }
        )?;
        for tag in tags {
            write!(f, " \"{tag}\"")?;
        }
        writeln!(f, ")")
    }
}

// Variants of contents included in outline views.
#[derive(Copy, Clone, Debug, serde::Deserialize)]
enum Outline {
    // Tag name only.
    #[serde(rename = "name")]
    Name,
    // Tag name and user.
    #[serde(rename = "user")]
    User,
    // Comment as a whole.
    // For block-comments restricted to first line by outline view.
    // Because of how the outline view currently works, this will be trailed by the tag name.
    #[serde(rename = "comment")]
    Comment,
}
