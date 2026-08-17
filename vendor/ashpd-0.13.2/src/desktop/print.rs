//! # Examples
//!
//! Print a file
//!
//! ```rust,no_run
//! use std::{fs::File, os::fd::AsFd};
//!
//! use ashpd::desktop::print::{PreparePrintOptions, PrintOptions, PrintProxy};
//!
//! async fn run() -> ashpd::Result<()> {
//!     let proxy = PrintProxy::new().await?;
//!
//!     let file =
//!         File::open("/home/bilelmoussaoui/gitlog.pdf").expect("file to print was not found");
//!     let pre_print = proxy
//!         .prepare_print(
//!             None,
//!             "prepare print",
//!             Default::default(),
//!             Default::default(),
//!             PreparePrintOptions::default().set_modal(true),
//!         )
//!         .await?
//!         .response()?;
//!     proxy
//!         .print(
//!             None,
//!             "test",
//!             &file.as_fd(),
//!             PrintOptions::default()
//!                 .set_token(pre_print.token)
//!                 .set_modal(true),
//!         )
//!         .await?;
//!
//!     Ok(())
//! }
//! ```

use std::{fmt, os::fd::AsFd, str::FromStr};

use serde::{Deserialize, Serialize};
use zbus::zvariant::{
    Fd, Optional, Type,
    as_value::{self, optional},
};

use super::{HandleToken, Request};
use crate::{Error, Uri, WindowIdentifier, proxy::Proxy};

#[cfg_attr(feature = "glib", derive(glib::Enum))]
#[cfg_attr(feature = "glib", enum_type(name = "AshpdOrientation"))]
#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq, Eq, Type)]
#[zvariant(signature = "s")]
#[serde(rename_all = "snake_case")]
/// The page orientation.
pub enum Orientation {
    /// Landscape.
    Landscape,
    /// Portrait.
    Portrait,
    /// Reverse landscape.
    ReverseLandscape,
    /// Reverse portrait.
    ReversePortrait,
}

impl fmt::Display for Orientation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Landscape => write!(f, "Landscape"),
            Self::Portrait => write!(f, "Portrait"),
            Self::ReverseLandscape => write!(f, "Reverse Landscape"),
            Self::ReversePortrait => write!(f, "Reverse Portrait"),
        }
    }
}

impl AsRef<str> for Orientation {
    fn as_ref(&self) -> &str {
        match self {
            Self::Landscape => "Landscape",
            Self::Portrait => "Portrait",
            Self::ReverseLandscape => "Reverse Landscape",
            Self::ReversePortrait => "Reverse Portrait",
        }
    }
}

impl From<Orientation> for &'static str {
    fn from(o: Orientation) -> Self {
        match o {
            Orientation::Landscape => "Landscape",
            Orientation::Portrait => "Portrait",
            Orientation::ReverseLandscape => "Reverse Landscape",
            Orientation::ReversePortrait => "Reverse Portrait",
        }
    }
}

impl FromStr for Orientation {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Landscape" | "landscape" => Ok(Orientation::Landscape),
            "Portrait" | "portrait" => Ok(Orientation::Portrait),
            "ReverseLandscape" | "reverse_landscape" => Ok(Orientation::ReverseLandscape),
            "ReversePortrait" | "reverse_portrait" => Ok(Orientation::ReversePortrait),
            _ => Err(Error::ParseError(
                "Failed to parse orientation, invalid value",
            )),
        }
    }
}

#[cfg_attr(feature = "glib", derive(glib::Enum))]
#[cfg_attr(feature = "glib", enum_type(name = "AshpdQuality"))]
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Type)]
#[zvariant(signature = "s")]
#[serde(rename_all = "lowercase")]
/// The print quality.
pub enum Quality {
    /// Draft quality.
    Draft,
    /// Low quality.
    Low,
    /// Normal quality.
    Normal,
    /// High quality.
    High,
}

impl fmt::Display for Quality {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Draft => write!(f, "Draft"),
            Self::Low => write!(f, "Low"),
            Self::Normal => write!(f, "Normal"),
            Self::High => write!(f, "High"),
        }
    }
}

impl AsRef<str> for Quality {
    fn as_ref(&self) -> &str {
        match self {
            Self::Draft => "Draft",
            Self::Low => "Low",
            Self::Normal => "Normal",
            Self::High => "High",
        }
    }
}

