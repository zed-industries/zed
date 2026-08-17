#![allow(dead_code)] // not all helpers are used for all targets

use std::{
    collections::HashSet,
    fmt::Debug,
    ops::Deref,
    path::{Path, PathBuf},
    sync::mpsc::{self},
    thread,
    time::{Duration, Instant},
};

use notify_types::event::Event;
use walkdir::WalkDir;

use crate::{
    Config, Error, ErrorKind, PollWatcher, RecommendedWatcher, RecursiveMode, Watcher, WatcherKind,
};
use pretty_assertions::assert_eq;

pub use expect::*;

/// Waits any events from the watcher and provides with some helper methods
pub struct Receiver {
    pub rx: mpsc::Receiver<Result<Event, Error>>,
    pub timeout: Duration,
    pub detect_changes: Option<Box<dyn Fn()>>,
    pub kind: WatcherKind,
}

#[derive(Debug)]
pub enum TryRecvError {
    Mpsc(mpsc::RecvTimeoutError),
    Watcher(Error),
}

impl Receiver {
    const DEFAULT_TIMEOUT: Duration = Duration::from_secs(1);

    fn wait_expected<C: ExpectedEvents>(&mut self, mut state: ExpectedState<C>) -> WaitState {
        self.detect_changes();
        let mut trackers = Trackers::default();
        while !state.is_empty() {
            match self.try_recv() {
                Ok(res) => match res {
                    Ok(event) => {
                        trackers.try_push(&event);
                        state.check(event)
                    }
                    Err(err) => {
                        let is_bad_file_descriptor = self.kind == WatcherKind::PollWatcher
                            && matches!(
                                &err.kind,
                                ErrorKind::Io(io_err)
                                    if io_err
                                        .to_string()
                                        .contains("Bad file descriptor (os error 9)")
                            );

                        if is_bad_file_descriptor {
                            continue;
                        }

                        panic!(
                            "Got an error from the watcher {:?}: {err:?}. State: {state:#?}",
                            self.kind
                        )
                    }
                },
                Err(e) => panic!(
                    "Recv error: {e:?}. Watcher: {:?}. State: {state:#?}",
                    self.kind
                ),
            }
        }

        let remain = self.rx.try_iter().collect::<Vec<_>>();

        WaitState {
            trackers,
            received: state.into_received(),
            remain,
        }
    }

    /// Waits for the events in the same order as they provided and fails on an unexpected one.
    pub fn wait_ordered_exact(
        &mut self,
        expected: impl IntoIterator<Item = ExpectedEvent>,
    ) -> WaitState {
        self.wait_expected(ExpectedState::ordered(expected).disallow_unexpected())
    }

    /// Waits for the events in the same order as they provided and ignores unexpected ones.
    pub fn wait_ordered(&mut self, expected: impl IntoIterator<Item = ExpectedEvent>) -> WaitState {
        self.wait_expected(ExpectedState::ordered(expected).allow_unexpected())
    }

    /// Waits for the events in any order and fails on an unexpected one.
    pub fn wait_unordered_exact(
        &mut self,
        expected: impl IntoIterator<Item = ExpectedEvent>,
    ) -> WaitState {
        self.wait_expected(ExpectedState::unordered(expected).disallow_unexpected())
    }

    /// Waits for the events in any order and ignores unexpected ones.
    pub fn wait_unordered(
        &mut self,
        expected: impl IntoIterator<Item = ExpectedEvent>,
    ) -> WaitState {
        self.wait_expected(ExpectedState::unordered(expected).allow_unexpected())
    }

    pub fn try_recv(&mut self) -> Result<Result<Event, Error>, mpsc::RecvTimeoutError> {
        self.rx.recv_timeout(self.timeout)
    }

    pub fn recv(&mut self) -> Event {
        self.recv_result()
            .unwrap_or_else(|e| panic!("Unexpected error from the watcher {:?}: {e:?}", self.kind))
    }

    pub fn recv_result(&mut self) -> Result<Event, Error> {
        self.try_recv().unwrap_or_else(|e| match e {
            mpsc::RecvTimeoutError::Timeout => panic!("Unable to wait the next event from the watcher {:?}: timeout", self.kind),
            mpsc::RecvTimeoutError::Disconnected => {
                panic!("Unable to wait the next event: the watcher {:?} part of the channel was disconnected", self.kind)
            }
        })
    }

