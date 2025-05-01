/// Configuration for a context server.
#[derive(Debug, Clone)]
pub struct ContextServerConfiguration {
    /// Installation instructions for the user.
    pub installation_instructions: String,
    /// Default settings for the context server.
    pub default_settings: String,
    /// JSON schema describing server settings.
    pub settings_schema: serde_json::Value,
}
