//! Bindings for [`AImageReader`] and [`AImage`]
//!
//! [`AImageReader`]: https://developer.android.com/ndk/reference/group/media#aimagereader
//! [`AImage`]: https://developer.android.com/ndk/reference/group/media#aimage
#![cfg(feature = "api-level-24")]

use crate::media_error::{construct, construct_never_null, MediaError, Result};
use crate::native_window::NativeWindow;
use crate::utils::abort_on_panic;
use num_enum::{FromPrimitive, IntoPrimitive};
use std::{
    ffi::c_void,
    fmt::{self, Debug, Formatter},
    mem::MaybeUninit,
    ptr::NonNull,
};

#[cfg(feature = "api-level-26")]
use std::os::fd::{FromRawFd, IntoRawFd, OwnedFd};

#[cfg(feature = "api-level-26")]
use crate::hardware_buffer::{HardwareBuffer, HardwareBufferUsage};

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, FromPrimitive, IntoPrimitive)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
pub enum ImageFormat {
    RGBA_8888 = ffi::AIMAGE_FORMATS::AIMAGE_FORMAT_RGBA_8888.0 as i32,
    RGBX_8888 = ffi::AIMAGE_FORMATS::AIMAGE_FORMAT_RGBX_8888.0 as i32,
    RGB_888 = ffi::AIMAGE_FORMATS::AIMAGE_FORMAT_RGB_888.0 as i32,
    RGB_565 = ffi::AIMAGE_FORMATS::AIMAGE_FORMAT_RGB_565.0 as i32,
    RGBA_FP16 = ffi::AIMAGE_FORMATS::AIMAGE_FORMAT_RGBA_FP16.0 as i32,
    YUV_420_888 = ffi::AIMAGE_FORMATS::AIMAGE_FORMAT_YUV_420_888.0 as i32,
    JPEG = ffi::AIMAGE_FORMATS::AIMAGE_FORMAT_JPEG.0 as i32,
    RAW16 = ffi::AIMAGE_FORMATS::AIMAGE_FORMAT_RAW16.0 as i32,
    RAW_PRIVATE = ffi::AIMAGE_FORMATS::AIMAGE_FORMAT_RAW_PRIVATE.0 as i32,
    RAW10 = ffi::AIMAGE_FORMATS::AIMAGE_FORMAT_RAW10.0 as i32,
    RAW12 = ffi::AIMAGE_FORMATS::AIMAGE_FORMAT_RAW12.0 as i32,
    DEPTH16 = ffi::AIMAGE_FORMATS::AIMAGE_FORMAT_DEPTH16.0 as i32,
    DEPTH_POINT_CLOUD = ffi::AIMAGE_FORMATS::AIMAGE_FORMAT_DEPTH_POINT_CLOUD.0 as i32,
    PRIVATE = ffi::AIMAGE_FORMATS::AIMAGE_FORMAT_PRIVATE.0 as i32,
    Y8 = ffi::AIMAGE_FORMATS::AIMAGE_FORMAT_Y8.0 as i32,
    HEIC = ffi::AIMAGE_FORMATS::AIMAGE_FORMAT_HEIC.0 as i32,
    DEPTH_JPEG = ffi::AIMAGE_FORMATS::AIMAGE_FORMAT_DEPTH_JPEG.0 as i32,

    #[doc(hidden)]
    #[num_enum(catch_all)]
    __Unknown(i32),
}

pub type ImageListener = Box<dyn FnMut(&ImageReader) + Send>;

#[cfg(feature = "api-level-26")]
pub type BufferRemovedListener = Box<dyn FnMut(&ImageReader, &HardwareBuffer) + Send>;

