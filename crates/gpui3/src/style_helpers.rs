use crate::{
    self as gpui3, hsla, point, px, relative, rems, AlignItems, BoxShadow, Display, Fill,
    FlexDirection, Hsla, JustifyContent, Length, Position, SharedString, Style, StyleRefinement,
    Styled, TextStyleRefinement,
};
use smallvec::smallvec;

pub trait StyleHelpers: Sized + Styled<Style = Style> {
    gpui3_macros::style_helpers!();

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

    fn shadow(mut self) -> Self {
        self.declared_style().box_shadow = Some(smallvec![
            BoxShadow {
                color: hsla(0., 0., 0., 0.1),
                offset: point(px(0.), px(1.)),
                blur_radius: px(3.),
                spread_radius: px(0.),
            },
            BoxShadow {
                color: hsla(0., 0., 0., 0.1),
                offset: point(px(0.), px(1.)),
                blur_radius: px(2.),
                spread_radius: px(-1.),
            }
        ]);
        self
    }

    fn shadow_none(mut self) -> Self {
        self.declared_style().box_shadow = Some(Default::default());
        self
    }

    fn shadow_sm(mut self) -> Self {
        self.declared_style().box_shadow = Some(smallvec![BoxShadow {
            color: hsla(0., 0., 0., 0.05),
            offset: point(px(0.), px(1.)),
            blur_radius: px(2.),
            spread_radius: px(0.),
        }]);
        self
    }

    fn shadow_md(mut self) -> Self {
        self.declared_style().box_shadow = Some(smallvec![
            BoxShadow {
                color: hsla(0.5, 0., 0., 0.1),
                offset: point(px(0.), px(4.)),
                blur_radius: px(6.),
                spread_radius: px(-1.),
            },
            BoxShadow {
                color: hsla(0., 0., 0., 0.1),
                offset: point(px(0.), px(2.)),
                blur_radius: px(4.),
                spread_radius: px(-2.),
            }
        ]);
        self
    }

    fn shadow_lg(mut self) -> Self {
        self.declared_style().box_shadow = Some(smallvec![
            BoxShadow {
                color: hsla(0., 0., 0., 0.1),
                offset: point(px(0.), px(10.)),
                blur_radius: px(15.),
                spread_radius: px(-3.),
            },
            BoxShadow {
                color: hsla(0., 0., 0., 0.1),
                offset: point(px(0.), px(4.)),
                blur_radius: px(6.),
                spread_radius: px(-4.),
            }
        ]);
        self
    }

    fn shadow_xl(mut self) -> Self {
        self.declared_style().box_shadow = Some(smallvec![
            BoxShadow {
                color: hsla(0., 0., 0., 0.1),
                offset: point(px(0.), px(20.)),
                blur_radius: px(25.),
                spread_radius: px(-5.),
            },
            BoxShadow {
                color: hsla(0., 0., 0., 0.1),
                offset: point(px(0.), px(8.)),
                blur_radius: px(10.),
                spread_radius: px(-6.),
            }
        ]);
        self
    }

    fn shadow_2xl(mut self) -> Self {
        self.declared_style().box_shadow = Some(smallvec![BoxShadow {
            color: hsla(0., 0., 0., 0.25),
            offset: point(px(0.), px(25.)),
            blur_radius: px(50.),
            spread_radius: px(-12.),
        }]);
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

    fn text_decoration_none(mut self) -> Self {
        self.text_style()
            .get_or_insert_with(Default::default)
            .underline = None;
        self
    }

    fn text_decoration_color(mut self, color: impl Into<Hsla>) -> Self {
        let style = self.text_style().get_or_insert_with(Default::default);
        let underline = style.underline.get_or_insert_with(Default::default);
        underline.color = Some(color.into());
        self
    }

    fn text_decoration_solid(mut self) -> Self {
        let style = self.text_style().get_or_insert_with(Default::default);
        let underline = style.underline.get_or_insert_with(Default::default);
        underline.wavy = false;
        self
    }

    fn text_decoration_wavy(mut self) -> Self {
        let style = self.text_style().get_or_insert_with(Default::default);
        let underline = style.underline.get_or_insert_with(Default::default);
        underline.wavy = true;
        self
    }

    fn text_decoration_0(mut self) -> Self {
        let style = self.text_style().get_or_insert_with(Default::default);
        let underline = style.underline.get_or_insert_with(Default::default);
        underline.thickness = px(0.);
        self
    }

    fn text_decoration_1(mut self) -> Self {
        let style = self.text_style().get_or_insert_with(Default::default);
        let underline = style.underline.get_or_insert_with(Default::default);
        underline.thickness = px(1.);
        self
    }

    fn text_decoration_2(mut self) -> Self {
        let style = self.text_style().get_or_insert_with(Default::default);
        let underline = style.underline.get_or_insert_with(Default::default);
        underline.thickness = px(2.);
        self
    }

    fn text_decoration_4(mut self) -> Self {
        let style = self.text_style().get_or_insert_with(Default::default);
        let underline = style.underline.get_or_insert_with(Default::default);
        underline.thickness = px(4.);
        self
    }

    fn text_decoration_8(mut self) -> Self {
        let style = self.text_style().get_or_insert_with(Default::default);
        let underline = style.underline.get_or_insert_with(Default::default);
        underline.thickness = px(8.);
        self
    }

    fn font(mut self, family_name: impl Into<SharedString>) -> Self {
        self.text_style()
            .get_or_insert_with(Default::default)
            .font_family = Some(family_name.into());
        self
    }
}

impl<E: Styled<Style = Style>> StyleHelpers for E {}
