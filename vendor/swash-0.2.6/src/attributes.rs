//! Basic font attributes: stretch, weight and style.

use super::internal::{head::Os2, RawFont};
use super::{tag_from_bytes, FontRef, Setting, Tag};

use core::fmt;
use core::hash::{Hash, Hasher};

// Variations that apply to attributes.
const WDTH: Tag = tag_from_bytes(b"wdth");
const WGHT: Tag = tag_from_bytes(b"wght");
const SLNT: Tag = tag_from_bytes(b"slnt");
const ITAL: Tag = tag_from_bytes(b"ital");

/// Primary attributes for font classification: stretch, weight and style.
///
/// This struct is created by the [`attributes`](FontRef::attributes) method on [`FontRef`].
#[derive(Copy, Clone)]
pub struct Attributes(pub u32);

impl Attributes {
    /// Creates new font attributes from the specified stretch, weight and
    /// style.
    pub const fn new(stretch: Stretch, weight: Weight, style: Style) -> Self {
        let stretch = stretch.0 as u32 & 0x1FF;
        let weight = weight.0 as u32 & 0x3FF;
        let style = style.pack();
        Self(style | weight << 9 | stretch << 19)
    }

    /// Extracts the attributes from the specified font.
    pub fn from_font<'a>(font: &FontRef<'a>) -> Self {
        let mut attrs = Self::from_os2(font.os2().as_ref());
        let mut var_bits = 0;
        for var in font.variations() {
            match var.tag() {
                WDTH => var_bits |= 1,
                WGHT => var_bits |= 2,
                SLNT => var_bits |= 4,
                ITAL => var_bits |= 8,
                _ => {}
            }
        }
        attrs.0 |= var_bits << 28;
        attrs
    }

    pub(crate) fn from_os2(os2: Option<&Os2>) -> Self {
        if let Some(os2) = os2 {
            let flags = os2.selection_flags();
            let style = if flags.italic() {
                Style::Italic
            } else if flags.oblique() {
                Style::Oblique(ObliqueAngle::default())
            } else {
                Style::Normal
            };
            let weight = Weight(os2.weight_class() as u16);
            let stretch = Stretch::from_raw(os2.width_class() as u16);
            Self::new(stretch, weight, style)
        } else {
            Self::default()
        }
    }

    /// Returns the stretch attribute.
    pub fn stretch(&self) -> Stretch {
        Stretch((self.0 >> 19 & 0x1FF) as u16)
    }

    /// Returns the weight attribute.
    pub fn weight(&self) -> Weight {
        Weight((self.0 >> 9 & 0x3FF) as u16)
    }

    /// Returns the style attribute.
    pub fn style(&self) -> Style {
        Style::unpack(self.0 & 0x1FF)
    }

    /// Returns a tuple containing all attributes.
    pub fn parts(&self) -> (Stretch, Weight, Style) {
        (self.stretch(), self.weight(), self.style())
    }

    /// Returns true if the font has variations corresponding to primary
    /// attributes.
    pub fn has_variations(&self) -> bool {
        (self.0 >> 28) != 0
    }

    /// Returns true if the font has a variation for the stretch attribute.
    pub fn has_stretch_variation(&self) -> bool {
        let var_bits = self.0 >> 28;
        var_bits & 1 != 0
    }

    /// Returns true if the font has a variation for the weight attribute.
    pub fn has_weight_variation(&self) -> bool {
        let var_bits = self.0 >> 28;
        var_bits & 2 != 0
    }

    /// Returns true if the font has a variation for the oblique style
    /// attribute.
    pub fn has_oblique_variation(&self) -> bool {
        let var_bits = self.0 >> 28;
        var_bits & 4 != 0
    }

    /// Returns true if the font has a variation for the italic style
    /// attribute.
    pub fn has_italic_variation(&self) -> bool {
        let var_bits = self.0 >> 28;
        var_bits & 8 != 0
    }

    /// Returns a synthesis analysis based on the requested attributes with
    /// respect to this set of attributes.
    pub fn synthesize(&self, requested: Attributes) -> Synthesis {
        let mut synth = Synthesis::default();
        if self.0 << 4 == requested.0 << 4 {
            return synth;
        }
        let mut len = 0usize;
        if self.has_stretch_variation() {
            let stretch = self.stretch();
            let req_stretch = requested.stretch();
            if stretch != requested.stretch() {
                synth.vars[len] = Setting {
                    tag: WDTH,
                    value: req_stretch.to_percentage(),
                };
                len += 1;
            }
        }
        let (weight, req_weight) = (self.weight(), requested.weight());
        if weight != req_weight {
            if self.has_weight_variation() {
                synth.vars[len] = Setting {
                    tag: WGHT,
                    value: req_weight.0 as f32,
                };
                len += 1;
            } else if req_weight > weight {
                synth.embolden = true;
            }
        }
        let (style, req_style) = (self.style(), requested.style());
        if style != req_style {
            match req_style {
                Style::Normal => {}
                Style::Italic => {
                    if style == Style::Normal {
                        if self.has_italic_variation() {
                            synth.vars[len] = Setting {
                                tag: ITAL,
                                value: 1.,
                            };
                            len += 1;
                        } else if self.has_oblique_variation() {
                            synth.vars[len] = Setting {
                                tag: SLNT,
                                value: 14.,
                            };
                            len += 1;
                        } else {
                            synth.skew = 14;
                        }
                    }
                }
                Style::Oblique(angle) => {
                    if style == Style::Normal {
                        let degrees = angle.to_degrees();
                        if self.has_oblique_variation() {
                            synth.vars[len] = Setting {
                                tag: SLNT,
                                value: degrees,
                            };
                            len += 1;
                        } else if self.has_italic_variation() && degrees > 0. {
                            synth.vars[len] = Setting {
                                tag: ITAL,
                                value: 1.,
                            };
                            len += 1;
                        } else {
                            synth.skew = degrees as i8;
                        }
                    }
                }
            }
        }
        synth.len = len as u8;
        synth
    }
}

