//! Handling for standard in using a worker task.
//!
//! Standard input is a global singleton resource for the entire program which
//! needs special care. Currently this implementation adheres to a few
//! constraints which make this nontrivial to implement.
//!
//! * Any number of guest wasm programs can read stdin. While this doesn't make
//!   a ton of sense semantically they shouldn't block forever. Instead it's a
//!   race to see who actually reads which parts of stdin.
//!
//! * Data from stdin isn't actually read unless requested. This is done to try
//!   to be a good neighbor to others running in the process. Under the
//!   assumption that most programs have one "thing" which reads stdin the
//!   actual consumption of bytes is delayed until the wasm guest is dynamically
//!   chosen to be that "thing". Before that data from stdin is not consumed to
//!   avoid taking it from other components in the process.
//!
//! * Tokio's documentation indicates that "interactive stdin" is best done with
//!   a helper thread to avoid blocking shutdown of the event loop. That's
//!   respected here where all stdin reading happens on a blocking helper thread
//!   that, at this time, is never shut down.
//!
//! This module is one that's likely to change over time though as new systems
//! are encountered along with preexisting bugs.

use crate::cli::{IsTerminal, StdinStream};
use bytes::{Bytes, BytesMut};
use std::io::Read;
use std::mem;
use std::pin::Pin;
use std::sync::{Condvar, Mutex, OnceLock};
use std::task::{Context, Poll};
use tokio::io::{self, AsyncRead, ReadBuf};
use tokio::sync::Notify;
use tokio::sync::futures::Notified;
use wasmtime_wasi_io::{
    poll::Pollable,
    streams::{InputStream, StreamError},
};

// Implementation for tokio::io::Stdin
impl IsTerminal for tokio::io::Stdin {
    fn is_terminal(&self) -> bool {
        std::io::stdin().is_terminal()
    }
}
impl StdinStream for tokio::io::Stdin {
    fn p2_stream(&self) -> Box<dyn InputStream> {
        Box::new(WasiStdin)
    }
    fn async_stream(&self) -> Box<dyn AsyncRead + Send + Sync> {
        Box::new(WasiStdinAsyncRead::Ready)
    }
}

// Implementation for std::io::Stdin
impl IsTerminal for std::io::Stdin {
    fn is_terminal(&self) -> bool {
        std::io::IsTerminal::is_terminal(self)
    }
}
impl StdinStream for std::io::Stdin {
    fn p2_stream(&self) -> Box<dyn InputStream> {
        Box::new(WasiStdin)
    }
    fn async_stream(&self) -> Box<dyn AsyncRead + Send + Sync> {
        Box::new(WasiStdinAsyncRead::Ready)
    }
}

#[derive(Default)]
struct GlobalStdin {
    state: Mutex<StdinState>,
    read_requested: Condvar,
    read_completed: Notify,
}

#[derive(Default, Debug)]
enum StdinState {
    #[default]
    ReadNotRequested,
    ReadRequested,
    Data(BytesMut),
    Error(std::io::Error),
    Closed,
}

impl GlobalStdin {
    fn get() -> &'static GlobalStdin {
        static STDIN: OnceLock<GlobalStdin> = OnceLock::new();
        STDIN.get_or_init(|| create())
    }
}

fn create() -> GlobalStdin {
    std::thread::spawn(|| {
        let state = GlobalStdin::get();
        loop {
            // Wait for a read to be requested, but don't hold the lock across
            // the blocking read.
            let mut lock = state.state.lock().unwrap();
            lock = state
                .read_requested
                .wait_while(lock, |state| !matches!(state, StdinState::ReadRequested))
                .unwrap();
            drop(lock);

            let mut bytes = BytesMut::zeroed(1024);
            let (new_state, done) = match std::io::stdin().read(&mut bytes) {
                Ok(0) => (StdinState::Closed, true),
                Ok(nbytes) => {
                    bytes.truncate(nbytes);
                    (StdinState::Data(bytes), false)
                }
                Err(e) => (StdinState::Error(e), true),
            };

            // After the blocking read completes the state should not have been
            // tampered with.
            debug_assert!(matches!(
                *state.state.lock().unwrap(),
                StdinState::ReadRequested
            ));
            *state.state.lock().unwrap() = new_state;
            state.read_completed.notify_waiters();
            if done {
                break;
            }
        }
    });

    GlobalStdin::default()
}

struct WasiStdin;

