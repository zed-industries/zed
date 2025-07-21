use gpui::{IntoElement, ParentElement};
use ui::{Banner, prelude::*};

#[derive(IntoElement)]
pub struct YoungAccountBanner;

impl RenderOnce for YoungAccountBanner {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        const YOUNG_ACCOUNT_DISCLAIMER: &str = "Given your GitHub account was created less than 30 days ago, we cannot put you in the Free plan or offer you a free trial of the Pro plan. We hope you'll understand, as this is unfortunately required to prevent abuse of our service. To continue, upgrade to Pro or use your own API keys for other providers.";

        let label = div()
            .w_full()
            .text_sm()
            .text_color(cx.theme().colors().text_muted)
            .child(YOUNG_ACCOUNT_DISCLAIMER);

        div()
            .my_1()
            .child(Banner::new().severity(ui::Severity::Warning).child(label))
    }
}
