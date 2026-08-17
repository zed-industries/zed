//! # Interop between Metal and [`raw-window-handle`]
//!
//! Helpers for constructing a [`CAMetalLayer`] from a handle given by [`raw-window-handle`]. See
//! the [`Layer`] type for the full API.
//!
//! [`raw-window-handle`]: https://crates.io/crates/raw-window-handle
//!
//!
//! ## Example
//!
//! Create a layer from a window that implements [`HasWindowHandle`].
//!
//! ```
//! use objc2::rc::Retained;
//! use objc2_quartz_core::CAMetalLayer;
//! use raw_window_handle::{RawWindowHandle, HasWindowHandle};
//! use raw_window_metal::Layer;
//! #
//! # let mtm = objc2::MainThreadMarker::new().expect("doc tests to run on main thread");
//! #
//! # #[cfg(target_os = "macos")]
//! # let view = unsafe { objc2_app_kit::NSView::new(mtm) };
//! # #[cfg(target_os = "macos")]
//! # let handle = RawWindowHandle::AppKit(raw_window_handle::AppKitWindowHandle::new(std::ptr::NonNull::from(&*view).cast()));
//! #
//! # #[cfg(not(target_os = "macos"))]
//! # let view = unsafe { objc2_ui_kit::UIView::new(mtm) };
//! # #[cfg(not(target_os = "macos"))]
//! # let handle = RawWindowHandle::UiKit(raw_window_handle::UiKitWindowHandle::new(std::ptr::NonNull::from(&*view).cast()));
//! # let window = unsafe { raw_window_handle::WindowHandle::borrow_raw(handle) };
//!
//! let layer = match window.window_handle().expect("handle available").as_raw() {
//!     // SAFETY: The handle is a valid `NSView` because it came from `WindowHandle<'_>`.
//!     RawWindowHandle::AppKit(handle) => unsafe { Layer::from_ns_view(handle.ns_view) },
//!     // SAFETY: The handle is a valid `UIView` because it came from `WindowHandle<'_>`.
//!     RawWindowHandle::UiKit(handle) => unsafe { Layer::from_ui_view(handle.ui_view) },
//!     _ => panic!("unsupported handle"),
//! };
//! let layer: *mut CAMetalLayer = layer.into_raw().as_ptr().cast();
//! // SAFETY: The pointer is a valid `CAMetalLayer`, and because we consumed `Layer` with
//! // `into_raw`, the pointer has +1 retain count.
//! let layer = unsafe { Retained::from_raw(layer).unwrap() };
//!
//! // Use `CAMetalLayer` here.
//! ```
//!
//! [`HasWindowHandle`]: https://docs.rs/raw-window-handle/0.6.2/raw_window_handle/trait.HasWindowHandle.html
//!
//!
//! ## Semantics
//!
//! As the user of this crate, you are likely creating a library yourself, and need to interface
//! with a layer provided by a windowing library like Winit or SDL.
//!
//! In that sense, when the user hands your library a view or a layer via. `raw-window-handle`, they
//! likely expect you to render into it. You should freely do that, but you should refrain from
//! doing things like resizing the layer by changing its `bounds`, changing its `contentsGravity`,
//! `opacity`, and similar such properties; semantically, these are things that are "outside" of
//! your library's control, and interferes with the platforms normal handling of such things (i.e.
//! the user creating a `MTKView`, and placing it inside a `NSStackView`. In such cases, you should
//! not change the bounds of the view, as that will be done automatically at a "higher" level).
//!
//! Properties specific to `CAMetalLayer` like `drawableSize`, `colorspace` and so on probably _are_
//! fine to change, because these are properties that the user _expects_ your library to change when
//! they've given it to you (and they won't be changed by e.g. the layer being inside a stack view).
//!
//!
//! ## Reasoning behind creating a sublayer
//!
//! If a view does not have a `CAMetalLayer` as the root layer (as is the default for most views),
//! then we're in a bit of a tricky position! We cannot use the existing layer with Metal, so we
//! must do something else. There are a few options:
//!
//! 1. Panic, and require the user to pass a view with a `CAMetalLayer` layer.
//!
//!    While this would "work", it doesn't solve the problem, and instead passes the ball onwards to
//!    the user and ecosystem to figure it out.
//!
//! 2. Override the existing layer with a newly created layer.
//!
//!    If we overlook that this does not work in UIKit since `UIView`'s `layer` is `readonly`, and
//!    that as such we will need to do something different there anyhow, this is actually a fairly
//!    good solution, and was what the original implementation did.
//!
//!    It has some problems though, due to:
//!
//!    a. Consumers of `raw-window-metal` like Wgpu and Ash in their API design choosing not to
//!       register a callback with `-[CALayerDelegate displayLayer:]`, but instead leaves it up to
//!       the user to figure out when to redraw. That is, they rely on other libraries' callbacks
//!       telling them when to render.
//!
//!       (If you were to make an API only for Metal, you would probably make the user provide a
//!       `render` closure that'd be called in the right situations).
//!
//!    b. Overwriting the `layer` on `NSView` makes the view "layer-hosting", see [wantsLayer],
//!       which disables drawing functionality on the view like `drawRect:`/`updateLayer`.
//!
//!    These two in combination makes it basically impossible for crates like Winit to provide a
//!    robust rendering callback that integrates with the system's built-in mechanisms for
//!    redrawing, exactly because overwriting the layer would be disabling those mechanisms!
//!
//!    [wantsLayer]: https://developer.apple.com/documentation/appkit/nsview/1483695-wantslayer?language=objc
//!
//! 3. Create a sublayer.
//!
//!    `CALayer` has the concept of "sublayers", which we can use instead of overriding the layer.
//!
//!    This is also the recommended solution on UIKit, so it's nice that we can use the same
//!    implementation regardless of operating system.
//!
//!    It _might_, however, perform ever so slightly worse than overriding the layer directly.
//!
//! 4. Create a new `MTKView` (or a custom view), and add it as a subview.
//!
//!    Similar to creating a sublayer (see above), but also provides a bunch of event handling that
//!    we don't need.
//!
//! Option 3 seems like the most robust solution, so this is what this crate does.
//!
//! Now we have another problem though: The `bounds` and `contentsScale` of sublayers are not
//! automatically updated from the super layer.
//!
//! We could again choose to let that be up to the user of our crate, but that would be very
//! cumbersome. Instead, this crate registers the necessary observers to make the sublayer track the
//! size and scale factor of its super layer automatically. This makes it extra important that you
//! do not modify common `CALayer` properties of the layer that `raw-window-metal` creates, since
//! they may just end up being overwritten (see also "Semantics" above).

