use anyhow::{Context, Result};
use collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use settings::parse_json_with_comments;

/// Represents a parsed devcontainer.json configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DevcontainerConfig {
    /// Name of the devcontainer
    pub name: Option<String>,

    /// Docker image to use
    pub image: Option<String>,

    /// Path to Dockerfile (relative to devcontainer.json)
    pub dockerfile: Option<String>,

    /// Build context path
    pub context: Option<String>,

    /// Build arguments for Dockerfile
    pub build: Option<BuildConfig>,

    /// Container arguments
    pub run_args: Option<Vec<String>>,

    /// App port mappings
    pub app_port: Option<Value>, // Can be number, string, or array

    /// Forward ports configuration
    pub forward_ports: Option<Vec<Value>>, // Can be numbers or port configs

    /// Mounts configuration
    pub mounts: Option<Vec<String>>,

    /// Environment variables
    pub container_env: Option<HashMap<String, String>>,

    /// Remote environment variables
    pub remote_env: Option<HashMap<String, String>>,

    /// Remote user
    pub remote_user: Option<String>,

    /// Container user
    pub container_user: Option<String>,

    /// Override command
    pub override_command: Option<bool>,

    /// Shutdown action
    pub shutdown_action: Option<String>,

    /// Update remote user UID
    pub update_remote_user_uid: Option<bool>,

    /// Workspace mount path in container
    pub workspace_mount: Option<String>,

    /// Workspace folder in container
    pub workspace_folder: Option<String>,

    /// Post create command
    pub post_create_command: Option<Value>, // Can be string or array

    /// Post start command
    pub post_start_command: Option<Value>, // Can be string or array

    /// Post attach command
    pub post_attach_command: Option<Value>, // Can be string or array

    /// Init flag
    pub init: Option<bool>,

    /// Privileged flag
    pub privileged: Option<bool>,

    /// Cap add
    pub cap_add: Option<Vec<String>>,

    /// Security options
    pub security_opt: Option<Vec<String>>,

    /// Features to install
    pub features: Option<HashMap<String, Value>>,

    /// VS Code specific settings (ignored for Zed)
    #[serde(rename = "customizations")]
    pub customizations: Option<Value>,

    /// Other properties we don't explicitly handle
    #[serde(flatten)]
    pub other: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildConfig {
    /// Dockerfile path
    pub dockerfile: Option<String>,

    /// Build context
    pub context: Option<String>,

    /// Build arguments
    pub args: Option<HashMap<String, String>>,

    /// Target stage for multi-stage builds
    pub target: Option<String>,

    /// Cache from configuration
    pub cache_from: Option<Vec<String>>,
}

impl DevcontainerConfig {
    /// Parse a devcontainer.json file content
    pub fn parse(content: &str) -> Result<Self> {
        let config: DevcontainerConfig = parse_json_with_comments(content)
            .context("Failed to parse devcontainer.json")?;
        
        // Validate the configuration
        config.validate()?;
        
        Ok(config)
    }

    /// Validate the devcontainer configuration
    fn validate(&self) -> Result<()> {
        // Must have either an image or dockerfile
        if self.image.is_none() && self.dockerfile.is_none() && self.build.is_none() {
            anyhow::bail!("Devcontainer configuration must specify either 'image', 'dockerfile', or 'build'");
        }

        // If dockerfile is specified in build config, use that
        if let Some(build) = &self.build {
            if build.dockerfile.is_some() {
                // This is valid
            }
        }

        Ok(())
    }

    /// Get the effective dockerfile path
    pub fn get_dockerfile_path(&self) -> Option<&str> {
        if let Some(build) = &self.build {
            if let Some(dockerfile) = &build.dockerfile {
                return Some(dockerfile);
            }
        }
        self.dockerfile.as_deref()
    }

    /// Get the effective build context
    pub fn get_build_context(&self) -> Option<&str> {
        if let Some(build) = &self.build {
            if let Some(context) = &build.context {
                return Some(context);
            }
        }
        self.context.as_deref()
    }

    /// Get build arguments
    pub fn get_build_args(&self) -> HashMap<String, String> {
        self.build
            .as_ref()
            .and_then(|b| b.args.as_ref())
            .cloned()
            .unwrap_or_default()
    }

    /// Get all environment variables (container + remote)
    pub fn get_all_env_vars(&self) -> HashMap<String, String> {
        let mut env = HashMap::default();
        
        if let Some(container_env) = &self.container_env {
            env.extend(container_env.clone());
        }
        
        if let Some(remote_env) = &self.remote_env {
            env.extend(remote_env.clone());
        }
        
        env
    }

