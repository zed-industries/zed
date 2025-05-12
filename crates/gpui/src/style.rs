use std::{
    hash::{Hash, Hasher},
    iter, mem,
    ops::Range,
};

use crate::{
    AbsoluteLength, App, Background, BackgroundTag, BorderStyle, Bounds, ContentMask, Corners,
    CornersRefinement, CursorStyle, DefiniteLength, DevicePixels, Edges, EdgesRefinement, Font,
    FontFallbacks, FontFeatures, FontStyle, FontWeight, Hsla, Length, Pixels, Point,
    PointRefinement, Rgba, SharedString, Size, SizeRefinement, Styled, TextRun, Window, black, phi,
    point, quad, rems, size,
};
use collections::HashSet;
use refineable::Refineable;
use smallvec::SmallVec;
pub use taffy::style::{
    AlignContent, AlignItems, AlignSelf, Display, FlexDirection, FlexWrap, JustifyContent,
    Overflow, Position,
};

/// Use this struct for interfacing with the 'debug_below' styling from your own elements.
/// If a parent element has this style set on it, then this struct will be set as a global in
/// GPUI.
#[cfg(debug_assertions)]
pub struct DebugBelow;

#[cfg(debug_assertions)]
impl crate::Global for DebugBelow {}

/// How to fit the image into the bounds of the element.
pub enum ObjectFit {
    /// The image will be stretched to fill the bounds of the element.
    Fill,
    /// The image will be scaled to fit within the bounds of the element.
    Contain,
    /// The image will be scaled to cover the bounds of the element.
    Cover,
    /// The image will be scaled down to fit within the bounds of the element.
    ScaleDown,
    /// The image will maintain its original size.
    None,
}

impl ObjectFit {
    /// Get the bounds of the image within the given bounds.
    pub fn get_bounds(
        &self,
        bounds: Bounds<Pixels>,
        image_size: Size<DevicePixels>,
    ) -> Bounds<Pixels> {
        let image_size = image_size.map(|dimension| Pixels::from(u32::from(dimension)));
        let image_ratio = image_size.width / image_size.height;
        let bounds_ratio = bounds.size.width / bounds.size.height;

        match self {
            ObjectFit::Fill => bounds,
            ObjectFit::Contain => {
                let new_size = if bounds_ratio > image_ratio {
                    size(
                        image_size.width * (bounds.size.height / image_size.height),
                        bounds.size.height,
                    )
                } else {
                    size(
                        bounds.size.width,
                        image_size.height * (bounds.size.width / image_size.width),
                    )
                };

                Bounds {
                    origin: point(
                        bounds.origin.x + (bounds.size.width - new_size.width) / 2.0,
                        bounds.origin.y + (bounds.size.height - new_size.height) / 2.0,
                    ),
                    size: new_size,
                }
            }
            ObjectFit::ScaleDown => {
                // Check if the image is larger than the bounds in either dimension.
                if image_size.width > bounds.size.width || image_size.height > bounds.size.height {
                    // If the image is larger, use the same logic as Contain to scale it down.
                    let new_size = if bounds_ratio > image_ratio {
                        size(
                            image_size.width * (bounds.size.height / image_size.height),
                            bounds.size.height,
                        )
                    } else {
                        size(
                            bounds.size.width,
                            image_size.height * (bounds.size.width / image_size.width),
                        )
                    };

                    Bounds {
                        origin: point(
                            bounds.origin.x + (bounds.size.width - new_size.width) / 2.0,
                            bounds.origin.y + (bounds.size.height - new_size.height) / 2.0,
                        ),
                        size: new_size,
                    }
                } else {
                    // If the image is smaller than or equal to the container, display it at its original size,
                    // centered within the container.
                    let original_size = size(image_size.width, image_size.height);
                    Bounds {
                        origin: point(
                            bounds.origin.x + (bounds.size.width - original_size.width) / 2.0,
                            bounds.origin.y + (bounds.size.height - original_size.height) / 2.0,
                        ),
                        size: original_size,
                    }
                }
            }
            ObjectFit::Cover => {
                let new_size = if bounds_ratio > image_ratio {
                    size(
                        bounds.size.width,
                        image_size.height * (bounds.size.width / image_size.width),
                    )
                } else {
                    size(
                        image_size.width * (bounds.size.height / image_size.height),
                        bounds.size.height,
                    )
                };

                Bounds {
                    origin: point(
                        bounds.origin.x + (bounds.size.width - new_size.width) / 2.0,
                        bounds.origin.y + (bounds.size.height - new_size.height) / 2.0,
                    ),
                    size: new_size,
                }
            }
            ObjectFit::None => Bounds {
                origin: bounds.origin,
                size: image_size,
            },
        }
    }
}

