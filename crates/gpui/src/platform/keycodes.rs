use crate::{KeyboardMapper, Modifiers};

/// On Windows, this is the Virtual-Key Codes
/// https://docs.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes
/// On macOS and Linux, this is the Scan Codes
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum KeyCode {
    /// Un-recognized key
    #[default]
    Unknown,
    /// Fn on macOS
    Function,
    /// Control-break processing, `VK_CANCEL` on Windows.
    Cancel,
    /// BACKSPACE key, `VK_BACK` on Windows.
    Backspace,
    /// TAB key, `VK_TAB` on Windows.
    Tab,
    /// CLEAR key, `VK_CLEAR` on Windows.
    Clear,
    /// RETURN key, `VK_RETURN` on Windows.
    Enter,
    /// SHIFT key, `VK_SHIFT` on Windows. Note, both left-shift and right-shift can
    /// trigger this.
    Shift(KeyPosition),
    /// CTRL key, `VK_CONTROL` on Windows. Note, both left-ctrl and right-ctrl can
    /// trigger this.
    Control(KeyPosition),
    /// Alt key, `VK_MENU` on Windows. Note, both left-alt and right-alt can
    /// trigger this.
    Alt(KeyPosition),
    /// PAUSE key, `VK_PAUSE` on Windows.
    Pause,
    /// CAPS LOCK key, `VK_CAPITAL` on Windows.
    Capital,
    /// ESC key, `VK_ESCAPE` on Windows.
    Escape,
    /// SPACEBAR, `VK_SPACE` on Windows.
    Space,
    /// PAGE UP key, `VK_PRIOR` on Windows.
    PageUp,
    /// PAGE DOWN key, `VK_NEXT` on Windows.
    PageDown,
    /// END key, `VK_END` on Windows.
    End,
    /// HOME key, `VK_HOME` on Windows.
    Home,
    /// LEFT ARROW key, `VK_LEFT` on Windows.
    Left,
    /// UP ARROW key, `VK_UP` on Windows.
    Up,
    /// RIGHT ARROW key, `VK_RIGHT` on Winodws.
    Right,
    /// DOWN ARROW key, `VK_DOWN` on Windows.
    Down,
    /// SELECT key, `VK_SELECT` on Winodws.
    Select,
    /// PRINT key, `VK_PRINT` on Windows.
    Print,
    /// PRINT SCREEN key, `VK_SNAPSHOT` on Windows.
    PrintScreen,
    /// INS key, `VK_INSERT` on Windows.
    Insert,
    /// DEL key, `VK_DELETE` on Windows.
    Delete,
    /// 0 key on the main keyboard, `VK_0` on Windows.
    Digital0,
    /// 0 key on the main keyboard, `VK_0` on Windows.
    Digital1,
    /// 0 key on the main keyboard, `VK_0` on Windows.
    Digital2,
    /// 0 key on the main keyboard, `VK_0` on Windows.
    Digital3,
    /// 0 key on the main keyboard, `VK_0` on Windows.
    Digital4,
    /// 0 key on the main keyboard, `VK_0` on Windows.
    Digital5,
    /// 0 key on the main keyboard, `VK_0` on Windows.
    Digital6,
    /// 0 key on the main keyboard, `VK_0` on Windows.
    Digital7,
    /// 0 key on the main keyboard, `VK_0` on Windows.
    Digital8,
    /// 0 key on the main keyboard, `VK_0` on Windows.
    Digital9,
    /// A key on the main keyboard, `VK_A` on Windows.
    A,
    /// A key on the main keyboard, `VK_A` on Windows.
    B,
    /// A key on the main keyboard, `VK_A` on Windows.
    C,
    /// A key on the main keyboard, `VK_A` on Windows.
    D,
    /// A key on the main keyboard, `VK_A` on Windows.
    E,
    /// A key on the main keyboard, `VK_A` on Windows.
    F,
    /// A key on the main keyboard, `VK_A` on Windows.
    G,
    /// A key on the main keyboard, `VK_A` on Windows.
    H,
    /// A key on the main keyboard, `VK_A` on Windows.
    I,
    /// A key on the main keyboard, `VK_A` on Windows.
    J,
    /// A key on the main keyboard, `VK_A` on Windows.
    K,
    /// A key on the main keyboard, `VK_A` on Windows.
    L,
    /// A key on the main keyboard, `VK_A` on Windows.
    M,
    /// A key on the main keyboard, `VK_A` on Windows.
    N,
    /// A key on the main keyboard, `VK_A` on Windows.
    O,
    /// A key on the main keyboard, `VK_A` on Windows.
    P,
    /// A key on the main keyboard, `VK_A` on Windows.
    Q,
    /// A key on the main keyboard, `VK_A` on Windows.
    R,
    /// A key on the main keyboard, `VK_A` on Windows.
    S,
    /// A key on the main keyboard, `VK_A` on Windows.
    T,
    /// A key on the main keyboard, `VK_A` on Windows.
    U,
    /// A key on the main keyboard, `VK_A` on Windows.
    V,
    /// A key on the main keyboard, `VK_A` on Windows.
    W,
    /// A key on the main keyboard, `VK_A` on Windows.
    X,
    /// A key on the main keyboard, `VK_A` on Windows.
    Y,
    /// A key on the main keyboard, `VK_A` on Windows.
    Z,
    /// WIN key
    Platform(KeyPosition),
    /// Applications key, `VK_APPS` on Windows.
    ContextMenu,
    // /// Numeric keypad 0 key, `VK_NUMPAD0` on Windows.
    // Numpad0,
    // /// Numeric keypad 0 key, `VK_NUMPAD0` on Windows.
    // Numpad1,
    // /// Numeric keypad 0 key, `VK_NUMPAD0` on Windows.
    // Numpad2,
    // /// Numeric keypad 0 key, `VK_NUMPAD0` on Windows.
    // Numpad3,
    // /// Numeric keypad 0 key, `VK_NUMPAD0` on Windows.
    // Numpad4,
    // /// Numeric keypad 0 key, `VK_NUMPAD0` on Windows.
    // Numpad5,
    // /// Numeric keypad 0 key, `VK_NUMPAD0` on Windows.
    // Numpad6,
    // /// Numeric keypad 0 key, `VK_NUMPAD0` on Windows.
    // Numpad7,
    // /// Numeric keypad 0 key, `VK_NUMPAD0` on Windows.
    // Numpad8,
    // /// Numeric keypad 0 key, `VK_NUMPAD0` on Windows.
    // Numpad9,
    // /// Multiply key, `VK_MULTIPLY` on Windows.
    // Multiply,
    // /// Add key, `VK_ADD` on Windows.
    // Add,
    // /// Separator key, `VK_SEPARATOR` on Windows.
    // Separator,
    // /// Subtract key, `VK_SUBTRACT` on Windows.
    // Subtract,
    // /// Decimal key, `VK_DECIMAL` on Windows.
    // Decimal,
    // /// Divide key, `VK_DIVIDE` on Windows.
    // Divide,
    /// F1 key
    F1,
    /// F1 key
    F2,
    /// F1 key
    F3,
    /// F1 key
    F4,
    /// F1 key
    F5,
    /// F1 key
    F6,
    /// F1 key
    F7,
    /// F1 key
    F8,
    /// F1 key
    F9,
    /// F1 key
    F10,
    /// F1 key
    F11,
    /// F1 key
    F12,
    /// F1 key
    F13,
    /// F1 key
    F14,
    /// F1 key
    F15,
    /// F1 key
    F16,
    /// F1 key
    F17,
    /// F1 key
    F18,
    /// F1 key
    F19,
    /// F20 key
    F20,
    /// F20 key
    F21,
    /// F20 key
    F22,
    /// F20 key
    F23,
    /// F20 key
    F24,
    // /// NUM LOCK key
    // NumLock,
    // /// SCROLL LOCK key
    // ScrollLock,
    /// Browser Back key, `VK_BROWSER_BACK` on Windows.
    BrowserBack,
    /// Browser Forward key
    BrowserForward,
    /// Used for miscellaneous characters, it can vary by keyboard.
    /// For the US standard keyboard, the `;:` key
    Semicolon,
    /// For any country/region, the `+` key
    Plus,
    /// For any country/region, the `,` key
    Comma,
    /// For any country/region, the `-` key
    Minus,
    /// For any country/region, the . key
    Period,
    /// Used for miscellaneous characters, it can vary by keyboard.
    /// For the US standard keyboard, the `/?` key
    Slash,
    /// Used for miscellaneous characters, it can vary by keyboard.
    /// For the US standard keyboard, the `~ key
    Tilde,
    /// Used for miscellaneous characters, it can vary by keyboard.
    /// For the US standard keyboard, the `[{` key
    LeftBracket,
    /// Used for miscellaneous characters, it can vary by keyboard.
    /// For the US standard keyboard, the `\|` key
    Backslash,
    /// Used for miscellaneous characters, it can vary by keyboard.
    /// For the US standard keyboard, the `]}` key
    RightBracket,
    /// Used for miscellaneous characters, it can vary by keyboard.
    /// For the US standard keyboard, the `'"` key
    Quote,
    /// Used for miscellaneous characters; it can vary by keyboard.
    OEM8,
    /// The `<>` keys on the US standard keyboard, or the `\|` key on the
    /// non-US 102-key keyboard
    OEM102,
}

