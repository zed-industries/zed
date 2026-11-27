use objc2::{
    DefinedClass, MainThreadMarker, MainThreadOnly, Message, define_class, msg_send,
    rc::Retained,
    runtime::{AnyObject, Sel},
    sel,
};
use objc2_foundation::{NSObject, NSObjectProtocol, NSRunLoop, NSRunLoopCommonModes};
pub use objc2_quartz_core::CADisplayLink;
use std::cell::Cell;

struct DisplayLinkTargetIvars {
    callback: Box<dyn Fn()>,
}

define_class!(
    // SAFETY: `NSObject` has no subclassing requirements.
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[ivars = DisplayLinkTargetIvars]
    struct DisplayLinkTarget;

    unsafe impl NSObjectProtocol for DisplayLinkTarget {}

    impl DisplayLinkTarget {
        #[unsafe(method(displayLinkFired:))]
        fn display_link_fired(&self, _link: &CADisplayLink) {
            // The callback may drop its `DisplayLink`, which calls `invalidate`
            // and releases the link's strong reference to `self`.
            let target = self.retain();
            (target.ivars().callback)();
        }
    }
);

impl DisplayLinkTarget {
    fn new(main_thread: MainThreadMarker, callback: impl Fn() + 'static) -> Retained<Self> {
        let this = Self::alloc(main_thread).set_ivars(DisplayLinkTargetIvars {
            callback: Box::new(callback),
        });
        // SAFETY: `NSObject`'s `init` is its designated initializer.
        unsafe { msg_send![super(this), init] }
    }
}

/// A main-run-loop `CADisplayLink` shared across Apple platforms (macOS 14+ and
/// iOS/iPadOS).
///
/// Unlike `CAMetalDisplayLink`, `CADisplayLink` only paces frames: the renderer
/// acquires drawables from `CAMetalLayer` and presents them immediately instead
/// of queuing them for a `targetPresentationTimestamp` several refreshes ahead.
pub struct DisplayLink {
    link: Retained<CADisplayLink>,
    _target: Retained<DisplayLinkTarget>,
    invalidated: Cell<bool>,
    _main_thread: MainThreadMarker,
}

impl DisplayLink {
    /// Starts delivering vsync ticks on the main run loop in common modes using
    /// the `CADisplayLink` returned by `create(target, selector)`.
    pub fn new(
        main_thread: MainThreadMarker,
        create: impl FnOnce(&AnyObject, Sel) -> Retained<CADisplayLink>,
        callback: impl Fn() + 'static,
    ) -> Self {
        Self::build(main_thread, false, create, callback)
    }

    /// Creates a `CADisplayLink` registered on the main run loop in common
    /// modes, initially paused until [`Self::set_paused`] resumes it.
    pub fn new_paused(
        main_thread: MainThreadMarker,
        create: impl FnOnce(&AnyObject, Sel) -> Retained<CADisplayLink>,
        callback: impl Fn() + 'static,
    ) -> Self {
        Self::build(main_thread, true, create, callback)
    }

    fn build(
        main_thread: MainThreadMarker,
        paused: bool,
        create: impl FnOnce(&AnyObject, Sel) -> Retained<CADisplayLink>,
        callback: impl Fn() + 'static,
    ) -> Self {
        let target = DisplayLinkTarget::new(main_thread, callback);
        let link = create(&target, sel!(displayLinkFired:));
        link.setPaused(paused);
        // SAFETY: `MainThreadMarker` confines `Self` to the main thread, and
        // the main run loop outlives the link.
        unsafe { link.addToRunLoop_forMode(&NSRunLoop::mainRunLoop(), NSRunLoopCommonModes) };
        Self {
            link,
            _target: target,
            invalidated: Cell::new(false),
            _main_thread: main_thread,
        }
    }

    /// Starts delivering vsync ticks for the main display on iOS/iPadOS.
    #[cfg(target_os = "ios")]
    pub fn for_main_display(main_thread: MainThreadMarker, callback: impl Fn() + 'static) -> Self {
        Self::new(
            main_thread,
            |target, selector| {
                // SAFETY: `DisplayLink::new` supplies a target implementing `selector`.
                unsafe { CADisplayLink::displayLinkWithTarget_selector(target, selector) }
            },
            callback,
        )
    }

    pub fn set_paused(&self, paused: bool) {
        self.link.setPaused(paused);
    }

    pub fn is_paused(&self) -> bool {
        self.link.isPaused()
    }

    pub fn invalidate(&self) {
        if !self.invalidated.replace(true) {
            self.link.invalidate();
        }
    }

    pub fn is_invalidated(&self) -> bool {
        self.invalidated.get()
    }
}