/// The CSS styling that can be applied to an element via the `Styled` trait
#[derive(Clone, Refineable, Debug)]
#[refineable(Debug)]
pub struct Style {
    /// What layout strategy should be used?
    pub display: Display,

    /// Should the element be painted on screen?
    pub visibility: Visibility,

    // Overflow properties
    /// How children overflowing their container should affect layout
    #[refineable]
    pub overflow: Point<Overflow>,
    /// How much space (in points) should be reserved for the scrollbars of `Overflow::Scroll` and `Overflow::Auto` nodes.
    pub scrollbar_width: f32,
    /// Whether both x and y axis should be scrollable at the same time.
    pub allow_concurrent_scroll: bool,
    /// Whether scrolling should be restricted to the axis indicated by the mouse wheel.
    ///
    /// This means that:
    /// - The mouse wheel alone will only ever scroll the Y axis.
    /// - Holding `Shift` and using the mouse wheel will scroll the X axis.
    ///
    /// ## Motivation
    ///
    /// On the web when scrolling with the mouse wheel, scrolling up and down will always scroll the Y axis, even when
    /// the mouse is over a horizontally-scrollable element.
    ///
    /// The only way to scroll horizontally is to hold down `Shift` while scrolling, which then changes the scroll axis
    /// to the X axis.
    ///
    /// Currently, GPUI operates differently from the web in that it will scroll an element in either the X or Y axis
    /// when scrolling with just the mouse wheel. This causes problems when scrolling in a vertical list that contains
    /// horizontally-scrollable elements, as when you get to the horizontally-scrollable elements the scroll will be
    /// hijacked.
    ///
    /// Ideally we would match the web's behavior and not have a need for this, but right now we're adding this opt-in
    /// style property to limit the potential blast radius.
    pub restrict_scroll_to_axis: bool,

    // Position properties
    /// What should the `position` value of this struct use as a base offset?
    pub position: Position,
    /// How should the position of this element be tweaked relative to the layout defined?
    #[refineable]
    pub inset: Edges<Length>,

    // Size properties
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

    // Flexbox properties
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
    pub background: Option<Fill>,

    /// The border color of this element
    pub border_color: Option<Hsla>,

    /// The border style of this element
    pub border_style: BorderStyle,

    /// The radius of the corners of this element
    #[refineable]
    pub corner_radii: Corners<AbsoluteLength>,

    /// Box shadow of the element
    pub box_shadow: SmallVec<[BoxShadow; 2]>,

    /// The text style of this element
    pub text: TextStyleRefinement,

    /// The mouse cursor style shown when the mouse pointer is over an element.
    pub mouse_cursor: Option<CursorStyle>,

    /// The opacity of this element
    pub opacity: Option<f32>,

    /// Whether to draw a red debugging outline around this element
    #[cfg(debug_assertions)]
    pub debug: bool,

    /// Whether to draw a red debugging outline around this element and all of its conforming children
    #[cfg(debug_assertions)]
    pub debug_below: bool,
}

impl Styled for StyleRefinement {
    fn style(&mut self) -> &mut StyleRefinement {
        self
    }
}

/// The value of the visibility property, similar to the CSS property `visibility`
#[derive(Default, Clone, Copy, Debug, Eq, PartialEq)]
pub enum Visibility {
    /// The element should be drawn as normal.
    #[default]
    Visible,
    /// The element should not be drawn, but should still take up space in the layout.
    Hidden,
}

/// The possible values of the box-shadow property
#[derive(Clone, Debug)]
pub struct BoxShadow {
    /// What color should the shadow have?
    pub color: Hsla,
    /// How should it be offset from its element?
    pub offset: Point<Pixels>,
    /// How much should the shadow be blurred?
    pub blur_radius: Pixels,
    /// How much should the shadow spread?
    pub spread_radius: Pixels,
}

/// How to handle whitespace in text
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum WhiteSpace {
    /// Normal line wrapping when text overflows the width of the element
    #[default]
    Normal,
    /// No line wrapping, text will overflow the width of the element
    Nowrap,
}

