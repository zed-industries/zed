// Generated from swizzle_impl.rs.tera template. Edit the template, not the generated file.

#![allow(clippy::useless_conversion)]

use crate::{Vec2, Vec3, Vec4, Vec4Swizzles};

#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

impl Vec4Swizzles for Vec4 {
    type Vec2 = Vec2;

    type Vec3 = Vec3;

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
    fn with_xy(self, rhs: Vec2) -> Self {
        Self::new(rhs.x, rhs.y, self.z, self.w)
    }

    #[inline]
    fn xz(self) -> Vec2 {
        Vec2 {
            x: self.x,
            y: self.z,
        }
    }

    #[inline]
    fn with_xz(self, rhs: Vec2) -> Self {
        Self::new(rhs.x, self.y, rhs.y, self.w)
    }

    #[inline]
    fn xw(self) -> Vec2 {
        Vec2 {
            x: self.x,
            y: self.w,
        }
    }

    #[inline]
    fn with_xw(self, rhs: Vec2) -> Self {
        Self::new(rhs.x, self.y, self.z, rhs.y)
    }

    #[inline]
    fn yx(self) -> Vec2 {
        Vec2 {
            x: self.y,
            y: self.x,
        }
    }

    #[inline]
    fn with_yx(self, rhs: Vec2) -> Self {
        Self::new(rhs.y, rhs.x, self.z, self.w)
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
    fn with_yz(self, rhs: Vec2) -> Self {
        Self::new(self.x, rhs.x, rhs.y, self.w)
    }

    #[inline]
    fn yw(self) -> Vec2 {
        Vec2 {
            x: self.y,
            y: self.w,
        }
    }

    #[inline]
    fn with_yw(self, rhs: Vec2) -> Self {
        Self::new(self.x, rhs.x, self.z, rhs.y)
    }

    #[inline]
    fn zx(self) -> Vec2 {
        Vec2 {
            x: self.z,
            y: self.x,
        }
    }

    #[inline]
    fn with_zx(self, rhs: Vec2) -> Self {
        Self::new(rhs.y, self.y, rhs.x, self.w)
    }

    #[inline]
    fn zy(self) -> Vec2 {
        Vec2 {
            x: self.z,
            y: self.y,
        }
    }

    #[inline]
    fn with_zy(self, rhs: Vec2) -> Self {
        Self::new(self.x, rhs.y, rhs.x, self.w)
    }

    #[inline]
    fn zz(self) -> Vec2 {
        Vec2 {
            x: self.z,
            y: self.z,
        }
    }

    #[inline]
    fn zw(self) -> Vec2 {
        Vec2 {
            x: self.z,
            y: self.w,
        }
    }

    #[inline]
    fn with_zw(self, rhs: Vec2) -> Self {
        Self::new(self.x, self.y, rhs.x, rhs.y)
    }

    #[inline]
    fn wx(self) -> Vec2 {
        Vec2 {
            x: self.w,
            y: self.x,
        }
    }

    #[inline]
    fn with_wx(self, rhs: Vec2) -> Self {
        Self::new(rhs.y, self.y, self.z, rhs.x)
    }

    #[inline]
    fn wy(self) -> Vec2 {
        Vec2 {
            x: self.w,
            y: self.y,
        }
    }

    #[inline]
    fn with_wy(self, rhs: Vec2) -> Self {
        Self::new(self.x, rhs.y, self.z, rhs.x)
    }

    #[inline]
    fn wz(self) -> Vec2 {
        Vec2 {
            x: self.w,
            y: self.z,
        }
    }

    #[inline]
    fn with_wz(self, rhs: Vec2) -> Self {
        Self::new(self.x, self.y, rhs.y, rhs.x)
    }

    #[inline]
    fn ww(self) -> Vec2 {
        Vec2 {
            x: self.w,
            y: self.w,
        }
    }

    #[inline]
    fn xxx(self) -> Vec3 {
        Vec3::new(self.x, self.x, self.x)
    }

    #[inline]
    fn xxy(self) -> Vec3 {
        Vec3::new(self.x, self.x, self.y)
    }

    #[inline]
    fn xxz(self) -> Vec3 {
        Vec3::new(self.x, self.x, self.z)
    }

    #[inline]
    fn xxw(self) -> Vec3 {
        Vec3::new(self.x, self.x, self.w)
    }

    #[inline]
    fn xyx(self) -> Vec3 {
        Vec3::new(self.x, self.y, self.x)
    }

    #[inline]
    fn xyy(self) -> Vec3 {
        Vec3::new(self.x, self.y, self.y)
    }

    #[inline]
    fn xyz(self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }

    #[inline]
    fn with_xyz(self, rhs: Vec3) -> Self {
        Self::new(rhs.x, rhs.y, rhs.z, self.w)
    }

    #[inline]
    fn xyw(self) -> Vec3 {
        Vec3::new(self.x, self.y, self.w)
    }

    #[inline]
    fn with_xyw(self, rhs: Vec3) -> Self {
        Self::new(rhs.x, rhs.y, self.z, rhs.z)
    }

    #[inline]
    fn xzx(self) -> Vec3 {
        Vec3::new(self.x, self.z, self.x)
    }

    #[inline]
    fn xzy(self) -> Vec3 {
        Vec3::new(self.x, self.z, self.y)
    }

    #[inline]
    fn with_xzy(self, rhs: Vec3) -> Self {
        Self::new(rhs.x, rhs.z, rhs.y, self.w)
    }

    #[inline]
    fn xzz(self) -> Vec3 {
        Vec3::new(self.x, self.z, self.z)
    }

    #[inline]
    fn xzw(self) -> Vec3 {
        Vec3::new(self.x, self.z, self.w)
    }

    #[inline]
    fn with_xzw(self, rhs: Vec3) -> Self {
        Self::new(rhs.x, self.y, rhs.y, rhs.z)
    }

    #[inline]
    fn xwx(self) -> Vec3 {
        Vec3::new(self.x, self.w, self.x)
    }

    #[inline]
    fn xwy(self) -> Vec3 {
        Vec3::new(self.x, self.w, self.y)
    }

    #[inline]
    fn with_xwy(self, rhs: Vec3) -> Self {
        Self::new(rhs.x, rhs.z, self.z, rhs.y)
    }

    #[inline]
    fn xwz(self) -> Vec3 {
        Vec3::new(self.x, self.w, self.z)
    }

    #[inline]
    fn with_xwz(self, rhs: Vec3) -> Self {
        Self::new(rhs.x, self.y, rhs.z, rhs.y)
    }

    #[inline]
    fn xww(self) -> Vec3 {
        Vec3::new(self.x, self.w, self.w)
    }

    #[inline]
    fn yxx(self) -> Vec3 {
        Vec3::new(self.y, self.x, self.x)
    }

    #[inline]
    fn yxy(self) -> Vec3 {
        Vec3::new(self.y, self.x, self.y)
    }

    #[inline]
    fn yxz(self) -> Vec3 {
        Vec3::new(self.y, self.x, self.z)
    }

    #[inline]
    fn with_yxz(self, rhs: Vec3) -> Self {
        Self::new(rhs.y, rhs.x, rhs.z, self.w)
    }

    #[inline]
    fn yxw(self) -> Vec3 {
        Vec3::new(self.y, self.x, self.w)
    }

    #[inline]
    fn with_yxw(self, rhs: Vec3) -> Self {
        Self::new(rhs.y, rhs.x, self.z, rhs.z)
    }

    #[inline]
    fn yyx(self) -> Vec3 {
        Vec3::new(self.y, self.y, self.x)
    }

    #[inline]
    fn yyy(self) -> Vec3 {
        Vec3::new(self.y, self.y, self.y)
    }

    #[inline]
    fn yyz(self) -> Vec3 {
        Vec3::new(self.y, self.y, self.z)
    }

    #[inline]
    fn yyw(self) -> Vec3 {
        Vec3::new(self.y, self.y, self.w)
    }

    #[inline]
    fn yzx(self) -> Vec3 {
        Vec3::new(self.y, self.z, self.x)
    }

    #[inline]
    fn with_yzx(self, rhs: Vec3) -> Self {
        Self::new(rhs.z, rhs.x, rhs.y, self.w)
    }

    #[inline]
    fn yzy(self) -> Vec3 {
        Vec3::new(self.y, self.z, self.y)
    }

    #[inline]
    fn yzz(self) -> Vec3 {
        Vec3::new(self.y, self.z, self.z)
    }

    #[inline]
    fn yzw(self) -> Vec3 {
        Vec3::new(self.y, self.z, self.w)
    }

    #[inline]
    fn with_yzw(self, rhs: Vec3) -> Self {
        Self::new(self.x, rhs.x, rhs.y, rhs.z)
    }

    #[inline]
    fn ywx(self) -> Vec3 {
        Vec3::new(self.y, self.w, self.x)
    }

    #[inline]
    fn with_ywx(self, rhs: Vec3) -> Self {
        Self::new(rhs.z, rhs.x, self.z, rhs.y)
    }

    #[inline]
    fn ywy(self) -> Vec3 {
        Vec3::new(self.y, self.w, self.y)
    }

    #[inline]
    fn ywz(self) -> Vec3 {
        Vec3::new(self.y, self.w, self.z)
    }

    #[inline]
    fn with_ywz(self, rhs: Vec3) -> Self {
        Self::new(self.x, rhs.x, rhs.z, rhs.y)
    }

    #[inline]
    fn yww(self) -> Vec3 {
        Vec3::new(self.y, self.w, self.w)
    }

    #[inline]
    fn zxx(self) -> Vec3 {
        Vec3::new(self.z, self.x, self.x)
    }

    #[inline]
    fn zxy(self) -> Vec3 {
        Vec3::new(self.z, self.x, self.y)
    }

    #[inline]
    fn with_zxy(self, rhs: Vec3) -> Self {
        Self::new(rhs.y, rhs.z, rhs.x, self.w)
    }

    #[inline]
    fn zxz(self) -> Vec3 {
        Vec3::new(self.z, self.x, self.z)
    }

    #[inline]
    fn zxw(self) -> Vec3 {
        Vec3::new(self.z, self.x, self.w)
    }

    #[inline]
    fn with_zxw(self, rhs: Vec3) -> Self {
        Self::new(rhs.y, self.y, rhs.x, rhs.z)
    }

    #[inline]
    fn zyx(self) -> Vec3 {
        Vec3::new(self.z, self.y, self.x)
    }

    #[inline]
    fn with_zyx(self, rhs: Vec3) -> Self {
        Self::new(rhs.z, rhs.y, rhs.x, self.w)
    }

    #[inline]
    fn zyy(self) -> Vec3 {
        Vec3::new(self.z, self.y, self.y)
    }

    #[inline]
    fn zyz(self) -> Vec3 {
        Vec3::new(self.z, self.y, self.z)
    }

    #[inline]
    fn zyw(self) -> Vec3 {
        Vec3::new(self.z, self.y, self.w)
    }

    #[inline]
    fn with_zyw(self, rhs: Vec3) -> Self {
        Self::new(self.x, rhs.y, rhs.x, rhs.z)
    }

    #[inline]
    fn zzx(self) -> Vec3 {
        Vec3::new(self.z, self.z, self.x)
    }

    #[inline]
    fn zzy(self) -> Vec3 {
        Vec3::new(self.z, self.z, self.y)
    }

    #[inline]
    fn zzz(self) -> Vec3 {
        Vec3::new(self.z, self.z, self.z)
    }

    #[inline]
    fn zzw(self) -> Vec3 {
        Vec3::new(self.z, self.z, self.w)
    }

    #[inline]
    fn zwx(self) -> Vec3 {
        Vec3::new(self.z, self.w, self.x)
    }

    #[inline]
    fn with_zwx(self, rhs: Vec3) -> Self {
        Self::new(rhs.z, self.y, rhs.x, rhs.y)
    }

    #[inline]
    fn zwy(self) -> Vec3 {
        Vec3::new(self.z, self.w, self.y)
    }

    #[inline]
    fn with_zwy(self, rhs: Vec3) -> Self {
        Self::new(self.x, rhs.z, rhs.x, rhs.y)
    }

    #[inline]
    fn zwz(self) -> Vec3 {
        Vec3::new(self.z, self.w, self.z)
    }

    #[inline]
    fn zww(self) -> Vec3 {
        Vec3::new(self.z, self.w, self.w)
    }

    #[inline]
    fn wxx(self) -> Vec3 {
        Vec3::new(self.w, self.x, self.x)
    }

    #[inline]
    fn wxy(self) -> Vec3 {
        Vec3::new(self.w, self.x, self.y)
    }

    #[inline]
    fn with_wxy(self, rhs: Vec3) -> Self {
        Self::new(rhs.y, rhs.z, self.z, rhs.x)
    }

    #[inline]
    fn wxz(self) -> Vec3 {
        Vec3::new(self.w, self.x, self.z)
    }

    #[inline]
    fn with_wxz(self, rhs: Vec3) -> Self {
        Self::new(rhs.y, self.y, rhs.z, rhs.x)
    }

    #[inline]
    fn wxw(self) -> Vec3 {
        Vec3::new(self.w, self.x, self.w)
    }

    #[inline]
    fn wyx(self) -> Vec3 {
        Vec3::new(self.w, self.y, self.x)
    }

    #[inline]
    fn with_wyx(self, rhs: Vec3) -> Self {
        Self::new(rhs.z, rhs.y, self.z, rhs.x)
    }

    #[inline]
    fn wyy(self) -> Vec3 {
        Vec3::new(self.w, self.y, self.y)
    }

    #[inline]
    fn wyz(self) -> Vec3 {
        Vec3::new(self.w, self.y, self.z)
    }

    #[inline]
    fn with_wyz(self, rhs: Vec3) -> Self {
        Self::new(self.x, rhs.y, rhs.z, rhs.x)
    }

    #[inline]
    fn wyw(self) -> Vec3 {
        Vec3::new(self.w, self.y, self.w)
    }

    #[inline]
    fn wzx(self) -> Vec3 {
        Vec3::new(self.w, self.z, self.x)
    }

    #[inline]
    fn with_wzx(self, rhs: Vec3) -> Self {
        Self::new(rhs.z, self.y, rhs.y, rhs.x)
    }

    #[inline]
    fn wzy(self) -> Vec3 {
        Vec3::new(self.w, self.z, self.y)
    }

    #[inline]
    fn with_wzy(self, rhs: Vec3) -> Self {
        Self::new(self.x, rhs.z, rhs.y, rhs.x)
    }

    #[inline]
    fn wzz(self) -> Vec3 {
        Vec3::new(self.w, self.z, self.z)
    }

    #[inline]
    fn wzw(self) -> Vec3 {
        Vec3::new(self.w, self.z, self.w)
    }

    #[inline]
    fn wwx(self) -> Vec3 {
        Vec3::new(self.w, self.w, self.x)
    }

    #[inline]
    fn wwy(self) -> Vec3 {
        Vec3::new(self.w, self.w, self.y)
    }

    #[inline]
    fn wwz(self) -> Vec3 {
        Vec3::new(self.w, self.w, self.z)
    }

    #[inline]
    fn www(self) -> Vec3 {
        Vec3::new(self.w, self.w, self.w)
    }

    #[inline]
    fn xxxx(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_00_00_00) })
    }

    #[inline]
    fn xxxy(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_00_00_00) })
    }

    #[inline]
    fn xxxz(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_00_00_00) })
    }

    #[inline]
    fn xxxw(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_00_00_00) })
    }

    #[inline]
    fn xxyx(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_01_00_00) })
    }

    #[inline]
    fn xxyy(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_01_00_00) })
    }

    #[inline]
    fn xxyz(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_01_00_00) })
    }

    #[inline]
    fn xxyw(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_01_00_00) })
    }

    #[inline]
    fn xxzx(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_10_00_00) })
    }

    #[inline]
    fn xxzy(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_10_00_00) })
    }

    #[inline]
    fn xxzz(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_10_00_00) })
    }

    #[inline]
    fn xxzw(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_10_00_00) })
    }

    #[inline]
    fn xxwx(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_11_00_00) })
    }

    #[inline]
    fn xxwy(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_11_00_00) })
    }

    #[inline]
    fn xxwz(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_11_00_00) })
    }

    #[inline]
    fn xxww(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_11_00_00) })
    }

    #[inline]
    fn xyxx(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_00_01_00) })
    }

    #[inline]
    fn xyxy(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_00_01_00) })
    }

    #[inline]
    fn xyxz(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_00_01_00) })
    }

    #[inline]
    fn xyxw(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_00_01_00) })
    }

    #[inline]
    fn xyyx(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_01_01_00) })
    }

    #[inline]
    fn xyyy(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_01_01_00) })
    }

    #[inline]
    fn xyyz(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_01_01_00) })
    }

    #[inline]
    fn xyyw(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_01_01_00) })
    }

    #[inline]
    fn xyzx(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_10_01_00) })
    }

    #[inline]
    fn xyzy(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_10_01_00) })
    }

    #[inline]
    fn xyzz(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_10_01_00) })
    }

    #[inline]
    fn xywx(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_11_01_00) })
    }

    #[inline]
    fn xywy(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_11_01_00) })
    }

    #[inline]
    fn xywz(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_11_01_00) })
    }

    #[inline]
    fn xyww(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_11_01_00) })
    }

    #[inline]
    fn xzxx(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_00_10_00) })
    }

    #[inline]
    fn xzxy(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_00_10_00) })
    }

    #[inline]
    fn xzxz(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_00_10_00) })
    }

    #[inline]
    fn xzxw(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_00_10_00) })
    }

    #[inline]
    fn xzyx(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_01_10_00) })
    }

    #[inline]
    fn xzyy(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_01_10_00) })
    }

    #[inline]
    fn xzyz(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_01_10_00) })
    }

    #[inline]
    fn xzyw(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_01_10_00) })
    }

    #[inline]
    fn xzzx(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_10_10_00) })
    }

    #[inline]
    fn xzzy(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_10_10_00) })
    }

    #[inline]
    fn xzzz(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_10_10_00) })
    }

    #[inline]
    fn xzzw(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_10_10_00) })
    }

    #[inline]
    fn xzwx(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_11_10_00) })
    }

    #[inline]
    fn xzwy(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_11_10_00) })
    }

    #[inline]
    fn xzwz(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_11_10_00) })
    }

    #[inline]
    fn xzww(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_11_10_00) })
    }

    #[inline]
    fn xwxx(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_00_11_00) })
    }

    #[inline]
    fn xwxy(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_00_11_00) })
    }

    #[inline]
    fn xwxz(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_00_11_00) })
    }

    #[inline]
    fn xwxw(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_00_11_00) })
    }

    #[inline]
    fn xwyx(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_01_11_00) })
    }

    #[inline]
    fn xwyy(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_01_11_00) })
    }

    #[inline]
    fn xwyz(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_01_11_00) })
    }

    #[inline]
    fn xwyw(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_01_11_00) })
    }

    #[inline]
    fn xwzx(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_10_11_00) })
    }

    #[inline]
    fn xwzy(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_10_11_00) })
    }

    #[inline]
    fn xwzz(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_10_11_00) })
    }

    #[inline]
    fn xwzw(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_10_11_00) })
    }

    #[inline]
    fn xwwx(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_11_11_00) })
    }

    #[inline]
    fn xwwy(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_11_11_00) })
    }

    #[inline]
    fn xwwz(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_11_11_00) })
    }

    #[inline]
    fn xwww(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_11_11_00) })
    }

    #[inline]
    fn yxxx(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_00_00_01) })
    }

    #[inline]
    fn yxxy(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_00_00_01) })
    }

    #[inline]
    fn yxxz(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_00_00_01) })
    }

    #[inline]
    fn yxxw(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_00_00_01) })
    }

    #[inline]
    fn yxyx(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_01_00_01) })
    }

    #[inline]
    fn yxyy(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_01_00_01) })
    }

    #[inline]
    fn yxyz(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_01_00_01) })
    }

    #[inline]
    fn yxyw(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_01_00_01) })
    }

    #[inline]
    fn yxzx(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_10_00_01) })
    }

    #[inline]
    fn yxzy(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_10_00_01) })
    }

    #[inline]
    fn yxzz(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_10_00_01) })
    }

    #[inline]
    fn yxzw(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_10_00_01) })
    }

    #[inline]
    fn yxwx(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_11_00_01) })
    }

    #[inline]
    fn yxwy(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_11_00_01) })
    }

    #[inline]
    fn yxwz(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_11_00_01) })
    }

    #[inline]
    fn yxww(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_11_00_01) })
    }

    #[inline]
    fn yyxx(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_00_01_01) })
    }

    #[inline]
    fn yyxy(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_00_01_01) })
    }

    #[inline]
    fn yyxz(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_00_01_01) })
    }

    #[inline]
    fn yyxw(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_00_01_01) })
    }

    #[inline]
    fn yyyx(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_01_01_01) })
    }

    #[inline]
    fn yyyy(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_01_01_01) })
    }

    #[inline]
    fn yyyz(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_01_01_01) })
    }

    #[inline]
    fn yyyw(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_01_01_01) })
    }

    #[inline]
    fn yyzx(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_10_01_01) })
    }

    #[inline]
    fn yyzy(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_10_01_01) })
    }

    #[inline]
    fn yyzz(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_10_01_01) })
    }

    #[inline]
    fn yyzw(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_10_01_01) })
    }

    #[inline]
    fn yywx(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_11_01_01) })
    }

    #[inline]
    fn yywy(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_11_01_01) })
    }

    #[inline]
    fn yywz(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_11_01_01) })
    }

    #[inline]
    fn yyww(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_11_01_01) })
    }

    #[inline]
    fn yzxx(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_00_10_01) })
    }

    #[inline]
    fn yzxy(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_00_10_01) })
    }

    #[inline]
    fn yzxz(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_00_10_01) })
    }

    #[inline]
    fn yzxw(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_00_10_01) })
    }

    #[inline]
    fn yzyx(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_01_10_01) })
    }

    #[inline]
    fn yzyy(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_01_10_01) })
    }

    #[inline]
    fn yzyz(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_01_10_01) })
    }

    #[inline]
    fn yzyw(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_01_10_01) })
    }

    #[inline]
    fn yzzx(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_10_10_01) })
    }

    #[inline]
    fn yzzy(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_10_10_01) })
    }

    #[inline]
    fn yzzz(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_10_10_01) })
    }

    #[inline]
    fn yzzw(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_10_10_01) })
    }

    #[inline]
    fn yzwx(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_11_10_01) })
    }

    #[inline]
    fn yzwy(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_11_10_01) })
    }

    #[inline]
    fn yzwz(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_11_10_01) })
    }

    #[inline]
    fn yzww(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_11_10_01) })
    }

    #[inline]
    fn ywxx(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_00_11_01) })
    }

    #[inline]
    fn ywxy(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_00_11_01) })
    }

    #[inline]
    fn ywxz(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_00_11_01) })
    }

    #[inline]
    fn ywxw(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_00_11_01) })
    }

    #[inline]
    fn ywyx(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_01_11_01) })
    }

    #[inline]
    fn ywyy(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_01_11_01) })
    }

    #[inline]
    fn ywyz(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_01_11_01) })
    }

    #[inline]
    fn ywyw(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_01_11_01) })
    }

    #[inline]
    fn ywzx(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_10_11_01) })
    }

    #[inline]
    fn ywzy(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_10_11_01) })
    }

    #[inline]
    fn ywzz(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_10_11_01) })
    }

    #[inline]
    fn ywzw(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_10_11_01) })
    }

    #[inline]
    fn ywwx(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_11_11_01) })
    }

    #[inline]
    fn ywwy(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_11_11_01) })
    }

    #[inline]
    fn ywwz(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_11_11_01) })
    }

    #[inline]
    fn ywww(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_11_11_01) })
    }

    #[inline]
    fn zxxx(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_00_00_10) })
    }

    #[inline]
    fn zxxy(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_00_00_10) })
    }

    #[inline]
    fn zxxz(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_00_00_10) })
    }

    #[inline]
    fn zxxw(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_00_00_10) })
    }

    #[inline]
    fn zxyx(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_01_00_10) })
    }

    #[inline]
    fn zxyy(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_01_00_10) })
    }

    #[inline]
    fn zxyz(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_01_00_10) })
    }

    #[inline]
    fn zxyw(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_01_00_10) })
    }

    #[inline]
    fn zxzx(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_10_00_10) })
    }

    #[inline]
    fn zxzy(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_10_00_10) })
    }

    #[inline]
    fn zxzz(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_10_00_10) })
    }

    #[inline]
    fn zxzw(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_10_00_10) })
    }

    #[inline]
    fn zxwx(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_11_00_10) })
    }

    #[inline]
    fn zxwy(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_11_00_10) })
    }

    #[inline]
    fn zxwz(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_11_00_10) })
    }

    #[inline]
    fn zxww(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_11_00_10) })
    }

    #[inline]
    fn zyxx(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_00_01_10) })
    }

    #[inline]
    fn zyxy(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_00_01_10) })
    }

    #[inline]
    fn zyxz(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_00_01_10) })
    }

    #[inline]
    fn zyxw(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_00_01_10) })
    }

    #[inline]
    fn zyyx(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_01_01_10) })
    }

    #[inline]
    fn zyyy(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_01_01_10) })
    }

    #[inline]
    fn zyyz(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_01_01_10) })
    }

    #[inline]
    fn zyyw(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_01_01_10) })
    }

    #[inline]
    fn zyzx(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_10_01_10) })
    }

    #[inline]
    fn zyzy(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_10_01_10) })
    }

    #[inline]
    fn zyzz(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_10_01_10) })
    }

    #[inline]
    fn zyzw(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_10_01_10) })
    }

    #[inline]
    fn zywx(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_11_01_10) })
    }

    #[inline]
    fn zywy(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_11_01_10) })
    }

    #[inline]
    fn zywz(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_11_01_10) })
    }

    #[inline]
    fn zyww(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_11_01_10) })
    }

    #[inline]
    fn zzxx(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_00_10_10) })
    }

    #[inline]
    fn zzxy(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_00_10_10) })
    }

    #[inline]
    fn zzxz(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_00_10_10) })
    }

    #[inline]
    fn zzxw(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_00_10_10) })
    }

    #[inline]
    fn zzyx(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_01_10_10) })
    }

    #[inline]
    fn zzyy(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_01_10_10) })
    }

    #[inline]
    fn zzyz(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_01_10_10) })
    }

    #[inline]
    fn zzyw(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_01_10_10) })
    }

    #[inline]
    fn zzzx(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_10_10_10) })
    }

    #[inline]
    fn zzzy(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_10_10_10) })
    }

    #[inline]
    fn zzzz(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_10_10_10) })
    }

    #[inline]
    fn zzzw(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_10_10_10) })
    }

    #[inline]
    fn zzwx(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_11_10_10) })
    }

    #[inline]
    fn zzwy(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_11_10_10) })
    }

    #[inline]
    fn zzwz(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_11_10_10) })
    }

    #[inline]
    fn zzww(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_11_10_10) })
    }

    #[inline]
    fn zwxx(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_00_11_10) })
    }

    #[inline]
    fn zwxy(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_00_11_10) })
    }

    #[inline]
    fn zwxz(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_00_11_10) })
    }

    #[inline]
    fn zwxw(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_00_11_10) })
    }

    #[inline]
    fn zwyx(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_01_11_10) })
    }

    #[inline]
    fn zwyy(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_01_11_10) })
    }

    #[inline]
    fn zwyz(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_01_11_10) })
    }

    #[inline]
    fn zwyw(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_01_11_10) })
    }

    #[inline]
    fn zwzx(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_10_11_10) })
    }

    #[inline]
    fn zwzy(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_10_11_10) })
    }

    #[inline]
    fn zwzz(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_10_11_10) })
    }

    #[inline]
    fn zwzw(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_10_11_10) })
    }

    #[inline]
    fn zwwx(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_11_11_10) })
    }

    #[inline]
    fn zwwy(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_11_11_10) })
    }

    #[inline]
    fn zwwz(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_11_11_10) })
    }

    #[inline]
    fn zwww(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_11_11_10) })
    }

    #[inline]
    fn wxxx(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_00_00_11) })
    }

    #[inline]
    fn wxxy(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_00_00_11) })
    }

    #[inline]
    fn wxxz(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_00_00_11) })
    }

    #[inline]
    fn wxxw(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_00_00_11) })
    }

    #[inline]
    fn wxyx(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_01_00_11) })
    }

    #[inline]
    fn wxyy(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_01_00_11) })
    }

    #[inline]
    fn wxyz(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_01_00_11) })
    }

    #[inline]
    fn wxyw(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_01_00_11) })
    }

    #[inline]
    fn wxzx(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_10_00_11) })
    }

    #[inline]
    fn wxzy(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_10_00_11) })
    }

    #[inline]
    fn wxzz(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_10_00_11) })
    }

    #[inline]
    fn wxzw(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_10_00_11) })
    }

    #[inline]
    fn wxwx(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_11_00_11) })
    }

    #[inline]
    fn wxwy(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_11_00_11) })
    }

    #[inline]
    fn wxwz(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_11_00_11) })
    }

    #[inline]
    fn wxww(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_11_00_11) })
    }

    #[inline]
    fn wyxx(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_00_01_11) })
    }

    #[inline]
    fn wyxy(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_00_01_11) })
    }

    #[inline]
    fn wyxz(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_00_01_11) })
    }

    #[inline]
    fn wyxw(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_00_01_11) })
    }

    #[inline]
    fn wyyx(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_01_01_11) })
    }

    #[inline]
    fn wyyy(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_01_01_11) })
    }

    #[inline]
    fn wyyz(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_01_01_11) })
    }

    #[inline]
    fn wyyw(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_01_01_11) })
    }

    #[inline]
    fn wyzx(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_10_01_11) })
    }

    #[inline]
    fn wyzy(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_10_01_11) })
    }

    #[inline]
    fn wyzz(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_10_01_11) })
    }

    #[inline]
    fn wyzw(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_10_01_11) })
    }

    #[inline]
    fn wywx(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_11_01_11) })
    }

    #[inline]
    fn wywy(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_11_01_11) })
    }

    #[inline]
    fn wywz(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_11_01_11) })
    }

    #[inline]
    fn wyww(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_11_01_11) })
    }

    #[inline]
    fn wzxx(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_00_10_11) })
    }

    #[inline]
    fn wzxy(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_00_10_11) })
    }

    #[inline]
    fn wzxz(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_00_10_11) })
    }

    #[inline]
    fn wzxw(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_00_10_11) })
    }

    #[inline]
    fn wzyx(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_01_10_11) })
    }

    #[inline]
    fn wzyy(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_01_10_11) })
    }

    #[inline]
    fn wzyz(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_01_10_11) })
    }

    #[inline]
    fn wzyw(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_01_10_11) })
    }

    #[inline]
    fn wzzx(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_10_10_11) })
    }

    #[inline]
    fn wzzy(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_10_10_11) })
    }

    #[inline]
    fn wzzz(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_10_10_11) })
    }

    #[inline]
    fn wzzw(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_10_10_11) })
    }

    #[inline]
    fn wzwx(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_11_10_11) })
    }

    #[inline]
    fn wzwy(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_11_10_11) })
    }

    #[inline]
    fn wzwz(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_11_10_11) })
    }

    #[inline]
    fn wzww(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_11_10_11) })
    }

    #[inline]
    fn wwxx(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_00_11_11) })
    }

    #[inline]
    fn wwxy(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_00_11_11) })
    }

    #[inline]
    fn wwxz(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_00_11_11) })
    }

    #[inline]
    fn wwxw(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_00_11_11) })
    }

    #[inline]
    fn wwyx(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_01_11_11) })
    }

    #[inline]
    fn wwyy(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_01_11_11) })
    }

    #[inline]
    fn wwyz(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_01_11_11) })
    }

    #[inline]
    fn wwyw(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_01_11_11) })
    }

    #[inline]
    fn wwzx(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_10_11_11) })
    }

    #[inline]
    fn wwzy(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_10_11_11) })
    }

    #[inline]
    fn wwzz(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_10_11_11) })
    }

    #[inline]
    fn wwzw(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_10_11_11) })
    }

    #[inline]
    fn wwwx(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b00_11_11_11) })
    }

    #[inline]
    fn wwwy(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b01_11_11_11) })
    }

    #[inline]
    fn wwwz(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b10_11_11_11) })
    }

    #[inline]
    fn wwww(self) -> Self {
        Self(unsafe { _mm_shuffle_ps(self.0, self.0, 0b11_11_11_11) })
    }
}
