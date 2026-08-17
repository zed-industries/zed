//! Bindings for [`AFont`], [`AFontMatcher`], and [`ASystemFontIterator`]
//!
//! [`AFont`]: https://developer.android.com/ndk/reference/group/font
//! [`AFontMatcher`]: https://developer.android.com/ndk/reference/group/font#afontmatcher_create
//! [`ASystemFontIterator`]: https://developer.android.com/ndk/reference/group/font#asystemfontiterator_open

#![cfg(feature = "api-level-29")]

use std::convert::TryFrom;
use std::ffi::{CStr, OsStr};
use std::fmt::{self, Write};
use std::os::unix::prelude::OsStrExt;
use std::path::Path;
use std::ptr::NonNull;

use num_enum::IntoPrimitive;

/// An integer holding a valid font weight value between 1 and 1000.
///
/// See the [`Font::weight`] definition for more details.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct FontWeight(u16);

impl FontWeight {
    pub const fn new(value: u16) -> Result<Self, FontWeightValueError> {
        if Self::MIN.0 <= value && value <= Self::MAX.0 {
            Ok(Self(value))
        } else {
            Err(FontWeightValueError(()))
        }
    }

    pub const fn to_u16(self) -> u16 {
        self.0
    }

    /// The minimum value for the font weight value. Unlike [`ffi::AFONT_WEIGHT_MIN`] being `0`,
    /// [`FontWeight::MIN`] is `1` to make the `MIN..MAX` range be inclusive, keeping consistency
    /// between [`FontWeight`] and other types like `std::num::NonZeroU*`.
    pub const MIN: FontWeight = FontWeight(ffi::AFONT_WEIGHT_MIN as u16 + 1);

    /// A font weight value for the thin weight.
    pub const THIN: FontWeight = FontWeight(ffi::AFONT_WEIGHT_THIN as u16);

    /// A font weight value for the extra-light weight.
    pub const EXTRA_LIGHT: FontWeight = FontWeight(ffi::AFONT_WEIGHT_EXTRA_LIGHT as u16);

    /// A font weight value for the light weight.
    pub const LIGHT: FontWeight = FontWeight(ffi::AFONT_WEIGHT_LIGHT as u16);

    /// A font weight value for the normal weight.
    pub const NORMAL: FontWeight = FontWeight(ffi::AFONT_WEIGHT_NORMAL as u16);

    /// A font weight value for the medium weight.
    pub const MEDIUM: FontWeight = FontWeight(ffi::AFONT_WEIGHT_MEDIUM as u16);

    /// A font weight value for the semi-bold weight.
    pub const SEMI_BOLD: FontWeight = FontWeight(ffi::AFONT_WEIGHT_SEMI_BOLD as u16);

    /// A font weight value for the bold weight.
    pub const BOLD: FontWeight = FontWeight(ffi::AFONT_WEIGHT_BOLD as u16);

    /// A font weight value for the extra-bold weight.
    pub const EXTRA_BOLD: FontWeight = FontWeight(ffi::AFONT_WEIGHT_EXTRA_BOLD as u16);

    /// A font weight value for the black weight.
    pub const BLACK: FontWeight = FontWeight(ffi::AFONT_WEIGHT_BLACK as u16);

    /// The maximum value for the font weight value.
    pub const MAX: FontWeight = FontWeight(ffi::AFONT_WEIGHT_MAX as u16);
}

impl fmt::Display for FontWeight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match *self {
            FontWeight::THIN => "Thin",
            FontWeight::EXTRA_LIGHT => "Extra Light (Ultra Light)",
            FontWeight::LIGHT => "Light",
            FontWeight::NORMAL => "Normal (Regular)",
            FontWeight::MEDIUM => "Medium",
            FontWeight::SEMI_BOLD => "Semi Bold (Demi Bold)",
            FontWeight::BOLD => "Bold",
            FontWeight::EXTRA_BOLD => "Extra Bold (Ultra Bold)",
            FontWeight::BLACK => "Black (Heavy)",
            _ => return writeln!(f, "{}", self.0),
        })
    }
}

/// The error type returned when an invalid font weight value is passed.
#[derive(Debug)]
pub struct FontWeightValueError(());

impl fmt::Display for FontWeightValueError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("font weight must be positive and less than or equal to 1000")
    }
}

