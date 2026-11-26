use objc2::{
    DefinedClass, MainThreadMarker, MainThreadOnly, Message, define_class, msg_send,
    rc::Retained,
    runtime::{AnyClass, AnyObject},
    sel,
};
use objc2_foundation::{NSObject, NSObjectProtocol, NSRunLoop, NSRunLoopCommonModes};
use objc2_quartz_core::CADisplayLink;
use std::cell::{Cell, RefCell};

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
    _main_thread: MainThreadMarker,
}

impl DisplayLink {
    /// Whether instances of `class` (such as `NSView`, `NSWindow`, `NSScreen`
    /// on macOS 14+, or `UIWindowScene` / `UIView` on iOS) vend a display-tracking
    /// `CADisplayLink` via `-displayLinkWithTarget:selector:`.
    pub fn supports_source(class: &AnyClass) -> bool {
        // SAFETY: `instancesRespondToSelector:` takes only a `SEL`.
        unsafe {
            msg_send![
                class,
                instancesRespondToSelector: sel!(displayLinkWithTarget:selector:)
            ]
        }
    }

    /// Starts delivering vsync ticks for `source` (an `NSView`, `NSWindow`,
    /// `NSScreen`, `UIWindowScene`, or `UIView`) on the main run loop in common
    /// modes, or returns `None` if `source` does not support creating one.
    ///
    /// # Safety
    ///
    /// `source` must be a valid Objective-C object whose
    /// `-displayLinkWithTarget:selector:` returns a `CADisplayLink *`.
    pub unsafe fn for_source(
        source: &AnyObject,
        main_thread: MainThreadMarker,
        callback: impl Fn() + 'static,
    ) -> Option<Self> {
        let responds: bool = unsafe {
            msg_send![
                source,
                respondsToSelector: sel!(displayLinkWithTarget:selector:)
            ]
        };
        if !responds {
            return None;
        }
        let target = DisplayLinkTarget::new(main_thread, callback);
        let link: Option<Retained<CADisplayLink>> = unsafe {
            msg_send![
                source,
                displayLinkWithTarget: &*target,
                selector: sel!(displayLinkFired:)
            ]
        };
        let link = link?;
        // SAFETY: `MainThreadMarker` confines `Self` to the main thread, and
        // the main run loop outlives the source.
        unsafe { link.addToRunLoop_forMode(&NSRunLoop::mainRunLoop(), NSRunLoopCommonModes) };
        Some(Self {
            link,
            _target: target,
            _main_thread: main_thread,
        })
    }

    /// Starts delivering vsync ticks for the main display on iOS/iPadOS.
    #[cfg(target_os = "ios")]
    pub fn for_main_display(main_thread: MainThreadMarker, callback: impl Fn() + 'static) -> Self {
        let target = DisplayLinkTarget::new(main_thread, callback);
        // SAFETY: `DisplayLinkTarget` implements `displayLinkFired:`.
        let link = unsafe {
            CADisplayLink::displayLinkWithTarget_selector(&target, sel!(displayLinkFired:))
        };
        // SAFETY: `MainThreadMarker` confines `Self` to the main thread.
        unsafe { link.addToRunLoop_forMode(&NSRunLoop::mainRunLoop(), NSRunLoopCommonModes) };
        Self {
            link,
            _target: target,
            _main_thread: main_thread,
        }
    }

    pub fn set_paused(&self, paused: bool) {
        self.link.setPaused(paused);
    }

    pub fn is_paused(&self) -> bool {
        self.link.isPaused()
    }
}

