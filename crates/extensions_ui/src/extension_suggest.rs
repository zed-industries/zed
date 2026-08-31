use std::collections::HashMap;
use std::sync::{Arc, OnceLock};

use db::kvp::KeyValueStore;
use editor::Editor;
use extension_host::{ExtensionSettings, ExtensionStore};
use gpui::{AppContext as _, Context, Entity, SharedString, Window};
use language::{Buffer, LanguageName};
use settings::Settings;
use ui::prelude::*;
use util::ResultExt;
use util::rel_path::RelPath;
use workspace::notifications::simple_message_notification::MessageNotification;
use workspace::{Workspace, notifications::NotificationId};

const SUGGESTIONS_BY_EXTENSION_ID: &[(&str, &[&str])] = &[
    ("astro", &["astro"]),
    ("beancount", &["beancount"]),
    ("clojure", &["bb", "clj", "cljc", "cljs", "edn"]),
    ("neocmake", &["CMakeLists.txt", "cmake"]),
    ("csharp", &["cs"]),
    ("cython", &["pyx", "pxd", "pxi"]),
    ("dart", &["dart"]),
    ("dockerfile", &["Dockerfile"]),
    ("elisp", &["el"]),
    ("elixir", &["eex", "ex", "exs", "heex", "leex", "neex"]),
    ("elm", &["elm"]),
    ("erlang", &["erl", "hrl"]),
    ("fish", &["fish"]),
    (
        "git-firefly",
        &[
            ".gitconfig",
            ".gitignore",
            "COMMIT_EDITMSG",
            "EDIT_DESCRIPTION",
            "MERGE_MSG",
            "NOTES_EDITMSG",
            "TAG_EDITMSG",
            "git-rebase-todo",
        ],
    ),
    ("gleam", &["gleam"]),
    ("glsl", &["vert", "frag"]),
    ("graphql", &["gql", "graphql"]),
    ("haskell", &["hs"]),
    ("html", &["htm", "html", "shtml"]),
    ("java", &["java"]),
    ("kotlin", &["kt"]),
    ("latex", &["tex"]),
    ("log", &["log"]),
    ("lua", &["lua"]),
    ("make", &["Makefile"]),
    ("nim", &["nim"]),
    ("nix", &["nix"]),
    ("nu", &["nu"]),
    ("ocaml", &["ml", "mli"]),
    ("php", &["php"]),
    ("powershell", &["ps1", "psm1"]),
    ("prisma", &["prisma"]),
    ("proto", &["proto"]),
    ("purescript", &["purs"]),
    ("r", &["r", "R"]),
    ("racket", &["rkt"]),
    ("rescript", &["res", "resi"]),
    ("rst", &["rst"]),
    ("ruby", &["rb", "erb"]),
    ("scheme", &["scm"]),
    ("scss", &["scss"]),
    ("sql", &["sql"]),
    ("svelte", &["svelte"]),
    ("swift", &["swift"]),
    ("templ", &["templ"]),
    ("terraform", &["tf", "tfvars", "hcl"]),
    ("toml", &["Cargo.lock", "toml"]),
    ("typst", &["typ"]),
    ("vue", &["vue"]),
    ("wgsl", &["wgsl"]),
    ("windows-batch", &["bat", "cmd"]),
    ("wit", &["wit"]),
    ("xml", &["xml"]),
    ("zig", &["zig"]),
];

struct OptionalExtensionSuggestion {
    extension_id: &'static str,
    languages: &'static [&'static str],
}

const OPTIONAL_EXTENSION_SUGGESTIONS: &[OptionalExtensionSuggestion] =
    &[OptionalExtensionSuggestion {
        extension_id: "emmet",
        // Extension manifests are not available until installation, so keep this in sync with the
        // languages in the Emmet extension's extension.toml.
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
    }];

