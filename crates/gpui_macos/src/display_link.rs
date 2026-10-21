//! Frame pacing for macOS windows, built on `CVDisplayLink`.
//!
//! CVDisplayLink has no safe teardown: `CVDisplayLinkStop` merely flags the
//! link's io thread to exit "soon" and returns immediately, and there is no
//! way to learn when (or whether) a final output callback has finished. Two
//! crash classes came from ignoring this:
//!
//! * releasing the link after `stop` let the still-running io thread read the
//!   freed link's internals (segfault in `CVHWTime::reset`, fixed by leaking
//!   the link in #32116);
//! * releasing the dispatch source that the output callback dereferenced via
//!   its context pointer let a straggler callback read freed memory (segfault
//!   in `dispatch_source_merge_data`, Sentry issue ZED-7XR).
//!
//! Instead of leaking one link and one source per teardown (and teardown
//! happens on every resize tick, activation, and occlusion change), we keep a
//! single immortal `CVDisplayLink` per display in a static registry and have
//! windows subscribe to it:
//!
//! * Links are created lazily and never released, so the release race cannot
//!   occur. The output callback's context is the display id (an integer, not
//!   a pointer), and the only memory the callback dereferences is the static
//!   registry, so a straggler callback after `stop` is a harmless no-op.
//! * Each window owns one dispatch source for the life of the window. On
//!   window close it is removed from the registry under the lock (making it
//!   unreachable from the callback), cancelled, and genuinely released.
//! * A display's link runs iff it has subscribers; windows never start or
//!   stop links directly, so interleaved starts/stops of windows sharing a
//!   display cannot conflict.
//!
//! One tradeoff of immortal entries: a link created for a given
//! `CGDirectDisplayID` is reused forever, including after the display is
//! unplugged and one reappears with the same id, or after mode/refresh-rate
//! changes. `CVDisplayLink` looks up display timing dynamically, so a cached
//! link keeps pacing correctly; if that ever proves untrue the fix is to
//! also refresh entries from a display-reconfiguration callback, not to
//! release links (which would reintroduce the teardown race).
//!
//! Lock ordering: the output callback runs on the link's io thread and takes
//! the registry lock, possibly while holding CVDisplayLink-internal locks. To
//! avoid a lock cycle through those (undocumented) internals, we never call a
//! CoreVideo function while holding the registry lock. Registry mutations and
//! link start/stop happen on the main thread only, which keeps the `running`
//! flag and the link's actual state consistent without holding the lock
//! across the calls.
//!
//! `std::sync::Mutex` rather than `parking_lot` is deliberate: on macOS it is
//! currently backed by `os_unfair_lock`, whose priority donation resolves
//! inversions between the high-priority io thread and the main thread. (That
//! is a std implementation detail, not a guarantee; if it changes, the cost
//! is added latency under contention, not incorrectness.)

use anyhow::Result;
use core_graphics::display::CGDirectDisplayID;
use dispatch2::{
    _dispatch_source_type_data_add, DispatchObject, DispatchQueue, DispatchRetained, DispatchSource,
};
use gpui::{FrameRequestSource, PlatformFrameSignal};
use gpui_util::ResultExt;
use scheduler::Instant;
use std::{
    collections::{BTreeMap, btree_map},
    ffi::c_void,
    sync::{Arc, Mutex, MutexGuard, PoisonError},
    time::Duration,
};

static REGISTRY: Mutex<Registry> = Mutex::new(Registry::new());

struct Registry {
    displays: BTreeMap<CGDirectDisplayID, DisplayEntry>,
    next_subscriber_id: u64,
}

impl Registry {
    const fn new() -> Self {
        Registry {
            displays: BTreeMap::new(),
            next_subscriber_id: 0,
        }
    }
}

struct DisplayEntry {
    link: sys::DisplayLink,
    running: bool,
    subscribers: Vec<DisplayLinkSubscriber>,
}

struct DisplayLinkSubscriber {
    id: SubscriberId,
    frame_requests: DispatchRetained<DispatchSource>,
    signal: Arc<PlatformFrameSignal>,
}

// SAFETY: Both fields wrapping raw pointers are refcounted handles to
// thread-safe objects that are valid on any thread: `sys::DisplayLink` to a
// CoreVideo object, and each subscriber's `DispatchRetained<DispatchSource>`
// to a GCD object (which the display's io thread really does use, calling
// `merge_data` from the output callback). All mutation of the entry itself
// is serialized by the registry lock.
unsafe impl Send for DisplayEntry {}

