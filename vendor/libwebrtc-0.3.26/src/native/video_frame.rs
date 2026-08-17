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

use std::slice;

use cxx::UniquePtr;
use webrtc_sys::{video_frame as vf_sys, video_frame_buffer as vfb_sys};

use super::yuv_helper;
use crate::video_frame::{self as vf, VideoFormatType, VideoRotation};

/// We don't use vf::VideoFrameBuffer trait for the types inside this module to avoid confusion
/// because directly using platform specific types is not valid (e.g user callback)
/// All the types inside this module are only used internally. For public types, see the top level
/// video_frame.rs

pub fn new_video_frame_buffer(
    mut sys_handle: UniquePtr<vfb_sys::ffi::VideoFrameBuffer>,
) -> Box<dyn vf::VideoBuffer + Send + Sync> {
    unsafe {
        match sys_handle.buffer_type() {
            vfb_sys::ffi::VideoFrameBufferType::Native => {
                Box::new(vf::native::NativeBuffer { handle: NativeBuffer { sys_handle } })
            }
            vfb_sys::ffi::VideoFrameBufferType::I420 => Box::new(vf::I420Buffer {
                handle: I420Buffer { sys_handle: sys_handle.pin_mut().get_i420() },
            }),
            vfb_sys::ffi::VideoFrameBufferType::I420A => Box::new(vf::I420ABuffer {
                handle: I420ABuffer { sys_handle: sys_handle.pin_mut().get_i420a() },
            }),
            vfb_sys::ffi::VideoFrameBufferType::I422 => Box::new(vf::I422Buffer {
                handle: I422Buffer { sys_handle: sys_handle.pin_mut().get_i422() },
            }),
            vfb_sys::ffi::VideoFrameBufferType::I444 => Box::new(vf::I444Buffer {
                handle: I444Buffer { sys_handle: sys_handle.pin_mut().get_i444() },
            }),
            vfb_sys::ffi::VideoFrameBufferType::I010 => Box::new(vf::I010Buffer {
                handle: I010Buffer { sys_handle: sys_handle.pin_mut().get_i010() },
            }),
            vfb_sys::ffi::VideoFrameBufferType::NV12 => Box::new(vf::NV12Buffer {
                handle: NV12Buffer { sys_handle: sys_handle.pin_mut().get_nv12() },
            }),
            _ => unreachable!(),
        }
    }
}

impl From<vf_sys::ffi::VideoRotation> for VideoRotation {
    fn from(rotation: vf_sys::ffi::VideoRotation) -> Self {
        match rotation {
            vf_sys::ffi::VideoRotation::VideoRotation0 => Self::VideoRotation0,
            vf_sys::ffi::VideoRotation::VideoRotation90 => Self::VideoRotation90,
            vf_sys::ffi::VideoRotation::VideoRotation180 => Self::VideoRotation180,
            vf_sys::ffi::VideoRotation::VideoRotation270 => Self::VideoRotation270,
            _ => panic!("invalid VideoRotation"),
        }
    }
}

impl From<VideoRotation> for vf_sys::ffi::VideoRotation {
    fn from(rotation: VideoRotation) -> Self {
        match rotation {
            VideoRotation::VideoRotation0 => Self::VideoRotation0,
            VideoRotation::VideoRotation90 => Self::VideoRotation90,
            VideoRotation::VideoRotation180 => Self::VideoRotation180,
            VideoRotation::VideoRotation270 => Self::VideoRotation270,
        }
    }
}

macro_rules! recursive_cast {
    ($ptr:expr $(, $fnc:ident)*) => {
        {
            let ptr = $ptr;
            $(
                let ptr = vfb_sys::ffi::$fnc(ptr);
            )*
            ptr
        }
    };
}

pub struct NativeBuffer {
    sys_handle: UniquePtr<vfb_sys::ffi::VideoFrameBuffer>,
}

pub struct I420Buffer {
    sys_handle: UniquePtr<vfb_sys::ffi::I420Buffer>,
}

pub struct I420ABuffer {
    sys_handle: UniquePtr<vfb_sys::ffi::I420ABuffer>,
}

pub struct I422Buffer {
    sys_handle: UniquePtr<vfb_sys::ffi::I422Buffer>,
}

pub struct I444Buffer {
    sys_handle: UniquePtr<vfb_sys::ffi::I444Buffer>,
}

