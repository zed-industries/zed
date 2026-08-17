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

#![allow(clippy::too_many_arguments)]

use webrtc_sys::yuv_helper as yuv_sys;

fn argb_assert_safety(src: &[u8], src_stride: u32, _width: i32, height: i32) {
    let height_abs = height.unsigned_abs();
    let min = (src_stride * height_abs) as usize;
    assert!(src.len() >= min, "src isn't large enough");
}

fn i420_assert_safety(
    src_y: &[u8],
    src_stride_y: u32,
    src_u: &[u8],
    src_stride_u: u32,
    src_v: &[u8],
    src_stride_v: u32,
    _width: i32,
    height: i32,
) {
    let height_abs = height.unsigned_abs();
    let chroma_height = (height_abs + 1) / 2;
    let min_y = (src_stride_y * height_abs) as usize;
    let min_u = (src_stride_u * chroma_height) as usize;
    let min_v = (src_stride_v * chroma_height) as usize;

    assert!(src_y.len() >= min_y, "src_y isn't large enough");
    assert!(src_u.len() >= min_u, "src_u isn't large enough");
    assert!(src_v.len() >= min_v, "src_v isn't large enough");
}

fn nv12_assert_safety(
    src_y: &[u8],
    src_stride_y: u32,
    src_uv: &[u8],
    src_stride_uv: u32,
    _width: i32,
    height: i32,
) {
    let height_abs = height.unsigned_abs();
    let chroma_height = (height_abs + 1) / 2;

    let min_y = (src_stride_y * height_abs) as usize;
    let min_uv = (src_stride_uv * chroma_height) as usize;

    assert!(src_y.len() >= min_y, "src_y isn't large enough");
    assert!(src_uv.len() >= min_uv, "src_uv isn't large enough");
}

fn i444_assert_safety(
    src_y: &[u8],
    src_stride_y: u32,
    src_u: &[u8],
    src_stride_u: u32,
    src_v: &[u8],
    src_stride_v: u32,
    _width: i32,
    height: i32,
) {
    let height_abs = height.unsigned_abs();
    let min_y = (src_stride_y * height_abs) as usize;
    let min_u = (src_stride_u * height_abs) as usize;
    let min_v = (src_stride_v * height_abs) as usize;

    assert!(src_y.len() >= min_y, "src_y isn't large enough");
    assert!(src_u.len() >= min_u, "src_u isn't large enough");
    assert!(src_v.len() >= min_v, "src_v isn't large enough");
}

fn i422_assert_safety(
    src_y: &[u8],
    src_stride_y: u32,
    src_u: &[u8],
    src_stride_u: u32,
    src_v: &[u8],
    src_stride_v: u32,
    _width: i32,
    height: i32,
) {
    let height_abs = height.unsigned_abs();
    let min_y = (src_stride_y * height_abs) as usize;
    let min_u = (src_stride_u * height_abs) as usize;
    let min_v = (src_stride_v * height_abs) as usize;

    assert!(src_y.len() >= min_y, "src_y isn't large enough");
    assert!(src_u.len() >= min_u, "src_u isn't large enough");
    assert!(src_v.len() >= min_v, "src_v isn't large enough");
}

fn i010_assert_safety(
    src_y: &[u16],
    src_stride_y: u32,
    src_u: &[u16],
    src_stride_u: u32,
    src_v: &[u16],
    src_stride_v: u32,
    _width: i32,
    height: i32,
) {
    let height_abs: u32 = height.unsigned_abs();
    let chroma_height = height_abs / 2;
    let min_y = (src_stride_y * height_abs) as usize / 2;
    let min_u = (src_stride_u * chroma_height) as usize / 2;
    let min_v = (src_stride_v * chroma_height) as usize / 2;

    assert!(src_y.len() >= min_y, "src_y isn't large enough");
    assert!(src_u.len() >= min_u, "src_u isn't large enough");
    assert!(src_v.len() >= min_v, "src_v isn't large enough");
}

macro_rules! i420_to_rgba {
    ($x:ident) => {
        pub fn $x(
            src_y: &[u8],
            src_stride_y: u32,
            src_u: &[u8],
            src_stride_u: u32,
            src_v: &[u8],
            src_stride_v: u32,
            dst: &mut [u8],
            dst_stride: u32,
            width: i32,
            height: i32,
        ) {
            i420_assert_safety(
                src_y,
                src_stride_y,
                src_u,
                src_stride_u,
                src_v,
                src_stride_v,
                width,
                height,
            );
            argb_assert_safety(dst, dst_stride, width, height);

            unsafe {
                yuv_sys::ffi::$x(
                    src_y.as_ptr(),
                    src_stride_y as i32,
                    src_u.as_ptr(),
                    src_stride_u as i32,
                    src_v.as_ptr(),
                    src_stride_v as i32,
                    dst.as_mut_ptr(),
                    dst_stride as i32,
                    width,
                    height,
                )
                .unwrap();
            }
        }
    };
}