/// TODO:
#[derive(Copy, Clone, Debug, Default)]
pub enum KeyPosition {
    /// TODO:
    #[default]
    Any,
    /// TODO:
    Left,
    /// TODO:
    Right,
}

impl PartialEq for KeyPosition {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (KeyPosition::Right, KeyPosition::Left) | (KeyPosition::Left, KeyPosition::Right) => {
                false
            }
            _ => true,
        }
    }
}

impl Eq for KeyPosition {}

impl std::hash::Hash for KeyPosition {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            KeyPosition::Any => 0,
            KeyPosition::Left => 1,
            KeyPosition::Right => 2,
        }
        .hash(state)
    }
}

impl KeyCode {
    fn parse_immutable(input: &str) -> Option<Self> {
        Some(match input {
            "fn" => Self::Function,
            "cancel" => Self::Cancel,
            "backspace" => Self::Backspace,
            "tab" => Self::Tab,
            "enter" => Self::Enter,
            "shift" => Self::Shift(KeyPosition::Any),
            "ctrl" => Self::Control(KeyPosition::Any),
            "alt" => Self::Alt(KeyPosition::Any),
            "capslock" => Self::Capital,
            "escape" => Self::Escape,
            "space" => Self::Space,
            "pageup" => Self::PageUp,
            "pagedown" => Self::PageDown,
            "end" => Self::End,
            "home" => Self::Home,
            "left" => Self::Left,
            "up" => Self::Up,
            "right" => Self::Right,
            "down" => Self::Down,
            // VirtualKeyCode::PrintScreen => "UnImplemented",
            "insert" => Self::Insert,
            "delete" => Self::Delete,
            "win" | "cmd" | "super" => Self::Platform(KeyPosition::Any),
            "menu" => Self::ContextMenu,
            // VirtualKeyCode::Numpad0 => "UnImplemented", // TODO: Handle numpad keys
            // VirtualKeyCode::Numpad1 => "UnImplemented",
            // VirtualKeyCode::Numpad2 => "UnImplemented",
            // VirtualKeyCode::Numpad3 => "UnImplemented",
            // VirtualKeyCode::Numpad4 => "UnImplemented",
            // VirtualKeyCode::Numpad5 => "UnImplemented",
            // VirtualKeyCode::Numpad6 => "UnImplemented",
            // VirtualKeyCode::Numpad7 => "UnImplemented",
            // VirtualKeyCode::Numpad8 => "UnImplemented",
            // VirtualKeyCode::Numpad9 => "UnImplemented",
            // VirtualKeyCode::Multiply => "UnImplemented",
            // VirtualKeyCode::Add => "UnImplemented",
            // VirtualKeyCode::Separator => "UnImplemented",
            // VirtualKeyCode::Subtract => "UnImplemented",
            // VirtualKeyCode::Decimal => "UnImplemented",
            // VirtualKeyCode::Divide => "UnImplemented",
            "f1" => Self::F1,
            "f2" => Self::F2,
            "f3" => Self::F3,
            "f4" => Self::F4,
            "f5" => Self::F5,
            "f6" => Self::F6,
            "f7" => Self::F7,
            "f8" => Self::F8,
            "f9" => Self::F9,
            "f10" => Self::F10,
            "f11" => Self::F11,
            "f12" => Self::F12,
            "f13" => Self::F13,
            "f14" => Self::F14,
            "f15" => Self::F15,
            "f16" => Self::F16,
            "f17" => Self::F17,
            "f18" => Self::F18,
            "f19" => Self::F19,
            "f20" => Self::F20,
            "f21" => Self::F21,
            "f22" => Self::F22,
            "f23" => Self::F23,
            "f24" => Self::F24,
            "back" => Self::BrowserBack,
            "forward" => Self::BrowserForward,
            _ => return None,
        })
    }