/// How to truncate text that overflows the width of the element
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TextOverflow {
    /// Truncate the text with an ellipsis, same as: `text-overflow: ellipsis;` in CSS
    Ellipsis(&'static str),
}

/// How to align text within the element
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum TextAlign {
    /// Align the text to the left of the element
    #[default]
    Left,

    /// Center the text within the element
    Center,

    /// Align the text to the right of the element
    Right,
}

/// The properties that can be used to style text in GPUI
#[derive(Refineable, Clone, Debug, PartialEq)]
#[refineable(Debug)]
pub struct TextStyle {
    /// The color of the text
    pub color: Hsla,

    /// The font family to use
    pub font_family: SharedString,

    /// The font features to use
    pub font_features: FontFeatures,

    /// The fallback fonts to use
    pub font_fallbacks: Option<FontFallbacks>,

    /// The font size to use, in pixels or rems.
    pub font_size: AbsoluteLength,

    /// The line height to use, in pixels or fractions
    pub line_height: DefiniteLength,

    /// The font weight, e.g. bold
    pub font_weight: FontWeight,

    /// The font style, e.g. italic
    pub font_style: FontStyle,

    /// The background color of the text
    pub background_color: Option<Hsla>,

    /// The underline style of the text
    pub underline: Option<UnderlineStyle>,

    /// The strikethrough style of the text
    pub strikethrough: Option<StrikethroughStyle>,

    /// How to handle whitespace in the text
    pub white_space: WhiteSpace,

    /// The text should be truncated if it overflows the width of the element
    pub text_overflow: Option<TextOverflow>,

    /// How the text should be aligned within the element
    pub text_align: TextAlign,

    /// The number of lines to display before truncating the text
    pub line_clamp: Option<usize>,
}

impl Default for TextStyle {
    fn default() -> Self {
        TextStyle {
            color: black(),
            // todo(linux) make this configurable or choose better default
            font_family: if cfg!(any(target_os = "linux", target_os = "freebsd")) {
                "FreeMono".into()
            } else if cfg!(target_os = "windows") {
                "Segoe UI".into()
            } else {
                "Helvetica".into()
            },
            font_features: FontFeatures::default(),
            font_fallbacks: None,
            font_size: rems(1.).into(),
            line_height: phi(),
            font_weight: FontWeight::default(),
            font_style: FontStyle::default(),
            background_color: None,
            underline: None,
            strikethrough: None,
            white_space: WhiteSpace::Normal,
            text_overflow: None,
            text_align: TextAlign::default(),
            line_clamp: None,
        }
    }
}

impl TextStyle {
    /// Create a new text style with the given highlighting applied.
    pub fn highlight(mut self, style: impl Into<HighlightStyle>) -> Self {
        let style = style.into();
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

        if let Some(background_color) = style.background_color {
            self.background_color = Some(background_color);
        }

        if let Some(underline) = style.underline {
            self.underline = Some(underline);
        }

        if let Some(strikethrough) = style.strikethrough {
            self.strikethrough = Some(strikethrough);
        }

        self
    }

    /// Get the font configured for this text style.
    pub fn font(&self) -> Font {
        Font {
            family: self.font_family.clone(),
            features: self.font_features.clone(),
            fallbacks: self.font_fallbacks.clone(),
            weight: self.font_weight,
            style: self.font_style,
        }
    }

    /// Returns the rounded line height in pixels.
    pub fn line_height_in_pixels(&self, rem_size: Pixels) -> Pixels {
        self.line_height.to_pixels(self.font_size, rem_size).round()
    }

    /// Convert this text style into a [`TextRun`], for the given length of the text.
    pub fn to_run(&self, len: usize) -> TextRun {
        TextRun {
            len,
            font: Font {
                family: self.font_family.clone(),
                features: self.font_features.clone(),
                fallbacks: self.font_fallbacks.clone(),
                weight: self.font_weight,
                style: self.font_style,
            },
            color: self.color,
            background_color: self.background_color,
            underline: self.underline,
            strikethrough: self.strikethrough,
        }
    }
}

/// A highlight style to apply, similar to a `TextStyle` except
/// for a single font, uniformly sized and spaced text.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct HighlightStyle {
    /// The color of the text
    pub color: Option<Hsla>,

    /// The font weight, e.g. bold
    pub font_weight: Option<FontWeight>,

    /// The font style, e.g. italic
    pub font_style: Option<FontStyle>,

    /// The background color of the text
    pub background_color: Option<Hsla>,

    /// The underline style of the text
    pub underline: Option<UnderlineStyle>,

    /// The underline style of the text
    pub strikethrough: Option<StrikethroughStyle>,

    /// Similar to the CSS `opacity` property, this will cause the text to be less vibrant.
    pub fade_out: Option<f32>,
}

