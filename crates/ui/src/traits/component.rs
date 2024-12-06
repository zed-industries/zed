use crate::prelude::*;
use gpui::{AnyElement, IntoElement, WindowContext};

/// A trait that all components must implement
pub trait ComponentElement {
    /// The name of the component, derived from it's type
    fn title() -> &'static str {
        std::any::type_name::<Self>()
    }

    /// The scope/category this component belongs to
    fn scope() -> &'static str;

    /// An optional description of the component
    ///
    /// Use to provide additional context or indicate usage
    fn description() -> impl Into<Option<&'static str>> {
        None
    }

    /// An optional preview of the component or it's states
    fn preview(_cx: &WindowContext) -> Option<AnyElement> {
        None
    }

    /// Render the component's preview if it has one
    fn render_preview(cx: &WindowContext) -> AnyElement {
        let title = Self::title();
        let (source, title) = title
            .rsplit_once("::")
            .map_or((None, title), |(s, t)| (Some(s), t));
        let description = Self::description().into();

        v_flex()
            .w_full()
            .gap_6()
            .p_4()
            .border_1()
            .border_color(cx.theme().colors().border)
            .rounded_md()
            .child(
                v_flex()
                    .gap_1()
                    .child(
                        h_flex()
                            .gap_1()
                            .child(Headline::new(title).size(HeadlineSize::Small))
                            .when_some(source, |this, source| {
                                this.child(Label::new(format!("({})", source)).color(Color::Muted))
                            }),
                    )
                    .when_some(description, |this, description| {
                        this.child(
                            div()
                                .text_ui_sm(cx)
                                .text_color(cx.theme().colors().text_muted)
                                .max_w(px(600.0))
                                .child(description),
                        )
                    }),
            )
            .when_some(Self::preview(cx), |this, preview| this.child(preview))
            .when(Self::preview(cx).is_none(), |this| {
                this.child(
                    div()
                        .text_ui_sm(cx)
                        .text_color(cx.theme().colors().text_muted)
                        .child("No preview available"),
                )
            })
            .into_any_element()
    }
}
