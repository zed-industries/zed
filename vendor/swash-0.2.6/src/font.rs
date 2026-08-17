use super::cache::CacheKey;
use super::internal::{raw_data, RawFont};
use super::Tag;

/// Reference to the content of a font file.
#[derive(Copy, Clone)]
pub struct FontDataRef<'a> {
    data: &'a [u8],
    len: usize,
}

impl<'a> FontDataRef<'a> {
    /// Creates font data from the specified bytes. Returns `None` if the bytes
    /// cannot trivially be determined to represent a font.
    pub fn new(data: &'a [u8]) -> Option<Self> {
        if !raw_data::is_font(data, 0) && !raw_data::is_collection(data) {
            None
        } else {
            Some(Self {
                data,
                len: raw_data::count(data) as usize,
            })
        }
    }

    /// Returns true if the data represents a font collection.
    pub fn is_collection(&self) -> bool {
        raw_data::is_collection(self.data)
    }

    /// Returns the underlying data.
    pub fn data(&self) -> &'a [u8] {
        self.data
    }

    /// Returns the number of available fonts.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns true if there are no available fonts.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns the font at the specified index.
    pub fn get(&self, index: usize) -> Option<FontRef<'a>> {
        FontRef::from_offset(self.data, raw_data::offset(self.data, index as u32)?)
    }

    /// Returns an iterator over the available fonts.
    pub fn fonts(&self) -> Fonts<'a> {
        Fonts {
            data: *self,
            pos: 0,
        }
    }
}

/// Reference to a font.
///
/// This struct encapsulates the data required to access font resources and
/// uniquely identify the font in various caches. Font files can be organized
/// into collections of multiple fonts, so along with a reference to the actual
/// content of the file, we also store a byte offset to the header of the
/// selected font (although most fonts are not collections so this is almost always
/// zero). Note that internal references in the font are stored relative to the
/// base of the file, so the entire file must be kept in memory and it is an error
/// to slice the data at the offset.
///
/// # Getting started
/// As a primer, let's write a function to read a font file, construct a font reference
/// and print the [`Attributes`](super::Attributes) and all of the associated
/// [`LocalizedString`](super::LocalizedString)s (including names, copyright information
/// and other metadata).
///
/// ```
/// fn print_localized_strings(font_path: &str) -> Option<()> {
///     use swash::FontRef;
///     // Read the full font file
///     let font_data = std::fs::read(font_path).ok()?;
///     // Create a font reference for the first font in the file
///     let font = FontRef::from_index(&font_data, 0)?;
///     // Print the font attributes (stretch, weight and style)
///     println!("{}", font.attributes());
///     // Iterate through the localized strings
///     for string in font.localized_strings() {
///         // Print the string identifier and the actual value
///         println!("[{:?}] {}", string.id(), string.to_string());
///     }
///     Some(())
/// }
/// ```
///
/// # Owning your fonts
/// The [`FontRef`] struct is designed to be agnostic with regard to the font management
/// policy of higher level crates and applications, and as such, contains borrowed
/// data and is intended to be used as a transient resource. If you're using this
/// library, you'll likely want to create your own type to represent a font.
/// Regardless of the complexity of your management strategy, the basic pattern remains
/// the same, so we'll build a simple `Font` struct here that can load fonts from a
/// file using a basic `Vec<u8>` as a backing store.
/// ```
/// use swash::{Attributes, CacheKey, Charmap, FontRef};
///
/// pub struct Font {
///     // Full content of the font file
///     data: Vec<u8>,
///     // Offset to the table directory
///     offset: u32,
///     // Cache key
///     key: CacheKey,
/// }
///
/// impl Font {
///     pub fn from_file(path: &str, index: usize) -> Option<Self> {
///         // Read the full font file
///         let data = std::fs::read(path).ok()?;
///         // Create a temporary font reference for the first font in the file.
///         // This will do some basic validation, compute the necessary offset
///         // and generate a fresh cache key for us.
///         let font = FontRef::from_index(&data, index)?;
///         let (offset, key) = (font.offset, font.key);
///         // Return our struct with the original file data and copies of the
///         // offset and key from the font reference
///         Some(Self { data, offset, key })
///     }
///
///     // As a convenience, you may want to forward some methods.
///     pub fn attributes(&self) -> Attributes {
///         self.as_ref().attributes()
///     }
///
///     pub fn charmap(&self) -> Charmap {
///         self.as_ref().charmap()
///     }
///
///     // Create the transient font reference for accessing this crate's
///     // functionality.
///     pub fn as_ref(&self) -> FontRef {
///         // Note that you'll want to initialize the struct directly here as
///         // using any of the FontRef constructors will generate a new key which,
///         // while completely safe, will nullify the performance optimizations of
///         // the caching mechanisms used in this crate.
///         FontRef {
///             data: &self.data,
///             offset: self.offset,
///             key: self.key
///         }
///     }
/// }
/// ```
/// In the example above, it's trivial to replace the `Vec<u8>` with an
/// `Rc<Vec<u8>>` for a reference counted version or an `Arc<Vec<u8>>` for fonts
/// that are shareable across threads. You may also consider memory mapping
/// your font data, particularly for larger fonts (hello Apple Color Emoji!).
///
#[derive(Copy, Clone)]
pub struct FontRef<'a> {
    /// Full content of a file containing the font.
    pub data: &'a [u8],
    /// Offset to the table directory of the font.
    pub offset: u32,
    /// Key for identifying a font in various caches.
    pub key: CacheKey,
}

