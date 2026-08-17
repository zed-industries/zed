use std::fmt::{self, Display, Formatter};

/// Country code for a [`Language`] dialect
///
/// Uses <https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2>
#[non_exhaustive]
#[repr(u32)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Country {
    // FIXME: V2: u32::from_ne_bytes for country codes, with `\0` for unused
    // FIXME: Add aliases up to 3-4 letters, but hidden
    /// Any dialect
    Any,
    /// `US`: United States of America
    #[doc(hidden)]
    Us,
}

impl Display for Country {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Any => "**",
            Self::Us => "US",
        })
    }
}

/// A spoken language
///
/// Use [`ToString::to_string()`] to convert to string of two letter lowercase
/// language code followed an forward slash and uppercase country code (example:
/// `en/US`).
///
/// Language codes defined in ISO 639 <https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes>,
/// Country codes defined in ISO 3166 <https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2>
#[non_exhaustive]
#[derive(Clone, Eq, PartialEq, Debug)]
// #[allow(variant_size_differences)]
pub enum Language {
    #[doc(hidden)]
    __(Box<String>),
    /// `en`: English
    #[doc(hidden)]
    En(Country),
    /// `es`: Spanish
    #[doc(hidden)]
    Es(Country),
}

impl Language {
    /// Retrieve the country code for this language dialect.
    pub fn country(&self) -> Country {
        match self {
            Self::__(_) => Country::Any,
            Self::En(country) | Self::Es(country) => *country,
        }
    }
}

impl Display for Language {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::__(code) => f.write_str(code.as_str()),
            Self::En(country) => {
                if *country != Country::Any {
                    f.write_str("en/")?;
                    <Country as Display>::fmt(country, f)
                } else {
                    f.write_str("en")
                }
            }
            Self::Es(country) => {
                if *country != Country::Any {
                    f.write_str("es/")?;
                    <Country as Display>::fmt(country, f)
                } else {
                    f.write_str("es")
                }
            }
        }
    }
}
