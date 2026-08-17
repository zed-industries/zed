use bitflags::bitflags;

/// A parsed ANSI escape sequence.
///
/// Check the [`Parser`](struct.Parser.html) structure documentation for examples
/// how to retrieve these values.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Sequence {
    /// A keyboard event sequence.
    Key(KeyCode, KeyModifiers),
    /// A mouse event sequence.
    Mouse(Mouse, KeyModifiers),
    /// A cursor position (`x`, `y`).
    ///
    /// Top/left cell is represented as `Sequence::CursorPosition(1, 1)`.
    CursorPosition(u16, u16),
}

bitflags! {
    /// A key modifiers.
    pub struct KeyModifiers: u8 {
        const SHIFT = 0b0000_0001;
        const CONTROL = 0b0000_0010;
        const ALT = 0b0000_0100;
        const META = 0b0000_1000;
    }
}

/// A key code.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum KeyCode {
    Backspace,
    Enter,
    Left,
    Right,
    Up,
    Down,
    Home,
    End,
    PageUp,
    PageDown,
    Tab,
    BackTab,
    Delete,
    Insert,
    F(u8),
    Char(char),
    Null,
    Esc,
}

/// A mouse event.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Mouse {
    /// A mouse button press.
    Down(MouseButton, u16, u16),
    /// A mouse button release.
    Up(MouseButton, u16, u16),
    /// A mouse movement with pressed button.
    Drag(MouseButton, u16, u16),
    /// A mouse wheel scrolled up.
    ScrollUp(u16, u16),
    /// A mouse wheel scrolled down.
    ScrollDown(u16, u16),
}

/// A mouse button.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    /// This variant is provided only if [`Parser`](struct.Parser.html) doesn't know which
    /// mouse button was pressed/released.
    ///
    /// An example is [rxvt](https://en.wikipedia.org/wiki/Rxvt) - it provides which mouse
    /// button was pressed, but doesn't provide which mouse button was released.
    Any,
}
