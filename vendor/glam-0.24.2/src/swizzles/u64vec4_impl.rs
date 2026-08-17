// Generated from swizzle_impl.rs.tera template. Edit the template, not the generated file.

use crate::{U64Vec2, U64Vec3, U64Vec4, Vec4Swizzles};

impl Vec4Swizzles for U64Vec4 {
    type Vec2 = U64Vec2;

    type Vec3 = U64Vec3;

    #[inline]
    fn xx(self) -> U64Vec2 {
        U64Vec2 {
            x: self.x,
            y: self.x,
        }
    }

    #[inline]
    fn xy(self) -> U64Vec2 {
        U64Vec2 {
            x: self.x,
            y: self.y,
        }
    }

    #[inline]
    fn xz(self) -> U64Vec2 {
        U64Vec2 {
            x: self.x,
            y: self.z,
        }
    }

    #[inline]
    fn xw(self) -> U64Vec2 {
        U64Vec2 {
            x: self.x,
            y: self.w,
        }
    }

    #[inline]
    fn yx(self) -> U64Vec2 {
        U64Vec2 {
            x: self.y,
            y: self.x,
        }
    }

    #[inline]
    fn yy(self) -> U64Vec2 {
        U64Vec2 {
            x: self.y,
            y: self.y,
        }
    }

    #[inline]
    fn yz(self) -> U64Vec2 {
        U64Vec2 {
            x: self.y,
            y: self.z,
        }
    }

    #[inline]
    fn yw(self) -> U64Vec2 {
        U64Vec2 {
            x: self.y,
            y: self.w,
        }
    }

    #[inline]
    fn zx(self) -> U64Vec2 {
        U64Vec2 {
            x: self.z,
            y: self.x,
        }
    }

    #[inline]
    fn zy(self) -> U64Vec2 {
        U64Vec2 {
            x: self.z,
            y: self.y,
        }
    }

    #[inline]
    fn zz(self) -> U64Vec2 {
        U64Vec2 {
            x: self.z,
            y: self.z,
        }
    }

    #[inline]
    fn zw(self) -> U64Vec2 {
        U64Vec2 {
            x: self.z,
            y: self.w,
        }
    }

    #[inline]
    fn wx(self) -> U64Vec2 {
        U64Vec2 {
            x: self.w,
            y: self.x,
        }
    }

    #[inline]
    fn wy(self) -> U64Vec2 {
        U64Vec2 {
            x: self.w,
            y: self.y,
        }
    }

    #[inline]
    fn wz(self) -> U64Vec2 {
        U64Vec2 {
            x: self.w,
            y: self.z,
        }
    }

    #[inline]
    fn ww(self) -> U64Vec2 {
        U64Vec2 {
            x: self.w,
            y: self.w,
        }
    }

    #[inline]
    fn xxx(self) -> U64Vec3 {
        U64Vec3 {
            x: self.x,
            y: self.x,
            z: self.x,
        }
    }

    #[inline]
    fn xxy(self) -> U64Vec3 {
        U64Vec3 {
            x: self.x,
            y: self.x,
            z: self.y,
        }
    }

    #[inline]
    fn xxz(self) -> U64Vec3 {
        U64Vec3 {
            x: self.x,
            y: self.x,
            z: self.z,
        }
    }

    #[inline]
    fn xxw(self) -> U64Vec3 {
        U64Vec3 {
            x: self.x,
            y: self.x,
            z: self.w,
        }
    }

    #[inline]
    fn xyx(self) -> U64Vec3 {
        U64Vec3 {
            x: self.x,
            y: self.y,
            z: self.x,
        }
    }

    #[inline]
    fn xyy(self) -> U64Vec3 {
        U64Vec3 {
            x: self.x,
            y: self.y,
            z: self.y,
        }
    }

    #[inline]
    fn xyz(self) -> U64Vec3 {
        U64Vec3 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }

    #[inline]
    fn xyw(self) -> U64Vec3 {
        U64Vec3 {
            x: self.x,
            y: self.y,
            z: self.w,
        }
    }

    #[inline]
    fn xzx(self) -> U64Vec3 {
        U64Vec3 {
            x: self.x,
            y: self.z,
            z: self.x,
        }
    }

    #[inline]
    fn xzy(self) -> U64Vec3 {
        U64Vec3 {
            x: self.x,
            y: self.z,
            z: self.y,
        }
    }

    #[inline]
    fn xzz(self) -> U64Vec3 {
        U64Vec3 {
            x: self.x,
            y: self.z,
            z: self.z,
        }
    }

    #[inline]
    fn xzw(self) -> U64Vec3 {
        U64Vec3 {
            x: self.x,
            y: self.z,
            z: self.w,
        }
    }

    #[inline]
    fn xwx(self) -> U64Vec3 {
        U64Vec3 {
            x: self.x,
            y: self.w,
            z: self.x,
        }
    }

    #[inline]
    fn xwy(self) -> U64Vec3 {
        U64Vec3 {
            x: self.x,
            y: self.w,
            z: self.y,
        }
    }

    #[inline]
    fn xwz(self) -> U64Vec3 {
        U64Vec3 {
            x: self.x,
            y: self.w,
            z: self.z,
        }
    }