    /// Get the workspace folder path in the container
    pub fn get_workspace_folder(&self) -> String {
        self.workspace_folder
            .clone()
            .or_else(|| self.workspace_mount.clone())
            .unwrap_or_else(|| "/workspace".to_string())
    }

    /// Get port forwards as a list of port mappings
    pub fn get_port_forwards(&self) -> Vec<PortForward> {
        let mut forwards = Vec::new();
        
        // Handle appPort
        if let Some(app_port) = &self.app_port {
            forwards.extend(Self::parse_port_value(app_port));
        }
        
        // Handle forwardPorts
        if let Some(forward_ports) = &self.forward_ports {
            for port in forward_ports {
                forwards.extend(Self::parse_port_value(port));
            }
        }
        
        forwards
    }

    fn parse_port_value(value: &Value) -> Vec<PortForward> {
        match value {
            Value::Number(n) => {
                if let Some(port) = n.as_u64() {
                    vec![PortForward {
                        host_port: port as u16,
                        container_port: port as u16,
                        protocol: "tcp".to_string(),
                    }]
                } else {
                    vec![]
                }
            }
            Value::String(s) => {
                if let Ok(port) = s.parse::<u16>() {
                    vec![PortForward {
                        host_port: port,
                        container_port: port,
                        protocol: "tcp".to_string(),
                    }]
                } else if s.contains(':') {
                    // Parse "host:container" format
                    let parts: Vec<&str> = s.split(':').collect();
                    if parts.len() == 2 {
                        if let (Ok(host), Ok(container)) = (parts[0].parse::<u16>(), parts[1].parse::<u16>()) {
                            return vec![PortForward {
                                host_port: host,
                                container_port: container,
                                protocol: "tcp".to_string(),
                            }];
                        }
                    }
                    vec![]
                } else {
                    vec![]
                }
            }
            Value::Array(arr) => {
                arr.iter().flat_map(Self::parse_port_value).collect()
            }
            _ => vec![],
        }
    }

    /// Get the effective container user
    pub fn get_effective_user(&self) -> Option<&str> {
        self.remote_user.as_deref()
            .or_else(|| self.container_user.as_deref())
    }
}

#[derive(Debug, Clone)]
pub struct PortForward {
    pub host_port: u16,
    pub container_port: u16,
    pub protocol: String,
}

impl Default for DevcontainerConfig {
    fn default() -> Self {
        Self {
            name: None,
            image: None,
            dockerfile: None,
            context: None,
            build: None,
            run_args: None,
            app_port: None,
            forward_ports: None,
            mounts: None,
            container_env: None,
            remote_env: None,
            remote_user: None,
            container_user: None,
            override_command: None,
            shutdown_action: None,
            update_remote_user_uid: None,
            workspace_mount: None,
            workspace_folder: None,
            post_create_command: None,
            post_start_command: None,
            post_attach_command: None,
            init: None,
            privileged: None,
            cap_add: None,
            security_opt: None,
            features: None,
            customizations: None,
            other: HashMap::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_config() {
        let config_json = r#"
        {
            "name": "Test Container",
            "image": "mcr.microsoft.com/devcontainers/typescript-node:1-20",
            "forwardPorts": [3000, 3001],
            "postCreateCommand": "npm install"
        }
        "#;

        let config = DevcontainerConfig::parse(config_json).unwrap();
        assert_eq!(config.name, Some("Test Container".to_string()));
        assert_eq!(config.image, Some("mcr.microsoft.com/devcontainers/typescript-node:1-20".to_string()));
        
        let forwards = config.get_port_forwards();
        assert_eq!(forwards.len(), 2);
        assert_eq!(forwards[0].host_port, 3000);
        assert_eq!(forwards[1].host_port, 3001);
    }

    #[test]
    fn test_parse_dockerfile_config() {
        let config_json = r#"
        {
            "name": "Custom Dockerfile",
            "build": {
                "dockerfile": "Dockerfile.dev",
                "context": ".",
                "args": {
                    "NODE_VERSION": "18"
                }
            },
            "remoteUser": "developer"
        }
        "#;

        let config = DevcontainerConfig::parse(config_json).unwrap();
        assert_eq!(config.name, Some("Custom Dockerfile".to_string()));
        assert_eq!(config.get_dockerfile_path(), Some("Dockerfile.dev"));
        assert_eq!(config.get_build_context(), Some("."));
        assert_eq!(config.get_effective_user(), Some("developer"));
        
        let build_args = config.get_build_args();
        assert_eq!(build_args.get("NODE_VERSION"), Some(&"18".to_string()));
    }
} 