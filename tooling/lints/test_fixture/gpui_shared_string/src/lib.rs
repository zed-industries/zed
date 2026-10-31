//! Minimal stand-in for the real `gpui_shared_string` crate. The lint keys
//! off the crate name and the type name `SharedString`, so we only need to
//! reproduce those.

#[derive(Clone)]
pub struct SharedString(String);

impl SharedString {
    pub const fn new_static(s: &'static str) -> Self {
        // The real implementation stores the `'static` pointer; we just wrap
        // an empty String at compile time to keep this `const`.
        let _ = s;
        SharedString(String::new())
    }

    pub fn new(s: impl AsRef<str>) -> Self {
        SharedString(s.as_ref().to_owned())
    }
}

impl From<&str> for SharedString {
    fn from(s: &str) -> Self {
        SharedString(s.to_owned())
    }
}

impl From<String> for SharedString {
    fn from(s: String) -> Self {
        SharedString(s)
    }
}
