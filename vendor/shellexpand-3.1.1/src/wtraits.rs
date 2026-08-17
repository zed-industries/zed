//! Type aliases and trait definitions for internal traits - **instantiated twice**
//!
//! Like `funcs`, this is instantiated twice, as
//! [`crate::strings::wtraits`] and [`crate::path::wtraits`].
//!
//! He type aliases, and even traits, here,
//! are all different, in the two contexts.

#![cfg_attr(not(feature = "full"), allow(dead_code))]

pub(crate) use std::borrow::Cow;
pub(crate) use std::fmt;
pub(crate) use std::path::PathBuf;

/// External borrowed form; inputs are generally of type `AsRef<Xstr>`
///
/// `str` for strings; [`Path`](std::path::Path) for paths.
pub(crate) type Xstr = super::Xstr;

/// Working, possibly-owned, form of the input
///
/// Usually converted from `AsRef<`[`Xstr`]`>`,
/// using [`.into_winput()`](AsRefXstrExt::into_winput).
///
/// For strings, this is simply `&'s str`; 
///
/// For paths, this is `Cow<'s, RawOsStr>`.
/// This is because it can be necessary to copy-convert the input
/// to a different representation.
/// (This happens only on Windows, whose native character encoding UTF-16, which is deranged;
/// on Unix, I believe this is always `Cow::Borrowed`.)
pub(crate) type WInput<'s> = super::WInput<'s>;

/// Working, definitely-borrowed, form of the input
///
/// Most input-processing, such as slicing up the input, is done with this type.
///
/// `str` for strings, `RawOsStr` for paths.
pub(crate) type Wstr = super::Wstr;

/// Output accumulator (owned)
///
/// As we process input chunks, we add them to this.
///
/// This type doesn't provide an easy way to rescan it again -
/// but we generally only want to scan things once.
///
/// Also used for error reporting, in `LookupError`.
///
/// `String`, or [`OsString`](std::ffi::OsString).
pub(crate) type OString = super::OString;

/// Owned version of Xstr
///
/// `String`, or `PathBuf`.
///
/// Currently used only as the return value from `home_dir()`.
pub(crate) type XString = <Xstr as ToOwned>::Owned;

/// Extra bounds on [`Xstr`] (and hence, restrictions on [`XString`])
///
/// This is implemented only for the single type `Xstr` (ie, for `str`, or `Path`).
///
/// The bound `str: AsRef<self>`
/// declares that we can cheaply borrow a normal string as an `Xstr`.
///
/// The bound `PathBuf: `[`PathBufExt`]
/// declares that
/// `Pathbuf.`[`try_into_xstring()`](PathBufExt::try_into_xstring) is available -
/// ie, that you can try to obtain an `XString` from a `PathBuf`,
/// which is relevant for home directories and tilde substitution.
///
/// These bounds (and therefore this trait), including the supertrait,
/// are not needed for the code to compile,
/// since we don't have significant amounts of generic code.
/// But they serve as compiler-checked documentation.
pub(crate) trait XstrRequirements: AsRefXstrExt where str: AsRef<Self>, PathBuf: PathBufExt {
}

/// Methods on `AsRef<`[`Xstr`]`>`
///
/// These methods are used by the implementation in `funcs`
/// to convert the input into the working form,
/// or, to be able to return it unmodified.
///
/// This is implemented (only) for `S: AsRef<Xstr>`.
pub(crate) trait AsRefXstrExt {
    /// Convert an input string into our working form
    fn into_winput(&self) -> WInput;

    /// Convert an unmodified input string back into the public output type
    ///
    /// Called when inspection of the string tells us we didn't need to make any substitutions.
    fn into_ocow(&self) -> Cow<'_, Xstr>;
}

/// Extension trait on `PathBuf`
///
/// Implemented separately by the code for
/// strings (returning `Option<String>`)
/// and paths (returning `Option<PathBuf>`)
pub(crate) trait PathBufExt {
    /// Converts a `PathBuf` into an `XString` (input string type), if possible
    ///
    /// We might not be able to represent a non-Unicode path.
    /// In that case, this function returns `None`.
    fn try_into_xstring(self) -> Option<XString>;
}

