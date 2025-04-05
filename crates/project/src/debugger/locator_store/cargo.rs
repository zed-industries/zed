use super::DapLocator;
use anyhow::{Result, anyhow};
use async_trait::async_trait;
use dap::DebugAdapterConfig;
use serde_json::{Value, json};
use smol::{
    io::AsyncReadExt,
    process::{Command, Stdio},
};
use util::maybe;

pub(super) struct CargoLocator;

#[async_trait]
impl DapLocator for CargoLocator {
    async fn run_locator(&self, debug_config: &mut DebugAdapterConfig) -> Result<()> {
        let Some(launch_config) = (match &mut debug_config.request {
            task::DebugRequestDisposition::UserConfigured(task::DebugRequestType::Launch(
                launch_config,
            )) => Some(launch_config),
            _ => None,
        }) else {
            return Err(anyhow!("Couldn't get launch config in locator"));
        };

        let Some(cwd) = launch_config.cwd.clone() else {
            return Err(anyhow!(
                "Couldn't get cwd from debug config which is needed for locators"
            ));
        };

        let mut child = Command::new("cargo")
            .args(&launch_config.args)
            .arg("--message-format=json")
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

        let Some(executable) = output
            .lines()
            .filter(|line| !line.trim().is_empty())
            .filter_map(|line| serde_json::from_str(line).ok())
            .find_map(|json: Value| {
                json.get("executable")
                    .and_then(Value::as_str)
                    .map(String::from)
            })
        else {
            return Err(anyhow!("Couldn't get executable in cargo locator"));
        };

        launch_config.program = executable;
        let mut test_name = None;

        if launch_config
            .args
            .first()
            .map_or(false, |arg| arg == "test")
        {
            if let Some(package_index) = launch_config
                .args
                .iter()
                .position(|arg| arg == "-p" || arg == "--package")
            {
                test_name = launch_config
                    .args
                    .get(package_index + 2)
                    .filter(|name| !name.starts_with("--"))
                    .cloned();
            }
        }

        if debug_config.adapter == "LLDB" && debug_config.initialize_args.is_none() {
            // Find Rust pretty-printers in current toolchain's sysroot
            let cwd = launch_config.cwd.clone();
            debug_config.initialize_args = maybe!(async move {
                let cwd = cwd?;

                let output = Command::new("rustc")
                    .arg("--print")
                    .arg("sysroot")
                    .current_dir(cwd)
                    .output()
                    .await
                    .ok()?;

                if !output.status.success() {
                    return None;
                }

                let sysroot_path = String::from_utf8(output.stdout).ok()?;
                let sysroot_path = sysroot_path.trim_end();
                let first_command = format!(
                    r#"command script import "{sysroot_path}/lib/rustlib/etc/lldb_lookup.py"#
                );
                let second_command =
                    format!(r#"command source -s 0 '{sysroot_path}/lib/rustlib/etc/lldb_commands"#);

                Some(json!({"initCommands": [first_command, second_command]}))
            })
            .await;
        }

        launch_config.args.clear();
        if let Some(test_name) = test_name {
            launch_config.args.push(test_name);
        }
        Ok(())
    }
}
