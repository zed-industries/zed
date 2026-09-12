//! Style primitives shared between GPUI and its platform backends.

use crate::{Hsla, Pixels};
use refineable::Refineable;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// The properties that can be applied to an underline.
#[derive(
    Refineable, Copy, Clone, Default, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema,
)]
pub struct UnderlineStyle {
    /// The thickness of the underline.
    pub thickness: Pixels,

    /// The color of the underline.
    pub color: Option<Hsla>,

    /// Whether the underline should be wavy, like in a spell checker.
    pub wavy: bool,
}
