use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsSources};

#[derive(Copy, Clone, Debug, Default, Serialize, PartialEq, Eq, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum Gpu {
    Discrete,
    #[default]
    Integrated,
}

impl Into<gpui::Gpu> for Gpu {
    fn into(self) -> gpui::Gpu {
        match self {
            Gpu::Discrete => gpui::Gpu::Discrete,
            Gpu::Integrated => gpui::Gpu::Integrated,
        }
    }
}

/// Settings related to the machine's GPU and how Zed uses it.
#[derive(Copy, Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Default)]
pub struct GpuSettings {
    /// Which GPU to prefer: discrete or integrated.
    /// Integrated are often lower-power and thus more battery-efficient.
    ///
    /// Default: 0.2
    pub gpu: Option<Gpu>,
}

impl Settings for GpuSettings {
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
