use std::{collections::HashSet, path::Path, process::Command};

use anyhow::{Context as _, Result, anyhow, bail};
use collections::HashMap;
use project::Project;
use regex::Regex;
use remote::Interactive;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use ui::{App, IconName};

const MAX_HERDR_SESSION_NAME_BYTES: usize = 64;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct HerdrTerminalSession {
    pub name: String,
    pub agent: Option<String>,
    pub agent_session: Option<HerdrAgentSession>,
    #[serde(default)]
    pub initial_command: Option<String>,
    #[serde(default)]
    pub initial_command_sent: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct HerdrAgentSession {
    pub source: String,
    pub agent: String,
    pub kind: String,
    pub value: String,
}

impl HerdrTerminalSession {
    fn snapshot_response(snapshot: &Value) -> &Value {
        let response = snapshot.get("result").unwrap_or(snapshot);
        response.get("snapshot").unwrap_or(response)
    }

    pub fn icon(&self) -> IconName {
        match self.agent.as_deref() {
            Some("claude") => IconName::AiClaude,
            Some("codex") => IconName::AiOpenAi,
            Some("gemini") => IconName::AiGemini,
            Some("opencode") => IconName::AiOpenCode,
            Some("copilot") => IconName::Copilot,
            Some("grok") => IconName::AiXAi,
            _ => IconName::Terminal,
        }
    }

    pub fn attach_command(&self) -> String {
        // A Herdr name is restricted to ASCII letters, digits, dots, underscores,
        // and hyphens, so this command cannot acquire shell syntax from a path.
        format!("herdr --session {}", self.name)
    }

    pub fn update_from_snapshot(&mut self, snapshot: &Value) -> bool {
        let response = Self::snapshot_response(snapshot);
        let Some(panes) = response.get("panes").and_then(Value::as_array) else {
            return false;
        };
        let focused_pane_id = response.get("focused_pane_id").and_then(Value::as_str);
        let pane = focused_pane_id
            .and_then(|id| {
                panes
                    .iter()
                    .find(|pane| pane.get("pane_id").and_then(Value::as_str) == Some(id))
            })
            .or_else(|| panes.first());
        let Some(pane) = pane else {
            return false;
        };
        let agent_session = pane
            .get("agent_session")
            .and_then(|session| serde_json::from_value(session.clone()).ok());
        let agent = pane
            .get("agent")
            .and_then(Value::as_str)
            .map(ToOwned::to_owned)
            .or_else(|| {
                agent_session
                    .as_ref()
                    .map(|session: &HerdrAgentSession| session.agent.clone())
            });
        if self.agent == agent && self.agent_session == agent_session {
            return false;
        }
        self.agent = agent;
        self.agent_session = agent_session;
        true
    }

    pub fn focused_pane_id(snapshot: &Value) -> Option<&str> {
        let response = Self::snapshot_response(snapshot);
        response.get("focused_pane_id").and_then(Value::as_str)
    }
}

pub fn session_base_name(path: &Path, pattern: Option<&str>) -> Result<String> {
    let matched = if let Some(pattern) = pattern {
        let regex = Regex::new(pattern).context("invalid Herdr session name regex")?;
        regex
            .captures(&path.to_string_lossy())
            .and_then(|captures| {
                captures
                    .get(1)
                    .or_else(|| captures.get(0))
                    .map(|capture| capture.as_str().to_owned())
            })
    } else {
        None
    };
    let source = matched.filter(|value| !value.is_empty()).or_else(|| {
        path.file_name()
            .map(|name| name.to_string_lossy().into_owned())
    });
    let source = source.ok_or_else(|| anyhow!("workspace has no directory name"))?;
    let name: String = source
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() || matches!(character, '-' | '_' | '.') {
                character
            } else {
                '-'
            }
        })
        .collect();
    let name = name.trim_matches(|character| character == '-' || character == '.');
    if name.is_empty() {
        bail!("workspace path yielded an empty Herdr session name");
    }
    Ok(name
        .get(..name.len().min(MAX_HERDR_SESSION_NAME_BYTES))
        .unwrap_or(name)
        .trim_end_matches(|character| character == '-' || character == '.')
        .to_owned())
}

pub fn available_name(base: &str, occupied: &HashSet<String>) -> String {
    if !occupied.contains(base) {
        return base.to_owned();
    }
    let mut suffix = 1u64;
    loop {
        let suffix_text = format!("-{suffix}");
        let prefix_length = MAX_HERDR_SESSION_NAME_BYTES.saturating_sub(suffix_text.len());
        let candidate = format!(
            "{}{}",
            base.get(..base.len().min(prefix_length))
                .unwrap_or(base)
                .trim_end_matches(|character| character == '-' || character == '.'),
            suffix_text
        );
        if !occupied.contains(&candidate) {
            return candidate;
        }
        suffix = suffix.saturating_add(1);
    }
}

