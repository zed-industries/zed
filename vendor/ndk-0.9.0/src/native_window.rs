//! Bindings for [`ANativeWindow`]
//!
//! [`ANativeWindow`]: https://developer.android.com/ndk/reference/group/a-native-window#anativewindow

use std::{ffi::c_void, io, mem::MaybeUninit, ptr::NonNull};

use jni_sys::{jobject, JNIEnv};

use super::{hardware_buffer_format::HardwareBufferFormat, utils::status_to_io_result};
#[cfg(all(feature = "nativewindow", feature = "api-level-28"))]
use crate::data_space::DataSpace;

pub type Rect = ffi::ARect;

// [`NativeWindow`] represents the producer end of an image queue
///
/// It is the C counterpart of the [`android.view.Surface`] object in Java, and can be converted
/// both ways. Depending on the consumer, images submitted to [`NativeWindow`] can be shown on the
/// display or sent to other consumers, such as video encoders.
///
/// [`android.view.Surface`]: https://developer.android.com/reference/android/view/Surface
#[derive(Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NativeWindow {
    ptr: NonNull<ffi::ANativeWindow>,
}

unsafe impl Send for NativeWindow {}
unsafe impl Sync for NativeWindow {}

impl Drop for NativeWindow {
    fn drop(&mut self) {
        unsafe { ffi::ANativeWindow_release(self.ptr.as_ptr()) }
    }
}

impl Clone for NativeWindow {
    fn clone(&self) -> Self {
        unsafe { ffi::ANativeWindow_acquire(self.ptr.as_ptr()) }
        Self { ptr: self.ptr }
    }
}

#[cfg(feature = "rwh_04")]
unsafe impl rwh_04::HasRawWindowHandle for NativeWindow {
    fn raw_window_handle(&self) -> rwh_04::RawWindowHandle {
        let mut handle = rwh_04::AndroidNdkHandle::empty();
        handle.a_native_window = self.ptr.as_ptr().cast();
        rwh_04::RawWindowHandle::AndroidNdk(handle)
    }
}

#[cfg(feature = "rwh_05")]
unsafe impl rwh_05::HasRawWindowHandle for NativeWindow {
    fn raw_window_handle(&self) -> rwh_05::RawWindowHandle {
        let mut handle = rwh_05::AndroidNdkWindowHandle::empty();
        handle.a_native_window = self.ptr.as_ptr().cast();
        rwh_05::RawWindowHandle::AndroidNdk(handle)
    }
}

#[cfg(feature = "rwh_06")]
impl rwh_06::HasWindowHandle for NativeWindow {
    fn window_handle(&self) -> Result<rwh_06::WindowHandle<'_>, rwh_06::HandleError> {
        let handle = rwh_06::AndroidNdkWindowHandle::new(self.ptr.cast());
        let handle = rwh_06::RawWindowHandle::AndroidNdk(handle);
        // SAFETY: All fields of the "raw" `AndroidNdkWindowHandle` struct are filled out.  The
        // returned pointer is also kept valid by `NativeWindow` (until `Drop`), which is lifetime-
        // borrowed in the returned `WindowHandle<'_>` and cannot be outlived.  Its value won't
        // change throughout the lifetime of this `NativeWindow`.
        Ok(unsafe { rwh_06::WindowHandle::borrow_raw(handle) })
    }
}

impl NativeWindow {
    /// Assumes ownership of `ptr`
    ///
    /// # Safety
    /// `ptr` must be a valid pointer to an Android [`ffi::ANativeWindow`].
    pub unsafe fn from_ptr(ptr: NonNull<ffi::ANativeWindow>) -> Self {
        Self { ptr }
    }

    /// Acquires ownership of `ptr`
    ///
    /// # Safety
    /// `ptr` must be a valid pointer to an Android [`ffi::ANativeWindow`].
    pub unsafe fn clone_from_ptr(ptr: NonNull<ffi::ANativeWindow>) -> Self {
        ffi::ANativeWindow_acquire(ptr.as_ptr());
        Self::from_ptr(ptr)
    }

    pub fn ptr(&self) -> NonNull<ffi::ANativeWindow> {
        self.ptr
    }

    pub fn height(&self) -> i32 {
        unsafe { ffi::ANativeWindow_getHeight(self.ptr.as_ptr()) }
    }

    pub fn width(&self) -> i32 {
        unsafe { ffi::ANativeWindow_getWidth(self.ptr.as_ptr()) }
    }

