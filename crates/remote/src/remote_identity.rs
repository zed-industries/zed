use crate::RemoteConnectionOptions;

/// A normalized remote identity for matching live remote hosts against
/// persisted remote metadata.
///
/// This mirrors workspace persistence identity semantics rather than full
/// `RemoteConnectionOptions` equality, so runtime-only fields like SSH
/// nicknames or Docker environment overrides do not affect matching.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RemoteConnectionIdentity {
    Ssh {
        host: String,
        username: Option<String>,
        port: Option<u16>,
    },
    Wsl {
        distro_name: String,
        user: Option<String>,
    },
    Docker {
        remote_user: String,
        key: DockerIdentityKey,
    },
    #[cfg(any(test, feature = "test-support"))]
    Mock { id: u64 },
}

/// What uniquely identifies a Docker remote, independent of runtime-only
/// details.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DockerIdentityKey {
    /// A dev container, identified by its host project folder
    /// (`devcontainer.local_folder`) and config file
    /// (`devcontainer.config_file`). Zed enforces that these labels are unique
    /// per dev container (it refuses to connect when two containers share
    /// them), and they stay stable across rebuilds/restarts even though the
    /// container's id changes each time. Keying on them here means an
    /// open/stop/rebuild cycle maps to a single persisted connection rather
    /// than accumulating a new one every time.
    DevContainer {
        local_folder: String,
        config_file: String,
    },
    /// A Docker remote with no dev-container labels: fall back to the ephemeral
    /// container id, preserving prior behavior.
    ContainerId(String),
}

impl RemoteConnectionIdentity {
    /// A stable string form of this identity, suitable for use in
    /// persistence keys (e.g. database keys scoped to a remote host).
    pub fn persistence_key(&self) -> String {
        match self {
            Self::Ssh {
                host,
                username,
                port,
            } => format!(
                "ssh:{}@{}:{}",
                username.as_deref().unwrap_or_default(),
                host,
                port.map(|port| port.to_string()).unwrap_or_default()
            ),
            Self::Wsl { distro_name, user } => format!(
                "wsl:{}@{}",
                user.as_deref().unwrap_or_default(),
                distro_name
            ),
            Self::Docker { remote_user, key } => match key {
                DockerIdentityKey::DevContainer {
                    local_folder,
                    config_file,
                } => format!("docker:{remote_user}@devcontainer:{local_folder}:{config_file}"),
                DockerIdentityKey::ContainerId(container_id) => {
                    format!("docker:{remote_user}@container:{container_id}")
                }
            },
            #[cfg(any(test, feature = "test-support"))]
            Self::Mock { id } => format!("mock:{id}"),
        }
    }
}

impl From<&RemoteConnectionOptions> for RemoteConnectionIdentity {
    fn from(options: &RemoteConnectionOptions) -> Self {
        match options {
            RemoteConnectionOptions::Ssh(options) => Self::Ssh {
                host: options.host.to_string(),
                username: options.username.clone(),
                port: options.port,
            },
            RemoteConnectionOptions::Wsl(options) => Self::Wsl {
                distro_name: options.distro_name.clone(),
                user: options.user.clone(),
            },
            RemoteConnectionOptions::Docker(options) => Self::Docker {
                remote_user: options.remote_user.clone(),
                key: match (&options.local_folder, &options.config_file) {
                    (Some(local_folder), Some(config_file)) => DockerIdentityKey::DevContainer {
                        local_folder: local_folder.clone(),
                        config_file: config_file.clone(),
                    },
                    _ => DockerIdentityKey::ContainerId(options.container_id.clone()),
                },
            },
            #[cfg(any(test, feature = "test-support"))]
            RemoteConnectionOptions::Mock(options) => Self::Mock { id: options.id },
        }
    }
}

pub fn remote_connection_identity(options: &RemoteConnectionOptions) -> RemoteConnectionIdentity {
    options.into()
}

