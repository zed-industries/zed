use std::{sync::Arc, hash::{DefaultHasher, Hash, Hasher}};

use db::kvp::KEY_VALUE_STORE;

use editor::{Editor, EditorMode};
use extension::{ExtensionStore, ExtensionApiResponse, ExtensionIndexEntry};
use gpui::{AppContext, ViewContext, VisualContext, SharedString};
use language::Language;
use workspace::{Workspace, notifications::simple_message_notification};
use language::Event;

use crate::ExtensionsWithQuery;


/// Function to initialize subscriptions to Editor Open and Buffer Language Change; called in the Extensions UI init function
pub(crate) fn init(cx: &mut AppContext) {

    // Watch when the editor is opened
    cx.observe_new_views(move |editor: &mut Editor, cx| {

        // Only check if the editor is editing in a programming language
        if let EditorMode::Full = editor.mode() {


            // Get languages in the current editor
            let editor_language_info = editor
                .buffer()
                .read(cx)
                .all_buffers()
                .iter()
                .flat_map(|buffer| {
                    let buffer = buffer.read(cx);

                    // Get the file extension or file name if there is no extension
                    Some((
                        match buffer.file()?.path().extension() {
                            Some(extension) => SharedString::from(extension.to_str()?.to_string()),
                            None => SharedString::from(buffer.file()?.path().to_str()?.to_string())
                    },
                        buffer.language().cloned()
                    ))
                }).collect::<Vec<_>>();

            // Check extensions for all languages
            for (file_name_or_extension, language) in editor_language_info {
                check_and_suggest(cx, language, Some(file_name_or_extension))
            }


            // Listen to language changes; If a user changes to an unsupported language, extension suggestions should happen
            let buffers = editor.buffer();
            let buffers = buffers.read(cx);
            for buffer_model in buffers.all_buffers().iter() {
                cx.subscribe(buffer_model, move |_subscriber, emitter, event, cx| {

                    match event {
                        Event::LanguageChanged => {
                            let buffer = emitter.read(cx);
                            let language_option = buffer.language();

                            if let Some(language) = language_option.cloned() {

                                check_and_suggest(cx, Some(language), None);
                            }

                        },
                        _ => {}
                    }

                }).detach();


            }

        }
    })
        .detach();
}

fn language_extension_key(file_context: &str) -> String {
    format!("{}_extension_suggest", file_context)
}


/// Query the installed extensions to check if any match the language of the document; if none do, call the `suggest_extensions` function
/// This accepts language and file_context as options, becuase for a language request, there either could be no language associated with
/// a buffer or no file associated with a buffer
fn check_and_suggest<T: 'static>(cx: &mut ViewContext<T>, language: Option<Arc<Language>>, file_context: Option<SharedString>) {
    if let Some(ref f_context) = file_context {
        let key = language_extension_key(f_context);

        let value = KEY_VALUE_STORE.read_kvp(&key);

        if let Ok(Some(value)) = value {
            if value == "some" {
                return
            }
        }
    }


    if let Some(ref language) = language {
        let key = language_extension_key(&language.name());

        let value = KEY_VALUE_STORE.read_kvp(&key);

        if let Ok(Some(value)) = value {
            if value == "some" {
                return
            }
        }
    }

    let extension_store = ExtensionStore::global(cx);

    let store = extension_store.read(cx);

    let installed_extensions = &store.installed_extensions().extensions;

    // check if any extensions support the current language; this searches just the descriptions as a sort of hack right now
    let check = installed_extensions.iter()
        .any(|(_, ext)| 
            ext.filter_by_language(language.as_deref(), file_context.clone())
        );

    if !check {
        suggest_extensions(cx, language, file_context);
    }
}

