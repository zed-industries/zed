// Generated from swizzle_impl.rs.tera template. Edit the template, not the generated file.

#![allow(clippy::useless_conversion)]

use crate::{Vec2, Vec3A, Vec3Swizzles, Vec4};

#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

impl Vec3Swizzles for Vec3A {
    type Vec2 = Vec2;

    type Vec4 = Vec4;

    #[inline]
    #[must_use]
    fn xx(self) -> Vec2 {
        Vec2 {
            x: self.x,
            y: self.x,
        }
    }

    #[inline]
    #[must_use]
    fn xy(self) -> Vec2 {
        Vec2 {
            x: self.x,
            y: self.y,
        }
    }

    #[inline]
    #[must_use]
    fn xz(self) -> Vec2 {
        Vec2 {
            x: self.x,
            y: self.z,
        }
    }

    #[inline]
    #[must_use]
    fn yx(self) -> Vec2 {
        Vec2 {
            x: self.y,
            y: self.x,
        }
    }

    #[inline]
    #[must_use]
    fn yy(self) -> Vec2 {
        Vec2 {
            x: self.y,
            y: self.y,
        }
    }

    #[inline]
    #[must_use]
    fn yz(self) -> Vec2 {
        Vec2 {
            x: self.y,
            y: self.z,
        }
    }

    #[inline]
    #[must_use]
    fn zx(self) -> Vec2 {
        Vec2 {
            x: self.z,
            y: self.x,
        }
    }

    #[inline]
    #[must_use]
    fn zy(self) -> Vec2 {
        Vec2 {
            x: self.z,
            y: self.y,
        }
    }

    #[inline]
    #[must_use]
    fn zz(self) -> Vec2 {
        Vec2 {
            x: self.z,
            y: self.z,
        }
    }

