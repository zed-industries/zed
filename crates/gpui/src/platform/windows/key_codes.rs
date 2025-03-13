/// TODO:
#[derive(Clone, Debug, Eq, PartialEq, Default, Hash)]
pub enum KeyCodes {
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
    /// input is standard US English layout key
    pub fn parse(input: &str) -> anyhow::Result<Self> {
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
            "0" => KeyCodes::Digital0,
            "1" => KeyCodes::Digital1,
            "2" => KeyCodes::Digital2,
            "3" => KeyCodes::Digital3,
            "4" => KeyCodes::Digital4,
            "5" => KeyCodes::Digital5,
            "6" => KeyCodes::Digital6,
            "7" => KeyCodes::Digital7,
            "8" => KeyCodes::Digital8,
            "9" => KeyCodes::Digital9,
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
            ";" => KeyCodes::Semicolon,
            "=" => KeyCodes::Plus,
            "," => KeyCodes::Comma,
            "-" => KeyCodes::Minus,
            "." => KeyCodes::Period,
            "/" => KeyCodes::Slash,
            "`" => KeyCodes::Tilde,
            "[" => KeyCodes::LeftBracket,
            "\\" => KeyCodes::Backslash,
            "]" => KeyCodes::RightBracket,
            "'" => KeyCodes::Quote,
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
            _ => KeyCodes::Unknown,
        };
        if map_result == KeyCodes::Unknown {
            Err(anyhow::anyhow!(
                "Error parsing keystroke to virtual keycode: {input}"
            ))
        } else {
            Ok(map_result)
        }
    }

    /// TODO:
    pub fn unparse(&self) -> String {
        match self {
            KeyCodes::Unknown => "UnImplemented",
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
            KeyCodes::App => "UnImplemented", // TODO: Chrome use this as Fn key
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
            KeyCodes::BrowserBack => "UnImplemented",
            KeyCodes::BrowserForward => "UnImplemented",
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
        .to_string()
    }
}
