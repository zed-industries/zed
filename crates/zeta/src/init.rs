use std::any::{Any, TypeId};

use command_palette_hooks::CommandPaletteFilter;
use feature_flags::{FeatureFlagAppExt as _, PredictEditsRateCompletionsFeatureFlag};
use gpui::actions;
use language::language_settings::{AllLanguageSettings, EditPredictionProvider};
use project::DisableAiSettings;
use settings::{Settings, SettingsStore, update_settings_file};
use ui::App;
use workspace::Workspace;

use crate::{RateCompletionModal, onboarding_modal::ZedPredictModal};

actions!(
    edit_prediction,
    [
        /// Resets the edit prediction onboarding state.
        ResetOnboarding,
        /// Opens the rate completions modal.
        RateCompletions
    ]
);

pub fn init(cx: &mut App) {
    feature_gate_predict_edits_actions(cx);

    cx.observe_new(move |workspace: &mut Workspace, _, _cx| {
        workspace.register_action(|workspace, _: &RateCompletions, window, cx| {
            if cx.has_flag::<PredictEditsRateCompletionsFeatureFlag>() {
                RateCompletionModal::toggle(workspace, window, cx);
            }
        });

        workspace.register_action(
            move |workspace, _: &zed_actions::OpenZedPredictOnboarding, window, cx| {
                ZedPredictModal::toggle(
                    workspace,
                    workspace.user_store().clone(),
                    workspace.client().clone(),
                    window,
                    cx,
                )
            },
        );

        workspace.register_action(|workspace, _: &ResetOnboarding, _window, cx| {
            update_settings_file::<AllLanguageSettings>(
                workspace.app_state().fs.clone(),
                cx,
                move |file, _| {
                    file.features
                        .get_or_insert(Default::default())
                        .edit_prediction_provider = Some(EditPredictionProvider::None)
                },
            );
        });
    })
    .detach();
}

fn feature_gate_predict_edits_actions(cx: &mut App) {
    let rate_completion_action_types = [TypeId::of::<RateCompletions>()];
    let reset_onboarding_action_types = [TypeId::of::<ResetOnboarding>()];
    let zeta_all_action_types = [
        TypeId::of::<RateCompletions>(),
        TypeId::of::<ResetOnboarding>(),
        zed_actions::OpenZedPredictOnboarding.type_id(),
        TypeId::of::<crate::ClearHistory>(),
        TypeId::of::<crate::ThumbsUpActiveCompletion>(),
        TypeId::of::<crate::ThumbsDownActiveCompletion>(),
        TypeId::of::<crate::NextEdit>(),
        TypeId::of::<crate::PreviousEdit>(),
    ];

    CommandPaletteFilter::update_global(cx, |filter, _cx| {
        filter.hide_action_types(&rate_completion_action_types);
        filter.hide_action_types(&reset_onboarding_action_types);
        filter.hide_action_types(&[zed_actions::OpenZedPredictOnboarding.type_id()]);
    });

    cx.observe_global::<SettingsStore>(move |cx| {
        let is_ai_disabled = DisableAiSettings::get_global(cx).disable_ai;
        let has_feature_flag = cx.has_flag::<PredictEditsRateCompletionsFeatureFlag>();

        CommandPaletteFilter::update_global(cx, |filter, _cx| {
            if is_ai_disabled {
                filter.hide_action_types(&zeta_all_action_types);
            } else if has_feature_flag {
                filter.show_action_types(&rate_completion_action_types);
            } else {
                filter.hide_action_types(&rate_completion_action_types);
            }
        });
    })
    .detach();

    cx.observe_flag::<PredictEditsRateCompletionsFeatureFlag, _>(move |is_enabled, cx| {
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
