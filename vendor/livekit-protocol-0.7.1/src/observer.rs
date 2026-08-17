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

use std::{pin::Pin, sync::Arc};

use futures_util::{
    sink::Sink,
    task::{Context, Poll},
};
use parking_lot::Mutex;
use tokio::sync::mpsc;

#[derive(Clone, Debug)]
pub struct Dispatcher<T>
where
    T: Clone,
{
    senders: Arc<Mutex<Vec<mpsc::UnboundedSender<T>>>>,
}

impl<T> Default for Dispatcher<T>
where
    T: Clone,
{
    fn default() -> Self {
        Self { senders: Default::default() }
    }
}

impl<T> Dispatcher<T>
where
    T: Clone,
{
    pub fn register(&self) -> mpsc::UnboundedReceiver<T> {
        let (tx, rx) = mpsc::unbounded_channel();
        self.senders.lock().push(tx);
        rx
    }

    pub fn dispatch(&self, msg: &T) {
        self.senders.lock().retain(|sender| sender.send(msg.clone()).is_ok());
    }

    pub fn clear(&self) {
        self.senders.lock().clear();
    }
}

impl<T> Sink<T> for Dispatcher<T>
where
    T: Clone,
{
    type Error = ();

    fn poll_ready(self: Pin<&mut Self>, _: &mut Context) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn start_send(self: Pin<&mut Self>, item: T) -> Result<(), Self::Error> {
        self.dispatch(&item);
        Ok(())
    }

    fn poll_flush(self: Pin<&mut Self>, _: &mut Context) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn poll_close(self: Pin<&mut Self>, _: &mut Context) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }
}