impl Drop for DisplayLink {
    fn drop(&mut self) {
        self.link.invalidate();
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DisplayLinkDebugState {
    pub attached: bool,
    pub active: bool,
    pub paused: Option<bool>,
    pub woken: bool,
}

/// Demand-driven `CADisplayLink` frame state shared with GPUI's frame waker so
/// that waking a paused link does not need to acquire a platform window lock.
///
/// The link pauses after any tick in which GPUI neither presented nor asked for
/// another frame, so an idle window receives no callbacks.
#[derive(Default)]
pub struct DisplayLinkFrames {
    link: RefCell<Option<DisplayLink>>,
    /// Whether the owner currently permits display-link ticks (e.g. visible and
    /// not inside a synchronous transaction draw).
    active: Cell<bool>,
    /// Whether GPUI asked for a frame since the current tick began.
    woken: Cell<bool>,
    /// Whether GPUI has presented since the previous tick began.
    presented: Cell<bool>,
}

impl DisplayLinkFrames {
    /// Resumes the link, creating one bound to `source` on first start.
    ///
    /// # Safety
    ///
    /// `source` must be a valid Objective-C object whose
    /// `-displayLinkWithTarget:selector:` returns a `CADisplayLink *`.
    pub unsafe fn start_for_source(
        &self,
        source: &AnyObject,
        main_thread: MainThreadMarker,
        callback: impl Fn() + 'static,
    ) {
        self.active.set(true);
        let mut slot = self.link.borrow_mut();
        if let Some(link) = slot.as_ref() {
            link.set_paused(false);
            return;
        }
        // SAFETY: Caller guarantees `source` vends a `CADisplayLink *`.
        *slot = unsafe { DisplayLink::for_source(source, main_thread, callback) };
    }

    pub fn stop(&self) {
        self.active.set(false);
        self.presented.set(false);
        self.pause();
    }

    fn pause(&self) {
        if let Ok(link) = self.link.try_borrow()
            && let Some(link) = link.as_ref()
        {
            link.set_paused(true);
        }
    }

    pub fn invalidate(&self) {
        self.active.set(false);
        self.presented.set(false);
        drop(self.link.borrow_mut().take());
    }

    pub fn wake(&self) {
        self.woken.set(true);
        // The link is only borrowed mutably while being created in `start_for_source`,
        // where a new link already begins unpaused.
        if self.active.get()
            && let Ok(link) = self.link.try_borrow()
            && let Some(link) = link.as_ref()
        {
            link.set_paused(false);
        }
    }

    pub fn has_pending_wake(&self) -> bool {
        self.woken.get()
    }

    pub fn clear_pending_wake(&self) {
        self.woken.set(false);
    }

    pub fn did_present(&self) {
        self.presented.set(true);
    }

    /// Whether an input-triggered draw can present immediately without
    /// presenting more than once in the current refresh interval.
    pub fn can_draw_early(&self) -> bool {
        self.active.get() && !self.presented.get()
    }

    pub fn begin_frame(&self) {
        self.woken.set(false);
        self.presented.set(false);
    }

    /// Ends a tick, pausing the link if GPUI neither presented nor asked for
    /// another frame while it ran. Returns whether it paused.
    pub fn end_frame(&self) -> bool {
        let pause = !self.presented.get() && !self.woken.get();
        if pause {
            self.pause();
        }
        pause
    }

    pub fn debug_state(&self) -> DisplayLinkDebugState {
        let paused = self
            .link
            .try_borrow()
            .ok()
            .and_then(|link| link.as_ref().map(DisplayLink::is_paused));
        DisplayLinkDebugState {
            attached: paused.is_some(),
            active: self.active.get(),
            paused,
            woken: self.woken.get(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frames_pause_only_when_gpui_neither_presents_nor_asks_again() {
        let frames = DisplayLinkFrames::default();

        frames.begin_frame();
        frames.did_present();
        assert!(
            !frames.end_frame(),
            "a frame that presented keeps the link running"
        );

        frames.begin_frame();
        frames.wake();
        assert!(
            !frames.end_frame(),
            "a wake during the tick, like a throttled frame's retry, keeps it running"
        );

        frames.wake();
        frames.begin_frame();
        assert!(
            frames.end_frame(),
            "a wake before the tick was served by it"
        );

        frames.did_present();
        frames.begin_frame();
        assert!(
            frames.end_frame(),
            "a present before the tick, like an input-driven frame, was not this tick's"
        );

        frames.begin_frame();
        assert!(frames.end_frame(), "an idle tick pauses the link");
    }

    #[test]
    fn early_draws_cap_at_one_present_until_next_tick() {
        let frames = DisplayLinkFrames::default();
        assert!(
            !frames.can_draw_early(),
            "an inactive link does not draw early"
        );

        frames.active.set(true);
        assert!(
            frames.can_draw_early(),
            "idle active window draws the first keypress immediately"
        );

        frames.did_present();
        assert!(
            !frames.can_draw_early(),
            "a second keypress before vsync waits for the display-link tick"
        );

        frames.begin_frame();
        assert!(
            frames.can_draw_early(),
            "the next vsync tick re-allows an early draw once idle"
        );

        frames.did_present();
        frames.stop();
        assert!(!frames.can_draw_early());
        frames.active.set(true);
        assert!(
            frames.can_draw_early(),
            "stopping resets the presented flag"
        );
    }
}
