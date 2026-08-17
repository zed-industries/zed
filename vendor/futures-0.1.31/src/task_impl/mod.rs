use core::fmt;
use core::marker::PhantomData;

use {Poll, Future, Stream, Sink, StartSend};

mod atomic_task;
pub use self::atomic_task::AtomicTask;

mod core;

#[cfg(feature = "use_std")]
mod std;
#[cfg(feature = "use_std")]
pub use self::std::*;
#[cfg(not(feature = "use_std"))]
pub use self::core::*;

pub struct BorrowedTask<'a> {
    id: usize,
    unpark: BorrowedUnpark<'a>,
    events: BorrowedEvents<'a>,
    // Task-local storage
    map: &'a LocalMap,
}

fn fresh_task_id() -> usize {
    use core::sync::atomic::{AtomicUsize, Ordering};
    #[allow(deprecated)]
    use core::sync::atomic::ATOMIC_USIZE_INIT;

    // TODO: this assert is a real bummer, need to figure out how to reuse
    //       old IDs that are no longer in use.
    //
    // Note, though, that it is intended that these ids go away entirely
    // eventually, see the comment on `is_current` below.
    #[allow(deprecated)]
    static NEXT_ID: AtomicUsize = ATOMIC_USIZE_INIT;
    let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
    assert!(id < usize::max_value() / 2,
            "too many previous tasks have been allocated");
    id
}

fn with<F: FnOnce(&BorrowedTask) -> R, R>(f: F) -> R {
    unsafe {
        let task = get_ptr().expect("no Task is currently running");
        assert!(!task.is_null(), "no Task is currently running");
        f(&*(task as *const BorrowedTask))
    }
}

/// A handle to a "task", which represents a single lightweight "thread" of
/// execution driving a future to completion.
///
/// In general, futures are composed into large units of work, which are then
/// spawned as tasks onto an *executor*. The executor is responsible for polling
/// the future as notifications arrive, until the future terminates.
///
/// This is obtained by the `task::current` function.
///
/// # FAQ
///
/// ### Why does `Task` not implement `Eq` and `Hash`?
///
/// A valid use case for `Task` to implement these two traits has not been
/// encountered.
///
/// Usually, this question is asked by someone who wants to store a `Task`
/// instance in a `HashSet`. This seems like an obvious way to implement a
/// future aware, multi-handle structure; e.g. a multi-producer channel.
///
/// In this case, the idea is that whenever a `start_send` is called on one of
/// the channel's send handles, if the channel is at capacity, the current task
/// is stored in a set. Then, when capacity is available, a task is removed from
/// the set and notified.
///
/// The problem with this strategy is that multiple `Sender` handles can be used
/// on the same task. In this case, when the second handle is used and the task
/// is stored in a set, there already is an entry. Then, when the first
/// handle is dropped, this entry is cleared, resulting in a dead lock.
///
/// See [here](https://github.com/rust-lang-nursery/futures-rs/issues/670) for
/// more discussion.
///
#[derive(Clone)]
pub struct Task {
    id: usize,
    unpark: TaskUnpark,
    events: UnparkEvents,
}

trait AssertSend: Send {}
impl AssertSend for Task {}

/// Returns a handle to the current task to call `notify` at a later date.
///
/// The returned handle implements the `Send` and `'static` bounds and may also
/// be cheaply cloned. This is useful for squirreling away the handle into a
/// location which is then later signaled that a future can make progress.
///
/// Implementations of the `Future` trait typically use this function if they
/// would otherwise perform a blocking operation. When something isn't ready
/// yet, this `current` function is called to acquire a handle to the current
/// task, and then the future arranges it such that when the blocking operation
/// otherwise finishes (perhaps in the background) it will `notify` the
/// returned handle.
///
/// It's sometimes necessary to pass extra information to the task when
/// unparking it, so that the task knows something about *why* it was woken.
/// See the `FutureQueue` documentation for details on how to do this.
///
/// # Panics
///
/// This function will panic if a task is not currently being executed. That
/// is, this method can be dangerous to call outside of an implementation of
/// `poll`.
pub fn current() -> Task {
    with(|borrowed| {
        let unpark = borrowed.unpark.to_owned();
        let events = borrowed.events.to_owned();

        Task {
            id: borrowed.id,
            unpark: unpark,
            events: events,
        }
    })
}

#[doc(hidden)]
#[deprecated(note = "renamed to `current`")]
pub fn park() -> Task {
    current()
}

