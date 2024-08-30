pub(crate) mod keyboard_layouts;

use serde::Deserialize;
use strum::EnumIter;

/// TODO:
/// https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes
/// https://source.chromium.org/chromium/chromium/src/+/main:ui/events/keycodes/keyboard_codes_win.h;drc=341564182474622e33c964e73a69ea8c1e004eb8;l=12
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default, Deserialize, Hash, EnumIter)]
#[serde(rename_all = "lowercase")]
pub enum VirtualKeyCode {
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
    Shift,
    /// CTRL key, `VK_CONTROL` on Windows. Note, both left-ctrl and right-ctrl can
    /// trigger this.
    Control,
    /// Alt key, `VK_MENU` on Windows. Note, both left-alt and right-alt can
    /// trigger this.
    Alt,
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
    #[serde(rename = "0")]
    Digital0,
    /// 0 key on the main keyboard, `VK_0` on Windows.
    #[serde(rename = "1")]
    Digital1,
    /// 0 key on the main keyboard, `VK_0` on Windows.
    #[serde(rename = "2")]
    Digital2,
    /// 0 key on the main keyboard, `VK_0` on Windows.
    #[serde(rename = "3")]
    Digital3,
    /// 0 key on the main keyboard, `VK_0` on Windows.
    #[serde(rename = "4")]
    Digital4,
    /// 0 key on the main keyboard, `VK_0` on Windows.
    #[serde(rename = "5")]
    Digital5,
    /// 0 key on the main keyboard, `VK_0` on Windows.
    #[serde(rename = "6")]
    Digital6,
    /// 0 key on the main keyboard, `VK_0` on Windows.
    #[serde(rename = "7")]
    Digital7,
    /// 0 key on the main keyboard, `VK_0` on Windows.
    #[serde(rename = "8")]
    Digital8,
    /// 0 key on the main keyboard, `VK_0` on Windows.
    #[serde(rename = "9")]
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
    /// Left WIN key `VK_LWIN` on Windows,
    /// TODO: macOS, Linux
    LeftPlatform,
    /// Right WIN key `VK_RWIN` on Windows,
    /// TODO: macOS, Linux
    RightPlatform,
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
    LeftShift,
    /// Right SHIFT key
    RightShift,
    /// Left CONTROL key
    LeftControl,
    /// Right CONTROL key
    RightControl,
    /// Left ALT key
    LeftAlt,
    /// Right ALT key
    RightAlt,
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
    #[serde(rename = ";")]
    OEM1,
    /// For any country/region, the `+` key
    #[serde(rename = "=")]
    OEMPlus,
    /// For any country/region, the `,` key
    #[serde(rename = ",")]
    OEMComma,
    /// For any country/region, the `-` key
    #[serde(rename = "-")]
    OEMMinus,
    /// For any country/region, the . key
    #[serde(rename = ".")]
    OEMPeriod,
    /// Used for miscellaneous characters, it can vary by keyboard.
    /// For the US standard keyboard, the `/?` key
    #[serde(rename = "/")]
    OEM2,
    /// Used for miscellaneous characters, it can vary by keyboard.
    /// For the US standard keyboard, the `~ key
    #[serde(rename = "`")]
    OEM3,
    /// Used for miscellaneous characters, it can vary by keyboard.
    /// For the US standard keyboard, the `[{` key
    #[serde(rename = "[")]
    OEM4,
    /// Used for miscellaneous characters, it can vary by keyboard.
    /// For the US standard keyboard, the `\|` key
    #[serde(rename = "\\")]
    OEM5,
    /// Used for miscellaneous characters, it can vary by keyboard.
    /// For the US standard keyboard, the `]}` key
    #[serde(rename = "]")]
    OEM6,
    /// Used for miscellaneous characters, it can vary by keyboard.
    /// For the US standard keyboard, the `'"` key
    #[serde(rename = "'")]
    OEM7,
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

impl VirtualKeyCode {
    /// TODO:
    pub fn is_printable(&self) -> bool {
        // match key {
        //     "up" | "down" | "left" | "right" | "pageup" | "pagedown" | "home" | "end" | "delete"
        //     | "escape" | "backspace" | "f1" | "f2" | "f3" | "f4" | "f5" | "f6" | "f7" | "f8" | "f9"
        //     | "f10" | "f11" | "f12" => false,
        //     _ => true,
        // }
        match self {
            VirtualKeyCode::Backspace
            | VirtualKeyCode::Delete
            | VirtualKeyCode::Left
            | VirtualKeyCode::Up
            | VirtualKeyCode::Right
            | VirtualKeyCode::Down
            | VirtualKeyCode::PageUp
            | VirtualKeyCode::PageDown
            | VirtualKeyCode::Insert
            | VirtualKeyCode::Home
            | VirtualKeyCode::End
            | VirtualKeyCode::Escape
            | VirtualKeyCode::F1
            | VirtualKeyCode::F2
            | VirtualKeyCode::F3
            | VirtualKeyCode::F4
            | VirtualKeyCode::F5
            | VirtualKeyCode::F6
            | VirtualKeyCode::F7
            | VirtualKeyCode::F8
            | VirtualKeyCode::F9
            | VirtualKeyCode::F10
            | VirtualKeyCode::F11
            | VirtualKeyCode::F12
            | VirtualKeyCode::F13
            | VirtualKeyCode::F14
            | VirtualKeyCode::F15
            | VirtualKeyCode::F16
            | VirtualKeyCode::F17
            | VirtualKeyCode::F18
            | VirtualKeyCode::F19
            | VirtualKeyCode::F20
            | VirtualKeyCode::F21
            | VirtualKeyCode::F22
            | VirtualKeyCode::F23
            | VirtualKeyCode::F24 => false,
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
            "shift" => Self::Shift,
            "ctrl" => Self::Control,
            "alt" => Self::Alt,
            // VirtualKeyCode::Pause => "UnImplemented",
            "capslock" => Self::Capital,
            // VirtualKeyCode::Kana => "UnImplemented",
            // VirtualKeyCode::Hangul => "UnImplemented",
            // VirtualKeyCode::Junja => "UnImplemented",
            // VirtualKeyCode::Final => "UnImplemented",
            // VirtualKeyCode::Hanja => "UnImplemented",
            // VirtualKeyCode::Kanji => "UnImplemented",
            "escape" => VirtualKeyCode::Escape,
            // VirtualKeyCode::Convert => "UnImplemented",
            // VirtualKeyCode::Nonconvert => "UnImplemented",
            // VirtualKeyCode::Accept => "UnImplemented",
            // VirtualKeyCode::ModeChange => "UnImplemented",
            "space" => VirtualKeyCode::Space, // TODO:
            "pageup" => VirtualKeyCode::PageUp,
            "pagedown" => VirtualKeyCode::PageDown,
            "end" => VirtualKeyCode::End,
            "home" => VirtualKeyCode::Home,
            "left" => VirtualKeyCode::Left,
            "up" => VirtualKeyCode::Up,
            "right" => VirtualKeyCode::Right,
            "down" => VirtualKeyCode::Down,
            // VirtualKeyCode::Select => "UnImplemented",
            // VirtualKeyCode::Print => "UnImplemented",
            // VirtualKeyCode::Execute => "UnImplemented",
            // VirtualKeyCode::PrintScreen => "UnImplemented",
            "insert" => VirtualKeyCode::Insert,
            "delete" => VirtualKeyCode::Delete,
            // VirtualKeyCode::Help => "UnImplemented",
            "0" => VirtualKeyCode::Digital0,
            "1" => VirtualKeyCode::Digital1,
            "2" => VirtualKeyCode::Digital2,
            "3" => VirtualKeyCode::Digital3,
            "4" => VirtualKeyCode::Digital4,
            "5" => VirtualKeyCode::Digital5,
            "6" => VirtualKeyCode::Digital6,
            "7" => VirtualKeyCode::Digital7,
            "8" => VirtualKeyCode::Digital8,
            "9" => VirtualKeyCode::Digital9,
            "a" => VirtualKeyCode::A,
            "b" => VirtualKeyCode::B,
            "c" => VirtualKeyCode::C,
            "d" => VirtualKeyCode::D,
            "e" => VirtualKeyCode::E,
            "f" => VirtualKeyCode::F,
            "g" => VirtualKeyCode::G,
            "h" => VirtualKeyCode::H,
            "i" => VirtualKeyCode::I,
            "j" => VirtualKeyCode::J,
            "k" => VirtualKeyCode::K,
            "l" => VirtualKeyCode::L,
            "m" => VirtualKeyCode::M,
            "n" => VirtualKeyCode::N,
            "o" => VirtualKeyCode::O,
            "p" => VirtualKeyCode::P,
            "q" => VirtualKeyCode::Q,
            "r" => VirtualKeyCode::R,
            "s" => VirtualKeyCode::S,
            "t" => VirtualKeyCode::T,
            "u" => VirtualKeyCode::U,
            "v" => VirtualKeyCode::V,
            "w" => VirtualKeyCode::W,
            "x" => VirtualKeyCode::X,
            "y" => VirtualKeyCode::Y,
            "z" => VirtualKeyCode::Z,
            "win" | "cmd" | "super" => Self::LeftPlatform, // TODO: RightPlatform?
            // VirtualKeyCode::RightPlatform => "platform",
            // VirtualKeyCode::App => "UnImplemented", // TODO: Chrome use this as Fn key
            // VirtualKeyCode::Sleep => "UnImplemented",
            // VirtualKeyCode::Numpad0 => "UnImplemented", // TODO: Hanlde numpad key
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
            "f1" => VirtualKeyCode::F1,
            "f2" => VirtualKeyCode::F2,
            "f3" => VirtualKeyCode::F3,
            "f4" => VirtualKeyCode::F4,
            "f5" => VirtualKeyCode::F5,
            "f6" => VirtualKeyCode::F6,
            "f7" => VirtualKeyCode::F7,
            "f8" => VirtualKeyCode::F8,
            "f9" => VirtualKeyCode::F9,
            "f10" => VirtualKeyCode::F10,
            "f11" => VirtualKeyCode::F11,
            "f12" => VirtualKeyCode::F12,
            "f13" => VirtualKeyCode::F13,
            "f14" => VirtualKeyCode::F14,
            "f15" => VirtualKeyCode::F15,
            "f16" => VirtualKeyCode::F16,
            "f17" => VirtualKeyCode::F17,
            "f18" => VirtualKeyCode::F18,
            "f19" => VirtualKeyCode::F19,
            "f20" => VirtualKeyCode::F20,
            "f21" => VirtualKeyCode::F21,
            "f22" => VirtualKeyCode::F22,
            "f23" => VirtualKeyCode::F23,
            "f24" => VirtualKeyCode::F24,
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
            ";" => VirtualKeyCode::OEM1,
            "=" => VirtualKeyCode::OEMPlus,
            "," => VirtualKeyCode::OEMComma,
            "-" => VirtualKeyCode::OEMMinus,
            "." => VirtualKeyCode::OEMPeriod,
            "/" => VirtualKeyCode::OEM2,
            "`" => VirtualKeyCode::OEM3,
            "[" => VirtualKeyCode::OEM4,
            "\\" => VirtualKeyCode::OEM5,
            "]" => VirtualKeyCode::OEM6,
            "'" => VirtualKeyCode::OEM7,
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
            _ => VirtualKeyCode::Unknown,
        };
        if map_result == VirtualKeyCode::Unknown {
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
            VirtualKeyCode::Unknown => "UnImplemented",
            VirtualKeyCode::Function => "fn",
            VirtualKeyCode::Cancel => "cancel",
            VirtualKeyCode::Backspace => "backspace",
            VirtualKeyCode::Tab => "tab",
            VirtualKeyCode::Clear => "UnImplemented",
            VirtualKeyCode::Enter => "enter",
            VirtualKeyCode::Shift => "shift",
            VirtualKeyCode::Control => "ctrl",
            VirtualKeyCode::Alt => "alt",
            VirtualKeyCode::Pause => "UnImplemented",
            VirtualKeyCode::Capital => "capslock",
            VirtualKeyCode::Kana => "UnImplemented",
            VirtualKeyCode::Hangul => "UnImplemented",
            VirtualKeyCode::Junja => "UnImplemented",
            VirtualKeyCode::Final => "UnImplemented",
            VirtualKeyCode::Hanja => "UnImplemented",
            VirtualKeyCode::Kanji => "UnImplemented",
            VirtualKeyCode::Escape => "escape",
            VirtualKeyCode::Convert => "UnImplemented",
            VirtualKeyCode::Nonconvert => "UnImplemented",
            VirtualKeyCode::Accept => "UnImplemented",
            VirtualKeyCode::ModeChange => "UnImplemented",
            VirtualKeyCode::Space => "space",
            VirtualKeyCode::PageUp => "pageup",
            VirtualKeyCode::PageDown => "pagedown",
            VirtualKeyCode::End => "end",
            VirtualKeyCode::Home => "home",
            VirtualKeyCode::Left => "left",
            VirtualKeyCode::Up => "up",
            VirtualKeyCode::Right => "right",
            VirtualKeyCode::Down => "down",
            VirtualKeyCode::Select => "UnImplemented",
            VirtualKeyCode::Print => "UnImplemented",
            VirtualKeyCode::Execute => "UnImplemented",
            VirtualKeyCode::PrintScreen => "UnImplemented",
            VirtualKeyCode::Insert => "insert",
            VirtualKeyCode::Delete => "delete",
            VirtualKeyCode::Help => "UnImplemented",
            VirtualKeyCode::Digital0 => "0",
            VirtualKeyCode::Digital1 => "1",
            VirtualKeyCode::Digital2 => "2",
            VirtualKeyCode::Digital3 => "3",
            VirtualKeyCode::Digital4 => "4",
            VirtualKeyCode::Digital5 => "5",
            VirtualKeyCode::Digital6 => "6",
            VirtualKeyCode::Digital7 => "7",
            VirtualKeyCode::Digital8 => "8",
            VirtualKeyCode::Digital9 => "9",
            VirtualKeyCode::A => "a",
            VirtualKeyCode::B => "b",
            VirtualKeyCode::C => "c",
            VirtualKeyCode::D => "d",
            VirtualKeyCode::E => "e",
            VirtualKeyCode::F => "f",
            VirtualKeyCode::G => "g",
            VirtualKeyCode::H => "h",
            VirtualKeyCode::I => "i",
            VirtualKeyCode::J => "j",
            VirtualKeyCode::K => "k",
            VirtualKeyCode::L => "l",
            VirtualKeyCode::M => "m",
            VirtualKeyCode::N => "n",
            VirtualKeyCode::O => "o",
            VirtualKeyCode::P => "p",
            VirtualKeyCode::Q => "q",
            VirtualKeyCode::R => "r",
            VirtualKeyCode::S => "s",
            VirtualKeyCode::T => "t",
            VirtualKeyCode::U => "u",
            VirtualKeyCode::V => "v",
            VirtualKeyCode::W => "w",
            VirtualKeyCode::X => "x",
            VirtualKeyCode::Y => "y",
            VirtualKeyCode::Z => "z",
            #[cfg(target_os = "windows")]
            VirtualKeyCode::LeftPlatform => "win",
            #[cfg(target_os = "macos")]
            VirtualKeyCode::LeftPlatform => "cmd",
            #[cfg(target_os = "linux")]
            VirtualKeyCode::LeftPlatform => "super", // TODO:
            VirtualKeyCode::RightPlatform => "UnImplemented",
            // #[cfg(target_os = "windows")]
            // VirtualKeyCode::RightPlatform => "win",
            // #[cfg(target_os = "macos")]
            // VirtualKeyCode::RightPlatform => "cmd",
            // #[cfg(target_os = "linux")]
            // VirtualKeyCode::RightPlatform => "super", // TODO:
            VirtualKeyCode::App => "UnImplemented", // TODO: Chrome use this as Fn key
            VirtualKeyCode::Sleep => "UnImplemented",
            VirtualKeyCode::Numpad0 => "UnImplemented", // TODO: handle numpad key
            VirtualKeyCode::Numpad1 => "UnImplemented",
            VirtualKeyCode::Numpad2 => "UnImplemented",
            VirtualKeyCode::Numpad3 => "UnImplemented",
            VirtualKeyCode::Numpad4 => "UnImplemented",
            VirtualKeyCode::Numpad5 => "UnImplemented",
            VirtualKeyCode::Numpad6 => "UnImplemented",
            VirtualKeyCode::Numpad7 => "UnImplemented",
            VirtualKeyCode::Numpad8 => "UnImplemented",
            VirtualKeyCode::Numpad9 => "UnImplemented",
            VirtualKeyCode::Multiply => "UnImplemented",
            VirtualKeyCode::Add => "UnImplemented",
            VirtualKeyCode::Separator => "UnImplemented",
            VirtualKeyCode::Subtract => "UnImplemented",
            VirtualKeyCode::Decimal => "UnImplemented",
            VirtualKeyCode::Divide => "UnImplemented",
            VirtualKeyCode::F1 => "f1",
            VirtualKeyCode::F2 => "f2",
            VirtualKeyCode::F3 => "f3",
            VirtualKeyCode::F4 => "f4",
            VirtualKeyCode::F5 => "f5",
            VirtualKeyCode::F6 => "f6",
            VirtualKeyCode::F7 => "f7",
            VirtualKeyCode::F8 => "f8",
            VirtualKeyCode::F9 => "f9",
            VirtualKeyCode::F10 => "f10",
            VirtualKeyCode::F11 => "f11",
            VirtualKeyCode::F12 => "f12",
            VirtualKeyCode::F13 => "f13",
            VirtualKeyCode::F14 => "f14",
            VirtualKeyCode::F15 => "f15",
            VirtualKeyCode::F16 => "f16",
            VirtualKeyCode::F17 => "f17",
            VirtualKeyCode::F18 => "f18",
            VirtualKeyCode::F19 => "f19",
            VirtualKeyCode::F20 => "f20",
            VirtualKeyCode::F21 => "f21",
            VirtualKeyCode::F22 => "f22",
            VirtualKeyCode::F23 => "f23",
            VirtualKeyCode::F24 => "f24",
            VirtualKeyCode::NumLock => "UnImplemented",
            VirtualKeyCode::ScrollLock => "UnImplemented",
            VirtualKeyCode::LeftShift => "UnImplemented",
            VirtualKeyCode::RightShift => "UnImplemented",
            VirtualKeyCode::LeftControl => "UnImplemented",
            VirtualKeyCode::RightControl => "UnImplemented",
            VirtualKeyCode::LeftAlt => "UnImplemented",
            VirtualKeyCode::RightAlt => "UnImplemented",
            // VirtualKeyCode::LeftShift => "shift", // TODO:
            // VirtualKeyCode::RightShift => "shift",
            // VirtualKeyCode::LeftControl => "control",
            // VirtualKeyCode::RightControl => "control",
            // VirtualKeyCode::LeftAlt => "alt",
            // VirtualKeyCode::RightAlt => "alt",
            VirtualKeyCode::BrowserBack => "UnImplemented",
            VirtualKeyCode::BrowserForward => "UnImplemented",
            VirtualKeyCode::BrowserRefresh => "UnImplemented",
            VirtualKeyCode::BrowserStop => "UnImplemented",
            VirtualKeyCode::BrowserSearch => "UnImplemented",
            VirtualKeyCode::BrowserFavorites => "UnImplemented",
            VirtualKeyCode::BrowserHome => "UnImplemented",
            VirtualKeyCode::VolumeMute => "UnImplemented",
            VirtualKeyCode::VolumeDown => "UnImplemented",
            VirtualKeyCode::VolumeUp => "UnImplemented",
            VirtualKeyCode::MediaNextTrack => "UnImplemented",
            VirtualKeyCode::MediaPrevTrack => "UnImplemented",
            VirtualKeyCode::MediaStop => "UnImplemented",
            VirtualKeyCode::MediaPlayPause => "UnImplemented",
            VirtualKeyCode::LaunchMail => "UnImplemented",
            VirtualKeyCode::LaunchMediaSelect => "UnImplemented",
            VirtualKeyCode::LaunchApp1 => "UnImplemented",
            VirtualKeyCode::LaunchApp2 => "UnImplemented",
            VirtualKeyCode::OEM1 => ";",
            VirtualKeyCode::OEMPlus => "=",
            VirtualKeyCode::OEMComma => ",",
            VirtualKeyCode::OEMMinus => "-",
            VirtualKeyCode::OEMPeriod => ".",
            VirtualKeyCode::OEM2 => "/",
            VirtualKeyCode::OEM3 => "`",
            VirtualKeyCode::OEM4 => "[",
            VirtualKeyCode::OEM5 => "\\",
            VirtualKeyCode::OEM6 => "]",
            VirtualKeyCode::OEM7 => "'",
            VirtualKeyCode::OEM8 => "UnImplemented",
            VirtualKeyCode::OEM102 => "UnImplemented",
            VirtualKeyCode::ProcessKey => "UnImplemented",
            VirtualKeyCode::Packet => "UnImplemented",
            VirtualKeyCode::Attn => "UnImplemented",
            VirtualKeyCode::CrSel => "UnImplemented",
            VirtualKeyCode::ExSel => "UnImplemented",
            VirtualKeyCode::EraseEOF => "UnImplemented",
            VirtualKeyCode::Play => "UnImplemented",
            VirtualKeyCode::Zoom => "UnImplemented",
            VirtualKeyCode::PA1 => "UnImplemented",
            VirtualKeyCode::OEMClear => "UnImplemented",
        }
        .to_string()
    }
}

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use crate::VirtualKeyCode;

    #[test]
    fn test_vkcode_parse_failure() {
        assert!(VirtualKeyCode::from_str("{").is_err());
        assert!(VirtualKeyCode::from_str("?").is_err());
        assert!(VirtualKeyCode::from_str(">").is_err());
    }

    #[test]
    fn test_vkcode_string() {
        for key in VirtualKeyCode::iter() {
            if let Ok(right) = VirtualKeyCode::from_str(&key.to_string()) {
                assert_eq!(key, right);
            }
        }
    }
}