impl Default for Attributes {
    fn default() -> Self {
        Self::new(Stretch::NORMAL, Weight::NORMAL, Style::Normal)
    }
}

impl PartialEq for Attributes {
    fn eq(&self, other: &Self) -> bool {
        self.0 << 4 == other.0 << 4
    }
}

impl Eq for Attributes {}

impl Hash for Attributes {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.0 << 4).hash(state);
    }
}

impl fmt::Display for Attributes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut space = "";
        let (stretch, weight, style) = self.parts();
        if style == Style::Normal && weight == Weight::NORMAL && stretch == Stretch::NORMAL {
            return write!(f, "regular");
        }
        if stretch != Stretch::NORMAL {
            write!(f, "{}", stretch)?;
            space = " ";
        }
        if style != Style::Normal {
            write!(f, "{}{}", space, style)?;
            space = " ";
        }
        if weight != Weight::NORMAL {
            write!(f, "{}{}", space, weight)?;
        }
        Ok(())
    }
}

impl fmt::Debug for Attributes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.parts())?;
        if self.has_stretch_variation() {
            write!(f, "+wdth")?;
        }
        if self.has_weight_variation() {
            write!(f, "+wght")?;
        }
        if self.has_italic_variation() {
            write!(f, "+ital")?;
        }
        if self.has_oblique_variation() {
            write!(f, "+slnt")?;
        }
        Ok(())
    }
}

impl From<Stretch> for Attributes {
    fn from(s: Stretch) -> Self {
        Self::new(s, Weight::default(), Style::default())
    }
}

impl From<Weight> for Attributes {
    fn from(w: Weight) -> Self {
        Self::new(Stretch::default(), w, Style::default())
    }
}

impl From<Style> for Attributes {
    fn from(s: Style) -> Self {
        Self::new(Stretch::default(), Weight::default(), s)
    }
}

