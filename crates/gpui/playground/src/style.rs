use playground_macros::tailwind_lengths;
use taffy::style::{
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
    };

    pub fn new() -> Self {
        Self::DEFAULT.clone()
    }

    // Display ////////////////////

    fn block(mut self) -> Self {
        self.display = Display::Block;
        self
    }

    fn flex(mut self) -> Self {
        self.display = Display::Flex;
        self
    }

    fn grid(mut self) -> Self {
        self.display = Display::Grid;
        self
    }

    // Overflow ///////////////////

    pub fn overflow_visible(mut self) -> Self {
        self.overflow.x = Overflow::Visible;
        self.overflow.y = Overflow::Visible;
        self
    }

    pub fn overflow_hidden(mut self) -> Self {
        self.overflow.x = Overflow::Hidden;
        self.overflow.y = Overflow::Hidden;
        self
    }

    pub fn overflow_scroll(mut self) -> Self {
        self.overflow.x = Overflow::Scroll;
        self.overflow.y = Overflow::Scroll;
        self
    }

    pub fn overflow_x_visible(mut self) -> Self {
        self.overflow.x = Overflow::Visible;
        self
    }

    pub fn overflow_x_hidden(mut self) -> Self {
        self.overflow.x = Overflow::Hidden;
        self
    }

    pub fn overflow_x_scroll(mut self) -> Self {
        self.overflow.x = Overflow::Scroll;
        self
    }

    pub fn overflow_y_visible(mut self) -> Self {
        self.overflow.y = Overflow::Visible;
        self
    }

    pub fn overflow_y_hidden(mut self) -> Self {
        self.overflow.y = Overflow::Hidden;
        self
    }

    pub fn overflow_y_scroll(mut self) -> Self {
        self.overflow.y = Overflow::Scroll;
        self
    }

    // Position ///////////////////

    pub fn relative(mut self) -> Self {
        self.position = Position::Relative;
        self
    }

    pub fn absolute(mut self) -> Self {
        self.position = Position::Absolute;

        self
    }

    #[tailwind_lengths]
    pub fn inset(mut self, length: Length) -> Self {
        self.inset.top = length;
        self.inset.right = length;
        self.inset.bottom = length;
        self.inset.left = length;
        self
    }

    #[tailwind_lengths]
    pub fn w(mut self, length: Length) -> Self {
        self.size.width = length;
        self
    }

    #[tailwind_lengths]
    pub fn min_w(mut self, length: Length) -> Self {
        self.size.width = length;
        self
    }

    #[tailwind_lengths]
    pub fn h(mut self, length: Length) -> Self {
        self.size.height = length;
        self
    }
}

#[derive(Clone)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
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
}

impl Size<LengthOrAuto> {
    pub const fn auto() -> Self {
        Self {
            width: LengthOrAuto::Auto,
            height: LengthOrAuto::Auto,
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
}

#[derive(Clone, Copy)]
pub enum Length {
    Pixels(f32),
    Rems(f32),
    Percent(f32), // 0. - 100.
}

#[derive(Clone, Copy)]
pub enum LengthOrAuto {
    Length(Length),
    Auto,
}

impl From<Length> for LengthOrAuto {
    fn from(value: Length) -> Self {
        LengthOrAuto::Length(value)
    }
}
