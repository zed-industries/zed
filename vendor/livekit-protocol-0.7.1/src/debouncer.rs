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

use std::time::Duration;

use futures_util::Future;
use thiserror::Error;
use tokio::sync::{mpsc, oneshot};

#[derive(Debug, Error)]
pub enum DebounceError {
    #[error("function already executed")]
    AlreadyExecuted,
}

pub struct Debouncer {
    cancel_tx: Option<oneshot::Sender<()>>,
    tx: mpsc::UnboundedSender<()>,
}

pub fn debounce<F>(duration: Duration, future: F) -> Debouncer
where
    F: Future + Send + 'static,
{
    let (tx, rx) = mpsc::unbounded_channel();
    let (cancel_tx, cancel_rx) = oneshot::channel();
    livekit_runtime::spawn(debounce_task(duration, future, rx, cancel_rx));
    Debouncer { tx, cancel_tx: Some(cancel_tx) }
}

async fn debounce_task<F>(
    duration: Duration,
    future: F,
    mut rx: mpsc::UnboundedReceiver<()>,
    mut cancel_rx: oneshot::Receiver<()>,
) where
    F: Future + Send + 'static,
{
    loop {
        tokio::select! {
            _ = &mut cancel_rx => break,
            _ = rx.recv() => continue,
            _ = livekit_runtime::sleep(duration) => {
                future.await;
                break;
            }
        }
    }
}

impl Debouncer {
    pub fn call(&self) -> Result<(), mpsc::error::SendError<()>> {
        self.tx.send(())
    }
}

impl Drop for Debouncer {
    fn drop(&mut self) {
        let _ = self.cancel_tx.take().unwrap().send(());
    }
}
