//! Asynchronous sinks
//!
//! This module contains the `Sink` trait, along with a number of adapter types
//! for it. An overview is available in the documentation for the trait itself.
//!
//! You can find more information/tutorials about streams [online at
//! https://tokio.rs][online]
//!
//! [online]: https://tokio.rs/docs/getting-started/streams-and-sinks/

use {IntoFuture, Poll, StartSend};
use stream::Stream;

mod with;
mod with_flat_map;
// mod with_map;
// mod with_filter;
// mod with_filter_map;
mod flush;
mod from_err;
mod send;
mod send_all;
mod map_err;
mod fanout;

if_std! {
    mod buffer;
    mod wait;

    pub use self::buffer::Buffer;
    pub use self::wait::Wait;

    // TODO: consider expanding this via e.g. FromIterator
    impl<T> Sink for ::std::vec::Vec<T> {
        type SinkItem = T;
        type SinkError = (); // Change this to ! once it stabilizes

        fn start_send(&mut self, item: Self::SinkItem)
                      -> StartSend<Self::SinkItem, Self::SinkError>
        {
            self.push(item);
            Ok(::AsyncSink::Ready)
        }

        fn poll_complete(&mut self) -> Poll<(), Self::SinkError> {
            Ok(::Async::Ready(()))
        }

        fn close(&mut self) -> Poll<(), Self::SinkError> {
            Ok(::Async::Ready(()))
        }
    }

    /// A type alias for `Box<Sink + Send>`
    pub type BoxSink<T, E> = ::std::boxed::Box<Sink<SinkItem = T, SinkError = E> +
                                               ::core::marker::Send>;

    impl<S: ?Sized + Sink> Sink for ::std::boxed::Box<S> {
        type SinkItem = S::SinkItem;
        type SinkError = S::SinkError;

        fn start_send(&mut self, item: Self::SinkItem)
                      -> StartSend<Self::SinkItem, Self::SinkError> {
            (**self).start_send(item)
        }

        fn poll_complete(&mut self) -> Poll<(), Self::SinkError> {
            (**self).poll_complete()
        }

        fn close(&mut self) -> Poll<(), Self::SinkError> {
            (**self).close()
        }
    }
}

pub use self::with::With;
pub use self::with_flat_map::WithFlatMap;
pub use self::flush::Flush;
pub use self::send::Send;
pub use self::send_all::SendAll;
pub use self::map_err::SinkMapErr;
pub use self::from_err::SinkFromErr;
pub use self::fanout::Fanout;

/// A `Sink` is a value into which other values can be sent, asynchronously.
///
/// Basic examples of sinks include the sending side of:
///
/// - Channels
/// - Sockets
/// - Pipes
///
/// In addition to such "primitive" sinks, it's typical to layer additional
/// functionality, such as buffering, on top of an existing sink.
///
/// Sending to a sink is "asynchronous" in the sense that the value may not be
/// sent in its entirety immediately. Instead, values are sent in a two-phase
/// way: first by initiating a send, and then by polling for completion. This
/// two-phase setup is analogous to buffered writing in synchronous code, where
/// writes often succeed immediately, but internally are buffered and are
/// *actually* written only upon flushing.
///
/// In addition, the `Sink` may be *full*, in which case it is not even possible
/// to start the sending process.
///
/// As with `Future` and `Stream`, the `Sink` trait is built from a few core
/// required methods, and a host of default methods for working in a
/// higher-level way. The `Sink::send_all` combinator is of particular
/// importance: you can use it to send an entire stream to a sink, which is
/// the simplest way to ultimately consume a sink.
///
/// You can find more information/tutorials about streams [online at
/// https://tokio.rs][online]
///
/// [online]: https://tokio.rs/docs/getting-started/streams-and-sinks/
pub trait Sink {
    /// The type of value that the sink accepts.
    type SinkItem;

    /// The type of value produced by the sink when an error occurs.
    type SinkError;

