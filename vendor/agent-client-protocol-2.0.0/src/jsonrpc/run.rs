//! Run trait for background tasks that run alongside a connection.
//!
//! Run implementations are composable background tasks that run while a connection is active.
//! They're used for things like MCP tool handlers that need to receive calls through
//! channels and invoke user-provided closures.

use std::future::Future;

use crate::{ConnectionTo, role::Role};

/// A background task that runs alongside a connection.
///
/// `RunIn<R>` means "run in the context of being role R". The task receives
/// a `ConnectionTo<R::Counterpart>` for communicating with the other side.
///
/// Implementations are composed using [`ChainRun`] and run in parallel
/// when the connection is active.
pub trait RunWithConnectionTo<Counterpart: Role>: Send {
    /// Run this task to completion.
    fn run_with_connection_to(
        self,
        cx: ConnectionTo<Counterpart>,
    ) -> impl Future<Output = Result<(), crate::Error>> + Send;
}

/// A no-op RunIn that completes immediately.
#[derive(Debug, Default)]
pub struct NullRun;

impl<Counterpart: Role> RunWithConnectionTo<Counterpart> for NullRun {
    async fn run_with_connection_to(
        self,
        _cx: ConnectionTo<Counterpart>,
    ) -> Result<(), crate::Error> {
        Ok(())
    }
}

/// Chains two RunIn implementations to run in parallel.
#[derive(Debug)]
pub struct ChainRun<A, B> {
    a: A,
    b: B,
}

impl<A, B> ChainRun<A, B> {
    /// Create a new chained RunIn from two RunIn implementations.
    pub fn new(a: A, b: B) -> Self {
        Self { a, b }
    }
}

impl<Counterpart: Role, A, B> RunWithConnectionTo<Counterpart> for ChainRun<A, B>
where
    A: RunWithConnectionTo<Counterpart>,
    B: RunWithConnectionTo<Counterpart>,
{
    async fn run_with_connection_to(
        self,
        cx: ConnectionTo<Counterpart>,
    ) -> Result<(), crate::Error> {
        // Box the futures to avoid stack overflow with deeply nested RunIn chains
        let a_fut = Box::pin(self.a.run_with_connection_to(cx.clone()));
        let b_fut = Box::pin(self.b.run_with_connection_to(cx.clone()));
        let ((), ()) = futures::future::try_join(a_fut, b_fut).await?;
        Ok(())
    }
}

/// A RunIn created from a closure via [`with_spawned`](crate::Builder::with_spawned).
pub struct SpawnedRun<F> {
    task_fn: F,
    location: &'static std::panic::Location<'static>,
}

impl<F> SpawnedRun<F> {
    /// Create a new spawned RunIn from a closure.
    pub fn new(location: &'static std::panic::Location<'static>, task_fn: F) -> Self {
        Self { task_fn, location }
    }
}

impl<Counterpart, F, Fut> RunWithConnectionTo<Counterpart> for SpawnedRun<F>
where
    Counterpart: Role,
    F: FnOnce(ConnectionTo<Counterpart>) -> Fut + Send,
    Fut: Future<Output = Result<(), crate::Error>> + Send,
{
    async fn run_with_connection_to(
        self,
        connection: ConnectionTo<Counterpart>,
    ) -> Result<(), crate::Error> {
        let location = self.location;
        (self.task_fn)(connection).await.map_err(|err| {
            let data = err.data.clone();
            err.data(serde_json::json! {
                {
                    "spawned_at": format!("{}:{}:{}", location.file(), location.line(), location.column()),
                    "data": data,
                }
            })
        })
    }
}
