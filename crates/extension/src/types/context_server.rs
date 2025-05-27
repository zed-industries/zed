/// Configuration for context server setup and installation.
#[derive(Debug, Clone)]
pub struct ContextServerConfiguration {
    /// Installation instructions in Markdown format.
    pub installation_instructions: String,
    /// JSON schema for settings validation.
    pub settings_schema: serde_json::Value,
    /// Default settings template.
    pub default_settings: String,
}
