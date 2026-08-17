// Generated from swizzle_impl.rs.tera template. Edit the template, not the generated file.

#![allow(clippy::useless_conversion)]

use crate::{Vec2, Vec3, Vec4, Vec4Swizzles};

use core::simd::*;

impl Vec4Swizzles for Vec4 {
    type Vec2 = Vec2;

    type Vec3 = Vec3;

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
    fn xw(self) -> Vec2 {
        Vec2 {
            x: self.x,
            y: self.w,
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
    fn yw(self) -> Vec2 {
        Vec2 {
            x: self.y,
            y: self.w,
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
    fn zw(self) -> Vec2 {
        Vec2 {
            x: self.z,
            y: self.w,
        }
    }

    #[inline]
    #[must_use]
    fn wx(self) -> Vec2 {
        Vec2 {
            x: self.w,
            y: self.x,
        }
    }

    #[inline]
    #[must_use]
    fn wy(self) -> Vec2 {
        Vec2 {
            x: self.w,
            y: self.y,
        }
    }

    #[inline]
    #[must_use]
    fn wz(self) -> Vec2 {
        Vec2 {
            x: self.w,
            y: self.z,
        }
    }

    #[inline]
    #[must_use]
    fn ww(self) -> Vec2 {
        Vec2 {
            x: self.w,
            y: self.w,
        }
    }

    #[inline]
    #[must_use]
    fn xxx(self) -> Vec3 {
        Vec3::new(self.x, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn xxy(self) -> Vec3 {
        Vec3::new(self.x, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn xxz(self) -> Vec3 {
        Vec3::new(self.x, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn xxw(self) -> Vec3 {
        Vec3::new(self.x, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn xyx(self) -> Vec3 {
        Vec3::new(self.x, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn xyy(self) -> Vec3 {
        Vec3::new(self.x, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn xyz(self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn xyw(self) -> Vec3 {
        Vec3::new(self.x, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn xzx(self) -> Vec3 {
        Vec3::new(self.x, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn xzy(self) -> Vec3 {
        Vec3::new(self.x, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn xzz(self) -> Vec3 {
        Vec3::new(self.x, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn xzw(self) -> Vec3 {
        Vec3::new(self.x, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn xwx(self) -> Vec3 {
        Vec3::new(self.x, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn xwy(self) -> Vec3 {
        Vec3::new(self.x, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn xwz(self) -> Vec3 {
        Vec3::new(self.x, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn xww(self) -> Vec3 {
        Vec3::new(self.x, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn yxx(self) -> Vec3 {
        Vec3::new(self.y, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn yxy(self) -> Vec3 {
        Vec3::new(self.y, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn yxz(self) -> Vec3 {
        Vec3::new(self.y, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn yxw(self) -> Vec3 {
        Vec3::new(self.y, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn yyx(self) -> Vec3 {
        Vec3::new(self.y, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn yyy(self) -> Vec3 {
        Vec3::new(self.y, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn yyz(self) -> Vec3 {
        Vec3::new(self.y, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn yyw(self) -> Vec3 {
        Vec3::new(self.y, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn yzx(self) -> Vec3 {
        Vec3::new(self.y, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn yzy(self) -> Vec3 {
        Vec3::new(self.y, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn yzz(self) -> Vec3 {
        Vec3::new(self.y, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn yzw(self) -> Vec3 {
        Vec3::new(self.y, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn ywx(self) -> Vec3 {
        Vec3::new(self.y, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn ywy(self) -> Vec3 {
        Vec3::new(self.y, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn ywz(self) -> Vec3 {
        Vec3::new(self.y, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn yww(self) -> Vec3 {
        Vec3::new(self.y, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn zxx(self) -> Vec3 {
        Vec3::new(self.z, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn zxy(self) -> Vec3 {
        Vec3::new(self.z, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn zxz(self) -> Vec3 {
        Vec3::new(self.z, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn zxw(self) -> Vec3 {
        Vec3::new(self.z, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn zyx(self) -> Vec3 {
        Vec3::new(self.z, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn zyy(self) -> Vec3 {
        Vec3::new(self.z, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn zyz(self) -> Vec3 {
        Vec3::new(self.z, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn zyw(self) -> Vec3 {
        Vec3::new(self.z, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn zzx(self) -> Vec3 {
        Vec3::new(self.z, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn zzy(self) -> Vec3 {
        Vec3::new(self.z, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn zzz(self) -> Vec3 {
        Vec3::new(self.z, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn zzw(self) -> Vec3 {
        Vec3::new(self.z, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn zwx(self) -> Vec3 {
        Vec3::new(self.z, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn zwy(self) -> Vec3 {
        Vec3::new(self.z, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn zwz(self) -> Vec3 {
        Vec3::new(self.z, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn zww(self) -> Vec3 {
        Vec3::new(self.z, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn wxx(self) -> Vec3 {
        Vec3::new(self.w, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn wxy(self) -> Vec3 {
        Vec3::new(self.w, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn wxz(self) -> Vec3 {
        Vec3::new(self.w, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn wxw(self) -> Vec3 {
        Vec3::new(self.w, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn wyx(self) -> Vec3 {
        Vec3::new(self.w, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn wyy(self) -> Vec3 {
        Vec3::new(self.w, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn wyz(self) -> Vec3 {
        Vec3::new(self.w, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn wyw(self) -> Vec3 {
        Vec3::new(self.w, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn wzx(self) -> Vec3 {
        Vec3::new(self.w, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn wzy(self) -> Vec3 {
        Vec3::new(self.w, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn wzz(self) -> Vec3 {
        Vec3::new(self.w, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn wzw(self) -> Vec3 {
        Vec3::new(self.w, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn wwx(self) -> Vec3 {
        Vec3::new(self.w, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn wwy(self) -> Vec3 {
        Vec3::new(self.w, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn wwz(self) -> Vec3 {
        Vec3::new(self.w, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn www(self) -> Vec3 {
        Vec3::new(self.w, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn xxxx(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [0, 0, 0, 0]))
    }

    #[inline]
    #[must_use]
    fn xxxy(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [0, 0, 0, 1]))
    }

    #[inline]
    #[must_use]
    fn xxxz(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [0, 0, 0, 2]))
    }

    #[inline]
    #[must_use]
    fn xxxw(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [0, 0, 0, 3]))
    }

    #[inline]
    #[must_use]
    fn xxyx(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [0, 0, 1, 0]))
    }

    #[inline]
    #[must_use]
    fn xxyy(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [0, 0, 1, 1]))
    }

    #[inline]
    #[must_use]
    fn xxyz(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [0, 0, 1, 2]))
    }

    #[inline]
    #[must_use]
    fn xxyw(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [0, 0, 1, 3]))
    }

    #[inline]
    #[must_use]
    fn xxzx(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [0, 0, 2, 0]))
    }

    #[inline]
    #[must_use]
    fn xxzy(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [0, 0, 2, 1]))
    }

    #[inline]
    #[must_use]
    fn xxzz(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [0, 0, 2, 2]))
    }

    #[inline]
    #[must_use]
    fn xxzw(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [0, 0, 2, 3]))
    }

    #[inline]
    #[must_use]
    fn xxwx(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [0, 0, 3, 0]))
    }

    #[inline]
    #[must_use]
    fn xxwy(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [0, 0, 3, 1]))
    }

    #[inline]
    #[must_use]
    fn xxwz(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [0, 0, 3, 2]))
    }

    #[inline]
    #[must_use]
    fn xxww(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [0, 0, 3, 3]))
    }

    #[inline]
    #[must_use]
    fn xyxx(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [0, 1, 0, 0]))
    }

    #[inline]
    #[must_use]
    fn xyxy(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [0, 1, 0, 1]))
    }

    #[inline]
    #[must_use]
    fn xyxz(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [0, 1, 0, 2]))
    }

    #[inline]
    #[must_use]
    fn xyxw(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [0, 1, 0, 3]))
    }

    #[inline]
    #[must_use]
    fn xyyx(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [0, 1, 1, 0]))
    }

    #[inline]
    #[must_use]
    fn xyyy(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [0, 1, 1, 1]))
    }

    #[inline]
    #[must_use]
    fn xyyz(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [0, 1, 1, 2]))
    }

    #[inline]
    #[must_use]
    fn xyyw(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [0, 1, 1, 3]))
    }

    #[inline]
    #[must_use]
    fn xyzx(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [0, 1, 2, 0]))
    }

    #[inline]
    #[must_use]
    fn xyzy(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [0, 1, 2, 1]))
    }

    #[inline]
    #[must_use]
    fn xyzz(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [0, 1, 2, 2]))
    }

    #[inline]
    #[must_use]
    fn xyzw(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [0, 1, 2, 3]))
    }

    #[inline]
    #[must_use]
    fn xywx(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [0, 1, 3, 0]))
    }

    #[inline]
    #[must_use]
    fn xywy(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [0, 1, 3, 1]))
    }

    #[inline]
    #[must_use]
    fn xywz(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [0, 1, 3, 2]))
    }

    #[inline]
    #[must_use]
    fn xyww(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [0, 1, 3, 3]))
    }

    #[inline]
    #[must_use]
    fn xzxx(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [0, 2, 0, 0]))
    }

    #[inline]
    #[must_use]
    fn xzxy(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [0, 2, 0, 1]))
    }

    #[inline]
    #[must_use]
    fn xzxz(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [0, 2, 0, 2]))
    }

    #[inline]
    #[must_use]
    fn xzxw(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [0, 2, 0, 3]))
    }

    #[inline]
    #[must_use]
    fn xzyx(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [0, 2, 1, 0]))
    }

    #[inline]
    #[must_use]
    fn xzyy(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [0, 2, 1, 1]))
    }

    #[inline]
    #[must_use]
    fn xzyz(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [0, 2, 1, 2]))
    }

    #[inline]
    #[must_use]
    fn xzyw(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [0, 2, 1, 3]))
    }

    #[inline]
    #[must_use]
    fn xzzx(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [0, 2, 2, 0]))
    }

    #[inline]
    #[must_use]
    fn xzzy(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [0, 2, 2, 1]))
    }

    #[inline]
    #[must_use]
    fn xzzz(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [0, 2, 2, 2]))
    }

    #[inline]
    #[must_use]
    fn xzzw(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [0, 2, 2, 3]))
    }

    #[inline]
    #[must_use]
    fn xzwx(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [0, 2, 3, 0]))
    }

    #[inline]
    #[must_use]
    fn xzwy(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [0, 2, 3, 1]))
    }

    #[inline]
    #[must_use]
    fn xzwz(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [0, 2, 3, 2]))
    }

    #[inline]
    #[must_use]
    fn xzww(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [0, 2, 3, 3]))
    }

    #[inline]
    #[must_use]
    fn xwxx(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [0, 3, 0, 0]))
    }

    #[inline]
    #[must_use]
    fn xwxy(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [0, 3, 0, 1]))
    }

    #[inline]
    #[must_use]
    fn xwxz(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [0, 3, 0, 2]))
    }

    #[inline]
    #[must_use]
    fn xwxw(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [0, 3, 0, 3]))
    }

    #[inline]
    #[must_use]
    fn xwyx(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [0, 3, 1, 0]))
    }

    #[inline]
    #[must_use]
    fn xwyy(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [0, 3, 1, 1]))
    }

    #[inline]
    #[must_use]
    fn xwyz(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [0, 3, 1, 2]))
    }

    #[inline]
    #[must_use]
    fn xwyw(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [0, 3, 1, 3]))
    }

    #[inline]
    #[must_use]
    fn xwzx(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [0, 3, 2, 0]))
    }

    #[inline]
    #[must_use]
    fn xwzy(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [0, 3, 2, 1]))
    }

    #[inline]
    #[must_use]
    fn xwzz(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [0, 3, 2, 2]))
    }

    #[inline]
    #[must_use]
    fn xwzw(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [0, 3, 2, 3]))
    }

    #[inline]
    #[must_use]
    fn xwwx(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [0, 3, 3, 0]))
    }

    #[inline]
    #[must_use]
    fn xwwy(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [0, 3, 3, 1]))
    }

    #[inline]
    #[must_use]
    fn xwwz(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [0, 3, 3, 2]))
    }

    #[inline]
    #[must_use]
    fn xwww(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [0, 3, 3, 3]))
    }

    #[inline]
    #[must_use]
    fn yxxx(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [1, 0, 0, 0]))
    }

    #[inline]
    #[must_use]
    fn yxxy(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [1, 0, 0, 1]))
    }

    #[inline]
    #[must_use]
    fn yxxz(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [1, 0, 0, 2]))
    }

    #[inline]
    #[must_use]
    fn yxxw(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [1, 0, 0, 3]))
    }

    #[inline]
    #[must_use]
    fn yxyx(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [1, 0, 1, 0]))
    }

    #[inline]
    #[must_use]
    fn yxyy(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [1, 0, 1, 1]))
    }

    #[inline]
    #[must_use]
    fn yxyz(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [1, 0, 1, 2]))
    }

    #[inline]
    #[must_use]
    fn yxyw(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [1, 0, 1, 3]))
    }

    #[inline]
    #[must_use]
    fn yxzx(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [1, 0, 2, 0]))
    }

    #[inline]
    #[must_use]
    fn yxzy(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [1, 0, 2, 1]))
    }

    #[inline]
    #[must_use]
    fn yxzz(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [1, 0, 2, 2]))
    }

    #[inline]
    #[must_use]
    fn yxzw(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [1, 0, 2, 3]))
    }

    #[inline]
    #[must_use]
    fn yxwx(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [1, 0, 3, 0]))
    }

    #[inline]
    #[must_use]
    fn yxwy(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [1, 0, 3, 1]))
    }

    #[inline]
    #[must_use]
    fn yxwz(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [1, 0, 3, 2]))
    }

    #[inline]
    #[must_use]
    fn yxww(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [1, 0, 3, 3]))
    }

    #[inline]
    #[must_use]
    fn yyxx(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [1, 1, 0, 0]))
    }

    #[inline]
    #[must_use]
    fn yyxy(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [1, 1, 0, 1]))
    }

    #[inline]
    #[must_use]
    fn yyxz(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [1, 1, 0, 2]))
    }

    #[inline]
    #[must_use]
    fn yyxw(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [1, 1, 0, 3]))
    }

    #[inline]
    #[must_use]
    fn yyyx(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [1, 1, 1, 0]))
    }

    #[inline]
    #[must_use]
    fn yyyy(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [1, 1, 1, 1]))
    }

    #[inline]
    #[must_use]
    fn yyyz(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [1, 1, 1, 2]))
    }

    #[inline]
    #[must_use]
    fn yyyw(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [1, 1, 1, 3]))
    }

    #[inline]
    #[must_use]
    fn yyzx(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [1, 1, 2, 0]))
    }

    #[inline]
    #[must_use]
    fn yyzy(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [1, 1, 2, 1]))
    }

    #[inline]
    #[must_use]
    fn yyzz(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [1, 1, 2, 2]))
    }

    #[inline]
    #[must_use]
    fn yyzw(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [1, 1, 2, 3]))
    }

    #[inline]
    #[must_use]
    fn yywx(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [1, 1, 3, 0]))
    }

    #[inline]
    #[must_use]
    fn yywy(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [1, 1, 3, 1]))
    }

    #[inline]
    #[must_use]
    fn yywz(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [1, 1, 3, 2]))
    }

    #[inline]
    #[must_use]
    fn yyww(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [1, 1, 3, 3]))
    }

    #[inline]
    #[must_use]
    fn yzxx(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [1, 2, 0, 0]))
    }

    #[inline]
    #[must_use]
    fn yzxy(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [1, 2, 0, 1]))
    }

    #[inline]
    #[must_use]
    fn yzxz(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [1, 2, 0, 2]))
    }

    #[inline]
    #[must_use]
    fn yzxw(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [1, 2, 0, 3]))
    }

    #[inline]
    #[must_use]
    fn yzyx(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [1, 2, 1, 0]))
    }

    #[inline]
    #[must_use]
    fn yzyy(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [1, 2, 1, 1]))
    }

    #[inline]
    #[must_use]
    fn yzyz(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [1, 2, 1, 2]))
    }

    #[inline]
    #[must_use]
    fn yzyw(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [1, 2, 1, 3]))
    }

    #[inline]
    #[must_use]
    fn yzzx(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [1, 2, 2, 0]))
    }

    #[inline]
    #[must_use]
    fn yzzy(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [1, 2, 2, 1]))
    }

    #[inline]
    #[must_use]
    fn yzzz(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [1, 2, 2, 2]))
    }

    #[inline]
    #[must_use]
    fn yzzw(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [1, 2, 2, 3]))
    }

    #[inline]
    #[must_use]
    fn yzwx(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [1, 2, 3, 0]))
    }

    #[inline]
    #[must_use]
    fn yzwy(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [1, 2, 3, 1]))
    }

    #[inline]
    #[must_use]
    fn yzwz(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [1, 2, 3, 2]))
    }

    #[inline]
    #[must_use]
    fn yzww(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [1, 2, 3, 3]))
    }

    #[inline]
    #[must_use]
    fn ywxx(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [1, 3, 0, 0]))
    }

    #[inline]
    #[must_use]
    fn ywxy(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [1, 3, 0, 1]))
    }

    #[inline]
    #[must_use]
    fn ywxz(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [1, 3, 0, 2]))
    }

    #[inline]
    #[must_use]
    fn ywxw(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [1, 3, 0, 3]))
    }

    #[inline]
    #[must_use]
    fn ywyx(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [1, 3, 1, 0]))
    }

    #[inline]
    #[must_use]
    fn ywyy(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [1, 3, 1, 1]))
    }

    #[inline]
    #[must_use]
    fn ywyz(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [1, 3, 1, 2]))
    }

    #[inline]
    #[must_use]
    fn ywyw(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [1, 3, 1, 3]))
    }

    #[inline]
    #[must_use]
    fn ywzx(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [1, 3, 2, 0]))
    }

    #[inline]
    #[must_use]
    fn ywzy(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [1, 3, 2, 1]))
    }

    #[inline]
    #[must_use]
    fn ywzz(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [1, 3, 2, 2]))
    }

    #[inline]
    #[must_use]
    fn ywzw(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [1, 3, 2, 3]))
    }

    #[inline]
    #[must_use]
    fn ywwx(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [1, 3, 3, 0]))
    }

    #[inline]
    #[must_use]
    fn ywwy(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [1, 3, 3, 1]))
    }

    #[inline]
    #[must_use]
    fn ywwz(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [1, 3, 3, 2]))
    }

    #[inline]
    #[must_use]
    fn ywww(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [1, 3, 3, 3]))
    }

    #[inline]
    #[must_use]
    fn zxxx(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [2, 0, 0, 0]))
    }

    #[inline]
    #[must_use]
    fn zxxy(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [2, 0, 0, 1]))
    }

    #[inline]
    #[must_use]
    fn zxxz(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [2, 0, 0, 2]))
    }

    #[inline]
    #[must_use]
    fn zxxw(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [2, 0, 0, 3]))
    }

    #[inline]
    #[must_use]
    fn zxyx(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [2, 0, 1, 0]))
    }

    #[inline]
    #[must_use]
    fn zxyy(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [2, 0, 1, 1]))
    }

    #[inline]
    #[must_use]
    fn zxyz(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [2, 0, 1, 2]))
    }

    #[inline]
    #[must_use]
    fn zxyw(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [2, 0, 1, 3]))
    }

    #[inline]
    #[must_use]
    fn zxzx(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [2, 0, 2, 0]))
    }

    #[inline]
    #[must_use]
    fn zxzy(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [2, 0, 2, 1]))
    }

    #[inline]
    #[must_use]
    fn zxzz(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [2, 0, 2, 2]))
    }

    #[inline]
    #[must_use]
    fn zxzw(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [2, 0, 2, 3]))
    }

    #[inline]
    #[must_use]
    fn zxwx(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [2, 0, 3, 0]))
    }

    #[inline]
    #[must_use]
    fn zxwy(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [2, 0, 3, 1]))
    }

    #[inline]
    #[must_use]
    fn zxwz(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [2, 0, 3, 2]))
    }

    #[inline]
    #[must_use]
    fn zxww(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [2, 0, 3, 3]))
    }

    #[inline]
    #[must_use]
    fn zyxx(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [2, 1, 0, 0]))
    }

    #[inline]
    #[must_use]
    fn zyxy(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [2, 1, 0, 1]))
    }

    #[inline]
    #[must_use]
    fn zyxz(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [2, 1, 0, 2]))
    }

    #[inline]
    #[must_use]
    fn zyxw(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [2, 1, 0, 3]))
    }

    #[inline]
    #[must_use]
    fn zyyx(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [2, 1, 1, 0]))
    }

    #[inline]
    #[must_use]
    fn zyyy(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [2, 1, 1, 1]))
    }

    #[inline]
    #[must_use]
    fn zyyz(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [2, 1, 1, 2]))
    }

    #[inline]
    #[must_use]
    fn zyyw(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [2, 1, 1, 3]))
    }

    #[inline]
    #[must_use]
    fn zyzx(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [2, 1, 2, 0]))
    }

    #[inline]
    #[must_use]
    fn zyzy(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [2, 1, 2, 1]))
    }

    #[inline]
    #[must_use]
    fn zyzz(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [2, 1, 2, 2]))
    }

    #[inline]
    #[must_use]
    fn zyzw(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [2, 1, 2, 3]))
    }

    #[inline]
    #[must_use]
    fn zywx(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [2, 1, 3, 0]))
    }

    #[inline]
    #[must_use]
    fn zywy(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [2, 1, 3, 1]))
    }

    #[inline]
    #[must_use]
    fn zywz(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [2, 1, 3, 2]))
    }

    #[inline]
    #[must_use]
    fn zyww(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [2, 1, 3, 3]))
    }

    #[inline]
    #[must_use]
    fn zzxx(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [2, 2, 0, 0]))
    }

    #[inline]
    #[must_use]
    fn zzxy(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [2, 2, 0, 1]))
    }

    #[inline]
    #[must_use]
    fn zzxz(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [2, 2, 0, 2]))
    }

    #[inline]
    #[must_use]
    fn zzxw(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [2, 2, 0, 3]))
    }

    #[inline]
    #[must_use]
    fn zzyx(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [2, 2, 1, 0]))
    }

    #[inline]
    #[must_use]
    fn zzyy(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [2, 2, 1, 1]))
    }

    #[inline]
    #[must_use]
    fn zzyz(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [2, 2, 1, 2]))
    }

    #[inline]
    #[must_use]
    fn zzyw(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [2, 2, 1, 3]))
    }

    #[inline]
    #[must_use]
    fn zzzx(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [2, 2, 2, 0]))
    }

    #[inline]
    #[must_use]
    fn zzzy(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [2, 2, 2, 1]))
    }

    #[inline]
    #[must_use]
    fn zzzz(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [2, 2, 2, 2]))
    }

    #[inline]
    #[must_use]
    fn zzzw(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [2, 2, 2, 3]))
    }

    #[inline]
    #[must_use]
    fn zzwx(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [2, 2, 3, 0]))
    }

    #[inline]
    #[must_use]
    fn zzwy(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [2, 2, 3, 1]))
    }

    #[inline]
    #[must_use]
    fn zzwz(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [2, 2, 3, 2]))
    }

    #[inline]
    #[must_use]
    fn zzww(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [2, 2, 3, 3]))
    }

    #[inline]
    #[must_use]
    fn zwxx(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [2, 3, 0, 0]))
    }

    #[inline]
    #[must_use]
    fn zwxy(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [2, 3, 0, 1]))
    }

    #[inline]
    #[must_use]
    fn zwxz(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [2, 3, 0, 2]))
    }

    #[inline]
    #[must_use]
    fn zwxw(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [2, 3, 0, 3]))
    }

    #[inline]
    #[must_use]
    fn zwyx(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [2, 3, 1, 0]))
    }

    #[inline]
    #[must_use]
    fn zwyy(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [2, 3, 1, 1]))
    }

    #[inline]
    #[must_use]
    fn zwyz(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [2, 3, 1, 2]))
    }

    #[inline]
    #[must_use]
    fn zwyw(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [2, 3, 1, 3]))
    }

    #[inline]
    #[must_use]
    fn zwzx(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [2, 3, 2, 0]))
    }

    #[inline]
    #[must_use]
    fn zwzy(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [2, 3, 2, 1]))
    }

    #[inline]
    #[must_use]
    fn zwzz(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [2, 3, 2, 2]))
    }

    #[inline]
    #[must_use]
    fn zwzw(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [2, 3, 2, 3]))
    }

    #[inline]
    #[must_use]
    fn zwwx(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [2, 3, 3, 0]))
    }

    #[inline]
    #[must_use]
    fn zwwy(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [2, 3, 3, 1]))
    }

    #[inline]
    #[must_use]
    fn zwwz(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [2, 3, 3, 2]))
    }

    #[inline]
    #[must_use]
    fn zwww(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [2, 3, 3, 3]))
    }

    #[inline]
    #[must_use]
    fn wxxx(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [3, 0, 0, 0]))
    }

    #[inline]
    #[must_use]
    fn wxxy(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [3, 0, 0, 1]))
    }

    #[inline]
    #[must_use]
    fn wxxz(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [3, 0, 0, 2]))
    }

    #[inline]
    #[must_use]
    fn wxxw(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [3, 0, 0, 3]))
    }

    #[inline]
    #[must_use]
    fn wxyx(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [3, 0, 1, 0]))
    }

    #[inline]
    #[must_use]
    fn wxyy(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [3, 0, 1, 1]))
    }

    #[inline]
    #[must_use]
    fn wxyz(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [3, 0, 1, 2]))
    }

    #[inline]
    #[must_use]
    fn wxyw(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [3, 0, 1, 3]))
    }

    #[inline]
    #[must_use]
    fn wxzx(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [3, 0, 2, 0]))
    }

    #[inline]
    #[must_use]
    fn wxzy(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [3, 0, 2, 1]))
    }

    #[inline]
    #[must_use]
    fn wxzz(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [3, 0, 2, 2]))
    }

    #[inline]
    #[must_use]
    fn wxzw(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [3, 0, 2, 3]))
    }

    #[inline]
    #[must_use]
    fn wxwx(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [3, 0, 3, 0]))
    }

    #[inline]
    #[must_use]
    fn wxwy(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [3, 0, 3, 1]))
    }

    #[inline]
    #[must_use]
    fn wxwz(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [3, 0, 3, 2]))
    }

    #[inline]
    #[must_use]
    fn wxww(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [3, 0, 3, 3]))
    }

    #[inline]
    #[must_use]
    fn wyxx(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [3, 1, 0, 0]))
    }

    #[inline]
    #[must_use]
    fn wyxy(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [3, 1, 0, 1]))
    }

    #[inline]
    #[must_use]
    fn wyxz(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [3, 1, 0, 2]))
    }

    #[inline]
    #[must_use]
    fn wyxw(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [3, 1, 0, 3]))
    }

    #[inline]
    #[must_use]
    fn wyyx(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [3, 1, 1, 0]))
    }

    #[inline]
    #[must_use]
    fn wyyy(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [3, 1, 1, 1]))
    }

    #[inline]
    #[must_use]
    fn wyyz(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [3, 1, 1, 2]))
    }

    #[inline]
    #[must_use]
    fn wyyw(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [3, 1, 1, 3]))
    }

    #[inline]
    #[must_use]
    fn wyzx(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [3, 1, 2, 0]))
    }

    #[inline]
    #[must_use]
    fn wyzy(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [3, 1, 2, 1]))
    }

    #[inline]
    #[must_use]
    fn wyzz(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [3, 1, 2, 2]))
    }

    #[inline]
    #[must_use]
    fn wyzw(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [3, 1, 2, 3]))
    }

    #[inline]
    #[must_use]
    fn wywx(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [3, 1, 3, 0]))
    }

    #[inline]
    #[must_use]
    fn wywy(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [3, 1, 3, 1]))
    }

    #[inline]
    #[must_use]
    fn wywz(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [3, 1, 3, 2]))
    }

    #[inline]
    #[must_use]
    fn wyww(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [3, 1, 3, 3]))
    }

    #[inline]
    #[must_use]
    fn wzxx(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [3, 2, 0, 0]))
    }

    #[inline]
    #[must_use]
    fn wzxy(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [3, 2, 0, 1]))
    }

    #[inline]
    #[must_use]
    fn wzxz(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [3, 2, 0, 2]))
    }

    #[inline]
    #[must_use]
    fn wzxw(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [3, 2, 0, 3]))
    }

    #[inline]
    #[must_use]
    fn wzyx(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [3, 2, 1, 0]))
    }

    #[inline]
    #[must_use]
    fn wzyy(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [3, 2, 1, 1]))
    }

    #[inline]
    #[must_use]
    fn wzyz(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [3, 2, 1, 2]))
    }

    #[inline]
    #[must_use]
    fn wzyw(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [3, 2, 1, 3]))
    }

    #[inline]
    #[must_use]
    fn wzzx(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [3, 2, 2, 0]))
    }

    #[inline]
    #[must_use]
    fn wzzy(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [3, 2, 2, 1]))
    }

    #[inline]
    #[must_use]
    fn wzzz(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [3, 2, 2, 2]))
    }

    #[inline]
    #[must_use]
    fn wzzw(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [3, 2, 2, 3]))
    }

    #[inline]
    #[must_use]
    fn wzwx(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [3, 2, 3, 0]))
    }

    #[inline]
    #[must_use]
    fn wzwy(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [3, 2, 3, 1]))
    }

    #[inline]
    #[must_use]
    fn wzwz(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [3, 2, 3, 2]))
    }

    #[inline]
    #[must_use]
    fn wzww(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [3, 2, 3, 3]))
    }

    #[inline]
    #[must_use]
    fn wwxx(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [3, 3, 0, 0]))
    }

    #[inline]
    #[must_use]
    fn wwxy(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [3, 3, 0, 1]))
    }

    #[inline]
    #[must_use]
    fn wwxz(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [3, 3, 0, 2]))
    }

    #[inline]
    #[must_use]
    fn wwxw(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [3, 3, 0, 3]))
    }

    #[inline]
    #[must_use]
    fn wwyx(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [3, 3, 1, 0]))
    }

    #[inline]
    #[must_use]
    fn wwyy(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [3, 3, 1, 1]))
    }

    #[inline]
    #[must_use]
    fn wwyz(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [3, 3, 1, 2]))
    }

    #[inline]
    #[must_use]
    fn wwyw(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [3, 3, 1, 3]))
    }

    #[inline]
    #[must_use]
    fn wwzx(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [3, 3, 2, 0]))
    }

    #[inline]
    #[must_use]
    fn wwzy(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [3, 3, 2, 1]))
    }

    #[inline]
    #[must_use]
    fn wwzz(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [3, 3, 2, 2]))
    }

    #[inline]
    #[must_use]
    fn wwzw(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [3, 3, 2, 3]))
    }

    #[inline]
    #[must_use]
    fn wwwx(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [3, 3, 3, 0]))
    }

    #[inline]
    #[must_use]
    fn wwwy(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [3, 3, 3, 1]))
    }

    #[inline]
    #[must_use]
    fn wwwz(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [3, 3, 3, 2]))
    }

    #[inline]
    #[must_use]
    fn wwww(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [3, 3, 3, 3]))
    }
}
