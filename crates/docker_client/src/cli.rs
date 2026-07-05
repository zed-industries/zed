use anyhow::{Context as _, Result};

use crate::{
    ComposeProject, ComposeService, Container, DockerClient, DockerEndpoint, Image, LogChunk,
    docker_host_for, parse,
};

/// [`DockerClient`] implementation that shells out to the `docker` CLI.
pub struct CliDockerClient;

fn command(endpoint: &DockerEndpoint, args: &[&str]) -> tokio::process::Command {
    let mut cmd = tokio::process::Command::new("docker");
    if let Some(host) = docker_host_for(endpoint) {
        cmd.env("DOCKER_HOST", host);
    }
    cmd.args(args);
    cmd.kill_on_drop(true);
    cmd
}

async fn run(endpoint: &DockerEndpoint, args: &[&str]) -> Result<String> {
    let output = command(endpoint, args)
        .output()
        .await
        .with_context(|| format!("running `docker {}`", args.join(" ")))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("docker {} failed: {}", args.join(" "), stderr.trim());
    }
    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

#[async_trait::async_trait]
impl DockerClient for CliDockerClient {
    async fn test_endpoint(&self, endpoint: &DockerEndpoint) -> Result<()> {
        run(endpoint, &["version", "--format", "json"]).await?;
        Ok(())
    }

    async fn list_containers(&self, endpoint: &DockerEndpoint) -> Result<Vec<Container>> {
        let stdout = run(endpoint, &["ps", "-a", "--format", "json"]).await?;
        parse::parse_containers(&stdout)
    }

    async fn list_images(&self, endpoint: &DockerEndpoint) -> Result<Vec<Image>> {
        let stdout = run(endpoint, &["images", "--format", "json"]).await?;
        parse::parse_images(&stdout)
    }

    async fn list_compose_projects(
        &self,
        endpoint: &DockerEndpoint,
    ) -> Result<Vec<ComposeProject>> {
        let stdout = run(endpoint, &["compose", "ls", "--all", "--format", "json"]).await?;
        parse::parse_compose_projects(&stdout)
    }

    async fn list_compose_services(
        &self,
        endpoint: &DockerEndpoint,
        project: &str,
    ) -> Result<Vec<ComposeService>> {
        let stdout = run(
            endpoint,
            &["compose", "-p", project, "ps", "--format", "json"],
        )
        .await?;
        parse::parse_compose_services(&stdout)
    }

    async fn inspect_container(&self, endpoint: &DockerEndpoint, id: &str) -> Result<String> {
        run(endpoint, &["inspect", id]).await
    }

    async fn start_container(&self, endpoint: &DockerEndpoint, id: &str) -> Result<()> {
        run(endpoint, &["start", id]).await?;
        Ok(())
    }

    async fn stop_container(&self, endpoint: &DockerEndpoint, id: &str) -> Result<()> {
        run(endpoint, &["stop", id]).await?;
        Ok(())
    }

    async fn restart_container(&self, endpoint: &DockerEndpoint, id: &str) -> Result<()> {
        run(endpoint, &["restart", id]).await?;
        Ok(())
    }

    async fn pull_image(&self, endpoint: &DockerEndpoint, reference: &str) -> Result<()> {
        run(endpoint, &["pull", reference]).await?;
        Ok(())
    }

    async fn remove_image(&self, endpoint: &DockerEndpoint, id: &str) -> Result<()> {
        run(endpoint, &["rmi", id]).await?;
        Ok(())
    }

    async fn compose_up(
        &self,
        endpoint: &DockerEndpoint,
        project: &str,
        service: Option<&str>,
    ) -> Result<()> {
        let mut args = vec!["compose", "-p", project, "up", "-d"];
        if let Some(service) = service {
            args.push(service);
        }
        run(endpoint, &args).await?;
        Ok(())
    }

    async fn compose_down(&self, endpoint: &DockerEndpoint, project: &str) -> Result<()> {
        run(endpoint, &["compose", "-p", project, "down"]).await?;
        Ok(())
    }

    async fn compose_restart(
        &self,
        endpoint: &DockerEndpoint,
        project: &str,
        service: Option<&str>,
    ) -> Result<()> {
        let mut args = vec!["compose", "-p", project, "restart"];
        if let Some(service) = service {
            args.push(service);
        }
        run(endpoint, &args).await?;
        Ok(())
    }

    async fn container_logs(
        &self,
        _endpoint: &DockerEndpoint,
        _id: &str,
        _tail: usize,
    ) -> Result<futures::channel::mpsc::UnboundedReceiver<LogChunk>> {
        anyhow::bail!("not implemented")
    }
}

#[cfg(test)]
mod tests {
    use crate::EndpointKind;

    use super::*;

    // Run manually with: cargo test -p docker_client --features test-support -- --ignored --test-threads=1
    #[tokio::test]
    #[ignore]
    async fn cli_lists_local_containers() {
        let client = CliDockerClient;
        let ep = DockerEndpoint {
            name: "local".into(),
            kind: EndpointKind::Local,
            read_only: false,
        };
        let containers = client.list_containers(&ep).await.unwrap();
        // At least the zed-db-test postgres container is expected to be present when run.
        assert!(containers.iter().any(|c| c.image.contains("postgres")));
    }

    #[tokio::test]
    #[ignore]
    async fn cli_lists_local_images() {
        let client = CliDockerClient;
        let ep = DockerEndpoint {
            name: "local".into(),
            kind: EndpointKind::Local,
            read_only: false,
        };
        let images = client.list_images(&ep).await.unwrap();
        assert!(!images.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn cli_restarts_known_container_round_trip() {
        let client = CliDockerClient;
        let ep = DockerEndpoint {
            name: "local".into(),
            kind: EndpointKind::Local,
            read_only: false,
        };
        // Expects a throwaway container named `zed-db-test` to exist locally.
        client.restart_container(&ep, "zed-db-test").await.unwrap();
        let containers = client.list_containers(&ep).await.unwrap();
        assert!(containers.iter().any(|c| c.names.contains("zed-db-test")));
    }
}