pub struct I010Buffer {
    sys_handle: UniquePtr<vfb_sys::ffi::I010Buffer>,
}

pub struct NV12Buffer {
    sys_handle: UniquePtr<vfb_sys::ffi::NV12Buffer>,
}

macro_rules! impl_to_argb {
    (I420Buffer [$($variant:ident: $fnc:ident),+], $format:ident, $self:ident, $dst:ident, $dst_stride:ident, $dst_width:ident, $dst_height:ident) => {
        match $format {
        $(
            VideoFormatType::$variant => {
                let (data_y, data_u, data_v) = $self.data();
                yuv_helper::$fnc(
                    data_y,
                    $self.stride_y(),
                    data_u,
                    $self.stride_u(),
                    data_v,
                    $self.stride_v(),
                    $dst,
                    $dst_stride,
                    $dst_width,
                    $dst_height,
                )
            }
        )+
        }
    };
    (I420ABuffer) => {
        todo!();
    }
}

#[allow(unused_unsafe)]
impl NativeBuffer {
    #[cfg(any(target_os = "macos", target_os = "ios"))]
    pub unsafe fn from_cv_pixel_buffer(
        cv_pixel_buffer: *mut std::ffi::c_void,
    ) -> vf::native::NativeBuffer {
        vf::native::NativeBuffer {
            handle: NativeBuffer {
                sys_handle: vfb_sys::ffi::new_native_buffer_from_platform_image_buffer(
                    cv_pixel_buffer as *mut _,
                ),
            },
        }
    }

    #[cfg(any(target_os = "macos", target_os = "ios"))]
    pub fn get_cv_pixel_buffer(&self) -> *mut std::ffi::c_void {
        unsafe { vfb_sys::ffi::native_buffer_to_platform_image_buffer(&self.sys_handle) as *mut _ }
    }

    pub fn sys_handle(&self) -> &vfb_sys::ffi::VideoFrameBuffer {
        &self.sys_handle
    }

    pub fn width(&self) -> u32 {
        self.sys_handle.width()
    }

    pub fn height(&self) -> u32 {
        self.sys_handle.height()
    }

    pub fn to_i420(&self) -> I420Buffer {
        I420Buffer { sys_handle: unsafe { self.sys_handle.to_i420() } }
    }

    pub fn to_argb(
        &self,
        format: VideoFormatType,
        dst: &mut [u8],
        dst_stride: u32,
        dst_width: i32,
        dst_height: i32,
    ) {
        self.to_i420().to_argb(format, dst, dst_stride, dst_width, dst_height)
    }
}

impl I420Buffer {
    pub fn new(
        width: u32,
        height: u32,
        stride_y: u32,
        stride_u: u32,
        stride_v: u32,
    ) -> vf::I420Buffer {
        vf::I420Buffer {
            handle: I420Buffer {
                sys_handle: vfb_sys::ffi::new_i420_buffer(
                    width.try_into().unwrap(),
                    height.try_into().unwrap(),
                    stride_y.try_into().unwrap(),
                    stride_u.try_into().unwrap(),
                    stride_v.try_into().unwrap(),
                ),
            },
        }
    }

    pub fn sys_handle(&self) -> &vfb_sys::ffi::VideoFrameBuffer {
        unsafe { &*recursive_cast!(&*self.sys_handle, i420_to_yuv8, yuv8_to_yuv, yuv_to_vfb) }
    }

    pub fn width(&self) -> u32 {
        unsafe {
            let ptr = recursive_cast!(&*self.sys_handle, i420_to_yuv8, yuv8_to_yuv, yuv_to_vfb);
            (*ptr).width()
        }
    }

    pub fn height(&self) -> u32 {
        unsafe {
            let ptr = recursive_cast!(&*self.sys_handle, i420_to_yuv8, yuv8_to_yuv, yuv_to_vfb);
            (*ptr).height()
        }
    }

    pub fn chroma_width(&self) -> u32 {
        unsafe {
            let ptr = recursive_cast!(&*self.sys_handle, i420_to_yuv8, yuv8_to_yuv);
            (*ptr).chroma_width()
        }
    }

    pub fn chroma_height(&self) -> u32 {
        unsafe {
            let ptr = recursive_cast!(&*self.sys_handle, i420_to_yuv8, yuv8_to_yuv);
            (*ptr).chroma_height()
        }
    }