    #[inline]
    fn xww(self) -> U64Vec3 {
        U64Vec3 {
            x: self.x,
            y: self.w,
            z: self.w,
        }
    }

    #[inline]
    fn yxx(self) -> U64Vec3 {
        U64Vec3 {
            x: self.y,
            y: self.x,
            z: self.x,
        }
    }

    #[inline]
    fn yxy(self) -> U64Vec3 {
        U64Vec3 {
            x: self.y,
            y: self.x,
            z: self.y,
        }
    }

    #[inline]
    fn yxz(self) -> U64Vec3 {
        U64Vec3 {
            x: self.y,
            y: self.x,
            z: self.z,
        }
    }

    #[inline]
    fn yxw(self) -> U64Vec3 {
        U64Vec3 {
            x: self.y,
            y: self.x,
            z: self.w,
        }
    }

    #[inline]
    fn yyx(self) -> U64Vec3 {
        U64Vec3 {
            x: self.y,
            y: self.y,
            z: self.x,
        }
    }

    #[inline]
    fn yyy(self) -> U64Vec3 {
        U64Vec3 {
            x: self.y,
            y: self.y,
            z: self.y,
        }
    }

    #[inline]
    fn yyz(self) -> U64Vec3 {
        U64Vec3 {
            x: self.y,
            y: self.y,
            z: self.z,
        }
    }

    #[inline]
    fn yyw(self) -> U64Vec3 {
        U64Vec3 {
            x: self.y,
            y: self.y,
            z: self.w,
        }
    }

    #[inline]
    fn yzx(self) -> U64Vec3 {
        U64Vec3 {
            x: self.y,
            y: self.z,
            z: self.x,
        }
    }

    #[inline]
    fn yzy(self) -> U64Vec3 {
        U64Vec3 {
            x: self.y,
            y: self.z,
            z: self.y,
        }
    }

    #[inline]
    fn yzz(self) -> U64Vec3 {
        U64Vec3 {
            x: self.y,
            y: self.z,
            z: self.z,
        }
    }

    #[inline]
    fn yzw(self) -> U64Vec3 {
        U64Vec3 {
            x: self.y,
            y: self.z,
            z: self.w,
        }
    }

    #[inline]
    fn ywx(self) -> U64Vec3 {
        U64Vec3 {
            x: self.y,
            y: self.w,
            z: self.x,
        }
    }

    #[inline]
    fn ywy(self) -> U64Vec3 {
        U64Vec3 {
            x: self.y,
            y: self.w,
            z: self.y,
        }
    }

    #[inline]
    fn ywz(self) -> U64Vec3 {
        U64Vec3 {
            x: self.y,
            y: self.w,
            z: self.z,
        }
    }

    #[inline]
    fn yww(self) -> U64Vec3 {
        U64Vec3 {
            x: self.y,
            y: self.w,
            z: self.w,
        }
    }

    #[inline]
    fn zxx(self) -> U64Vec3 {
        U64Vec3 {
            x: self.z,
            y: self.x,
            z: self.x,
        }
    }

    #[inline]
    fn zxy(self) -> U64Vec3 {
        U64Vec3 {
            x: self.z,
            y: self.x,
            z: self.y,
        }
    }

    #[inline]
    fn zxz(self) -> U64Vec3 {
        U64Vec3 {
            x: self.z,
            y: self.x,
            z: self.z,
        }
    }

    #[inline]
    fn zxw(self) -> U64Vec3 {
        U64Vec3 {
            x: self.z,
            y: self.x,
            z: self.w,
        }
    }

    #[inline]
    fn zyx(self) -> U64Vec3 {
        U64Vec3 {
            x: self.z,
            y: self.y,
            z: self.x,
        }
    }

    #[inline]
    fn zyy(self) -> U64Vec3 {
        U64Vec3 {
            x: self.z,
            y: self.y,
            z: self.y,
        }
    }

    #[inline]
    fn zyz(self) -> U64Vec3 {
        U64Vec3 {
            x: self.z,
            y: self.y,
            z: self.z,
        }
    }

    #[inline]
    fn zyw(self) -> U64Vec3 {
        U64Vec3 {
            x: self.z,
            y: self.y,
            z: self.w,
        }
    }

    #[inline]
    fn zzx(self) -> U64Vec3 {
        U64Vec3 {
            x: self.z,
            y: self.z,
            z: self.x,
        }
    }

    #[inline]
    fn zzy(self) -> U64Vec3 {
        U64Vec3 {
            x: self.z,
            y: self.z,
            z: self.y,
        }
    }

    #[inline]
    fn zzz(self) -> U64Vec3 {
        U64Vec3 {
            x: self.z,
            y: self.z,
            z: self.z,
        }
    }

    #[inline]
    fn zzw(self) -> U64Vec3 {
        U64Vec3 {
            x: self.z,
            y: self.z,
            z: self.w,
        }
    }

    #[inline]
    fn zwx(self) -> U64Vec3 {
        U64Vec3 {
            x: self.z,
            y: self.w,
            z: self.x,
        }
    }

    #[inline]
    fn zwy(self) -> U64Vec3 {
        U64Vec3 {
            x: self.z,
            y: self.w,
            z: self.y,
        }
    }

