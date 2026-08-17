use std::{future::Future, pin::Pin};

use zbus::message::Flags;
use zvariant::DynamicType;

use crate::{Connection, Result, message::Message};
use tracing::trace;

/// A helper type returned by [`Interface`](`crate::object_server::Interface`) callbacks.
pub enum DispatchResult<'a> {
    /// This interface does not support the given method.
    NotFound,

    /// Retry with [Interface::call_mut](`crate::object_server::Interface::call_mut).
    ///
    /// This is equivalent to NotFound if returned by call_mut.
    RequiresMut,

    /// The method was found and will be completed by running this Future.
    Async(Pin<Box<dyn Future<Output = Result<()>> + Send + 'a>>),
}

impl<'a> DispatchResult<'a> {
    /// Helper for creating the Async variant.
    pub fn new_async<F, T, E>(conn: &'a Connection, msg: &'a Message, f: F) -> Self
    where
        F: Future<Output = ::std::result::Result<T, E>> + Send + 'a,
        T: serde::Serialize + DynamicType + Send + Sync,
        E: zbus::DBusError + Send,
    {
        DispatchResult::Async(Box::pin(async move {
            let hdr = msg.header();
            let ret = f.await;
            if !hdr.primary().flags().contains(Flags::NoReplyExpected) {
                match ret {
                    Ok(r) => conn.reply(&hdr, &r).await,
                    Err(e) => conn.reply_dbus_error(&hdr, e).await,
                }
                .map(|_seq| ())
            } else {
                trace!("No reply expected for {:?} by the caller.", msg);
                Ok(())
            }
        }))
    }
}
