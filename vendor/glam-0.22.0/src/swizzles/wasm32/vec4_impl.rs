// Generated from swizzle_impl.rs.tera template. Edit the template, not the generated file.

#![allow(clippy::useless_conversion)]

use crate::{Vec2, Vec3, Vec4, Vec4Swizzles};

use core::arch::wasm32::*;

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
    fn xz(self) -> Vec2 {
        Vec2 {
            x: self.x,
            y: self.z,
        }
    }

    #[inline]
    fn xw(self) -> Vec2 {
        Vec2 {
            x: self.x,
            y: self.w,
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
    fn yw(self) -> Vec2 {
        Vec2 {
            x: self.y,
            y: self.w,
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
    fn zw(self) -> Vec2 {
        Vec2 {
            x: self.z,
            y: self.w,
        }
    }

    #[inline]
    fn wx(self) -> Vec2 {
        Vec2 {
            x: self.w,
            y: self.x,
        }
    }

    #[inline]
    fn wy(self) -> Vec2 {
        Vec2 {
            x: self.w,
            y: self.y,
        }
    }

    #[inline]
    fn wz(self) -> Vec2 {
        Vec2 {
            x: self.w,
            y: self.z,
        }
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
        Vec3 {
            x: self.x,
            y: self.x,
            z: self.x,
        }
    }

    #[inline]
    fn xxy(self) -> Vec3 {
        Vec3 {
            x: self.x,
            y: self.x,
            z: self.y,
        }
    }

    #[inline]
    fn xxz(self) -> Vec3 {
        Vec3 {
            x: self.x,
            y: self.x,
            z: self.z,
        }
    }

    #[inline]
    fn xxw(self) -> Vec3 {
        Vec3 {
            x: self.x,
            y: self.x,
            z: self.w,
        }
    }

    #[inline]
    fn xyx(self) -> Vec3 {
        Vec3 {
            x: self.x,
            y: self.y,
            z: self.x,
        }
    }

    #[inline]
    fn xyy(self) -> Vec3 {
        Vec3 {
            x: self.x,
            y: self.y,
            z: self.y,
        }
    }

    #[inline]
    fn xyz(self) -> Vec3 {
        Vec3 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }

    #[inline]
    fn xyw(self) -> Vec3 {
        Vec3 {
            x: self.x,
            y: self.y,
            z: self.w,
        }
    }

    #[inline]
    fn xzx(self) -> Vec3 {
        Vec3 {
            x: self.x,
            y: self.z,
            z: self.x,
        }
    }

    #[inline]
    fn xzy(self) -> Vec3 {
        Vec3 {
            x: self.x,
            y: self.z,
            z: self.y,
        }
    }

    #[inline]
    fn xzz(self) -> Vec3 {
        Vec3 {
            x: self.x,
            y: self.z,
            z: self.z,
        }
    }

    #[inline]
    fn xzw(self) -> Vec3 {
        Vec3 {
            x: self.x,
            y: self.z,
            z: self.w,
        }
    }

    #[inline]
    fn xwx(self) -> Vec3 {
        Vec3 {
            x: self.x,
            y: self.w,
            z: self.x,
        }
    }

    #[inline]
    fn xwy(self) -> Vec3 {
        Vec3 {
            x: self.x,
            y: self.w,
            z: self.y,
        }
    }

    #[inline]
    fn xwz(self) -> Vec3 {
        Vec3 {
            x: self.x,
            y: self.w,
            z: self.z,
        }
    }

    #[inline]
    fn xww(self) -> Vec3 {
        Vec3 {
            x: self.x,
            y: self.w,
            z: self.w,
        }
    }

    #[inline]
    fn yxx(self) -> Vec3 {
        Vec3 {
            x: self.y,
            y: self.x,
            z: self.x,
        }
    }

    #[inline]
    fn yxy(self) -> Vec3 {
        Vec3 {
            x: self.y,
            y: self.x,
            z: self.y,
        }
    }

    #[inline]
    fn yxz(self) -> Vec3 {
        Vec3 {
            x: self.y,
            y: self.x,
            z: self.z,
        }
    }

    #[inline]
    fn yxw(self) -> Vec3 {
        Vec3 {
            x: self.y,
            y: self.x,
            z: self.w,
        }
    }

    #[inline]
    fn yyx(self) -> Vec3 {
        Vec3 {
            x: self.y,
            y: self.y,
            z: self.x,
        }
    }

    #[inline]
    fn yyy(self) -> Vec3 {
        Vec3 {
            x: self.y,
            y: self.y,
            z: self.y,
        }
    }

    #[inline]
    fn yyz(self) -> Vec3 {
        Vec3 {
            x: self.y,
            y: self.y,
            z: self.z,
        }
    }

    #[inline]
    fn yyw(self) -> Vec3 {
        Vec3 {
            x: self.y,
            y: self.y,
            z: self.w,
        }
    }

    #[inline]
    fn yzx(self) -> Vec3 {
        Vec3 {
            x: self.y,
            y: self.z,
            z: self.x,
        }
    }

    #[inline]
    fn yzy(self) -> Vec3 {
        Vec3 {
            x: self.y,
            y: self.z,
            z: self.y,
        }
    }

    #[inline]
    fn yzz(self) -> Vec3 {
        Vec3 {
            x: self.y,
            y: self.z,
            z: self.z,
        }
    }

    #[inline]
    fn yzw(self) -> Vec3 {
        Vec3 {
            x: self.y,
            y: self.z,
            z: self.w,
        }
    }

    #[inline]
    fn ywx(self) -> Vec3 {
        Vec3 {
            x: self.y,
            y: self.w,
            z: self.x,
        }
    }

    #[inline]
    fn ywy(self) -> Vec3 {
        Vec3 {
            x: self.y,
            y: self.w,
            z: self.y,
        }
    }

    #[inline]
    fn ywz(self) -> Vec3 {
        Vec3 {
            x: self.y,
            y: self.w,
            z: self.z,
        }
    }

    #[inline]
    fn yww(self) -> Vec3 {
        Vec3 {
            x: self.y,
            y: self.w,
            z: self.w,
        }
    }

    #[inline]
    fn zxx(self) -> Vec3 {
        Vec3 {
            x: self.z,
            y: self.x,
            z: self.x,
        }
    }

    #[inline]
    fn zxy(self) -> Vec3 {
        Vec3 {
            x: self.z,
            y: self.x,
            z: self.y,
        }
    }

    #[inline]
    fn zxz(self) -> Vec3 {
        Vec3 {
            x: self.z,
            y: self.x,
            z: self.z,
        }
    }

    #[inline]
    fn zxw(self) -> Vec3 {
        Vec3 {
            x: self.z,
            y: self.x,
            z: self.w,
        }
    }

    #[inline]
    fn zyx(self) -> Vec3 {
        Vec3 {
            x: self.z,
            y: self.y,
            z: self.x,
        }
    }

    #[inline]
    fn zyy(self) -> Vec3 {
        Vec3 {
            x: self.z,
            y: self.y,
            z: self.y,
        }
    }

    #[inline]
    fn zyz(self) -> Vec3 {
        Vec3 {
            x: self.z,
            y: self.y,
            z: self.z,
        }
    }

    #[inline]
    fn zyw(self) -> Vec3 {
        Vec3 {
            x: self.z,
            y: self.y,
            z: self.w,
        }
    }

    #[inline]
    fn zzx(self) -> Vec3 {
        Vec3 {
            x: self.z,
            y: self.z,
            z: self.x,
        }
    }

    #[inline]
    fn zzy(self) -> Vec3 {
        Vec3 {
            x: self.z,
            y: self.z,
            z: self.y,
        }
    }

    #[inline]
    fn zzz(self) -> Vec3 {
        Vec3 {
            x: self.z,
            y: self.z,
            z: self.z,
        }
    }

    #[inline]
    fn zzw(self) -> Vec3 {
        Vec3 {
            x: self.z,
            y: self.z,
            z: self.w,
        }
    }

    #[inline]
    fn zwx(self) -> Vec3 {
        Vec3 {
            x: self.z,
            y: self.w,
            z: self.x,
        }
    }

    #[inline]
    fn zwy(self) -> Vec3 {
        Vec3 {
            x: self.z,
            y: self.w,
            z: self.y,
        }
    }

    #[inline]
    fn zwz(self) -> Vec3 {
        Vec3 {
            x: self.z,
            y: self.w,
            z: self.z,
        }
    }

    #[inline]
    fn zww(self) -> Vec3 {
        Vec3 {
            x: self.z,
            y: self.w,
            z: self.w,
        }
    }

    #[inline]
    fn wxx(self) -> Vec3 {
        Vec3 {
            x: self.w,
            y: self.x,
            z: self.x,
        }
    }

    #[inline]
    fn wxy(self) -> Vec3 {
        Vec3 {
            x: self.w,
            y: self.x,
            z: self.y,
        }
    }

    #[inline]
    fn wxz(self) -> Vec3 {
        Vec3 {
            x: self.w,
            y: self.x,
            z: self.z,
        }
    }

    #[inline]
    fn wxw(self) -> Vec3 {
        Vec3 {
            x: self.w,
            y: self.x,
            z: self.w,
        }
    }

    #[inline]
    fn wyx(self) -> Vec3 {
        Vec3 {
            x: self.w,
            y: self.y,
            z: self.x,
        }
    }

    #[inline]
    fn wyy(self) -> Vec3 {
        Vec3 {
            x: self.w,
            y: self.y,
            z: self.y,
        }
    }

    #[inline]
    fn wyz(self) -> Vec3 {
        Vec3 {
            x: self.w,
            y: self.y,
            z: self.z,
        }
    }

    #[inline]
    fn wyw(self) -> Vec3 {
        Vec3 {
            x: self.w,
            y: self.y,
            z: self.w,
        }
    }

    #[inline]
    fn wzx(self) -> Vec3 {
        Vec3 {
            x: self.w,
            y: self.z,
            z: self.x,
        }
    }

    #[inline]
    fn wzy(self) -> Vec3 {
        Vec3 {
            x: self.w,
            y: self.z,
            z: self.y,
        }
    }

    #[inline]
    fn wzz(self) -> Vec3 {
        Vec3 {
            x: self.w,
            y: self.z,
            z: self.z,
        }
    }

    #[inline]
    fn wzw(self) -> Vec3 {
        Vec3 {
            x: self.w,
            y: self.z,
            z: self.w,
        }
    }

    #[inline]
    fn wwx(self) -> Vec3 {
        Vec3 {
            x: self.w,
            y: self.w,
            z: self.x,
        }
    }

    #[inline]
    fn wwy(self) -> Vec3 {
        Vec3 {
            x: self.w,
            y: self.w,
            z: self.y,
        }
    }

    #[inline]
    fn wwz(self) -> Vec3 {
        Vec3 {
            x: self.w,
            y: self.w,
            z: self.z,
        }
    }

    #[inline]
    fn www(self) -> Vec3 {
        Vec3 {
            x: self.w,
            y: self.w,
            z: self.w,
        }
    }

    #[inline]
    fn xxxx(self) -> Vec4 {
        Vec4(i32x4_shuffle::<0, 0, 4, 4>(self.0, self.0))
    }

    #[inline]
    fn xxxy(self) -> Vec4 {
        Vec4(i32x4_shuffle::<0, 0, 4, 5>(self.0, self.0))
    }

    #[inline]
    fn xxxz(self) -> Vec4 {
        Vec4(i32x4_shuffle::<0, 0, 4, 6>(self.0, self.0))
    }

    #[inline]
    fn xxxw(self) -> Vec4 {
        Vec4(i32x4_shuffle::<0, 0, 4, 7>(self.0, self.0))
    }

    #[inline]
    fn xxyx(self) -> Vec4 {
        Vec4(i32x4_shuffle::<0, 0, 5, 4>(self.0, self.0))
    }

    #[inline]
    fn xxyy(self) -> Vec4 {
        Vec4(i32x4_shuffle::<0, 0, 5, 5>(self.0, self.0))
    }

    #[inline]
    fn xxyz(self) -> Vec4 {
        Vec4(i32x4_shuffle::<0, 0, 5, 6>(self.0, self.0))
    }

    #[inline]
    fn xxyw(self) -> Vec4 {
        Vec4(i32x4_shuffle::<0, 0, 5, 7>(self.0, self.0))
    }

    #[inline]
    fn xxzx(self) -> Vec4 {
        Vec4(i32x4_shuffle::<0, 0, 6, 4>(self.0, self.0))
    }

    #[inline]
    fn xxzy(self) -> Vec4 {
        Vec4(i32x4_shuffle::<0, 0, 6, 5>(self.0, self.0))
    }

    #[inline]
    fn xxzz(self) -> Vec4 {
        Vec4(i32x4_shuffle::<0, 0, 6, 6>(self.0, self.0))
    }

    #[inline]
    fn xxzw(self) -> Vec4 {
        Vec4(i32x4_shuffle::<0, 0, 6, 7>(self.0, self.0))
    }

    #[inline]
    fn xxwx(self) -> Vec4 {
        Vec4(i32x4_shuffle::<0, 0, 7, 4>(self.0, self.0))
    }

    #[inline]
    fn xxwy(self) -> Vec4 {
        Vec4(i32x4_shuffle::<0, 0, 7, 5>(self.0, self.0))
    }

    #[inline]
    fn xxwz(self) -> Vec4 {
        Vec4(i32x4_shuffle::<0, 0, 7, 6>(self.0, self.0))
    }

    #[inline]
    fn xxww(self) -> Vec4 {
        Vec4(i32x4_shuffle::<0, 0, 7, 7>(self.0, self.0))
    }

    #[inline]
    fn xyxx(self) -> Vec4 {
        Vec4(i32x4_shuffle::<0, 1, 4, 4>(self.0, self.0))
    }

    #[inline]
    fn xyxy(self) -> Vec4 {
        Vec4(i32x4_shuffle::<0, 1, 4, 5>(self.0, self.0))
    }

    #[inline]
    fn xyxz(self) -> Vec4 {
        Vec4(i32x4_shuffle::<0, 1, 4, 6>(self.0, self.0))
    }

    #[inline]
    fn xyxw(self) -> Vec4 {
        Vec4(i32x4_shuffle::<0, 1, 4, 7>(self.0, self.0))
    }

    #[inline]
    fn xyyx(self) -> Vec4 {
        Vec4(i32x4_shuffle::<0, 1, 5, 4>(self.0, self.0))
    }

    #[inline]
    fn xyyy(self) -> Vec4 {
        Vec4(i32x4_shuffle::<0, 1, 5, 5>(self.0, self.0))
    }

    #[inline]
    fn xyyz(self) -> Vec4 {
        Vec4(i32x4_shuffle::<0, 1, 5, 6>(self.0, self.0))
    }

    #[inline]
    fn xyyw(self) -> Vec4 {
        Vec4(i32x4_shuffle::<0, 1, 5, 7>(self.0, self.0))
    }

    #[inline]
    fn xyzx(self) -> Vec4 {
        Vec4(i32x4_shuffle::<0, 1, 6, 4>(self.0, self.0))
    }

    #[inline]
    fn xyzy(self) -> Vec4 {
        Vec4(i32x4_shuffle::<0, 1, 6, 5>(self.0, self.0))
    }

    #[inline]
    fn xyzz(self) -> Vec4 {
        Vec4(i32x4_shuffle::<0, 1, 6, 6>(self.0, self.0))
    }

    #[inline]
    fn xyzw(self) -> Vec4 {
        Vec4(i32x4_shuffle::<0, 1, 6, 7>(self.0, self.0))
    }

    #[inline]
    fn xywx(self) -> Vec4 {
        Vec4(i32x4_shuffle::<0, 1, 7, 4>(self.0, self.0))
    }

    #[inline]
    fn xywy(self) -> Vec4 {
        Vec4(i32x4_shuffle::<0, 1, 7, 5>(self.0, self.0))
    }

    #[inline]
    fn xywz(self) -> Vec4 {
        Vec4(i32x4_shuffle::<0, 1, 7, 6>(self.0, self.0))
    }

    #[inline]
    fn xyww(self) -> Vec4 {
        Vec4(i32x4_shuffle::<0, 1, 7, 7>(self.0, self.0))
    }

    #[inline]
    fn xzxx(self) -> Vec4 {
        Vec4(i32x4_shuffle::<0, 2, 4, 4>(self.0, self.0))
    }

    #[inline]
    fn xzxy(self) -> Vec4 {
        Vec4(i32x4_shuffle::<0, 2, 4, 5>(self.0, self.0))
    }

    #[inline]
    fn xzxz(self) -> Vec4 {
        Vec4(i32x4_shuffle::<0, 2, 4, 6>(self.0, self.0))
    }

    #[inline]
    fn xzxw(self) -> Vec4 {
        Vec4(i32x4_shuffle::<0, 2, 4, 7>(self.0, self.0))
    }

    #[inline]
    fn xzyx(self) -> Vec4 {
        Vec4(i32x4_shuffle::<0, 2, 5, 4>(self.0, self.0))
    }

    #[inline]
    fn xzyy(self) -> Vec4 {
        Vec4(i32x4_shuffle::<0, 2, 5, 5>(self.0, self.0))
    }

    #[inline]
    fn xzyz(self) -> Vec4 {
        Vec4(i32x4_shuffle::<0, 2, 5, 6>(self.0, self.0))
    }

    #[inline]
    fn xzyw(self) -> Vec4 {
        Vec4(i32x4_shuffle::<0, 2, 5, 7>(self.0, self.0))
    }

    #[inline]
    fn xzzx(self) -> Vec4 {
        Vec4(i32x4_shuffle::<0, 2, 6, 4>(self.0, self.0))
    }

    #[inline]
    fn xzzy(self) -> Vec4 {
        Vec4(i32x4_shuffle::<0, 2, 6, 5>(self.0, self.0))
    }

    #[inline]
    fn xzzz(self) -> Vec4 {
        Vec4(i32x4_shuffle::<0, 2, 6, 6>(self.0, self.0))
    }

    #[inline]
    fn xzzw(self) -> Vec4 {
        Vec4(i32x4_shuffle::<0, 2, 6, 7>(self.0, self.0))
    }

    #[inline]
    fn xzwx(self) -> Vec4 {
        Vec4(i32x4_shuffle::<0, 2, 7, 4>(self.0, self.0))
    }

    #[inline]
    fn xzwy(self) -> Vec4 {
        Vec4(i32x4_shuffle::<0, 2, 7, 5>(self.0, self.0))
    }

    #[inline]
    fn xzwz(self) -> Vec4 {
        Vec4(i32x4_shuffle::<0, 2, 7, 6>(self.0, self.0))
    }

    #[inline]
    fn xzww(self) -> Vec4 {
        Vec4(i32x4_shuffle::<0, 2, 7, 7>(self.0, self.0))
    }

    #[inline]
    fn xwxx(self) -> Vec4 {
        Vec4(i32x4_shuffle::<0, 3, 4, 4>(self.0, self.0))
    }

    #[inline]
    fn xwxy(self) -> Vec4 {
        Vec4(i32x4_shuffle::<0, 3, 4, 5>(self.0, self.0))
    }

    #[inline]
    fn xwxz(self) -> Vec4 {
        Vec4(i32x4_shuffle::<0, 3, 4, 6>(self.0, self.0))
    }

    #[inline]
    fn xwxw(self) -> Vec4 {
        Vec4(i32x4_shuffle::<0, 3, 4, 7>(self.0, self.0))
    }

    #[inline]
    fn xwyx(self) -> Vec4 {
        Vec4(i32x4_shuffle::<0, 3, 5, 4>(self.0, self.0))
    }

    #[inline]
    fn xwyy(self) -> Vec4 {
        Vec4(i32x4_shuffle::<0, 3, 5, 5>(self.0, self.0))
    }

    #[inline]
    fn xwyz(self) -> Vec4 {
        Vec4(i32x4_shuffle::<0, 3, 5, 6>(self.0, self.0))
    }

    #[inline]
    fn xwyw(self) -> Vec4 {
        Vec4(i32x4_shuffle::<0, 3, 5, 7>(self.0, self.0))
    }

    #[inline]
    fn xwzx(self) -> Vec4 {
        Vec4(i32x4_shuffle::<0, 3, 6, 4>(self.0, self.0))
    }

    #[inline]
    fn xwzy(self) -> Vec4 {
        Vec4(i32x4_shuffle::<0, 3, 6, 5>(self.0, self.0))
    }

    #[inline]
    fn xwzz(self) -> Vec4 {
        Vec4(i32x4_shuffle::<0, 3, 6, 6>(self.0, self.0))
    }

    #[inline]
    fn xwzw(self) -> Vec4 {
        Vec4(i32x4_shuffle::<0, 3, 6, 7>(self.0, self.0))
    }

    #[inline]
    fn xwwx(self) -> Vec4 {
        Vec4(i32x4_shuffle::<0, 3, 7, 4>(self.0, self.0))
    }

    #[inline]
    fn xwwy(self) -> Vec4 {
        Vec4(i32x4_shuffle::<0, 3, 7, 5>(self.0, self.0))
    }

    #[inline]
    fn xwwz(self) -> Vec4 {
        Vec4(i32x4_shuffle::<0, 3, 7, 6>(self.0, self.0))
    }

    #[inline]
    fn xwww(self) -> Vec4 {
        Vec4(i32x4_shuffle::<0, 3, 7, 7>(self.0, self.0))
    }

    #[inline]
    fn yxxx(self) -> Vec4 {
        Vec4(i32x4_shuffle::<1, 0, 4, 4>(self.0, self.0))
    }

    #[inline]
    fn yxxy(self) -> Vec4 {
        Vec4(i32x4_shuffle::<1, 0, 4, 5>(self.0, self.0))
    }

    #[inline]
    fn yxxz(self) -> Vec4 {
        Vec4(i32x4_shuffle::<1, 0, 4, 6>(self.0, self.0))
    }

    #[inline]
    fn yxxw(self) -> Vec4 {
        Vec4(i32x4_shuffle::<1, 0, 4, 7>(self.0, self.0))
    }

    #[inline]
    fn yxyx(self) -> Vec4 {
        Vec4(i32x4_shuffle::<1, 0, 5, 4>(self.0, self.0))
    }

    #[inline]
    fn yxyy(self) -> Vec4 {
        Vec4(i32x4_shuffle::<1, 0, 5, 5>(self.0, self.0))
    }

    #[inline]
    fn yxyz(self) -> Vec4 {
        Vec4(i32x4_shuffle::<1, 0, 5, 6>(self.0, self.0))
    }

    #[inline]
    fn yxyw(self) -> Vec4 {
        Vec4(i32x4_shuffle::<1, 0, 5, 7>(self.0, self.0))
    }

    #[inline]
    fn yxzx(self) -> Vec4 {
        Vec4(i32x4_shuffle::<1, 0, 6, 4>(self.0, self.0))
    }

    #[inline]
    fn yxzy(self) -> Vec4 {
        Vec4(i32x4_shuffle::<1, 0, 6, 5>(self.0, self.0))
    }

    #[inline]
    fn yxzz(self) -> Vec4 {
        Vec4(i32x4_shuffle::<1, 0, 6, 6>(self.0, self.0))
    }

    #[inline]
    fn yxzw(self) -> Vec4 {
        Vec4(i32x4_shuffle::<1, 0, 6, 7>(self.0, self.0))
    }

    #[inline]
    fn yxwx(self) -> Vec4 {
        Vec4(i32x4_shuffle::<1, 0, 7, 4>(self.0, self.0))
    }

    #[inline]
    fn yxwy(self) -> Vec4 {
        Vec4(i32x4_shuffle::<1, 0, 7, 5>(self.0, self.0))
    }

    #[inline]
    fn yxwz(self) -> Vec4 {
        Vec4(i32x4_shuffle::<1, 0, 7, 6>(self.0, self.0))
    }

    #[inline]
    fn yxww(self) -> Vec4 {
        Vec4(i32x4_shuffle::<1, 0, 7, 7>(self.0, self.0))
    }

    #[inline]
    fn yyxx(self) -> Vec4 {
        Vec4(i32x4_shuffle::<1, 1, 4, 4>(self.0, self.0))
    }

    #[inline]
    fn yyxy(self) -> Vec4 {
        Vec4(i32x4_shuffle::<1, 1, 4, 5>(self.0, self.0))
    }

    #[inline]
    fn yyxz(self) -> Vec4 {
        Vec4(i32x4_shuffle::<1, 1, 4, 6>(self.0, self.0))
    }

    #[inline]
    fn yyxw(self) -> Vec4 {
        Vec4(i32x4_shuffle::<1, 1, 4, 7>(self.0, self.0))
    }

    #[inline]
    fn yyyx(self) -> Vec4 {
        Vec4(i32x4_shuffle::<1, 1, 5, 4>(self.0, self.0))
    }

    #[inline]
    fn yyyy(self) -> Vec4 {
        Vec4(i32x4_shuffle::<1, 1, 5, 5>(self.0, self.0))
    }

    #[inline]
    fn yyyz(self) -> Vec4 {
        Vec4(i32x4_shuffle::<1, 1, 5, 6>(self.0, self.0))
    }

    #[inline]
    fn yyyw(self) -> Vec4 {
        Vec4(i32x4_shuffle::<1, 1, 5, 7>(self.0, self.0))
    }

    #[inline]
    fn yyzx(self) -> Vec4 {
        Vec4(i32x4_shuffle::<1, 1, 6, 4>(self.0, self.0))
    }

    #[inline]
    fn yyzy(self) -> Vec4 {
        Vec4(i32x4_shuffle::<1, 1, 6, 5>(self.0, self.0))
    }

    #[inline]
    fn yyzz(self) -> Vec4 {
        Vec4(i32x4_shuffle::<1, 1, 6, 6>(self.0, self.0))
    }

    #[inline]
    fn yyzw(self) -> Vec4 {
        Vec4(i32x4_shuffle::<1, 1, 6, 7>(self.0, self.0))
    }

    #[inline]
    fn yywx(self) -> Vec4 {
        Vec4(i32x4_shuffle::<1, 1, 7, 4>(self.0, self.0))
    }

    #[inline]
    fn yywy(self) -> Vec4 {
        Vec4(i32x4_shuffle::<1, 1, 7, 5>(self.0, self.0))
    }

    #[inline]
    fn yywz(self) -> Vec4 {
        Vec4(i32x4_shuffle::<1, 1, 7, 6>(self.0, self.0))
    }

    #[inline]
    fn yyww(self) -> Vec4 {
        Vec4(i32x4_shuffle::<1, 1, 7, 7>(self.0, self.0))
    }

    #[inline]
    fn yzxx(self) -> Vec4 {
        Vec4(i32x4_shuffle::<1, 2, 4, 4>(self.0, self.0))
    }

    #[inline]
    fn yzxy(self) -> Vec4 {
        Vec4(i32x4_shuffle::<1, 2, 4, 5>(self.0, self.0))
    }

    #[inline]
    fn yzxz(self) -> Vec4 {
        Vec4(i32x4_shuffle::<1, 2, 4, 6>(self.0, self.0))
    }

    #[inline]
    fn yzxw(self) -> Vec4 {
        Vec4(i32x4_shuffle::<1, 2, 4, 7>(self.0, self.0))
    }

    #[inline]
    fn yzyx(self) -> Vec4 {
        Vec4(i32x4_shuffle::<1, 2, 5, 4>(self.0, self.0))
    }

    #[inline]
    fn yzyy(self) -> Vec4 {
        Vec4(i32x4_shuffle::<1, 2, 5, 5>(self.0, self.0))
    }

    #[inline]
    fn yzyz(self) -> Vec4 {
        Vec4(i32x4_shuffle::<1, 2, 5, 6>(self.0, self.0))
    }

    #[inline]
    fn yzyw(self) -> Vec4 {
        Vec4(i32x4_shuffle::<1, 2, 5, 7>(self.0, self.0))
    }

    #[inline]
    fn yzzx(self) -> Vec4 {
        Vec4(i32x4_shuffle::<1, 2, 6, 4>(self.0, self.0))
    }

    #[inline]
    fn yzzy(self) -> Vec4 {
        Vec4(i32x4_shuffle::<1, 2, 6, 5>(self.0, self.0))
    }

    #[inline]
    fn yzzz(self) -> Vec4 {
        Vec4(i32x4_shuffle::<1, 2, 6, 6>(self.0, self.0))
    }

    #[inline]
    fn yzzw(self) -> Vec4 {
        Vec4(i32x4_shuffle::<1, 2, 6, 7>(self.0, self.0))
    }

    #[inline]
    fn yzwx(self) -> Vec4 {
        Vec4(i32x4_shuffle::<1, 2, 7, 4>(self.0, self.0))
    }

    #[inline]
    fn yzwy(self) -> Vec4 {
        Vec4(i32x4_shuffle::<1, 2, 7, 5>(self.0, self.0))
    }

    #[inline]
    fn yzwz(self) -> Vec4 {
        Vec4(i32x4_shuffle::<1, 2, 7, 6>(self.0, self.0))
    }

    #[inline]
    fn yzww(self) -> Vec4 {
        Vec4(i32x4_shuffle::<1, 2, 7, 7>(self.0, self.0))
    }

    #[inline]
    fn ywxx(self) -> Vec4 {
        Vec4(i32x4_shuffle::<1, 3, 4, 4>(self.0, self.0))
    }

    #[inline]
    fn ywxy(self) -> Vec4 {
        Vec4(i32x4_shuffle::<1, 3, 4, 5>(self.0, self.0))
    }

    #[inline]
    fn ywxz(self) -> Vec4 {
        Vec4(i32x4_shuffle::<1, 3, 4, 6>(self.0, self.0))
    }

    #[inline]
    fn ywxw(self) -> Vec4 {
        Vec4(i32x4_shuffle::<1, 3, 4, 7>(self.0, self.0))
    }

    #[inline]
    fn ywyx(self) -> Vec4 {
        Vec4(i32x4_shuffle::<1, 3, 5, 4>(self.0, self.0))
    }

    #[inline]
    fn ywyy(self) -> Vec4 {
        Vec4(i32x4_shuffle::<1, 3, 5, 5>(self.0, self.0))
    }

    #[inline]
    fn ywyz(self) -> Vec4 {
        Vec4(i32x4_shuffle::<1, 3, 5, 6>(self.0, self.0))
    }

    #[inline]
    fn ywyw(self) -> Vec4 {
        Vec4(i32x4_shuffle::<1, 3, 5, 7>(self.0, self.0))
    }

    #[inline]
    fn ywzx(self) -> Vec4 {
        Vec4(i32x4_shuffle::<1, 3, 6, 4>(self.0, self.0))
    }

    #[inline]
    fn ywzy(self) -> Vec4 {
        Vec4(i32x4_shuffle::<1, 3, 6, 5>(self.0, self.0))
    }

    #[inline]
    fn ywzz(self) -> Vec4 {
        Vec4(i32x4_shuffle::<1, 3, 6, 6>(self.0, self.0))
    }

    #[inline]
    fn ywzw(self) -> Vec4 {
        Vec4(i32x4_shuffle::<1, 3, 6, 7>(self.0, self.0))
    }

    #[inline]
    fn ywwx(self) -> Vec4 {
        Vec4(i32x4_shuffle::<1, 3, 7, 4>(self.0, self.0))
    }

    #[inline]
    fn ywwy(self) -> Vec4 {
        Vec4(i32x4_shuffle::<1, 3, 7, 5>(self.0, self.0))
    }

    #[inline]
    fn ywwz(self) -> Vec4 {
        Vec4(i32x4_shuffle::<1, 3, 7, 6>(self.0, self.0))
    }

    #[inline]
    fn ywww(self) -> Vec4 {
        Vec4(i32x4_shuffle::<1, 3, 7, 7>(self.0, self.0))
    }

    #[inline]
    fn zxxx(self) -> Vec4 {
        Vec4(i32x4_shuffle::<2, 0, 4, 4>(self.0, self.0))
    }

    #[inline]
    fn zxxy(self) -> Vec4 {
        Vec4(i32x4_shuffle::<2, 0, 4, 5>(self.0, self.0))
    }

    #[inline]
    fn zxxz(self) -> Vec4 {
        Vec4(i32x4_shuffle::<2, 0, 4, 6>(self.0, self.0))
    }

    #[inline]
    fn zxxw(self) -> Vec4 {
        Vec4(i32x4_shuffle::<2, 0, 4, 7>(self.0, self.0))
    }

    #[inline]
    fn zxyx(self) -> Vec4 {
        Vec4(i32x4_shuffle::<2, 0, 5, 4>(self.0, self.0))
    }

    #[inline]
    fn zxyy(self) -> Vec4 {
        Vec4(i32x4_shuffle::<2, 0, 5, 5>(self.0, self.0))
    }

    #[inline]
    fn zxyz(self) -> Vec4 {
        Vec4(i32x4_shuffle::<2, 0, 5, 6>(self.0, self.0))
    }

    #[inline]
    fn zxyw(self) -> Vec4 {
        Vec4(i32x4_shuffle::<2, 0, 5, 7>(self.0, self.0))
    }

    #[inline]
    fn zxzx(self) -> Vec4 {
        Vec4(i32x4_shuffle::<2, 0, 6, 4>(self.0, self.0))
    }

    #[inline]
    fn zxzy(self) -> Vec4 {
        Vec4(i32x4_shuffle::<2, 0, 6, 5>(self.0, self.0))
    }

    #[inline]
    fn zxzz(self) -> Vec4 {
        Vec4(i32x4_shuffle::<2, 0, 6, 6>(self.0, self.0))
    }

    #[inline]
    fn zxzw(self) -> Vec4 {
        Vec4(i32x4_shuffle::<2, 0, 6, 7>(self.0, self.0))
    }

    #[inline]
    fn zxwx(self) -> Vec4 {
        Vec4(i32x4_shuffle::<2, 0, 7, 4>(self.0, self.0))
    }

    #[inline]
    fn zxwy(self) -> Vec4 {
        Vec4(i32x4_shuffle::<2, 0, 7, 5>(self.0, self.0))
    }

    #[inline]
    fn zxwz(self) -> Vec4 {
        Vec4(i32x4_shuffle::<2, 0, 7, 6>(self.0, self.0))
    }

    #[inline]
    fn zxww(self) -> Vec4 {
        Vec4(i32x4_shuffle::<2, 0, 7, 7>(self.0, self.0))
    }

    #[inline]
    fn zyxx(self) -> Vec4 {
        Vec4(i32x4_shuffle::<2, 1, 4, 4>(self.0, self.0))
    }

    #[inline]
    fn zyxy(self) -> Vec4 {
        Vec4(i32x4_shuffle::<2, 1, 4, 5>(self.0, self.0))
    }

    #[inline]
    fn zyxz(self) -> Vec4 {
        Vec4(i32x4_shuffle::<2, 1, 4, 6>(self.0, self.0))
    }

    #[inline]
    fn zyxw(self) -> Vec4 {
        Vec4(i32x4_shuffle::<2, 1, 4, 7>(self.0, self.0))
    }

    #[inline]
    fn zyyx(self) -> Vec4 {
        Vec4(i32x4_shuffle::<2, 1, 5, 4>(self.0, self.0))
    }

    #[inline]
    fn zyyy(self) -> Vec4 {
        Vec4(i32x4_shuffle::<2, 1, 5, 5>(self.0, self.0))
    }

    #[inline]
    fn zyyz(self) -> Vec4 {
        Vec4(i32x4_shuffle::<2, 1, 5, 6>(self.0, self.0))
    }

    #[inline]
    fn zyyw(self) -> Vec4 {
        Vec4(i32x4_shuffle::<2, 1, 5, 7>(self.0, self.0))
    }

    #[inline]
    fn zyzx(self) -> Vec4 {
        Vec4(i32x4_shuffle::<2, 1, 6, 4>(self.0, self.0))
    }

    #[inline]
    fn zyzy(self) -> Vec4 {
        Vec4(i32x4_shuffle::<2, 1, 6, 5>(self.0, self.0))
    }

    #[inline]
    fn zyzz(self) -> Vec4 {
        Vec4(i32x4_shuffle::<2, 1, 6, 6>(self.0, self.0))
    }

    #[inline]
    fn zyzw(self) -> Vec4 {
        Vec4(i32x4_shuffle::<2, 1, 6, 7>(self.0, self.0))
    }

    #[inline]
    fn zywx(self) -> Vec4 {
        Vec4(i32x4_shuffle::<2, 1, 7, 4>(self.0, self.0))
    }

    #[inline]
    fn zywy(self) -> Vec4 {
        Vec4(i32x4_shuffle::<2, 1, 7, 5>(self.0, self.0))
    }

    #[inline]
    fn zywz(self) -> Vec4 {
        Vec4(i32x4_shuffle::<2, 1, 7, 6>(self.0, self.0))
    }

    #[inline]
    fn zyww(self) -> Vec4 {
        Vec4(i32x4_shuffle::<2, 1, 7, 7>(self.0, self.0))
    }

    #[inline]
    fn zzxx(self) -> Vec4 {
        Vec4(i32x4_shuffle::<2, 2, 4, 4>(self.0, self.0))
    }

    #[inline]
    fn zzxy(self) -> Vec4 {
        Vec4(i32x4_shuffle::<2, 2, 4, 5>(self.0, self.0))
    }

    #[inline]
    fn zzxz(self) -> Vec4 {
        Vec4(i32x4_shuffle::<2, 2, 4, 6>(self.0, self.0))
    }

    #[inline]
    fn zzxw(self) -> Vec4 {
        Vec4(i32x4_shuffle::<2, 2, 4, 7>(self.0, self.0))
    }

    #[inline]
    fn zzyx(self) -> Vec4 {
        Vec4(i32x4_shuffle::<2, 2, 5, 4>(self.0, self.0))
    }

    #[inline]
    fn zzyy(self) -> Vec4 {
        Vec4(i32x4_shuffle::<2, 2, 5, 5>(self.0, self.0))
    }

    #[inline]
    fn zzyz(self) -> Vec4 {
        Vec4(i32x4_shuffle::<2, 2, 5, 6>(self.0, self.0))
    }

    #[inline]
    fn zzyw(self) -> Vec4 {
        Vec4(i32x4_shuffle::<2, 2, 5, 7>(self.0, self.0))
    }

    #[inline]
    fn zzzx(self) -> Vec4 {
        Vec4(i32x4_shuffle::<2, 2, 6, 4>(self.0, self.0))
    }

    #[inline]
    fn zzzy(self) -> Vec4 {
        Vec4(i32x4_shuffle::<2, 2, 6, 5>(self.0, self.0))
    }

    #[inline]
    fn zzzz(self) -> Vec4 {
        Vec4(i32x4_shuffle::<2, 2, 6, 6>(self.0, self.0))
    }

    #[inline]
    fn zzzw(self) -> Vec4 {
        Vec4(i32x4_shuffle::<2, 2, 6, 7>(self.0, self.0))
    }

    #[inline]
    fn zzwx(self) -> Vec4 {
        Vec4(i32x4_shuffle::<2, 2, 7, 4>(self.0, self.0))
    }

    #[inline]
    fn zzwy(self) -> Vec4 {
        Vec4(i32x4_shuffle::<2, 2, 7, 5>(self.0, self.0))
    }

    #[inline]
    fn zzwz(self) -> Vec4 {
        Vec4(i32x4_shuffle::<2, 2, 7, 6>(self.0, self.0))
    }

    #[inline]
    fn zzww(self) -> Vec4 {
        Vec4(i32x4_shuffle::<2, 2, 7, 7>(self.0, self.0))
    }

    #[inline]
    fn zwxx(self) -> Vec4 {
        Vec4(i32x4_shuffle::<2, 3, 4, 4>(self.0, self.0))
    }

    #[inline]
    fn zwxy(self) -> Vec4 {
        Vec4(i32x4_shuffle::<2, 3, 4, 5>(self.0, self.0))
    }

    #[inline]
    fn zwxz(self) -> Vec4 {
        Vec4(i32x4_shuffle::<2, 3, 4, 6>(self.0, self.0))
    }

    #[inline]
    fn zwxw(self) -> Vec4 {
        Vec4(i32x4_shuffle::<2, 3, 4, 7>(self.0, self.0))
    }

    #[inline]
    fn zwyx(self) -> Vec4 {
        Vec4(i32x4_shuffle::<2, 3, 5, 4>(self.0, self.0))
    }

    #[inline]
    fn zwyy(self) -> Vec4 {
        Vec4(i32x4_shuffle::<2, 3, 5, 5>(self.0, self.0))
    }

    #[inline]
    fn zwyz(self) -> Vec4 {
        Vec4(i32x4_shuffle::<2, 3, 5, 6>(self.0, self.0))
    }

    #[inline]
    fn zwyw(self) -> Vec4 {
        Vec4(i32x4_shuffle::<2, 3, 5, 7>(self.0, self.0))
    }

    #[inline]
    fn zwzx(self) -> Vec4 {
        Vec4(i32x4_shuffle::<2, 3, 6, 4>(self.0, self.0))
    }

    #[inline]
    fn zwzy(self) -> Vec4 {
        Vec4(i32x4_shuffle::<2, 3, 6, 5>(self.0, self.0))
    }

    #[inline]
    fn zwzz(self) -> Vec4 {
        Vec4(i32x4_shuffle::<2, 3, 6, 6>(self.0, self.0))
    }

    #[inline]
    fn zwzw(self) -> Vec4 {
        Vec4(i32x4_shuffle::<2, 3, 6, 7>(self.0, self.0))
    }

    #[inline]
    fn zwwx(self) -> Vec4 {
        Vec4(i32x4_shuffle::<2, 3, 7, 4>(self.0, self.0))
    }

    #[inline]
    fn zwwy(self) -> Vec4 {
        Vec4(i32x4_shuffle::<2, 3, 7, 5>(self.0, self.0))
    }

    #[inline]
    fn zwwz(self) -> Vec4 {
        Vec4(i32x4_shuffle::<2, 3, 7, 6>(self.0, self.0))
    }

    #[inline]
    fn zwww(self) -> Vec4 {
        Vec4(i32x4_shuffle::<2, 3, 7, 7>(self.0, self.0))
    }

    #[inline]
    fn wxxx(self) -> Vec4 {
        Vec4(i32x4_shuffle::<3, 0, 4, 4>(self.0, self.0))
    }

    #[inline]
    fn wxxy(self) -> Vec4 {
        Vec4(i32x4_shuffle::<3, 0, 4, 5>(self.0, self.0))
    }

    #[inline]
    fn wxxz(self) -> Vec4 {
        Vec4(i32x4_shuffle::<3, 0, 4, 6>(self.0, self.0))
    }

    #[inline]
    fn wxxw(self) -> Vec4 {
        Vec4(i32x4_shuffle::<3, 0, 4, 7>(self.0, self.0))
    }

    #[inline]
    fn wxyx(self) -> Vec4 {
        Vec4(i32x4_shuffle::<3, 0, 5, 4>(self.0, self.0))
    }

    #[inline]
    fn wxyy(self) -> Vec4 {
        Vec4(i32x4_shuffle::<3, 0, 5, 5>(self.0, self.0))
    }

    #[inline]
    fn wxyz(self) -> Vec4 {
        Vec4(i32x4_shuffle::<3, 0, 5, 6>(self.0, self.0))
    }

    #[inline]
    fn wxyw(self) -> Vec4 {
        Vec4(i32x4_shuffle::<3, 0, 5, 7>(self.0, self.0))
    }

    #[inline]
    fn wxzx(self) -> Vec4 {
        Vec4(i32x4_shuffle::<3, 0, 6, 4>(self.0, self.0))
    }

    #[inline]
    fn wxzy(self) -> Vec4 {
        Vec4(i32x4_shuffle::<3, 0, 6, 5>(self.0, self.0))
    }

    #[inline]
    fn wxzz(self) -> Vec4 {
        Vec4(i32x4_shuffle::<3, 0, 6, 6>(self.0, self.0))
    }

    #[inline]
    fn wxzw(self) -> Vec4 {
        Vec4(i32x4_shuffle::<3, 0, 6, 7>(self.0, self.0))
    }

    #[inline]
    fn wxwx(self) -> Vec4 {
        Vec4(i32x4_shuffle::<3, 0, 7, 4>(self.0, self.0))
    }

    #[inline]
    fn wxwy(self) -> Vec4 {
        Vec4(i32x4_shuffle::<3, 0, 7, 5>(self.0, self.0))
    }

    #[inline]
    fn wxwz(self) -> Vec4 {
        Vec4(i32x4_shuffle::<3, 0, 7, 6>(self.0, self.0))
    }

    #[inline]
    fn wxww(self) -> Vec4 {
        Vec4(i32x4_shuffle::<3, 0, 7, 7>(self.0, self.0))
    }

    #[inline]
    fn wyxx(self) -> Vec4 {
        Vec4(i32x4_shuffle::<3, 1, 4, 4>(self.0, self.0))
    }

    #[inline]
    fn wyxy(self) -> Vec4 {
        Vec4(i32x4_shuffle::<3, 1, 4, 5>(self.0, self.0))
    }

    #[inline]
    fn wyxz(self) -> Vec4 {
        Vec4(i32x4_shuffle::<3, 1, 4, 6>(self.0, self.0))
    }

    #[inline]
    fn wyxw(self) -> Vec4 {
        Vec4(i32x4_shuffle::<3, 1, 4, 7>(self.0, self.0))
    }

    #[inline]
    fn wyyx(self) -> Vec4 {
        Vec4(i32x4_shuffle::<3, 1, 5, 4>(self.0, self.0))
    }

    #[inline]
    fn wyyy(self) -> Vec4 {
        Vec4(i32x4_shuffle::<3, 1, 5, 5>(self.0, self.0))
    }

    #[inline]
    fn wyyz(self) -> Vec4 {
        Vec4(i32x4_shuffle::<3, 1, 5, 6>(self.0, self.0))
    }

    #[inline]
    fn wyyw(self) -> Vec4 {
        Vec4(i32x4_shuffle::<3, 1, 5, 7>(self.0, self.0))
    }

    #[inline]
    fn wyzx(self) -> Vec4 {
        Vec4(i32x4_shuffle::<3, 1, 6, 4>(self.0, self.0))
    }

    #[inline]
    fn wyzy(self) -> Vec4 {
        Vec4(i32x4_shuffle::<3, 1, 6, 5>(self.0, self.0))
    }

    #[inline]
    fn wyzz(self) -> Vec4 {
        Vec4(i32x4_shuffle::<3, 1, 6, 6>(self.0, self.0))
    }

    #[inline]
    fn wyzw(self) -> Vec4 {
        Vec4(i32x4_shuffle::<3, 1, 6, 7>(self.0, self.0))
    }

    #[inline]
    fn wywx(self) -> Vec4 {
        Vec4(i32x4_shuffle::<3, 1, 7, 4>(self.0, self.0))
    }

    #[inline]
    fn wywy(self) -> Vec4 {
        Vec4(i32x4_shuffle::<3, 1, 7, 5>(self.0, self.0))
    }

    #[inline]
    fn wywz(self) -> Vec4 {
        Vec4(i32x4_shuffle::<3, 1, 7, 6>(self.0, self.0))
    }

    #[inline]
    fn wyww(self) -> Vec4 {
        Vec4(i32x4_shuffle::<3, 1, 7, 7>(self.0, self.0))
    }

    #[inline]
    fn wzxx(self) -> Vec4 {
        Vec4(i32x4_shuffle::<3, 2, 4, 4>(self.0, self.0))
    }

    #[inline]
    fn wzxy(self) -> Vec4 {
        Vec4(i32x4_shuffle::<3, 2, 4, 5>(self.0, self.0))
    }

    #[inline]
    fn wzxz(self) -> Vec4 {
        Vec4(i32x4_shuffle::<3, 2, 4, 6>(self.0, self.0))
    }

    #[inline]
    fn wzxw(self) -> Vec4 {
        Vec4(i32x4_shuffle::<3, 2, 4, 7>(self.0, self.0))
    }

    #[inline]
    fn wzyx(self) -> Vec4 {
        Vec4(i32x4_shuffle::<3, 2, 5, 4>(self.0, self.0))
    }

    #[inline]
    fn wzyy(self) -> Vec4 {
        Vec4(i32x4_shuffle::<3, 2, 5, 5>(self.0, self.0))
    }

    #[inline]
    fn wzyz(self) -> Vec4 {
        Vec4(i32x4_shuffle::<3, 2, 5, 6>(self.0, self.0))
    }

    #[inline]
    fn wzyw(self) -> Vec4 {
        Vec4(i32x4_shuffle::<3, 2, 5, 7>(self.0, self.0))
    }

    #[inline]
    fn wzzx(self) -> Vec4 {
        Vec4(i32x4_shuffle::<3, 2, 6, 4>(self.0, self.0))
    }

    #[inline]
    fn wzzy(self) -> Vec4 {
        Vec4(i32x4_shuffle::<3, 2, 6, 5>(self.0, self.0))
    }

    #[inline]
    fn wzzz(self) -> Vec4 {
        Vec4(i32x4_shuffle::<3, 2, 6, 6>(self.0, self.0))
    }

    #[inline]
    fn wzzw(self) -> Vec4 {
        Vec4(i32x4_shuffle::<3, 2, 6, 7>(self.0, self.0))
    }

    #[inline]
    fn wzwx(self) -> Vec4 {
        Vec4(i32x4_shuffle::<3, 2, 7, 4>(self.0, self.0))
    }

    #[inline]
    fn wzwy(self) -> Vec4 {
        Vec4(i32x4_shuffle::<3, 2, 7, 5>(self.0, self.0))
    }

    #[inline]
    fn wzwz(self) -> Vec4 {
        Vec4(i32x4_shuffle::<3, 2, 7, 6>(self.0, self.0))
    }

    #[inline]
    fn wzww(self) -> Vec4 {
        Vec4(i32x4_shuffle::<3, 2, 7, 7>(self.0, self.0))
    }

    #[inline]
    fn wwxx(self) -> Vec4 {
        Vec4(i32x4_shuffle::<3, 3, 4, 4>(self.0, self.0))
    }

    #[inline]
    fn wwxy(self) -> Vec4 {
        Vec4(i32x4_shuffle::<3, 3, 4, 5>(self.0, self.0))
    }

    #[inline]
    fn wwxz(self) -> Vec4 {
        Vec4(i32x4_shuffle::<3, 3, 4, 6>(self.0, self.0))
    }

    #[inline]
    fn wwxw(self) -> Vec4 {
        Vec4(i32x4_shuffle::<3, 3, 4, 7>(self.0, self.0))
    }

    #[inline]
    fn wwyx(self) -> Vec4 {
        Vec4(i32x4_shuffle::<3, 3, 5, 4>(self.0, self.0))
    }

    #[inline]
    fn wwyy(self) -> Vec4 {
        Vec4(i32x4_shuffle::<3, 3, 5, 5>(self.0, self.0))
    }

    #[inline]
    fn wwyz(self) -> Vec4 {
        Vec4(i32x4_shuffle::<3, 3, 5, 6>(self.0, self.0))
    }

    #[inline]
    fn wwyw(self) -> Vec4 {
        Vec4(i32x4_shuffle::<3, 3, 5, 7>(self.0, self.0))
    }

    #[inline]
    fn wwzx(self) -> Vec4 {
        Vec4(i32x4_shuffle::<3, 3, 6, 4>(self.0, self.0))
    }

    #[inline]
    fn wwzy(self) -> Vec4 {
        Vec4(i32x4_shuffle::<3, 3, 6, 5>(self.0, self.0))
    }

    #[inline]
    fn wwzz(self) -> Vec4 {
        Vec4(i32x4_shuffle::<3, 3, 6, 6>(self.0, self.0))
    }

    #[inline]
    fn wwzw(self) -> Vec4 {
        Vec4(i32x4_shuffle::<3, 3, 6, 7>(self.0, self.0))
    }

    #[inline]
    fn wwwx(self) -> Vec4 {
        Vec4(i32x4_shuffle::<3, 3, 7, 4>(self.0, self.0))
    }

    #[inline]
    fn wwwy(self) -> Vec4 {
        Vec4(i32x4_shuffle::<3, 3, 7, 5>(self.0, self.0))
    }

    #[inline]
    fn wwwz(self) -> Vec4 {
        Vec4(i32x4_shuffle::<3, 3, 7, 6>(self.0, self.0))
    }

    #[inline]
    fn wwww(self) -> Vec4 {
        Vec4(i32x4_shuffle::<3, 3, 7, 7>(self.0, self.0))
    }
}
