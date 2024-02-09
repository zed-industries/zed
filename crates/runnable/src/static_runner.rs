//! This module is responsible for executing static runnables, that is runnables defined by the user
//! in the config file.
use std::{error::Error, sync::Arc};

use crate::{ExecutionResult, Runnable, TaskHandle};
use async_process::Command;
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
        TaskHandle::new(
            Command::new(self.runnable.command.clone())
                .args(self.runnable.args.clone())
                .output()
                .map(|output| {
                    let (status, details): (Result<_, Arc<dyn Error>>, _) = match output {
                        Ok(output) => {
                            let details = String::from_utf8_lossy(&output.stdout).into_owned();
                            (Ok(()), details)
                        }
                        e @ Err(_) => (
                            e.map(|_| ()).map_err(|e| Arc::new(e) as Arc<dyn Error>),
                            "".to_owned(),
                        ),
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
        let mut runner = StaticRunner::new(Definition {
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