#[derive(Copy, Clone, PartialEq, Eq)]
struct SubscriberId(u64);

fn lock_registry() -> MutexGuard<'static, Registry> {
    // Proceeding past poison is safe here (the map's invariants hold after
    // any partial mutation), and panicking instead would abort the process
    // when this is reached from the extern "C" output callback.
    REGISTRY.lock().unwrap_or_else(PoisonError::into_inner)
}

fn debug_assert_main_thread() {
    #[cfg(debug_assertions)]
    {
        use objc::{class, msg_send, sel, sel_impl};
        let is_main_thread: objc::runtime::BOOL =
            unsafe { msg_send![class!(NSThread), isMainThread] };
        debug_assert!(
            is_main_thread == objc::runtime::YES,
            "display link registry mutations must happen on the main thread; \
             the registry's lock ordering and state consistency depend on it"
        );
    }
}

unsafe extern "C" fn display_link_output_callback(
    _display_link_out: *mut sys::CVDisplayLink,
    current_time: *const sys::CVTimeStamp,
    _output_time: *const sys::CVTimeStamp,
    _flags_in: i64,
    _flags_out: *mut i64,
    display_id: *mut c_void,
) -> i32 {
    let signal_at = PlatformFrameSignal::capture(|| {
        // Instant cannot be constructed from Mach ticks, so sample both clock domains.
        let current_host_time = objc2_core_video::CVGetCurrentHostTime();
        let received_at = Instant::now();
        let host_clock_frequency = objc2_core_video::CVGetHostClockFrequency();
        // SAFETY: CoreVideo owns this timestamp for the lifetime of the callback.
        let current_time = unsafe { current_time.as_ref() };
        frame_request_instant(
            current_time,
            current_host_time,
            host_clock_frequency,
            received_at,
        )
    });
    let display_id = display_id as usize as CGDirectDisplayID;
    let registry = lock_registry();
    if let Some(entry) = registry.displays.get(&display_id) {
        for subscriber in &entry.subscribers {
            if let Some(signal_at) = signal_at {
                subscriber
                    .signal
                    .record(signal_at, FrameRequestSource::NativeCallback);
            }
            subscriber.frame_requests.merge_data(1);
        }
    }
    0
}

/// Maps a CoreVideo host timestamp using a paired clock sample.
///
/// Invalid timestamps or clock conversions fall back to callback receipt time.
fn frame_request_instant(
    timestamp: Option<&sys::CVTimeStamp>,
    current_host_time: u64,
    host_clock_frequency: f64,
    received_at: Instant,
) -> Instant {
    let Some(timestamp) = timestamp.filter(|timestamp| {
        timestamp.version == 0 && timestamp.flags & sys::kCVTimeStampHostTimeValid != 0
    }) else {
        return received_at;
    };
    if !host_clock_frequency.is_finite() || host_clock_frequency <= 0.0 {
        return received_at;
    }
    current_host_time
        .checked_sub(timestamp.host_time)
        .and_then(|elapsed_ticks| {
            Duration::try_from_secs_f64(elapsed_ticks as f64 / host_clock_frequency).ok()
        })
        .and_then(|elapsed| received_at.checked_sub(elapsed))
        .unwrap_or(received_at)
}