    #[inline]
    fn zwz(self) -> U64Vec3 {
        U64Vec3 {
            x: self.z,
            y: self.w,
            z: self.z,
        }
    }

    #[inline]
    fn zww(self) -> U64Vec3 {
        U64Vec3 {
            x: self.z,
            y: self.w,
            z: self.w,
        }
    }

    #[inline]
    fn wxx(self) -> U64Vec3 {
        U64Vec3 {
            x: self.w,
            y: self.x,
            z: self.x,
        }
    }

    #[inline]
    fn wxy(self) -> U64Vec3 {
        U64Vec3 {
            x: self.w,
            y: self.x,
            z: self.y,
        }
    }

    #[inline]
    fn wxz(self) -> U64Vec3 {
        U64Vec3 {
            x: self.w,
            y: self.x,
            z: self.z,
        }
    }

    #[inline]
    fn wxw(self) -> U64Vec3 {
        U64Vec3 {
            x: self.w,
            y: self.x,
            z: self.w,
        }
    }

    #[inline]
    fn wyx(self) -> U64Vec3 {
        U64Vec3 {
            x: self.w,
            y: self.y,
            z: self.x,
        }
    }

    #[inline]
    fn wyy(self) -> U64Vec3 {
        U64Vec3 {
            x: self.w,
            y: self.y,
            z: self.y,
        }
    }

    #[inline]
    fn wyz(self) -> U64Vec3 {
        U64Vec3 {
            x: self.w,
            y: self.y,
            z: self.z,
        }
    }

    #[inline]
    fn wyw(self) -> U64Vec3 {
        U64Vec3 {
            x: self.w,
            y: self.y,
            z: self.w,
        }
    }

    #[inline]
    fn wzx(self) -> U64Vec3 {
        U64Vec3 {
            x: self.w,
            y: self.z,
            z: self.x,
        }
    }

    #[inline]
    fn wzy(self) -> U64Vec3 {
        U64Vec3 {
            x: self.w,
            y: self.z,
            z: self.y,
        }
    }

    #[inline]
    fn wzz(self) -> U64Vec3 {
        U64Vec3 {
            x: self.w,
            y: self.z,
            z: self.z,
        }
    }

    #[inline]
    fn wzw(self) -> U64Vec3 {
        U64Vec3 {
            x: self.w,
            y: self.z,
            z: self.w,
        }
    }

    #[inline]
    fn wwx(self) -> U64Vec3 {
        U64Vec3 {
            x: self.w,
            y: self.w,
            z: self.x,
        }
    }

    #[inline]
    fn wwy(self) -> U64Vec3 {
        U64Vec3 {
            x: self.w,
            y: self.w,
            z: self.y,
        }
    }

    #[inline]
    fn wwz(self) -> U64Vec3 {
        U64Vec3 {
            x: self.w,
            y: self.w,
            z: self.z,
        }
    }

    #[inline]
    fn www(self) -> U64Vec3 {
        U64Vec3 {
            x: self.w,
            y: self.w,
            z: self.w,
        }
    }