impl Task {
    /// Indicate that the task should attempt to poll its future in a timely
    /// fashion.
    ///
    /// It's typically guaranteed that, after calling `notify`, `poll` will
    /// be called at least once subsequently (unless the future has terminated).
    /// If the task is currently polling its future when `notify` is called, it
    /// must poll the future *again* afterwards, ensuring that all relevant
    /// events are eventually observed by the future.
    pub fn notify(&self) {
        self.events.notify();
        self.unpark.notify();
    }

    #[doc(hidden)]
    #[deprecated(note = "renamed to `notify`")]
    pub fn unpark(&self) {
        self.notify()
    }

    /// Returns `true` when called from within the context of the task.
    ///
    /// In other words, the task is currently running on the thread calling the
    /// function. Note that this is currently, and has historically, been
    /// implemented by tracking an `id` on every instance of `Spawn` created.
    /// When a `Spawn` is being polled it stores in thread-local-storage the id
    /// of the instance, and then `task::current` will return a `Task` that also
    /// stores this id.
    ///
    /// The intention of this function was to answer questions like "if I
    /// `notify` this task, is it equivalent to `task::current().notify()`?"
    /// The answer "yes" may be able to avoid some extra work to block the
    /// current task, such as sending a task along a channel or updating a
    /// stored `Task` somewhere. An answer of "no" typically results in doing
    /// the work anyway.
    ///
    /// Unfortunately this function has been somewhat buggy in the past and is
    /// not intended to be supported in the future. By simply matching `id` the
    /// intended question above isn't accurately taking into account, for
    /// example, unpark events (now deprecated, but still a feature). Thus many
    /// old users of this API weren't fully accounting for the question it was
    /// intended they were asking.
    ///
    /// This API continues to be implemented but will in the future, e.g. in the
    /// 0.1.x series of this crate, eventually return `false` unconditionally.
    /// It is intended that this function will be removed in the next breaking
    /// change of this crate. If you'd like to continue to be able to answer the
    /// example question above, it's recommended you use the
    /// `will_notify_current` method.
    ///
    /// If you've got questions about this though please let us know! We'd like
    /// to learn about other use cases here that we did not consider.
    ///
    /// # Panics
    ///
    /// This function will panic if no current future is being polled.
    #[deprecated(note = "intended to be removed, see docs for details")]
    pub fn is_current(&self) -> bool {
        with(|current| current.id == self.id)
    }

    /// This function is intended as a performance optimization for structures
    /// which store a `Task` internally.
    ///
    /// The purpose of this function is to answer the question "if I `notify`
    /// this task is it equivalent to `task::current().notify()`". An answer
    /// "yes" may mean that you don't actually need to call `task::current()`
    /// and store it, but rather you can simply leave a stored task in place. An
    /// answer of "no" typically means that you need to call `task::current()`
    /// and store it somewhere.
    ///
    /// As this is purely a performance optimization a valid implementation for
    /// this function is to always return `false`. A best effort is done to
    /// return `true` where possible, but false negatives may happen. Note that
    /// this function will not return a false positive, however.
    ///
    /// # Panics
    ///
    /// This function will panic if no current future is being polled.
    #[allow(deprecated)]
    pub fn will_notify_current(&self) -> bool {
        with(|current| {
            self.unpark.will_notify(&current.unpark) &&
                self.events.will_notify(&current.events)
        })
    }
}

impl fmt::Debug for Task {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Task")
         .finish()
    }
}

/// Representation of a spawned future/stream.
///
/// This object is returned by the `spawn` function in this module. This
/// represents a "fused task and future", storing all necessary pieces of a task
/// and owning the top-level future that's being driven as well.
///
/// A `Spawn` can be poll'd for completion or execution of the current thread
/// can be blocked indefinitely until a notification arrives. This can be used
/// with either futures or streams, with different methods being available on
/// `Spawn` depending which is used.
pub struct Spawn<T: ?Sized> {
    id: usize,
    data: LocalMap,
    obj: T,
}

/// Spawns a future or stream, returning it and the new task responsible for
/// running it to completion.
///
/// This function is the termination endpoint for running futures. This method
/// will conceptually allocate a new task to run the given object, which is
/// normally either a `Future` or `Stream`.
///
/// This function is similar to the `thread::spawn` function but does not
/// attempt to run code in the background. The future will not make progress
/// until the methods on `Spawn` are called in turn.
pub fn spawn<T>(obj: T) -> Spawn<T> {
    Spawn {
        id: fresh_task_id(),
        obj: obj,
        data: local_map(),
    }
}