impl std::error::Error for FontWeightValueError {}

impl TryFrom<u16> for FontWeight {
    type Error = FontWeightValueError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        FontWeight::new(value)
    }
}

/// A 4-byte integer representing an OpenType axis tag.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct AxisTag(u32);

impl AxisTag {
    /// Checks whether the given 4-byte array can construct a valid axis tag and returns
    /// [`Ok(AxisTag)`] if the array is valid.
    ///
    /// Each byte in a tag must be in the range 0x20 to 0x7E. A space character cannot be followed
    /// by a non-space character. A tag must have one to four non-space characters. See the
    /// [OpenType spec] for more details.
    ///
    /// [OpenType spec]: https://learn.microsoft.com/en-us/typography/opentype/spec/otff#data-types
    pub const fn from_be_bytes_checked(value: [u8; 4]) -> Result<Self, AxisTagValueError> {
        // Each byte in a tag must be in the range 0x20 to 0x7E.
        macro_rules! check_byte_range {
            ($($e:expr)+) => {
                $(
                    if !(value[$e] as char).is_ascii_graphic() && value[$e] != b' ' {
                        return Err(AxisTagValueError::InvalidCharacter);
                    }
                )+
            };
        }
        check_byte_range!(0 1 2 3);

        if value[0] == b' ' {
            return Err(
                if value[1] == b' ' && value[2] == b' ' && value[3] == b' ' {
                    // A tag must have one to four non-space characters.
                    AxisTagValueError::EmptyTag
                } else {
                    // A space character cannot be followed by a non-space character.
                    AxisTagValueError::InvalidSpacePadding
                },
            );
        }

        macro_rules! check_if_valid {
            ($e:expr ; $($f:expr)+) => {
                if value[$e] == b' ' {
                    return if true $(&& value[$f] == b' ')+ {
                        Ok(Self(u32::from_be_bytes(value)))
                    } else {
                        // A space character cannot be followed by a non-space character.
                        Err(AxisTagValueError::InvalidSpacePadding)
                    };
                }
            };
        }

        check_if_valid!(1; 2 3);
        check_if_valid!(2; 3);

        // Whether or not value[3] is b' ', value is a valid axis tag.
        Ok(Self(u32::from_be_bytes(value)))
    }

    /// Checks whether the given 4-byte array can construct a valid axis tag and returns
    /// [`Ok(AxisTag)`] if the array is valid.
    ///
    /// See [`AxisTag::from_be()`] for more details.
    pub const fn from_be_checked(value: u32) -> Result<Self, AxisTagValueError> {
        Self::from_be_bytes_checked(value.to_be_bytes())
    }

    /// Construct an axis tag from the given 4-byte array. If the resulting axis tag is invalid,
    /// this function panics.
    ///
    /// See [`AxisTag::from_be()`] for more details.
    pub const fn from_be_bytes(value: [u8; 4]) -> Self {
        Self::unwrap_result(Self::from_be_bytes_checked(value))
    }

    /// Construct an axis tag from the given 4-byte integer. If the resulting axis tag is invalid,
    /// this function panics.
    ///
    /// See [`AxisTag::from_be()`] for more details.
    pub const fn from_be(value: u32) -> Self {
        Self::unwrap_result(Self::from_be_checked(value))
    }

    /// const-version of [`Result::unwrap`]. Should be removed when [`Option::unwrap`] or
    /// [`Result::unwrap`] become `const`-stable.
    const fn unwrap_result(result: Result<Self, AxisTagValueError>) -> Self {
        match result {
            Ok(t) => t,
            Err(e) => panic!("{}", e.as_str()),
        }
    }

    pub const fn to_u32(self) -> u32 {
        self.0
    }

    pub const fn to_be_bytes(self) -> [u8; 4] {
        self.0.to_be_bytes()
    }
}

impl fmt::Display for AxisTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bytes = self.to_be_bytes();
        f.write_char(bytes[0] as char)?;
        f.write_char(bytes[1] as char)?;
        f.write_char(bytes[2] as char)?;
        f.write_char(bytes[3] as char)
    }
}

impl fmt::Debug for AxisTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AxisTag({} {:#x})", self, self.0)
    }
}

