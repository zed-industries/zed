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

use futures::{Future, FutureExt, StreamExt};
use std::time::Duration;

pub use async_std::future::timeout;
pub use async_std::net::TcpStream;
pub use async_std::task::spawn;
pub use async_std::task::JoinHandle;
pub use futures::Stream;
pub use std::time::Instant;

/// This is semantically equivalent to Tokio's MissedTickBehavior:
/// https://docs.rs/tokio/1.36.0/tokio/time/enum.MissedTickBehavior.html
pub enum MissedTickBehavior {
    Burst,
    Delay,
    Skip,
}

pub struct Interval {
    duration: Duration,
    timer: async_io::Timer,
}

impl Interval {
    pub fn reset(&mut self) {
        self.timer.set_after(self.duration)
    }

    pub async fn tick(&mut self) -> Instant {
        self.timer.next().await.unwrap()
    }

    pub fn set_missed_tick_behavior(&mut self, _: MissedTickBehavior) {
        // noop, this runtime does not support this feature
    }
}

pub fn interval(duration: Duration) -> Interval {
    Interval { duration, timer: async_io::Timer::interval(duration) }
}

pub struct Sleep {
    timer: async_io::Timer,
}

impl Sleep {
    pub fn reset(&mut self, deadline: Instant) {
        self.timer.set_at(deadline)
    }
}

pub fn sleep(duration: Duration) -> Sleep {
    Sleep { timer: async_io::Timer::after(duration) }
}

impl Future for Sleep {
    type Output = ();

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        self.timer.poll_unpin(cx).map(|_| ())
    }
}
