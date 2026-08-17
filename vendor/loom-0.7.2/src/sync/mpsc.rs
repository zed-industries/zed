//! A stub for `std::sync::mpsc`.

use crate::rt;

/// Mock implementation of `std::sync::mpsc::channel`.
#[track_caller]
pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let location = location!();
    let (sender_channel, receiver_channel) = std::sync::mpsc::channel();
    let channel = std::sync::Arc::new(rt::Channel::new(location));
    let sender = Sender {
        object: std::sync::Arc::clone(&channel),
        sender: sender_channel,
    };
    let receiver = Receiver {
        object: std::sync::Arc::clone(&channel),
        receiver: receiver_channel,
    };
    (sender, receiver)
}

#[derive(Debug)]
/// Mock implementation of `std::sync::mpsc::Sender`.
pub struct Sender<T> {
    object: std::sync::Arc<rt::Channel>,
    sender: std::sync::mpsc::Sender<T>,
}

impl<T> Sender<T> {
    /// Attempts to send a value on this channel, returning it back if it could
    /// not be sent.
    #[track_caller]
    pub fn send(&self, msg: T) -> Result<(), std::sync::mpsc::SendError<T>> {
        self.object.send(location!());
        self.sender.send(msg)
    }
}

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Sender<T> {
        Sender {
            object: std::sync::Arc::clone(&self.object),
            sender: self.sender.clone(),
        }
    }
}

#[derive(Debug)]
/// Mock implementation of `std::sync::mpsc::Receiver`.
pub struct Receiver<T> {
    object: std::sync::Arc<rt::Channel>,
    receiver: std::sync::mpsc::Receiver<T>,
}

impl<T> Receiver<T> {
    /// Attempts to wait for a value on this receiver, returning an error if the
    /// corresponding channel has hung up.
    #[track_caller]
    pub fn recv(&self) -> Result<T, std::sync::mpsc::RecvError> {
        self.object.recv(location!());
        self.receiver.recv()
    }
    /// Attempts to wait for a value on this receiver, returning an error if the
    /// corresponding channel has hung up, or if it waits more than `timeout`.
    pub fn recv_timeout(
        &self,
        _timeout: std::time::Duration,
    ) -> Result<T, std::sync::mpsc::RecvTimeoutError> {
        unimplemented!("std::sync::mpsc::Receiver::recv_timeout is not supported yet in Loom.")
    }

    /// Attempts to return a pending value on this receiver without blocking.
    pub fn try_recv(&self) -> Result<T, std::sync::mpsc::TryRecvError> {
        if self.object.is_empty() {
            return Err(std::sync::mpsc::TryRecvError::Empty);
        } else {
            self.recv().map_err(|e| e.into())
        }
    }
}

impl<T> Drop for Receiver<T> {
    fn drop(&mut self) {
        // Drain the channel.
        while !self.object.is_empty() {
            self.recv().unwrap();
        }
    }
}
