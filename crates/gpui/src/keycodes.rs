pub(crate) mod keyboard_layouts;

use serde::Deserialize;
use strum::EnumIter;

/// TODO:
/// https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes
/// https://source.chromium.org/chromium/chromium/src/+/main:ui/events/keycodes/keyboard_codes_win.h;drc=341564182474622e33c964e73a69ea8c1e004eb8;l=12
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default, Deserialize, Hash, EnumIter)]
pub enum Keys {
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
    /// IME Kana mode, `VK_KANA` on Windows.
    Kana,
    /// IME Hangul mode, `VK_HANGUL` on Windows.
    Hangul,
    ///IME Junja mode, `VK_JUNJA` on Windows.
    Junja,
    /// IME final mode, `VK_FINAL` on Windows.
    Final,
    /// IME Hanja mode, `VK_HANJA` on Windows.
    Hanja,
    /// IME Kanji mode, `VK_KANJI` on Winodws.
    Kanji,
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
    /// TODO:
    Platform(KeyPosition),
    /// Left WIN key `VK_LWIN` on Windows,
    /// TODO: macOS, Linux
    // LeftPlatform,
    /// Right WIN key `VK_RWIN` on Windows,
    /// TODO: macOS, Linux
    // RightPlatform,
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
    /// Left SHIFT key
    // LeftShift,
    /// Right SHIFT key
    // RightShift,
    /// Left CONTROL key
    // LeftControl,
    /// Right CONTROL key
    // RightControl,
    /// Left ALT key
    // LeftAlt,
    /// Right ALT key
    // RightAlt,
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
    /// IME PROCESS key
    ProcessKey,
    /// Used to pass Unicode characters as if they were keystrokes.
    /// The `VK_PACKET` on Windows, this key is the low word of a 32-bit Virtual Key
    /// value used for non-keyboard input methods.
    ///
    /// For more information, see Remark in KEYBDINPUT, SendInput, WM_KEYDOWN, and WM_KEYUP
    Packet,
    // TODO: These keys not presented on Windows doc, but on Chrome.
    // VKEY_OEM_ATTN = VK_OEM_ATTN,
    // VKEY_OEM_FINISH = VK_OEM_FINISH,
    // VKEY_OEM_COPY = VK_OEM_COPY,
    // VKEY_DBE_SBCSCHAR = VK_DBE_SBCSCHAR,
    // VKEY_DBE_DBCSCHAR = VK_DBE_DBCSCHAR,
    // VKEY_OEM_BACKTAB = VK_OEM_BACKTAB,
    /// Attn key
    Attn,
    /// CrSel key
    CrSel,
    /// ExSel key
    ExSel,
    /// Erase EOF key
    EraseEOF,
    /// Play key
    Play,
    /// Zoom key
    Zoom,
    // TODO: These keys are reserved by Windows but are used by Chrome, `VK_NONAME`
    // NoName
    // Paste
    /// PA1 key
    PA1,
    /// Clear key
    OEMClear,
}

/// TODO:
#[derive(Copy, Clone, Debug, Default, Deserialize, Hash)]
pub enum KeyPosition {
    /// TODO:
    #[default]
    Any,
    /// TODO:
    Left,
    /// TODO:
    Right,
}

impl Keys {
    /// TODO:
    pub fn is_printable(&self) -> bool {
        // match key {
        //     "up" | "down" | "left" | "right" | "pageup" | "pagedown" | "home" | "end" | "delete"
        //     | "escape" | "backspace" | "f1" | "f2" | "f3" | "f4" | "f5" | "f6" | "f7" | "f8" | "f9"
        //     | "f10" | "f11" | "f12" => false,
        //     _ => true,
        // }
        match self {
            Keys::Backspace
            | Keys::Delete
            | Keys::Left
            | Keys::Up
            | Keys::Right
            | Keys::Down
            | Keys::PageUp
            | Keys::PageDown
            | Keys::Insert
            | Keys::Home
            | Keys::End
            | Keys::Escape
            | Keys::F1
            | Keys::F2
            | Keys::F3
            | Keys::F4
            | Keys::F5
            | Keys::F6
            | Keys::F7
            | Keys::F8
            | Keys::F9
            | Keys::F10
            | Keys::F11
            | Keys::F12
            | Keys::F13
            | Keys::F14
            | Keys::F15
            | Keys::F16
            | Keys::F17
            | Keys::F18
            | Keys::F19
            | Keys::F20
            | Keys::F21
            | Keys::F22
            | Keys::F23
            | Keys::F24 => false,
            _ => true,
        }
    }

