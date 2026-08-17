//! Style types for float layout

/// Floats a box to the left or right.
/// This property only applies to children of a block layout
///
/// See <https://developer.mozilla.org/en-US/docs/Web/CSS/float>
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Float {
    /// The box is floated to the left
    Left,
    /// The box is floated to the right
    Right,
    /// The box is not floated
    #[default]
    None,
}

#[cfg(feature = "parse")]
crate::util::parse::impl_parse_for_keyword_enum!(Float,
    "left" => Left,
    "right" => Right,
    "none" => None,
);

/// Whether a box that is definitely floated is floated to the left
/// of to the right.
///
/// This type is only used in the low-level parts of the
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u8)]
pub enum FloatDirection {
    /// The box is floated to the left
    Left = 0,
    /// The box is floated to the right
    Right = 1,
}

impl Float {
    /// Whether the box is floated
    pub fn is_floated(self) -> bool {
        matches!(self, Self::Left | Self::Right)
    }

    /// Converts [`Float`] into `Option<FloatDirection>`ca
    pub fn float_direction(&self) -> Option<FloatDirection> {
        match self {
            Float::Left => Some(FloatDirection::Left),
            Float::Right => Some(FloatDirection::Right),
            Float::None => None,
        }
    }
}

/// Gives a box "clearance", which moves it below floated boxes which precede
/// it in the tree.
///
/// See <https://developer.mozilla.org/en-US/docs/Web/CSS/clear>
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Clear {
    /// The box clears left-floated boxes
    Left,
    /// The box clears right-floated boxes
    Right,
    /// The box clears boxes floated in either direction
    Both,
    /// The box does not clear floated boxes
    #[default]
    None,
}

#[cfg(feature = "parse")]
crate::util::parse::impl_parse_for_keyword_enum!(Clear,
    "left" => Left,
    "right" => Right,
    "both" => Both,
    "none" => None,
);
