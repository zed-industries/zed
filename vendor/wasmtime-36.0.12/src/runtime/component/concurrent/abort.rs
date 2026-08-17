use std::future;
use std::pin::pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};

/// Handle to a task which may be used to abort it.
///
/// This represents a handle to a running task which can be cancelled with
/// [`AbortHandle::abort`].
pub struct AbortHandle {
    state: Arc<Mutex<AbortState>>,
}

#[derive(Default)]
struct AbortState {
    aborted: bool,
    waker: Option<Waker>,
}

impl AbortHandle {
    /// Abort the task.
    ///
    /// This flags the connected task should abort in the near future, but note
    /// that if this is called while the future is being polled then that call
    /// will still complete.
    pub fn abort(&self) {
        let waker = {
            let mut state = self.state.lock().unwrap();
            state.aborted = true;
            state.waker.take()
        };
        if let Some(waker) = waker {
            waker.wake();
        }
    }

    fn is_aborted(&self, cx: &mut Context<'_>) -> bool {
        let mut state = self.state.lock().unwrap();
        if state.aborted {
            return true;
        }
        state.waker = Some(cx.waker().clone());
        false
    }

    /// Wraps the `future` provided in a new future which is "abortable" where
    /// if the returned `AbortHandle` is flagged then the future will resolve
    /// ASAP with `None` and drop the provided `future`.
    pub(crate) fn run<F>(future: F) -> (AbortHandle, impl Future<Output = Option<F::Output>>)
    where
        F: Future,
    {
        let handle = AbortHandle {
            state: Default::default(),
        };
        let handle2 = AbortHandle {
            state: handle.state.clone(),
        };
        let future = async move {
            let mut future = pin!(future);
            future::poll_fn(|cx| {
                if handle2.is_aborted(cx) {
                    return Poll::Ready(None);
                }
                future.as_mut().poll(cx).map(Some)
            })
            .await
        };
        (handle, future)
    }
}
