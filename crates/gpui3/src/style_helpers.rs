use crate::{
    self as gpui3, relative, rems, AlignItems, Display, Fill, FlexDirection, Hsla, JustifyContent,
    Length, Position, SharedString, Style, StyleRefinement, Styled, TextStyleRefinement,
};

pub trait StyleHelpers: Sized + Styled<Style = Style> {
    gpui3_macros::style_helpers!();

    fn h(mut self, height: Length) -> Self {
        self.declared_style().size.height = Some(height);
        self
    }

    /// size_{n}: Sets width & height to {n}
    ///
    /// Example:
    /// size_1: Sets width & height to 1
    fn size(mut self, size: Length) -> Self {
        self.declared_style().size.height = Some(size);
        self.declared_style().size.width = Some(size);
        self
    }

    fn full(mut self) -> Self {
        self.declared_style().size.width = Some(relative(1.).into());
        self.declared_style().size.height = Some(relative(1.).into());
        self
    }

    fn relative(mut self) -> Self {
        self.declared_style().position = Some(Position::Relative);
        self
    }

    fn absolute(mut self) -> Self {
        self.declared_style().position = Some(Position::Absolute);
        self
    }

    fn block(mut self) -> Self {
        self.declared_style().display = Some(Display::Block);
        self
    }

    fn flex(mut self) -> Self {
        self.declared_style().display = Some(Display::Flex);
        self
    }

    fn flex_col(mut self) -> Self {
        self.declared_style().flex_direction = Some(FlexDirection::Column);
        self
    }

    fn flex_row(mut self) -> Self {
        self.declared_style().flex_direction = Some(FlexDirection::Row);
        self
    }

    fn flex_1(mut self) -> Self {
        self.declared_style().flex_grow = Some(1.);
        self.declared_style().flex_shrink = Some(1.);
        self.declared_style().flex_basis = Some(relative(0.).into());
        self
    }

    fn flex_auto(mut self) -> Self {
        self.declared_style().flex_grow = Some(1.);
        self.declared_style().flex_shrink = Some(1.);
        self.declared_style().flex_basis = Some(Length::Auto);
        self
    }

    fn flex_initial(mut self) -> Self {
        self.declared_style().flex_grow = Some(0.);
        self.declared_style().flex_shrink = Some(1.);
        self.declared_style().flex_basis = Some(Length::Auto);
        self
    }

    fn flex_none(mut self) -> Self {
        self.declared_style().flex_grow = Some(0.);
        self.declared_style().flex_shrink = Some(0.);
        self
    }

    fn grow(mut self) -> Self {
        self.declared_style().flex_grow = Some(1.);
        self
    }

    fn items_start(mut self) -> Self {
        self.declared_style().align_items = Some(AlignItems::FlexStart);
        self
    }

    fn items_end(mut self) -> Self {
        self.declared_style().align_items = Some(AlignItems::FlexEnd);
        self
    }

    fn items_center(mut self) -> Self {
        self.declared_style().align_items = Some(AlignItems::Center);
        self
    }

    fn justify_between(mut self) -> Self {
        self.declared_style().justify_content = Some(JustifyContent::SpaceBetween);
        self
    }

    fn justify_center(mut self) -> Self {
        self.declared_style().justify_content = Some(JustifyContent::Center);
        self
    }

    fn justify_start(mut self) -> Self {
        self.declared_style().justify_content = Some(JustifyContent::Start);
        self
    }

    fn justify_end(mut self) -> Self {
        self.declared_style().justify_content = Some(JustifyContent::End);
        self
    }

    fn justify_around(mut self) -> Self {
        self.declared_style().justify_content = Some(JustifyContent::SpaceAround);
        self
    }

    fn fill<F>(mut self, fill: F) -> Self
    where
        F: Into<Fill>,
        Self: Sized,
    {
        self.declared_style().fill = Some(fill.into());
        self
    }

    fn border_color<C>(mut self, border_color: C) -> Self
    where
        C: Into<Hsla>,
        Self: Sized,
    {
        self.declared_style().border_color = Some(border_color.into());
        self
    }

    fn text_style(&mut self) -> &mut Option<TextStyleRefinement> {
        let style: &mut StyleRefinement = self.declared_style();
        &mut style.text
    }

    fn text_color(mut self, color: impl Into<Hsla>) -> Self {
        self.text_style().get_or_insert_with(Default::default).color = Some(color.into());
        self
    }

    fn text_xs(mut self) -> Self {
        self.text_style()
            .get_or_insert_with(Default::default)
            .font_size = Some(rems(0.75));
        self
    }

    fn text_sm(mut self) -> Self {
        self.text_style()
            .get_or_insert_with(Default::default)
            .font_size = Some(rems(0.875));
        self
    }

    fn text_base(mut self) -> Self {
        self.text_style()
            .get_or_insert_with(Default::default)
            .font_size = Some(rems(1.0));
        self
    }

    fn text_lg(mut self) -> Self {
        self.text_style()
            .get_or_insert_with(Default::default)
            .font_size = Some(rems(1.125));
        self
    }

    fn text_xl(mut self) -> Self {
        self.text_style()
            .get_or_insert_with(Default::default)
            .font_size = Some(rems(1.25));
        self
    }

    fn text_2xl(mut self) -> Self {
        self.text_style()
            .get_or_insert_with(Default::default)
            .font_size = Some(rems(1.5));
        self
    }

    fn text_3xl(mut self) -> Self {
        self.text_style()
            .get_or_insert_with(Default::default)
            .font_size = Some(rems(1.875));
        self
    }

    fn font(mut self, family_name: impl Into<SharedString>) -> Self {
        self.text_style()
            .get_or_insert_with(Default::default)
            .font_family = Some(family_name.into());
        self
    }
}
