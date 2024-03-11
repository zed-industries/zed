use crate::{
    self as gpui, hsla, point, px, relative, rems, AbsoluteLength, AlignItems, CursorStyle,
    DefiniteLength, Fill, FlexDirection, FlexWrap, FontWeight, Hsla, JustifyContent, Length,
    Position, SharedString, StyleRefinement, Visibility, WhiteSpace,
};
use crate::{BoxShadow, TextStyleRefinement};
use smallvec::{smallvec, SmallVec};
use taffy::style::{AlignContent, Display, Overflow};

/// A trait for elements that can be styled.
/// Use this to opt-in to a CSS-like styling API.
pub trait Styled: Sized {
    /// Returns a reference to the style memory of this element.
    fn style(&mut self) -> &mut StyleRefinement;

    gpui_macros::style_helpers!();

    /// Sets the position of the element to `relative`.
    /// [Docs](https://tailwindcss.com/docs/position)
    fn relative(mut self) -> Self {
        self.style().position = Some(Position::Relative);
        self
    }

    /// Sets the position of the element to `absolute`.
    /// [Docs](https://tailwindcss.com/docs/position)
    fn absolute(mut self) -> Self {
        self.style().position = Some(Position::Absolute);
        self
    }

    /// Sets the display type of the element to `block`.
    /// [Docs](https://tailwindcss.com/docs/display)
    fn block(mut self) -> Self {
        self.style().display = Some(Display::Block);
        self
    }

    /// Sets the display type of the element to `flex`.
    /// [Docs](https://tailwindcss.com/docs/display)
    fn flex(mut self) -> Self {
        self.style().display = Some(Display::Flex);
        self
    }

    /// Sets the visibility of the element to `visible`.
    /// [Docs](https://tailwindcss.com/docs/visibility)
    fn visible(mut self) -> Self {
        self.style().visibility = Some(Visibility::Visible);
        self
    }

    /// Sets the visibility of the element to `hidden`.
    /// [Docs](https://tailwindcss.com/docs/visibility)
    fn invisible(mut self) -> Self {
        self.style().visibility = Some(Visibility::Hidden);
        self
    }

    /// Sets the behavior of content that overflows the container to be hidden.
    /// [Docs](https://tailwindcss.com/docs/overflow#hiding-content-that-overflows)
    fn overflow_hidden(mut self) -> Self {
        self.style().overflow.x = Some(Overflow::Hidden);
        self.style().overflow.y = Some(Overflow::Hidden);
        self
    }

    /// Sets the behavior of content that overflows the container on the X axis to be hidden.
    /// [Docs](https://tailwindcss.com/docs/overflow#hiding-content-that-overflows)
    fn overflow_x_hidden(mut self) -> Self {
        self.style().overflow.x = Some(Overflow::Hidden);
        self
    }

    /// Sets the behavior of content that overflows the container on the Y axis to be hidden.
    /// [Docs](https://tailwindcss.com/docs/overflow#hiding-content-that-overflows)
    fn overflow_y_hidden(mut self) -> Self {
        self.style().overflow.y = Some(Overflow::Hidden);
        self
    }

    /// Set the cursor style when hovering over this element
    fn cursor(mut self, cursor: CursorStyle) -> Self {
        self.style().mouse_cursor = Some(cursor);
        self
    }

    /// Sets the cursor style when hovering an element to `default`.
    /// [Docs](https://tailwindcss.com/docs/cursor)
    fn cursor_default(mut self) -> Self {
        self.style().mouse_cursor = Some(CursorStyle::Arrow);
        self
    }

    /// Sets the cursor style when hovering an element to `pointer`.
    /// [Docs](https://tailwindcss.com/docs/cursor)
    fn cursor_pointer(mut self) -> Self {
        self.style().mouse_cursor = Some(CursorStyle::PointingHand);
        self
    }