#![no_std]
#![cfg(target_vendor = "apple")]
#![cfg_attr(docsrs, feature(doc_auto_cfg, doc_cfg_hide), doc(cfg_hide(doc)))]
#![deny(unsafe_op_in_unsafe_fn)]
#![warn(clippy::undocumented_unsafe_blocks)]
// Update in Cargo.toml as well.
#![doc(html_root_url = "https://docs.rs/raw-window-metal/1.1.0")]

mod observer;

use core::ffi::{c_void, CStr};
use core::hash;
use core::panic::{RefUnwindSafe, UnwindSafe};
use core::ptr::NonNull;

use objc2::rc::Retained;
use objc2::runtime::AnyClass;
use objc2::{msg_send, ClassType, MainThreadMarker, Message};
use objc2_foundation::{NSObject, NSObjectProtocol};
use objc2_quartz_core::{CALayer, CAMetalLayer};

use crate::observer::ObserverLayer;

#[cfg(not(feature = "alloc"))]
compile_error!("The `alloc` feature must currently be enabled.");

#[cfg(not(feature = "std"))]
compile_error!("The `std` feature must currently be enabled.");

/// A wrapper around [`CAMetalLayer`].
#[doc(alias = "CAMetalLayer")]
#[derive(Debug, Clone)]
pub struct Layer {
    layer: Retained<CAMetalLayer>,
    pre_existing: bool,
}

impl PartialEq for Layer {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.layer.eq(&other.layer)
    }
}

impl Eq for Layer {}

impl hash::Hash for Layer {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.layer.hash(state);
    }
}