    /// Detects changes. It is useful for [`PollWatcher`]
    pub fn detect_changes(&self) {
        if let Some(detect_changes) = self.detect_changes.as_deref() {
            detect_changes()
        }
    }

    /// Returns an iterator iterating by events
    ///
    /// It doesn't fail on timeout, instead it returns None
    ///
    /// This behaviour is better for tests, because allows us to determine which events was received
    pub fn iter(&mut self) -> impl Iterator<Item = Event> + '_ {
        struct Iter<'a> {
            rx: &'a mut Receiver,
        }

        impl Iterator for Iter<'_> {
            type Item = Event;

            fn next(&mut self) -> Option<Self::Item> {
                self.rx
                    .try_recv()
                    .ok()
                    .map(|res| res.unwrap_or_else(|err| panic!("Got an error: {err:#?}")))
            }
        }

        Iter { rx: self }
    }

    /// Ensures, that the receiver part is empty. It doesn't wait anything, just check the channel
    pub fn ensure_empty(&mut self) {
        if let Ok(event) = self.rx.try_recv() {
            panic!("Unexpected event was received: {event:#?}")
        }
    }

    /// see [`sleep_until`].
    ///
    /// it uses timeout from [`Self::timeout`]
    #[must_use]
    pub fn sleep_until<F: FnMut() -> bool>(&self, check: F) -> bool {
        sleep_until(check, self.timeout)
    }

    pub fn sleep_until_walkdir_returns_set<Q>(
        &self,
        root: impl AsRef<Path>,
        paths: impl IntoIterator<Item = Q>,
    ) where
        Q: AsRef<Path>,
    {
        let mut actual = HashSet::new();
        let expected: HashSet<PathBuf> = paths
            .into_iter()
            .map(|p| p.as_ref().to_path_buf())
            .collect();
        let walkdir_set_eq = self.sleep_until(|| {
            actual.clear();
            WalkDir::new(root.as_ref())
                .follow_links(true)
                .min_depth(1)
                .into_iter()
                .filter_map(|v| v.ok())
                .map(|v| v.into_path())
                .for_each(|path| {
                    actual.insert(path);
                });

            actual == expected
        });

        assert!(
            walkdir_set_eq,
            "Walkdir returns unexpected result: {actual:?}"
        )
    }

    pub fn sleep_until_exists(&self, path: impl AsRef<Path>) {
        let path = path.as_ref();
        assert!(
            self.sleep_until(|| path.exists()),
            "the fs entry {path:?} has still not been exist after timeout {:?}",
            self.timeout
        )
    }

    pub fn sleep_while_exists(&self, path: impl AsRef<Path>) {
        let path = path.as_ref();
        assert!(
            self.sleep_until(|| !path.exists()),
            "the fs entry {path:?} has been exist yet after timeout {:?}",
            self.timeout
        )
    }

    /// Sleeps, while the parent directory of the provided path
    /// does not contain the provided path in the [`std::fs::read_dir`] result.
    ///
    /// Errors will be ignored
    ///
    /// It is useful for the [`PollWatcher`], because on some file systems the directory
    /// may contain a DirEntry after deletion
    pub fn sleep_until_parent_contains(&self, path: impl AsRef<Path>) {
        let path = path.as_ref();
        let parent = path
            .parent()
            .expect("The path {path:?} does not have a parent");

        assert!(
            self.sleep_until(|| {
                std::fs::read_dir(parent)
                    .into_iter()
                    .flatten()
                    .flatten()
                    .any(|r| r.path() == path)
            }),
            "the path {parent:?} has not contained an expected entry {:?} after timeout {:?}",
            path.file_name(),
            self.timeout
        )
    }

    /// Sleeps, while the parent directory of the provided path
    /// contains the provided path in the [`std::fs::read_dir`] result.
    ///
    /// Errors will be ignored
    ///
    /// It is useful for the [`PollWatcher`], because on some file systems the directory
    /// may contain a DirEntry after deletion
    pub fn sleep_while_parent_contains(&self, path: impl AsRef<Path>) {
        let path = path.as_ref();
        let parent = path
            .parent()
            .expect("The path {path:?} does not have a parent");

        assert!(
            self.sleep_until(|| {
                !std::fs::read_dir(parent)
                    .into_iter()
                    .flatten()
                    .flatten()
                    .any(|r| r.path() == path)
            }),
            "the path {parent:?} has contained an expected entry {:?} yet after timeout {:?}",
            path.file_name(),
            self.timeout
        )
    }
}

