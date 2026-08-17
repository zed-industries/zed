//! Debouncer for [notify](https://crates.io/crates/notify). Filters incoming events and emits only one event per timeframe per file.
//!
//! # Installation
//!
//! ```toml
//! [dependencies]
//! notify-debouncer-mini = "0.4.1"
//! ```
//! In case you want to select specific features of notify,
//! specify notify as dependency explicitly in your dependencies.
//! Otherwise you can just use the re-export of notify from debouncer-mini.
//! ```toml
//! notify-debouncer-mini = "0.4.1"
//! notify = { version = "..", features = [".."] }
//! ```
//!  
//! # Examples
//! See also the full configuration example [here](https://github.com/notify-rs/notify/blob/main/examples/debouncer_mini_custom.rs).
//!
//! ```rust,no_run
//! # use std::path::Path;
//! # use std::time::Duration;
//! use notify_debouncer_mini::{notify::*,new_debouncer,DebounceEventResult};
//!
//! # fn main() {
//!   // Select recommended watcher for debouncer.
//!   // Using a callback here, could also be a channel.
//!   let mut debouncer = new_debouncer(Duration::from_secs(2), |res: DebounceEventResult| {
//!       match res {
//!           Ok(events) => events.iter().for_each(|e|println!("Event {:?} for {:?}",e.kind,e.path)),
//!           Err(e) => println!("Error {:?}",e),
//!       }
//!   }).unwrap();
//!
//!   // Add a path to be watched. All files and directories at that path and
//!   // below will be monitored for changes.
//!   debouncer.watcher().watch(Path::new("."), RecursiveMode::Recursive).unwrap();
//! 
//!   // note that dropping the debouncer (as will happen here) also ends the debouncer
//!   // thus this demo would need an endless loop to keep running
//! # }
//! ```
//!
//! # Features
//!
//! The following crate features can be turned on or off in your cargo dependency config:
//!
//! - `crossbeam` enabled by default, adds [`DebounceEventHandler`](DebounceEventHandler) support for crossbeam channels.
//!   Also enables crossbeam-channel in the re-exported notify. You may want to disable this when using the tokio async runtime.
//! - `serde` enables serde support for events.
//! 
//! # Caveats
//! 
//! As all file events are sourced from notify, the [known problems](https://docs.rs/notify/latest/notify/#known-problems) section applies here too.
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    path::PathBuf,
    sync::mpsc::{RecvTimeoutError, Sender},
    time::{Duration, Instant},
};

pub use notify;
use notify::{Error, Event, RecommendedWatcher, Watcher};

/// The set of requirements for watcher debounce event handling functions.
///
/// # Example implementation
///
/// ```rust,no_run
/// # use notify::{Event, Result, EventHandler};
/// # use notify_debouncer_mini::{DebounceEventHandler,DebounceEventResult};
///
/// /// Prints received events
/// struct EventPrinter;
///
/// impl DebounceEventHandler for EventPrinter {
///     fn handle_event(&mut self, event: DebounceEventResult) {
///         match event {
///             Ok(events) => {
///                 for event in events {
///                     println!("Event {:?} for path {:?}",event.kind,event.path);
///                 }
///             },
///             // errors are immediately reported
///             Err(error) => println!("Got error {:?}",error),
///         }
///     }
/// }
/// ```
pub trait DebounceEventHandler: Send + 'static {
    /// Handles an event.
    fn handle_event(&mut self, event: DebounceEventResult);
}

/// Config for debouncer-mini
/// ```rust
/// # use std::time::Duration;
/// use notify_debouncer_mini::Config;
/// let backend_config = notify::Config::default();
///
/// let config = Config::default().with_timeout(Duration::from_secs(1)).with_batch_mode(true)
///     .with_notify_config(backend_config);
/// ```
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Config {
    timeout: Duration,
    batch_mode: bool,
    notify_config: notify::Config,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            timeout: Duration::from_millis(500),
            batch_mode: true,
            notify_config: notify::Config::default(),
        }
    }
}

impl Config {
    /// Set timeout
    ///
    /// Timeout is the amount of time after which a debounced event is emitted or a continuous event is send, if there still are events incoming for the specific path.
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }
    /// Set batch mode
    ///
    /// When `batch_mode` is enabled, events may be delayed (at most 2x the specified timout) and delivered with others.
    /// If disabled, all events are delivered immediately when their debounce timeout is reached.
    pub fn with_batch_mode(mut self, batch_mode: bool) -> Self {
        self.batch_mode = batch_mode;
        self
    }
    /// Set [`notify::Config`] for the backend
    pub fn with_notify_config(mut self, notify_config: notify::Config) -> Self {
        self.notify_config = notify_config;
        self
    }
}

