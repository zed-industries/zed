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
    fn xx(self) -> Vec2 {
        Vec2 {
            x: self.x,
            y: self.x,
        }
    }

    #[inline]
    fn xy(self) -> Vec2 {
        Vec2 {
            x: self.x,
            y: self.y,
        }
    }

    #[inline]
    fn xz(self) -> Vec2 {
        Vec2 {
            x: self.x,
            y: self.z,
        }
    }

    #[inline]
    fn yx(self) -> Vec2 {
        Vec2 {
            x: self.y,
            y: self.x,
        }
    }

    #[inline]
    fn yy(self) -> Vec2 {
        Vec2 {
            x: self.y,
            y: self.y,
        }
    }

    #[inline]
    fn yz(self) -> Vec2 {
        Vec2 {
            x: self.y,
            y: self.z,
        }
    }

    #[inline]
    fn zx(self) -> Vec2 {
        Vec2 {
            x: self.z,
            y: self.x,
        }
    }

    #[inline]
    fn zy(self) -> Vec2 {
        Vec2 {
            x: self.z,
            y: self.y,
        }
    }

    #[inline]
    fn zz(self) -> Vec2 {
        Vec2 {
            x: self.z,
            y: self.z,
        }
    }

    #[inline]
    fn xxx(self) -> Vec3A {
        Vec3A((unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_00_00_00) }).into())
    }

    #[inline]
    fn xxy(self) -> Vec3A {
        Vec3A((unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_01_00_00) }).into())
    }

    #[inline]
    fn xxz(self) -> Vec3A {
        Vec3A((unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_10_00_00) }).into())
    }

    #[inline]
    fn xyx(self) -> Vec3A {
        Vec3A((unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_00_01_00) }).into())
    }

    #[inline]
    fn xyy(self) -> Vec3A {
        Vec3A((unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_01_01_00) }).into())
    }

    #[inline]
    fn xyz(self) -> Vec3A {
        Vec3A((unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_10_01_00) }).into())
    }

    #[inline]
    fn xzx(self) -> Vec3A {
        Vec3A((unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_00_10_00) }).into())
    }

    #[inline]
    fn xzy(self) -> Vec3A {
        Vec3A((unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_01_10_00) }).into())
    }

    #[inline]
    fn xzz(self) -> Vec3A {
        Vec3A((unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_10_10_00) }).into())
    }

    #[inline]
    fn yxx(self) -> Vec3A {
        Vec3A((unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_00_00_01) }).into())
    }

    #[inline]
    fn yxy(self) -> Vec3A {
        Vec3A((unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_01_00_01) }).into())
    }

    #[inline]
    fn yxz(self) -> Vec3A {
        Vec3A((unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_10_00_01) }).into())
    }

    #[inline]
    fn yyx(self) -> Vec3A {
        Vec3A((unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_00_01_01) }).into())
    }

    #[inline]
    fn yyy(self) -> Vec3A {
        Vec3A((unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_01_01_01) }).into())
    }

    #[inline]
    fn yyz(self) -> Vec3A {
        Vec3A((unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_10_01_01) }).into())
    }

    #[inline]
    fn yzx(self) -> Vec3A {
        Vec3A((unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_00_10_01) }).into())
    }

    #[inline]
    fn yzy(self) -> Vec3A {
        Vec3A((unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_01_10_01) }).into())
    }

    #[inline]
    fn yzz(self) -> Vec3A {
        Vec3A((unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_10_10_01) }).into())
    }

    #[inline]
    fn zxx(self) -> Vec3A {
        Vec3A((unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_00_00_10) }).into())
    }

    #[inline]
    fn zxy(self) -> Vec3A {
        Vec3A((unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_01_00_10) }).into())
    }

    #[inline]
    fn zxz(self) -> Vec3A {
        Vec3A((unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_10_00_10) }).into())
    }

    #[inline]
    fn zyx(self) -> Vec3A {
        Vec3A((unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_00_01_10) }).into())
    }

    #[inline]
    fn zyy(self) -> Vec3A {
        Vec3A((unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_01_01_10) }).into())
    }

    #[inline]
    fn zyz(self) -> Vec3A {
        Vec3A((unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_10_01_10) }).into())
    }

    #[inline]
    fn zzx(self) -> Vec3A {
        Vec3A((unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_00_10_10) }).into())
    }

    #[inline]
    fn zzy(self) -> Vec3A {
        Vec3A((unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_01_10_10) }).into())
    }

    #[inline]
    fn zzz(self) -> Vec3A {
        Vec3A((unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_10_10_10) }).into())
    }

    #[inline]
    fn xxxx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_00_00_00) })
    }

    #[inline]
    fn xxxy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_00_00_00) })
    }

    #[inline]
    fn xxxz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_00_00_00) })
    }

    #[inline]
    fn xxyx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_01_00_00) })
    }

    #[inline]
    fn xxyy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_01_00_00) })
    }

    #[inline]
    fn xxyz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_01_00_00) })
    }

    #[inline]
    fn xxzx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_10_00_00) })
    }

    #[inline]
    fn xxzy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_10_00_00) })
    }

    #[inline]
    fn xxzz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_10_00_00) })
    }

    #[inline]
    fn xyxx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_00_01_00) })
    }

    #[inline]
    fn xyxy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_00_01_00) })
    }

    #[inline]
    fn xyxz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_00_01_00) })
    }

    #[inline]
    fn xyyx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_01_01_00) })
    }

    #[inline]
    fn xyyy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_01_01_00) })
    }

    #[inline]
    fn xyyz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_01_01_00) })
    }

    #[inline]
    fn xyzx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_10_01_00) })
    }

    #[inline]
    fn xyzy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_10_01_00) })
    }

    #[inline]
    fn xyzz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_10_01_00) })
    }

    #[inline]
    fn xzxx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_00_10_00) })
    }

    #[inline]
    fn xzxy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_00_10_00) })
    }

    #[inline]
    fn xzxz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_00_10_00) })
    }

    #[inline]
    fn xzyx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_01_10_00) })
    }

    #[inline]
    fn xzyy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_01_10_00) })
    }

    #[inline]
    fn xzyz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_01_10_00) })
    }

    #[inline]
    fn xzzx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_10_10_00) })
    }

    #[inline]
    fn xzzy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_10_10_00) })
    }

    #[inline]
    fn xzzz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_10_10_00) })
    }

    #[inline]
    fn yxxx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_00_00_01) })
    }

    #[inline]
    fn yxxy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_00_00_01) })
    }

    #[inline]
    fn yxxz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_00_00_01) })
    }

    #[inline]
    fn yxyx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_01_00_01) })
    }

    #[inline]
    fn yxyy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_01_00_01) })
    }

    #[inline]
    fn yxyz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_01_00_01) })
    }

    #[inline]
    fn yxzx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_10_00_01) })
    }

    #[inline]
    fn yxzy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_10_00_01) })
    }

    #[inline]
    fn yxzz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_10_00_01) })
    }

    #[inline]
    fn yyxx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_00_01_01) })
    }

    #[inline]
    fn yyxy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_00_01_01) })
    }

    #[inline]
    fn yyxz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_00_01_01) })
    }

    #[inline]
    fn yyyx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_01_01_01) })
    }

    #[inline]
    fn yyyy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_01_01_01) })
    }

    #[inline]
    fn yyyz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_01_01_01) })
    }

    #[inline]
    fn yyzx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_10_01_01) })
    }

    #[inline]
    fn yyzy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_10_01_01) })
    }

    #[inline]
    fn yyzz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_10_01_01) })
    }

    #[inline]
    fn yzxx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_00_10_01) })
    }

    #[inline]
    fn yzxy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_00_10_01) })
    }

    #[inline]
    fn yzxz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_00_10_01) })
    }

    #[inline]
    fn yzyx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_01_10_01) })
    }

    #[inline]
    fn yzyy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_01_10_01) })
    }

    #[inline]
    fn yzyz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_01_10_01) })
    }

    #[inline]
    fn yzzx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_10_10_01) })
    }

    #[inline]
    fn yzzy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_10_10_01) })
    }

    #[inline]
    fn yzzz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_10_10_01) })
    }

    #[inline]
    fn zxxx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_00_00_10) })
    }

    #[inline]
    fn zxxy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_00_00_10) })
    }

    #[inline]
    fn zxxz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_00_00_10) })
    }

    #[inline]
    fn zxyx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_01_00_10) })
    }

    #[inline]
    fn zxyy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_01_00_10) })
    }

    #[inline]
    fn zxyz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_01_00_10) })
    }

    #[inline]
    fn zxzx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_10_00_10) })
    }

    #[inline]
    fn zxzy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_10_00_10) })
    }

    #[inline]
    fn zxzz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_10_00_10) })
    }

    #[inline]
    fn zyxx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_00_01_10) })
    }

    #[inline]
    fn zyxy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_00_01_10) })
    }

    #[inline]
    fn zyxz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_00_01_10) })
    }

    #[inline]
    fn zyyx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_01_01_10) })
    }

    #[inline]
    fn zyyy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_01_01_10) })
    }

    #[inline]
    fn zyyz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_01_01_10) })
    }

    #[inline]
    fn zyzx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_10_01_10) })
    }

    #[inline]
    fn zyzy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_10_01_10) })
    }

    #[inline]
    fn zyzz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_10_01_10) })
    }

    #[inline]
    fn zzxx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_00_10_10) })
    }

    #[inline]
    fn zzxy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_00_10_10) })
    }

    #[inline]
    fn zzxz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_00_10_10) })
    }

    #[inline]
    fn zzyx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_01_10_10) })
    }

    #[inline]
    fn zzyy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_01_10_10) })
    }

    #[inline]
    fn zzyz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_01_10_10) })
    }

    #[inline]
    fn zzzx(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_10_10_10) })
    }

    #[inline]
    fn zzzy(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_10_10_10) })
    }

    #[inline]
    fn zzzz(self) -> Vec4 {
        Vec4(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_10_10_10) })
    }
}
