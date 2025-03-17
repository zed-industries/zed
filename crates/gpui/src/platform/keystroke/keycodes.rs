use anyhow::Context;
use util::ResultExt;
use windows::Win32::UI::Input::KeyboardAndMouse::*;

use crate::Modifiers;

/// TODO:
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum KeyCodes {
    /// Un-recognized key
    Unknown(String),
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
    // /// IME Kana mode, `VK_KANA` on Windows.
    // Kana,
    // /// IME Hangul mode, `VK_HANGUL` on Windows.
    // Hangul,
    // ///IME Junja mode, `VK_JUNJA` on Windows.
    // Junja,
    // /// IME final mode, `VK_FINAL` on Windows.
    // Final,
    // /// IME Hanja mode, `VK_HANJA` on Windows.
    // Hanja,
    // /// IME Kanji mode, `VK_KANJI` on Winodws.
    // Kanji,
    /// ESC key, `VK_ESCAPE` on Windows.
    Escape,
    /// IME convert, `VK_CONVERT` on Windows.
    Convert,
    /// IME nonconvert, `VK_NONCONVERT` on Windows.
    Nonconvert,
    /// IME accept, `VK_ACCEPT` on Windows.
    Accept,
    /// IME mode change request, `VK_MODECHANGE` on Windows.
    ModeChange,
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
    /// EXECUTE key, `VK_EXECUTE` on Windows.
    Execute,
    /// PRINT SCREEN key, `VK_SNAPSHOT` on Windows.
    PrintScreen,
    /// INS key, `VK_INSERT` on Windows.
    Insert,
    /// DEL key, `VK_DELETE` on Windows.
    Delete,
    /// HELP key, `VK_HELP` on Windows.
    Help,
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
    /// Computer Sleep key, `VK_SLEEP` on Windows.
    Sleep,
    /// Numeric keypad 0 key, `VK_NUMPAD0` on Windows.
    Numpad0,
    /// Numeric keypad 0 key, `VK_NUMPAD0` on Windows.
    Numpad1,
    /// Numeric keypad 0 key, `VK_NUMPAD0` on Windows.
    Numpad2,
    /// Numeric keypad 0 key, `VK_NUMPAD0` on Windows.
    Numpad3,
    /// Numeric keypad 0 key, `VK_NUMPAD0` on Windows.
    Numpad4,
    /// Numeric keypad 0 key, `VK_NUMPAD0` on Windows.
    Numpad5,
    /// Numeric keypad 0 key, `VK_NUMPAD0` on Windows.
    Numpad6,
    /// Numeric keypad 0 key, `VK_NUMPAD0` on Windows.
    Numpad7,
    /// Numeric keypad 0 key, `VK_NUMPAD0` on Windows.
    Numpad8,
    /// Numeric keypad 0 key, `VK_NUMPAD0` on Windows.
    Numpad9,
    /// Multiply key, `VK_MULTIPLY` on Windows.
    Multiply,
    /// Add key, `VK_ADD` on Windows.
    Add,
    /// Separator key, `VK_SEPARATOR` on Windows.
    Separator,
    /// Subtract key, `VK_SUBTRACT` on Windows.
    Subtract,
    /// Decimal key, `VK_DECIMAL` on Windows.
    Decimal,
    /// Divide key, `VK_DIVIDE` on Windows.
    Divide,
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
    /// NUM LOCK key
    NumLock,
    /// SCROLL LOCK key
    ScrollLock,
    /// Browser Back key, `VK_BROWSER_BACK` on Windows.
    BrowserBack,
    /// Browser Forward key
    BrowserForward,
    /// Browser Refresh key
    BrowserRefresh,
    /// Browser Stop key
    BrowserStop,
    /// Browser Search key
    BrowserSearch,
    /// Browser Favorites key
    BrowserFavorites,
    /// Browser Start and Home key
    BrowserHome,
    /// Volume Mute key
    VolumeMute,
    /// Volume Down key
    VolumeDown,
    /// Volume Up key
    VolumeUp,
    /// Next Track key
    MediaNextTrack,
    /// Previous Track key
    MediaPrevTrack,
    /// Stop Track key
    MediaStop,
    /// Play/Pause Media key
    MediaPlayPause,
    /// Start Mail key
    LaunchMail,
    /// Select Media key
    LaunchMediaSelect,
    /// Start Application 1 key
    LaunchApp1,
    /// Start Application 2 key
    LaunchApp2,
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
    // /// IME PROCESS key
    // ProcessKey,
    // /// Used to pass Unicode characters as if they were keystrokes.
    // /// The `VK_PACKET` on Windows, this key is the low word of a 32-bit Virtual Key
    // /// value used for non-keyboard input methods.
    // ///
    // /// For more information, see Remark in KEYBDINPUT, SendInput, WM_KEYDOWN, and WM_KEYUP
    // Packet,
    // TODO: These keys not presented on Windows doc, but on Chrome.
    // VKEY_OEM_ATTN = VK_OEM_ATTN,
    // VKEY_OEM_FINISH = VK_OEM_FINISH,
    // VKEY_OEM_COPY = VK_OEM_COPY,
    // VKEY_DBE_SBCSCHAR = VK_DBE_SBCSCHAR,
    // VKEY_DBE_DBCSCHAR = VK_DBE_DBCSCHAR,
    // VKEY_OEM_BACKTAB = VK_OEM_BACKTAB,
    // /// Attn key
    // Attn,
    // /// CrSel key
    // CrSel,
    // /// ExSel key
    // ExSel,
    // /// Erase EOF key
    // EraseEOF,
    // /// Play key
    // Play,
    // /// Zoom key
    // Zoom,
    // // TODO: These keys are reserved by Windows but are used by Chrome, `VK_NONAME`
    // // NoName
    // // Paste
    // /// PA1 key
    // PA1,
    // /// Clear key
    // OEMClear,
}

