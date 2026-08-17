use crate::{Expression, Value};

macro_rules! define_property {
    (
        $(
            $(#[$attr:meta])*
            $variant:ident($value_ty:ident, $name:expr),
        )+
    ) => {
        #[derive(Clone, Debug, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub enum Property {
            $(
                $(#[$attr])*
                $variant(Expression),
            )+
            Dynamic(String, Expression),
        }

        impl Property {
            pub fn kind(&self) -> PropertyKind {
                match self {
                    $(
                        Property::$variant(_) => PropertyKind::$variant,
                    )+
                    Property::Dynamic(s, _) => PropertyKind::Dynamic(s.clone()),
                }
            }
        }

        #[derive(Clone, Debug, PartialEq, Eq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub enum PropertyKind {
            $(
                $(#[$attr])*
                $variant,
            )+
            Dynamic(String),
        }

        parse_enum! {
            PropertyKind,
            $(
                ($variant, $name),
            )+
            |s| Ok(PropertyKind::Dynamic(s.into())),
        }

        impl PropertyKind {
            pub fn make_property(self, expr: Expression) -> Property {
                match self {
                    $(
                        PropertyKind::$variant => Property::$variant(expr),
                    )+
                    PropertyKind::Dynamic(name) => Property::Dynamic(name.clone(), expr),
                }
            }
        }
    };
}

define_property! {
    /// Font family names
    Family(String, "family"),
    /// Languages corresponding to each family
    FamilyLang(String, "familylang"),
    /// Font style. Overrides weight and slant
    Style(String, "style"),
    /// Languages corresponding to each style
    StyleLang(String, "stylelang"),
    /// Font full names (often includes style)
    FullName(String, "fullname"),
    /// Languages corresponding to each fullname
    FullNameLang(String, "fullnamelang"),

    /// Italic, oblique or roman
    Slant(Int, "slant"),
    /// Light, medium, demibold, bold or black
    Weight(Int, "weight"),
    /// Point size
    Size(Double, "size"),
    /// Condensed, normal or expanded
    Width(Int, "width"),
    /// Stretches glyphs horizontally before hinting
    Aspect(Double, "aspect"),
    /// Pixel size
    PixelSize(Double, "pixelsize"),
    /// Proportional, dual-width, monospace or charcell
    Spacing(Int, "spacing"),
    /// Font foundry name
    Foundry(String, "foundry"),
    /// Whether glyphs can be antialiased
    Antialias(Bool, "antialias"),
    /// Whether the rasterizer should use hinting
    Hinting(Bool, "hinting"),
    /// Automatic hinting style
    HintStyle(Int, "hintstyle"),
    /// Automatic hinting style
    VerticalLayout(Bool, "verticallayout"),
    /// Use autohinter instead of normal hinter
    AutoHint(Bool, "autohint"),
    /// Use font global advance data (deprecated)
    GlobalAdvance(Bool, "globaladvance"),

    /// The filename holding the font
    File(String, "file"),
    /// The index of the font within the file
    Index(Int, "index"),
    // TODO:
    // /// Use the specified FreeType face object
    // Ftface(FT_Face),
    /// Which rasterizer is in use (deprecated)
    Rasterizer(String, "rasterizer"),
    /// Whether the glyphs are outlines
    Outline(Bool, "outline"),
    /// Whether glyphs can be scaled
    Scalable(Bool, "scalable"),
    /// Whether any glyphs have color
    Color(Bool, "color"),
    /// Scale factor for point->pixel conversions (deprecated)
    Scale(Double, "scale"),
    /// Target dots per inch
    Dpi(Double, "dpi"),
    /// unknown, rgb, bgr, vrgb, vbgr, none - subpixel geometry
    Rgba(Int, "rgba"),
    /// Type of LCD filter
    Lcdfilter(Int, "lcdfilter"),
    /// Eliminate leading from line spacing
    Minspace(Bool, "minspace"),
    /// Unicode chars encoded by the font
    Charset(CharSet, "charset"),
    /// List of RFC-3066-style languages this font supports
    Lang(String, "lang"),
    /// Version number of the font
    Fontversion(Int, "fontversion"),
    /// List of layout capabilities in the font
    Capability(String, "capability"),
    /// String name of the font format
    Fontformat(String, "fontformat"),
    /// Rasterizer should synthetically embolden the font
    Embolden(Bool, "embolden"),
    /// Use the embedded bitmap instead of the outline
    Embeddedbitmap(Bool, "embeddedbitmap"),
    /// Whether the style is a decorative variant
    Decorative(Bool, "decorative"),
    /// List of the feature tags in OpenType to be enabled
    Fontfeatures(String, "fontfeatures"),
    /// Language name to be used for the default value of familylang, stylelang, and fullnamelang
    Namelang(String, "namelang"),
    /// String  Name of the running program
    Prgname(String, "prgname"),
    /// Font family name in PostScript
    Postscriptname(String, "postscriptname"),
    /// Whether the font has hinting
    Fonthashint(Bool, "fonthashint"),
    /// Order number of the font
    Order(Int, "order"),

    // custom

    Matrix(Matrix, "matrix"),
    PixelSizeFixupFactor(Double, "pixelsizefixupfactor"),
    ScalingNotNeeded(Bool, "scalingnotneeded"),
}

impl Default for Property {
    fn default() -> Self {
        Property::Family(Expression::Simple(Value::String(String::default())))
    }
}

impl Default for PropertyKind {
    fn default() -> Self {
        PropertyKind::Family
    }
}
