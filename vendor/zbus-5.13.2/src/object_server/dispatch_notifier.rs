//! The object server API.

use event_listener::{Event, EventListener};
use serde::{Deserialize, Serialize};

use zvariant::{Signature, Type};

/// A response wrapper that notifies after the response has been sent.
///
/// Sometimes in [`interface`] method implementations we need to do some other work after the
/// response has been sent off. This wrapper type allows us to do that. Instead of returning your
/// intended response type directly, wrap it in this type and return it from your method. The
/// returned `EventListener` from the `new` method will be notified when the response has been sent.
///
/// A typical use case is sending off signals after the response has been sent. The easiest way to
/// do that is to spawn a task from the method that sends the signal but only after being notified
/// of the response dispatch.
///
/// # Caveats
///
/// The notification indicates that the response has been sent off, not that destination peer has
/// received it. That can only be guaranteed for a peer-to-peer connection.
///
/// [`interface`]: crate::interface
#[derive(Debug)]
pub struct ResponseDispatchNotifier<R> {
    response: R,
    event: Option<Event>,
}

impl<R> ResponseDispatchNotifier<R> {
    /// Create a new `NotifyResponse`.
    pub fn new(response: R) -> (Self, EventListener) {
        let event = Event::new();
        let listener = event.listen();
        (
            Self {
                response,
                event: Some(event),
            },
            listener,
        )
    }

    /// Get the response.
    pub fn response(&self) -> &R {
        &self.response
    }
}

impl<R> Serialize for ResponseDispatchNotifier<R>
where
    R: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.response.serialize(serializer)
    }
}

impl<'de, R> Deserialize<'de> for ResponseDispatchNotifier<R>
where
    R: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Self {
            response: R::deserialize(deserializer)?,
            event: None,
        })
    }
}

impl<R> Type for ResponseDispatchNotifier<R>
where
    R: Type,
{
    const SIGNATURE: &'static Signature = R::SIGNATURE;
}

impl<T> Drop for ResponseDispatchNotifier<T> {
    fn drop(&mut self) {
        if let Some(event) = self.event.take() {
            event.notify(usize::MAX);
        }
    }
}
