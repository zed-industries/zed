//! Bindings for [`ASurfaceTexture`]
//!
//! See <https://source.android.com/devices/graphics/arch-st> for an architectural overview of
//! [`SurfaceTexture`] internals.
//!
//! [`ASurfaceTexture`]: https://developer.android.com/ndk/reference/group/surface-texture
#![cfg(feature = "api-level-28")]

use crate::{native_window::NativeWindow, utils::status_to_io_result};
use jni_sys::{jobject, JNIEnv};
use std::{io::Result, ptr::NonNull, time::Duration};

/// An opaque type to manage [`android.graphics.SurfaceTexture`] from native code
///
/// [`android.graphics.SurfaceTexture`]: https://developer.android.com/reference/android/graphics/SurfaceTexture
#[derive(Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SurfaceTexture {
    ptr: NonNull<ffi::ASurfaceTexture>,
}

unsafe impl Send for SurfaceTexture {}

impl Drop for SurfaceTexture {
    fn drop(&mut self) {
        unsafe { ffi::ASurfaceTexture_release(self.ptr.as_ptr()) }
    }
}

impl SurfaceTexture {
    /// Assumes ownership of `ptr`
    ///
    /// # Safety
    /// `ptr` must be a valid pointer to an Android [`ffi::ASurfaceTexture`].
    pub unsafe fn from_ptr(ptr: NonNull<ffi::ASurfaceTexture>) -> Self {
        Self { ptr }
    }

    /// Get a reference to the native [`SurfaceTexture`] from the corresponding Java object.
    ///
    /// # Safety
    ///
    /// This function should be called with a healthy JVM pointer and with a non-null
    /// [`android.graphics.SurfaceTexture`], which must be kept alive on the Java/Kotlin side.
    ///
    /// The caller must keep a reference to the Java [`android.graphics.SurfaceTexture`] during the
    /// lifetime of the returned [`SurfaceTexture`]. Failing to do so could result in the
    /// [`SurfaceTexture`] to stop functioning properly once the Java object gets finalized.
    /// However, this will not result in program termination.
    ///
    /// [`android.graphics.SurfaceTexture`]: https://developer.android.com/reference/android/graphics/SurfaceTexture
    pub unsafe fn from_surface_texture(env: *mut JNIEnv, surface_texture: jobject) -> Option<Self> {
        let a_surface_texture_ptr = ffi::ASurfaceTexture_fromSurfaceTexture(env, surface_texture);
        let s = NonNull::new(a_surface_texture_ptr)?;
        Some(SurfaceTexture::from_ptr(s))
    }

    /// Returns a pointer to the native [`ffi::ASurfaceTexture`].
    pub fn ptr(&self) -> NonNull<ffi::ASurfaceTexture> {
        self.ptr
    }

    /// Returns a reference to a [`NativeWindow`] (i.e. the Producer) for this [`SurfaceTexture`].
    ///
    /// This is equivalent to Java's:
    /// ```java
    /// Surface sur = new Surface(surfaceTexture);
    /// ```
    pub fn acquire_native_window(&self) -> Option<NativeWindow> {
        let native_window = unsafe { ffi::ASurfaceTexture_acquireANativeWindow(self.ptr.as_ptr()) };
        let n = NonNull::new(native_window)?;
        Some(unsafe { NativeWindow::from_ptr(n) })
    }

    /// Attach the [`SurfaceTexture`] to the OpenGL ES context that is current on the calling
    /// thread.
    ///
    /// A new OpenGL ES texture object is created and populated with the [`SurfaceTexture`] image
    /// frame that was current at the time of the last call to
    /// [`detach_from_gl_context()`][Self::detach_from_gl_context()]. This new texture is bound to
    /// the `GL_TEXTURE_EXTERNAL_OES` texture target.
    ///
    /// This can be used to access the [`SurfaceTexture`] image contents from multiple OpenGL ES
    /// contexts. Note, however, that the image contents are only accessible from one OpenGL ES
    /// context at a time.
    pub fn attach_to_gl_context(&self, tex_name: u32) -> Result<()> {
        let status = unsafe { ffi::ASurfaceTexture_attachToGLContext(self.ptr.as_ptr(), tex_name) };
        status_to_io_result(status)
    }