// SAFETY: `CAMetalLayer` is thread safe, like most things in Core Animation, see:
// https://developer.apple.com/documentation/quartzcore/catransaction/1448267-lock?language=objc
// https://stackoverflow.com/questions/76250226/how-to-render-content-of-calayer-on-a-background-thread
//
// TODO(madsmtm): Move this to `objc2-quartz-core`.
unsafe impl Send for Layer {}
// SAFETY: Same as above.
unsafe impl Sync for Layer {}

// Layer methods may panic, but that won't leave the layer in an invalid state.
//
// TODO(madsmtm): Move this to `objc2-quartz-core`.
impl UnwindSafe for Layer {}
impl RefUnwindSafe for Layer {}

impl Layer {
    /// Get a pointer to the underlying [`CAMetalLayer`].
    ///
    /// The pointer is valid for at least as long as the [`Layer`] is valid, but can be extended by
    /// retaining it.
    ///
    /// You should usually not change general `CALayer` properties like `bounds`, `contentsScale`
    /// and so on of this layer, but instead modify the layer that it was created from.
    ///
    /// You can safely modify `CAMetalLayer` properties like `drawableSize` to match your needs,
    /// though beware that if it does not match the actual size of the layer, the contents will be
    /// scaled.
    ///
    ///
    /// # Example
    ///
    /// ```no_run
    /// use objc2_quartz_core::CAMetalLayer;
    /// use raw_window_metal::Layer;
    ///
    /// let layer: Layer;
    /// # layer = unimplemented!();
    ///
    /// // SAFETY: The pointer is a valid `CAMetalLayer`.
    /// let layer: &CAMetalLayer = unsafe { layer.as_ptr().cast().as_ref() };
    ///
    /// // Use the `CAMetalLayer` here.
    /// ```
    #[inline]
    pub fn as_ptr(&self) -> NonNull<c_void> {
        let ptr: *const CAMetalLayer = Retained::as_ptr(&self.layer);
        // Unwrap is fine, `Retained` stores a non-null pointer
        NonNull::new(ptr as *mut _).unwrap()
    }

    /// Consume the layer, and return a pointer with +1 retain count to the underlying
    /// [`CAMetalLayer`].
    ///
    /// After calling this function, the caller is responsible for releasing the pointer, otherwise
    /// the layer will be leaked.
    ///
    ///
    /// # Example
    ///
    /// Convert a layer to a [`Retained`] `CAMetalLayer`.
    ///
    /// ```no_run
    /// use objc2::rc::Retained;
    /// use objc2_quartz_core::CAMetalLayer;
    /// use raw_window_metal::Layer;
    ///
    /// let layer: Layer;
    /// # layer = unimplemented!();
    ///
    /// let layer: *mut CAMetalLayer = layer.into_raw().as_ptr().cast();
    /// // SAFETY: The pointer is a valid `CAMetalLayer`, and because we consumed `Layer` with
    /// // `into_raw`, the pointer has +1 retain count.
    /// let layer = unsafe { Retained::from_raw(layer).unwrap() };
    ///
    /// // Use the `CAMetalLayer` here.
    /// ```
    #[inline]
    pub fn into_raw(self) -> NonNull<c_void> {
        // Unwrap is fine, `Retained` stores a non-null pointer
        NonNull::new(Retained::into_raw(self.layer).cast()).unwrap()
    }

    /// If `raw-window-metal` created a new [`CAMetalLayer`] for you, this returns `false`.
    ///
    /// This may be useful if you want to override some part of `raw-window-metal`'s behaviour, and
    /// need to do so based on whether it ended up creating a layer or not.
    ///
    /// You should try to avoid this, and instead:
    /// - Modify `CALayer` properties on the layer that you created this from.
    /// - Modify `CAMetalLayer` properties on the layer returned from `as_ptr`.
    #[inline]
    pub fn pre_existing(&self) -> bool {
        self.pre_existing
    }

