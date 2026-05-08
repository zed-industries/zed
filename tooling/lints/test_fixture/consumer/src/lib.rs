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

// ---- owned_string_into_shared cases targeting `SharedString` ----

// Should fire (owned_string_into_shared): `String::from(<lit>).into()`.
pub fn shared_string_from_string_from() -> SharedString {
    String::from("label").into()
}

// Should fire (owned_string_into_shared): `<lit>.to_string().into()`.
pub fn shared_string_from_to_string() -> SharedString {
    "label".to_string().into()
}

// Should fire (owned_string_into_shared): `<lit>.to_owned().into()`.
pub fn shared_string_from_to_owned() -> SharedString {
    "label".to_owned().into()
}

// Should NOT fire owned_string_into_shared: the source is a non-literal `String`.
pub fn shared_string_from_dynamic_string(s: String) -> SharedString {
    s.into()
}
