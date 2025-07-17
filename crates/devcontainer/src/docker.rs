use anyhow::{Context, Result};
use log::{info, warn};
use serde_json::Value;
use std::{
    path::Path,
    process::Stdio,
};

use crate::config::DevcontainerConfig;

/// Manages Docker operations for devcontainers
#[derive(Clone)]
pub struct DockerManager;

impl DockerManager {
    pub fn new() -> Self {
        Self
    }

    /// Check if a port is likely in use by checking for existing containers using it
    async fn is_port_likely_in_use(&self, port: u16) -> bool {
        // Check if any running containers are using this port
        let output = util::command::new_smol_command("docker")
            .arg("ps")
            .arg("--format")
            .arg("{{.Ports}}")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await;
            
        if let Ok(output) = output {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                return stdout.contains(&format!("{}->", port)) || 
                       stdout.contains(&format!(":{}/", port));
            }
        }
        
        false
    }

    /// Check if Docker is available and running
    pub async fn check_availability(&self) -> Result<()> {
        let output = util::command::new_smol_command("docker")
            .arg("version")
            .arg("--format")
            .arg("json")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await
            .context("Failed to execute docker command")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Docker is not available: {}", stderr);
        }

        info!("Docker is available");
        Ok(())
    }

    /// Pull a Docker image
    pub async fn pull_image(&self, image: &str) -> Result<()> {
        info!("Pulling Docker image: {}", image);

        let output = util::command::new_smol_command("docker")
            .arg("pull")
            .arg(image)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()
            .await
            .context("Failed to execute docker pull command")?;

        if !output.status.success() {
            anyhow::bail!("Failed to pull Docker image: {}", image);
        }

        info!("Successfully pulled image: {}", image);
        Ok(())
    }

    /// Build a Docker image from a Dockerfile
    pub async fn build_image(
        &self,
        image_name: &str,
        build_context: &Path,
        dockerfile: &str,
    ) -> Result<()> {
        info!("Building Docker image: {} from {}", image_name, dockerfile);

        let mut command = util::command::new_smol_command("docker");
        command
            .arg("build")
            .arg("-t")
            .arg(image_name)
            .arg("-f")
            .arg(dockerfile)
            .arg(".")
            .current_dir(build_context)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit());

        let output = command
            .output()
            .await
            .context("Failed to execute docker build command")?;

        if !output.status.success() {
            anyhow::bail!("Failed to build Docker image: {}", image_name);
        }

        info!("Successfully built image: {}", image_name);
        Ok(())
    }

    /// Create a container from an image
    pub async fn create_container(
        &self,
        image: &str,
        config: &DevcontainerConfig,
        project_path: &Path,
    ) -> Result<String> {
        info!("Creating container from image: {}", image);

        let container_name = format!(
            "zed-devcontainer-{}",
            project_path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown")
        );

        let mut command = util::command::new_smol_command("docker");
        command
            .arg("create")
            .arg("--name")
            .arg(&container_name)
            .arg("--interactive")
            .arg("--tty");

        // Add workspace mount
        let workspace_folder = config.get_workspace_folder();
        command
            .arg("--volume")
            .arg(format!("{}:{}", project_path.display(), workspace_folder));

        // Add port forwards (skip if ports are likely to conflict)
        for port_forward in config.get_port_forwards() {
            // Check if port is likely to be in use by other containers
            if self.is_port_likely_in_use(port_forward.host_port).await {
                warn!("Skipping port forward {}:{} as host port {} may be in use", 
                      port_forward.host_port, port_forward.container_port, port_forward.host_port);
                continue;
            }
            command
                .arg("--publish")
                .arg(format!("{}:{}", port_forward.host_port, port_forward.container_port));
        }

        // Add environment variables
        for (key, value) in config.get_all_env_vars() {
            command.arg("--env").arg(format!("{}={}", key, value));
        }

        // Add run args if specified
        if let Some(run_args) = &config.run_args {
            for arg in run_args {
                command.arg(arg);
            }
        }

        // Add privileged flag if needed
        if config.privileged.unwrap_or(false) {
            command.arg("--privileged");
        }

        // Add init flag if needed  
        if config.init.unwrap_or(false) {
            command.arg("--init");
        }

        // Add cap-add
        if let Some(cap_add) = &config.cap_add {
            for cap in cap_add {
                command.arg("--cap-add").arg(cap);
            }
        }

        // Add security options
        if let Some(security_opt) = &config.security_opt {
            for opt in security_opt {
                command.arg("--security-opt").arg(opt);
            }
        }

        // Add mounts
        if let Some(mounts) = &config.mounts {
            for mount in mounts {
                command.arg("--mount").arg(mount);
            }
        }

        // Set user if specified
        if let Some(user) = config.get_effective_user() {
            command.arg("--user").arg(user);
        }

        // Set working directory
        command
            .arg("--workdir")
            .arg(&workspace_folder);

        // Specify the image
        command.arg(image);

        // Default command to keep container running
        command.arg("sleep").arg("infinity");

        // Log the full command for debugging
        info!("Docker create command: {:?}", command);

        let output = command
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await
            .context("Failed to execute docker create command")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let stdout = String::from_utf8_lossy(&output.stdout);
            anyhow::bail!("Failed to create container: {}\nStdout: {}", stderr, stdout);
        }

        let container_id = String::from_utf8_lossy(&output.stdout).trim().to_string();
        info!("Created container with ID: {}", container_id);
        Ok(container_id)
    }

    /// Start a container
    pub async fn start_container(&self, container_id: &str) -> Result<()> {
        info!("Starting container: {}", container_id);

        let output = util::command::new_smol_command("docker")
            .arg("start")
            .arg(container_id)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await
            .context("Failed to execute docker start command")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let stdout = String::from_utf8_lossy(&output.stdout);
            anyhow::bail!("Failed to start container: {}\nStdout: {}", stderr, stdout);
        }

        info!("Container started successfully: {}", container_id);
        Ok(())
    }

    /// Stop a container
    pub async fn stop_container(&self, container_id: &str) -> Result<()> {
        info!("Stopping container: {}", container_id);

        let output = util::command::new_smol_command("docker")
            .arg("stop")
            .arg(container_id)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await
            .context("Failed to execute docker stop command")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            warn!("Failed to stop container (may already be stopped): {}", stderr);
        } else {
            info!("Container stopped successfully: {}", container_id);
        }

        Ok(())
    }

    /// Remove a container
    pub async fn remove_container(&self, container_id: &str) -> Result<()> {
        info!("Removing container: {}", container_id);

        let output = util::command::new_smol_command("docker")
            .arg("rm")
            .arg(container_id)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await
            .context("Failed to execute docker rm command")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            warn!("Failed to remove container (may already be removed): {}", stderr);
        } else {
            info!("Container removed successfully: {}", container_id);
        }

        Ok(())
    }

    /// Inspect a container and return information
    pub async fn inspect_container(&self, container_id: &str) -> Result<ContainerInfo> {
        let output = util::command::new_smol_command("docker")
            .arg("inspect")
            .arg(container_id)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await
            .context("Failed to execute docker inspect command")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Failed to inspect container: {}", stderr);
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let inspect_data: Value = serde_json::from_str(&stdout)
            .context("Failed to parse docker inspect output")?;

        // Extract the first item from the array
        let container_data = inspect_data
            .as_array()
            .and_then(|arr| arr.first())
            .context("No container data in inspect response")?;

        Ok(ContainerInfo {
            id: container_data
                .get("Id")
                .and_then(|v| v.as_str())
                .unwrap_or(container_id)
                .to_string(),
            name: container_data
                .get("Name")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            state: container_data
                .get("State")
                .and_then(|v| v.get("Status"))
                .and_then(|v| v.as_str())
                .unwrap_or("unknown")
                .to_string(),
        })
    }

    /// Execute a command in a running container
    pub async fn exec_command(
        &self,
        container_id: &str,
        command: &[&str],
        working_dir: Option<&str>,
    ) -> Result<String> {
        let mut docker_cmd = util::command::new_smol_command("docker");
        docker_cmd
            .arg("exec")
            .arg("-i");

        if let Some(workdir) = working_dir {
            docker_cmd.arg("-w").arg(workdir);
        }

        docker_cmd.arg(container_id);
        docker_cmd.args(command);

        let output = docker_cmd
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await
            .context("Failed to execute docker exec command")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Command failed in container: {}", stderr);
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}

#[derive(Debug, Clone)]
pub struct ContainerInfo {
    pub id: String,
    pub name: String,
    pub state: String,
}

impl Default for DockerManager {
    fn default() -> Self {
        Self::new()
    }
} 