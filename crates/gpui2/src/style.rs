use crate::{
    color::Hsla,
    elements::hoverable::{hoverable, Hoverable},
    elements::pressable::{pressable, Pressable},
    ViewContext,
};
pub use fonts::Style as FontStyle;
pub use fonts::Weight as FontWeight;
pub use gpui::taffy::style::{
    AlignContent, AlignItems, AlignSelf, Display, FlexDirection, FlexWrap, JustifyContent,
    Overflow, Position,
};
use gpui::{
    fonts::{self, TextStyleRefinement},
    geometry::{
        rect::RectF, relative, vector::Vector2F, AbsoluteLength, DefiniteLength, Edges,
        EdgesRefinement, Length, Point, PointRefinement, Size, SizeRefinement,
    },
    scene, taffy, WindowContext,
};
use gpui2_macros::styleable_helpers;
use refineable::{Refineable, RefinementCascade};
use std::sync::Arc;

pub type StyleCascade = RefinementCascade<Style>;

#[derive(Clone, Refineable, Debug)]
#[refineable(debug)]
pub struct Style {
    /// What layout strategy should be used?
    pub display: Display,

    // Overflow properties
    /// How children overflowing their container should affect layout
    #[refineable]
    pub overflow: Point<Overflow>,
    /// How much space (in points) should be reserved for the scrollbars of `Overflow::Scroll` and `Overflow::Auto` nodes.
    pub scrollbar_width: f32,

    // Position properties
    /// What should the `position` value of this struct use as a base offset?
    pub position: Position,
    /// How should the position of this element be tweaked relative to the layout defined?
    #[refineable]
    pub inset: Edges<Length>,

    // Size properies
    /// Sets the initial size of the item
    #[refineable]
    pub size: Size<Length>,
    /// Controls the minimum size of the item
    #[refineable]
    pub min_size: Size<Length>,
    /// Controls the maximum size of the item
    #[refineable]
    pub max_size: Size<Length>,
    /// Sets the preferred aspect ratio for the item. The ratio is calculated as width divided by height.
    pub aspect_ratio: Option<f32>,

    // Spacing Properties
    /// How large should the margin be on each side?
    #[refineable]
    pub margin: Edges<Length>,
    /// How large should the padding be on each side?
    #[refineable]
    pub padding: Edges<DefiniteLength>,
    /// How large should the border be on each side?
    #[refineable]
    pub border_widths: Edges<AbsoluteLength>,

    // Alignment properties
    /// How this node's children aligned in the cross/block axis?
    pub align_items: Option<AlignItems>,
    /// How this node should be aligned in the cross/block axis. Falls back to the parents [`AlignItems`] if not set
    pub align_self: Option<AlignSelf>,
    /// How should content contained within this item be aligned in the cross/block axis
    pub align_content: Option<AlignContent>,
    /// How should contained within this item be aligned in the main/inline axis
    pub justify_content: Option<JustifyContent>,
    /// How large should the gaps between items in a flex container be?
    #[refineable]
    pub gap: Size<DefiniteLength>,

    // Flexbox properies
    /// Which direction does the main axis flow in?
    pub flex_direction: FlexDirection,
    /// Should elements wrap, or stay in a single line?
    pub flex_wrap: FlexWrap,
    /// Sets the initial main axis size of the item
    pub flex_basis: Length,
    /// The relative rate at which this item grows when it is expanding to fill space, 0.0 is the default value, and this value must be positive.
    pub flex_grow: f32,
    /// The relative rate at which this item shrinks when it is contracting to fit into space, 1.0 is the default value, and this value must be positive.
    pub flex_shrink: f32,

    /// The fill color of this element
    pub fill: Option<Fill>,

    /// The border color of this element
    pub border_color: Option<Hsla>,

    /// The radius of the corners of this element
    #[refineable]
    pub corner_radii: CornerRadii,

    /// The color of text within this element. Cascades to children unless overridden.
    pub text_color: Option<Hsla>,

