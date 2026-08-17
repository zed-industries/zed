//! Bindings for [`AHardwareBuffer`]
//!
//! [`AHardwareBuffer`]: https://developer.android.com/ndk/reference/group/a-hardware-buffer#ahardwarebuffer

#![cfg(feature = "api-level-26")]

use std::{
    io::Result,
    mem::MaybeUninit,
    ops::Deref,
    os::{
        fd::{AsRawFd, BorrowedFd, FromRawFd, IntoRawFd, OwnedFd},
        raw::c_void,
    },
    ptr::NonNull,
};

use jni_sys::{jobject, JNIEnv};

use super::{hardware_buffer_format::HardwareBufferFormat, utils::status_to_io_result};

bitflags::bitflags! {
    /// Buffer usage flags, specifying how the buffer will be accessed.
    #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
    #[doc(alias = "AHardwareBuffer_UsageFlags")]
    pub struct HardwareBufferUsage : u64 {
        /// The buffer will never be locked for direct CPU reads using the
        /// [`HardwareBuffer::lock()`] function. Note that reading the buffer using OpenGL or Vulkan
        /// functions or memory mappings is still allowed.
        #[doc(alias = "AHARDWAREBUFFER_USAGE_CPU_READ_NEVER")]
        const CPU_READ_NEVER = ffi::AHardwareBuffer_UsageFlags::AHARDWAREBUFFER_USAGE_CPU_READ_NEVER.0;
        /// The buffer will sometimes be locked for direct CPU reads using the
        /// [`HardwareBuffer::lock()`] function. Note that reading the buffer using OpenGL or Vulkan
        /// functions or memory mappings does not require the presence of this flag.
        #[doc(alias = "AHARDWAREBUFFER_USAGE_CPU_READ_RARELY")]
        const CPU_READ_RARELY = ffi::AHardwareBuffer_UsageFlags::AHARDWAREBUFFER_USAGE_CPU_READ_RARELY.0;
        /// The buffer will often be locked for direct CPU reads using the
        /// [`HardwareBuffer::lock()`] function. Note that reading the buffer using OpenGL or Vulkan
        /// functions or memory mappings does not require the presence of this flag.
        #[doc(alias = "AHARDWAREBUFFER_USAGE_CPU_READ_OFTEN")]
        const CPU_READ_OFTEN = ffi::AHardwareBuffer_UsageFlags::AHARDWAREBUFFER_USAGE_CPU_READ_OFTEN.0;
        /// CPU read value mask.
        #[doc(alias = "AHARDWAREBUFFER_USAGE_CPU_READ_MASK")]
        const CPU_READ_MASK = ffi::AHardwareBuffer_UsageFlags::AHARDWAREBUFFER_USAGE_CPU_READ_MASK.0;

        /// The buffer will never be locked for direct CPU writes using the
        /// [`HardwareBuffer::lock()`] function. Note that writing the buffer using OpenGL or Vulkan
        /// functions or memory mappings is still allowed.
        #[doc(alias = "AHARDWAREBUFFER_USAGE_CPU_WRITE_NEVER")]
        const CPU_WRITE_NEVER = ffi::AHardwareBuffer_UsageFlags::AHARDWAREBUFFER_USAGE_CPU_WRITE_NEVER.0;
        /// The buffer will sometimes be locked for direct CPU writes using the
        /// [`HardwareBuffer::lock()`] function. Note that writing the buffer using OpenGL or Vulkan
        /// functions or memory mappings does not require the presence of this flag.
        #[doc(alias = "AHARDWAREBUFFER_USAGE_CPU_WRITE_RARELY")]
        const CPU_WRITE_RARELY = ffi::AHardwareBuffer_UsageFlags::AHARDWAREBUFFER_USAGE_CPU_WRITE_RARELY.0;
        /// The buffer will often be locked for direct CPU writes using the
        /// [`HardwareBuffer::lock()`] function. Note that writing the buffer using OpenGL or Vulkan
        /// functions or memory mappings does not require the presence of this flag.
        #[doc(alias = "AHARDWAREBUFFER_USAGE_CPU_WRITE_OFTEN")]
        const CPU_WRITE_OFTEN = ffi::AHardwareBuffer_UsageFlags::AHARDWAREBUFFER_USAGE_CPU_WRITE_OFTEN.0;
        /// CPU write value mask.
        #[doc(alias = "AHARDWAREBUFFER_USAGE_CPU_WRITE_MASK")]
        const CPU_WRITE_MASK = ffi::AHardwareBuffer_UsageFlags::AHARDWAREBUFFER_USAGE_CPU_WRITE_MASK.0;

        /// The buffer will be read from by the GPU as a texture.
        #[doc(alias = "AHARDWAREBUFFER_USAGE_GPU_SAMPLED_IMAGE")]
        const GPU_SAMPLED_IMAGE = ffi::AHardwareBuffer_UsageFlags::AHARDWAREBUFFER_USAGE_GPU_SAMPLED_IMAGE.0;
        /// The buffer will be written to by the GPU as a framebuffer attachment.
        #[doc(alias = "AHARDWAREBUFFER_USAGE_GPU_FRAMEBUFFER")]
        const GPU_FRAMEBUFFER = ffi::AHardwareBuffer_UsageFlags::AHARDWAREBUFFER_USAGE_GPU_FRAMEBUFFER.0;
        /// The buffer will be written to by the GPU as a framebuffer attachment.
        ///
        /// Note that the name of this flag is somewhat misleading: it does not imply that the
        /// buffer contains a color format. A buffer with depth or stencil format that will be
        /// used as a framebuffer attachment should also have this flag. Use the equivalent flag
        /// [`HardwareBufferusage::GPU_FRAMEBUFFER`] to avoid this confusion.
        #[doc(alias = "AHARDWAREBUFFER_USAGE_GPU_COLOR_OUTPUT")]
        const GPU_COLOR_OUTPUT = ffi::AHardwareBuffer_UsageFlags::AHARDWAREBUFFER_USAGE_GPU_COLOR_OUTPUT.0;
        /// The buffer will be used as a composer HAL overlay layer.
        ///
        /// This flag is currently only needed when using [`SurfaceTransaction::set_buffer()`] to
        /// set a buffer. In all other cases, the framework adds this flag internally to buffers
        /// that could be presented in a composer overlay. [`SurfaceTransaction::set_buffer()`]
        /// is special because it uses buffers allocated directly through
        /// [`HardwareBuffer::allocate()`] instead of buffers allocated by the framework.
        #[doc(alias = "AHARDWAREBUFFER_USAGE_COMPOSER_OVERLAY")]
        const COMPOSER_OVERLAY = ffi::AHardwareBuffer_UsageFlags::AHARDWAREBUFFER_USAGE_COMPOSER_OVERLAY.0;
        /// The buffer is protected from direct CPU access or being read by non-secure hardware,
        /// such as video encoders.
        ///
        /// This flag is incompatible with CPU read and write flags. It is mainly used when handling
        /// DRM video. Refer to the EGL extension [`EGL_EXT_protected_content`] and GL extension
        /// [`GL_EXT_protected_textures`] for more information on how these buffers are expected
        /// to behave.
        ///
        /// [`EGL_EXT_protected_content`]: https://registry.khronos.org/EGL/extensions/EXT/EGL_EXT_protected_content.txt
        /// [`GL_EXT_protected_textures`]: https://registry.khronos.org/OpenGL/extensions/EXT/EXT_protected_textures.txt
        #[doc(alias = "AHARDWAREBUFFER_USAGE_PROTECTED_CONTENT")]
        const PROTECTED_CONTENT = ffi::AHardwareBuffer_UsageFlags::AHARDWAREBUFFER_USAGE_PROTECTED_CONTENT.0;
        /// The buffer will be read by a hardware video encoder.
        #[doc(alias = "AHARDWAREBUFFER_USAGE_VIDEO_ENCODE")]
        const VIDEO_ENCODE = ffi::AHardwareBuffer_UsageFlags::AHARDWAREBUFFER_USAGE_VIDEO_ENCODE.0;
        /// The buffer will be used for direct writes from sensors. When this flag is present, the
        /// format must be [`HardwareBufferFormat::Blob`].
        #[doc(alias = "AHARDWAREBUFFER_USAGE_SENSOR_DIRECT_DATA")]
        const SENSOR_DIRECT_DATA = ffi::AHardwareBuffer_UsageFlags::AHARDWAREBUFFER_USAGE_SENSOR_DIRECT_DATA.0;
        /// The buffer will be used as a shader storage or uniform buffer object. When this flag is
        /// present, the format must be [`HardwareBufferFormat::Blob`].
        #[doc(alias = "AHARDWAREBUFFER_USAGE_GPU_DATA_BUFFER")]
        const GPU_DATA_BUFFER = ffi::AHardwareBuffer_UsageFlags::AHARDWAREBUFFER_USAGE_GPU_DATA_BUFFER.0;
        /// The buffer will be used as a cube map texture. When this flag is present, the buffer
        /// must have a layer count that is a multiple of 6. Note that buffers with this flag must
        /// be bound to OpenGL textures using the extension [`GL_EXT_EGL_image_storage`] instead
        /// of [`GL_KHR_EGL_image`].
        ///
        /// [`GL_EXT_EGL_image_storage`]: https://registry.khronos.org/OpenGL/extensions/EXT/EXT_EGL_image_storage.txt
        // TODO: This extension only exists for VG. Reported at https://issuetracker.google.com/issues/300602767#comment16
        /// [`GL_KHR_EGL_image`]: https://registry.khronos.org/OpenVG/extensions/KHR/VG_KHR_EGL_image.txt
        #[doc(alias = "AHARDWAREBUFFER_USAGE_GPU_CUBE_MAP")]
        const GPU_CUBE_MAP = ffi::AHardwareBuffer_UsageFlags::AHARDWAREBUFFER_USAGE_GPU_CUBE_MAP.0;
        /// The buffer contains a complete mipmap hierarchy. Note that buffers with this flag must
        /// be bound to OpenGL textures using the extension [`GL_EXT_EGL_image_storage`] instead
        /// of [`GL_KHR_EGL_image`].
        ///
        /// [`GL_EXT_EGL_image_storage`]: https://registry.khronos.org/OpenGL/extensions/EXT/EXT_EGL_image_storage.txt
        // TODO: This extension only exists for VG. Reported at https://issuetracker.google.com/issues/300602767#comment16
        /// [`GL_KHR_EGL_image`]: https://registry.khronos.org/OpenVG/extensions/KHR/VG_KHR_EGL_image.txt
        #[doc(alias = "AHARDWAREBUFFER_USAGE_GPU_MIPMAP_COMPLETE")]
        const GPU_MIPMAP_COMPLETE = ffi::AHardwareBuffer_UsageFlags::AHARDWAREBUFFER_USAGE_GPU_MIPMAP_COMPLETE.0;

        // TODO: Only available in a newer NDK
        // /// Usage: The buffer is used for front-buffer rendering. When front-buffering rendering
        // /// is specified, different usages may adjust their behavior as a result. For example, when
        // /// used as [`HardwareBufferFormat::GPU_COLOR_OUTPUT`] the buffer will behave similar to a
        // /// single-buffered window. When used with [`HardwareBufferFormat::COMPOSER_OVERLAY`], the
        // /// system will try to prioritize the buffer receiving an overlay plane & avoid caching it
        // /// in intermediate composition buffers.
        // #[doc(alias = "AHARDWAREBUFFER_USAGE_FRONT_BUFFER")]
        // const USAGE_FRONT_BUFFER = ffi::AHardwareBuffer_UsageFlags::AHARDWAREBUFFER_USAGE_FRONT_BUFFER.0;

        #[doc(alias = "AHARDWAREBUFFER_USAGE_VENDOR_0")]
        const VENDOR_0 = ffi::AHardwareBuffer_UsageFlags::AHARDWAREBUFFER_USAGE_VENDOR_0.0;
        #[doc(alias = "AHARDWAREBUFFER_USAGE_VENDOR_1")]
        const VENDOR_1 = ffi::AHardwareBuffer_UsageFlags::AHARDWAREBUFFER_USAGE_VENDOR_1.0;
        #[doc(alias = "AHARDWAREBUFFER_USAGE_VENDOR_2")]
        const VENDOR_2 = ffi::AHardwareBuffer_UsageFlags::AHARDWAREBUFFER_USAGE_VENDOR_2.0;
        #[doc(alias = "AHARDWAREBUFFER_USAGE_VENDOR_3")]
        const VENDOR_3 = ffi::AHardwareBuffer_UsageFlags::AHARDWAREBUFFER_USAGE_VENDOR_3.0;
        #[doc(alias = "AHARDWAREBUFFER_USAGE_VENDOR_4")]
        const VENDOR_4 = ffi::AHardwareBuffer_UsageFlags::AHARDWAREBUFFER_USAGE_VENDOR_4.0;
        #[doc(alias = "AHARDWAREBUFFER_USAGE_VENDOR_5")]
        const VENDOR_5 = ffi::AHardwareBuffer_UsageFlags::AHARDWAREBUFFER_USAGE_VENDOR_5.0;
        #[doc(alias = "AHARDWAREBUFFER_USAGE_VENDOR_6")]
        const VENDOR_6 = ffi::AHardwareBuffer_UsageFlags::AHARDWAREBUFFER_USAGE_VENDOR_6.0;
        #[doc(alias = "AHARDWAREBUFFER_USAGE_VENDOR_7")]
        const VENDOR_7 = ffi::AHardwareBuffer_UsageFlags::AHARDWAREBUFFER_USAGE_VENDOR_7.0;
        #[doc(alias = "AHARDWAREBUFFER_USAGE_VENDOR_8")]
        const VENDOR_8 = ffi::AHardwareBuffer_UsageFlags::AHARDWAREBUFFER_USAGE_VENDOR_8.0;
        #[doc(alias = "AHARDWAREBUFFER_USAGE_VENDOR_9")]
        const VENDOR_9 = ffi::AHardwareBuffer_UsageFlags::AHARDWAREBUFFER_USAGE_VENDOR_9.0;
        #[doc(alias = "AHARDWAREBUFFER_USAGE_VENDOR_10")]
        const VENDOR_10 = ffi::AHardwareBuffer_UsageFlags::AHARDWAREBUFFER_USAGE_VENDOR_10.0;
        #[doc(alias = "AHARDWAREBUFFER_USAGE_VENDOR_11")]
        const VENDOR_11 = ffi::AHardwareBuffer_UsageFlags::AHARDWAREBUFFER_USAGE_VENDOR_11.0;
        #[doc(alias = "AHARDWAREBUFFER_USAGE_VENDOR_12")]
        const VENDOR_12 = ffi::AHardwareBuffer_UsageFlags::AHARDWAREBUFFER_USAGE_VENDOR_12.0;
        #[doc(alias = "AHARDWAREBUFFER_USAGE_VENDOR_13")]
        const VENDOR_13 = ffi::AHardwareBuffer_UsageFlags::AHARDWAREBUFFER_USAGE_VENDOR_13.0;
        #[doc(alias = "AHARDWAREBUFFER_USAGE_VENDOR_14")]
        const VENDOR_14 = ffi::AHardwareBuffer_UsageFlags::AHARDWAREBUFFER_USAGE_VENDOR_14.0;
        #[doc(alias = "AHARDWAREBUFFER_USAGE_VENDOR_15")]
        const VENDOR_15 = ffi::AHardwareBuffer_UsageFlags::AHARDWAREBUFFER_USAGE_VENDOR_15.0;
        #[doc(alias = "AHARDWAREBUFFER_USAGE_VENDOR_16")]
        const VENDOR_16 = ffi::AHardwareBuffer_UsageFlags::AHARDWAREBUFFER_USAGE_VENDOR_16.0;
        #[doc(alias = "AHARDWAREBUFFER_USAGE_VENDOR_17")]
        const VENDOR_17 = ffi::AHardwareBuffer_UsageFlags::AHARDWAREBUFFER_USAGE_VENDOR_17.0;
        #[doc(alias = "AHARDWAREBUFFER_USAGE_VENDOR_18")]
        const VENDOR_18 = ffi::AHardwareBuffer_UsageFlags::AHARDWAREBUFFER_USAGE_VENDOR_18.0;
        #[doc(alias = "AHARDWAREBUFFER_USAGE_VENDOR_19")]
        const VENDOR_19 = ffi::AHardwareBuffer_UsageFlags::AHARDWAREBUFFER_USAGE_VENDOR_19.0;
    }
}

