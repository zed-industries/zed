// pathfinder/simd/src/x86/swizzle_i32x4.rs
//
// Copyright © 2019 The Pathfinder Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::x86::I32x4;

#[cfg(target_pointer_width = "32")]
use std::arch::x86;
#[cfg(target_pointer_width = "64")]
use std::arch::x86_64 as x86;

impl I32x4 {
    #[inline]
    pub fn xxxx(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 0,
            )))
        }
    }

    #[inline]
    pub fn yxxx(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 1,
            )))
        }
    }

    #[inline]
    pub fn zxxx(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 2,
            )))
        }
    }

    #[inline]
    pub fn wxxx(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 3,
            )))
        }
    }

    #[inline]
    pub fn xyxx(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 4,
            )))
        }
    }

    #[inline]
    pub fn yyxx(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 5,
            )))
        }
    }

    #[inline]
    pub fn zyxx(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 6,
            )))
        }
    }

    #[inline]
    pub fn wyxx(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 7,
            )))
        }
    }

    #[inline]
    pub fn xzxx(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 8,
            )))
        }
    }

    #[inline]
    pub fn yzxx(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 9,
            )))
        }
    }

    #[inline]
    pub fn zzxx(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 10,
            )))
        }
    }

    #[inline]
    pub fn wzxx(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 11,
            )))
        }
    }

    #[inline]
    pub fn xwxx(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 12,
            )))
        }
    }

    #[inline]
    pub fn ywxx(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 13,
            )))
        }
    }

    #[inline]
    pub fn zwxx(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 14,
            )))
        }
    }

    #[inline]
    pub fn wwxx(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 15,
            )))
        }
    }

    #[inline]
    pub fn xxyx(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 16,
            )))
        }
    }

    #[inline]
    pub fn yxyx(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 17,
            )))
        }
    }

    #[inline]
    pub fn zxyx(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 18,
            )))
        }
    }

    #[inline]
    pub fn wxyx(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 19,
            )))
        }
    }

    #[inline]
    pub fn xyyx(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 20,
            )))
        }
    }

    #[inline]
    pub fn yyyx(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 21,
            )))
        }
    }

    #[inline]
    pub fn zyyx(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 22,
            )))
        }
    }

    #[inline]
    pub fn wyyx(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 23,
            )))
        }
    }

    #[inline]
    pub fn xzyx(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 24,
            )))
        }
    }

    #[inline]
    pub fn yzyx(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 25,
            )))
        }
    }

    #[inline]
    pub fn zzyx(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 26,
            )))
        }
    }

    #[inline]
    pub fn wzyx(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 27,
            )))
        }
    }

    #[inline]
    pub fn xwyx(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 28,
            )))
        }
    }

    #[inline]
    pub fn ywyx(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 29,
            )))
        }
    }

    #[inline]
    pub fn zwyx(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 30,
            )))
        }
    }

    #[inline]
    pub fn wwyx(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 31,
            )))
        }
    }

    #[inline]
    pub fn xxzx(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 32,
            )))
        }
    }

    #[inline]
    pub fn yxzx(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 33,
            )))
        }
    }

    #[inline]
    pub fn zxzx(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 34,
            )))
        }
    }

    #[inline]
    pub fn wxzx(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 35,
            )))
        }
    }

    #[inline]
    pub fn xyzx(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 36,
            )))
        }
    }

    #[inline]
    pub fn yyzx(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 37,
            )))
        }
    }

    #[inline]
    pub fn zyzx(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 38,
            )))
        }
    }

    #[inline]
    pub fn wyzx(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 39,
            )))
        }
    }

    #[inline]
    pub fn xzzx(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 40,
            )))
        }
    }

    #[inline]
    pub fn yzzx(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 41,
            )))
        }
    }

    #[inline]
    pub fn zzzx(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 42,
            )))
        }
    }

    #[inline]
    pub fn wzzx(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 43,
            )))
        }
    }

    #[inline]
    pub fn xwzx(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 44,
            )))
        }
    }

    #[inline]
    pub fn ywzx(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 45,
            )))
        }
    }

    #[inline]
    pub fn zwzx(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 46,
            )))
        }
    }

    #[inline]
    pub fn wwzx(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 47,
            )))
        }
    }

    #[inline]
    pub fn xxwx(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 48,
            )))
        }
    }

    #[inline]
    pub fn yxwx(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 49,
            )))
        }
    }

    #[inline]
    pub fn zxwx(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 50,
            )))
        }
    }

    #[inline]
    pub fn wxwx(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 51,
            )))
        }
    }

    #[inline]
    pub fn xywx(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 52,
            )))
        }
    }

    #[inline]
    pub fn yywx(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 53,
            )))
        }
    }

    #[inline]
    pub fn zywx(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 54,
            )))
        }
    }

    #[inline]
    pub fn wywx(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 55,
            )))
        }
    }

    #[inline]
    pub fn xzwx(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 56,
            )))
        }
    }

    #[inline]
    pub fn yzwx(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 57,
            )))
        }
    }

    #[inline]
    pub fn zzwx(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 58,
            )))
        }
    }

    #[inline]
    pub fn wzwx(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 59,
            )))
        }
    }

    #[inline]
    pub fn xwwx(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 60,
            )))
        }
    }

    #[inline]
    pub fn ywwx(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 61,
            )))
        }
    }

    #[inline]
    pub fn zwwx(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 62,
            )))
        }
    }

    #[inline]
    pub fn wwwx(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 63,
            )))
        }
    }

    #[inline]
    pub fn xxxy(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 64,
            )))
        }
    }

    #[inline]
    pub fn yxxy(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 65,
            )))
        }
    }

    #[inline]
    pub fn zxxy(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 66,
            )))
        }
    }

    #[inline]
    pub fn wxxy(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 67,
            )))
        }
    }

    #[inline]
    pub fn xyxy(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 68,
            )))
        }
    }

    #[inline]
    pub fn yyxy(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 69,
            )))
        }
    }

    #[inline]
    pub fn zyxy(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 70,
            )))
        }
    }

    #[inline]
    pub fn wyxy(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 71,
            )))
        }
    }

    #[inline]
    pub fn xzxy(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 72,
            )))
        }
    }

    #[inline]
    pub fn yzxy(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 73,
            )))
        }
    }

    #[inline]
    pub fn zzxy(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 74,
            )))
        }
    }

    #[inline]
    pub fn wzxy(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 75,
            )))
        }
    }

    #[inline]
    pub fn xwxy(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 76,
            )))
        }
    }

    #[inline]
    pub fn ywxy(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 77,
            )))
        }
    }

    #[inline]
    pub fn zwxy(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 78,
            )))
        }
    }

    #[inline]
    pub fn wwxy(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 79,
            )))
        }
    }

    #[inline]
    pub fn xxyy(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 80,
            )))
        }
    }

    #[inline]
    pub fn yxyy(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 81,
            )))
        }
    }

    #[inline]
    pub fn zxyy(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 82,
            )))
        }
    }

    #[inline]
    pub fn wxyy(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 83,
            )))
        }
    }

    #[inline]
    pub fn xyyy(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 84,
            )))
        }
    }

    #[inline]
    pub fn yyyy(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 85,
            )))
        }
    }

    #[inline]
    pub fn zyyy(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 86,
            )))
        }
    }

    #[inline]
    pub fn wyyy(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 87,
            )))
        }
    }

    #[inline]
    pub fn xzyy(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 88,
            )))
        }
    }

    #[inline]
    pub fn yzyy(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 89,
            )))
        }
    }

    #[inline]
    pub fn zzyy(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 90,
            )))
        }
    }

    #[inline]
    pub fn wzyy(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 91,
            )))
        }
    }

    #[inline]
    pub fn xwyy(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 92,
            )))
        }
    }

    #[inline]
    pub fn ywyy(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 93,
            )))
        }
    }

    #[inline]
    pub fn zwyy(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 94,
            )))
        }
    }

    #[inline]
    pub fn wwyy(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 95,
            )))
        }
    }

    #[inline]
    pub fn xxzy(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 96,
            )))
        }
    }

    #[inline]
    pub fn yxzy(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 97,
            )))
        }
    }

    #[inline]
    pub fn zxzy(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 98,
            )))
        }
    }

    #[inline]
    pub fn wxzy(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 99,
            )))
        }
    }

    #[inline]
    pub fn xyzy(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 100,
            )))
        }
    }

    #[inline]
    pub fn yyzy(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 101,
            )))
        }
    }

    #[inline]
    pub fn zyzy(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 102,
            )))
        }
    }

    #[inline]
    pub fn wyzy(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 103,
            )))
        }
    }

    #[inline]
    pub fn xzzy(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 104,
            )))
        }
    }

    #[inline]
    pub fn yzzy(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 105,
            )))
        }
    }

    #[inline]
    pub fn zzzy(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 106,
            )))
        }
    }

    #[inline]
    pub fn wzzy(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 107,
            )))
        }
    }

    #[inline]
    pub fn xwzy(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 108,
            )))
        }
    }

    #[inline]
    pub fn ywzy(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 109,
            )))
        }
    }

    #[inline]
    pub fn zwzy(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 110,
            )))
        }
    }

    #[inline]
    pub fn wwzy(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 111,
            )))
        }
    }

    #[inline]
    pub fn xxwy(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 112,
            )))
        }
    }

    #[inline]
    pub fn yxwy(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 113,
            )))
        }
    }

    #[inline]
    pub fn zxwy(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 114,
            )))
        }
    }

    #[inline]
    pub fn wxwy(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 115,
            )))
        }
    }

    #[inline]
    pub fn xywy(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 116,
            )))
        }
    }

    #[inline]
    pub fn yywy(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 117,
            )))
        }
    }

    #[inline]
    pub fn zywy(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 118,
            )))
        }
    }

    #[inline]
    pub fn wywy(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 119,
            )))
        }
    }

    #[inline]
    pub fn xzwy(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 120,
            )))
        }
    }

    #[inline]
    pub fn yzwy(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 121,
            )))
        }
    }

    #[inline]
    pub fn zzwy(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 122,
            )))
        }
    }

    #[inline]
    pub fn wzwy(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 123,
            )))
        }
    }

    #[inline]
    pub fn xwwy(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 124,
            )))
        }
    }

    #[inline]
    pub fn ywwy(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 125,
            )))
        }
    }

    #[inline]
    pub fn zwwy(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 126,
            )))
        }
    }

    #[inline]
    pub fn wwwy(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 127,
            )))
        }
    }

    #[inline]
    pub fn xxxz(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 128,
            )))
        }
    }

    #[inline]
    pub fn yxxz(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 129,
            )))
        }
    }

    #[inline]
    pub fn zxxz(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 130,
            )))
        }
    }

    #[inline]
    pub fn wxxz(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 131,
            )))
        }
    }

    #[inline]
    pub fn xyxz(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 132,
            )))
        }
    }

    #[inline]
    pub fn yyxz(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 133,
            )))
        }
    }

    #[inline]
    pub fn zyxz(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 134,
            )))
        }
    }

    #[inline]
    pub fn wyxz(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 135,
            )))
        }
    }

    #[inline]
    pub fn xzxz(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 136,
            )))
        }
    }

    #[inline]
    pub fn yzxz(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 137,
            )))
        }
    }

    #[inline]
    pub fn zzxz(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 138,
            )))
        }
    }

    #[inline]
    pub fn wzxz(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 139,
            )))
        }
    }

    #[inline]
    pub fn xwxz(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 140,
            )))
        }
    }

    #[inline]
    pub fn ywxz(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 141,
            )))
        }
    }

    #[inline]
    pub fn zwxz(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 142,
            )))
        }
    }

    #[inline]
    pub fn wwxz(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 143,
            )))
        }
    }

    #[inline]
    pub fn xxyz(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 144,
            )))
        }
    }

    #[inline]
    pub fn yxyz(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 145,
            )))
        }
    }

    #[inline]
    pub fn zxyz(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 146,
            )))
        }
    }

    #[inline]
    pub fn wxyz(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 147,
            )))
        }
    }

    #[inline]
    pub fn xyyz(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 148,
            )))
        }
    }

    #[inline]
    pub fn yyyz(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 149,
            )))
        }
    }

    #[inline]
    pub fn zyyz(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 150,
            )))
        }
    }

    #[inline]
    pub fn wyyz(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 151,
            )))
        }
    }

    #[inline]
    pub fn xzyz(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 152,
            )))
        }
    }

    #[inline]
    pub fn yzyz(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 153,
            )))
        }
    }

    #[inline]
    pub fn zzyz(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 154,
            )))
        }
    }

    #[inline]
    pub fn wzyz(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 155,
            )))
        }
    }

    #[inline]
    pub fn xwyz(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 156,
            )))
        }
    }

    #[inline]
    pub fn ywyz(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 157,
            )))
        }
    }

    #[inline]
    pub fn zwyz(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 158,
            )))
        }
    }

    #[inline]
    pub fn wwyz(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 159,
            )))
        }
    }

    #[inline]
    pub fn xxzz(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 160,
            )))
        }
    }

    #[inline]
    pub fn yxzz(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 161,
            )))
        }
    }

    #[inline]
    pub fn zxzz(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 162,
            )))
        }
    }

    #[inline]
    pub fn wxzz(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 163,
            )))
        }
    }

    #[inline]
    pub fn xyzz(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 164,
            )))
        }
    }

    #[inline]
    pub fn yyzz(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 165,
            )))
        }
    }

    #[inline]
    pub fn zyzz(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 166,
            )))
        }
    }

    #[inline]
    pub fn wyzz(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 167,
            )))
        }
    }

    #[inline]
    pub fn xzzz(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 168,
            )))
        }
    }

    #[inline]
    pub fn yzzz(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 169,
            )))
        }
    }

    #[inline]
    pub fn zzzz(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 170,
            )))
        }
    }

    #[inline]
    pub fn wzzz(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 171,
            )))
        }
    }

    #[inline]
    pub fn xwzz(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 172,
            )))
        }
    }

    #[inline]
    pub fn ywzz(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 173,
            )))
        }
    }

    #[inline]
    pub fn zwzz(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 174,
            )))
        }
    }

    #[inline]
    pub fn wwzz(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 175,
            )))
        }
    }

    #[inline]
    pub fn xxwz(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 176,
            )))
        }
    }

    #[inline]
    pub fn yxwz(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 177,
            )))
        }
    }

    #[inline]
    pub fn zxwz(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 178,
            )))
        }
    }

    #[inline]
    pub fn wxwz(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 179,
            )))
        }
    }

    #[inline]
    pub fn xywz(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 180,
            )))
        }
    }

    #[inline]
    pub fn yywz(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 181,
            )))
        }
    }

    #[inline]
    pub fn zywz(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 182,
            )))
        }
    }

    #[inline]
    pub fn wywz(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 183,
            )))
        }
    }

    #[inline]
    pub fn xzwz(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 184,
            )))
        }
    }

    #[inline]
    pub fn yzwz(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 185,
            )))
        }
    }

    #[inline]
    pub fn zzwz(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 186,
            )))
        }
    }

    #[inline]
    pub fn wzwz(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 187,
            )))
        }
    }

    #[inline]
    pub fn xwwz(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 188,
            )))
        }
    }

    #[inline]
    pub fn ywwz(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 189,
            )))
        }
    }

    #[inline]
    pub fn zwwz(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 190,
            )))
        }
    }

    #[inline]
    pub fn wwwz(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 191,
            )))
        }
    }

    #[inline]
    pub fn xxxw(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 192,
            )))
        }
    }

    #[inline]
    pub fn yxxw(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 193,
            )))
        }
    }

    #[inline]
    pub fn zxxw(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 194,
            )))
        }
    }

    #[inline]
    pub fn wxxw(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 195,
            )))
        }
    }

    #[inline]
    pub fn xyxw(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 196,
            )))
        }
    }

    #[inline]
    pub fn yyxw(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 197,
            )))
        }
    }

    #[inline]
    pub fn zyxw(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 198,
            )))
        }
    }

    #[inline]
    pub fn wyxw(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 199,
            )))
        }
    }

    #[inline]
    pub fn xzxw(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 200,
            )))
        }
    }

    #[inline]
    pub fn yzxw(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 201,
            )))
        }
    }

    #[inline]
    pub fn zzxw(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 202,
            )))
        }
    }

    #[inline]
    pub fn wzxw(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 203,
            )))
        }
    }

    #[inline]
    pub fn xwxw(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 204,
            )))
        }
    }

    #[inline]
    pub fn ywxw(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 205,
            )))
        }
    }

    #[inline]
    pub fn zwxw(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 206,
            )))
        }
    }

    #[inline]
    pub fn wwxw(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 207,
            )))
        }
    }

    #[inline]
    pub fn xxyw(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 208,
            )))
        }
    }

    #[inline]
    pub fn yxyw(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 209,
            )))
        }
    }

    #[inline]
    pub fn zxyw(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 210,
            )))
        }
    }

    #[inline]
    pub fn wxyw(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 211,
            )))
        }
    }

    #[inline]
    pub fn xyyw(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 212,
            )))
        }
    }

    #[inline]
    pub fn yyyw(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 213,
            )))
        }
    }

    #[inline]
    pub fn zyyw(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 214,
            )))
        }
    }

    #[inline]
    pub fn wyyw(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 215,
            )))
        }
    }

    #[inline]
    pub fn xzyw(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 216,
            )))
        }
    }

    #[inline]
    pub fn yzyw(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 217,
            )))
        }
    }

    #[inline]
    pub fn zzyw(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 218,
            )))
        }
    }

    #[inline]
    pub fn wzyw(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 219,
            )))
        }
    }

    #[inline]
    pub fn xwyw(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 220,
            )))
        }
    }

    #[inline]
    pub fn ywyw(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 221,
            )))
        }
    }

    #[inline]
    pub fn zwyw(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 222,
            )))
        }
    }

    #[inline]
    pub fn wwyw(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 223,
            )))
        }
    }

    #[inline]
    pub fn xxzw(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 224,
            )))
        }
    }

    #[inline]
    pub fn yxzw(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 225,
            )))
        }
    }

    #[inline]
    pub fn zxzw(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 226,
            )))
        }
    }

    #[inline]
    pub fn wxzw(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 227,
            )))
        }
    }

    #[inline]
    pub fn xyzw(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 228,
            )))
        }
    }

    #[inline]
    pub fn yyzw(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 229,
            )))
        }
    }

    #[inline]
    pub fn zyzw(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 230,
            )))
        }
    }

    #[inline]
    pub fn wyzw(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 231,
            )))
        }
    }

    #[inline]
    pub fn xzzw(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 232,
            )))
        }
    }

    #[inline]
    pub fn yzzw(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 233,
            )))
        }
    }

    #[inline]
    pub fn zzzw(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 234,
            )))
        }
    }

    #[inline]
    pub fn wzzw(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 235,
            )))
        }
    }

    #[inline]
    pub fn xwzw(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 236,
            )))
        }
    }

    #[inline]
    pub fn ywzw(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 237,
            )))
        }
    }

    #[inline]
    pub fn zwzw(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 238,
            )))
        }
    }

    #[inline]
    pub fn wwzw(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 239,
            )))
        }
    }

    #[inline]
    pub fn xxww(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 240,
            )))
        }
    }

    #[inline]
    pub fn yxww(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 241,
            )))
        }
    }

    #[inline]
    pub fn zxww(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 242,
            )))
        }
    }

    #[inline]
    pub fn wxww(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 243,
            )))
        }
    }

    #[inline]
    pub fn xyww(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 244,
            )))
        }
    }

    #[inline]
    pub fn yyww(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 245,
            )))
        }
    }

    #[inline]
    pub fn zyww(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 246,
            )))
        }
    }

    #[inline]
    pub fn wyww(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 247,
            )))
        }
    }

    #[inline]
    pub fn xzww(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 248,
            )))
        }
    }

    #[inline]
    pub fn yzww(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 249,
            )))
        }
    }

    #[inline]
    pub fn zzww(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 250,
            )))
        }
    }

    #[inline]
    pub fn wzww(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 251,
            )))
        }
    }

    #[inline]
    pub fn xwww(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 252,
            )))
        }
    }

    #[inline]
    pub fn ywww(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 253,
            )))
        }
    }

    #[inline]
    pub fn zwww(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 254,
            )))
        }
    }

    #[inline]
    pub fn wwww(self) -> I32x4 {
        unsafe {
            let this = x86::_mm_castsi128_ps(self.0);
            I32x4(x86::_mm_castps_si128(x86::_mm_shuffle_ps(
                this, this, 255,
            )))
        }
    }
}
