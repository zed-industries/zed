use anyhow::{Context as _, Result};
use async_process::{ChildStderr, ChildStdout, ExitStatus};
use futures::future::{BoxFuture, Shared};
pub use futures::stream::Aborted as RunnableTerminated;
use futures::stream::{AbortHandle, Abortable};
use futures::{AsyncBufReadExt, AsyncRead, Future, FutureExt};
use gpui::{AppContext, AsyncAppContext, Context as _, EventEmitter, Model, Task, WeakModel};
use smol::io::BufReader;
use std::sync::Arc;
use std::task::Poll;
use util::ResultExt;

use crate::ExecutionResult;

/// Represents a runnable that's already underway. That runnable can be cancelled at any time.
#[derive(Clone)]
pub struct Handle {
    pub(crate) fut:
        Shared<Task<Result<Result<ExitStatus, Arc<anyhow::Error>>, RunnableTerminated>>>,
    pub output: Option<Model<PendingOutput>>,
    cancel_token: AbortHandle,
}

pub struct NewLineAvailable {
    inner: Box<str>,
}

impl AsRef<str> for NewLineAvailable {
    fn as_ref(&self) -> &str {
        &self.inner
    }
}

#[derive(Clone, Debug)]
pub struct PendingOutput {
    _output_read_tasks: [Shared<Task<()>>; 2],
}

impl EventEmitter<NewLineAvailable> for PendingOutput {}
impl PendingOutput {
    pub(super) fn new(
        stdout: ChildStdout,
        stderr: ChildStderr,
        cx: &mut AppContext,
    ) -> Model<Self> {
        cx.new_model(|cx| {
            let stdout_task = cx
                .spawn(|this, cx| async move {
                    handle_output(stdout, this, cx)
                        .await
                        .context("stdout capture")
                        .log_err();
                })
                .shared();

            let stderr_task = cx
                .spawn(|this, cx| async move {
                    handle_output(stderr, this, cx)
                        .await
                        .context("stderr capture")
                        .log_err();
                })
                .shared();

            Self {
                _output_read_tasks: [stdout_task, stderr_task],
            }
        })
    }
}

impl Handle {
    pub fn new(
        fut: BoxFuture<'static, Result<ExitStatus, Arc<anyhow::Error>>>,
        output: Option<Model<PendingOutput>>,
        cx: AsyncAppContext,
    ) -> Result<Self> {
        let (cancel_token, abort_registration) = AbortHandle::new_pair();
        let fut = cx
            .spawn(move |_| Abortable::new(fut, abort_registration))
            .shared();
        Ok(Self {
            fut,
            output,
            cancel_token,
        })
    }

    /// Returns a handle that can be used to cancel this runnable.
    pub fn termination_handle(&self) -> AbortHandle {
        self.cancel_token.clone()
    }

    pub fn result<'a>(&self) -> Option<Result<ExecutionResult, RunnableTerminated>> {
        self.fut.peek().cloned().map(|res| {
            res.map(|runnable_result| ExecutionResult {
                status: runnable_result,
                output: self.output.clone(),
            })
        })
    }
}

impl Future for Handle {
    type Output = Result<ExecutionResult, RunnableTerminated>;

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Self::Output> {
        match self.fut.poll_unpin(cx) {
            Poll::Ready(res) => match res {
                Ok(runnable_result) => Poll::Ready(Ok(ExecutionResult {
                    status: runnable_result,
                    output: self.output.clone(),
                })),
                Err(aborted) => Poll::Ready(Err(aborted)),
            },
            Poll::Pending => Poll::Pending,
        }
    }
}

async fn handle_output<Output>(
    output: Output,
    pending_output: WeakModel<PendingOutput>,
    mut cx: AsyncAppContext,
) -> anyhow::Result<()>
where
    Output: AsyncRead + Unpin + Send + 'static,
{
    let mut output = BufReader::new(output);
    let mut buffer = Vec::new();

    loop {
        buffer.clear();

        let bytes_read = output
            .read_until(b'\n', &mut buffer)
            .await
            .context("reading output newline")?;
        if bytes_read == 0 {
            return Ok(());
        }

        let inner: Box<_> = String::from_utf8_lossy(&buffer).into_owned().into();
        let Some(()) = pending_output
            .update(&mut cx, |_, cx| {
                cx.emit(NewLineAvailable { inner });
            })
            .log_err()
        else {
            return Ok(());
        };

        // Don't starve the main thread when receiving lots of messages at once.
        smol::future::yield_now().await;
    }
}