impl Eq for HighlightStyle {}

impl Hash for HighlightStyle {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.color.hash(state);
        self.font_weight.hash(state);
        self.font_style.hash(state);
        self.background_color.hash(state);
        self.underline.hash(state);
        self.strikethrough.hash(state);
        state.write_u32(u32::from_be_bytes(
            self.fade_out.map(|f| f.to_be_bytes()).unwrap_or_default(),
        ));
    }
}

impl Style {
    /// Returns true if the style is visible and the background is opaque.
    pub fn has_opaque_background(&self) -> bool {
        self.background
            .as_ref()
            .is_some_and(|fill| fill.color().is_some_and(|color| !color.is_transparent()))
    }

    /// Get the text style in this element style.
    pub fn text_style(&self) -> Option<&TextStyleRefinement> {
        if self.text.is_some() {
            Some(&self.text)
        } else {
            None
        }
    }

    /// Get the content mask for this element style, based on the given bounds.
    /// If the element does not hide its overflow, this will return `None`.
    pub fn overflow_mask(
        &self,
        bounds: Bounds<Pixels>,
        rem_size: Pixels,
    ) -> Option<ContentMask<Pixels>> {
        match self.overflow {
            Point {
                x: Overflow::Visible,
                y: Overflow::Visible,
            } => None,
            _ => {
                let mut min = bounds.origin;
                let mut max = bounds.bottom_right();

                if self
                    .border_color
                    .map_or(false, |color| !color.is_transparent())
                {
                    min.x += self.border_widths.left.to_pixels(rem_size);
                    max.x -= self.border_widths.right.to_pixels(rem_size);
                    min.y += self.border_widths.top.to_pixels(rem_size);
                    max.y -= self.border_widths.bottom.to_pixels(rem_size);
                }

                let bounds = match (
                    self.overflow.x == Overflow::Visible,
                    self.overflow.y == Overflow::Visible,
                ) {
                    // x and y both visible
                    (true, true) => return None,
                    // x visible, y hidden
                    (true, false) => Bounds::from_corners(
                        point(min.x, bounds.origin.y),
                        point(max.x, bounds.bottom_right().y),
                    ),
                    // x hidden, y visible
                    (false, true) => Bounds::from_corners(
                        point(bounds.origin.x, min.y),
                        point(bounds.bottom_right().x, max.y),
                    ),
                    // both hidden
                    (false, false) => Bounds::from_corners(min, max),
                };

                Some(ContentMask { bounds })
            }
        }
    }

