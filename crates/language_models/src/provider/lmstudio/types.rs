use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq)]
pub struct LmStudioSettings {
    pub servers: Vec<LmStudioServer>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct LmStudioServer {
    pub id: String,
    pub name: String,
    pub api_url: String,
    pub enabled: bool,
    pub available_models: Option<Vec<AvailableModel>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct AvailableModel {
    pub name: String,
    pub display_name: Option<String>,
    pub server_max_tokens: usize,
    pub custom_max_tokens: Option<usize>,
    pub server_id: Option<String>,
    #[serde(default = "default_true")]
    pub enabled: bool,
    
    #[serde(default)]
    #[serde(skip_serializing_if = "max_tokens_is_default")]
    #[deprecated(note = "Use server_max_tokens and custom_max_tokens instead")]
    #[doc(hidden)]
    max_tokens: usize,
}

// Helper functions
fn default_true() -> bool {
    true
}

fn max_tokens_is_default(max_tokens: &usize) -> bool {
    *max_tokens == 0
}

// Implementation for LmStudioSettings
impl LmStudioSettings {
    pub fn default_with_legacy() -> Self {
        Self {
            servers: Vec::new(),
        }
    }
    
    pub fn migrate_from_legacy(legacy_api_url: &str) -> Self {
        // Don't automatically create a default server from legacy settings
        // Users should manually configure their LM Studio servers instead
        log::debug!("Migrating from legacy LM Studio api_url '{}' - not creating default server", legacy_api_url);
        Self {
            servers: Vec::new(),
        }
    }
    
    pub fn first_enabled_server(&self) -> Option<&LmStudioServer> {
        self.servers.iter().find(|server| server.enabled)
    }
}

// Implementation for LmStudioServer
impl LmStudioServer {
    pub async fn healthcheck(&self, http_client: &dyn http_client::HttpClient) -> anyhow::Result<bool> {
        if !self.enabled {
            return Ok(false);
        }
        
        log::info!("Performing healthcheck for server {} at {}", self.name, self.api_url);
        match lmstudio::healthcheck(http_client, &self.api_url).await {
            Ok(healthy) => {
                log::info!(
                    "Server {} healthcheck result: {}", 
                    self.name, 
                    if healthy { "healthy" } else { "unhealthy" }
                );
                Ok(healthy)
            },
            Err(e) => {
                log::warn!("Server {} healthcheck failed: {}", self.name, e);
                Err(anyhow::anyhow!("Healthcheck failed: {}", e))
            }
        }
    }
    
    pub fn has_models(&self) -> bool {
        self.enabled && 
            self.available_models.as_ref().map(|models| !models.is_empty()).unwrap_or(false)
    }
}

// Implementation for AvailableModel
impl AvailableModel {
    pub fn new(
        name: String,
        display_name: Option<String>,
        server_max_tokens: usize,
        custom_max_tokens: Option<usize>,
        server_id: Option<String>,
        enabled: bool,
    ) -> Self {
        #[allow(deprecated)]
        Self {
            name,
            display_name,
            server_max_tokens,
            custom_max_tokens,
            server_id,
            enabled,
            max_tokens: 0,
        }
    }

    pub fn effective_max_tokens(&self) -> usize {
        self.custom_max_tokens.unwrap_or(self.server_max_tokens)
    }
    
    pub fn migrate_max_tokens(&mut self) {
        #[allow(deprecated)]
        let legacy_max_tokens = self.max_tokens;
        
        if legacy_max_tokens > 0 && self.custom_max_tokens.is_none() {
            self.custom_max_tokens = Some(legacy_max_tokens);
        }
    }
} 