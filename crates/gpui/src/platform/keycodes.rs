use strum::EnumIter;

/// Scan codes for the keyboard, which are used to identify keys in a keyboard layout-independent way.
/// Currently, we only support a limited set of scan codes here:
/// https://code.visualstudio.com/docs/configure/keybindings#_keyboard-layoutindependent-bindings
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum ScanCode {
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
    /// A key on the main keyboard.
    A,
    /// B key on the main keyboard.
    B,
    /// C key on the main keyboard.
    C,
    /// D key on the main keyboard.
    D,
    /// E key on the main keyboard.
    E,
    /// F key on the main keyboard.
    F,
    /// G key on the main keyboard.
    G,
    /// H key on the main keyboard.
    H,
    /// I key on the main keyboard.
    I,
    /// J key on the main keyboard.
    J,
    /// K key on the main keyboard.
    K,
    /// L key on the main keyboard.
    L,
    /// M key on the main keyboard.
    M,
    /// N key on the main keyboard.
    N,
    /// O key on the main keyboard.
    O,
    /// P key on the main keyboard.
    P,
    /// Q key on the main keyboard.
    Q,
    /// R key on the main keyboard.
    R,
    /// S key on the main keyboard.
    S,
    /// T key on the main keyboard.
    T,
    /// U key on the main keyboard.
    U,
    /// V key on the main keyboard.
    V,
    /// W key on the main keyboard.
    W,
    /// X key on the main keyboard.
    X,
    /// Y key on the main keyboard.
    Y,
    /// Z key on the main keyboard.
    Z,
    /// 0 key on the main keyboard.
    Digit0,
    /// 1 key on the main keyboard.
    Digit1,
    /// 2 key on the main keyboard.
    Digit2,
    /// 3 key on the main keyboard.
    Digit3,
    /// 4 key on the main keyboard.
    Digit4,
    /// 5 key on the main keyboard.
    Digit5,
    /// 6 key on the main keyboard.
    Digit6,
    /// 7 key on the main keyboard.
    Digit7,
    /// 8 key on the main keyboard.
    Digit8,
    /// 9 key on the main keyboard.
    Digit9,

    /// Backquote key on the main keyboard: `
    Backquote,
    /// Minus key on the main keyboard: -
    Minus,
    /// Equal key on the main keyboard: =
    Equal,
    /// BracketLeft key on the main keyboard: [
    BracketLeft,
    /// BracketRight key on the main keyboard: ]
    BracketRight,
    /// Backslash key on the main keyboard: \
    Backslash,
    /// Semicolon key on the main keyboard: ;
    Semicolon,
    /// Quote key on the main keyboard: '
    Quote,
    /// Comma key on the main keyboard: ,
    Comma,
    /// Period key on the main keyboard: .
    Period,
    /// Slash key on the main keyboard: /
    Slash,

    /// Left arrow key
    Left,
    /// Up arrow key
    Up,
    /// Right arrow key
    Right,
    /// Down arrow key
    Down,
    /// PAGE UP key
    PageUp,
    /// PAGE DOWN key
    PageDown,
    /// END key
    End,
    /// HOME key
    Home,

    /// TAB key
    Tab,
    /// ENTER key, also known as RETURN key
    /// This does not distinguish between the main Enter key and the numeric keypad Enter key.
    Enter,
    /// ESCAPE key
    Escape,
    /// SPACE key
    Space,
    /// BACKSPACE key
    Backspace,
    /// DELETE key
    Delete,

    // Pause, not supported yet
    // CapsLock, not supported yet
    /// INSERT key
    Insert,
    // The following keys are not supported yet:
    // Numpad0,
    // Numpad1,
    // Numpad2,
    // Numpad3,
    // Numpad4,
    // Numpad5,
    // Numpad6,
    // Numpad7,
    // Numpad8,
    // Numpad9,
    // NumpadMultiply,
    // NumpadAdd,
    // NumpadComma,
    // NumpadSubtract,
    // NumpadDecimal,
    // NumpadDivide,
}

