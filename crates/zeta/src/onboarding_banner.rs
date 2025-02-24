use chrono::Utc;
use feature_flags::{FeatureFlagAppExt as _, PredictEditsFeatureFlag};
use gpui::Subscription;
use language::language_settings::{all_language_settings, EditPredictionProvider};
use settings::SettingsStore;
use ui::{prelude::*, ButtonLike, Tooltip};
use util::ResultExt;

use crate::onboarding_event;

/// Prompts the user to try Zed's Edit Prediction feature
pub struct ZedPredictBanner {
    dismissed: bool,
    provider: EditPredictionProvider,
    _subscription: Subscription,
}

impl ZedPredictBanner {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            dismissed: get_dismissed(),
            provider: all_language_settings(None, cx).edit_predictions.provider,
            _subscription: cx.observe_global::<SettingsStore>(Self::handle_settings_changed),
        }
    }

    fn should_show(&self, cx: &mut App) -> bool {
        cx.has_flag::<PredictEditsFeatureFlag>() && !self.dismissed && !self.provider.is_zed()
    }

    fn handle_settings_changed(&mut self, cx: &mut Context<Self>) {
        let new_provider = all_language_settings(None, cx).edit_predictions.provider;

        if new_provider == self.provider {
            return;
        }

        if new_provider.is_zed() {
            self.dismiss(cx);
        } else {
            self.dismissed = get_dismissed();
        }

        self.provider = new_provider;
        cx.notify();
    }

    fn dismiss(&mut self, cx: &mut Context<Self>) {
        onboarding_event!("Banner Dismissed");
        persist_dismissed(cx);
        self.dismissed = true;
        cx.notify();
    }
}

const DISMISSED_AT_KEY: &str = "zed_predict_banner_dismissed_at";

fn get_dismissed() -> bool {
    db::kvp::KEY_VALUE_STORE
        .read_kvp(DISMISSED_AT_KEY)
        .log_err()
        .map_or(false, |dismissed| dismissed.is_some())
}

fn persist_dismissed(cx: &mut App) {
    cx.spawn(|_| {
        let time = Utc::now().to_rfc3339();
        db::kvp::KEY_VALUE_STORE.write_kvp(DISMISSED_AT_KEY.into(), time)
    })
    .detach_and_log_err(cx);
}

pub(crate) fn clear_dismissed(cx: &mut App) {
    cx.spawn(|_| db::kvp::KEY_VALUE_STORE.delete_kvp(DISMISSED_AT_KEY.into()))
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
                    .on_click(|_, window, cx| {
                        onboarding_event!("Banner Clicked");
                        window.dispatch_action(Box::new(zed_actions::OpenZedPredictOnboarding), cx)
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

        div().pr_2().child(banner)
    }
}