    pub fn stride_y(&self) -> u32 {
        unsafe {
            let ptr = recursive_cast!(&*self.sys_handle, i420_to_yuv8, yuv8_to_yuv);
            (*ptr).stride_y()
        }
    }

    pub fn stride_u(&self) -> u32 {
        unsafe {
            let ptr = recursive_cast!(&*self.sys_handle, i420_to_yuv8, yuv8_to_yuv);
            (*ptr).stride_u()
        }
    }

    pub fn stride_v(&self) -> u32 {
        unsafe {
            let ptr = recursive_cast!(&*self.sys_handle, i420_to_yuv8, yuv8_to_yuv);
            (*ptr).stride_v()
        }
    }

    pub fn to_i420(&self) -> I420Buffer {
        I420Buffer {
            sys_handle: unsafe {
                // We make a copy of the buffer because internally, when calling ToI420()
                // if the buffer is of type I420, libwebrtc will reuse the same underlying pointer
                // for the new created type
                let copy = vfb_sys::ffi::copy_i420_buffer(&self.sys_handle);
                let ptr = recursive_cast!(&*copy, i420_to_yuv8, yuv8_to_yuv, yuv_to_vfb);
                (*ptr).to_i420()
            },
        }
    }

    pub fn to_argb(
        &self,
        format: VideoFormatType,
        dst: &mut [u8],
        dst_stride: u32,
        dst_width: i32,
        dst_height: i32,
    ) {
        impl_to_argb!(
            I420Buffer
            [
                ARGB: i420_to_argb,
                BGRA: i420_to_bgra,
                ABGR: i420_to_abgr,
                RGBA: i420_to_rgba
            ],
            format, self, dst, dst_stride, dst_width, dst_height
        )
    }

    pub fn data(&self) -> (&[u8], &[u8], &[u8]) {
        unsafe {
            let ptr = recursive_cast!(&*self.sys_handle, i420_to_yuv8);
            let chroma_height = (self.height() + 1) / 2;
            (
                slice::from_raw_parts((*ptr).data_y(), (self.stride_y() * self.height()) as usize),
                slice::from_raw_parts((*ptr).data_u(), (self.stride_u() * chroma_height) as usize),
                slice::from_raw_parts((*ptr).data_v(), (self.stride_v() * chroma_height) as usize),
            )
        }
    }

    pub fn scale(&mut self, scaled_width: i32, scaled_height: i32) -> vf::I420Buffer {
        vf::I420Buffer {
            handle: I420Buffer {
                sys_handle: self.sys_handle.pin_mut().scale(scaled_width, scaled_height),
            },
        }
    }
}

impl I420ABuffer {
    pub fn sys_handle(&self) -> &vfb_sys::ffi::VideoFrameBuffer {
        unsafe { &*recursive_cast!(&*self.sys_handle, i420a_to_yuv8, yuv8_to_yuv, yuv_to_vfb) }
    }

    pub fn width(&self) -> u32 {
        unsafe {
            let ptr = recursive_cast!(&*self.sys_handle, i420a_to_yuv8, yuv8_to_yuv, yuv_to_vfb);
            (*ptr).width()
        }
    }

    pub fn height(&self) -> u32 {
        unsafe {
            let ptr = recursive_cast!(&*self.sys_handle, i420a_to_yuv8, yuv8_to_yuv, yuv_to_vfb);
            (*ptr).height()
        }
    }

    pub fn chroma_width(&self) -> u32 {
        unsafe {
            let ptr = recursive_cast!(&*self.sys_handle, i420a_to_yuv8, yuv8_to_yuv);
            (*ptr).chroma_width()
        }
    }

    pub fn chroma_height(&self) -> u32 {
        unsafe {
            let ptr = recursive_cast!(&*self.sys_handle, i420a_to_yuv8, yuv8_to_yuv);
            (*ptr).chroma_height()
        }
    }

    pub fn stride_y(&self) -> u32 {
        unsafe {
            let ptr = recursive_cast!(&*self.sys_handle, i420a_to_yuv8, yuv8_to_yuv);
            (*ptr).stride_y()
        }
    }

    pub fn stride_u(&self) -> u32 {
        unsafe {
            let ptr = recursive_cast!(&*self.sys_handle, i420a_to_yuv8, yuv8_to_yuv);
            (*ptr).stride_u()
        }
    }

    pub fn stride_v(&self) -> u32 {
        unsafe {
            let ptr = recursive_cast!(&*self.sys_handle, i420a_to_yuv8, yuv8_to_yuv);
            (*ptr).stride_v()
        }
    }