impl<T: ?Sized> Spawn<T> {
    /// Get a shared reference to the object the Spawn is wrapping.
    pub fn get_ref(&self) -> &T {
        &self.obj
    }

    /// Get a mutable reference to the object the Spawn is wrapping.
    pub fn get_mut(&mut self) -> &mut T {
        &mut self.obj
    }

    /// Consume the Spawn, returning its inner object
    pub fn into_inner(self) -> T where T: Sized {
        self.obj
    }

    /// Calls the provided closure, scheduling notifications to be sent to the
    /// `notify` argument.
    pub fn poll_fn_notify<N, F, R>(&mut self,
                             notify: &N,
                             id: usize,
                             f: F) -> R
        where F: FnOnce(&mut T) -> R,
              N: Clone + Into<NotifyHandle>,
    {
        let mk = || notify.clone().into();
        self.enter(BorrowedUnpark::new(&mk, id), f)
    }

    /// Polls the internal future, scheduling notifications to be sent to the
    /// `notify` argument.
    ///
    /// This method will poll the internal future, testing if it's completed
    /// yet. The `notify` argument is used as a sink for notifications sent to
    /// this future. That is, while the future is being polled, any call to
    /// `task::current()` will return a handle that contains the `notify`
    /// specified.
    ///
    /// If this function returns `NotReady`, then the `notify` should have been
    /// scheduled to receive a notification when poll can be called again.
    /// Otherwise if `Ready` or `Err` is returned, the `Spawn` task can be
    /// safely destroyed.
    ///
    /// Note that `notify` itself is passed as a shared reference, and is itself
    /// not required to be a `NotifyHandle`. The `Clone` and `Into` trait bounds
    /// will be used to convert this `notify` to a `NotifyHandle` if necessary.
    /// This construction can avoid an unnecessary atomic reference count bump
    /// in some situations.
    ///
    /// ## Unsafety and `id`
    ///
    /// This function and all other `*_notify` functions on this type will treat
    /// the `id` specified very carefully, explicitly calling functions like the
    /// `notify` argument's `clone_id` and `drop_id` functions. It should be
    /// safe to encode a pointer itself into the `id` specified, such as an
    /// `Arc<N>` or a `Box<N>`. The `clone_id` and `drop_id` functions are then
    /// intended to be sufficient for the memory management related to that
    /// pointer.
    pub fn poll_future_notify<N>(&mut self,
                                 notify: &N,
                                 id: usize) -> Poll<T::Item, T::Error>
        where N: Clone + Into<NotifyHandle>,
              T: Future,
    {
        self.poll_fn_notify(notify, id, |f| f.poll())
    }

    /// Like `poll_future_notify`, except polls the underlying stream.
    pub fn poll_stream_notify<N>(&mut self,
                                 notify: &N,
                                 id: usize)
                                 -> Poll<Option<T::Item>, T::Error>
        where N: Clone + Into<NotifyHandle>,
              T: Stream,
    {
        self.poll_fn_notify(notify, id, |s| s.poll())
    }

    /// Invokes the underlying `start_send` method with this task in place.
    ///
    /// If the underlying operation returns `NotReady` then the `notify` value
    /// passed in will receive a notification when the operation is ready to be
    /// attempted again.
    pub fn start_send_notify<N>(&mut self,
                                value: T::SinkItem,
                                notify: &N,
                                id: usize)
                               -> StartSend<T::SinkItem, T::SinkError>
        where N: Clone + Into<NotifyHandle>,
              T: Sink,
    {
        self.poll_fn_notify(notify, id, |s| s.start_send(value))
    }

    /// Invokes the underlying `poll_complete` method with this task in place.
    ///
    /// If the underlying operation returns `NotReady` then the `notify` value
    /// passed in will receive a notification when the operation is ready to be
    /// attempted again.
    pub fn poll_flush_notify<N>(&mut self,
                                notify: &N,
                                id: usize)
                                -> Poll<(), T::SinkError>
        where N: Clone + Into<NotifyHandle>,
              T: Sink,
    {
        self.poll_fn_notify(notify, id, |s| s.poll_complete())
    }

