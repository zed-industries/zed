use anyhow::Result;
use serde::{Deserialize, Serialize};

pub mod cli;
pub mod parse;

#[cfg(any(test, feature = "test-support"))]
pub mod fake;

pub use cli::CliDockerClient;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EndpointKind {
    Local,
    Ssh { host: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DockerEndpoint {
    pub name: String,
    pub kind: EndpointKind,
    pub read_only: bool,
}

/// A context reported by `docker context ls`, used to auto-discover endpoints
/// beyond what the user has explicitly configured.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DockerContext {
    pub name: String,
    pub docker_endpoint: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContainerState {
    Running,
    Exited,
    Paused,
    Created,
    Restarting,
    Dead,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Container {
    pub id: String,
    pub names: String,
    pub image: String,
    pub state: ContainerState,
    pub status: String,
    pub ports: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Image {
    pub id: String,
    pub repository: String,
    pub tag: String,
    pub size: String,
    pub created: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComposeProject {
    pub name: String,
    pub status: String,
    pub config_files: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComposeService {
    pub name: String,
    pub state: String,
    pub project: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogChunk {
    pub line: String,
}

pub fn docker_host_for(endpoint: &DockerEndpoint) -> Option<String> {
    match &endpoint.kind {
        EndpointKind::Local => None,
        EndpointKind::Ssh { host } => Some(format!("ssh://{host}")),
    }
}

/// Parses a context's `DockerEndpoint` string (e.g. `"ssh://user@host"` or
/// `"unix:///var/run/docker.sock"`) into an [`EndpointKind`]. Anything that
/// isn't an `ssh://` URL (unix sockets, `"default"`, npipe, etc.) is treated
/// as [`EndpointKind::Local`].
fn endpoint_kind_from_docker_endpoint(docker_endpoint: &str) -> EndpointKind {
    match docker_endpoint.strip_prefix("ssh://") {
        Some(host) => EndpointKind::Ssh {
            host: host.to_string(),
        },
        None => EndpointKind::Local,
    }
}

/// Merges auto-discovered `docker context ls` contexts into the user's
/// configured endpoints. Configured entries always win by name: a context
/// whose name clashes with a configured endpoint is ignored, and the
/// configured endpoint keeps whatever `read_only` the user set. Every other
/// context becomes a new [`DockerEndpoint`] with `read_only: true`:
/// auto-discovered endpoints (possibly production) are view-only until the
/// user explicitly adds them to `docker.connections` with `read_only: false`.
/// This also prevents a subtle downgrade: if the user deletes a configured
/// `read_only: true` endpoint but the underlying `docker context` still
/// exists, re-running discovery must not resurrect it as writable.
pub fn merge_endpoints(
    configured: Vec<DockerEndpoint>,
    contexts: Vec<DockerContext>,
) -> Vec<DockerEndpoint> {
    let mut merged = configured;
    for context in contexts {
        if merged.iter().any(|endpoint| endpoint.name == context.name) {
            continue;
        }
        merged.push(DockerEndpoint {
            name: context.name,
            kind: endpoint_kind_from_docker_endpoint(&context.docker_endpoint),
            read_only: true,
        });
    }
    merged
}

#[async_trait::async_trait]
pub trait DockerClient: Send + Sync {
    /// Runs `docker context ls` on the LOCAL docker CLI (no endpoint argument:
    /// contexts are a property of the local docker config, not of a remote
    /// daemon) to discover endpoints beyond what's explicitly configured.
    async fn list_contexts(&self) -> Result<Vec<DockerContext>>;
    /// Runs `docker version` to verify the endpoint is reachable.
    async fn test_endpoint(&self, endpoint: &DockerEndpoint) -> Result<()>;
    async fn list_containers(&self, endpoint: &DockerEndpoint) -> Result<Vec<Container>>;
    async fn list_images(&self, endpoint: &DockerEndpoint) -> Result<Vec<Image>>;
    async fn list_compose_projects(&self, endpoint: &DockerEndpoint)
    -> Result<Vec<ComposeProject>>;
    async fn list_compose_services(
        &self,
        endpoint: &DockerEndpoint,
        project: &str,
    ) -> Result<Vec<ComposeService>>;
    /// Returns the pretty-printed JSON output of `docker inspect`.
    async fn inspect_container(&self, endpoint: &DockerEndpoint, id: &str) -> Result<String>;
    async fn start_container(&self, endpoint: &DockerEndpoint, id: &str) -> Result<()>;
    async fn stop_container(&self, endpoint: &DockerEndpoint, id: &str) -> Result<()>;
    async fn restart_container(&self, endpoint: &DockerEndpoint, id: &str) -> Result<()>;
    async fn pull_image(&self, endpoint: &DockerEndpoint, reference: &str) -> Result<()>;
    async fn remove_image(&self, endpoint: &DockerEndpoint, id: &str) -> Result<()>;
    async fn compose_up(
        &self,
        endpoint: &DockerEndpoint,
        project: &str,
        service: Option<&str>,
    ) -> Result<()>;
    async fn compose_down(&self, endpoint: &DockerEndpoint, project: &str) -> Result<()>;
    async fn compose_restart(
        &self,
        endpoint: &DockerEndpoint,
        project: &str,
        service: Option<&str>,
    ) -> Result<()>;
    async fn container_logs(
        &self,
        endpoint: &DockerEndpoint,
        id: &str,
        tail: usize,
    ) -> Result<futures::channel::mpsc::UnboundedReceiver<LogChunk>>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn docker_host_for_ssh_and_local() {
        let local = DockerEndpoint {
            name: "local".into(),
            kind: EndpointKind::Local,
            read_only: false,
        };
        let remote = DockerEndpoint {
            name: "prod".into(),
            kind: EndpointKind::Ssh {
                host: "deploy@1.2.3.4".into(),
            },
            read_only: true,
        };
        assert_eq!(docker_host_for(&local), None);
        assert_eq!(
            docker_host_for(&remote),
            Some("ssh://deploy@1.2.3.4".to_string())
        );
    }

    #[test]
    fn merge_prefers_configured_and_imports_new_contexts() {
        let configured = vec![DockerEndpoint {
            name: "prod".into(),
            kind: EndpointKind::Ssh {
                host: "me@h".into(),
            },
            read_only: true,
        }];
        let contexts = vec![
            DockerContext {
                name: "prod".into(),
                docker_endpoint: "ssh://other@h2".into(),
            }, // ignored (name clash)
            DockerContext {
                name: "staging".into(),
                docker_endpoint: "ssh://deploy@stg".into(),
            },
            DockerContext {
                name: "default".into(),
                docker_endpoint: "unix:///var/run/docker.sock".into(),
            },
        ];
        let merged = merge_endpoints(configured, contexts);
        let prod = merged.iter().find(|e| e.name == "prod").unwrap();
        assert!(prod.read_only); // configured keeps its own flag and wins by name

        let staging = merged.iter().find(|e| e.name == "staging").unwrap();
        assert!(matches!(staging.kind, EndpointKind::Ssh { .. }));
        assert!(
            staging.read_only,
            "auto-imported contexts must default to read_only: true"
        );

        let default = merged.iter().find(|e| e.name == "default").unwrap();
        assert!(matches!(default.kind, EndpointKind::Local));
        assert!(
            default.read_only,
            "auto-imported contexts must default to read_only: true"
        );
    }

    /// Resurrection scenario: a configured `read_only: true` endpoint is
    /// removed from settings, but `docker context ls` still reports a
    /// context with the same name (e.g. the user only deleted the
    /// `docker.connections` entry, not the underlying docker context). The
    /// re-imported endpoint must come back read-only, never silently
    /// downgraded to writable.
    #[test]
    fn removed_configured_endpoint_resurrects_as_read_only() {
        let configured = vec![]; // "prod" no longer configured
        let contexts = vec![DockerContext {
            name: "prod".into(),
            docker_endpoint: "ssh://deploy@1.2.3.4".into(),
        }];
        let merged = merge_endpoints(configured, contexts);
        let prod = merged.iter().find(|e| e.name == "prod").unwrap();
        assert!(
            prod.read_only,
            "a context resurrecting a removed configured endpoint must be read-only, not writable"
        );
    }
}