    /// The font size in rems.
    pub font_size: Option<f32>,

    pub font_family: Option<Arc<str>>,

    pub font_weight: Option<FontWeight>,

    pub font_style: Option<FontStyle>,
}

impl Style {
    pub fn text_style(&self, cx: &WindowContext) -> Option<TextStyleRefinement> {
        if self.text_color.is_none()
            && self.font_size.is_none()
            && self.font_family.is_none()
            && self.font_weight.is_none()
            && self.font_style.is_none()
        {
            return None;
        }

        Some(TextStyleRefinement {
            color: self.text_color.map(Into::into),
            font_family: self.font_family.clone(),
            font_size: self.font_size.map(|size| size * cx.rem_size()),
            font_weight: self.font_weight,
            font_style: self.font_style,
            underline: None,
        })
    }

    pub fn to_taffy(&self, rem_size: f32) -> taffy::style::Style {
        taffy::style::Style {
            display: self.display,
            overflow: self.overflow.clone().into(),
            scrollbar_width: self.scrollbar_width,
            position: self.position,
            inset: self.inset.to_taffy(rem_size),
            size: self.size.to_taffy(rem_size),
            min_size: self.min_size.to_taffy(rem_size),
            max_size: self.max_size.to_taffy(rem_size),
            aspect_ratio: self.aspect_ratio,
            margin: self.margin.to_taffy(rem_size),
            padding: self.padding.to_taffy(rem_size),
            border: self.border_widths.to_taffy(rem_size),
            align_items: self.align_items,
            align_self: self.align_self,
            align_content: self.align_content,
            justify_content: self.justify_content,
            gap: self.gap.to_taffy(rem_size),
            flex_direction: self.flex_direction,
            flex_wrap: self.flex_wrap,
            flex_basis: self.flex_basis.to_taffy(rem_size).into(),
            flex_grow: self.flex_grow,
            flex_shrink: self.flex_shrink,
            ..Default::default() // Ignore grid properties for now
        }
    }

    /// Paints the background of an element styled with this style.
    pub fn paint_background<V: 'static>(&self, bounds: RectF, cx: &mut ViewContext<V>) {
        let rem_size = cx.rem_size();
        if let Some(color) = self.fill.as_ref().and_then(Fill::color) {
            cx.scene().push_quad(gpui::Quad {
                bounds,
                background: Some(color.into()),
                corner_radii: self.corner_radii.to_gpui(bounds.size(), rem_size),
                border: Default::default(),
            });
        }
    }

    /// Paints the foreground of an element styled with this style.
    pub fn paint_foreground<V: 'static>(&self, bounds: RectF, cx: &mut ViewContext<V>) {
        let rem_size = cx.rem_size();

        if let Some(color) = self.border_color {
            let border = self.border_widths.to_pixels(rem_size);
            if !border.is_empty() {
                cx.scene().push_quad(gpui::Quad {
                    bounds,
                    background: None,
                    corner_radii: self.corner_radii.to_gpui(bounds.size(), rem_size),
                    border: scene::Border {
                        color: color.into(),
                        top: border.top,
                        right: border.right,
                        bottom: border.bottom,
                        left: border.left,
                    },
                });
            }
        }
    }
}

