use std::sync::Arc;

use crate::{EditPredictionStore, ZedPredictUpsell};
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
use ui::prelude::*;
use workspace::{ModalView, Workspace};

#[macro_export]
macro_rules! onboarding_event {
    ($name:expr) => {
        telemetry::event!($name, source = "Edit Prediction Onboarding");
    };
    ($name:expr, $($key:ident $(= $value:expr)?),+ $(,)?) => {
        telemetry::event!($name, source = "Edit Prediction Onboarding", $($key $(= $value)?),+);
    };
}

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
            .edit_predictions
            .get_or_insert(Default::default())
            .provider = Some(provider);
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
        let project = workspace.project().clone();
        workspace.toggle_modal(window, cx, |_window, cx| {
            let weak_entity = cx.weak_entity();
            let copilot = EditPredictionStore::try_global(cx)
                .and_then(|store| store.read(cx).copilot_for_project(&project));
            Self {
                onboarding: cx.new(|cx| {
                    EditPredictionOnboarding::new(
                        user_store.clone(),
                        client.clone(),
                        copilot
                            .as_ref()
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
                                if let Some(copilot) = copilot.clone() {
                                    copilot_ui::initiate_sign_in(copilot, window, cx);
                                }
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
        let color = cx.theme().colors();

        v_flex()
            .id("edit-prediction-onboarding")
            .key_context("ZedPredictModal")
            .relative()
            .w(px(550.))
            .h_full()
            .max_h(max_height)
            .p_1()
            .gap_2()
            .elevation_3(cx)
            .track_focus(&self.focus_handle(cx))
            .overflow_hidden()
            .on_action(cx.listener(Self::cancel))
            .on_action(cx.listener(|_, _: &menu::Cancel, _window, cx| {
                onboarding_event!("Cancelled", trigger = "Action");
                cx.emit(DismissEvent);
            }))
            .on_any_mouse_down(cx.listener(|this, _: &MouseDownEvent, window, cx| {
                this.focus_handle.focus(window, cx);
            }))
            .child(
                div()
                    .p_3()
                    .size_full()
                    .border_1()
                    .border_color(cx.theme().colors().border)
                    .rounded(px(5.))
                    .bg(linear_gradient(
                        360.,
                        linear_color_stop(color.panel_background, 1.0),
                        linear_color_stop(color.editor_background, 0.45),
                    ))
                    .child(self.onboarding.clone()),
            )
            .child(h_flex().absolute().top_3().right_3().child(
                IconButton::new("cancel", IconName::Close).on_click(cx.listener(
                    |_, _: &ClickEvent, _window, cx| {
                        onboarding_event!("Cancelled", trigger = "X click");
                        cx.emit(DismissEvent);
                    },
                )),
            ))
    }
}
