use crate::{prelude::*, traits::component::ComponentElement};
use gpui::{AnyElement, IntoElement, Length, ParentElement, StyleRefinement, Styled};
use smallvec::SmallVec;

/// Creates a new [ContentGroup].
pub fn content_group() -> ContentGroup {
    ContentGroup::new()
}

/// A [ContentGroup] that vertically stacks its children.
///
/// This is a convenience function that simply combines [`ContentGroup`] and [`v_flex`](crate::v_flex).
pub fn v_group() -> ContentGroup {
    content_group().v_flex()
}

/// Creates a new horizontal [ContentGroup].
///
/// This is a convenience function that simply combines [`ContentGroup`] and [`h_flex`](crate::h_flex).
pub fn h_group() -> ContentGroup {
    content_group().h_flex()
}

/// A flexible container component that can hold other elements.
#[derive(IntoElement)]
pub struct ContentGroup {
    base: Div,
    border: bool,
    padding: Pixels,
    fill: bool,
    outset_amount: Option<AbsoluteLength>,
    children: SmallVec<[AnyElement; 2]>,
}

impl ContentGroup {
    /// Creates a new [ContentBox].
    pub fn new() -> Self {
        Self {
            base: div(),
            border: true,
            padding: px(8.),
            fill: true,
            outset_amount: None,
            children: SmallVec::new(),
        }
    }

    /// Removes the border from the [ContentBox].
    pub fn borderless(mut self) -> Self {
        self.border = false;
        self
    }

    /// Removes the background fill from the [ContentBox].
    pub fn unfilled(mut self) -> Self {
        self.fill = false;
        self
    }

    /// Outset the [ContentBox] to create a visual "outset" effect.
    ///
    /// This sets a default outset amount which matches the width of the border and padding,
    /// useful for lining up content with other elements.
    pub fn outset(mut self) -> Self {
        let border_width = if self.border { px(1.) } else { px(0.) };
        let outset_amount = self.padding + border_width;
        self.outset_amount = Some(outset_amount.into());
        self
    }

    /// Sets the amount of outset to apply to the [ContentBox].
    ///
    /// This will add negative left and right margin to the [ContentBox] to create a visual "outset" effect.
    pub fn outset_amount(mut self, amount: impl Into<AbsoluteLength>) -> Self {
        self.outset_amount = Some(amount.into());
        self
    }
}

impl ParentElement for ContentGroup {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}

impl Styled for ContentGroup {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for ContentGroup {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        // TODO:
        // Baked in padding will make scrollable views inside of content boxes awkward.
        //
        // Do we make the padding optional, or do we push to use a different component?

        self.base
            .when(self.fill, |this| {
                this.bg(cx.theme().colors().text.opacity(0.05))
            })
            .when(self.border, |this| {
                this.border_1().border_color(cx.theme().colors().border)
            })
            .rounded_md()
            .p(self.padding)
            .when_some(self.outset_amount, |this, amount| {
                let outset_rems: Rems = match amount {
                    AbsoluteLength::Pixels(p) => rems(p / cx.rem_size()),
                    AbsoluteLength::Rems(r) => r,
                };
                this.mx(-outset_rems)
            })
            .children(self.children)
    }
}

// impl Component for ContentGroup {
//     fn name(&self) -> &'static str {
//         "Content Group"
//     }

//     fn scope(&self) -> &'static str {
//         "layout"
//     }
// }

// impl Component for ContentGroup {
//     fn description() -> impl Into<Option<&'static str>> {
//         "A flexible container component that can hold other elements. It can be customized with or without a border and background fill."
//     }

//     fn example_label_side() -> ExampleLabelSide {
//         ExampleLabelSide::Bottom
//     }

//     fn examples(_: &WindowContext) -> Vec<ComponentExampleGroup<Self>> {
//         vec![example_group(vec![
//             single_example(
//                 "Default",
//                 ContentGroup::new()
//                     .flex_1()
//                     .items_center()
//                     .justify_center()
//                     .h_48()
//                     .child(Label::new("Default ContentBox")),
//             )
//             .grow(),
//             single_example(
//                 "Without Border",
//                 ContentGroup::new()
//                     .flex_1()
//                     .items_center()
//                     .justify_center()
//                     .h_48()
//                     .borderless()
//                     .child(Label::new("Borderless ContentBox")),
//             )
//             .grow(),
//             single_example(
//                 "Without Fill",
//                 ContentGroup::new()
//                     .flex_1()
//                     .items_center()
//                     .justify_center()
//                     .h_48()
//                     .unfilled()
//                     .child(Label::new("Unfilled ContentBox")),
//             )
//             .grow(),
//         ])
//         .grow()]
//     }
// }
