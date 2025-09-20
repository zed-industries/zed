use std::sync::Arc;

use crate::{ZedPredictUpsell, onboarding_event};
use ai_onboarding::EditPredictionOnboarding;
use client::{Client, UserStore};
use db::kvp::Dismissable;
use fs::Fs;
use gpui::{
    ClickEvent, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, MouseDownEvent, Render,
    linear_color_stop, linear_gradient,
};
use language::language_settings::EditPredictionProvider;
use settings::update_settings_file;
use ui::{Vector, VectorName, prelude::*};
use workspace::{ModalView, Workspace};

/// Introduces user to Zed's Edit Prediction feature
pub struct ZedPredictModal {
    onboarding: Entity<EditPredictionOnboarding>,
    focus_handle: FocusHandle,
}

pub(crate) fn set_edit_prediction_provider(provider: EditPredictionProvider, cx: &mut App) {
    let fs = <dyn Fs>::global(cx);
    update_settings_file(fs, cx, move |settings, _| {
        settings
            .project
            .all_languages
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
                            .is_some_and(|copilot| copilot.read(cx).status().is_configured()),
                        Arc::new({
                            let this = weak_entity.clone();
                            move |_window, cx| {
                                ZedPredictUpsell::set_dismissed(true, cx);
                                set_edit_prediction_provider(EditPredictionProvider::Zed, cx);
                                this.update(cx, |_, cx| cx.emit(DismissEvent)).ok();
                            }
                        }),
                        Arc::new({
                            let this = weak_entity.clone();
                            move |window, cx| {
                                ZedPredictUpsell::set_dismissed(true, cx);
                                set_edit_prediction_provider(EditPredictionProvider::Copilot, cx);
                                this.update(cx, |_, cx| cx.emit(DismissEvent)).ok();
                                copilot::initiate_sign_in(window, cx);
                            }
                        }),
                        cx,
                    )
                }),
                focus_handle: cx.focus_handle(),
            }
        });
    }

    fn cancel(&mut self, _: &menu::Cancel, _: &mut Window, cx: &mut Context<Self>) {
        ZedPredictUpsell::set_dismissed(true, cx);
        cx.emit(DismissEvent);
    }
}

impl EventEmitter<DismissEvent> for ZedPredictModal {}

impl Focusable for ZedPredictModal {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl ModalView for ZedPredictModal {
    fn on_before_dismiss(
        &mut self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> workspace::DismissDecision {
        ZedPredictUpsell::set_dismissed(true, cx);
        workspace::DismissDecision::Dismiss(true)
    }
}

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
            .elevation_3(cx)
            .track_focus(&self.focus_handle(cx))
            .overflow_hidden()
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
                IconButton::new("cancel", IconName::Close).on_click(cx.listener(
                    |_, _: &ClickEvent, _window, cx| {
                        onboarding_event!("Cancelled", trigger = "X click");
                        cx.emit(DismissEvent);
                    },
                )),
            ))
            .child(self.onboarding.clone())
    }
}
