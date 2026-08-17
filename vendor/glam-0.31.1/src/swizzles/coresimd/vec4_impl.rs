// Generated from swizzle_impl.rs.tera template. Edit the template, not the generated file.

#![allow(clippy::useless_conversion)]

use crate::{Vec2, Vec3, Vec4, Vec4Swizzles};

use core::simd::*;

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
        Self(simd_swizzle!(self.0, [0, 0, 0, 0]))
    }

    #[inline]
    fn xxxy(self) -> Self {
        Self(simd_swizzle!(self.0, [0, 0, 0, 1]))
    }

    #[inline]
    fn xxxz(self) -> Self {
        Self(simd_swizzle!(self.0, [0, 0, 0, 2]))
    }

    #[inline]
    fn xxxw(self) -> Self {
        Self(simd_swizzle!(self.0, [0, 0, 0, 3]))
    }

    #[inline]
    fn xxyx(self) -> Self {
        Self(simd_swizzle!(self.0, [0, 0, 1, 0]))
    }

    #[inline]
    fn xxyy(self) -> Self {
        Self(simd_swizzle!(self.0, [0, 0, 1, 1]))
    }

    #[inline]
    fn xxyz(self) -> Self {
        Self(simd_swizzle!(self.0, [0, 0, 1, 2]))
    }

    #[inline]
    fn xxyw(self) -> Self {
        Self(simd_swizzle!(self.0, [0, 0, 1, 3]))
    }

    #[inline]
    fn xxzx(self) -> Self {
        Self(simd_swizzle!(self.0, [0, 0, 2, 0]))
    }

    #[inline]
    fn xxzy(self) -> Self {
        Self(simd_swizzle!(self.0, [0, 0, 2, 1]))
    }

    #[inline]
    fn xxzz(self) -> Self {
        Self(simd_swizzle!(self.0, [0, 0, 2, 2]))
    }

    #[inline]
    fn xxzw(self) -> Self {
        Self(simd_swizzle!(self.0, [0, 0, 2, 3]))
    }

    #[inline]
    fn xxwx(self) -> Self {
        Self(simd_swizzle!(self.0, [0, 0, 3, 0]))
    }

    #[inline]
    fn xxwy(self) -> Self {
        Self(simd_swizzle!(self.0, [0, 0, 3, 1]))
    }

    #[inline]
    fn xxwz(self) -> Self {
        Self(simd_swizzle!(self.0, [0, 0, 3, 2]))
    }

    #[inline]
    fn xxww(self) -> Self {
        Self(simd_swizzle!(self.0, [0, 0, 3, 3]))
    }

    #[inline]
    fn xyxx(self) -> Self {
        Self(simd_swizzle!(self.0, [0, 1, 0, 0]))
    }

    #[inline]
    fn xyxy(self) -> Self {
        Self(simd_swizzle!(self.0, [0, 1, 0, 1]))
    }

    #[inline]
    fn xyxz(self) -> Self {
        Self(simd_swizzle!(self.0, [0, 1, 0, 2]))
    }

    #[inline]
    fn xyxw(self) -> Self {
        Self(simd_swizzle!(self.0, [0, 1, 0, 3]))
    }

    #[inline]
    fn xyyx(self) -> Self {
        Self(simd_swizzle!(self.0, [0, 1, 1, 0]))
    }

    #[inline]
    fn xyyy(self) -> Self {
        Self(simd_swizzle!(self.0, [0, 1, 1, 1]))
    }

    #[inline]
    fn xyyz(self) -> Self {
        Self(simd_swizzle!(self.0, [0, 1, 1, 2]))
    }

    #[inline]
    fn xyyw(self) -> Self {
        Self(simd_swizzle!(self.0, [0, 1, 1, 3]))
    }

    #[inline]
    fn xyzx(self) -> Self {
        Self(simd_swizzle!(self.0, [0, 1, 2, 0]))
    }

    #[inline]
    fn xyzy(self) -> Self {
        Self(simd_swizzle!(self.0, [0, 1, 2, 1]))
    }

    #[inline]
    fn xyzz(self) -> Self {
        Self(simd_swizzle!(self.0, [0, 1, 2, 2]))
    }

    #[inline]
    fn xywx(self) -> Self {
        Self(simd_swizzle!(self.0, [0, 1, 3, 0]))
    }

    #[inline]
    fn xywy(self) -> Self {
        Self(simd_swizzle!(self.0, [0, 1, 3, 1]))
    }

    #[inline]
    fn xywz(self) -> Self {
        Self(simd_swizzle!(self.0, [0, 1, 3, 2]))
    }

    #[inline]
    fn xyww(self) -> Self {
        Self(simd_swizzle!(self.0, [0, 1, 3, 3]))
    }

    #[inline]
    fn xzxx(self) -> Self {
        Self(simd_swizzle!(self.0, [0, 2, 0, 0]))
    }

    #[inline]
    fn xzxy(self) -> Self {
        Self(simd_swizzle!(self.0, [0, 2, 0, 1]))
    }

    #[inline]
    fn xzxz(self) -> Self {
        Self(simd_swizzle!(self.0, [0, 2, 0, 2]))
    }

    #[inline]
    fn xzxw(self) -> Self {
        Self(simd_swizzle!(self.0, [0, 2, 0, 3]))
    }

    #[inline]
    fn xzyx(self) -> Self {
        Self(simd_swizzle!(self.0, [0, 2, 1, 0]))
    }

    #[inline]
    fn xzyy(self) -> Self {
        Self(simd_swizzle!(self.0, [0, 2, 1, 1]))
    }

    #[inline]
    fn xzyz(self) -> Self {
        Self(simd_swizzle!(self.0, [0, 2, 1, 2]))
    }

    #[inline]
    fn xzyw(self) -> Self {
        Self(simd_swizzle!(self.0, [0, 2, 1, 3]))
    }

    #[inline]
    fn xzzx(self) -> Self {
        Self(simd_swizzle!(self.0, [0, 2, 2, 0]))
    }

    #[inline]
    fn xzzy(self) -> Self {
        Self(simd_swizzle!(self.0, [0, 2, 2, 1]))
    }

    #[inline]
    fn xzzz(self) -> Self {
        Self(simd_swizzle!(self.0, [0, 2, 2, 2]))
    }

    #[inline]
    fn xzzw(self) -> Self {
        Self(simd_swizzle!(self.0, [0, 2, 2, 3]))
    }

    #[inline]
    fn xzwx(self) -> Self {
        Self(simd_swizzle!(self.0, [0, 2, 3, 0]))
    }

    #[inline]
    fn xzwy(self) -> Self {
        Self(simd_swizzle!(self.0, [0, 2, 3, 1]))
    }

    #[inline]
    fn xzwz(self) -> Self {
        Self(simd_swizzle!(self.0, [0, 2, 3, 2]))
    }

    #[inline]
    fn xzww(self) -> Self {
        Self(simd_swizzle!(self.0, [0, 2, 3, 3]))
    }

    #[inline]
    fn xwxx(self) -> Self {
        Self(simd_swizzle!(self.0, [0, 3, 0, 0]))
    }

    #[inline]
    fn xwxy(self) -> Self {
        Self(simd_swizzle!(self.0, [0, 3, 0, 1]))
    }

    #[inline]
    fn xwxz(self) -> Self {
        Self(simd_swizzle!(self.0, [0, 3, 0, 2]))
    }

    #[inline]
    fn xwxw(self) -> Self {
        Self(simd_swizzle!(self.0, [0, 3, 0, 3]))
    }

    #[inline]
    fn xwyx(self) -> Self {
        Self(simd_swizzle!(self.0, [0, 3, 1, 0]))
    }

    #[inline]
    fn xwyy(self) -> Self {
        Self(simd_swizzle!(self.0, [0, 3, 1, 1]))
    }

    #[inline]
    fn xwyz(self) -> Self {
        Self(simd_swizzle!(self.0, [0, 3, 1, 2]))
    }

    #[inline]
    fn xwyw(self) -> Self {
        Self(simd_swizzle!(self.0, [0, 3, 1, 3]))
    }

    #[inline]
    fn xwzx(self) -> Self {
        Self(simd_swizzle!(self.0, [0, 3, 2, 0]))
    }

    #[inline]
    fn xwzy(self) -> Self {
        Self(simd_swizzle!(self.0, [0, 3, 2, 1]))
    }

    #[inline]
    fn xwzz(self) -> Self {
        Self(simd_swizzle!(self.0, [0, 3, 2, 2]))
    }

    #[inline]
    fn xwzw(self) -> Self {
        Self(simd_swizzle!(self.0, [0, 3, 2, 3]))
    }

    #[inline]
    fn xwwx(self) -> Self {
        Self(simd_swizzle!(self.0, [0, 3, 3, 0]))
    }

    #[inline]
    fn xwwy(self) -> Self {
        Self(simd_swizzle!(self.0, [0, 3, 3, 1]))
    }

    #[inline]
    fn xwwz(self) -> Self {
        Self(simd_swizzle!(self.0, [0, 3, 3, 2]))
    }

    #[inline]
    fn xwww(self) -> Self {
        Self(simd_swizzle!(self.0, [0, 3, 3, 3]))
    }

    #[inline]
    fn yxxx(self) -> Self {
        Self(simd_swizzle!(self.0, [1, 0, 0, 0]))
    }

    #[inline]
    fn yxxy(self) -> Self {
        Self(simd_swizzle!(self.0, [1, 0, 0, 1]))
    }

    #[inline]
    fn yxxz(self) -> Self {
        Self(simd_swizzle!(self.0, [1, 0, 0, 2]))
    }

    #[inline]
    fn yxxw(self) -> Self {
        Self(simd_swizzle!(self.0, [1, 0, 0, 3]))
    }

    #[inline]
    fn yxyx(self) -> Self {
        Self(simd_swizzle!(self.0, [1, 0, 1, 0]))
    }

    #[inline]
    fn yxyy(self) -> Self {
        Self(simd_swizzle!(self.0, [1, 0, 1, 1]))
    }

    #[inline]
    fn yxyz(self) -> Self {
        Self(simd_swizzle!(self.0, [1, 0, 1, 2]))
    }

    #[inline]
    fn yxyw(self) -> Self {
        Self(simd_swizzle!(self.0, [1, 0, 1, 3]))
    }

    #[inline]
    fn yxzx(self) -> Self {
        Self(simd_swizzle!(self.0, [1, 0, 2, 0]))
    }

    #[inline]
    fn yxzy(self) -> Self {
        Self(simd_swizzle!(self.0, [1, 0, 2, 1]))
    }

    #[inline]
    fn yxzz(self) -> Self {
        Self(simd_swizzle!(self.0, [1, 0, 2, 2]))
    }

    #[inline]
    fn yxzw(self) -> Self {
        Self(simd_swizzle!(self.0, [1, 0, 2, 3]))
    }

    #[inline]
    fn yxwx(self) -> Self {
        Self(simd_swizzle!(self.0, [1, 0, 3, 0]))
    }

    #[inline]
    fn yxwy(self) -> Self {
        Self(simd_swizzle!(self.0, [1, 0, 3, 1]))
    }

    #[inline]
    fn yxwz(self) -> Self {
        Self(simd_swizzle!(self.0, [1, 0, 3, 2]))
    }

    #[inline]
    fn yxww(self) -> Self {
        Self(simd_swizzle!(self.0, [1, 0, 3, 3]))
    }

    #[inline]
    fn yyxx(self) -> Self {
        Self(simd_swizzle!(self.0, [1, 1, 0, 0]))
    }

    #[inline]
    fn yyxy(self) -> Self {
        Self(simd_swizzle!(self.0, [1, 1, 0, 1]))
    }

    #[inline]
    fn yyxz(self) -> Self {
        Self(simd_swizzle!(self.0, [1, 1, 0, 2]))
    }

    #[inline]
    fn yyxw(self) -> Self {
        Self(simd_swizzle!(self.0, [1, 1, 0, 3]))
    }

    #[inline]
    fn yyyx(self) -> Self {
        Self(simd_swizzle!(self.0, [1, 1, 1, 0]))
    }

    #[inline]
    fn yyyy(self) -> Self {
        Self(simd_swizzle!(self.0, [1, 1, 1, 1]))
    }

    #[inline]
    fn yyyz(self) -> Self {
        Self(simd_swizzle!(self.0, [1, 1, 1, 2]))
    }

    #[inline]
    fn yyyw(self) -> Self {
        Self(simd_swizzle!(self.0, [1, 1, 1, 3]))
    }

    #[inline]
    fn yyzx(self) -> Self {
        Self(simd_swizzle!(self.0, [1, 1, 2, 0]))
    }

    #[inline]
    fn yyzy(self) -> Self {
        Self(simd_swizzle!(self.0, [1, 1, 2, 1]))
    }

    #[inline]
    fn yyzz(self) -> Self {
        Self(simd_swizzle!(self.0, [1, 1, 2, 2]))
    }

    #[inline]
    fn yyzw(self) -> Self {
        Self(simd_swizzle!(self.0, [1, 1, 2, 3]))
    }

    #[inline]
    fn yywx(self) -> Self {
        Self(simd_swizzle!(self.0, [1, 1, 3, 0]))
    }

    #[inline]
    fn yywy(self) -> Self {
        Self(simd_swizzle!(self.0, [1, 1, 3, 1]))
    }

    #[inline]
    fn yywz(self) -> Self {
        Self(simd_swizzle!(self.0, [1, 1, 3, 2]))
    }

    #[inline]
    fn yyww(self) -> Self {
        Self(simd_swizzle!(self.0, [1, 1, 3, 3]))
    }

    #[inline]
    fn yzxx(self) -> Self {
        Self(simd_swizzle!(self.0, [1, 2, 0, 0]))
    }

    #[inline]
    fn yzxy(self) -> Self {
        Self(simd_swizzle!(self.0, [1, 2, 0, 1]))
    }

    #[inline]
    fn yzxz(self) -> Self {
        Self(simd_swizzle!(self.0, [1, 2, 0, 2]))
    }

    #[inline]
    fn yzxw(self) -> Self {
        Self(simd_swizzle!(self.0, [1, 2, 0, 3]))
    }

    #[inline]
    fn yzyx(self) -> Self {
        Self(simd_swizzle!(self.0, [1, 2, 1, 0]))
    }

    #[inline]
    fn yzyy(self) -> Self {
        Self(simd_swizzle!(self.0, [1, 2, 1, 1]))
    }

    #[inline]
    fn yzyz(self) -> Self {
        Self(simd_swizzle!(self.0, [1, 2, 1, 2]))
    }

    #[inline]
    fn yzyw(self) -> Self {
        Self(simd_swizzle!(self.0, [1, 2, 1, 3]))
    }

    #[inline]
    fn yzzx(self) -> Self {
        Self(simd_swizzle!(self.0, [1, 2, 2, 0]))
    }

    #[inline]
    fn yzzy(self) -> Self {
        Self(simd_swizzle!(self.0, [1, 2, 2, 1]))
    }

    #[inline]
    fn yzzz(self) -> Self {
        Self(simd_swizzle!(self.0, [1, 2, 2, 2]))
    }

    #[inline]
    fn yzzw(self) -> Self {
        Self(simd_swizzle!(self.0, [1, 2, 2, 3]))
    }

    #[inline]
    fn yzwx(self) -> Self {
        Self(simd_swizzle!(self.0, [1, 2, 3, 0]))
    }

    #[inline]
    fn yzwy(self) -> Self {
        Self(simd_swizzle!(self.0, [1, 2, 3, 1]))
    }

    #[inline]
    fn yzwz(self) -> Self {
        Self(simd_swizzle!(self.0, [1, 2, 3, 2]))
    }

    #[inline]
    fn yzww(self) -> Self {
        Self(simd_swizzle!(self.0, [1, 2, 3, 3]))
    }

    #[inline]
    fn ywxx(self) -> Self {
        Self(simd_swizzle!(self.0, [1, 3, 0, 0]))
    }

    #[inline]
    fn ywxy(self) -> Self {
        Self(simd_swizzle!(self.0, [1, 3, 0, 1]))
    }

    #[inline]
    fn ywxz(self) -> Self {
        Self(simd_swizzle!(self.0, [1, 3, 0, 2]))
    }

    #[inline]
    fn ywxw(self) -> Self {
        Self(simd_swizzle!(self.0, [1, 3, 0, 3]))
    }

    #[inline]
    fn ywyx(self) -> Self {
        Self(simd_swizzle!(self.0, [1, 3, 1, 0]))
    }

    #[inline]
    fn ywyy(self) -> Self {
        Self(simd_swizzle!(self.0, [1, 3, 1, 1]))
    }

    #[inline]
    fn ywyz(self) -> Self {
        Self(simd_swizzle!(self.0, [1, 3, 1, 2]))
    }

    #[inline]
    fn ywyw(self) -> Self {
        Self(simd_swizzle!(self.0, [1, 3, 1, 3]))
    }

    #[inline]
    fn ywzx(self) -> Self {
        Self(simd_swizzle!(self.0, [1, 3, 2, 0]))
    }

    #[inline]
    fn ywzy(self) -> Self {
        Self(simd_swizzle!(self.0, [1, 3, 2, 1]))
    }

    #[inline]
    fn ywzz(self) -> Self {
        Self(simd_swizzle!(self.0, [1, 3, 2, 2]))
    }

    #[inline]
    fn ywzw(self) -> Self {
        Self(simd_swizzle!(self.0, [1, 3, 2, 3]))
    }

    #[inline]
    fn ywwx(self) -> Self {
        Self(simd_swizzle!(self.0, [1, 3, 3, 0]))
    }

    #[inline]
    fn ywwy(self) -> Self {
        Self(simd_swizzle!(self.0, [1, 3, 3, 1]))
    }

    #[inline]
    fn ywwz(self) -> Self {
        Self(simd_swizzle!(self.0, [1, 3, 3, 2]))
    }

    #[inline]
    fn ywww(self) -> Self {
        Self(simd_swizzle!(self.0, [1, 3, 3, 3]))
    }

    #[inline]
    fn zxxx(self) -> Self {
        Self(simd_swizzle!(self.0, [2, 0, 0, 0]))
    }

    #[inline]
    fn zxxy(self) -> Self {
        Self(simd_swizzle!(self.0, [2, 0, 0, 1]))
    }

    #[inline]
    fn zxxz(self) -> Self {
        Self(simd_swizzle!(self.0, [2, 0, 0, 2]))
    }

    #[inline]
    fn zxxw(self) -> Self {
        Self(simd_swizzle!(self.0, [2, 0, 0, 3]))
    }

    #[inline]
    fn zxyx(self) -> Self {
        Self(simd_swizzle!(self.0, [2, 0, 1, 0]))
    }

    #[inline]
    fn zxyy(self) -> Self {
        Self(simd_swizzle!(self.0, [2, 0, 1, 1]))
    }

    #[inline]
    fn zxyz(self) -> Self {
        Self(simd_swizzle!(self.0, [2, 0, 1, 2]))
    }

    #[inline]
    fn zxyw(self) -> Self {
        Self(simd_swizzle!(self.0, [2, 0, 1, 3]))
    }

    #[inline]
    fn zxzx(self) -> Self {
        Self(simd_swizzle!(self.0, [2, 0, 2, 0]))
    }

    #[inline]
    fn zxzy(self) -> Self {
        Self(simd_swizzle!(self.0, [2, 0, 2, 1]))
    }

    #[inline]
    fn zxzz(self) -> Self {
        Self(simd_swizzle!(self.0, [2, 0, 2, 2]))
    }

    #[inline]
    fn zxzw(self) -> Self {
        Self(simd_swizzle!(self.0, [2, 0, 2, 3]))
    }

    #[inline]
    fn zxwx(self) -> Self {
        Self(simd_swizzle!(self.0, [2, 0, 3, 0]))
    }

    #[inline]
    fn zxwy(self) -> Self {
        Self(simd_swizzle!(self.0, [2, 0, 3, 1]))
    }

    #[inline]
    fn zxwz(self) -> Self {
        Self(simd_swizzle!(self.0, [2, 0, 3, 2]))
    }

    #[inline]
    fn zxww(self) -> Self {
        Self(simd_swizzle!(self.0, [2, 0, 3, 3]))
    }

    #[inline]
    fn zyxx(self) -> Self {
        Self(simd_swizzle!(self.0, [2, 1, 0, 0]))
    }

    #[inline]
    fn zyxy(self) -> Self {
        Self(simd_swizzle!(self.0, [2, 1, 0, 1]))
    }

    #[inline]
    fn zyxz(self) -> Self {
        Self(simd_swizzle!(self.0, [2, 1, 0, 2]))
    }

    #[inline]
    fn zyxw(self) -> Self {
        Self(simd_swizzle!(self.0, [2, 1, 0, 3]))
    }

    #[inline]
    fn zyyx(self) -> Self {
        Self(simd_swizzle!(self.0, [2, 1, 1, 0]))
    }

    #[inline]
    fn zyyy(self) -> Self {
        Self(simd_swizzle!(self.0, [2, 1, 1, 1]))
    }

    #[inline]
    fn zyyz(self) -> Self {
        Self(simd_swizzle!(self.0, [2, 1, 1, 2]))
    }

    #[inline]
    fn zyyw(self) -> Self {
        Self(simd_swizzle!(self.0, [2, 1, 1, 3]))
    }

    #[inline]
    fn zyzx(self) -> Self {
        Self(simd_swizzle!(self.0, [2, 1, 2, 0]))
    }

    #[inline]
    fn zyzy(self) -> Self {
        Self(simd_swizzle!(self.0, [2, 1, 2, 1]))
    }

    #[inline]
    fn zyzz(self) -> Self {
        Self(simd_swizzle!(self.0, [2, 1, 2, 2]))
    }

    #[inline]
    fn zyzw(self) -> Self {
        Self(simd_swizzle!(self.0, [2, 1, 2, 3]))
    }

    #[inline]
    fn zywx(self) -> Self {
        Self(simd_swizzle!(self.0, [2, 1, 3, 0]))
    }

    #[inline]
    fn zywy(self) -> Self {
        Self(simd_swizzle!(self.0, [2, 1, 3, 1]))
    }

    #[inline]
    fn zywz(self) -> Self {
        Self(simd_swizzle!(self.0, [2, 1, 3, 2]))
    }

    #[inline]
    fn zyww(self) -> Self {
        Self(simd_swizzle!(self.0, [2, 1, 3, 3]))
    }

    #[inline]
    fn zzxx(self) -> Self {
        Self(simd_swizzle!(self.0, [2, 2, 0, 0]))
    }

    #[inline]
    fn zzxy(self) -> Self {
        Self(simd_swizzle!(self.0, [2, 2, 0, 1]))
    }

    #[inline]
    fn zzxz(self) -> Self {
        Self(simd_swizzle!(self.0, [2, 2, 0, 2]))
    }

    #[inline]
    fn zzxw(self) -> Self {
        Self(simd_swizzle!(self.0, [2, 2, 0, 3]))
    }

    #[inline]
    fn zzyx(self) -> Self {
        Self(simd_swizzle!(self.0, [2, 2, 1, 0]))
    }

    #[inline]
    fn zzyy(self) -> Self {
        Self(simd_swizzle!(self.0, [2, 2, 1, 1]))
    }

    #[inline]
    fn zzyz(self) -> Self {
        Self(simd_swizzle!(self.0, [2, 2, 1, 2]))
    }

    #[inline]
    fn zzyw(self) -> Self {
        Self(simd_swizzle!(self.0, [2, 2, 1, 3]))
    }

    #[inline]
    fn zzzx(self) -> Self {
        Self(simd_swizzle!(self.0, [2, 2, 2, 0]))
    }

    #[inline]
    fn zzzy(self) -> Self {
        Self(simd_swizzle!(self.0, [2, 2, 2, 1]))
    }

    #[inline]
    fn zzzz(self) -> Self {
        Self(simd_swizzle!(self.0, [2, 2, 2, 2]))
    }

    #[inline]
    fn zzzw(self) -> Self {
        Self(simd_swizzle!(self.0, [2, 2, 2, 3]))
    }

    #[inline]
    fn zzwx(self) -> Self {
        Self(simd_swizzle!(self.0, [2, 2, 3, 0]))
    }

    #[inline]
    fn zzwy(self) -> Self {
        Self(simd_swizzle!(self.0, [2, 2, 3, 1]))
    }

    #[inline]
    fn zzwz(self) -> Self {
        Self(simd_swizzle!(self.0, [2, 2, 3, 2]))
    }

    #[inline]
    fn zzww(self) -> Self {
        Self(simd_swizzle!(self.0, [2, 2, 3, 3]))
    }

    #[inline]
    fn zwxx(self) -> Self {
        Self(simd_swizzle!(self.0, [2, 3, 0, 0]))
    }

    #[inline]
    fn zwxy(self) -> Self {
        Self(simd_swizzle!(self.0, [2, 3, 0, 1]))
    }

    #[inline]
    fn zwxz(self) -> Self {
        Self(simd_swizzle!(self.0, [2, 3, 0, 2]))
    }

    #[inline]
    fn zwxw(self) -> Self {
        Self(simd_swizzle!(self.0, [2, 3, 0, 3]))
    }

    #[inline]
    fn zwyx(self) -> Self {
        Self(simd_swizzle!(self.0, [2, 3, 1, 0]))
    }

    #[inline]
    fn zwyy(self) -> Self {
        Self(simd_swizzle!(self.0, [2, 3, 1, 1]))
    }

    #[inline]
    fn zwyz(self) -> Self {
        Self(simd_swizzle!(self.0, [2, 3, 1, 2]))
    }

    #[inline]
    fn zwyw(self) -> Self {
        Self(simd_swizzle!(self.0, [2, 3, 1, 3]))
    }

    #[inline]
    fn zwzx(self) -> Self {
        Self(simd_swizzle!(self.0, [2, 3, 2, 0]))
    }

    #[inline]
    fn zwzy(self) -> Self {
        Self(simd_swizzle!(self.0, [2, 3, 2, 1]))
    }

    #[inline]
    fn zwzz(self) -> Self {
        Self(simd_swizzle!(self.0, [2, 3, 2, 2]))
    }

    #[inline]
    fn zwzw(self) -> Self {
        Self(simd_swizzle!(self.0, [2, 3, 2, 3]))
    }

    #[inline]
    fn zwwx(self) -> Self {
        Self(simd_swizzle!(self.0, [2, 3, 3, 0]))
    }

    #[inline]
    fn zwwy(self) -> Self {
        Self(simd_swizzle!(self.0, [2, 3, 3, 1]))
    }

    #[inline]
    fn zwwz(self) -> Self {
        Self(simd_swizzle!(self.0, [2, 3, 3, 2]))
    }

    #[inline]
    fn zwww(self) -> Self {
        Self(simd_swizzle!(self.0, [2, 3, 3, 3]))
    }

    #[inline]
    fn wxxx(self) -> Self {
        Self(simd_swizzle!(self.0, [3, 0, 0, 0]))
    }

    #[inline]
    fn wxxy(self) -> Self {
        Self(simd_swizzle!(self.0, [3, 0, 0, 1]))
    }

    #[inline]
    fn wxxz(self) -> Self {
        Self(simd_swizzle!(self.0, [3, 0, 0, 2]))
    }

    #[inline]
    fn wxxw(self) -> Self {
        Self(simd_swizzle!(self.0, [3, 0, 0, 3]))
    }

    #[inline]
    fn wxyx(self) -> Self {
        Self(simd_swizzle!(self.0, [3, 0, 1, 0]))
    }

    #[inline]
    fn wxyy(self) -> Self {
        Self(simd_swizzle!(self.0, [3, 0, 1, 1]))
    }

    #[inline]
    fn wxyz(self) -> Self {
        Self(simd_swizzle!(self.0, [3, 0, 1, 2]))
    }

    #[inline]
    fn wxyw(self) -> Self {
        Self(simd_swizzle!(self.0, [3, 0, 1, 3]))
    }

    #[inline]
    fn wxzx(self) -> Self {
        Self(simd_swizzle!(self.0, [3, 0, 2, 0]))
    }

    #[inline]
    fn wxzy(self) -> Self {
        Self(simd_swizzle!(self.0, [3, 0, 2, 1]))
    }

    #[inline]
    fn wxzz(self) -> Self {
        Self(simd_swizzle!(self.0, [3, 0, 2, 2]))
    }

    #[inline]
    fn wxzw(self) -> Self {
        Self(simd_swizzle!(self.0, [3, 0, 2, 3]))
    }

    #[inline]
    fn wxwx(self) -> Self {
        Self(simd_swizzle!(self.0, [3, 0, 3, 0]))
    }

    #[inline]
    fn wxwy(self) -> Self {
        Self(simd_swizzle!(self.0, [3, 0, 3, 1]))
    }

    #[inline]
    fn wxwz(self) -> Self {
        Self(simd_swizzle!(self.0, [3, 0, 3, 2]))
    }

    #[inline]
    fn wxww(self) -> Self {
        Self(simd_swizzle!(self.0, [3, 0, 3, 3]))
    }

    #[inline]
    fn wyxx(self) -> Self {
        Self(simd_swizzle!(self.0, [3, 1, 0, 0]))
    }

    #[inline]
    fn wyxy(self) -> Self {
        Self(simd_swizzle!(self.0, [3, 1, 0, 1]))
    }

    #[inline]
    fn wyxz(self) -> Self {
        Self(simd_swizzle!(self.0, [3, 1, 0, 2]))
    }

    #[inline]
    fn wyxw(self) -> Self {
        Self(simd_swizzle!(self.0, [3, 1, 0, 3]))
    }

    #[inline]
    fn wyyx(self) -> Self {
        Self(simd_swizzle!(self.0, [3, 1, 1, 0]))
    }

    #[inline]
    fn wyyy(self) -> Self {
        Self(simd_swizzle!(self.0, [3, 1, 1, 1]))
    }

    #[inline]
    fn wyyz(self) -> Self {
        Self(simd_swizzle!(self.0, [3, 1, 1, 2]))
    }

    #[inline]
    fn wyyw(self) -> Self {
        Self(simd_swizzle!(self.0, [3, 1, 1, 3]))
    }

    #[inline]
    fn wyzx(self) -> Self {
        Self(simd_swizzle!(self.0, [3, 1, 2, 0]))
    }

    #[inline]
    fn wyzy(self) -> Self {
        Self(simd_swizzle!(self.0, [3, 1, 2, 1]))
    }

    #[inline]
    fn wyzz(self) -> Self {
        Self(simd_swizzle!(self.0, [3, 1, 2, 2]))
    }

    #[inline]
    fn wyzw(self) -> Self {
        Self(simd_swizzle!(self.0, [3, 1, 2, 3]))
    }

    #[inline]
    fn wywx(self) -> Self {
        Self(simd_swizzle!(self.0, [3, 1, 3, 0]))
    }

    #[inline]
    fn wywy(self) -> Self {
        Self(simd_swizzle!(self.0, [3, 1, 3, 1]))
    }

    #[inline]
    fn wywz(self) -> Self {
        Self(simd_swizzle!(self.0, [3, 1, 3, 2]))
    }

    #[inline]
    fn wyww(self) -> Self {
        Self(simd_swizzle!(self.0, [3, 1, 3, 3]))
    }

    #[inline]
    fn wzxx(self) -> Self {
        Self(simd_swizzle!(self.0, [3, 2, 0, 0]))
    }

    #[inline]
    fn wzxy(self) -> Self {
        Self(simd_swizzle!(self.0, [3, 2, 0, 1]))
    }

    #[inline]
    fn wzxz(self) -> Self {
        Self(simd_swizzle!(self.0, [3, 2, 0, 2]))
    }

    #[inline]
    fn wzxw(self) -> Self {
        Self(simd_swizzle!(self.0, [3, 2, 0, 3]))
    }

    #[inline]
    fn wzyx(self) -> Self {
        Self(simd_swizzle!(self.0, [3, 2, 1, 0]))
    }

    #[inline]
    fn wzyy(self) -> Self {
        Self(simd_swizzle!(self.0, [3, 2, 1, 1]))
    }

    #[inline]
    fn wzyz(self) -> Self {
        Self(simd_swizzle!(self.0, [3, 2, 1, 2]))
    }

    #[inline]
    fn wzyw(self) -> Self {
        Self(simd_swizzle!(self.0, [3, 2, 1, 3]))
    }

    #[inline]
    fn wzzx(self) -> Self {
        Self(simd_swizzle!(self.0, [3, 2, 2, 0]))
    }

    #[inline]
    fn wzzy(self) -> Self {
        Self(simd_swizzle!(self.0, [3, 2, 2, 1]))
    }

    #[inline]
    fn wzzz(self) -> Self {
        Self(simd_swizzle!(self.0, [3, 2, 2, 2]))
    }

    #[inline]
    fn wzzw(self) -> Self {
        Self(simd_swizzle!(self.0, [3, 2, 2, 3]))
    }

    #[inline]
    fn wzwx(self) -> Self {
        Self(simd_swizzle!(self.0, [3, 2, 3, 0]))
    }

    #[inline]
    fn wzwy(self) -> Self {
        Self(simd_swizzle!(self.0, [3, 2, 3, 1]))
    }

    #[inline]
    fn wzwz(self) -> Self {
        Self(simd_swizzle!(self.0, [3, 2, 3, 2]))
    }

    #[inline]
    fn wzww(self) -> Self {
        Self(simd_swizzle!(self.0, [3, 2, 3, 3]))
    }

    #[inline]
    fn wwxx(self) -> Self {
        Self(simd_swizzle!(self.0, [3, 3, 0, 0]))
    }

    #[inline]
    fn wwxy(self) -> Self {
        Self(simd_swizzle!(self.0, [3, 3, 0, 1]))
    }

    #[inline]
    fn wwxz(self) -> Self {
        Self(simd_swizzle!(self.0, [3, 3, 0, 2]))
    }

    #[inline]
    fn wwxw(self) -> Self {
        Self(simd_swizzle!(self.0, [3, 3, 0, 3]))
    }

    #[inline]
    fn wwyx(self) -> Self {
        Self(simd_swizzle!(self.0, [3, 3, 1, 0]))
    }

    #[inline]
    fn wwyy(self) -> Self {
        Self(simd_swizzle!(self.0, [3, 3, 1, 1]))
    }

    #[inline]
    fn wwyz(self) -> Self {
        Self(simd_swizzle!(self.0, [3, 3, 1, 2]))
    }

    #[inline]
    fn wwyw(self) -> Self {
        Self(simd_swizzle!(self.0, [3, 3, 1, 3]))
    }

    #[inline]
    fn wwzx(self) -> Self {
        Self(simd_swizzle!(self.0, [3, 3, 2, 0]))
    }

    #[inline]
    fn wwzy(self) -> Self {
        Self(simd_swizzle!(self.0, [3, 3, 2, 1]))
    }

    #[inline]
    fn wwzz(self) -> Self {
        Self(simd_swizzle!(self.0, [3, 3, 2, 2]))
    }

    #[inline]
    fn wwzw(self) -> Self {
        Self(simd_swizzle!(self.0, [3, 3, 2, 3]))
    }

    #[inline]
    fn wwwx(self) -> Self {
        Self(simd_swizzle!(self.0, [3, 3, 3, 0]))
    }

    #[inline]
    fn wwwy(self) -> Self {
        Self(simd_swizzle!(self.0, [3, 3, 3, 1]))
    }

    #[inline]
    fn wwwz(self) -> Self {
        Self(simd_swizzle!(self.0, [3, 3, 3, 2]))
    }

    #[inline]
    fn wwww(self) -> Self {
        Self(simd_swizzle!(self.0, [3, 3, 3, 3]))
    }
}