    /// Return the current pixel format ([`HardwareBufferFormat`]) of the window surface.
    pub fn format(&self) -> HardwareBufferFormat {
        let value = unsafe { ffi::ANativeWindow_getFormat(self.ptr.as_ptr()) };
        value.into()
    }

    /// Change the format and size of the window buffers.
    ///
    /// The width and height control the number of pixels in the buffers, not the dimensions of the
    /// window on screen. If these are different than the window's physical size, then its buffer
    /// will be scaled to match that size when compositing it to the screen. The width and height
    /// must be either both zero or both non-zero.
    ///
    /// For all of these parameters, if `0` or [`None`] is supplied then the window's base value
    /// will come back in force.
    pub fn set_buffers_geometry(
        &self,
        width: i32,
        height: i32,
        format: Option<HardwareBufferFormat>,
    ) -> io::Result<()> {
        let format = format.map_or(0i32, |f| f.into());
        let status = unsafe {
            ffi::ANativeWindow_setBuffersGeometry(self.ptr.as_ptr(), width, height, format)
        };
        status_to_io_result(status)
    }

    /// Set a transform that will be applied to future buffers posted to the window.
    #[cfg(all(feature = "nativewindow", feature = "api-level-26"))]
    #[doc(alias = "ANativeWindow_setBuffersTransform")]
    pub fn set_buffers_transform(&self, transform: NativeWindowTransform) -> io::Result<()> {
        let status =
            unsafe { ffi::ANativeWindow_setBuffersTransform(self.ptr.as_ptr(), transform.bits()) };
        status_to_io_result(status)
    }

    /// All buffers queued after this call will be associated with the dataSpace parameter
    /// specified.
    ///
    /// `data_space` specifies additional information about the buffer. For example, it can be used
    /// to convey the color space of the image data in the buffer, or it can be used to indicate
    /// that the buffers contain depth measurement data instead of color images. The default
    /// dataSpace is `0`, [`DataSpace::Unknown`], unless it has been overridden by the producer.
    #[cfg(all(feature = "nativewindow", feature = "api-level-28"))]
    #[doc(alias = "ANativeWindow_setBuffersDataSpace")]
    pub fn set_buffers_data_space(&self, data_space: DataSpace) -> io::Result<()> {
        let status =
            unsafe { ffi::ANativeWindow_setBuffersDataSpace(self.ptr.as_ptr(), data_space.into()) };
        status_to_io_result(status)
    }

    /// Get the dataspace of the buffers in this [`NativeWindow`].
    #[cfg(all(feature = "nativewindow", feature = "api-level-28"))]
    #[doc(alias = "ANativeWindow_getBuffersDataSpace")]
    pub fn buffers_data_space(&self) -> io::Result<DataSpace> {
        let status = unsafe { ffi::ANativeWindow_getBuffersDataSpace(self.ptr.as_ptr()) };
        if status >= 0 {
            Ok(status.into())
        } else {
            Err(status_to_io_result(status).unwrap_err())
        }
    }

