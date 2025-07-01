mod bookmark_picker;

use command_palette_hooks::CommandPaletteFilter;
use feature_flags::FeatureFlagAppExt as _;
use gpui::App;
use jj::JujutsuStore;
use workspace::Workspace;

pub fn init(cx: &mut App) {
    JujutsuStore::init_global(cx);

    cx.observe_new(|workspace: &mut Workspace, _window, _cx| {
        bookmark_picker::register(workspace);
    })
    .detach();

    feature_gate_jj_ui_actions(cx);
}

fn feature_gate_jj_ui_actions(cx: &mut App) {
    const JJ_ACTION_NAMESPACE: &str = "jj";

    CommandPaletteFilter::update_global(cx, |filter, _cx| {
        filter.hide_namespace(JJ_ACTION_NAMESPACE);
    });

    cx.observe_flag::<feature_flags::JjUiFeatureFlag, _>({
        move |is_enabled, cx| {
            CommandPaletteFilter::update_global(cx, |filter, _cx| {
                if is_enabled {
                    filter.show_namespace(JJ_ACTION_NAMESPACE);
                } else {
                    filter.hide_namespace(JJ_ACTION_NAMESPACE);
                }
            });
        }
    })
    .detach();
}
