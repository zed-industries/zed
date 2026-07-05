use docker_client::{DockerEndpoint, EndpointKind};
use settings::{DockerEndpointKindContent, RegisterSetting, Settings};

#[derive(Debug, Clone, PartialEq, RegisterSetting)]
pub struct DockerSettings {
    pub poll_interval_seconds: u64,
    pub endpoints: Vec<DockerEndpoint>,
}

impl Settings for DockerSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        let docker = content.docker.clone().unwrap();
        Self {
            poll_interval_seconds: docker.poll_interval_seconds.unwrap(),
            endpoints: docker
                .connections
                .unwrap_or_default()
                .into_iter()
                .map(|connection| DockerEndpoint {
                    name: connection.name,
                    kind: match connection.kind {
                        DockerEndpointKindContent::Local => EndpointKind::Local,
                        DockerEndpointKindContent::Ssh => EndpointKind::Ssh {
                            host: connection.ssh_host.unwrap_or_default(),
                        },
                    },
                    read_only: connection.read_only.unwrap_or(false),
                })
                .collect(),
        }
    }
}