fn subscribe(
    display_id: CGDirectDisplayID,
    frame_requests: DispatchRetained<DispatchSource>,
    signal: Arc<PlatformFrameSignal>,
) -> Result<SubscriberId> {
    debug_assert_main_thread();

    let needs_link = !lock_registry().displays.contains_key(&display_id);
    let new_link = if needs_link {
        // Created outside the registry lock; see the lock ordering note above.
        Some(unsafe {
            sys::DisplayLink::new(
                display_id,
                display_link_output_callback,
                display_id as usize as *mut c_void,
            )?
        })
    } else {
        None
    };

    let (subscriber_id, link_to_start) = {
        let mut registry = lock_registry();
        let registry = &mut *registry;
        let subscriber_id = SubscriberId(registry.next_subscriber_id);
        registry.next_subscriber_id += 1;
        let entry = match (registry.displays.entry(display_id), new_link) {
            // If an entry appeared since the check above, dropping `new_link`
            // is safe: a never-started link has no io thread, so releasing it
            // doesn't race.
            (btree_map::Entry::Occupied(entry), _) => entry.into_mut(),
            (btree_map::Entry::Vacant(vacant), Some(link)) => vacant.insert(DisplayEntry {
                link,
                running: false,
                subscribers: Vec::new(),
            }),
            (btree_map::Entry::Vacant(_), None) => {
                // Entries are never removed, so an entry observed above cannot
                // have disappeared while subscriptions stay on the main thread.
                anyhow::bail!("display link registry entry vanished for display {display_id}");
            }
        };
        entry.subscribers.push(DisplayLinkSubscriber {
            id: subscriber_id,
            frame_requests,
            signal,
        });
        let link_to_start = if entry.running {
            None
        } else {
            entry.running = true;
            // Clone the refcounted handle so the CVDisplayLinkStart call can
            // happen after the lock is released.
            Some(entry.link.clone())
        };
        (subscriber_id, link_to_start)
    };

    if let Some(mut link) = link_to_start {
        if let Err(error) = unsafe { link.start() } {
            let mut registry = lock_registry();
            if let Some(entry) = registry.displays.get_mut(&display_id) {
                entry.running = false;
                entry.subscribers.retain(|subscriber| {
                    if subscriber.id == subscriber_id {
                        subscriber.signal.take();
                        false
                    } else {
                        true
                    }
                });
            }
            return Err(error);
        }
    }

    Ok(subscriber_id)
}

fn unsubscribe(display_id: CGDirectDisplayID, subscriber_id: SubscriberId) {
    debug_assert_main_thread();

    let link_to_stop = {
        let mut registry = lock_registry();
        let Some(entry) = registry.displays.get_mut(&display_id) else {
            return;
        };
        entry
            .subscribers
            .retain(|subscriber| subscriber.id != subscriber_id);
        if entry.subscribers.is_empty() && entry.running {
            entry.running = false;
            Some(entry.link.clone())
        } else {
            None
        }
    };

    if let Some(mut link) = link_to_stop {
        // A final output callback can still fire after this returns; it finds
        // no subscribers for this display and does nothing.
        unsafe { link.stop().log_err() };
    }
}

/// A per-window source of frame requests, paced by the display the window is
/// on. The wrapped dispatch source coalesces vsync ticks from the display's
/// io thread and invokes `callback(data)` on the main queue.
pub struct WindowFrameSource {
    frame_requests: DispatchRetained<DispatchSource>,
    registration: Option<(CGDirectDisplayID, SubscriberId)>,
    signal: Arc<PlatformFrameSignal>,
}

impl WindowFrameSource {
    pub fn new(data: *mut c_void, callback: extern "C" fn(*mut c_void)) -> Self {
        let frame_requests = unsafe {
            let frame_requests = DispatchSource::new(
                &raw const _dispatch_source_type_data_add as *mut _,
                0,
                0,
                Some(DispatchQueue::main()),
            );
            frame_requests.set_context(data);
            frame_requests.set_event_handler_f(callback);
            // Resume before this source can ever be dropped: destroying a
            // suspended dispatch source is undefined behavior (#50875).
            frame_requests.resume();
            frame_requests
        };
        Self {
            frame_requests,
            registration: None,
            signal: Arc::new(PlatformFrameSignal::new()),
        }
    }

    pub fn start(&mut self, display_id: CGDirectDisplayID) -> Result<()> {
        self.stop();
        let subscriber_id =
            subscribe(display_id, self.frame_requests.clone(), self.signal.clone())?;
        self.registration = Some((display_id, subscriber_id));
        Ok(())
    }

    pub fn take_signal(&self) -> Option<(Instant, FrameRequestSource)> {
        self.signal.take()
    }

    pub fn stop(&mut self) {
        if let Some((display_id, subscriber_id)) = self.registration.take() {
            unsubscribe(display_id, subscriber_id);
        }
        self.signal.take();
    }
}

impl Drop for WindowFrameSource {
    fn drop(&mut self) {
        self.stop();
        // Unsubscribing makes this source unreachable from the output
        // callback, so unlike before (ZED-7XR) it is safe to actually release
        // it. Cancelling first guarantees the event handler never runs again;
        // its context points at the window's native view, which may be
        // deallocated after this.
        self.frame_requests.cancel();
    }
}

mod sys {
    //! Derived from display-link crate under the following license:
    //! <https://github.com/BrainiumLLC/display-link/blob/master/LICENSE-MIT>
    //! Apple docs: [CVDisplayLink](https://developer.apple.com/documentation/corevideo/cvdisplaylinkoutputcallback?language=objc)
    #![allow(dead_code, non_upper_case_globals)]

