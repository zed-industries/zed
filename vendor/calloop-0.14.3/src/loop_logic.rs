use std::cell::{Cell, RefCell};
use std::collections::VecDeque;
use std::fmt::Debug;
use std::rc::{Rc, Weak};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use std::{io, slice};

#[cfg(feature = "block_on")]
use std::future::Future;

#[cfg(unix)]
use std::os::unix::io::{AsFd, AsRawFd, BorrowedFd, RawFd};
#[cfg(windows)]
use std::os::windows::io::{AsHandle, AsRawHandle, AsSocket as AsFd, BorrowedHandle, RawHandle};

use polling::Poller;
use tracing::{trace, warn};

use crate::list::{SourceEntry, SourceList};
use crate::sources::{Dispatcher, EventSource, Idle, IdleDispatcher};
use crate::sys::{Notifier, PollEvent};
use crate::token::TokenInner;
use crate::{
    AdditionalLifecycleEventsSet, InsertError, Poll, PostAction, Readiness, Token, TokenFactory,
};

type IdleCallback<'i, Data> = Rc<RefCell<dyn IdleDispatcher<Data> + 'i>>;

/// A token representing a registration in the [`EventLoop`].
///
/// This token is given to you by the [`EventLoop`] when an [`EventSource`] is inserted or
/// a [`Dispatcher`] is registered. You can use it to [disable](LoopHandle#method.disable),
/// [enable](LoopHandle#method.enable), [update`](LoopHandle#method.update),
/// [remove](LoopHandle#method.remove) or [kill](LoopHandle#method.kill) it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RegistrationToken {
    inner: TokenInner,
}

impl RegistrationToken {
    /// Create the RegistrationToken corresponding to the given raw key
    /// This is needed because some methods use `RegistrationToken`s as
    /// raw usizes within this crate
    pub(crate) fn new(inner: TokenInner) -> Self {
        Self { inner }
    }
}

pub(crate) struct LoopInner<'l, Data> {
    pub(crate) poll: RefCell<Poll>,
    // The `Option` is used to keep slots of the slab occupied, to prevent id reuse
    // while in-flight events might still refer to a recently destroyed event source.
    pub(crate) sources: RefCell<SourceList<'l, Data>>,
    pub(crate) sources_with_additional_lifecycle_events: RefCell<AdditionalLifecycleEventsSet>,
    idles: RefCell<VecDeque<IdleCallback<'l, Data>>>,
    pending_action: Cell<PostAction>,
}

/// A handle to an event loop
///
/// This handle allows you to insert new sources and idles in this event loop,
/// it can be cloned, and it is possible to insert new sources from within a source
/// callback.
pub struct LoopHandle<'l, Data> {
    inner: Rc<LoopInner<'l, Data>>,
}

/// Weak variant of a [`LoopHandle`]
pub struct WeakLoopHandle<'l, Data> {
    inner: Weak<LoopInner<'l, Data>>,
}

impl<Data> Debug for LoopHandle<'_, Data> {
    #[cfg_attr(feature = "nightly_coverage", coverage(off))]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("LoopHandle { ... }")
    }
}

/// Manually implemented `Clone`.
///
/// The derived implementation adds a `Clone` bound on the generic parameter `Data`.
impl<Data> Clone for LoopHandle<'_, Data> {
    #[cfg_attr(feature = "nightly_coverage", coverage(off))]
    fn clone(&self) -> Self {
        LoopHandle {
            inner: self.inner.clone(),
        }
    }
}

impl<'l, Data> LoopHandle<'l, Data> {
    /// Inserts a new event source in the loop.
    ///
    /// The provided callback will be called during the dispatching cycles whenever the
    /// associated source generates events, see `EventLoop::dispatch(..)` for details.
    ///
    /// This function takes ownership of the event source. Use `register_dispatcher`
    /// if you need access to the event source after this call.
    pub fn insert_source<S, F>(
        &self,
        source: S,
        callback: F,
    ) -> Result<RegistrationToken, InsertError<S>>
    where
        S: EventSource + 'l,
        F: FnMut(S::Event, &mut S::Metadata, &mut Data) -> S::Ret + 'l,
    {
        let dispatcher = Dispatcher::new(source, callback);
        self.register_dispatcher(dispatcher.clone())
            .map_err(|error| InsertError {
                error,
                inserted: dispatcher.into_source_inner(),
            })
    }

    /// Registers a `Dispatcher` in the loop.
    ///
    /// Use this function if you need access to the event source after its insertion in the loop.
    ///
    /// See also `insert_source`.
    #[cfg_attr(feature = "nightly_coverage", coverage(off))] // Contains a branch we can't hit w/o OOM
    pub fn register_dispatcher<S>(
        &self,
        dispatcher: Dispatcher<'l, S, Data>,
    ) -> crate::Result<RegistrationToken>
    where
        S: EventSource + 'l,
    {
        let mut sources = self.inner.sources.borrow_mut();
        let mut poll = self.inner.poll.borrow_mut();

        // Find an empty slot if any
        let slot = sources.vacant_entry();

        slot.source = Some(dispatcher.clone_as_event_dispatcher());
        trace!(source = slot.token.get_id(), "Inserting new source");
        let ret = slot.source.as_ref().unwrap().register(
            &mut poll,
            &mut self
                .inner
                .sources_with_additional_lifecycle_events
                .borrow_mut(),
            &mut TokenFactory::new(slot.token),
        );

        if let Err(error) = ret {
            slot.source = None;
            return Err(error);
        }

        Ok(RegistrationToken { inner: slot.token })
    }

    /// Inserts an idle callback.
    ///
    /// This callback will be called during a dispatching cycle when the event loop has
    /// finished processing all pending events from the sources and becomes idle.
    pub fn insert_idle<'i, F: FnOnce(&mut Data) + 'l + 'i>(&self, callback: F) -> Idle<'i> {
        let mut opt_cb = Some(callback);
        let callback = Rc::new(RefCell::new(Some(move |data: &mut Data| {
            if let Some(cb) = opt_cb.take() {
                cb(data);
            }
        })));
        self.inner.idles.borrow_mut().push_back(callback.clone());
        Idle { callback }
    }

    /// Enables this previously disabled event source.
    ///
    /// This previously disabled source will start generating events again.
    ///
    /// **Note:** this cannot be done from within the source callback.
    pub fn enable(&self, token: &RegistrationToken) -> crate::Result<()> {
        if let &SourceEntry {
            token: entry_token,
            source: Some(ref source),
        } = self.inner.sources.borrow().get(token.inner)?
        {
            trace!(source = entry_token.get_id(), "Registering source");
            source.register(
                &mut self.inner.poll.borrow_mut(),
                &mut self
                    .inner
                    .sources_with_additional_lifecycle_events
                    .borrow_mut(),
                &mut TokenFactory::new(entry_token),
            )
        } else {
            Err(crate::Error::InvalidToken)
        }
    }