impl HardwareBufferUsage {
    /// Helper to read [`HardwareBufferUsage::CPU_READ_MASK`] values.
    #[doc(alias = "AHARDWAREBUFFER_USAGE_CPU_READ_MASK")]
    pub fn cpu_read(self) -> HardwareBufferUsage {
        self.intersection(Self::CPU_READ_MASK)
    }

    /// Helper to read [`HardwareBufferUsage::CPU_WRITE_MASK`] values.
    #[doc(alias = "AHARDWAREBUFFER_USAGE_CPU_WRITE_MASK")]
    pub fn cpu_write(self) -> HardwareBufferUsage {
        self.intersection(Self::CPU_WRITE_MASK)
    }
}

pub type Rect = ffi::ARect;

fn construct<T>(with_ptr: impl FnOnce(*mut T) -> i32) -> Result<T> {
    let mut result = MaybeUninit::uninit();
    let status = with_ptr(result.as_mut_ptr());
    status_to_io_result(status).map(|()| unsafe { result.assume_init() })
}

/// A native [`AHardwareBuffer *`]
///
/// [`HardwareBuffer`] objects represent chunks of memory that can be accessed by various hardware
/// components in the system.
///
/// It can be easily converted to the Java counterpart [`android.hardware.HardwareBuffer`] and
/// passed between processes using Binder. All operations involving [`HardwareBuffer`] and
/// [`android.hardware.HardwareBuffer`] are zero-copy, i.e., passing [`HardwareBuffer`] to another
/// process creates a shared view of the same region of memory.
///
/// [`HardwareBuffer`] can be bound to EGL/OpenGL and Vulkan primitives. For EGL, use the extension
/// function [`eglGetNativeClientBufferANDROID`] to obtain an `EGLClientBuffer` and pass it
/// directly to [`eglCreateImageKHR`]. Refer to the EGL extensions
/// [`EGL_ANDROID_get_native_client_buffer`] and [`EGL_ANDROID_image_native_buffer`] for more
/// information. In Vulkan, the contents of the [`HardwareBuffer`] can be accessed as [external
/// memory]. See the [`VK_ANDROID_external_memory_android_hardware_buffer`] extension for details.
///
/// [`AHardwareBuffer *`]: https://developer.android.com/ndk/reference/group/a-hardware-buffer#ahardwarebuffer
/// [`android.hardware.HardwareBuffer`]: https://developer.android.com/reference/android/hardware/HardwareBuffer
/// [`eglGetNativeClientBufferANDROID`]: https://www.khronos.org/registry/EGL/extensions/ANDROID/EGL_ANDROID_get_native_client_buffer.txt
/// [`eglCreateImageKHR`]: https://www.khronos.org/registry/EGL/extensions/KHR/EGL_KHR_image_base.txt
/// [`EGL_ANDROID_get_native_client_buffer`]: https://www.khronos.org/registry/EGL/extensions/ANDROID/EGL_ANDROID_get_native_client_buffer.txt
/// [`EGL_ANDROID_image_native_buffer`]: https://www.khronos.org/registry/EGL/extensions/ANDROID/EGL_ANDROID_image_native_buffer.txt
/// [external memory]: https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_external_memory.html
/// [`VK_ANDROID_external_memory_android_hardware_buffer`]: https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_ANDROID_external_memory_android_hardware_buffer.html
#[derive(Debug)]
pub struct HardwareBuffer {
    inner: NonNull<ffi::AHardwareBuffer>,
}

