// Generated from swizzle_impl.rs.tera template. Edit the template, not the generated file.

use crate::{I64Vec2, I64Vec3, I64Vec4, Vec4Swizzles};

impl Vec4Swizzles for I64Vec4 {
    type Vec2 = I64Vec2;

    type Vec3 = I64Vec3;

    #[inline]
    fn xx(self) -> I64Vec2 {
        I64Vec2 {
            x: self.x,
            y: self.x,
        }
    }

    #[inline]
    fn xy(self) -> I64Vec2 {
        I64Vec2 {
            x: self.x,
            y: self.y,
        }
    }

    #[inline]
    fn xz(self) -> I64Vec2 {
        I64Vec2 {
            x: self.x,
            y: self.z,
        }
    }

    #[inline]
    fn xw(self) -> I64Vec2 {
        I64Vec2 {
            x: self.x,
            y: self.w,
        }
    }

    #[inline]
    fn yx(self) -> I64Vec2 {
        I64Vec2 {
            x: self.y,
            y: self.x,
        }
    }

    #[inline]
    fn yy(self) -> I64Vec2 {
        I64Vec2 {
            x: self.y,
            y: self.y,
        }
    }

    #[inline]
    fn yz(self) -> I64Vec2 {
        I64Vec2 {
            x: self.y,
            y: self.z,
        }
    }

    #[inline]
    fn yw(self) -> I64Vec2 {
        I64Vec2 {
            x: self.y,
            y: self.w,
        }
    }

    #[inline]
    fn zx(self) -> I64Vec2 {
        I64Vec2 {
            x: self.z,
            y: self.x,
        }
    }

    #[inline]
    fn zy(self) -> I64Vec2 {
        I64Vec2 {
            x: self.z,
            y: self.y,
        }
    }

    #[inline]
    fn zz(self) -> I64Vec2 {
        I64Vec2 {
            x: self.z,
            y: self.z,
        }
    }

    #[inline]
    fn zw(self) -> I64Vec2 {
        I64Vec2 {
            x: self.z,
            y: self.w,
        }
    }

    #[inline]
    fn wx(self) -> I64Vec2 {
        I64Vec2 {
            x: self.w,
            y: self.x,
        }
    }

    #[inline]
    fn wy(self) -> I64Vec2 {
        I64Vec2 {
            x: self.w,
            y: self.y,
        }
    }

    #[inline]
    fn wz(self) -> I64Vec2 {
        I64Vec2 {
            x: self.w,
            y: self.z,
        }
    }

    #[inline]
    fn ww(self) -> I64Vec2 {
        I64Vec2 {
            x: self.w,
            y: self.w,
        }
    }

    #[inline]
    fn xxx(self) -> I64Vec3 {
        I64Vec3 {
            x: self.x,
            y: self.x,
            z: self.x,
        }
    }

    #[inline]
    fn xxy(self) -> I64Vec3 {
        I64Vec3 {
            x: self.x,
            y: self.x,
            z: self.y,
        }
    }

    #[inline]
    fn xxz(self) -> I64Vec3 {
        I64Vec3 {
            x: self.x,
            y: self.x,
            z: self.z,
        }
    }

    #[inline]
    fn xxw(self) -> I64Vec3 {
        I64Vec3 {
            x: self.x,
            y: self.x,
            z: self.w,
        }
    }

    #[inline]
    fn xyx(self) -> I64Vec3 {
        I64Vec3 {
            x: self.x,
            y: self.y,
            z: self.x,
        }
    }

    #[inline]
    fn xyy(self) -> I64Vec3 {
        I64Vec3 {
            x: self.x,
            y: self.y,
            z: self.y,
        }
    }

    #[inline]
    fn xyz(self) -> I64Vec3 {
        I64Vec3 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }

    #[inline]
    fn xyw(self) -> I64Vec3 {
        I64Vec3 {
            x: self.x,
            y: self.y,
            z: self.w,
        }
    }

    #[inline]
    fn xzx(self) -> I64Vec3 {
        I64Vec3 {
            x: self.x,
            y: self.z,
            z: self.x,
        }
    }

    #[inline]
    fn xzy(self) -> I64Vec3 {
        I64Vec3 {
            x: self.x,
            y: self.z,
            z: self.y,
        }
    }

    #[inline]
    fn xzz(self) -> I64Vec3 {
        I64Vec3 {
            x: self.x,
            y: self.z,
            z: self.z,
        }
    }

    #[inline]
    fn xzw(self) -> I64Vec3 {
        I64Vec3 {
            x: self.x,
            y: self.z,
            z: self.w,
        }
    }

    #[inline]
    fn xwx(self) -> I64Vec3 {
        I64Vec3 {
            x: self.x,
            y: self.w,
            z: self.x,
        }
    }

