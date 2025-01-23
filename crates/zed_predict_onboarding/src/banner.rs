use std::sync::Arc;

use client::UserStore;
use feature_flags::{FeatureFlagAppExt as _, PredictEditsFeatureFlag};
use fs::Fs;
use gpui::{Model, WeakView};
use language::language_settings::{all_language_settings, InlineCompletionProvider};
use ui::{prelude::*, ButtonLike};
use workspace::Workspace;

use crate::ZedPredictModal;

/// Prompts user to try AI inline prediction feature
#[derive(IntoElement)]
pub struct ZedPredictBanner {
    workspace: WeakView<Workspace>,
    user_store: Model<UserStore>,
    fs: Arc<dyn Fs>,
}

impl ZedPredictBanner {
    pub fn new(
        workspace: WeakView<Workspace>,
        user_store: Model<UserStore>,
        fs: Arc<dyn Fs>,
        cx: &mut WindowContext,
    ) -> Option<Self> {
        if !cx.has_flag::<PredictEditsFeatureFlag>() {
            return None;
        }

        let provider = all_language_settings(None, cx).inline_completions.provider;

        match provider {
            InlineCompletionProvider::None
            | InlineCompletionProvider::Copilot
            | InlineCompletionProvider::Supermaven => {},
            InlineCompletionProvider::Zed => return None,
        }

        Some(Self {
            workspace,
            user_store,
            fs,
        })
    }
}

impl RenderOnce for ZedPredictBanner {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        h_flex()
            .h_5()
            .rounded_md()
            .border_1()
            .border_color(cx.theme().colors().editor_foreground.opacity(0.3))
            .child(
                ButtonLike::new("try-zed-predict")
                    .child(
                        h_flex()
                            .h_full()
                            .items_center()
                            .gap_1p5()
                            .child(Icon::new(IconName::ZedPredict).size(IconSize::Small))
                            .child(
                                h_flex()
                                    .gap_0p5()
                                    .child(
                                        Label::new("Introducing:")
                                            .size(LabelSize::Small)
                                            .color(Color::Muted),
                                    )
                                    .child(Label::new("Edit Prediction").size(LabelSize::Small)),
                            ),
                    )
                    .on_click({
                        let workspace = self.workspace.clone();
                        let user_store = self.user_store.clone();
                        let fs = self.fs.clone();
                        move |_, cx| {
                            let Some(workspace) = workspace.upgrade() else {
                                return;
                            };
                            ZedPredictModal::toggle(workspace, user_store.clone(), fs.clone(), cx);
                        }
                    }),
            )
            .child(
                div()
                    .border_l_1()
                    .border_color(cx.theme().colors().editor_foreground.opacity(0.1))
                    .child(
                        IconButton::new("close", IconName::Close).icon_size(IconSize::Indicator),
                    ),
            )
    }
}