    /// Invokes the underlying `close` method with this task in place.
    ///
    /// If the underlying operation returns `NotReady` then the `notify` value
    /// passed in will receive a notification when the operation is ready to be
    /// attempted again.
    pub fn close_notify<N>(&mut self,
                           notify: &N,
                           id: usize)
                           -> Poll<(), T::SinkError>
        where N: Clone + Into<NotifyHandle>,
              T: Sink,
    {
        self.poll_fn_notify(notify, id, |s| s.close())
    }

    fn enter<F, R>(&mut self, unpark: BorrowedUnpark, f: F) -> R
        where F: FnOnce(&mut T) -> R
    {
        let borrowed = BorrowedTask {
            id: self.id,
            unpark: unpark,
            events: BorrowedEvents::new(),
            map: &self.data,
        };
        let obj = &mut self.obj;
        set(&borrowed, || f(obj))
    }
}

impl<T: fmt::Debug + ?Sized> fmt::Debug for Spawn<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Spawn")
         .field("obj", &&self.obj)
         .finish()
    }
}

/// A trait which represents a sink of notifications that a future is ready to
/// make progress.
///
/// This trait is provided as an argument to the `Spawn::*_notify` family of
/// functions. It's transitively used as part of the `Task::notify` method to
/// internally deliver notifications of readiness of a future to move forward.
///
/// An instance of `Notify` has one primary method, `notify`, which is given a
/// contextual argument as to what's being notified. This contextual argument is
/// *also* provided to the `Spawn::*_notify` family of functions and can be used
/// to reuse an instance of `Notify` across many futures.
///
/// Instances of `Notify` must be safe to share across threads, and the methods
/// be invoked concurrently. They must also live for the `'static` lifetime,
/// not containing any stack references.
pub trait Notify: Send + Sync {
    /// Indicates that an associated future and/or task are ready to make
    /// progress.
    ///
    /// Typically this means that the receiver of the notification should
    /// arrange for the future to get poll'd in a prompt fashion.
    ///
    /// This method takes an `id` as an argument which was transitively passed
    /// in from the original call to `Spawn::*_notify`. This id can be used to
    /// disambiguate which precise future became ready for polling.
    ///
    /// # Panics
    ///
    /// Since `unpark` may be invoked from arbitrary contexts, it should
    /// endeavor not to panic and to do as little work as possible. However, it
    /// is not guaranteed not to panic, and callers should be wary. If a panic
    /// occurs, that panic may or may not be propagated to the end-user of the
    /// future that you'd otherwise wake up.
    fn notify(&self, id: usize);

    /// This function is called whenever a new copy of `id` is needed.
    ///
    /// This is called in one of two situations:
    ///
    /// * A `Task` is being created through `task::current` while a future is
    ///   being polled. In that case the instance of `Notify` passed in to one
    ///   of the `poll_*` functions is called with the `id` passed into the same
    ///   `poll_*` function.
    /// * A `Task` is itself being cloned. Each `Task` contains its own id and a
    ///   handle to the `Notify` behind it, and the task's `Notify` is used to
    ///   clone the internal `id` to assign to the new task.
    ///
    /// The `id` returned here will be stored in the `Task`-to-be and used later
    /// to pass to `notify` when the `Task::notify` function is called on that
    /// `Task`.
    ///
    /// Note that typically this is just the identity function, passing through
    /// the identifier. For more unsafe situations, however, if `id` is itself a
    /// pointer of some kind this can be used as a hook to "clone" the pointer,
    /// depending on what that means for the specified pointer.
    fn clone_id(&self, id: usize) -> usize {
        id
    }

    /// All instances of `Task` store an `id` that they're going to internally
    /// notify with, and this function is called when the `Task` is dropped.
    ///
    /// This function provides a hook for schemes which encode pointers in this
    /// `id` argument to deallocate resources associated with the pointer. It's
    /// guaranteed that after this function is called the `Task` containing this
    /// `id` will no longer use the `id`.
    fn drop_id(&self, id: usize) {
        drop(id);
    }
}