    /// Begin the process of sending a value to the sink.
    ///
    /// As the name suggests, this method only *begins* the process of sending
    /// the item. If the sink employs buffering, the item isn't fully processed
    /// until the buffer is fully flushed. Since sinks are designed to work with
    /// asynchronous I/O, the process of actually writing out the data to an
    /// underlying object takes place asynchronously. **You *must* use
    /// `poll_complete` in order to drive completion of a send**. In particular,
    /// `start_send` does not begin the flushing process
    ///
    /// # Return value
    ///
    /// This method returns `AsyncSink::Ready` if the sink was able to start
    /// sending `item`. In that case, you *must* ensure that you call
    /// `poll_complete` to process the sent item to completion. Note, however,
    /// that several calls to `start_send` can be made prior to calling
    /// `poll_complete`, which will work on completing all pending items.
    ///
    /// The method returns `AsyncSink::NotReady` if the sink was unable to begin
    /// sending, usually due to being full. The sink must have attempted to
    /// complete processing any outstanding requests (equivalent to
    /// `poll_complete`) before yielding this result. The current task will be
    /// automatically scheduled for notification when the sink may be ready to
    /// receive new values.
    ///
    /// # Errors
    ///
    /// If the sink encounters an error other than being temporarily full, it
    /// uses the `Err` variant to signal that error. In most cases, such errors
    /// mean that the sink will permanently be unable to receive items.
    ///
    /// # Panics
    ///
    /// This method may panic in a few situations, depending on the specific
    /// sink:
    ///
    /// - It is called outside of the context of a task.
    /// - A previous call to `start_send` or `poll_complete` yielded an error.
    fn start_send(&mut self, item: Self::SinkItem)
                  -> StartSend<Self::SinkItem, Self::SinkError>;

    /// Flush all output from this sink, if necessary.
    ///
    /// Some sinks may buffer intermediate data as an optimization to improve
    /// throughput. In other words, if a sink has a corresponding receiver then
    /// a successful `start_send` above may not guarantee that the value is
    /// actually ready to be received by the receiver. This function is intended
    /// to be used to ensure that values do indeed make their way to the
    /// receiver.
    ///
    /// This function will attempt to process any pending requests on behalf of
    /// the sink and drive it to completion.
    ///
    /// # Return value
    ///
    /// Returns `Ok(Async::Ready(()))` when no buffered items remain. If this
    /// value is returned then it is guaranteed that all previous values sent
    /// via `start_send` will be guaranteed to be available to a listening
    /// receiver.
    ///
    /// Returns `Ok(Async::NotReady)` if there is more work left to do, in which
    /// case the current task is scheduled to wake up when more progress may be
    /// possible.
    ///
    /// # Errors
    ///
    /// Returns `Err` if the sink encounters an error while processing one of
    /// its pending requests. Due to the buffered nature of requests, it is not
    /// generally possible to correlate the error with a particular request. As
    /// with `start_send`, these errors are generally "fatal" for continued use
    /// of the sink.
    ///
    /// # Panics
    ///
    /// This method may panic in a few situations, depending on the specific sink:
    ///
    /// - It is called outside of the context of a task.
    /// - A previous call to `start_send` or `poll_complete` yielded an error.
    ///
    /// # Compatibility nodes
    ///
    /// The name of this method may be slightly misleading as the original
    /// intention was to have this method be more general than just flushing
    /// requests. Over time though it was decided to trim back the ambitions of
    /// this method to what it's always done, just flushing.
    ///
    /// In the 0.2 release series of futures this method will be renamed to
    /// `poll_flush`. For 0.1, however, the breaking change is not happening
    /// yet.
    fn poll_complete(&mut self) -> Poll<(), Self::SinkError>;

