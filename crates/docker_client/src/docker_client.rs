use serde::{Deserialize, Serialize};

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
}
