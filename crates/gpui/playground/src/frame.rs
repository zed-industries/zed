use playground_macros::tailwind_lengths;
use taffy::style::{Position, *};

#[derive(Clone, Debug, Default)]
struct Style {
    display: Display,
    position: Position,
    overflow: Point<Overflow>,
    inset: Edges<Length>,
    size: Size<Length>,
    max_size: Size<Length>,
    min_size: Size<Length>,
}

impl Style {
    pub fn new() -> Self {
        Self::default()
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

#[derive(Clone, Copy, Debug)]
pub enum Length {
    Rems(f32),
    Pixels(f32),
    Percent(f32),
    Auto,
}

impl Default for Length {
    fn default() -> Self {
        Self::Rems(0.)
    }
}

#[derive(Clone, Default, Debug)]
pub struct Edges<T> {
    top: T,
    bottom: T,
    left: T,
    right: T,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Size<T> {
    width: T,
    height: T,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Point<T> {
    x: T,
    y: T,
}

#[test]
fn test_style() {
    Style::new().inset_1_5();
}