impl Default for KeyCodes {
    fn default() -> Self {
        Self::Unknown("".to_string())
    }
}

/// TODO:
#[derive(Copy, Clone, Debug, Default, Hash)]
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

impl KeyCodes {
    fn basic_parse(input: &str) -> Option<Self> {
        Some(match input {
            "fn" => Self::Function,
            "cancel" => Self::Cancel,
            "backspace" => Self::Backspace,
            "tab" => Self::Tab,
            // VirtualKeyCode::Clear => "UnImplemented",
            "enter" => Self::Enter,
            "shift" => Self::Shift(KeyPosition::Any),
            "ctrl" => Self::Control(KeyPosition::Any),
            "alt" => Self::Alt(KeyPosition::Any),
            // VirtualKeyCode::Pause => "UnImplemented",
            "capslock" => Self::Capital,
            // VirtualKeyCode::Kana => "UnImplemented",
            // VirtualKeyCode::Hangul => "UnImplemented",
            // VirtualKeyCode::Junja => "UnImplemented",
            // VirtualKeyCode::Final => "UnImplemented",
            // VirtualKeyCode::Hanja => "UnImplemented",
            // VirtualKeyCode::Kanji => "UnImplemented",
            "escape" => KeyCodes::Escape,
            // VirtualKeyCode::Convert => "UnImplemented",
            // VirtualKeyCode::Nonconvert => "UnImplemented",
            // VirtualKeyCode::Accept => "UnImplemented",
            // VirtualKeyCode::ModeChange => "UnImplemented",
            "space" => KeyCodes::Space, // TODO:
            "pageup" => KeyCodes::PageUp,
            "pagedown" => KeyCodes::PageDown,
            "end" => KeyCodes::End,
            "home" => KeyCodes::Home,
            "left" => KeyCodes::Left,
            "up" => KeyCodes::Up,
            "right" => KeyCodes::Right,
            "down" => KeyCodes::Down,
            // VirtualKeyCode::Select => "UnImplemented",
            // VirtualKeyCode::Print => "UnImplemented",
            // VirtualKeyCode::Execute => "UnImplemented",
            // VirtualKeyCode::PrintScreen => "UnImplemented",
            "insert" => KeyCodes::Insert,
            "delete" => KeyCodes::Delete,
            // VirtualKeyCode::Help => "UnImplemented",
            "win" | "cmd" | "super" => Self::Platform(KeyPosition::Any),
            "menu" => KeyCodes::App, // TODO: Chrome use this as Fn key
            // VirtualKeyCode::Sleep => "UnImplemented",
            "a" => KeyCodes::A,
            "b" => KeyCodes::B,
            "c" => KeyCodes::C,
            "d" => KeyCodes::D,
            "e" => KeyCodes::E,
            "f" => KeyCodes::F,
            "g" => KeyCodes::G,
            "h" => KeyCodes::H,
            "i" => KeyCodes::I,
            "j" => KeyCodes::J,
            "k" => KeyCodes::K,
            "l" => KeyCodes::L,
            "m" => KeyCodes::M,
            "n" => KeyCodes::N,
            "o" => KeyCodes::O,
            "p" => KeyCodes::P,
            "q" => KeyCodes::Q,
            "r" => KeyCodes::R,
            "s" => KeyCodes::S,
            "t" => KeyCodes::T,
            "u" => KeyCodes::U,
            "v" => KeyCodes::V,
            "w" => KeyCodes::W,
            "x" => KeyCodes::X,
            "y" => KeyCodes::Y,
            "z" => KeyCodes::Z,
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
            "f1" => KeyCodes::F1,
            "f2" => KeyCodes::F2,
            "f3" => KeyCodes::F3,
            "f4" => KeyCodes::F4,
            "f5" => KeyCodes::F5,
            "f6" => KeyCodes::F6,
            "f7" => KeyCodes::F7,
            "f8" => KeyCodes::F8,
            "f9" => KeyCodes::F9,
            "f10" => KeyCodes::F10,
            "f11" => KeyCodes::F11,
            "f12" => KeyCodes::F12,
            "f13" => KeyCodes::F13,
            "f14" => KeyCodes::F14,
            "f15" => KeyCodes::F15,
            "f16" => KeyCodes::F16,
            "f17" => KeyCodes::F17,
            "f18" => KeyCodes::F18,
            "f19" => KeyCodes::F19,
            "f20" => KeyCodes::F20,
            "f21" => KeyCodes::F21,
            "f22" => KeyCodes::F22,
            "f23" => KeyCodes::F23,
            "f24" => KeyCodes::F24,
            // VirtualKeyCode::NumLock => "UnImplemented",
            // VirtualKeyCode::ScrollLock => "UnImplemented",
            // VirtualKeyCode::LeftShift => "shift", // TODO:
            // VirtualKeyCode::RightShift => "shift", // TODO:
            // VirtualKeyCode::LeftControl => "control", // TODO:
            // VirtualKeyCode::RightControl => "control", // TODO:
            // VirtualKeyCode::LeftAlt => "alt", // TODO:
            // VirtualKeyCode::RightAlt => "alt", // TODO:
            "back" => KeyCodes::BrowserBack,
            "forward" => KeyCodes::BrowserForward,
            // VirtualKeyCode::BrowserRefresh => "UnImplemented",
            // VirtualKeyCode::BrowserStop => "UnImplemented",
            // VirtualKeyCode::BrowserSearch => "UnImplemented",
            // VirtualKeyCode::BrowserFavorites => "UnImplemented",
            // VirtualKeyCode::BrowserHome => "UnImplemented",
            // VirtualKeyCode::VolumeMute => "UnImplemented",
            // VirtualKeyCode::VolumeDown => "UnImplemented",
            // VirtualKeyCode::VolumeUp => "UnImplemented",
            // VirtualKeyCode::MediaNextTrack => "UnImplemented",
            // VirtualKeyCode::MediaPrevTrack => "UnImplemented",
            // VirtualKeyCode::MediaStop => "UnImplemented",
            // VirtualKeyCode::MediaPlayPause => "UnImplemented",
            // VirtualKeyCode::LaunchMail => "UnImplemented",
            // VirtualKeyCode::LaunchMediaSelect => "UnImplemented",
            // VirtualKeyCode::LaunchApp1 => "UnImplemented",
            // VirtualKeyCode::LaunchApp2 => "UnImplemented",
            // VirtualKeyCode::OEM8 => "UnImplemented",
            // VirtualKeyCode::OEM102 => "UnImplemented",
            // VirtualKeyCode::ProcessKey => "UnImplemented",
            // VirtualKeyCode::Packet => "UnImplemented",
            // VirtualKeyCode::Attn => "UnImplemented",
            // VirtualKeyCode::CrSel => "UnImplemented",
            // VirtualKeyCode::ExSel => "UnImplemented",
            // VirtualKeyCode::EraseEOF => "UnImplemented",
            // VirtualKeyCode::Play => "UnImplemented",
            // VirtualKeyCode::Zoom => "UnImplemented",
            // VirtualKeyCode::PA1 => "UnImplemented",
            // VirtualKeyCode::OEMClear => "UnImplemented",
            _ => return None,
        })
    }
    /// input is standard US English layout key
    fn parse(input: &str) -> anyhow::Result<(Self, bool)> {
        if let Some(key) = Self::basic_parse(input) {
            return Ok((key, false));
        }
        match input {
            "0" => Ok((KeyCodes::Digital0, false)),
            "1" => Ok((KeyCodes::Digital1, false)),
            "2" => Ok((KeyCodes::Digital2, false)),
            "3" => Ok((KeyCodes::Digital3, false)),
            "4" => Ok((KeyCodes::Digital4, false)),
            "5" => Ok((KeyCodes::Digital5, false)),
            "6" => Ok((KeyCodes::Digital6, false)),
            "7" => Ok((KeyCodes::Digital7, false)),
            "8" => Ok((KeyCodes::Digital8, false)),
            "9" => Ok((KeyCodes::Digital9, false)),
            ";" => Ok((KeyCodes::Semicolon, false)),
            "=" => Ok((KeyCodes::Plus, false)),
            "," => Ok((KeyCodes::Comma, false)),
            "-" => Ok((KeyCodes::Minus, false)),
            "." => Ok((KeyCodes::Period, false)),
            "/" => Ok((KeyCodes::Slash, false)),
            "`" => Ok((KeyCodes::Tilde, false)),
            "[" => Ok((KeyCodes::LeftBracket, false)),
            "\\" => Ok((KeyCodes::Backslash, false)),
            "]" => Ok((KeyCodes::RightBracket, false)),
            "'" => Ok((KeyCodes::Quote, false)),
            "~" => Ok((KeyCodes::Tilde, true)),
            "!" => Ok((KeyCodes::Digital1, true)),
            "@" => Ok((KeyCodes::Digital2, true)),
            "#" => Ok((KeyCodes::Digital3, true)),
            "$" => Ok((KeyCodes::Digital4, true)),
            "%" => Ok((KeyCodes::Digital5, true)),
            "^" => Ok((KeyCodes::Digital6, true)),
            "&" => Ok((KeyCodes::Digital7, true)),
            "*" => Ok((KeyCodes::Digital8, true)),
            "(" => Ok((KeyCodes::Digital9, true)),
            ")" => Ok((KeyCodes::Digital0, true)),
            "_" => Ok((KeyCodes::Minus, true)),
            "+" => Ok((KeyCodes::Plus, true)),
            "{" => Ok((KeyCodes::LeftBracket, true)),
            "}" => Ok((KeyCodes::RightBracket, true)),
            "|" => Ok((KeyCodes::Backslash, true)),
            ":" => Ok((KeyCodes::Semicolon, true)),
            "\"" => Ok((KeyCodes::Quote, true)),
            "<" => Ok((KeyCodes::Comma, true)),
            ">" => Ok((KeyCodes::Period, true)),
            "?" => Ok((KeyCodes::Slash, true)),
            _ => Err(anyhow::anyhow!(
                "Error parsing keystroke to virtual keycode: {input}"
            )),
        }
    }