    /// Get or create a new `CAMetalLayer` from the given `CALayer`.
    ///
    /// If the given layer is a `CAMetalLayer`, this will simply return that layer. If not, a new
    /// `CAMetalLayer` is created and inserted as a sublayer, and then configured such that it will
    /// have the same bounds and scale factor as the given layer.
    ///
    ///
    /// # Safety
    ///
    /// The given layer pointer must be a valid instance of `CALayer`.
    ///
    ///
    /// # Examples
    ///
    /// Create a new layer from a `CAMetalLayer`.
    ///
    /// ```
    /// use std::ptr::NonNull;
    /// use objc2_quartz_core::CAMetalLayer;
    /// use raw_window_metal::Layer;
    ///
    /// let layer = unsafe { CAMetalLayer::new() };
    /// let ptr: NonNull<CAMetalLayer> = NonNull::from(&*layer);
    ///
    /// let layer = unsafe { Layer::from_ca_layer(ptr.cast()) };
    /// assert!(layer.pre_existing());
    /// ```
    ///
    /// Create a `CAMetalLayer` sublayer in a `CALayer`.
    ///
    /// ```
    /// use std::ptr::NonNull;
    /// use objc2_quartz_core::CALayer;
    /// use raw_window_metal::Layer;
    ///
    /// let layer = CALayer::new();
    /// let ptr: NonNull<CALayer> = NonNull::from(&*layer);
    ///
    /// let layer = unsafe { Layer::from_ca_layer(ptr.cast()) };
    /// assert!(!layer.pre_existing());
    /// ```
    pub unsafe fn from_ca_layer(layer_ptr: NonNull<c_void>) -> Self {
        // SAFETY: Caller ensures that the pointer is a valid `CALayer`.
        let root_layer: &CALayer = unsafe { layer_ptr.cast().as_ref() };

        // Debug check that the given layer actually _is_ a CALayer.
        if cfg!(debug_assertions) {
            assert!(
                root_layer.isKindOfClass(CALayer::class()),
                "view was not a valid CALayer"
            );
        }

        if let Some(layer) = root_layer.downcast_ref::<CAMetalLayer>() {
            Layer {
                layer: layer.retain(),
                pre_existing: true,
            }
        } else {
            let layer = ObserverLayer::new(root_layer);
            Layer {
                layer: layer.into_super(),
                pre_existing: false,
            }
        }
    }

    fn from_retained_layer(root_layer: Retained<CALayer>) -> Self {
        match root_layer.downcast::<CAMetalLayer>() {
            Ok(layer) => Layer {
                layer,
                pre_existing: true,
            },
            Err(root_layer) => {
                let layer = ObserverLayer::new(&root_layer);
                Layer {
                    layer: layer.into_super(),
                    pre_existing: false,
                }
            }
        }
    }

