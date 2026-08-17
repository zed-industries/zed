//! A byte-offset based string table.
//! Commonly used in ELF binaries, Unix archives, and even PE binaries.

use core::fmt;
use core::ops::Index;
use core::str;
use scroll::{ctx, Pread};
if_alloc! {
    use crate::error;
    use alloc::vec::Vec;
}

/// A common string table format which is indexed by byte offsets (and not
/// member index). Constructed using [`parse`](#method.parse)
/// with your choice of delimiter. Please be careful.
pub struct Strtab<'a> {
    delim: ctx::StrCtx,
    bytes: &'a [u8],
    #[cfg(feature = "alloc")]
    strings: Vec<(usize, &'a str)>,
}

#[inline(always)]
fn get_str(offset: usize, bytes: &[u8], delim: ctx::StrCtx) -> scroll::Result<&str> {
    bytes.pread_with::<&str>(offset, delim)
}

impl<'a> Strtab<'a> {
    /// Creates a `Strtab` with `bytes` as the backing string table, using `delim` as the delimiter between entries.
    ///
    /// NB: this does *not* preparse the string table, which can have non-optimal access patterns.
    /// See <https://github.com/m4b/goblin/pull/275>
    pub fn new(bytes: &'a [u8], delim: u8) -> Self {
        Self::from_slice_unparsed(bytes, 0, bytes.len(), delim)
    }

    /// Returns the length of this `Strtab` in bytes
    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    /// Creates a `Strtab` directly without bounds check and without parsing it.
    ///
    /// This is potentially unsafe and should only be used if `feature = "alloc"` is disabled.
    pub fn from_slice_unparsed(bytes: &'a [u8], offset: usize, len: usize, delim: u8) -> Self {
        Self {
            delim: ctx::StrCtx::Delimiter(delim),
            bytes: &bytes[offset..offset + len],
            #[cfg(feature = "alloc")]
            strings: Vec::new(),
        }
    }
    /// Gets a str reference from the backing bytes starting at byte `offset`.
    ///
    /// If the index is out of bounds, `None` is returned. Panics if bytes are invalid UTF-8.
    /// Use this method if the `Strtab` was created using `from_slice_unparsed()`.
    pub fn get_unsafe(&self, offset: usize) -> Option<&'a str> {
        if offset >= self.bytes.len() {
            None
        } else {
            Some(get_str(offset, self.bytes, self.delim).unwrap())
        }
    }
    #[cfg(feature = "alloc")]
    /// Parses a `Strtab` from `bytes` at `offset` with `len` size as the backing string table, using `delim` as the delimiter.
    ///
    /// Errors if bytes are invalid UTF-8.
    /// Requires `feature = "alloc"`
    pub fn parse(bytes: &'a [u8], offset: usize, len: usize, delim: u8) -> error::Result<Self> {
        let (end, overflow) = offset.overflowing_add(len);
        if overflow || end > bytes.len() {
            return Err(error::Error::Malformed(format!(
                "Strtable size ({}) + offset ({}) is out of bounds for {} #bytes. Overflowed: {}",
                len,
                offset,
                bytes.len(),
                overflow
            )));
        }
        let mut result = Self::from_slice_unparsed(bytes, offset, len, delim);
        let mut i = 0;
        while i < result.bytes.len() {
            let string = get_str(i, result.bytes, result.delim)?;
            result.strings.push((i, string));
            i += string.len() + 1;
        }
        Ok(result)
    }
    #[cfg(feature = "alloc")]
    /// Parses a `Strtab` with `bytes` as the backing string table, using `delim` as the delimiter between entries.
    ///
    /// Requires `feature = "alloc"`
    pub fn new_preparsed(bytes: &'a [u8], delim: u8) -> error::Result<Self> {
        Self::parse(bytes, 0, bytes.len(), delim)
    }
    #[cfg(feature = "alloc")]
    /// Converts the string table to a vector of parsed strings.
    ///
    /// Note: This method is used to check the parsed contents of `strtab`.
    /// If you want to get the correct contents of `strtab` as `Vec`, use the following example.
    ///
    /// # Examples
    /// ```rust
    /// use goblin::error::Error;
    ///
    /// pub fn show_shdr_strtab(bytes: &[u8]) -> Result<(), Error> {
    ///     let elf = goblin::elf::Elf::parse(&bytes)?;
    ///
    ///     for section in elf.section_headers {
    ///         println!("{}", elf.shdr_strtab.get_at(section.sh_name).unwrap_or(""));
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// Requires `feature = "alloc"`
    pub fn to_vec(&self) -> error::Result<Vec<&'a str>> {
        // Fallback in case `Strtab` was created using `from_slice_unparsed()`.
        if self.strings.is_empty() {
            let mut result = Vec::new();
            let mut i = 0;
            while i < self.bytes.len() {
                let string = get_str(i, self.bytes, self.delim)?;
                result.push(string);
                i += string.len() + 1;
            }
            return Ok(result);
        }
        Ok(self.strings.iter().map(|&(_key, value)| value).collect())
    }
    #[cfg(feature = "alloc")]
    /// Safely gets a str reference from the parsed table starting at byte `offset`.
    ///
    /// If the index is out of bounds, `None` is returned.
    /// Requires `feature = "alloc"`
    pub fn get_at(&self, offset: usize) -> Option<&'a str> {
        match self
            .strings
            .binary_search_by_key(&offset, |&(key, _value)| key)
        {
            Ok(index) => Some(self.strings[index].1),
            Err(index) => {
                if index == 0 {
                    return None;
                }
                let (string_begin_offset, entire_string) = self.strings[index - 1];
                entire_string.get(offset - string_begin_offset..)
            }
        }
    }
    #[deprecated(since = "0.4.2", note = "Use from_slice_unparsed() instead")]
    /// Construct a strtab from a `ptr`, and a `size`, using `delim` as the delimiter
    ///
    /// # Safety
    /// This function creates a `Strtab` directly from a raw pointer and size
    pub unsafe fn from_raw(ptr: *const u8, len: usize, delim: u8) -> Strtab<'a> {
        Self::from_slice_unparsed(core::slice::from_raw_parts(ptr, len), 0, len, delim)
    }
    #[deprecated(since = "0.4.2", note = "Bad performance, use get_at() instead")]
    #[cfg(feature = "alloc")]
    /// Parses a str reference from the parsed table starting at byte `offset`.
    ///
    /// If the index is out of bounds, `None` is returned.
    /// Requires `feature = "alloc"`
    pub fn get(&self, offset: usize) -> Option<error::Result<&'a str>> {
        if offset >= self.bytes.len() {
            None
        } else {
            Some(get_str(offset, self.bytes, self.delim).map_err(core::convert::Into::into))
        }
    }
}

