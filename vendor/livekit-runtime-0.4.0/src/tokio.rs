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

use std::future::Future;
use std::pin::Pin;

pub use tokio::net::TcpStream;
pub use tokio::time::interval;
pub use tokio::time::sleep;
pub use tokio::time::timeout;
pub use tokio::time::Instant;
pub use tokio::time::MissedTickBehavior;
pub use tokio_stream::Stream;

pub type JoinHandle<T> = TokioJoinHandle<T>;
pub type Interval = tokio::time::Interval;

#[derive(Debug)]
pub struct TokioJoinHandle<T> {
    handle: tokio::task::JoinHandle<T>,
}

pub fn spawn<F>(future: F) -> JoinHandle<F::Output>
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    TokioJoinHandle { handle: tokio::task::spawn(future) }
}

impl<T> Future for TokioJoinHandle<T> {
    type Output = T;

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let this = &mut *self;
        let mut handle = &mut this.handle;
        match Pin::new(&mut handle).poll(cx) {
            std::task::Poll::Ready(value) => {
                std::task::Poll::Ready(value.expect("Tasks should not panic"))
            }
            std::task::Poll::Pending => std::task::Poll::Pending,
        }
    }
}