    /// Get or create a new `CAMetalLayer` from the given `NSView`.
    ///
    /// If the given view is not [layer-backed], it will be made so.
    ///
    /// If the given view has a `CAMetalLayer` as the root layer (which can happen for example if
    /// the view has overwritten `-[NSView layerClass]` or the view is `MTKView`) this will simply
    /// return that layer. If not, a new `CAMetalLayer` is created and inserted as a sublayer into
    /// the view's layer, and then configured such that it will have the same bounds and scale
    /// factor as the given view.
    ///
    ///
    /// # Panics
    ///
    /// Panics if called from a thread that is not the main thread.
    ///
    ///
    /// # Safety
    ///
    /// The given view pointer must be a valid instance of `NSView`.
    ///
    ///
    /// # Example
    ///
    /// Construct a layer from an [`AppKitWindowHandle`].
    ///
    /// ```
    /// use raw_window_handle::AppKitWindowHandle;
    /// use raw_window_metal::Layer;
    ///
    /// let handle: AppKitWindowHandle;
    /// # let mtm = objc2::MainThreadMarker::new().expect("doc tests to run on main thread");
    /// # #[cfg(target_os = "macos")]
    /// # let view = unsafe { objc2_app_kit::NSView::new(mtm) };
    /// # #[cfg(target_os = "macos")]
    /// # { handle = AppKitWindowHandle::new(std::ptr::NonNull::from(&*view).cast()) };
    /// # #[cfg(not(target_os = "macos"))]
    /// # { handle = unimplemented!() };
    /// let layer = unsafe { Layer::from_ns_view(handle.ns_view) };
    /// ```
    ///
    /// [layer-backed]: https://developer.apple.com/documentation/appkit/nsview/1483695-wantslayer?language=objc
    /// [`AppKitWindowHandle`]: https://docs.rs/raw-window-handle/0.6.2/raw_window_handle/struct.AppKitWindowHandle.html
    pub unsafe fn from_ns_view(ns_view_ptr: NonNull<c_void>) -> Self {
        let _mtm = MainThreadMarker::new().expect("can only access NSView on the main thread");

        // SAFETY: Caller ensures that the pointer is a valid `NSView`.
        //
        // We use `NSObject` here to avoid importing `objc2-app-kit`.
        let ns_view: &NSObject = unsafe { ns_view_ptr.cast().as_ref() };

        // Debug check that the given view actually _is_ a NSView.
        if cfg!(debug_assertions) {
            // Load the class at runtime (instead of using `class!`)
            // to ensure that this still works if AppKit isn't linked.
            let cls = AnyClass::get(CStr::from_bytes_with_nul(b"NSView\0").unwrap()).unwrap();
            assert!(ns_view.isKindOfClass(cls), "view was not a valid NSView");
        }

        // Force the view to become layer backed
        // SAFETY: The signature of `NSView::setWantsLayer` is correctly specified.
        let _: () = unsafe { msg_send![ns_view, setWantsLayer: true] };

        // SAFETY: `-[NSView layer]` returns an optional `CALayer`
        let root_layer: Option<Retained<CALayer>> = unsafe { msg_send![ns_view, layer] };
        let root_layer = root_layer.expect("failed making the view layer-backed");

        Self::from_retained_layer(root_layer)
    }

    /// Get or create a new `CAMetalLayer` from the given `UIView`.
    ///
    /// If the given view has a `CAMetalLayer` as the root layer (which can happen for example if
    /// the view has overwritten `-[UIView layerClass]` or the view is `MTKView`) this will simply
    /// return that layer. If not, a new `CAMetalLayer` is created and inserted as a sublayer into
    /// the view's layer, and then configured such that it will have the same bounds and scale
    /// factor as the given view.
    ///
    ///
    /// # Panics
    ///
    /// Panics if called from a thread that is not the main thread.
    ///
    ///
    /// # Safety
    ///
    /// The given view pointer must be a valid instance of `UIView`.
    ///
    ///
    /// # Example
    ///
    /// Construct a layer from a [`UiKitWindowHandle`].
    ///
    /// ```no_run
    /// use raw_window_handle::UiKitWindowHandle;
    /// use raw_window_metal::Layer;
    ///
    /// let handle: UiKitWindowHandle;
    /// # handle = unimplemented!();
    /// let layer = unsafe { Layer::from_ui_view(handle.ui_view) };
    /// ```
    ///
    /// [`UiKitWindowHandle`]: https://docs.rs/raw-window-handle/0.6.2/raw_window_handle/struct.UiKitWindowHandle.html
    pub unsafe fn from_ui_view(ui_view_ptr: NonNull<c_void>) -> Self {
        let _mtm = MainThreadMarker::new().expect("can only access UIView on the main thread");

        // SAFETY: Caller ensures that the pointer is a valid `UIView`.
        //
        // We use `NSObject` here to avoid importing `objc2-ui-kit`.
        let ui_view: &NSObject = unsafe { ui_view_ptr.cast().as_ref() };

        // Debug check that the given view actually _is_ a UIView.
        if cfg!(debug_assertions) {
            // Load the class at runtime (instead of using `class!`)
            // to ensure that this still works if UIKit isn't linked.
            let cls = AnyClass::get(CStr::from_bytes_with_nul(b"UIView\0").unwrap()).unwrap();
            assert!(ui_view.isKindOfClass(cls), "view was not a valid UIView");
        }

        // SAFETY: `-[UIView layer]` returns a non-optional `CALayer`
        let root_layer: Retained<CALayer> = unsafe { msg_send![ui_view, layer] };

        // Unlike on macOS, we cannot replace the main view as `UIView` does
        // not allow it (when `NSView` does).
        Self::from_retained_layer(root_layer)
    }
}