    #[inline]
    #[must_use]
    fn xxx(self) -> Vec3A {
        Vec3A((unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_00_00_00) }).into())
    }

    #[inline]
    #[must_use]
    fn xxy(self) -> Vec3A {
        Vec3A((unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_01_00_00) }).into())
    }

    #[inline]
    #[must_use]
    fn xxz(self) -> Vec3A {
        Vec3A((unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_10_00_00) }).into())
    }

    #[inline]
    #[must_use]
    fn xyx(self) -> Vec3A {
        Vec3A((unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_00_01_00) }).into())
    }

    #[inline]
    #[must_use]
    fn xyy(self) -> Vec3A {
        Vec3A((unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_01_01_00) }).into())
    }

    #[inline]
    #[must_use]
    fn xyz(self) -> Vec3A {
        Vec3A((unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_10_01_00) }).into())
    }

    #[inline]
    #[must_use]
    fn xzx(self) -> Vec3A {
        Vec3A((unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_00_10_00) }).into())
    }

    #[inline]
    #[must_use]
    fn xzy(self) -> Vec3A {
        Vec3A((unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_01_10_00) }).into())
    }

    #[inline]
    #[must_use]
    fn xzz(self) -> Vec3A {
        Vec3A((unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_10_10_00) }).into())
    }

    #[inline]
    #[must_use]
    fn yxx(self) -> Vec3A {
        Vec3A((unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_00_00_01) }).into())
    }

    #[inline]
    #[must_use]
    fn yxy(self) -> Vec3A {
        Vec3A((unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_01_00_01) }).into())
    }

    #[inline]
    #[must_use]
    fn yxz(self) -> Vec3A {
        Vec3A((unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_10_00_01) }).into())
    }

    #[inline]
    #[must_use]
    fn yyx(self) -> Vec3A {
        Vec3A((unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_00_01_01) }).into())
    }

    #[inline]
    #[must_use]
    fn yyy(self) -> Vec3A {
        Vec3A((unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_01_01_01) }).into())
    }

    #[inline]
    #[must_use]
    fn yyz(self) -> Vec3A {
        Vec3A((unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_10_01_01) }).into())
    }

    #[inline]
    #[must_use]
    fn yzx(self) -> Vec3A {
        Vec3A((unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_00_10_01) }).into())
    }

    #[inline]
    #[must_use]
    fn yzy(self) -> Vec3A {
        Vec3A((unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_01_10_01) }).into())
    }

    #[inline]
    #[must_use]
    fn yzz(self) -> Vec3A {
        Vec3A((unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_10_10_01) }).into())
    }

    #[inline]
    #[must_use]
    fn zxx(self) -> Vec3A {
        Vec3A((unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_00_00_10) }).into())
    }

    #[inline]
    #[must_use]
    fn zxy(self) -> Vec3A {
        Vec3A((unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_01_00_10) }).into())
    }

    #[inline]
    #[must_use]
    fn zxz(self) -> Vec3A {
        Vec3A((unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_10_00_10) }).into())
    }

    #[inline]
    #[must_use]
    fn zyx(self) -> Vec3A {
        Vec3A((unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_00_01_10) }).into())
    }

    #[inline]
    #[must_use]
    fn zyy(self) -> Vec3A {
        Vec3A((unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_01_01_10) }).into())
    }

    #[inline]
    #[must_use]
    fn zyz(self) -> Vec3A {
        Vec3A((unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_10_01_10) }).into())
    }

    #[inline]
    #[must_use]
    fn zzx(self) -> Vec3A {
        Vec3A((unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_00_10_10) }).into())
    }

    #[inline]
    #[must_use]
    fn zzy(self) -> Vec3A {
        Vec3A((unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_01_10_10) }).into())
    }

    #[inline]
    #[must_use]
    fn zzz(self) -> Vec3A {
        Vec3A((unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_10_10_10) }).into())
    }

    #[inline]
    #[must_use]
    fn xxxx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_00_00_00) })
    }

    #[inline]
    #[must_use]
    fn xxxy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_00_00_00) })
    }

    #[inline]
    #[must_use]
    fn xxxz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_00_00_00) })
    }

    #[inline]
    #[must_use]
    fn xxyx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_01_00_00) })
    }

    #[inline]
    #[must_use]
    fn xxyy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_01_00_00) })
    }

    #[inline]
    #[must_use]
    fn xxyz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_01_00_00) })
    }

    #[inline]
    #[must_use]
    fn xxzx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_10_00_00) })
    }

    #[inline]
    #[must_use]
    fn xxzy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_10_00_00) })
    }

    #[inline]
    #[must_use]
    fn xxzz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_10_00_00) })
    }

    #[inline]
    #[must_use]
    fn xyxx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_00_01_00) })
    }

    #[inline]
    #[must_use]
    fn xyxy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_00_01_00) })
    }

    #[inline]
    #[must_use]
    fn xyxz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_00_01_00) })
    }

    #[inline]
    #[must_use]
    fn xyyx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_01_01_00) })
    }

    #[inline]
    #[must_use]
    fn xyyy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_01_01_00) })
    }

    #[inline]
    #[must_use]
    fn xyyz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_01_01_00) })
    }

    #[inline]
    #[must_use]
    fn xyzx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_10_01_00) })
    }

    #[inline]
    #[must_use]
    fn xyzy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_10_01_00) })
    }

    #[inline]
    #[must_use]
    fn xyzz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_10_01_00) })
    }

    #[inline]
    #[must_use]
    fn xzxx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_00_10_00) })
    }

    #[inline]
    #[must_use]
    fn xzxy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_00_10_00) })
    }

    #[inline]
    #[must_use]
    fn xzxz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_00_10_00) })
    }

    #[inline]
    #[must_use]
    fn xzyx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_01_10_00) })
    }

    #[inline]
    #[must_use]
    fn xzyy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_01_10_00) })
    }

    #[inline]
    #[must_use]
    fn xzyz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_01_10_00) })
    }

    #[inline]
    #[must_use]
    fn xzzx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_10_10_00) })
    }

    #[inline]
    #[must_use]
    fn xzzy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_10_10_00) })
    }

    #[inline]
    #[must_use]
    fn xzzz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_10_10_00) })
    }

    #[inline]
    #[must_use]
    fn yxxx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_00_00_01) })
    }

    #[inline]
    #[must_use]
    fn yxxy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_00_00_01) })
    }

    #[inline]
    #[must_use]
    fn yxxz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_00_00_01) })
    }

    #[inline]
    #[must_use]
    fn yxyx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_01_00_01) })
    }

    #[inline]
    #[must_use]
    fn yxyy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_01_00_01) })
    }

    #[inline]
    #[must_use]
    fn yxyz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_01_00_01) })
    }

    #[inline]
    #[must_use]
    fn yxzx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_10_00_01) })
    }

    #[inline]
    #[must_use]
    fn yxzy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_10_00_01) })
    }

    #[inline]
    #[must_use]
    fn yxzz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_10_00_01) })
    }

    #[inline]
    #[must_use]
    fn yyxx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_00_01_01) })
    }

    #[inline]
    #[must_use]
    fn yyxy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_00_01_01) })
    }

    #[inline]
    #[must_use]
    fn yyxz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_00_01_01) })
    }

    #[inline]
    #[must_use]
    fn yyyx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_01_01_01) })
    }

    #[inline]
    #[must_use]
    fn yyyy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_01_01_01) })
    }

    #[inline]
    #[must_use]
    fn yyyz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_01_01_01) })
    }

    #[inline]
    #[must_use]
    fn yyzx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_10_01_01) })
    }

    #[inline]
    #[must_use]
    fn yyzy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_10_01_01) })
    }

    #[inline]
    #[must_use]
    fn yyzz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_10_01_01) })
    }

    #[inline]
    #[must_use]
    fn yzxx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_00_10_01) })
    }

    #[inline]
    #[must_use]
    fn yzxy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_00_10_01) })
    }

    #[inline]
    #[must_use]
    fn yzxz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_00_10_01) })
    }

    #[inline]
    #[must_use]
    fn yzyx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_01_10_01) })
    }

    #[inline]
    #[must_use]
    fn yzyy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_01_10_01) })
    }

    #[inline]
    #[must_use]
    fn yzyz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_01_10_01) })
    }

    #[inline]
    #[must_use]
    fn yzzx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_10_10_01) })
    }

    #[inline]
    #[must_use]
    fn yzzy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_10_10_01) })
    }

    #[inline]
    #[must_use]
    fn yzzz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_10_10_01) })
    }

    #[inline]
    #[must_use]
    fn zxxx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_00_00_10) })
    }

    #[inline]
    #[must_use]
    fn zxxy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_00_00_10) })
    }

    #[inline]
    #[must_use]
    fn zxxz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_00_00_10) })
    }

    #[inline]
    #[must_use]
    fn zxyx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_01_00_10) })
    }

    #[inline]
    #[must_use]
    fn zxyy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_01_00_10) })
    }

    #[inline]
    #[must_use]
    fn zxyz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_01_00_10) })
    }

    #[inline]
    #[must_use]
    fn zxzx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_10_00_10) })
    }

    #[inline]
    #[must_use]
    fn zxzy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_10_00_10) })
    }

    #[inline]
    #[must_use]
    fn zxzz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_10_00_10) })
    }

    #[inline]
    #[must_use]
    fn zyxx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_00_01_10) })
    }

    #[inline]
    #[must_use]
    fn zyxy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_00_01_10) })
    }

    #[inline]
    #[must_use]
    fn zyxz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_00_01_10) })
    }

    #[inline]
    #[must_use]
    fn zyyx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_01_01_10) })
    }

    #[inline]
    #[must_use]
    fn zyyy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_01_01_10) })
    }

    #[inline]
    #[must_use]
    fn zyyz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_01_01_10) })
    }

    #[inline]
    #[must_use]
    fn zyzx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_10_01_10) })
    }

    #[inline]
    #[must_use]
    fn zyzy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_10_01_10) })
    }

    #[inline]
    #[must_use]
    fn zyzz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_10_01_10) })
    }

    #[inline]
    #[must_use]
    fn zzxx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_00_10_10) })
    }

    #[inline]
    #[must_use]
    fn zzxy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_00_10_10) })
    }

    #[inline]
    #[must_use]
    fn zzxz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_00_10_10) })
    }

    #[inline]
    #[must_use]
    fn zzyx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_01_10_10) })
    }

    #[inline]
    #[must_use]
    fn zzyy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_01_10_10) })
    }

    #[inline]
    #[must_use]
    fn zzyz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_01_10_10) })
    }

    #[inline]
    #[must_use]
    fn zzzx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_10_10_10) })
    }

    #[inline]
    #[must_use]
    fn zzzy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_10_10_10) })
    }

    #[inline]
    #[must_use]
    fn zzzz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_10_10_10) })
    }
}
