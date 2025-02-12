use client::{Client, UserStore};
use collections::HashMap;
use editor::{Editor, EditorMode};
use feature_flags::{FeatureFlagAppExt, PredictEditsFeatureFlag};
use gpui::{AnyWindowHandle, App, Context, Entity, WeakEntity};
use language::language_settings::{all_language_settings, EditPredictionProvider};
use settings::SettingsStore;
use std::{cell::RefCell, rc::Rc, sync::Arc};
use ui::Window;

pub fn init(client: Arc<Client>, user_store: Entity<UserStore>, cx: &mut App) {
    let editors: Rc<RefCell<HashMap<WeakEntity<Editor>, AnyWindowHandle>>> = Rc::default();
    cx.observe_new({
        let editors = editors.clone();
        let client = client.clone();
        let user_store = user_store.clone();
        move |editor: &mut Editor, window, cx: &mut Context<Editor>| {
            if editor.mode() != EditorMode::Full {
                return;
            }

            let Some(window) = window else {
                return;
            };

            let editor_handle = cx.entity().downgrade();
            cx.on_release({
                let editor_handle = editor_handle.clone();
                let editors = editors.clone();
                move |_, _| {
                    editors.borrow_mut().remove(&editor_handle);
                }
            })
            .detach();

            editors
                .borrow_mut()
                .insert(editor_handle, window.window_handle());
            let provider = all_language_settings(None, cx).edit_predictions.provider;
            assign_edit_prediction_provider(
                editor,
                provider,
                &client,
                user_store.clone(),
                window,
                cx,
            );
        }
    })
    .detach();

    let mut provider = all_language_settings(None, cx).edit_predictions.provider;
    for (editor, window) in editors.borrow().iter() {
        _ = window.update(cx, |_window, window, cx| {
            _ = editor.update(cx, |editor, cx| {
                assign_edit_prediction_provider(
                    editor,
                    provider,
                    &client,
                    user_store.clone(),
                    window,
                    cx,
                );
            })
        });
    }

    cx.observe_flag::<PredictEditsFeatureFlag, _>({
        let editors = editors.clone();
        let client = client.clone();
        let user_store = user_store.clone();
        move |_, cx| {
            let provider = all_language_settings(None, cx).edit_predictions.provider;
            assign_edit_prediction_providers(&editors, provider, &client, user_store.clone(), cx);
        }
    })
    .detach();

    cx.observe_global::<SettingsStore>({
        let editors = editors.clone();
        let client = client.clone();
        let user_store = user_store.clone();
        move |cx| {
            let new_provider = all_language_settings(None, cx).edit_predictions.provider;

            if new_provider != provider {
                provider = new_provider;
                assign_edit_prediction_providers(
                    &editors,
                    provider,
                    &client,
                    user_store.clone(),
                    cx,
                );
                match provider {
                    EditPredictionProvider::None => {}
                }
            }
        }
    })
    .detach();
}

fn assign_edit_prediction_providers(
    editors: &Rc<RefCell<HashMap<WeakEntity<Editor>, AnyWindowHandle>>>,
    provider: EditPredictionProvider,
    client: &Arc<Client>,
    user_store: Entity<UserStore>,
    cx: &mut App,
) {
    for (editor, window) in editors.borrow().iter() {
        _ = window.update(cx, |_window, window, cx| {
            _ = editor.update(cx, |editor, cx| {
                assign_edit_prediction_provider(
                    editor,
                    provider,
                    &client,
                    user_store.clone(),
                    window,
                    cx,
                );
            })
        });
    }
}

fn assign_edit_prediction_provider(
    _editor: &mut Editor,
    provider: EditPredictionProvider,
    _client: &Arc<Client>,
    _user_store: Entity<UserStore>,
    _window: &mut Window,
    _cx: &mut Context<Editor>,
) {
    // TODO: Do we really want to collect data? No.

    match provider {
        EditPredictionProvider::None => {}
    }
}