    pub fn stride_a(&self) -> u32 {
        self.sys_handle.stride_a()
    }

    pub fn to_i420(&self) -> I420Buffer {
        I420Buffer {
            sys_handle: unsafe {
                let ptr =
                    recursive_cast!(&*self.sys_handle, i420a_to_yuv8, yuv8_to_yuv, yuv_to_vfb);
                (*ptr).to_i420()
            },
        }
    }

    pub fn to_argb(
        &self,
        format: VideoFormatType,
        dst: &mut [u8],
        dst_stride: u32,
        dst_width: i32,
        dst_height: i32,
    ) {
        self.to_i420().to_argb(format, dst, dst_stride, dst_width, dst_height)
    }

    pub fn data(&self) -> (&[u8], &[u8], &[u8], Option<&[u8]>) {
        unsafe {
            let ptr = recursive_cast!(&*self.sys_handle, i420a_to_yuv8);
            let chroma_height = (self.height() + 1) / 2;
            let data_a = self.sys_handle.data_a();
            let has_data_a = !data_a.is_null();
            (
                slice::from_raw_parts((*ptr).data_y(), (self.stride_y() * self.height()) as usize),
                slice::from_raw_parts((*ptr).data_u(), (self.stride_u() * chroma_height) as usize),
                slice::from_raw_parts((*ptr).data_v(), (self.stride_v() * chroma_height) as usize),
                has_data_a.then_some(slice::from_raw_parts(
                    data_a,
                    (self.stride_a() * self.height()) as usize,
                )),
            )
        }
    }

    pub fn scale(&mut self, scaled_width: i32, scaled_height: i32) -> vf::I420ABuffer {
        vf::I420ABuffer {
            handle: I420ABuffer {
                sys_handle: self.sys_handle.pin_mut().scale(scaled_width, scaled_height),
            },
        }
    }
}

impl I422Buffer {
    pub fn new(
        width: u32,
        height: u32,
        stride_y: u32,
        stride_u: u32,
        stride_v: u32,
    ) -> vf::I422Buffer {
        vf::I422Buffer {
            handle: I422Buffer {
                sys_handle: vfb_sys::ffi::new_i422_buffer(
                    width.try_into().unwrap(),
                    height.try_into().unwrap(),
                    stride_y.try_into().unwrap(),
                    stride_u.try_into().unwrap(),
                    stride_v.try_into().unwrap(),
                ),
            },
        }
    }

    pub fn sys_handle(&self) -> &vfb_sys::ffi::VideoFrameBuffer {
        unsafe { &*recursive_cast!(&*self.sys_handle, i422_to_yuv8, yuv8_to_yuv, yuv_to_vfb) }
    }

    pub fn width(&self) -> u32 {
        unsafe {
            let ptr = recursive_cast!(&*self.sys_handle, i422_to_yuv8, yuv8_to_yuv, yuv_to_vfb);
            (*ptr).width()
        }
    }

    pub fn height(&self) -> u32 {
        unsafe {
            let ptr = recursive_cast!(&*self.sys_handle, i422_to_yuv8, yuv8_to_yuv, yuv_to_vfb);
            (*ptr).height()
        }
    }

    pub fn chroma_width(&self) -> u32 {
        unsafe {
            let ptr = recursive_cast!(&*self.sys_handle, i422_to_yuv8, yuv8_to_yuv);
            (*ptr).chroma_width()
        }
    }

    pub fn chroma_height(&self) -> u32 {
        unsafe {
            let ptr = recursive_cast!(&*self.sys_handle, i422_to_yuv8, yuv8_to_yuv);
            (*ptr).chroma_height()
        }
    }

    pub fn stride_y(&self) -> u32 {
        unsafe {
            let ptr = recursive_cast!(&*self.sys_handle, i422_to_yuv8, yuv8_to_yuv);
            (*ptr).stride_y()
        }
    }

    pub fn stride_u(&self) -> u32 {
        unsafe {
            let ptr = recursive_cast!(&*self.sys_handle, i422_to_yuv8, yuv8_to_yuv);
            (*ptr).stride_u()
        }
    }

    pub fn stride_v(&self) -> u32 {
        unsafe {
            let ptr = recursive_cast!(&*self.sys_handle, i422_to_yuv8, yuv8_to_yuv);
            (*ptr).stride_v()
        }
    }

