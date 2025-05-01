/// Scan codes for the keyboard, which are used to identify keys in a keyboard layout-independent way.
/// Currently, we only support a limited set of scan codes here:
/// https://code.visualstudio.com/docs/configure/keybindings#_keyboard-layoutindependent-bindings
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
