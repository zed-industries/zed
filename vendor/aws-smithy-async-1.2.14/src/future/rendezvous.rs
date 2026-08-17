/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Rendezvous channel implementation
//!
//! Rendezvous channels are equivalent to a channel with a 0-sized buffer: A sender cannot send
//! until there is an active receiver waiting. This implementation uses a Semaphore to record demand
//! and coordinate with the receiver.
//!
//! Rendezvous channels should be used with care—it's inherently easy to deadlock unless they're being
//! used from separate tasks or an a coroutine setup (e.g. [`crate::future::pagination_stream::fn_stream::FnStream`])

use std::future::poll_fn;
use std::sync::Arc;
use std::task::{Context, Poll};
use tokio::sync::Semaphore;

/// Create a new rendezvous channel
///
/// Rendezvous channels are equivalent to a channel with a 0-sized buffer: A sender cannot send
/// until this is an active receiver waiting. This implementation uses a semaphore to record demand
/// and coordinate with the receiver.
pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let (tx, rx) = tokio::sync::mpsc::channel(1);
    let semaphore = Arc::new(Semaphore::new(0));
    (
        Sender {
            semaphore: semaphore.clone(),
            chan: tx,
        },
        Receiver {
            semaphore,
            chan: rx,
            needs_permit: false,
        },
    )
}

/// Errors for rendezvous channel
pub mod error {
    use std::fmt;
    use tokio::sync::mpsc::error::SendError as TokioSendError;

    /// Error when [crate::future::rendezvous::Sender] fails to send a value to the associated `Receiver`
    #[derive(Debug)]
    pub struct SendError<T> {
        source: TokioSendError<T>,
    }

    impl<T> SendError<T> {
        pub(crate) fn tokio_send_error(source: TokioSendError<T>) -> Self {
            Self { source }
        }
    }

    impl<T> fmt::Display for SendError<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "failed to send value to the receiver")
        }
    }

    impl<T: fmt::Debug + 'static> std::error::Error for SendError<T> {
        fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
            Some(&self.source)
        }
    }
}

#[derive(Debug)]
/// Sender-half of a channel
pub struct Sender<T> {
    semaphore: Arc<Semaphore>,
    chan: tokio::sync::mpsc::Sender<T>,
}

impl<T> Sender<T> {
    /// Send `item` into the channel waiting until there is matching demand
    ///
    /// Unlike something like `tokio::sync::mpsc::Channel` where sending a value will be buffered until
    /// demand exists, a rendezvous sender will wait until matching demand exists before this function will return.
    pub async fn send(&self, item: T) -> Result<(), error::SendError<T>> {
        let result = self.chan.send(item).await;
        // If this is an error, the rx half has been dropped. We will never get demand.
        if result.is_ok() {
            // The key here is that we block _after_ the send until more demand exists
            self.semaphore
                .acquire()
                .await
                .expect("semaphore is never closed")
                .forget();
        }
        result.map_err(error::SendError::tokio_send_error)
    }
}

#[derive(Debug)]
/// Receiver half of the rendezvous channel
pub struct Receiver<T> {
    semaphore: Arc<Semaphore>,
    chan: tokio::sync::mpsc::Receiver<T>,
    needs_permit: bool,
}

impl<T> Receiver<T> {
    /// Polls to receive an item from the channel
    pub async fn recv(&mut self) -> Option<T> {
        poll_fn(|cx| self.poll_recv(cx)).await
    }

    pub(crate) fn poll_recv(&mut self, cx: &mut Context<'_>) -> Poll<Option<T>> {
        // This uses `needs_permit` to track whether this is the first poll since we last returned an item.
        // If it is, we will grant a permit to the semaphore. Otherwise, we'll just forward the response through.
        let resp = self.chan.poll_recv(cx);
        // If there is no data on the channel, but we are reading, then give a permit so we can load data
        if self.needs_permit && matches!(resp, Poll::Pending) {
            self.needs_permit = false;
            self.semaphore.add_permits(1);
        }

        if matches!(resp, Poll::Ready(_)) {
            // we returned an item, no need to provide another permit until we fail to read from the channel again
            self.needs_permit = true;
        }
        resp
    }
}

#[cfg(test)]
mod test {
    use crate::future::rendezvous::channel;
    use std::sync::{Arc, Mutex};

    #[tokio::test]
    async fn send_blocks_caller() {
        let (tx, mut rx) = channel::<u8>();
        let done = Arc::new(Mutex::new(0));
        let idone = done.clone();
        let send = tokio::spawn(async move {
            *idone.lock().unwrap() = 1;
            tx.send(0).await.unwrap();
            *idone.lock().unwrap() = 2;
            tx.send(1).await.unwrap();
            *idone.lock().unwrap() = 3;
        });
        assert_eq!(*done.lock().unwrap(), 0);
        assert_eq!(rx.recv().await, Some(0));
        assert_eq!(*done.lock().unwrap(), 1);
        assert_eq!(rx.recv().await, Some(1));
        assert_eq!(*done.lock().unwrap(), 2);
        assert_eq!(rx.recv().await, None);
        assert_eq!(*done.lock().unwrap(), 3);
        let _ = send.await;
    }

    #[tokio::test]
    async fn send_errors_when_rx_dropped() {
        let (tx, rx) = channel::<u8>();
        drop(rx);
        tx.send(0).await.expect_err("rx half dropped");
    }
}