impl HardwareBuffer {
    /// Create an _unowned_ [`HardwareBuffer`] from a native pointer
    ///
    /// To wrap a strong reference (that is `release`d on [`Drop`]), call
    /// [`HardwareBufferRef::from_ptr()`] instead.
    ///
    /// # Safety
    /// By calling this function, you assert that it is a valid pointer to an NDK
    /// [`ffi::AHardwareBuffer`] that is kept alive externally, or retrieve a strong reference
    /// using [`HardwareBuffer::acquire()`].
    pub unsafe fn from_ptr(ptr: NonNull<ffi::AHardwareBuffer>) -> Self {
        Self { inner: ptr }
    }

    /// Returns the underlying [`ffi::AHardwareBuffer`] pointer
    ///
    /// See the top-level [`HardwareBuffer`] struct documentation for (graphics) APIs that accept
    /// this pointer.
    pub fn as_ptr(&self) -> *mut ffi::AHardwareBuffer {
        self.inner.as_ptr()
    }

    /// Allocates a buffer that matches the passed [`HardwareBufferDesc`].
    ///
    /// If allocation succeeds, the buffer can be used according to the usage flags specified in
    /// its description. If a buffer is used in ways not compatible with its usage flags, the
    /// results are undefined and may include program termination.
    pub fn allocate(desc: HardwareBufferDesc) -> Result<HardwareBufferRef> {
        unsafe {
            let ptr = construct(|res| ffi::AHardwareBuffer_allocate(&desc.into_native(), res))?;

            Ok(HardwareBufferRef::from_ptr(NonNull::new_unchecked(ptr)))
        }
    }