/// Result of a `wait` call on a [`Receiver`]
#[derive(Debug)]
pub struct WaitState {
    received: Vec<Event>,
    remain: Vec<Result<Event, Error>>,
    trackers: Trackers,
}

impl WaitState {
    /// Ensure the received trackers count is equal to the provided one
    pub fn ensure_trackers_len(self, len: usize) -> Self {
        assert_eq!(
            self.trackers.len(),
            len,
            "Unexpected cookies len. events: {:#?}",
            self.received
        );
        self
    }

    /// Ensure there is no one event after expected
    pub fn ensure_no_tail(self) -> Self {
        assert!(
            self.remain.is_empty(),
            "Unexpected events from the watcher: unexpected: {:#?}",
            self.remain
        );
        self
    }
}

#[derive(Debug)]
pub struct ChannelConfig {
    timeout: Duration,
    watcher_config: Config,
}

impl ChannelConfig {
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn with_watcher_config(mut self, config: Config) -> Self {
        self.watcher_config = config;
        self
    }
}

impl Default for ChannelConfig {
    fn default() -> Self {
        Self {
            timeout: Receiver::DEFAULT_TIMEOUT,
            // Use Config::default() which uses EventKindMask::ALL,
            // ensuring tests cover all event types including Access events
            watcher_config: Config::default(),
        }
    }
}

/// Simplify [`Watcher`] methods call: unwrap them and etc.
pub struct TestWatcher<W> {
    pub watcher: W,
    pub kind: WatcherKind,
}

impl<W: Watcher> TestWatcher<W> {
    pub fn watch_recursively(&mut self, path: impl AsRef<Path>) {
        self.watch(path, RecursiveMode::Recursive);
    }

    pub fn watch_nonrecursively(&mut self, path: impl AsRef<Path>) {
        self.watch(path, RecursiveMode::NonRecursive);
    }

    pub fn watch(&mut self, path: impl AsRef<Path>, recursive_mode: RecursiveMode) {
        let path = path.as_ref();

        const FSEVENT_WATCH_RETRIES: usize = 5;
        const FSEVENT_WATCH_RETRY_BASE_DELAY: Duration = Duration::from_millis(50);
        for attempt in 0..=FSEVENT_WATCH_RETRIES {
            match self.watcher.watch(path, recursive_mode) {
                Ok(()) => return,
                Err(err) => {
                    let is_transient_fsevent_start_error = self.kind == WatcherKind::Fsevent
                        && matches!(
                            &err.kind,
                            ErrorKind::Generic(message)
                            if message == "unable to start FSEvent stream"
                        );

                    if is_transient_fsevent_start_error && attempt < FSEVENT_WATCH_RETRIES {
                        let _ = self.watcher.unwatch(path);
                        let delay_factor = 1u32 << attempt;
                        thread::sleep(FSEVENT_WATCH_RETRY_BASE_DELAY * delay_factor);
                        continue;
                    }

                    panic!("Unable to watch {path:?}: {err:#?}")
                }
            }
        }

        unreachable!("watch() retries must return or panic")
    }
}

/// Calls the provided closure
/// * If it returned true, returns true
/// * If it returned false, waits for a short period and tries again
/// * If the long timeout was reached, returns false
///
/// It's useful for the [`PollWatcher`] due to race conditions between
/// file system and changes detector - sometimes we can encounter an error while scanning fs,
/// and it's hard to deal with.
#[must_use]
pub fn sleep_until<F: FnMut() -> bool>(mut check: F, timeout: Duration) -> bool {
    let start = Instant::now();
    loop {
        if Instant::now()
            .checked_duration_since(start)
            .is_some_and(|since| since > timeout)
        {
            return false;
        }

        if check() {
            return true;
        }

        thread::sleep(Duration::from_millis(10));
    }
}

/// Creates a [`TestWatcher`] and connected [`Receiver`]
pub fn channel_with_config<W: Watcher>(config: ChannelConfig) -> (TestWatcher<W>, Receiver) {
    let (tx, rx) = mpsc::channel();
    let watcher = W::new(tx, config.watcher_config).expect("Unable to create a watcher");
    (
        TestWatcher {
            watcher,
            kind: W::kind(),
        },
        Receiver {
            rx,
            timeout: config.timeout,
            detect_changes: None,
            kind: W::kind(),
        },
    )
}

