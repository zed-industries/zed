mod edit_prediction_button;
mod edit_prediction_context_view;
mod rate_prediction_modal;
mod sweep_api_token_modal;

use std::any::{Any as _, TypeId};

use command_palette_hooks::CommandPaletteFilter;
use edit_prediction::{ResetOnboarding, Zeta2FeatureFlag};
use edit_prediction_context_view::EditPredictionContextView;
use feature_flags::FeatureFlagAppExt as _;
use gpui::actions;
use project::DisableAiSettings;
use rate_prediction_modal::RatePredictionsModal;
use settings::{Settings as _, SettingsStore};
use ui::{App, prelude::*};
use workspace::{SplitDirection, Workspace};

pub use edit_prediction_button::{EditPredictionButton, ToggleMenu};
pub use sweep_api_token_modal::SweepApiKeyModal;

use crate::rate_prediction_modal::PredictEditsRatePredictionsFeatureFlag;

actions!(
    dev,
    [
        /// Opens the edit prediction context view.
        OpenEditPredictionContextView,
    ]
);

actions!(
    edit_prediction,
    [
        /// Opens the rate completions modal.
        RatePredictions,
    ]
);

pub fn init(cx: &mut App) {
    feature_gate_predict_edits_actions(cx);

    cx.observe_new(move |workspace: &mut Workspace, _, _cx| {
        workspace.register_action(|workspace, _: &RatePredictions, window, cx| {
            if cx.has_flag::<PredictEditsRatePredictionsFeatureFlag>() {
                RatePredictionsModal::toggle(workspace, window, cx);
            }
        });

        workspace.register_action_renderer(|div, _, _, cx| {
            let has_flag = cx.has_flag::<Zeta2FeatureFlag>();
            div.when(has_flag, |div| {
                div.on_action(cx.listener(
                    move |workspace, _: &OpenEditPredictionContextView, window, cx| {
                        let project = workspace.project();
                        workspace.split_item(
                            SplitDirection::Right,
                            Box::new(cx.new(|cx| {
                                EditPredictionContextView::new(
                                    project.clone(),
                                    workspace.client(),
                                    workspace.user_store(),
                                    window,
                                    cx,
                                )
                            })),
                            window,
                            cx,
                        );
                    },
                ))
            })
        });
    })
    .detach();
}

fn feature_gate_predict_edits_actions(cx: &mut App) {
    let rate_completion_action_types = [TypeId::of::<RatePredictions>()];
    let reset_onboarding_action_types = [TypeId::of::<ResetOnboarding>()];
    let all_action_types = [
        TypeId::of::<RatePredictions>(),
        TypeId::of::<edit_prediction::ResetOnboarding>(),
        zed_actions::OpenZedPredictOnboarding.type_id(),
        TypeId::of::<edit_prediction::ClearHistory>(),
        TypeId::of::<rate_prediction_modal::ThumbsUpActivePrediction>(),
        TypeId::of::<rate_prediction_modal::ThumbsDownActivePrediction>(),
        TypeId::of::<rate_prediction_modal::NextEdit>(),
        TypeId::of::<rate_prediction_modal::PreviousEdit>(),
    ];

    CommandPaletteFilter::update_global(cx, |filter, _cx| {
        filter.hide_action_types(&rate_completion_action_types);
        filter.hide_action_types(&reset_onboarding_action_types);
        filter.hide_action_types(&[zed_actions::OpenZedPredictOnboarding.type_id()]);
    });

    cx.observe_global::<SettingsStore>(move |cx| {
        let is_ai_disabled = DisableAiSettings::get_global(cx).disable_ai;
        let has_feature_flag = cx.has_flag::<PredictEditsRatePredictionsFeatureFlag>();

        CommandPaletteFilter::update_global(cx, |filter, _cx| {
            if is_ai_disabled {
                filter.hide_action_types(&all_action_types);
            } else if has_feature_flag {
                filter.show_action_types(&rate_completion_action_types);
            } else {
                filter.hide_action_types(&rate_completion_action_types);
            }
        });
    })
    .detach();

    cx.observe_flag::<PredictEditsRatePredictionsFeatureFlag, _>(move |is_enabled, cx| {
        if !DisableAiSettings::get_global(cx).disable_ai {
            if is_enabled {
                CommandPaletteFilter::update_global(cx, |filter, _cx| {
                    filter.show_action_types(&rate_completion_action_types);
                });
            } else {
                CommandPaletteFilter::update_global(cx, |filter, _cx| {
                    filter.hide_action_types(&rate_completion_action_types);
                });
            }
        }
    })
    .detach();
}