    /// Paints the background of an element styled with this style.
    pub fn paint(
        &self,
        bounds: Bounds<Pixels>,
        window: &mut Window,
        cx: &mut App,
        continuation: impl FnOnce(&mut Window, &mut App),
    ) {
        #[cfg(debug_assertions)]
        if self.debug_below {
            cx.set_global(DebugBelow)
        }

        #[cfg(debug_assertions)]
        if self.debug || cx.has_global::<DebugBelow>() {
            window.paint_quad(crate::outline(bounds, crate::red(), BorderStyle::default()));
        }

        let rem_size = window.rem_size();
        let corner_radii = self
            .corner_radii
            .to_pixels(rem_size)
            .clamp_radii_for_quad_size(bounds.size);

        window.paint_shadows(bounds, corner_radii, &self.box_shadow);

        let background_color = self.background.as_ref().and_then(Fill::color);
        if background_color.map_or(false, |color| !color.is_transparent()) {
            let mut border_color = match background_color {
                Some(color) => match color.tag {
                    BackgroundTag::Solid => color.solid,
                    BackgroundTag::LinearGradient => color
                        .colors
                        .first()
                        .map(|stop| stop.color)
                        .unwrap_or_default(),
                    BackgroundTag::PatternSlash => color.solid,
                },
                None => Hsla::default(),
            };
            border_color.a = 0.;
            window.paint_quad(quad(
                bounds,
                corner_radii,
                background_color.unwrap_or_default(),
                Edges::default(),
                border_color,
                self.border_style,
            ));
        }

        continuation(window, cx);

        if self.is_border_visible() {
            let border_widths = self.border_widths.to_pixels(rem_size);
            let max_border_width = border_widths.max();
            let max_corner_radius = corner_radii.max();

            let top_bounds = Bounds::from_corners(
                bounds.origin,
                bounds.top_right() + point(Pixels::ZERO, max_border_width.max(max_corner_radius)),
            );
            let bottom_bounds = Bounds::from_corners(
                bounds.bottom_left() - point(Pixels::ZERO, max_border_width.max(max_corner_radius)),
                bounds.bottom_right(),
            );
            let left_bounds = Bounds::from_corners(
                top_bounds.bottom_left(),
                bottom_bounds.origin + point(max_border_width, Pixels::ZERO),
            );
            let right_bounds = Bounds::from_corners(
                top_bounds.bottom_right() - point(max_border_width, Pixels::ZERO),
                bottom_bounds.top_right(),
            );

            let mut background = self.border_color.unwrap_or_default();
            background.a = 0.;
            let quad = quad(
                bounds,
                corner_radii,
                background,
                border_widths,
                self.border_color.unwrap_or_default(),
                self.border_style,
            );

            window.with_content_mask(Some(ContentMask { bounds: top_bounds }), |window| {
                window.paint_quad(quad.clone());
            });
            window.with_content_mask(
                Some(ContentMask {
                    bounds: right_bounds,
                }),
                |window| {
                    window.paint_quad(quad.clone());
                },
            );
            window.with_content_mask(
                Some(ContentMask {
                    bounds: bottom_bounds,
                }),
                |window| {
                    window.paint_quad(quad.clone());
                },
            );
            window.with_content_mask(
                Some(ContentMask {
                    bounds: left_bounds,
                }),
                |window| {
                    window.paint_quad(quad);
                },
            );
        }

        #[cfg(debug_assertions)]
        if self.debug_below {
            cx.remove_global::<DebugBelow>();
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
            visibility: Visibility::Visible,
            overflow: Point {
                x: Overflow::Visible,
                y: Overflow::Visible,
            },
            allow_concurrent_scroll: false,
            restrict_scroll_to_axis: false,
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
            gap: Size::default(),
            // Alignment
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
            background: None,
            border_color: None,
            border_style: BorderStyle::default(),
            corner_radii: Corners::default(),
            box_shadow: Default::default(),
            text: TextStyleRefinement::default(),
            mouse_cursor: None,
            opacity: None,

            #[cfg(debug_assertions)]
            debug: false,
            #[cfg(debug_assertions)]
            debug_below: false,
        }
    }
}

/// The properties that can be applied to an underline.
#[derive(Refineable, Copy, Clone, Default, Debug, PartialEq, Eq, Hash)]
#[refineable(Debug)]
pub struct UnderlineStyle {
    /// The thickness of the underline.
    pub thickness: Pixels,

    /// The color of the underline.
    pub color: Option<Hsla>,

    /// Whether the underline should be wavy, like in a spell checker.
    pub wavy: bool,
}

/// The properties that can be applied to a strikethrough.
#[derive(Refineable, Copy, Clone, Default, Debug, PartialEq, Eq, Hash)]
#[refineable(Debug)]
pub struct StrikethroughStyle {
    /// The thickness of the strikethrough.
    pub thickness: Pixels,

    /// The color of the strikethrough.
    pub color: Option<Hsla>,
}

/// The kinds of fill that can be applied to a shape.
#[derive(Clone, Debug)]
pub enum Fill {
    /// A solid color fill.
    Color(Background),
}

impl Fill {
    /// Unwrap this fill into a solid color, if it is one.
    ///
    /// If the fill is not a solid color, this method returns `None`.
    pub fn color(&self) -> Option<Background> {
        match self {
            Fill::Color(color) => Some(*color),
        }
    }
}

impl Default for Fill {
    fn default() -> Self {
        Self::Color(Background::default())
    }
}

impl From<Hsla> for Fill {
    fn from(color: Hsla) -> Self {
        Self::Color(color.into())
    }
}

impl From<Rgba> for Fill {
    fn from(color: Rgba) -> Self {
        Self::Color(color.into())
    }
}

impl From<Background> for Fill {
    fn from(background: Background) -> Self {
        Self::Color(background)
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
            background_color: other.background_color,
            underline: other.underline,
            strikethrough: other.strikethrough,
            fade_out: None,
        }
    }
}

impl HighlightStyle {
    /// Create a highlight style with just a color
    pub fn color(color: Hsla) -> Self {
        Self {
            color: Some(color),
            ..Default::default()
        }
    }
    /// Blend this highlight style with another.
    /// Non-continuous properties, like font_weight and font_style, are overwritten.
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

