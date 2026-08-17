//! Macros for helping with common operations in Calloop.

/// Register a set of event sources. Effectively calls
/// [`EventSource::register()`] for all the sources provided.
///
/// Usage:
///
/// ```none,actually-rust-but-see-https://github.com/rust-lang/rust/issues/63193
/// calloop::batch_register!(
///     poll, token_factory,
///     self.source_one,
///     self.source_two,
///     self.source_three,
///     self.source_four,
/// )
/// ```
///
/// Note that there is no scope for customisation; if you need to do special
/// things with a particular source, you'll need to leave it off the list. Also
/// note that this only does try-or-early-return error handling in the order
/// that you list the sources; if you need anything else, don't use this macro.
///
/// [`EventSource::register()`]: crate::EventSource::register()
#[macro_export]
macro_rules! batch_register {
    ($poll:ident, $token_fac:ident, $( $source:expr ),* $(,)?) => {
        {
            $(
                $source.register($poll, $token_fac)?;
            )*
                $crate::Result::<_>::Ok(())
        }
    };
}

/// Reregister a set of event sources. Effectively calls
/// [`EventSource::reregister()`] for all the sources provided.
///
/// Usage:
///
/// ```none,actually-rust-but-see-https://github.com/rust-lang/rust/issues/63193
/// calloop::batch_reregister!(
///     poll, token_factory,
///     self.source_one,
///     self.source_two,
///     self.source_three,
///     self.source_four,
/// )
/// ```
///
/// Note that there is no scope for customisation; if you need to do special
/// things with a particular source, you'll need to leave it off the list. Also
/// note that this only does try-or-early-return error handling in the order
/// that you list the sources; if you need anything else, don't use this macro.
///
/// [`EventSource::reregister()`]: crate::EventSource::reregister()
#[macro_export]
macro_rules! batch_reregister {
    ($poll:ident, $token_fac:ident, $( $source:expr ),* $(,)?) => {
        {
            $(
                $source.reregister($poll, $token_fac)?;
            )*
                $crate::Result::<_>::Ok(())
        }
    };
}

/// Unregister a set of event sources. Effectively calls
/// [`EventSource::unregister()`] for all the sources provided.
///
/// Usage:
///
/// ```none,actually-rust-but-see-https://github.com/rust-lang/rust/issues/63193
/// calloop::batch_unregister!(
///     poll,
///     self.source_one,
///     self.source_two,
///     self.source_three,
///     self.source_four,
/// )
/// ```
///
/// Note that there is no scope for customisation; if you need to do special
/// things with a particular source, you'll need to leave it off the list. Also
/// note that this only does try-or-early-return error handling in the order
/// that you list the sources; if you need anything else, don't use this macro.
///
/// [`EventSource::unregister()`]: crate::EventSource::unregister()
#[macro_export]
macro_rules! batch_unregister {
    ($poll:ident, $( $source:expr ),* $(,)?) => {
        {
            $(
                $source.unregister($poll)?;
            )*
                $crate::Result::<_>::Ok(())
        }
    };
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::{
        ping::{make_ping, PingSource},
        EventSource, PostAction,
    };

    struct BatchSource {
        ping0: PingSource,
        ping1: PingSource,
        ping2: PingSource,
    }

    impl EventSource for BatchSource {
        type Event = usize;
        type Metadata = ();
        type Ret = ();
        type Error = Box<dyn std::error::Error + Sync + Send>;

        fn process_events<F>(
            &mut self,
            readiness: crate::Readiness,
            token: crate::Token,
            mut callback: F,
        ) -> Result<crate::PostAction, Self::Error>
        where
            F: FnMut(Self::Event, &mut Self::Metadata) -> Self::Ret,
        {
            self.ping0
                .process_events(readiness, token, |_, m| callback(0, m))?;
            self.ping1
                .process_events(readiness, token, |_, m| callback(1, m))?;
            self.ping2
                .process_events(readiness, token, |_, m| callback(2, m))?;
            Ok(PostAction::Continue)
        }

        fn register(
            &mut self,
            poll: &mut crate::Poll,
            token_factory: &mut crate::TokenFactory,
        ) -> crate::Result<()> {
            crate::batch_register!(poll, token_factory, self.ping0, self.ping1, self.ping2)
        }

        fn reregister(
            &mut self,
            poll: &mut crate::Poll,
            token_factory: &mut crate::TokenFactory,
        ) -> crate::Result<()> {
            crate::batch_reregister!(poll, token_factory, self.ping0, self.ping1, self.ping2)
        }

        fn unregister(&mut self, poll: &mut crate::Poll) -> crate::Result<()> {
            crate::batch_unregister!(poll, self.ping0, self.ping1, self.ping2)
        }
    }

    #[test]
    fn test_batch_operations() {
        let mut fired = [false; 3];

        let (send0, ping0) = make_ping().unwrap();
        let (send1, ping1) = make_ping().unwrap();
        let (send2, ping2) = make_ping().unwrap();

        let top = BatchSource {
            ping0,
            ping1,
            ping2,
        };

        let mut event_loop = crate::EventLoop::<[bool; 3]>::try_new().unwrap();
        let handle = event_loop.handle();

        let token = handle
            .insert_source(top, |idx, _, fired| {
                fired[idx] = true;
            })
            .unwrap();

        send0.ping();
        send1.ping();
        send2.ping();

        event_loop
            .dispatch(Duration::new(0, 0), &mut fired)
            .unwrap();

        assert_eq!(fired, [true; 3]);

        fired = [false; 3];

        handle.update(&token).unwrap();

        send0.ping();
        send1.ping();
        send2.ping();

        event_loop
            .dispatch(Duration::new(0, 0), &mut fired)
            .unwrap();

        assert_eq!(fired, [true; 3]);

        fired = [false; 3];

        handle.remove(token);

        send0.ping();
        send1.ping();
        send2.ping();

        event_loop
            .dispatch(Duration::new(0, 0), &mut fired)
            .unwrap();

        assert_eq!(fired, [false; 3]);
    }
}