/// Query for all remote extensions; filter for the ones that match `language`; get the highest downloaded match; send a notification with
/// An option to install the that extensions and also to go to the extensions page to find all extensions matching the language
fn suggest_extensions<T: 'static>(cx: &mut ViewContext<T>, language: Option<Arc<Language>>, file_context: Option<SharedString>) -> Option<()> {
    let extension_store = ExtensionStore::global(cx);

    let search = match (language.clone(), file_context.clone()) {
        (Some(language), _) => Some(language.name().to_string()),
        (None, Some(extension)) => Some(extension.to_string()),
        (None, None) => return None
    }?;

    let remote_extensions_task = extension_store.update(cx, |store, cx| {
        store.fetch_extensions(None, cx)
    });

    cx.spawn(|_view, mut cx| async move {

        // Get the most downloaded extension for the language
        let all_extensions = remote_extensions_task.await.ok()?;

        let most_downloaded = all_extensions
            .into_iter()
            .filter(|extension| 
                extension.filter_by_language(language.as_deref(), file_context.clone().map(Into::into))
            )
            .max_by_key(|extension| extension.download_count)?;


        // Make the notification

        let workspace = cx.window_handle().downcast::<Workspace>()?;

        let _ = workspace.update(&mut cx, |workspace, cx| {


            let mut hasher = DefaultHasher::new();
            search.hash(&mut hasher);
            let id = hasher.finish();
            workspace.show_notification(id as usize, cx, |cx| { // TODO: should we avoid using `as` here?
                cx.new_view(move |_cx| {
                    simple_message_notification::MessageNotification::new(
                        format!("Extensions for {language} not installed", language = search)
                    )
                        .with_click_message("View Extensions")
                        .on_click(move |cx| {

                            cx.dispatch_action(Box::new(ExtensionsWithQuery{
                                query: search.clone().into()
                            }));

                        })
                        .with_secondary_click_message(format!("Install {}", most_downloaded.name))
                        .secondary_on_click(move |cx| {
                            let extension_store = ExtensionStore::global(cx);
                            extension_store.update(cx, |store, cx| {
                                store.install_extension(most_downloaded.id.clone(), most_downloaded.version.clone(), cx)
                            });

                            cx.dispatch_action(Box::new(ExtensionsWithQuery{
                                query: most_downloaded.name.clone().into()
                            }));
                        })
                })
            })
        });

        // state in the local database that the language was suggested
        if let Some(ref f_context) = file_context {
            let _ = KEY_VALUE_STORE.write_kvp(language_extension_key(f_context), "some".into()).await;
        } 
        // write to both because either could be None; when checking for keys, we check for both
        if let Some(ref language) = language {
            let _ = KEY_VALUE_STORE.write_kvp(language_extension_key(&language.name()), "some".into()).await;
        }

        Some(())
    }).detach();

    Some(())
}


trait LanguageFilterable {
    /// Does an extension match the language of a file? TODO: Use a new backend extension API that has language data
    fn filter_by_language(&self, language: Option<&Language>, file_extension: Option<SharedString>) -> bool;
}

impl LanguageFilterable for ExtensionIndexEntry {
    fn filter_by_language(&self, language: Option<&Language>, file_extension: Option<SharedString>) -> bool {
        self.manifest
            .languages
            .iter()
            .any(|extension_language| 
                extension_language.to_str().is_some_and(|extension_language| 
                    language.is_some_and(|lang| 
                        extension_language.to_lowercase().contains(&lang.name().to_string().to_lowercase())
                        || lang.path_suffixes().iter().any(|suffix| extension_language.contains(suffix))
                    )
                || file_extension.clone().is_some_and(|ex| extension_language.to_lowercase().contains(&ex.to_string()))
            ))
    }
}

impl LanguageFilterable for ExtensionApiResponse {
    fn filter_by_language(&self, language: Option<&Language>, file_extension: Option<SharedString>) -> bool {
        language.is_some_and(|lang| 
            self.description
                .as_ref()
                .is_some_and(|description| 
                    description.to_lowercase().contains(&lang.name().to_lowercase())
                    || file_extension.clone().is_some_and(|ex| description.to_lowercase().contains(&ex.to_string()))
                )
            || self.name.to_lowercase().contains(&lang.name().to_lowercase())
        )
        || file_extension.clone().is_some_and(|ex| self.name.to_lowercase().contains(&ex.to_string()))

    }
}