/// Sets the `NotifyHandle` of the current task for the duration of the provided
/// closure.
///
/// This function takes a type that can be converted into a notify handle,
/// `notify` and `id`, and a closure `f`. The closure `f` will be executed such
/// that calls to `task::current()` will store a reference to the notify handle
/// provided, not the one previously in the environment.
///
/// Note that calls to `task::current()` in the closure provided *will not* be
/// equivalent to `task::current()` before this method is called. The two tasks
/// returned will notify different handles, and the task handles pulled out
/// during the duration of this closure will not notify the previous task. It's
/// recommended that you call `task::current()` in some capacity before calling
/// this function to ensure that calls to `task::current()` inside of this
/// closure can transitively wake up the outer task.
///
/// # Panics
///
/// This function will panic if it is called outside the context of a future's
/// task. This is only valid to call once you've already entered a future via
/// `Spawn::poll_*` functions.
pub fn with_notify<F, T, R>(notify: &T, id: usize, f: F) -> R
    where F: FnOnce() -> R,
          T: Clone + Into<NotifyHandle>,
{
    with(|task| {
        let mk = || notify.clone().into();
        let new_task = BorrowedTask {
            id: task.id,
            unpark: BorrowedUnpark::new(&mk, id),
            events: task.events,
            map: task.map,
        };

        set(&new_task, f)
    })
}

/// An unsafe trait for implementing custom forms of memory management behind a
/// `Task`.
///
/// The `futures` critically relies on "notification handles" to extract for
/// futures to contain and then later inform that they're ready to make
/// progress. These handles, however, must be cheap to create and cheap
/// to clone to ensure that this operation is efficient throughout the
/// execution of a program.
///
/// Typically this sort of memory management is done in the standard library
/// with the `Arc` type. An `Arc` is relatively cheap to allocate an is
/// quite cheap to clone and pass around. Plus, it's 100% safe!
///
/// When working outside the standard library, however, you don't always have
/// and `Arc` type available to you. This trait, `UnsafeNotify`, is intended
/// to be the "unsafe version" of the `Notify` trait. This trait encodes the
/// memory management operations of a `Task`'s notification handle, allowing
/// custom implementations for the memory management of a notification handle.
///
/// Put another way, the core notification type in this library,
/// `NotifyHandle`, simply internally contains an instance of
/// `*mut UnsafeNotify`. This "unsafe trait object" is then used exclusively
/// to operate with, dynamically dispatching calls to clone, drop, and notify.
/// Critically though as a raw pointer it doesn't require a particular form
/// of memory management, allowing external implementations.
///
/// A default implementation of the `UnsafeNotify` trait is provided for the
/// `Arc` type in the standard library. If the `use_std` feature of this crate
/// is not available however, you'll be required to implement your own
/// instance of this trait to pass it into `NotifyHandle::new`.
///
/// # Unsafety
///
/// This trait is manually encoding the memory management of the underlying
/// handle, and as a result is quite unsafe to implement! Implementors of
/// this trait must guarantee:
///
/// * Calls to `clone_raw` produce uniquely owned handles. It should be safe
///   to drop the current handle and have the returned handle still be valid.
/// * Calls to `drop_raw` work with `self` as a raw pointer, deallocating
///   resources associated with it. This is a pretty unsafe operation as it's
///   invalidating the `self` pointer, so extreme care needs to be taken.
///
/// In general it's recommended to review the trait documentation as well as
/// the implementation for `Arc` in this crate. When in doubt ping the
/// `futures` authors to clarify an unsafety question here.
pub unsafe trait UnsafeNotify: Notify {
    /// Creates a new `NotifyHandle` from this instance of `UnsafeNotify`.
    ///
    /// This function will create a new uniquely owned handle that under the
    /// hood references the same notification instance. In other words calls
    /// to `notify` on the returned handle should be equivalent to calls to
    /// `notify` on this handle.
    ///
    /// # Unsafety
    ///
    /// This trait is unsafe to implement, as are all these methods. This
    /// method is also unsafe to call as it's asserting the `UnsafeNotify`
    /// value is in a consistent state. In general it's recommended to
    /// review the trait documentation as well as the implementation for `Arc`
    /// in this crate. When in doubt ping the `futures` authors to clarify
    /// an unsafety question here.
    unsafe fn clone_raw(&self) -> NotifyHandle;

    /// Drops this instance of `UnsafeNotify`, deallocating resources
    /// associated with it.
    ///
    /// This method is intended to have a signature such as:
    ///
    /// ```ignore
    /// fn drop_raw(self: *mut Self);
    /// ```
    ///
    /// Unfortunately in Rust today that signature is not object safe.
    /// Nevertheless it's recommended to implement this function *as if* that
    /// were its signature. As such it is not safe to call on an invalid
    /// pointer, nor is the validity of the pointer guaranteed after this
    /// function returns.
    ///
    /// # Unsafety
    ///
    /// This trait is unsafe to implement, as are all these methods. This
    /// method is also unsafe to call as it's asserting the `UnsafeNotify`
    /// value is in a consistent state. In general it's recommended to
    /// review the trait documentation as well as the implementation for `Arc`
    /// in this crate. When in doubt ping the `futures` authors to clarify
    /// an unsafety question here.
    unsafe fn drop_raw(&self);
}