fn suggested_extensions() -> &'static HashMap<&'static str, Arc<str>> {
    static SUGGESTIONS_BY_PATH_SUFFIX: OnceLock<HashMap<&str, Arc<str>>> = OnceLock::new();
    SUGGESTIONS_BY_PATH_SUFFIX.get_or_init(|| {
        SUGGESTIONS_BY_EXTENSION_ID
            .iter()
            .flat_map(|(name, path_suffixes)| {
                let name = Arc::<str>::from(*name);
                path_suffixes
                    .iter()
                    .map(move |suffix| (*suffix, name.clone()))
            })
            .collect()
    })
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct SuggestedExtension {
    pub extension_id: Arc<str>,
    pub display_name: Arc<str>,
}

/// Returns the suggested extension for the given [`Path`].
fn suggested_extension(path: &RelPath) -> Option<SuggestedExtension> {
    let file_extension: Option<Arc<str>> = path.extension().map(|extension| extension.into());
    let file_name: Option<Arc<str>> = path.file_name().map(|name| name.into());

    let (display_name, extension_id) = None
        // We suggest against file names first, as these suggestions will be more
        // specific than ones based on the file extension.
        .or_else(|| {
            file_name.clone().zip(
                file_name
                    .as_deref()
                    .and_then(|file_name| suggested_extensions().get(file_name)),
            )
        })
        .or_else(|| {
            file_extension.clone().zip(
                file_extension
                    .as_deref()
                    .and_then(|file_extension| suggested_extensions().get(file_extension)),
            )
        })?;

    Some(SuggestedExtension {
        extension_id: extension_id.clone(),
        display_name,
    })
}

fn suggested_extension_for_language(
    language_name: &LanguageName,
    suggestions: &[OptionalExtensionSuggestion],
    mut extension_is_eligible: impl FnMut(&str) -> bool,
) -> Option<SuggestedExtension> {
    suggestions
        .iter()
        .filter(|suggestion| suggestion.languages.contains(&language_name.as_ref()))
        .find(|suggestion| extension_is_eligible(suggestion.extension_id))
        .map(|suggestion| SuggestedExtension {
            extension_id: suggestion.extension_id.into(),
            display_name: language_name.as_ref().into(),
        })
}

fn extension_suggestion_key(extension_id: &str) -> String {
    format!("{}_extension_suggest", extension_id)
}

pub(crate) fn suggest_for_path(
    buffer: Entity<Buffer>,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    let Some(file) = buffer.read(cx).file().cloned() else {
        return;
    };

    let Some(SuggestedExtension {
        extension_id,
        display_name,
    }) = suggested_extension(file.path())
    else {
        return;
    };

    let key = extension_suggestion_key(&extension_id);
    let kvp = KeyValueStore::global(cx);
    let Ok(None) = kvp.read_kvp(&key) else {
        return;
    };

    show_suggestion(buffer, extension_id, display_name, window, cx);
}

pub(crate) fn suggest_for_language(
    buffer: Entity<Buffer>,
    language_name: &LanguageName,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    let extension_store = ExtensionStore::global(cx);
    let extension_store = extension_store.read(cx);
    let extension_settings = ExtensionSettings::get_global(cx);
    let kvp = KeyValueStore::global(cx);

    let Some(SuggestedExtension {
        extension_id,
        display_name,
    }) = suggested_extension_for_language(
        language_name,
        OPTIONAL_EXTENSION_SUGGESTIONS,
        |extension_id| {
            !extension_store
                .installed_extensions()
                .contains_key(extension_id)
                && !extension_store
                    .outstanding_operations()
                    .contains_key(extension_id)
                && extension_settings
                    .auto_install_extensions
                    .get(extension_id)
                    .copied()
                    != Some(true)
                && matches!(
                    kvp.read_kvp(&extension_suggestion_key(extension_id)),
                    Ok(None)
                )
        },
    )
    else {
        return;
    };

    show_suggestion(buffer, extension_id, display_name, window, cx);
}

fn show_suggestion(
    buffer: Entity<Buffer>,
    extension_id: Arc<str>,
    display_name: Arc<str>,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    cx.on_next_frame(window, move |workspace, _, cx| {
        let Some(editor) = workspace.active_item_as::<Editor>(cx) else {
            return;
        };

        if editor.read(cx).buffer().read(cx).as_singleton().as_ref() != Some(&buffer) {
            return;
        }

        struct ExtensionSuggestionNotification;

        let notification_id = NotificationId::composite::<ExtensionSuggestionNotification>(
            SharedString::from(extension_id.clone()),
        );

        workspace.show_notification(notification_id, cx, |cx| {
            cx.new(move |cx| {
                MessageNotification::new(
                    format!(
                        "Do you want to install the recommended '{}' extension for '{}' files?",
                        extension_id, display_name
                    ),
                    cx,
                )
                .primary_message("Yes, install extension")
                .primary_icon(IconName::Check)
                .primary_icon_color(Color::Success)
                .primary_on_click({
                    let extension_id = extension_id.clone();
                    move |_window, cx| {
                        let extension_id = extension_id.clone();
                        let extension_store = ExtensionStore::global(cx);
                        extension_store.update(cx, move |store, cx| {
                            store.install_latest_extension(extension_id, cx);
                        });
                    }
                })
                .secondary_message("No, don't install it")
                .secondary_icon(IconName::Close)
                .secondary_icon_color(Color::Error)
                .secondary_on_click(move |_window, cx| {
                    let key = extension_suggestion_key(&extension_id);
                    let kvp = KeyValueStore::global(cx);
                    cx.background_spawn(async move {
                        kvp.write_kvp(key, "dismissed".to_string()).await.log_err()
                    })
                    .detach();
                })
            })
        });
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use util::rel_path::rel_path;

    #[test]
    pub fn test_suggested_extension() {
        assert_eq!(
            suggested_extension(rel_path("Cargo.toml")),
            Some(SuggestedExtension {
                extension_id: "toml".into(),
                display_name: "toml".into()
            })
        );
        assert_eq!(
            suggested_extension(rel_path("Cargo.lock")),
            Some(SuggestedExtension {
                extension_id: "toml".into(),
                display_name: "Cargo.lock".into()
            })
        );
        assert_eq!(
            suggested_extension(rel_path("Dockerfile")),
            Some(SuggestedExtension {
                extension_id: "dockerfile".into(),
                display_name: "Dockerfile".into()
            })
        );
        assert_eq!(
            suggested_extension(rel_path("a/b/c/d/.gitignore")),
            Some(SuggestedExtension {
                extension_id: "git-firefly".into(),
                display_name: ".gitignore".into()
            })
        );
        assert_eq!(
            suggested_extension(rel_path("a/b/c/d/test.gleam")),
            Some(SuggestedExtension {
                extension_id: "gleam".into(),
                display_name: "gleam".into()
            })
        );
    }

    #[test]
    fn test_suggested_extension_for_language() {
        for language_name in [
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
        ] {
            assert_eq!(
                suggested_extension_for_language(
                    &LanguageName::new(language_name),
                    OPTIONAL_EXTENSION_SUGGESTIONS,
                    |_| true,
                ),
                Some(SuggestedExtension {
                    extension_id: "emmet".into(),
                    display_name: language_name.into(),
                })
            );
        }

        assert_eq!(
            suggested_extension_for_language(
                &LanguageName::new("Rust"),
                OPTIONAL_EXTENSION_SUGGESTIONS,
                |_| true,
            ),
            None
        );
    }

    #[test]
    fn test_suggested_extension_for_language_uses_first_eligible_suggestion() {
        let suggestions = [
            OptionalExtensionSuggestion {
                extension_id: "first",
                languages: &["HTML"],
            },
            OptionalExtensionSuggestion {
                extension_id: "second",
                languages: &["HTML"],
            },
        ];

        assert_eq!(
            suggested_extension_for_language(
                &LanguageName::new("HTML"),
                &suggestions,
                |extension_id| extension_id != "first",
            ),
            Some(SuggestedExtension {
                extension_id: "second".into(),
                display_name: "HTML".into(),
            })
        );
    }
}