    /// Create a [`HardwareBuffer`] from JNI pointers
    ///
    /// # Safety
    /// By calling this function, you assert that these are valid pointers to JNI objects.
    ///
    /// This method does not acquire any additional reference to the AHardwareBuffer that is
    /// returned. To keep the [`HardwareBuffer`] alive after the [Java `HardwareBuffer`] object
    /// is closed, explicitly or by the garbage collector, be sure to retrieve a strong reference
    /// using [`HardwareBuffer::acquire()`].
    ///
    /// [Java `HardwareBuffer`]: https://developer.android.com/reference/android/hardware/HardwareBuffer
    pub unsafe fn from_jni(env: *mut JNIEnv, hardware_buffer: jobject) -> Self {
        let ptr = ffi::AHardwareBuffer_fromHardwareBuffer(env, hardware_buffer);

        Self::from_ptr(NonNull::new_unchecked(ptr))
    }

    /// # Safety
    /// By calling this function, you assert that `env` is a valid pointer to a [`JNIEnv`].
    pub unsafe fn to_jni(&self, env: *mut JNIEnv) -> jobject {
        ffi::AHardwareBuffer_toHardwareBuffer(env, self.as_ptr())
    }

    /// Return a description of the [`HardwareBuffer`] in the passed [`HardwareBufferDesc`] struct.
    pub fn describe(&self) -> HardwareBufferDesc {
        let desc = unsafe {
            let mut result = MaybeUninit::uninit();
            ffi::AHardwareBuffer_describe(self.as_ptr(), result.as_mut_ptr());
            result.assume_init()
        };

        HardwareBufferDesc {
            width: desc.width,
            height: desc.height,
            layers: desc.layers,
            format: i32::try_from(desc.format)
                .expect("i32->u32 overflow in HardwareBuffer::describe()")
                .into(),
            usage: HardwareBufferUsage::from_bits_retain(desc.usage),
            stride: desc.stride,
        }
    }

