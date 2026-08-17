use core::{num::NonZeroU32, ops::Range};

#[cfg(feature = "deserialize")]
use serde::Deserialize;
#[cfg(feature = "serialize")]
use serde::Serialize;

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
pub enum Coord {
    #[default]
    Normalized,
    Pixel,
}

impl Coord {
    pub const fn as_str(&self) -> &'static str {
        match *self {
            Self::Normalized => "normalized",
            Self::Pixel => "pixel",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
pub enum Address {
    Repeat,
    MirroredRepeat,
    #[default]
    ClampToEdge,
    ClampToZero,
    ClampToBorder,
}

impl Address {
    pub const fn as_str(&self) -> &'static str {
        match *self {
            Self::Repeat => "repeat",
            Self::MirroredRepeat => "mirrored_repeat",
            Self::ClampToEdge => "clamp_to_edge",
            Self::ClampToZero => "clamp_to_zero",
            Self::ClampToBorder => "clamp_to_border",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
pub enum BorderColor {
    #[default]
    TransparentBlack,
    OpaqueBlack,
    OpaqueWhite,
}

impl BorderColor {
    pub const fn as_str(&self) -> &'static str {
        match *self {
            Self::TransparentBlack => "transparent_black",
            Self::OpaqueBlack => "opaque_black",
            Self::OpaqueWhite => "opaque_white",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
pub enum Filter {
    #[default]
    Nearest,
    Linear,
}

impl Filter {
    pub const fn as_str(&self) -> &'static str {
        match *self {
            Self::Nearest => "nearest",
            Self::Linear => "linear",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
pub enum CompareFunc {
    #[default]
    Never,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Equal,
    NotEqual,
    Always,
}

impl CompareFunc {
    pub const fn as_str(&self) -> &'static str {
        match *self {
            Self::Never => "never",
            Self::Less => "less",
            Self::LessEqual => "less_equal",
            Self::Greater => "greater",
            Self::GreaterEqual => "greater_equal",
            Self::Equal => "equal",
            Self::NotEqual => "not_equal",
            Self::Always => "always",
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
pub struct InlineSampler {
    pub coord: Coord,
    pub address: [Address; 3],
    pub border_color: BorderColor,
    pub mag_filter: Filter,
    pub min_filter: Filter,
    pub mip_filter: Option<Filter>,
    pub lod_clamp: Option<Range<f32>>,
    pub max_anisotropy: Option<NonZeroU32>,
    pub compare_func: CompareFunc,
}

impl Eq for InlineSampler {}

impl core::hash::Hash for InlineSampler {
    fn hash<H: core::hash::Hasher>(&self, hasher: &mut H) {
        self.coord.hash(hasher);
        self.address.hash(hasher);
        self.border_color.hash(hasher);
        self.mag_filter.hash(hasher);
        self.min_filter.hash(hasher);
        self.mip_filter.hash(hasher);
        self.lod_clamp
            .as_ref()
            .map(|range| (range.start.to_bits(), range.end.to_bits()))
            .hash(hasher);
        self.max_anisotropy.hash(hasher);
        self.compare_func.hash(hasher);
    }
}
