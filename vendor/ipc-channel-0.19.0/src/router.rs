// Copyright 2015 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Routers allow converting IPC channels to crossbeam channels.
//! The [RouterProxy](crate::router::RouterProxy) provides various methods to register
//! `IpcReceiver<T>`s. The router will then either call the appropriate callback or route the
//! message to a crossbeam `Sender<T>` or `Receiver<T>`. You should use the global `ROUTER` to
//! access the `RouterProxy` methods (via `ROUTER`'s `Deref` for `RouterProxy`.

use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;
use std::thread;

use crate::ipc::OpaqueIpcReceiver;
use crate::ipc::{self, IpcMessage, IpcReceiver, IpcReceiverSet, IpcSelectionResult, IpcSender};
use crossbeam_channel::{self, Receiver, Sender};
use serde::{Deserialize, Serialize};

lazy_static! {
    /// Global object wrapping a `RouterProxy`.
    /// Add routes ([add_route](RouterProxy::add_route)), or convert IpcReceiver<T>
    /// to crossbeam channels (e.g. [route_ipc_receiver_to_new_crossbeam_receiver](RouterProxy::route_ipc_receiver_to_new_crossbeam_receiver))
    pub static ref ROUTER: RouterProxy = RouterProxy::new();
}

/// A `RouterProxy` provides methods for talking to the router. Calling
/// [new](RouterProxy::new) automatically spins up a router thread which
/// waits for events on its registered `IpcReceiver<T>`s. The `RouterProxy`'s
/// methods communicate with the running router thread to register new
/// `IpcReceiver<T>`'s
pub struct RouterProxy {
    comm: Mutex<RouterProxyComm>,
}

#[allow(clippy::new_without_default)]
impl RouterProxy {
    pub fn new() -> RouterProxy {
        // Router acts like a receiver, running in its own thread with both
        // receiver ends.
        // Router proxy takes both sending ends.
        let (msg_sender, msg_receiver) = crossbeam_channel::unbounded();
        let (wakeup_sender, wakeup_receiver) = ipc::channel().unwrap();
        thread::spawn(move || Router::new(msg_receiver, wakeup_receiver).run());
        RouterProxy {
            comm: Mutex::new(RouterProxyComm {
                msg_sender,
                wakeup_sender,
                shutdown: false,
            }),
        }
    }

    /// Add a new (receiver, callback) pair to the router, and send a wakeup message
    /// to the router.
    ///
    /// Consider using [add_typed_route](Self::add_typed_route) instead, which prevents
    /// mismatches between the receiver and callback types.
    #[deprecated(since = "0.19.0", note = "please use 'add_typed_route' instead")]
    pub fn add_route(&self, receiver: OpaqueIpcReceiver, callback: RouterHandler) {
        let comm = self.comm.lock().unwrap();

        if comm.shutdown {
            return;
        }

        comm.msg_sender
            .send(RouterMsg::AddRoute(receiver, callback))
            .unwrap();
        comm.wakeup_sender.send(()).unwrap();
    }

    /// Add a new `(receiver, callback)` pair to the router, and send a wakeup message
    /// to the router.
    ///
    /// Unlike [add_route](Self::add_route) this method is strongly typed and guarantees
    /// that the `receiver` and the `callback` use the same message type.
    pub fn add_typed_route<T>(&self, receiver: IpcReceiver<T>, mut callback: TypedRouterHandler<T>)
    where
        T: Serialize + for<'de> Deserialize<'de> + 'static,
    {
        // Before passing the message on to the callback, turn it into the appropriate type
        let modified_callback = move |msg: IpcMessage| {
            let typed_message = msg.to::<T>();
            callback(typed_message)
        };

        #[allow(deprecated)]
        self.add_route(receiver.to_opaque(), Box::new(modified_callback));
    }

    /// Send a shutdown message to the router containing a ACK sender,
    /// send a wakeup message to the router, and block on the ACK.
    /// Calling it is idempotent,
    /// which can be useful when running a multi-process system in single-process mode.
    pub fn shutdown(&self) {
        let mut comm = self.comm.lock().unwrap();

        if comm.shutdown {
            return;
        }
        comm.shutdown = true;

        let (ack_sender, ack_receiver) = crossbeam_channel::unbounded();
        comm.wakeup_sender
            .send(())
            .map(|_| {
                comm.msg_sender
                    .send(RouterMsg::Shutdown(ack_sender))
                    .unwrap();
                ack_receiver.recv().unwrap();
            })
            .unwrap();
    }

