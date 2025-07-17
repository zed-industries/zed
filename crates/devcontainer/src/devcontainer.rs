use anyhow::{Context, Result};
use fs::Fs;
use log::info;
use std::{
    path::{Path as StdPath, PathBuf},
    sync::Arc,
};

pub use crate::config::DevcontainerConfig;
pub use crate::docker::DockerManager;

/// Main devcontainer manager that handles the lifecycle of devcontainers
#[derive(Clone)]
pub struct DevcontainerManager {
    docker: DockerManager,
    fs: Arc<dyn Fs>,
}

impl DevcontainerManager {
    pub fn new(fs: Arc<dyn Fs>) -> Self {
        Self {
            docker: DockerManager::new(),
            fs,
        }
    }

    /// Detect and parse devcontainer configuration from a project path
    pub async fn detect_devcontainer(&self, project_path: &StdPath) -> Result<Option<DevcontainerConfig>> {
        let devcontainer_path = project_path.join(".devcontainer");
        
        // Try .devcontainer/devcontainer.json first
        let config_path = devcontainer_path.join("devcontainer.json");
        if self.fs.is_file(&config_path).await {
            let content = self.fs.load(&config_path).await
                .with_context(|| format!("Failed to read {:?}", config_path))?;
            let config = DevcontainerConfig::parse(&content)
                .with_context(|| format!("Failed to parse devcontainer config at {:?}", config_path))?;
            return Ok(Some(config));
        }

        // Try .devcontainer.json in project root
        let root_config_path = project_path.join(".devcontainer.json");
        if self.fs.is_file(&root_config_path).await {
            let content = self.fs.load(&root_config_path).await
                .with_context(|| format!("Failed to read {:?}", root_config_path))?;
            let config = DevcontainerConfig::parse(&content)
                .with_context(|| format!("Failed to parse devcontainer config at {:?}", root_config_path))?;
            return Ok(Some(config));
        }

        Ok(None)
    }

    /// Start a devcontainer for the given project
    pub async fn start_devcontainer(
        &self,
        project_path: &StdPath,
        config: DevcontainerConfig,
    ) -> Result<DevcontainerInstance> {
        info!("Starting devcontainer for project: {:?}", project_path);

        // Check if Docker is available
        self.docker.check_availability().await
            .context("Docker is not available")?;

        // Build or pull the container image
        let image_name = self.prepare_image(&config, project_path).await
            .context("Failed to prepare container image")?;

        // Create and start the container
        let container_id = self.docker.create_container(&image_name, &config, project_path).await
            .context("Failed to create container")?;

        self.docker.start_container(&container_id).await
            .context("Failed to start container")?;

        info!("Devcontainer started with ID: {}", container_id);

        Ok(DevcontainerInstance {
            container_id,
            image_name,
            project_path: project_path.to_path_buf(),
            config,
        })
    }

    /// Prepare the container image (build or pull)
    async fn prepare_image(&self, config: &DevcontainerConfig, project_path: &StdPath) -> Result<String> {
        if let Some(dockerfile) = &config.dockerfile {
            // Build from Dockerfile
            let dockerfile_path = project_path.join(dockerfile);
            if !dockerfile_path.exists() {
                anyhow::bail!("Dockerfile not found: {:?}", dockerfile_path);
            }
            
            let image_name = format!("zed-devcontainer-{}", 
                project_path.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown"));
            
            self.docker.build_image(&image_name, project_path, dockerfile).await?;
            Ok(image_name)
        } else if let Some(image) = &config.image {
            // Pull existing image
            self.docker.pull_image(image).await?;
            Ok(image.clone())
        } else {
            anyhow::bail!("No image or dockerfile specified in devcontainer configuration");
        }
    }

    /// Stop and remove a devcontainer
    pub async fn stop_devcontainer(&self, instance: &DevcontainerInstance) -> Result<()> {
        info!("Stopping devcontainer: {}", instance.container_id);
        
        self.docker.stop_container(&instance.container_id).await
            .context("Failed to stop container")?;
        
        self.docker.remove_container(&instance.container_id).await
            .context("Failed to remove container")?;
        
        Ok(())
    }

    /// Get connection information for a running devcontainer
    pub async fn get_connection_info(&self, instance: &DevcontainerInstance) -> Result<DevcontainerConnectionInfo> {
        let _container_info = self.docker.inspect_container(&instance.container_id).await?;
        
        // For now, we'll use docker exec for connection
        // In the future, this could be enhanced to support SSH or other protocols
        Ok(DevcontainerConnectionInfo {
            container_id: instance.container_id.clone(),
            connection_type: ConnectionType::DockerExec,
            working_directory: instance.config.workspace_mount.clone()
                .unwrap_or_else(|| "/workspace".to_string()),
        })
    }
}

/// Represents a running devcontainer instance
#[derive(Debug, Clone)]
pub struct DevcontainerInstance {
    pub container_id: String,
    pub image_name: String,
    pub project_path: PathBuf,
    pub config: DevcontainerConfig,
}

/// Connection information for accessing a devcontainer
#[derive(Debug, Clone)]
pub struct DevcontainerConnectionInfo {
    pub container_id: String,
    pub connection_type: ConnectionType,
    pub working_directory: String,
}

#[derive(Debug, Clone)]
pub enum ConnectionType {
    DockerExec,
    Ssh { host: String, port: u16 },
}

// Note: Default implementation removed since DevcontainerManager now requires an Fs instance 