    pub fn to_i420(&self) -> I420Buffer {
        I420Buffer {
            sys_handle: unsafe {
                let ptr = recursive_cast!(&*self.sys_handle, i422_to_yuv8, yuv8_to_yuv, yuv_to_vfb);
                (*ptr).to_i420()
            },
        }
    }

    pub fn to_argb(
        &self,
        format: VideoFormatType,
        dst: &mut [u8],
        dst_stride: u32,
        dst_width: i32,
        dst_height: i32,
    ) {
        self.to_i420().to_argb(format, dst, dst_stride, dst_width, dst_height)
    }

    pub fn data(&self) -> (&[u8], &[u8], &[u8]) {
        unsafe {
            let ptr = recursive_cast!(&*self.sys_handle, i422_to_yuv8);
            (
                slice::from_raw_parts((*ptr).data_y(), (self.stride_y() * self.height()) as usize),
                slice::from_raw_parts((*ptr).data_u(), (self.stride_u() * self.height()) as usize),
                slice::from_raw_parts((*ptr).data_v(), (self.stride_v() * self.height()) as usize),
            )
        }
    }

    pub fn scale(&mut self, scaled_width: i32, scaled_height: i32) -> vf::I422Buffer {
        vf::I422Buffer {
            handle: I422Buffer {
                sys_handle: self.sys_handle.pin_mut().scale(scaled_width, scaled_height),
            },
        }
    }
}
impl I444Buffer {
    pub fn new(
        width: u32,
        height: u32,
        stride_y: u32,
        stride_u: u32,
        stride_v: u32,
    ) -> vf::I444Buffer {
        vf::I444Buffer {
            handle: I444Buffer {
                sys_handle: vfb_sys::ffi::new_i444_buffer(
                    width.try_into().unwrap(),
                    height.try_into().unwrap(),
                    stride_y.try_into().unwrap(),
                    stride_u.try_into().unwrap(),
                    stride_v.try_into().unwrap(),
                ),
            },
        }
    }

    pub fn sys_handle(&self) -> &vfb_sys::ffi::VideoFrameBuffer {
        unsafe { &*recursive_cast!(&*self.sys_handle, i444_to_yuv8, yuv8_to_yuv, yuv_to_vfb) }
    }

    pub fn width(&self) -> u32 {
        unsafe {
            let ptr = recursive_cast!(&*self.sys_handle, i444_to_yuv8, yuv8_to_yuv, yuv_to_vfb);
            (*ptr).width()
        }
    }

    pub fn height(&self) -> u32 {
        unsafe {
            let ptr = recursive_cast!(&*self.sys_handle, i444_to_yuv8, yuv8_to_yuv, yuv_to_vfb);
            (*ptr).height()
        }
    }

    pub fn chroma_width(&self) -> u32 {
        unsafe {
            let ptr = recursive_cast!(&*self.sys_handle, i444_to_yuv8, yuv8_to_yuv);
            (*ptr).chroma_width()
        }
    }

    pub fn chroma_height(&self) -> u32 {
        unsafe {
            let ptr = recursive_cast!(&*self.sys_handle, i444_to_yuv8, yuv8_to_yuv);
            (*ptr).chroma_height()
        }
    }

    pub fn stride_y(&self) -> u32 {
        unsafe {
            let ptr = recursive_cast!(&*self.sys_handle, i444_to_yuv8, yuv8_to_yuv);
            (*ptr).stride_y()
        }
    }

    pub fn stride_u(&self) -> u32 {
        unsafe {
            let ptr = recursive_cast!(&*self.sys_handle, i444_to_yuv8, yuv8_to_yuv);
            (*ptr).stride_u()
        }
    }

    pub fn stride_v(&self) -> u32 {
        unsafe {
            let ptr = recursive_cast!(&*self.sys_handle, i444_to_yuv8, yuv8_to_yuv);
            (*ptr).stride_v()
        }
    }

    pub fn to_i420(&self) -> I420Buffer {
        I420Buffer {
            sys_handle: unsafe {
                let ptr = recursive_cast!(&*self.sys_handle, i444_to_yuv8, yuv8_to_yuv, yuv_to_vfb);
                (*ptr).to_i420()
            },
        }
    }

    pub fn to_argb(
        &self,
        format: VideoFormatType,
        dst: &mut [u8],
        dst_stride: u32,
        dst_width: i32,
        dst_height: i32,
    ) {
        self.to_i420().to_argb(format, dst, dst_stride, dst_width, dst_height)
    }