    /// Detach the [`SurfaceTexture`] from the OpenGL ES context that owns the OpenGL ES texture
    /// object.
    ///
    /// This call must be made with the OpenGL ES context current on the calling thread. The OpenGL
    /// ES texture object will be deleted as a result of this call. After calling this method all
    /// calls to [`update_tex_image()`][Self::update_tex_image()] will fail until a successful call
    /// to [`attach_to_gl_context()`][Self::attach_to_gl_context()] is made.
    ///
    /// This can be used to access the [`SurfaceTexture`] image contents from multiple OpenGL ES
    /// contexts. Note, however, that the image contents are only accessible from one OpenGL ES
    /// context at a time.
    pub fn detach_from_gl_context(&self) -> Result<()> {
        let status = unsafe { ffi::ASurfaceTexture_detachFromGLContext(self.ptr.as_ptr()) };
        status_to_io_result(status)
    }

    /// Retrieve the 4x4 texture coordinate transform matrix associated with the texture image set
    /// by the most recent call to [`update_tex_image()`][Self::update_tex_image()].
    ///
    /// This transform matrix maps 2D homogeneous texture coordinates of the form `(s, t, 0, 1)`
    /// with `s` and `t` in the inclusive range `[0, 1]` to the texture coordinate that should be
    /// used to sample that location from the texture. Sampling the texture outside of the range of
    /// this transform is undefined.
    ///
    /// The matrix is stored in column-major order so that it may be passed directly to OpenGL ES
    /// via the [`glLoadMatrixf()`] or [`glUniformMatrix4fv()`] functions.
    ///
    /// [`glLoadMatrixf()`]: https://www.khronos.org/registry/OpenGL-Refpages/gl2.1/xhtml/glLoadMatrix.xml
    /// [`gluniformmatrix4fv()`]: https://www.khronos.org/registry/OpenGL-Refpages/es3.1/html/glUniform.xhtml
    pub fn transform_matrix(&self) -> [f32; 16] {
        let mut r = [0f32; 16];
        unsafe { ffi::ASurfaceTexture_getTransformMatrix(self.ptr.as_ptr(), r.as_mut_ptr()) };
        r
    }

    /// Retrieve the timestamp associated with the texture image set by the most recent call to
    /// [`update_tex_image()`][Self::update_tex_image()].
    ///
    /// This timestamp is in nanoseconds, and is normally monotonically increasing. The timestamp
    /// should be unaffected by time-of-day adjustments, and for a camera should be strictly
    /// monotonic but for a [`MediaPlayer`] may be reset when the position is set. The specific
    /// meaning and zero point of the timestamp depends on the source providing images to the
    /// [`SurfaceTexture`]. Unless otherwise specified by the image source, timestamps cannot
    /// generally be compared across [`SurfaceTexture`] instances, or across multiple program
    /// invocations. It is mostly useful for determining time offsets between subsequent frames.
    ///
    /// For EGL/Vulkan producers, this timestamp is the desired present time set with the
    /// [`EGL_ANDROID_presentation_time`] or [`VK_GOOGLE_display_timing`] extensions.
    ///
    /// [`MediaPlayer`]: https://developer.android.com/reference/android/media/MediaPlayer
    /// [`EGL_ANDROID_presentation_time`]: https://www.khronos.org/registry/EGL/extensions/ANDROID/EGL_ANDROID_presentation_time.txt
    /// [`VK_GOOGLE_display_timing`]: https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_GOOGLE_display_timing.html
    pub fn timestamp(&self) -> Duration {
        Duration::from_nanos(
            unsafe { ffi::ASurfaceTexture_getTimestamp(self.ptr.as_ptr()) }
                .try_into()
                .unwrap(),
        )
    }

    /// Update the texture image to the most recent frame from the image stream.
    ///
    /// This may only be called while the OpenGL ES context that owns the texture is current on the
    /// calling thread. It will implicitly bind its texture to the `GL_TEXTURE_EXTERNAL_OES`
    /// texture target.
    pub fn update_tex_image(&self) -> Result<()> {
        let status = unsafe { ffi::ASurfaceTexture_updateTexImage(self.ptr.as_ptr()) };
        status_to_io_result(status)
    }
}
