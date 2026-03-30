use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Specifies the density of the UI.
/// Note: This setting is still experimental. See [this tracking issue](https://github.com/zed-industries/zed/issues/18078)
#[derive(
    Debug,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Clone,
    Copy,
    Serialize,
    Deserialize,
    JsonSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum UiDensity {
    /// A denser UI with tighter spacing and smaller elements.
    #[serde(alias = "compact")]
    Compact,
    #[default]
    #[serde(alias = "default")]
    /// The default UI density.
    Default,
    #[serde(alias = "comfortable")]
    /// A looser UI with more spacing and larger elements.
    Comfortable,
}

impl UiDensity {
    /// The spacing ratio of a given density.
    /// TODO: Standardize usage throughout the app or remove
    pub fn spacing_ratio(self) -> f32 {
        match self {
            UiDensity::Compact => 0.75,
            UiDensity::Default => 1.0,
            UiDensity::Comfortable => 1.25,
        }
    }
}

impl From<String> for UiDensity {
    fn from(s: String) -> Self {
        match s.as_str() {
            "compact" => Self::Compact,
            "default" => Self::Default,
            "comfortable" => Self::Comfortable,
            _ => Self::default(),
        }
    }
}

impl From<UiDensity> for String {
    fn from(val: UiDensity) -> Self {
        match val {
            UiDensity::Compact => "compact".to_string(),
            UiDensity::Default => "default".to_string(),
            UiDensity::Comfortable => "comfortable".to_string(),
        }
    }
}
