use std::any::{Any, TypeId};

use command_palette_hooks::CommandPaletteFilter;
use feature_flags::{
    FeatureFlagAppExt as _, PredictEditsFeatureFlag, PredictEditsRateCompletionsFeatureFlag,
};
use ui::App;
use workspace::Workspace;

use crate::{onboarding_modal::ZedPredictModal, RateCompletionModal, RateCompletions};

pub fn init(cx: &mut App) {
    cx.observe_new(move |workspace: &mut Workspace, _, _cx| {
        workspace.register_action(|workspace, _: &RateCompletions, window, cx| {
            if cx.has_flag::<PredictEditsRateCompletionsFeatureFlag>() {
                RateCompletionModal::toggle(workspace, window, cx);
            }
        });

        workspace.register_action(
            move |workspace, _: &zed_actions::OpenZedPredictOnboarding, window, cx| {
                if cx.has_flag::<PredictEditsFeatureFlag>() {
                    ZedPredictModal::toggle(
                        workspace,
                        workspace.user_store().clone(),
                        workspace.client().clone(),
                        workspace.app_state().fs.clone(),
                        window,
                        cx,
                    )
                }
            },
        );
    })
    .detach();

    feature_gate_predict_edits_rating_actions(cx);
}

fn feature_gate_predict_edits_rating_actions(cx: &mut App) {
    let rate_completion_action_types = [TypeId::of::<RateCompletions>()];

    CommandPaletteFilter::update_global(cx, |filter, _cx| {
        filter.hide_action_types(&rate_completion_action_types);
        filter.hide_action_types(&[zed_actions::OpenZedPredictOnboarding.type_id()]);
    });

    cx.observe_flag::<PredictEditsRateCompletionsFeatureFlag, _>(move |is_enabled, cx| {
        if is_enabled {
            CommandPaletteFilter::update_global(cx, |filter, _cx| {
                filter.show_action_types(rate_completion_action_types.iter());
            });
        } else {
            CommandPaletteFilter::update_global(cx, |filter, _cx| {
                filter.hide_action_types(&rate_completion_action_types);
            });
        }
    })
    .detach();
}
