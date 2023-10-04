use crate::{
    phi, point, rems, AbsoluteLength, BorrowAppContext, BorrowWindow, Bounds, ContentMask, Corners,
    CornersRefinement, DefiniteLength, Edges, EdgesRefinement, Font, FontFeatures, FontStyle,
    FontWeight, Hsla, Length, Pixels, Point, PointRefinement, Quad, Rems, Result, RunStyle,
    SharedString, Size, SizeRefinement, ViewContext, WindowContext,
};
use refineable::Refineable;
pub use taffy::style::{
    AlignContent, AlignItems, AlignSelf, Display, FlexDirection, FlexWrap, JustifyContent,
    Overflow, Position,
};

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
    pub corner_radii: Corners<AbsoluteLength>,

    /// TEXT
    pub text: TextStyleRefinement,
}

#[derive(Refineable, Clone, Debug)]
#[refineable(debug)]
pub struct TextStyle {
    pub color: Hsla,
    pub font_family: SharedString,
    pub font_features: FontFeatures,
    pub font_size: Rems,
    pub line_height: DefiniteLength,
    pub font_weight: FontWeight,
    pub font_style: FontStyle,
    pub underline: Option<UnderlineStyle>,
}

impl Default for TextStyle {
    fn default() -> Self {
        TextStyle {
            color: Hsla::default(),
            font_family: SharedString::default(),
            font_features: FontFeatures::default(),
            font_size: rems(1.),
            line_height: phi(),
            font_weight: FontWeight::default(),
            font_style: FontStyle::default(),
            underline: None,
        }
    }
}

impl TextStyle {
    pub fn highlight(mut self, style: HighlightStyle) -> Result<Self> {
        if let Some(weight) = style.font_weight {
            self.font_weight = weight;
        }
        if let Some(style) = style.font_style {
            self.font_style = style;
        }

        if let Some(color) = style.color {
            self.color = self.color.blend(color);
        }

        if let Some(factor) = style.fade_out {
            self.color.fade_out(factor);
        }

        if let Some(underline) = style.underline {
            self.underline = Some(underline);
        }

        Ok(self)
    }

    pub fn to_run(&self) -> RunStyle {
        RunStyle {
            font: Font {
                family: self.font_family.clone(),
                features: Default::default(),
                weight: self.font_weight,
                style: self.font_style,
            },
            color: self.color,
            underline: self.underline.clone(),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct HighlightStyle {
    pub color: Option<Hsla>,
    pub font_weight: Option<FontWeight>,
    pub font_style: Option<FontStyle>,
    pub underline: Option<UnderlineStyle>,
    pub fade_out: Option<f32>,
}

impl Eq for HighlightStyle {}

impl Style {
    pub fn text_style(&self, _cx: &WindowContext) -> Option<&TextStyleRefinement> {
        if self.text.is_some() {
            Some(&self.text)
        } else {
            None
        }
    }

    pub fn apply_text_style<C, F, R>(&self, cx: &mut C, f: F) -> R
    where
        C: BorrowAppContext,
        F: FnOnce(&mut C) -> R,
    {
        if self.text.is_some() {
            cx.with_text_style(self.text.clone(), f)
        } else {
            f(cx)
        }
    }

    /// Apply overflow to content mask
    pub fn apply_overflow<C, F, R>(&self, bounds: Bounds<Pixels>, cx: &mut C, f: F) -> R
    where
        C: BorrowWindow,
        F: FnOnce(&mut C) -> R,
    {
        let current_mask = cx.content_mask();

        let min = current_mask.bounds.origin;
        let max = current_mask.bounds.lower_right();

        let mask_bounds = match (
            self.overflow.x == Overflow::Visible,
            self.overflow.y == Overflow::Visible,
        ) {
            // x and y both visible
            (true, true) => return f(cx),
            // x visible, y hidden
            (true, false) => Bounds::from_corners(
                point(min.x, bounds.origin.y),
                point(max.x, bounds.lower_right().y),
            ),
            // x hidden, y visible
            (false, true) => Bounds::from_corners(
                point(bounds.origin.x, min.y),
                point(bounds.lower_right().x, max.y),
            ),
            // both hidden
            (false, false) => bounds,
        };
        let mask = ContentMask {
            bounds: mask_bounds,
        };

        cx.with_content_mask(mask, f)
    }

    /// Paints the background of an element styled with this style.
    pub fn paint<V: 'static>(&self, order: u32, bounds: Bounds<Pixels>, cx: &mut ViewContext<V>) {
        let rem_size = cx.rem_size();
        let scale = cx.scale_factor();

        let background_color = self.fill.as_ref().and_then(Fill::color);
        if background_color.is_some() || self.is_border_visible() {
            let layer_id = cx.current_layer_id();
            cx.scene().insert(
                layer_id,
                Quad {
                    order,
                    bounds: bounds.scale(scale),
                    clip_bounds: bounds.scale(scale), // todo!
                    clip_corner_radii: self
                        .corner_radii
                        .map(|length| length.to_pixels(rem_size).scale(scale)),
                    background: background_color.unwrap_or_default(),
                    border_color: self.border_color.unwrap_or_default(),
                    corner_radii: self
                        .corner_radii
                        .map(|length| length.to_pixels(rem_size).scale(scale)),
                    border_widths: self
                        .border_widths
                        .map(|length| length.to_pixels(rem_size).scale(scale)),
                },
            );
        }
    }

    fn is_border_visible(&self) -> bool {
        self.border_color
            .map_or(false, |color| !color.is_transparent())
            && self.border_widths.any(|length| !length.is_zero())
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
            corner_radii: Corners::default(),
            text: TextStyleRefinement::default(),
        }
    }
}

#[derive(Refineable, Clone, Default, Debug, PartialEq, Eq)]
#[refineable(debug)]
pub struct UnderlineStyle {
    pub thickness: Pixels,
    pub color: Option<Hsla>,
    pub squiggly: bool,
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

impl From<TextStyle> for HighlightStyle {
    fn from(other: TextStyle) -> Self {
        Self::from(&other)
    }
}

impl From<&TextStyle> for HighlightStyle {
    fn from(other: &TextStyle) -> Self {
        Self {
            color: Some(other.color),
            font_weight: Some(other.font_weight),
            font_style: Some(other.font_style),
            underline: other.underline.clone(),
            fade_out: None,
        }
    }
}

impl HighlightStyle {
    pub fn highlight(&mut self, other: HighlightStyle) {
        match (self.color, other.color) {
            (Some(self_color), Some(other_color)) => {
                self.color = Some(Hsla::blend(other_color, self_color));
            }
            (None, Some(other_color)) => {
                self.color = Some(other_color);
            }
            _ => {}
        }

        if other.font_weight.is_some() {
            self.font_weight = other.font_weight;
        }

        if other.font_style.is_some() {
            self.font_style = other.font_style;
        }

        if other.underline.is_some() {
            self.underline = other.underline;
        }

        match (other.fade_out, self.fade_out) {
            (Some(source_fade), None) => self.fade_out = Some(source_fade),
            (Some(source_fade), Some(dest_fade)) => {
                self.fade_out = Some((dest_fade * (1. + source_fade)).clamp(0., 1.));
            }
            _ => {}
        }
    }
}

impl From<Hsla> for HighlightStyle {
    fn from(color: Hsla) -> Self {
        Self {
            color: Some(color),
            ..Default::default()
        }
    }
}