    /// Sets cursor style when hovering over an element to `text`.
    /// [Docs](https://tailwindcss.com/docs/cursor)
    fn cursor_text(mut self) -> Self {
        self.style().mouse_cursor = Some(CursorStyle::IBeam);
        self
    }

    /// Sets cursor style when hovering over an element to `move`.
    /// [Docs](https://tailwindcss.com/docs/cursor)
    fn cursor_move(mut self) -> Self {
        self.style().mouse_cursor = Some(CursorStyle::ClosedHand);
        self
    }

    /// Sets cursor style when hovering over an element to `not-allowed`.
    /// [Docs](https://tailwindcss.com/docs/cursor)
    fn cursor_not_allowed(mut self) -> Self {
        self.style().mouse_cursor = Some(CursorStyle::OperationNotAllowed);
        self
    }

    /// Sets cursor style when hovering over an element to `context-menu`.
    /// [Docs](https://tailwindcss.com/docs/cursor)
    fn cursor_context_menu(mut self) -> Self {
        self.style().mouse_cursor = Some(CursorStyle::ContextualMenu);
        self
    }

    /// Sets cursor style when hovering over an element to `crosshair`.
    /// [Docs](https://tailwindcss.com/docs/cursor)
    fn cursor_crosshair(mut self) -> Self {
        self.style().mouse_cursor = Some(CursorStyle::Crosshair);
        self
    }

    /// Sets cursor style when hovering over an element to `vertical-text`.
    /// [Docs](https://tailwindcss.com/docs/cursor)
    fn cursor_vertical_text(mut self) -> Self {
        self.style().mouse_cursor = Some(CursorStyle::IBeamCursorForVerticalLayout);
        self
    }

    /// Sets cursor style when hovering over an element to `alias`.
    /// [Docs](https://tailwindcss.com/docs/cursor)
    fn cursor_alias(mut self) -> Self {
        self.style().mouse_cursor = Some(CursorStyle::DragLink);
        self
    }

    /// Sets cursor style when hovering over an element to `copy`.
    /// [Docs](https://tailwindcss.com/docs/cursor)
    fn cursor_copy(mut self) -> Self {
        self.style().mouse_cursor = Some(CursorStyle::DragCopy);
        self
    }

    /// Sets cursor style when hovering over an element to `no-drop`.
    /// [Docs](https://tailwindcss.com/docs/cursor)
    fn cursor_no_drop(mut self) -> Self {
        self.style().mouse_cursor = Some(CursorStyle::OperationNotAllowed);
        self
    }

    /// Sets cursor style when hovering over an element to `grab`.
    /// [Docs](https://tailwindcss.com/docs/cursor)
    fn cursor_grab(mut self) -> Self {
        self.style().mouse_cursor = Some(CursorStyle::OpenHand);
        self
    }

    /// Sets cursor style when hovering over an element to `grabbing`.
    /// [Docs](https://tailwindcss.com/docs/cursor)
    fn cursor_grabbing(mut self) -> Self {
        self.style().mouse_cursor = Some(CursorStyle::ClosedHand);
        self
    }

    /// Sets cursor style when hovering over an element to `col-resize`.
    /// [Docs](https://tailwindcss.com/docs/cursor)
    fn cursor_col_resize(mut self) -> Self {
        self.style().mouse_cursor = Some(CursorStyle::ResizeLeftRight);
        self
    }

    /// Sets cursor style when hovering over an element to `row-resize`.
    /// [Docs](https://tailwindcss.com/docs/cursor)
    fn cursor_row_resize(mut self) -> Self {
        self.style().mouse_cursor = Some(CursorStyle::ResizeUpDown);
        self
    }

    /// Sets cursor style when hovering over an element to `n-resize`.
    /// [Docs](https://tailwindcss.com/docs/cursor)
    fn cursor_n_resize(mut self) -> Self {
        self.style().mouse_cursor = Some(CursorStyle::ResizeUp);
        self
    }