impl From<()> for Attributes {
    fn from(_: ()) -> Self {
        Self::default()
    }
}

impl From<(Stretch, Weight, Style)> for Attributes {
    fn from(parts: (Stretch, Weight, Style)) -> Self {
        Self::new(parts.0, parts.1, parts.2)
    }
}

/// Angle of an oblique style in degrees from -90 to 90.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct ObliqueAngle(pub(crate) u8);

impl ObliqueAngle {
    /// Creates a new oblique angle from degrees.
    pub fn from_degrees(degrees: f32) -> Self {
        let a = degrees.clamp(-90., 90.) + 90.;
        Self(a as u8)
    }

    /// Creates a new oblique angle from radians.
    pub fn from_radians(radians: f32) -> Self {
        let degrees = radians * 180. / core::f32::consts::PI;
        Self::from_degrees(degrees)
    }

    /// Creates a new oblique angle from gradians.
    pub fn from_gradians(gradians: f32) -> Self {
        Self::from_degrees(gradians / 400. * 360.)
    }

    /// Creates a new oblique angle from turns.
    pub fn from_turns(turns: f32) -> Self {
        Self::from_degrees(turns * 360.)
    }

    /// Returns the oblique angle in degrees.
    pub fn to_degrees(self) -> f32 {
        self.0 as f32 - 90.
    }
}

impl Default for ObliqueAngle {
    fn default() -> Self {
        Self::from_degrees(14.)
    }
}

/// Visual style or 'slope' of a font.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Style {
    Normal,
    Italic,
    Oblique(ObliqueAngle),
}

impl Style {
    /// Parses a style from a CSS style value.
    pub fn parse(mut s: &str) -> Option<Self> {
        s = s.trim();
        Some(match s {
            "normal" => Self::Normal,
            "italic" => Self::Italic,
            "oblique" => Self::Oblique(ObliqueAngle::from_degrees(14.)),
            _ => {
                if s.starts_with("oblique ") {
                    s = s.get(8..)?;
                    if s.ends_with("deg") {
                        s = s.get(..s.len() - 3)?;
                        if let Ok(a) = s.trim().parse::<f32>() {
                            return Some(Self::Oblique(ObliqueAngle::from_degrees(a)));
                        }
                    } else if s.ends_with("grad") {
                        s = s.get(..s.len() - 4)?;
                        if let Ok(a) = s.trim().parse::<f32>() {
                            return Some(Self::Oblique(ObliqueAngle::from_gradians(a)));
                        }
                    } else if s.ends_with("rad") {
                        s = s.get(..s.len() - 3)?;
                        if let Ok(a) = s.trim().parse::<f32>() {
                            return Some(Self::Oblique(ObliqueAngle::from_radians(a)));
                        }
                    } else if s.ends_with("turn") {
                        s = s.get(..s.len() - 4)?;
                        if let Ok(a) = s.trim().parse::<f32>() {
                            return Some(Self::Oblique(ObliqueAngle::from_turns(a)));
                        }
                    }
                    return Some(Self::Oblique(ObliqueAngle::default()));
                }
                return None;
            }
        })
    }

    /// Creates a new oblique style with the specified angle
    /// in degrees.
    pub fn from_degrees(degrees: f32) -> Self {
        Self::Oblique(ObliqueAngle::from_degrees(degrees))
    }

    /// Returns the angle of the style in degrees.
    pub fn to_degrees(self) -> f32 {
        match self {
            Self::Italic => 14.,
            Self::Oblique(angle) => angle.to_degrees(),
            _ => 0.,
        }
    }

    fn unpack(bits: u32) -> Self {
        if bits & 1 != 0 {
            Self::Oblique(ObliqueAngle((bits >> 1) as u8))
        } else if bits == 0b110 {
            Self::Italic
        } else {
            Self::Normal
        }
    }

    const fn pack(&self) -> u32 {
        match self {
            Self::Normal => 0b10,
            Self::Italic => 0b110,
            Self::Oblique(angle) => 1 | (angle.0 as u32) << 1,
        }
    }
}

