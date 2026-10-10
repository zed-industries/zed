use crate::{
    RemoteConnectionOptions, SshConnectionOptions, WslConnectionOptions,
    transport::ssh::SshConnectionHost,
};

/// Identifies a remote host for project and workspace matching.
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
        remote_user: String,
    },
    #[cfg(any(test, feature = "test-support"))]
    Mock { id: u64 },
}

impl From<&RemoteConnectionOptions> for RemoteConnectionIdentity {
    fn from(options: &RemoteConnectionOptions) -> Self {
        match options {
            RemoteConnectionOptions::Ssh(options) => options.into(),
            RemoteConnectionOptions::Wsl(options) => options.into(),
            RemoteConnectionOptions::Docker(options) => Self::Docker {
                container_id: options.container_id.clone(),
                remote_user: options.remote_user.clone(),
            },
            #[cfg(any(test, feature = "test-support"))]
            RemoteConnectionOptions::Mock(options) => Self::Mock { id: options.id },
        }
    }
}

impl From<&SshConnectionOptions> for RemoteConnectionIdentity {
    fn from(options: &SshConnectionOptions) -> Self {
        Self::Ssh {
            host: options.host.to_string(),
            username: options.username.clone(),
            port: options.port,
        }
    }
}

impl From<&settings_content::SshConnection> for RemoteConnectionIdentity {
    fn from(connection: &settings_content::SshConnection) -> Self {
        Self::Ssh {
            host: SshConnectionHost::from(connection.host.as_str()).to_string(),
            username: connection.username.clone(),
            port: connection.port,
        }
    }
}

impl From<&WslConnectionOptions> for RemoteConnectionIdentity {
    fn from(options: &WslConnectionOptions) -> Self {
        Self::Wsl {
            distro_name: options.distro_name.clone(),
            user: options.user.clone(),
        }
    }
}

impl From<&settings_content::WslConnection> for RemoteConnectionIdentity {
    fn from(connection: &settings_content::WslConnection) -> Self {
        Self::Wsl {
            distro_name: connection.distro_name.clone(),
            user: connection.user.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use crate::{
        DockerConnectionOptions, RemoteConnectionOptions, SshConnectionOptions,
        WslConnectionOptions,
    };

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

        assert_eq!(left.identity(), right.identity());
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

        assert_ne!(left.identity(), right.identity());
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

        assert_ne!(left.identity(), right.identity());
    }

    #[test]
    fn docker_identity_ignores_display_name_and_runtime_fields() {
        let left = RemoteConnectionOptions::Docker(DockerConnectionOptions {
            name: "renamed-dev".to_string(),
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

        assert_eq!(left.identity(), right.identity());
    }
}
