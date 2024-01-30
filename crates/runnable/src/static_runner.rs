//! This module is responsible for executing static runnables, that is runnables defined by the user
//! in the config file.
use crate::{ExecutionResult, Runnable, TaskHandle};
use async_process::Command;
use futures::FutureExt;

#[derive(Clone, Debug, PartialEq)]
pub struct StaticRunner {
    runnable: super::static_runnable::Definition,
}

impl Runnable for StaticRunner {
    fn name(&self) -> String {
        self.runnable.label.clone()
    }

    fn exec(self, _: &mut gpui::AppContext) -> crate::TaskHandle {
        TaskHandle::new(
            Command::new("echo")
                .arg("Hello world!")
                .output()
                .map(|fut| ExecutionResult {
                    status: Result::Ok(()),
                    details: Default::default(),
                })
                .boxed(),
        )
    }

    fn boxed_clone(&self) -> Box<dyn Runnable> {
        Box::new(self.clone())
    }
}