/// Methods (and bounds) on `Wstr`
///
/// `funcs` may also use inherent methods, including slicing,
/// provided they are implemented for both `str` and `RawOsString`,
/// with suitable semantics.
///
/// The bound declares that `&str` or `&RawOsStr` implements `WstrRefExt`
/// so that [`.chars_approx()`](WstrRefExt::chars_approx) is available.
pub(crate) trait WstrExt: where for <'s> &'s Self: WstrRefExt {
    /// Turn something which maybe derefs to a `Wstr` into a `Wstr`
    ///
    /// This allows us to use method call autoderef.
    ///
    /// We want this at call sites that have [`WInput`],
    /// which don't otherwise know whether to deref:
    /// for strings, `WInput` is `&str`,
    /// and is therefore already a `&Wstr` (which is also `&str`).
    /// whereas for paths, `WInput` is `Cow<Path>` which derefs to `&Wstr` (ie `&Path`).
    fn as_wstr(&self) -> &Self { self }

    /// Get this bit of the input as a plain string, if it is Unicode
    ///
    /// Used for variable name lookups.  Our variable lookup API passes Unicode variable names
    /// to the caller.
    /// (Any non-Unicode we find is therefore, by definition, an undefined variable).
    fn as_str(&self) -> Option<&str>;

    /// Length, in a form suitable for slicing
    ///
    /// Calling this `len` is only reasonable because the `Wstr` type either
    /// doesn't have a `len` method of its own (true of `RawOsStr`, for paths),
    /// or has one which works the same (true of `str`, for strings).
    fn len(&self) -> usize;

    /// Convert to an `OString`, as used for error reporting etc.
    fn to_ostring(&self) -> OString;

    /// Like `str::strip_prefix`, but available on our str MSRV 1.31
    fn strip_prefix(&self, c: char) -> Option<&Self>;
}

/// Method on the reference [`&Wstr`](Wstr)
///
/// This can't be in the main [`WstrExt`] trait because
/// the type `Chars` would have a lifetime -
/// ie it would be a GAT, which is very new in Rust and we don't want to rely on.
pub(crate) trait WstrRefExt {
    /// Iterator over characters
    type Chars: Iterator<Item=char> + CharsExt;

    /// Iterate over, roughly, characters
    ///
    /// This is not guaranteed to work very well in the presence of non-Unicode input,
    /// or strange encodings.
    ///
    /// The properties we rely on are:
    ///
    ///  * Where the input contains ASCII characters `$` `{` `}` `~` `/` `\`,
    ///    they come out of the iterator in the right place.
    ///
    ///  * Where the input contains characters that are found in variable names the user
    ///    wants to substitute, they come out properly in the right place,
    ///    and they are recognised as Unicode letters.
    ///
    ///  * The return value from [`.wstr_len()`](CharsExt::wstr_len)
    ///    can be used in slicing calculations.
    fn chars_approx(self) -> Self::Chars;
}

/// Methods on the characters iterator from [`Wstr.chars_approx()`](WstrRefExt::chars_approx)
pub(crate) trait CharsExt {
    fn wstr_len(&self) -> usize;
}

/// Methods on [`OString`]
///
/// `funcs` may also use inherent methods,
/// provided they are implemented for both `String` and `RawOsString`,
/// with suitable semantics.
pub(crate) trait OStringExt {
    /// Append a plain `str` (used for literal text)
    fn push_str(&mut self, x: &str);

    /// Append a `Wstr` (used for copying through pieces of the input)
    fn push_wstr(&mut self, x: &Wstr);

    /// Append an `Xstr` (used for copying through variable lookup results)
    fn push_xstr(&mut self, x: &Xstr);

    /// Convert an output string we have been accumulating into the public output type
    fn into_ocow(self) -> Cow<'static, Xstr>;

    /// Write this input string in a possibly-lossy way, for use in error messages
    fn display_possibly_lossy(&self, f: &mut fmt::Formatter) -> fmt::Result;
}
