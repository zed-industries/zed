// Copyright 2025 LiveKit, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use futures::{channel::mpsc::UnboundedReceiver, select_biased, Future, FutureExt, StreamExt};
use std::{sync::OnceLock, task::Poll, time::Duration};

pub use async_std::net::TcpStream;
pub use async_task::Runnable;
pub use futures::Stream;
pub use std::time::Instant;

/// This is semantically equivalent to Tokio's MissedTickBehavior:
/// https://docs.rs/tokio/1.36.0/tokio/time/enum.MissedTickBehavior.html
#[derive(Default, Copy, Clone)]
pub enum MissedTickBehavior {
    #[default]
    Burst,
    Delay,
    Skip,
}

static DISPATCHER: OnceLock<&'static dyn Dispatcher> = OnceLock::new();

pub trait Dispatcher: 'static + Send + Sync {
    fn dispatch(&self, runnable: Runnable);
    fn dispatch_after(&self, duration: Duration, runnable: Runnable);
}

pub fn set_dispatcher(dispatcher: impl Dispatcher) {
    let dispatcher = Box::leak(Box::new(dispatcher));
    DISPATCHER.set(dispatcher).ok();
}

fn get_dispatcher() -> &'static dyn Dispatcher {
    *DISPATCHER.get().expect("The livekit dispatcher requires a call to set_dispatcher()")
}

#[derive(Debug)]
pub struct JoinHandle<T> {
    task: Option<async_task::Task<T>>,
}

pub fn spawn<F>(future: F) -> JoinHandle<F::Output>
where
    F: Future + 'static + Send,
    F::Output: 'static + Send,
{
    let dispatcher = get_dispatcher();
    let (runnable, task) = async_task::spawn(future, |runnable| dispatcher.dispatch(runnable));
    runnable.schedule();
    JoinHandle { task: Some(task) }
}

impl<T> Future for JoinHandle<T> {
    type Output = T;

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Self::Output> {
        self.task.as_mut().expect("poll should not be called after drop").poll_unpin(cx)
    }
}

impl<T> Drop for JoinHandle<T> {
    fn drop(&mut self) {
        self.task.take().expect("This is the only place the option is mutated").detach();
    }
}

pub struct Sleep {
    task: async_task::Task<()>,
}

pub fn sleep(time: Duration) -> Sleep {
    let dispatcher = get_dispatcher();
    let (runnable, task) =
        async_task::spawn(async {}, move |runnable| dispatcher.dispatch_after(time, runnable));
    runnable.schedule();

    Sleep { task }
}

impl Sleep {
    pub fn reset(&mut self, deadline: Instant) {
        let duration = deadline.saturating_duration_since(Instant::now());
        self.task = sleep(duration).task
    }
}

impl Future for Sleep {
    type Output = ();

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Self::Output> {
        self.task.poll_unpin(cx)
    }
}

pub struct TimeoutError {}

pub fn timeout<T>(
    duration: Duration,
    future: T,
) -> impl Future<Output = Result<T::Output, TimeoutError>>
where
    T: Future,
{
    async move {
        select_biased! {
            res = future.fuse() => Ok(res),
            _ = sleep(duration).fuse() => Err(TimeoutError {}),
        }
    }
}

pub struct Interval {
    duration: Duration,
    _timer_loop: JoinHandle<()>,
    missed_tick_behavior: MissedTickBehavior,
    rx: UnboundedReceiver<Instant>,
}

pub fn interval(duration: Duration) -> Interval {
    let (tx, rx) = futures::channel::mpsc::unbounded();
    let timer_loop = spawn(async move {
        loop {
            sleep(duration).await;
            tx.unbounded_send(Instant::now()).ok();
        }
    });

    Interval {
        duration,
        rx,
        _timer_loop: timer_loop,
        missed_tick_behavior: MissedTickBehavior::default(),
    }
}

impl Interval {
    pub fn reset(&mut self) {
        let missed_tick_behavior = self.missed_tick_behavior;
        *self = interval(self.duration);
        self.set_missed_tick_behavior(missed_tick_behavior);
    }

    pub async fn tick(&mut self) -> Instant {
        self.rx.next().await.expect("timer loop should always be running")
    }

    pub fn set_missed_tick_behavior(&mut self, behavior: MissedTickBehavior) {
        self.missed_tick_behavior = behavior;
    }
}

impl Future for Interval {
    type Output = Instant;

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Self::Output> {
        self.rx.next().poll_unpin(cx).map(|option| {
            option.expect("join loop should be running for as long as the interval exists")
        })
    }
}