    /// Sets cursor style when hovering over an element to `e-resize`.
    /// [Docs](https://tailwindcss.com/docs/cursor)
    fn cursor_e_resize(mut self) -> Self {
        self.style().mouse_cursor = Some(CursorStyle::ResizeRight);
        self
    }

    /// Sets cursor style when hovering over an element to `s-resize`.
    /// [Docs](https://tailwindcss.com/docs/cursor)
    fn cursor_s_resize(mut self) -> Self {
        self.style().mouse_cursor = Some(CursorStyle::ResizeDown);
        self
    }

    /// Sets cursor style when hovering over an element to `w-resize`.
    /// [Docs](https://tailwindcss.com/docs/cursor)
    fn cursor_w_resize(mut self) -> Self {
        self.style().mouse_cursor = Some(CursorStyle::ResizeLeft);
        self
    }

    /// Sets the whitespace of the element to `normal`.
    /// [Docs](https://tailwindcss.com/docs/whitespace#normal)
    fn whitespace_normal(mut self) -> Self {
        self.text_style()
            .get_or_insert_with(Default::default)
            .white_space = Some(WhiteSpace::Normal);
        self
    }

    /// Sets the whitespace of the element to `nowrap`.
    /// [Docs](https://tailwindcss.com/docs/whitespace#nowrap)
    fn whitespace_nowrap(mut self) -> Self {
        self.text_style()
            .get_or_insert_with(Default::default)
            .white_space = Some(WhiteSpace::Nowrap);
        self
    }

    /// Sets the flex direction of the element to `column`.
    /// [Docs](https://tailwindcss.com/docs/flex-direction#column)
    fn flex_col(mut self) -> Self {
        self.style().flex_direction = Some(FlexDirection::Column);
        self
    }

    /// Sets the flex direction of the element to `column-reverse`.
    /// [Docs](https://tailwindcss.com/docs/flex-direction#column-reverse)
    fn flex_col_reverse(mut self) -> Self {
        self.style().flex_direction = Some(FlexDirection::ColumnReverse);
        self
    }

    /// Sets the flex direction of the element to `row`.
    /// [Docs](https://tailwindcss.com/docs/flex-direction#row)
    fn flex_row(mut self) -> Self {
        self.style().flex_direction = Some(FlexDirection::Row);
        self
    }

    /// Sets the flex direction of the element to `row-reverse`.
    /// [Docs](https://tailwindcss.com/docs/flex-direction#row-reverse)
    fn flex_row_reverse(mut self) -> Self {
        self.style().flex_direction = Some(FlexDirection::RowReverse);
        self
    }

    /// Sets the element to allow a flex item to grow and shrink as needed, ignoring its initial size.
    /// [Docs](https://tailwindcss.com/docs/flex#flex-1)
    fn flex_1(mut self) -> Self {
        self.style().flex_grow = Some(1.);
        self.style().flex_shrink = Some(1.);
        self.style().flex_basis = Some(relative(0.).into());
        self
    }

    /// Sets the element to allow a flex item to grow and shrink, taking into account its initial size.
    /// [Docs](https://tailwindcss.com/docs/flex#auto)
    fn flex_auto(mut self) -> Self {
        self.style().flex_grow = Some(1.);
        self.style().flex_shrink = Some(1.);
        self.style().flex_basis = Some(Length::Auto);
        self
    }

    /// Sets the element to allow a flex item to shrink but not grow, taking into account its initial size.
    /// [Docs](https://tailwindcss.com/docs/flex#initial)
    fn flex_initial(mut self) -> Self {
        self.style().flex_grow = Some(0.);
        self.style().flex_shrink = Some(1.);
        self.style().flex_basis = Some(Length::Auto);
        self
    }

    /// Sets the element to prevent a flex item from growing or shrinking.
    /// [Docs](https://tailwindcss.com/docs/flex#none)
    fn flex_none(mut self) -> Self {
        self.style().flex_grow = Some(0.);
        self.style().flex_shrink = Some(0.);
        self
    }

