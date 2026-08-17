use crate::async_rt;
use crate::error::{ZmqError, ZmqResult};

use futures::channel::oneshot;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TaskError {
    #[error("Internal task error: {0}")]
    Internal(#[from] Box<ZmqError>),
    #[error("Task panicked")]
    Panic,
    #[error("Task cancelled")]
    Cancelled,
}
impl From<async_rt::task::JoinError> for TaskError {
    fn from(err: async_rt::task::JoinError) -> Self {
        if err.is_panic() {
            TaskError::Panic
        } else {
            debug_assert!(err.is_cancelled());
            TaskError::Cancelled
        }
    }
}

pub struct TaskHandle<T> {
    // Using options to allow us to move resource without consuming `self`
    stop_channel: oneshot::Sender<()>,
    join_handle: async_rt::task::JoinHandle<ZmqResult<T>>,
}
impl<T> TaskHandle<T> {
    #[allow(unused)]
    pub(crate) fn new(
        stop_channel: oneshot::Sender<()>,
        join_handle: async_rt::task::JoinHandle<ZmqResult<T>>,
    ) -> Self {
        Self {
            stop_channel,
            join_handle,
        }
    }

    /// Shutdown the task and return the task's result, consuming the handle
    /// in the process
    #[allow(dead_code)]
    pub(crate) async fn shutdown(self) -> ZmqResult<T> {
        // Its ok to ignore possible error, because a dropped channel it has the same
        // effect as sending stop signal
        let _ = self.stop_channel.send(());
        let join_result = self.join_handle.await.map_err(TaskError::from);
        match join_result {
            Ok(Ok(ok)) => Ok(ok),
            Ok(Err(zmq_err)) => Err(zmq_err),
            Err(task_err) => Err(task_err.into()),
        }
    }
}