impl From<Quality> for &'static str {
    fn from(q: Quality) -> Self {
        match q {
            Quality::Draft => "Draft",
            Quality::Low => "Low",
            Quality::Normal => "Normal",
            Quality::High => "High",
        }
    }
}

impl FromStr for Quality {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Draft" | "draft" => Ok(Quality::Draft),
            "Low" | "low" => Ok(Quality::Low),
            "Normal" | "normal" => Ok(Quality::Normal),
            "High" | "high" => Ok(Quality::High),
            _ => Err(Error::ParseError("Failed to parse quality, invalid value")),
        }
    }
}

#[cfg_attr(feature = "glib", derive(glib::Enum))]
#[cfg_attr(feature = "glib", enum_type(name = "AshpdDuplex"))]
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Type)]
#[zvariant(signature = "s")]
#[serde(rename_all = "lowercase")]
/// Duplex printing mode.
pub enum Duplex {
    /// Simplex (single-sided) printing.
    Simplex,
    /// Horizontal duplex (flip on short edge).
    Horizontal,
    /// Vertical duplex (flip on long edge).
    Vertical,
}

#[cfg_attr(feature = "glib", derive(glib::Enum))]
#[cfg_attr(feature = "glib", enum_type(name = "AshpdPrintPages"))]
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Type)]
#[zvariant(signature = "s")]
#[serde(rename_all = "lowercase")]
/// What pages to print.
pub enum PrintPages {
    /// Print all pages.
    All,
    /// Print selected pages.
    Selection,
    /// Print current page.
    Current,
    /// Print page ranges.
    Ranges,
}

#[cfg_attr(feature = "glib", derive(glib::Enum))]
#[cfg_attr(feature = "glib", enum_type(name = "AshpdPageSet"))]
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Type)]
#[zvariant(signature = "s")]
#[serde(rename_all = "lowercase")]
/// What pages to print.
pub enum PageSet {
    /// Print all pages.
    All,
    /// Print even pages only.
    Even,
    /// Print odd pages only.
    Odd,
}

#[cfg_attr(feature = "glib", derive(glib::Enum))]
#[cfg_attr(feature = "glib", enum_type(name = "AshpdNumberUpLayout"))]
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Type)]
#[zvariant(signature = "s")]
#[serde(rename_all = "lowercase")]
/// Layout for number-up printing.
pub enum NumberUpLayout {
    /// Left to right, top to bottom.
    #[serde(rename = "lrtb")]
    Lrtb,
    /// Left to right, bottom to top.
    #[serde(rename = "lrbt")]
    Lrbt,
    /// Right to left, top to bottom.
    #[serde(rename = "rltb")]
    Rltb,
    /// Right to left, bottom to top.
    #[serde(rename = "rlbt")]
    Rlbt,
    /// Top to bottom, left to right.
    #[serde(rename = "tblr")]
    Tblr,
    /// Top to bottom, right to left.
    #[serde(rename = "tbrl")]
    Tbrl,
    /// Bottom to top, left to right.
    #[serde(rename = "btlr")]
    Btlr,
    /// Bottom to top, right to left.
    #[serde(rename = "btrl")]
    Btrl,
}

#[cfg_attr(feature = "glib", derive(glib::Enum))]
#[cfg_attr(feature = "glib", enum_type(name = "AshpdOutputFileFormat"))]
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Type)]
#[zvariant(signature = "s")]
#[serde(rename_all = "UPPERCASE")]
/// Output file format for print-to-file.
pub enum OutputFileFormat {
    /// PDF format.
    Pdf,
    /// PostScript format.
    Ps,
    /// SVG format.
    Svg,
}

#[cfg_attr(feature = "glib", derive(glib::Enum))]
#[cfg_attr(feature = "glib", enum_type(name = "AshpdDither"))]
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Type)]
#[zvariant(signature = "s")]
#[serde(rename_all = "kebab-case")]
/// Dithering to use for printing.
pub enum Dither {
    /// Fine dithering.
    Fine,
    /// No dithering.
    None,
    /// Coarse dithering.
    Coarse,
    /// Line art dithering.
    Lineart,
    /// Grayscale dithering.
    Grayscale,
    /// Error diffusion dithering.
    ErrorDiffusion,
}

// Custom serialization modules for string-wrapped types
mod string_bool {
    use serde::{Deserializer, Serializer, de::Error};
    use zbus::zvariant::as_value::optional;