/// Result returned by:
/// - [`ImageReader::acquire_next_image()`]`
/// - [`ImageReader::acquire_next_image_async()`]`
/// - [`ImageReader::acquire_latest_image()`]`
/// - [`ImageReader::acquire_latest_image_async()`]`
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AcquireResult<T> {
    /// Returned if there is no buffers currently available in the reader queue.
    #[doc(alias = "AMEDIA_IMGREADER_NO_BUFFER_AVAILABLE")]
    NoBufferAvailable,
    /// Returned if the number of concurrently acquired images has reached the limit.
    #[doc(alias = "AMEDIA_IMGREADER_MAX_IMAGES_ACQUIRED")]
    MaxImagesAcquired,

    /// Returned if an [`Image`] (optionally with fence) was successfully acquired.
    Image(T),
}

impl<T> AcquireResult<T> {
    fn map<U>(self, f: impl FnOnce(T) -> U) -> AcquireResult<U> {
        match self {
            AcquireResult::Image(img) => AcquireResult::Image(f(img)),
            AcquireResult::NoBufferAvailable => AcquireResult::NoBufferAvailable,
            AcquireResult::MaxImagesAcquired => AcquireResult::MaxImagesAcquired,
        }
    }
}

impl AcquireResult<Image> {
    /// Inlined version of [`construct_never_null()`] with IMGREADER-specific result mapping.
    fn construct_never_null(
        with_ptr: impl FnOnce(*mut *mut ffi::AImage) -> ffi::media_status_t,
    ) -> Result<Self> {
        let mut result = MaybeUninit::uninit();
        let status = with_ptr(result.as_mut_ptr());
        match status {
            ffi::media_status_t::AMEDIA_IMGREADER_NO_BUFFER_AVAILABLE => {
                Ok(Self::NoBufferAvailable)
            }
            ffi::media_status_t::AMEDIA_IMGREADER_MAX_IMAGES_ACQUIRED => {
                Ok(Self::MaxImagesAcquired)
            }
            status => MediaError::from_status(status).map(|()| {
                let result = unsafe { result.assume_init() };
                Self::Image(Image {
                    inner: if cfg!(debug_assertions) {
                        NonNull::new(result).expect("result should never be null")
                    } else {
                        unsafe { NonNull::new_unchecked(result) }
                    },
                })
            }),
        }
    }
}

/// A native [`AImageReader *`]
///
/// [`AImageReader *`]: https://developer.android.com/ndk/reference/group/media#aimagereader
pub struct ImageReader {
    inner: NonNull<ffi::AImageReader>,
    image_cb: Option<Box<ImageListener>>,
    #[cfg(feature = "api-level-26")]
    buffer_removed_cb: Option<Box<BufferRemovedListener>>,
}

impl Debug for ImageReader {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("ImageReader")
            .field("inner", &self.inner)
            .field(
                "image_cb",
                match &self.image_cb {
                    Some(_) => &"Some(_)",
                    None => &"None",
                },
            )
            .finish()
    }
}

impl ImageReader {
    fn from_ptr(inner: NonNull<ffi::AImageReader>) -> Self {
        Self {
            inner,
            image_cb: None,
            #[cfg(feature = "api-level-26")]
            buffer_removed_cb: None,
        }
    }

    fn as_ptr(&self) -> *mut ffi::AImageReader {
        self.inner.as_ptr()
    }

    pub fn new(width: i32, height: i32, format: ImageFormat, max_images: i32) -> Result<Self> {
        let inner = construct_never_null(|res| unsafe {
            ffi::AImageReader_new(width, height, format.into(), max_images, res)
        })?;

        Ok(Self::from_ptr(inner))
    }

    #[cfg(feature = "api-level-26")]
    pub fn new_with_usage(
        width: i32,
        height: i32,
        format: ImageFormat,
        usage: HardwareBufferUsage,
        max_images: i32,
    ) -> Result<Self> {
        let inner = construct_never_null(|res| unsafe {
            ffi::AImageReader_newWithUsage(
                width,
                height,
                format.into(),
                usage.bits(),
                max_images,
                res,
            )
        })?;

        Ok(Self::from_ptr(inner))
    }

