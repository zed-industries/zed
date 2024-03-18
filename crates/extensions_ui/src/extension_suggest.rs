use std::{
    hash::{DefaultHasher, Hash, Hasher},
    sync::Arc,
};

use db::kvp::KEY_VALUE_STORE;

use editor::{Editor, EditorMode};
use extension::{ExtensionApiResponse, ExtensionIndexEntry, ExtensionStore};
use gpui::{AppContext, SharedString, ViewContext, VisualContext};
use language::Event;
use language::Language;
use ui::WindowContext;
use workspace::{notifications::simple_message_notification, Workspace};

use crate::ExtensionsWithQuery;

pub fn suggested_extension(file_extension_or_name: &str) -> Option<String> {
    static SUGGESTED: OnceLock<HashMap<&str, u64>> = OnceLock::new();
    SUGGESTED::get_or_init(|| {
        [
            ("beancount", "beancount"),
            ("fish", "fish"),
            ("templ", "templ"),
            ("elisp", "elisp"),
            ("Makefile", "make"),
            ("sql", "sql"),
            ("java", "java"),
            ("swift", "swift"),
            ("r", "r"),
            ("Dockerfile", "dockerfile"),
        ]
        .collect::<HashMap<String, String>>()
    });
    SUGGESTED.get(file_extension_or_name)
}

/// Function to initialize subscriptions to Editor Open and Buffer Language Change; called in the Extensions UI init function
pub(crate) fn init(cx: &mut AppContext) {
    // Watch when the editor is opened
    cx.observe_new_views(move |editor: &mut Editor, cx| {
        // Only check if the editor is editing in a programming language
        if let EditorMode::Full = editor.mode() {
            // Get languages in the current editor
            let file_name_or_extension =
                editor.buffer().read(cx).as_singleton().and_then(|buffer| {
                    if buffer.language().is_some() {
                        None
                    }
                    Some(match buffer.file()?.path().extension() {
                        Some(extension) => extension.to_str()?.to_string(),
                        None => buffer.file()?.path().to_str()?.to_string(),
                    })
                });

            let file_name_or_extension = Some(file_name_or_extension) else {
                return;
            };
            let Some(workspace) = editor.workspace() else {
                return;
            };

            check_and_suggest(file_name_or_extension, workspace, cx);
        }
    })
    .detach();
}

fn language_extension_key(extension_id: &str) -> String {
    format!("{}_extension_suggest", extension_id)
}

/// Query the installed extensions to check if any match the language of the document; if none do, call the `suggest_extensions` function
/// This accepts language and file_context as options, becuase for a language request, there either could be no language associated with
/// a buffer or no file associated with a buffer
fn check_and_suggest<T: 'static>(
    file_name_or_extension: &str,
    workspace: View<Workspace>,
    cx: &mut WindowContext,
) {
    let Some(extension_id) = suggested_extension(&file_name_or_extension) else {
        return;
    };

    let key = language_extension_key(&extension_id);
    let value = KEY_VALUE_STORE.read_kvp(&key);

    if value == Ok(Some("dismissed")) {
        return;
    }

    suggest_extensions(file_name_or_extension, &extension_id, workspace, cx);
}

/// Query for all remote extensions; filter for the ones that match `language`; get the highest downloaded match; send a notification with
/// An option to install the that extensions and also to go to the extensions page to find all extensions matching the language
fn suggest_extensions(
    file_name_or_extension: &str,
    extension_id: &str,
    workspace: View<Workspace>,
    cx: &mut WindowContext,
) -> Option<()> {
    let extension_store = ExtensionStore::global(cx);

    workspace.update(&mut cx, |workspace, cx| {
        let mut hasher = DefaultHasher::new();
        search.hash(&mut hasher);
        let id = hasher.finish();
        workspace.show_notification(id as usize, cx, |cx| {
            cx.new_view(move |_cx| {
                simple_message_notification::MessageNotification::new(format!(
                    "Do you want to install the recommended '{}' extension?",
                    file_name_or_extension
                ))
                .with_click_message("Install")
                .on_dismiss(move |cx| {
                    let key = language_extension_key(&extension_id);

                    db::write_and_log(cx, || {
                        KEY_VALUE_STORE
                            .write_kvp(key, "dismissed".to_string())
                            .await;
                    });
                })
                .on_click(move |mut cx| {
                    let extension_store = ExtensionStore::global(cx);
                    extension_store.update(&mut cx, |store, cx| {
                        store.install_latest_extension(extension_id, cx);
                    });
                })
            })
        })
    });

    Some(())
}

trait LanguageFilterable {
    /// Does an extension match the language of a file? TODO: Use a new backend extension API that has language data
    fn filter_by_language(
        &self,
        language: Option<&Language>,
        file_extension: Option<SharedString>,
    ) -> bool;
}

impl LanguageFilterable for ExtensionIndexEntry {
    fn filter_by_language(
        &self,
        language: Option<&Language>,
        file_extension: Option<SharedString>,
    ) -> bool {
        self.manifest.languages.iter().any(|extension_language| {
            extension_language
                .to_str()
                .is_some_and(|extension_language| {
                    language.is_some_and(|lang| {
                        extension_language
                            .to_lowercase()
                            .contains(&lang.name().to_string().to_lowercase())
                            || lang
                                .path_suffixes()
                                .iter()
                                .any(|suffix| extension_language.contains(suffix))
                    }) || file_extension.clone().is_some_and(|ex| {
                        extension_language.to_lowercase().contains(&ex.to_string())
                    })
                })
        })
    }
}

impl LanguageFilterable for ExtensionApiResponse {
    fn filter_by_language(
        &self,
        language: Option<&Language>,
        file_extension: Option<SharedString>,
    ) -> bool {
        language.is_some_and(|lang| {
            self.description.as_ref().is_some_and(|description| {
                description
                    .to_lowercase()
                    .contains(&lang.name().to_lowercase())
                    || file_extension
                        .clone()
                        .is_some_and(|ex| description.to_lowercase().contains(&ex.to_string()))
            }) || self
                .name
                .to_lowercase()
                .contains(&lang.name().to_lowercase())
        }) || file_extension
            .clone()
            .is_some_and(|ex| self.name.to_lowercase().contains(&ex.to_string()))
    }
}