    pub fn serialize<S>(value: &Option<bool>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match value {
            Some(true) => optional::serialize(&Some("true"), serializer),
            Some(false) => optional::serialize(&Some("false"), serializer),
            None => optional::serialize(&None::<&str>, serializer),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let opt: Option<String> = optional::deserialize(deserializer)?;
        opt.map(|s| match s.as_str() {
            "true" => Ok(true),
            "false" => Ok(false),
            _ => Err(D::Error::custom(format!("invalid boolean string: {}", s))),
        })
        .transpose()
    }
}

mod string_u32 {
    use serde::{Deserializer, Serializer, de::Error};
    use zbus::zvariant::as_value::optional;

    pub fn serialize<S>(value: &Option<u32>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let str_value = value.map(|v| v.to_string());
        optional::serialize(&str_value, serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<u32>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let opt: Option<String> = optional::deserialize(deserializer)?;
        opt.map(|s| s.parse::<u32>().map_err(D::Error::custom))
            .transpose()
    }
}

mod string_i32 {
    use serde::{Deserializer, Serializer, de::Error};
    use zbus::zvariant::as_value::optional;

    pub fn serialize<S>(value: &Option<i32>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let str_value = value.map(|v| v.to_string());
        optional::serialize(&str_value, serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<i32>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let opt: Option<String> = optional::deserialize(deserializer)?;
        opt.map(|s| s.parse::<i32>().map_err(D::Error::custom))
            .transpose()
    }
}

#[derive(Serialize, Deserialize, Type, Debug, Default)]
/// Print settings to set in the print dialog.
#[zvariant(signature = "dict")]
#[serde(rename_all = "kebab-case")]
pub struct Settings {
    /// One of landscape, portrait, reverse_landscape or reverse_portrait.
    #[serde(with = "optional", skip_serializing_if = "Option::is_none")]
    pub orientation: Option<Orientation>,
    /// A paper name according to [PWG 5101.1-2002](ftp://ftp.pwg.org/pub/pwg/candidates/cs-pwgmsn10-20020226-5101.1.pdf)
    #[serde(with = "optional", skip_serializing_if = "Option::is_none")]
    pub paper_format: Option<String>,
    /// Paper width, in millimeters.
    #[serde(with = "optional", skip_serializing_if = "Option::is_none")]
    pub paper_width: Option<String>,
    /// Paper height, in millimeters.
    #[serde(with = "optional", skip_serializing_if = "Option::is_none")]
    pub paper_height: Option<String>,
    /// The number of copies to print.
    #[serde(with = "string_u32", skip_serializing_if = "Option::is_none")]
    pub n_copies: Option<u32>,
    /// The default paper source.
    #[serde(with = "optional", skip_serializing_if = "Option::is_none")]
    pub default_source: Option<String>,
    /// Print quality.
    #[serde(with = "optional", skip_serializing_if = "Option::is_none")]
    pub quality: Option<Quality>,
    /// The resolution, sets both resolution-x & resolution-y
    #[serde(with = "optional", skip_serializing_if = "Option::is_none")]
    pub resolution: Option<String>,
    /// Whether to use color.
    #[serde(with = "string_bool", skip_serializing_if = "Option::is_none")]
    pub use_color: Option<bool>,
    /// Duplex printing mode.
    #[serde(with = "optional", skip_serializing_if = "Option::is_none")]
    pub duplex: Option<Duplex>,
    /// Whether to collate copies.
    #[serde(with = "string_bool", skip_serializing_if = "Option::is_none")]
    pub collate: Option<bool>,
    /// Whether to reverse the order of printed pages.
    #[serde(with = "string_bool", skip_serializing_if = "Option::is_none")]
    pub reverse: Option<bool>,
    /// A media type according to [PWG 5101.1-2002](ftp://ftp.pwg.org/pub/pwg/candidates/cs-pwgmsn10-20020226-5101.1.pdf)
    #[serde(with = "optional", skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
    /// The dithering to use.
    #[serde(with = "optional", skip_serializing_if = "Option::is_none")]
    pub dither: Option<Dither>,
    /// The scale in percent.
    #[serde(with = "string_u32", skip_serializing_if = "Option::is_none")]
    pub scale: Option<u32>,
    /// What pages to print.
    #[serde(with = "optional", skip_serializing_if = "Option::is_none")]
    pub print_pages: Option<PrintPages>,
    /// A list of page ranges, formatted like this: 0-2,4,9-11.
    #[serde(with = "optional", skip_serializing_if = "Option::is_none")]
    pub page_ranges: Option<String>,
    /// Whether to print all, even, or odd pages.
    #[serde(with = "optional", skip_serializing_if = "Option::is_none")]
    pub page_set: Option<PageSet>,
    /// The finishings.
    #[serde(with = "optional", skip_serializing_if = "Option::is_none")]
    pub finishings: Option<String>,
    /// The number of pages per sheet.
    #[serde(with = "string_u32", skip_serializing_if = "Option::is_none")]
    pub number_up: Option<u32>,
    /// Layout for number-up printing.
    #[serde(with = "optional", skip_serializing_if = "Option::is_none")]
    pub number_up_layout: Option<NumberUpLayout>,
    /// The output bin.
    #[serde(with = "optional", skip_serializing_if = "Option::is_none")]
    pub output_bin: Option<String>,
    /// The horizontal resolution in dpi.
    #[serde(with = "string_i32", skip_serializing_if = "Option::is_none")]
    pub resolution_x: Option<i32>,
    /// The vertical resolution in dpi.
    #[serde(with = "string_i32", skip_serializing_if = "Option::is_none")]
    pub resolution_y: Option<i32>,
    /// The resolution in lpi (lines per inch).
    #[serde(with = "optional", skip_serializing_if = "Option::is_none")]
    pub printer_lpi: Option<String>,
    /// Basename to use for print-to-file.
    #[serde(with = "optional", skip_serializing_if = "Option::is_none")]
    pub output_basename: Option<String>,
    /// Format to use for print-to-file.
    #[serde(with = "optional", skip_serializing_if = "Option::is_none")]
    pub output_file_format: Option<OutputFileFormat>,
    /// The uri used for print-to file.
    #[serde(with = "optional", skip_serializing_if = "Option::is_none")]
    pub output_uri: Option<Uri>,
}

impl Settings {
    /// Sets the orientation.
    #[must_use]
    pub fn set_orientation(mut self, orientation: impl Into<Option<Orientation>>) -> Self {
        self.orientation = orientation.into();
        self
    }