    /// Makes this source update its registration.
    ///
    /// If after accessing the source you changed its parameters in a way that requires
    /// updating its registration.
    pub fn update(&self, token: &RegistrationToken) -> crate::Result<()> {
        if let &SourceEntry {
            token: entry_token,
            source: Some(ref source),
        } = self.inner.sources.borrow().get(token.inner)?
        {
            trace!(
                source = entry_token.get_id(),
                "Updating registration of source"
            );
            if !source.reregister(
                &mut self.inner.poll.borrow_mut(),
                &mut self
                    .inner
                    .sources_with_additional_lifecycle_events
                    .borrow_mut(),
                &mut TokenFactory::new(entry_token),
            )? {
                trace!(
                    source = entry_token.get_id(),
                    "Can't update registration withing a callback, storing for later."
                );
                // we are in a callback, store for later processing
                self.inner.pending_action.set(PostAction::Reregister);
            }
            Ok(())
        } else {
            Err(crate::Error::InvalidToken)
        }
    }

    /// Disables this event source.
    ///
    /// The source remains in the event loop, but it'll no longer generate events
    pub fn disable(&self, token: &RegistrationToken) -> crate::Result<()> {
        if let &SourceEntry {
            token: entry_token,
            source: Some(ref source),
        } = self.inner.sources.borrow().get(token.inner)?
        {
            if !token.inner.same_source_as(entry_token) {
                // The token provided by the user is no longer valid
                return Err(crate::Error::InvalidToken);
            }
            trace!(source = entry_token.get_id(), "Unregistering source");
            if !source.unregister(
                &mut self.inner.poll.borrow_mut(),
                &mut self
                    .inner
                    .sources_with_additional_lifecycle_events
                    .borrow_mut(),
                *token,
            )? {
                trace!(
                    source = entry_token.get_id(),
                    "Cannot unregister source in callback, storing for later."
                );
                // we are in a callback, store for later processing
                self.inner.pending_action.set(PostAction::Disable);
            }
            Ok(())
        } else {
            Err(crate::Error::InvalidToken)
        }
    }

    /// Removes this source from the event loop.
    pub fn remove(&self, token: RegistrationToken) {
        if let Ok(&mut SourceEntry {
            token: entry_token,
            ref mut source,
        }) = self.inner.sources.borrow_mut().get_mut(token.inner)
        {
            if let Some(source) = source.take() {
                trace!(source = entry_token.get_id(), "Removing source");
                if let Err(e) = source.unregister(
                    &mut self.inner.poll.borrow_mut(),
                    &mut self
                        .inner
                        .sources_with_additional_lifecycle_events
                        .borrow_mut(),
                    token,
                ) {
                    warn!("Failed to unregister source from the polling system: {e:?}");
                }
            }
        }
    }

    /// Wrap an IO object into an async adapter
    ///
    /// This adapter turns the IO object into an async-aware one that can be used in futures.
    /// The readiness of these futures will be driven by the event loop.
    ///
    /// The produced futures can be polled in any executor, and notably the one provided by
    /// calloop.
    pub fn adapt_io<F: AsFd>(&self, fd: F) -> crate::Result<crate::io::Async<'l, F>> {
        crate::io::Async::new(self.inner.clone(), fd)
    }

    /// Create a weak reference to this loop data.
    ///
    /// # Examples
    ///
    /// ```
    /// use calloop::timer::{TimeoutAction, Timer};
    /// use calloop::EventLoop;
    ///
    /// let event_loop: EventLoop<()> = EventLoop::try_new().unwrap();
    /// let weak_handle = event_loop.handle().downgrade();
    ///
    /// event_loop
    ///    .handle()
    ///    .insert_source(Timer::immediate(), move |_, _, _| {
    ///        // Hold its weak handle in the event loop's callback to break the reference cycle.
    ///        let handle = weak_handle.upgrade().unwrap();
    ///
    ///        // Use the upgraded handle later...
    ///
    ///        TimeoutAction::Drop
    ///    })
    ///    .unwrap();
    /// ```
    pub fn downgrade(&self) -> WeakLoopHandle<'l, Data> {
        WeakLoopHandle {
            inner: Rc::downgrade(&self.inner),
        }
    }
}

impl<Data> Debug for WeakLoopHandle<'_, Data> {
    #[cfg_attr(feature = "nightly_coverage", coverage(off))]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("WeakLoopHandle { ... }")
    }
}

/// Manually implemented `Clone`.
///
/// The derived implementation adds a `Clone` bound on the generic parameter `Data`.
impl<Data> Clone for WeakLoopHandle<'_, Data> {
    #[cfg_attr(feature = "nightly_coverage", coverage(off))]
    fn clone(&self) -> Self {
        WeakLoopHandle {
            inner: self.inner.clone(),
        }
    }
}

/// Manually implemented `Default`.
///
/// The derived implementation adds a `Default` bound on the generic parameter `Data`.
impl<Data> Default for WeakLoopHandle<'_, Data> {
    #[cfg_attr(feature = "nightly_coverage", coverage(off))]
    fn default() -> Self {
        WeakLoopHandle {
            inner: Weak::default(),
        }
    }
}

impl<'l, Data> WeakLoopHandle<'l, Data> {
    /// Try to get a [`LoopHandle`] from this weak reference.
    ///
    /// Returns [`None`] if the loop data has been dropped.
    pub fn upgrade(&self) -> Option<LoopHandle<'l, Data>> {
        self.inner.upgrade().map(|inner| LoopHandle { inner })
    }

    /// Check if the loop data has been dropped.
    pub fn expired(&self) -> bool {
        self.inner.strong_count() == 0
    }
}

/// An event loop
///
/// This loop can host several event sources, that can be dynamically added or removed.
pub struct EventLoop<'l, Data> {
    #[allow(dead_code)]
    poller: Arc<Poller>,
    handle: LoopHandle<'l, Data>,
    signals: Arc<Signals>,
    // A caching vector for synthetic poll events
    synthetic_events: Vec<PollEvent>,
}

impl<Data> Debug for EventLoop<'_, Data> {
    #[cfg_attr(feature = "nightly_coverage", coverage(off))]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("EventLoop { ... }")
    }
}

/// Signals related to the event loop.
struct Signals {
    /// Signal to stop the event loop.
    stop: AtomicBool,