impl<F> DebounceEventHandler for F
where
    F: FnMut(DebounceEventResult) + Send + 'static,
{
    fn handle_event(&mut self, event: DebounceEventResult) {
        (self)(event);
    }
}

#[cfg(feature = "crossbeam")]
impl DebounceEventHandler for crossbeam_channel::Sender<DebounceEventResult> {
    fn handle_event(&mut self, event: DebounceEventResult) {
        let _ = self.send(event);
    }
}

impl DebounceEventHandler for std::sync::mpsc::Sender<DebounceEventResult> {
    fn handle_event(&mut self, event: DebounceEventResult) {
        let _ = self.send(event);
    }
}

/// Deduplicate event data entry
#[derive(Debug)]
struct EventData {
    /// Insertion Time
    insert: Instant,
    /// Last Update
    update: Instant,
}

impl EventData {
    #[inline(always)]
    fn new_any(time: Instant) -> Self {
        Self {
            insert: time,
            update: time,
        }
    }
}

/// A result of debounced events.
/// Comes with either a vec of events or an immediate error.
pub type DebounceEventResult = Result<Vec<DebouncedEvent>, Error>;

/// A debounced event kind.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub enum DebouncedEventKind {
    /// No precise events
    Any,
    /// Event but debounce timed out (for example continuous writes)
    AnyContinuous,
}

/// A debounced event.
///
/// Does not emit any specific event type on purpose, only distinguishes between an any event and a continuous any event.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DebouncedEvent {
    /// Event path
    pub path: PathBuf,
    /// Event kind
    pub kind: DebouncedEventKind,
}

impl DebouncedEvent {
    #[inline(always)]
    fn new(path: PathBuf, kind: DebouncedEventKind) -> Self {
        Self { path, kind }
    }
}

enum InnerEvent {
    NotifyEvent(Result<Event, Error>),
    Shutdown,
}

struct DebounceDataInner {
    /// Path -> Event data
    event_map: HashMap<PathBuf, EventData>,
    /// timeout used to compare all events against, config
    timeout: Duration,
    /// Whether to time events exactly, or batch multiple together.
    /// This reduces the amount of updates but possibly waiting longer than necessary for some events
    batch_mode: bool,
    /// next debounce deadline
    debounce_deadline: Option<Instant>,
}

impl DebounceDataInner {
    pub fn new(timeout: Duration, batch_mode: bool) -> Self {
        Self {
            timeout,
            debounce_deadline: None,
            event_map: Default::default(),
            batch_mode,
        }
    }

    /// Returns a duration to wait for the next tick
    #[inline]
    pub fn next_tick(&self) -> Option<Duration> {
        self.debounce_deadline
            .map(|deadline| deadline.saturating_duration_since(Instant::now()))
    }

    /// Retrieve a vec of debounced events, removing them if not continuous
    ///
    /// Updates the internal tracker for the next tick
    pub fn debounced_events(&mut self) -> Vec<DebouncedEvent> {
        let mut events_expired = Vec::with_capacity(self.event_map.len());
        let mut data_back = HashMap::with_capacity(self.event_map.len());
        // TODO: perfect fit for drain_filter https://github.com/rust-lang/rust/issues/59618
        // reset deadline
        self.debounce_deadline = None;
        for (path, event) in self.event_map.drain() {
            if event.update.elapsed() >= self.timeout {
                log::trace!("debounced event: {:?}", DebouncedEventKind::Any);
                events_expired.push(DebouncedEvent::new(path, DebouncedEventKind::Any));
            } else if event.insert.elapsed() >= self.timeout {
                log::trace!("debounced event: {:?}", DebouncedEventKind::AnyContinuous);
                // set a new deadline, otherwise an 'AnyContinuous' will never resolve to a final 'Any' event
                Self::check_deadline(
                    self.batch_mode,
                    self.timeout,
                    &mut self.debounce_deadline,
                    &event,
                );
                data_back.insert(path.clone(), event);
                events_expired.push(DebouncedEvent::new(path, DebouncedEventKind::AnyContinuous));
            } else {
                // event is neither old enough for continous event, nor is it expired for an Any event
                Self::check_deadline(
                    self.batch_mode,
                    self.timeout,
                    &mut self.debounce_deadline,
                    &event,
                );
                data_back.insert(path, event);
            }
        }
        self.event_map = data_back;
        events_expired
    }