    /// Sets the paper name.
    #[must_use]
    pub fn set_paper_format<'a>(mut self, paper_format: impl Into<Option<&'a str>>) -> Self {
        self.paper_format = paper_format.into().map(ToOwned::to_owned);
        self
    }

    /// Sets the paper width.
    #[must_use]
    pub fn set_paper_width<'a>(mut self, paper_width: impl Into<Option<&'a str>>) -> Self {
        self.paper_width = paper_width.into().map(ToOwned::to_owned);
        self
    }

    /// Sets the paper height.
    #[must_use]
    pub fn set_paper_height<'a>(mut self, paper_height: impl Into<Option<&'a str>>) -> Self {
        self.paper_height = paper_height.into().map(ToOwned::to_owned);
        self
    }

    /// Sets the number of copies to print.
    #[must_use]
    pub fn set_n_copies(mut self, n_copies: impl Into<Option<u32>>) -> Self {
        self.n_copies = n_copies.into();
        self
    }

    /// Sets the default paper source.
    #[must_use]
    pub fn set_default_source<'a>(mut self, default_source: impl Into<Option<&'a str>>) -> Self {
        self.default_source = default_source.into().map(ToOwned::to_owned);
        self
    }

    /// Sets the print quality.
    #[must_use]
    pub fn set_quality(mut self, quality: impl Into<Option<Quality>>) -> Self {
        self.quality = quality.into();
        self
    }

    /// Sets the resolution, both resolution-x & resolution-y.
    #[must_use]
    pub fn set_resolution<'a>(mut self, resolution: impl Into<Option<&'a str>>) -> Self {
        self.resolution = resolution.into().map(ToOwned::to_owned);
        self
    }

    /// Sets whether to use color.
    #[must_use]
    pub fn set_use_color(mut self, use_color: impl Into<Option<bool>>) -> Self {
        self.use_color = use_color.into();
        self
    }

    /// Sets the duplex printing mode.
    #[must_use]
    pub fn set_duplex(mut self, duplex: impl Into<Option<Duplex>>) -> Self {
        self.duplex = duplex.into();
        self
    }

    /// Whether to collate copies.
    #[must_use]
    pub fn set_collate(mut self, collate: impl Into<Option<bool>>) -> Self {
        self.collate = collate.into();
        self
    }

    /// Sets whether to reverse the order of the printed pages.
    #[must_use]
    pub fn set_reverse(mut self, reverse: impl Into<Option<bool>>) -> Self {
        self.reverse = reverse.into();
        self
    }

    /// Sets the media type.
    #[must_use]
    pub fn set_media_type<'a>(mut self, media_type: impl Into<Option<&'a str>>) -> Self {
        self.media_type = media_type.into().map(ToOwned::to_owned);
        self
    }

    /// Sets the dithering to use.
    #[must_use]
    pub fn set_dither(mut self, dither: impl Into<Option<Dither>>) -> Self {
        self.dither = dither.into();
        self
    }

    /// Sets the page scale in percent.
    #[must_use]
    pub fn set_scale(mut self, scale: impl Into<Option<u32>>) -> Self {
        self.scale = scale.into();
        self
    }

    /// Sets what pages to print.
    #[must_use]
    pub fn set_print_pages(mut self, print_pages: impl Into<Option<PrintPages>>) -> Self {
        self.print_pages = print_pages.into();
        self
    }

    /// Sets a list of page ranges, formatted like this: 0-2,4,9-11.
    #[must_use]
    pub fn set_page_ranges<'a>(mut self, page_ranges: impl Into<Option<&'a str>>) -> Self {
        self.page_ranges = page_ranges.into().map(ToOwned::to_owned);
        self
    }

    /// Sets whether to print all, even, or odd pages.
    #[must_use]
    pub fn set_page_set(mut self, page_set: impl Into<Option<PageSet>>) -> Self {
        self.page_set = page_set.into();
        self
    }

    /// Sets the finishings.
    #[must_use]
    pub fn set_finishings<'a>(mut self, finishings: impl Into<Option<&'a str>>) -> Self {
        self.finishings = finishings.into().map(ToOwned::to_owned);
        self
    }

    /// Sets the number of pages per sheet.
    #[must_use]
    pub fn set_number_up(mut self, number_up: impl Into<Option<u32>>) -> Self {
        self.number_up = number_up.into();
        self
    }

    /// Sets the number-up layout.
    #[must_use]
    pub fn set_number_up_layout(
        mut self,
        number_up_layout: impl Into<Option<NumberUpLayout>>,
    ) -> Self {
        self.number_up_layout = number_up_layout.into();
        self
    }

    /// Sets the output bin
    #[must_use]
    pub fn set_output_bin<'a>(mut self, output_bin: impl Into<Option<&'a str>>) -> Self {
        self.output_bin = output_bin.into().map(ToOwned::to_owned);
        self
    }

    /// Sets the horizontal resolution in dpi.
    #[must_use]
    pub fn set_resolution_x(mut self, resolution_x: impl Into<Option<i32>>) -> Self {
        self.resolution_x = resolution_x.into();
        self
    }

    /// Sets the vertical resolution in dpi.
    #[must_use]
    pub fn set_resolution_y(mut self, resolution_y: impl Into<Option<i32>>) -> Self {
        self.resolution_y = resolution_y.into();
        self
    }

    /// Sets the resolution in lines per inch.
    #[must_use]
    pub fn set_printer_lpi<'a>(mut self, printer_lpi: impl Into<Option<&'a str>>) -> Self {
        self.printer_lpi = printer_lpi.into().map(ToOwned::to_owned);
        self
    }

    /// Sets the print-to-file base name.
    #[must_use]
    pub fn set_output_basename<'a>(mut self, output_basename: impl Into<Option<&'a str>>) -> Self {
        self.output_basename = output_basename.into().map(ToOwned::to_owned);
        self
    }

    /// Sets the print-to-file format.
    #[must_use]
    pub fn set_output_file_format(
        mut self,
        output_file_format: impl Into<Option<OutputFileFormat>>,
    ) -> Self {
        self.output_file_format = output_file_format.into();
        self
    }

    /// Sets the print-to-file output uri.
    #[must_use]
    pub fn set_output_uri<'a>(mut self, output_uri: impl Into<Option<&'a Uri>>) -> Self {
        self.output_uri = output_uri.into().map(ToOwned::to_owned);
        self
    }
}