/// Creates a [`TestWatcher`] and connected [`Receiver`]
pub fn channel<W: Watcher>() -> (TestWatcher<W>, Receiver) {
    channel_with_config(Default::default())
}

/// Creates a [`TestWatcher`] for the [`RecommendedWatcher`] and connected [`Receiver`]
pub fn recommended_channel() -> (TestWatcher<RecommendedWatcher>, Receiver) {
    channel()
}

/// Creates a [`PollWatcher`] with `with_compare_contents(true)` and manual polling.
///
/// Returned [`Receiver`] will send a message to poll changes before wait-methods
pub fn poll_watcher_channel() -> (TestWatcher<PollWatcher>, Receiver) {
    let (tx, rx) = mpsc::channel();
    let watcher = PollWatcher::new(
        tx,
        Config::default()
            .with_compare_contents(true)
            .with_manual_polling(),
    )
    .expect("Unable to create PollWatcher");
    let sender = watcher.poll_sender();
    let watcher = TestWatcher {
        watcher,
        kind: PollWatcher::kind(),
    };
    let rx = Receiver {
        rx,
        timeout: Receiver::DEFAULT_TIMEOUT,
        detect_changes: Some(Box::new(move || {
            sender
                .send(crate::poll::PollMessage::Poll)
                .expect("PollWatcher receiver part was disconnected")
        })),
        kind: watcher.kind,
    };

    (watcher, rx)
}

/// This is a canonicalized path due to macos behaviour - it creates
/// a dir with path '/var/...' but actually it is '/private/var/...'
///
/// FsEventWatcher uses canonicalized paths
/// and send events with canonicalized paths, tho we need it converted to compare with expected
pub struct TestDir {
    _dir: tempfile::TempDir,

    path: PathBuf,
}

impl TestDir {
    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl AsRef<Path> for TestDir {
    fn as_ref(&self) -> &Path {
        self.path()
    }
}

/// Creates a [`TestDir`]
pub fn testdir() -> TestDir {
    let dir = tempfile::tempdir().expect("Unable to create tempdir");
    let path = std::fs::canonicalize(dir.path()).unwrap_or_else(|e| {
        panic!(
            "unable to canonicalize tempdir path {:?}: {e:?}",
            dir.path()
        )
    });
    TestDir { _dir: dir, path }
}

/// Collection to store [`notify_types::event::EventAttributes::tracker`]
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Trackers(Vec<usize>);

impl Trackers {
    /// Pushes new tracker if the provided event has some and it is not equal to the last one
    pub fn try_push(&mut self, event: &Event) -> bool {
        let Some(tracker) = event.attrs.tracker() else {
            return false;
        };

        if self.0.last() != Some(&tracker) {
            self.0.push(tracker);
            true
        } else {
            false
        }
    }
}

impl Deref for Trackers {
    type Target = [usize];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

mod expect {
    use std::{
        collections::VecDeque,
        fmt::Debug,
        path::{Path, PathBuf},
    };

    use notify_types::event::{
        AccessKind, AccessMode, CreateKind, DataChange, Event, EventKind, MetadataKind, ModifyKind,
        RemoveKind, RenameMode,
    };

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum UnexpectedEventBehaviour {
        Ignore,
        Panic,
    }

    /// Helper to check events and them sort order
    #[derive(Debug)]
    pub struct ExpectedState<C> {
        pub remain: C,
        pub received: Vec<Event>,
        pub unexpected_event_behaviour: UnexpectedEventBehaviour,
        /// If it is Some, then any next equal events are acceptable
        /// while they are equal the stored ones. Ones mismatch clears the field
        multiple_event: Option<Event>,
    }

    impl ExpectedState<Ordered> {
        pub fn ordered(iter: impl IntoIterator<Item = ExpectedEvent>) -> Self {
            Self::new(iter)
        }
    }

    impl ExpectedState<Unordered> {
        pub fn unordered(iter: impl IntoIterator<Item = ExpectedEvent>) -> Self {
            Self::new(iter)
        }
    }

    impl<C: ExpectedEvents + Debug> ExpectedState<C> {
        pub fn new(iter: impl IntoIterator<Item = ExpectedEvent>) -> Self {
            Self {
                remain: iter.into_iter().collect(),
                received: Default::default(),
                unexpected_event_behaviour: UnexpectedEventBehaviour::Ignore,
                multiple_event: None,
            }
        }