/// The error type returned when an invalid axis tag value is passed.
#[derive(Clone, Copy, Debug)]
pub enum AxisTagValueError {
    /// There is a byte not in the range 0x20 to 0x7E.
    InvalidCharacter,
    /// There is a space character followed by a non-space character.
    InvalidSpacePadding,
    /// The tag only consists of space characters.
    EmptyTag,
}

impl AxisTagValueError {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::InvalidCharacter => "each byte in an axis tag must be in the range 0x20 to 0x7E",
            Self::InvalidSpacePadding => {
                "a space character cannot be followed by a non-space character"
            }
            Self::EmptyTag => "a tag must have one to four non-space characters",
        }
    }
}

impl fmt::Display for AxisTagValueError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str(self.as_str())
    }
}

impl std::error::Error for AxisTagValueError {}

/// A native [`AFont *`]
///
/// [`AFont *`]: https://developer.android.com/ndk/reference/group/font
#[derive(Debug)]
pub struct Font {
    ptr: NonNull<ffi::AFont>,
}

impl Font {
    /// Assumes ownership of `ptr`.
    ///
    /// # Safety
    /// `ptr` must be a valid owning pointer to an Android [`ffi::AFont`].
    pub unsafe fn from_ptr(ptr: NonNull<ffi::AFont>) -> Self {
        Self { ptr }
    }

    /// Returns s the pointer to the native [`ffi::AFont`].
    pub fn ptr(&self) -> NonNull<ffi::AFont> {
        self.ptr
    }

    /// Returns a count of font variation settings associated with the current font.
    ///
    /// The font variation settings are provided as multiple tag-value pairs.
    ///
    /// For example, bold italic font may have following font variation settings: `'wght' 700`,
    /// `'slnt' -12`. In this case, [`Font::axis_count()`] returns `2` and [`Font::axis_tag_at()`] and
    /// [`Font::axis_value_at()`] return those variation names and the corresponding values.
    ///
    /// ```no_run
    /// use ndk::font::Font;
    ///
    /// let font: Font = todo!();
    /// for idx in 0..font.axis_count() {
    ///     log::debug!("{}: {}", font.axis_tag_at(idx), font.axis_value_at(idx));
    /// }
    /// // Output:
    /// // wght: 700
    /// // slnt: -12
    /// ```
    pub fn axis_count(&self) -> usize {
        unsafe { ffi::AFont_getAxisCount(self.ptr.as_ptr()) }
    }

    /// Returns an OpenType axis tag associated with the current font.
    ///
    /// See [`Font::axis_count()`] for more details.
    pub fn axis_tag_at(&self, idx: usize) -> AxisTag {
        // Android returns Axis Tag in big-endian.
        // See https://cs.android.com/android/platform/superproject/+/refs/heads/master:frameworks/base/native/android/system_fonts.cpp;l=197 for details
        AxisTag(unsafe { ffi::AFont_getAxisTag(self.ptr.as_ptr(), idx as u32) })
    }

    /// Returns an OpenType axis value associated with the current font.
    ///
    /// See [`Font::axis_count()`] for more details.
    pub fn axis_value_at(&self, idx: usize) -> f32 {
        unsafe { ffi::AFont_getAxisValue(self.ptr.as_ptr(), idx as u32) }
    }

    /// Returns a font collection index value associated with the current font.
    ///
    /// In case the target font file is a font collection (e.g. `.ttc` or `.otc`), this returns a
    /// non-negative value as a font offset in the collection. This always returns 0 if the target
    /// font file is a regular font.
    pub fn collection_index(&self) -> usize {
        unsafe { ffi::AFont_getCollectionIndex(self.ptr.as_ptr()) }
    }

    /// Returns an absolute path to the current font file.
    ///
    /// Here is a list of font formats returned by this method:
    ///
    /// * OpenType
    /// * OpenType Font Collection
    /// * TrueType
    /// * TrueType Collection
    ///
    /// The file extension could be one of `*.otf`, `*.ttf`, `*.otc` or `*.ttc`.
    /// The font file specified by the returned path is guaranteed to be openable with `O_RDONLY`.
    pub fn path(&self) -> &Path {
        let path = unsafe { CStr::from_ptr(ffi::AFont_getFontFilePath(self.ptr.as_ptr())) };
        OsStr::from_bytes(path.to_bytes()).as_ref()
    }