#[derive(Serialize, Deserialize, Type, Debug, Default)]
/// Setup the printed pages.
#[zvariant(signature = "dict")]
#[serde(rename_all = "PascalCase")]
pub struct PageSetup {
    /// the PPD name. It's the name to select a given driver.
    #[serde(rename = "PPDName")]
    #[serde(with = "optional", skip_serializing_if = "Option::is_none")]
    pub ppdname: Option<String>,
    /// The name of the page setup.
    #[serde(with = "optional", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The user-visible name of the page setup.
    #[serde(with = "optional", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// Paper width in millimeters.
    #[serde(with = "optional", skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
    /// Paper height in millimeters.
    #[serde(with = "optional", skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
    /// Top margin in millimeters.
    #[serde(with = "optional", skip_serializing_if = "Option::is_none")]
    pub margin_top: Option<f64>,
    /// Bottom margin in millimeters.
    #[serde(with = "optional", skip_serializing_if = "Option::is_none")]
    pub margin_bottom: Option<f64>,
    /// Right margin in millimeters.
    #[serde(with = "optional", skip_serializing_if = "Option::is_none")]
    pub margin_right: Option<f64>,
    /// Left margin in millimeters.
    #[serde(with = "optional", skip_serializing_if = "Option::is_none")]
    pub margin_left: Option<f64>,
    /// The page orientation.
    #[serde(with = "optional", skip_serializing_if = "Option::is_none")]
    pub orientation: Option<Orientation>,
}