    /// TODO:
    fn parse_char(input: &str) -> anyhow::Result<(Self, bool, bool, bool)> {
        if let Some(key) = Self::basic_parse(input) {
            return Ok((key, false, false, false));
        }
        if input.chars().count() != 1 {
            return Err(anyhow::anyhow!(
                "Error parsing keystroke to virtual keycode (char based): {input}"
            ));
        }
        let ch = input.chars().next().unwrap();
        let result = unsafe { VkKeyScanW(ch as u16) };
        if result == -1 {
            return Err(anyhow::anyhow!(
                "Error parsing keystroke to virtual keycode (char based): {input}"
            ));
        }
        let high = (result >> 8) as u8;
        let low = result as u8;
        let shift = high & 1;
        let ctrl = high & 2;
        let alt = high & 8;
        let this = VIRTUAL_KEY(low as u16).try_into()?;
        Ok((this, shift != 0, ctrl != 0, alt != 0))
    }

    /// TODO:
    pub fn unparse(&self) -> &str {
        match self {
            KeyCodes::Unknown(content) => &content,
            KeyCodes::Function => "fn",
            KeyCodes::Cancel => "cancel",
            KeyCodes::Backspace => "backspace",
            KeyCodes::Tab => "tab",
            KeyCodes::Clear => "UnImplemented",
            KeyCodes::Enter => "enter",
            // TODO: position
            KeyCodes::Shift(_) => "shift",
            KeyCodes::Control(_) => "ctrl",
            KeyCodes::Alt(_) => "alt",
            KeyCodes::Pause => "UnImplemented",
            KeyCodes::Capital => "capslock",
            // KeyCodes::Kana => "UnImplemented",
            // KeyCodes::Hangul => "UnImplemented",
            // KeyCodes::Junja => "UnImplemented",
            // KeyCodes::Final => "UnImplemented",
            // KeyCodes::Hanja => "UnImplemented",
            // KeyCodes::Kanji => "UnImplemented",
            KeyCodes::Escape => "escape",
            KeyCodes::Convert => "UnImplemented",
            KeyCodes::Nonconvert => "UnImplemented",
            KeyCodes::Accept => "UnImplemented",
            KeyCodes::ModeChange => "UnImplemented",
            KeyCodes::Space => "space",
            KeyCodes::PageUp => "pageup",
            KeyCodes::PageDown => "pagedown",
            KeyCodes::End => "end",
            KeyCodes::Home => "home",
            KeyCodes::Left => "left",
            KeyCodes::Up => "up",
            KeyCodes::Right => "right",
            KeyCodes::Down => "down",
            KeyCodes::Select => "UnImplemented",
            KeyCodes::Print => "UnImplemented",
            KeyCodes::Execute => "UnImplemented",
            KeyCodes::PrintScreen => "UnImplemented",
            KeyCodes::Insert => "insert",
            KeyCodes::Delete => "delete",
            KeyCodes::Help => "UnImplemented",
            KeyCodes::Digital0 => "0",
            KeyCodes::Digital1 => "1",
            KeyCodes::Digital2 => "2",
            KeyCodes::Digital3 => "3",
            KeyCodes::Digital4 => "4",
            KeyCodes::Digital5 => "5",
            KeyCodes::Digital6 => "6",
            KeyCodes::Digital7 => "7",
            KeyCodes::Digital8 => "8",
            KeyCodes::Digital9 => "9",
            KeyCodes::A => "a",
            KeyCodes::B => "b",
            KeyCodes::C => "c",
            KeyCodes::D => "d",
            KeyCodes::E => "e",
            KeyCodes::F => "f",
            KeyCodes::G => "g",
            KeyCodes::H => "h",
            KeyCodes::I => "i",
            KeyCodes::J => "j",
            KeyCodes::K => "k",
            KeyCodes::L => "l",
            KeyCodes::M => "m",
            KeyCodes::N => "n",
            KeyCodes::O => "o",
            KeyCodes::P => "p",
            KeyCodes::Q => "q",
            KeyCodes::R => "r",
            KeyCodes::S => "s",
            KeyCodes::T => "t",
            KeyCodes::U => "u",
            KeyCodes::V => "v",
            KeyCodes::W => "w",
            KeyCodes::X => "x",
            KeyCodes::Y => "y",
            KeyCodes::Z => "z",
            // TODO: handle position
            KeyCodes::Platform(_) => "win",
            KeyCodes::App => "menu", // TODO: Chrome use this as Fn key
            KeyCodes::Sleep => "UnImplemented",
            KeyCodes::Numpad0 => "UnImplemented", // TODO: handle numpad key
            KeyCodes::Numpad1 => "UnImplemented",
            KeyCodes::Numpad2 => "UnImplemented",
            KeyCodes::Numpad3 => "UnImplemented",
            KeyCodes::Numpad4 => "UnImplemented",
            KeyCodes::Numpad5 => "UnImplemented",
            KeyCodes::Numpad6 => "UnImplemented",
            KeyCodes::Numpad7 => "UnImplemented",
            KeyCodes::Numpad8 => "UnImplemented",
            KeyCodes::Numpad9 => "UnImplemented",
            KeyCodes::Multiply => "UnImplemented",
            KeyCodes::Add => "UnImplemented",
            KeyCodes::Separator => "UnImplemented",
            KeyCodes::Subtract => "UnImplemented",
            KeyCodes::Decimal => "UnImplemented",
            KeyCodes::Divide => "UnImplemented",
            KeyCodes::F1 => "f1",
            KeyCodes::F2 => "f2",
            KeyCodes::F3 => "f3",
            KeyCodes::F4 => "f4",
            KeyCodes::F5 => "f5",
            KeyCodes::F6 => "f6",
            KeyCodes::F7 => "f7",
            KeyCodes::F8 => "f8",
            KeyCodes::F9 => "f9",
            KeyCodes::F10 => "f10",
            KeyCodes::F11 => "f11",
            KeyCodes::F12 => "f12",
            KeyCodes::F13 => "f13",
            KeyCodes::F14 => "f14",
            KeyCodes::F15 => "f15",
            KeyCodes::F16 => "f16",
            KeyCodes::F17 => "f17",
            KeyCodes::F18 => "f18",
            KeyCodes::F19 => "f19",
            KeyCodes::F20 => "f20",
            KeyCodes::F21 => "f21",
            KeyCodes::F22 => "f22",
            KeyCodes::F23 => "f23",
            KeyCodes::F24 => "f24",
            KeyCodes::NumLock => "UnImplemented",
            KeyCodes::ScrollLock => "UnImplemented",
            KeyCodes::BrowserBack => "back",
            KeyCodes::BrowserForward => "forward",
            KeyCodes::BrowserRefresh => "UnImplemented",
            KeyCodes::BrowserStop => "UnImplemented",
            KeyCodes::BrowserSearch => "UnImplemented",
            KeyCodes::BrowserFavorites => "UnImplemented",
            KeyCodes::BrowserHome => "UnImplemented",
            KeyCodes::VolumeMute => "UnImplemented",
            KeyCodes::VolumeDown => "UnImplemented",
            KeyCodes::VolumeUp => "UnImplemented",
            KeyCodes::MediaNextTrack => "UnImplemented",
            KeyCodes::MediaPrevTrack => "UnImplemented",
            KeyCodes::MediaStop => "UnImplemented",
            KeyCodes::MediaPlayPause => "UnImplemented",
            KeyCodes::LaunchMail => "UnImplemented",
            KeyCodes::LaunchMediaSelect => "UnImplemented",
            KeyCodes::LaunchApp1 => "UnImplemented",
            KeyCodes::LaunchApp2 => "UnImplemented",
            KeyCodes::Semicolon => ";",
            KeyCodes::Plus => "=",
            KeyCodes::Comma => ",",
            KeyCodes::Minus => "-",
            KeyCodes::Period => ".",
            KeyCodes::Slash => "/",
            KeyCodes::Tilde => "`",
            KeyCodes::LeftBracket => "[",
            KeyCodes::Backslash => "\\",
            KeyCodes::RightBracket => "]",
            KeyCodes::Quote => "'",
            KeyCodes::OEM8 => "UnImplemented",
            KeyCodes::OEM102 => "UnImplemented",
            // KeyCodes::ProcessKey => "UnImplemented",
            // KeyCodes::Packet => "UnImplemented",
            // KeyCodes::Attn => "UnImplemented",
            // KeyCodes::CrSel => "UnImplemented",
            // KeyCodes::ExSel => "UnImplemented",
            // KeyCodes::EraseEOF => "UnImplemented",
            // KeyCodes::Play => "UnImplemented",
            // KeyCodes::Zoom => "UnImplemented",
            // KeyCodes::PA1 => "UnImplemented",
            // KeyCodes::OEMClear => "UnImplemented",
        }
    }