    /// Sets the initial size of flex items for this element.
    /// [Docs](https://tailwindcss.com/docs/flex-basis)
    fn flex_basis(mut self, basis: impl Into<Length>) -> Self {
        self.style().flex_basis = Some(basis.into());
        self
    }

    /// Sets the element to allow a flex item to grow to fill any available space.
    /// [Docs](https://tailwindcss.com/docs/flex-grow)
    fn flex_grow(mut self) -> Self {
        self.style().flex_grow = Some(1.);
        self
    }

    /// Sets the element to allow a flex item to shrink if needed.
    /// [Docs](https://tailwindcss.com/docs/flex-shrink)
    fn flex_shrink(mut self) -> Self {
        self.style().flex_shrink = Some(1.);
        self
    }

    /// Sets the element to prevent a flex item from shrinking.
    /// [Docs](https://tailwindcss.com/docs/flex-shrink#dont-shrink)
    fn flex_shrink_0(mut self) -> Self {
        self.style().flex_shrink = Some(0.);
        self
    }

    /// Sets the element to allow flex items to wrap.
    /// [Docs](https://tailwindcss.com/docs/flex-wrap#wrap-normally)
    fn flex_wrap(mut self) -> Self {
        self.style().flex_wrap = Some(FlexWrap::Wrap);
        self
    }

    /// Sets the element wrap flex items in the reverse direction.
    /// [Docs](https://tailwindcss.com/docs/flex-wrap#wrap-reversed)
    fn flex_wrap_reverse(mut self) -> Self {
        self.style().flex_wrap = Some(FlexWrap::WrapReverse);
        self
    }

    /// Sets the element to prevent flex items from wrapping, causing inflexible items to overflow the container if necessary.
    /// [Docs](https://tailwindcss.com/docs/flex-wrap#dont-wrap)
    fn flex_nowrap(mut self) -> Self {
        self.style().flex_wrap = Some(FlexWrap::NoWrap);
        self
    }

    /// Sets the element to align flex items to the start of the container's cross axis.
    /// [Docs](https://tailwindcss.com/docs/align-items#start)
    fn items_start(mut self) -> Self {
        self.style().align_items = Some(AlignItems::FlexStart);
        self
    }

    /// Sets the element to align flex items to the end of the container's cross axis.
    /// [Docs](https://tailwindcss.com/docs/align-items#end)
    fn items_end(mut self) -> Self {
        self.style().align_items = Some(AlignItems::FlexEnd);
        self
    }

    /// Sets the element to align flex items along the center of the container's cross axis.
    /// [Docs](https://tailwindcss.com/docs/align-items#center)
    fn items_center(mut self) -> Self {
        self.style().align_items = Some(AlignItems::Center);
        self
    }

    /// Sets the element to justify flex items along the container's main axis
    /// such that there is an equal amount of space between each item.
    /// [Docs](https://tailwindcss.com/docs/justify-content#space-between)
    fn justify_between(mut self) -> Self {
        self.style().justify_content = Some(JustifyContent::SpaceBetween);
        self
    }

    /// Sets the element to justify flex items along the center of the container's main axis.
    /// [Docs](https://tailwindcss.com/docs/justify-content#center)
    fn justify_center(mut self) -> Self {
        self.style().justify_content = Some(JustifyContent::Center);
        self
    }

    /// Sets the element to justify flex items against the start of the container's main axis.
    /// [Docs](https://tailwindcss.com/docs/justify-content#start)
    fn justify_start(mut self) -> Self {
        self.style().justify_content = Some(JustifyContent::Start);
        self
    }

    /// Sets the element to justify flex items against the end of the container's main axis.
    /// [Docs](https://tailwindcss.com/docs/justify-content#end)
    fn justify_end(mut self) -> Self {
        self.style().justify_content = Some(JustifyContent::End);
        self
    }

    /// Sets the element to justify items along the container's main axis such
    /// that there is an equal amount of space on each side of each item.
    /// [Docs](https://tailwindcss.com/docs/justify-content#space-around)
    fn justify_around(mut self) -> Self {
        self.style().justify_content = Some(JustifyContent::SpaceAround);
        self
    }