    /// Signal that the future is ready.
    #[cfg(feature = "block_on")]
    future_ready: AtomicBool,
}

impl<'l, Data> EventLoop<'l, Data> {
    /// Create a new event loop
    ///
    /// Fails if the initialization of the polling system failed.
    pub fn try_new() -> crate::Result<Self> {
        let poll = Poll::new()?;
        let poller = poll.poller.clone();
        let handle = LoopHandle {
            inner: Rc::new(LoopInner {
                poll: RefCell::new(poll),
                sources: RefCell::new(SourceList::new()),
                idles: RefCell::new(VecDeque::new()),
                pending_action: Cell::new(PostAction::Continue),
                sources_with_additional_lifecycle_events: Default::default(),
            }),
        };

        Ok(EventLoop {
            handle,
            signals: Arc::new(Signals {
                stop: AtomicBool::new(false),
                #[cfg(feature = "block_on")]
                future_ready: AtomicBool::new(false),
            }),
            poller,
            synthetic_events: vec![],
        })
    }

    /// Retrieve a loop handle
    pub fn handle(&self) -> LoopHandle<'l, Data> {
        self.handle.clone()
    }

    fn dispatch_events(
        &mut self,
        mut timeout: Option<Duration>,
        data: &mut Data,
    ) -> crate::Result<()> {
        let now = Instant::now();
        {
            let mut extra_lifecycle_sources = self
                .handle
                .inner
                .sources_with_additional_lifecycle_events
                .borrow_mut();
            let sources = &self.handle.inner.sources.borrow();
            for source in &mut *extra_lifecycle_sources.values {
                if let Ok(SourceEntry {
                    source: Some(disp), ..
                }) = sources.get(source.inner)
                {
                    if let Some((readiness, token)) = disp.before_sleep()? {
                        // Wake up instantly after polling if we recieved an event
                        timeout = Some(Duration::ZERO);
                        self.synthetic_events.push(PollEvent { readiness, token });
                    }
                } else {
                    unreachable!()
                }
            }
        }
        let events = {
            let poll = self.handle.inner.poll.borrow();
            loop {
                let result = poll.poll(timeout);

                match result {
                    Ok(events) => break events,
                    Err(crate::Error::IoError(err)) if err.kind() == io::ErrorKind::Interrupted => {
                        // Interrupted by a signal. Update timeout and retry.
                        if let Some(to) = timeout {
                            let elapsed = now.elapsed();
                            if elapsed >= to {
                                return Ok(());
                            } else {
                                timeout = Some(to - elapsed);
                            }
                        }
                    }
                    Err(err) => return Err(err),
                };
            }
        };
        {
            let mut extra_lifecycle_sources = self
                .handle
                .inner
                .sources_with_additional_lifecycle_events
                .borrow_mut();
            if !extra_lifecycle_sources.values.is_empty() {
                for source in &mut *extra_lifecycle_sources.values {
                    if let Ok(SourceEntry {
                        source: Some(disp), ..
                    }) = self.handle.inner.sources.borrow().get(source.inner)
                    {
                        let iter = EventIterator {
                            inner: events.iter(),
                            registration_token: *source,
                        };
                        disp.before_handle_events(iter);
                    } else {
                        unreachable!()
                    }
                }
            }
        }

        for event in self.synthetic_events.drain(..).chain(events) {
            // Get the registration token associated with the event.
            let reg_token = event.token.inner.forget_sub_id();

            let opt_disp = self
                .handle
                .inner
                .sources
                .borrow()
                .get(reg_token)
                .ok()
                .and_then(|entry| entry.source.clone());

            if let Some(disp) = opt_disp {
                trace!(source = reg_token.get_id(), "Dispatching events for source");
                let mut ret = disp.process_events(event.readiness, event.token, data)?;

                // if the returned PostAction is Continue, it may be overwritten by a user-specified pending action
                let pending_action = self
                    .handle
                    .inner
                    .pending_action
                    .replace(PostAction::Continue);
                if let PostAction::Continue = ret {
                    ret = pending_action;
                }

                match ret {
                    PostAction::Reregister => {
                        trace!(
                            source = reg_token.get_id(),
                            "Postaction reregister for source"
                        );
                        disp.reregister(
                            &mut self.handle.inner.poll.borrow_mut(),
                            &mut self
                                .handle
                                .inner
                                .sources_with_additional_lifecycle_events
                                .borrow_mut(),
                            &mut TokenFactory::new(reg_token),
                        )?;
                    }
                    PostAction::Disable => {
                        trace!(
                            source = reg_token.get_id(),
                            "Postaction unregister for source"
                        );
                        disp.unregister(
                            &mut self.handle.inner.poll.borrow_mut(),
                            &mut self
                                .handle
                                .inner
                                .sources_with_additional_lifecycle_events
                                .borrow_mut(),
                            RegistrationToken::new(reg_token),
                        )?;
                    }
                    PostAction::Remove => {
                        trace!(source = reg_token.get_id(), "Postaction remove for source");
                        if let Ok(entry) = self.handle.inner.sources.borrow_mut().get_mut(reg_token)
                        {
                            entry.source = None;
                        }
                    }
                    PostAction::Continue => {}
                }

                if self
                    .handle
                    .inner
                    .sources
                    .borrow()
                    .get(reg_token)
                    .ok()
                    .map(|entry| entry.source.is_none())
                    .unwrap_or(true)
                {
                    // the source has been removed from within its callback, unregister it
                    let mut poll = self.handle.inner.poll.borrow_mut();
                    if let Err(e) = disp.unregister(
                        &mut poll,
                        &mut self
                            .handle
                            .inner
                            .sources_with_additional_lifecycle_events
                            .borrow_mut(),
                        RegistrationToken::new(reg_token),
                    ) {
                        warn!("Failed to unregister source from the polling system: {e:?}",);
                    }
                }
            } else {
                warn!(?reg_token, "Received an event for non-existent source");
            }
        }

        Ok(())
    }

    fn dispatch_idles(&mut self, data: &mut Data) {
        let mut idles = self.handle.inner.idles.borrow_mut();
        let now = Instant::now();

        // make sure to check events every 10ms
        // let mut idles_dispatched = 0;
        while now.elapsed().as_millis() < 10 {
            let Some(idle) = idles.pop_front() else {
                return;
            };

            idle.borrow_mut().dispatch(data);
        }
    }

    /// Dispatch pending events to their callbacks
    ///
    /// If some sources have events available, their callbacks will be immediately called.
    /// Otherwise, this will wait until an event is received or the provided `timeout`
    /// is reached. If `timeout` is `None`, it will wait without a duration limit.
    ///
    /// Once pending events have been processed or the timeout is reached, all pending
    /// idle callbacks will be fired before this method returns.
    pub fn dispatch<D: Into<Option<Duration>>>(
        &mut self,
        timeout: D,
        data: &mut Data,
    ) -> crate::Result<()> {
        self.dispatch_events(timeout.into(), data)?;
        self.dispatch_idles(data);

        Ok(())
    }

    /// Get a signal to stop this event loop from running
    ///
    /// To be used in conjunction with the `run()` method.
    pub fn get_signal(&self) -> LoopSignal {
        LoopSignal {
            signal: self.signals.clone(),
            notifier: self.handle.inner.poll.borrow().notifier(),
        }
    }

    /// Run this event loop
    ///
    /// This will repeatedly try to dispatch events (see the `dispatch()` method) on
    /// this event loop, waiting at most `timeout` every time.
    ///
    /// Between each dispatch wait, your provided callback will be called.
    ///
    /// You can use the `get_signal()` method to retrieve a way to stop or wakeup
    /// the event loop from anywhere.
    pub fn run<F, D: Into<Option<Duration>>>(
        &mut self,
        timeout: D,
        data: &mut Data,
        mut cb: F,
    ) -> crate::Result<()>
    where
        F: FnMut(&mut Data),
    {
        let timeout = timeout.into();
        self.signals.stop.store(false, Ordering::Release);
        while !self.signals.stop.load(Ordering::Acquire) {
            self.dispatch(timeout, data)?;
            cb(data);
        }
        Ok(())
    }

    /// Block a future on this event loop.
    ///
    /// This will run the provided future on this event loop, blocking until it is
    /// resolved.
    ///
    /// If [`LoopSignal::stop()`] is called before the future is resolved, this function returns
    /// `None`.
    #[cfg(feature = "block_on")]
    pub fn block_on<R>(
        &mut self,
        future: impl Future<Output = R>,
        data: &mut Data,
        mut cb: impl FnMut(&mut Data),
    ) -> crate::Result<Option<R>> {
        use std::task::{Context, Poll, Wake, Waker};

        /// A waker that will wake up the event loop when it is ready to make progress.
        struct EventLoopWaker(LoopSignal);

        impl Wake for EventLoopWaker {
            fn wake(self: Arc<Self>) {
                // Set the waker.
                self.0.signal.future_ready.store(true, Ordering::Release);
                self.0.notifier.notify().ok();
            }

            fn wake_by_ref(self: &Arc<Self>) {
                // Set the waker.
                self.0.signal.future_ready.store(true, Ordering::Release);
                self.0.notifier.notify().ok();
            }
        }

        // Pin the future to the stack.
        pin_utils::pin_mut!(future);

        // Create a waker that will wake up the event loop when it is ready to make progress.
        let waker = {
            let handle = EventLoopWaker(self.get_signal());

            Waker::from(Arc::new(handle))
        };
        let mut context = Context::from_waker(&waker);

        // Begin running the loop.
        let mut output = None;

        self.signals.stop.store(false, Ordering::Release);
        self.signals.future_ready.store(true, Ordering::Release);

        while !self.signals.stop.load(Ordering::Acquire) {
            // If the future is ready to be polled, poll it.
            if self.signals.future_ready.swap(false, Ordering::AcqRel) {
                // Poll the future and break the loop if it's ready.
                if let Poll::Ready(result) = future.as_mut().poll(&mut context) {
                    output = Some(result);
                    break;
                }
            }

            // Otherwise, block on the event loop.
            self.dispatch_events(None, data)?;
            self.dispatch_idles(data);
            cb(data);
        }

        Ok(output)
    }
}

