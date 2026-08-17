// TODO: Switch to interior mutability (e.g. use Mutexes or thread-local
// RefCells) and remove this, since even in single-threaded mode `static mut`
// references can be a hazard due to recursive access.
#![allow(static_mut_refs)]

use crate::rt::async_support::BoxFuture;
use futures::stream::{FuturesUnordered, StreamExt};
use std::boxed::Box;
use std::future::Future;
use std::task::{Context, Poll};
use std::vec::Vec;

/// Any newly-deferred work queued by calls to the `spawn` function while
/// polling the current task.
static mut SPAWNED: Vec<BoxFuture> = Vec::new();

#[derive(Default)]
pub struct Tasks<'a> {
    tasks: FuturesUnordered<BoxFuture<'a>>,
}

impl<'a> Tasks<'a> {
    pub fn new(root: BoxFuture<'a>) -> Tasks<'a> {
        Tasks {
            tasks: [root].into_iter().collect(),
        }
    }

    pub fn poll_next(&mut self, cx: &mut Context<'_>) -> Poll<Option<()>> {
        unsafe {
            let ret = self.tasks.poll_next_unpin(cx);
            if !SPAWNED.is_empty() {
                self.tasks.extend(SPAWNED.drain(..));
            }
            ret
        }
    }

    pub fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }
}

/// Spawn the provided `future` to get executed concurrently with the
/// currently-running async computation.
///
/// This API is somewhat similar to `tokio::task::spawn` for example but has a
/// number of limitations to be aware of. If possible it's recommended to avoid
/// this, but it can be convenient if these limitations do not apply to you:
///
/// * Spawned tasks do not work when the version of the `wit-bindgen` crate
///   managing the export bindings is different from the version of this crate.
///   To work correctly the `spawn` function and export executor must be at
///   exactly the same version. Given the major-version-breaking nature of
///   `wit-bindgen` this is not always easy to rely on. This is tracked in
///   [#1305].
///
/// * Spawned tasks do not outlive the scope of the async computation they are
///   spawned within. For example with an async export function spawned tasks
///   will be polled within the context of that component-model async task. For
///   computations executing within a [`block_on`] call, however, the spawned
///   tasks will be executed within that scope. This notably means that for
///   [`block_on`] spawned tasks will prevent the [`block_on`] function from
///   returning, even if a value is available to return.
///
/// * There is no handle returned to the spawned task meaning that it cannot be
///   cancelled or monitored.
///
/// * The task spawned here is executed *concurrently*, not in *parallel*. This
///   means that while one future is being polled no other future can be polled
///   at the same time. This is similar to a single-thread executor in Tokio.
///
/// With these restrictions in mind this can be used to express
/// execution-after-returning in the component model. For example once an
/// exported async function has produced a value this can be used to continue to
/// execute some more code before the component model async task exits.
///
/// [`block_on`]: crate::block_on
/// [#1305]: https://github.com/bytecodealliance/wit-bindgen/issues/1305
pub fn spawn(future: impl Future<Output = ()> + 'static) {
    unsafe { SPAWNED.push(Box::pin(future)) }
}
