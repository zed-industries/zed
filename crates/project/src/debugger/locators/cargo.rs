use anyhow::{Result, anyhow};
use async_trait::async_trait;
use dap::{DapLocator, DebugRequest, adapters::DebugAdapterName};
use gpui::SharedString;
use serde_json::Value;
use smol::{
    io::AsyncReadExt,
    process::{Command, Stdio},
};
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
        if let Some(mut stdout) = child.stdout.take() {
            stdout.read_to_string(&mut test_lines).await.ok();
            for line in test_lines.lines() {
                if line.contains(&test_name) {
                    return Some(executable.clone());
                }
            }
        }
    }
    None
}
#[async_trait]
impl DapLocator for CargoLocator {
    fn name(&self) -> SharedString {
        SharedString::new_static("rust-cargo-locator")
    }
    fn create_scenario(
        &self,
        build_config: &TaskTemplate,
        resolved_label: &str,
        adapter: DebugAdapterName,
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
            "run" => {
                *cargo_action = "build".to_owned();
            }
            "test" | "bench" => {
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
        Some(DebugScenario {
            adapter: adapter.0,
            label: resolved_label.to_string().into(),
            build: Some(BuildTaskDefinition::Template {
                task_template,
                locator_name: Some(self.name()),
            }),
            config: serde_json::Value::Null,
            tcp_connection: None,
            stop_on_entry: None,
        })
    }

    async fn run(&self, build_config: SpawnInTerminal) -> Result<DebugRequest> {
        let Some(cwd) = build_config.cwd.clone() else {
            return Err(anyhow!(
                "Couldn't get cwd from debug config which is needed for locators"
            ));
        };
        let builder = ShellBuilder::new(true, &build_config.shell).non_interactive();
        let (program, args) = builder.build(
            "cargo".into(),
            &build_config
                .args
                .iter()
                .cloned()
                .take_while(|arg| arg != "--")
                .chain(Some("--message-format=json".to_owned()))
                .collect(),
        );
        let mut child = Command::new(program)
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
        if !status.success() {
            return Err(anyhow::anyhow!("Cargo command failed"));
        }

        let executables = output
            .lines()
            .filter(|line| !line.trim().is_empty())
            .filter_map(|line| serde_json::from_str(line).ok())
            .filter_map(|json: Value| {
                json.get("executable")
                    .and_then(Value::as_str)
                    .map(String::from)
            })
            .collect::<Vec<_>>();
        if executables.is_empty() {
            return Err(anyhow!("Couldn't get executable in cargo locator"));
        };
        let is_test = build_config.args.first().map_or(false, |arg| arg == "test");

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
            if let Some(ref name) = test_name.as_ref().and_then(|name| {
                name.strip_prefix('$')
                    .map(|name| build_config.env.get(name))
                    .unwrap_or(Some(name))
            }) {
                find_best_executable(&executables, &name).await
            } else {
                None
            }
        };

        let Some(executable) = executable.or_else(|| executables.first().cloned()) else {
            return Err(anyhow!("Couldn't get executable in cargo locator"));
        };

        let args = test_name.into_iter().collect();

        Ok(DebugRequest::Launch(task::LaunchRequest {
            program: executable,
            cwd: build_config.cwd.clone(),
            args,
            env: build_config
                .env
                .iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect(),
        }))
    }
}