    /// input is standard US English layout key
    pub fn from_str(input: &str) -> anyhow::Result<Self> {
        let map_result = match input {
            "UnImplemented" | "Unknown" => Self::Unknown,
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
            "escape" => Keys::Escape,
            // VirtualKeyCode::Convert => "UnImplemented",
            // VirtualKeyCode::Nonconvert => "UnImplemented",
            // VirtualKeyCode::Accept => "UnImplemented",
            // VirtualKeyCode::ModeChange => "UnImplemented",
            "space" => Keys::Space, // TODO:
            "pageup" => Keys::PageUp,
            "pagedown" => Keys::PageDown,
            "end" => Keys::End,
            "home" => Keys::Home,
            "left" => Keys::Left,
            "up" => Keys::Up,
            "right" => Keys::Right,
            "down" => Keys::Down,
            // VirtualKeyCode::Select => "UnImplemented",
            // VirtualKeyCode::Print => "UnImplemented",
            // VirtualKeyCode::Execute => "UnImplemented",
            // VirtualKeyCode::PrintScreen => "UnImplemented",
            "insert" => Keys::Insert,
            "delete" => Keys::Delete,
            // VirtualKeyCode::Help => "UnImplemented",
            "0" => Keys::Digital0,
            "1" => Keys::Digital1,
            "2" => Keys::Digital2,
            "3" => Keys::Digital3,
            "4" => Keys::Digital4,
            "5" => Keys::Digital5,
            "6" => Keys::Digital6,
            "7" => Keys::Digital7,
            "8" => Keys::Digital8,
            "9" => Keys::Digital9,
            "a" => Keys::A,
            "b" => Keys::B,
            "c" => Keys::C,
            "d" => Keys::D,
            "e" => Keys::E,
            "f" => Keys::F,
            "g" => Keys::G,
            "h" => Keys::H,
            "i" => Keys::I,
            "j" => Keys::J,
            "k" => Keys::K,
            "l" => Keys::L,
            "m" => Keys::M,
            "n" => Keys::N,
            "o" => Keys::O,
            "p" => Keys::P,
            "q" => Keys::Q,
            "r" => Keys::R,
            "s" => Keys::S,
            "t" => Keys::T,
            "u" => Keys::U,
            "v" => Keys::V,
            "w" => Keys::W,
            "x" => Keys::X,
            "y" => Keys::Y,
            "z" => Keys::Z,
            "win" | "cmd" | "super" => Self::Platform(KeyPosition::Any),
            // VirtualKeyCode::App => "UnImplemented", // TODO: Chrome use this as Fn key
            // VirtualKeyCode::Sleep => "UnImplemented",
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
            "f1" => Keys::F1,
            "f2" => Keys::F2,
            "f3" => Keys::F3,
            "f4" => Keys::F4,
            "f5" => Keys::F5,
            "f6" => Keys::F6,
            "f7" => Keys::F7,
            "f8" => Keys::F8,
            "f9" => Keys::F9,
            "f10" => Keys::F10,
            "f11" => Keys::F11,
            "f12" => Keys::F12,
            "f13" => Keys::F13,
            "f14" => Keys::F14,
            "f15" => Keys::F15,
            "f16" => Keys::F16,
            "f17" => Keys::F17,
            "f18" => Keys::F18,
            "f19" => Keys::F19,
            "f20" => Keys::F20,
            "f21" => Keys::F21,
            "f22" => Keys::F22,
            "f23" => Keys::F23,
            "f24" => Keys::F24,
            // VirtualKeyCode::NumLock => "UnImplemented",
            // VirtualKeyCode::ScrollLock => "UnImplemented",
            // VirtualKeyCode::LeftShift => "shift", // TODO:
            // VirtualKeyCode::RightShift => "shift", // TODO:
            // VirtualKeyCode::LeftControl => "control", // TODO:
            // VirtualKeyCode::RightControl => "control", // TODO:
            // VirtualKeyCode::LeftAlt => "alt", // TODO:
            // VirtualKeyCode::RightAlt => "alt", // TODO:
            // VirtualKeyCode::BrowserBack => "UnImplemented",
            // VirtualKeyCode::BrowserForward => "UnImplemented",
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
            ";" => Keys::Semicolon,
            "=" => Keys::Plus,
            "," => Keys::Comma,
            "-" => Keys::Minus,
            "." => Keys::Period,
            "/" => Keys::Slash,
            "`" => Keys::Tilde,
            "[" => Keys::LeftBracket,
            "\\" => Keys::Backslash,
            "]" => Keys::RightBracket,
            "'" => Keys::Quote,
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
            _ => Keys::Unknown,
        };
        if map_result == Keys::Unknown {
            Err(anyhow::anyhow!(
                "Error parsing keystroke to virtual keycode: {input}"
            ))
        } else {
            Ok(map_result)
        }
    }

