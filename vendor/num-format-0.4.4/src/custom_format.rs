use crate::strings::{
    DecString, DecimalStr, InfString, InfinityStr, MinString, MinusSignStr, NanStr, NanString,
    PlusSignStr, PlusString, SepString, SeparatorStr,
};
use crate::{CustomFormatBuilder, Format, Grouping, Locale};

/// Type for representing your own custom formats. Implements [`Format`].
///
/// # Example
/// ```rust
/// use num_format::{Buffer, Error, CustomFormat, Grouping};
///
/// fn main() -> Result<(), Error> {
///     let format = CustomFormat::builder()
///         .grouping(Grouping::Indian)
///         .minus_sign("🙌")
///         .separator("😀")
///         .build()?;
///
///     let mut buf = Buffer::new();
///     buf.write_formatted(&(-1000000), &format);
///     assert_eq!("🙌10😀00😀000", buf.as_str());
///
///     Ok(())
/// }
/// ```
///
/// [`Format`]: trait.Format.html
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub struct CustomFormat {
    pub(crate) dec: DecString,
    pub(crate) grp: Grouping,
    pub(crate) inf: InfString,
    pub(crate) min: MinString,
    pub(crate) nan: NanString,
    pub(crate) plus: PlusString,
    pub(crate) sep: SepString,
}

impl CustomFormat {
    /// Constructs a [`CustomFormatBuilder`].
    ///
    /// [`CustomFormatBuilder`]: struct.CustomFormatBuilder.html
    pub fn builder() -> CustomFormatBuilder {
        CustomFormatBuilder::new()
    }

    /// Turns `self` into a [`CustomFormatBuilder`].
    ///
    /// [`CustomFormatBuilder`]: struct.CustomFormatBuilder.html
    pub fn into_builder(self) -> CustomFormatBuilder {
        self.into()
    }

    /// Returns this format's representation of decimal points.
    pub fn decimal(&self) -> &str {
        &self.dec
    }

    /// Returns this format's [`Grouping`], which governs how digits are separated (see [`Grouping`]).
    ///
    /// [`Grouping`]: enum.Grouping.html
    pub fn grouping(&self) -> Grouping {
        self.grp
    }

    /// Returns this format's representation of infinity.
    pub fn infinity(&self) -> &str {
        &self.inf
    }

    /// Returns this format's representation of minus signs.
    pub fn minus_sign(&self) -> &str {
        &self.min
    }

    /// Returns this format's representation of NaN.
    pub fn nan(&self) -> &str {
        &self.nan
    }

    /// Returns this format's representation of plus signs.
    pub fn plus_sign(&self) -> &str {
        &self.plus
    }

    /// Returns this format's representation of separators.
    pub fn separator(&self) -> &str {
        &self.sep
    }
}

impl Default for CustomFormat {
    /// Returns a `CustomFormat` with settings equal to `Locale::en`.
    fn default() -> Self {
        Locale::en.into()
    }
}

impl Format for CustomFormat {
    #[inline(always)]
    fn decimal(&self) -> DecimalStr<'_> {
        DecimalStr::new(self.decimal()).unwrap()
    }

    #[inline(always)]
    fn grouping(&self) -> Grouping {
        self.grouping()
    }

    #[inline(always)]
    fn infinity(&self) -> InfinityStr<'_> {
        InfinityStr::new(self.infinity()).unwrap()
    }

    #[inline(always)]
    fn minus_sign(&self) -> MinusSignStr<'_> {
        MinusSignStr::new(self.minus_sign()).unwrap()
    }

    #[inline(always)]
    fn nan(&self) -> NanStr<'_> {
        NanStr::new(self.nan()).unwrap()
    }

    #[inline(always)]
    fn plus_sign(&self) -> PlusSignStr<'_> {
        PlusSignStr::new(self.plus_sign()).unwrap()
    }

    #[inline(always)]
    fn separator(&self) -> SeparatorStr<'_> {
        SeparatorStr::new(self.separator()).unwrap()
    }
}

impl From<Locale> for CustomFormat {
    fn from(locale: Locale) -> Self {
        Self {
            dec: DecString::new(locale.decimal()).unwrap(),
            grp: locale.grouping(),
            inf: InfString::new(locale.infinity()).unwrap(),
            min: MinString::new(locale.minus_sign()).unwrap(),
            nan: NanString::new(locale.nan()).unwrap(),
            plus: PlusString::new(locale.plus_sign()).unwrap(),
            sep: SepString::new(locale.separator()).unwrap(),
        }
    }
}

#[cfg(all(feature = "with-system-locale", any(unix, windows)))]
mod system {
    use super::*;
    use crate::SystemLocale;

    impl From<SystemLocale> for CustomFormat {
        fn from(locale: SystemLocale) -> Self {
            Self {
                dec: DecString::new(locale.decimal()).unwrap(),
                grp: locale.grouping(),
                inf: InfString::new(locale.infinity()).unwrap(),
                min: MinString::new(locale.minus_sign()).unwrap(),
                nan: NanString::new(locale.nan()).unwrap(),
                plus: PlusString::new(locale.plus_sign()).unwrap(),
                sep: SepString::new(locale.separator()).unwrap(),
            }
        }
    }
}

#[cfg(all(test, feature = "with-serde"))]
mod tests {
    use super::*;

    #[test]
    fn test_serialization() {
        let locale = CustomFormat::builder().build().unwrap();
        let s = serde_json::to_string(&locale).unwrap();
        let expected =
            r#"{"dec":".","grp":"Standard","inf":"∞","min":"-","nan":"NaN","plus":"+","sep":","}"#;
        assert_eq!(expected, &s);
    }
}
