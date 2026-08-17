// pathfinder/simd/src/scalar/swizzle_i32x4.rs
//
// Copyright © 2019 The Pathfinder Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::scalar::I32x4;

impl I32x4 {
    /// Constructs a new vector from the first, first, first, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xxxx(self) -> I32x4 {
        I32x4([self[0], self[0], self[0], self[0]])
    }

    /// Constructs a new vector from the second, first, first, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yxxx(self) -> I32x4 {
        I32x4([self[1], self[0], self[0], self[0]])
    }

    /// Constructs a new vector from the third, first, first, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zxxx(self) -> I32x4 {
        I32x4([self[2], self[0], self[0], self[0]])
    }

    /// Constructs a new vector from the fourth, first, first, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wxxx(self) -> I32x4 {
        I32x4([self[3], self[0], self[0], self[0]])
    }

    /// Constructs a new vector from the first, second, first, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xyxx(self) -> I32x4 {
        I32x4([self[0], self[1], self[0], self[0]])
    }

    /// Constructs a new vector from the second, second, first, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yyxx(self) -> I32x4 {
        I32x4([self[1], self[1], self[0], self[0]])
    }

    /// Constructs a new vector from the third, second, first, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zyxx(self) -> I32x4 {
        I32x4([self[2], self[1], self[0], self[0]])
    }

    /// Constructs a new vector from the fourth, second, first, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wyxx(self) -> I32x4 {
        I32x4([self[3], self[1], self[0], self[0]])
    }

    /// Constructs a new vector from the first, third, first, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xzxx(self) -> I32x4 {
        I32x4([self[0], self[2], self[0], self[0]])
    }

    /// Constructs a new vector from the second, third, first, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yzxx(self) -> I32x4 {
        I32x4([self[1], self[2], self[0], self[0]])
    }

    /// Constructs a new vector from the third, third, first, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zzxx(self) -> I32x4 {
        I32x4([self[2], self[2], self[0], self[0]])
    }

    /// Constructs a new vector from the fourth, third, first, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wzxx(self) -> I32x4 {
        I32x4([self[3], self[2], self[0], self[0]])
    }

    /// Constructs a new vector from the first, fourth, first, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xwxx(self) -> I32x4 {
        I32x4([self[0], self[3], self[0], self[0]])
    }

    /// Constructs a new vector from the second, fourth, first, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn ywxx(self) -> I32x4 {
        I32x4([self[1], self[3], self[0], self[0]])
    }

    /// Constructs a new vector from the third, fourth, first, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zwxx(self) -> I32x4 {
        I32x4([self[2], self[3], self[0], self[0]])
    }

    /// Constructs a new vector from the fourth, fourth, first, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wwxx(self) -> I32x4 {
        I32x4([self[3], self[3], self[0], self[0]])
    }

    /// Constructs a new vector from the first, first, second, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xxyx(self) -> I32x4 {
        I32x4([self[0], self[0], self[1], self[0]])
    }

    /// Constructs a new vector from the second, first, second, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yxyx(self) -> I32x4 {
        I32x4([self[1], self[0], self[1], self[0]])
    }

    /// Constructs a new vector from the third, first, second, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zxyx(self) -> I32x4 {
        I32x4([self[2], self[0], self[1], self[0]])
    }

    /// Constructs a new vector from the fourth, first, second, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wxyx(self) -> I32x4 {
        I32x4([self[3], self[0], self[1], self[0]])
    }

    /// Constructs a new vector from the first, second, second, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xyyx(self) -> I32x4 {
        I32x4([self[0], self[1], self[1], self[0]])
    }

    /// Constructs a new vector from the second, second, second, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yyyx(self) -> I32x4 {
        I32x4([self[1], self[1], self[1], self[0]])
    }

    /// Constructs a new vector from the third, second, second, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zyyx(self) -> I32x4 {
        I32x4([self[2], self[1], self[1], self[0]])
    }

    /// Constructs a new vector from the fourth, second, second, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wyyx(self) -> I32x4 {
        I32x4([self[3], self[1], self[1], self[0]])
    }

    /// Constructs a new vector from the first, third, second, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xzyx(self) -> I32x4 {
        I32x4([self[0], self[2], self[1], self[0]])
    }

    /// Constructs a new vector from the second, third, second, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yzyx(self) -> I32x4 {
        I32x4([self[1], self[2], self[1], self[0]])
    }

    /// Constructs a new vector from the third, third, second, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zzyx(self) -> I32x4 {
        I32x4([self[2], self[2], self[1], self[0]])
    }

    /// Constructs a new vector from the fourth, third, second, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wzyx(self) -> I32x4 {
        I32x4([self[3], self[2], self[1], self[0]])
    }

    /// Constructs a new vector from the first, fourth, second, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xwyx(self) -> I32x4 {
        I32x4([self[0], self[3], self[1], self[0]])
    }

    /// Constructs a new vector from the second, fourth, second, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn ywyx(self) -> I32x4 {
        I32x4([self[1], self[3], self[1], self[0]])
    }

    /// Constructs a new vector from the third, fourth, second, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zwyx(self) -> I32x4 {
        I32x4([self[2], self[3], self[1], self[0]])
    }

    /// Constructs a new vector from the fourth, fourth, second, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wwyx(self) -> I32x4 {
        I32x4([self[3], self[3], self[1], self[0]])
    }

    /// Constructs a new vector from the first, first, third, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xxzx(self) -> I32x4 {
        I32x4([self[0], self[0], self[2], self[0]])
    }

    /// Constructs a new vector from the second, first, third, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yxzx(self) -> I32x4 {
        I32x4([self[1], self[0], self[2], self[0]])
    }

    /// Constructs a new vector from the third, first, third, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zxzx(self) -> I32x4 {
        I32x4([self[2], self[0], self[2], self[0]])
    }

    /// Constructs a new vector from the fourth, first, third, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wxzx(self) -> I32x4 {
        I32x4([self[3], self[0], self[2], self[0]])
    }

    /// Constructs a new vector from the first, second, third, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xyzx(self) -> I32x4 {
        I32x4([self[0], self[1], self[2], self[0]])
    }

    /// Constructs a new vector from the second, second, third, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yyzx(self) -> I32x4 {
        I32x4([self[1], self[1], self[2], self[0]])
    }

    /// Constructs a new vector from the third, second, third, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zyzx(self) -> I32x4 {
        I32x4([self[2], self[1], self[2], self[0]])
    }

    /// Constructs a new vector from the fourth, second, third, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wyzx(self) -> I32x4 {
        I32x4([self[3], self[1], self[2], self[0]])
    }

    /// Constructs a new vector from the first, third, third, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xzzx(self) -> I32x4 {
        I32x4([self[0], self[2], self[2], self[0]])
    }

    /// Constructs a new vector from the second, third, third, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yzzx(self) -> I32x4 {
        I32x4([self[1], self[2], self[2], self[0]])
    }

    /// Constructs a new vector from the third, third, third, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zzzx(self) -> I32x4 {
        I32x4([self[2], self[2], self[2], self[0]])
    }

    /// Constructs a new vector from the fourth, third, third, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wzzx(self) -> I32x4 {
        I32x4([self[3], self[2], self[2], self[0]])
    }

    /// Constructs a new vector from the first, fourth, third, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xwzx(self) -> I32x4 {
        I32x4([self[0], self[3], self[2], self[0]])
    }

    /// Constructs a new vector from the second, fourth, third, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn ywzx(self) -> I32x4 {
        I32x4([self[1], self[3], self[2], self[0]])
    }

    /// Constructs a new vector from the third, fourth, third, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zwzx(self) -> I32x4 {
        I32x4([self[2], self[3], self[2], self[0]])
    }

    /// Constructs a new vector from the fourth, fourth, third, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wwzx(self) -> I32x4 {
        I32x4([self[3], self[3], self[2], self[0]])
    }

    /// Constructs a new vector from the first, first, fourth, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xxwx(self) -> I32x4 {
        I32x4([self[0], self[0], self[3], self[0]])
    }

    /// Constructs a new vector from the second, first, fourth, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yxwx(self) -> I32x4 {
        I32x4([self[1], self[0], self[3], self[0]])
    }

    /// Constructs a new vector from the third, first, fourth, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zxwx(self) -> I32x4 {
        I32x4([self[2], self[0], self[3], self[0]])
    }

    /// Constructs a new vector from the fourth, first, fourth, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wxwx(self) -> I32x4 {
        I32x4([self[3], self[0], self[3], self[0]])
    }

    /// Constructs a new vector from the first, second, fourth, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xywx(self) -> I32x4 {
        I32x4([self[0], self[1], self[3], self[0]])
    }

    /// Constructs a new vector from the second, second, fourth, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yywx(self) -> I32x4 {
        I32x4([self[1], self[1], self[3], self[0]])
    }

    /// Constructs a new vector from the third, second, fourth, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zywx(self) -> I32x4 {
        I32x4([self[2], self[1], self[3], self[0]])
    }

    /// Constructs a new vector from the fourth, second, fourth, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wywx(self) -> I32x4 {
        I32x4([self[3], self[1], self[3], self[0]])
    }

    /// Constructs a new vector from the first, third, fourth, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xzwx(self) -> I32x4 {
        I32x4([self[0], self[2], self[3], self[0]])
    }

    /// Constructs a new vector from the second, third, fourth, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yzwx(self) -> I32x4 {
        I32x4([self[1], self[2], self[3], self[0]])
    }

    /// Constructs a new vector from the third, third, fourth, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zzwx(self) -> I32x4 {
        I32x4([self[2], self[2], self[3], self[0]])
    }

    /// Constructs a new vector from the fourth, third, fourth, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wzwx(self) -> I32x4 {
        I32x4([self[3], self[2], self[3], self[0]])
    }

    /// Constructs a new vector from the first, fourth, fourth, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xwwx(self) -> I32x4 {
        I32x4([self[0], self[3], self[3], self[0]])
    }

    /// Constructs a new vector from the second, fourth, fourth, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn ywwx(self) -> I32x4 {
        I32x4([self[1], self[3], self[3], self[0]])
    }

    /// Constructs a new vector from the third, fourth, fourth, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zwwx(self) -> I32x4 {
        I32x4([self[2], self[3], self[3], self[0]])
    }

    /// Constructs a new vector from the fourth, fourth, fourth, and first
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wwwx(self) -> I32x4 {
        I32x4([self[3], self[3], self[3], self[0]])
    }

    /// Constructs a new vector from the first, first, first, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xxxy(self) -> I32x4 {
        I32x4([self[0], self[0], self[0], self[1]])
    }

    /// Constructs a new vector from the second, first, first, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yxxy(self) -> I32x4 {
        I32x4([self[1], self[0], self[0], self[1]])
    }

    /// Constructs a new vector from the third, first, first, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zxxy(self) -> I32x4 {
        I32x4([self[2], self[0], self[0], self[1]])
    }

    /// Constructs a new vector from the fourth, first, first, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wxxy(self) -> I32x4 {
        I32x4([self[3], self[0], self[0], self[1]])
    }

    /// Constructs a new vector from the first, second, first, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xyxy(self) -> I32x4 {
        I32x4([self[0], self[1], self[0], self[1]])
    }

    /// Constructs a new vector from the second, second, first, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yyxy(self) -> I32x4 {
        I32x4([self[1], self[1], self[0], self[1]])
    }

    /// Constructs a new vector from the third, second, first, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zyxy(self) -> I32x4 {
        I32x4([self[2], self[1], self[0], self[1]])
    }

    /// Constructs a new vector from the fourth, second, first, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wyxy(self) -> I32x4 {
        I32x4([self[3], self[1], self[0], self[1]])
    }

    /// Constructs a new vector from the first, third, first, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xzxy(self) -> I32x4 {
        I32x4([self[0], self[2], self[0], self[1]])
    }

    /// Constructs a new vector from the second, third, first, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yzxy(self) -> I32x4 {
        I32x4([self[1], self[2], self[0], self[1]])
    }

    /// Constructs a new vector from the third, third, first, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zzxy(self) -> I32x4 {
        I32x4([self[2], self[2], self[0], self[1]])
    }

    /// Constructs a new vector from the fourth, third, first, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wzxy(self) -> I32x4 {
        I32x4([self[3], self[2], self[0], self[1]])
    }

    /// Constructs a new vector from the first, fourth, first, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xwxy(self) -> I32x4 {
        I32x4([self[0], self[3], self[0], self[1]])
    }

    /// Constructs a new vector from the second, fourth, first, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn ywxy(self) -> I32x4 {
        I32x4([self[1], self[3], self[0], self[1]])
    }

    /// Constructs a new vector from the third, fourth, first, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zwxy(self) -> I32x4 {
        I32x4([self[2], self[3], self[0], self[1]])
    }

    /// Constructs a new vector from the fourth, fourth, first, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wwxy(self) -> I32x4 {
        I32x4([self[3], self[3], self[0], self[1]])
    }

    /// Constructs a new vector from the first, first, second, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xxyy(self) -> I32x4 {
        I32x4([self[0], self[0], self[1], self[1]])
    }

    /// Constructs a new vector from the second, first, second, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yxyy(self) -> I32x4 {
        I32x4([self[1], self[0], self[1], self[1]])
    }

    /// Constructs a new vector from the third, first, second, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zxyy(self) -> I32x4 {
        I32x4([self[2], self[0], self[1], self[1]])
    }

    /// Constructs a new vector from the fourth, first, second, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wxyy(self) -> I32x4 {
        I32x4([self[3], self[0], self[1], self[1]])
    }

    /// Constructs a new vector from the first, second, second, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xyyy(self) -> I32x4 {
        I32x4([self[0], self[1], self[1], self[1]])
    }

    /// Constructs a new vector from the second, second, second, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yyyy(self) -> I32x4 {
        I32x4([self[1], self[1], self[1], self[1]])
    }

    /// Constructs a new vector from the third, second, second, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zyyy(self) -> I32x4 {
        I32x4([self[2], self[1], self[1], self[1]])
    }

    /// Constructs a new vector from the fourth, second, second, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wyyy(self) -> I32x4 {
        I32x4([self[3], self[1], self[1], self[1]])
    }

    /// Constructs a new vector from the first, third, second, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xzyy(self) -> I32x4 {
        I32x4([self[0], self[2], self[1], self[1]])
    }

    /// Constructs a new vector from the second, third, second, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yzyy(self) -> I32x4 {
        I32x4([self[1], self[2], self[1], self[1]])
    }

    /// Constructs a new vector from the third, third, second, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zzyy(self) -> I32x4 {
        I32x4([self[2], self[2], self[1], self[1]])
    }

    /// Constructs a new vector from the fourth, third, second, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wzyy(self) -> I32x4 {
        I32x4([self[3], self[2], self[1], self[1]])
    }

    /// Constructs a new vector from the first, fourth, second, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xwyy(self) -> I32x4 {
        I32x4([self[0], self[3], self[1], self[1]])
    }

    /// Constructs a new vector from the second, fourth, second, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn ywyy(self) -> I32x4 {
        I32x4([self[1], self[3], self[1], self[1]])
    }

    /// Constructs a new vector from the third, fourth, second, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zwyy(self) -> I32x4 {
        I32x4([self[2], self[3], self[1], self[1]])
    }

    /// Constructs a new vector from the fourth, fourth, second, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wwyy(self) -> I32x4 {
        I32x4([self[3], self[3], self[1], self[1]])
    }

    /// Constructs a new vector from the first, first, third, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xxzy(self) -> I32x4 {
        I32x4([self[0], self[0], self[2], self[1]])
    }

    /// Constructs a new vector from the second, first, third, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yxzy(self) -> I32x4 {
        I32x4([self[1], self[0], self[2], self[1]])
    }

    /// Constructs a new vector from the third, first, third, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zxzy(self) -> I32x4 {
        I32x4([self[2], self[0], self[2], self[1]])
    }

    /// Constructs a new vector from the fourth, first, third, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wxzy(self) -> I32x4 {
        I32x4([self[3], self[0], self[2], self[1]])
    }

    /// Constructs a new vector from the first, second, third, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xyzy(self) -> I32x4 {
        I32x4([self[0], self[1], self[2], self[1]])
    }

    /// Constructs a new vector from the second, second, third, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yyzy(self) -> I32x4 {
        I32x4([self[1], self[1], self[2], self[1]])
    }

    /// Constructs a new vector from the third, second, third, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zyzy(self) -> I32x4 {
        I32x4([self[2], self[1], self[2], self[1]])
    }

    /// Constructs a new vector from the fourth, second, third, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wyzy(self) -> I32x4 {
        I32x4([self[3], self[1], self[2], self[1]])
    }

    /// Constructs a new vector from the first, third, third, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xzzy(self) -> I32x4 {
        I32x4([self[0], self[2], self[2], self[1]])
    }

    /// Constructs a new vector from the second, third, third, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yzzy(self) -> I32x4 {
        I32x4([self[1], self[2], self[2], self[1]])
    }

    /// Constructs a new vector from the third, third, third, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zzzy(self) -> I32x4 {
        I32x4([self[2], self[2], self[2], self[1]])
    }

    /// Constructs a new vector from the fourth, third, third, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wzzy(self) -> I32x4 {
        I32x4([self[3], self[2], self[2], self[1]])
    }

    /// Constructs a new vector from the first, fourth, third, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xwzy(self) -> I32x4 {
        I32x4([self[0], self[3], self[2], self[1]])
    }

    /// Constructs a new vector from the second, fourth, third, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn ywzy(self) -> I32x4 {
        I32x4([self[1], self[3], self[2], self[1]])
    }

    /// Constructs a new vector from the third, fourth, third, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zwzy(self) -> I32x4 {
        I32x4([self[2], self[3], self[2], self[1]])
    }

    /// Constructs a new vector from the fourth, fourth, third, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wwzy(self) -> I32x4 {
        I32x4([self[3], self[3], self[2], self[1]])
    }

    /// Constructs a new vector from the first, first, fourth, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xxwy(self) -> I32x4 {
        I32x4([self[0], self[0], self[3], self[1]])
    }

    /// Constructs a new vector from the second, first, fourth, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yxwy(self) -> I32x4 {
        I32x4([self[1], self[0], self[3], self[1]])
    }

    /// Constructs a new vector from the third, first, fourth, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zxwy(self) -> I32x4 {
        I32x4([self[2], self[0], self[3], self[1]])
    }

    /// Constructs a new vector from the fourth, first, fourth, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wxwy(self) -> I32x4 {
        I32x4([self[3], self[0], self[3], self[1]])
    }

    /// Constructs a new vector from the first, second, fourth, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xywy(self) -> I32x4 {
        I32x4([self[0], self[1], self[3], self[1]])
    }

    /// Constructs a new vector from the second, second, fourth, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yywy(self) -> I32x4 {
        I32x4([self[1], self[1], self[3], self[1]])
    }

    /// Constructs a new vector from the third, second, fourth, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zywy(self) -> I32x4 {
        I32x4([self[2], self[1], self[3], self[1]])
    }

    /// Constructs a new vector from the fourth, second, fourth, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wywy(self) -> I32x4 {
        I32x4([self[3], self[1], self[3], self[1]])
    }

    /// Constructs a new vector from the first, third, fourth, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xzwy(self) -> I32x4 {
        I32x4([self[0], self[2], self[3], self[1]])
    }

    /// Constructs a new vector from the second, third, fourth, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yzwy(self) -> I32x4 {
        I32x4([self[1], self[2], self[3], self[1]])
    }

    /// Constructs a new vector from the third, third, fourth, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zzwy(self) -> I32x4 {
        I32x4([self[2], self[2], self[3], self[1]])
    }

    /// Constructs a new vector from the fourth, third, fourth, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wzwy(self) -> I32x4 {
        I32x4([self[3], self[2], self[3], self[1]])
    }

    /// Constructs a new vector from the first, fourth, fourth, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xwwy(self) -> I32x4 {
        I32x4([self[0], self[3], self[3], self[1]])
    }

    /// Constructs a new vector from the second, fourth, fourth, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn ywwy(self) -> I32x4 {
        I32x4([self[1], self[3], self[3], self[1]])
    }

    /// Constructs a new vector from the third, fourth, fourth, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zwwy(self) -> I32x4 {
        I32x4([self[2], self[3], self[3], self[1]])
    }

    /// Constructs a new vector from the fourth, fourth, fourth, and second
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wwwy(self) -> I32x4 {
        I32x4([self[3], self[3], self[3], self[1]])
    }

    /// Constructs a new vector from the first, first, first, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xxxz(self) -> I32x4 {
        I32x4([self[0], self[0], self[0], self[2]])
    }

    /// Constructs a new vector from the second, first, first, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yxxz(self) -> I32x4 {
        I32x4([self[1], self[0], self[0], self[2]])
    }

    /// Constructs a new vector from the third, first, first, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zxxz(self) -> I32x4 {
        I32x4([self[2], self[0], self[0], self[2]])
    }

    /// Constructs a new vector from the fourth, first, first, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wxxz(self) -> I32x4 {
        I32x4([self[3], self[0], self[0], self[2]])
    }

    /// Constructs a new vector from the first, second, first, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xyxz(self) -> I32x4 {
        I32x4([self[0], self[1], self[0], self[2]])
    }

    /// Constructs a new vector from the second, second, first, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yyxz(self) -> I32x4 {
        I32x4([self[1], self[1], self[0], self[2]])
    }

    /// Constructs a new vector from the third, second, first, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zyxz(self) -> I32x4 {
        I32x4([self[2], self[1], self[0], self[2]])
    }

    /// Constructs a new vector from the fourth, second, first, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wyxz(self) -> I32x4 {
        I32x4([self[3], self[1], self[0], self[2]])
    }

    /// Constructs a new vector from the first, third, first, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xzxz(self) -> I32x4 {
        I32x4([self[0], self[2], self[0], self[2]])
    }

    /// Constructs a new vector from the second, third, first, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yzxz(self) -> I32x4 {
        I32x4([self[1], self[2], self[0], self[2]])
    }

    /// Constructs a new vector from the third, third, first, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zzxz(self) -> I32x4 {
        I32x4([self[2], self[2], self[0], self[2]])
    }

    /// Constructs a new vector from the fourth, third, first, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wzxz(self) -> I32x4 {
        I32x4([self[3], self[2], self[0], self[2]])
    }

    /// Constructs a new vector from the first, fourth, first, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xwxz(self) -> I32x4 {
        I32x4([self[0], self[3], self[0], self[2]])
    }

    /// Constructs a new vector from the second, fourth, first, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn ywxz(self) -> I32x4 {
        I32x4([self[1], self[3], self[0], self[2]])
    }

    /// Constructs a new vector from the third, fourth, first, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zwxz(self) -> I32x4 {
        I32x4([self[2], self[3], self[0], self[2]])
    }

    /// Constructs a new vector from the fourth, fourth, first, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wwxz(self) -> I32x4 {
        I32x4([self[3], self[3], self[0], self[2]])
    }

    /// Constructs a new vector from the first, first, second, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xxyz(self) -> I32x4 {
        I32x4([self[0], self[0], self[1], self[2]])
    }

    /// Constructs a new vector from the second, first, second, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yxyz(self) -> I32x4 {
        I32x4([self[1], self[0], self[1], self[2]])
    }

    /// Constructs a new vector from the third, first, second, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zxyz(self) -> I32x4 {
        I32x4([self[2], self[0], self[1], self[2]])
    }

    /// Constructs a new vector from the fourth, first, second, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wxyz(self) -> I32x4 {
        I32x4([self[3], self[0], self[1], self[2]])
    }

    /// Constructs a new vector from the first, second, second, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xyyz(self) -> I32x4 {
        I32x4([self[0], self[1], self[1], self[2]])
    }

    /// Constructs a new vector from the second, second, second, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yyyz(self) -> I32x4 {
        I32x4([self[1], self[1], self[1], self[2]])
    }

    /// Constructs a new vector from the third, second, second, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zyyz(self) -> I32x4 {
        I32x4([self[2], self[1], self[1], self[2]])
    }

    /// Constructs a new vector from the fourth, second, second, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wyyz(self) -> I32x4 {
        I32x4([self[3], self[1], self[1], self[2]])
    }

    /// Constructs a new vector from the first, third, second, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xzyz(self) -> I32x4 {
        I32x4([self[0], self[2], self[1], self[2]])
    }

    /// Constructs a new vector from the second, third, second, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yzyz(self) -> I32x4 {
        I32x4([self[1], self[2], self[1], self[2]])
    }

    /// Constructs a new vector from the third, third, second, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zzyz(self) -> I32x4 {
        I32x4([self[2], self[2], self[1], self[2]])
    }

    /// Constructs a new vector from the fourth, third, second, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wzyz(self) -> I32x4 {
        I32x4([self[3], self[2], self[1], self[2]])
    }

    /// Constructs a new vector from the first, fourth, second, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xwyz(self) -> I32x4 {
        I32x4([self[0], self[3], self[1], self[2]])
    }

    /// Constructs a new vector from the second, fourth, second, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn ywyz(self) -> I32x4 {
        I32x4([self[1], self[3], self[1], self[2]])
    }

    /// Constructs a new vector from the third, fourth, second, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zwyz(self) -> I32x4 {
        I32x4([self[2], self[3], self[1], self[2]])
    }

    /// Constructs a new vector from the fourth, fourth, second, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wwyz(self) -> I32x4 {
        I32x4([self[3], self[3], self[1], self[2]])
    }

    /// Constructs a new vector from the first, first, third, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xxzz(self) -> I32x4 {
        I32x4([self[0], self[0], self[2], self[2]])
    }

    /// Constructs a new vector from the second, first, third, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yxzz(self) -> I32x4 {
        I32x4([self[1], self[0], self[2], self[2]])
    }

    /// Constructs a new vector from the third, first, third, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zxzz(self) -> I32x4 {
        I32x4([self[2], self[0], self[2], self[2]])
    }

    /// Constructs a new vector from the fourth, first, third, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wxzz(self) -> I32x4 {
        I32x4([self[3], self[0], self[2], self[2]])
    }

    /// Constructs a new vector from the first, second, third, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xyzz(self) -> I32x4 {
        I32x4([self[0], self[1], self[2], self[2]])
    }

    /// Constructs a new vector from the second, second, third, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yyzz(self) -> I32x4 {
        I32x4([self[1], self[1], self[2], self[2]])
    }

    /// Constructs a new vector from the third, second, third, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zyzz(self) -> I32x4 {
        I32x4([self[2], self[1], self[2], self[2]])
    }

    /// Constructs a new vector from the fourth, second, third, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wyzz(self) -> I32x4 {
        I32x4([self[3], self[1], self[2], self[2]])
    }

    /// Constructs a new vector from the first, third, third, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xzzz(self) -> I32x4 {
        I32x4([self[0], self[2], self[2], self[2]])
    }

    /// Constructs a new vector from the second, third, third, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yzzz(self) -> I32x4 {
        I32x4([self[1], self[2], self[2], self[2]])
    }

    /// Constructs a new vector from the third, third, third, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zzzz(self) -> I32x4 {
        I32x4([self[2], self[2], self[2], self[2]])
    }

    /// Constructs a new vector from the fourth, third, third, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wzzz(self) -> I32x4 {
        I32x4([self[3], self[2], self[2], self[2]])
    }

    /// Constructs a new vector from the first, fourth, third, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xwzz(self) -> I32x4 {
        I32x4([self[0], self[3], self[2], self[2]])
    }

    /// Constructs a new vector from the second, fourth, third, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn ywzz(self) -> I32x4 {
        I32x4([self[1], self[3], self[2], self[2]])
    }

    /// Constructs a new vector from the third, fourth, third, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zwzz(self) -> I32x4 {
        I32x4([self[2], self[3], self[2], self[2]])
    }

    /// Constructs a new vector from the fourth, fourth, third, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wwzz(self) -> I32x4 {
        I32x4([self[3], self[3], self[2], self[2]])
    }

    /// Constructs a new vector from the first, first, fourth, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xxwz(self) -> I32x4 {
        I32x4([self[0], self[0], self[3], self[2]])
    }

    /// Constructs a new vector from the second, first, fourth, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yxwz(self) -> I32x4 {
        I32x4([self[1], self[0], self[3], self[2]])
    }

    /// Constructs a new vector from the third, first, fourth, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zxwz(self) -> I32x4 {
        I32x4([self[2], self[0], self[3], self[2]])
    }

    /// Constructs a new vector from the fourth, first, fourth, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wxwz(self) -> I32x4 {
        I32x4([self[3], self[0], self[3], self[2]])
    }

    /// Constructs a new vector from the first, second, fourth, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xywz(self) -> I32x4 {
        I32x4([self[0], self[1], self[3], self[2]])
    }

    /// Constructs a new vector from the second, second, fourth, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yywz(self) -> I32x4 {
        I32x4([self[1], self[1], self[3], self[2]])
    }

    /// Constructs a new vector from the third, second, fourth, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zywz(self) -> I32x4 {
        I32x4([self[2], self[1], self[3], self[2]])
    }

    /// Constructs a new vector from the fourth, second, fourth, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wywz(self) -> I32x4 {
        I32x4([self[3], self[1], self[3], self[2]])
    }

    /// Constructs a new vector from the first, third, fourth, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xzwz(self) -> I32x4 {
        I32x4([self[0], self[2], self[3], self[2]])
    }

    /// Constructs a new vector from the second, third, fourth, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yzwz(self) -> I32x4 {
        I32x4([self[1], self[2], self[3], self[2]])
    }

    /// Constructs a new vector from the third, third, fourth, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zzwz(self) -> I32x4 {
        I32x4([self[2], self[2], self[3], self[2]])
    }

    /// Constructs a new vector from the fourth, third, fourth, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wzwz(self) -> I32x4 {
        I32x4([self[3], self[2], self[3], self[2]])
    }

    /// Constructs a new vector from the first, fourth, fourth, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xwwz(self) -> I32x4 {
        I32x4([self[0], self[3], self[3], self[2]])
    }

    /// Constructs a new vector from the second, fourth, fourth, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn ywwz(self) -> I32x4 {
        I32x4([self[1], self[3], self[3], self[2]])
    }

    /// Constructs a new vector from the third, fourth, fourth, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zwwz(self) -> I32x4 {
        I32x4([self[2], self[3], self[3], self[2]])
    }

    /// Constructs a new vector from the fourth, fourth, fourth, and third
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wwwz(self) -> I32x4 {
        I32x4([self[3], self[3], self[3], self[2]])
    }

    /// Constructs a new vector from the first, first, first, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xxxw(self) -> I32x4 {
        I32x4([self[0], self[0], self[0], self[3]])
    }

    /// Constructs a new vector from the second, first, first, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yxxw(self) -> I32x4 {
        I32x4([self[1], self[0], self[0], self[3]])
    }

    /// Constructs a new vector from the third, first, first, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zxxw(self) -> I32x4 {
        I32x4([self[2], self[0], self[0], self[3]])
    }

    /// Constructs a new vector from the fourth, first, first, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wxxw(self) -> I32x4 {
        I32x4([self[3], self[0], self[0], self[3]])
    }

    /// Constructs a new vector from the first, second, first, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xyxw(self) -> I32x4 {
        I32x4([self[0], self[1], self[0], self[3]])
    }

    /// Constructs a new vector from the second, second, first, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yyxw(self) -> I32x4 {
        I32x4([self[1], self[1], self[0], self[3]])
    }

    /// Constructs a new vector from the third, second, first, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zyxw(self) -> I32x4 {
        I32x4([self[2], self[1], self[0], self[3]])
    }

    /// Constructs a new vector from the fourth, second, first, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wyxw(self) -> I32x4 {
        I32x4([self[3], self[1], self[0], self[3]])
    }

    /// Constructs a new vector from the first, third, first, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xzxw(self) -> I32x4 {
        I32x4([self[0], self[2], self[0], self[3]])
    }

    /// Constructs a new vector from the second, third, first, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yzxw(self) -> I32x4 {
        I32x4([self[1], self[2], self[0], self[3]])
    }

    /// Constructs a new vector from the third, third, first, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zzxw(self) -> I32x4 {
        I32x4([self[2], self[2], self[0], self[3]])
    }

    /// Constructs a new vector from the fourth, third, first, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wzxw(self) -> I32x4 {
        I32x4([self[3], self[2], self[0], self[3]])
    }

    /// Constructs a new vector from the first, fourth, first, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xwxw(self) -> I32x4 {
        I32x4([self[0], self[3], self[0], self[3]])
    }

    /// Constructs a new vector from the second, fourth, first, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn ywxw(self) -> I32x4 {
        I32x4([self[1], self[3], self[0], self[3]])
    }

    /// Constructs a new vector from the third, fourth, first, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zwxw(self) -> I32x4 {
        I32x4([self[2], self[3], self[0], self[3]])
    }

    /// Constructs a new vector from the fourth, fourth, first, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wwxw(self) -> I32x4 {
        I32x4([self[3], self[3], self[0], self[3]])
    }

    /// Constructs a new vector from the first, first, second, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xxyw(self) -> I32x4 {
        I32x4([self[0], self[0], self[1], self[3]])
    }

    /// Constructs a new vector from the second, first, second, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yxyw(self) -> I32x4 {
        I32x4([self[1], self[0], self[1], self[3]])
    }

    /// Constructs a new vector from the third, first, second, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zxyw(self) -> I32x4 {
        I32x4([self[2], self[0], self[1], self[3]])
    }

    /// Constructs a new vector from the fourth, first, second, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wxyw(self) -> I32x4 {
        I32x4([self[3], self[0], self[1], self[3]])
    }

    /// Constructs a new vector from the first, second, second, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xyyw(self) -> I32x4 {
        I32x4([self[0], self[1], self[1], self[3]])
    }

    /// Constructs a new vector from the second, second, second, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yyyw(self) -> I32x4 {
        I32x4([self[1], self[1], self[1], self[3]])
    }

    /// Constructs a new vector from the third, second, second, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zyyw(self) -> I32x4 {
        I32x4([self[2], self[1], self[1], self[3]])
    }

    /// Constructs a new vector from the fourth, second, second, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wyyw(self) -> I32x4 {
        I32x4([self[3], self[1], self[1], self[3]])
    }

    /// Constructs a new vector from the first, third, second, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xzyw(self) -> I32x4 {
        I32x4([self[0], self[2], self[1], self[3]])
    }

    /// Constructs a new vector from the second, third, second, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yzyw(self) -> I32x4 {
        I32x4([self[1], self[2], self[1], self[3]])
    }

    /// Constructs a new vector from the third, third, second, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zzyw(self) -> I32x4 {
        I32x4([self[2], self[2], self[1], self[3]])
    }

    /// Constructs a new vector from the fourth, third, second, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wzyw(self) -> I32x4 {
        I32x4([self[3], self[2], self[1], self[3]])
    }

    /// Constructs a new vector from the first, fourth, second, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xwyw(self) -> I32x4 {
        I32x4([self[0], self[3], self[1], self[3]])
    }

    /// Constructs a new vector from the second, fourth, second, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn ywyw(self) -> I32x4 {
        I32x4([self[1], self[3], self[1], self[3]])
    }

    /// Constructs a new vector from the third, fourth, second, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zwyw(self) -> I32x4 {
        I32x4([self[2], self[3], self[1], self[3]])
    }

    /// Constructs a new vector from the fourth, fourth, second, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wwyw(self) -> I32x4 {
        I32x4([self[3], self[3], self[1], self[3]])
    }

    /// Constructs a new vector from the first, first, third, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xxzw(self) -> I32x4 {
        I32x4([self[0], self[0], self[2], self[3]])
    }

    /// Constructs a new vector from the second, first, third, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yxzw(self) -> I32x4 {
        I32x4([self[1], self[0], self[2], self[3]])
    }

    /// Constructs a new vector from the third, first, third, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zxzw(self) -> I32x4 {
        I32x4([self[2], self[0], self[2], self[3]])
    }

    /// Constructs a new vector from the fourth, first, third, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wxzw(self) -> I32x4 {
        I32x4([self[3], self[0], self[2], self[3]])
    }

    /// Constructs a new vector from the first, second, third, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xyzw(self) -> I32x4 {
        I32x4([self[0], self[1], self[2], self[3]])
    }

    /// Constructs a new vector from the second, second, third, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yyzw(self) -> I32x4 {
        I32x4([self[1], self[1], self[2], self[3]])
    }

    /// Constructs a new vector from the third, second, third, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zyzw(self) -> I32x4 {
        I32x4([self[2], self[1], self[2], self[3]])
    }

    /// Constructs a new vector from the fourth, second, third, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wyzw(self) -> I32x4 {
        I32x4([self[3], self[1], self[2], self[3]])
    }

    /// Constructs a new vector from the first, third, third, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xzzw(self) -> I32x4 {
        I32x4([self[0], self[2], self[2], self[3]])
    }

    /// Constructs a new vector from the second, third, third, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yzzw(self) -> I32x4 {
        I32x4([self[1], self[2], self[2], self[3]])
    }

    /// Constructs a new vector from the third, third, third, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zzzw(self) -> I32x4 {
        I32x4([self[2], self[2], self[2], self[3]])
    }

    /// Constructs a new vector from the fourth, third, third, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wzzw(self) -> I32x4 {
        I32x4([self[3], self[2], self[2], self[3]])
    }

    /// Constructs a new vector from the first, fourth, third, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xwzw(self) -> I32x4 {
        I32x4([self[0], self[3], self[2], self[3]])
    }

    /// Constructs a new vector from the second, fourth, third, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn ywzw(self) -> I32x4 {
        I32x4([self[1], self[3], self[2], self[3]])
    }

    /// Constructs a new vector from the third, fourth, third, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zwzw(self) -> I32x4 {
        I32x4([self[2], self[3], self[2], self[3]])
    }

    /// Constructs a new vector from the fourth, fourth, third, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wwzw(self) -> I32x4 {
        I32x4([self[3], self[3], self[2], self[3]])
    }

    /// Constructs a new vector from the first, first, fourth, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xxww(self) -> I32x4 {
        I32x4([self[0], self[0], self[3], self[3]])
    }

    /// Constructs a new vector from the second, first, fourth, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yxww(self) -> I32x4 {
        I32x4([self[1], self[0], self[3], self[3]])
    }

    /// Constructs a new vector from the third, first, fourth, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zxww(self) -> I32x4 {
        I32x4([self[2], self[0], self[3], self[3]])
    }

    /// Constructs a new vector from the fourth, first, fourth, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wxww(self) -> I32x4 {
        I32x4([self[3], self[0], self[3], self[3]])
    }

    /// Constructs a new vector from the first, second, fourth, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xyww(self) -> I32x4 {
        I32x4([self[0], self[1], self[3], self[3]])
    }

    /// Constructs a new vector from the second, second, fourth, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yyww(self) -> I32x4 {
        I32x4([self[1], self[1], self[3], self[3]])
    }

    /// Constructs a new vector from the third, second, fourth, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zyww(self) -> I32x4 {
        I32x4([self[2], self[1], self[3], self[3]])
    }

    /// Constructs a new vector from the fourth, second, fourth, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wyww(self) -> I32x4 {
        I32x4([self[3], self[1], self[3], self[3]])
    }

    /// Constructs a new vector from the first, third, fourth, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xzww(self) -> I32x4 {
        I32x4([self[0], self[2], self[3], self[3]])
    }

    /// Constructs a new vector from the second, third, fourth, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn yzww(self) -> I32x4 {
        I32x4([self[1], self[2], self[3], self[3]])
    }

    /// Constructs a new vector from the third, third, fourth, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zzww(self) -> I32x4 {
        I32x4([self[2], self[2], self[3], self[3]])
    }

    /// Constructs a new vector from the fourth, third, fourth, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wzww(self) -> I32x4 {
        I32x4([self[3], self[2], self[3], self[3]])
    }

    /// Constructs a new vector from the first, fourth, fourth, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn xwww(self) -> I32x4 {
        I32x4([self[0], self[3], self[3], self[3]])
    }

    /// Constructs a new vector from the second, fourth, fourth, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn ywww(self) -> I32x4 {
        I32x4([self[1], self[3], self[3], self[3]])
    }

    /// Constructs a new vector from the third, fourth, fourth, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn zwww(self) -> I32x4 {
        I32x4([self[2], self[3], self[3], self[3]])
    }

    /// Constructs a new vector from the fourth, fourth, fourth, and fourth
    /// lanes in this vector, respectively.
    #[inline]
    pub fn wwww(self) -> I32x4 {
        I32x4([self[3], self[3], self[3], self[3]])
    }
}
