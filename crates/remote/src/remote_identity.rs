use crate::{RemoteConnectionOptions, transport::ssh::SshConnectionHost};
use std::borrow::Cow;

/// Identifies a remote host for project and workspace matching.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RemoteConnectionIdentity<'a> {
    Ssh {
        host: Cow<'a, str>,
        username: Option<Cow<'a, str>>,
        port: Option<u16>,
    },
    Wsl {
        distro_name: Cow<'a, str>,
        user: Option<Cow<'a, str>>,
    },
    Docker {
        container_id: Cow<'a, str>,
        remote_user: Cow<'a, str>,
    },
    #[cfg(any(test, feature = "test-support"))]
    Mock { id: u64 },
}

impl RemoteConnectionIdentity<'_> {
    pub fn into_owned(self) -> RemoteConnectionIdentity<'static> {
        match self {
            Self::Ssh {
                host,
                username,
                port,
            } => RemoteConnectionIdentity::Ssh {
                host: Cow::Owned(host.into_owned()),
                username: username.map(|username| Cow::Owned(username.into_owned())),
                port,
            },
            Self::Wsl { distro_name, user } => RemoteConnectionIdentity::Wsl {
                distro_name: Cow::Owned(distro_name.into_owned()),
                user: user.map(|user| Cow::Owned(user.into_owned())),
            },
            Self::Docker {
                container_id,
                remote_user,
            } => RemoteConnectionIdentity::Docker {
                container_id: Cow::Owned(container_id.into_owned()),
                remote_user: Cow::Owned(remote_user.into_owned()),
            },
            #[cfg(any(test, feature = "test-support"))]
            Self::Mock { id } => RemoteConnectionIdentity::Mock { id },
        }
    }
}

impl<'a> From<&'a RemoteConnectionOptions> for RemoteConnectionIdentity<'a> {
    fn from(options: &'a RemoteConnectionOptions) -> Self {
        match options {
            RemoteConnectionOptions::Ssh(options) => Self::Ssh {
                host: match &options.host {
                    SshConnectionHost::Hostname(hostname) => Cow::Borrowed(hostname),
                    SshConnectionHost::IpAddr(address) => Cow::Owned(address.to_string()),
                },
                username: options.username.as_deref().map(Cow::Borrowed),
                port: options.port,
            },
            RemoteConnectionOptions::Wsl(options) => Self::Wsl {
                distro_name: Cow::Borrowed(&options.distro_name),
                user: options.user.as_deref().map(Cow::Borrowed),
            },
            RemoteConnectionOptions::Docker(options) => Self::Docker {
                container_id: Cow::Borrowed(&options.container_id),
                remote_user: Cow::Borrowed(&options.remote_user),
            },
            #[cfg(any(test, feature = "test-support"))]
            RemoteConnectionOptions::Mock(options) => Self::Mock { id: options.id },
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
    fn ssh_connection_matching() {
        let left = |args| {
            RemoteConnectionOptions::Ssh(SshConnectionOptions {
                host: "example.com".into(),
                username: Some("anth".to_string()),
                port: Some(2222),
                password: Some("secret".to_string()),
                args,
                connection_timeout: Some(30),
                nickname: Some("work".to_string()),
                upload_binary_over_ssh: true,
                ..Default::default()
            })
        };
        let right = |args| {
            RemoteConnectionOptions::Ssh(SshConnectionOptions {
                host: "example.com".into(),
                username: Some("anth".to_string()),
                port: Some(2222),
                password: None,
                args,
                connection_timeout: None,
                nickname: None,
                upload_binary_over_ssh: false,
                ..Default::default()
            })
        };

        for (left_args, right_args, expected) in [
            (Some(vec!["-v".into()]), Some(vec!["-v".into()]), true),
            (Some(vec!["-v".into()]), Some(vec!["-q".into()]), false),
            (None, Some(vec![]), true),
        ] {
            let left = left(left_args);
            let right = right(right_args);

            assert_eq!(left.identity(), right.identity());
            assert_eq!(left.matches_project_connection(&right), expected);
        }
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
        assert!(!left.matches_project_connection(&right));
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