    pub fn data(&self) -> (&[u8], &[u8], &[u8]) {
        unsafe {
            let ptr = recursive_cast!(&*self.sys_handle, i444_to_yuv8);
            (
                slice::from_raw_parts((*ptr).data_y(), (self.stride_y() * self.height()) as usize),
                slice::from_raw_parts((*ptr).data_u(), (self.stride_u() * self.height()) as usize),
                slice::from_raw_parts((*ptr).data_v(), (self.stride_v() * self.height()) as usize),
            )
        }
    }

    pub fn scale(&mut self, scaled_width: i32, scaled_height: i32) -> vf::I444Buffer {
        vf::I444Buffer {
            handle: I444Buffer {
                sys_handle: self.sys_handle.pin_mut().scale(scaled_width, scaled_height),
            },
        }
    }
}

impl I010Buffer {
    pub fn new(
        width: u32,
        height: u32,
        stride_y: u32,
        stride_u: u32,
        stride_v: u32,
    ) -> vf::I010Buffer {
        vf::I010Buffer {
            handle: I010Buffer {
                sys_handle: vfb_sys::ffi::new_i010_buffer(
                    width.try_into().unwrap(),
                    height.try_into().unwrap(),
                    stride_y.try_into().unwrap(),
                    stride_u.try_into().unwrap(),
                    stride_v.try_into().unwrap(),
                ),
            },
        }
    }

    pub fn sys_handle(&self) -> &vfb_sys::ffi::VideoFrameBuffer {
        unsafe { &*recursive_cast!(&*self.sys_handle, i010_to_yuv16b, yuv16b_to_yuv, yuv_to_vfb) }
    }

    pub fn width(&self) -> u32 {
        unsafe {
            let ptr = recursive_cast!(&*self.sys_handle, i010_to_yuv16b, yuv16b_to_yuv, yuv_to_vfb);
            (*ptr).width()
        }
    }

    pub fn height(&self) -> u32 {
        unsafe {
            let ptr = recursive_cast!(&*self.sys_handle, i010_to_yuv16b, yuv16b_to_yuv, yuv_to_vfb);
            (*ptr).height()
        }
    }

    pub fn chroma_width(&self) -> u32 {
        unsafe {
            let ptr = recursive_cast!(&*self.sys_handle, i010_to_yuv16b, yuv16b_to_yuv);
            (*ptr).chroma_width()
        }
    }

    pub fn chroma_height(&self) -> u32 {
        unsafe {
            let ptr = recursive_cast!(&*self.sys_handle, i010_to_yuv16b, yuv16b_to_yuv);
            (*ptr).chroma_height()
        }
    }

    pub fn stride_y(&self) -> u32 {
        unsafe {
            let ptr = recursive_cast!(&*self.sys_handle, i010_to_yuv16b, yuv16b_to_yuv);
            (*ptr).stride_y()
        }
    }

    pub fn stride_u(&self) -> u32 {
        unsafe {
            let ptr = recursive_cast!(&*self.sys_handle, i010_to_yuv16b, yuv16b_to_yuv);
            (*ptr).stride_u()
        }
    }

    pub fn stride_v(&self) -> u32 {
        unsafe {
            let ptr = recursive_cast!(&*self.sys_handle, i010_to_yuv16b, yuv16b_to_yuv);
            (*ptr).stride_v()
        }
    }

    pub fn to_i420(&self) -> I420Buffer {
        I420Buffer {
            sys_handle: unsafe {
                let ptr =
                    recursive_cast!(&*self.sys_handle, i010_to_yuv16b, yuv16b_to_yuv, yuv_to_vfb);
                (*ptr).to_i420()
            },
        }
    }

    pub fn to_argb(
        &self,
        format: VideoFormatType,
        dst: &mut [u8],
        dst_stride: u32,
        dst_width: i32,
        dst_height: i32,
    ) {
        self.to_i420().to_argb(format, dst, dst_stride, dst_width, dst_height)
    }