        if other.background_color.is_some() {
            self.background_color = other.background_color;
        }

        if other.underline.is_some() {
            self.underline = other.underline;
        }

        if other.strikethrough.is_some() {
            self.strikethrough = other.strikethrough;
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

impl From<FontWeight> for HighlightStyle {
    fn from(font_weight: FontWeight) -> Self {
        Self {
            font_weight: Some(font_weight),
            ..Default::default()
        }
    }
}

impl From<FontStyle> for HighlightStyle {
    fn from(font_style: FontStyle) -> Self {
        Self {
            font_style: Some(font_style),
            ..Default::default()
        }
    }
}

impl From<Rgba> for HighlightStyle {
    fn from(color: Rgba) -> Self {
        Self {
            color: Some(color.into()),
            ..Default::default()
        }
    }
}

/// Combine and merge the highlights and ranges in the two iterators.
pub fn combine_highlights(
    a: impl IntoIterator<Item = (Range<usize>, HighlightStyle)>,
    b: impl IntoIterator<Item = (Range<usize>, HighlightStyle)>,
) -> impl Iterator<Item = (Range<usize>, HighlightStyle)> {
    let mut endpoints = Vec::new();
    let mut highlights = Vec::new();
    for (range, highlight) in a.into_iter().chain(b) {
        if !range.is_empty() {
            let highlight_id = highlights.len();
            endpoints.push((range.start, highlight_id, true));
            endpoints.push((range.end, highlight_id, false));
            highlights.push(highlight);
        }
    }
    endpoints.sort_unstable_by_key(|(position, _, _)| *position);
    let mut endpoints = endpoints.into_iter().peekable();

    let mut active_styles = HashSet::default();
    let mut ix = 0;
    iter::from_fn(move || {
        while let Some((endpoint_ix, highlight_id, is_start)) = endpoints.peek() {
            let prev_index = mem::replace(&mut ix, *endpoint_ix);
            if ix > prev_index && !active_styles.is_empty() {
                let mut current_style = HighlightStyle::default();
                for highlight_id in &active_styles {
                    current_style.highlight(highlights[*highlight_id]);
                }
                return Some((prev_index..ix, current_style));
            }

            if *is_start {
                active_styles.insert(*highlight_id);
            } else {
                active_styles.remove(highlight_id);
            }
            endpoints.next();
        }
        None
    })
}

#[cfg(test)]
mod tests {
    use crate::{blue, green, red, yellow};

    use super::*;

    #[test]
    fn test_combine_highlights() {
        assert_eq!(
            combine_highlights(
                [
                    (0..5, green().into()),
                    (4..10, FontWeight::BOLD.into()),
                    (15..20, yellow().into()),
                ],
                [
                    (2..6, FontStyle::Italic.into()),
                    (1..3, blue().into()),
                    (21..23, red().into()),
                ]
            )
            .collect::<Vec<_>>(),
            [
                (
                    0..1,
                    HighlightStyle {
                        color: Some(green()),
                        ..Default::default()
                    }
                ),
                (
                    1..2,
                    HighlightStyle {
                        color: Some(green()),
                        ..Default::default()
                    }
                ),
                (
                    2..3,
                    HighlightStyle {
                        color: Some(green()),
                        font_style: Some(FontStyle::Italic),
                        ..Default::default()
                    }
                ),
                (
                    3..4,
                    HighlightStyle {
                        color: Some(green()),
                        font_style: Some(FontStyle::Italic),
                        ..Default::default()
                    }
                ),
                (
                    4..5,
                    HighlightStyle {
                        color: Some(green()),
                        font_weight: Some(FontWeight::BOLD),
                        font_style: Some(FontStyle::Italic),
                        ..Default::default()
                    }
                ),
                (
                    5..6,
                    HighlightStyle {
                        font_weight: Some(FontWeight::BOLD),
                        font_style: Some(FontStyle::Italic),
                        ..Default::default()
                    }
                ),
                (
                    6..10,
                    HighlightStyle {
                        font_weight: Some(FontWeight::BOLD),
                        ..Default::default()
                    }
                ),
                (
                    15..20,
                    HighlightStyle {
                        color: Some(yellow()),
                        ..Default::default()
                    }
                ),
                (
                    21..23,
                    HighlightStyle {
                        color: Some(red()),
                        ..Default::default()
                    }
                )
            ]
        );
    }
}
