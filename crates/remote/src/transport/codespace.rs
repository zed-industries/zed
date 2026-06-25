use crate::transport::ssh::{SshConnectionHost, SshConnectionOptions};
use anyhow::{Context as _, Result};
use smol::fs;
use tempfile::TempDir;
use util::command::Stdio;

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct CodespaceConnectionOptions {
    pub name: String,
}

pub(crate) struct CodespaceSshConnection {
    pub(crate) ssh_options: SshConnectionOptions,
    pub(crate) ssh_config_dir: TempDir,
}

pub(crate) async fn codespace_ssh_connection(
    connection_options: &CodespaceConnectionOptions,
) -> Result<CodespaceSshConnection> {
    log::info!("Connecting to GitHub Codespace {}", connection_options.name);
    let ssh_config = generate_ssh_config(&connection_options.name).await?;
    let host = parse_codespace_ssh_host(&ssh_config)?;

    let ssh_config_dir = tempfile::Builder::new()
        .prefix("zed-codespace-ssh-config")
        .tempdir()?;
    let ssh_config_path = ssh_config_dir.path().join("config");
    fs::write(&ssh_config_path, ssh_config)
        .await
        .with_context(|| {
            format!(
                "writing Codespaces SSH config to {}",
                ssh_config_path.display()
            )
        })?;

    Ok(CodespaceSshConnection {
        ssh_options: SshConnectionOptions {
            host: SshConnectionHost::from(host),
            args: Some(vec![
                "-F".to_string(),
                ssh_config_path.display().to_string(),
            ]),
            ..Default::default()
        },
        ssh_config_dir,
    })
}

pub async fn list_codespaces() -> Result<Vec<CodespaceConnectionOptions>> {
    let output = run_gh_command(
        &["codespace", "list", "--json", "name"],
        "list GitHub Codespaces",
    )
    .await?;
    parse_codespace_list(&output)
}

async fn generate_ssh_config(codespace_name: &str) -> Result<String> {
    run_gh_command(
        &[
            "codespace",
            "ssh",
            "--config",
            "--codespace",
            codespace_name,
        ],
        "generate Codespaces SSH configuration",
    )
    .await
}

async fn run_gh_command(args: &[&str], context: &str) -> Result<String> {
    let mut command = util::command::new_command("gh");
    command
        .args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let output = command
        .output()
        .await
        .with_context(|| format!("failed to run `gh` to {context}"))?;
    anyhow::ensure!(
        output.status.success(),
        "`gh` failed to {context}: {}",
        String::from_utf8_lossy(&output.stderr).trim()
    );
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

pub(crate) fn parse_codespace_list(output: &str) -> Result<Vec<CodespaceConnectionOptions>> {
    serde_json::from_str(output).context("parsing `gh codespace list` output")
}

fn parse_codespace_ssh_host(output: &str) -> Result<String> {
    let mut host = None;
    for line in output.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let mut parts = line.splitn(2, char::is_whitespace);
        let Some(key) = parts.next() else {
            continue;
        };
        let Some(value) = parts.next() else {
            continue;
        };
        if key.to_ascii_lowercase().as_str() == "host" {
            if let Some(host_name) = value.trim().split_whitespace().find(|host_name| {
                !host_name.is_empty() && !host_name.contains('*') && !host_name.contains('?')
            }) {
                host = Some(host_name.to_string());
            }
        }
    }
    host.context("Codespaces SSH config did not include Host")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_codespace_list_output() {
        let output = r#"[
            {
                "name": "octocat-hello-123"
            }
        ]"#;

        assert_eq!(
            parse_codespace_list(output).unwrap(),
            vec![CodespaceConnectionOptions {
                name: "octocat-hello-123".to_string(),
            }]
        );
    }

    #[test]
    fn parses_empty_codespace_list_output() {
        assert!(parse_codespace_list("[]").unwrap().is_empty());
    }

    #[test]
    fn parses_codespace_list_with_missing_optional_fields() {
        let output = r#"[{ "name": "octocat-hello-123" }]"#;

        assert_eq!(
            parse_codespace_list(output).unwrap(),
            vec![CodespaceConnectionOptions {
                name: "octocat-hello-123".to_string(),
            }]
        );
    }

    #[test]
    fn rejects_malformed_codespace_list_output() {
        assert!(parse_codespace_list("not json").is_err());
    }

    #[test]
    fn parses_codespace_ssh_config() {
        let config = r#"
            Host cs.octocat-hello-123.main
              User codespace
              ProxyCommand gh cs ssh -c octocat-hello-123 --stdio -- -i /home/me/.ssh/codespaces.auto
              UserKnownHostsFile=/dev/null
              StrictHostKeyChecking no
              LogLevel quiet
              ControlMaster auto
              IdentityFile /home/me/.ssh/codespaces.auto
        "#;

        assert_eq!(
            parse_codespace_ssh_host(config).unwrap(),
            "cs.octocat-hello-123.main"
        );
    }

    #[test]
    fn ignores_wildcard_codespace_ssh_hosts() {
        let config = r#"
            Host *
              StrictHostKeyChecking no

            Host cs.octocat-hello-123.main
              User codespace
        "#;

        assert_eq!(
            parse_codespace_ssh_host(config).unwrap(),
            "cs.octocat-hello-123.main"
        );
    }

    #[test]
    fn rejects_incomplete_codespace_ssh_config() {
        assert!(parse_codespace_ssh_host("User codespace").is_err());
    }
}