impl PageSetup {
    /// Sets the ppdname.
    #[must_use]
    pub fn set_ppdname<'a>(mut self, ppdname: impl Into<Option<&'a str>>) -> Self {
        self.ppdname = ppdname.into().map(ToOwned::to_owned);
        self
    }

    /// Sets the name of the page setup.
    #[must_use]
    pub fn set_name<'a>(mut self, name: impl Into<Option<&'a str>>) -> Self {
        self.name = name.into().map(ToOwned::to_owned);
        self
    }

    /// Sets the user visible name of the page setup.
    #[must_use]
    pub fn set_display_name<'a>(mut self, display_name: impl Into<Option<&'a str>>) -> Self {
        self.display_name = display_name.into().map(ToOwned::to_owned);
        self
    }

    /// Sets the orientation.
    #[must_use]
    pub fn set_orientation(mut self, orientation: impl Into<Option<Orientation>>) -> Self {
        self.orientation = orientation.into();
        self
    }

    /// Sets the page width.
    #[must_use]
    pub fn set_width(mut self, width: impl Into<Option<f64>>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the page height.
    #[must_use]
    pub fn set_height(mut self, height: impl Into<Option<f64>>) -> Self {
        self.height = height.into();
        self
    }

    /// Sets the page top margin.
    #[must_use]
    pub fn set_margin_top(mut self, margin_top: impl Into<Option<f64>>) -> Self {
        self.margin_top = margin_top.into();
        self
    }

    /// Sets the page bottom margin.
    #[must_use]
    pub fn set_margin_bottom(mut self, margin_bottom: impl Into<Option<f64>>) -> Self {
        self.margin_bottom = margin_bottom.into();
        self
    }

    /// Sets the page right margin.
    #[must_use]
    pub fn set_margin_right(mut self, margin_right: impl Into<Option<f64>>) -> Self {
        self.margin_right = margin_right.into();
        self
    }

    /// Sets the page margin left.
    #[must_use]
    pub fn set_margin_left(mut self, margin_left: impl Into<Option<f64>>) -> Self {
        self.margin_left = margin_left.into();
        self
    }
}

#[derive(Serialize, Deserialize, Type, Debug, Default)]
/// Specified options for a [`PrintProxy::prepare_print`] request.
#[zvariant(signature = "dict")]
#[serde(rename_all = "kebab-case")]
pub struct PreparePrintOptions {
    /// A string that will be used as the last element of the handle.
    #[serde(rename = "handle_token", with = "as_value", skip_deserializing)]
    handle_token: HandleToken,
    /// Whether to make the dialog modal.
    #[serde(default, with = "optional", skip_serializing_if = "Option::is_none")]
    modal: Option<bool>,
    /// Label for the accept button. Mnemonic underlines are allowed.
    #[serde(default, with = "optional", skip_serializing_if = "Option::is_none")]
    accept_label: Option<String>,
    /// File formats supported by the app for print-to-file.
    /// Added in version 3.
    #[serde(
        default,
        with = "as_value",
        skip_serializing_if = "Vec::is_empty",
        skip_deserializing
    )]
    supported_output_file_formats: Vec<OutputFileFormat>,
    /// Whether it makes sense to return "current" for the print-pages setting.
    /// Added in version 4.
    #[serde(
        default,
        with = "optional",
        skip_serializing_if = "Option::is_none",
        skip_deserializing
    )]
    has_current_page: Option<bool>,
    /// Whether it makes sense to return "selection" for the print-pages
    /// setting. Added in version 4.
    #[serde(
        default,
        with = "optional",
        skip_serializing_if = "Option::is_none",
        skip_deserializing
    )]
    has_selected_pages: Option<bool>,
}