impl fmt::Display for Style {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Normal => "normal",
                Self::Italic => "italic",
                Self::Oblique(angle) => {
                    let degrees = angle.to_degrees();
                    if degrees == 14. {
                        "oblique"
                    } else {
                        return write!(f, "oblique({}deg)", degrees);
                    }
                }
            }
        )
    }
}

impl Default for Style {
    fn default() -> Self {
        Self::Normal
    }
}

/// Visual weight class of a font on a scale from 1 to 1000.
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Debug)]
pub struct Weight(pub u16);

impl Weight {
    pub const THIN: Weight = Weight(100);
    pub const EXTRA_LIGHT: Weight = Weight(200);
    pub const LIGHT: Weight = Weight(300);
    pub const NORMAL: Weight = Weight(400);
    pub const MEDIUM: Weight = Weight(500);
    pub const SEMI_BOLD: Weight = Weight(600);
    pub const BOLD: Weight = Weight(700);
    pub const EXTRA_BOLD: Weight = Weight(800);
    pub const BLACK: Weight = Weight(900);

    /// Parses a CSS style font weight attribute.
    pub fn parse(s: &str) -> Option<Self> {
        let s = s.trim();
        Some(match s {
            "normal" => Self::NORMAL,
            "bold" => Self::BOLD,
            _ => Self(s.parse::<u32>().ok()?.clamp(1, 1000) as u16),
        })
    }
}

impl fmt::Display for Weight {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            Self::THIN => "thin",
            Self::EXTRA_LIGHT => "extra-light",
            Self::LIGHT => "light",
            Self::NORMAL => "normal",
            Self::MEDIUM => "medium",
            Self::SEMI_BOLD => "semi-bold",
            Self::BOLD => "bold",
            Self::EXTRA_BOLD => "extra-bold",
            Self::BLACK => "black",
            _ => "",
        };
        if s.is_empty() {
            write!(f, "{}", self.0)
        } else {
            write!(f, "{}", s)
        }
    }
}

impl Default for Weight {
    fn default() -> Self {
        Self::NORMAL
    }
}

/// Visual width of a font-- a relative change from the normal aspect
/// ratio.
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct Stretch(pub(crate) u16);

impl Stretch {
    pub const ULTRA_CONDENSED: Self = Self(0);
    pub const EXTRA_CONDENSED: Self = Self(25);
    pub const CONDENSED: Self = Self(50);
    pub const SEMI_CONDENSED: Self = Self(75);
    pub const NORMAL: Self = Self(100);
    pub const SEMI_EXPANDED: Self = Self(125);
    pub const EXPANDED: Self = Self(150);
    pub const EXTRA_EXPANDED: Self = Self(200);
    pub const ULTRA_EXPANDED: Self = Self(300);

    /// Creates a stretch attribute from a percentage. The value will be
    /// clamped at half percentage increments between 50% and 200%,
    /// inclusive.
    pub fn from_percentage(percentage: f32) -> Self {
        let value = ((percentage.clamp(50., 200.) - 50.) * 2.) as u16;
        Self(value)
    }

    /// Converts the stretch value to a percentage.
    pub fn to_percentage(self) -> f32 {
        (self.0 as f32) * 0.5 + 50.
    }

    /// Returns true if the stretch is normal.
    pub fn is_normal(self) -> bool {
        self == Self::NORMAL
    }

    /// Returns true if the stretch is condensed (less than normal).
    pub fn is_condensed(self) -> bool {
        self < Self::NORMAL
    }

    /// Returns true if the stretch is expanded (greater than normal).
    pub fn is_expanded(self) -> bool {
        self > Self::NORMAL
    }

