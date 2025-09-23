use gpui::{IntoElement, ParentElement};
use ui::{Banner, prelude::*};

#[derive(IntoElement)]
pub struct YoungAccountBanner {
    is_v2: bool,
}

impl YoungAccountBanner {
    pub fn new(is_v2: bool) -> Self {
        Self { is_v2 }
    }
}

impl RenderOnce for YoungAccountBanner {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        const YOUNG_ACCOUNT_DISCLAIMER: &str = "To prevent abuse of our service, GitHub accounts created fewer than 30 days ago are not eligible for free plan usage or Pro plan free trial. You can request an exception by reaching out to billing-support@zed.dev";
        const YOUNG_ACCOUNT_DISCLAIMER_V2: &str = "To prevent abuse of our service, GitHub accounts created fewer than 30 days ago are not eligible for the Pro trial. You can request an exception by reaching out to billing-support@zed.dev";

        let label = div()
            .w_full()
            .text_sm()
            .text_color(cx.theme().colors().text_muted)
            .child(if self.is_v2 {
                YOUNG_ACCOUNT_DISCLAIMER_V2
            } else {
                YOUNG_ACCOUNT_DISCLAIMER
            });

        div()
            .max_w_full()
            .my_1()
            .child(Banner::new().severity(Severity::Warning).child(label))
    }
}
