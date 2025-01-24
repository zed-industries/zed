use std::sync::Arc;

use client::{Client, UserStore};
use feature_flags::{FeatureFlagAppExt as _, PredictEditsFeatureFlag};
use fs::Fs;
use gpui::{ClickEvent, Model, Subscription, WeakView};
use language::language_settings::{all_language_settings, InlineCompletionProvider};
use ui::{prelude::*, ButtonLike};
use workspace::Workspace;

use crate::ZedPredictModal;

/// Prompts user to try AI inline prediction feature
pub struct ZedPredictBanner {
    workspace: WeakView<Workspace>,
    user_store: Model<UserStore>,
    client: Arc<Client>,
    fs: Arc<dyn Fs>,
    _subscription: Subscription,
}

impl ZedPredictBanner {
    pub fn new(
        workspace: WeakView<Workspace>,
        user_store: Model<UserStore>,
        client: Arc<Client>,
        fs: Arc<dyn Fs>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let subscription = cx.subscribe(&user_store, Self::handle_user_store_event);

        Self {
            workspace,
            user_store,
            client,
            fs,
            _subscription: subscription,
        }
    }

    pub fn should_show(&self, cx: &mut WindowContext) -> bool {
        if !cx.has_flag::<PredictEditsFeatureFlag>() {
            return false;
        }

        if self
            .user_store
            .read(cx)
            .current_user_has_dismissed_zed_predict_banner()
            .unwrap_or(true)
        {
            return false;
        }

        let provider = all_language_settings(None, cx).inline_completions.provider;

        match provider {
            InlineCompletionProvider::None
            | InlineCompletionProvider::Copilot
            | InlineCompletionProvider::Supermaven => true,
            InlineCompletionProvider::Zed => false,
        }
    }

    fn dismiss(&mut self, _: &ClickEvent, cx: &mut ViewContext<Self>) {
        self.user_store.update(cx, |user_store, cx| {
            user_store.dismiss_zed_predict_banner(cx);
        });
    }

    fn handle_user_store_event(
        &mut self,
        _: Model<UserStore>,
        event: &client::user::Event,
        cx: &mut ViewContext<Self>,
    ) {
        if let client::user::Event::PrivateUserInfoUpdated = event {
            cx.notify();
        }
    }
}

impl Render for ZedPredictBanner {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        if !self.should_show(cx) {
            return div();
        }

        let banner = h_flex()
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
                        let client = self.client.clone();
                        let fs = self.fs.clone();
                        move |_, cx| {
                            let Some(workspace) = workspace.upgrade() else {
                                return;
                            };
                            ZedPredictModal::toggle(
                                workspace,
                                user_store.clone(),
                                client.clone(),
                                fs.clone(),
                                cx,
                            );
                        }
                    }),
            )
            .child(
                div()
                    .border_l_1()
                    .border_color(cx.theme().colors().editor_foreground.opacity(0.1))
                    .child(
                        IconButton::new("close", IconName::Close)
                            .icon_size(IconSize::Indicator)
                            .on_click(cx.listener(Self::dismiss)),
                    ),
            );

        div().pr_1().child(banner)
    }
}
