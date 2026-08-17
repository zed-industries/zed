// Copyright 2025 LiveKit, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::fmt::Debug;

use thiserror::Error;

use crate::imp::video_frame as vf_imp;

#[derive(Debug, Error)]
pub enum SinkError {
    #[error("platform error: {0}")]
    Platform(String),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum VideoRotation {
    VideoRotation0 = 0,
    VideoRotation90 = 90,
    VideoRotation180 = 180,
    VideoRotation270 = 270,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum VideoFormatType {
    ARGB,
    BGRA,
    ABGR,
    RGBA,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum VideoBufferType {
    Native,
    I420,
    I420A,
    I422,
    I444,
    I010,
    NV12,
}

#[derive(Debug)]
pub struct VideoFrame<T>
where
    T: AsRef<dyn VideoBuffer>,
{
    pub rotation: VideoRotation,
    pub timestamp_us: i64, // When the frame was captured in microseconds
    pub buffer: T,
}

pub type BoxVideoBuffer = Box<dyn VideoBuffer>;
pub type BoxVideoFrame = VideoFrame<BoxVideoBuffer>;

pub(crate) mod internal {
    use super::{I420Buffer, VideoFormatType};

    pub trait BufferSealed: Send + Sync {
        #[cfg(not(target_arch = "wasm32"))]
        fn sys_handle(&self) -> &webrtc_sys::video_frame_buffer::ffi::VideoFrameBuffer;

        #[cfg(not(target_arch = "wasm32"))]
        fn to_i420(&self) -> I420Buffer;

        #[cfg(not(target_arch = "wasm32"))]
        fn to_argb(
            &self,
            format: VideoFormatType,
            dst: &mut [u8],
            dst_stride: u32,
            dst_width: i32,
            dst_height: i32,
        );
    }
}

pub trait VideoBuffer: internal::BufferSealed + Debug {
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn buffer_type(&self) -> VideoBufferType;

    #[cfg(not(target_arch = "wasm32"))]
    fn as_native(&self) -> Option<&native::NativeBuffer> {
        None
    }

    fn as_i420(&self) -> Option<&I420Buffer> {
        None
    }

    fn as_i420a(&self) -> Option<&I420ABuffer> {
        None
    }

    fn as_i422(&self) -> Option<&I422Buffer> {
        None
    }

    fn as_i444(&self) -> Option<&I444Buffer> {
        None
    }

    fn as_i010(&self) -> Option<&I010Buffer> {
        None
    }

    fn as_nv12(&self) -> Option<&NV12Buffer> {
        None
    }
}

macro_rules! new_buffer_type {
    ($type:ident, $variant:ident, $as:ident) => {
        pub struct $type {
            pub(crate) handle: vf_imp::$type,
        }

        impl $crate::video_frame::internal::BufferSealed for $type {
            #[cfg(not(target_arch = "wasm32"))]
            fn sys_handle(&self) -> &webrtc_sys::video_frame_buffer::ffi::VideoFrameBuffer {
                self.handle.sys_handle()
            }

            #[cfg(not(target_arch = "wasm32"))]
            fn to_i420(&self) -> I420Buffer {
                I420Buffer { handle: self.handle.to_i420() }
            }

            #[cfg(not(target_arch = "wasm32"))]
            fn to_argb(
                &self,
                format: VideoFormatType,
                dst: &mut [u8],
                stride: u32,
                width: i32,
                height: i32,
            ) {
                self.handle.to_argb(format, dst, stride, width, height)
            }
        }

        impl VideoBuffer for $type {
            fn width(&self) -> u32 {
                self.handle.width()
            }

            fn height(&self) -> u32 {
                self.handle.height()
            }

            fn buffer_type(&self) -> VideoBufferType {
                VideoBufferType::$variant
            }

            fn $as(&self) -> Option<&$type> {
                Some(self)
            }
        }

        impl Debug for $type {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(stringify!($type))
                    .field("width", &self.width())
                    .field("height", &self.height())
                    .finish()
            }
        }

        impl AsRef<dyn VideoBuffer> for $type {
            fn as_ref(&self) -> &(dyn VideoBuffer + 'static) {
                self
            }
        }
    };
}

new_buffer_type!(I420Buffer, I420, as_i420);
new_buffer_type!(I420ABuffer, I420A, as_i420a);
new_buffer_type!(I422Buffer, I422, as_i422);
new_buffer_type!(I444Buffer, I444, as_i444);
new_buffer_type!(I010Buffer, I010, as_i010);
new_buffer_type!(NV12Buffer, NV12, as_nv12);

impl I420Buffer {
    pub fn with_strides(
        width: u32,
        height: u32,
        stride_y: u32,
        stride_u: u32,
        stride_v: u32,
    ) -> I420Buffer {
        vf_imp::I420Buffer::new(width, height, stride_y, stride_u, stride_v)
    }

    pub fn new(width: u32, height: u32) -> I420Buffer {
        Self::with_strides(width, height, width, (width + 1) / 2, (width + 1) / 2)
    }

    pub fn chroma_width(&self) -> u32 {
        self.handle.chroma_width()
    }

    pub fn chroma_height(&self) -> u32 {
        self.handle.chroma_height()
    }

    pub fn strides(&self) -> (u32, u32, u32) {
        (self.handle.stride_y(), self.handle.stride_u(), self.handle.stride_v())
    }

    pub fn data(&self) -> (&[u8], &[u8], &[u8]) {
        self.handle.data()
    }

    pub fn data_mut(&mut self) -> (&mut [u8], &mut [u8], &mut [u8]) {
        let (data_y, data_u, data_v) = self.handle.data();
        unsafe {
            (
                std::slice::from_raw_parts_mut(data_y.as_ptr() as *mut u8, data_y.len()),
                std::slice::from_raw_parts_mut(data_u.as_ptr() as *mut u8, data_u.len()),
                std::slice::from_raw_parts_mut(data_v.as_ptr() as *mut u8, data_v.len()),
            )
        }
    }

    pub fn scale(&mut self, scaled_width: i32, scaled_height: i32) -> I420Buffer {
        self.handle.scale(scaled_width, scaled_height)
    }
}

impl I420ABuffer {
    pub fn chroma_width(&self) -> u32 {
        self.handle.chroma_width()
    }

    pub fn chroma_height(&self) -> u32 {
        self.handle.chroma_height()
    }

    pub fn strides(&self) -> (u32, u32, u32, u32) {
        (
            self.handle.stride_y(),
            self.handle.stride_u(),
            self.handle.stride_v(),
            self.handle.stride_a(),
        )
    }

    #[allow(clippy::type_complexity)]
    pub fn data(&self) -> (&[u8], &[u8], &[u8], Option<&[u8]>) {
        self.handle.data()
    }

    #[allow(clippy::type_complexity)]
    pub fn data_mut(&self) -> (&mut [u8], &mut [u8], &mut [u8], Option<&mut [u8]>) {
        let (data_y, data_u, data_v, data_a) = self.handle.data();
        unsafe {
            (
                std::slice::from_raw_parts_mut(data_y.as_ptr() as *mut u8, data_y.len()),
                std::slice::from_raw_parts_mut(data_u.as_ptr() as *mut u8, data_u.len()),
                std::slice::from_raw_parts_mut(data_v.as_ptr() as *mut u8, data_v.len()),
                data_a.map(|data_a| {
                    std::slice::from_raw_parts_mut(data_a.as_ptr() as *mut u8, data_a.len())
                }),
            )
        }
    }

    pub fn scale(&mut self, scaled_width: i32, scaled_height: i32) -> I420ABuffer {
        self.handle.scale(scaled_width, scaled_height)
    }
}

impl I422Buffer {
    pub fn with_strides(
        width: u32,
        height: u32,
        stride_y: u32,
        stride_u: u32,
        stride_v: u32,
    ) -> I422Buffer {
        vf_imp::I422Buffer::new(width, height, stride_y, stride_u, stride_v)
    }

    pub fn new(width: u32, height: u32) -> I422Buffer {
        Self::with_strides(width, height, width, (width + 1) / 2, (width + 1) / 2)
    }

    pub fn chroma_width(&self) -> u32 {
        self.handle.chroma_width()
    }

    pub fn chroma_height(&self) -> u32 {
        self.handle.chroma_height()
    }

    pub fn strides(&self) -> (u32, u32, u32) {
        (self.handle.stride_y(), self.handle.stride_u(), self.handle.stride_v())
    }

    pub fn data(&self) -> (&[u8], &[u8], &[u8]) {
        self.handle.data()
    }

    pub fn data_mut(&mut self) -> (&mut [u8], &mut [u8], &mut [u8]) {
        let (data_y, data_u, data_v) = self.handle.data();
        unsafe {
            (
                std::slice::from_raw_parts_mut(data_y.as_ptr() as *mut u8, data_y.len()),
                std::slice::from_raw_parts_mut(data_u.as_ptr() as *mut u8, data_u.len()),
                std::slice::from_raw_parts_mut(data_v.as_ptr() as *mut u8, data_v.len()),
            )
        }
    }

    pub fn scale(&mut self, scaled_width: i32, scaled_height: i32) -> I422Buffer {
        self.handle.scale(scaled_width, scaled_height)
    }
}

impl I444Buffer {
    pub fn with_strides(
        width: u32,
        height: u32,
        stride_y: u32,
        stride_u: u32,
        stride_v: u32,
    ) -> I444Buffer {
        vf_imp::I444Buffer::new(width, height, stride_y, stride_u, stride_v)
    }

    pub fn new(width: u32, height: u32) -> I444Buffer {
        Self::with_strides(width, height, width, width, width)
    }

    pub fn chroma_width(&self) -> u32 {
        self.handle.chroma_width()
    }

    pub fn chroma_height(&self) -> u32 {
        self.handle.chroma_height()
    }

    pub fn strides(&self) -> (u32, u32, u32) {
        (self.handle.stride_y(), self.handle.stride_u(), self.handle.stride_v())
    }

    pub fn data(&self) -> (&[u8], &[u8], &[u8]) {
        self.handle.data()
    }

    pub fn data_mut(&mut self) -> (&mut [u8], &mut [u8], &mut [u8]) {
        let (data_y, data_u, data_v) = self.handle.data();
        unsafe {
            (
                std::slice::from_raw_parts_mut(data_y.as_ptr() as *mut u8, data_y.len()),
                std::slice::from_raw_parts_mut(data_u.as_ptr() as *mut u8, data_u.len()),
                std::slice::from_raw_parts_mut(data_v.as_ptr() as *mut u8, data_v.len()),
            )
        }
    }

    pub fn scale(&mut self, scaled_width: i32, scaled_height: i32) -> I444Buffer {
        self.handle.scale(scaled_width, scaled_height)
    }
}

impl I010Buffer {
    pub fn with_strides(
        width: u32,
        height: u32,
        stride_y: u32,
        stride_u: u32,
        stride_v: u32,
    ) -> I010Buffer {
        vf_imp::I010Buffer::new(width, height, stride_y, stride_u, stride_v)
    }

    pub fn new(width: u32, height: u32) -> I010Buffer {
        Self::with_strides(width, height, width, (width + 1) / 2, (width + 1) / 2)
    }

    pub fn chroma_width(&self) -> u32 {
        self.handle.chroma_width()
    }

    pub fn chroma_height(&self) -> u32 {
        self.handle.chroma_height()
    }

    pub fn strides(&self) -> (u32, u32, u32) {
        (self.handle.stride_y(), self.handle.stride_u(), self.handle.stride_v())
    }

    pub fn data(&self) -> (&[u16], &[u16], &[u16]) {
        self.handle.data()
    }

    pub fn data_mut(&mut self) -> (&mut [u16], &mut [u16], &mut [u16]) {
        let (data_y, data_u, data_v) = self.handle.data();
        unsafe {
            (
                std::slice::from_raw_parts_mut(data_y.as_ptr() as *mut u16, data_y.len()),
                std::slice::from_raw_parts_mut(data_u.as_ptr() as *mut u16, data_u.len()),
                std::slice::from_raw_parts_mut(data_v.as_ptr() as *mut u16, data_v.len()),
            )
        }
    }

    pub fn scale(&mut self, scaled_width: i32, scaled_height: i32) -> I010Buffer {
        self.handle.scale(scaled_width, scaled_height)
    }
}

impl NV12Buffer {
    pub fn with_strides(width: u32, height: u32, stride_y: u32, stride_uv: u32) -> NV12Buffer {
        vf_imp::NV12Buffer::new(width, height, stride_y, stride_uv)
    }

    pub fn new(width: u32, height: u32) -> NV12Buffer {
        Self::with_strides(width, height, width, width + width % 2)
    }

    pub fn chroma_width(&self) -> u32 {
        self.handle.chroma_width()
    }

    pub fn chroma_height(&self) -> u32 {
        self.handle.chroma_height()
    }

    pub fn strides(&self) -> (u32, u32) {
        (self.handle.stride_y(), self.handle.stride_uv())
    }

    pub fn data(&self) -> (&[u8], &[u8]) {
        self.handle.data()
    }

    pub fn data_mut(&mut self) -> (&mut [u8], &mut [u8]) {
        let (data_y, data_uv) = self.handle.data();
        unsafe {
            (
                std::slice::from_raw_parts_mut(data_y.as_ptr() as *mut u8, data_y.len()),
                std::slice::from_raw_parts_mut(data_uv.as_ptr() as *mut u8, data_uv.len()),
            )
        }
    }

    pub fn scale(&mut self, scaled_width: i32, scaled_height: i32) -> NV12Buffer {
        self.handle.scale(scaled_width, scaled_height)
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub mod native {
    use std::fmt::Debug;

    use super::{vf_imp, I420Buffer, VideoBuffer, VideoBufferType, VideoFormatType};

    new_buffer_type!(NativeBuffer, Native, as_native);

    impl NativeBuffer {
        /// Creates a `NativeBuffer` from a `CVPixelBufferRef` pointer.
        ///
        /// This function does not bump the reference count of the pixel buffer.
        ///
        /// Safety: The given pointer must be a valid `CVPixelBufferRef`.
        #[cfg(any(target_os = "macos", target_os = "ios"))]
        pub unsafe fn from_cv_pixel_buffer(cv_pixel_buffer: *mut std::ffi::c_void) -> Self {
            vf_imp::NativeBuffer::from_cv_pixel_buffer(cv_pixel_buffer)
        }

        /// Returns the `CVPixelBufferRef` that backs this buffer, or `null` if
        /// this buffer is not currently backed by a `CVPixelBufferRef`.
        ///
        /// This function does not bump the reference count of the pixel buffer.
        #[cfg(any(target_os = "macos", target_os = "ios"))]
        pub fn get_cv_pixel_buffer(&self) -> *mut std::ffi::c_void {
            self.handle.get_cv_pixel_buffer()
        }
    }

    pub trait VideoFrameBufferExt: VideoBuffer {
        fn to_i420(&self) -> I420Buffer;
        fn to_argb(
            &self,
            format: VideoFormatType,
            dst: &mut [u8],
            dst_stride: u32,
            dst_width: i32,
            dst_height: i32,
        );
    }

    impl<T: VideoBuffer> VideoFrameBufferExt for T {
        fn to_i420(&self) -> I420Buffer {
            self.to_i420()
        }

        fn to_argb(
            &self,
            format: VideoFormatType,
            dst: &mut [u8],
            dst_stride: u32,
            dst_width: i32,
            dst_height: i32,
        ) {
            self.to_argb(format, dst, dst_stride, dst_width, dst_height)
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub mod web {
    use super::VideoFrameBuffer;

    #[derive(Debug)]
    pub struct WebGlBuffer {}

    impl VideoFrameBuffer for WebGlBuffer {}
}