    #[inline]
    fn xwy(self) -> I64Vec3 {
        I64Vec3 {
            x: self.x,
            y: self.w,
            z: self.y,
        }
    }

    #[inline]
    fn xwz(self) -> I64Vec3 {
        I64Vec3 {
            x: self.x,
            y: self.w,
            z: self.z,
        }
    }

    #[inline]
    fn xww(self) -> I64Vec3 {
        I64Vec3 {
            x: self.x,
            y: self.w,
            z: self.w,
        }
    }

    #[inline]
    fn yxx(self) -> I64Vec3 {
        I64Vec3 {
            x: self.y,
            y: self.x,
            z: self.x,
        }
    }

    #[inline]
    fn yxy(self) -> I64Vec3 {
        I64Vec3 {
            x: self.y,
            y: self.x,
            z: self.y,
        }
    }

    #[inline]
    fn yxz(self) -> I64Vec3 {
        I64Vec3 {
            x: self.y,
            y: self.x,
            z: self.z,
        }
    }

    #[inline]
    fn yxw(self) -> I64Vec3 {
        I64Vec3 {
            x: self.y,
            y: self.x,
            z: self.w,
        }
    }

    #[inline]
    fn yyx(self) -> I64Vec3 {
        I64Vec3 {
            x: self.y,
            y: self.y,
            z: self.x,
        }
    }

    #[inline]
    fn yyy(self) -> I64Vec3 {
        I64Vec3 {
            x: self.y,
            y: self.y,
            z: self.y,
        }
    }

    #[inline]
    fn yyz(self) -> I64Vec3 {
        I64Vec3 {
            x: self.y,
            y: self.y,
            z: self.z,
        }
    }

    #[inline]
    fn yyw(self) -> I64Vec3 {
        I64Vec3 {
            x: self.y,
            y: self.y,
            z: self.w,
        }
    }

    #[inline]
    fn yzx(self) -> I64Vec3 {
        I64Vec3 {
            x: self.y,
            y: self.z,
            z: self.x,
        }
    }

    #[inline]
    fn yzy(self) -> I64Vec3 {
        I64Vec3 {
            x: self.y,
            y: self.z,
            z: self.y,
        }
    }

    #[inline]
    fn yzz(self) -> I64Vec3 {
        I64Vec3 {
            x: self.y,
            y: self.z,
            z: self.z,
        }
    }

    #[inline]
    fn yzw(self) -> I64Vec3 {
        I64Vec3 {
            x: self.y,
            y: self.z,
            z: self.w,
        }
    }

    #[inline]
    fn ywx(self) -> I64Vec3 {
        I64Vec3 {
            x: self.y,
            y: self.w,
            z: self.x,
        }
    }

    #[inline]
    fn ywy(self) -> I64Vec3 {
        I64Vec3 {
            x: self.y,
            y: self.w,
            z: self.y,
        }
    }

    #[inline]
    fn ywz(self) -> I64Vec3 {
        I64Vec3 {
            x: self.y,
            y: self.w,
            z: self.z,
        }
    }

    #[inline]
    fn yww(self) -> I64Vec3 {
        I64Vec3 {
            x: self.y,
            y: self.w,
            z: self.w,
        }
    }

    #[inline]
    fn zxx(self) -> I64Vec3 {
        I64Vec3 {
            x: self.z,
            y: self.x,
            z: self.x,
        }
    }

    #[inline]
    fn zxy(self) -> I64Vec3 {
        I64Vec3 {
            x: self.z,
            y: self.x,
            z: self.y,
        }
    }

    #[inline]
    fn zxz(self) -> I64Vec3 {
        I64Vec3 {
            x: self.z,
            y: self.x,
            z: self.z,
        }
    }

    #[inline]
    fn zxw(self) -> I64Vec3 {
        I64Vec3 {
            x: self.z,
            y: self.x,
            z: self.w,
        }
    }

    #[inline]
    fn zyx(self) -> I64Vec3 {
        I64Vec3 {
            x: self.z,
            y: self.y,
            z: self.x,
        }
    }

    #[inline]
    fn zyy(self) -> I64Vec3 {
        I64Vec3 {
            x: self.z,
            y: self.y,
            z: self.y,
        }
    }

    #[inline]
    fn zyz(self) -> I64Vec3 {
        I64Vec3 {
            x: self.z,
            y: self.y,
            z: self.z,
        }
    }

    #[inline]
    fn zyw(self) -> I64Vec3 {
        I64Vec3 {
            x: self.z,
            y: self.y,
            z: self.w,
        }
    }

    #[inline]
    fn zzx(self) -> I64Vec3 {
        I64Vec3 {
            x: self.z,
            y: self.z,
            z: self.x,
        }
    }