    /// A method to indicate that no more values will ever be pushed into this
    /// sink.
    ///
    /// This method is used to indicate that a sink will no longer even be given
    /// another value by the caller. That is, the `start_send` method above will
    /// be called no longer (nor `poll_complete`). This method is intended to
    /// model "graceful shutdown" in various protocols where the intent to shut
    /// down is followed by a little more blocking work.
    ///
    /// Callers of this function should work it it in a similar fashion to
    /// `poll_complete`. Once called it may return `NotReady` which indicates
    /// that more external work needs to happen to make progress. The current
    /// task will be scheduled to receive a notification in such an event,
    /// however.
    ///
    /// Note that this function will imply `poll_complete` above. That is, if a
    /// sink has buffered data, then it'll be flushed out during a `close`
    /// operation. It is not necessary to have `poll_complete` return `Ready`
    /// before a `close` is called. Once a `close` is called, though,
    /// `poll_complete` cannot be called.
    ///
    /// # Return value
    ///
    /// This function, like `poll_complete`, returns a `Poll`. The value is
    /// `Ready` once the close operation has completed. At that point it should
    /// be safe to drop the sink and deallocate associated resources.
    ///
    /// If the value returned is `NotReady` then the sink is not yet closed and
    /// work needs to be done to close it. The work has been scheduled and the
    /// current task will receive a notification when it's next ready to call
    /// this method again.
    ///
    /// Finally, this function may also return an error.
    ///
    /// # Errors
    ///
    /// This function will return an `Err` if any operation along the way during
    /// the close operation fails. An error typically is fatal for a sink and is
    /// unable to be recovered from, but in specific situations this may not
    /// always be true.
    ///
    /// Note that it's also typically an error to call `start_send` or
    /// `poll_complete` after the `close` function is called. This method will
    /// *initiate* a close, and continuing to send values after that (or attempt
    /// to flush) may result in strange behavior, panics, errors, etc. Once this
    /// method is called, it must be the only method called on this `Sink`.
    ///
    /// # Panics
    ///
    /// This method may panic or cause panics if:
    ///
    /// * It is called outside the context of a future's task
    /// * It is called and then `start_send` or `poll_complete` is called
    ///
    /// # Compatibility notes
    ///
    /// Note that this function is currently by default a provided function,
    /// defaulted to calling `poll_complete` above. This function was added
    /// in the 0.1 series of the crate as a backwards-compatible addition. It
    /// is intended that in the 0.2 series the method will no longer be a
    /// default method.
    ///
    /// It is highly recommended to consider this method a required method and
    /// to implement it whenever you implement `Sink` locally. It is especially
    /// crucial to be sure to close inner sinks, if applicable.
    #[cfg(feature = "with-deprecated")]
    fn close(&mut self) -> Poll<(), Self::SinkError> {
        self.poll_complete()
    }

    /// dox (you should see the above, not this)
    #[cfg(not(feature = "with-deprecated"))]
    fn close(&mut self) -> Poll<(), Self::SinkError>;

    /// Creates a new object which will produce a synchronous sink.
    ///
    /// The sink returned does **not** implement the `Sink` trait, and instead
    /// only has two methods: `send` and `flush`. These two methods correspond
    /// to `start_send` and `poll_complete` above except are executed in a
    /// blocking fashion.
    #[cfg(feature = "use_std")]
    fn wait(self) -> Wait<Self>
        where Self: Sized
    {
        wait::new(self)
    }

    /// Composes a function *in front of* the sink.
    ///
    /// This adapter produces a new sink that passes each value through the
    /// given function `f` before sending it to `self`.
    ///
    /// To process each value, `f` produces a *future*, which is then polled to
    /// completion before passing its result down to the underlying sink. If the
    /// future produces an error, that error is returned by the new sink.
    ///
    /// Note that this function consumes the given sink, returning a wrapped
    /// version, much like `Iterator::map`.
    fn with<U, F, Fut>(self, f: F) -> With<Self, U, F, Fut>
        where F: FnMut(U) -> Fut,
              Fut: IntoFuture<Item = Self::SinkItem>,
              Fut::Error: From<Self::SinkError>,
              Self: Sized
    {
        with::new(self, f)
    }

    /// Composes a function *in front of* the sink.
    ///
    /// This adapter produces a new sink that passes each value through the
    /// given function `f` before sending it to `self`.
    ///
    /// To process each value, `f` produces a *stream*, of which each value
    /// is passed to the underlying sink. A new value will not be accepted until
    /// the stream has been drained
    ///
    /// Note that this function consumes the given sink, returning a wrapped
    /// version, much like `Iterator::flat_map`.
    ///
    /// # Examples
    /// ---
    /// Using this function with an iterator through use of the `stream::iter_ok()`
    /// function
    ///
    /// ```
    /// use futures::prelude::*;
    /// use futures::stream;
    /// use futures::sync::mpsc;
    ///
    /// let (tx, rx) = mpsc::channel::<i32>(5);
    ///
    /// let tx = tx.with_flat_map(|x| {
    ///     stream::iter_ok(vec![42; x].into_iter().map(|y| y))
    /// });
    /// tx.send(5).wait().unwrap();
    /// assert_eq!(rx.collect().wait(), Ok(vec![42, 42, 42, 42, 42]))
    /// ```
    fn with_flat_map<U, F, St>(self, f: F) -> WithFlatMap<Self, U, F, St>
        where F: FnMut(U) -> St,
              St: Stream<Item = Self::SinkItem, Error=Self::SinkError>,
              Self: Sized
        {
            with_flat_map::new(self, f)
        }

