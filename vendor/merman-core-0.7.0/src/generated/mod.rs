use crate::MermaidConfig;
use std::sync::OnceLock;

pub mod dompurify_defaults;

static DEFAULT_SITE_CONFIG: OnceLock<MermaidConfig> = OnceLock::new();

pub fn default_site_config() -> MermaidConfig {
    DEFAULT_SITE_CONFIG
        .get_or_init(|| {
            let json_text = include_str!("default_config.json");
            let value: serde_json::Value =
                serde_json::from_str(json_text).expect("generated default config JSON is valid");
            MermaidConfig::from_value(value)
        })
        .clone()
}