    #[inline]
    fn zzy(self) -> I64Vec3 {
        I64Vec3 {
            x: self.z,
            y: self.z,
            z: self.y,
        }
    }

    #[inline]
    fn zzz(self) -> I64Vec3 {
        I64Vec3 {
            x: self.z,
            y: self.z,
            z: self.z,
        }
    }

    #[inline]
    fn zzw(self) -> I64Vec3 {
        I64Vec3 {
            x: self.z,
            y: self.z,
            z: self.w,
        }
    }

    #[inline]
    fn zwx(self) -> I64Vec3 {
        I64Vec3 {
            x: self.z,
            y: self.w,
            z: self.x,
        }
    }

    #[inline]
    fn zwy(self) -> I64Vec3 {
        I64Vec3 {
            x: self.z,
            y: self.w,
            z: self.y,
        }
    }

    #[inline]
    fn zwz(self) -> I64Vec3 {
        I64Vec3 {
            x: self.z,
            y: self.w,
            z: self.z,
        }
    }

    #[inline]
    fn zww(self) -> I64Vec3 {
        I64Vec3 {
            x: self.z,
            y: self.w,
            z: self.w,
        }
    }

    #[inline]
    fn wxx(self) -> I64Vec3 {
        I64Vec3 {
            x: self.w,
            y: self.x,
            z: self.x,
        }
    }

    #[inline]
    fn wxy(self) -> I64Vec3 {
        I64Vec3 {
            x: self.w,
            y: self.x,
            z: self.y,
        }
    }

    #[inline]
    fn wxz(self) -> I64Vec3 {
        I64Vec3 {
            x: self.w,
            y: self.x,
            z: self.z,
        }
    }

    #[inline]
    fn wxw(self) -> I64Vec3 {
        I64Vec3 {
            x: self.w,
            y: self.x,
            z: self.w,
        }
    }

    #[inline]
    fn wyx(self) -> I64Vec3 {
        I64Vec3 {
            x: self.w,
            y: self.y,
            z: self.x,
        }
    }

    #[inline]
    fn wyy(self) -> I64Vec3 {
        I64Vec3 {
            x: self.w,
            y: self.y,
            z: self.y,
        }
    }

    #[inline]
    fn wyz(self) -> I64Vec3 {
        I64Vec3 {
            x: self.w,
            y: self.y,
            z: self.z,
        }
    }

    #[inline]
    fn wyw(self) -> I64Vec3 {
        I64Vec3 {
            x: self.w,
            y: self.y,
            z: self.w,
        }
    }

    #[inline]
    fn wzx(self) -> I64Vec3 {
        I64Vec3 {
            x: self.w,
            y: self.z,
            z: self.x,
        }
    }

    #[inline]
    fn wzy(self) -> I64Vec3 {
        I64Vec3 {
            x: self.w,
            y: self.z,
            z: self.y,
        }
    }

    #[inline]
    fn wzz(self) -> I64Vec3 {
        I64Vec3 {
            x: self.w,
            y: self.z,
            z: self.z,
        }
    }

    #[inline]
    fn wzw(self) -> I64Vec3 {
        I64Vec3 {
            x: self.w,
            y: self.z,
            z: self.w,
        }
    }

    #[inline]
    fn wwx(self) -> I64Vec3 {
        I64Vec3 {
            x: self.w,
            y: self.w,
            z: self.x,
        }
    }

    #[inline]
    fn wwy(self) -> I64Vec3 {
        I64Vec3 {
            x: self.w,
            y: self.w,
            z: self.y,
        }
    }

    #[inline]
    fn wwz(self) -> I64Vec3 {
        I64Vec3 {
            x: self.w,
            y: self.w,
            z: self.z,
        }
    }

    #[inline]
    fn www(self) -> I64Vec3 {
        I64Vec3 {
            x: self.w,
            y: self.w,
            z: self.w,
        }
    }

