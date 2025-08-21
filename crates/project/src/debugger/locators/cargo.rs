use anyhow::{Context as _, Result};
use async_trait::async_trait;
use dap::{DapLocator, DebugRequest, adapters::DebugAdapterName};
use gpui::SharedString;
use serde_json::{Value, json};
use smol::{
    Timer,
    io::AsyncReadExt,
    process::{Command, Stdio},
};
use std::time::Duration;
use task::{BuildTaskDefinition, DebugScenario, ShellBuilder, SpawnInTerminal, TaskTemplate};

pub(crate) struct CargoLocator;

async fn find_best_executable(executables: &[String], test_name: &str) -> Option<String> {
    if executables.len() == 1 {
        return executables.first().cloned();
    }
    for executable in executables {
        let Some(mut child) = Command::new(&executable)
            .arg("--list")
            .stdout(Stdio::piped())
            .spawn()
            .ok()
        else {
            continue;
        };
        let mut test_lines = String::default();
        let exec_result = smol::future::race(
            async {
                if let Some(mut stdout) = child.stdout.take() {
                    stdout.read_to_string(&mut test_lines).await?;
                }
                Ok(())
            },
            async {
                Timer::after(Duration::from_secs(3)).await;
                anyhow::bail!("Timed out waiting for executable stdout")
            },
        );

        if let Err(err) = exec_result.await {
            log::warn!("Failed to list tests for {executable}: {err}");
        } else {
            for line in test_lines.lines() {
                if line.contains(&test_name) {
                    return Some(executable.clone());
                }
            }
        }
        let _ = child.kill();
    }
    None
}
#[async_trait]
impl DapLocator for CargoLocator {
    fn name(&self) -> SharedString {
        SharedString::new_static("rust-cargo-locator")
    }
    async fn create_scenario(
        &self,
        build_config: &TaskTemplate,
        resolved_label: &str,
        adapter: &DebugAdapterName,
    ) -> Option<DebugScenario> {
        if build_config.command != "cargo" {
            return None;
        }
        let mut task_template = build_config.clone();
        let cargo_action = task_template.args.first_mut()?;
        if cargo_action == "check" || cargo_action == "clean" {
            return None;
        }

        match cargo_action.as_ref() {
            "run" | "r" => {
                *cargo_action = "build".to_owned();
            }
            "test" | "t" | "bench" => {
                let delimiter = task_template
                    .args
                    .iter()
                    .position(|arg| arg == "--")
                    .unwrap_or(task_template.args.len());
                if !task_template.args[..delimiter]
                    .iter()
                    .any(|arg| arg == "--no-run")
                {
                    task_template.args.insert(delimiter, "--no-run".to_owned());
                }
            }
            _ => {}
        }

        let config = if adapter.as_ref() == "CodeLLDB" {
            json!({
                "sourceLanguages": ["rust"]
            })
        } else {
            Value::Null
        };
        Some(DebugScenario {
            adapter: adapter.0.clone(),
            label: resolved_label.to_string().into(),
            build: Some(BuildTaskDefinition::Template {
                task_template,
                locator_name: Some(self.name()),
            }),
            config,
            tcp_connection: None,
        })
    }

    async fn run(&self, build_config: SpawnInTerminal) -> Result<DebugRequest> {
        let cwd = build_config
            .cwd
            .clone()
            .context("Couldn't get cwd from debug config which is needed for locators")?;
        let builder = ShellBuilder::new(true, &build_config.shell).non_interactive();
        let (program, args) = builder.build(
            Some("cargo".into()),
            &build_config
                .args
                .iter()
                .cloned()
                .take_while(|arg| arg != "--")
                .chain(Some("--message-format=json".to_owned()))
                .collect(),
        );
        let mut child = util::command::new_smol_command(program)
            .args(args)
            .envs(build_config.env.iter().map(|(k, v)| (k.clone(), v.clone())))
            .current_dir(cwd)
            .stdout(Stdio::piped())
            .spawn()?;

        let mut output = String::new();
        if let Some(mut stdout) = child.stdout.take() {
            stdout.read_to_string(&mut output).await?;
        }

        let status = child.status().await?;
        anyhow::ensure!(status.success(), "Cargo command failed");

        let is_test = build_config
            .args
            .first()
            .is_some_and(|arg| arg == "test" || arg == "t");

        let executables = output
            .lines()
            .filter(|line| !line.trim().is_empty())
            .filter_map(|line| serde_json::from_str(line).ok())
            .filter(|json: &Value| {
                let is_test_binary = json
                    .get("profile")
                    .and_then(|profile| profile.get("test"))
                    .and_then(Value::as_bool)
                    .unwrap_or(false);

                if is_test {
                    is_test_binary
                } else {
                    !is_test_binary
                }
            })
            .filter_map(|json: Value| {
                json.get("executable")
                    .and_then(Value::as_str)
                    .map(String::from)
            })
            .collect::<Vec<_>>();
        anyhow::ensure!(
            !executables.is_empty(),
            "Couldn't get executable in cargo locator"
        );

        let mut test_name = None;
        if is_test {
            test_name = build_config
                .args
                .iter()
                .rev()
                .take_while(|name| "--" != name.as_str())
                .find(|name| !name.starts_with("-"))
                .cloned();
        }
        let executable = {
            if let Some(name) = test_name.as_ref().and_then(|name| {
                name.strip_prefix('$')
                    .map(|name| build_config.env.get(name))
                    .unwrap_or(Some(name))
            }) {
                find_best_executable(&executables, name).await
            } else {
                None
            }
        };

        let Some(executable) = executable.or_else(|| executables.first().cloned()) else {
            anyhow::bail!("Couldn't get executable in cargo locator");
        };

        let mut args: Vec<_> = test_name.into_iter().collect();
        if is_test {
            args.push("--nocapture".to_owned());
        }

        Ok(DebugRequest::Launch(task::LaunchRequest {
            program: executable,
            cwd: build_config.cwd,
            args,
            env: build_config.env.into_iter().collect(),
        }))
    }
}