    /// input is standard US English layout key
    pub fn parse(
        input: &str,
        char_matching: bool,
        keyboard_mapper: &dyn KeyboardMapper,
    ) -> anyhow::Result<(Self, Modifiers, Option<String>)> {
        if let Some(key) = Self::parse_immutable(input) {
            return Ok((key, Modifiers::none(), None));
        }
        if let Some((code, modifers)) = keyboard_mapper.parse(input, char_matching) {
            return Ok((code, modifers, keyboard_mapper.keycode_to_face(code)));
        }
        Err(anyhow::anyhow!(
            "Error parsing keystroke to keycode: {input}"
        ))
    }

    /// TODO
    pub fn parse_us_layout(
        input: &str,
        keyboard_mapper: &dyn KeyboardMapper,
    ) -> (Self, Modifiers, Option<String>) {
        let (code, modifiers) = match input {
            "0" => (Self::Digital0, Modifiers::none()),
            "1" => (Self::Digital1, Modifiers::none()),
            "2" => (Self::Digital2, Modifiers::none()),
            "3" => (Self::Digital3, Modifiers::none()),
            "4" => (Self::Digital4, Modifiers::none()),
            "5" => (Self::Digital5, Modifiers::none()),
            "6" => (Self::Digital6, Modifiers::none()),
            "7" => (Self::Digital7, Modifiers::none()),
            "8" => (Self::Digital8, Modifiers::none()),
            "9" => (Self::Digital9, Modifiers::none()),
            "!" => (Self::Digital1, Modifiers::shift()),
            "@" => (Self::Digital2, Modifiers::shift()),
            "#" => (Self::Digital3, Modifiers::shift()),
            "$" => (Self::Digital4, Modifiers::shift()),
            "%" => (Self::Digital5, Modifiers::shift()),
            "^" => (Self::Digital6, Modifiers::shift()),
            "&" => (Self::Digital7, Modifiers::shift()),
            "*" => (Self::Digital8, Modifiers::shift()),
            "(" => (Self::Digital9, Modifiers::shift()),
            ")" => (Self::Digital0, Modifiers::shift()),
            "`" => (Self::Tilde, Modifiers::none()),
            "~" => (Self::Tilde, Modifiers::shift()),
            "-" => (Self::Minus, Modifiers::none()),
            "_" => (Self::Minus, Modifiers::shift()),
            "=" => (Self::Plus, Modifiers::none()),
            "+" => (Self::Plus, Modifiers::shift()),
            "[" => (Self::LeftBracket, Modifiers::none()),
            "{" => (Self::LeftBracket, Modifiers::shift()),
            "]" => (Self::RightBracket, Modifiers::none()),
            "}" => (Self::RightBracket, Modifiers::shift()),
            "\\" => (Self::Backslash, Modifiers::none()),
            "|" => (Self::Backslash, Modifiers::shift()),
            ";" => (Self::Semicolon, Modifiers::none()),
            ":" => (Self::Semicolon, Modifiers::shift()),
            "'" => (Self::Quote, Modifiers::none()),
            "\"" => (Self::Quote, Modifiers::shift()),
            "," => (Self::Comma, Modifiers::none()),
            "<" => (Self::Comma, Modifiers::shift()),
            "." => (Self::Period, Modifiers::none()),
            ">" => (Self::Period, Modifiers::shift()),
            "/" => (Self::Slash, Modifiers::none()),
            "?" => (Self::Slash, Modifiers::shift()),
            _ => {
                log::error!("Failed to parse us-layout keystroke: {input}");
                (Self::Unknown, Modifiers::none())
            }
        };
        (code, modifiers, keyboard_mapper.keycode_to_face(code))
    }

    /// TODO:
    pub fn is_printable(&self) -> bool {
        !matches!(
            self,
            Self::F1
                | Self::F2
                | Self::F3
                | Self::F4
                | Self::F5
                | Self::F6
                | Self::F7
                | Self::F8
                | Self::F9
                | Self::F10
                | Self::F11
                | Self::F12
                | Self::F13
                | Self::F14
                | Self::F15
                | Self::F16
                | Self::F17
                | Self::F18
                | Self::F19
                | Self::F20
                | Self::F21
                | Self::F22
                | Self::F23
                | Self::F24
                | Self::Backspace
                | Self::Delete
                | Self::Left
                | Self::Up
                | Self::Right
                | Self::Down
                | Self::PageUp
                | Self::PageDown
                | Self::Insert
                | Self::Home
                | Self::End
                | Self::BrowserBack
                | Self::BrowserForward
                | Self::Escape
        )
    }
}