    #[inline]
    fn xxxx(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.x, self.x, self.x)
    }

    #[inline]
    fn xxxy(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.x, self.x, self.y)
    }

    #[inline]
    fn xxxz(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.x, self.x, self.z)
    }

    #[inline]
    fn xxxw(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.x, self.x, self.w)
    }

    #[inline]
    fn xxyx(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.x, self.y, self.x)
    }

    #[inline]
    fn xxyy(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.x, self.y, self.y)
    }

    #[inline]
    fn xxyz(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.x, self.y, self.z)
    }

    #[inline]
    fn xxyw(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.x, self.y, self.w)
    }

    #[inline]
    fn xxzx(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.x, self.z, self.x)
    }

    #[inline]
    fn xxzy(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.x, self.z, self.y)
    }

    #[inline]
    fn xxzz(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.x, self.z, self.z)
    }

    #[inline]
    fn xxzw(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.x, self.z, self.w)
    }

    #[inline]
    fn xxwx(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.x, self.w, self.x)
    }

    #[inline]
    fn xxwy(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.x, self.w, self.y)
    }

    #[inline]
    fn xxwz(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.x, self.w, self.z)
    }

    #[inline]
    fn xxww(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.x, self.w, self.w)
    }

    #[inline]
    fn xyxx(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.y, self.x, self.x)
    }

    #[inline]
    fn xyxy(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.y, self.x, self.y)
    }

    #[inline]
    fn xyxz(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.y, self.x, self.z)
    }

    #[inline]
    fn xyxw(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.y, self.x, self.w)
    }

    #[inline]
    fn xyyx(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.y, self.y, self.x)
    }

    #[inline]
    fn xyyy(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.y, self.y, self.y)
    }

    #[inline]
    fn xyyz(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.y, self.y, self.z)
    }

    #[inline]
    fn xyyw(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.y, self.y, self.w)
    }

    #[inline]
    fn xyzx(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.y, self.z, self.x)
    }

    #[inline]
    fn xyzy(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.y, self.z, self.y)
    }

    #[inline]
    fn xyzz(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.y, self.z, self.z)
    }

    #[inline]
    fn xyzw(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.y, self.z, self.w)
    }

    #[inline]
    fn xywx(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.y, self.w, self.x)
    }

    #[inline]
    fn xywy(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.y, self.w, self.y)
    }

    #[inline]
    fn xywz(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.y, self.w, self.z)
    }

    #[inline]
    fn xyww(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.y, self.w, self.w)
    }

    #[inline]
    fn xzxx(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.z, self.x, self.x)
    }

    #[inline]
    fn xzxy(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.z, self.x, self.y)
    }

    #[inline]
    fn xzxz(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.z, self.x, self.z)
    }

    #[inline]
    fn xzxw(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.z, self.x, self.w)
    }

    #[inline]
    fn xzyx(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.z, self.y, self.x)
    }

    #[inline]
    fn xzyy(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.z, self.y, self.y)
    }

    #[inline]
    fn xzyz(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.z, self.y, self.z)
    }

    #[inline]
    fn xzyw(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.z, self.y, self.w)
    }

    #[inline]
    fn xzzx(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.z, self.z, self.x)
    }

    #[inline]
    fn xzzy(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.z, self.z, self.y)
    }

    #[inline]
    fn xzzz(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.z, self.z, self.z)
    }

    #[inline]
    fn xzzw(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.z, self.z, self.w)
    }

    #[inline]
    fn xzwx(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.z, self.w, self.x)
    }

    #[inline]
    fn xzwy(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.z, self.w, self.y)
    }

    #[inline]
    fn xzwz(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.z, self.w, self.z)
    }

    #[inline]
    fn xzww(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.z, self.w, self.w)
    }

    #[inline]
    fn xwxx(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.w, self.x, self.x)
    }

    #[inline]
    fn xwxy(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.w, self.x, self.y)
    }

    #[inline]
    fn xwxz(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.w, self.x, self.z)
    }

    #[inline]
    fn xwxw(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.w, self.x, self.w)
    }

    #[inline]
    fn xwyx(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.w, self.y, self.x)
    }

    #[inline]
    fn xwyy(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.w, self.y, self.y)
    }

    #[inline]
    fn xwyz(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.w, self.y, self.z)
    }

    #[inline]
    fn xwyw(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.w, self.y, self.w)
    }

    #[inline]
    fn xwzx(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.w, self.z, self.x)
    }

    #[inline]
    fn xwzy(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.w, self.z, self.y)
    }

    #[inline]
    fn xwzz(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.w, self.z, self.z)
    }

    #[inline]
    fn xwzw(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.w, self.z, self.w)
    }

    #[inline]
    fn xwwx(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.w, self.w, self.x)
    }

    #[inline]
    fn xwwy(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.w, self.w, self.y)
    }

    #[inline]
    fn xwwz(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.w, self.w, self.z)
    }

    #[inline]
    fn xwww(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.w, self.w, self.w)
    }

    #[inline]
    fn yxxx(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.x, self.x, self.x)
    }

    #[inline]
    fn yxxy(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.x, self.x, self.y)
    }

    #[inline]
    fn yxxz(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.x, self.x, self.z)
    }

    #[inline]
    fn yxxw(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.x, self.x, self.w)
    }

    #[inline]
    fn yxyx(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.x, self.y, self.x)
    }

    #[inline]
    fn yxyy(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.x, self.y, self.y)
    }

    #[inline]
    fn yxyz(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.x, self.y, self.z)
    }

    #[inline]
    fn yxyw(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.x, self.y, self.w)
    }

    #[inline]
    fn yxzx(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.x, self.z, self.x)
    }

    #[inline]
    fn yxzy(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.x, self.z, self.y)
    }

    #[inline]
    fn yxzz(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.x, self.z, self.z)
    }

    #[inline]
    fn yxzw(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.x, self.z, self.w)
    }

    #[inline]
    fn yxwx(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.x, self.w, self.x)
    }

    #[inline]
    fn yxwy(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.x, self.w, self.y)
    }

    #[inline]
    fn yxwz(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.x, self.w, self.z)
    }

    #[inline]
    fn yxww(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.x, self.w, self.w)
    }

    #[inline]
    fn yyxx(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.y, self.x, self.x)
    }

    #[inline]
    fn yyxy(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.y, self.x, self.y)
    }

    #[inline]
    fn yyxz(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.y, self.x, self.z)
    }

    #[inline]
    fn yyxw(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.y, self.x, self.w)
    }

    #[inline]
    fn yyyx(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.y, self.y, self.x)
    }

    #[inline]
    fn yyyy(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.y, self.y, self.y)
    }

    #[inline]
    fn yyyz(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.y, self.y, self.z)
    }

    #[inline]
    fn yyyw(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.y, self.y, self.w)
    }

    #[inline]
    fn yyzx(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.y, self.z, self.x)
    }

    #[inline]
    fn yyzy(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.y, self.z, self.y)
    }

    #[inline]
    fn yyzz(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.y, self.z, self.z)
    }

    #[inline]
    fn yyzw(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.y, self.z, self.w)
    }

    #[inline]
    fn yywx(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.y, self.w, self.x)
    }

    #[inline]
    fn yywy(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.y, self.w, self.y)
    }

    #[inline]
    fn yywz(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.y, self.w, self.z)
    }

    #[inline]
    fn yyww(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.y, self.w, self.w)
    }

    #[inline]
    fn yzxx(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.z, self.x, self.x)
    }

    #[inline]
    fn yzxy(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.z, self.x, self.y)
    }

    #[inline]
    fn yzxz(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.z, self.x, self.z)
    }

    #[inline]
    fn yzxw(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.z, self.x, self.w)
    }

    #[inline]
    fn yzyx(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.z, self.y, self.x)
    }

    #[inline]
    fn yzyy(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.z, self.y, self.y)
    }

    #[inline]
    fn yzyz(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.z, self.y, self.z)
    }

    #[inline]
    fn yzyw(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.z, self.y, self.w)
    }

    #[inline]
    fn yzzx(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.z, self.z, self.x)
    }

    #[inline]
    fn yzzy(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.z, self.z, self.y)
    }

    #[inline]
    fn yzzz(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.z, self.z, self.z)
    }

    #[inline]
    fn yzzw(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.z, self.z, self.w)
    }

    #[inline]
    fn yzwx(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.z, self.w, self.x)
    }

    #[inline]
    fn yzwy(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.z, self.w, self.y)
    }

    #[inline]
    fn yzwz(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.z, self.w, self.z)
    }

    #[inline]
    fn yzww(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.z, self.w, self.w)
    }

    #[inline]
    fn ywxx(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.w, self.x, self.x)
    }

    #[inline]
    fn ywxy(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.w, self.x, self.y)
    }

    #[inline]
    fn ywxz(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.w, self.x, self.z)
    }

    #[inline]
    fn ywxw(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.w, self.x, self.w)
    }

    #[inline]
    fn ywyx(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.w, self.y, self.x)
    }

    #[inline]
    fn ywyy(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.w, self.y, self.y)
    }

    #[inline]
    fn ywyz(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.w, self.y, self.z)
    }

    #[inline]
    fn ywyw(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.w, self.y, self.w)
    }

    #[inline]
    fn ywzx(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.w, self.z, self.x)
    }

    #[inline]
    fn ywzy(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.w, self.z, self.y)
    }

    #[inline]
    fn ywzz(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.w, self.z, self.z)
    }

    #[inline]
    fn ywzw(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.w, self.z, self.w)
    }

    #[inline]
    fn ywwx(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.w, self.w, self.x)
    }

    #[inline]
    fn ywwy(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.w, self.w, self.y)
    }

    #[inline]
    fn ywwz(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.w, self.w, self.z)
    }

    #[inline]
    fn ywww(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.w, self.w, self.w)
    }

    #[inline]
    fn zxxx(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.x, self.x, self.x)
    }

    #[inline]
    fn zxxy(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.x, self.x, self.y)
    }

    #[inline]
    fn zxxz(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.x, self.x, self.z)
    }

    #[inline]
    fn zxxw(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.x, self.x, self.w)
    }

    #[inline]
    fn zxyx(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.x, self.y, self.x)
    }

    #[inline]
    fn zxyy(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.x, self.y, self.y)
    }

    #[inline]
    fn zxyz(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.x, self.y, self.z)
    }

    #[inline]
    fn zxyw(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.x, self.y, self.w)
    }

    #[inline]
    fn zxzx(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.x, self.z, self.x)
    }

    #[inline]
    fn zxzy(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.x, self.z, self.y)
    }

    #[inline]
    fn zxzz(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.x, self.z, self.z)
    }

    #[inline]
    fn zxzw(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.x, self.z, self.w)
    }

    #[inline]
    fn zxwx(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.x, self.w, self.x)
    }

    #[inline]
    fn zxwy(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.x, self.w, self.y)
    }

    #[inline]
    fn zxwz(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.x, self.w, self.z)
    }

    #[inline]
    fn zxww(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.x, self.w, self.w)
    }

    #[inline]
    fn zyxx(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.y, self.x, self.x)
    }

    #[inline]
    fn zyxy(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.y, self.x, self.y)
    }

    #[inline]
    fn zyxz(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.y, self.x, self.z)
    }

    #[inline]
    fn zyxw(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.y, self.x, self.w)
    }

    #[inline]
    fn zyyx(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.y, self.y, self.x)
    }

    #[inline]
    fn zyyy(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.y, self.y, self.y)
    }

    #[inline]
    fn zyyz(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.y, self.y, self.z)
    }

    #[inline]
    fn zyyw(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.y, self.y, self.w)
    }

    #[inline]
    fn zyzx(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.y, self.z, self.x)
    }

    #[inline]
    fn zyzy(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.y, self.z, self.y)
    }

    #[inline]
    fn zyzz(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.y, self.z, self.z)
    }

    #[inline]
    fn zyzw(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.y, self.z, self.w)
    }

    #[inline]
    fn zywx(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.y, self.w, self.x)
    }

    #[inline]
    fn zywy(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.y, self.w, self.y)
    }

    #[inline]
    fn zywz(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.y, self.w, self.z)
    }

    #[inline]
    fn zyww(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.y, self.w, self.w)
    }

    #[inline]
    fn zzxx(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.z, self.x, self.x)
    }

    #[inline]
    fn zzxy(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.z, self.x, self.y)
    }

    #[inline]
    fn zzxz(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.z, self.x, self.z)
    }

    #[inline]
    fn zzxw(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.z, self.x, self.w)
    }

    #[inline]
    fn zzyx(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.z, self.y, self.x)
    }

    #[inline]
    fn zzyy(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.z, self.y, self.y)
    }

    #[inline]
    fn zzyz(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.z, self.y, self.z)
    }

    #[inline]
    fn zzyw(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.z, self.y, self.w)
    }

    #[inline]
    fn zzzx(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.z, self.z, self.x)
    }

    #[inline]
    fn zzzy(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.z, self.z, self.y)
    }

    #[inline]
    fn zzzz(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.z, self.z, self.z)
    }

    #[inline]
    fn zzzw(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.z, self.z, self.w)
    }

    #[inline]
    fn zzwx(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.z, self.w, self.x)
    }

    #[inline]
    fn zzwy(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.z, self.w, self.y)
    }

    #[inline]
    fn zzwz(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.z, self.w, self.z)
    }

    #[inline]
    fn zzww(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.z, self.w, self.w)
    }

    #[inline]
    fn zwxx(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.w, self.x, self.x)
    }

    #[inline]
    fn zwxy(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.w, self.x, self.y)
    }

    #[inline]
    fn zwxz(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.w, self.x, self.z)
    }

    #[inline]
    fn zwxw(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.w, self.x, self.w)
    }

    #[inline]
    fn zwyx(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.w, self.y, self.x)
    }

    #[inline]
    fn zwyy(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.w, self.y, self.y)
    }

    #[inline]
    fn zwyz(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.w, self.y, self.z)
    }

    #[inline]
    fn zwyw(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.w, self.y, self.w)
    }

    #[inline]
    fn zwzx(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.w, self.z, self.x)
    }

    #[inline]
    fn zwzy(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.w, self.z, self.y)
    }

    #[inline]
    fn zwzz(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.w, self.z, self.z)
    }

    #[inline]
    fn zwzw(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.w, self.z, self.w)
    }

    #[inline]
    fn zwwx(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.w, self.w, self.x)
    }

    #[inline]
    fn zwwy(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.w, self.w, self.y)
    }

    #[inline]
    fn zwwz(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.w, self.w, self.z)
    }

    #[inline]
    fn zwww(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.w, self.w, self.w)
    }

    #[inline]
    fn wxxx(self) -> I64Vec4 {
        I64Vec4::new(self.w, self.x, self.x, self.x)
    }

    #[inline]
    fn wxxy(self) -> I64Vec4 {
        I64Vec4::new(self.w, self.x, self.x, self.y)
    }

    #[inline]
    fn wxxz(self) -> I64Vec4 {
        I64Vec4::new(self.w, self.x, self.x, self.z)
    }

    #[inline]
    fn wxxw(self) -> I64Vec4 {
        I64Vec4::new(self.w, self.x, self.x, self.w)
    }

    #[inline]
    fn wxyx(self) -> I64Vec4 {
        I64Vec4::new(self.w, self.x, self.y, self.x)
    }

    #[inline]
    fn wxyy(self) -> I64Vec4 {
        I64Vec4::new(self.w, self.x, self.y, self.y)
    }

    #[inline]
    fn wxyz(self) -> I64Vec4 {
        I64Vec4::new(self.w, self.x, self.y, self.z)
    }

    #[inline]
    fn wxyw(self) -> I64Vec4 {
        I64Vec4::new(self.w, self.x, self.y, self.w)
    }

    #[inline]
    fn wxzx(self) -> I64Vec4 {
        I64Vec4::new(self.w, self.x, self.z, self.x)
    }

    #[inline]
    fn wxzy(self) -> I64Vec4 {
        I64Vec4::new(self.w, self.x, self.z, self.y)
    }

    #[inline]
    fn wxzz(self) -> I64Vec4 {
        I64Vec4::new(self.w, self.x, self.z, self.z)
    }

    #[inline]
    fn wxzw(self) -> I64Vec4 {
        I64Vec4::new(self.w, self.x, self.z, self.w)
    }

    #[inline]
    fn wxwx(self) -> I64Vec4 {
        I64Vec4::new(self.w, self.x, self.w, self.x)
    }

    #[inline]
    fn wxwy(self) -> I64Vec4 {
        I64Vec4::new(self.w, self.x, self.w, self.y)
    }

    #[inline]
    fn wxwz(self) -> I64Vec4 {
        I64Vec4::new(self.w, self.x, self.w, self.z)
    }

    #[inline]
    fn wxww(self) -> I64Vec4 {
        I64Vec4::new(self.w, self.x, self.w, self.w)
    }

    #[inline]
    fn wyxx(self) -> I64Vec4 {
        I64Vec4::new(self.w, self.y, self.x, self.x)
    }

    #[inline]
    fn wyxy(self) -> I64Vec4 {
        I64Vec4::new(self.w, self.y, self.x, self.y)
    }

    #[inline]
    fn wyxz(self) -> I64Vec4 {
        I64Vec4::new(self.w, self.y, self.x, self.z)
    }

    #[inline]
    fn wyxw(self) -> I64Vec4 {
        I64Vec4::new(self.w, self.y, self.x, self.w)
    }

    #[inline]
    fn wyyx(self) -> I64Vec4 {
        I64Vec4::new(self.w, self.y, self.y, self.x)
    }

    #[inline]
    fn wyyy(self) -> I64Vec4 {
        I64Vec4::new(self.w, self.y, self.y, self.y)
    }

    #[inline]
    fn wyyz(self) -> I64Vec4 {
        I64Vec4::new(self.w, self.y, self.y, self.z)
    }

    #[inline]
    fn wyyw(self) -> I64Vec4 {
        I64Vec4::new(self.w, self.y, self.y, self.w)
    }

    #[inline]
    fn wyzx(self) -> I64Vec4 {
        I64Vec4::new(self.w, self.y, self.z, self.x)
    }

    #[inline]
    fn wyzy(self) -> I64Vec4 {
        I64Vec4::new(self.w, self.y, self.z, self.y)
    }

    #[inline]
    fn wyzz(self) -> I64Vec4 {
        I64Vec4::new(self.w, self.y, self.z, self.z)
    }

    #[inline]
    fn wyzw(self) -> I64Vec4 {
        I64Vec4::new(self.w, self.y, self.z, self.w)
    }

    #[inline]
    fn wywx(self) -> I64Vec4 {
        I64Vec4::new(self.w, self.y, self.w, self.x)
    }

    #[inline]
    fn wywy(self) -> I64Vec4 {
        I64Vec4::new(self.w, self.y, self.w, self.y)
    }

    #[inline]
    fn wywz(self) -> I64Vec4 {
        I64Vec4::new(self.w, self.y, self.w, self.z)
    }

    #[inline]
    fn wyww(self) -> I64Vec4 {
        I64Vec4::new(self.w, self.y, self.w, self.w)
    }

    #[inline]
    fn wzxx(self) -> I64Vec4 {
        I64Vec4::new(self.w, self.z, self.x, self.x)
    }

    #[inline]
    fn wzxy(self) -> I64Vec4 {
        I64Vec4::new(self.w, self.z, self.x, self.y)
    }

    #[inline]
    fn wzxz(self) -> I64Vec4 {
        I64Vec4::new(self.w, self.z, self.x, self.z)
    }

    #[inline]
    fn wzxw(self) -> I64Vec4 {
        I64Vec4::new(self.w, self.z, self.x, self.w)
    }

    #[inline]
    fn wzyx(self) -> I64Vec4 {
        I64Vec4::new(self.w, self.z, self.y, self.x)
    }

    #[inline]
    fn wzyy(self) -> I64Vec4 {
        I64Vec4::new(self.w, self.z, self.y, self.y)
    }

    #[inline]
    fn wzyz(self) -> I64Vec4 {
        I64Vec4::new(self.w, self.z, self.y, self.z)
    }

    #[inline]
    fn wzyw(self) -> I64Vec4 {
        I64Vec4::new(self.w, self.z, self.y, self.w)
    }

    #[inline]
    fn wzzx(self) -> I64Vec4 {
        I64Vec4::new(self.w, self.z, self.z, self.x)
    }

    #[inline]
    fn wzzy(self) -> I64Vec4 {
        I64Vec4::new(self.w, self.z, self.z, self.y)
    }

    #[inline]
    fn wzzz(self) -> I64Vec4 {
        I64Vec4::new(self.w, self.z, self.z, self.z)
    }

    #[inline]
    fn wzzw(self) -> I64Vec4 {
        I64Vec4::new(self.w, self.z, self.z, self.w)
    }

    #[inline]
    fn wzwx(self) -> I64Vec4 {
        I64Vec4::new(self.w, self.z, self.w, self.x)
    }

    #[inline]
    fn wzwy(self) -> I64Vec4 {
        I64Vec4::new(self.w, self.z, self.w, self.y)
    }

    #[inline]
    fn wzwz(self) -> I64Vec4 {
        I64Vec4::new(self.w, self.z, self.w, self.z)
    }

    #[inline]
    fn wzww(self) -> I64Vec4 {
        I64Vec4::new(self.w, self.z, self.w, self.w)
    }

    #[inline]
    fn wwxx(self) -> I64Vec4 {
        I64Vec4::new(self.w, self.w, self.x, self.x)
    }

    #[inline]
    fn wwxy(self) -> I64Vec4 {
        I64Vec4::new(self.w, self.w, self.x, self.y)
    }

    #[inline]
    fn wwxz(self) -> I64Vec4 {
        I64Vec4::new(self.w, self.w, self.x, self.z)
    }

    #[inline]
    fn wwxw(self) -> I64Vec4 {
        I64Vec4::new(self.w, self.w, self.x, self.w)
    }

    #[inline]
    fn wwyx(self) -> I64Vec4 {
        I64Vec4::new(self.w, self.w, self.y, self.x)
    }

    #[inline]
    fn wwyy(self) -> I64Vec4 {
        I64Vec4::new(self.w, self.w, self.y, self.y)
    }

    #[inline]
    fn wwyz(self) -> I64Vec4 {
        I64Vec4::new(self.w, self.w, self.y, self.z)
    }

    #[inline]
    fn wwyw(self) -> I64Vec4 {
        I64Vec4::new(self.w, self.w, self.y, self.w)
    }

    #[inline]
    fn wwzx(self) -> I64Vec4 {
        I64Vec4::new(self.w, self.w, self.z, self.x)
    }

    #[inline]
    fn wwzy(self) -> I64Vec4 {
        I64Vec4::new(self.w, self.w, self.z, self.y)
    }

    #[inline]
    fn wwzz(self) -> I64Vec4 {
        I64Vec4::new(self.w, self.w, self.z, self.z)
    }

    #[inline]
    fn wwzw(self) -> I64Vec4 {
        I64Vec4::new(self.w, self.w, self.z, self.w)
    }

    #[inline]
    fn wwwx(self) -> I64Vec4 {
        I64Vec4::new(self.w, self.w, self.w, self.x)
    }

    #[inline]
    fn wwwy(self) -> I64Vec4 {
        I64Vec4::new(self.w, self.w, self.w, self.y)
    }

    #[inline]
    fn wwwz(self) -> I64Vec4 {
        I64Vec4::new(self.w, self.w, self.w, self.z)
    }

    #[inline]
    fn wwww(self) -> I64Vec4 {
        I64Vec4::new(self.w, self.w, self.w, self.w)
    }
}
