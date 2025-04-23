use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct ContextServer {
    pub id: Arc<str>,
    pub setup: ContextServerSetupInstructions,
}

#[derive(Debug, Clone)]
pub struct ContextServerSetupInstructions {
    pub installation_instructions: Arc<str>,
    pub settings_hint: Arc<str>,
}
