
use std::fmt::Display;
use std::str::Chars;

#[path="wtraits.rs"]
pub mod wtraits;
use self::wtraits::*;

#[path="funcs.rs"]
pub mod funcs;

pub type Xstr = str;

pub type WInput<'x> = &'x str;

pub type Wstr = str;

pub type OString = String;

impl XstrRequirements for str {
}

impl<S: AsRef<str> + ?Sized> AsRefXstrExt for S {
    fn into_winput(&self) -> &str { self.as_ref() }
    fn into_ocow(&self) -> Cow<'_, str> { self.as_ref().into() }
}

impl WstrExt for str {
    fn as_str(&self) -> Option<&str> { Some(self) }
    fn len(&self) -> usize { str::len(self) }
    fn to_ostring(&self) -> String { self.to_owned() }
    fn strip_prefix(&self, c: char) -> Option<&Self> {
        if self.starts_with(c) {
            Some(&self[c.len_utf8()..])
        } else {
            None
        }
    }
}

impl<'s> WstrRefExt for &'s str {
    type Chars = Chars<'s>;

    /// Must be used only for the-{}-unbracketed $varname expansion variable name termination detection
    ///
    /// The implementation for `paths.rs` is ... limited.
    fn chars_approx(self) -> Chars<'s> {
        self.chars()
    }
}

impl CharsExt for Chars<'_> {
    fn wstr_len(&self) -> usize {
        self.as_str().len()
    }
}

impl OStringExt for String {
    fn push_str(&mut self, x: &str) { self.push_str(x) }
    fn push_wstr(&mut self, x: &str) { self.push_str(x) }
    fn push_xstr(&mut self, x: &str) { self.push_str(x) }
    fn into_ocow(self) -> Cow<'static, str> { self.into() }

    fn display_possibly_lossy(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(self, f)
    }
}

impl PathBufExt for PathBuf {
    fn try_into_xstring(self) -> Option<String> {
        self.into_os_string().into_string().ok()
    }
}