    /// Sets the element to pack content items in their default position as if no align-content value was set.
    /// [Docs](https://tailwindcss.com/docs/align-content#normal)
    fn content_normal(mut self) -> Self {
        self.style().align_content = None;
        self
    }

    /// Sets the element to pack content items in the center of the container's cross axis.
    /// [Docs](https://tailwindcss.com/docs/align-content#center)
    fn content_center(mut self) -> Self {
        self.style().align_content = Some(AlignContent::Center);
        self
    }

    /// Sets the element to pack content items against the start of the container's cross axis.
    /// [Docs](https://tailwindcss.com/docs/align-content#start)
    fn content_start(mut self) -> Self {
        self.style().align_content = Some(AlignContent::FlexStart);
        self
    }

    /// Sets the element to pack content items against the end of the container's cross axis.
    /// [Docs](https://tailwindcss.com/docs/align-content#end)
    fn content_end(mut self) -> Self {
        self.style().align_content = Some(AlignContent::FlexEnd);
        self
    }

    /// Sets the element to pack content items along the container's cross axis
    /// such that there is an equal amount of space between each item.
    /// [Docs](https://tailwindcss.com/docs/align-content#space-between)
    fn content_between(mut self) -> Self {
        self.style().align_content = Some(AlignContent::SpaceBetween);
        self
    }

    /// Sets the element to pack content items along the container's cross axis
    /// such that there is an equal amount of space on each side of each item.
    /// [Docs](https://tailwindcss.com/docs/align-content#space-around)
    fn content_around(mut self) -> Self {
        self.style().align_content = Some(AlignContent::SpaceAround);
        self
    }

    /// Sets the element to pack content items along the container's cross axis
    /// such that there is an equal amount of space between each item.
    /// [Docs](https://tailwindcss.com/docs/align-content#space-evenly)
    fn content_evenly(mut self) -> Self {
        self.style().align_content = Some(AlignContent::SpaceEvenly);
        self
    }

    /// Sets the element to allow content items to fill the available space along the container's cross axis.
    /// [Docs](https://tailwindcss.com/docs/align-content#stretch)
    fn content_stretch(mut self) -> Self {
        self.style().align_content = Some(AlignContent::Stretch);
        self
    }

    /// Sets the background color of the element.
    fn bg<F>(mut self, fill: F) -> Self
    where
        F: Into<Fill>,
        Self: Sized,
    {
        self.style().background = Some(fill.into());
        self
    }

    /// Sets the border color of the element.
    fn border_color<C>(mut self, border_color: C) -> Self
    where
        C: Into<Hsla>,
        Self: Sized,
    {
        self.style().border_color = Some(border_color.into());
        self
    }

    /// Sets the box shadow of the element.
    /// [Docs](https://tailwindcss.com/docs/box-shadow)
    fn shadow(mut self, shadows: SmallVec<[BoxShadow; 2]>) -> Self {
        self.style().box_shadow = Some(shadows);
        self
    }

    /// Clears the box shadow of the element.
    /// [Docs](https://tailwindcss.com/docs/box-shadow)
    fn shadow_none(mut self) -> Self {
        self.style().box_shadow = Some(Default::default());
        self
    }

    /// Sets the box shadow of the element.
    /// [Docs](https://tailwindcss.com/docs/box-shadow)
    fn shadow_sm(mut self) -> Self {
        self.style().box_shadow = Some(smallvec::smallvec![BoxShadow {
            color: hsla(0., 0., 0., 0.05),
            offset: point(px(0.), px(1.)),
            blur_radius: px(2.),
            spread_radius: px(0.),
        }]);
        self
    }