impl Drop for DisplayLink {
    fn drop(&mut self) {
        self.invalidate();
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DisplayLinkDebugState {
    pub attached: bool,
    pub active: bool,
    pub paused: bool,
    pub woken: bool,
}

#[derive(Default)]
struct FrameDemand {
    /// Whether the owner currently permits display-link ticks (e.g. visible and
    /// not inside a synchronous transaction draw).
    active: Cell<bool>,
    /// Whether GPUI asked for a frame since the current tick began.
    woken: Cell<bool>,
    /// Whether GPUI has presented since the previous tick began.
    presented: Cell<bool>,
}

impl FrameDemand {
    fn stop(&self) {
        self.active.set(false);
        self.presented.set(false);
    }

    fn wake(&self) -> bool {
        self.woken.set(true);
        self.active.get()
    }

    fn can_draw_early(&self) -> bool {
        self.active.get() && !self.presented.get()
    }

    fn begin_frame(&self) {
        self.woken.set(false);
        self.presented.set(false);
    }

    fn should_pause_after_frame(&self) -> bool {
        !self.presented.get() && !self.woken.get()
    }
}

/// Demand-driven `CADisplayLink` frame state shared with GPUI's frame waker so
/// that waking a paused link does not need to acquire a platform window lock.
///
/// The link is created once when the window opens and pauses after any tick in
/// which GPUI neither presented nor asked for another frame, so an idle window
/// receives no callbacks.
pub struct DisplayLinkFrames {
    link: DisplayLink,
    demand: FrameDemand,
}

impl DisplayLinkFrames {
    pub fn new(
        main_thread: MainThreadMarker,
        create: impl FnOnce(&AnyObject, Sel) -> Retained<CADisplayLink>,
        callback: impl Fn() + 'static,
    ) -> Self {
        Self {
            link: DisplayLink::new_paused(main_thread, create, callback),
            demand: FrameDemand::default(),
        }
    }

    pub fn resume(&self) {
        self.demand.active.set(true);
        self.link.set_paused(false);
    }

    pub fn stop(&self) {
        self.demand.stop();
        self.link.set_paused(true);
    }

    pub fn invalidate(&self) {
        self.demand.stop();
        self.link.invalidate();
    }

    pub fn wake(&self) {
        if self.demand.wake() {
            self.link.set_paused(false);
        }
    }

    pub fn has_pending_wake(&self) -> bool {
        self.demand.woken.get()
    }

    pub fn clear_pending_wake(&self) {
        self.demand.woken.set(false);
    }

    pub fn did_present(&self) {
        self.demand.presented.set(true);
    }

    /// Whether an input-triggered draw can present immediately without
    /// presenting more than once in the current refresh interval.
    pub fn can_draw_early(&self) -> bool {
        self.demand.can_draw_early()
    }

    pub fn begin_frame(&self) {
        self.demand.begin_frame();
    }

    /// Ends a tick, pausing the link if GPUI neither presented nor asked for
    /// another frame while it ran. Returns whether it paused.
    pub fn end_frame(&self) -> bool {
        let pause = self.demand.should_pause_after_frame();
        if pause {
            self.link.set_paused(true);
        }
        pause
    }

    pub fn debug_state(&self) -> DisplayLinkDebugState {
        DisplayLinkDebugState {
            attached: !self.link.is_invalidated(),
            active: self.demand.active.get(),
            paused: self.link.is_paused(),
            woken: self.demand.woken.get(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frames_pause_only_when_gpui_neither_presents_nor_asks_again() {
        let demand = FrameDemand::default();

        demand.begin_frame();
        demand.presented.set(true);
        assert!(
            !demand.should_pause_after_frame(),
            "a frame that presented keeps the link running"
        );

        demand.begin_frame();
        demand.wake();
        assert!(
            !demand.should_pause_after_frame(),
            "a wake during the tick, like a throttled frame's retry, keeps it running"
        );

        demand.wake();
        demand.begin_frame();
        assert!(
            demand.should_pause_after_frame(),
            "a wake before the tick was served by it"
        );

        demand.presented.set(true);
        demand.begin_frame();
        assert!(
            demand.should_pause_after_frame(),
            "a present before the tick, like an input-driven frame, was not this tick's"
        );

        demand.begin_frame();
        assert!(
            demand.should_pause_after_frame(),
            "an idle tick pauses the link"
        );
    }

    #[test]
    fn early_draws_cap_at_one_present_until_next_tick() {
        let demand = FrameDemand::default();
        assert!(
            !demand.can_draw_early(),
            "an inactive link does not draw early"
        );

        demand.active.set(true);
        assert!(
            demand.can_draw_early(),
            "idle active window draws the first keypress immediately"
        );

        demand.presented.set(true);
        assert!(
            !demand.can_draw_early(),
            "a second keypress before vsync waits for the display-link tick"
        );

        demand.begin_frame();
        assert!(
            demand.can_draw_early(),
            "the next vsync tick re-allows an early draw once idle"
        );

        demand.presented.set(true);
        demand.stop();
        assert!(!demand.can_draw_early());
        demand.active.set(true);
        assert!(
            demand.can_draw_early(),
            "stopping resets the presented flag"
        );
    }
}