    /// Updates the deadline if none is set or when batch mode is disabled and the current deadline would miss the next event.
    /// The new deadline is calculated based on the last event update time and the debounce timeout.
    ///  
    /// can't sub-function this due to event_map.drain() holding &mut self
    fn check_deadline(
        batch_mode: bool,
        timeout: Duration,
        debounce_deadline: &mut Option<Instant>,
        event: &EventData,
    ) {
        let deadline_candidate = event.update + timeout;
        match debounce_deadline {
            Some(current_deadline) => {
                // shorten deadline to not delay the event
                // with batch mode simply wait for the incoming deadline and delay the event untill then
                if !batch_mode && *current_deadline > deadline_candidate {
                    *debounce_deadline = Some(deadline_candidate);
                }
            }
            None => *debounce_deadline = Some(deadline_candidate),
        }
    }

    /// Add new event to debouncer cache
    #[inline(always)]
    fn add_event(&mut self, event: Event) {
        log::trace!("raw event: {event:?}");
        let time = Instant::now();
        if self.debounce_deadline.is_none() {
            self.debounce_deadline = Some(time + self.timeout);
        }
        for path in event.paths.into_iter() {
            if let Some(v) = self.event_map.get_mut(&path) {
                v.update = time;
            } else {
                self.event_map.insert(path, EventData::new_any(time));
            }
        }
    }
}

/// Debouncer guard, stops the debouncer on drop
#[derive(Debug)]
pub struct Debouncer<T: Watcher> {
    watcher: T,
    stop_channel: Sender<InnerEvent>,
}

impl<T: Watcher> Debouncer<T> {
    /// Access to the internally used notify Watcher backend
    pub fn watcher(&mut self) -> &mut dyn Watcher {
        &mut self.watcher
    }
}

impl<T: Watcher> Drop for Debouncer<T> {
    fn drop(&mut self) {
        // send error just means that it is stopped, can't do much else
        let _ = self.stop_channel.send(InnerEvent::Shutdown);
    }
}

/// Creates a new debounced watcher with custom configuration.
pub fn new_debouncer_opt<F: DebounceEventHandler, T: Watcher>(
    config: Config,
    mut event_handler: F,
) -> Result<Debouncer<T>, Error> {
    let (tx, rx) = std::sync::mpsc::channel();

    std::thread::Builder::new()
        .name("notify-rs debouncer loop".to_string())
        .spawn(move || {
            let mut data = DebounceDataInner::new(config.timeout, config.batch_mode);
            let mut run = true;
            while run {
                match data.next_tick() {
                    Some(timeout) => {
                        // wait for wakeup
                        match rx.recv_timeout(timeout) {
                            Ok(InnerEvent::NotifyEvent(event_result)) => match event_result {
                                Ok(event) => data.add_event(event),
                                Err(err) => event_handler.handle_event(Err(err)),
                            },
                            Err(RecvTimeoutError::Timeout) => {
                                let send_data = data.debounced_events();
                                if !send_data.is_empty() {
                                    event_handler.handle_event(Ok(send_data));
                                }
                            }
                            Ok(InnerEvent::Shutdown) | Err(RecvTimeoutError::Disconnected) => {
                                run = false
                            }
                        }
                    }
                    None => match rx.recv() {
                        // no timeout, wait for event
                        Ok(InnerEvent::NotifyEvent(e)) => match e {
                            Ok(event) => data.add_event(event),
                            Err(err) => event_handler.handle_event(Err(err)),
                        },
                        Ok(InnerEvent::Shutdown) => run = false,
                        Err(_) => run = false,
                    },
                }
            }
        })?;

    let tx_c = tx.clone();
    let watcher = T::new(
        move |e: Result<Event, Error>| {
            // send failure can't be handled, would need a working channel to signal that
            // also probably means that we're in the process of shutting down
            let _ = tx_c.send(InnerEvent::NotifyEvent(e));
        },
        config.notify_config,
    )?;

    let guard = Debouncer {
        watcher,
        stop_channel: tx,
    };

    Ok(guard)
}

/// Short function to create a new debounced watcher with the recommended debouncer.
///
/// Timeout is the amount of time after which a debounced event is emitted or a continuous event is send, if there still are events incoming for the specific path.
pub fn new_debouncer<F: DebounceEventHandler>(
    timeout: Duration,
    event_handler: F,
) -> Result<Debouncer<RecommendedWatcher>, Error> {
    let config = Config::default().with_timeout(timeout);
    new_debouncer_opt::<F, RecommendedWatcher>(config, event_handler)
}