#[cfg(unix)]
impl<Data> AsRawFd for EventLoop<'_, Data> {
    /// Get the underlying raw-fd of the poller.
    ///
    /// This could be used to create [`Generic`] source out of the current loop
    /// and inserting into some other [`EventLoop`]. It's recommended to clone `fd`
    /// before doing so.
    ///
    /// [`Generic`]: crate::generic::Generic
    fn as_raw_fd(&self) -> RawFd {
        self.poller.as_raw_fd()
    }
}

#[cfg(unix)]
impl<Data> AsFd for EventLoop<'_, Data> {
    /// Get the underlying fd of the poller.
    ///
    /// This could be used to create [`Generic`] source out of the current loop
    /// and inserting into some other [`EventLoop`].
    ///
    /// [`Generic`]: crate::generic::Generic
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.poller.as_fd()
    }
}

#[cfg(windows)]
impl<Data> AsRawHandle for EventLoop<'_, Data> {
    fn as_raw_handle(&self) -> RawHandle {
        self.poller.as_raw_handle()
    }
}

#[cfg(windows)]
impl<Data> AsHandle for EventLoop<'_, Data> {
    fn as_handle(&self) -> BorrowedHandle<'_> {
        self.poller.as_handle()
    }
}

#[derive(Clone, Debug)]
/// The EventIterator is an `Iterator` over the events relevant to a particular source
/// This type is used in the [`EventSource::before_handle_events`] methods for
/// two main reasons:
///
/// - To avoid dynamic dispatch overhead
/// - Secondly, it is to allow this type to be `Clone`, which is not
///   possible with dynamic dispatch
pub struct EventIterator<'a> {
    inner: slice::Iter<'a, PollEvent>,
    registration_token: RegistrationToken,
}

impl Iterator for EventIterator<'_> {
    type Item = (Readiness, Token);

    fn next(&mut self) -> Option<Self::Item> {
        for next in self.inner.by_ref() {
            if next
                .token
                .inner
                .same_source_as(self.registration_token.inner)
            {
                return Some((next.readiness, next.token));
            }
        }
        None
    }
}

/// A signal that can be shared between thread to stop or wakeup a running
/// event loop
#[derive(Clone)]
pub struct LoopSignal {
    signal: Arc<Signals>,
    notifier: Notifier,
}

impl Debug for LoopSignal {
    #[cfg_attr(feature = "nightly_coverage", coverage(off))]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("LoopSignal { ... }")
    }
}

impl LoopSignal {
    /// Stop the event loop
    ///
    /// Once this method is called, the next time the event loop has finished
    /// waiting for events, it will return rather than starting to wait again.
    ///
    /// This is only useful if you are using the `EventLoop::run()` method.
    pub fn stop(&self) {
        self.signal.stop.store(true, Ordering::Release);
    }

    /// Wake up the event loop
    ///
    /// This sends a dummy event to the event loop to simulate the reception
    /// of an event, making the wait return early. Called after `stop()`, this
    /// ensures the event loop will terminate quickly if you specified a long
    /// timeout (or no timeout at all) to the `dispatch` or `run` method.
    pub fn wakeup(&self) {
        self.notifier.notify().ok();
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::Cell, rc::Rc, time::Duration};

