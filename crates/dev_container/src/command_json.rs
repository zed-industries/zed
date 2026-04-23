use std::process::Output;

use async_trait::async_trait;
use serde::Deserialize;
use util::command::Command;

use crate::devcontainer_api::DevContainerError;

pub(crate) struct DefaultCommandRunner;

impl DefaultCommandRunner {
    pub(crate) fn new() -> Self {
        Self
    }
}

#[async_trait]
impl CommandRunner for DefaultCommandRunner {
    async fn run_command(&self, command: &mut Command) -> Result<Output, std::io::Error> {
        command.output().await
    }
}

#[async_trait]
pub(crate) trait CommandRunner: Send + Sync {
    async fn run_command(&self, command: &mut Command) -> Result<Output, std::io::Error>;
}

pub(crate) async fn evaluate_json_command<T>(
    mut command: Command,
) -> Result<Option<T>, DevContainerError>
where
    T: for<'de> Deserialize<'de>,
{
    let output = command.output().await.map_err(|e| {
        log::error!("Error running command {:?}: {e}", command);
        DevContainerError::CommandFailed(command.get_program().display().to_string())
    })?;

    deserialize_json_output(output).map_err(|e| {
        log::error!("Error running command {:?}: {e}", command);
        DevContainerError::CommandFailed(command.get_program().display().to_string())
    })
}

pub(crate) fn deserialize_json_output<T>(output: Output) -> Result<Option<T>, String>
where
    T: for<'de> Deserialize<'de>,
{
    if output.status.success() {
        let raw = String::from_utf8_lossy(&output.stdout);
        if raw.is_empty() || raw.trim() == "[]" || raw.trim() == "{}" {
            return Ok(None);
        }
        serde_json_lenient::from_str(&raw)
            .map_err(|e| format!("Error deserializing from raw json: {e}"))
    } else {
        let std_err = String::from_utf8_lossy(&output.stderr);
        Err(format!(
            "Sent non-successful output; cannot deserialize. StdErr: {std_err}"
        ))
    }
}

#[cfg(test)]
mod tests {
    use std::process::ExitStatus;

    use super::*;

    fn success_output(stdout: &str) -> Output {
        Output {
            status: ExitStatus::default(),
            stdout: stdout.as_bytes().to_vec(),
            stderr: Vec::new(),
        }
    }

    #[derive(Debug, Deserialize, PartialEq)]
    struct TestItem {
        id: String,
    }

    #[test]
    fn test_deserialize_newline_delimited_json_rejected() {
        // Strict single-value contract: NDJSON must be rejected. Commands that
        // may legitimately return multiple rows (e.g. `docker ps`) parse their
        // output themselves rather than routing through this helper.
        let output = success_output("{\"id\":\"first\"}\n{\"id\":\"second\"}\n");
        let result: Result<Option<TestItem>, String> = deserialize_json_output(output);
        assert!(result.is_err(), "expected parse error, got {result:?}");
    }

    #[test]
    fn test_deserialize_empty_output() {
        let output = success_output("");
        let result: Option<TestItem> = deserialize_json_output(output).unwrap();
        assert_eq!(result, None);
    }

    #[test]
    fn test_deserialize_empty_object() {
        let output = success_output("{}");
        let result: Option<TestItem> = deserialize_json_output(output).unwrap();
        assert_eq!(result, None);
    }
}