macro_rules! rgba_to_i420 {
    ($x:ident) => {
        pub fn $x(
            src_argb: &[u8],
            src_stride_argb: u32,
            dst_y: &mut [u8],
            dst_stride_y: u32,
            dst_u: &mut [u8],
            dst_stride_u: u32,
            dst_v: &mut [u8],
            dst_stride_v: u32,
            width: i32,
            height: i32,
        ) {
            i420_assert_safety(
                dst_y,
                dst_stride_y,
                dst_u,
                dst_stride_u,
                dst_v,
                dst_stride_v,
                width,
                height,
            );
            argb_assert_safety(src_argb, src_stride_argb, width, height);

            unsafe {
                yuv_sys::ffi::$x(
                    src_argb.as_ptr(),
                    src_stride_argb as i32,
                    dst_y.as_mut_ptr(),
                    dst_stride_y as i32,
                    dst_u.as_mut_ptr(),
                    dst_stride_u as i32,
                    dst_v.as_mut_ptr(),
                    dst_stride_v as i32,
                    width,
                    height,
                )
                .unwrap();
            }
        }
    };
}

pub fn argb_to_rgb24(
    src_argb: &[u8],
    src_stride_argb: u32,
    dst_rgb24: &mut [u8],
    dst_stride_rgb24: u32,
    width: i32,
    height: i32,
) {
    argb_assert_safety(src_argb, src_stride_argb, width, height);
    argb_assert_safety(dst_rgb24, dst_stride_rgb24, width, height);

    unsafe {
        yuv_sys::ffi::argb_to_rgb24(
            src_argb.as_ptr(),
            src_stride_argb as i32,
            dst_rgb24.as_mut_ptr(),
            dst_stride_rgb24 as i32,
            width,
            height,
        )
        .unwrap();
    }
}

// I420 <> RGB conversion
rgba_to_i420!(argb_to_i420);
rgba_to_i420!(abgr_to_i420);

i420_to_rgba!(i420_to_argb);
i420_to_rgba!(i420_to_bgra);
i420_to_rgba!(i420_to_abgr);
i420_to_rgba!(i420_to_rgba);

pub fn i420_to_nv12(
    src_y: &[u8],
    src_stride_y: u32,
    src_u: &[u8],
    src_stride_u: u32,
    src_v: &[u8],
    src_stride_v: u32,
    dst_y: &mut [u8],
    dst_stride_y: u32,
    dst_uv: &mut [u8],
    dst_stride_uv: u32,
    width: i32,
    height: i32,
) {
    i420_assert_safety(
        src_y,
        src_stride_y,
        src_u,
        src_stride_u,
        src_v,
        src_stride_v,
        width,
        height,
    );
    nv12_assert_safety(dst_y, dst_stride_y, dst_uv, dst_stride_uv, width, height);

    unsafe {
        yuv_sys::ffi::i420_to_nv12(
            src_y.as_ptr(),
            src_stride_y as i32,
            src_u.as_ptr(),
            src_stride_u as i32,
            src_v.as_ptr(),
            src_stride_v as i32,
            dst_y.as_mut_ptr(),
            dst_stride_y as i32,
            dst_uv.as_mut_ptr(),
            dst_stride_uv as i32,
            width,
            height,
        )
        .unwrap();
    }
}

pub fn nv12_to_i420(
    src_y: &[u8],
    src_stride_y: u32,
    src_uv: &[u8],
    src_stride_uv: u32,
    dst_y: &mut [u8],
    dst_stride_y: u32,
    dst_u: &mut [u8],
    dst_stride_u: u32,
    dst_v: &mut [u8],
    dst_stride_v: u32,
    width: i32,
    height: i32,
) {
    nv12_assert_safety(src_y, src_stride_y, src_uv, src_stride_uv, width, height);
    i420_assert_safety(
        dst_y,
        dst_stride_y,
        dst_u,
        dst_stride_u,
        dst_v,
        dst_stride_v,
        width,
        height,
    );

    unsafe {
        yuv_sys::ffi::nv12_to_i420(
            src_y.as_ptr(),
            src_stride_y as i32,
            src_uv.as_ptr(),
            src_stride_uv as i32,
            dst_y.as_mut_ptr(),
            dst_stride_y as i32,
            dst_u.as_mut_ptr(),
            dst_stride_u as i32,
            dst_v.as_mut_ptr(),
            dst_stride_v as i32,
            width,
            height,
        )
        .unwrap();
    }
}