        pub fn allow_unexpected(mut self) -> Self {
            self.unexpected_event_behaviour = UnexpectedEventBehaviour::Ignore;
            self
        }

        pub fn disallow_unexpected(mut self) -> Self {
            self.unexpected_event_behaviour = UnexpectedEventBehaviour::Panic;
            self
        }

        pub fn is_empty(&self) -> bool {
            self.remain.is_empty()
        }

        pub fn check(&mut self, event: Event) {
            let expected = self.remain.expected(&event);

            if let Some(expected) = &expected {
                self.multiple_event = expected.is_multiple().then(|| event.clone());
            }

            self.received.push(event.clone());

            if let Some(multiple_event) = &self.multiple_event {
                if multiple_event.kind == event.kind && multiple_event.paths == event.paths {
                    return;
                }
            }

            if expected.is_none()
                && self.unexpected_event_behaviour == UnexpectedEventBehaviour::Panic
            {
                panic!("Unexpected event. State: {self:#?}")
            }
        }

        pub fn into_received(self) -> Vec<Event> {
            self.received
        }
    }

    pub trait ExpectedEvents: Debug + FromIterator<ExpectedEvent> {
        fn is_empty(&self) -> bool;

        /// Returns
        /// * None if the event is unexpected
        /// * Some(expected) if the event was matched with the expected one
        fn expected(&mut self, event: &Event) -> Option<ExpectedEvent>;
    }

    /// Collection of [`ExpectedEvent`]s that allows events to be received in any order
    ///
    /// Stores original indexes for events to debug purposes
    #[derive(Debug)]
    pub struct Unordered(Vec<(usize, ExpectedEvent)>);

    impl ExpectedEvents for Unordered {
        fn is_empty(&self) -> bool {
            self.0.is_empty()
        }

        fn expected(&mut self, event: &Event) -> Option<ExpectedEvent> {
            let found_idx = self
                .0
                .iter()
                .enumerate()
                .find(|(_, (_, expected))| expected == event)
                .map(|(idx, _)| idx);
            match found_idx {
                Some(found_idx) => {
                    let (_, expected) = self.0.swap_remove(found_idx);
                    Some(expected)
                }
                None => None,
            }
        }
    }

    /// Collection of [`ExpectedEvent`]s that allows events to be received in the specified order
    #[derive(Debug)]
    pub struct Ordered(VecDeque<ExpectedEvent>);

    impl ExpectedEvents for Ordered {
        fn is_empty(&self) -> bool {
            self.0.is_empty()
        }

        fn expected(&mut self, event: &Event) -> Option<ExpectedEvent> {
            loop {
                match self.0.front() {
                    Some(expected) => {
                        if expected == event {
                            break self.0.pop_front();
                        } else if expected.is_optional() {
                            self.0.pop_front();
                        } else {
                            break None;
                        }
                    }
                    None => break None,
                }
            }
        }
    }

    impl FromIterator<ExpectedEvent> for Unordered {
        fn from_iter<T: IntoIterator<Item = ExpectedEvent>>(iter: T) -> Self {
            Self(iter.into_iter().enumerate().collect())
        }
    }

    impl FromIterator<ExpectedEvent> for Ordered {
        fn from_iter<T: IntoIterator<Item = ExpectedEvent>>(iter: T) -> Self {
            Self(iter.into_iter().collect())
        }
    }

    /// Creates an [`ExpectedEvent`] with the provided paths
    pub fn expected(path: impl ExpectedPath) -> ExpectedEvent {
        let mut event = ExpectedEvent::default();
        path.add_to_event(&mut event);
        event
    }

    /// A helper trait to allow us to pass [`Path`] / [`PathBuf`] or array of them to [`expected`]
    pub trait ExpectedPath {
        fn add_to_event(self, event: &mut ExpectedEvent);
    }

    impl ExpectedPath for &Path {
        fn add_to_event(self, event: &mut ExpectedEvent) {
            event.push_path(self.to_path_buf());
        }
    }

    impl ExpectedPath for &PathBuf {
        fn add_to_event(self, event: &mut ExpectedEvent) {
            event.push_path(self.to_path_buf());
        }
    }

