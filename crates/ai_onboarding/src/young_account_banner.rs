use gpui::{IntoElement, ParentElement};
use ui::{Banner, List, prelude::*};

use crate::BulletItem;

#[derive(IntoElement)]
pub struct YoungAccountBanner;

impl RenderOnce for YoungAccountBanner {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        const YOUNG_ACCOUNT_DISCLAIMER: &str = "Given your GitHub account was created less than 30 days ago, we cannot put you in the Free plan or offer you a free trial of the Pro plan. We hope you'll understand, as this is unfortunately required to prevent abuse of our service.";
        const MOVE_FORWARD_PATHS: &str = "To continue, chose one of these options:";

        let label = v_flex()
            .w_full()
            .text_sm()
            .text_color(cx.theme().colors().text_muted)
            .child(YOUNG_ACCOUNT_DISCLAIMER)
            .child(MOVE_FORWARD_PATHS)
            .child(
                List::new()
                    .child(BulletItem::new("Upgrade to Pro"))
                    .child(BulletItem::new("Use your own API keys for other providers"))
                    .child(BulletItem::new("Send an email to billing-support@zed.dev")),
            );

        div()
            .my_1()
            .child(Banner::new().severity(ui::Severity::Warning).child(label))
    }
}
