// pathfinder/simd/src/scalar/swizzle_f32x4.rs
//
// Copyright © 2019 The Pathfinder Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::scalar::F32x4;

impl F32x4 {
    /// Constructs a new vector from the first, first, first, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xxxx(self) -> F32x4 {
        F32x4([self[0], self[0], self[0], self[0]])
    }

    /// Constructs a new vector from the second, first, first, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yxxx(self) -> F32x4 {
        F32x4([self[1], self[0], self[0], self[0]])
    }

    /// Constructs a new vector from the third, first, first, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zxxx(self) -> F32x4 {
        F32x4([self[2], self[0], self[0], self[0]])
    }

    /// Constructs a new vector from the fourth, first, first, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wxxx(self) -> F32x4 {
        F32x4([self[3], self[0], self[0], self[0]])
    }

    /// Constructs a new vector from the first, second, first, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xyxx(self) -> F32x4 {
        F32x4([self[0], self[1], self[0], self[0]])
    }

    /// Constructs a new vector from the second, second, first, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yyxx(self) -> F32x4 {
        F32x4([self[1], self[1], self[0], self[0]])
    }

    /// Constructs a new vector from the third, second, first, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zyxx(self) -> F32x4 {
        F32x4([self[2], self[1], self[0], self[0]])
    }

    /// Constructs a new vector from the fourth, second, first, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wyxx(self) -> F32x4 {
        F32x4([self[3], self[1], self[0], self[0]])
    }

    /// Constructs a new vector from the first, third, first, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xzxx(self) -> F32x4 {
        F32x4([self[0], self[2], self[0], self[0]])
    }

    /// Constructs a new vector from the second, third, first, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yzxx(self) -> F32x4 {
        F32x4([self[1], self[2], self[0], self[0]])
    }

    /// Constructs a new vector from the third, third, first, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zzxx(self) -> F32x4 {
        F32x4([self[2], self[2], self[0], self[0]])
    }

    /// Constructs a new vector from the fourth, third, first, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wzxx(self) -> F32x4 {
        F32x4([self[3], self[2], self[0], self[0]])
    }

    /// Constructs a new vector from the first, fourth, first, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xwxx(self) -> F32x4 {
        F32x4([self[0], self[3], self[0], self[0]])
    }

    /// Constructs a new vector from the second, fourth, first, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn ywxx(self) -> F32x4 {
        F32x4([self[1], self[3], self[0], self[0]])
    }

    /// Constructs a new vector from the third, fourth, first, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zwxx(self) -> F32x4 {
        F32x4([self[2], self[3], self[0], self[0]])
    }

    /// Constructs a new vector from the fourth, fourth, first, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wwxx(self) -> F32x4 {
        F32x4([self[3], self[3], self[0], self[0]])
    }

    /// Constructs a new vector from the first, first, second, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xxyx(self) -> F32x4 {
        F32x4([self[0], self[0], self[1], self[0]])
    }

    /// Constructs a new vector from the second, first, second, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yxyx(self) -> F32x4 {
        F32x4([self[1], self[0], self[1], self[0]])
    }

    /// Constructs a new vector from the third, first, second, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zxyx(self) -> F32x4 {
        F32x4([self[2], self[0], self[1], self[0]])
    }

    /// Constructs a new vector from the fourth, first, second, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wxyx(self) -> F32x4 {
        F32x4([self[3], self[0], self[1], self[0]])
    }

    /// Constructs a new vector from the first, second, second, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xyyx(self) -> F32x4 {
        F32x4([self[0], self[1], self[1], self[0]])
    }

    /// Constructs a new vector from the second, second, second, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yyyx(self) -> F32x4 {
        F32x4([self[1], self[1], self[1], self[0]])
    }

    /// Constructs a new vector from the third, second, second, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zyyx(self) -> F32x4 {
        F32x4([self[2], self[1], self[1], self[0]])
    }

    /// Constructs a new vector from the fourth, second, second, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wyyx(self) -> F32x4 {
        F32x4([self[3], self[1], self[1], self[0]])
    }

    /// Constructs a new vector from the first, third, second, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xzyx(self) -> F32x4 {
        F32x4([self[0], self[2], self[1], self[0]])
    }

    /// Constructs a new vector from the second, third, second, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yzyx(self) -> F32x4 {
        F32x4([self[1], self[2], self[1], self[0]])
    }

    /// Constructs a new vector from the third, third, second, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zzyx(self) -> F32x4 {
        F32x4([self[2], self[2], self[1], self[0]])
    }

    /// Constructs a new vector from the fourth, third, second, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wzyx(self) -> F32x4 {
        F32x4([self[3], self[2], self[1], self[0]])
    }

    /// Constructs a new vector from the first, fourth, second, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xwyx(self) -> F32x4 {
        F32x4([self[0], self[3], self[1], self[0]])
    }

    /// Constructs a new vector from the second, fourth, second, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn ywyx(self) -> F32x4 {
        F32x4([self[1], self[3], self[1], self[0]])
    }

    /// Constructs a new vector from the third, fourth, second, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zwyx(self) -> F32x4 {
        F32x4([self[2], self[3], self[1], self[0]])
    }

    /// Constructs a new vector from the fourth, fourth, second, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wwyx(self) -> F32x4 {
        F32x4([self[3], self[3], self[1], self[0]])
    }

    /// Constructs a new vector from the first, first, third, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xxzx(self) -> F32x4 {
        F32x4([self[0], self[0], self[2], self[0]])
    }

    /// Constructs a new vector from the second, first, third, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yxzx(self) -> F32x4 {
        F32x4([self[1], self[0], self[2], self[0]])
    }

    /// Constructs a new vector from the third, first, third, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zxzx(self) -> F32x4 {
        F32x4([self[2], self[0], self[2], self[0]])
    }

    /// Constructs a new vector from the fourth, first, third, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wxzx(self) -> F32x4 {
        F32x4([self[3], self[0], self[2], self[0]])
    }

    /// Constructs a new vector from the first, second, third, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xyzx(self) -> F32x4 {
        F32x4([self[0], self[1], self[2], self[0]])
    }

    /// Constructs a new vector from the second, second, third, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yyzx(self) -> F32x4 {
        F32x4([self[1], self[1], self[2], self[0]])
    }

    /// Constructs a new vector from the third, second, third, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zyzx(self) -> F32x4 {
        F32x4([self[2], self[1], self[2], self[0]])
    }

    /// Constructs a new vector from the fourth, second, third, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wyzx(self) -> F32x4 {
        F32x4([self[3], self[1], self[2], self[0]])
    }

    /// Constructs a new vector from the first, third, third, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xzzx(self) -> F32x4 {
        F32x4([self[0], self[2], self[2], self[0]])
    }

    /// Constructs a new vector from the second, third, third, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yzzx(self) -> F32x4 {
        F32x4([self[1], self[2], self[2], self[0]])
    }

    /// Constructs a new vector from the third, third, third, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zzzx(self) -> F32x4 {
        F32x4([self[2], self[2], self[2], self[0]])
    }

    /// Constructs a new vector from the fourth, third, third, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wzzx(self) -> F32x4 {
        F32x4([self[3], self[2], self[2], self[0]])
    }

    /// Constructs a new vector from the first, fourth, third, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xwzx(self) -> F32x4 {
        F32x4([self[0], self[3], self[2], self[0]])
    }

    /// Constructs a new vector from the second, fourth, third, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn ywzx(self) -> F32x4 {
        F32x4([self[1], self[3], self[2], self[0]])
    }

    /// Constructs a new vector from the third, fourth, third, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zwzx(self) -> F32x4 {
        F32x4([self[2], self[3], self[2], self[0]])
    }

    /// Constructs a new vector from the fourth, fourth, third, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wwzx(self) -> F32x4 {
        F32x4([self[3], self[3], self[2], self[0]])
    }

    /// Constructs a new vector from the first, first, fourth, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xxwx(self) -> F32x4 {
        F32x4([self[0], self[0], self[3], self[0]])
    }

    /// Constructs a new vector from the second, first, fourth, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yxwx(self) -> F32x4 {
        F32x4([self[1], self[0], self[3], self[0]])
    }

    /// Constructs a new vector from the third, first, fourth, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zxwx(self) -> F32x4 {
        F32x4([self[2], self[0], self[3], self[0]])
    }

    /// Constructs a new vector from the fourth, first, fourth, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wxwx(self) -> F32x4 {
        F32x4([self[3], self[0], self[3], self[0]])
    }

    /// Constructs a new vector from the first, second, fourth, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xywx(self) -> F32x4 {
        F32x4([self[0], self[1], self[3], self[0]])
    }

    /// Constructs a new vector from the second, second, fourth, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yywx(self) -> F32x4 {
        F32x4([self[1], self[1], self[3], self[0]])
    }

    /// Constructs a new vector from the third, second, fourth, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zywx(self) -> F32x4 {
        F32x4([self[2], self[1], self[3], self[0]])
    }

    /// Constructs a new vector from the fourth, second, fourth, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wywx(self) -> F32x4 {
        F32x4([self[3], self[1], self[3], self[0]])
    }

    /// Constructs a new vector from the first, third, fourth, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xzwx(self) -> F32x4 {
        F32x4([self[0], self[2], self[3], self[0]])
    }

    /// Constructs a new vector from the second, third, fourth, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yzwx(self) -> F32x4 {
        F32x4([self[1], self[2], self[3], self[0]])
    }

    /// Constructs a new vector from the third, third, fourth, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zzwx(self) -> F32x4 {
        F32x4([self[2], self[2], self[3], self[0]])
    }

    /// Constructs a new vector from the fourth, third, fourth, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wzwx(self) -> F32x4 {
        F32x4([self[3], self[2], self[3], self[0]])
    }

    /// Constructs a new vector from the first, fourth, fourth, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xwwx(self) -> F32x4 {
        F32x4([self[0], self[3], self[3], self[0]])
    }

    /// Constructs a new vector from the second, fourth, fourth, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn ywwx(self) -> F32x4 {
        F32x4([self[1], self[3], self[3], self[0]])
    }

    /// Constructs a new vector from the third, fourth, fourth, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zwwx(self) -> F32x4 {
        F32x4([self[2], self[3], self[3], self[0]])
    }

    /// Constructs a new vector from the fourth, fourth, fourth, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wwwx(self) -> F32x4 {
        F32x4([self[3], self[3], self[3], self[0]])
    }

    /// Constructs a new vector from the first, first, first, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xxxy(self) -> F32x4 {
        F32x4([self[0], self[0], self[0], self[1]])
    }

    /// Constructs a new vector from the second, first, first, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yxxy(self) -> F32x4 {
        F32x4([self[1], self[0], self[0], self[1]])
    }

    /// Constructs a new vector from the third, first, first, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zxxy(self) -> F32x4 {
        F32x4([self[2], self[0], self[0], self[1]])
    }

    /// Constructs a new vector from the fourth, first, first, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wxxy(self) -> F32x4 {
        F32x4([self[3], self[0], self[0], self[1]])
    }

    /// Constructs a new vector from the first, second, first, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xyxy(self) -> F32x4 {
        F32x4([self[0], self[1], self[0], self[1]])
    }

    /// Constructs a new vector from the second, second, first, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yyxy(self) -> F32x4 {
        F32x4([self[1], self[1], self[0], self[1]])
    }

    /// Constructs a new vector from the third, second, first, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zyxy(self) -> F32x4 {
        F32x4([self[2], self[1], self[0], self[1]])
    }

    /// Constructs a new vector from the fourth, second, first, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wyxy(self) -> F32x4 {
        F32x4([self[3], self[1], self[0], self[1]])
    }

    /// Constructs a new vector from the first, third, first, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xzxy(self) -> F32x4 {
        F32x4([self[0], self[2], self[0], self[1]])
    }

    /// Constructs a new vector from the second, third, first, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yzxy(self) -> F32x4 {
        F32x4([self[1], self[2], self[0], self[1]])
    }

    /// Constructs a new vector from the third, third, first, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zzxy(self) -> F32x4 {
        F32x4([self[2], self[2], self[0], self[1]])
    }

    /// Constructs a new vector from the fourth, third, first, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wzxy(self) -> F32x4 {
        F32x4([self[3], self[2], self[0], self[1]])
    }

    /// Constructs a new vector from the first, fourth, first, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xwxy(self) -> F32x4 {
        F32x4([self[0], self[3], self[0], self[1]])
    }

    /// Constructs a new vector from the second, fourth, first, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn ywxy(self) -> F32x4 {
        F32x4([self[1], self[3], self[0], self[1]])
    }

    /// Constructs a new vector from the third, fourth, first, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zwxy(self) -> F32x4 {
        F32x4([self[2], self[3], self[0], self[1]])
    }

    /// Constructs a new vector from the fourth, fourth, first, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wwxy(self) -> F32x4 {
        F32x4([self[3], self[3], self[0], self[1]])
    }

    /// Constructs a new vector from the first, first, second, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xxyy(self) -> F32x4 {
        F32x4([self[0], self[0], self[1], self[1]])
    }

    /// Constructs a new vector from the second, first, second, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yxyy(self) -> F32x4 {
        F32x4([self[1], self[0], self[1], self[1]])
    }

    /// Constructs a new vector from the third, first, second, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zxyy(self) -> F32x4 {
        F32x4([self[2], self[0], self[1], self[1]])
    }

    /// Constructs a new vector from the fourth, first, second, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wxyy(self) -> F32x4 {
        F32x4([self[3], self[0], self[1], self[1]])
    }

    /// Constructs a new vector from the first, second, second, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xyyy(self) -> F32x4 {
        F32x4([self[0], self[1], self[1], self[1]])
    }

    /// Constructs a new vector from the second, second, second, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yyyy(self) -> F32x4 {
        F32x4([self[1], self[1], self[1], self[1]])
    }

    /// Constructs a new vector from the third, second, second, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zyyy(self) -> F32x4 {
        F32x4([self[2], self[1], self[1], self[1]])
    }

    /// Constructs a new vector from the fourth, second, second, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wyyy(self) -> F32x4 {
        F32x4([self[3], self[1], self[1], self[1]])
    }

    /// Constructs a new vector from the first, third, second, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xzyy(self) -> F32x4 {
        F32x4([self[0], self[2], self[1], self[1]])
    }

    /// Constructs a new vector from the second, third, second, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yzyy(self) -> F32x4 {
        F32x4([self[1], self[2], self[1], self[1]])
    }

    /// Constructs a new vector from the third, third, second, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zzyy(self) -> F32x4 {
        F32x4([self[2], self[2], self[1], self[1]])
    }

    /// Constructs a new vector from the fourth, third, second, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wzyy(self) -> F32x4 {
        F32x4([self[3], self[2], self[1], self[1]])
    }

    /// Constructs a new vector from the first, fourth, second, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xwyy(self) -> F32x4 {
        F32x4([self[0], self[3], self[1], self[1]])
    }

    /// Constructs a new vector from the second, fourth, second, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn ywyy(self) -> F32x4 {
        F32x4([self[1], self[3], self[1], self[1]])
    }

    /// Constructs a new vector from the third, fourth, second, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zwyy(self) -> F32x4 {
        F32x4([self[2], self[3], self[1], self[1]])
    }

    /// Constructs a new vector from the fourth, fourth, second, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wwyy(self) -> F32x4 {
        F32x4([self[3], self[3], self[1], self[1]])
    }

    /// Constructs a new vector from the first, first, third, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xxzy(self) -> F32x4 {
        F32x4([self[0], self[0], self[2], self[1]])
    }

    /// Constructs a new vector from the second, first, third, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yxzy(self) -> F32x4 {
        F32x4([self[1], self[0], self[2], self[1]])
    }

    /// Constructs a new vector from the third, first, third, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zxzy(self) -> F32x4 {
        F32x4([self[2], self[0], self[2], self[1]])
    }

    /// Constructs a new vector from the fourth, first, third, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wxzy(self) -> F32x4 {
        F32x4([self[3], self[0], self[2], self[1]])
    }

    /// Constructs a new vector from the first, second, third, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xyzy(self) -> F32x4 {
        F32x4([self[0], self[1], self[2], self[1]])
    }

    /// Constructs a new vector from the second, second, third, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yyzy(self) -> F32x4 {
        F32x4([self[1], self[1], self[2], self[1]])
    }

    /// Constructs a new vector from the third, second, third, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zyzy(self) -> F32x4 {
        F32x4([self[2], self[1], self[2], self[1]])
    }

    /// Constructs a new vector from the fourth, second, third, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wyzy(self) -> F32x4 {
        F32x4([self[3], self[1], self[2], self[1]])
    }

    /// Constructs a new vector from the first, third, third, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xzzy(self) -> F32x4 {
        F32x4([self[0], self[2], self[2], self[1]])
    }

    /// Constructs a new vector from the second, third, third, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yzzy(self) -> F32x4 {
        F32x4([self[1], self[2], self[2], self[1]])
    }

    /// Constructs a new vector from the third, third, third, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zzzy(self) -> F32x4 {
        F32x4([self[2], self[2], self[2], self[1]])
    }

    /// Constructs a new vector from the fourth, third, third, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wzzy(self) -> F32x4 {
        F32x4([self[3], self[2], self[2], self[1]])
    }

    /// Constructs a new vector from the first, fourth, third, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xwzy(self) -> F32x4 {
        F32x4([self[0], self[3], self[2], self[1]])
    }

    /// Constructs a new vector from the second, fourth, third, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn ywzy(self) -> F32x4 {
        F32x4([self[1], self[3], self[2], self[1]])
    }

    /// Constructs a new vector from the third, fourth, third, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zwzy(self) -> F32x4 {
        F32x4([self[2], self[3], self[2], self[1]])
    }

    /// Constructs a new vector from the fourth, fourth, third, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wwzy(self) -> F32x4 {
        F32x4([self[3], self[3], self[2], self[1]])
    }

    /// Constructs a new vector from the first, first, fourth, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xxwy(self) -> F32x4 {
        F32x4([self[0], self[0], self[3], self[1]])
    }

    /// Constructs a new vector from the second, first, fourth, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yxwy(self) -> F32x4 {
        F32x4([self[1], self[0], self[3], self[1]])
    }

    /// Constructs a new vector from the third, first, fourth, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zxwy(self) -> F32x4 {
        F32x4([self[2], self[0], self[3], self[1]])
    }

    /// Constructs a new vector from the fourth, first, fourth, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wxwy(self) -> F32x4 {
        F32x4([self[3], self[0], self[3], self[1]])
    }

    /// Constructs a new vector from the first, second, fourth, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xywy(self) -> F32x4 {
        F32x4([self[0], self[1], self[3], self[1]])
    }

    /// Constructs a new vector from the second, second, fourth, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yywy(self) -> F32x4 {
        F32x4([self[1], self[1], self[3], self[1]])
    }

    /// Constructs a new vector from the third, second, fourth, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zywy(self) -> F32x4 {
        F32x4([self[2], self[1], self[3], self[1]])
    }

    /// Constructs a new vector from the fourth, second, fourth, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wywy(self) -> F32x4 {
        F32x4([self[3], self[1], self[3], self[1]])
    }

    /// Constructs a new vector from the first, third, fourth, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xzwy(self) -> F32x4 {
        F32x4([self[0], self[2], self[3], self[1]])
    }

    /// Constructs a new vector from the second, third, fourth, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yzwy(self) -> F32x4 {
        F32x4([self[1], self[2], self[3], self[1]])
    }

    /// Constructs a new vector from the third, third, fourth, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zzwy(self) -> F32x4 {
        F32x4([self[2], self[2], self[3], self[1]])
    }

    /// Constructs a new vector from the fourth, third, fourth, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wzwy(self) -> F32x4 {
        F32x4([self[3], self[2], self[3], self[1]])
    }

    /// Constructs a new vector from the first, fourth, fourth, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xwwy(self) -> F32x4 {
        F32x4([self[0], self[3], self[3], self[1]])
    }

    /// Constructs a new vector from the second, fourth, fourth, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn ywwy(self) -> F32x4 {
        F32x4([self[1], self[3], self[3], self[1]])
    }

    /// Constructs a new vector from the third, fourth, fourth, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zwwy(self) -> F32x4 {
        F32x4([self[2], self[3], self[3], self[1]])
    }

    /// Constructs a new vector from the fourth, fourth, fourth, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wwwy(self) -> F32x4 {
        F32x4([self[3], self[3], self[3], self[1]])
    }

    /// Constructs a new vector from the first, first, first, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xxxz(self) -> F32x4 {
        F32x4([self[0], self[0], self[0], self[2]])
    }

    /// Constructs a new vector from the second, first, first, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yxxz(self) -> F32x4 {
        F32x4([self[1], self[0], self[0], self[2]])
    }

    /// Constructs a new vector from the third, first, first, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zxxz(self) -> F32x4 {
        F32x4([self[2], self[0], self[0], self[2]])
    }

    /// Constructs a new vector from the fourth, first, first, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wxxz(self) -> F32x4 {
        F32x4([self[3], self[0], self[0], self[2]])
    }

    /// Constructs a new vector from the first, second, first, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xyxz(self) -> F32x4 {
        F32x4([self[0], self[1], self[0], self[2]])
    }

    /// Constructs a new vector from the second, second, first, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yyxz(self) -> F32x4 {
        F32x4([self[1], self[1], self[0], self[2]])
    }

    /// Constructs a new vector from the third, second, first, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zyxz(self) -> F32x4 {
        F32x4([self[2], self[1], self[0], self[2]])
    }

    /// Constructs a new vector from the fourth, second, first, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wyxz(self) -> F32x4 {
        F32x4([self[3], self[1], self[0], self[2]])
    }

    /// Constructs a new vector from the first, third, first, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xzxz(self) -> F32x4 {
        F32x4([self[0], self[2], self[0], self[2]])
    }

    /// Constructs a new vector from the second, third, first, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yzxz(self) -> F32x4 {
        F32x4([self[1], self[2], self[0], self[2]])
    }

    /// Constructs a new vector from the third, third, first, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zzxz(self) -> F32x4 {
        F32x4([self[2], self[2], self[0], self[2]])
    }

    /// Constructs a new vector from the fourth, third, first, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wzxz(self) -> F32x4 {
        F32x4([self[3], self[2], self[0], self[2]])
    }

    /// Constructs a new vector from the first, fourth, first, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xwxz(self) -> F32x4 {
        F32x4([self[0], self[3], self[0], self[2]])
    }

    /// Constructs a new vector from the second, fourth, first, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn ywxz(self) -> F32x4 {
        F32x4([self[1], self[3], self[0], self[2]])
    }

    /// Constructs a new vector from the third, fourth, first, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zwxz(self) -> F32x4 {
        F32x4([self[2], self[3], self[0], self[2]])
    }

    /// Constructs a new vector from the fourth, fourth, first, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wwxz(self) -> F32x4 {
        F32x4([self[3], self[3], self[0], self[2]])
    }

    /// Constructs a new vector from the first, first, second, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xxyz(self) -> F32x4 {
        F32x4([self[0], self[0], self[1], self[2]])
    }

    /// Constructs a new vector from the second, first, second, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yxyz(self) -> F32x4 {
        F32x4([self[1], self[0], self[1], self[2]])
    }

    /// Constructs a new vector from the third, first, second, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zxyz(self) -> F32x4 {
        F32x4([self[2], self[0], self[1], self[2]])
    }

    /// Constructs a new vector from the fourth, first, second, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wxyz(self) -> F32x4 {
        F32x4([self[3], self[0], self[1], self[2]])
    }

    /// Constructs a new vector from the first, second, second, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xyyz(self) -> F32x4 {
        F32x4([self[0], self[1], self[1], self[2]])
    }

    /// Constructs a new vector from the second, second, second, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yyyz(self) -> F32x4 {
        F32x4([self[1], self[1], self[1], self[2]])
    }

    /// Constructs a new vector from the third, second, second, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zyyz(self) -> F32x4 {
        F32x4([self[2], self[1], self[1], self[2]])
    }

    /// Constructs a new vector from the fourth, second, second, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wyyz(self) -> F32x4 {
        F32x4([self[3], self[1], self[1], self[2]])
    }

    /// Constructs a new vector from the first, third, second, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xzyz(self) -> F32x4 {
        F32x4([self[0], self[2], self[1], self[2]])
    }

    /// Constructs a new vector from the second, third, second, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yzyz(self) -> F32x4 {
        F32x4([self[1], self[2], self[1], self[2]])
    }

    /// Constructs a new vector from the third, third, second, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zzyz(self) -> F32x4 {
        F32x4([self[2], self[2], self[1], self[2]])
    }

    /// Constructs a new vector from the fourth, third, second, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wzyz(self) -> F32x4 {
        F32x4([self[3], self[2], self[1], self[2]])
    }

    /// Constructs a new vector from the first, fourth, second, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xwyz(self) -> F32x4 {
        F32x4([self[0], self[3], self[1], self[2]])
    }

    /// Constructs a new vector from the second, fourth, second, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn ywyz(self) -> F32x4 {
        F32x4([self[1], self[3], self[1], self[2]])
    }

    /// Constructs a new vector from the third, fourth, second, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zwyz(self) -> F32x4 {
        F32x4([self[2], self[3], self[1], self[2]])
    }

    /// Constructs a new vector from the fourth, fourth, second, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wwyz(self) -> F32x4 {
        F32x4([self[3], self[3], self[1], self[2]])
    }

    /// Constructs a new vector from the first, first, third, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xxzz(self) -> F32x4 {
        F32x4([self[0], self[0], self[2], self[2]])
    }

    /// Constructs a new vector from the second, first, third, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yxzz(self) -> F32x4 {
        F32x4([self[1], self[0], self[2], self[2]])
    }

    /// Constructs a new vector from the third, first, third, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zxzz(self) -> F32x4 {
        F32x4([self[2], self[0], self[2], self[2]])
    }

    /// Constructs a new vector from the fourth, first, third, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wxzz(self) -> F32x4 {
        F32x4([self[3], self[0], self[2], self[2]])
    }

    /// Constructs a new vector from the first, second, third, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xyzz(self) -> F32x4 {
        F32x4([self[0], self[1], self[2], self[2]])
    }

    /// Constructs a new vector from the second, second, third, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yyzz(self) -> F32x4 {
        F32x4([self[1], self[1], self[2], self[2]])
    }

    /// Constructs a new vector from the third, second, third, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zyzz(self) -> F32x4 {
        F32x4([self[2], self[1], self[2], self[2]])
    }

    /// Constructs a new vector from the fourth, second, third, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wyzz(self) -> F32x4 {
        F32x4([self[3], self[1], self[2], self[2]])
    }

    /// Constructs a new vector from the first, third, third, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xzzz(self) -> F32x4 {
        F32x4([self[0], self[2], self[2], self[2]])
    }

    /// Constructs a new vector from the second, third, third, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yzzz(self) -> F32x4 {
        F32x4([self[1], self[2], self[2], self[2]])
    }

    /// Constructs a new vector from the third, third, third, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zzzz(self) -> F32x4 {
        F32x4([self[2], self[2], self[2], self[2]])
    }

    /// Constructs a new vector from the fourth, third, third, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wzzz(self) -> F32x4 {
        F32x4([self[3], self[2], self[2], self[2]])
    }

    /// Constructs a new vector from the first, fourth, third, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xwzz(self) -> F32x4 {
        F32x4([self[0], self[3], self[2], self[2]])
    }

    /// Constructs a new vector from the second, fourth, third, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn ywzz(self) -> F32x4 {
        F32x4([self[1], self[3], self[2], self[2]])
    }

    /// Constructs a new vector from the third, fourth, third, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zwzz(self) -> F32x4 {
        F32x4([self[2], self[3], self[2], self[2]])
    }

    /// Constructs a new vector from the fourth, fourth, third, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wwzz(self) -> F32x4 {
        F32x4([self[3], self[3], self[2], self[2]])
    }

    /// Constructs a new vector from the first, first, fourth, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xxwz(self) -> F32x4 {
        F32x4([self[0], self[0], self[3], self[2]])
    }

    /// Constructs a new vector from the second, first, fourth, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yxwz(self) -> F32x4 {
        F32x4([self[1], self[0], self[3], self[2]])
    }

    /// Constructs a new vector from the third, first, fourth, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zxwz(self) -> F32x4 {
        F32x4([self[2], self[0], self[3], self[2]])
    }

    /// Constructs a new vector from the fourth, first, fourth, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wxwz(self) -> F32x4 {
        F32x4([self[3], self[0], self[3], self[2]])
    }

    /// Constructs a new vector from the first, second, fourth, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xywz(self) -> F32x4 {
        F32x4([self[0], self[1], self[3], self[2]])
    }

    /// Constructs a new vector from the second, second, fourth, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yywz(self) -> F32x4 {
        F32x4([self[1], self[1], self[3], self[2]])
    }

    /// Constructs a new vector from the third, second, fourth, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zywz(self) -> F32x4 {
        F32x4([self[2], self[1], self[3], self[2]])
    }

    /// Constructs a new vector from the fourth, second, fourth, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wywz(self) -> F32x4 {
        F32x4([self[3], self[1], self[3], self[2]])
    }

    /// Constructs a new vector from the first, third, fourth, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xzwz(self) -> F32x4 {
        F32x4([self[0], self[2], self[3], self[2]])
    }

    /// Constructs a new vector from the second, third, fourth, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yzwz(self) -> F32x4 {
        F32x4([self[1], self[2], self[3], self[2]])
    }

    /// Constructs a new vector from the third, third, fourth, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zzwz(self) -> F32x4 {
        F32x4([self[2], self[2], self[3], self[2]])
    }

    /// Constructs a new vector from the fourth, third, fourth, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wzwz(self) -> F32x4 {
        F32x4([self[3], self[2], self[3], self[2]])
    }

    /// Constructs a new vector from the first, fourth, fourth, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xwwz(self) -> F32x4 {
        F32x4([self[0], self[3], self[3], self[2]])
    }

    /// Constructs a new vector from the second, fourth, fourth, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn ywwz(self) -> F32x4 {
        F32x4([self[1], self[3], self[3], self[2]])
    }

    /// Constructs a new vector from the third, fourth, fourth, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zwwz(self) -> F32x4 {
        F32x4([self[2], self[3], self[3], self[2]])
    }

    /// Constructs a new vector from the fourth, fourth, fourth, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wwwz(self) -> F32x4 {
        F32x4([self[3], self[3], self[3], self[2]])
    }

    /// Constructs a new vector from the first, first, first, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xxxw(self) -> F32x4 {
        F32x4([self[0], self[0], self[0], self[3]])
    }

    /// Constructs a new vector from the second, first, first, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yxxw(self) -> F32x4 {
        F32x4([self[1], self[0], self[0], self[3]])
    }

    /// Constructs a new vector from the third, first, first, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zxxw(self) -> F32x4 {
        F32x4([self[2], self[0], self[0], self[3]])
    }

    /// Constructs a new vector from the fourth, first, first, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wxxw(self) -> F32x4 {
        F32x4([self[3], self[0], self[0], self[3]])
    }

    /// Constructs a new vector from the first, second, first, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xyxw(self) -> F32x4 {
        F32x4([self[0], self[1], self[0], self[3]])
    }

    /// Constructs a new vector from the second, second, first, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yyxw(self) -> F32x4 {
        F32x4([self[1], self[1], self[0], self[3]])
    }

    /// Constructs a new vector from the third, second, first, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zyxw(self) -> F32x4 {
        F32x4([self[2], self[1], self[0], self[3]])
    }

    /// Constructs a new vector from the fourth, second, first, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wyxw(self) -> F32x4 {
        F32x4([self[3], self[1], self[0], self[3]])
    }

    /// Constructs a new vector from the first, third, first, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xzxw(self) -> F32x4 {
        F32x4([self[0], self[2], self[0], self[3]])
    }

    /// Constructs a new vector from the second, third, first, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yzxw(self) -> F32x4 {
        F32x4([self[1], self[2], self[0], self[3]])
    }

    /// Constructs a new vector from the third, third, first, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zzxw(self) -> F32x4 {
        F32x4([self[2], self[2], self[0], self[3]])
    }

    /// Constructs a new vector from the fourth, third, first, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wzxw(self) -> F32x4 {
        F32x4([self[3], self[2], self[0], self[3]])
    }

    /// Constructs a new vector from the first, fourth, first, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xwxw(self) -> F32x4 {
        F32x4([self[0], self[3], self[0], self[3]])
    }

    /// Constructs a new vector from the second, fourth, first, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn ywxw(self) -> F32x4 {
        F32x4([self[1], self[3], self[0], self[3]])
    }

    /// Constructs a new vector from the third, fourth, first, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zwxw(self) -> F32x4 {
        F32x4([self[2], self[3], self[0], self[3]])
    }

    /// Constructs a new vector from the fourth, fourth, first, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wwxw(self) -> F32x4 {
        F32x4([self[3], self[3], self[0], self[3]])
    }

    /// Constructs a new vector from the first, first, second, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xxyw(self) -> F32x4 {
        F32x4([self[0], self[0], self[1], self[3]])
    }

    /// Constructs a new vector from the second, first, second, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yxyw(self) -> F32x4 {
        F32x4([self[1], self[0], self[1], self[3]])
    }

    /// Constructs a new vector from the third, first, second, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zxyw(self) -> F32x4 {
        F32x4([self[2], self[0], self[1], self[3]])
    }

    /// Constructs a new vector from the fourth, first, second, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wxyw(self) -> F32x4 {
        F32x4([self[3], self[0], self[1], self[3]])
    }

    /// Constructs a new vector from the first, second, second, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xyyw(self) -> F32x4 {
        F32x4([self[0], self[1], self[1], self[3]])
    }

    /// Constructs a new vector from the second, second, second, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yyyw(self) -> F32x4 {
        F32x4([self[1], self[1], self[1], self[3]])
    }

    /// Constructs a new vector from the third, second, second, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zyyw(self) -> F32x4 {
        F32x4([self[2], self[1], self[1], self[3]])
    }

    /// Constructs a new vector from the fourth, second, second, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wyyw(self) -> F32x4 {
        F32x4([self[3], self[1], self[1], self[3]])
    }

    /// Constructs a new vector from the first, third, second, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xzyw(self) -> F32x4 {
        F32x4([self[0], self[2], self[1], self[3]])
    }

    /// Constructs a new vector from the second, third, second, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yzyw(self) -> F32x4 {
        F32x4([self[1], self[2], self[1], self[3]])
    }

    /// Constructs a new vector from the third, third, second, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zzyw(self) -> F32x4 {
        F32x4([self[2], self[2], self[1], self[3]])
    }

    /// Constructs a new vector from the fourth, third, second, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wzyw(self) -> F32x4 {
        F32x4([self[3], self[2], self[1], self[3]])
    }

    /// Constructs a new vector from the first, fourth, second, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xwyw(self) -> F32x4 {
        F32x4([self[0], self[3], self[1], self[3]])
    }

    /// Constructs a new vector from the second, fourth, second, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn ywyw(self) -> F32x4 {
        F32x4([self[1], self[3], self[1], self[3]])
    }

    /// Constructs a new vector from the third, fourth, second, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zwyw(self) -> F32x4 {
        F32x4([self[2], self[3], self[1], self[3]])
    }

    /// Constructs a new vector from the fourth, fourth, second, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wwyw(self) -> F32x4 {
        F32x4([self[3], self[3], self[1], self[3]])
    }

    /// Constructs a new vector from the first, first, third, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xxzw(self) -> F32x4 {
        F32x4([self[0], self[0], self[2], self[3]])
    }

    /// Constructs a new vector from the second, first, third, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yxzw(self) -> F32x4 {
        F32x4([self[1], self[0], self[2], self[3]])
    }

    /// Constructs a new vector from the third, first, third, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zxzw(self) -> F32x4 {
        F32x4([self[2], self[0], self[2], self[3]])
    }

    /// Constructs a new vector from the fourth, first, third, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wxzw(self) -> F32x4 {
        F32x4([self[3], self[0], self[2], self[3]])
    }

    /// Constructs a new vector from the first, second, third, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xyzw(self) -> F32x4 {
        F32x4([self[0], self[1], self[2], self[3]])
    }

    /// Constructs a new vector from the second, second, third, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yyzw(self) -> F32x4 {
        F32x4([self[1], self[1], self[2], self[3]])
    }

    /// Constructs a new vector from the third, second, third, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zyzw(self) -> F32x4 {
        F32x4([self[2], self[1], self[2], self[3]])
    }

    /// Constructs a new vector from the fourth, second, third, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wyzw(self) -> F32x4 {
        F32x4([self[3], self[1], self[2], self[3]])
    }

    /// Constructs a new vector from the first, third, third, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xzzw(self) -> F32x4 {
        F32x4([self[0], self[2], self[2], self[3]])
    }

    /// Constructs a new vector from the second, third, third, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yzzw(self) -> F32x4 {
        F32x4([self[1], self[2], self[2], self[3]])
    }

    /// Constructs a new vector from the third, third, third, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zzzw(self) -> F32x4 {
        F32x4([self[2], self[2], self[2], self[3]])
    }

    /// Constructs a new vector from the fourth, third, third, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wzzw(self) -> F32x4 {
        F32x4([self[3], self[2], self[2], self[3]])
    }

    /// Constructs a new vector from the first, fourth, third, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xwzw(self) -> F32x4 {
        F32x4([self[0], self[3], self[2], self[3]])
    }

    /// Constructs a new vector from the second, fourth, third, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn ywzw(self) -> F32x4 {
        F32x4([self[1], self[3], self[2], self[3]])
    }

    /// Constructs a new vector from the third, fourth, third, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zwzw(self) -> F32x4 {
        F32x4([self[2], self[3], self[2], self[3]])
    }

    /// Constructs a new vector from the fourth, fourth, third, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wwzw(self) -> F32x4 {
        F32x4([self[3], self[3], self[2], self[3]])
    }

    /// Constructs a new vector from the first, first, fourth, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xxww(self) -> F32x4 {
        F32x4([self[0], self[0], self[3], self[3]])
    }

    /// Constructs a new vector from the second, first, fourth, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yxww(self) -> F32x4 {
        F32x4([self[1], self[0], self[3], self[3]])
    }

    /// Constructs a new vector from the third, first, fourth, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zxww(self) -> F32x4 {
        F32x4([self[2], self[0], self[3], self[3]])
    }

    /// Constructs a new vector from the fourth, first, fourth, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wxww(self) -> F32x4 {
        F32x4([self[3], self[0], self[3], self[3]])
    }

    /// Constructs a new vector from the first, second, fourth, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xyww(self) -> F32x4 {
        F32x4([self[0], self[1], self[3], self[3]])
    }

    /// Constructs a new vector from the second, second, fourth, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yyww(self) -> F32x4 {
        F32x4([self[1], self[1], self[3], self[3]])
    }

    /// Constructs a new vector from the third, second, fourth, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zyww(self) -> F32x4 {
        F32x4([self[2], self[1], self[3], self[3]])
    }

    /// Constructs a new vector from the fourth, second, fourth, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wyww(self) -> F32x4 {
        F32x4([self[3], self[1], self[3], self[3]])
    }

    /// Constructs a new vector from the first, third, fourth, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xzww(self) -> F32x4 {
        F32x4([self[0], self[2], self[3], self[3]])
    }

    /// Constructs a new vector from the second, third, fourth, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yzww(self) -> F32x4 {
        F32x4([self[1], self[2], self[3], self[3]])
    }

    /// Constructs a new vector from the third, third, fourth, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zzww(self) -> F32x4 {
        F32x4([self[2], self[2], self[3], self[3]])
    }

    /// Constructs a new vector from the fourth, third, fourth, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wzww(self) -> F32x4 {
        F32x4([self[3], self[2], self[3], self[3]])
    }

    /// Constructs a new vector from the first, fourth, fourth, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xwww(self) -> F32x4 {
        F32x4([self[0], self[3], self[3], self[3]])
    }

    /// Constructs a new vector from the second, fourth, fourth, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn ywww(self) -> F32x4 {
        F32x4([self[1], self[3], self[3], self[3]])
    }

    /// Constructs a new vector from the third, fourth, fourth, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zwww(self) -> F32x4 {
        F32x4([self[2], self[3], self[3], self[3]])
    }

    /// Constructs a new vector from the fourth, fourth, fourth, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wwww(self) -> F32x4 {
        F32x4([self[3], self[3], self[3], self[3]])
    }
}