impl<'a> FontRef<'a> {
    /// Creates a new font from the specified font data and the index of the
    /// desired font. Returns `None` if the data does not represent a font file
    /// or the index is out of bounds.
    pub fn from_index(data: &'a [u8], index: usize) -> Option<Self> {
        FontDataRef::new(data)?.get(index)
    }

    /// Creates a new font from the specified font data and offset to the
    /// table directory. Returns `None` if the offset is out of bounds or the
    /// data at the offset does not represent a table directory.
    pub fn from_offset(data: &'a [u8], offset: u32) -> Option<Self> {
        if !raw_data::is_font(data, offset) {
            None
        } else {
            Some(Self {
                data,
                offset,
                key: CacheKey::new(),
            })
        }
    }
}

impl<'a> RawFont<'a> for FontRef<'a> {
    fn data(&self) -> &'a [u8] {
        self.data
    }

    fn offset(&self) -> u32 {
        self.offset
    }
}

/// Iterator over a collection of fonts.
pub struct Fonts<'a> {
    data: FontDataRef<'a>,
    pos: usize,
}

impl<'a> Iterator for Fonts<'a> {
    type Item = FontRef<'a>;

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.data.len - self.pos;
        (remaining, Some(remaining))
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let pos = self.pos.checked_add(n)?;
        self.pos = pos.checked_add(1)?;
        self.data.get(pos)
    }

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.data.len {
            None
        } else {
            let pos = self.pos;
            self.pos += 1;
            self.data.get(pos)
        }
    }
}

impl<'a> ExactSizeIterator for Fonts<'a> {
    fn len(&self) -> usize {
        self.data.len - self.pos
    }
}

impl<'a> IntoIterator for FontDataRef<'a> {
    type IntoIter = Fonts<'a>;
    type Item = FontRef<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.fonts()
    }
}

/// Source that can provide table data by tag.
pub trait TableProvider {
    /// Returns the table for the specified tag.
    fn table_by_tag(&self, tag: Tag) -> Option<&[u8]>;
}

impl<'a> TableProvider for FontRef<'a> {
    fn table_by_tag(&self, tag: Tag) -> Option<&[u8]> {
        self.table_data(tag)
    }
}

impl<'a> TableProvider for &'a FontRef<'a> {
    fn table_by_tag(&self, tag: Tag) -> Option<&[u8]> {
        self.table_data(tag)
    }
}