pub fn i444_to_i420(
    src_y: &[u8],
    src_stride_y: u32,
    src_u: &[u8],
    src_stride_u: u32,
    src_v: &[u8],
    src_stride_v: u32,
    dst_y: &mut [u8],
    dst_stride_y: u32,
    dst_u: &mut [u8],
    dst_stride_u: u32,
    dst_v: &mut [u8],
    dst_stride_v: u32,
    width: i32,
    height: i32,
) {
    i444_assert_safety(
        src_y,
        src_stride_y,
        src_u,
        src_stride_u,
        src_v,
        src_stride_v,
        width,
        height,
    );
    i420_assert_safety(
        dst_y,
        dst_stride_y,
        dst_u,
        dst_stride_u,
        dst_v,
        dst_stride_v,
        width,
        height,
    );

    unsafe {
        yuv_sys::ffi::i444_to_i420(
            src_y.as_ptr(),
            src_stride_y as i32,
            src_u.as_ptr(),
            src_stride_u as i32,
            src_v.as_ptr(),
            src_stride_v as i32,
            dst_y.as_mut_ptr(),
            dst_stride_y as i32,
            dst_u.as_mut_ptr(),
            dst_stride_u as i32,
            dst_v.as_mut_ptr(),
            dst_stride_v as i32,
            width,
            height,
        )
        .unwrap();
    }
}

pub fn i422_to_i420(
    src_y: &[u8],
    src_stride_y: u32,
    src_u: &[u8],
    src_stride_u: u32,
    src_v: &[u8],
    src_stride_v: u32,
    dst_y: &mut [u8],
    dst_stride_y: u32,
    dst_u: &mut [u8],
    dst_stride_u: u32,
    dst_v: &mut [u8],
    dst_stride_v: u32,
    width: i32,
    height: i32,
) {
    i422_assert_safety(
        src_y,
        src_stride_y,
        src_u,
        src_stride_u,
        src_v,
        src_stride_v,
        width,
        height,
    );
    i420_assert_safety(
        dst_y,
        dst_stride_y,
        dst_u,
        dst_stride_u,
        dst_v,
        dst_stride_v,
        width,
        height,
    );

    unsafe {
        yuv_sys::ffi::i422_to_i420(
            src_y.as_ptr(),
            src_stride_y as i32,
            src_u.as_ptr(),
            src_stride_u as i32,
            src_v.as_ptr(),
            src_stride_v as i32,
            dst_y.as_mut_ptr(),
            dst_stride_y as i32,
            dst_u.as_mut_ptr(),
            dst_stride_u as i32,
            dst_v.as_mut_ptr(),
            dst_stride_v as i32,
            width,
            height,
        )
        .unwrap()
    }
}

pub fn i010_to_i420(
    src_y: &[u16],
    src_stride_y: u32,
    src_u: &[u16],
    src_stride_u: u32,
    src_v: &[u16],
    src_stride_v: u32,
    dst_y: &mut [u8],
    dst_stride_y: u32,
    dst_u: &mut [u8],
    dst_stride_u: u32,
    dst_v: &mut [u8],
    dst_stride_v: u32,
    width: i32,
    height: i32,
) {
    i010_assert_safety(
        src_y,
        src_stride_y,
        src_u,
        src_stride_u,
        src_v,
        src_stride_v,
        width,
        height,
    );
    i420_assert_safety(
        dst_y,
        dst_stride_y,
        dst_u,
        dst_stride_u,
        dst_v,
        dst_stride_v,
        width,
        height,
    );

    unsafe {
        yuv_sys::ffi::i010_to_i420(
            src_y.as_ptr(),
            src_stride_y as i32,
            src_u.as_ptr(),
            src_stride_u as i32,
            src_v.as_ptr(),
            src_stride_v as i32,
            dst_y.as_mut_ptr(),
            dst_stride_y as i32,
            dst_u.as_mut_ptr(),
            dst_stride_u as i32,
            dst_v.as_mut_ptr(),
            dst_stride_v as i32,
            width,
            height,
        )
        .unwrap()
    }
}

