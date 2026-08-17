#![cfg(all(feature = "with-system-locale", any(unix, windows)))]

mod nix;
mod windows;

use std::collections::HashSet;

use crate::error::Error;
use crate::format::Format;
use crate::grouping::Grouping;
use crate::strings::{
    DecString, DecimalStr, InfString, InfinityStr, MinString, MinusSignStr, NanStr, NanString,
    PlusSignStr, PlusString, SepString, SeparatorStr,
};

/// <b><u>A key type</u></b>. Represents formats obtained from your operating system. Implements
/// [`Format`].
///
/// # Example
/// ```rust
/// use num_format::SystemLocale;
///
/// fn main() {
///     let locale = SystemLocale::default().unwrap();
///     println!("My system's default locale is...");
///     println!("{:#?}", &locale);
///
///     let available = SystemLocale::available_names().unwrap();
///     println!("My available locale names are...");
///     println!("{:#?}", available);
///
///     match SystemLocale::from_name("en_US") {
///         Ok(_) => println!("My system has the 'en_US' locale."),
///         Err(_) => println!("The 'en_US' locale is not included with my system."),
///     }
/// }
/// ```
///
/// [`Format`]: trait.Format.html
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub struct SystemLocale {
    pub(crate) dec: DecString,
    pub(crate) grp: Grouping,
    pub(crate) inf: InfString,
    pub(crate) min: MinString,
    pub(crate) name: String,
    pub(crate) nan: NanString,
    pub(crate) plus: PlusString,
    pub(crate) sep: SepString,
}

impl SystemLocale {
    /// Same as [`default`].
    ///
    /// [`default`]: struct.SystemLocale.html#method.default
    pub fn new() -> Result<SystemLocale, Error> {
        SystemLocale::default()
    }

    /// Constucts a [`SystemLocale`] based on your operating system's default locale.
    ///
    /// * **Unix-based systems (including macOS)**: The default locale is controlled by "locale
    ///   category environment variables," such as `LANG`, `LC_MONETARY`, `LC_NUMERIC`, and
    ///   `LC_ALL`. For more information, see
    ///   [here](https://www.gnu.org/software/libc/manual/html_node/Locale-Categories.html#Locale-Categories).
    /// * **Windows**: The default locale is controlled by the regional and language options portion
    ///   of the Control Panel. For more information, see
    ///   [here](https://docs.microsoft.com/en-us/windows/desktop/intl/locales-and-languages).
    ///
    /// # Errors
    ///
    /// Returns an error if the operating system returned something unexpected (such as a null
    /// pointer).
    ///
    /// [`SystemLocale`]: struct.SystemLocale.html
    pub fn default() -> Result<SystemLocale, Error> {
        #[cfg(unix)]
        return self::nix::new(None);

        #[cfg(windows)]
        return self::windows::new(None);

        #[cfg(not(any(unix, windows)))]
        unreachable!()
    }

    /// Constucts a [`SystemLocale`] from the provided locale name. For a list of locale names
    /// available on your system, see [`available_names`].
    ///
    /// # Errors
    ///
    /// Returns an error if the name provided could not be parsed into a [`SystemLocale`] or if the
    /// operating system returned something unexpected (such as a null pointer).
    ///
    /// [`available_names`]: struct.SystemLocale.html#method.available_names
    /// [`SystemLocale`]: struct.SystemLocale.html
    pub fn from_name<S>(name: S) -> Result<SystemLocale, Error>
    where
        S: Into<String>,
    {
        #[cfg(unix)]
        return self::nix::new(Some(name.into()));

        #[cfg(windows)]
        return self::windows::new(Some(name.into()));

        #[cfg(not(any(unix, windows)))]
        unreachable!()
    }

    /// Returns a set of the locale names available on your operating system.
    ///
    /// * **Unix-based systems (including macOS)**: The underlying implementation uses the
    ///   [`locale`] command.
    /// * **Windows**: The underlying implementation uses the [`EnumSystemLocalesEx`] function.
    /// # Errors
    ///
    /// Returns an error if the operating system returned something unexpected (such as a null
    /// pointer).
    ///
    /// [`EnumSystemLocalesEx`]: https://docs.microsoft.com/en-us/windows/desktop/api/winnls/nf-winnls-enumsystemlocalesex
    /// [`locale`]: http://man7.org/linux/man-pages/man1/locale.1.html
    pub fn available_names() -> Result<HashSet<String>, Error> {
        #[cfg(unix)]
        return Ok(self::nix::available_names());

        #[cfg(windows)]
        return self::windows::available_names();

        #[cfg(not(any(unix, windows)))]
        unreachable!()
    }

    /// Returns this locale's string representation of a decimal point.
    pub fn decimal(&self) -> &str {
        &self.dec
    }

    /// Returns this locale's [`Grouping`], which governs how digits are separated
    /// (see [`Grouping`]).
    ///
    /// [`Grouping`]: enum.Grouping.html
    pub fn grouping(&self) -> Grouping {
        self.grp
    }

    /// Returns this locale's string representation of infinity.
    pub fn infinity(&self) -> &str {
        &self.inf
    }

    /// Returns this locale's string representation of a minus sign.
    pub fn minus_sign(&self) -> &str {
        &self.min
    }

    /// Returns this locale's name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns this locale's string representation of NaN.
    pub fn nan(&self) -> &str {
        &self.nan
    }

    /// Returns this locale's string representation of a plus sign.
    pub fn plus_sign(&self) -> &str {
        &self.plus
    }

    /// Returns this locale's string representation of a thousands separator.
    pub fn separator(&self) -> &str {
        &self.sep
    }

    #[cfg(unix)]
    /// Unix-based operating systems (including macOS) do not provide information on how to
    /// represent infinity symbols; so num-format uses `"∞"` (U+221E) as a default. This
    /// method allows you to change that default.
    ///
    /// # Errors
    ///
    /// Returns an error the provided string is longer than 128 bytes.
    pub fn set_infinity<S>(&mut self, s: S) -> Result<(), Error>
    where
        S: AsRef<str>,
    {
        self.inf = InfString::new(s)?;
        Ok(())
    }

    #[cfg(unix)]
    /// Unix-based operating systems (including macOS) do not provide information on how to
    /// represent NaN; so num-format uses `"NaN"` as a default. This method allows you to change
    /// that default.
    ///
    /// # Errors
    ///
    /// Returns an error the provided string is longer than 64 bytes.
    pub fn set_nan<S>(&mut self, s: S) -> Result<(), Error>
    where
        S: AsRef<str>,
    {
        self.nan = NanString::new(s)?;
        Ok(())
    }
}

impl std::str::FromStr for SystemLocale {
    type Err = Error;

    /// Same as [`from_name`].
    ///
    /// [`from_name`]: struct.SystemLocale.html#method.from_name
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        SystemLocale::from_name(s)
    }
}

impl Format for SystemLocale {
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