impl Default for Style {
    fn default() -> Self {
        Style {
            display: Display::Block,
            overflow: Point {
                x: Overflow::Visible,
                y: Overflow::Visible,
            },
            scrollbar_width: 0.0,
            position: Position::Relative,
            inset: Edges::auto(),
            margin: Edges::<Length>::zero(),
            padding: Edges::<DefiniteLength>::zero(),
            border_widths: Edges::<AbsoluteLength>::zero(),
            size: Size::auto(),
            min_size: Size::auto(),
            max_size: Size::auto(),
            aspect_ratio: None,
            gap: Size::zero(),
            // Aligment
            align_items: None,
            align_self: None,
            align_content: None,
            justify_content: None,
            // Flexbox
            flex_direction: FlexDirection::Row,
            flex_wrap: FlexWrap::NoWrap,
            flex_grow: 0.0,
            flex_shrink: 1.0,
            flex_basis: Length::Auto,
            fill: None,
            border_color: None,
            corner_radii: CornerRadii::default(),
            text_color: None,
            font_size: Some(1.),
            font_family: None,
            font_weight: None,
            font_style: None,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Fill {
    Color(Hsla),
}

impl Fill {
    pub fn color(&self) -> Option<Hsla> {
        match self {
            Fill::Color(color) => Some(*color),
        }
    }
}

impl Default for Fill {
    fn default() -> Self {
        Self::Color(Hsla::default())
    }
}

impl From<Hsla> for Fill {
    fn from(color: Hsla) -> Self {
        Self::Color(color)
    }
}

#[derive(Clone, Refineable, Default, Debug)]
#[refineable(debug)]
pub struct CornerRadii {
    top_left: AbsoluteLength,
    top_right: AbsoluteLength,
    bottom_left: AbsoluteLength,
    bottom_right: AbsoluteLength,
}

impl CornerRadii {
    pub fn to_gpui(&self, box_size: Vector2F, rem_size: f32) -> gpui::scene::CornerRadii {
        let max_radius = box_size.x().min(box_size.y()) / 2.;

        gpui::scene::CornerRadii {
            top_left: self.top_left.to_pixels(rem_size).min(max_radius),
            top_right: self.top_right.to_pixels(rem_size).min(max_radius),
            bottom_left: self.bottom_left.to_pixels(rem_size).min(max_radius),
            bottom_right: self.bottom_right.to_pixels(rem_size).min(max_radius),
        }
    }
}

pub trait Styleable {
    type Style: Refineable + Default;

    fn style_cascade(&mut self) -> &mut RefinementCascade<Self::Style>;
    fn declared_style(&mut self) -> &mut <Self::Style as Refineable>::Refinement;

    fn computed_style(&mut self) -> Self::Style {
        Self::Style::from_refinement(&self.style_cascade().merged())
    }

    fn hover(self) -> Hoverable<Self>
    where
        Self: Sized,
    {
        hoverable(self)
    }

    fn active(self) -> Pressable<Self>
    where
        Self: Sized,
    {
        pressable(self)
    }
}

// Helpers methods that take and return mut self. This includes tailwind style methods for standard sizes etc.
//
// Example:
// // Sets the padding to 0.5rem, just like class="p-2" in Tailwind.
// fn p_2(mut self) -> Self where Self: Sized;
pub trait StyleHelpers: Styleable<Style = Style> {
    styleable_helpers!();

    fn h(mut self, height: Length) -> Self
    where
        Self: Sized,
    {
        self.declared_style().size.height = Some(height);
        self
    }

    /// size_{n}: Sets width & height to {n}
    ///
    /// Example:
    /// size_1: Sets width & height to 1
    fn size(mut self, size: Length) -> Self
    where
        Self: Sized,
    {
        self.declared_style().size.height = Some(size);
        self.declared_style().size.width = Some(size);
        self
    }

    fn full(mut self) -> Self
    where
        Self: Sized,
    {
        self.declared_style().size.width = Some(relative(1.));
        self.declared_style().size.height = Some(relative(1.));
        self
    }

    fn relative(mut self) -> Self
    where
        Self: Sized,
    {
        self.declared_style().position = Some(Position::Relative);
        self
    }

    fn absolute(mut self) -> Self
    where
        Self: Sized,
    {
        self.declared_style().position = Some(Position::Absolute);
        self
    }

    fn block(mut self) -> Self
    where
        Self: Sized,
    {
        self.declared_style().display = Some(Display::Block);
        self
    }

    fn flex(mut self) -> Self
    where
        Self: Sized,
    {
        self.declared_style().display = Some(Display::Flex);
        self
    }

    fn flex_col(mut self) -> Self
    where
        Self: Sized,
    {
        self.declared_style().flex_direction = Some(FlexDirection::Column);
        self
    }

    fn flex_row(mut self) -> Self
    where
        Self: Sized,
    {
        self.declared_style().flex_direction = Some(FlexDirection::Row);
        self
    }

    fn flex_1(mut self) -> Self
    where
        Self: Sized,
    {
        self.declared_style().flex_grow = Some(1.);
        self.declared_style().flex_shrink = Some(1.);
        self.declared_style().flex_basis = Some(relative(0.));
        self
    }

    fn flex_auto(mut self) -> Self
    where
        Self: Sized,
    {
        self.declared_style().flex_grow = Some(1.);
        self.declared_style().flex_shrink = Some(1.);
        self.declared_style().flex_basis = Some(Length::Auto);
        self
    }

    fn flex_initial(mut self) -> Self
    where
        Self: Sized,
    {
        self.declared_style().flex_grow = Some(0.);
        self.declared_style().flex_shrink = Some(1.);
        self.declared_style().flex_basis = Some(Length::Auto);
        self
    }

    fn flex_none(mut self) -> Self
    where
        Self: Sized,
    {
        self.declared_style().flex_grow = Some(0.);
        self.declared_style().flex_shrink = Some(0.);
        self
    }

    fn grow(mut self) -> Self
    where
        Self: Sized,
    {
        self.declared_style().flex_grow = Some(1.);
        self
    }

    fn items_start(mut self) -> Self
    where
        Self: Sized,
    {
        self.declared_style().align_items = Some(AlignItems::FlexStart);
        self
    }

    fn items_end(mut self) -> Self
    where
        Self: Sized,
    {
        self.declared_style().align_items = Some(AlignItems::FlexEnd);
        self
    }

    fn items_center(mut self) -> Self
    where
        Self: Sized,
    {
        self.declared_style().align_items = Some(AlignItems::Center);
        self
    }

    fn justify_between(mut self) -> Self
    where
        Self: Sized,
    {
        self.declared_style().justify_content = Some(JustifyContent::SpaceBetween);
        self
    }

    fn justify_center(mut self) -> Self
    where
        Self: Sized,
    {
        self.declared_style().justify_content = Some(JustifyContent::Center);
        self
    }

    fn justify_start(mut self) -> Self
    where
        Self: Sized,
    {
        self.declared_style().justify_content = Some(JustifyContent::Start);
        self
    }

    fn justify_end(mut self) -> Self
    where
        Self: Sized,
    {
        self.declared_style().justify_content = Some(JustifyContent::End);
        self
    }

    fn justify_around(mut self) -> Self
    where
        Self: Sized,
    {
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

    fn text_color<C>(mut self, color: C) -> Self
    where
        C: Into<Hsla>,
        Self: Sized,
    {
        self.declared_style().text_color = Some(color.into());
        self
    }

    fn text_xs(mut self) -> Self
    where
        Self: Sized,
    {
        self.declared_style().font_size = Some(0.75);
        self
    }

    fn text_sm(mut self) -> Self
    where
        Self: Sized,
    {
        self.declared_style().font_size = Some(0.875);
        self
    }

    fn text_base(mut self) -> Self
    where
        Self: Sized,
    {
        self.declared_style().font_size = Some(1.0);
        self
    }

    fn text_lg(mut self) -> Self
    where
        Self: Sized,
    {
        self.declared_style().font_size = Some(1.125);
        self
    }

    fn text_xl(mut self) -> Self
    where
        Self: Sized,
    {
        self.declared_style().font_size = Some(1.25);
        self
    }

    fn text_2xl(mut self) -> Self
    where
        Self: Sized,
    {
        self.declared_style().font_size = Some(1.5);
        self
    }

    fn text_3xl(mut self) -> Self
    where
        Self: Sized,
    {
        self.declared_style().font_size = Some(1.875);
        self
    }

    fn font(mut self, family_name: impl Into<Arc<str>>) -> Self
    where
        Self: Sized,
    {
        self.declared_style().font_family = Some(family_name.into());
        self
    }
}
