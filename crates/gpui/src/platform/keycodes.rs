/// On Windows, this is the Virtual-Key Codes
/// https://docs.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes
/// On macOS and Linux, this is the Scan Codes
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum KeyCode {
    /// Un-recognized key
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
    App,
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

// static KEYBOARD_CODES: [KeyCode; 128] = [
//     KeyCode::A, // 0x00
//     KeyCode::S,
//     KeyCode::D,
//     KeyCode::F,
//     KeyCode::H,
//     KeyCode::G,
//     KeyCode::Z,
//     KeyCode::X,
//     KeyCode::C,
//     KeyCode::V,
//     KeyCode::Unknown, // Section key
//     KeyCode::B,
//     KeyCode::Q,
//     KeyCode::W,
//     KeyCode::E,
//     KeyCode::R,
//     KeyCode::Y,
//     KeyCode::T,
//     KeyCode::Digital1,
//     KeyCode::Digital2,
//     KeyCode::Digital3,
//     KeyCode::Digital4,
//     KeyCode::Digital6,
//     KeyCode::Digital5,
//     KeyCode::Plus, // =+
//     KeyCode::Digital9,
//     KeyCode::Digital7,
//     KeyCode::Minus, // -_
//     KeyCode::Digital8,
//     KeyCode::Digital0,
//     KeyCode::RightBracket, // ]}
//     KeyCode::O,
//     KeyCode::U,
//     KeyCode::LeftBracket, // [{
//     KeyCode::I,
//     KeyCode::P,
//     KeyCode::Enter,
//     KeyCode::L,
//     KeyCode::J,
//     KeyCode::Quote, // '"
//     KeyCode::K,
//     KeyCode::Semicolon, // ;:
//     KeyCode::Backslash, // \|
//     KeyCode::Comma,     // ,<
//     KeyCode::Slash,     // /?
//     KeyCode::N,
//     KeyCode::M,
//     KeyCode::Period, // .>
//     KeyCode::Tab,
//     KeyCode::Space,
//     KeyCode::Tilde, // `~
//     KeyCode::Backspace,
//     KeyCode::Unknown, // n/a
//     KeyCode::Escape,
//     KeyCode::App, // Right command
//     KeyCode::Platform(KeyPosition::Left),
//     KeyCode::Shift(KeyPosition::Left),
//     KeyCode::Capital,                     // Capslock
//     KeyCode::Alt(KeyPosition::Left),      // Left option
//     KeyCode::Control(KeyPosition::Left),  // Left control
//     KeyCode::Shift(KeyPosition::Right),   // Right shift
//     KeyCode::Alt(KeyPosition::Right),     // Right option
//     KeyCode::Control(KeyPosition::Right), // Right control
//     KeyCode::Function,                    // TODO: VK_UNKNOWN on Chrome
//     KeyCode::F17,
//     KeyCode::Decimal,  // Numpad .
//     KeyCode::Unknown,  // n/a
//     KeyCode::Multiply, // Numpad *
//     KeyCode::Unknown,  // n/a
//     KeyCode::Add,      // Numpad +
//     KeyCode::Unknown,  // n/a
//     KeyCode::Clear,    // Numpad clear
//     KeyCode::VolumeUp,
//     KeyCode::VolumeDown,
//     KeyCode::VolumeMute,
//     KeyCode::Divide,   // Numpad /
//     KeyCode::Enter,    // Numpad enter
//     KeyCode::Unknown,  // n/a
//     KeyCode::Subtract, // Numpad -
//     KeyCode::F18,
//     KeyCode::F19,
//     KeyCode::Plus, // Numpad =.
//     KeyCode::Numpad0,
//     KeyCode::Numpad1,
//     KeyCode::Numpad2,
//     KeyCode::Numpad3,
//     KeyCode::Numpad4,
//     KeyCode::Numpad5,
//     KeyCode::Numpad6,
//     KeyCode::Numpad7,
//     KeyCode::F20,
//     KeyCode::Numpad8,
//     KeyCode::Numpad9,
//     KeyCode::Unknown, // Yen, JIS keyboad only
//     KeyCode::Unknown, // Underscore, JIS keyboard only
//     KeyCode::Unknown, // Keypad comma, JIS keyboard only
//     KeyCode::F5,
//     KeyCode::F6,
//     KeyCode::F7,
//     KeyCode::F3,
//     KeyCode::F8,
//     KeyCode::F9,
//     KeyCode::Unknown, // Eisu, JIS keyboard only
//     KeyCode::F11,
//     KeyCode::Unknown, // Kana, JIS keyboard only
//     KeyCode::F13,
//     KeyCode::F16,
//     KeyCode::F14,
//     KeyCode::Unknown, // n/a
//     KeyCode::F10,
//     KeyCode::App, // Context menu key
//     KeyCode::F12,
//     KeyCode::Unknown, // n/a
//     KeyCode::F15,
//     KeyCode::Insert, // Help
//     KeyCode::Home,   // Home
//     KeyCode::PageUp,
//     KeyCode::Delete, // Forward delete
//     KeyCode::F4,
//     KeyCode::End,
//     KeyCode::F2,
//     KeyCode::PageDown,
//     KeyCode::F1,
//     KeyCode::Left,
//     KeyCode::Right,
//     KeyCode::Down,
//     KeyCode::Up,
//     KeyCode::Unknown, // n/a
// ];