impl<'a> fmt::Debug for Strtab<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Strtab")
            .field("delim", &self.delim)
            .field("bytes", &str::from_utf8(self.bytes))
            .finish()
    }
}

impl<'a> Default for Strtab<'a> {
    fn default() -> Self {
        Self {
            delim: ctx::StrCtx::default(),
            bytes: &[],
            #[cfg(feature = "alloc")]
            strings: Vec::new(),
        }
    }
}

impl<'a> Index<usize> for Strtab<'a> {
    type Output = str;
    /// Gets str reference at starting at byte `offset`.
    /// **NB**: this will panic if the underlying bytes are not valid utf8, or the offset is invalid
    #[inline(always)]
    fn index(&self, offset: usize) -> &Self::Output {
        // This can't delegate to get() because get() requires #[cfg(features = "alloc")]
        // It's also slightly less useful than get() because the lifetime -- specified by the Index
        // trait -- matches &self, even though we could return &'a instead
        get_str(offset, self.bytes, self.delim).unwrap()
    }
}

#[test]
fn as_vec_no_final_null() {
    let strtab = Strtab::new_preparsed(b"\0printf\0memmove\0busta", 0x0).unwrap();
    let vec = strtab.to_vec().unwrap();
    assert_eq!(vec.len(), 4);
    assert_eq!(vec, vec!["", "printf", "memmove", "busta"]);
}

#[test]
fn as_vec_no_first_null_no_final_null() {
    let strtab = Strtab::new_preparsed(b"printf\0memmove\0busta", 0x0).unwrap();
    let vec = strtab.to_vec().unwrap();
    assert_eq!(vec.len(), 3);
    assert_eq!(vec, vec!["printf", "memmove", "busta"]);
}

#[test]
fn to_vec_final_null() {
    let strtab = Strtab::new_preparsed(b"\0printf\0memmove\0busta\0", 0x0).unwrap();
    let vec = strtab.to_vec().unwrap();
    assert_eq!(vec.len(), 4);
    assert_eq!(vec, vec!["", "printf", "memmove", "busta"]);
}

#[test]
fn to_vec_newline_delim() {
    let strtab = Strtab::new_preparsed(b"\nprintf\nmemmove\nbusta\n", b'\n').unwrap();
    let vec = strtab.to_vec().unwrap();
    assert_eq!(vec.len(), 4);
    assert_eq!(vec, vec!["", "printf", "memmove", "busta"]);
}

#[test]
fn parse_utf8() {
    assert!(match Strtab::new_preparsed(&[0x80, 0x80], b'\n') {
        Err(error::Error::Scroll(scroll::Error::BadInput {
            size: 2,
            msg: "invalid utf8",
        })) => true,
        _ => false,
    });
    assert!(
        match Strtab::new_preparsed(&[0xC6, 0x92, 0x6F, 0x6F], b'\n') {
            Ok(_) => true,
            _ => false,
        }
    );
}

#[test]
fn get_at_utf8() {
    let strtab = Strtab::new_preparsed("\nƒoo\nmemmove\n🅱️usta\n".as_bytes(), b'\n').unwrap();
    assert_eq!(strtab.get_at(0), Some(""));
    assert_eq!(strtab.get_at(5), Some(""));
    assert_eq!(strtab.get_at(6), Some("memmove"));
    assert_eq!(strtab.get_at(14), Some("\u{1f171}\u{fe0f}usta"));
    assert_eq!(strtab.get_at(16), None);
    assert_eq!(strtab.get_at(18), Some("\u{fe0f}usta"));
    assert_eq!(strtab.get_at(21), Some("usta"));
    assert_eq!(strtab.get_at(25), Some(""));
    assert_eq!(strtab.get_at(26), None);
}