    /// Test whether the given format and usage flag combination is allocatable.
    ///
    /// If this function returns [`true`], it means that a buffer with the given description can
    /// be allocated on this implementation, unless resource exhaustion occurs. If this function
    /// returns [`false`], it means that the allocation of the given description will never
    /// succeed.
    ///
    /// The return value of this function may depend on all fields in the description, except
    /// [`HardwareBufferDesc::stride`], which is always ignored. For example, some implementations
    /// have implementation-defined limits on texture size and layer count.
    #[cfg(feature = "api-level-29")]
    pub fn is_supported(desc: HardwareBufferDesc) -> bool {
        let res = unsafe { ffi::AHardwareBuffer_isSupported(&desc.into_native()) };
        res == 1
    }

    /// Get the system-wide unique id for this [`HardwareBuffer`].
    #[cfg(feature = "api-level-31")]
    #[doc(alias = "AHardwareBuffer_getId")]
    pub fn id(&self) -> Result<u64> {
        construct(|res| unsafe { ffi::AHardwareBuffer_getId(self.as_ptr(), res) })
    }

    /// Lock the [`HardwareBuffer`] for direct CPU access.
    ///
    /// This function can lock the buffer for either reading or writing. It may block if the
    /// hardware needs to finish rendering, if CPU caches need to be synchronized, or possibly for
    /// other implementation-specific reasons.
    ///
    /// The [`HardwareBuffer`] must have one layer, otherwise the call will fail.
    ///
    /// If `fence` is not [`None`], it specifies a fence file descriptor on which to wait before
    /// locking the buffer. If it's [`None`], the caller is responsible for ensuring that writes
    /// to the buffer have completed before calling this function. Using this parameter is more
    /// efficient than waiting on the fence and then calling this function.
    ///
    /// The `usage` parameter may only specify `HardwareBufferUsage::CPU_*`. If set, then the
    /// address of the buffer in virtual memory is returned. The flags must also be compatible with
    /// usage flags specified at buffer creation: if a read flag is passed, the buffer must have
    /// been created with [`HardwareBufferUsage::CPU_READ_RARELY`] or
    /// [`HardwareBufferUsage::CPU_READ_OFTEN`]. If a write flag is passed, it must have been
    /// created with [`HardwareBufferUsage::CPU_WRITE_RARELY`] or
    /// [`HardwareBufferUsage::CPU_WRITE_OFTEN`].
    ///
    /// If `rect` is not [`None`], the caller promises to modify only data in the area specified by
    /// `rect`. If rect is [`None`], the caller may modify the contents of the entire buffer. The
    /// content of the buffer outside of the specified rect is NOT modified by this call.
    ///
    /// It is legal for several different threads to lock a buffer for read access; none of the
    /// threads are blocked.
    ///
    /// Locking a buffer simultaneously for write or read/write is undefined, but will neither
    /// terminate the process nor block the caller. This function may return an error or leave the
    /// buffer's content in an indeterminate state.
    ///
    /// If the buffer has [`HardwareBufferFormat::BLOB`], it is legal lock it for reading and
    /// writing in multiple threads and/or processes simultaneously, and the contents of the buffer
    /// behave like shared memory.
    pub fn lock(
        &self,
        usage: HardwareBufferUsage,
        fence: Option<OwnedFd>,
        rect: Option<Rect>,
    ) -> Result<*mut c_void> {
        let fence = fence.map_or(-1, IntoRawFd::into_raw_fd);
        let rect = match rect {
            Some(rect) => &rect,
            None => std::ptr::null(),
        };
        construct(|res| unsafe {
            ffi::AHardwareBuffer_lock(self.as_ptr(), usage.bits(), fence, rect, res)
        })
    }

