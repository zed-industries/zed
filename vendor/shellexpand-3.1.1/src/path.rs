//! Expansion of [`Path`] values
//!
//! These functions are the same as the ones in the crate toplevel,
//! except that they take [`Path`]s as input and return `Cow<Path>`.
//!
//! (Note that the individual doc comments and examples still refer to
//! `str` and `String` and so on - please refer to the actual types.
//! The semantics are as described.)

use std::ffi::OsString;
use std::path::Path;

use bstr::ByteSlice as _;

#[path="wtraits.rs"]
pub(crate) mod wtraits;
use wtraits::*;

#[path="funcs.rs"]
pub(crate) mod funcs;
pub use funcs::*;

use os_str_bytes::RawOsStr;

type Xstr = std::path::Path;

pub(crate) type WInput<'x> = Cow<'x, RawOsStr>;

pub(crate) type Wstr = RawOsStr;

pub(crate) type OString = OsString;

impl XstrRequirements for Path {
}

impl<P: AsRef<Path> + ?Sized> AsRefXstrExt for P {
    fn into_winput(&self) -> Cow<'_, RawOsStr> { RawOsStr::new(self.as_ref().as_os_str()) }
    fn into_ocow(&self) -> Cow<'_, Path> { self.as_ref().into() }
}

impl WstrExt for RawOsStr {
    fn as_str(&self) -> Option<&str> { self.to_str() }
    fn len(&self) -> usize { self.raw_len() }
    fn to_ostring(&self) -> OsString { self.to_os_str().into_owned() }
    fn strip_prefix(&self, c: char) -> Option<&Self> { self.strip_prefix(c) }
}

impl<'s> WstrRefExt for &'s RawOsStr {
    type Chars = bstr::Chars<'s>;

    /// This is quite approximate, really.
    ///
    /// os_str_bytes says the encoding is "compatible with" UTF-8, and that splitting on UTF-8
    /// characters yields valid substrings.
    ///
    /// We assume, without justification, that the characters we care about handling correctly
    /// in un-{} $var expansions, are represented closely enough that this works.
    ///
    /// On Unix-using-UTF-8 this will be true because it's all, in fact, Unicode.
    /// On other Unix, at least ASCII will work right.
    /// On Windows things that use surrogates will possibly go wrong?
    /// On platforms where this is some mutant form of EBCDIC or something, this will be hopeless.
    fn chars_approx(self) -> bstr::Chars<'s> {
        self.as_raw_bytes().chars()
    }

}

impl CharsExt for bstr::Chars<'_> {
    fn wstr_len(&self) -> usize {
        self.as_bytes().len()
    }
}

impl OStringExt for OsString {
    fn push_str(&mut self, x: &str) { self.push(x) }
    fn push_wstr(&mut self, x: &RawOsStr) { self.push(x.to_os_str()) }
    fn push_xstr(&mut self, x: &Path) { self.push(x.as_os_str()) }
    fn into_ocow(self) -> Cow<'static, Path> { PathBuf::from(self).into() }

    fn display_possibly_lossy(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.to_string_lossy())
    }
}

impl PathBufExt for PathBuf {
    fn try_into_xstring(self) -> Option<PathBuf> { Some(self) }
}