impl ScanCode {
    /// Parse a scan code from a string.
    pub fn parse(source: &str) -> Option<Self> {
        match source {
            "[F1]" => Some(Self::F1),
            "[F2]" => Some(Self::F2),
            "[F3]" => Some(Self::F3),
            "[F4]" => Some(Self::F4),
            "[F5]" => Some(Self::F5),
            "[F6]" => Some(Self::F6),
            "[F7]" => Some(Self::F7),
            "[F8]" => Some(Self::F8),
            "[F9]" => Some(Self::F9),
            "[F10]" => Some(Self::F10),
            "[F11]" => Some(Self::F11),
            "[F12]" => Some(Self::F12),
            "[F13]" => Some(Self::F13),
            "[F14]" => Some(Self::F14),
            "[F15]" => Some(Self::F15),
            "[F16]" => Some(Self::F16),
            "[F17]" => Some(Self::F17),
            "[F18]" => Some(Self::F18),
            "[F19]" => Some(Self::F19),
            "[F20]" => Some(Self::F20),
            "[F21]" => Some(Self::F21),
            "[F22]" => Some(Self::F22),
            "[F23]" => Some(Self::F23),
            "[F24]" => Some(Self::F24),
            "[A]" | "[KeyA]" => Some(Self::A),
            "[B]" | "[KeyB]" => Some(Self::B),
            "[C]" | "[KeyC]" => Some(Self::C),
            "[D]" | "[KeyD]" => Some(Self::D),
            "[E]" | "[KeyE]" => Some(Self::E),
            "[F]" | "[KeyF]" => Some(Self::F),
            "[G]" | "[KeyG]" => Some(Self::G),
            "[H]" | "[KeyH]" => Some(Self::H),
            "[I]" | "[KeyI]" => Some(Self::I),
            "[J]" | "[KeyJ]" => Some(Self::J),
            "[K]" | "[KeyK]" => Some(Self::K),
            "[L]" | "[KeyL]" => Some(Self::L),
            "[M]" | "[KeyM]" => Some(Self::M),
            "[N]" | "[KeyN]" => Some(Self::N),
            "[O]" | "[KeyO]" => Some(Self::O),
            "[P]" | "[KeyP]" => Some(Self::P),
            "[Q]" | "[KeyQ]" => Some(Self::Q),
            "[R]" | "[KeyR]" => Some(Self::R),
            "[S]" | "[KeyS]" => Some(Self::S),
            "[T]" | "[KeyT]" => Some(Self::T),
            "[U]" | "[KeyU]" => Some(Self::U),
            "[V]" | "[KeyV]" => Some(Self::V),
            "[W]" | "[KeyW]" => Some(Self::W),
            "[X]" | "[KeyX]" => Some(Self::X),
            "[Y]" | "[KeyY]" => Some(Self::Y),
            "[Z]" | "[KeyZ]" => Some(Self::Z),
            "[0]" | "[Digit0]" => Some(Self::Digit0),
            "[1]" | "[Digit1]" => Some(Self::Digit1),
            "[2]" | "[Digit2]" => Some(Self::Digit2),
            "[3]" | "[Digit3]" => Some(Self::Digit3),
            "[4]" | "[Digit4]" => Some(Self::Digit4),
            "[5]" | "[Digit5]" => Some(Self::Digit5),
            "[6]" | "[Digit6]" => Some(Self::Digit6),
            "[7]" | "[Digit7]" => Some(Self::Digit7),
            "[8]" | "[Digit8]" => Some(Self::Digit8),
            "[9]" | "[Digit9]" => Some(Self::Digit9),

            "[`]" | "[Backquote]" => Some(Self::Backquote),
            "[-]" | "[Minus]" => Some(Self::Minus),
            "[=]" | "[Equal]" => Some(Self::Equal),
            "[[]" | "[BracketLeft]" => Some(Self::BracketLeft),
            "[]]" | "[BracketRight]" => Some(Self::BracketRight),
            "[\\]" | "[Backslash]" => Some(Self::Backslash),
            "[;]" | "[Semicolon]" => Some(Self::Semicolon),
            "[']" | "[Quote]" => Some(Self::Quote),
            "[,]" | "[Comma]" => Some(Self::Comma),
            "[.]" | "[Period]" => Some(Self::Period),
            "[/]" | "[Slash]" => Some(Self::Slash),

            "[Left]" | "[ArrowLeft]" => Some(Self::Left),
            "[Up]" | "[ArrowUp]" => Some(Self::Up),
            "[Right]" | "[ArrowRight]" => Some(Self::Right),
            "[Down]" | "[ArrowDown]" => Some(Self::Down),
            "[PageUp]" => Some(Self::PageUp),
            "[PageDown]" => Some(Self::PageDown),
            "[End]" => Some(Self::End),
            "[Home]" => Some(Self::Home),

            "[Tab]" => Some(Self::Tab),
            "[Enter]" => Some(Self::Enter),
            "[Escape]" => Some(Self::Escape),
            "[Space]" => Some(Self::Space),
            "[Backspace]" => Some(Self::Backspace),
            "[Delete]" => Some(Self::Delete),

            // "[Pause]" => Some(Self::Pause),
            // "[CapsLock]" => Some(Self::CapsLock),
            "[Insert]" => Some(Self::Insert),

            // "[Numpad0]" => Some(Self::Numpad0),
            // "[Numpad1]" => Some(Self::Numpad1),
            // "[Numpad2]" => Some(Self::Numpad2),
            // "[Numpad3]" => Some(Self::Numpad3),
            // "[Numpad4]" => Some(Self::Numpad4),
            // "[Numpad5]" => Some(Self::Numpad5),
            // "[Numpad6]" => Some(Self::Numpad6),
            // "[Numpad7]" => Some(Self::Numpad7),
            // "[Numpad8]" => Some(Self::Numpad8),
            // "[Numpad9]" => Some(Self::Numpad9),
            // "[NumpadMultiply]" => Some(Self::NumpadMultiply),
            // "[NumpadAdd]" => Some(Self::NumpadAdd),
            // "[NumpadComma]" => Some(Self::NumpadComma),
            // "[NumpadSubtract]" => Some(Self::NumpadSubtract),
            // "[NumpadDecimal]" => Some(Self::NumpadDecimal),
            // "[NumpadDivide]" => Some(Self::NumpadDivide),
            _ => None,
        }
    }