    /// Returns an IETF BCP47 compliant language tag associated with the current font.
    ///
    /// For information about IETF BCP47, read [`Locale.forLanguageTag(java.lang.String)`].
    ///
    /// [`Locale.forLanguageTag(java.lang.String)`]: https://developer.android.com/reference/java/util/Locale.html#forLanguageTag(java.lang.String)
    pub fn locale(&self) -> Option<&CStr> {
        let ptr = unsafe { ffi::AFont_getLocale(self.ptr.as_ptr()) };
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { CStr::from_ptr(ptr) })
        }
    }

    /// Returns a weight value associated with the current font.
    ///
    /// The weight values are positive and less than or equal to 1000. Here are pairs of the common
    /// names and their values.
    ///
    /// | Value | Name                      | NDK Definition              |
    /// | ----- | ------------------------- | --------------------------- |
    /// | 100   | Thin                      | [`FontWeight::THIN`]        |
    /// | 200   | Extra Light (Ultra Light) | [`FontWeight::EXTRA_LIGHT`] |
    /// | 300   | Light                     | [`FontWeight::LIGHT`]       |
    /// | 400   | Normal (Regular)          | [`FontWeight::NORMAL`]      |
    /// | 500   | Medium                    | [`FontWeight::MEDIUM`]      |
    /// | 600   | Semi Bold (Demi Bold)     | [`FontWeight::SEMI_BOLD`]   |
    /// | 700   | Bold                      | [`FontWeight::BOLD`]        |
    /// | 800   | Extra Bold (Ultra Bold)   | [`FontWeight::EXTRA_BOLD`]  |
    /// | 900   | Black (Heavy)             | [`FontWeight::BLACK`]       |
    pub fn weight(&self) -> FontWeight {
        FontWeight(unsafe { ffi::AFont_getWeight(self.ptr.as_ptr()) })
    }

    /// Returns [`true`] if the current font is italic, otherwise returns [`false`].
    pub fn is_italic(&self) -> bool {
        unsafe { ffi::AFont_isItalic(self.ptr.as_ptr()) }
    }
}

impl Drop for Font {
    fn drop(&mut self) {
        unsafe { ffi::AFont_close(self.ptr.as_ptr()) }
    }
}

/// Corresponds to [`AFAMILY_VARIANT_*`].
///
/// [`AFAMILY_VARIANT_*`]: https://developer.android.com/ndk/reference/group/font#group___font_1gga96a58e29e8dbf2b5bdeb775cba46556ea662aafc7016e35d6758da93416fc0833
#[repr(u32)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, IntoPrimitive)]
#[non_exhaustive]
pub enum FamilyVariant {
    /// A family variant value for the compact font family variant.
    /// The compact font family has Latin-based vertical metrics.
    Compact = ffi::AFAMILY_VARIANT_COMPACT,
    /// A family variant value for the system default variant.
    Default = ffi::AFAMILY_VARIANT_DEFAULT,
    /// A family variant value for the elegant font family variant.
    /// The elegant font family may have larger vertical metrics than Latin font.
    Elegant = ffi::AFAMILY_VARIANT_ELEGANT,

    #[doc(hidden)]
    #[num_enum(catch_all)]
    __Unknown(u32),
}

/// A native [`AFontMatcher *`]
///
/// [`AFontMatcher *`]: https://developer.android.com/ndk/reference/group/font#afontmatcher_create
#[derive(Debug)]
pub struct FontMatcher {
    ptr: NonNull<ffi::AFontMatcher>,
}

impl FontMatcher {
    /// Assumes ownership of `ptr`.
    ///
    /// # Safety
    /// `ptr` must be a valid owning pointer to an Android [`ffi::AFontMatcher`].
    pub unsafe fn from_ptr(ptr: NonNull<ffi::AFontMatcher>) -> Self {
        Self { ptr }
    }

    /// Returns s the pointer to the native [`ffi::AFontMatcher`].
    pub fn ptr(&self) -> NonNull<ffi::AFontMatcher> {
        self.ptr
    }

    /// Creates a new [`FontMatcher`] object. [`FontMatcher`] selects the best font from the
    /// parameters set by the user.
    pub fn new() -> Self {
        let ptr = NonNull::new(unsafe { ffi::AFontMatcher_create() })
            .expect("AFontMatcher_create returned NULL");
        unsafe { FontMatcher::from_ptr(ptr) }
    }