#[async_trait::async_trait]
impl InputStream for WasiStdin {
    fn read(&mut self, size: usize) -> Result<Bytes, StreamError> {
        let g = GlobalStdin::get();
        let mut locked = g.state.lock().unwrap();
        match mem::replace(&mut *locked, StdinState::ReadRequested) {
            StdinState::ReadNotRequested => {
                g.read_requested.notify_one();
                Ok(Bytes::new())
            }
            StdinState::ReadRequested => Ok(Bytes::new()),
            StdinState::Data(mut data) => {
                let size = data.len().min(size);
                let bytes = data.split_to(size);
                *locked = if data.is_empty() {
                    StdinState::ReadNotRequested
                } else {
                    StdinState::Data(data)
                };
                Ok(bytes.freeze())
            }
            StdinState::Error(e) => {
                *locked = StdinState::Closed;
                Err(StreamError::LastOperationFailed(e.into()))
            }
            StdinState::Closed => {
                *locked = StdinState::Closed;
                Err(StreamError::Closed)
            }
        }
    }
}

#[async_trait::async_trait]
impl Pollable for WasiStdin {
    async fn ready(&mut self) {
        let g = GlobalStdin::get();

        // Scope the synchronous `state.lock()` to this block which does not
        // `.await` inside of it.
        let notified = {
            let mut locked = g.state.lock().unwrap();
            match *locked {
                // If a read isn't requested yet
                StdinState::ReadNotRequested => {
                    g.read_requested.notify_one();
                    *locked = StdinState::ReadRequested;
                    g.read_completed.notified()
                }
                StdinState::ReadRequested => g.read_completed.notified(),
                StdinState::Data(_) | StdinState::Closed | StdinState::Error(_) => return,
            }
        };

        notified.await;
    }
}

enum WasiStdinAsyncRead {
    Ready,
    Waiting(Notified<'static>),
}

impl AsyncRead for WasiStdinAsyncRead {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        let g = GlobalStdin::get();

        // Perform everything below in a `loop` to handle the case that a read
        // was stolen by another thread, for example, or perhaps a spurious
        // notification to `Notified`.
        loop {
            // If we were previously blocked on reading a "ready" notification,
            // wait for that notification to complete.
            if let Some(notified) = self.as_mut().notified_future() {
                match notified.poll(cx) {
                    Poll::Ready(()) => self.set(WasiStdinAsyncRead::Ready),
                    Poll::Pending => break Poll::Pending,
                }
            }

            assert!(matches!(*self, WasiStdinAsyncRead::Ready));

            // Once we're in the "ready" state then take a look at the global
            // state of stdin.
            let mut locked = g.state.lock().unwrap();
            match mem::replace(&mut *locked, StdinState::ReadRequested) {
                // If data is available then drain what we can into `buf`.
                StdinState::Data(mut data) => {
                    let size = data.len().min(buf.remaining());
                    let bytes = data.split_to(size);
                    *locked = if data.is_empty() {
                        StdinState::ReadNotRequested
                    } else {
                        StdinState::Data(data)
                    };
                    buf.put_slice(&bytes);
                    break Poll::Ready(Ok(()));
                }

                // If stdin failed to be read then we fail with that error and
                // transition to "closed"
                StdinState::Error(e) => {
                    *locked = StdinState::Closed;
                    break Poll::Ready(Err(e));
                }

                // If stdin is closed, keep it closed.
                StdinState::Closed => {
                    *locked = StdinState::Closed;
                    break Poll::Ready(Ok(()));
                }

                // For these states we indicate that a read is requested, if it
                // wasn't previously requested, and then we transition to
                // `Waiting` below by falling through outside this `match`.
                StdinState::ReadNotRequested => {
                    g.read_requested.notify_one();
                }
                StdinState::ReadRequested => {}
            }

            self.set(WasiStdinAsyncRead::Waiting(g.read_completed.notified()));

            // Intentionally drop the lock after the `notified()` future
            // creation just above as to work correctly this needs to happen
            // within the lock.
            drop(locked);
        }
    }
}

impl WasiStdinAsyncRead {
    fn notified_future(self: Pin<&mut Self>) -> Option<Pin<&mut Notified<'static>>> {
        // SAFETY: this is a pin-projection from `self` to the field `Notified`
        // internally. Given that `self` is pinned it should be safe to acquire
        // a pinned version of the internal field.
        unsafe {
            match self.get_unchecked_mut() {
                WasiStdinAsyncRead::Ready => None,
                WasiStdinAsyncRead::Waiting(notified) => Some(Pin::new_unchecked(notified)),
            }
        }
    }
}
