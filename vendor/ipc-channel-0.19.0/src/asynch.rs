// Copyright 2015 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::ipc::{
    self, IpcMessage, IpcReceiver, IpcReceiverSet, IpcSelectionResult, IpcSender, OpaqueIpcReceiver,
};
use futures::channel::mpsc::UnboundedReceiver;
use futures::channel::mpsc::UnboundedSender;
use futures::stream::FusedStream;
use futures::task::Context;
use futures::task::Poll;
use futures::Stream;
use lazy_static::lazy_static;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::pin::Pin;
use std::sync::Mutex;
use std::thread;

/// A stream built from an IPC channel.
pub struct IpcStream<T>(UnboundedReceiver<IpcMessage>, PhantomData<T>);

impl<T> Unpin for IpcStream<T> {}

// A router which routes from an IPC channel to a stream.
struct Router {
    // Send `(ipc_recv, send)` to this router to add a route
    // from the IPC receiver to the sender.
    add_route: UnboundedSender<(OpaqueIpcReceiver, UnboundedSender<IpcMessage>)>,

    // Wake up the routing thread.
    wakeup: Mutex<IpcSender<()>>,
}

// Lazily initialize a singleton router,
// so we only end up with one routing thread per process.
lazy_static! {
    static ref ROUTER: Router = {
        let (send, mut recv) = futures::channel::mpsc::unbounded();
        let (waker, wakee) = ipc::channel().expect("Failed to create IPC channel");
        thread::spawn(move || {
            let mut receivers = IpcReceiverSet::new().expect("Failed to create receiver set");
            let mut senders = HashMap::<u64, UnboundedSender<IpcMessage>>::new();
            let _ = receivers.add(wakee);
            while let Ok(mut selections) = receivers.select() {
                for selection in selections.drain(..) {
                    match selection {
                        IpcSelectionResult::MessageReceived(id, msg) => {
                            if let Some(sender) = senders.get(&id) {
                                let _ = sender.unbounded_send(msg);
                            }
                        },
                        IpcSelectionResult::ChannelClosed(id) => {
                            senders.remove(&id);
                        },
                    }
                }
                if !recv.is_terminated() {
                    while let Ok(Some((receiver, sender))) = recv.try_next() {
                        if let Ok(id) = receivers.add_opaque(receiver) {
                            senders.insert(id, sender);
                        }
                    }
                }
            }
        });
        Router {
            add_route: send,
            wakeup: Mutex::new(waker),
        }
    };
}

impl<T> IpcReceiver<T>
where
    T: for<'de> Deserialize<'de> + Serialize,
{
    /// Convert this IPC receiver into a stream.
    pub fn to_stream(self) -> IpcStream<T> {
        let opaque = self.to_opaque();
        let (send, recv) = futures::channel::mpsc::unbounded();
        let _ = ROUTER.add_route.unbounded_send((opaque, send));
        if let Ok(waker) = ROUTER.wakeup.lock() {
            let _ = waker.send(());
        }
        IpcStream(recv, PhantomData)
    }
}

impl<T> Stream for IpcStream<T>
where
    T: for<'de> Deserialize<'de> + Serialize,
{
    type Item = Result<T, bincode::Error>;

    fn poll_next(mut self: Pin<&mut Self>, ctx: &mut Context) -> Poll<Option<Self::Item>> {
        let recv = Pin::new(&mut self.0);
        match recv.poll_next(ctx) {
            Poll::Ready(Some(msg)) => Poll::Ready(Some(msg.to())),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}

impl<T> FusedStream for IpcStream<T>
where
    T: for<'de> Deserialize<'de> + Serialize,
{
    fn is_terminated(&self) -> bool {
        self.0.is_terminated()
    }
}
