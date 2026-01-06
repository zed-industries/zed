//! Command execution for command-based rules.
//!
//! This module provides functionality to execute shell commands specified in rules,
//! with timeouts, output limits, and error handling.

use anyhow::{Context as _, Result, anyhow};
use async_process::{Command, Stdio};
use futures::io::AsyncReadExt;
use futures::{FutureExt, select};
use smol::Timer;
use std::time::Duration;

use crate::file_store::CommandConfig;

/// Result of command execution
#[derive(Debug, Clone)]
pub struct CommandExecutionResult {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: Option<i32>,
    pub success: bool,
    pub truncated: bool,
    pub timed_out: bool,
}

impl CommandExecutionResult {
    /// Get the output to use as rule content (stdout only if successful)
    pub fn output_for_rule(&self) -> Option<String> {
        if self.success && !self.stdout.is_empty() {
            Some(self.stdout.clone())
        } else {
            None
        }
    }
}

/// Execute a command with the given configuration
pub async fn execute_command(config: &CommandConfig) -> Result<CommandExecutionResult> {
    let timeout_duration = Duration::from_secs(config.timeout_seconds);

    // Execute with timeout using futures::select
    let execute_future = execute_command_inner(config).fuse();
    let timeout_future = Timer::after(timeout_duration).fuse();

    futures::pin_mut!(execute_future, timeout_future);

    select! {
        result = execute_future => result,
        _ = timeout_future => {
            // Timeout occurred
            Ok(CommandExecutionResult {
                stdout: String::new(),
                stderr: format!("Command timed out after {} seconds", config.timeout_seconds),
                exit_code: None,
                success: false,
                truncated: false,
                timed_out: true,
            })
        }
    }
}

async fn execute_command_inner(config: &CommandConfig) -> Result<CommandExecutionResult> {
    let mut cmd = Command::new(&config.cmd);
    cmd.args(&config.args);
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());

    // Spawn the process
    let mut child = cmd
        .spawn()
        .context(format!("Failed to spawn command: {}", config.cmd))?;

    let mut stdout_handle = child
        .stdout
        .take()
        .ok_or_else(|| anyhow!("Failed to capture stdout"))?;

    let mut stderr_handle = child
        .stderr
        .take()
        .ok_or_else(|| anyhow!("Failed to capture stderr"))?;

    // Read stdout with size limit
    let mut stdout_bytes = Vec::with_capacity(config.max_output_bytes.min(65536));
    let mut buffer = vec![0u8; 8192];
    let mut total_read = 0;
    let mut truncated = false;

    loop {
        match stdout_handle.read(&mut buffer).await {
            Ok(0) => break, // EOF
            Ok(n) => {
                total_read += n;
                if total_read <= config.max_output_bytes {
                    stdout_bytes.extend_from_slice(&buffer[..n]);
                } else {
                    // Truncate to max size
                    let remaining = config.max_output_bytes.saturating_sub(stdout_bytes.len());
                    if remaining > 0 {
                        stdout_bytes.extend_from_slice(&buffer[..remaining.min(n)]);
                    }
                    truncated = true;
                    break;
                }
            }
            Err(e) => {
                return Err(anyhow!("Failed to read stdout: {}", e));
            }
        }
    }

    // Read stderr (with same limit)
    let mut stderr_bytes = Vec::with_capacity(config.max_output_bytes.min(65536));
    let mut stderr_total = 0;

    loop {
        match stderr_handle.read(&mut buffer).await {
            Ok(0) => break,
            Ok(n) => {
                stderr_total += n;
                if stderr_total <= config.max_output_bytes {
                    stderr_bytes.extend_from_slice(&buffer[..n]);
                } else {
                    let remaining = config.max_output_bytes.saturating_sub(stderr_bytes.len());
                    if remaining > 0 {
                        stderr_bytes.extend_from_slice(&buffer[..remaining.min(n)]);
                    }
                    break;
                }
            }
            Err(e) => {
                return Err(anyhow!("Failed to read stderr: {}", e));
            }
        }
    }

    // Wait for the process to complete
    let status = child.status().await.context("Failed to wait for command")?;

    let stdout = String::from_utf8_lossy(&stdout_bytes).to_string();
    let stderr = String::from_utf8_lossy(&stderr_bytes).to_string();

    Ok(CommandExecutionResult {
        stdout,
        stderr,
        exit_code: status.code(),
        success: status.success(),
        truncated,
        timed_out: false,
    })
}

#[cfg(test)]
mod tests {
    use super::{CommandConfig, execute_command};

    #[gpui::test]
    async fn test_execute_simple_command() {
        let config = CommandConfig {
            cmd: "echo".to_string(),
            args: vec!["hello world".to_string()],
            timeout_seconds: 5,
            max_output_bytes: 10_000,
            on_startup: false,
            on_new_chat: true,
            on_every_message: false,
        };

        let result = execute_command(&config).await.unwrap();
        assert!(result.success);
        assert_eq!(result.stdout.trim(), "hello world");
        assert!(result.stderr.is_empty());
        assert!(!result.truncated);
        assert!(!result.timed_out);
    }

    #[gpui::test]
    async fn test_command_with_stderr() {
        let config = CommandConfig {
            cmd: "sh".to_string(),
            args: vec!["-c".to_string(), "echo error >&2".to_string()],
            timeout_seconds: 5,
            max_output_bytes: 10_000,
            on_startup: false,
            on_new_chat: true,
            on_every_message: false,
        };

        let result = execute_command(&config).await.unwrap();
        assert!(result.success);
        assert!(result.stdout.is_empty());
        assert_eq!(result.stderr.trim(), "error");
    }

    #[gpui::test]
    async fn test_command_timeout() {
        let config = CommandConfig {
            cmd: "sleep".to_string(),
            args: vec!["10".to_string()],
            timeout_seconds: 1,
            max_output_bytes: 10_000,
            on_startup: false,
            on_new_chat: true,
            on_every_message: false,
        };

        let result = execute_command(&config).await.unwrap();
        assert!(!result.success);
        assert!(result.timed_out);
    }

    #[gpui::test]
    async fn test_output_truncation() {
        let config = CommandConfig {
            cmd: "sh".to_string(),
            args: vec![
                "-c".to_string(),
                "for i in $(seq 1 1000); do echo $i; done".to_string(),
            ],
            timeout_seconds: 5,
            max_output_bytes: 100,
            on_startup: false,
            on_new_chat: true,
            on_every_message: false,
        };

        let result = execute_command(&config).await.unwrap();
        assert!(result.success);
        assert!(result.truncated);
        assert!(result.stdout.len() <= 100);
    }

    #[gpui::test]
    async fn test_failed_command() {
        let config = CommandConfig {
            cmd: "sh".to_string(),
            args: vec!["-c".to_string(), "exit 1".to_string()],
            timeout_seconds: 5,
            max_output_bytes: 10_000,
            on_startup: false,
            on_new_chat: true,
            on_every_message: false,
        };

        let result = execute_command(&config).await.unwrap();
        assert!(!result.success);
        assert_eq!(result.exit_code, Some(1));
    }
}