impl PreparePrintOptions {
    /// Sets whether the dialog should be a modal.
    #[must_use]
    pub fn set_modal(mut self, modal: impl Into<Option<bool>>) -> Self {
        self.modal = modal.into();
        self
    }

    /// Gets whether the dialog should be a modal.
    #[cfg(feature = "backend")]
    pub fn modal(&self) -> Option<bool> {
        self.modal
    }

    /// Sets the label for the accept button. Mnemonic underlines are allowed.
    #[must_use]
    pub fn set_accept_label<'a>(mut self, accept_label: impl Into<Option<&'a str>>) -> Self {
        self.accept_label = accept_label.into().map(ToOwned::to_owned);
        self
    }

    /// Gets the label for the accept button.
    #[cfg(feature = "backend")]
    pub fn accept_label(&self) -> Option<&str> {
        self.accept_label.as_deref()
    }

    /// Sets the supported output file formats for print-to-file.
    /// Added in version 3.
    #[must_use]
    pub fn set_supported_output_file_formats(
        mut self,
        formats: impl IntoIterator<Item = OutputFileFormat>,
    ) -> Self {
        self.supported_output_file_formats = formats.into_iter().collect();
        self
    }

    /// Sets whether it makes sense to return "current" for print-pages.
    /// Added in version 4.
    #[must_use]
    pub fn set_has_current_page(mut self, has_current_page: impl Into<Option<bool>>) -> Self {
        self.has_current_page = has_current_page.into();
        self
    }

    /// Sets whether it makes sense to return "selection" for print-pages.
    /// Added in version 4.
    #[must_use]
    pub fn set_has_selected_pages(mut self, has_selected_pages: impl Into<Option<bool>>) -> Self {
        self.has_selected_pages = has_selected_pages.into();
        self
    }
}

#[derive(Serialize, Deserialize, Type, Debug, Default)]
/// Specified options for a [`PrintProxy::print`] request.
#[zvariant(signature = "dict")]
#[serde(rename_all = "kebab-case")]
pub struct PrintOptions {
    /// A string that will be used as the last element of the handle.
    #[serde(rename = "handle_token", with = "as_value", skip_deserializing)]
    handle_token: HandleToken,
    /// Whether to make the dialog modal.
    #[serde(default, with = "optional", skip_serializing_if = "Option::is_none")]
    modal: Option<bool>,
    /// Token that was returned by a previous [`PrintProxy::prepare_print`]
    /// call.
    #[serde(default, with = "optional", skip_serializing_if = "Option::is_none")]
    token: Option<u32>,
    /// File formats supported by the app for print-to-file.
    /// Added in version 3.
    #[serde(
        default,
        with = "as_value",
        skip_serializing_if = "Vec::is_empty",
        skip_deserializing
    )]
    supported_output_file_formats: Vec<OutputFileFormat>,
}

impl PrintOptions {
    /// Sets the token retrieved from [`PrintProxy::prepare_print`].
    #[must_use]
    pub fn set_token(mut self, token: impl Into<Option<u32>>) -> Self {
        self.token = token.into();
        self
    }

    /// Gets the token.
    #[cfg(feature = "backend")]
    pub fn token(&self) -> Option<u32> {
        self.token
    }

    /// Sets whether the dialog should be a modal.
    #[must_use]
    pub fn set_modal(mut self, modal: impl Into<Option<bool>>) -> Self {
        self.modal = modal.into();
        self
    }

    /// Gets whether the dialog should be a modal.
    #[cfg(feature = "backend")]
    pub fn modal(&self) -> Option<bool> {
        self.modal
    }

