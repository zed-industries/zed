use std::collections::HashMap;
use std::sync::{Arc, OnceLock};

use db::kvp::KeyValueStore;
use editor::Editor;
use extension_host::ExtensionStore;
use gpui::{AppContext as _, Context, Entity, Global, SharedString, Window};
use language::Buffer;
use markdown::{Markdown, MarkdownElement};
use ui::prelude::*;
use util::ResultExt;
use util::rel_path::RelPath;
use workspace::Workspace;
use workspace::notifications::{
    NotificationId, markdown_style, simple_message_notification::MessageNotification,
};

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

const EMMET_EXTENSION_ID: &str = "emmet";
const EMMET_SUPPORTED_LANGUAGES: &[&str] = &[
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
];

#[derive(Default)]
struct EmmetSuggestionState {
    dismissed: bool,
}

impl Global for EmmetSuggestionState {}

struct EmmetSuggestionNotification;

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
    pub file_name_or_extension: Arc<str>,
}

/// Returns the suggested extension for the given [`Path`].
fn suggested_extension(path: &RelPath) -> Option<SuggestedExtension> {
    let file_extension: Option<Arc<str>> = path.extension().map(|extension| extension.into());
    let file_name: Option<Arc<str>> = path.file_name().map(|name| name.into());

    let (file_name_or_extension, extension_id) = None
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
        file_name_or_extension,
    })
}

fn language_extension_key(extension_id: &str) -> String {
    format!("{}_extension_suggest", extension_id)
}