    /// Performs the matching from the generic font family for the text and select one font.
    ///
    /// For more information about generic font families, please read the
    /// [W3C spec](https://www.w3.org/TR/css-fonts-4/#generic-font-families).
    ///
    /// Even if no font can render the given text, this function will return a non-null result for
    /// drawing Tofu character.
    ///
    /// # Parameters
    ///
    /// - `family_name`: A font family name.
    /// - `text`: A UTF-16 encoded text buffer to be rendered. If an empty string is given, this
    ///   function will panic.
    /// - `run_length_out`: Set this to [`Some`] if you want to get the length of the text run with
    ///   the font returned.
    pub fn match_font(
        &mut self,
        family_name: &CStr,
        text: &[u16],
        run_length_out: Option<&mut u32>,
    ) -> Font {
        if text.is_empty() {
            panic!("text is empty");
        }
        unsafe {
            Font::from_ptr(
                NonNull::new(ffi::AFontMatcher_match(
                    self.ptr.as_ptr(),
                    family_name.as_ptr(),
                    text.as_ptr(),
                    text.len() as _,
                    run_length_out.map_or(std::ptr::null_mut(), |u| u),
                ))
                .expect("AFontMatcher_match returned NULL"),
            )
        }
    }

    /// Sets the family variant of the font to be matched.
    ///
    /// If this function is not called, the match is performed with [`FamilyVariant::Default`].
    pub fn set_family_variant(&mut self, family_variant: FamilyVariant) {
        unsafe { ffi::AFontMatcher_setFamilyVariant(self.ptr.as_ptr(), family_variant.into()) }
    }

    /// Sets the locale of the font to be matched.
    ///
    /// If this function is not called, the match is performed with an empty locale list.
    ///
    /// # Parameters
    ///
    /// - `language_tags`: comma separated IETF BCP47 compliant language tags.
    pub fn set_locales(&mut self, language_tags: &CStr) {
        unsafe { ffi::AFontMatcher_setLocales(self.ptr.as_ptr(), language_tags.as_ptr()) }
    }

    /// Sets the style of the font to be matched.
    ///
    /// If this function is not called, the match is performed with [`FontWeight::NORMAL`] with non-italic style.
    pub fn set_style(&mut self, weight: FontWeight, italic: bool) {
        unsafe { ffi::AFontMatcher_setStyle(self.ptr.as_ptr(), weight.to_u16(), italic) }
    }
}

impl Drop for FontMatcher {
    fn drop(&mut self) {
        unsafe { ffi::AFontMatcher_destroy(self.ptr.as_ptr()) }
    }
}

/// A native [`ASystemFontIterator *`]
///
/// [`ASystemFontIterator *`]: https://developer.android.com/ndk/reference/group/font#asystemfontiterator_open
#[derive(Debug)]
pub struct SystemFontIterator {
    ptr: NonNull<ffi::ASystemFontIterator>,
}

impl SystemFontIterator {
    /// Assumes ownership of `ptr`.
    ///
    /// # Safety
    /// `ptr` must be a valid owning pointer to an Android [`ffi::ASystemFontIterator`].
    pub unsafe fn from_ptr(ptr: NonNull<ffi::ASystemFontIterator>) -> Self {
        Self { ptr }
    }

    /// Returns the pointer to the native [`ffi::ASystemFontIterator`].
    pub fn ptr(&self) -> NonNull<ffi::ASystemFontIterator> {
        self.ptr
    }

    /// Creates a system font iterator.
    pub fn new() -> Option<Self> {
        NonNull::new(unsafe { ffi::ASystemFontIterator_open() })
            .map(|p| unsafe { SystemFontIterator::from_ptr(p) })
    }
}

impl Iterator for SystemFontIterator {
    type Item = Font;

    fn next(&mut self) -> Option<Self::Item> {
        NonNull::new(unsafe { ffi::ASystemFontIterator_next(self.ptr.as_ptr()) })
            .map(|p| unsafe { Font::from_ptr(p) })
    }
}

impl Drop for SystemFontIterator {
    fn drop(&mut self) {
        unsafe { ffi::ASystemFontIterator_close(self.ptr.as_ptr()) }
    }
}