pub fn nv12_to_argb(
    src_y: &[u8],
    src_stride_y: u32,
    src_uv: &[u8],
    src_stride_uv: u32,
    dst_argb: &mut [u8],
    dst_stride_argb: u32,
    width: i32,
    height: i32,
) {
    nv12_assert_safety(src_y, src_stride_y, src_uv, src_stride_uv, width, height);
    argb_assert_safety(dst_argb, dst_stride_argb, width, height);

    unsafe {
        yuv_sys::ffi::nv12_to_argb(
            src_y.as_ptr(),
            src_stride_y as i32,
            src_uv.as_ptr(),
            src_stride_uv as i32,
            dst_argb.as_mut_ptr(),
            dst_stride_argb as i32,
            width,
            height,
        )
        .unwrap();
    }
}

pub fn nv12_to_abgr(
    src_y: &[u8],
    src_stride_y: u32,
    src_uv: &[u8],
    src_stride_uv: u32,
    dst_abgr: &mut [u8],
    dst_stride_abgr: u32,
    width: i32,
    height: i32,
) {
    nv12_assert_safety(src_y, src_stride_y, src_uv, src_stride_uv, width, height);
    argb_assert_safety(dst_abgr, dst_stride_abgr, width, height);

    unsafe {
        yuv_sys::ffi::nv12_to_abgr(
            src_y.as_ptr(),
            src_stride_y as i32,
            src_uv.as_ptr(),
            src_stride_uv as i32,
            dst_abgr.as_mut_ptr(),
            dst_stride_abgr as i32,
            width,
            height,
        )
        .unwrap();
    }
}

pub fn i444_to_argb(
    src_y: &[u8],
    src_stride_y: u32,
    src_u: &[u8],
    src_stride_u: u32,
    src_v: &[u8],
    src_stride_v: u32,
    dst_argb: &mut [u8],
    dst_stride_argb: u32,
    width: i32,
    height: i32,
) {
    i444_assert_safety(
        src_y,
        src_stride_y,
        src_u,
        src_stride_u,
        src_v,
        src_stride_v,
        width,
        height,
    );
    argb_assert_safety(dst_argb, dst_stride_argb, width, height);

    unsafe {
        yuv_sys::ffi::i444_to_argb(
            src_y.as_ptr(),
            src_stride_y as i32,
            src_u.as_ptr(),
            src_stride_u as i32,
            src_v.as_ptr(),
            src_stride_v as i32,
            dst_argb.as_mut_ptr(),
            dst_stride_argb as i32,
            width,
            height,
        )
        .unwrap();
    }
}

pub fn i444_to_abgr(
    src_y: &[u8],
    src_stride_y: u32,
    src_u: &[u8],
    src_stride_u: u32,
    src_v: &[u8],
    src_stride_v: u32,
    dst_abgr: &mut [u8],
    dst_stride_abgr: u32,
    width: i32,
    height: i32,
) {
    i444_assert_safety(
        src_y,
        src_stride_y,
        src_u,
        src_stride_u,
        src_v,
        src_stride_v,
        width,
        height,
    );
    argb_assert_safety(dst_abgr, dst_stride_abgr, width, height);

    unsafe {
        yuv_sys::ffi::i444_to_abgr(
            src_y.as_ptr(),
            src_stride_y as i32,
            src_u.as_ptr(),
            src_stride_u as i32,
            src_v.as_ptr(),
            src_stride_v as i32,
            dst_abgr.as_mut_ptr(),
            dst_stride_abgr as i32,
            width,
            height,
        )
        .unwrap()
    }
}

pub fn i422_to_argb(
    src_y: &[u8],
    src_stride_y: u32,
    src_u: &[u8],
    src_stride_u: u32,
    src_v: &[u8],
    src_stride_v: u32,
    dst_argb: &mut [u8],
    dst_stride_argb: u32,
    width: i32,
    height: i32,
) {
    i422_assert_safety(
        src_y,
        src_stride_y,
        src_u,
        src_stride_u,
        src_v,
        src_stride_v,
        width,
        height,
    );
    argb_assert_safety(dst_argb, dst_stride_argb, width, height);

    unsafe {
        yuv_sys::ffi::i422_to_argb(
            src_y.as_ptr(),
            src_stride_y as i32,
            src_u.as_ptr(),
            src_stride_u as i32,
            src_v.as_ptr(),
            src_stride_v as i32,
            dst_argb.as_mut_ptr(),
            dst_stride_argb as i32,
            width,
            height,
        )
        .unwrap();
    }
}