    use crate::{
        channel::{channel, Channel},
        ping::*,
        timer, EventIterator, EventSource, Poll, PostAction, Readiness, RegistrationToken, Token,
        TokenFactory,
    };

    #[cfg(unix)]
    use crate::{generic::Generic, Dispatcher, Interest, Mode};

    use super::EventLoop;

    #[test]
    fn dispatch_idle() {
        let mut event_loop = EventLoop::try_new().unwrap();

        let mut dispatched = false;

        event_loop.handle().insert_idle(|d| {
            *d = true;
        });

        event_loop
            .dispatch(Some(Duration::ZERO), &mut dispatched)
            .unwrap();

        assert!(dispatched);
    }

    #[test]
    fn cancel_idle() {
        let mut event_loop = EventLoop::try_new().unwrap();

        let mut dispatched = false;

        let handle = event_loop.handle();
        let idle = handle.insert_idle(move |d| {
            *d = true;
        });

        idle.cancel();

        event_loop
            .dispatch(Duration::ZERO, &mut dispatched)
            .unwrap();

        assert!(!dispatched);
    }

    #[test]
    fn wakeup() {
        let mut event_loop = EventLoop::try_new().unwrap();

        let signal = event_loop.get_signal();

        ::std::thread::spawn(move || {
            ::std::thread::sleep(Duration::from_millis(500));
            signal.wakeup();
        });

        // the test should return
        event_loop.dispatch(None, &mut ()).unwrap();
    }

    #[test]
    fn wakeup_stop() {
        let mut event_loop = EventLoop::try_new().unwrap();

        let signal = event_loop.get_signal();

        ::std::thread::spawn(move || {
            ::std::thread::sleep(Duration::from_millis(500));
            signal.stop();
            signal.wakeup();
        });

        // the test should return
        event_loop.run(None, &mut (), |_| {}).unwrap();
    }

    #[test]
    fn additional_events() {
        let mut event_loop: EventLoop<'_, Lock> = EventLoop::try_new().unwrap();
        let mut lock = Lock {
            lock: Rc::new((
                // Whether the lock is locked
                Cell::new(false),
                // The total number of events processed in process_events
                Cell::new(0),
                // The total number of events processed in before_handle_events
                // This is used to ensure that the count seen in before_handle_events is expected
                Cell::new(0),
            )),
        };
        let (sender, channel) = channel();
        let token = event_loop
            .handle()
            .insert_source(
                LockingSource {
                    channel,
                    lock: lock.clone(),
                },
                |_, _, lock| {
                    lock.lock();
                    lock.unlock();
                },
            )
            .unwrap();
        sender.send(()).unwrap();

        event_loop.dispatch(None, &mut lock).unwrap();
        // We should have been locked twice so far
        assert_eq!(lock.lock.1.get(), 2);
        // And we should have received one event
        assert_eq!(lock.lock.2.get(), 1);
        event_loop.handle().disable(&token).unwrap();
        event_loop
            .dispatch(Some(Duration::ZERO), &mut lock)
            .unwrap();
        assert_eq!(lock.lock.1.get(), 2);

        event_loop.handle().enable(&token).unwrap();
        event_loop
            .dispatch(Some(Duration::ZERO), &mut lock)
            .unwrap();
        assert_eq!(lock.lock.1.get(), 3);
        event_loop.handle().remove(token);
        event_loop
            .dispatch(Some(Duration::ZERO), &mut lock)
            .unwrap();
        assert_eq!(lock.lock.1.get(), 3);
        assert_eq!(lock.lock.2.get(), 1);

        #[derive(Clone)]
        struct Lock {
            lock: Rc<(Cell<bool>, Cell<u32>, Cell<u32>)>,
        }
        impl Lock {
            fn lock(&self) {
                if self.lock.0.get() {
                    panic!();
                }
                // Increase the count
                self.lock.1.set(self.lock.1.get() + 1);
                self.lock.0.set(true)
            }
            fn unlock(&self) {
                if !self.lock.0.get() {
                    panic!();
                }
                self.lock.0.set(false);
            }
        }
        struct LockingSource {
            channel: Channel<()>,
            lock: Lock,
        }
        impl EventSource for LockingSource {
            type Event = <Channel<()> as EventSource>::Event;

            type Metadata = <Channel<()> as EventSource>::Metadata;

            type Ret = <Channel<()> as EventSource>::Ret;

            type Error = <Channel<()> as EventSource>::Error;

            fn process_events<F>(
                &mut self,
                readiness: Readiness,
                token: Token,
                callback: F,
            ) -> Result<PostAction, Self::Error>
            where
                F: FnMut(Self::Event, &mut Self::Metadata) -> Self::Ret,
            {
                self.channel.process_events(readiness, token, callback)
            }

            fn register(
                &mut self,
                poll: &mut Poll,
                token_factory: &mut TokenFactory,
            ) -> crate::Result<()> {
                self.channel.register(poll, token_factory)
            }

            fn reregister(
                &mut self,
                poll: &mut Poll,
                token_factory: &mut TokenFactory,
            ) -> crate::Result<()> {
                self.channel.reregister(poll, token_factory)
            }

            fn unregister(&mut self, poll: &mut Poll) -> crate::Result<()> {
                self.channel.unregister(poll)
            }

            const NEEDS_EXTRA_LIFECYCLE_EVENTS: bool = true;

            fn before_sleep(&mut self) -> crate::Result<Option<(Readiness, Token)>> {
                self.lock.lock();
                Ok(None)
            }

            fn before_handle_events(&mut self, events: EventIterator) {
                let events_count = events.count();
                let lock = &self.lock.lock;
                lock.2.set(lock.2.get() + events_count as u32);
                self.lock.unlock();
            }
        }
    }
    #[test]
    fn default_additional_events() {
        let (sender, channel) = channel();
        let mut test_source = NoopWithDefaultHandlers { channel };
        let mut event_loop = EventLoop::try_new().unwrap();
        event_loop
            .handle()
            .insert_source(Box::new(&mut test_source), |_, _, _| {})
            .unwrap();
        sender.send(()).unwrap();

        event_loop.dispatch(None, &mut ()).unwrap();
        struct NoopWithDefaultHandlers {
            channel: Channel<()>,
        }
        impl EventSource for NoopWithDefaultHandlers {
            type Event = <Channel<()> as EventSource>::Event;

            type Metadata = <Channel<()> as EventSource>::Metadata;

            type Ret = <Channel<()> as EventSource>::Ret;

            type Error = <Channel<()> as EventSource>::Error;

            fn process_events<F>(
                &mut self,
                readiness: Readiness,
                token: Token,
                callback: F,
            ) -> Result<PostAction, Self::Error>
            where
                F: FnMut(Self::Event, &mut Self::Metadata) -> Self::Ret,
            {
                self.channel.process_events(readiness, token, callback)
            }

            fn register(
                &mut self,
                poll: &mut Poll,
                token_factory: &mut TokenFactory,
            ) -> crate::Result<()> {
                self.channel.register(poll, token_factory)
            }

            fn reregister(
                &mut self,
                poll: &mut Poll,
                token_factory: &mut TokenFactory,
            ) -> crate::Result<()> {
                self.channel.reregister(poll, token_factory)
            }

            fn unregister(&mut self, poll: &mut Poll) -> crate::Result<()> {
                self.channel.unregister(poll)
            }

            const NEEDS_EXTRA_LIFECYCLE_EVENTS: bool = true;
        }
    }