pub fn same_remote_connection_identity(
    left: Option<&RemoteConnectionOptions>,
    right: Option<&RemoteConnectionOptions>,
) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => {
            remote_connection_identity(left) == remote_connection_identity(right)
        }
        (None, None) => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;
    use crate::{DockerConnectionOptions, SshConnectionOptions, WslConnectionOptions};

    #[test]
    fn ssh_identity_ignores_non_persisted_runtime_fields() {
        let left = RemoteConnectionOptions::Ssh(SshConnectionOptions {
            host: "example.com".into(),
            username: Some("anth".to_string()),
            port: Some(2222),
            password: Some("secret".to_string()),
            args: Some(vec!["-v".to_string()]),
            connection_timeout: Some(30),
            nickname: Some("work".to_string()),
            upload_binary_over_ssh: true,
            ..Default::default()
        });
        let right = RemoteConnectionOptions::Ssh(SshConnectionOptions {
            host: "example.com".into(),
            username: Some("anth".to_string()),
            port: Some(2222),
            password: None,
            args: None,
            connection_timeout: None,
            nickname: None,
            upload_binary_over_ssh: false,
            ..Default::default()
        });

        assert!(same_remote_connection_identity(Some(&left), Some(&right),));
    }

    #[test]
    fn ssh_identity_distinguishes_persistence_key_fields() {
        let left = RemoteConnectionOptions::Ssh(SshConnectionOptions {
            host: "example.com".into(),
            username: Some("anth".to_string()),
            port: Some(2222),
            ..Default::default()
        });
        let right = RemoteConnectionOptions::Ssh(SshConnectionOptions {
            host: "example.com".into(),
            username: Some("anth".to_string()),
            port: Some(2223),
            ..Default::default()
        });

        assert!(!same_remote_connection_identity(Some(&left), Some(&right),));
    }

    #[test]
    fn wsl_identity_includes_user() {
        let left = RemoteConnectionOptions::Wsl(WslConnectionOptions {
            distro_name: "Ubuntu".to_string(),
            user: Some("anth".to_string()),
        });
        let right = RemoteConnectionOptions::Wsl(WslConnectionOptions {
            distro_name: "Ubuntu".to_string(),
            user: Some("root".to_string()),
        });

        assert!(!same_remote_connection_identity(Some(&left), Some(&right),));
    }

    #[test]
    fn docker_identity_ignores_non_persisted_runtime_fields() {
        let left = RemoteConnectionOptions::Docker(DockerConnectionOptions {
            name: "zed-dev".to_string(),
            container_id: "container-123".to_string(),
            remote_user: "anth".to_string(),
            local_folder: Some("/home/anth/project".to_string()),
            config_file: Some("/home/anth/project/.devcontainer/devcontainer.json".to_string()),
            upload_binary_over_docker_exec: true,
            use_podman: true,
            remote_env: BTreeMap::from([("FOO".to_string(), "BAR".to_string())]),
        });
        let right = RemoteConnectionOptions::Docker(DockerConnectionOptions {
            name: "zed-dev".to_string(),
            container_id: "container-123".to_string(),
            remote_user: "anth".to_string(),
            local_folder: Some("/home/anth/project".to_string()),
            config_file: Some("/home/anth/project/.devcontainer/devcontainer.json".to_string()),
            upload_binary_over_docker_exec: false,
            use_podman: false,
            remote_env: BTreeMap::new(),
        });

        assert!(same_remote_connection_identity(Some(&left), Some(&right),));
    }

    #[test]
    fn dev_container_identity_is_stable_across_rebuilds() {
        // A rebuild mints a new `container_id` (and may pick a new project
        // `name`), but the host labels stay the same, so the identity must not
        // change. This is what keeps a single sidebar/recent-projects entry per
        // dev container across open/stop/rebuild cycles.
        let before = RemoteConnectionOptions::Docker(DockerConnectionOptions {
            name: "zed-dev".to_string(),
            container_id: "container-before".to_string(),
            remote_user: "anth".to_string(),
            local_folder: Some("/home/anth/project".to_string()),
            config_file: Some("/home/anth/project/.devcontainer/devcontainer.json".to_string()),
            ..Default::default()
        });
        let after = RemoteConnectionOptions::Docker(DockerConnectionOptions {
            name: "zed-dev-renamed".to_string(),
            container_id: "container-after".to_string(),
            remote_user: "anth".to_string(),
            local_folder: Some("/home/anth/project".to_string()),
            config_file: Some("/home/anth/project/.devcontainer/devcontainer.json".to_string()),
            ..Default::default()
        });

        assert!(same_remote_connection_identity(Some(&before), Some(&after)));
    }

    #[test]
    fn dev_container_identity_distinguishes_config_file() {
        // Same folder, different config file (a second, named dev container in
        // the same project) is a genuinely different remote.
        let left = RemoteConnectionOptions::Docker(DockerConnectionOptions {
            container_id: "container-123".to_string(),
            remote_user: "anth".to_string(),
            local_folder: Some("/home/anth/project".to_string()),
            config_file: Some("/home/anth/project/.devcontainer/devcontainer.json".to_string()),
            ..Default::default()
        });
        let right = RemoteConnectionOptions::Docker(DockerConnectionOptions {
            container_id: "container-123".to_string(),
            remote_user: "anth".to_string(),
            local_folder: Some("/home/anth/project".to_string()),
            config_file: Some(
                "/home/anth/project/.devcontainer/backend/devcontainer.json".to_string(),
            ),
            ..Default::default()
        });

        assert!(!same_remote_connection_identity(Some(&left), Some(&right)));
    }

    #[test]
    fn docker_identity_without_labels_falls_back_to_container_id() {
        // A Docker remote with no dev-container labels keeps the prior behavior
        // of identifying by container id.
        let left = RemoteConnectionOptions::Docker(DockerConnectionOptions {
            container_id: "container-123".to_string(),
            remote_user: "anth".to_string(),
            ..Default::default()
        });
        let right = RemoteConnectionOptions::Docker(DockerConnectionOptions {
            container_id: "container-456".to_string(),
            remote_user: "anth".to_string(),
            ..Default::default()
        });

        assert!(!same_remote_connection_identity(Some(&left), Some(&right)));
    }

    #[test]
    fn local_identity_matches_only_local_identity() {
        let remote = RemoteConnectionOptions::Wsl(WslConnectionOptions {
            distro_name: "Ubuntu".to_string(),
            user: Some("anth".to_string()),
        });

        assert!(same_remote_connection_identity(None, None));
        assert!(!same_remote_connection_identity(None, Some(&remote)));
    }
}