    /// A convenience function to route an `IpcReceiver<T>` to an existing `Sender<T>`.
    pub fn route_ipc_receiver_to_crossbeam_sender<T>(
        &self,
        ipc_receiver: IpcReceiver<T>,
        crossbeam_sender: Sender<T>,
    ) where
        T: for<'de> Deserialize<'de> + Serialize + Send + 'static,
    {
        self.add_typed_route(
            ipc_receiver,
            Box::new(move |message| drop(crossbeam_sender.send(message.unwrap()))),
        )
    }

    /// A convenience function to route an `IpcReceiver<T>` to a `Receiver<T>`: the most common
    /// use of a `Router`.
    pub fn route_ipc_receiver_to_new_crossbeam_receiver<T>(
        &self,
        ipc_receiver: IpcReceiver<T>,
    ) -> Receiver<T>
    where
        T: for<'de> Deserialize<'de> + Serialize + Send + 'static,
    {
        let (crossbeam_sender, crossbeam_receiver) = crossbeam_channel::unbounded();
        self.route_ipc_receiver_to_crossbeam_sender(ipc_receiver, crossbeam_sender);
        crossbeam_receiver
    }
}

struct RouterProxyComm {
    msg_sender: Sender<RouterMsg>,
    wakeup_sender: IpcSender<()>,
    shutdown: bool,
}

/// Router runs in its own thread listening for events. Adds events to its IpcReceiverSet
/// and listens for events using select().
struct Router {
    /// Get messages from RouterProxy.
    msg_receiver: Receiver<RouterMsg>,
    /// The ID/index of the special channel we use to identify messages from msg_receiver.
    msg_wakeup_id: u64,
    /// Set of all receivers which have been registered for us to select on.
    ipc_receiver_set: IpcReceiverSet,
    /// Maps ids to their handler functions.
    handlers: HashMap<u64, RouterHandler>,
}

impl Router {
    fn new(msg_receiver: Receiver<RouterMsg>, wakeup_receiver: IpcReceiver<()>) -> Router {
        let mut ipc_receiver_set = IpcReceiverSet::new().unwrap();
        let msg_wakeup_id = ipc_receiver_set.add(wakeup_receiver).unwrap();
        Router {
            msg_receiver,
            msg_wakeup_id,
            ipc_receiver_set,
            handlers: HashMap::new(),
        }
    }

    /// Continuously loop waiting for wakeup signals from router proxy.
    /// Iterate over events either:
    /// 1) If a message comes in from our special `wakeup_receiver` (identified through
    ///    msg_wakeup_id. Read message from `msg_receiver` and add a new receiver
    ///    to our receiver set.
    /// 2) Call appropriate handler based on message id.
    /// 3) Remove handler once channel closes.
    fn run(&mut self) {
        loop {
            // Wait for events to come from our select() new channels are added to
            // our ReceiverSet below.
            let results = match self.ipc_receiver_set.select() {
                Ok(results) => results,
                Err(_) => break,
            };

            // Iterate over numerous events that were ready at this time.
            for result in results.into_iter() {
                match result {
                    // Message came from the RouterProxy. Listen on our `msg_receiver`
                    // channel.
                    IpcSelectionResult::MessageReceived(id, _) if id == self.msg_wakeup_id => {
                        match self.msg_receiver.recv().unwrap() {
                            RouterMsg::AddRoute(receiver, handler) => {
                                let new_receiver_id =
                                    self.ipc_receiver_set.add_opaque(receiver).unwrap();
                                self.handlers.insert(new_receiver_id, handler);
                            },
                            RouterMsg::Shutdown(sender) => {
                                sender
                                    .send(())
                                    .expect("Failed to send comfirmation of shutdown.");
                                break;
                            },
                        }
                    },
                    // Event from one of our registered receivers, call callback.
                    IpcSelectionResult::MessageReceived(id, message) => {
                        self.handlers.get_mut(&id).unwrap()(message)
                    },
                    IpcSelectionResult::ChannelClosed(id) => {
                        let _ = self.handlers.remove(&id).unwrap();
                    },
                }
            }
        }
    }
}

enum RouterMsg {
    /// Register the receiver OpaqueIpcReceiver for listening for events on.
    /// When a message comes from this receiver, call RouterHandler.
    AddRoute(OpaqueIpcReceiver, RouterHandler),
    /// Shutdown the router, providing a sender to send an acknowledgement.
    Shutdown(Sender<()>),
}

/// Function to call when a new event is received from the corresponding receiver.
pub type RouterHandler = Box<dyn FnMut(IpcMessage) + Send>;

/// Like [RouterHandler] but includes the type that will be passed to the callback
pub type TypedRouterHandler<T> = Box<dyn FnMut(Result<T, bincode::Error>) + Send>;