    /// Convert the scan code to its key face for immutable keys.
    pub fn try_to_key(&self) -> Option<String> {
        Some(
            match self {
                ScanCode::F1 => "f1",
                ScanCode::F2 => "f2",
                ScanCode::F3 => "f3",
                ScanCode::F4 => "f4",
                ScanCode::F5 => "f5",
                ScanCode::F6 => "f6",
                ScanCode::F7 => "f7",
                ScanCode::F8 => "f8",
                ScanCode::F9 => "f9",
                ScanCode::F10 => "f10",
                ScanCode::F11 => "f11",
                ScanCode::F12 => "f12",
                ScanCode::F13 => "f13",
                ScanCode::F14 => "f14",
                ScanCode::F15 => "f15",
                ScanCode::F16 => "f16",
                ScanCode::F17 => "f17",
                ScanCode::F18 => "f18",
                ScanCode::F19 => "f19",
                ScanCode::F20 => "f20",
                ScanCode::F21 => "f21",
                ScanCode::F22 => "f22",
                ScanCode::F23 => "f23",
                ScanCode::F24 => "f24",
                ScanCode::Left => "left",
                ScanCode::Up => "up",
                ScanCode::Right => "right",
                ScanCode::Down => "down",
                ScanCode::PageUp => "pageup",
                ScanCode::PageDown => "pagedown",
                ScanCode::End => "end",
                ScanCode::Home => "home",
                ScanCode::Tab => "tab",
                ScanCode::Enter => "enter",
                ScanCode::Escape => "escape",
                ScanCode::Space => "space",
                ScanCode::Backspace => "backspace",
                ScanCode::Delete => "delete",
                ScanCode::Insert => "insert",
                _ => return None,
            }
            .to_string(),
        )
    }

