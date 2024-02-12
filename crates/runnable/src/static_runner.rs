//! This module is responsible for executing static runnables, that is runnables defined by the user
//! in the config file.
use std::{error::Error, sync::Arc};

use crate::{ExecutionResult, Runnable, TaskHandle};
use anyhow::Context;
use async_process::{Command, Stdio};
use futures::FutureExt;

#[derive(Clone, Debug, PartialEq)]
pub struct StaticRunner {
    runnable: super::static_runnable_file::Definition,
}

impl StaticRunner {
    pub fn new(runnable: super::static_runnable_file::Definition) -> Self {
        Self { runnable }
    }
}

impl Runnable for StaticRunner {
    fn boxed_clone(&self) -> Box<dyn Runnable> {
        Box::new(self.clone())
    }

    fn exec(&self, cx: gpui::AsyncAppContext) -> anyhow::Result<crate::TaskHandle> {
        let mut command = Command::new(self.runnable.command.clone());
        let mut command = command.args(self.runnable.args.clone());
        if let Some(env_path) = std::env::var_os("PATH") {
            command = command.env("PATH", env_path);
        }
        command = command
            .stderr(Stdio::piped())
            .stdout(Stdio::piped())
            .kill_on_drop(true);
        let command_handle = command
            .spawn()
            .with_context(|| format!("Failed to spawn command `{command:?}`"))?;

        TaskHandle::new(
            command_handle
                .output()
                .map(|output| {
                    let (status, details): (Result<_, Arc<dyn Error>>, _) = match output {
                        Ok(output) => {
                            let details = String::from_utf8_lossy(&output.stdout).into_owned();
                            let sterr_details =
                                String::from_utf8_lossy(&output.stderr).into_owned();
                            // TODO kb remove this and send the handle into new terminal tab to print its output
                            dbg!(output.status, sterr_details, &details);
                            (Ok(()), details)
                        }
                        Err(e) => (Err(Arc::new(e) as Arc<dyn Error>), String::new()),
                    };

                    ExecutionResult { status, details }
                })
                .boxed(),
            cx.clone(),
        )
    }

    fn name(&self) -> String {
        self.runnable.label.clone()
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::{static_runnable_file::Definition, Runnable};
    use gpui::TestAppContext;

    use crate::StaticRunner;

    fn definition_fill_in() -> Definition {
        Definition {
            command: Default::default(),
            args: vec![],
            label: Default::default(),
            options: Default::default(),
            presentation: Default::default(),
        }
    }
    #[gpui::test]
    async fn test_echo(cx: &mut TestAppContext) {
        cx.executor().allow_parking();
        let mut runner = StaticRunner::new(Definition {
            command: "echo".into(),
            args: vec!["-n".into(), "Hello!".into()],
            ..definition_fill_in()
        });
        let ex = cx.executor().clone();
        ex.spawn(async_process::driver()).detach();
        let runnable_result = cx
            .update(|cx| runner.exec(cx.to_async()))
            .unwrap()
            .await
            .unwrap();

        assert!(runnable_result.status.is_ok());
        assert_eq!(runnable_result.details, "Hello!");
    }

    #[gpui::test]
    async fn test_cancel(cx: &mut TestAppContext) {
        cx.executor().allow_parking();
        let runner = StaticRunner::new(Definition {
            command: "sleep".into(),
            args: vec!["500".into()],
            ..definition_fill_in()
        });
        let ex = cx.executor().clone();
        ex.spawn(async_process::driver()).detach();
        let runnable = cx.update(|cx| runner.exec(cx.to_async())).unwrap();
        let cancel_token = runnable.termination_handle();
        std::thread::spawn(move || {
            std::thread::sleep(Duration::from_secs(3));
            cancel_token.abort();
        });
        let runnable_result = runnable.await;

        assert!(runnable_result.is_err());
    }
}
