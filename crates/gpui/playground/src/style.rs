use crate::color::Hsla;
pub use taffy::style::{
    AlignContent, AlignItems, AlignSelf, Display, FlexDirection, FlexWrap, JustifyContent,
    Overflow, Position,
};

#[derive(Clone)]
pub struct Style {
    /// What layout strategy should be used?
    pub display: Display,

    // Overflow properties
    /// How children overflowing their container should affect layout
    pub overflow: Point<Overflow>,
    /// How much space (in points) should be reserved for the scrollbars of `Overflow::Scroll` and `Overflow::Auto` nodes.
    pub scrollbar_width: f32,

    // Position properties
    /// What should the `position` value of this struct use as a base offset?
    pub position: Position,
    /// How should the position of this element be tweaked relative to the layout defined?
    pub inset: Edges<LengthOrAuto>,

    // Size properies
    /// Sets the initial size of the item
    pub size: Size<LengthOrAuto>,
    /// Controls the minimum size of the item
    pub min_size: Size<LengthOrAuto>,
    /// Controls the maximum size of the item
    pub max_size: Size<LengthOrAuto>,
    /// Sets the preferred aspect ratio for the item. The ratio is calculated as width divided by height.
    pub aspect_ratio: Option<f32>,

    // Spacing Properties
    /// How large should the margin be on each side?
    pub margin: Edges<LengthOrAuto>,
    /// How large should the padding be on each side?
    pub padding: Edges<Length>,
    /// How large should the border be on each side?
    pub border: Edges<Length>,

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
    pub gap: Size<Length>,

    // Flexbox properies
    /// Which direction does the main axis flow in?
    pub flex_direction: FlexDirection,
    /// Should elements wrap, or stay in a single line?
    pub flex_wrap: FlexWrap,
    /// Sets the initial main axis size of the item
    pub flex_basis: LengthOrAuto,
    /// The relative rate at which this item grows when it is expanding to fill space, 0.0 is the default value, and this value must be positive.
    pub flex_grow: f32,
    /// The relative rate at which this item shrinks when it is contracting to fit into space, 1.0 is the default value, and this value must be positive.
    pub flex_shrink: f32,

    /// The fill color of this element
    pub fill: Fill,
}

impl Style {
    pub const DEFAULT: Style = Style {
        display: Display::DEFAULT,
        overflow: Point {
            x: Overflow::Visible,
            y: Overflow::Visible,
        },
        scrollbar_width: 0.0,
        position: Position::Relative,
        inset: Edges::auto(),
        margin: Edges::<LengthOrAuto>::zero(),
        padding: Edges::<Length>::zero(),
        border: Edges::<Length>::zero(),
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
        flex_basis: LengthOrAuto::Auto,
        fill: Fill::Color(Hsla {
            h: 0.,
            s: 0.,
            l: 0.,
            a: 0.,
        }),
    };

    pub fn new() -> Self {
        Self::DEFAULT.clone()
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
            border: self.border.to_taffy(rem_size),
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
}

#[derive(Clone)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Into<taffy::geometry::Point<T>> for Point<T> {
    fn into(self) -> taffy::geometry::Point<T> {
        taffy::geometry::Point {
            x: self.x,
            y: self.y,
        }
    }
}

#[derive(Clone)]
pub struct Size<T> {
    pub width: T,
    pub height: T,
}

impl Size<Length> {
    pub const fn zero() -> Self {
        Self {
            width: Length::Pixels(0.),
            height: Length::Pixels(0.),
        }
    }

    pub fn to_taffy(&self, rem_size: f32) -> taffy::geometry::Size<taffy::style::LengthPercentage> {
        taffy::geometry::Size {
            width: self.width.to_taffy(rem_size),
            height: self.height.to_taffy(rem_size),
        }
    }
}

impl Size<LengthOrAuto> {
    pub const fn auto() -> Self {
        Self {
            width: LengthOrAuto::Auto,
            height: LengthOrAuto::Auto,
        }
    }

    pub fn to_taffy<T: From<taffy::prelude::LengthPercentageAuto>>(
        &self,
        rem_size: f32,
    ) -> taffy::geometry::Size<T> {
        taffy::geometry::Size {
            width: self.width.to_taffy(rem_size).into(),
            height: self.height.to_taffy(rem_size).into(),
        }
    }
}

#[derive(Clone)]
pub struct Edges<T> {
    pub top: T,
    pub right: T,
    pub bottom: T,
    pub left: T,
}

impl Edges<Length> {
    pub const fn zero() -> Self {
        Self {
            top: Length::Pixels(0.0),
            right: Length::Pixels(0.0),
            bottom: Length::Pixels(0.0),
            left: Length::Pixels(0.0),
        }
    }

    pub fn to_taffy(&self, rem_size: f32) -> taffy::geometry::Rect<taffy::style::LengthPercentage> {
        taffy::geometry::Rect {
            top: self.top.to_taffy(rem_size),
            right: self.right.to_taffy(rem_size),
            bottom: self.bottom.to_taffy(rem_size),
            left: self.left.to_taffy(rem_size),
        }
    }
}

impl Edges<LengthOrAuto> {
    pub const fn auto() -> Self {
        Self {
            top: LengthOrAuto::Auto,
            right: LengthOrAuto::Auto,
            bottom: LengthOrAuto::Auto,
            left: LengthOrAuto::Auto,
        }
    }

    pub const fn zero() -> Self {
        Self {
            top: LengthOrAuto::Length(Length::Pixels(0.0)),
            right: LengthOrAuto::Length(Length::Pixels(0.0)),
            bottom: LengthOrAuto::Length(Length::Pixels(0.0)),
            left: LengthOrAuto::Length(Length::Pixels(0.0)),
        }
    }

    pub fn to_taffy(
        &self,
        rem_size: f32,
    ) -> taffy::geometry::Rect<taffy::style::LengthPercentageAuto> {
        taffy::geometry::Rect {
            top: self.top.to_taffy(rem_size),
            right: self.right.to_taffy(rem_size),
            bottom: self.bottom.to_taffy(rem_size),
            left: self.left.to_taffy(rem_size),
        }
    }
}

#[derive(Clone, Copy)]
pub enum Length {
    Pixels(f32),
    Rems(f32),
    Percent(f32), // 0. - 100.
}

impl Length {
    fn to_taffy(&self, rem_size: f32) -> taffy::style::LengthPercentage {
        match self {
            Length::Pixels(pixels) => taffy::style::LengthPercentage::Length(*pixels),
            Length::Rems(rems) => taffy::style::LengthPercentage::Length(rems * rem_size),
            Length::Percent(percent) => taffy::style::LengthPercentage::Percent(*percent),
        }
    }
}

#[derive(Clone, Copy)]
pub enum LengthOrAuto {
    Length(Length),
    Auto,
}

impl LengthOrAuto {
    fn to_taffy(&self, rem_size: f32) -> taffy::prelude::LengthPercentageAuto {
        match self {
            LengthOrAuto::Length(length) => length.to_taffy(rem_size).into(),
            LengthOrAuto::Auto => taffy::prelude::LengthPercentageAuto::Auto,
        }
    }
}

impl From<Length> for LengthOrAuto {
    fn from(value: Length) -> Self {
        LengthOrAuto::Length(value)
    }
}

#[derive(Clone)]
pub enum Fill {
    Color(Hsla),
}

impl Default for Fill {
    fn default() -> Self {
        Self::Color(Hsla::default())
    }
}
