mod zeta2_context_view;

use edit_prediction::Zeta2FeatureFlag;
use feature_flags::FeatureFlagAppExt as _;
use gpui::{actions, prelude::*};
use ui::prelude::*;
use workspace::{SplitDirection, Workspace};
use zeta2_context_view::Zeta2ContextView;

actions!(
    dev,
    [
        /// Opens the edit prediction context view.
        OpenZeta2ContextView,
    ]
);

pub fn init(cx: &mut App) {
    cx.observe_new(move |workspace: &mut Workspace, _, _cx| {
        workspace.register_action_renderer(|div, _, _, cx| {
            let has_flag = cx.has_flag::<Zeta2FeatureFlag>();
            div.when(has_flag, |div| {
                div.on_action(cx.listener(
                    move |workspace, _: &OpenZeta2ContextView, window, cx| {
                        let project = workspace.project();
                        workspace.split_item(
                            SplitDirection::Right,
                            Box::new(cx.new(|cx| {
                                Zeta2ContextView::new(
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