    #[doc(alias = "AImageReader_setImageListener")]
    pub fn set_image_listener(&mut self, listener: ImageListener) -> Result<()> {
        let mut boxed = Box::new(listener);
        let ptr: *mut ImageListener = &mut *boxed;

        unsafe extern "C" fn on_image_available(
            context: *mut c_void,
            reader: *mut ffi::AImageReader,
        ) {
            abort_on_panic(|| {
                let reader = ImageReader::from_ptr(NonNull::new_unchecked(reader));
                let listener: *mut ImageListener = context.cast();
                (*listener)(&reader);
                std::mem::forget(reader);
            })
        }

        let mut listener = ffi::AImageReader_ImageListener {
            context: ptr as _,
            onImageAvailable: Some(on_image_available),
        };
        let status = unsafe { ffi::AImageReader_setImageListener(self.as_ptr(), &mut listener) };

        // keep listener alive until Drop or new listener is assigned
        self.image_cb = Some(boxed);

        MediaError::from_status(status)
    }

    #[cfg(feature = "api-level-26")]
    #[doc(alias = "AImageReader_setBufferRemovedListener")]
    pub fn set_buffer_removed_listener(&mut self, listener: BufferRemovedListener) -> Result<()> {
        let mut boxed = Box::new(listener);
        let ptr: *mut BufferRemovedListener = &mut *boxed;

        unsafe extern "C" fn on_buffer_removed(
            context: *mut c_void,
            reader: *mut ffi::AImageReader,
            buffer: *mut ffi::AHardwareBuffer,
        ) {
            abort_on_panic(|| {
                let reader = ImageReader::from_ptr(NonNull::new_unchecked(reader));
                let buffer = HardwareBuffer::from_ptr(NonNull::new_unchecked(buffer));
                let listener: *mut BufferRemovedListener = context.cast();
                (*listener)(&reader, &buffer);
                std::mem::forget(reader);
            })
        }

        let mut listener = ffi::AImageReader_BufferRemovedListener {
            context: ptr as _,
            onBufferRemoved: Some(on_buffer_removed),
        };
        let status =
            unsafe { ffi::AImageReader_setBufferRemovedListener(self.as_ptr(), &mut listener) };

        // keep listener alive until Drop or new listener is assigned
        self.buffer_removed_cb = Some(boxed);

        MediaError::from_status(status)
    }

    /// Get a [`NativeWindow`] that can be used to produce [`Image`]s for this [`ImageReader`].
    ///
    /// <https://developer.android.com/ndk/reference/group/media#aimagereader_getwindow>
    #[doc(alias = "AImageReader_getWindow")]
    pub fn window(&self) -> Result<NativeWindow> {
        unsafe {
            let ptr = construct_never_null(|res| ffi::AImageReader_getWindow(self.as_ptr(), res))?;
            Ok(NativeWindow::clone_from_ptr(ptr))
        }
    }

    #[doc(alias = "AImageReader_getWidth")]
    pub fn width(&self) -> Result<i32> {
        construct(|res| unsafe { ffi::AImageReader_getWidth(self.as_ptr(), res) })
    }

    #[doc(alias = "AImageReader_getHeight")]
    pub fn height(&self) -> Result<i32> {
        construct(|res| unsafe { ffi::AImageReader_getHeight(self.as_ptr(), res) })
    }

    #[doc(alias = "AImageReader_getFormat")]
    pub fn format(&self) -> Result<ImageFormat> {
        let format = construct(|res| unsafe { ffi::AImageReader_getFormat(self.as_ptr(), res) })?;
        Ok(format.into())
    }

    #[doc(alias = "AImageReader_getMaxImages")]
    pub fn max_images(&self) -> Result<i32> {
        construct(|res| unsafe { ffi::AImageReader_getMaxImages(self.as_ptr(), res) })
    }