    /// Sets the intended frame rate for this window.
    ///
    /// Same as [`set_frame_rate_with_change_strategy(window, frame_rate, compatibility, ChangeFrameRateStrategy::OnlyIfSeamless)`][`NativeWindow::set_frame_rate_with_change_strategy()`].
    ///
    #[cfg_attr(
        not(feature = "api-level-31"),
        doc = "[`NativeWindow::set_frame_rate_with_change_strategy()`]: https://developer.android.com/ndk/reference/group/a-native-window#anativewindow_setframeratewithchangestrategy"
    )]
    #[cfg(all(feature = "nativewindow", feature = "api-level-30"))]
    #[doc(alias = "ANativeWindow_setFrameRate")]
    pub fn set_frame_rate(
        &self,
        frame_rate: f32,
        compatibility: FrameRateCompatibility,
    ) -> io::Result<()> {
        let status = unsafe {
            ffi::ANativeWindow_setFrameRate(self.ptr.as_ptr(), frame_rate, compatibility as i8)
        };
        status_to_io_result(status)
    }

    /// Sets the intended frame rate for this window.
    ///
    /// On devices that are capable of running the display at different refresh rates, the system
    /// may choose a display refresh rate to better match this window's frame rate. Usage of this
    /// API won't introduce frame rate throttling, or affect other aspects of the application's
    /// frame production pipeline. However, because the system may change the display refresh rate,
    /// calls to this function may result in changes to Choreographer callback timings, and changes
    /// to the time interval at which the system releases buffers back to the application.
    ///
    /// Note that this only has an effect for windows presented on the display. If this
    /// [`NativeWindow`] is consumed by something other than the system compositor, e.g. a media
    /// codec, this call has no effect.
    ///
    /// You can register for changes in the refresh rate using
    /// [`ffi::AChoreographer_registerRefreshRateCallback()`].
    ///
    /// # Parameters
    ///
    /// - `frame_rate`: The intended frame rate of this window, in frames per second. `0` is a
    ///   special value that indicates the app will accept the system's choice for the display
    ///   frame rate, which is the default behavior if this function isn't called. The `frame_rate`
    ///   param does not need to be a valid refresh rate for this device's display - e.g., it's
    ///   fine to pass `30`fps to a device that can only run the display at `60`fps.
    /// - `compatibility`: The frame rate compatibility of this window. The compatibility value may
    ///   influence the system's choice of display refresh rate. See the [`FrameRateCompatibility`]
    ///   values for more info. This parameter is ignored when `frame_rate` is `0`.
    /// - `change_frame_rate_strategy`: Whether display refresh rate transitions caused by this
    ///   window should be seamless. A seamless transition is one that doesn't have any visual
    ///   interruptions, such as a black screen for a second or two. See the
    ///   [`ChangeFrameRateStrategy`] values. This parameter is ignored when `frame_rate` is `0`.
    #[cfg(all(feature = "nativewindow", feature = "api-level-31"))]
    #[doc(alias = "ANativeWindow_setFrameRateWithChangeStrategy")]
    pub fn set_frame_rate_with_change_strategy(
        &self,
        frame_rate: f32,
        compatibility: FrameRateCompatibility,
        change_frame_rate_strategy: ChangeFrameRateStrategy,
    ) -> io::Result<()> {
        let status = unsafe {
            ffi::ANativeWindow_setFrameRateWithChangeStrategy(
                self.ptr.as_ptr(),
                frame_rate,
                compatibility as i8,
                change_frame_rate_strategy as i8,
            )
        };
        status_to_io_result(status)
    }

    /// Provides a hint to the window that buffers should be preallocated ahead of time.
    ///
    /// Note that the window implementation is not guaranteed to preallocate any buffers, for
    /// instance if an implementation disallows allocation of new buffers, or if there is
    /// insufficient memory in the system to preallocate additional buffers
    #[cfg(all(feature = "nativewindow", feature = "api-level-30"))]
    pub fn try_allocate_buffers(&self) {
        unsafe { ffi::ANativeWindow_tryAllocateBuffers(self.ptr.as_ptr()) }
    }

    /// Return the [`NativeWindow`] associated with a JNI [`android.view.Surface`] pointer.
    ///
    /// # Safety
    /// By calling this function, you assert that `env` is a valid pointer to a [`JNIEnv`] and
    /// `surface` is a valid pointer to an [`android.view.Surface`].
    ///
    /// [`android.view.Surface`]: https://developer.android.com/reference/android/view/Surface
    pub unsafe fn from_surface(env: *mut JNIEnv, surface: jobject) -> Option<Self> {
        let ptr = ffi::ANativeWindow_fromSurface(env, surface);
        Some(Self::from_ptr(NonNull::new(ptr)?))
    }

    /// Return a JNI [`android.view.Surface`] pointer derived from this [`NativeWindow`].
    ///
    /// # Safety
    /// By calling this function, you assert that `env` is a valid pointer to a [`JNIEnv`].
    ///
    /// [`android.view.Surface`]: https://developer.android.com/reference/android/view/Surface
    #[cfg(feature = "api-level-26")]
    pub unsafe fn to_surface(&self, env: *mut JNIEnv) -> jobject {
        ffi::ANativeWindow_toSurface(env, self.ptr().as_ptr())
    }

    /// Lock the window's next drawing surface for writing.
    ///
    /// Optionally pass the region you intend to draw into `dirty_bounds`.  When this function
    /// returns it is updated (commonly enlarged) with the actual area the caller needs to redraw.
    pub fn lock(
        &self,
        dirty_bounds: Option<&mut Rect>,
    ) -> io::Result<NativeWindowBufferLockGuard<'_>> {
        let dirty_bounds = match dirty_bounds {
            Some(dirty_bounds) => dirty_bounds,
            None => std::ptr::null_mut(),
        };
        let mut buffer = MaybeUninit::uninit();
        let status = unsafe {
            ffi::ANativeWindow_lock(self.ptr.as_ptr(), buffer.as_mut_ptr(), dirty_bounds)
        };
        status_to_io_result(status)?;

        Ok(NativeWindowBufferLockGuard {
            window: self,
            buffer: unsafe { buffer.assume_init() },
        })
    }
}

