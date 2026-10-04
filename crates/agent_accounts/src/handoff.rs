//! Moves a conversation to another agent or account with txcript.
//!
//! Both kinds of switch are the same operation: export the source session
//! with the source account's environment into a Simple document, then
//! continue that document into the target harness with the target account's
//! environment. txcript writes a brand-new native session there and never
//! touches the source; the caller opens the new session with ACP
//! `session/load`.

use std::path::{Path, PathBuf};

use anyhow::{Context as _, Result, anyhow, bail};
use collections::HashMap;

use crate::{AccountId, AccountProvider};

/// One side of a transfer: whose store the session lives in.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionEndpoint {
    pub provider: AccountProvider,
    /// `None` is the default account.
    pub account: Option<AccountId>,
}

#[derive(Debug, Clone)]
pub struct TransferRequest {
    pub source: SessionEndpoint,
    pub source_session_id: String,
    pub target: SessionEndpoint,
    /// The project directory the conversation belongs to.
    pub cwd: PathBuf,
    /// The user's shell environment, used to find txcript and as the base
    /// environment of both steps.
    pub shell_env: HashMap<String, String>,
}

/// Runs the transfer and returns the id of the new native session.
///
/// txcript runs with `kill_on_drop`, so dropping the future (for example on
/// a caller's timeout) stops it.
pub async fn transfer(request: TransferRequest) -> Result<String> {
    let txcript = find_txcript(&request.shell_env).context(
        "txcript was not found in your shell PATH; install it to continue conversations in another agent",
    )?;
    let document_dir = tempfile::tempdir().context("creating a temporary directory")?;
    let document = document_dir.path().join("session.simple.md");

    let export_output = run_step(
        &txcript,
        &[
            "export".as_ref(),
            request.source_session_id.as_ref(),
            "--from".as_ref(),
            request.source.provider.harness().as_ref(),
            "--out".as_ref(),
            document.as_os_str(),
        ],
        &request.cwd,
        &step_env(&request.shell_env, &request.source),
    )
    .await
    .context("exporting the conversation")?;
    if !document.is_file() {
        bail!(
            "txcript export did not write the conversation: {}",
            export_output.trim()
        );
    }

    let continue_output = run_step(
        &txcript,
        &[
            "continue".as_ref(),
            document.as_os_str(),
            "--with".as_ref(),
            request.target.provider.harness().as_ref(),
            "--no-resume".as_ref(),
        ],
        &request.cwd,
        &step_env(&request.shell_env, &request.target),
    )
    .await
    .context("writing the conversation for the target agent")?;

    parse_new_session_id(&continue_output, &request.source_session_id).ok_or_else(|| {
        anyhow!(
            "txcript did not report the new session id: {}",
            continue_output.trim()
        )
    })
}

fn step_env(
    shell_env: &HashMap<String, String>,
    endpoint: &SessionEndpoint,
) -> HashMap<String, String> {
    let mut env = shell_env.clone();
    if let Some(account) = &endpoint.account {
        env.extend(endpoint.provider.account_env(account.as_str()));
    }
    env
}

/// Runs one txcript step and returns its combined output.
async fn run_step(
    program: &Path,
    args: &[&std::ffi::OsStr],
    cwd: &Path,
    env: &HashMap<String, String>,
) -> Result<String> {
    let mut command = util::command::new_command(program);
    command
        .args(args)
        .current_dir(cwd)
        .env_clear()
        .envs(env)
        .kill_on_drop(true);
    let output = command.output().await.context("running txcript")?;
    let mut text = String::from_utf8_lossy(&output.stdout).into_owned();
    text.push_str(&String::from_utf8_lossy(&output.stderr));
    if !output.status.success() {
        bail!("txcript failed ({}): {}", output.status, text.trim());
    }
    Ok(text)
}

fn find_txcript(shell_env: &HashMap<String, String>) -> Option<PathBuf> {
    let mut dirs: Vec<PathBuf> = shell_env
        .get("PATH")
        .map(|path| std::env::split_paths(path).collect())
        .unwrap_or_default();
    // Its installer's default location, which GUI-launched shells may miss.
    dirs.push(util::paths::home_dir().join(".local/bin"));
    dirs.into_iter()
        .map(|dir| dir.join("txcript"))
        .find(|candidate| candidate.is_file())
}

/// Reads the id from txcript's `resume with: <cli> … <id>` line, falling
/// back to the last id-shaped token that is not the source.
fn parse_new_session_id(output: &str, source_session_id: &str) -> Option<String> {
    let from_resume_line = output
        .lines()
        .filter(|line| line.contains("resume with:"))
        .filter_map(|line| line.split_whitespace().last())
        .rfind(|token| is_session_id(token));
    from_resume_line
        .or_else(|| {
            output
                .split(|c: char| !(c.is_ascii_hexdigit() || c == '-'))
                .rfind(|token| is_session_id(token) && *token != source_session_id)
        })
        .map(str::to_string)
}

