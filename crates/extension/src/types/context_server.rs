/// Configuration for a context server
#[derive(Debug, Clone)]
pub struct ContextServerConfiguration {
    /// Installation instructions for the user
    pub installation_instructions: String,
    /// JSON schema describing server settings
    pub settings_schema: String,
}