pub(crate) fn suggest(buffer: Entity<Buffer>, window: &mut Window, cx: &mut Context<Workspace>) {
    let Some(file) = buffer.read(cx).file().cloned() else {
        return;
    };

    let Some(SuggestedExtension {
        extension_id,
        file_name_or_extension,
    }) = suggested_extension(file.path())
    else {
        return;
    };

    let key = language_extension_key(&extension_id);
    let kvp = KeyValueStore::global(cx);
    let Ok(None) = kvp.read_kvp(&key) else {
        return;
    };

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
                        extension_id, file_name_or_extension
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
                    let key = language_extension_key(&extension_id);
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

pub(crate) fn suggest_emmet(
    workspace: &mut Workspace,
    buffer: Entity<Buffer>,
    cx: &mut Context<Workspace>,
) {
    let supported = buffer
        .read(cx)
        .language()
        .is_some_and(|language| EMMET_SUPPORTED_LANGUAGES.contains(&language.name().as_ref()));
    if !supported {
        return;
    }

    let Some(editor) = workspace.active_item_as::<Editor>(cx) else {
        return;
    };
    if editor.read(cx).buffer().read(cx).as_singleton().as_ref() != Some(&buffer) {
        return;
    }

    let extension_store = ExtensionStore::global(cx);
    let extension_store = extension_store.read(cx);
    if extension_store
        .installed_extensions()
        .contains_key(EMMET_EXTENSION_ID)
        || extension_store
            .outstanding_operations()
            .contains_key(EMMET_EXTENSION_ID)
    {
        return;
    }

    if cx.default_global::<EmmetSuggestionState>().dismissed {
        return;
    }

    let key = language_extension_key(EMMET_EXTENSION_ID);
    let kvp = KeyValueStore::global(cx);
    let Some(dismissal) = kvp.read_kvp(&key).log_err() else {
        return;
    };
    if dismissal.is_some() {
        cx.default_global::<EmmetSuggestionState>().dismissed = true;
        return;
    }

    workspace.show_notification(
        NotificationId::unique::<EmmetSuggestionNotification>(),
        cx,
        |cx| {
            cx.new(|cx| {
                let markdown = cx.new(|cx| {
                    Markdown::new(
                        SharedString::new_static(
                            "Emmet expands abbreviations such as `ul>li*3` into HTML and `m10` into CSS.",
                        ),
                        None,
                        None,
                        cx,
                    )
                });
                MessageNotification::new_from_builder(cx, move |window, cx| {
                    MarkdownElement::new(markdown.clone(), markdown_style(window, cx))
                        .text_size(TextSize::Default.rems(cx))
                        .into_any_element()
                })
                .with_title("Emmet is available for this file")
                .more_info_message("Learn more")
                .more_info_url("https://zed.dev/docs/languages/emmet")
                .primary_message("Install Emmet")
                .primary_icon(IconName::Check)
                .primary_icon_color(Color::Success)
                .primary_on_click(|_window, cx| {
                    ExtensionStore::global(cx).update(cx, |store, cx| {
                        store.install_latest_extension(Arc::from(EMMET_EXTENSION_ID), cx);
                    });
                })
                .secondary_message("Don't show again")
                .secondary_icon(IconName::Close)
                .secondary_icon_color(Color::Error)
                .secondary_on_click(|_window, cx| {
                    cx.default_global::<EmmetSuggestionState>().dismissed = true;
                    let key = language_extension_key(EMMET_EXTENSION_ID);
                    let kvp = KeyValueStore::global(cx);
                    cx.background_spawn(async move {
                        kvp.write_kvp(key, "dismissed".to_string()).await.log_err()
                    })
                    .detach();
                })
            })
        },
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use extension::ExtensionHostProxy;
    use extension_host::RELOAD_DEBOUNCE_DURATION;
    use fs::FakeFs;
    use gpui::{TestAppContext, VisualTestContext};
    use language::{Language, LanguageConfig, LanguageMatcher};
    use project::{Project, lsp_store::LspStoreEvent};
    use serde_json::json;
    use settings::SettingsStore;
    use util::path;
    use util::rel_path::rel_path;
    use workspace::AppState;

    #[test]
    pub fn test_suggested_extension() {
        assert_eq!(
            suggested_extension(rel_path("Cargo.toml")),
            Some(SuggestedExtension {
                extension_id: "toml".into(),
                file_name_or_extension: "toml".into()
            })
        );
        assert_eq!(
            suggested_extension(rel_path("Cargo.lock")),
            Some(SuggestedExtension {
                extension_id: "toml".into(),
                file_name_or_extension: "Cargo.lock".into()
            })
        );
        assert_eq!(
            suggested_extension(rel_path("Dockerfile")),
            Some(SuggestedExtension {
                extension_id: "dockerfile".into(),
                file_name_or_extension: "Dockerfile".into()
            })
        );
        assert_eq!(
            suggested_extension(rel_path("a/b/c/d/.gitignore")),
            Some(SuggestedExtension {
                extension_id: "git-firefly".into(),
                file_name_or_extension: ".gitignore".into()
            })
        );
        assert_eq!(
            suggested_extension(rel_path("a/b/c/d/test.gleam")),
            Some(SuggestedExtension {
                extension_id: "gleam".into(),
                file_name_or_extension: "gleam".into()
            })
        );
    }

    #[gpui::test]
    async fn test_emmet_is_suggested_for_supported_language(cx: &mut TestAppContext) {
        let fs = init_test(cx);
        let (workspace, cx) = open_test_workspace(fs, cx).await;

        open_file(&workspace, "index.html", cx).await;

        assert_eq!(
            notification_ids(&workspace, cx),
            vec![NotificationId::unique::<EmmetSuggestionNotification>()]
        );
    }

    #[gpui::test]
    async fn test_emmet_is_suggested_when_language_is_detected_after_editor(
        cx: &mut TestAppContext,
    ) {
        let fs = init_test(cx);
        let (workspace, cx) = open_test_workspace(fs, cx).await;

        open_file(&workspace, "main.rs", cx).await;
        assert_eq!(notification_ids(&workspace, cx), Vec::new());

        let (buffer, lsp_store) = workspace.read_with(cx, |workspace, cx| {
            let editor = workspace.active_item_as::<Editor>(cx).unwrap();
            let buffer = editor.read(cx).buffer().read(cx).as_singleton().unwrap();
            let lsp_store = workspace.project().read(cx).lsp_store();
            (buffer, lsp_store)
        });
        let language = Arc::new(test_language("HTML", "html"));
        cx.update(|_, cx| {
            buffer.update(cx, |buffer, cx| {
                buffer.set_language(Some(language.clone()), cx)
            });
            lsp_store.update(cx, |_, cx| {
                cx.emit(LspStoreEvent::LanguageDetected {
                    buffer,
                    new_language: Some(language),
                });
            });
        });

        assert_eq!(
            notification_ids(&workspace, cx),
            vec![NotificationId::unique::<EmmetSuggestionNotification>()]
        );
    }

    #[gpui::test]
    async fn test_emmet_is_not_suggested_for_unsupported_language(cx: &mut TestAppContext) {
        let fs = init_test(cx);
        let (workspace, cx) = open_test_workspace(fs, cx).await;

        open_file(&workspace, "main.rs", cx).await;

        assert_eq!(notification_ids(&workspace, cx), Vec::new());
    }

    #[gpui::test]
    async fn test_emmet_is_not_suggested_after_dismissal(cx: &mut TestAppContext) {
        let fs = init_test(cx);
        let kvp = cx.update(|cx| KeyValueStore::global(cx));
        kvp.write_kvp(
            language_extension_key(EMMET_EXTENSION_ID),
            "dismissed".to_string(),
        )
        .await
        .unwrap();
        let (workspace, cx) = open_test_workspace(fs, cx).await;

        open_file(&workspace, "index.html", cx).await;

        assert_eq!(notification_ids(&workspace, cx), Vec::new());
        assert!(cx.update(|_, cx| cx.default_global::<EmmetSuggestionState>().dismissed));
    }

    #[gpui::test]
    async fn test_emmet_is_not_suggested_from_cached_dismissal(cx: &mut TestAppContext) {
        let fs = init_test(cx);
        cx.update(|cx| {
            cx.default_global::<EmmetSuggestionState>().dismissed = true;
        });
        let (workspace, cx) = open_test_workspace(fs, cx).await;

        open_file(&workspace, "index.html", cx).await;

        assert_eq!(notification_ids(&workspace, cx), Vec::new());
    }

    #[gpui::test]
    fn test_emmet_dismissal_cache_is_scoped_to_app(cx: &mut TestAppContext) {
        cx.update(|cx| {
            cx.default_global::<EmmetSuggestionState>().dismissed = true;
        });

        let other_app = cx.new_app();

        assert!(!other_app.update(|cx| cx.default_global::<EmmetSuggestionState>().dismissed));
    }

    #[gpui::test]
    async fn test_emmet_is_not_suggested_when_installed(cx: &mut TestAppContext) {
        let fs = init_test(cx);
        fs.insert_tree(
            paths::extensions_dir().join("installed"),
            json!({
                "emmet": {
                    "extension.toml": r#"
                        id = "emmet"
                        name = "Emmet"
                        version = "0.0.14"
                        schema_version = 1
                    "#
                }
            }),
        )
        .await;
        cx.executor().advance_clock(RELOAD_DEBOUNCE_DURATION);
        cx.run_until_parked();
        cx.update(|cx| {
            assert_eq!(
                ExtensionStore::global(cx)
                    .read(cx)
                    .installed_extensions()
                    .keys()
                    .collect::<Vec<_>>(),
                vec![&Arc::from(EMMET_EXTENSION_ID)]
            );
        });
        let (workspace, cx) = open_test_workspace(fs, cx).await;

        open_file(&workspace, "index.html", cx).await;

        assert_eq!(notification_ids(&workspace, cx), Vec::new());
    }

    fn init_test(cx: &mut TestAppContext) -> Arc<FakeFs> {
        cx.update(|cx| {
            let app_state = AppState::test(cx);
            cx.update_global::<SettingsStore, _>(|store, cx| {
                store.update_user_settings(cx, |content| {
                    content
                        .extension
                        .auto_install_extensions
                        .insert(Arc::from("html"), false);
                });
            });
            cx.set_global(db::AppDatabase::test_new());
            release_channel::init(semver::Version::new(0, 0, 0), cx);
            extension::init(cx);
            extension_host::init(
                Arc::new(ExtensionHostProxy::new()),
                app_state.fs.clone(),
                app_state.client.clone(),
                app_state.node_runtime.clone(),
                cx,
            );
            editor::init(cx);
            crate::init(cx);
            app_state.fs.as_fake()
        })
    }

    async fn open_test_workspace(
        fs: Arc<FakeFs>,
        cx: &mut TestAppContext,
    ) -> (Entity<Workspace>, &mut VisualTestContext) {
        fs.insert_tree(
            path!("/root"),
            json!({
                "index.html": "<div></div>",
                "main.rs": "fn main() {}",
            }),
        )
        .await;
        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        project.read_with(cx, |project, _| {
            project
                .languages()
                .add(Arc::new(test_language("HTML", "html")));
            project
                .languages()
                .add(Arc::new(test_language("Rust", "rs")));
        });
        cx.add_window_view(|window, cx| Workspace::test_new(project, window, cx))
    }

    async fn open_file(workspace: &Entity<Workspace>, file_name: &str, cx: &mut VisualTestContext) {
        let worktree_id = workspace.read_with(cx, |workspace, cx| {
            workspace
                .project()
                .read(cx)
                .worktrees(cx)
                .next()
                .unwrap()
                .read(cx)
                .id()
        });
        workspace
            .update_in(cx, |workspace, window, cx| {
                workspace.open_path((worktree_id, rel_path(file_name)), None, true, window, cx)
            })
            .await
            .unwrap();
        cx.run_until_parked();
    }

    fn notification_ids(
        workspace: &Entity<Workspace>,
        cx: &VisualTestContext,
    ) -> Vec<NotificationId> {
        workspace.read_with(cx, |workspace, _| workspace.notification_ids())
    }

    fn test_language(name: &'static str, path_suffix: &str) -> Language {
        Language::new(
            LanguageConfig {
                name: name.into(),
                matcher: LanguageMatcher {
                    path_suffixes: vec![path_suffix.to_string()],
                    ..LanguageMatcher::default()
                }
                .into(),
                ..LanguageConfig::default()
            },
            None,
        )
    }
}
