use std::sync::Arc;

use editor::{Editor, EditorMode};
use gpui::{AppContext, ViewContext, VisualContext};
use language::Language;
use workspace::{Workspace, notifications::simple_message_notification};
use language::Event;

use crate::LanguageExtensions;


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

fn check_and_suggest<T>(cx: &mut ViewContext<T>, language: Arc<Language>) {
    suggest_extensions(cx, language);
}

fn suggest_extensions<T>(cx: &mut ViewContext<T>, language: Arc<Language>) -> Option<()> {
    let workspace = cx.windows().get(0)?.downcast::<Workspace>()?; // for some reason cx.active_window() returns none

    let _ = workspace
        .update(cx, |workspace, cx| {
            workspace.show_notification(0, cx, |cx| {


                cx.new_view(move |_cx| {
                    simple_message_notification::MessageNotification::new(
                        format!("Language Extensions for {language} not installed", language = language.name())
                    )
                        .with_click_message("View Extensions for Language")
                        .on_click(move |cx| {

                            cx.dispatch_action(Box::new(LanguageExtensions{
                                language_string: language.name().to_string().into()
                            }));

                        })
                        .with_secondary_click_message("Install")
                })
            })
        });

    Some(())
}