    /// Lock a [`HardwareBuffer`] for direct CPU access.
    ///
    /// This function is the same as the above [`lock()`][Self::lock()] function, but passes back
    /// additional information about the bytes per pixel and the bytes per stride of the locked
    /// buffer. If the bytes per pixel or bytes per stride are unknown or variable, or if the
    /// underlying mapper implementation does not support returning additional information, then
    /// this call will fail with [`std::io::Error::kind()`] = [`std::io::ErrorKind::Unsupported`].
    #[cfg(feature = "api-level-29")]
    pub fn lock_and_get_info(
        &self,
        usage: HardwareBufferUsage,
        fence: Option<OwnedFd>,
        rect: Option<Rect>,
    ) -> Result<LockedPlaneInfo> {
        let fence = fence.map_or(-1, IntoRawFd::into_raw_fd);
        let rect = match rect {
            Some(rect) => &rect,
            None => std::ptr::null(),
        };
        let mut virtual_address = MaybeUninit::uninit();
        let mut bytes_per_pixel = MaybeUninit::uninit();
        let mut bytes_per_stride = MaybeUninit::uninit();
        let status = unsafe {
            ffi::AHardwareBuffer_lockAndGetInfo(
                self.as_ptr(),
                usage.bits(),
                fence,
                rect,
                virtual_address.as_mut_ptr(),
                bytes_per_pixel.as_mut_ptr(),
                bytes_per_stride.as_mut_ptr(),
            )
        };
        status_to_io_result(status).map(|()| unsafe {
            LockedPlaneInfo {
                virtual_address: virtual_address.assume_init(),
                bytes_per_pixel: bytes_per_pixel.assume_init() as u32,
                bytes_per_stride: bytes_per_stride.assume_init() as u32,
            }
        })
    }

