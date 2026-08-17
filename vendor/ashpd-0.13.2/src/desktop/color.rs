use serde::{Deserialize, Serialize};

use crate::zvariant::{self, Type, as_value};

#[derive(
    Serialize, Deserialize, Clone, Copy, PartialEq, Type, zvariant::Value, zvariant::OwnedValue,
)]
/// A color as a RGB tuple.
///
/// **Note** the values are normalized in the [0.0, 1.0] range.
#[zvariant(signature = "dict")]
pub struct Color {
    #[serde(with = "as_value")]
    color: (f64, f64, f64),
}

impl From<(f64, f64, f64)> for Color {
    fn from(value: (f64, f64, f64)) -> Self {
        Self::new(value.0, value.1, value.2)
    }
}

impl Color {
    /// Create a new instance of Color.
    pub fn new(red: f64, green: f64, blue: f64) -> Self {
        Self {
            color: (red, green, blue),
        }
    }

    /// Red.
    pub fn red(&self) -> f64 {
        self.color.0
    }

    /// Green.
    pub fn green(&self) -> f64 {
        self.color.1
    }

    /// Blue.
    pub fn blue(&self) -> f64 {
        self.color.2
    }
}

#[cfg(feature = "gtk4")]
impl From<Color> for gtk4::gdk::RGBA {
    fn from(color: Color) -> Self {
        gtk4::gdk::RGBA::builder()
            .red(color.red() as f32)
            .green(color.green() as f32)
            .blue(color.blue() as f32)
            .build()
    }
}

impl std::fmt::Debug for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Color")
            .field("red", &self.red())
            .field("green", &self.green())
            .field("blue", &self.blue())
            .finish()
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "({}, {}, {})",
            self.red(),
            self.green(),
            self.blue()
        ))
    }
}