    pub fn data(&self) -> (&[u16], &[u16], &[u16]) {
        unsafe {
            let ptr = recursive_cast!(&*self.sys_handle, i010_to_yuv16b);
            let chroma_height = (self.height() + 1) / 2;
            (
                slice::from_raw_parts(
                    (*ptr).data_y(),
                    (self.stride_y() * self.height()) as usize / 2,
                ),
                slice::from_raw_parts(
                    (*ptr).data_u(),
                    (self.stride_u() * chroma_height) as usize / 2,
                ),
                slice::from_raw_parts(
                    (*ptr).data_v(),
                    (self.stride_v() * chroma_height) as usize / 2,
                ),
            )
        }
    }

    pub fn scale(&mut self, scaled_width: i32, scaled_height: i32) -> vf::I010Buffer {
        vf::I010Buffer {
            handle: I010Buffer {
                sys_handle: self.sys_handle.pin_mut().scale(scaled_width, scaled_height),
            },
        }
    }
}

impl NV12Buffer {
    pub fn new(width: u32, height: u32, stride_y: u32, stride_uv: u32) -> vf::NV12Buffer {
        vf::NV12Buffer {
            handle: NV12Buffer {
                sys_handle: vfb_sys::ffi::new_nv12_buffer(
                    width.try_into().unwrap(),
                    height.try_into().unwrap(),
                    stride_y.try_into().unwrap(),
                    stride_uv.try_into().unwrap(),
                ),
            },
        }
    }

    pub fn sys_handle(&self) -> &vfb_sys::ffi::VideoFrameBuffer {
        unsafe {
            &*recursive_cast!(&*self.sys_handle, nv12_to_biyuv8, biyuv8_to_biyuv, biyuv_to_vfb)
        }
    }

    pub fn width(&self) -> u32 {
        unsafe {
            let ptr =
                recursive_cast!(&*self.sys_handle, nv12_to_biyuv8, biyuv8_to_biyuv, biyuv_to_vfb);
            (*ptr).width()
        }
    }

    pub fn height(&self) -> u32 {
        unsafe {
            let ptr =
                recursive_cast!(&*self.sys_handle, nv12_to_biyuv8, biyuv8_to_biyuv, biyuv_to_vfb);
            (*ptr).height()
        }
    }

    pub fn chroma_width(&self) -> u32 {
        unsafe {
            let ptr = recursive_cast!(&*self.sys_handle, nv12_to_biyuv8, biyuv8_to_biyuv);
            (*ptr).chroma_width()
        }
    }

    pub fn chroma_height(&self) -> u32 {
        unsafe {
            let ptr = recursive_cast!(&*self.sys_handle, nv12_to_biyuv8, biyuv8_to_biyuv);
            (*ptr).chroma_height()
        }
    }

    pub fn stride_y(&self) -> u32 {
        unsafe {
            let ptr = recursive_cast!(&*self.sys_handle, nv12_to_biyuv8, biyuv8_to_biyuv);
            (*ptr).stride_y()
        }
    }

    pub fn stride_uv(&self) -> u32 {
        unsafe {
            let ptr = recursive_cast!(&*self.sys_handle, nv12_to_biyuv8, biyuv8_to_biyuv);
            (*ptr).stride_uv()
        }
    }

    pub fn to_i420(&self) -> I420Buffer {
        I420Buffer {
            sys_handle: unsafe {
                let ptr = recursive_cast!(
                    &*self.sys_handle,
                    nv12_to_biyuv8,
                    biyuv8_to_biyuv,
                    biyuv_to_vfb
                );
                (*ptr).to_i420()
            },
        }
    }

    pub fn to_argb(
        &self,
        format: VideoFormatType,
        dst: &mut [u8],
        dst_stride: u32,
        dst_width: i32,
        dst_height: i32,
    ) {
        self.to_i420().to_argb(format, dst, dst_stride, dst_width, dst_height)
    }

    pub fn data(&self) -> (&[u8], &[u8]) {
        unsafe {
            let ptr = recursive_cast!(&*self.sys_handle, nv12_to_biyuv8);
            let chroma_height = (self.height() + 1) / 2;

            (
                slice::from_raw_parts((*ptr).data_y(), (self.stride_y() * self.height()) as usize),
                slice::from_raw_parts(
                    (*ptr).data_uv(),
                    (self.stride_uv() * chroma_height) as usize,
                ),
            )
        }
    }

    pub fn scale(&mut self, scaled_width: i32, scaled_height: i32) -> vf::NV12Buffer {
        vf::NV12Buffer {
            handle: NV12Buffer {
                sys_handle: self.sys_handle.pin_mut().scale(scaled_width, scaled_height),
            },
        }
    }
}
