use gpui_shared_string::SharedString;

// Should fire: `from` with a short literal (≤ 23 bytes).
pub fn from_short() -> SharedString {
    SharedString::from("Favorites")
}

// Should fire at elevated severity: `from` with a long literal (> 23 bytes).
pub fn from_long() -> SharedString {
    SharedString::from("Right-click for more options")
}

// Should fire: explicit `.into()` from a string literal.
pub fn into_short() -> SharedString {
    "hello".into()
}

// Should fire: `SharedString::new("...")`.
pub fn new_short() -> SharedString {
    SharedString::new("hi")
}

// Should NOT fire: the zero-cost constructor.
pub fn new_static_short() -> SharedString {
    SharedString::new_static("Favorites")
}

// Should NOT fire: non-literal input.
pub fn from_variable(s: &str) -> SharedString {
    SharedString::from(s)
}

// Should NOT fire: `.into()` on a non-literal.
pub fn into_variable(s: &str) -> SharedString {
    s.into()
}

// Should fire: `.into()` on a string literal that exceeds the 23-byte cap.
pub fn into_long() -> SharedString {
    "this literal is definitely longer than twenty three bytes".into()
}
