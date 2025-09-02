use gpui::{AnyElement, Div, StyleRefinement};
use smallvec::SmallVec;
use ui::prelude::*;

#[derive(IntoElement)]
pub struct FeatureUpsell {
    base: Div,
    text: SharedString,
    docs_url: Option<SharedString>,
    children: SmallVec<[AnyElement; 2]>,
}

impl FeatureUpsell {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self {
            base: h_flex(),
            text: text.into(),
            docs_url: None,
            children: SmallVec::new(),
        }
    }

    pub fn docs_url(mut self, docs_url: impl Into<SharedString>) -> Self {
        self.docs_url = Some(docs_url.into());
        self
    }
}

impl ParentElement for FeatureUpsell {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}

// Style methods.
impl FeatureUpsell {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }

    gpui::border_style_methods!({
        visibility: pub
    });
}

impl RenderOnce for FeatureUpsell {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        self.base
            .py_2()
            .px_4()
            .justify_between()
            .flex_wrap()
            .border_color(cx.theme().colors().border_variant)
            .child(Label::new(self.text))
            .child(h_flex().gap_2().children(self.children).when_some(
                self.docs_url,
                |el, docs_url| {
                    el.child(
                        Button::new("open_docs", "View Documentation")
                            .icon(IconName::ArrowUpRight)
                            .icon_size(IconSize::Small)
                            .icon_position(IconPosition::End)
                            .on_click({
                                move |_event, _window, cx| {
                                    telemetry::event!(
                                        "Documentation Viewed",
                                        source = "Feature Upsell",
                                        url = docs_url,
                                    );
                                    cx.open_url(&docs_url)
                                }
                            }),
                    )
                },
            ))
    }
}
