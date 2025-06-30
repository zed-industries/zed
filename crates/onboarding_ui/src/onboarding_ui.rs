use command_palette_hooks::CommandPaletteFilter;
use feature_flags::FeatureFlagAppExt as _;
use gpui::App;
use settings_ui::SettingsUiFeatureFlag;
use workspace::Workspace;

use gpui::actions;

actions!(onboarding, [ShowOnboarding]);

pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, _, _cx| {
        workspace.register_action(|_workspace, _: &ShowOnboarding, _window, _cx| {
            // Onboarding implementation will go here
        });
    })
    .detach();

    feature_gate_onboarding_ui_actions(cx);
}

fn feature_gate_onboarding_ui_actions(cx: &mut App) {
    const ONBOARDING_ACTION_NAMESPACE: &str = "onboarding";

    CommandPaletteFilter::update_global(cx, |filter, _cx| {
        filter.hide_namespace(ONBOARDING_ACTION_NAMESPACE);
    });

    cx.observe_flag::<SettingsUiFeatureFlag, _>({
        move |is_enabled, cx| {
            CommandPaletteFilter::update_global(cx, |filter, _cx| {
                if is_enabled {
                    filter.show_namespace(ONBOARDING_ACTION_NAMESPACE);
                } else {
                    filter.hide_namespace(ONBOARDING_ACTION_NAMESPACE);
                }
            });
        }
    })
    .detach();
}
