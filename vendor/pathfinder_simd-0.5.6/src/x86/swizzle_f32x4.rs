// pathfinder/simd/src/x86/swizzle_f32x4.rs
//
// Copyright © 2019 The Pathfinder Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::x86::F32x4;

#[cfg(target_pointer_width = "32")]
use std::arch::x86;
#[cfg(target_pointer_width = "64")]
use std::arch::x86_64 as x86;

impl F32x4 {
    #[inline]
    pub fn xxxx(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 0)) }
    }

    #[inline]
    pub fn yxxx(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 1)) }
    }

    #[inline]
    pub fn zxxx(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 2)) }
    }

    #[inline]
    pub fn wxxx(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 3)) }
    }

    #[inline]
    pub fn xyxx(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 4)) }
    }

    #[inline]
    pub fn yyxx(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 5)) }
    }

    #[inline]
    pub fn zyxx(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 6)) }
    }

    #[inline]
    pub fn wyxx(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 7)) }
    }

    #[inline]
    pub fn xzxx(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 8)) }
    }

    #[inline]
    pub fn yzxx(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 9)) }
    }

    #[inline]
    pub fn zzxx(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 10)) }
    }

    #[inline]
    pub fn wzxx(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 11)) }
    }

    #[inline]
    pub fn xwxx(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 12)) }
    }

    #[inline]
    pub fn ywxx(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 13)) }
    }

    #[inline]
    pub fn zwxx(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 14)) }
    }

    #[inline]
    pub fn wwxx(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 15)) }
    }

    #[inline]
    pub fn xxyx(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 16)) }
    }

    #[inline]
    pub fn yxyx(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 17)) }
    }

    #[inline]
    pub fn zxyx(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 18)) }
    }

    #[inline]
    pub fn wxyx(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 19)) }
    }

    #[inline]
    pub fn xyyx(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 20)) }
    }

    #[inline]
    pub fn yyyx(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 21)) }
    }

    #[inline]
    pub fn zyyx(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 22)) }
    }

    #[inline]
    pub fn wyyx(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 23)) }
    }

    #[inline]
    pub fn xzyx(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 24)) }
    }

    #[inline]
    pub fn yzyx(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 25)) }
    }

    #[inline]
    pub fn zzyx(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 26)) }
    }

    #[inline]
    pub fn wzyx(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 27)) }
    }

    #[inline]
    pub fn xwyx(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 28)) }
    }

    #[inline]
    pub fn ywyx(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 29)) }
    }

    #[inline]
    pub fn zwyx(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 30)) }
    }

    #[inline]
    pub fn wwyx(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 31)) }
    }

    #[inline]
    pub fn xxzx(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 32)) }
    }

    #[inline]
    pub fn yxzx(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 33)) }
    }

    #[inline]
    pub fn zxzx(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 34)) }
    }

    #[inline]
    pub fn wxzx(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 35)) }
    }

    #[inline]
    pub fn xyzx(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 36)) }
    }

    #[inline]
    pub fn yyzx(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 37)) }
    }

    #[inline]
    pub fn zyzx(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 38)) }
    }

    #[inline]
    pub fn wyzx(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 39)) }
    }

    #[inline]
    pub fn xzzx(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 40)) }
    }

    #[inline]
    pub fn yzzx(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 41)) }
    }

    #[inline]
    pub fn zzzx(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 42)) }
    }

    #[inline]
    pub fn wzzx(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 43)) }
    }

    #[inline]
    pub fn xwzx(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 44)) }
    }

    #[inline]
    pub fn ywzx(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 45)) }
    }

    #[inline]
    pub fn zwzx(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 46)) }
    }

    #[inline]
    pub fn wwzx(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 47)) }
    }

    #[inline]
    pub fn xxwx(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 48)) }
    }

    #[inline]
    pub fn yxwx(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 49)) }
    }

    #[inline]
    pub fn zxwx(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 50)) }
    }

    #[inline]
    pub fn wxwx(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 51)) }
    }

    #[inline]
    pub fn xywx(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 52)) }
    }

    #[inline]
    pub fn yywx(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 53)) }
    }

    #[inline]
    pub fn zywx(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 54)) }
    }

    #[inline]
    pub fn wywx(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 55)) }
    }

    #[inline]
    pub fn xzwx(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 56)) }
    }

    #[inline]
    pub fn yzwx(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 57)) }
    }

    #[inline]
    pub fn zzwx(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 58)) }
    }

    #[inline]
    pub fn wzwx(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 59)) }
    }

    #[inline]
    pub fn xwwx(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 60)) }
    }

    #[inline]
    pub fn ywwx(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 61)) }
    }

    #[inline]
    pub fn zwwx(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 62)) }
    }

    #[inline]
    pub fn wwwx(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 63)) }
    }

    #[inline]
    pub fn xxxy(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 64)) }
    }

    #[inline]
    pub fn yxxy(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 65)) }
    }

    #[inline]
    pub fn zxxy(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 66)) }
    }

    #[inline]
    pub fn wxxy(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 67)) }
    }

    #[inline]
    pub fn xyxy(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 68)) }
    }

    #[inline]
    pub fn yyxy(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 69)) }
    }

    #[inline]
    pub fn zyxy(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 70)) }
    }

    #[inline]
    pub fn wyxy(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 71)) }
    }

    #[inline]
    pub fn xzxy(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 72)) }
    }

    #[inline]
    pub fn yzxy(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 73)) }
    }

    #[inline]
    pub fn zzxy(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 74)) }
    }

    #[inline]
    pub fn wzxy(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 75)) }
    }

    #[inline]
    pub fn xwxy(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 76)) }
    }

    #[inline]
    pub fn ywxy(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 77)) }
    }

    #[inline]
    pub fn zwxy(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 78)) }
    }

    #[inline]
    pub fn wwxy(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 79)) }
    }

    #[inline]
    pub fn xxyy(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 80)) }
    }

    #[inline]
    pub fn yxyy(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 81)) }
    }

    #[inline]
    pub fn zxyy(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 82)) }
    }

    #[inline]
    pub fn wxyy(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 83)) }
    }

    #[inline]
    pub fn xyyy(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 84)) }
    }

    #[inline]
    pub fn yyyy(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 85)) }
    }

    #[inline]
    pub fn zyyy(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 86)) }
    }

    #[inline]
    pub fn wyyy(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 87)) }
    }

    #[inline]
    pub fn xzyy(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 88)) }
    }

    #[inline]
    pub fn yzyy(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 89)) }
    }

    #[inline]
    pub fn zzyy(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 90)) }
    }

    #[inline]
    pub fn wzyy(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 91)) }
    }

    #[inline]
    pub fn xwyy(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 92)) }
    }

    #[inline]
    pub fn ywyy(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 93)) }
    }

    #[inline]
    pub fn zwyy(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 94)) }
    }

    #[inline]
    pub fn wwyy(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 95)) }
    }

    #[inline]
    pub fn xxzy(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 96)) }
    }

    #[inline]
    pub fn yxzy(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 97)) }
    }

    #[inline]
    pub fn zxzy(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 98)) }
    }

    #[inline]
    pub fn wxzy(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 99)) }
    }

    #[inline]
    pub fn xyzy(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 100)) }
    }

    #[inline]
    pub fn yyzy(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 101)) }
    }

    #[inline]
    pub fn zyzy(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 102)) }
    }

    #[inline]
    pub fn wyzy(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 103)) }
    }

    #[inline]
    pub fn xzzy(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 104)) }
    }

    #[inline]
    pub fn yzzy(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 105)) }
    }

    #[inline]
    pub fn zzzy(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 106)) }
    }

    #[inline]
    pub fn wzzy(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 107)) }
    }

    #[inline]
    pub fn xwzy(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 108)) }
    }

    #[inline]
    pub fn ywzy(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 109)) }
    }

    #[inline]
    pub fn zwzy(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 110)) }
    }

    #[inline]
    pub fn wwzy(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 111)) }
    }

    #[inline]
    pub fn xxwy(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 112)) }
    }

    #[inline]
    pub fn yxwy(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 113)) }
    }

    #[inline]
    pub fn zxwy(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 114)) }
    }

    #[inline]
    pub fn wxwy(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 115)) }
    }

    #[inline]
    pub fn xywy(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 116)) }
    }

    #[inline]
    pub fn yywy(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 117)) }
    }

    #[inline]
    pub fn zywy(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 118)) }
    }

    #[inline]
    pub fn wywy(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 119)) }
    }

    #[inline]
    pub fn xzwy(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 120)) }
    }

    #[inline]
    pub fn yzwy(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 121)) }
    }

    #[inline]
    pub fn zzwy(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 122)) }
    }

    #[inline]
    pub fn wzwy(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 123)) }
    }

    #[inline]
    pub fn xwwy(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 124)) }
    }

    #[inline]
    pub fn ywwy(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 125)) }
    }

    #[inline]
    pub fn zwwy(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 126)) }
    }

    #[inline]
    pub fn wwwy(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 127)) }
    }

    #[inline]
    pub fn xxxz(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 128)) }
    }

    #[inline]
    pub fn yxxz(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 129)) }
    }

    #[inline]
    pub fn zxxz(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 130)) }
    }

    #[inline]
    pub fn wxxz(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 131)) }
    }

    #[inline]
    pub fn xyxz(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 132)) }
    }

    #[inline]
    pub fn yyxz(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 133)) }
    }

    #[inline]
    pub fn zyxz(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 134)) }
    }

    #[inline]
    pub fn wyxz(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 135)) }
    }

    #[inline]
    pub fn xzxz(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 136)) }
    }

    #[inline]
    pub fn yzxz(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 137)) }
    }

    #[inline]
    pub fn zzxz(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 138)) }
    }

    #[inline]
    pub fn wzxz(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 139)) }
    }

    #[inline]
    pub fn xwxz(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 140)) }
    }

    #[inline]
    pub fn ywxz(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 141)) }
    }

    #[inline]
    pub fn zwxz(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 142)) }
    }

    #[inline]
    pub fn wwxz(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 143)) }
    }

    #[inline]
    pub fn xxyz(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 144)) }
    }

    #[inline]
    pub fn yxyz(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 145)) }
    }

    #[inline]
    pub fn zxyz(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 146)) }
    }

    #[inline]
    pub fn wxyz(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 147)) }
    }

    #[inline]
    pub fn xyyz(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 148)) }
    }

    #[inline]
    pub fn yyyz(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 149)) }
    }

    #[inline]
    pub fn zyyz(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 150)) }
    }

    #[inline]
    pub fn wyyz(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 151)) }
    }

    #[inline]
    pub fn xzyz(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 152)) }
    }

    #[inline]
    pub fn yzyz(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 153)) }
    }

    #[inline]
    pub fn zzyz(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 154)) }
    }

    #[inline]
    pub fn wzyz(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 155)) }
    }

    #[inline]
    pub fn xwyz(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 156)) }
    }

    #[inline]
    pub fn ywyz(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 157)) }
    }

    #[inline]
    pub fn zwyz(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 158)) }
    }

    #[inline]
    pub fn wwyz(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 159)) }
    }

    #[inline]
    pub fn xxzz(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 160)) }
    }

    #[inline]
    pub fn yxzz(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 161)) }
    }

    #[inline]
    pub fn zxzz(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 162)) }
    }

    #[inline]
    pub fn wxzz(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 163)) }
    }

    #[inline]
    pub fn xyzz(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 164)) }
    }

    #[inline]
    pub fn yyzz(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 165)) }
    }

    #[inline]
    pub fn zyzz(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 166)) }
    }

    #[inline]
    pub fn wyzz(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 167)) }
    }

    #[inline]
    pub fn xzzz(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 168)) }
    }

    #[inline]
    pub fn yzzz(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 169)) }
    }

    #[inline]
    pub fn zzzz(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 170)) }
    }

    #[inline]
    pub fn wzzz(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 171)) }
    }

    #[inline]
    pub fn xwzz(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 172)) }
    }

    #[inline]
    pub fn ywzz(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 173)) }
    }

    #[inline]
    pub fn zwzz(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 174)) }
    }

    #[inline]
    pub fn wwzz(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 175)) }
    }

    #[inline]
    pub fn xxwz(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 176)) }
    }

    #[inline]
    pub fn yxwz(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 177)) }
    }

    #[inline]
    pub fn zxwz(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 178)) }
    }

    #[inline]
    pub fn wxwz(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 179)) }
    }

    #[inline]
    pub fn xywz(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 180)) }
    }

    #[inline]
    pub fn yywz(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 181)) }
    }

    #[inline]
    pub fn zywz(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 182)) }
    }

    #[inline]
    pub fn wywz(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 183)) }
    }

    #[inline]
    pub fn xzwz(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 184)) }
    }

    #[inline]
    pub fn yzwz(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 185)) }
    }

    #[inline]
    pub fn zzwz(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 186)) }
    }

    #[inline]
    pub fn wzwz(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 187)) }
    }

    #[inline]
    pub fn xwwz(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 188)) }
    }

    #[inline]
    pub fn ywwz(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 189)) }
    }

    #[inline]
    pub fn zwwz(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 190)) }
    }

    #[inline]
    pub fn wwwz(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 191)) }
    }

    #[inline]
    pub fn xxxw(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 192)) }
    }

    #[inline]
    pub fn yxxw(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 193)) }
    }

    #[inline]
    pub fn zxxw(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 194)) }
    }

    #[inline]
    pub fn wxxw(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 195)) }
    }

    #[inline]
    pub fn xyxw(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 196)) }
    }

    #[inline]
    pub fn yyxw(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 197)) }
    }

    #[inline]
    pub fn zyxw(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 198)) }
    }

    #[inline]
    pub fn wyxw(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 199)) }
    }

    #[inline]
    pub fn xzxw(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 200)) }
    }

    #[inline]
    pub fn yzxw(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 201)) }
    }

    #[inline]
    pub fn zzxw(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 202)) }
    }

    #[inline]
    pub fn wzxw(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 203)) }
    }

    #[inline]
    pub fn xwxw(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 204)) }
    }

    #[inline]
    pub fn ywxw(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 205)) }
    }

    #[inline]
    pub fn zwxw(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 206)) }
    }

    #[inline]
    pub fn wwxw(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 207)) }
    }

    #[inline]
    pub fn xxyw(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 208)) }
    }

    #[inline]
    pub fn yxyw(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 209)) }
    }

    #[inline]
    pub fn zxyw(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 210)) }
    }

    #[inline]
    pub fn wxyw(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 211)) }
    }

    #[inline]
    pub fn xyyw(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 212)) }
    }

    #[inline]
    pub fn yyyw(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 213)) }
    }

    #[inline]
    pub fn zyyw(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 214)) }
    }

    #[inline]
    pub fn wyyw(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 215)) }
    }

    #[inline]
    pub fn xzyw(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 216)) }
    }

    #[inline]
    pub fn yzyw(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 217)) }
    }

    #[inline]
    pub fn zzyw(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 218)) }
    }

    #[inline]
    pub fn wzyw(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 219)) }
    }

    #[inline]
    pub fn xwyw(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 220)) }
    }

    #[inline]
    pub fn ywyw(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 221)) }
    }

    #[inline]
    pub fn zwyw(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 222)) }
    }

    #[inline]
    pub fn wwyw(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 223)) }
    }

    #[inline]
    pub fn xxzw(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 224)) }
    }

    #[inline]
    pub fn yxzw(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 225)) }
    }

    #[inline]
    pub fn zxzw(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 226)) }
    }

    #[inline]
    pub fn wxzw(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 227)) }
    }

    #[inline]
    pub fn xyzw(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 228)) }
    }

    #[inline]
    pub fn yyzw(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 229)) }
    }

    #[inline]
    pub fn zyzw(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 230)) }
    }

    #[inline]
    pub fn wyzw(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 231)) }
    }

    #[inline]
    pub fn xzzw(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 232)) }
    }

    #[inline]
    pub fn yzzw(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 233)) }
    }

    #[inline]
    pub fn zzzw(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 234)) }
    }

    #[inline]
    pub fn wzzw(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 235)) }
    }

    #[inline]
    pub fn xwzw(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 236)) }
    }

    #[inline]
    pub fn ywzw(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 237)) }
    }

    #[inline]
    pub fn zwzw(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 238)) }
    }

    #[inline]
    pub fn wwzw(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 239)) }
    }

    #[inline]
    pub fn xxww(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 240)) }
    }

    #[inline]
    pub fn yxww(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 241)) }
    }

    #[inline]
    pub fn zxww(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 242)) }
    }

    #[inline]
    pub fn wxww(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 243)) }
    }

    #[inline]
    pub fn xyww(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 244)) }
    }

    #[inline]
    pub fn yyww(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 245)) }
    }

    #[inline]
    pub fn zyww(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 246)) }
    }

    #[inline]
    pub fn wyww(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 247)) }
    }

    #[inline]
    pub fn xzww(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 248)) }
    }

    #[inline]
    pub fn yzww(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 249)) }
    }

    #[inline]
    pub fn zzww(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 250)) }
    }

    #[inline]
    pub fn wzww(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 251)) }
    }

    #[inline]
    pub fn xwww(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 252)) }
    }

    #[inline]
    pub fn ywww(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 253)) }
    }

    #[inline]
    pub fn zwww(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 254)) }
    }

    #[inline]
    pub fn wwww(self) -> F32x4 {
        unsafe { F32x4(x86::_mm_shuffle_ps(self.0, self.0, 255)) }
    }
}
