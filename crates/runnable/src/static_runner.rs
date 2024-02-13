//! This module is responsible for executing static runnables, that is runnables defined by the user
//! in the config file.
use std::{path::PathBuf, sync::Arc};

use crate::{PendingOutput, Runnable, RunnableHandle};
use anyhow::Context;
use async_process::{Command, Stdio};
use futures::FutureExt;
use gpui::AsyncAppContext;

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

    fn exec(
        &self,
        cwd: Option<PathBuf>,
        mut cx: AsyncAppContext,
    ) -> anyhow::Result<RunnableHandle> {
        let mut command = Command::new(self.runnable.command.clone());
        let mut command = command.args(self.runnable.args.clone());
        if let Some(env_path) = std::env::var_os("PATH") {
            command = command.env("PATH", env_path);
        }
        if let Some(cwd) = cwd {
            command = command.current_dir(cwd);
        }
        command = command
            .stderr(Stdio::piped())
            .stdout(Stdio::piped())
            .kill_on_drop(true);
        let mut command_handle = command
            .spawn()
            .with_context(|| format!("Failed to spawn command `{command:?}`"))?;

        let output = Some(PendingOutput::new(
            command_handle
                .stdout
                .take()
                .expect("stdout should be present due to `Stdio::piped` usage above"),
            command_handle
                .stderr
                .take()
                .expect("stdout should be present due to `Stdio::piped` usage above"),
            &mut cx,
        ));

        RunnableHandle::new(
            command_handle
                .status()
                .map(|task_result| {
                    task_result
                        .context("waiting for task to finish")
                        .map_err(Arc::new)
                })
                .boxed(),
            output,
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
        let runner = StaticRunner::new(Definition {
            command: "echo".into(),
            args: vec!["-n".into(), "Hello!".into()],
            ..definition_fill_in()
        });
        let ex = cx.executor().clone();
        ex.spawn(async_process::driver()).detach();
        let task_handle = cx.update(|cx| runner.exec(None, cx.to_async())).unwrap();
        let runnable_result = task_handle.await.unwrap();
        assert!(runnable_result.status.unwrap().success());
        assert_eq!(
            cx.update(|cx| runnable_result.output.unwrap().full_output(cx))
                .await,
            "Hello!"
        );
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
        let task_handle = cx.update(|cx| runner.exec(None, cx.to_async())).unwrap();
        let cancel_token = task_handle.termination_handle();
        std::thread::spawn(move || {
            std::thread::sleep(Duration::from_secs(3));
            cancel_token.abort();
        });
        let runnable_result = task_handle.await;
        assert!(runnable_result.is_err());
    }
}
