use std::sync::Arc;

use crate::{ZED_PREDICT_DATA_COLLECTION_CHOICE, onboarding_event};
use ai_onboarding::EditPredictionOnboarding;
use client::{Client, UserStore};
use fs::Fs;
use gpui::{
    ClickEvent, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, MouseDownEvent, Render,
    linear_color_stop, linear_gradient,
};
use language::language_settings::{AllLanguageSettings, EditPredictionProvider};
use settings::update_settings_file;
use ui::{Vector, VectorName, prelude::*};
use workspace::{ModalView, Workspace};

/// Introduces user to Zed's Edit Prediction feature and terms of service
pub struct ZedPredictModal {
    onboarding: Entity<EditPredictionOnboarding>,
    fs: Arc<dyn Fs>,
    focus_handle: FocusHandle,
    data_collection_expanded: bool,
}

pub(crate) fn set_edit_prediction_provider(provider: EditPredictionProvider, cx: &mut App) {
    let fs = <dyn Fs>::global(cx);
    update_settings_file::<AllLanguageSettings>(fs, cx, move |settings, _| {
        settings
            .features
            .get_or_insert(Default::default())
            .edit_prediction_provider = Some(provider);
    });
}

impl ZedPredictModal {
    pub fn toggle(
        workspace: &mut Workspace,
        user_store: Entity<UserStore>,
        client: Arc<Client>,
        fs: Arc<dyn Fs>,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        workspace.toggle_modal(window, cx, |_window, cx| {
            let weak_entity = cx.weak_entity();
            Self {
                onboarding: cx.new(|cx| {
                    EditPredictionOnboarding::new(
                        user_store.clone(),
                        client.clone(),
                        copilot::Copilot::global(cx)
                            .map_or(false, |copilot| copilot.read(cx).status().is_configured()),
                        Arc::new({
                            let this = weak_entity.clone();
                            move |_window, cx| {
                                set_edit_prediction_provider(EditPredictionProvider::Zed, cx);
                                this.update(cx, |_, cx| cx.emit(DismissEvent)).ok();
                            }
                        }),
                        Arc::new({
                            let this = weak_entity.clone();
                            move |window, cx| {
                                set_edit_prediction_provider(EditPredictionProvider::Copilot, cx);
                                this.update(cx, |_, cx| cx.emit(DismissEvent)).ok();
                                copilot::initiate_sign_in(window, cx);
                            }
                        }),
                        cx,
                    )
                }),
                fs,
                focus_handle: cx.focus_handle(),
                data_collection_expanded: false,
            }
        });
    }

    // fn inline_completions_doc(&mut self, _: &ClickEvent, _: &mut Window, cx: &mut Context<Self>) {
    //     cx.open_url("https://zed.dev/docs/configuring-zed#disabled-globs");
    //     cx.notify();

    //     onboarding_event!("Docs Link Clicked");
    // }

    // fn accept_and_enable(&mut self, _: &ClickEvent, window: &mut Window, cx: &mut Context<Self>) {
    //     let task = self
    //         .user_store
    //         .update(cx, |this, cx| this.accept_terms_of_service(cx));
    //     let fs = self.fs.clone();

    //     cx.spawn(async move |this, cx| {
    //         task.await?;

    //         let mut data_collection_opted_in = false;
    //         this.update(cx, |this, _cx| {
    //             data_collection_opted_in = this.data_collection_opted_in;
    //         })
    //         .ok();

    //         KEY_VALUE_STORE
    //             .write_kvp(
    //                 ZED_PREDICT_DATA_COLLECTION_CHOICE.into(),
    //                 data_collection_opted_in.to_string(),
    //             )
    //             .await
    //             .log_err();

    //         // Make sure edit prediction provider setting is using the new key
    //         let settings_path = paths::settings_file().as_path();
    //         let settings_path = fs.canonicalize(settings_path).await.with_context(|| {
    //             format!("Failed to canonicalize settings path {:?}", settings_path)
    //         })?;

    //         if let Some(settings) = fs.load(&settings_path).await.log_err() {
    //             if let Some(new_settings) =
    //                 migrator::migrate_edit_prediction_provider_settings(&settings)?
    //             {
    //                 fs.atomic_write(settings_path, new_settings).await?;
    //             }
    //         }

    //         this.update(cx, |this, cx| {
    //             update_settings_file::<AllLanguageSettings>(this.fs.clone(), cx, move |file, _| {
    //                 file.features
    //                     .get_or_insert(Default::default())
    //                     .edit_prediction_provider = Some(EditPredictionProvider::Zed);
    //             });

    //             cx.emit(DismissEvent);
    //         })
    //     })
    //     .detach_and_notify_err(window, cx);

    //     onboarding_event!(
    //         "Enable Clicked",
    //         data_collection_opted_in = self.data_collection_opted_in,
    //     );
    // }

    fn cancel(&mut self, _: &menu::Cancel, _: &mut Window, cx: &mut Context<Self>) {
        cx.emit(DismissEvent);
    }
}

impl EventEmitter<DismissEvent> for ZedPredictModal {}

impl Focusable for ZedPredictModal {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl ModalView for ZedPredictModal {}

impl Render for ZedPredictModal {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let window_height = window.viewport_size().height;
        let max_height = window_height - px(200.);

        v_flex()
            .id("edit-prediction-onboarding")
            .key_context("ZedPredictModal")
            .relative()
            .w(px(550.))
            .h_full()
            .max_h(max_height)
            .p_4()
            .gap_2()
            .when(self.data_collection_expanded, |element| {
                element.overflow_y_scroll()
            })
            .when(!self.data_collection_expanded, |element| {
                element.overflow_hidden()
            })
            .elevation_3(cx)
            .track_focus(&self.focus_handle(cx))
            .on_action(cx.listener(Self::cancel))
            .on_action(cx.listener(|_, _: &menu::Cancel, _window, cx| {
                onboarding_event!("Cancelled", trigger = "Action");
                cx.emit(DismissEvent);
            }))
            .on_any_mouse_down(cx.listener(|this, _: &MouseDownEvent, window, _cx| {
                this.focus_handle.focus(window);
            }))
            .child(
                div()
                    .opacity(0.5)
                    .absolute()
                    .top(px(-8.0))
                    .right_0()
                    .w(px(400.))
                    .h(px(92.))
                    .child(
                        Vector::new(VectorName::AiGrid, rems_from_px(400.), rems_from_px(92.))
                            .color(Color::Custom(cx.theme().colors().text.alpha(0.32))),
                    ),
            )
            .child(
                div()
                    .absolute()
                    .top_0()
                    .right_0()
                    .w(px(660.))
                    .h(px(401.))
                    .overflow_hidden()
                    .bg(linear_gradient(
                        75.,
                        linear_color_stop(cx.theme().colors().panel_background.alpha(0.01), 1.0),
                        linear_color_stop(cx.theme().colors().panel_background, 0.45),
                    )),
            )
            .child(h_flex().absolute().top_2().right_2().child(
                IconButton::new("cancel", IconName::X).on_click(cx.listener(
                    |_, _: &ClickEvent, _window, cx| {
                        onboarding_event!("Cancelled", trigger = "X click");
                        cx.emit(DismissEvent);
                    },
                )),
            ))
            .child(self.onboarding.clone())
    }
}