/// A `NotifyHandle` is the core value through which notifications are routed
/// in the `futures` crate.
///
/// All instances of `Task` will contain a `NotifyHandle` handle internally.
/// This handle itself contains a trait object pointing to an instance of the
/// `Notify` trait, allowing notifications to get routed through it.
///
/// The `NotifyHandle` type internally does not codify any particular memory
/// management strategy. Internally it contains an instance of `*mut
/// UnsafeNotify`, and more details about that trait can be found on its own
/// documentation. Consequently, though, the one constructor of this type,
/// `NotifyHandle::new`, is `unsafe` to call. It is not recommended to call
/// this constructor directly.
///
/// If you're working with the standard library then it's recommended to
/// work with the `Arc` type. If you have a struct, `T`, which implements the
/// `Notify` trait, then you can construct this with
/// `NotifyHandle::from(t: Arc<T>)`. The coercion to `UnsafeNotify` will
/// happen automatically and safely for you.
///
/// When working externally from the standard library it's recommended to
/// provide a similar safe constructor for your custom type as opposed to
/// recommending an invocation of `NotifyHandle::new` directly.
pub struct NotifyHandle {
    inner: *mut UnsafeNotify,
}

unsafe impl Send for NotifyHandle {}
unsafe impl Sync for NotifyHandle {}

impl NotifyHandle {
    /// Constructs a new `NotifyHandle` directly.
    ///
    /// Note that most code will not need to call this. Implementers of the
    /// `UnsafeNotify` trait will typically provide a wrapper that calls this
    /// but you otherwise shouldn't call it directly.
    ///
    /// If you're working with the standard library then it's recommended to
    /// use the `NotifyHandle::from` function instead which works with the safe
    /// `Arc` type and the safe `Notify` trait.
    #[inline]
    pub unsafe fn new(inner: *mut UnsafeNotify) -> NotifyHandle {
        NotifyHandle { inner: inner }
    }

    /// Invokes the underlying instance of `Notify` with the provided `id`.
    pub fn notify(&self, id: usize) {
        unsafe { (*self.inner).notify(id) }
    }

    fn clone_id(&self, id: usize) -> usize {
        unsafe { (*self.inner).clone_id(id) }
    }

    fn drop_id(&self, id: usize) {
        unsafe { (*self.inner).drop_id(id) }
    }
}

impl Clone for NotifyHandle {
    #[inline]
    fn clone(&self) -> Self {
        unsafe {
            (*self.inner).clone_raw()
        }
    }
}

impl fmt::Debug for NotifyHandle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("NotifyHandle")
         .finish()
    }
}

impl Drop for NotifyHandle {
    fn drop(&mut self) {
        unsafe {
            (*self.inner).drop_raw()
        }
    }
}

/// Marker for a `T` that is behind &'static.
struct StaticRef<T>(PhantomData<T>);

impl<T: Notify> Notify for StaticRef<T> {
    fn notify(&self, id: usize) {
        let me = unsafe { &*(self as *const _ as *const T) };
        me.notify(id);
    }

    fn clone_id(&self, id: usize) -> usize {
        let me = unsafe { &*(self as *const _ as *const T) };
        me.clone_id(id)
    }

    fn drop_id(&self, id: usize) {
        let me = unsafe { &*(self as *const _ as *const T) };
        me.drop_id(id);
    }
}

unsafe impl<T: Notify + 'static> UnsafeNotify for StaticRef<T> {
    unsafe fn clone_raw(&self) -> NotifyHandle {
        NotifyHandle::new(self as *const _ as *mut StaticRef<T>)
    }

    unsafe fn drop_raw(&self) {}
}

impl<T: Notify> From<&'static T> for NotifyHandle {
    fn from(src : &'static T) -> NotifyHandle {
        unsafe { NotifyHandle::new(src as *const _ as *mut StaticRef<T>) }
    }
}

#[cfg(feature = "nightly")]
mod nightly {
    use super::NotifyHandle;
    use core::marker::Unpin;

    impl Unpin for NotifyHandle {}
}
