use core::ffi::c_void;
use objc2::rc::{Retained, Weak};
use objc2::runtime::{AnyClass, AnyObject};
use objc2::{define_class, msg_send, AllocAnyThread, ClassType, DefinedClass};
use objc2_foundation::{
    ns_string, NSDictionary, NSKeyValueChangeKey, NSKeyValueChangeNewKey,
    NSKeyValueObservingOptions, NSNumber, NSObjectNSKeyValueObserverRegistration, NSString,
    NSValue,
};
use objc2_quartz_core::{CALayer, CAMetalLayer};

define_class!(
    /// A `CAMetalLayer` layer that will automatically update its bounds and scale factor to match
    /// its super layer.
    ///
    /// We do this by subclassing, to allow the user to just store the layer as
    /// `Retained<CAMetalLayer>`, and still have our observers work.
    ///
    /// See the documentation on Key-Value Observing for details on how this works in general:
    /// <https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/KeyValueObserving/KeyValueObserving.html>
    //
    // SAFETY:
    // - The superclass CAMetalLayer does not have any subclassing requirements.
    // - CustomLayer implements `Drop` and ensures that:
    //   - It does not call an overridden method.
    //   - It does not `retain` itself.
    #[unsafe(super(CAMetalLayer))]
    #[name = "RawWindowMetalLayer"]
    #[ivars = Weak<CALayer>]
    pub(crate) struct ObserverLayer;

    // `NSKeyValueObserving` category.
    //
    // SAFETY: The method is correctly defined.
    impl ObserverLayer {
        #[unsafe(method(observeValueForKeyPath:ofObject:change:context:))]
        fn _observe_value(
            &self,
            key_path: Option<&NSString>,
            object: Option<&AnyObject>,
            change: Option<&NSDictionary<NSKeyValueChangeKey, AnyObject>>,
            context: *mut c_void,
        ) {
            self.observe_value(key_path, object, change, context);
        }
    }
);

impl Drop for ObserverLayer {
    fn drop(&mut self) {
        // It is possible for the root layer to be de-allocated before the custom layer.
        //
        // In that case, the observer is already de-registered, and so we don't need to do anything.
        //
        // We use a weak variable here to avoid issues if the layer was removed from the super
        // layer, and then later de-allocated, without de-registering these observers.
        if let Some(root_layer) = self.ivars().load() {
            // SAFETY: The observer is registered for these key paths in `new`.
            unsafe {
                root_layer.removeObserver_forKeyPath(self, ns_string!("contentsScale"));
                root_layer.removeObserver_forKeyPath(self, ns_string!("bounds"));
            }
        }
    }
}

impl ObserverLayer {
    /// The context pointer, to differentiate between key-value observing registered by this class,
    /// and the superclass.
    fn context() -> *mut c_void {
        ObserverLayer::class() as *const AnyClass as *mut c_void
    }

    /// Create a new custom layer that tracks parameters from the given super layer.
    pub fn new(root_layer: &CALayer) -> Retained<Self> {
        let this = Self::alloc().set_ivars(Weak::new(root_layer));
        // SAFETY: Initializing `CAMetalLayer` is safe.
        let this: Retained<Self> = unsafe { msg_send![super(this), init] };

        // Add the layer as a sublayer of the root layer.
        root_layer.addSublayer(&this);

        // Do not use auto-resizing mask.
        //
        // This is done to work around a bug in macOS 14 and above, where views using auto layout
        // may end up setting fractional values as the bounds, and that in turn doesn't propagate
        // properly through the auto-resizing mask and with contents gravity.
        //
        // Instead, we keep the bounds of the layer in sync with the root layer using an observer,
        // see below.
        //
        // this.setAutoresizingMask(kCALayerHeightSizable | kCALayerWidthSizable);

        // AppKit / UIKit automatically sets the correct scale factor and bounds for layers attached
        // to a view. Our layer, however, is not directly attached to a view, and so we need to
        // observe changes to the root layer's parameters, and apply them to our layer.
        //
        // Note the use of `NSKeyValueObservingOptionInitial` to also set the initial values here.
        //
        // Note that for AppKit, we _could_ make the layer match the window by adding a delegate on
        // the layer with the `layer:shouldInheritContentsScale:fromWindow:` method returning `true`
        // - this tells the system to automatically update the scale factor when it changes on a
        // window. But this wouldn't support headless rendering very well, and doesn't work on UIKit
        // anyhow, so we might as well just always use the observer technique.
        //
        // SAFETY: Observer deregistered in `Drop` before the observer object is deallocated.
        unsafe {
            root_layer.addObserver_forKeyPath_options_context(
                &this,
                ns_string!("contentsScale"),
                NSKeyValueObservingOptions::New | NSKeyValueObservingOptions::Initial,
                ObserverLayer::context(),
            );
            root_layer.addObserver_forKeyPath_options_context(
                &this,
                ns_string!("bounds"),
                NSKeyValueObservingOptions::New | NSKeyValueObservingOptions::Initial,
                ObserverLayer::context(),
            );
        }

        // The default content gravity (`kCAGravityResize`) is a fine choice for most applications,
        // as it masks / alleviates issues with resizing and behaves better when moving the window
        // between monitors, so we won't modify that.
        //
        // Unfortunately, it may also make it harder to debug resize issues, swap this for
        // `kCAGravityTopLeft` instead when doing so.
        //
        // this.setContentsGravity(unsafe { kCAGravityResize });

        this
    }