    /// Sets the box shadow of the element.
    /// [Docs](https://tailwindcss.com/docs/box-shadow)
    fn shadow_md(mut self) -> Self {
        self.style().box_shadow = Some(smallvec![
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

    /// Sets the box shadow of the element.
    /// [Docs](https://tailwindcss.com/docs/box-shadow)
    fn shadow_lg(mut self) -> Self {
        self.style().box_shadow = Some(smallvec![
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

    /// Sets the box shadow of the element.
    /// [Docs](https://tailwindcss.com/docs/box-shadow)
    fn shadow_xl(mut self) -> Self {
        self.style().box_shadow = Some(smallvec![
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

    /// Sets the box shadow of the element.
    /// [Docs](https://tailwindcss.com/docs/box-shadow)
    fn shadow_2xl(mut self) -> Self {
        self.style().box_shadow = Some(smallvec![BoxShadow {
            color: hsla(0., 0., 0., 0.25),
            offset: point(px(0.), px(25.)),
            blur_radius: px(50.),
            spread_radius: px(-12.),
        }]);
        self
    }

    /// Get the text style that has been configured on this element.
    fn text_style(&mut self) -> &mut Option<TextStyleRefinement> {
        let style: &mut StyleRefinement = self.style();
        &mut style.text
    }

    /// Set the text color of this element, this value cascades to its child elements.
    fn text_color(mut self, color: impl Into<Hsla>) -> Self {
        self.text_style().get_or_insert_with(Default::default).color = Some(color.into());
        self
    }

    /// Set the font weight of this element, this value cascades to its child elements.
    fn font_weight(mut self, weight: FontWeight) -> Self {
        self.text_style()
            .get_or_insert_with(Default::default)
            .font_weight = Some(weight);
        self
    }

    /// Set the background color of this element, this value cascades to its child elements.
    fn text_bg(mut self, bg: impl Into<Hsla>) -> Self {
        self.text_style()
            .get_or_insert_with(Default::default)
            .background_color = Some(bg.into());
        self
    }

    /// Set the text size of this element, this value cascades to its child elements.
    fn text_size(mut self, size: impl Into<AbsoluteLength>) -> Self {
        self.text_style()
            .get_or_insert_with(Default::default)
            .font_size = Some(size.into());
        self
    }

    /// Set the text size to 'extra small',
    /// see the [Tailwind Docs](https://tailwindcss.com/docs/font-size#setting-the-font-size)
    fn text_xs(mut self) -> Self {
        self.text_style()
            .get_or_insert_with(Default::default)
            .font_size = Some(rems(0.75).into());
        self
    }

    /// Set the text size to 'small',
    /// see the [Tailwind Docs](https://tailwindcss.com/docs/font-size#setting-the-font-size)
    fn text_sm(mut self) -> Self {
        self.text_style()
            .get_or_insert_with(Default::default)
            .font_size = Some(rems(0.875).into());
        self
    }

    /// Reset the text styling for this element and its children.
    fn text_base(mut self) -> Self {
        self.text_style()
            .get_or_insert_with(Default::default)
            .font_size = Some(rems(1.0).into());
        self
    }

    /// Set the text size to 'large',
    /// see the [Tailwind Docs](https://tailwindcss.com/docs/font-size#setting-the-font-size)
    fn text_lg(mut self) -> Self {
        self.text_style()
            .get_or_insert_with(Default::default)
            .font_size = Some(rems(1.125).into());
        self
    }

    /// Set the text size to 'extra large',
    /// see the [Tailwind Docs](https://tailwindcss.com/docs/font-size#setting-the-font-size)
    fn text_xl(mut self) -> Self {
        self.text_style()
            .get_or_insert_with(Default::default)
            .font_size = Some(rems(1.25).into());
        self
    }

    /// Set the text size to 'extra-extra large',
    /// see the [Tailwind Docs](https://tailwindcss.com/docs/font-size#setting-the-font-size)
    fn text_2xl(mut self) -> Self {
        self.text_style()
            .get_or_insert_with(Default::default)
            .font_size = Some(rems(1.5).into());
        self
    }

    /// Set the text size to 'extra-extra-extra large',
    /// see the [Tailwind Docs](https://tailwindcss.com/docs/font-size#setting-the-font-size)
    fn text_3xl(mut self) -> Self {
        self.text_style()
            .get_or_insert_with(Default::default)
            .font_size = Some(rems(1.875).into());
        self
    }

    /// Remove the text decoration on this element, this value cascades to its child elements.
    fn text_decoration_none(mut self) -> Self {
        self.text_style()
            .get_or_insert_with(Default::default)
            .underline = None;
        self
    }

    /// Set the color for the underline on this element
    fn text_decoration_color(mut self, color: impl Into<Hsla>) -> Self {
        let style = self.text_style().get_or_insert_with(Default::default);
        let underline = style.underline.get_or_insert_with(Default::default);
        underline.color = Some(color.into());
        self
    }

    /// Set the underline to a solid line
    fn text_decoration_solid(mut self) -> Self {
        let style = self.text_style().get_or_insert_with(Default::default);
        let underline = style.underline.get_or_insert_with(Default::default);
        underline.wavy = false;
        self
    }

    /// Set the underline to a wavy line
    fn text_decoration_wavy(mut self) -> Self {
        let style = self.text_style().get_or_insert_with(Default::default);
        let underline = style.underline.get_or_insert_with(Default::default);
        underline.wavy = true;
        self
    }

    /// Set the underline to be 0 thickness, see the [Tailwind Docs](https://tailwindcss.com/docs/text-decoration-thickness)
    fn text_decoration_0(mut self) -> Self {
        let style = self.text_style().get_or_insert_with(Default::default);
        let underline = style.underline.get_or_insert_with(Default::default);
        underline.thickness = px(0.);
        self
    }

    /// Set the underline to be 1px thick, see the [Tailwind Docs](https://tailwindcss.com/docs/text-decoration-thickness)
    fn text_decoration_1(mut self) -> Self {
        let style = self.text_style().get_or_insert_with(Default::default);
        let underline = style.underline.get_or_insert_with(Default::default);
        underline.thickness = px(1.);
        self
    }

    /// Set the underline to be 2px thick, see the [Tailwind Docs](https://tailwindcss.com/docs/text-decoration-thickness)
    fn text_decoration_2(mut self) -> Self {
        let style = self.text_style().get_or_insert_with(Default::default);
        let underline = style.underline.get_or_insert_with(Default::default);
        underline.thickness = px(2.);
        self
    }

    /// Set the underline to be 4px thick, see the [Tailwind Docs](https://tailwindcss.com/docs/text-decoration-thickness)
    fn text_decoration_4(mut self) -> Self {
        let style = self.text_style().get_or_insert_with(Default::default);
        let underline = style.underline.get_or_insert_with(Default::default);
        underline.thickness = px(4.);
        self
    }

    /// Set the underline to be 8px thick, see the [Tailwind Docs](https://tailwindcss.com/docs/text-decoration-thickness)
    fn text_decoration_8(mut self) -> Self {
        let style = self.text_style().get_or_insert_with(Default::default);
        let underline = style.underline.get_or_insert_with(Default::default);
        underline.thickness = px(8.);
        self
    }

    /// Change the font on this element and its children.
    fn font(mut self, family_name: impl Into<SharedString>) -> Self {
        self.text_style()
            .get_or_insert_with(Default::default)
            .font_family = Some(family_name.into());
        self
    }

    /// Set the line height on this element and its children.
    fn line_height(mut self, line_height: impl Into<DefiniteLength>) -> Self {
        self.text_style()
            .get_or_insert_with(Default::default)
            .line_height = Some(line_height.into());
        self
    }

    /// Draw a debug border around this element.
    #[cfg(debug_assertions)]
    fn debug(mut self) -> Self {
        self.style().debug = Some(true);
        self
    }

    /// Draw a debug border on all conforming elements below this element.
    #[cfg(debug_assertions)]
    fn debug_below(mut self) -> Self {
        self.style().debug_below = Some(true);
        self
    }
}