    /// Lock a potentially multi-planar [`HardwareBuffer`] for direct CPU access.
    ///
    /// This function is similar to [`lock()`][Self::lock()], but can lock multi-planar formats.
    /// Note, that multi-planar should not be confused with multi-layer images, which this locking
    /// function does not support.
    ///
    /// YUV formats are always represented by three separate planes of data, one for each color
    /// plane. The order of planes in the array is guaranteed such that plane #0 is always `Y`,
    /// plane #1 is always `U` (`Cb`), and plane #2 is always `V` (`Cr`). All other formats are
    /// represented by a single plane.
    ///
    /// Additional information always accompanies the buffers, describing the row stride and the
    /// pixel stride for each plane.
    ///
    /// In case the buffer cannot be locked, this will return zero planes.
    ///
    /// See the [`lock()`][Self::lock()] documentation for all other locking semantics.
    #[cfg(feature = "api-level-29")]
    pub fn lock_planes(
        &self,
        usage: HardwareBufferUsage,
        fence: Option<OwnedFd>,
        rect: Option<Rect>,
    ) -> Result<HardwareBufferPlanes> {
        let fence = fence.map_or(-1, IntoRawFd::into_raw_fd);
        let rect = match rect {
            Some(rect) => &rect,
            None => std::ptr::null(),
        };
        let planes = construct(|res| unsafe {
            ffi::AHardwareBuffer_lockPlanes(self.as_ptr(), usage.bits(), fence, rect, res)
        })?;

        Ok(HardwareBufferPlanes {
            inner: planes,
            index: 0,
        })
    }

    /// Unlock the [`HardwareBuffer`] from direct CPU access.
    ///
    /// Must be called after all changes to the buffer are completed by the caller. The function
    /// will block until all work is completed. See [`unlock_async()`][Self::unlock_async()] for
    /// a non-blocking variant that returns a file descriptor to be signaled on unlocking instead.
    pub fn unlock(&self) -> Result<()> {
        let status = unsafe { ffi::AHardwareBuffer_unlock(self.as_ptr(), std::ptr::null_mut()) };
        status_to_io_result(status)
    }

    /// Unlock the [`HardwareBuffer`] from direct CPU access.
    ///
    /// Returns a fence file descriptor that will become signaled when unlocking is completed, or
    /// [`None`] if unlocking is already finished. The caller is responsible for closing the file
    /// descriptor once it's no longer needed. See [`unlock()`][Self::unlock()] for a variant that
    /// blocks instead.
    pub fn unlock_async(&self) -> Result<Option<OwnedFd>> {
        let fence = construct(|res| unsafe { ffi::AHardwareBuffer_unlock(self.as_ptr(), res) })?;
        Ok(match fence {
            -1 => None,
            fence => Some(unsafe { OwnedFd::from_raw_fd(fence) }),
        })
    }

    /// Receive a [`HardwareBuffer`] from an `AF_UNIX` socket.
    ///
    /// `AF_UNIX` sockets are wrapped by [`std::os::unix::net::UnixListener`] and
    /// [`std::os::unix::net::UnixStream`] in Rust and have a corresponding
    /// [`std::os::unix::io::AsFd::as_fd()`] implementation.
    pub fn recv_handle_from_unix_socket(socket_fd: BorrowedFd<'_>) -> Result<Self> {
        unsafe {
            let ptr = construct(|res| {
                ffi::AHardwareBuffer_recvHandleFromUnixSocket(socket_fd.as_raw_fd(), res)
            })?;

            Ok(Self::from_ptr(NonNull::new_unchecked(ptr)))
        }
    }

