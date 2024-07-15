use ui::{prelude::*, ButtonLike};

#[derive(IntoElement, Clone)]
pub struct FeatureUpsell {
    text: SharedString,
    docs_url: Option<SharedString>,
    /// Whether this is the first upsell being displayed.
    is_first: bool,
}

impl FeatureUpsell {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self {
            text: text.into(),
            docs_url: None,
            is_first: false,
        }
    }

    pub fn docs_url(mut self, docs_url: impl Into<SharedString>) -> Self {
        self.docs_url = Some(docs_url.into());
        self
    }

    pub fn is_first(mut self, is_first: bool) -> Self {
        self.is_first = is_first;
        self
    }
}

impl RenderOnce for FeatureUpsell {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        h_flex()
            .p_4()
            .justify_between()
            .when(!self.is_first, |el| el.border_t_1())
            .border_b_1()
            .border_color(cx.theme().colors().border)
            .child(Label::new(self.text))
            .when_some(self.docs_url, |el, docs_url| {
                el.child(
                    ButtonLike::new("open_docs")
                        .child(
                            h_flex()
                                .gap_2()
                                .child(Label::new("View docs"))
                                .child(Icon::new(IconName::ArrowUpRight)),
                        )
                        .on_click({
                            let docs_url = docs_url.clone();
                            move |_event, cx| cx.open_url(&docs_url)
                        }),
                )
            })
    }
}
