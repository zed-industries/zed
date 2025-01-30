use std::sync::Arc;

use crate::ZedPredictModal;
use chrono::Utc;
use client::{Client, UserStore};
use feature_flags::{FeatureFlagAppExt as _, PredictEditsFeatureFlag};
use fs::Fs;
use gpui::{Entity, Subscription, WeakEntity};
use language::language_settings::{all_language_settings, InlineCompletionProvider};
use settings::SettingsStore;
use ui::{prelude::*, ButtonLike, Tooltip};
use util::ResultExt;
use workspace::Workspace;

/// Prompts user to try AI inline prediction feature
pub struct ZedPredictBanner {
    workspace: WeakEntity<Workspace>,
    user_store: Entity<UserStore>,
    client: Arc<Client>,
    fs: Arc<dyn Fs>,
    dismissed: bool,
    _subscription: Subscription,
}

impl ZedPredictBanner {
    pub fn new(
        workspace: WeakEntity<Workspace>,
        user_store: Entity<UserStore>,
        client: Arc<Client>,
        fs: Arc<dyn Fs>,
        cx: &mut Context<Self>,
    ) -> Self {
        Self {
            workspace,
            user_store,
            client,
            fs,
            dismissed: get_dismissed(),
            _subscription: cx.observe_global::<SettingsStore>(Self::handle_settings_changed),
        }
    }

    fn should_show(&self, cx: &mut App) -> bool {
        if !cx.has_flag::<PredictEditsFeatureFlag>() || self.dismissed {
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

    fn handle_settings_changed(&mut self, cx: &mut Context<Self>) {
        if self.dismissed {
            return;
        }

        let provider = all_language_settings(None, cx).inline_completions.provider;

        match provider {
            InlineCompletionProvider::None
            | InlineCompletionProvider::Copilot
            | InlineCompletionProvider::Supermaven => {}
            InlineCompletionProvider::Zed => {
                self.dismiss(cx);
            }
        }
    }

    fn dismiss(&mut self, cx: &mut Context<Self>) {
        persist_dismissed(cx);
        self.dismissed = true;
        cx.notify();
    }
}

const DISMISSED_AT_KEY: &str = "zed_predict_banner_dismissed_at";

pub(crate) fn get_dismissed() -> bool {
    db::kvp::KEY_VALUE_STORE
        .read_kvp(DISMISSED_AT_KEY)
        .log_err()
        .map_or(false, |dismissed| dismissed.is_some())
}

pub(crate) fn persist_dismissed(cx: &mut App) {
    cx.spawn(|_| {
        let time = Utc::now().to_rfc3339();
        db::kvp::KEY_VALUE_STORE.write_kvp(DISMISSED_AT_KEY.into(), time)
    })
    .detach_and_log_err(cx);
}

impl Render for ZedPredictBanner {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if !self.should_show(cx) {
            return div();
        }

        let border_color = cx.theme().colors().editor_foreground.opacity(0.3);
        let banner = h_flex()
            .rounded_md()
            .border_1()
            .border_color(border_color)
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
                        move |_, window, cx| {
                            let Some(workspace) = workspace.upgrade() else {
                                return;
                            };
                            ZedPredictModal::toggle(
                                workspace,
                                user_store.clone(),
                                client.clone(),
                                fs.clone(),
                                window,
                                cx,
                            );
                        }
                    }),
            )
            .child(
                div().border_l_1().border_color(border_color).child(
                    IconButton::new("close", IconName::Close)
                        .icon_size(IconSize::Indicator)
                        .on_click(cx.listener(|this, _, _window, cx| this.dismiss(cx)))
                        .tooltip(|window, cx| {
                            Tooltip::with_meta(
                                "Close Announcement Banner",
                                None,
                                "It won't show again for this feature",
                                window,
                                cx,
                            )
                        }),
                ),
            );

        div().pr_1().child(banner)
    }
}