    #[test]
    fn additional_events_synthetic() {
        let mut event_loop: EventLoop<'_, Lock> = EventLoop::try_new().unwrap();
        let mut lock = Lock {
            lock: Rc::new(Cell::new(false)),
        };
        event_loop
            .handle()
            .insert_source(
                InstantWakeupLockingSource {
                    lock: lock.clone(),
                    token: None,
                },
                |_, _, lock| {
                    lock.lock();
                    lock.unlock();
                },
            )
            .unwrap();

        // Loop should finish, as
        event_loop.dispatch(None, &mut lock).unwrap();
        #[derive(Clone)]
        struct Lock {
            lock: Rc<Cell<bool>>,
        }
        impl Lock {
            fn lock(&self) {
                if self.lock.get() {
                    panic!();
                }
                self.lock.set(true)
            }
            fn unlock(&self) {
                if !self.lock.get() {
                    panic!();
                }
                self.lock.set(false);
            }
        }
        struct InstantWakeupLockingSource {
            lock: Lock,
            token: Option<Token>,
        }
        impl EventSource for InstantWakeupLockingSource {
            type Event = ();

            type Metadata = ();

            type Ret = ();

            type Error = <Channel<()> as EventSource>::Error;

            fn process_events<F>(
                &mut self,
                _: Readiness,
                token: Token,
                mut callback: F,
            ) -> Result<PostAction, Self::Error>
            where
                F: FnMut(Self::Event, &mut Self::Metadata) -> Self::Ret,
            {
                assert_eq!(token, self.token.unwrap());
                callback((), &mut ());
                Ok(PostAction::Continue)
            }

            fn register(
                &mut self,
                _: &mut Poll,
                token_factory: &mut TokenFactory,
            ) -> crate::Result<()> {
                self.token = Some(token_factory.token());
                Ok(())
            }

            fn reregister(&mut self, _: &mut Poll, _: &mut TokenFactory) -> crate::Result<()> {
                unreachable!()
            }

            fn unregister(&mut self, _: &mut Poll) -> crate::Result<()> {
                unreachable!()
            }

            const NEEDS_EXTRA_LIFECYCLE_EVENTS: bool = true;

            fn before_sleep(&mut self) -> crate::Result<Option<(Readiness, Token)>> {
                self.lock.lock();
                Ok(Some((Readiness::EMPTY, self.token.unwrap())))
            }

            fn before_handle_events(&mut self, _: EventIterator) {
                self.lock.unlock();
            }
        }
    }

    #[cfg(unix)]
    #[test]
    fn insert_bad_source() {
        use std::mem::ManuallyDrop;
        use std::os::unix::io::{AsFd, FromRawFd, OwnedFd};

        struct LeakedFd(ManuallyDrop<OwnedFd>);

        impl AsFd for LeakedFd {
            fn as_fd(&self) -> std::os::unix::prelude::BorrowedFd<'_> {
                self.0.as_fd()
            }
        }