    fn observe_value(
        &self,
        key_path: Option<&NSString>,
        object: Option<&AnyObject>,
        change: Option<&NSDictionary<NSKeyValueChangeKey, AnyObject>>,
        context: *mut c_void,
    ) {
        // An unrecognized context must belong to the super class.
        if context != ObserverLayer::context() {
            // SAFETY: The signature is correct, and it's safe to forward to the superclass' method
            // when we're overriding the method.
            return unsafe {
                msg_send![
                    super(self),
                    observeValueForKeyPath: key_path,
                    ofObject: object,
                    change: change,
                    context: context,
                ]
            };
        }

        let change =
            change.expect("requested a change dictionary in `addObserver`, but none was provided");
        // SAFETY: The static is declared with the correct type in `objc2`.
        let key = unsafe { NSKeyValueChangeNewKey };
        let new = change
            .objectForKey(key)
            .expect("requested change dictionary did not contain `NSKeyValueChangeNewKey`");

        // NOTE: Setting these values usually causes a quarter second animation to occur, which is
        // undesirable.
        //
        // However, since we're setting them inside an observer, there already is a transaction
        // ongoing, and as such we don't need to wrap this in a `CATransaction` ourselves.

        if key_path == Some(ns_string!("contentsScale")) {
            // `contentsScale` is a CGFloat, and so the observed value is always a NSNumber.
            let new = new.downcast::<NSNumber>().unwrap();
            let scale_factor = new.as_cgfloat();

            // Set the scale factor of the layer to match the root layer when it changes (e.g. if
            // moved to a different monitor, or monitor settings changed).
            self.setContentsScale(scale_factor);
        } else if key_path == Some(ns_string!("bounds")) {
            // `bounds` is a CGRect, and so the observed value is always a NSNumber.
            let new = new.downcast::<NSValue>().unwrap();
            let bounds = new.get_rect().expect("new bounds value was not CGRect");

            // Set `bounds` and `position` so that the new layer is inside the superlayer.
            //
            // This differs from just setting the `bounds`, as it also takes into account any
            // translation that the superlayer may have that we'd want to preserve.
            self.setFrame(bounds);
        } else {
            panic!("unknown observed keypath {key_path:?}");
        }
    }
}

#[cfg(test)]
mod tests {
    use objc2_core_foundation::{CGPoint, CGRect, CGSize};

    use super::*;

    #[test]
    fn release_order_does_not_matter() {
        let root_layer = CALayer::new();
        let layer = ObserverLayer::new(&root_layer);
        drop(root_layer);
        drop(layer);

        let root_layer = CALayer::new();
        let layer = ObserverLayer::new(&root_layer);
        drop(layer);
        drop(root_layer);
    }

    #[test]
    fn scale_factor_propagates() {
        let root_layer = CALayer::new();
        let layer = ObserverLayer::new(&root_layer);

        root_layer.setContentsScale(3.0);
        assert_eq!(layer.contentsScale(), 3.0);
    }

    #[test]
    fn bounds_propagates() {
        let root_layer = CALayer::new();
        let layer = ObserverLayer::new(&root_layer);

        root_layer.setBounds(CGRect::new(
            CGPoint::new(10.0, 20.0),
            CGSize::new(30.0, 40.0),
        ));
        assert_eq!(layer.position(), CGPoint::new(25.0, 40.0));
        assert_eq!(
            layer.bounds(),
            CGRect::new(CGPoint::new(0.0, 0.0), CGSize::new(30.0, 40.0),)
        );
    }

    #[test]
    fn superlayer_can_remove_all_sublayers() {
        let root_layer = CALayer::new();
        let layer = ObserverLayer::new(&root_layer);
        layer.removeFromSuperlayer();
        drop(layer);
        root_layer.setContentsScale(3.0);
    }
}