pub fn i422_to_abgr(
    src_y: &[u8],
    src_stride_y: u32,
    src_u: &[u8],
    src_stride_u: u32,
    src_v: &[u8],
    src_stride_v: u32,
    dst_abgr: &mut [u8],
    dst_stride_abgr: u32,
    width: i32,
    height: i32,
) {
    i422_assert_safety(
        src_y,
        src_stride_y,
        src_u,
        src_stride_u,
        src_v,
        src_stride_v,
        width,
        height,
    );
    argb_assert_safety(dst_abgr, dst_stride_abgr, width, height);

    unsafe {
        yuv_sys::ffi::i422_to_abgr(
            src_y.as_ptr(),
            src_stride_y as i32,
            src_u.as_ptr(),
            src_stride_u as i32,
            src_v.as_ptr(),
            src_stride_v as i32,
            dst_abgr.as_mut_ptr(),
            dst_stride_abgr as i32,
            width,
            height,
        )
        .unwrap()
    }
}

pub fn i010_to_argb(
    src_y: &[u16],
    src_stride_y: u32,
    src_u: &[u16],
    src_stride_u: u32,
    src_v: &[u16],
    src_stride_v: u32,
    dst_argb: &mut [u8],
    dst_stride_argb: u32,
    width: i32,
    height: i32,
) {
    i010_assert_safety(
        src_y,
        src_stride_y,
        src_u,
        src_stride_u,
        src_v,
        src_stride_v,
        width,
        height,
    );
    argb_assert_safety(dst_argb, dst_stride_argb, width, height);

    unsafe {
        yuv_sys::ffi::i010_to_argb(
            src_y.as_ptr(),
            src_stride_y as i32,
            src_u.as_ptr(),
            src_stride_u as i32,
            src_v.as_ptr(),
            src_stride_v as i32,
            dst_argb.as_mut_ptr(),
            dst_stride_argb as i32,
            width,
            height,
        )
        .unwrap()
    }
}

pub fn i010_to_abgr(
    src_y: &[u16],
    src_stride_y: u32,
    src_u: &[u16],
    src_stride_u: u32,
    src_v: &[u16],
    src_stride_v: u32,
    dst_abgr: &mut [u8],
    dst_stride_abgr: u32,
    width: i32,
    height: i32,
) {
    i010_assert_safety(
        src_y,
        src_stride_y,
        src_u,
        src_stride_u,
        src_v,
        src_stride_v,
        width,
        height,
    );
    argb_assert_safety(dst_abgr, dst_stride_abgr, width, height);

    unsafe {
        yuv_sys::ffi::i010_to_abgr(
            src_y.as_ptr(),
            src_stride_y as i32,
            src_u.as_ptr(),
            src_stride_u as i32,
            src_v.as_ptr(),
            src_stride_v as i32,
            dst_abgr.as_mut_ptr(),
            dst_stride_abgr as i32,
            width,
            height,
        )
        .unwrap()
    }
}

pub fn abgr_to_nv12(
    src_abgr: &[u8],
    src_stride_abgr: u32,
    dst_y: &mut [u8],
    dst_stride_y: u32,
    dst_uv: &mut [u8],
    dst_stride_uv: u32,
    width: i32,
    height: i32,
) {
    argb_assert_safety(src_abgr, src_stride_abgr, width, height);
    nv12_assert_safety(dst_y, dst_stride_y, dst_uv, dst_stride_uv, width, height);

    unsafe {
        yuv_sys::ffi::abgr_to_nv12(
            src_abgr.as_ptr(),
            src_stride_abgr as i32,
            dst_y.as_mut_ptr(),
            dst_stride_y as i32,
            dst_uv.as_mut_ptr(),
            dst_stride_uv as i32,
            width,
            height,
        )
        .unwrap()
    }
}

pub fn argb_to_nv12(
    src_argb: &[u8],
    src_stride_argb: u32,
    dst_y: &mut [u8],
    dst_stride_y: u32,
    dst_uv: &mut [u8],
    dst_stride_uv: u32,
    width: i32,
    height: i32,
) {
    argb_assert_safety(src_argb, src_stride_argb, width, height);
    nv12_assert_safety(dst_y, dst_stride_y, dst_uv, dst_stride_uv, width, height);

    unsafe {
        yuv_sys::ffi::argb_to_nv12(
            src_argb.as_ptr(),
            src_stride_argb as i32,
            dst_y.as_mut_ptr(),
            dst_stride_y as i32,
            dst_uv.as_mut_ptr(),
            dst_stride_uv as i32,
            width,
            height,
        )
        .unwrap()
    }
}