        let event_loop = EventLoop::<()>::try_new().unwrap();
        let fd = LeakedFd(ManuallyDrop::new(unsafe {
            std::os::unix::io::OwnedFd::from_raw_fd(420)
        }));
        let ret = event_loop.handle().insert_source(
            crate::sources::generic::Generic::new(fd, Interest::READ, Mode::Level),
            |_, _, _| Ok(PostAction::Continue),
        );
        assert!(ret.is_err());
    }

    #[test]
    fn invalid_token() {
        let (_ping, source) = crate::sources::ping::make_ping().unwrap();

        let event_loop = EventLoop::<()>::try_new().unwrap();
        let handle = event_loop.handle();
        let reg_token = handle.insert_source(source, |_, _, _| {}).unwrap();
        handle.remove(reg_token);

        let ret = handle.enable(&reg_token);
        assert!(ret.is_err());
    }

    #[cfg(unix)]
    #[test]
    fn insert_source_no_interest() {
        use rustix::pipe::pipe;

        // Create a pipe to get an arbitrary fd.
        let (read, _write) = pipe().unwrap();

        let source = crate::sources::generic::Generic::new(read, Interest::EMPTY, Mode::Level);
        let dispatcher = Dispatcher::new(source, |_, _, _| Ok(PostAction::Continue));

        let event_loop = EventLoop::<()>::try_new().unwrap();
        let handle = event_loop.handle();
        let ret = handle.register_dispatcher(dispatcher.clone());

        if let Ok(token) = ret {
            // Unwrap the dispatcher+source and close the read end.
            handle.remove(token);
        } else {
            // Fail the test.
            panic!();
        }
    }

    #[test]
    fn disarm_rearm() {
        let mut event_loop = EventLoop::<bool>::try_new().unwrap();
        let (ping, ping_source) = make_ping().unwrap();

        let ping_token = event_loop
            .handle()
            .insert_source(ping_source, |(), &mut (), dispatched| {
                *dispatched = true;
            })
            .unwrap();

        ping.ping();
        let mut dispatched = false;
        event_loop
            .dispatch(Duration::ZERO, &mut dispatched)
            .unwrap();
        assert!(dispatched);

        // disable the source
        ping.ping();
        event_loop.handle().disable(&ping_token).unwrap();
        let mut dispatched = false;
        event_loop
            .dispatch(Duration::ZERO, &mut dispatched)
            .unwrap();
        assert!(!dispatched);

        // reenable it, the previous ping now gets dispatched
        event_loop.handle().enable(&ping_token).unwrap();
        let mut dispatched = false;
        event_loop
            .dispatch(Duration::ZERO, &mut dispatched)
            .unwrap();
        assert!(dispatched);
    }

    #[test]
    fn multiple_tokens() {
        struct DoubleSource {
            ping1: PingSource,
            ping2: PingSource,
        }

        impl crate::EventSource for DoubleSource {
            type Event = u32;
            type Metadata = ();
            type Ret = ();
            type Error = PingError;

            fn process_events<F>(
                &mut self,
                readiness: Readiness,
                token: Token,
                mut callback: F,
            ) -> Result<PostAction, Self::Error>
            where
                F: FnMut(Self::Event, &mut Self::Metadata) -> Self::Ret,
            {
                self.ping1
                    .process_events(readiness, token, |(), &mut ()| callback(1, &mut ()))?;
                self.ping2
                    .process_events(readiness, token, |(), &mut ()| callback(2, &mut ()))?;
                Ok(PostAction::Continue)
            }

            fn register(
                &mut self,
                poll: &mut Poll,
                token_factory: &mut TokenFactory,
            ) -> crate::Result<()> {
                self.ping1.register(poll, token_factory)?;
                self.ping2.register(poll, token_factory)?;
                Ok(())
            }

            fn reregister(
                &mut self,
                poll: &mut Poll,
                token_factory: &mut TokenFactory,
            ) -> crate::Result<()> {
                self.ping1.reregister(poll, token_factory)?;
                self.ping2.reregister(poll, token_factory)?;
                Ok(())
            }

            fn unregister(&mut self, poll: &mut Poll) -> crate::Result<()> {
                self.ping1.unregister(poll)?;
                self.ping2.unregister(poll)?;
                Ok(())
            }
        }

        let mut event_loop = EventLoop::<u32>::try_new().unwrap();

        let (ping1, source1) = make_ping().unwrap();
        let (ping2, source2) = make_ping().unwrap();

        let source = DoubleSource {
            ping1: source1,
            ping2: source2,
        };

        event_loop
            .handle()
            .insert_source(source, |i, _, d| {
                eprintln!("Dispatching {}", i);
                *d += i
            })
            .unwrap();

        let mut dispatched = 0;
        ping1.ping();
        event_loop
            .dispatch(Duration::ZERO, &mut dispatched)
            .unwrap();
        assert_eq!(dispatched, 1);

        dispatched = 0;
        ping2.ping();
        event_loop
            .dispatch(Duration::ZERO, &mut dispatched)
            .unwrap();
        assert_eq!(dispatched, 2);

        dispatched = 0;
        ping1.ping();
        ping2.ping();
        event_loop
            .dispatch(Duration::ZERO, &mut dispatched)
            .unwrap();
        assert_eq!(dispatched, 3);
    }

    #[cfg(unix)]
    #[test]
    fn change_interests() {
        use rustix::io::write;
        use rustix::net::{recv, socketpair, AddressFamily, RecvFlags, SocketFlags, SocketType};
        let mut event_loop = EventLoop::<bool>::try_new().unwrap();

        let (sock1, sock2) = socketpair(
            AddressFamily::UNIX,
            SocketType::STREAM,
            SocketFlags::empty(),
            None, // recv with DONTWAIT will suffice for platforms without SockFlag::SOCK_NONBLOCKING such as macOS
        )
        .unwrap();

        let source = Generic::new(sock1, Interest::READ, Mode::Level);
        let dispatcher = Dispatcher::new(source, |_, fd, dispatched| {
            *dispatched = true;
            // read all contents available to drain the socket
            let mut buf = [0u8; 32];
            loop {
                match recv(&*fd, &mut buf, RecvFlags::DONTWAIT) {
                    Ok((0, _)) => break, // closed pipe, we are now inert
                    Ok(_) => {}
                    Err(e) => {
                        let e: std::io::Error = e.into();
                        if e.kind() == std::io::ErrorKind::WouldBlock {
                            break;
                        // nothing more to read
                        } else {
                            // propagate error
                            return Err(e);
                        }
                    }
                }
            }
            Ok(PostAction::Continue)
        });

        let sock_token_1 = event_loop
            .handle()
            .register_dispatcher(dispatcher.clone())
            .unwrap();

        // first dispatch, nothing is readable
        let mut dispatched = false;
        event_loop
            .dispatch(Duration::ZERO, &mut dispatched)
            .unwrap();
        assert!(!dispatched);

        // write something, the socket becomes readable
        write(&sock2, &[1, 2, 3]).unwrap();
        dispatched = false;
        event_loop
            .dispatch(Duration::ZERO, &mut dispatched)
            .unwrap();
        assert!(dispatched);

        // All has been read, no longer readable
        dispatched = false;
        event_loop
            .dispatch(Duration::ZERO, &mut dispatched)
            .unwrap();
        assert!(!dispatched);

        // change the interests for writability instead
        dispatcher.as_source_mut().interest = Interest::WRITE;
        event_loop.handle().update(&sock_token_1).unwrap();

        // the socket is writable
        dispatched = false;
        event_loop
            .dispatch(Duration::ZERO, &mut dispatched)
            .unwrap();
        assert!(dispatched);

        // change back to readable
        dispatcher.as_source_mut().interest = Interest::READ;
        event_loop.handle().update(&sock_token_1).unwrap();

        // the socket is not readable
        dispatched = false;
        event_loop
            .dispatch(Duration::ZERO, &mut dispatched)
            .unwrap();
        assert!(!dispatched);
    }

    #[test]
    fn kill_source() {
        let mut event_loop = EventLoop::<Option<RegistrationToken>>::try_new().unwrap();

        let handle = event_loop.handle();
        let (ping, ping_source) = make_ping().unwrap();
        let ping_token = event_loop
            .handle()
            .insert_source(ping_source, move |(), &mut (), opt_src| {
                if let Some(src) = opt_src.take() {
                    handle.remove(src);
                }
            })
            .unwrap();

        ping.ping();

        let mut opt_src = Some(ping_token);

        event_loop.dispatch(Duration::ZERO, &mut opt_src).unwrap();

        assert!(opt_src.is_none());
    }

    #[test]
    fn non_static_data() {
        use std::sync::mpsc;

        let (sender, receiver) = mpsc::channel();

        {
            struct RefSender<'a>(&'a mpsc::Sender<()>);
            let mut ref_sender = RefSender(&sender);

            let mut event_loop = EventLoop::<RefSender<'_>>::try_new().unwrap();
            let (ping, ping_source) = make_ping().unwrap();
            let _ping_token = event_loop
                .handle()
                .insert_source(ping_source, |_, _, ref_sender| {
                    ref_sender.0.send(()).unwrap();
                })
                .unwrap();

            ping.ping();

            event_loop
                .dispatch(Duration::ZERO, &mut ref_sender)
                .unwrap();
        }

        receiver.recv().unwrap();
        // sender still usable (e.g. for another EventLoop)
        drop(sender);
    }

    #[cfg(feature = "block_on")]
    #[test]
    fn block_on_test() {
        use crate::sources::timer::TimeoutFuture;
        use std::time::Duration;

        let mut evl = EventLoop::<()>::try_new().unwrap();

        let mut data = 22;
        let timeout = {
            let data = &mut data;
            let evl_handle = evl.handle();

            async move {
                TimeoutFuture::from_duration(&evl_handle, Duration::from_secs(2)).await;
                *data = 32;
                11
            }
        };

        let result = evl.block_on(timeout, &mut (), |&mut ()| {}).unwrap();
        assert_eq!(result, Some(11));
        assert_eq!(data, 32);
    }

    #[cfg(feature = "block_on")]
    #[test]
    fn block_on_early_cancel() {
        use crate::sources::timer;
        use std::time::Duration;

        let mut evl = EventLoop::<()>::try_new().unwrap();

        let mut data = 22;
        let timeout = {
            let data = &mut data;
            let evl_handle = evl.handle();

            async move {
                timer::TimeoutFuture::from_duration(&evl_handle, Duration::from_secs(2)).await;
                *data = 32;
                11
            }
        };

        let timer_source = timer::Timer::from_duration(Duration::from_secs(1));
        let handle = evl.get_signal();
        let _timer_token = evl
            .handle()
            .insert_source(timer_source, move |_, _, _| {
                handle.stop();
                timer::TimeoutAction::Drop
            })
            .unwrap();

        let result = evl.block_on(timeout, &mut (), |&mut ()| {}).unwrap();
        assert_eq!(result, None);
        assert_eq!(data, 22);
    }

    #[test]
    fn reuse() {
        use crate::sources::timer;
        use std::sync::{Arc, Mutex};
        use std::time::{Duration, Instant};

        let mut evl = EventLoop::<RegistrationToken>::try_new().unwrap();
        let handle = evl.handle();

        let data = Arc::new(Mutex::new(1));
        let data_cloned = data.clone();

        let timer_source = timer::Timer::from_duration(Duration::from_secs(1));
        let mut first_timer_token = evl
            .handle()
            .insert_source(timer_source, move |_, _, own_token| {
                handle.remove(*own_token);
                let data_cloned = data_cloned.clone();
                let _ = handle.insert_source(timer::Timer::immediate(), move |_, _, _| {
                    *data_cloned.lock().unwrap() = 2;
                    timer::TimeoutAction::Drop
                });
                timer::TimeoutAction::Drop
            })
            .unwrap();

        let now = Instant::now();
        loop {
            evl.dispatch(Some(Duration::from_secs(3)), &mut first_timer_token)
                .unwrap();
            if Instant::now().duration_since(now) > Duration::from_secs(3) {
                break;
            }
        }

        assert_eq!(*data.lock().unwrap(), 2);
    }

    #[test]
    fn drop_of_subsource() {
        struct WithSubSource {
            token: Option<Token>,
        }

        impl crate::EventSource for WithSubSource {
            type Event = ();
            type Metadata = ();
            type Ret = ();
            type Error = crate::Error;
            const NEEDS_EXTRA_LIFECYCLE_EVENTS: bool = true;

            fn process_events<F>(
                &mut self,
                _: Readiness,
                _: Token,
                mut callback: F,
            ) -> Result<PostAction, Self::Error>
            where
                F: FnMut(Self::Event, &mut Self::Metadata) -> Self::Ret,
            {
                callback((), &mut ());
                // Drop the source
                Ok(PostAction::Remove)
            }

            fn register(&mut self, _: &mut Poll, fact: &mut TokenFactory) -> crate::Result<()> {
                // produce a few tokens to emulate a subsource
                fact.token();
                fact.token();
                self.token = Some(fact.token());
                Ok(())
            }

            fn reregister(&mut self, _: &mut Poll, _: &mut TokenFactory) -> crate::Result<()> {
                Ok(())
            }

            fn unregister(&mut self, _: &mut Poll) -> crate::Result<()> {
                Ok(())
            }

            // emulate a readiness
            fn before_sleep(&mut self) -> crate::Result<Option<(Readiness, Token)>> {
                Ok(self.token.map(|token| {
                    (
                        Readiness {
                            readable: true,
                            writable: false,
                            error: false,
                        },
                        token,
                    )
                }))
            }
        }

        // Now the actual test
        let mut evl = EventLoop::<bool>::try_new().unwrap();
        evl.handle()
            .insert_source(WithSubSource { token: None }, |_, _, ran| {
                *ran = true;
            })
            .unwrap();

        let mut ran = false;

        evl.dispatch(Some(Duration::ZERO), &mut ran).unwrap();

        assert!(ran);
    }

    // A dummy EventSource to test insertion and removal of sources
    struct DummySource;

    impl crate::EventSource for DummySource {
        type Event = ();
        type Metadata = ();
        type Ret = ();
        type Error = crate::Error;

        fn process_events<F>(
            &mut self,
            _: Readiness,
            _: Token,
            mut callback: F,
        ) -> Result<PostAction, Self::Error>
        where
            F: FnMut(Self::Event, &mut Self::Metadata) -> Self::Ret,
        {
            callback((), &mut ());
            Ok(PostAction::Continue)
        }

        fn register(&mut self, _: &mut Poll, _: &mut TokenFactory) -> crate::Result<()> {
            Ok(())
        }

        fn reregister(&mut self, _: &mut Poll, _: &mut TokenFactory) -> crate::Result<()> {
            Ok(())
        }

        fn unregister(&mut self, _: &mut Poll) -> crate::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn weak_loop_handle() {
        let mut event_loop: EventLoop<()> = EventLoop::try_new().unwrap();
        let weak_handle1 = event_loop.handle().downgrade();
        let weak_handle2 = weak_handle1.clone();
        let weak_handle3 = weak_handle1.clone();

        event_loop
            .handle()
            .insert_source(timer::Timer::immediate(), move |_, _, _| {
                // Hold and use its weak handle in the event loop's callback
                assert!(weak_handle1.upgrade().is_some());
                timer::TimeoutAction::Drop
            })
            .unwrap();

        event_loop.handle().insert_idle(move |_| {
            // Hold and use its weak handle in the event loop's idle callback
            assert!(weak_handle2.upgrade().is_some());
        });

        event_loop.dispatch(None, &mut ()).unwrap();

        drop(event_loop);

        // Test if memory is freed
        assert!(weak_handle3.expired());
    }
}