    /// TODO:
    pub fn to_string(&self) -> String {
        match self {
            Keys::Unknown => "UnImplemented",
            Keys::Function => "fn",
            Keys::Cancel => "cancel",
            Keys::Backspace => "backspace",
            Keys::Tab => "tab",
            Keys::Clear => "UnImplemented",
            Keys::Enter => "enter",
            // TODO: position
            Keys::Shift(_) => "shift",
            Keys::Control(_) => "ctrl",
            Keys::Alt(_) => "alt",
            Keys::Pause => "UnImplemented",
            Keys::Capital => "capslock",
            Keys::Kana => "UnImplemented",
            Keys::Hangul => "UnImplemented",
            Keys::Junja => "UnImplemented",
            Keys::Final => "UnImplemented",
            Keys::Hanja => "UnImplemented",
            Keys::Kanji => "UnImplemented",
            Keys::Escape => "escape",
            Keys::Convert => "UnImplemented",
            Keys::Nonconvert => "UnImplemented",
            Keys::Accept => "UnImplemented",
            Keys::ModeChange => "UnImplemented",
            Keys::Space => "space",
            Keys::PageUp => "pageup",
            Keys::PageDown => "pagedown",
            Keys::End => "end",
            Keys::Home => "home",
            Keys::Left => "left",
            Keys::Up => "up",
            Keys::Right => "right",
            Keys::Down => "down",
            Keys::Select => "UnImplemented",
            Keys::Print => "UnImplemented",
            Keys::Execute => "UnImplemented",
            Keys::PrintScreen => "UnImplemented",
            Keys::Insert => "insert",
            Keys::Delete => "delete",
            Keys::Help => "UnImplemented",
            Keys::Digital0 => "0",
            Keys::Digital1 => "1",
            Keys::Digital2 => "2",
            Keys::Digital3 => "3",
            Keys::Digital4 => "4",
            Keys::Digital5 => "5",
            Keys::Digital6 => "6",
            Keys::Digital7 => "7",
            Keys::Digital8 => "8",
            Keys::Digital9 => "9",
            Keys::A => "a",
            Keys::B => "b",
            Keys::C => "c",
            Keys::D => "d",
            Keys::E => "e",
            Keys::F => "f",
            Keys::G => "g",
            Keys::H => "h",
            Keys::I => "i",
            Keys::J => "j",
            Keys::K => "k",
            Keys::L => "l",
            Keys::M => "m",
            Keys::N => "n",
            Keys::O => "o",
            Keys::P => "p",
            Keys::Q => "q",
            Keys::R => "r",
            Keys::S => "s",
            Keys::T => "t",
            Keys::U => "u",
            Keys::V => "v",
            Keys::W => "w",
            Keys::X => "x",
            Keys::Y => "y",
            Keys::Z => "z",
            // TODO: handle position
            #[cfg(target_os = "windows")]
            Keys::Platform(_) => "win",
            #[cfg(target_os = "macos")]
            Keys::Platform(_) => "cmd",
            #[cfg(target_os = "linux")]
            Keys::Platform(_) => "super",
            Keys::App => "UnImplemented", // TODO: Chrome use this as Fn key
            Keys::Sleep => "UnImplemented",
            Keys::Numpad0 => "UnImplemented", // TODO: handle numpad key
            Keys::Numpad1 => "UnImplemented",
            Keys::Numpad2 => "UnImplemented",
            Keys::Numpad3 => "UnImplemented",
            Keys::Numpad4 => "UnImplemented",
            Keys::Numpad5 => "UnImplemented",
            Keys::Numpad6 => "UnImplemented",
            Keys::Numpad7 => "UnImplemented",
            Keys::Numpad8 => "UnImplemented",
            Keys::Numpad9 => "UnImplemented",
            Keys::Multiply => "UnImplemented",
            Keys::Add => "UnImplemented",
            Keys::Separator => "UnImplemented",
            Keys::Subtract => "UnImplemented",
            Keys::Decimal => "UnImplemented",
            Keys::Divide => "UnImplemented",
            Keys::F1 => "f1",
            Keys::F2 => "f2",
            Keys::F3 => "f3",
            Keys::F4 => "f4",
            Keys::F5 => "f5",
            Keys::F6 => "f6",
            Keys::F7 => "f7",
            Keys::F8 => "f8",
            Keys::F9 => "f9",
            Keys::F10 => "f10",
            Keys::F11 => "f11",
            Keys::F12 => "f12",
            Keys::F13 => "f13",
            Keys::F14 => "f14",
            Keys::F15 => "f15",
            Keys::F16 => "f16",
            Keys::F17 => "f17",
            Keys::F18 => "f18",
            Keys::F19 => "f19",
            Keys::F20 => "f20",
            Keys::F21 => "f21",
            Keys::F22 => "f22",
            Keys::F23 => "f23",
            Keys::F24 => "f24",
            Keys::NumLock => "UnImplemented",
            Keys::ScrollLock => "UnImplemented",
            Keys::BrowserBack => "UnImplemented",
            Keys::BrowserForward => "UnImplemented",
            Keys::BrowserRefresh => "UnImplemented",
            Keys::BrowserStop => "UnImplemented",
            Keys::BrowserSearch => "UnImplemented",
            Keys::BrowserFavorites => "UnImplemented",
            Keys::BrowserHome => "UnImplemented",
            Keys::VolumeMute => "UnImplemented",
            Keys::VolumeDown => "UnImplemented",
            Keys::VolumeUp => "UnImplemented",
            Keys::MediaNextTrack => "UnImplemented",
            Keys::MediaPrevTrack => "UnImplemented",
            Keys::MediaStop => "UnImplemented",
            Keys::MediaPlayPause => "UnImplemented",
            Keys::LaunchMail => "UnImplemented",
            Keys::LaunchMediaSelect => "UnImplemented",
            Keys::LaunchApp1 => "UnImplemented",
            Keys::LaunchApp2 => "UnImplemented",
            Keys::Semicolon => ";",
            Keys::Plus => "=",
            Keys::Comma => ",",
            Keys::Minus => "-",
            Keys::Period => ".",
            Keys::Slash => "/",
            Keys::Tilde => "`",
            Keys::LeftBracket => "[",
            Keys::Backslash => "\\",
            Keys::RightBracket => "]",
            Keys::Quote => "'",
            Keys::OEM8 => "UnImplemented",
            Keys::OEM102 => "UnImplemented",
            Keys::ProcessKey => "UnImplemented",
            Keys::Packet => "UnImplemented",
            Keys::Attn => "UnImplemented",
            Keys::CrSel => "UnImplemented",
            Keys::ExSel => "UnImplemented",
            Keys::EraseEOF => "UnImplemented",
            Keys::Play => "UnImplemented",
            Keys::Zoom => "UnImplemented",
            Keys::PA1 => "UnImplemented",
            Keys::OEMClear => "UnImplemented",
        }
        .to_string()
    }
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

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use crate::Keys;

    #[test]
    fn test_vkcode_parse_failure() {
        assert!(Keys::from_str("{").is_err());
        assert!(Keys::from_str("?").is_err());
        assert!(Keys::from_str(">").is_err());
    }

    #[test]
    fn test_vkcode_string() {
        for key in Keys::iter() {
            if let Ok(right) = Keys::from_str(&key.to_string()) {
                assert_eq!(key, right);
            }
        }
    }
}