    /// Parses the stretch from a CSS style keyword or a percentage value.
    pub fn parse(s: &str) -> Option<Self> {
        let s = s.trim();
        Some(match s {
            "ultra-condensed" => Self::ULTRA_CONDENSED,
            "extra-condensed" => Self::EXTRA_CONDENSED,
            "condensed" => Self::CONDENSED,
            "semi-condensed" => Self::SEMI_CONDENSED,
            "normal" => Self::NORMAL,
            "semi-expanded" => Self::SEMI_EXPANDED,
            "extra-expanded" => Self::EXTRA_EXPANDED,
            "ultra-expanded" => Self::ULTRA_EXPANDED,
            _ => {
                if s.ends_with('%') {
                    let p = s.get(..s.len() - 1)?.parse::<f32>().ok()?;
                    return Some(Self::from_percentage(p));
                }
                return None;
            }
        })
    }

    /// Returns the raw value of the stretch attribute.
    pub fn raw(self) -> u16 {
        self.0
    }

    pub(crate) fn from_raw(raw: u16) -> Self {
        match raw {
            1 => Self::ULTRA_CONDENSED,
            2 => Self::EXTRA_CONDENSED,
            3 => Self::CONDENSED,
            4 => Self::SEMI_CONDENSED,
            5 => Self::NORMAL,
            6 => Self::SEMI_EXPANDED,
            7 => Self::EXPANDED,
            8 => Self::EXTRA_EXPANDED,
            9 => Self::ULTRA_EXPANDED,
            _ => Self::NORMAL,
        }
    }
}

impl fmt::Display for Stretch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Self::ULTRA_CONDENSED => "ultra-condensed",
                Self::EXTRA_CONDENSED => "extra-condensed",
                Self::CONDENSED => "condensed",
                Self::SEMI_CONDENSED => "semi-condensed",
                Self::NORMAL => "normal",
                Self::SEMI_EXPANDED => "semi-expanded",
                Self::EXPANDED => "expanded",
                Self::EXTRA_EXPANDED => "extra-expanded",
                Self::ULTRA_EXPANDED => "ultra-expanded",
                _ => {
                    return write!(f, "{}%", self.to_percentage());
                }
            }
        )
    }
}

impl fmt::Debug for Stretch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Stretch({})", self.to_percentage())
    }
}

impl Default for Stretch {
    fn default() -> Self {
        Self::NORMAL
    }
}

/// Synthesis suggestions for mismatched font attributes.
///
/// This is generated by the [`synthesize`](Attributes::synthesize) method on
/// [`Attributes`].
#[derive(Copy, Clone, Default, Debug)]
pub struct Synthesis {
    vars: [Setting<f32>; 4],
    len: u8,
    embolden: bool,
    skew: i8,
}

impl Synthesis {
    #[doc(hidden)]
    pub fn new(variations: impl Iterator<Item = Setting<f32>>, embolden: bool, skew: f32) -> Self {
        let mut synth = Self {
            embolden,
            skew: skew as i8,
            ..Default::default()
        };
        for (i, setting) in variations.take(4).enumerate() {
            synth.vars[i] = setting;
            synth.len = i as u8 + 1;
        }
        synth
    }

    /// Returns true if any synthesis suggestions are available.
    pub fn any(&self) -> bool {
        self.len != 0 || self.embolden || self.skew != 0
    }

    /// Returns the variations that should be applied to match the requested
    /// attributes.
    pub fn variations(&self) -> &[Setting<f32>] {
        &self.vars[..self.len as usize]
    }

    /// Returns true if the scaler should apply a faux bold.
    pub fn embolden(&self) -> bool {
        self.embolden
    }

    /// Returns a skew angle for faux italic/oblique, if requested.
    pub fn skew(&self) -> Option<f32> {
        if self.skew != 0 {
            Some(self.skew as f32)
        } else {
            None
        }
    }
}

impl PartialEq for Synthesis {
    fn eq(&self, other: &Self) -> bool {
        if self.len != other.len {
            return false;
        }
        if self.len != 0 && self.variations() != other.variations() {
            return false;
        }
        self.embolden == other.embolden && self.skew == other.skew
    }
}