    /// TODO:
    pub fn to_output_string(&self, shift: bool) -> String {
        if shift {
            match self {
                KeyCodes::Semicolon => ":".to_string(),
                KeyCodes::Plus => "+".to_string(),
                KeyCodes::Comma => "<".to_string(),
                KeyCodes::Minus => "_".to_string(),
                KeyCodes::Period => ">".to_string(),
                KeyCodes::Slash => "?".to_string(),
                KeyCodes::Tilde => "~".to_string(),
                KeyCodes::LeftBracket => "{".to_string(),
                KeyCodes::Backslash => "|".to_string(),
                KeyCodes::RightBracket => "}".to_string(),
                KeyCodes::Quote => "\"".to_string(),
                KeyCodes::Digital0 => ")".to_string(),
                KeyCodes::Digital1 => "!".to_string(),
                KeyCodes::Digital2 => "@".to_string(),
                KeyCodes::Digital3 => "#".to_string(),
                KeyCodes::Digital4 => "$".to_string(),
                KeyCodes::Digital5 => "%".to_string(),
                KeyCodes::Digital6 => "^".to_string(),
                KeyCodes::Digital7 => "&".to_string(),
                KeyCodes::Digital8 => "*".to_string(),
                KeyCodes::Digital9 => "(".to_string(),
                _ => self.unparse().to_uppercase(),
            }
        } else {
            match self {
                KeyCodes::Semicolon => ";",
                KeyCodes::Plus => "=",
                KeyCodes::Comma => ",",
                KeyCodes::Minus => "-",
                KeyCodes::Period => ".",
                KeyCodes::Slash => "/",
                KeyCodes::Tilde => "`",
                KeyCodes::LeftBracket => "[",
                KeyCodes::Backslash => "\\",
                KeyCodes::RightBracket => "]",
                KeyCodes::Quote => "'",
                KeyCodes::Digital0 => "0",
                KeyCodes::Digital1 => "1",
                KeyCodes::Digital2 => "2",
                KeyCodes::Digital3 => "3",
                KeyCodes::Digital4 => "4",
                KeyCodes::Digital5 => "5",
                KeyCodes::Digital6 => "6",
                KeyCodes::Digital7 => "7",
                KeyCodes::Digital8 => "8",
                KeyCodes::Digital9 => "9",
                _ => self.unparse(),
            }
            .to_string()
        }
    }
}