    #[inline]
    fn xxxx(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.x, self.x, self.x)
    }

    #[inline]
    fn xxxy(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.x, self.x, self.y)
    }

    #[inline]
    fn xxxz(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.x, self.x, self.z)
    }

    #[inline]
    fn xxxw(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.x, self.x, self.w)
    }

    #[inline]
    fn xxyx(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.x, self.y, self.x)
    }

    #[inline]
    fn xxyy(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.x, self.y, self.y)
    }

    #[inline]
    fn xxyz(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.x, self.y, self.z)
    }

    #[inline]
    fn xxyw(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.x, self.y, self.w)
    }

    #[inline]
    fn xxzx(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.x, self.z, self.x)
    }

    #[inline]
    fn xxzy(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.x, self.z, self.y)
    }

    #[inline]
    fn xxzz(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.x, self.z, self.z)
    }

    #[inline]
    fn xxzw(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.x, self.z, self.w)
    }

    #[inline]
    fn xxwx(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.x, self.w, self.x)
    }

    #[inline]
    fn xxwy(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.x, self.w, self.y)
    }

    #[inline]
    fn xxwz(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.x, self.w, self.z)
    }

    #[inline]
    fn xxww(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.x, self.w, self.w)
    }

    #[inline]
    fn xyxx(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.y, self.x, self.x)
    }

    #[inline]
    fn xyxy(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.y, self.x, self.y)
    }

    #[inline]
    fn xyxz(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.y, self.x, self.z)
    }

    #[inline]
    fn xyxw(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.y, self.x, self.w)
    }

    #[inline]
    fn xyyx(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.y, self.y, self.x)
    }

    #[inline]
    fn xyyy(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.y, self.y, self.y)
    }

    #[inline]
    fn xyyz(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.y, self.y, self.z)
    }

    #[inline]
    fn xyyw(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.y, self.y, self.w)
    }

    #[inline]
    fn xyzx(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.y, self.z, self.x)
    }

    #[inline]
    fn xyzy(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.y, self.z, self.y)
    }

    #[inline]
    fn xyzz(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.y, self.z, self.z)
    }

    #[inline]
    fn xyzw(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.y, self.z, self.w)
    }

    #[inline]
    fn xywx(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.y, self.w, self.x)
    }

    #[inline]
    fn xywy(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.y, self.w, self.y)
    }

    #[inline]
    fn xywz(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.y, self.w, self.z)
    }

    #[inline]
    fn xyww(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.y, self.w, self.w)
    }

    #[inline]
    fn xzxx(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.z, self.x, self.x)
    }

    #[inline]
    fn xzxy(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.z, self.x, self.y)
    }

    #[inline]
    fn xzxz(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.z, self.x, self.z)
    }

    #[inline]
    fn xzxw(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.z, self.x, self.w)
    }

    #[inline]
    fn xzyx(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.z, self.y, self.x)
    }

    #[inline]
    fn xzyy(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.z, self.y, self.y)
    }

    #[inline]
    fn xzyz(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.z, self.y, self.z)
    }

    #[inline]
    fn xzyw(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.z, self.y, self.w)
    }

    #[inline]
    fn xzzx(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.z, self.z, self.x)
    }

    #[inline]
    fn xzzy(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.z, self.z, self.y)
    }

    #[inline]
    fn xzzz(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.z, self.z, self.z)
    }

    #[inline]
    fn xzzw(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.z, self.z, self.w)
    }

    #[inline]
    fn xzwx(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.z, self.w, self.x)
    }

    #[inline]
    fn xzwy(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.z, self.w, self.y)
    }

    #[inline]
    fn xzwz(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.z, self.w, self.z)
    }

    #[inline]
    fn xzww(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.z, self.w, self.w)
    }

    #[inline]
    fn xwxx(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.w, self.x, self.x)
    }

    #[inline]
    fn xwxy(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.w, self.x, self.y)
    }

    #[inline]
    fn xwxz(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.w, self.x, self.z)
    }

    #[inline]
    fn xwxw(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.w, self.x, self.w)
    }

    #[inline]
    fn xwyx(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.w, self.y, self.x)
    }

    #[inline]
    fn xwyy(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.w, self.y, self.y)
    }

    #[inline]
    fn xwyz(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.w, self.y, self.z)
    }

    #[inline]
    fn xwyw(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.w, self.y, self.w)
    }

    #[inline]
    fn xwzx(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.w, self.z, self.x)
    }

    #[inline]
    fn xwzy(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.w, self.z, self.y)
    }

    #[inline]
    fn xwzz(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.w, self.z, self.z)
    }

    #[inline]
    fn xwzw(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.w, self.z, self.w)
    }

    #[inline]
    fn xwwx(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.w, self.w, self.x)
    }

    #[inline]
    fn xwwy(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.w, self.w, self.y)
    }

    #[inline]
    fn xwwz(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.w, self.w, self.z)
    }

    #[inline]
    fn xwww(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.w, self.w, self.w)
    }

    #[inline]
    fn yxxx(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.x, self.x, self.x)
    }

    #[inline]
    fn yxxy(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.x, self.x, self.y)
    }

    #[inline]
    fn yxxz(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.x, self.x, self.z)
    }

    #[inline]
    fn yxxw(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.x, self.x, self.w)
    }

    #[inline]
    fn yxyx(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.x, self.y, self.x)
    }

    #[inline]
    fn yxyy(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.x, self.y, self.y)
    }

    #[inline]
    fn yxyz(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.x, self.y, self.z)
    }

    #[inline]
    fn yxyw(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.x, self.y, self.w)
    }

    #[inline]
    fn yxzx(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.x, self.z, self.x)
    }

    #[inline]
    fn yxzy(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.x, self.z, self.y)
    }

    #[inline]
    fn yxzz(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.x, self.z, self.z)
    }

    #[inline]
    fn yxzw(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.x, self.z, self.w)
    }

    #[inline]
    fn yxwx(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.x, self.w, self.x)
    }

    #[inline]
    fn yxwy(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.x, self.w, self.y)
    }

    #[inline]
    fn yxwz(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.x, self.w, self.z)
    }

    #[inline]
    fn yxww(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.x, self.w, self.w)
    }

    #[inline]
    fn yyxx(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.y, self.x, self.x)
    }

    #[inline]
    fn yyxy(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.y, self.x, self.y)
    }

    #[inline]
    fn yyxz(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.y, self.x, self.z)
    }

    #[inline]
    fn yyxw(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.y, self.x, self.w)
    }

    #[inline]
    fn yyyx(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.y, self.y, self.x)
    }

    #[inline]
    fn yyyy(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.y, self.y, self.y)
    }

    #[inline]
    fn yyyz(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.y, self.y, self.z)
    }

    #[inline]
    fn yyyw(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.y, self.y, self.w)
    }

    #[inline]
    fn yyzx(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.y, self.z, self.x)
    }

    #[inline]
    fn yyzy(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.y, self.z, self.y)
    }

    #[inline]
    fn yyzz(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.y, self.z, self.z)
    }

    #[inline]
    fn yyzw(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.y, self.z, self.w)
    }

    #[inline]
    fn yywx(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.y, self.w, self.x)
    }

    #[inline]
    fn yywy(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.y, self.w, self.y)
    }

    #[inline]
    fn yywz(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.y, self.w, self.z)
    }

    #[inline]
    fn yyww(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.y, self.w, self.w)
    }

    #[inline]
    fn yzxx(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.z, self.x, self.x)
    }

    #[inline]
    fn yzxy(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.z, self.x, self.y)
    }

    #[inline]
    fn yzxz(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.z, self.x, self.z)
    }

    #[inline]
    fn yzxw(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.z, self.x, self.w)
    }

    #[inline]
    fn yzyx(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.z, self.y, self.x)
    }

    #[inline]
    fn yzyy(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.z, self.y, self.y)
    }

    #[inline]
    fn yzyz(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.z, self.y, self.z)
    }

    #[inline]
    fn yzyw(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.z, self.y, self.w)
    }

    #[inline]
    fn yzzx(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.z, self.z, self.x)
    }

    #[inline]
    fn yzzy(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.z, self.z, self.y)
    }

    #[inline]
    fn yzzz(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.z, self.z, self.z)
    }

    #[inline]
    fn yzzw(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.z, self.z, self.w)
    }

    #[inline]
    fn yzwx(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.z, self.w, self.x)
    }

    #[inline]
    fn yzwy(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.z, self.w, self.y)
    }

    #[inline]
    fn yzwz(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.z, self.w, self.z)
    }

    #[inline]
    fn yzww(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.z, self.w, self.w)
    }

    #[inline]
    fn ywxx(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.w, self.x, self.x)
    }

    #[inline]
    fn ywxy(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.w, self.x, self.y)
    }

    #[inline]
    fn ywxz(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.w, self.x, self.z)
    }

    #[inline]
    fn ywxw(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.w, self.x, self.w)
    }

    #[inline]
    fn ywyx(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.w, self.y, self.x)
    }

    #[inline]
    fn ywyy(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.w, self.y, self.y)
    }

    #[inline]
    fn ywyz(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.w, self.y, self.z)
    }

    #[inline]
    fn ywyw(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.w, self.y, self.w)
    }

    #[inline]
    fn ywzx(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.w, self.z, self.x)
    }

    #[inline]
    fn ywzy(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.w, self.z, self.y)
    }

    #[inline]
    fn ywzz(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.w, self.z, self.z)
    }

    #[inline]
    fn ywzw(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.w, self.z, self.w)
    }

    #[inline]
    fn ywwx(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.w, self.w, self.x)
    }

    #[inline]
    fn ywwy(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.w, self.w, self.y)
    }

    #[inline]
    fn ywwz(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.w, self.w, self.z)
    }

    #[inline]
    fn ywww(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.w, self.w, self.w)
    }

    #[inline]
    fn zxxx(self) -> U64Vec4 {
        U64Vec4::new(self.z, self.x, self.x, self.x)
    }

    #[inline]
    fn zxxy(self) -> U64Vec4 {
        U64Vec4::new(self.z, self.x, self.x, self.y)
    }

    #[inline]
    fn zxxz(self) -> U64Vec4 {
        U64Vec4::new(self.z, self.x, self.x, self.z)
    }

    #[inline]
    fn zxxw(self) -> U64Vec4 {
        U64Vec4::new(self.z, self.x, self.x, self.w)
    }

    #[inline]
    fn zxyx(self) -> U64Vec4 {
        U64Vec4::new(self.z, self.x, self.y, self.x)
    }

    #[inline]
    fn zxyy(self) -> U64Vec4 {
        U64Vec4::new(self.z, self.x, self.y, self.y)
    }

    #[inline]
    fn zxyz(self) -> U64Vec4 {
        U64Vec4::new(self.z, self.x, self.y, self.z)
    }

    #[inline]
    fn zxyw(self) -> U64Vec4 {
        U64Vec4::new(self.z, self.x, self.y, self.w)
    }

    #[inline]
    fn zxzx(self) -> U64Vec4 {
        U64Vec4::new(self.z, self.x, self.z, self.x)
    }

    #[inline]
    fn zxzy(self) -> U64Vec4 {
        U64Vec4::new(self.z, self.x, self.z, self.y)
    }

    #[inline]
    fn zxzz(self) -> U64Vec4 {
        U64Vec4::new(self.z, self.x, self.z, self.z)
    }

    #[inline]
    fn zxzw(self) -> U64Vec4 {
        U64Vec4::new(self.z, self.x, self.z, self.w)
    }

    #[inline]
    fn zxwx(self) -> U64Vec4 {
        U64Vec4::new(self.z, self.x, self.w, self.x)
    }

    #[inline]
    fn zxwy(self) -> U64Vec4 {
        U64Vec4::new(self.z, self.x, self.w, self.y)
    }

    #[inline]
    fn zxwz(self) -> U64Vec4 {
        U64Vec4::new(self.z, self.x, self.w, self.z)
    }

    #[inline]
    fn zxww(self) -> U64Vec4 {
        U64Vec4::new(self.z, self.x, self.w, self.w)
    }

    #[inline]
    fn zyxx(self) -> U64Vec4 {
        U64Vec4::new(self.z, self.y, self.x, self.x)
    }

    #[inline]
    fn zyxy(self) -> U64Vec4 {
        U64Vec4::new(self.z, self.y, self.x, self.y)
    }

    #[inline]
    fn zyxz(self) -> U64Vec4 {
        U64Vec4::new(self.z, self.y, self.x, self.z)
    }

    #[inline]
    fn zyxw(self) -> U64Vec4 {
        U64Vec4::new(self.z, self.y, self.x, self.w)
    }

    #[inline]
    fn zyyx(self) -> U64Vec4 {
        U64Vec4::new(self.z, self.y, self.y, self.x)
    }

    #[inline]
    fn zyyy(self) -> U64Vec4 {
        U64Vec4::new(self.z, self.y, self.y, self.y)
    }

    #[inline]
    fn zyyz(self) -> U64Vec4 {
        U64Vec4::new(self.z, self.y, self.y, self.z)
    }

    #[inline]
    fn zyyw(self) -> U64Vec4 {
        U64Vec4::new(self.z, self.y, self.y, self.w)
    }

    #[inline]
    fn zyzx(self) -> U64Vec4 {
        U64Vec4::new(self.z, self.y, self.z, self.x)
    }

    #[inline]
    fn zyzy(self) -> U64Vec4 {
        U64Vec4::new(self.z, self.y, self.z, self.y)
    }

    #[inline]
    fn zyzz(self) -> U64Vec4 {
        U64Vec4::new(self.z, self.y, self.z, self.z)
    }

    #[inline]
    fn zyzw(self) -> U64Vec4 {
        U64Vec4::new(self.z, self.y, self.z, self.w)
    }

    #[inline]
    fn zywx(self) -> U64Vec4 {
        U64Vec4::new(self.z, self.y, self.w, self.x)
    }

    #[inline]
    fn zywy(self) -> U64Vec4 {
        U64Vec4::new(self.z, self.y, self.w, self.y)
    }

    #[inline]
    fn zywz(self) -> U64Vec4 {
        U64Vec4::new(self.z, self.y, self.w, self.z)
    }

    #[inline]
    fn zyww(self) -> U64Vec4 {
        U64Vec4::new(self.z, self.y, self.w, self.w)
    }

    #[inline]
    fn zzxx(self) -> U64Vec4 {
        U64Vec4::new(self.z, self.z, self.x, self.x)
    }

    #[inline]
    fn zzxy(self) -> U64Vec4 {
        U64Vec4::new(self.z, self.z, self.x, self.y)
    }

    #[inline]
    fn zzxz(self) -> U64Vec4 {
        U64Vec4::new(self.z, self.z, self.x, self.z)
    }

    #[inline]
    fn zzxw(self) -> U64Vec4 {
        U64Vec4::new(self.z, self.z, self.x, self.w)
    }

    #[inline]
    fn zzyx(self) -> U64Vec4 {
        U64Vec4::new(self.z, self.z, self.y, self.x)
    }

    #[inline]
    fn zzyy(self) -> U64Vec4 {
        U64Vec4::new(self.z, self.z, self.y, self.y)
    }

    #[inline]
    fn zzyz(self) -> U64Vec4 {
        U64Vec4::new(self.z, self.z, self.y, self.z)
    }

    #[inline]
    fn zzyw(self) -> U64Vec4 {
        U64Vec4::new(self.z, self.z, self.y, self.w)
    }

    #[inline]
    fn zzzx(self) -> U64Vec4 {
        U64Vec4::new(self.z, self.z, self.z, self.x)
    }

    #[inline]
    fn zzzy(self) -> U64Vec4 {
        U64Vec4::new(self.z, self.z, self.z, self.y)
    }

    #[inline]
    fn zzzz(self) -> U64Vec4 {
        U64Vec4::new(self.z, self.z, self.z, self.z)
    }

    #[inline]
    fn zzzw(self) -> U64Vec4 {
        U64Vec4::new(self.z, self.z, self.z, self.w)
    }

    #[inline]
    fn zzwx(self) -> U64Vec4 {
        U64Vec4::new(self.z, self.z, self.w, self.x)
    }

    #[inline]
    fn zzwy(self) -> U64Vec4 {
        U64Vec4::new(self.z, self.z, self.w, self.y)
    }

    #[inline]
    fn zzwz(self) -> U64Vec4 {
        U64Vec4::new(self.z, self.z, self.w, self.z)
    }

    #[inline]
    fn zzww(self) -> U64Vec4 {
        U64Vec4::new(self.z, self.z, self.w, self.w)
    }

    #[inline]
    fn zwxx(self) -> U64Vec4 {
        U64Vec4::new(self.z, self.w, self.x, self.x)
    }

    #[inline]
    fn zwxy(self) -> U64Vec4 {
        U64Vec4::new(self.z, self.w, self.x, self.y)
    }

    #[inline]
    fn zwxz(self) -> U64Vec4 {
        U64Vec4::new(self.z, self.w, self.x, self.z)
    }

    #[inline]
    fn zwxw(self) -> U64Vec4 {
        U64Vec4::new(self.z, self.w, self.x, self.w)
    }

    #[inline]
    fn zwyx(self) -> U64Vec4 {
        U64Vec4::new(self.z, self.w, self.y, self.x)
    }

    #[inline]
    fn zwyy(self) -> U64Vec4 {
        U64Vec4::new(self.z, self.w, self.y, self.y)
    }

    #[inline]
    fn zwyz(self) -> U64Vec4 {
        U64Vec4::new(self.z, self.w, self.y, self.z)
    }

    #[inline]
    fn zwyw(self) -> U64Vec4 {
        U64Vec4::new(self.z, self.w, self.y, self.w)
    }

    #[inline]
    fn zwzx(self) -> U64Vec4 {
        U64Vec4::new(self.z, self.w, self.z, self.x)
    }

    #[inline]
    fn zwzy(self) -> U64Vec4 {
        U64Vec4::new(self.z, self.w, self.z, self.y)
    }

    #[inline]
    fn zwzz(self) -> U64Vec4 {
        U64Vec4::new(self.z, self.w, self.z, self.z)
    }

    #[inline]
    fn zwzw(self) -> U64Vec4 {
        U64Vec4::new(self.z, self.w, self.z, self.w)
    }

    #[inline]
    fn zwwx(self) -> U64Vec4 {
        U64Vec4::new(self.z, self.w, self.w, self.x)
    }

    #[inline]
    fn zwwy(self) -> U64Vec4 {
        U64Vec4::new(self.z, self.w, self.w, self.y)
    }

    #[inline]
    fn zwwz(self) -> U64Vec4 {
        U64Vec4::new(self.z, self.w, self.w, self.z)
    }

    #[inline]
    fn zwww(self) -> U64Vec4 {
        U64Vec4::new(self.z, self.w, self.w, self.w)
    }

    #[inline]
    fn wxxx(self) -> U64Vec4 {
        U64Vec4::new(self.w, self.x, self.x, self.x)
    }

    #[inline]
    fn wxxy(self) -> U64Vec4 {
        U64Vec4::new(self.w, self.x, self.x, self.y)
    }

    #[inline]
    fn wxxz(self) -> U64Vec4 {
        U64Vec4::new(self.w, self.x, self.x, self.z)
    }

    #[inline]
    fn wxxw(self) -> U64Vec4 {
        U64Vec4::new(self.w, self.x, self.x, self.w)
    }

    #[inline]
    fn wxyx(self) -> U64Vec4 {
        U64Vec4::new(self.w, self.x, self.y, self.x)
    }

    #[inline]
    fn wxyy(self) -> U64Vec4 {
        U64Vec4::new(self.w, self.x, self.y, self.y)
    }

    #[inline]
    fn wxyz(self) -> U64Vec4 {
        U64Vec4::new(self.w, self.x, self.y, self.z)
    }

    #[inline]
    fn wxyw(self) -> U64Vec4 {
        U64Vec4::new(self.w, self.x, self.y, self.w)
    }

    #[inline]
    fn wxzx(self) -> U64Vec4 {
        U64Vec4::new(self.w, self.x, self.z, self.x)
    }

    #[inline]
    fn wxzy(self) -> U64Vec4 {
        U64Vec4::new(self.w, self.x, self.z, self.y)
    }

    #[inline]
    fn wxzz(self) -> U64Vec4 {
        U64Vec4::new(self.w, self.x, self.z, self.z)
    }

    #[inline]
    fn wxzw(self) -> U64Vec4 {
        U64Vec4::new(self.w, self.x, self.z, self.w)
    }

    #[inline]
    fn wxwx(self) -> U64Vec4 {
        U64Vec4::new(self.w, self.x, self.w, self.x)
    }

    #[inline]
    fn wxwy(self) -> U64Vec4 {
        U64Vec4::new(self.w, self.x, self.w, self.y)
    }

    #[inline]
    fn wxwz(self) -> U64Vec4 {
        U64Vec4::new(self.w, self.x, self.w, self.z)
    }

    #[inline]
    fn wxww(self) -> U64Vec4 {
        U64Vec4::new(self.w, self.x, self.w, self.w)
    }

    #[inline]
    fn wyxx(self) -> U64Vec4 {
        U64Vec4::new(self.w, self.y, self.x, self.x)
    }

    #[inline]
    fn wyxy(self) -> U64Vec4 {
        U64Vec4::new(self.w, self.y, self.x, self.y)
    }

    #[inline]
    fn wyxz(self) -> U64Vec4 {
        U64Vec4::new(self.w, self.y, self.x, self.z)
    }

    #[inline]
    fn wyxw(self) -> U64Vec4 {
        U64Vec4::new(self.w, self.y, self.x, self.w)
    }

    #[inline]
    fn wyyx(self) -> U64Vec4 {
        U64Vec4::new(self.w, self.y, self.y, self.x)
    }

    #[inline]
    fn wyyy(self) -> U64Vec4 {
        U64Vec4::new(self.w, self.y, self.y, self.y)
    }

    #[inline]
    fn wyyz(self) -> U64Vec4 {
        U64Vec4::new(self.w, self.y, self.y, self.z)
    }

    #[inline]
    fn wyyw(self) -> U64Vec4 {
        U64Vec4::new(self.w, self.y, self.y, self.w)
    }

    #[inline]
    fn wyzx(self) -> U64Vec4 {
        U64Vec4::new(self.w, self.y, self.z, self.x)
    }

    #[inline]
    fn wyzy(self) -> U64Vec4 {
        U64Vec4::new(self.w, self.y, self.z, self.y)
    }

    #[inline]
    fn wyzz(self) -> U64Vec4 {
        U64Vec4::new(self.w, self.y, self.z, self.z)
    }

    #[inline]
    fn wyzw(self) -> U64Vec4 {
        U64Vec4::new(self.w, self.y, self.z, self.w)
    }

    #[inline]
    fn wywx(self) -> U64Vec4 {
        U64Vec4::new(self.w, self.y, self.w, self.x)
    }

    #[inline]
    fn wywy(self) -> U64Vec4 {
        U64Vec4::new(self.w, self.y, self.w, self.y)
    }

    #[inline]
    fn wywz(self) -> U64Vec4 {
        U64Vec4::new(self.w, self.y, self.w, self.z)
    }

    #[inline]
    fn wyww(self) -> U64Vec4 {
        U64Vec4::new(self.w, self.y, self.w, self.w)
    }

    #[inline]
    fn wzxx(self) -> U64Vec4 {
        U64Vec4::new(self.w, self.z, self.x, self.x)
    }

    #[inline]
    fn wzxy(self) -> U64Vec4 {
        U64Vec4::new(self.w, self.z, self.x, self.y)
    }

    #[inline]
    fn wzxz(self) -> U64Vec4 {
        U64Vec4::new(self.w, self.z, self.x, self.z)
    }

    #[inline]
    fn wzxw(self) -> U64Vec4 {
        U64Vec4::new(self.w, self.z, self.x, self.w)
    }

    #[inline]
    fn wzyx(self) -> U64Vec4 {
        U64Vec4::new(self.w, self.z, self.y, self.x)
    }

    #[inline]
    fn wzyy(self) -> U64Vec4 {
        U64Vec4::new(self.w, self.z, self.y, self.y)
    }

    #[inline]
    fn wzyz(self) -> U64Vec4 {
        U64Vec4::new(self.w, self.z, self.y, self.z)
    }

    #[inline]
    fn wzyw(self) -> U64Vec4 {
        U64Vec4::new(self.w, self.z, self.y, self.w)
    }

    #[inline]
    fn wzzx(self) -> U64Vec4 {
        U64Vec4::new(self.w, self.z, self.z, self.x)
    }

    #[inline]
    fn wzzy(self) -> U64Vec4 {
        U64Vec4::new(self.w, self.z, self.z, self.y)
    }

    #[inline]
    fn wzzz(self) -> U64Vec4 {
        U64Vec4::new(self.w, self.z, self.z, self.z)
    }

    #[inline]
    fn wzzw(self) -> U64Vec4 {
        U64Vec4::new(self.w, self.z, self.z, self.w)
    }

    #[inline]
    fn wzwx(self) -> U64Vec4 {
        U64Vec4::new(self.w, self.z, self.w, self.x)
    }

    #[inline]
    fn wzwy(self) -> U64Vec4 {
        U64Vec4::new(self.w, self.z, self.w, self.y)
    }

    #[inline]
    fn wzwz(self) -> U64Vec4 {
        U64Vec4::new(self.w, self.z, self.w, self.z)
    }

    #[inline]
    fn wzww(self) -> U64Vec4 {
        U64Vec4::new(self.w, self.z, self.w, self.w)
    }

    #[inline]
    fn wwxx(self) -> U64Vec4 {
        U64Vec4::new(self.w, self.w, self.x, self.x)
    }

    #[inline]
    fn wwxy(self) -> U64Vec4 {
        U64Vec4::new(self.w, self.w, self.x, self.y)
    }

    #[inline]
    fn wwxz(self) -> U64Vec4 {
        U64Vec4::new(self.w, self.w, self.x, self.z)
    }

    #[inline]
    fn wwxw(self) -> U64Vec4 {
        U64Vec4::new(self.w, self.w, self.x, self.w)
    }

    #[inline]
    fn wwyx(self) -> U64Vec4 {
        U64Vec4::new(self.w, self.w, self.y, self.x)
    }

    #[inline]
    fn wwyy(self) -> U64Vec4 {
        U64Vec4::new(self.w, self.w, self.y, self.y)
    }

    #[inline]
    fn wwyz(self) -> U64Vec4 {
        U64Vec4::new(self.w, self.w, self.y, self.z)
    }

    #[inline]
    fn wwyw(self) -> U64Vec4 {
        U64Vec4::new(self.w, self.w, self.y, self.w)
    }

    #[inline]
    fn wwzx(self) -> U64Vec4 {
        U64Vec4::new(self.w, self.w, self.z, self.x)
    }

    #[inline]
    fn wwzy(self) -> U64Vec4 {
        U64Vec4::new(self.w, self.w, self.z, self.y)
    }

    #[inline]
    fn wwzz(self) -> U64Vec4 {
        U64Vec4::new(self.w, self.w, self.z, self.z)
    }

    #[inline]
    fn wwzw(self) -> U64Vec4 {
        U64Vec4::new(self.w, self.w, self.z, self.w)
    }

    #[inline]
    fn wwwx(self) -> U64Vec4 {
        U64Vec4::new(self.w, self.w, self.w, self.x)
    }

    #[inline]
    fn wwwy(self) -> U64Vec4 {
        U64Vec4::new(self.w, self.w, self.w, self.y)
    }

    #[inline]
    fn wwwz(self) -> U64Vec4 {
        U64Vec4::new(self.w, self.w, self.w, self.z)
    }

    #[inline]
    fn wwww(self) -> U64Vec4 {
        U64Vec4::new(self.w, self.w, self.w, self.w)
    }
}