    /*
    fn with_map<U, F>(self, f: F) -> WithMap<Self, U, F>
        where F: FnMut(U) -> Self::SinkItem,
              Self: Sized;

    fn with_filter<F>(self, f: F) -> WithFilter<Self, F>
        where F: FnMut(Self::SinkItem) -> bool,
              Self: Sized;

    fn with_filter_map<U, F>(self, f: F) -> WithFilterMap<Self, U, F>
        where F: FnMut(U) -> Option<Self::SinkItem>,
              Self: Sized;
     */

    /// Transforms the error returned by the sink.
    fn sink_map_err<F, E>(self, f: F) -> SinkMapErr<Self, F>
        where F: FnOnce(Self::SinkError) -> E,
              Self: Sized,
    {
        map_err::new(self, f)
    }

    /// Map this sink's error to any error implementing `From` for this sink's
    /// `Error`, returning a new sink.
    ///
    /// If wanting to map errors of a `Sink + Stream`, use `.sink_from_err().from_err()`.
    fn sink_from_err<E: From<Self::SinkError>>(self) -> from_err::SinkFromErr<Self, E>
        where Self: Sized,
    {
        from_err::new(self)
    }


    /// Adds a fixed-size buffer to the current sink.
    ///
    /// The resulting sink will buffer up to `amt` items when the underlying
    /// sink is unwilling to accept additional items. Calling `poll_complete` on
    /// the buffered sink will attempt to both empty the buffer and complete
    /// processing on the underlying sink.
    ///
    /// Note that this function consumes the given sink, returning a wrapped
    /// version, much like `Iterator::map`.
    ///
    /// This method is only available when the `use_std` feature of this
    /// library is activated, and it is activated by default.
    #[cfg(feature = "use_std")]
    fn buffer(self, amt: usize) -> Buffer<Self>
        where Self: Sized
    {
        buffer::new(self, amt)
    }

    /// Fanout items to multiple sinks.
    ///
    /// This adapter clones each incoming item and forwards it to both this as well as
    /// the other sink at the same time.
    fn fanout<S>(self, other: S) -> Fanout<Self, S>
        where Self: Sized,
              Self::SinkItem: Clone,
              S: Sink<SinkItem=Self::SinkItem, SinkError=Self::SinkError>
    {
        fanout::new(self, other)
    }

    /// A future that completes when the sink has finished processing all
    /// pending requests.
    ///
    /// The sink itself is returned after flushing is complete; this adapter is
    /// intended to be used when you want to stop sending to the sink until
    /// all current requests are processed.
    fn flush(self) -> Flush<Self>
        where Self: Sized
    {
        flush::new(self)
    }

    /// A future that completes after the given item has been fully processed
    /// into the sink, including flushing.
    ///
    /// Note that, **because of the flushing requirement, it is usually better
    /// to batch together items to send via `send_all`, rather than flushing
    /// between each item.**
    ///
    /// On completion, the sink is returned.
    fn send(self, item: Self::SinkItem) -> Send<Self>
        where Self: Sized
    {
        send::new(self, item)
    }

    /// A future that completes after the given stream has been fully processed
    /// into the sink, including flushing.
    ///
    /// This future will drive the stream to keep producing items until it is
    /// exhausted, sending each item to the sink. It will complete once both the
    /// stream is exhausted, the sink has received all items, the sink has been
    /// flushed, and the sink has been closed.
    ///
    /// Doing `sink.send_all(stream)` is roughly equivalent to
    /// `stream.forward(sink)`. The returned future will exhaust all items from
    /// `stream` and send them to `self`, closing `self` when all items have been
    /// received.
    ///
    /// On completion, the pair `(sink, source)` is returned.
    fn send_all<S>(self, stream: S) -> SendAll<Self, S>
        where S: Stream<Item = Self::SinkItem>,
              Self::SinkError: From<S::Error>,
              Self: Sized
    {
        send_all::new(self, stream)
    }
}

impl<'a, S: ?Sized + Sink> Sink for &'a mut S {
    type SinkItem = S::SinkItem;
    type SinkError = S::SinkError;

    fn start_send(&mut self, item: Self::SinkItem)
                  -> StartSend<Self::SinkItem, Self::SinkError> {
        (**self).start_send(item)
    }

    fn poll_complete(&mut self) -> Poll<(), Self::SinkError> {
        (**self).poll_complete()
    }

    fn close(&mut self) -> Poll<(), Self::SinkError> {
        (**self).close()
    }
}
