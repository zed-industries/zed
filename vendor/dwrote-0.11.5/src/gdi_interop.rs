/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::cell::UnsafeCell;
use std::ptr;
use winapi::um::dwrite::IDWriteBitmapRenderTarget;
use winapi::um::dwrite::IDWriteGdiInterop;
use wio::com::ComPtr;

use super::{BitmapRenderTarget, DWriteFactory};

pub struct GdiInterop {
    native: UnsafeCell<ComPtr<IDWriteGdiInterop>>,
}

impl GdiInterop {
    pub fn create() -> GdiInterop {
        unsafe {
            let mut native: *mut IDWriteGdiInterop = ptr::null_mut();
            let hr = (*DWriteFactory()).GetGdiInterop(&mut native);
            assert!(hr == 0);
            GdiInterop::take(ComPtr::from_raw(native))
        }
    }

    pub fn take(native: ComPtr<IDWriteGdiInterop>) -> GdiInterop {
        GdiInterop {
            native: UnsafeCell::new(native),
        }
    }

    pub fn create_bitmap_render_target(&self, width: u32, height: u32) -> BitmapRenderTarget {
        unsafe {
            let mut native: *mut IDWriteBitmapRenderTarget = ptr::null_mut();
            let hr = (*self.native.get()).CreateBitmapRenderTarget(
                ptr::null_mut(),
                width,
                height,
                &mut native,
            );
            assert!(hr == 0);
            BitmapRenderTarget::take(ComPtr::from_raw(native))
        }
    }
}
