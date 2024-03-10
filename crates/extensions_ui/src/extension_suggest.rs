use std::sync::Arc;

use anyhow::anyhow;
use editor::{Editor, EditorMode};
use extension::{ExtensionStore, ExtensionApiResponse};
use gpui::{AppContext, ViewContext, VisualContext, Context};
use language::Language;
use workspace::{Workspace, notifications::simple_message_notification};
use language::Event;

use crate::ExtensionsWithQuery;


pub(crate) fn init(cx: &mut AppContext) {

    // Watch when the editor is opened
    cx.observe_new_views(move |editor: &mut Editor, cx| {

        // Only if the editor is editing in a programming language
        if let EditorMode::Full = editor.mode() {

            // Get languages in the current editor
            let languages = editor
                .buffer()
                .read(cx)
                .all_buffers()
                .iter()
                .flat_map(|buffer| 
                    buffer
                        .read(cx)
                        .language()
                        .cloned()
                ).collect::<Vec<_>>();

            for language in languages {
                check_and_suggest(cx, language)
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

                            println!("Language changed/set to {:?}", language_option);

                            if let Some(language) = language_option.cloned() {

                                check_and_suggest(cx, language);
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

fn check_and_suggest<T: 'static>(cx: &mut ViewContext<T>, language: Arc<Language>) {

    let extension_store = ExtensionStore::global(cx);

    let store = extension_store.read(cx);

    let installed_extensions = &store.installed_extensions().extensions;

    // check if any extensions support the current language; this searches just the descriptions as a sort of hack right now
    let check = installed_extensions.iter()
        .any(|(_, ext)| 
            ext.manifest.description.as_ref().map(|description| description.contains(&language.name().to_lowercase())) == Some(true)
        );

    if !check {
        suggest_extensions(cx, language);
    }
}

fn suggest_extensions<T: 'static>(cx: &mut ViewContext<T>, language: Arc<Language>) -> Option<()> {
    let extension_store = ExtensionStore::global(cx);


    let remote_extensions_task = extension_store.update(cx, |store, cx| {
        store.fetch_extensions(Some(language.name().to_string().as_str()), cx)
    });

    cx.spawn(|_view, mut cx| async move {

        // Get the most downloaded extension for the language
        let all_extensions = remote_extensions_task.await.ok()?;


        let most_downloaded = all_extensions
            .into_iter()
            .filter(|extension| extension.description.clone().is_some_and(|description| description.contains(&language.name().to_string())))
            .max_by_key(|extension| extension.download_count)?;

        // Make the notification

        let workspace = cx.window_handle().downcast::<Workspace>()?;

        let _ = workspace.update(&mut cx, |workspace, cx| {

            workspace.show_notification(0, cx, |cx| {
                cx.new_view(move |_cx| {
                    simple_message_notification::MessageNotification::new(
                        format!("Language Extensions for {language} not installed", language = language.name())
                    )
                        .with_click_message("View Extensions")
                        .on_click(move |cx| {

                            cx.dispatch_action(Box::new(ExtensionsWithQuery{
                                query: language.name().to_string().into()
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

        Some(())
    }).detach();

    Some(())
}
