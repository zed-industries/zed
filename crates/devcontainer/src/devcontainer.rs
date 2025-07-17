use anyhow::{Context, Result};
use fs::Fs;
use log::info;
use std::{
    collections::HashMap,
    path::{Path as StdPath, PathBuf},
    sync::Arc,
};

pub use crate::config::DevcontainerConfig;
pub use crate::docker::DockerManager;

/// Progress callback function type
pub type ProgressCallback = Box<dyn Fn(String) + Send + Sync>;

/// Main devcontainer manager that handles the lifecycle of devcontainers
#[derive(Clone)]
pub struct DevcontainerManager {
    docker: DockerManager,
    fs: Arc<dyn Fs>,
    active_instances: Arc<std::sync::Mutex<HashMap<String, DevcontainerInstance>>>,
}

impl DevcontainerManager {
    pub fn new(fs: Arc<dyn Fs>) -> Self {
        Self {
            docker: DockerManager::new(),
            fs,
            active_instances: Arc::new(std::sync::Mutex::new(HashMap::new())),
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

    /// Start a devcontainer for the given project with progress reporting
    pub async fn start_devcontainer(
        &self,
        project_path: &StdPath,
        config: DevcontainerConfig,
        progress_callback: Option<ProgressCallback>,
    ) -> Result<DevcontainerInstance> {
        let callback = |message: String| {
            info!("Devcontainer progress: {}", message);
            if let Some(ref cb) = progress_callback {
                cb(message);
            }
        };

        callback("Starting devcontainer...".to_string());

        // Check if Docker is available
        callback("Checking Docker availability...".to_string());
        self.docker.check_availability().await
            .context("Docker is not available")?;

        // Generate container name
        let container_name = format!(
            "zed-devcontainer-{}",
            project_path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown")
        );

        // Check if container already exists
        callback("Checking for existing container...".to_string());
        if let Some(existing_container_id) = self.docker.get_container_id_by_name(&container_name).await? {
            info!("Found existing container: {}", existing_container_id);
            
            // Check if it's running
            if self.docker.container_is_running(&container_name).await? {
                callback("Connecting to existing running container...".to_string());
                info!("Container {} is already running", existing_container_id);
            } else {
                callback("Starting existing container...".to_string());
                self.docker.start_container(&existing_container_id).await
                    .context("Failed to start existing container")?;
                info!("Started existing container: {}", existing_container_id);
            }

            // Get the image name from the config
            let image_name = self.get_image_name_from_config(&config, project_path).await?;

            let instance = DevcontainerInstance {
                container_id: existing_container_id.clone(),
                image_name,
                project_path: project_path.to_path_buf(),
                config,
            };

            // Store the active instance
            self.active_instances.lock().unwrap().insert(existing_container_id.clone(), instance.clone());
            
            callback("Connected to existing devcontainer successfully!".to_string());
            return Ok(instance);
        }

        // Build or pull the container image
        callback("Preparing container image...".to_string());
        let image_name = self.prepare_image(&config, project_path, &callback).await
            .context("Failed to prepare container image")?;

        // Create and start the container
        callback("Creating container...".to_string());
        let container_id = self.docker.create_container(&image_name, &config, project_path).await
            .context("Failed to create container")?;

        callback("Starting container...".to_string());
        self.docker.start_container(&container_id).await
            .context("Failed to start container")?;

        info!("Devcontainer started with ID: {}", container_id);
        callback("Devcontainer started successfully!".to_string());

        let instance = DevcontainerInstance {
            container_id: container_id.clone(),
            image_name,
            project_path: project_path.to_path_buf(),
            config,
        };

        // Store the active instance
        self.active_instances.lock().unwrap().insert(container_id.clone(), instance.clone());

        Ok(instance)
    }

    /// Get the image name from the config without building/pulling
    async fn get_image_name_from_config(
        &self,
        config: &DevcontainerConfig,
        project_path: &StdPath,
    ) -> Result<String> {
        if config.dockerfile.is_some() {
            // For Dockerfile builds, use the same naming convention
            Ok(format!("zed-devcontainer-{}", 
                project_path.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown")))
        } else if let Some(image) = &config.image {
            // For pre-built images, use the image name directly
            Ok(image.clone())
        } else {
            anyhow::bail!("No image or dockerfile specified in devcontainer configuration");
        }
    }

    /// Prepare the container image (build or pull) with progress reporting
    async fn prepare_image(
        &self, 
        config: &DevcontainerConfig, 
        project_path: &StdPath,
        callback: &impl Fn(String),
    ) -> Result<String> {
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
            
            callback(format!("Building image from Dockerfile: {}", dockerfile));
            self.docker.build_image(&image_name, project_path, dockerfile).await?;
            Ok(image_name)
        } else if let Some(image) = &config.image {
            // Pull existing image
            callback(format!("Pulling image: {}", image));
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
        
        // Remove from active instances
        self.active_instances.lock().unwrap().remove(&instance.container_id);
        
        Ok(())
    }

    /// Stop all active devcontainers
    pub async fn stop_all_devcontainers(&self) -> Result<()> {
        let instances: Vec<DevcontainerInstance> = {
            let active = self.active_instances.lock().unwrap();
            active.values().cloned().collect()
        };

        for instance in instances {
            if let Err(e) = self.stop_devcontainer(&instance).await {
                log::error!("Failed to stop devcontainer {}: {}", instance.container_id, e);
            }
        }

        Ok(())
    }

    /// Get all active devcontainer instances
    pub fn get_active_instances(&self) -> Vec<DevcontainerInstance> {
        self.active_instances.lock().unwrap().values().cloned().collect()
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