    /// Send the [`HardwareBuffer`] to an `AF_UNIX` socket.
    ///
    /// `AF_UNIX` sockets are wrapped by [`std::os::unix::net::UnixListener`] and
    /// [`std::os::unix::net::UnixStream`] in Rust and have a corresponding
    /// [`std::os::unix::io::AsFd::as_fd()`] implementation.
    pub fn send_handle_to_unix_socket(&self, socket_fd: BorrowedFd<'_>) -> Result<()> {
        let status = unsafe {
            ffi::AHardwareBuffer_sendHandleToUnixSocket(self.as_ptr(), socket_fd.as_raw_fd())
        };
        status_to_io_result(status)
    }

    /// Acquire a reference on the given [`HardwareBuffer`] object.
    ///
    /// This prevents the object from being deleted until the last strong reference, represented
    /// by [`HardwareBufferRef`], is [`drop()`]ped.
    pub fn acquire(&self) -> HardwareBufferRef {
        unsafe {
            ffi::AHardwareBuffer_acquire(self.as_ptr());
            HardwareBufferRef::from_ptr(self.inner)
        }
    }
}

/// A [`HardwareBuffer`] with an owned reference, that is released when dropped.
/// It behaves much like a strong [`std::rc::Rc`] reference.
#[derive(Debug)]
pub struct HardwareBufferRef {
    inner: HardwareBuffer,
}

impl HardwareBufferRef {
    /// Create an _owned_ [`HardwareBuffer`] from a native pointer
    ///
    /// To wrap a weak reference (that is **not** `release`d on [`Drop`]), call
    /// [`HardwareBuffer::from_ptr()`] instead.
    ///
    /// # Safety
    /// By calling this function, you assert that it is a valid pointer to an NDK
    /// [`ffi::AHardwareBuffer`].
    pub unsafe fn from_ptr(ptr: NonNull<ffi::AHardwareBuffer>) -> Self {
        Self {
            inner: HardwareBuffer { inner: ptr },
        }
    }
}

impl Deref for HardwareBufferRef {
    type Target = HardwareBuffer;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl Drop for HardwareBufferRef {
    fn drop(&mut self) {
        unsafe { ffi::AHardwareBuffer_release(self.inner.as_ptr()) }
    }
}

impl Clone for HardwareBufferRef {
    fn clone(&self) -> Self {
        self.acquire()
    }
}

/// Buffer description.
///
/// Used for allocating new buffers and querying parameters of existing ones.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct HardwareBufferDesc {
    pub width: u32,
    pub height: u32,
    pub layers: u32,
    pub format: HardwareBufferFormat,
    pub usage: HardwareBufferUsage,
    pub stride: u32,
}

impl HardwareBufferDesc {
    fn into_native(self) -> ffi::AHardwareBuffer_Desc {
        ffi::AHardwareBuffer_Desc {
            width: self.width,
            height: self.height,
            layers: self.layers,
            format: i32::from(self.format)
                .try_into()
                .expect("i32->u32 overflow in HardwareBufferDesc::into_native()"),
            usage: self.usage.bits(),
            stride: self.stride,
            rfu0: 0,
            rfu1: 0,
        }
    }
}

/// A native [`AHardwareBuffer_Plane`]
///
/// Contains the same fields as [`ffi::AHardwareBuffer_Plane`].
///
/// [`AHardwareBuffer_Plane`]: https://developer.android.com/ndk/reference/group/a-hardware-buffer#ahardwarebuffer_plane
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct LockedPlaneInfo {
    pub virtual_address: *mut c_void,
    pub bytes_per_pixel: u32,
    pub bytes_per_stride: u32,
}

/// Iterator over [`ffi::AHardwareBuffer_Planes`], containing a list of [`LockedPlaneInfo`].
#[derive(Debug)]
pub struct HardwareBufferPlanes {
    inner: ffi::AHardwareBuffer_Planes,
    index: u32,
}

impl Iterator for HardwareBufferPlanes {
    type Item = LockedPlaneInfo;

    fn next(&mut self) -> Option<LockedPlaneInfo> {
        if self.index == self.inner.planeCount {
            None
        } else {
            let plane = self.inner.planes[self.index as usize];
            self.index += 1;
            Some(LockedPlaneInfo {
                virtual_address: plane.data,
                bytes_per_pixel: plane.pixelStride,
                bytes_per_stride: plane.rowStride,
            })
        }
    }
}
