//! Basic predefined colors.
use super::{RGBAColor, RGBColor};

// Taken from https://stackoverflow.com/questions/60905060/prevent-line-break-in-doc-test
/// Macro for allowing dynamic creation of doc attributes.
#[macro_export]
macro_rules! doc {
    {
        $(#[$m:meta])*
        $(
            [$doc:expr]
            $(#[$n:meta])*
        )*
        @ $thing:item
    } => {
        $(#[$m])*
        $(
            #[doc = $doc]
            $(#[$n])*
        )*
        $thing
    }
}

/// Defines and names a color based on its R, G, B, A values.
#[macro_export]
macro_rules! define_color {
    ($name:ident, $r:expr, $g:expr, $b:expr, $doc:expr) => {
        doc! {
        [$doc]
        // Format a colored box that will show up in the docs
        [concat!("(<span style='color: rgb(",  $r,",", $g, ",", $b, "); background-color: #ddd; padding: 0 0.2em;'>■</span>" )]
        [concat!("*rgb = (", $r,", ", $g, ", ", $b, ")*)")]
        @pub const $name: RGBColor = RGBColor($r, $g, $b);
        }
    };

    ($name:ident, $r:expr, $g:expr, $b:expr, $a: expr, $doc:expr) => {
        doc! {
        [$doc]
        // Format a colored box that will show up in the docs
        [concat!("(<span style='color: rgba(",  $r,",", $g, ",", $b, ",", $a, "); background-color: #ddd; padding: 0 0.2em;'>■</span>" )]
        [concat!("*rgba = (", $r,", ", $g, ", ", $b, ", ", $a, ")*)")]
        @pub const $name: RGBAColor = RGBAColor($r, $g, $b, $a);
        }
    };
}

define_color!(WHITE, 255, 255, 255, "White");
define_color!(BLACK, 0, 0, 0, "Black");
define_color!(RED, 255, 0, 0, "Red");
define_color!(GREEN, 0, 255, 0, "Green");
define_color!(BLUE, 0, 0, 255, "Blue");
define_color!(YELLOW, 255, 255, 0, "Yellow");
define_color!(CYAN, 0, 255, 255, "Cyan");
define_color!(MAGENTA, 255, 0, 255, "Magenta");
define_color!(TRANSPARENT, 0, 0, 0, 0.0, "Transparent");

#[cfg(feature = "colormaps")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "colormaps")))]
/// Colormaps can be used to simply go from a scalar value to a color value which will be more/less
/// intense corresponding to the value of the supplied scalar.
/// These colormaps can also be defined by the user and be used with lower and upper bounds.
pub mod colormaps;
#[cfg(feature = "full_palette")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "full_palette")))]
pub mod full_palette;
