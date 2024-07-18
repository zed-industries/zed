use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsSources};

#[derive(Copy, Clone, Debug, Default, Serialize, PartialEq, Eq, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum GPU {
    Discrete,
    #[default]
    Integrated,
}

impl Into<gpui::GPU> for GPU {
    fn into(self) -> gpui::GPU {
        match self {
            GPU::Discrete => gpui::GPU::Discrete,
            GPU::Integrated => gpui::GPU::Integrated,
        }
    }
}

/// Settings related to the machine's GPU and how Zed uses it.
#[derive(Copy, Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Default)]
pub struct GPUSettings {
    /// Which GPU to prefer: discrete or integrated.
    /// Integrated are often lower-power and thus more battery-efficient.
    ///
    /// Default: 0.2
    pub gpu: Option<GPU>,
}

impl Settings for GPUSettings {
    const KEY: Option<&'static str> = None;

    type FileContent = Option<Self>;

    fn load(
        sources: SettingsSources<Self::FileContent>,
        _: &mut gpui::AppContext,
    ) -> anyhow::Result<Self> {
        if let Some(Some(user_value)) = sources.user.copied() {
            return Ok(user_value);
        }
        Ok(Self::default())
    }
}