impl TryFrom<VIRTUAL_KEY> for KeyCodes {
    type Error = anyhow::Error;

    fn try_from(value: VIRTUAL_KEY) -> Result<Self, Self::Error> {
        Ok(match value {
            // VirtualKeyCode::Unknown => todo!(),
            // VirtualKeyCode::Function => todo!(),
            VK_CANCEL => KeyCodes::Cancel,
            VK_BACK => KeyCodes::Backspace,
            VK_TAB => KeyCodes::Tab,
            VK_CLEAR => KeyCodes::Clear,
            VK_RETURN => KeyCodes::Enter,
            VK_SHIFT => KeyCodes::Shift(KeyPosition::Any),
            VK_CONTROL => KeyCodes::Control(KeyPosition::Any),
            VK_MENU => KeyCodes::Alt(KeyPosition::Any),
            VK_PAUSE => KeyCodes::Pause,
            VK_CAPITAL => KeyCodes::Capital,
            // VK_KANA => KeyCodes::Kana,
            // VK_HANGUL => VirtualKeyCode::Hangul,
            // VK_JUNJA => KeyCodes::Junja,
            // VK_FINAL => KeyCodes::Final,
            // VK_HANJA => KeyCodes::Hanja,
            // VK_KANJI => VirtualKeyCode::Kanji,
            VK_ESCAPE => KeyCodes::Escape,
            VK_CONVERT => KeyCodes::Convert,
            VK_NONCONVERT => KeyCodes::Nonconvert,
            VK_ACCEPT => KeyCodes::Accept,
            VK_MODECHANGE => KeyCodes::ModeChange,
            VK_SPACE => KeyCodes::Space,
            VK_PRIOR => KeyCodes::PageUp,
            VK_NEXT => KeyCodes::PageDown,
            VK_END => KeyCodes::End,
            VK_HOME => KeyCodes::Home,
            VK_LEFT => KeyCodes::Left,
            VK_UP => KeyCodes::Up,
            VK_RIGHT => KeyCodes::Right,
            VK_DOWN => KeyCodes::Down,
            VK_SELECT => KeyCodes::Select,
            VK_PRINT => KeyCodes::Print,
            VK_EXECUTE => KeyCodes::Execute,
            VK_SNAPSHOT => KeyCodes::PrintScreen,
            VK_INSERT => KeyCodes::Insert,
            VK_DELETE => KeyCodes::Delete,
            VK_HELP => KeyCodes::Help,
            VK_0 => KeyCodes::Digital0,
            VK_1 => KeyCodes::Digital1,
            VK_2 => KeyCodes::Digital2,
            VK_3 => KeyCodes::Digital3,
            VK_4 => KeyCodes::Digital4,
            VK_5 => KeyCodes::Digital5,
            VK_6 => KeyCodes::Digital6,
            VK_7 => KeyCodes::Digital7,
            VK_8 => KeyCodes::Digital8,
            VK_9 => KeyCodes::Digital9,
            VK_A => KeyCodes::A,
            VK_B => KeyCodes::B,
            VK_C => KeyCodes::C,
            VK_D => KeyCodes::D,
            VK_E => KeyCodes::E,
            VIRTUAL_KEY(70u16) => KeyCodes::F,
            VK_G => KeyCodes::G,
            VK_H => KeyCodes::H,
            VK_I => KeyCodes::I,
            VK_J => KeyCodes::J,
            VK_K => KeyCodes::K,
            VK_L => KeyCodes::L,
            VK_M => KeyCodes::M,
            VK_N => KeyCodes::N,
            VK_O => KeyCodes::O,
            VK_P => KeyCodes::P,
            VK_Q => KeyCodes::Q,
            VK_R => KeyCodes::R,
            VK_S => KeyCodes::S,
            VK_T => KeyCodes::T,
            VK_U => KeyCodes::U,
            VK_V => KeyCodes::V,
            VK_W => KeyCodes::W,
            VK_X => KeyCodes::X,
            VK_Y => KeyCodes::Y,
            VK_Z => KeyCodes::Z,
            VK_LWIN => KeyCodes::Platform(KeyPosition::Left),
            VK_RWIN => KeyCodes::Platform(KeyPosition::Right),
            VK_APPS => KeyCodes::App,
            VK_SLEEP => KeyCodes::Sleep,
            VK_NUMPAD0 => KeyCodes::Numpad0,
            VK_NUMPAD1 => KeyCodes::Numpad1,
            VK_NUMPAD2 => KeyCodes::Numpad2,
            VK_NUMPAD3 => KeyCodes::Numpad3,
            VK_NUMPAD4 => KeyCodes::Numpad4,
            VK_NUMPAD5 => KeyCodes::Numpad5,
            VK_NUMPAD6 => KeyCodes::Numpad6,
            VK_NUMPAD7 => KeyCodes::Numpad7,
            VK_NUMPAD8 => KeyCodes::Numpad8,
            VK_NUMPAD9 => KeyCodes::Numpad9,
            VK_MULTIPLY => KeyCodes::Multiply,
            VK_ADD => KeyCodes::Add,
            VK_SEPARATOR => KeyCodes::Separator,
            VK_SUBTRACT => KeyCodes::Subtract,
            VK_DECIMAL => KeyCodes::Decimal,
            VK_DIVIDE => KeyCodes::Divide,
            VK_F1 => KeyCodes::F1,
            VK_F2 => KeyCodes::F2,
            VK_F3 => KeyCodes::F3,
            VK_F4 => KeyCodes::F4,
            VK_F5 => KeyCodes::F5,
            VK_F6 => KeyCodes::F6,
            VK_F7 => KeyCodes::F7,
            VK_F8 => KeyCodes::F8,
            VK_F9 => KeyCodes::F9,
            VK_F10 => KeyCodes::F10,
            VK_F11 => KeyCodes::F11,
            VK_F12 => KeyCodes::F12,
            VK_F13 => KeyCodes::F13,
            VK_F14 => KeyCodes::F14,
            VK_F15 => KeyCodes::F15,
            VK_F16 => KeyCodes::F16,
            VK_F17 => KeyCodes::F17,
            VK_F18 => KeyCodes::F18,
            VK_F19 => KeyCodes::F19,
            VK_F20 => KeyCodes::F20,
            VK_F21 => KeyCodes::F21,
            VK_F22 => KeyCodes::F22,
            VK_F23 => KeyCodes::F23,
            VK_F24 => KeyCodes::F24,
            VK_NUMLOCK => KeyCodes::NumLock,
            VK_SCROLL => KeyCodes::ScrollLock,
            VK_LSHIFT => KeyCodes::Shift(KeyPosition::Left),
            VK_RSHIFT => KeyCodes::Shift(KeyPosition::Right),
            VK_LCONTROL => KeyCodes::Control(KeyPosition::Left),
            VK_RCONTROL => KeyCodes::Control(KeyPosition::Right),
            VK_LMENU => KeyCodes::Alt(KeyPosition::Left),
            VK_RMENU => KeyCodes::Alt(KeyPosition::Right),
            VK_BROWSER_BACK => KeyCodes::BrowserBack,
            VK_BROWSER_FORWARD => KeyCodes::BrowserForward,
            VK_BROWSER_REFRESH => KeyCodes::BrowserRefresh,
            VK_BROWSER_STOP => KeyCodes::BrowserStop,
            VK_BROWSER_SEARCH => KeyCodes::BrowserSearch,
            VK_BROWSER_FAVORITES => KeyCodes::BrowserFavorites,
            VK_BROWSER_HOME => KeyCodes::BrowserHome,
            VK_VOLUME_MUTE => KeyCodes::VolumeMute,
            VK_VOLUME_DOWN => KeyCodes::VolumeDown,
            VK_VOLUME_UP => KeyCodes::VolumeUp,
            VK_MEDIA_NEXT_TRACK => KeyCodes::MediaNextTrack,
            VK_MEDIA_PREV_TRACK => KeyCodes::MediaPrevTrack,
            VK_MEDIA_STOP => KeyCodes::MediaStop,
            VK_MEDIA_PLAY_PAUSE => KeyCodes::MediaPlayPause,
            VK_LAUNCH_MAIL => KeyCodes::LaunchMail,
            VK_LAUNCH_MEDIA_SELECT => KeyCodes::LaunchMediaSelect,
            VK_LAUNCH_APP1 => KeyCodes::LaunchApp1,
            VK_LAUNCH_APP2 => KeyCodes::LaunchApp2,
            VK_OEM_1 => KeyCodes::Semicolon,
            VK_OEM_PLUS => KeyCodes::Plus,
            VK_OEM_COMMA => KeyCodes::Comma,
            VK_OEM_MINUS => KeyCodes::Minus,
            VK_OEM_PERIOD => KeyCodes::Period,
            VK_OEM_2 => KeyCodes::Slash,
            VK_OEM_3 => KeyCodes::Tilde,
            VK_OEM_4 => KeyCodes::LeftBracket,
            VK_OEM_5 => KeyCodes::Backslash,
            VK_OEM_6 => KeyCodes::RightBracket,
            VK_OEM_7 => KeyCodes::Quote,
            VK_OEM_8 => KeyCodes::OEM8,
            VK_OEM_102 => KeyCodes::OEM102,
            // VK_PROCESSKEY => KeyCodes::ProcessKey,
            // VK_PACKET => KeyCodes::Packet,
            // VK_ATTN => KeyCodes::Attn,
            // VK_CRSEL => KeyCodes::CrSel,
            // VK_EXSEL => KeyCodes::ExSel,
            // VK_EREOF => KeyCodes::EraseEOF,
            // VK_PLAY => KeyCodes::Play,
            // VK_ZOOM => KeyCodes::Zoom,
            // VK_PA1 => KeyCodes::PA1,
            // VK_OEM_CLEAR => KeyCodes::OEMClear,
            _ => return Err(anyhow::anyhow!("Unknown VIRTUAL_KEY({})", value.0)),
        })
    }
}

pub(crate) fn keystroke_remapping(
    input: &str,
    char_matching: bool,
) -> anyhow::Result<(KeyCodes, Modifiers)> {
    let mut modifiers = Modifiers::default();
    if char_matching {
        if let Some((key, shift, ctrl, alt)) = KeyCodes::parse_char(input)
        .context(format!("Failed to remap keystroke based on char matching: source {}, fallback to use Virtual Key based remapping", input))
        .log_err()
        {
            modifiers.shift = shift;
            modifiers.control = ctrl;
            modifiers.alt = alt;
            return Ok((key, modifiers));
        }
    }
    let (key, shift) = KeyCodes::parse(input)?;
    modifiers.shift = shift;
    Ok((key, modifiers))
}
