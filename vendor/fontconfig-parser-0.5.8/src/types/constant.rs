use crate::PropertyKind;

macro_rules! define_constant {
    (
        $(
            $variant:ident = $(($ty:ident, $value:expr),)+
        )+
    ) => {
        #[derive(Copy, Clone, Debug, Eq, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub enum Constant {
            $(
                $variant,
            )+
        }

        impl Constant {
            pub fn get_value(self, kind: PropertyKind) -> Option<u32> {
                match (self, kind) {
                    $(
                        $(
                            (Constant::$variant, PropertyKind::$ty) => Some($value),
                        )+
                    )+
                    _ => None,
                }
            }
        }
    };
}

define_constant! {
    Thin = (Weight, 0),
    Extralight = (Weight, 40),
    Ultralight = (Weight, 40),
    Light = (Weight, 50),
    Demilight = (Weight, 55),
    Semilight = (Weight, 55),
    Book = (Weight, 75),
    Regular = (Weight, 80),
    Normal = (Weight, 80), (Width, 100),
    Medium = (Weight, 100),
    Demibold = (Weight, 180),
    Semibold = (Weight, 180),
    Bold = (Weight, 200),
    Extrabold = (Weight, 205),
    Black = (Weight, 210),
    Heavy = (Weight, 210),
    Roman = (Slant, 0),
    Italic = (Slant, 100),
    Oblique = (Slant, 110),
    Ultracondensed = (Width, 50),
    Extracondensed = (Width, 63),
    Condensed = (Width, 75),
    Semicondensed = (Width, 87),
    // Merged into above Normal
    // Normal = (Width, 100),
    Semiexpanded = (Width, 113),
    Expanded = (Width, 125),
    Extraexpanded = (Width, 150),
    Ultraexpanded = (Width, 200),
    Proportional = (Spacing, 0),
    Dual = (Spacing, 90),
    Mono = (Spacing, 100),
    Charcell = (Spacing, 110),
    Unknown = (Rgba, 0),
    Rgb = (Rgba, 1),
    Bgr = (Rgba, 2),
    Vrgb = (Rgba, 3),
    Vbgr = (Rgba, 4),
    None = (Rgba, 5),
    Lcdnone = (Lcdfilter, 0),
    Lcddefault = (Lcdfilter, 1),
    Lcdlight = (Lcdfilter, 2),
    Lcdlegacy = (Lcdfilter, 3),
    Hintnone = (HintStyle, 0),
    Hintslight = (HintStyle, 1),
    Hintmedium = (HintStyle, 2),
    Hintfull = (HintStyle, 3),
}

parse_enum! {
    Constant,
    (Thin, "thin"),
    (Extralight, "extralight"),
    (Ultralight, "ultralight"),
    (Light, "light"),
    (Demilight, "demilight"),
    (Semilight, "semilight"),
    (Book, "book"),
    (Regular, "regular"),
    (Normal, "normal"),
    (Medium, "medium"),
    (Demibold, "demibold"),
    (Semibold, "semibold"),
    (Bold, "bold"),
    (Extrabold, "extrabold"),
    (Black, "black"),
    (Heavy, "heavy"),
    (Roman, "roman"),
    (Italic, "italic"),
    (Oblique, "oblique"),
    (Ultracondensed, "ultracondensed"),
    (Extracondensed, "extracondensed"),
    (Condensed, "condensed"),
    (Semicondensed, "semicondensed"),
    (Semiexpanded, "semiexpanded"),
    (Expanded, "expanded"),
    (Extraexpanded, "extraexpanded"),
    (Ultraexpanded, "ultraexpanded"),
    (Proportional, "proportional"),
    (Dual, "dual"),
    (Mono, "mono"),
    (Charcell, "charcell"),
    (Unknown, "unknown"),
    (Rgb, "rgb"),
    (Bgr, "bgr"),
    (Vrgb, "vrgb"),
    (Vbgr, "vbgr"),
    (None, "none"),
    (Lcdnone, "lcdnone"),
    (Lcddefault, "lcddefault"),
    (Lcdlight, "lcdlight"),
    (Lcdlegacy, "lcdlegacy"),
    (Hintnone, "hintnone"),
    (Hintslight, "hintslight"),
    (Hintmedium, "hintmedium"),
    (Hintfull, "hintfull"),
}

#[test]
fn convert_test() {
    assert_eq!(Constant::Roman.get_value(PropertyKind::Slant).unwrap(), 0,);
}