    use anyhow::Result;
    use core_graphics::display::CGDirectDisplayID;
    use foreign_types::{ForeignType, foreign_type};
    use std::{
        ffi::c_void,
        fmt::{self, Debug, Formatter},
    };

    #[derive(Debug)]
    pub enum CVDisplayLink {}

    foreign_type! {
        pub unsafe type DisplayLink {
            type CType = CVDisplayLink;
            fn drop = CVDisplayLinkRelease;
            fn clone = CVDisplayLinkRetain;
        }
    }

    impl Debug for DisplayLink {
        fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
            formatter
                .debug_tuple("DisplayLink")
                .field(&self.as_ptr())
                .finish()
        }
    }

    #[repr(C)]
    #[derive(Clone, Copy, Default)]
    pub(crate) struct CVTimeStamp {
        pub version: u32,
        pub video_time_scale: i32,
        pub video_time: i64,
        pub host_time: u64,
        pub rate_scalar: f64,
        pub video_refresh_period: i64,
        pub smpte_time: CVSMPTETime,
        pub flags: u64,
        pub reserved: u64,
    }

    pub type CVTimeStampFlags = u64;

    pub const kCVTimeStampVideoTimeValid: CVTimeStampFlags = 1 << 0;
    pub const kCVTimeStampHostTimeValid: CVTimeStampFlags = 1 << 1;
    pub const kCVTimeStampSMPTETimeValid: CVTimeStampFlags = 1 << 2;
    pub const kCVTimeStampVideoRefreshPeriodValid: CVTimeStampFlags = 1 << 3;
    pub const kCVTimeStampRateScalarValid: CVTimeStampFlags = 1 << 4;
    pub const kCVTimeStampTopField: CVTimeStampFlags = 1 << 16;
    pub const kCVTimeStampBottomField: CVTimeStampFlags = 1 << 17;
    pub const kCVTimeStampVideoHostTimeValid: CVTimeStampFlags =
        kCVTimeStampVideoTimeValid | kCVTimeStampHostTimeValid;
    pub const kCVTimeStampIsInterlaced: CVTimeStampFlags =
        kCVTimeStampTopField | kCVTimeStampBottomField;

    #[repr(C)]
    #[derive(Clone, Copy, Default)]
    pub(crate) struct CVSMPTETime {
        pub subframes: i16,
        pub subframe_divisor: i16,
        pub counter: u32,
        pub time_type: u32,
        pub flags: u32,
        pub hours: i16,
        pub minutes: i16,
        pub seconds: i16,
        pub frames: i16,
    }

    pub type CVSMPTETimeType = u32;

    pub const kCVSMPTETimeType24: CVSMPTETimeType = 0;
    pub const kCVSMPTETimeType25: CVSMPTETimeType = 1;
    pub const kCVSMPTETimeType30Drop: CVSMPTETimeType = 2;
    pub const kCVSMPTETimeType30: CVSMPTETimeType = 3;
    pub const kCVSMPTETimeType2997: CVSMPTETimeType = 4;
    pub const kCVSMPTETimeType2997Drop: CVSMPTETimeType = 5;
    pub const kCVSMPTETimeType60: CVSMPTETimeType = 6;
    pub const kCVSMPTETimeType5994: CVSMPTETimeType = 7;

    pub type CVSMPTETimeFlags = u32;

    pub const kCVSMPTETimeValid: CVSMPTETimeFlags = 1 << 0;
    pub const kCVSMPTETimeRunning: CVSMPTETimeFlags = 1 << 1;

    pub type CVDisplayLinkOutputCallback = unsafe extern "C" fn(
        display_link_out: *mut CVDisplayLink,
        // A pointer to the current timestamp. This represents the timestamp when the callback is called.
        current_time: *const CVTimeStamp,
        // A pointer to the output timestamp. This represents the timestamp for when the frame will be displayed.
        output_time: *const CVTimeStamp,
        // Unused
        flags_in: i64,
        // Unused
        flags_out: *mut i64,
        // A pointer to app-defined data.
        display_link_context: *mut c_void,
    ) -> i32;

    #[link(name = "CoreFoundation", kind = "framework")]
    #[link(name = "CoreVideo", kind = "framework")]
    #[allow(improper_ctypes, unknown_lints, clippy::duplicated_attributes)]
    unsafe extern "C" {
        pub fn CVDisplayLinkCreateWithActiveCGDisplays(
            display_link_out: *mut *mut CVDisplayLink,
        ) -> i32;
        pub fn CVDisplayLinkSetCurrentCGDisplay(
            display_link: &mut DisplayLinkRef,
            display_id: u32,
        ) -> i32;
        pub fn CVDisplayLinkSetOutputCallback(
            display_link: &mut DisplayLinkRef,
            callback: CVDisplayLinkOutputCallback,
            user_info: *mut c_void,
        ) -> i32;
        pub fn CVDisplayLinkStart(display_link: &mut DisplayLinkRef) -> i32;
        pub fn CVDisplayLinkStop(display_link: &mut DisplayLinkRef) -> i32;
        pub fn CVDisplayLinkRelease(display_link: *mut CVDisplayLink);
        pub fn CVDisplayLinkRetain(display_link: *mut CVDisplayLink) -> *mut CVDisplayLink;
    }

    impl DisplayLink {
        /// Apple docs: [CVDisplayLinkCreateWithCGDisplay](https://developer.apple.com/documentation/corevideo/1456981-cvdisplaylinkcreatewithcgdisplay?language=objc)
        pub unsafe fn new(
            display_id: CGDirectDisplayID,
            callback: CVDisplayLinkOutputCallback,
            user_info: *mut c_void,
        ) -> Result<Self> {
            unsafe {
                let mut display_link: *mut CVDisplayLink = 0 as _;

                let code = CVDisplayLinkCreateWithActiveCGDisplays(&mut display_link);
                anyhow::ensure!(code == 0, "could not create display link, code: {}", code);

                let mut display_link = DisplayLink::from_ptr(display_link);

                let code = CVDisplayLinkSetOutputCallback(&mut display_link, callback, user_info);
                anyhow::ensure!(code == 0, "could not set output callback, code: {}", code);

                let code = CVDisplayLinkSetCurrentCGDisplay(&mut display_link, display_id);
                anyhow::ensure!(
                    code == 0,
                    "could not assign display to display link, code: {}",
                    code
                );

                Ok(display_link)
            }
        }
    }

    impl DisplayLinkRef {
        /// Apple docs: [CVDisplayLinkStart](https://developer.apple.com/documentation/corevideo/1457193-cvdisplaylinkstart?language=objc)
        pub unsafe fn start(&mut self) -> Result<()> {
            unsafe {
                let code = CVDisplayLinkStart(self);
                anyhow::ensure!(code == 0, "could not start display link, code: {}", code);
                Ok(())
            }
        }

        /// Apple docs: [CVDisplayLinkStop](https://developer.apple.com/documentation/corevideo/1457281-cvdisplaylinkstop?language=objc)
        pub unsafe fn stop(&mut self) -> Result<()> {
            unsafe {
                let code = CVDisplayLinkStop(self);
                anyhow::ensure!(code == 0, "could not stop display link, code: {}", code);
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn host_timestamp_preserves_request_age_at_different_clock_rates() {
        let received_at = Instant::now();
        for (host_time, current_host_time, frequency, age) in [
            (4_000_000, 10_000_000, 1e9, Duration::from_millis(6)),
            (1_760_000, 2_000_000, 24e6, Duration::from_millis(10)),
        ] {
            let timestamp = timestamp(host_time);
            assert_eq!(
                frame_request_instant(Some(&timestamp), current_host_time, frequency, received_at),
                received_at - age,
            );
        }
    }

    #[test]
    fn invalid_host_timestamps_fall_back_to_receipt_time() {
        let received_at = Instant::now();
        let valid = timestamp(1_000);
        let mut missing_host_time = valid;
        missing_host_time.flags = 0;
        let mut unknown_version = valid;
        unknown_version.version = 1;
        for timestamp in [None, Some(&missing_host_time), Some(&unknown_version)] {
            assert_eq!(
                frame_request_instant(timestamp, 2_000, 1e9, received_at),
                received_at,
            );
        }
        assert_eq!(
            frame_request_instant(Some(&valid), 999, 1e9, received_at),
            received_at,
        );
        for frequency in [0.0, -1.0, f64::NAN, f64::INFINITY, f64::MIN_POSITIVE] {
            assert_eq!(
                frame_request_instant(Some(&valid), 2_000, frequency, received_at),
                received_at,
            );
        }
    }

    fn timestamp(host_time: u64) -> sys::CVTimeStamp {
        sys::CVTimeStamp {
            host_time,
            flags: sys::kCVTimeStampHostTimeValid,
            ..Default::default()
        }
    }
}