    impl ExpectedPath for PathBuf {
        fn add_to_event(self, event: &mut ExpectedEvent) {
            event.push_path(self);
        }
    }

    impl<const C: usize> ExpectedPath for [PathBuf; C] {
        fn add_to_event(self, event: &mut ExpectedEvent) {
            for path in self {
                path.add_to_event(event);
            }
        }
    }

    impl<const C: usize> ExpectedPath for [&PathBuf; C] {
        fn add_to_event(self, event: &mut ExpectedEvent) {
            for path in self {
                path.add_to_event(event);
            }
        }
    }
    impl<const C: usize> ExpectedPath for [&Path; C] {
        fn add_to_event(self, event: &mut ExpectedEvent) {
            for path in self {
                path.add_to_event(event);
            }
        }
    }

    /// Predicate to accept or refuse an event
    ///
    /// We need it, because sometimes we should check an unspecified kind / paths
    ///
    /// It implements `PartialEq<Event>`
    #[derive(Debug, Default, Clone)]
    pub struct ExpectedEvent {
        kind: Option<ExpectedEventKind>,
        paths: Option<Vec<PathBuf>>,
        tracker: Option<Option<usize>>,
        multiple: bool,
        optional: bool,
    }

    #[derive(Debug, Clone, Copy)]
    enum ExpectedEventKind {
        Any,
        Access(Option<ExpectedAccessKind>),
        Create(Option<CreateKind>),
        Modify(Option<ExpectedModifyKind>),
        Remove(Option<RemoveKind>),
        Other,
    }

    impl PartialEq<EventKind> for ExpectedEventKind {
        fn eq(&self, other: &EventKind) -> bool {
            match self {
                ExpectedEventKind::Any => matches!(other, EventKind::Any),
                ExpectedEventKind::Access(expected) => {
                    let EventKind::Access(other) = other else {
                        return false;
                    };
                    expected.is_none_or(|expected| &expected == other)
                }
                ExpectedEventKind::Create(expected) => {
                    let EventKind::Create(other) = other else {
                        return false;
                    };
                    expected.is_none_or(|expected| &expected == other)
                }
                ExpectedEventKind::Modify(expected) => {
                    let EventKind::Modify(other) = other else {
                        return false;
                    };
                    expected.is_none_or(|expected| &expected == other)
                }
                ExpectedEventKind::Remove(expected) => {
                    let EventKind::Remove(other) = other else {
                        return false;
                    };
                    expected.is_none_or(|expected| &expected == other)
                }
                ExpectedEventKind::Other => matches!(other, EventKind::Other),
            }
        }
    }

    #[derive(Debug, Clone, Copy)]
    enum ExpectedAccessKind {
        Any,
        Read,
        Open(Option<AccessMode>),
        Close(Option<AccessMode>),
        Other,
    }

    impl PartialEq<AccessKind> for ExpectedAccessKind {
        fn eq(&self, other: &AccessKind) -> bool {
            match self {
                ExpectedAccessKind::Any => matches!(other, AccessKind::Any),
                ExpectedAccessKind::Read => matches!(other, AccessKind::Read),
                ExpectedAccessKind::Open(expected) => {
                    let AccessKind::Open(other) = other else {
                        return false;
                    };
                    expected.is_none_or(|expected| &expected == other)
                }
                ExpectedAccessKind::Close(expected) => {
                    let AccessKind::Close(other) = other else {
                        return false;
                    };
                    expected.is_none_or(|expected| &expected == other)
                }
                ExpectedAccessKind::Other => matches!(other, AccessKind::Other),
            }
        }
    }

    #[derive(Debug, Clone, Copy)]
    enum ExpectedModifyKind {
        Any,
        Data(Option<DataChange>),
        Metadata(Option<MetadataKind>),
        Name(Option<RenameMode>),
        Other,
    }

    impl PartialEq<ModifyKind> for ExpectedModifyKind {
        fn eq(&self, other: &ModifyKind) -> bool {
            match self {
                ExpectedModifyKind::Any => matches!(other, ModifyKind::Any),
                ExpectedModifyKind::Data(expected) => {
                    let ModifyKind::Data(other) = other else {
                        return false;
                    };
                    expected.is_none_or(|expected| &expected == other)
                }
                ExpectedModifyKind::Metadata(expected) => {
                    let ModifyKind::Metadata(other) = other else {
                        return false;
                    };
                    expected.is_none_or(|expected| &expected == other)
                }
                ExpectedModifyKind::Name(expected) => {
                    let ModifyKind::Name(other) = other else {
                        return false;
                    };
                    expected.is_none_or(|expected| &expected == other)
                }
                ExpectedModifyKind::Other => matches!(other, ModifyKind::Other),
            }
        }
    }