    #[doc(alias = "AImageReader_acquireNextImage")]
    pub fn acquire_next_image(&self) -> Result<AcquireResult<Image>> {
        AcquireResult::construct_never_null(|res| unsafe {
            ffi::AImageReader_acquireNextImage(self.as_ptr(), res)
        })
    }

    /// Acquire the next [`Image`] from the image reader's queue asynchronously.
    ///
    /// # Safety
    /// If the returned file descriptor is not [`None`], it must be awaited before attempting to
    /// access the [`Image`] returned.
    ///
    /// <https://developer.android.com/ndk/reference/group/media#aimagereader_acquirenextimageasync>
    #[cfg(feature = "api-level-26")]
    #[doc(alias = "AImageReader_acquireNextImageAsync")]
    pub unsafe fn acquire_next_image_async(
        &self,
    ) -> Result<AcquireResult<(Image, Option<OwnedFd>)>> {
        let mut fence = MaybeUninit::uninit();
        AcquireResult::construct_never_null(|res| {
            ffi::AImageReader_acquireNextImageAsync(self.as_ptr(), res, fence.as_mut_ptr())
        })
        .map(|result| {
            result.map(|image| match fence.assume_init() {
                -1 => (image, None),
                fence => (image, Some(unsafe { OwnedFd::from_raw_fd(fence) })),
            })
        })
    }

    #[doc(alias = "AImageReader_acquireLatestImage")]
    pub fn acquire_latest_image(&self) -> Result<AcquireResult<Image>> {
        AcquireResult::construct_never_null(|res| unsafe {
            ffi::AImageReader_acquireLatestImage(self.as_ptr(), res)
        })
    }

    /// Acquire the latest [`Image`] from the image reader's queue asynchronously, dropping older images.
    ///
    /// # Safety
    /// If the returned file descriptor is not [`None`], it must be awaited before attempting to
    /// access the [`Image`] returned.
    ///
    /// <https://developer.android.com/ndk/reference/group/media#aimagereader_acquirelatestimageasync>
    #[cfg(feature = "api-level-26")]
    #[doc(alias = "AImageReader_acquireLatestImageAsync")]
    pub unsafe fn acquire_latest_image_async(
        &self,
    ) -> Result<AcquireResult<(Image, Option<OwnedFd>)>> {
        let mut fence = MaybeUninit::uninit();
        AcquireResult::construct_never_null(|res| {
            ffi::AImageReader_acquireLatestImageAsync(self.as_ptr(), res, fence.as_mut_ptr())
        })
        .map(|result| {
            result.map(|image| match fence.assume_init() {
                -1 => (image, None),
                fence => (image, Some(unsafe { OwnedFd::from_raw_fd(fence) })),
            })
        })
    }
}

impl Drop for ImageReader {
    #[doc(alias = "AImageReader_delete")]
    fn drop(&mut self) {
        unsafe { ffi::AImageReader_delete(self.as_ptr()) };
    }
}

/// A native [`AImage *`]
///
/// [`AImage *`]: https://developer.android.com/ndk/reference/group/media#aimage
#[derive(Debug)]
#[doc(alias = "AImage")]
pub struct Image {
    inner: NonNull<ffi::AImage>,
}

#[doc(alias = "AImageCropRect")]
pub type CropRect = ffi::AImageCropRect;

impl Image {
    fn as_ptr(&self) -> *mut ffi::AImage {
        self.inner.as_ptr()
    }

    #[doc(alias = "AImage_getPlaneData")]
    pub fn plane_data(&self, plane_idx: i32) -> Result<&[u8]> {
        let mut result_ptr = MaybeUninit::uninit();
        let mut result_len = MaybeUninit::uninit();
        let status = unsafe {
            ffi::AImage_getPlaneData(
                self.as_ptr(),
                plane_idx,
                result_ptr.as_mut_ptr(),
                result_len.as_mut_ptr(),
            )
        };

        MediaError::from_status(status).map(|()| unsafe {
            std::slice::from_raw_parts(result_ptr.assume_init(), result_len.assume_init() as _)
        })
    }

