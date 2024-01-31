//! This module is responsible for executing static runnables, that is runnables defined by the user
//! in the config file.
use crate::{ExecutionResult, Runnable, TaskHandle};
use async_process::Command;
use futures::FutureExt;

#[derive(Clone, Debug, PartialEq)]
pub struct StaticRunner {
    runnable: super::static_runnable::Definition,
}

impl StaticRunner {
    pub fn new(runnable: super::static_runnable::Definition) -> Self {
        Self { runnable }
    }
}
impl Runnable for StaticRunner {
    fn name(&self) -> String {
        self.runnable.label.clone()
    }

    fn exec(self, mut cx: gpui::AsyncWindowContext) -> anyhow::Result<crate::TaskHandle> {
        TaskHandle::new(
            Command::new(self.runnable.command)
                .args(self.runnable.args)
                .output()
                .map(|output| {
                    let (status, details) = match output {
                        Ok(output) => {
                            let details = String::from_utf8_lossy(&output.stdout).into_owned();
                            (Ok(()), details)
                        }
                        e @ Err(_) => (e.map(|_| ()).map_err(|e| e.into()), "".to_owned()),
                    };

                    ExecutionResult { status, details }
                })
                .boxed(),
            cx.clone(),
        )
    }

    fn boxed_clone(&self) -> Box<dyn Runnable> {
        Box::new(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::{static_runnable::Definition, Runnable};
    use async_process::Command;
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
        let cx = cx.add_empty_window();
        let runner = StaticRunner::new(Definition {
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
        let cx = cx.add_empty_window();
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