pub fn command(
    project: &Project,
    args: &[String],
    session: Option<&str>,
    cx: &App,
) -> Result<Command> {
    let mut environment = HashMap::default();
    if let Some(session) = session {
        environment.insert("HERDR_SESSION".to_owned(), session.to_owned());
    }
    if let Some(remote_client) = project.remote_client() {
        let template = remote_client.read(cx).build_command(
            Some("herdr".to_owned()),
            args,
            &environment,
            None,
            None,
            Interactive::No,
        )?;
        let mut command = Command::new(template.program);
        command.args(template.args).envs(template.env);
        Ok(command)
    } else {
        let mut command = Command::new("herdr");
        command.args(args).envs(environment);
        Ok(command)
    }
}

pub fn run(mut command: Command) -> Result<Value> {
    let output = command.output().context("run Herdr command")?;
    let response: Option<Value> = serde_json::from_slice(&output.stdout).ok();
    if let Some(error) = response.as_ref().and_then(|value| value.get("error")) {
        bail!("Herdr returned an error: {error}");
    }
    if !output.status.success() {
        bail!(
            "Herdr command failed: {}{}",
            String::from_utf8_lossy(&output.stderr),
            String::from_utf8_lossy(&output.stdout)
        );
    }
    if output.stdout.is_empty() {
        return Ok(Value::Null);
    }
    response.ok_or_else(|| anyhow!("Herdr returned an invalid JSON response"))
}

pub fn session_names(value: &Value) -> Result<HashSet<String>> {
    let sessions = value
        .get("sessions")
        .and_then(Value::as_array)
        .ok_or_else(|| anyhow!("Herdr returned no session list"))?;
    Ok(sessions
        .iter()
        .filter_map(|session| session.get("name").and_then(Value::as_str))
        .map(ToOwned::to_owned)
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_from_workspace_path_and_collisions() {
        let path = Path::new(
            "/home/vatolin/arcadia-worktrees/sync-perfomance-CLEANWEB-6797/yweb/antispam/clean_web/pingers",
        );
        let base = session_base_name(path, Some("/arcadia-worktrees/([^/]+)"))
            .expect("valid session name");
        assert_eq!(base, "sync-perfomance-CLEANWEB-6797");
        let occupied = HashSet::from([base.clone(), format!("{base}-1")]);
        assert_eq!(available_name(&base, &occupied), format!("{base}-2"));
        let long_base = session_base_name(Path::new(&format!("/{}", "a".repeat(70))), None)
            .expect("long path still yields a valid name");
        assert_eq!(long_base.len(), 64);
        let candidate = available_name(&long_base, &HashSet::from([long_base.clone()]));
        assert_eq!(candidate.len(), 64);
        assert!(candidate.ends_with("-1"));
    }

    #[test]
    fn pane_snapshot_tracks_session_switches() {
        let mut session = HerdrTerminalSession {
            name: "project".to_owned(),
            agent: None,
            agent_session: None,
            initial_command: None,
            initial_command_sent: false,
        };
        let codex = serde_json::json!({"result": {"snapshot": {"focused_pane_id": "w1:p1", "panes": [{"pane_id": "w1:p1", "agent": "codex", "agent_session": {"source": "herdr:codex", "agent": "codex", "kind": "id", "value": "first"}}]}}});
        assert!(session.update_from_snapshot(&codex));
        assert_eq!(
            session
                .agent_session
                .as_ref()
                .map(|value| value.value.as_str()),
            Some("first")
        );
        let next_codex = serde_json::json!({"result": {"snapshot": {"focused_pane_id": "w1:p1", "panes": [{"pane_id": "w1:p1", "agent": "codex", "agent_session": {"source": "herdr:codex", "agent": "codex", "kind": "id", "value": "next"}}]}}});
        assert!(session.update_from_snapshot(&next_codex));
        assert_eq!(
            session
                .agent_session
                .as_ref()
                .map(|value| value.value.as_str()),
            Some("next")
        );
        let claude = serde_json::json!({"result": {"snapshot": {"focused_pane_id": "w1:p1", "panes": [{"pane_id": "w1:p1", "agent": "claude", "agent_session": {"source": "herdr:claude", "agent": "claude", "kind": "id", "value": "second"}}]}}});
        assert!(session.update_from_snapshot(&claude));
        assert_eq!(session.icon(), IconName::AiClaude);
        assert_eq!(
            session
                .agent_session
                .as_ref()
                .map(|value| value.value.as_str()),
            Some("second")
        );
    }
}
