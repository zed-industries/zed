mod ui;
mod provider;
mod model;
mod types;
mod utils;

pub use provider::LmStudioLanguageModelProvider;
pub use model::LmStudioLanguageModel;
pub use types::{LmStudioSettings, LmStudioServer, AvailableModel};

// Constants
pub const LMSTUDIO_DOWNLOAD_URL: &str = "https://lmstudio.ai/download";
pub const LMSTUDIO_CATALOG_URL: &str = "https://lmstudio.ai/models";
pub const LMSTUDIO_SITE: &str = "https://lmstudio.ai/";
pub const PROVIDER_ID: &str = "lmstudio";
pub const PROVIDER_NAME: &str = "LM Studio"; 