    impl PartialEq<Event> for ExpectedEvent {
        fn eq(&self, other: &Event) -> bool {
            let Self {
                kind,
                paths,
                tracker,
                multiple: _,
                optional: _,
            } = self;

            kind.is_none_or(|kind| kind == other.kind)
                && tracker.is_none_or(|expected| expected == other.attrs.tracker())
                && paths
                    .as_ref()
                    .is_none_or(|expected| expected == &other.paths)
        }
    }

    impl PartialEq<ExpectedEvent> for Event {
        fn eq(&self, other: &ExpectedEvent) -> bool {
            other.eq(self)
        }
    }

    macro_rules! kind {
        ($name: ident, $kind: expr) => {
            pub fn $name(self) -> Self {
                self.kind($kind)
            }
        };
    }

    #[rustfmt::skip] // due to annoying macro invocations formatting 
    impl ExpectedEvent {
        pub fn add_path(mut self, path: impl AsRef<Path>) -> Self {
            self.push_path(path.as_ref().to_path_buf());
            self
        }

        pub fn push_path(&mut self, path: PathBuf) {
            match &mut self.paths {
                Some(paths) => paths.push(path),
                None => self.paths = Some(vec![path]),
            }
        }

        pub fn without_tracker(mut self) -> Self { 
            self.tracker = Some(None);
            self
        }

        pub fn tracker(mut self, tracker: usize) -> Self {
            self.tracker = Some(Some(tracker));
            self
        }

        /// There may be more than one that kind of the event.
        /// 
        /// For instance, it helps you to merge "flaky" events, like "write" data: 
        /// one call may cause more than one event.
        /// 
        /// If a checker encounters that kind of an event, it stores the last received and
        /// compare it with the next ones
        /// 
        /// It **does not** affect PartialEq, it is external information
        pub fn multiple(mut self) -> Self {
            self.multiple = true;
            self
        }

        /// There may be no event.
        /// 
        /// It **does not** affect PartialEq, it is external information
        pub fn optional(mut self) -> Self {
            self.optional = true;
            self
        }

        pub fn is_multiple(&self) -> bool {
            self.multiple
        }

        pub fn is_optional(&self) -> bool {
            self.optional
        }

        kind!(any, ExpectedEventKind::Any);
        kind!(other, ExpectedEventKind::Other);

        kind!(modify, ExpectedEventKind::Modify(None));
        kind!(modify_any, ExpectedEventKind::Modify(Some(ExpectedModifyKind::Any)));
        kind!(modify_other,ExpectedEventKind::Modify(Some(ExpectedModifyKind::Other)));

        kind!(modify_data,ExpectedEventKind::Modify(Some(ExpectedModifyKind::Data(None))));
        kind!(modify_data_any, ExpectedEventKind::Modify(Some(ExpectedModifyKind::Data(Some(DataChange::Any)))));
        kind!(modify_data_other, ExpectedEventKind::Modify(Some(ExpectedModifyKind::Data(Some(DataChange::Other)))));
        kind!(modify_data_content, ExpectedEventKind::Modify(Some(ExpectedModifyKind::Data(Some(DataChange::Content)))));
        kind!(modify_data_size, ExpectedEventKind::Modify(Some(ExpectedModifyKind::Data(Some(DataChange::Size)))));

        kind!(modify_meta, ExpectedEventKind::Modify(Some(ExpectedModifyKind::Metadata(None))));
        kind!(modify_meta_any, ExpectedEventKind::Modify(Some(ExpectedModifyKind::Metadata(Some(MetadataKind::Any)))));
        kind!(modify_meta_other, ExpectedEventKind::Modify(Some(ExpectedModifyKind::Metadata(Some(MetadataKind::Other )))));
        kind!(modify_meta_atime, ExpectedEventKind::Modify(Some(ExpectedModifyKind::Metadata(Some(MetadataKind::AccessTime )))));
        kind!(modify_meta_mtime, ExpectedEventKind::Modify(Some(ExpectedModifyKind::Metadata(Some(MetadataKind::WriteTime )))));
        kind!(modify_meta_owner, ExpectedEventKind::Modify(Some(ExpectedModifyKind::Metadata(Some(MetadataKind::Ownership )))));
        kind!(modify_meta_ext, ExpectedEventKind::Modify(Some(ExpectedModifyKind::Metadata(Some(MetadataKind::Extended )))));
        kind!(modify_meta_perm, ExpectedEventKind::Modify(Some(ExpectedModifyKind::Metadata(Some(MetadataKind::Permissions )))));