/// Lock holding the next drawing surface for writing.  It is unlocked and posted on [`drop()`].
#[derive(Debug)]
pub struct NativeWindowBufferLockGuard<'a> {
    window: &'a NativeWindow,
    buffer: ffi::ANativeWindow_Buffer,
}

impl<'a> NativeWindowBufferLockGuard<'a> {
    /// The number of pixels that are shown horizontally.
    pub fn width(&self) -> usize {
        usize::try_from(self.buffer.width).unwrap()
    }

    // The number of pixels that are shown vertically.
    pub fn height(&self) -> usize {
        usize::try_from(self.buffer.height).unwrap()
    }

    /// The number of _pixels_ that a line in the buffer takes in memory.
    ///
    /// This may be `>= width`.
    pub fn stride(&self) -> usize {
        usize::try_from(self.buffer.stride).unwrap()
    }

    /// The format of the buffer. One of [`HardwareBufferFormat`].
    pub fn format(&self) -> HardwareBufferFormat {
        self.buffer.format.into()
    }

    /// The actual bits.
    ///
    /// This points to a memory segment of [`stride()`][Self::stride()] *
    /// [`height()`][Self::height()] * [`HardwareBufferFormat::bytes_per_pixel()`] bytes.
    ///
    /// Only [`width()`][Self::width()] pixels are visible for each [`stride()`][Self::stride()]
    /// line of pixels in the buffer.
    ///
    /// See [`bytes()`][Self::bytes()] for safe access to these bytes.
    pub fn bits(&mut self) -> *mut c_void {
        self.buffer.bits
    }

    /// Safe write access to likely uninitialized pixel buffer data.
    ///
    /// Returns [`None`] when there is no [`HardwareBufferFormat::bytes_per_pixel()`] size
    /// available for this [`format()`][Self::format()].
    ///
    /// The returned slice consists of [`stride()`][Self::stride()] * [`height()`][Self::height()]
    /// \* [`HardwareBufferFormat::bytes_per_pixel()`] bytes.
    ///
    /// Only [`width()`][Self::width()] pixels are visible for each [`stride()`][Self::stride()]
    /// line of pixels in the buffer.
    pub fn bytes(&mut self) -> Option<&mut [MaybeUninit<u8>]> {
        let num_pixels = self.stride() * self.height();
        let num_bytes = num_pixels * self.format().bytes_per_pixel()?;
        Some(unsafe { std::slice::from_raw_parts_mut(self.bits().cast(), num_bytes) })
    }

    /// Returns a slice of bytes for each line of visible pixels in the buffer, ignoring any
    /// padding pixels incurred by the stride.
    ///
    /// See [`bits()`][Self::bits()] and [`bytes()`][Self::bytes()] for contiguous access to the
    /// underlying buffer.
    pub fn lines(&mut self) -> Option<impl Iterator<Item = &mut [MaybeUninit<u8>]>> {
        let bpp = self.format().bytes_per_pixel()?;
        let scanline_bytes = bpp * self.stride();
        let width_bytes = bpp * self.width();
        let bytes = self.bytes()?;

        Some(
            bytes
                .chunks_exact_mut(scanline_bytes)
                .map(move |scanline| &mut scanline[..width_bytes]),
        )
    }
}

impl<'a> Drop for NativeWindowBufferLockGuard<'a> {
    fn drop(&mut self) {
        let ret = unsafe { ffi::ANativeWindow_unlockAndPost(self.window.ptr.as_ptr()) };
        assert_eq!(ret, 0);
    }
}

