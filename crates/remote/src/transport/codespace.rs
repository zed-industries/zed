use anyhow::{Context as _, Result};
use util::command::Stdio;

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct CodespaceConnectionOptions {
    pub name: String,
}

pub async fn list_codespaces() -> Result<Vec<CodespaceConnectionOptions>> {
    let output = run_gh_command(
        &["codespace", "list", "--json", "name"],
        "list GitHub Codespaces",
    )
    .await?;
    parse_codespace_list(&output)
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
}