fn is_session_id(token: &str) -> bool {
    let groups: Vec<&str> = token.split('-').collect();
    groups.len() == 5
        && groups
            .iter()
            .zip([8, 4, 4, 4, 12])
            .all(|(group, len)| group.len() == len && group.chars().all(|c| c.is_ascii_hexdigit()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn parses_claude_and_codex_output() {
        let claude = "simple → claude_code  /tmp/t/projects/-x/b022b18c-36da-4f2c-bc52-0fccf3078562.jsonl\n  resume with: claude --resume b022b18c-36da-4f2c-bc52-0fccf3078562\n";
        assert_eq!(
            parse_new_session_id(claude, "4cfbecbb-2fb1-4fe0-860a-6f06a214cc2e").as_deref(),
            Some("b022b18c-36da-4f2c-bc52-0fccf3078562")
        );
        let codex = "simple → codex  /tmp/t/sessions/2026/10/03/rollout-2026-10-03T18-44-03-01a10314-9706-7be0-b8f2-11747cccb1cb.jsonl\n  resume with: codex resume 01a10314-9706-7be0-b8f2-11747cccb1cb\n";
        assert_eq!(
            parse_new_session_id(codex, "01a10277-56f1-7501-869a-1b8230b5ea5d").as_deref(),
            Some("01a10314-9706-7be0-b8f2-11747cccb1cb")
        );
    }

    #[test]
    fn falls_back_to_last_id_that_is_not_the_source() {
        let output = "converted 4cfbecbb-2fb1-4fe0-860a-6f06a214cc2e into 95f46939-a30b-40c3-a855-d1122fa1a938";
        assert_eq!(
            parse_new_session_id(output, "4cfbecbb-2fb1-4fe0-860a-6f06a214cc2e").as_deref(),
            Some("95f46939-a30b-40c3-a855-d1122fa1a938")
        );
        assert_eq!(parse_new_session_id("nothing here", "x"), None);
    }

    #[test]
    fn step_env_sets_only_the_account_variable() {
        let shell_env = HashMap::from_iter([("PATH".to_string(), "/bin".to_string())]);
        let env = step_env(
            &shell_env,
            &SessionEndpoint {
                provider: AccountProvider::Codex,
                account: Some(AccountId::from("/Users/me/.codex-2")),
            },
        );
        assert_eq!(
            env.get("CODEX_HOME").map(String::as_str),
            Some("/Users/me/.codex-2")
        );
        assert_eq!(env.get("PATH").map(String::as_str), Some("/bin"));

        let default = step_env(
            &shell_env,
            &SessionEndpoint {
                provider: AccountProvider::Claude,
                account: None,
            },
        );
        assert_eq!(default, shell_env);
    }

    #[cfg(unix)]
    #[test]
    fn transfer_runs_export_then_continue_with_each_account() {
        use std::os::unix::fs::PermissionsExt as _;

        let temp = tempfile::tempdir().unwrap();
        let bin = temp.path().join("bin");
        std::fs::create_dir_all(&bin).unwrap();
        let log = temp.path().join("calls.log");
        let script = format!(
            r#"#!/bin/sh
echo "$1 CLAUDE_CONFIG_DIR=$CLAUDE_CONFIG_DIR CODEX_HOME=$CODEX_HOME" >> "{log}"
case "$1" in
  export) echo "doc" > "$6" ;;
  continue) echo "  resume with: codex resume 01a10314-9826-7152-9f3c-9f9e03b7c7f1" ;;
esac
"#,
            log = log.display()
        );
        let txcript = bin.join("txcript");
        std::fs::write(&txcript, script).unwrap();
        std::fs::set_permissions(&txcript, std::fs::Permissions::from_mode(0o755)).unwrap();

        let request = TransferRequest {
            source: SessionEndpoint {
                provider: AccountProvider::Claude,
                account: Some(AccountId::from("/p/.claude-work")),
            },
            source_session_id: "4cfbecbb-2fb1-4fe0-860a-6f06a214cc2e".into(),
            target: SessionEndpoint {
                provider: AccountProvider::Codex,
                account: Some(AccountId::from("/p/.codex-2")),
            },
            cwd: temp.path().to_path_buf(),
            shell_env: HashMap::from_iter([(
                "PATH".to_string(),
                format!("{}:/bin:/usr/bin", bin.display()),
            )]),
        };
        let new_id = smol::block_on(transfer(request)).unwrap();
        assert_eq!(new_id, "01a10314-9826-7152-9f3c-9f9e03b7c7f1");
        assert_eq!(
            std::fs::read_to_string(&log).unwrap(),
            "export CLAUDE_CONFIG_DIR=/p/.claude-work CODEX_HOME=\ncontinue CLAUDE_CONFIG_DIR= CODEX_HOME=/p/.codex-2\n"
        );
    }
}
