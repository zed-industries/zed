use crate::custom_format::CustomFormat;
use crate::error::Error;
use crate::format::Format;
use crate::grouping::Grouping;
use crate::locale::Locale;
use crate::strings::{DecString, InfString, MinString, NanString, PlusString, SepString};

/// Type for building [`CustomFormat`]s.
///
/// [`CustomFormat`]: struct.CustomFormat.html
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub struct CustomFormatBuilder {
    dec: Result<DecString, Error>,
    grp: Grouping,
    inf: Result<InfString, Error>,
    min: Result<MinString, Error>,
    nan: Result<NanString, Error>,
    plus: Result<PlusString, Error>,
    sep: Result<SepString, Error>,
}

impl CustomFormatBuilder {
    pub(crate) fn new() -> Self {
        Self {
            dec: DecString::new(Locale::en.decimal()),
            grp: Locale::en.grouping(),
            inf: InfString::new(Locale::en.infinity()),
            min: MinString::new(Locale::en.minus_sign()),
            nan: NanString::new(Locale::en.nan()),
            plus: PlusString::new(Locale::en.plus_sign()),
            sep: SepString::new(Locale::en.separator()),
        }
    }

    /// Construct a [`CustomFormat`].
    ///
    /// # Errors
    ///
    /// Return an error if:
    /// - The "decimal" is longer than 8 bytes
    /// - The "infinity sign" is longer than 128 bytes
    /// - The "minus sign" is longer than 8 bytes
    /// - The "nan symbol" is longer than 64 bytes
    /// - The "plus sign" is longer than 8 bytes
    /// - The "separator" is longer than 8 bytes
    ///
    /// [`CustomFormat`]: struct.CustomFormat.html
    pub fn build(self) -> Result<CustomFormat, Error> {
        Ok(CustomFormat {
            dec: self.dec?,
            grp: self.grp,
            inf: self.inf?,
            min: self.min?,
            nan: self.nan?,
            plus: self.plus?,
            sep: self.sep?,
        })
    }

    /// Sets the character used to represent decimal points.
    pub fn decimal<S>(mut self, s: S) -> Self
    where
        S: AsRef<str>,
    {
        self.dec = DecString::new(s);
        self
    }

    /// Sets all fields based on the provided format.
    pub fn format<F>(mut self, value: &F) -> Self
    where
        F: Format,
    {
        self.dec = DecString::new(value.decimal());
        self.grp = value.grouping();
        self.inf = InfString::new(value.infinity());
        self.min = MinString::new(value.minus_sign());
        self.nan = NanString::new(value.nan());
        self.plus = PlusString::new(value.plus_sign());
        self.sep = SepString::new(value.separator());
        self
    }

    /// Sets the [`Grouping`] used to separate digits.
    ///
    /// [`Grouping`]: enum.Grouping.html
    pub fn grouping(mut self, value: Grouping) -> Self {
        self.grp = value;
        self
    }

    /// Sets the string representation of infinity.
    pub fn infinity<S>(mut self, s: S) -> Self
    where
        S: AsRef<str>,
    {
        self.inf = InfString::new(s);
        self
    }

    /// Sets the string representation of a minus sign.
    pub fn minus_sign<S>(mut self, s: S) -> Self
    where
        S: AsRef<str>,
    {
        self.min = MinString::new(s);
        self
    }

    /// Sets the string representation of NaN.
    pub fn nan<S>(mut self, s: S) -> Self
    where
        S: AsRef<str>,
    {
        self.nan = NanString::new(s);
        self
    }

    /// Sets the string representation of a plus sign.
    pub fn plus_sign<S>(mut self, s: S) -> Self
    where
        S: AsRef<str>,
    {
        self.plus = PlusString::new(s);
        self
    }

    /// Sets the string representation of a thousands separator.
    pub fn separator<S>(mut self, s: S) -> Self
    where
        S: AsRef<str>,
    {
        self.sep = SepString::new(s);
        self
    }
}

impl From<CustomFormat> for CustomFormatBuilder {
    fn from(format: CustomFormat) -> Self {
        CustomFormat::builder().format(&format)
    }
}

impl From<Locale> for CustomFormatBuilder {
    fn from(locale: Locale) -> Self {
        CustomFormat::builder().format(&locale)
    }
}

#[cfg(all(feature = "with-system-locale", any(unix, windows)))]
mod standard {
    use super::*;
    use crate::system_locale::SystemLocale;

    impl From<SystemLocale> for CustomFormatBuilder {
        fn from(locale: SystemLocale) -> Self {
            CustomFormat::builder().format(&locale)
        }
    }
}