        kind!(rename, ExpectedEventKind::Modify(Some(ExpectedModifyKind::Name(None))));
        kind!(rename_any, ExpectedEventKind::Modify(Some(ExpectedModifyKind::Name(Some(RenameMode::Any)))));
        kind!(rename_other, ExpectedEventKind::Modify(Some(ExpectedModifyKind::Name(Some(RenameMode::Other)))));
        kind!(rename_from, ExpectedEventKind::Modify(Some(ExpectedModifyKind::Name(Some(RenameMode::From)))));
        kind!(rename_to, ExpectedEventKind::Modify(Some(ExpectedModifyKind::Name(Some(RenameMode::To)))));
        kind!(rename_both, ExpectedEventKind::Modify(Some(ExpectedModifyKind::Name(Some(RenameMode::Both)))));

        kind!(create, ExpectedEventKind::Create(None));
        kind!(create_any, ExpectedEventKind::Create(Some(CreateKind::Any)));
        kind!(create_other, ExpectedEventKind::Create(Some(CreateKind::Other)));
        kind!(create_file, ExpectedEventKind::Create(Some(CreateKind::File)));
        kind!(create_folder, ExpectedEventKind::Create(Some(CreateKind::Folder)));

        kind!(remove, ExpectedEventKind::Remove(None));
        kind!(remove_any, ExpectedEventKind::Remove(Some(RemoveKind::Any)));
        kind!(remove_other, ExpectedEventKind::Remove(Some(RemoveKind::Other)));
        kind!(remove_file, ExpectedEventKind::Remove(Some(RemoveKind::File)));
        kind!(remove_folder, ExpectedEventKind::Remove(Some(RemoveKind::Folder)));

        kind!(access, ExpectedEventKind::Access(None));
        kind!(access_any, ExpectedEventKind::Access(Some(ExpectedAccessKind::Any)));
        kind!(access_other, ExpectedEventKind::Access(Some(ExpectedAccessKind::Other)));
        kind!(access_read, ExpectedEventKind::Access(Some(ExpectedAccessKind::Read)));

        kind!(access_open, ExpectedEventKind::Access(Some(ExpectedAccessKind::Open(None))));
        kind!(access_open_any, ExpectedEventKind::Access(Some(ExpectedAccessKind::Open(Some(AccessMode::Any)))));
        kind!(access_open_other, ExpectedEventKind::Access(Some(ExpectedAccessKind::Open(Some(AccessMode::Other)))));
        kind!(access_open_read, ExpectedEventKind::Access(Some(ExpectedAccessKind::Open(Some(AccessMode::Read)))));
        kind!(access_open_write, ExpectedEventKind::Access(Some(ExpectedAccessKind::Open(Some(AccessMode::Write)))));
        kind!(access_open_execute, ExpectedEventKind::Access(Some(ExpectedAccessKind::Open(Some(AccessMode::Execute)))));

        kind!(access_close, ExpectedEventKind::Access(Some(ExpectedAccessKind::Close(None))));
        kind!(access_close_any, ExpectedEventKind::Access(Some(ExpectedAccessKind::Close(Some(AccessMode::Any)))));
        kind!(access_close_other, ExpectedEventKind::Access(Some(ExpectedAccessKind::Close(Some(AccessMode::Other)))));
        kind!(access_close_read, ExpectedEventKind::Access(Some(ExpectedAccessKind::Close(Some(AccessMode::Read)))));
        kind!(access_close_write, ExpectedEventKind::Access(Some(ExpectedAccessKind::Close(Some(AccessMode::Write)))));
        kind!(access_close_execute, ExpectedEventKind::Access(Some(ExpectedAccessKind::Close(Some(AccessMode::Execute)))));

        fn kind(mut self, kind: ExpectedEventKind) -> Self {
            self.kind = Some(kind);
            self
        }
    }
}