#[cfg(all(feature = "nativewindow", feature = "api-level-26"))]
bitflags::bitflags! {
    /// Transforms that can be applied to buffers as they are displayed to a window.
    ///
    /// Supported transforms are any combination of horizontal mirror, vertical mirror, and
    /// clockwise 90 degree rotation, in that order. Rotations of 180 and 270 degrees are made up
    /// of those basic transforms.
    #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
    #[doc(alias = "ANativeWindowTransform")]
    pub struct NativeWindowTransform : i32 {
        #[doc(alias = "ANATIVEWINDOW_TRANSFORM_IDENTITY")]
        const IDENTITY = ffi::ANativeWindowTransform::ANATIVEWINDOW_TRANSFORM_IDENTITY.0 as i32;
        #[doc(alias = "ANATIVEWINDOW_TRANSFORM_MIRROR_HORIZONTAL")]
        const MIRROR_HORIZONTAL = ffi::ANativeWindowTransform::ANATIVEWINDOW_TRANSFORM_MIRROR_HORIZONTAL.0 as i32;
        #[doc(alias = "ANATIVEWINDOW_TRANSFORM_MIRROR_VERTICAL")]
        const MIRROR_VERTICAL = ffi::ANativeWindowTransform::ANATIVEWINDOW_TRANSFORM_MIRROR_VERTICAL.0 as i32;
        #[doc(alias = "ANATIVEWINDOW_TRANSFORM_ROTATE_90")]
        const ROTATE_90 = ffi::ANativeWindowTransform::ANATIVEWINDOW_TRANSFORM_ROTATE_90.0 as i32;
        /// Defined as [`Self::MIRROR_HORIZONTAL`] `|` [`Self::MIRROR_VERTICAL`].
        #[doc(alias = "ANATIVEWINDOW_TRANSFORM_ROTATE_180")]
        const ROTATE_180 = ffi::ANativeWindowTransform::ANATIVEWINDOW_TRANSFORM_ROTATE_180.0 as i32;
        /// Defined as [`Self::ROTATE_180`] `|` [`Self::ROTATE_90`].
        #[doc(alias = "ANATIVEWINDOW_TRANSFORM_ROTATE_270")]
        const ROTATE_270 = ffi::ANativeWindowTransform::ANATIVEWINDOW_TRANSFORM_ROTATE_270.0 as i32;

        // https://docs.rs/bitflags/latest/bitflags/#externally-defined-flags
        const _ = !0;
    }
}

/// Compatibility value for [`NativeWindow::set_frame_rate()`]
#[cfg_attr(
    feature = "api-level-31",
    doc = " and [`NativeWindow::set_frame_rate_with_change_strategy()`]"
)]
/// .
#[cfg(all(feature = "nativewindow", feature = "api-level-30"))]
#[repr(i8)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
#[doc(alias = "ANativeWindow_FrameRateCompatibility")]
#[non_exhaustive]
pub enum FrameRateCompatibility {
    /// There are no inherent restrictions on the frame rate of this window.
    ///
    /// When the system selects a frame rate other than what the app requested, the app will be
    /// able to run at the system frame rate without requiring pull down. This value should be used
    /// when displaying game content, UIs, and anything that isn't video.
    #[doc(alias = "ANATIVEWINDOW_FRAME_RATE_COMPATIBILITY_DEFAULT")]
    Default =
        ffi::ANativeWindow_FrameRateCompatibility::ANATIVEWINDOW_FRAME_RATE_COMPATIBILITY_DEFAULT.0 as i8,
    /// This window is being used to display content with an inherently fixed frame rate, e.g. a
    /// video that has a specific frame rate.
    ///
    /// When the system selects a frame rate other than what the app requested, the app will need
    /// to do pull down or use some other technique to adapt to the system's frame rate. The user
    /// experience is likely to be worse (e.g. more frame stuttering) than it would be if the
    /// system had chosen the app's requested frame rate. This value should be used for video
    /// content.
    #[doc(alias = "ANATIVEWINDOW_FRAME_RATE_COMPATIBILITY_FIXED_SOURCE")]
    FixedSource = ffi::ANativeWindow_FrameRateCompatibility::ANATIVEWINDOW_FRAME_RATE_COMPATIBILITY_FIXED_SOURCE.0 as i8,
}

/// Change frame rate strategy value for [`NativeWindow::set_frame_rate_with_change_strategy()`].
#[cfg(all(feature = "nativewindow", feature = "api-level-31"))]
#[repr(i8)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
#[doc(alias = "ANativeWindow_ChangeFrameRateStrategy")]
#[non_exhaustive]
pub enum ChangeFrameRateStrategy {
    /// Change the frame rate only if the transition is going to be seamless.
    #[doc(alias = "ANATIVEWINDOW_CHANGE_FRAME_RATE_ONLY_IF_SEAMLESS")]
    OnlyIfSeamless =
        ffi::ANativeWindow_ChangeFrameRateStrategy::ANATIVEWINDOW_CHANGE_FRAME_RATE_ONLY_IF_SEAMLESS
            .0 as i8,
    /// Change the frame rate even if the transition is going to be non-seamless, i.e. with visual interruptions for the user.
    #[doc(alias = "ANATIVEWINDOW_CHANGE_FRAME_RATE_ALWAYS")]
    Always =
        ffi::ANativeWindow_ChangeFrameRateStrategy::ANATIVEWINDOW_CHANGE_FRAME_RATE_ALWAYS.0 as i8,
}
