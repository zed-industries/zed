use std::sync::Arc;

use anyhow::{Result, bail};
use extension::{ExtensionCapability, ExtensionManifest};
use url::Url;

pub struct CapabilityGranter {
    granted_capabilities: Vec<ExtensionCapability>,
    manifest: Arc<ExtensionManifest>,
}

impl CapabilityGranter {
    pub fn new(
        granted_capabilities: Vec<ExtensionCapability>,
        manifest: Arc<ExtensionManifest>,
    ) -> Self {
        Self {
            granted_capabilities,
            manifest,
        }
    }

    pub fn grant_exec(
        &self,
        desired_command: &str,
        desired_args: &[impl AsRef<str> + std::fmt::Debug],
    ) -> Result<()> {
        self.manifest.allow_exec(desired_command, desired_args)?;

        let is_allowed = self
            .granted_capabilities
            .iter()
            .any(|capability| match capability {
                ExtensionCapability::ProcessExec(capability) => {
                    capability.allows(desired_command, desired_args)
                }
                _ => false,
            });

        if !is_allowed {
            bail!(
                "capability for process:exec {desired_command} {desired_args:?} is not granted by the extension host",
            );
        }

        Ok(())
    }

    pub fn grant_download_file(&self, desired_url: &Url) -> Result<()> {
        let is_allowed = self
            .granted_capabilities
            .iter()
            .any(|capability| match capability {
                ExtensionCapability::DownloadFile(capability) => capability.allows(desired_url),
                _ => false,
            });

        if !is_allowed {
            bail!(
                "capability for download_file {desired_url} is not granted by the extension host",
            );
        }

        Ok(())
    }

    pub fn grant_npm_install_package(&self, package_name: &str) -> Result<()> {
        let is_allowed = self
            .granted_capabilities
            .iter()
            .any(|capability| match capability {
                ExtensionCapability::NpmInstallPackage(capability) => {
                    capability.allows(package_name)
                }
                _ => false,
            });

        if !is_allowed {
            bail!("capability for npm:install {package_name} is not granted by the extension host",);
        }

        Ok(())
    }

    /// Grants a local TCP connection only when both the extension manifest and
    /// the user's settings name the same loopback endpoint.
    pub fn grant_tcp_connect(&self, desired_host: &str, desired_port: u16) -> Result<()> {
        let is_manifest_allowed =
            self.manifest
                .capabilities
                .iter()
                .any(|capability| match capability {
                    ExtensionCapability::TcpConnect(capability) => {
                        capability.allows(desired_host, desired_port)
                    }
                    _ => false,
                });

        if !is_manifest_allowed {
            bail!(
                "capability for network:tcp-local {desired_host}:{desired_port} was not listed in the extension manifest"
            );
        }

        let is_granted = self
            .granted_capabilities
            .iter()
            .any(|capability| match capability {
                ExtensionCapability::TcpConnect(capability) => {
                    capability.allows(desired_host, desired_port)
                }
                _ => false,
            });

        if !is_granted {
            bail!(
                "capability for network:tcp-local {desired_host}:{desired_port} is not granted by the extension host"
            );
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use extension::{ProcessExecCapability, SchemaVersion, TcpConnectCapability};

    use super::*;

    fn extension_manifest() -> ExtensionManifest {
        ExtensionManifest {
            id: "test".into(),
            name: "Test".to_string(),
            version: "1.0.0".into(),
            schema_version: SchemaVersion::ZERO,
            description: None,
            repository: None,
            authors: vec![],
            lib: Default::default(),
            themes: vec![],
            icon_themes: vec![],
            languages: vec![],
            grammars: BTreeMap::default(),
            language_servers: BTreeMap::default(),
            context_servers: BTreeMap::default(),
            slash_commands: BTreeMap::default(),
            snippets: None,
            capabilities: vec![],
            debug_adapters: Default::default(),
            debug_locators: Default::default(),
            language_model_providers: BTreeMap::default(),
        }
    }

    #[test]
    fn test_grant_exec() {
        let manifest = Arc::new(ExtensionManifest {
            capabilities: vec![ExtensionCapability::ProcessExec(ProcessExecCapability {
                command: "ls".to_string(),
                args: vec!["-la".to_string()],
            })],
            ..extension_manifest()
        });

        // It returns an error when the extension host has no granted capabilities.
        let granter = CapabilityGranter::new(Vec::new(), manifest.clone());
        assert!(granter.grant_exec("ls", &["-la"]).is_err());

        // It succeeds when the extension host has the exact capability.
        let granter = CapabilityGranter::new(
            vec![ExtensionCapability::ProcessExec(ProcessExecCapability {
                command: "ls".to_string(),
                args: vec!["-la".to_string()],
            })],
            manifest.clone(),
        );
        assert!(granter.grant_exec("ls", &["-la"]).is_ok());

        // It succeeds when the extension host has a wildcard capability.
        let granter = CapabilityGranter::new(
            vec![ExtensionCapability::ProcessExec(ProcessExecCapability {
                command: "*".to_string(),
                args: vec!["**".to_string()],
            })],
            manifest,
        );
        assert!(granter.grant_exec("ls", &["-la"]).is_ok());
    }

    #[test]
    fn test_grant_tcp_connect_requires_manifest_and_user_grant() {
        let tcp_capability = TcpConnectCapability {
            host: "localhost".to_string(),
            port: Some(7888),
        };
        let manifest = Arc::new(ExtensionManifest {
            capabilities: vec![ExtensionCapability::TcpConnect(tcp_capability.clone())],
            ..extension_manifest()
        });

        let granter = CapabilityGranter::new(Vec::new(), manifest.clone());
        assert!(granter.grant_tcp_connect("localhost", 7888).is_err());

        let granter = CapabilityGranter::new(
            vec![ExtensionCapability::TcpConnect(tcp_capability)],
            manifest,
        );
        assert!(granter.grant_tcp_connect("localhost", 7888).is_ok());
        assert!(granter.grant_tcp_connect("localhost", 7889).is_err());
        assert!(granter.grant_tcp_connect("example.com", 7888).is_err());
    }
}
