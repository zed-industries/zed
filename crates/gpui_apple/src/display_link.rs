use objc2::{
    AnyThread, DefinedClass, MainThreadMarker, MainThreadOnly, Message, define_class, msg_send,
    rc::Retained,
    runtime::{AnyClass, ProtocolObject},
};
use objc2_foundation::{NSObject, NSObjectProtocol, NSRunLoop, NSRunLoopCommonModes};
use objc2_quartz_core::{
    CAMetalDisplayLink, CAMetalDisplayLinkDelegate, CAMetalDisplayLinkUpdate, CAMetalLayer,
};

struct MetalDisplayLinkDelegateIvars {
    callback: Box<dyn Fn(&CAMetalDisplayLinkUpdate)>,
}

define_class!(
    // SAFETY: `NSObject` has no subclassing requirements.
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[ivars = MetalDisplayLinkDelegateIvars]
    struct MetalDisplayLinkDelegate;

    unsafe impl NSObjectProtocol for MetalDisplayLinkDelegate {}

    unsafe impl CAMetalDisplayLinkDelegate for MetalDisplayLinkDelegate {
        #[unsafe(method(metalDisplayLink:needsUpdate:))]
        fn needs_update(&self, _link: &CAMetalDisplayLink, update: &CAMetalDisplayLinkUpdate) {
            // The callback may destroy its frame source, releasing our owner's retain.
            let delegate = self.retain();
            (delegate.ivars().callback)(update);
        }
    }
);

impl MetalDisplayLinkDelegate {
    fn new(
        main_thread: MainThreadMarker,
        callback: impl Fn(&CAMetalDisplayLinkUpdate) + 'static,
    ) -> Retained<Self> {
        let this = Self::alloc(main_thread).set_ivars(MetalDisplayLinkDelegateIvars {
            callback: Box::new(callback),
        });
        // SAFETY: `NSObject`'s `init` is its designated initializer.
        unsafe { msg_send![super(this), init] }
    }
}

/// A main-run-loop source of Metal drawables on macOS 14+ and iOS/iPadOS 17+.
///
/// Callbacks run on the run loop the link is registered with:
/// <https://developer.apple.com/documentation/quartzcore/cametaldisplaylink>.
/// This wrapper uses the main run loop because GPUI renders on the foreground thread.
pub struct MetalFrameSource {
    link: Retained<CAMetalDisplayLink>,
    // The link holds its delegate weakly.
    _delegate: Retained<MetalDisplayLinkDelegate>,
    _main_thread: MainThreadMarker,
}

impl MetalFrameSource {
    /// Whether this system has `CAMetalDisplayLink` (macOS 14+, iOS/iPadOS 17+).
    pub fn is_available() -> bool {
        AnyClass::get(c"CAMetalDisplayLink").is_some()
    }

    /// Starts delivering updates for `layer`, or returns `None` where
    /// `CAMetalDisplayLink` is unavailable.
    pub fn new(
        layer: &CAMetalLayer,
        main_thread: MainThreadMarker,
        callback: impl Fn(&CAMetalDisplayLinkUpdate) + 'static,
    ) -> Option<Self> {
        if !Self::is_available() {
            return None;
        }
        let delegate = MetalDisplayLinkDelegate::new(main_thread, callback);
        let link = CAMetalDisplayLink::initWithMetalLayer(CAMetalDisplayLink::alloc(), layer);
        link.setDelegate(Some(ProtocolObject::from_ref(&*delegate)));
        // The default range follows the display and system power policy:
        // https://developer.apple.com/documentation/quartzcore/cametaldisplaylink/preferredframeraterange
        // SAFETY: the retained marker confines the source to the main thread,
        // and the main run loop outlives the source.
        unsafe { link.addToRunLoop_forMode(&NSRunLoop::mainRunLoop(), NSRunLoopCommonModes) };
        Some(Self {
            link,
            _delegate: delegate,
            _main_thread: main_thread,
        })
    }

    /// A paused link delivers no updates and acquires no drawables.
    pub fn set_paused(&self, paused: bool) {
        self.link.setPaused(paused);
    }
}

impl Drop for MetalFrameSource {
    fn drop(&mut self) {
        self.link.invalidate();
    }
}