    /// This function is used to convert the scan code to its key face on US keyboard layout.
    /// Test only.
    #[cfg(test)]
    pub fn to_key(&self) -> &str {
        match self {
            ScanCode::F1 => "f1",
            ScanCode::F2 => "f2",
            ScanCode::F3 => "f3",
            ScanCode::F4 => "f4",
            ScanCode::F5 => "f5",
            ScanCode::F6 => "f6",
            ScanCode::F7 => "f7",
            ScanCode::F8 => "f8",
            ScanCode::F9 => "f9",
            ScanCode::F10 => "f10",
            ScanCode::F11 => "f11",
            ScanCode::F12 => "f12",
            ScanCode::F13 => "f13",
            ScanCode::F14 => "f14",
            ScanCode::F15 => "f15",
            ScanCode::F16 => "f16",
            ScanCode::F17 => "f17",
            ScanCode::F18 => "f18",
            ScanCode::F19 => "f19",
            ScanCode::F20 => "f20",
            ScanCode::F21 => "f21",
            ScanCode::F22 => "f22",
            ScanCode::F23 => "f23",
            ScanCode::F24 => "f24",
            ScanCode::A => "a",
            ScanCode::B => "b",
            ScanCode::C => "c",
            ScanCode::D => "d",
            ScanCode::E => "e",
            ScanCode::F => "f",
            ScanCode::G => "g",
            ScanCode::H => "h",
            ScanCode::I => "i",
            ScanCode::J => "j",
            ScanCode::K => "k",
            ScanCode::L => "l",
            ScanCode::M => "m",
            ScanCode::N => "n",
            ScanCode::O => "o",
            ScanCode::P => "p",
            ScanCode::Q => "q",
            ScanCode::R => "r",
            ScanCode::S => "s",
            ScanCode::T => "t",
            ScanCode::U => "u",
            ScanCode::V => "v",
            ScanCode::W => "w",
            ScanCode::X => "x",
            ScanCode::Y => "y",
            ScanCode::Z => "z",
            ScanCode::Digit0 => "0",
            ScanCode::Digit1 => "1",
            ScanCode::Digit2 => "2",
            ScanCode::Digit3 => "3",
            ScanCode::Digit4 => "4",
            ScanCode::Digit5 => "5",
            ScanCode::Digit6 => "6",
            ScanCode::Digit7 => "7",
            ScanCode::Digit8 => "8",
            ScanCode::Digit9 => "9",
            ScanCode::Backquote => "`",
            ScanCode::Minus => "-",
            ScanCode::Equal => "=",
            ScanCode::BracketLeft => "[",
            ScanCode::BracketRight => "]",
            ScanCode::Backslash => "\\",
            ScanCode::Semicolon => ";",
            ScanCode::Quote => "'",
            ScanCode::Comma => ",",
            ScanCode::Period => ".",
            ScanCode::Slash => "/",
            ScanCode::Left => "left",
            ScanCode::Up => "up",
            ScanCode::Right => "right",
            ScanCode::Down => "down",
            ScanCode::PageUp => "pageup",
            ScanCode::PageDown => "pagedown",
            ScanCode::End => "end",
            ScanCode::Home => "home",
            ScanCode::Tab => "tab",
            ScanCode::Enter => "enter",
            ScanCode::Escape => "escape",
            ScanCode::Space => "space",
            ScanCode::Backspace => "backspace",
            ScanCode::Delete => "delete",
            ScanCode::Insert => "insert",
        }
    }
}
