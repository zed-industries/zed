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
        container_id: String,
        name: String,
        remote_user: String,
    },
    #[cfg(any(test, feature = "test-support"))]
    Mock { id: u64 },
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
                container_id: options.container_id.clone(),
                name: options.name.clone(),
                remote_user: options.remote_user.clone(),
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
            upload_binary_over_docker_exec: true,
            use_podman: true,
            remote_env: BTreeMap::from([("FOO".to_string(), "BAR".to_string())]),
        });
        let right = RemoteConnectionOptions::Docker(DockerConnectionOptions {
            name: "zed-dev".to_string(),
            container_id: "container-123".to_string(),
            remote_user: "anth".to_string(),
            upload_binary_over_docker_exec: false,
            use_podman: false,
            remote_env: BTreeMap::new(),
        });

        assert!(same_remote_connection_identity(Some(&left), Some(&right),));
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