    #[doc(alias = "AImage_getPlanePixelStride")]
    pub fn plane_pixel_stride(&self, plane_idx: i32) -> Result<i32> {
        construct(|res| unsafe { ffi::AImage_getPlanePixelStride(self.as_ptr(), plane_idx, res) })
    }

    #[doc(alias = "AImage_getPlaneRowStride")]
    pub fn plane_row_stride(&self, plane_idx: i32) -> Result<i32> {
        construct(|res| unsafe { ffi::AImage_getPlaneRowStride(self.as_ptr(), plane_idx, res) })
    }

    #[doc(alias = "AImage_getCropRect")]
    pub fn crop_rect(&self) -> Result<CropRect> {
        construct(|res| unsafe { ffi::AImage_getCropRect(self.as_ptr(), res) })
    }

    #[doc(alias = "AImage_getWidth")]
    pub fn width(&self) -> Result<i32> {
        construct(|res| unsafe { ffi::AImage_getWidth(self.as_ptr(), res) })
    }

    #[doc(alias = "AImage_getHeight")]
    pub fn height(&self) -> Result<i32> {
        construct(|res| unsafe { ffi::AImage_getHeight(self.as_ptr(), res) })
    }

    #[doc(alias = "AImage_getFormat")]
    pub fn format(&self) -> Result<ImageFormat> {
        let format = construct(|res| unsafe { ffi::AImage_getFormat(self.as_ptr(), res) })?;
        Ok(format.into())
    }

    #[doc(alias = "AImage_getTimestamp")]
    pub fn timestamp(&self) -> Result<i64> {
        construct(|res| unsafe { ffi::AImage_getTimestamp(self.as_ptr(), res) })
    }

    #[doc(alias = "AImage_getNumberOfPlanes")]
    pub fn number_of_planes(&self) -> Result<i32> {
        construct(|res| unsafe { ffi::AImage_getNumberOfPlanes(self.as_ptr(), res) })
    }

    /// Get the hardware buffer handle of the input image intended for GPU and/or hardware access.
    ///
    /// Note that no reference on the returned [`HardwareBuffer`] handle is acquired automatically.
    /// Once the [`Image`] or the parent [`ImageReader`] is deleted, the [`HardwareBuffer`] handle
    /// from previous [`Image::hardware_buffer()`] becomes invalid.
    ///
    /// If the caller ever needs to hold on a reference to the [`HardwareBuffer`] handle after the
    /// [`Image`] or the parent [`ImageReader`] is deleted, it must call
    /// [`HardwareBuffer::acquire()`] to acquire an extra reference, and [`drop()`] it when
    /// finished using it in order to properly deallocate the underlying memory managed by
    /// [`HardwareBuffer`]. If the caller has acquired an extra reference on a [`HardwareBuffer`]
    /// returned from this function, it must also register a listener using
    /// [`ImageReader::set_buffer_removed_listener()`] to be notified when the buffer is no longer
    /// used by [`ImageReader`].
    #[cfg(feature = "api-level-26")]
    #[doc(alias = "AImage_getHardwareBuffer")]
    pub fn hardware_buffer(&self) -> Result<HardwareBuffer> {
        unsafe {
            let ptr =
                construct_never_null(|res| ffi::AImage_getHardwareBuffer(self.as_ptr(), res))?;
            Ok(HardwareBuffer::from_ptr(ptr))
        }
    }

    #[cfg(feature = "api-level-26")]
    #[doc(alias = "AImage_deleteAsync")]
    pub fn delete_async(self, release_fence_fd: OwnedFd) {
        unsafe { ffi::AImage_deleteAsync(self.as_ptr(), release_fence_fd.into_raw_fd()) };
        std::mem::forget(self);
    }
}

impl Drop for Image {
    #[doc(alias = "AImage_delete")]
    fn drop(&mut self) {
        unsafe { ffi::AImage_delete(self.as_ptr()) };
    }
}