    /// Sets the supported output file formats for print-to-file.
    /// Added in version 3.
    #[must_use]
    pub fn set_supported_output_file_formats(
        mut self,
        formats: impl IntoIterator<Item = OutputFileFormat>,
    ) -> Self {
        self.supported_output_file_formats = formats.into_iter().collect();
        self
    }
}

#[derive(Deserialize, Serialize, Type, Debug)]
/// A response to a [`PrintProxy::prepare_print`] request.
#[zvariant(signature = "dict")]
pub struct PreparePrint {
    /// The printing settings.
    #[serde(with = "as_value")]
    pub settings: Settings,
    /// The printed pages setup.
    #[serde(rename = "page-setup", with = "as_value")]
    pub page_setup: PageSetup,
    /// A token to pass to the print request.
    #[serde(with = "as_value")]
    pub token: u32,
}

/// The interface lets sandboxed applications print.
///
/// Wrapper of the DBus interface: [`org.freedesktop.portal.Print`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Print.html).
#[derive(Debug)]
#[doc(alias = "org.freedesktop.portal.Print")]
pub struct PrintProxy(Proxy<'static>);

impl PrintProxy {
    /// Create a new instance of [`PrintProxy`].
    pub async fn new() -> Result<Self, Error> {
        let proxy = Proxy::new_desktop("org.freedesktop.portal.Print").await?;
        Ok(Self(proxy))
    }

    /// Create a new instance of [`PrintProxy`].
    pub async fn with_connection(connection: zbus::Connection) -> Result<Self, Error> {
        let proxy =
            Proxy::new_desktop_with_connection(connection, "org.freedesktop.portal.Print").await?;
        Ok(Self(proxy))
    }

    /// Returns the version of the portal interface.
    pub fn version(&self) -> u32 {
        self.0.version()
    }

    /// Presents a print dialog to the user and returns print settings and page
    /// setup.
    ///
    /// # Arguments
    ///
    /// * `identifier` - Identifier for the application window.
    /// * `title` - Title for the print dialog.
    /// * `settings` - [`Settings`].
    /// * `page_setup` - [`PageSetup`].
    /// * `options` - [`PreparePrintOptions`]. allowed.
    ///
    /// # Specifications
    ///
    /// See also [`PreparePrint`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Print.html#org-freedesktop-portal-print-prepareprint).
    #[doc(alias = "PreparePrint")]
    #[doc(alias = "xdp_portal_prepare_print")]
    pub async fn prepare_print(
        &self,
        identifier: Option<&WindowIdentifier>,
        title: &str,
        settings: Settings,
        page_setup: PageSetup,
        mut options: PreparePrintOptions,
    ) -> Result<Request<PreparePrint>, Error> {
        let version = self.version();

        // Clear version-specific fields if not supported
        if version < 2 {
            options.accept_label = None;
        }
        if version < 3 {
            options.supported_output_file_formats.clear();
        }
        if version < 4 {
            options.has_current_page = None;
            options.has_selected_pages = None;
        }

        let identifier = Optional::from(identifier);
        self.0
            .request(
                &options.handle_token,
                "PreparePrint",
                &(identifier, title, settings, page_setup, &options),
            )
            .await
    }

    /// Asks to print a file.
    /// The file must be passed in the form of a file descriptor open for
    /// reading. This ensures that sandboxed applications only print files
    /// that they have access to.
    ///
    /// # Arguments
    ///
    /// * `identifier` - The application window identifier.
    /// * `title` - The title for the print dialog.
    /// * `fd` - File descriptor for reading the content to print.
    /// * `options` - [`PrintOptions`].
    ///
    /// # Specifications
    ///
    /// See also [`Print`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Print.html#org-freedesktop-portal-print-print).
    #[doc(alias = "Print")]
    #[doc(alias = "xdp_portal_print_file")]
    pub async fn print(
        &self,
        identifier: Option<&WindowIdentifier>,
        title: &str,
        fd: &impl AsFd,
        mut options: PrintOptions,
    ) -> Result<Request<()>, Error> {
        let version = self.version();

        // Clear version-specific fields if not supported
        if version < 3 {
            options.supported_output_file_formats.clear();
        }

        let identifier = Optional::from(identifier);

        self.0
            .empty_request(
                &options.handle_token,
                "Print",
                &(identifier, title, Fd::from(fd), &options),
            )
            .await
    }
}

impl std::ops::Deref for PrintProxy {
    type Target = zbus::Proxy<'static>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
