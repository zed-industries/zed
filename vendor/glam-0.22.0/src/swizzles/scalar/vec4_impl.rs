// Generated from swizzle_impl.rs.tera template. Edit the template, not the generated file.

use crate::{Vec2, Vec3, Vec4, Vec4Swizzles};

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
        Vec4::new(self.x, self.x, self.x, self.x)
    }

    #[inline]
    fn xxxy(self) -> Vec4 {
        Vec4::new(self.x, self.x, self.x, self.y)
    }

    #[inline]
    fn xxxz(self) -> Vec4 {
        Vec4::new(self.x, self.x, self.x, self.z)
    }

    #[inline]
    fn xxxw(self) -> Vec4 {
        Vec4::new(self.x, self.x, self.x, self.w)
    }

    #[inline]
    fn xxyx(self) -> Vec4 {
        Vec4::new(self.x, self.x, self.y, self.x)
    }

    #[inline]
    fn xxyy(self) -> Vec4 {
        Vec4::new(self.x, self.x, self.y, self.y)
    }

    #[inline]
    fn xxyz(self) -> Vec4 {
        Vec4::new(self.x, self.x, self.y, self.z)
    }

    #[inline]
    fn xxyw(self) -> Vec4 {
        Vec4::new(self.x, self.x, self.y, self.w)
    }

    #[inline]
    fn xxzx(self) -> Vec4 {
        Vec4::new(self.x, self.x, self.z, self.x)
    }

    #[inline]
    fn xxzy(self) -> Vec4 {
        Vec4::new(self.x, self.x, self.z, self.y)
    }

    #[inline]
    fn xxzz(self) -> Vec4 {
        Vec4::new(self.x, self.x, self.z, self.z)
    }

    #[inline]
    fn xxzw(self) -> Vec4 {
        Vec4::new(self.x, self.x, self.z, self.w)
    }

    #[inline]
    fn xxwx(self) -> Vec4 {
        Vec4::new(self.x, self.x, self.w, self.x)
    }

    #[inline]
    fn xxwy(self) -> Vec4 {
        Vec4::new(self.x, self.x, self.w, self.y)
    }

    #[inline]
    fn xxwz(self) -> Vec4 {
        Vec4::new(self.x, self.x, self.w, self.z)
    }

    #[inline]
    fn xxww(self) -> Vec4 {
        Vec4::new(self.x, self.x, self.w, self.w)
    }

    #[inline]
    fn xyxx(self) -> Vec4 {
        Vec4::new(self.x, self.y, self.x, self.x)
    }

    #[inline]
    fn xyxy(self) -> Vec4 {
        Vec4::new(self.x, self.y, self.x, self.y)
    }

    #[inline]
    fn xyxz(self) -> Vec4 {
        Vec4::new(self.x, self.y, self.x, self.z)
    }

    #[inline]
    fn xyxw(self) -> Vec4 {
        Vec4::new(self.x, self.y, self.x, self.w)
    }

    #[inline]
    fn xyyx(self) -> Vec4 {
        Vec4::new(self.x, self.y, self.y, self.x)
    }

    #[inline]
    fn xyyy(self) -> Vec4 {
        Vec4::new(self.x, self.y, self.y, self.y)
    }

    #[inline]
    fn xyyz(self) -> Vec4 {
        Vec4::new(self.x, self.y, self.y, self.z)
    }

    #[inline]
    fn xyyw(self) -> Vec4 {
        Vec4::new(self.x, self.y, self.y, self.w)
    }

    #[inline]
    fn xyzx(self) -> Vec4 {
        Vec4::new(self.x, self.y, self.z, self.x)
    }

    #[inline]
    fn xyzy(self) -> Vec4 {
        Vec4::new(self.x, self.y, self.z, self.y)
    }

    #[inline]
    fn xyzz(self) -> Vec4 {
        Vec4::new(self.x, self.y, self.z, self.z)
    }

    #[inline]
    fn xyzw(self) -> Vec4 {
        Vec4::new(self.x, self.y, self.z, self.w)
    }

    #[inline]
    fn xywx(self) -> Vec4 {
        Vec4::new(self.x, self.y, self.w, self.x)
    }

    #[inline]
    fn xywy(self) -> Vec4 {
        Vec4::new(self.x, self.y, self.w, self.y)
    }

    #[inline]
    fn xywz(self) -> Vec4 {
        Vec4::new(self.x, self.y, self.w, self.z)
    }

    #[inline]
    fn xyww(self) -> Vec4 {
        Vec4::new(self.x, self.y, self.w, self.w)
    }

    #[inline]
    fn xzxx(self) -> Vec4 {
        Vec4::new(self.x, self.z, self.x, self.x)
    }

    #[inline]
    fn xzxy(self) -> Vec4 {
        Vec4::new(self.x, self.z, self.x, self.y)
    }

    #[inline]
    fn xzxz(self) -> Vec4 {
        Vec4::new(self.x, self.z, self.x, self.z)
    }

    #[inline]
    fn xzxw(self) -> Vec4 {
        Vec4::new(self.x, self.z, self.x, self.w)
    }

    #[inline]
    fn xzyx(self) -> Vec4 {
        Vec4::new(self.x, self.z, self.y, self.x)
    }

    #[inline]
    fn xzyy(self) -> Vec4 {
        Vec4::new(self.x, self.z, self.y, self.y)
    }

    #[inline]
    fn xzyz(self) -> Vec4 {
        Vec4::new(self.x, self.z, self.y, self.z)
    }

    #[inline]
    fn xzyw(self) -> Vec4 {
        Vec4::new(self.x, self.z, self.y, self.w)
    }

    #[inline]
    fn xzzx(self) -> Vec4 {
        Vec4::new(self.x, self.z, self.z, self.x)
    }

    #[inline]
    fn xzzy(self) -> Vec4 {
        Vec4::new(self.x, self.z, self.z, self.y)
    }

    #[inline]
    fn xzzz(self) -> Vec4 {
        Vec4::new(self.x, self.z, self.z, self.z)
    }

    #[inline]
    fn xzzw(self) -> Vec4 {
        Vec4::new(self.x, self.z, self.z, self.w)
    }

    #[inline]
    fn xzwx(self) -> Vec4 {
        Vec4::new(self.x, self.z, self.w, self.x)
    }

    #[inline]
    fn xzwy(self) -> Vec4 {
        Vec4::new(self.x, self.z, self.w, self.y)
    }

    #[inline]
    fn xzwz(self) -> Vec4 {
        Vec4::new(self.x, self.z, self.w, self.z)
    }

    #[inline]
    fn xzww(self) -> Vec4 {
        Vec4::new(self.x, self.z, self.w, self.w)
    }

    #[inline]
    fn xwxx(self) -> Vec4 {
        Vec4::new(self.x, self.w, self.x, self.x)
    }

    #[inline]
    fn xwxy(self) -> Vec4 {
        Vec4::new(self.x, self.w, self.x, self.y)
    }

    #[inline]
    fn xwxz(self) -> Vec4 {
        Vec4::new(self.x, self.w, self.x, self.z)
    }

    #[inline]
    fn xwxw(self) -> Vec4 {
        Vec4::new(self.x, self.w, self.x, self.w)
    }

    #[inline]
    fn xwyx(self) -> Vec4 {
        Vec4::new(self.x, self.w, self.y, self.x)
    }

    #[inline]
    fn xwyy(self) -> Vec4 {
        Vec4::new(self.x, self.w, self.y, self.y)
    }

    #[inline]
    fn xwyz(self) -> Vec4 {
        Vec4::new(self.x, self.w, self.y, self.z)
    }

    #[inline]
    fn xwyw(self) -> Vec4 {
        Vec4::new(self.x, self.w, self.y, self.w)
    }

    #[inline]
    fn xwzx(self) -> Vec4 {
        Vec4::new(self.x, self.w, self.z, self.x)
    }

    #[inline]
    fn xwzy(self) -> Vec4 {
        Vec4::new(self.x, self.w, self.z, self.y)
    }

    #[inline]
    fn xwzz(self) -> Vec4 {
        Vec4::new(self.x, self.w, self.z, self.z)
    }

    #[inline]
    fn xwzw(self) -> Vec4 {
        Vec4::new(self.x, self.w, self.z, self.w)
    }

    #[inline]
    fn xwwx(self) -> Vec4 {
        Vec4::new(self.x, self.w, self.w, self.x)
    }

    #[inline]
    fn xwwy(self) -> Vec4 {
        Vec4::new(self.x, self.w, self.w, self.y)
    }

    #[inline]
    fn xwwz(self) -> Vec4 {
        Vec4::new(self.x, self.w, self.w, self.z)
    }

    #[inline]
    fn xwww(self) -> Vec4 {
        Vec4::new(self.x, self.w, self.w, self.w)
    }

    #[inline]
    fn yxxx(self) -> Vec4 {
        Vec4::new(self.y, self.x, self.x, self.x)
    }

    #[inline]
    fn yxxy(self) -> Vec4 {
        Vec4::new(self.y, self.x, self.x, self.y)
    }

    #[inline]
    fn yxxz(self) -> Vec4 {
        Vec4::new(self.y, self.x, self.x, self.z)
    }

    #[inline]
    fn yxxw(self) -> Vec4 {
        Vec4::new(self.y, self.x, self.x, self.w)
    }

    #[inline]
    fn yxyx(self) -> Vec4 {
        Vec4::new(self.y, self.x, self.y, self.x)
    }

    #[inline]
    fn yxyy(self) -> Vec4 {
        Vec4::new(self.y, self.x, self.y, self.y)
    }

    #[inline]
    fn yxyz(self) -> Vec4 {
        Vec4::new(self.y, self.x, self.y, self.z)
    }

    #[inline]
    fn yxyw(self) -> Vec4 {
        Vec4::new(self.y, self.x, self.y, self.w)
    }

    #[inline]
    fn yxzx(self) -> Vec4 {
        Vec4::new(self.y, self.x, self.z, self.x)
    }

    #[inline]
    fn yxzy(self) -> Vec4 {
        Vec4::new(self.y, self.x, self.z, self.y)
    }

    #[inline]
    fn yxzz(self) -> Vec4 {
        Vec4::new(self.y, self.x, self.z, self.z)
    }

    #[inline]
    fn yxzw(self) -> Vec4 {
        Vec4::new(self.y, self.x, self.z, self.w)
    }

    #[inline]
    fn yxwx(self) -> Vec4 {
        Vec4::new(self.y, self.x, self.w, self.x)
    }

    #[inline]
    fn yxwy(self) -> Vec4 {
        Vec4::new(self.y, self.x, self.w, self.y)
    }

    #[inline]
    fn yxwz(self) -> Vec4 {
        Vec4::new(self.y, self.x, self.w, self.z)
    }

    #[inline]
    fn yxww(self) -> Vec4 {
        Vec4::new(self.y, self.x, self.w, self.w)
    }

    #[inline]
    fn yyxx(self) -> Vec4 {
        Vec4::new(self.y, self.y, self.x, self.x)
    }

    #[inline]
    fn yyxy(self) -> Vec4 {
        Vec4::new(self.y, self.y, self.x, self.y)
    }

    #[inline]
    fn yyxz(self) -> Vec4 {
        Vec4::new(self.y, self.y, self.x, self.z)
    }

    #[inline]
    fn yyxw(self) -> Vec4 {
        Vec4::new(self.y, self.y, self.x, self.w)
    }

    #[inline]
    fn yyyx(self) -> Vec4 {
        Vec4::new(self.y, self.y, self.y, self.x)
    }

    #[inline]
    fn yyyy(self) -> Vec4 {
        Vec4::new(self.y, self.y, self.y, self.y)
    }

    #[inline]
    fn yyyz(self) -> Vec4 {
        Vec4::new(self.y, self.y, self.y, self.z)
    }

    #[inline]
    fn yyyw(self) -> Vec4 {
        Vec4::new(self.y, self.y, self.y, self.w)
    }

    #[inline]
    fn yyzx(self) -> Vec4 {
        Vec4::new(self.y, self.y, self.z, self.x)
    }

    #[inline]
    fn yyzy(self) -> Vec4 {
        Vec4::new(self.y, self.y, self.z, self.y)
    }

    #[inline]
    fn yyzz(self) -> Vec4 {
        Vec4::new(self.y, self.y, self.z, self.z)
    }

    #[inline]
    fn yyzw(self) -> Vec4 {
        Vec4::new(self.y, self.y, self.z, self.w)
    }

    #[inline]
    fn yywx(self) -> Vec4 {
        Vec4::new(self.y, self.y, self.w, self.x)
    }

    #[inline]
    fn yywy(self) -> Vec4 {
        Vec4::new(self.y, self.y, self.w, self.y)
    }

    #[inline]
    fn yywz(self) -> Vec4 {
        Vec4::new(self.y, self.y, self.w, self.z)
    }

    #[inline]
    fn yyww(self) -> Vec4 {
        Vec4::new(self.y, self.y, self.w, self.w)
    }

    #[inline]
    fn yzxx(self) -> Vec4 {
        Vec4::new(self.y, self.z, self.x, self.x)
    }

    #[inline]
    fn yzxy(self) -> Vec4 {
        Vec4::new(self.y, self.z, self.x, self.y)
    }

    #[inline]
    fn yzxz(self) -> Vec4 {
        Vec4::new(self.y, self.z, self.x, self.z)
    }

    #[inline]
    fn yzxw(self) -> Vec4 {
        Vec4::new(self.y, self.z, self.x, self.w)
    }

    #[inline]
    fn yzyx(self) -> Vec4 {
        Vec4::new(self.y, self.z, self.y, self.x)
    }

    #[inline]
    fn yzyy(self) -> Vec4 {
        Vec4::new(self.y, self.z, self.y, self.y)
    }

    #[inline]
    fn yzyz(self) -> Vec4 {
        Vec4::new(self.y, self.z, self.y, self.z)
    }

    #[inline]
    fn yzyw(self) -> Vec4 {
        Vec4::new(self.y, self.z, self.y, self.w)
    }

    #[inline]
    fn yzzx(self) -> Vec4 {
        Vec4::new(self.y, self.z, self.z, self.x)
    }

    #[inline]
    fn yzzy(self) -> Vec4 {
        Vec4::new(self.y, self.z, self.z, self.y)
    }

    #[inline]
    fn yzzz(self) -> Vec4 {
        Vec4::new(self.y, self.z, self.z, self.z)
    }

    #[inline]
    fn yzzw(self) -> Vec4 {
        Vec4::new(self.y, self.z, self.z, self.w)
    }

    #[inline]
    fn yzwx(self) -> Vec4 {
        Vec4::new(self.y, self.z, self.w, self.x)
    }

    #[inline]
    fn yzwy(self) -> Vec4 {
        Vec4::new(self.y, self.z, self.w, self.y)
    }

    #[inline]
    fn yzwz(self) -> Vec4 {
        Vec4::new(self.y, self.z, self.w, self.z)
    }

    #[inline]
    fn yzww(self) -> Vec4 {
        Vec4::new(self.y, self.z, self.w, self.w)
    }

    #[inline]
    fn ywxx(self) -> Vec4 {
        Vec4::new(self.y, self.w, self.x, self.x)
    }

    #[inline]
    fn ywxy(self) -> Vec4 {
        Vec4::new(self.y, self.w, self.x, self.y)
    }

    #[inline]
    fn ywxz(self) -> Vec4 {
        Vec4::new(self.y, self.w, self.x, self.z)
    }

    #[inline]
    fn ywxw(self) -> Vec4 {
        Vec4::new(self.y, self.w, self.x, self.w)
    }

    #[inline]
    fn ywyx(self) -> Vec4 {
        Vec4::new(self.y, self.w, self.y, self.x)
    }

    #[inline]
    fn ywyy(self) -> Vec4 {
        Vec4::new(self.y, self.w, self.y, self.y)
    }

    #[inline]
    fn ywyz(self) -> Vec4 {
        Vec4::new(self.y, self.w, self.y, self.z)
    }

    #[inline]
    fn ywyw(self) -> Vec4 {
        Vec4::new(self.y, self.w, self.y, self.w)
    }

    #[inline]
    fn ywzx(self) -> Vec4 {
        Vec4::new(self.y, self.w, self.z, self.x)
    }

    #[inline]
    fn ywzy(self) -> Vec4 {
        Vec4::new(self.y, self.w, self.z, self.y)
    }

    #[inline]
    fn ywzz(self) -> Vec4 {
        Vec4::new(self.y, self.w, self.z, self.z)
    }

    #[inline]
    fn ywzw(self) -> Vec4 {
        Vec4::new(self.y, self.w, self.z, self.w)
    }

    #[inline]
    fn ywwx(self) -> Vec4 {
        Vec4::new(self.y, self.w, self.w, self.x)
    }

    #[inline]
    fn ywwy(self) -> Vec4 {
        Vec4::new(self.y, self.w, self.w, self.y)
    }

    #[inline]
    fn ywwz(self) -> Vec4 {
        Vec4::new(self.y, self.w, self.w, self.z)
    }

    #[inline]
    fn ywww(self) -> Vec4 {
        Vec4::new(self.y, self.w, self.w, self.w)
    }

    #[inline]
    fn zxxx(self) -> Vec4 {
        Vec4::new(self.z, self.x, self.x, self.x)
    }

    #[inline]
    fn zxxy(self) -> Vec4 {
        Vec4::new(self.z, self.x, self.x, self.y)
    }

    #[inline]
    fn zxxz(self) -> Vec4 {
        Vec4::new(self.z, self.x, self.x, self.z)
    }

    #[inline]
    fn zxxw(self) -> Vec4 {
        Vec4::new(self.z, self.x, self.x, self.w)
    }

    #[inline]
    fn zxyx(self) -> Vec4 {
        Vec4::new(self.z, self.x, self.y, self.x)
    }

    #[inline]
    fn zxyy(self) -> Vec4 {
        Vec4::new(self.z, self.x, self.y, self.y)
    }

    #[inline]
    fn zxyz(self) -> Vec4 {
        Vec4::new(self.z, self.x, self.y, self.z)
    }

    #[inline]
    fn zxyw(self) -> Vec4 {
        Vec4::new(self.z, self.x, self.y, self.w)
    }

    #[inline]
    fn zxzx(self) -> Vec4 {
        Vec4::new(self.z, self.x, self.z, self.x)
    }

    #[inline]
    fn zxzy(self) -> Vec4 {
        Vec4::new(self.z, self.x, self.z, self.y)
    }

    #[inline]
    fn zxzz(self) -> Vec4 {
        Vec4::new(self.z, self.x, self.z, self.z)
    }

    #[inline]
    fn zxzw(self) -> Vec4 {
        Vec4::new(self.z, self.x, self.z, self.w)
    }

    #[inline]
    fn zxwx(self) -> Vec4 {
        Vec4::new(self.z, self.x, self.w, self.x)
    }

    #[inline]
    fn zxwy(self) -> Vec4 {
        Vec4::new(self.z, self.x, self.w, self.y)
    }

    #[inline]
    fn zxwz(self) -> Vec4 {
        Vec4::new(self.z, self.x, self.w, self.z)
    }

    #[inline]
    fn zxww(self) -> Vec4 {
        Vec4::new(self.z, self.x, self.w, self.w)
    }

    #[inline]
    fn zyxx(self) -> Vec4 {
        Vec4::new(self.z, self.y, self.x, self.x)
    }

    #[inline]
    fn zyxy(self) -> Vec4 {
        Vec4::new(self.z, self.y, self.x, self.y)
    }

    #[inline]
    fn zyxz(self) -> Vec4 {
        Vec4::new(self.z, self.y, self.x, self.z)
    }

    #[inline]
    fn zyxw(self) -> Vec4 {
        Vec4::new(self.z, self.y, self.x, self.w)
    }

    #[inline]
    fn zyyx(self) -> Vec4 {
        Vec4::new(self.z, self.y, self.y, self.x)
    }

    #[inline]
    fn zyyy(self) -> Vec4 {
        Vec4::new(self.z, self.y, self.y, self.y)
    }

    #[inline]
    fn zyyz(self) -> Vec4 {
        Vec4::new(self.z, self.y, self.y, self.z)
    }

    #[inline]
    fn zyyw(self) -> Vec4 {
        Vec4::new(self.z, self.y, self.y, self.w)
    }

    #[inline]
    fn zyzx(self) -> Vec4 {
        Vec4::new(self.z, self.y, self.z, self.x)
    }

    #[inline]
    fn zyzy(self) -> Vec4 {
        Vec4::new(self.z, self.y, self.z, self.y)
    }

    #[inline]
    fn zyzz(self) -> Vec4 {
        Vec4::new(self.z, self.y, self.z, self.z)
    }

    #[inline]
    fn zyzw(self) -> Vec4 {
        Vec4::new(self.z, self.y, self.z, self.w)
    }

    #[inline]
    fn zywx(self) -> Vec4 {
        Vec4::new(self.z, self.y, self.w, self.x)
    }

    #[inline]
    fn zywy(self) -> Vec4 {
        Vec4::new(self.z, self.y, self.w, self.y)
    }

    #[inline]
    fn zywz(self) -> Vec4 {
        Vec4::new(self.z, self.y, self.w, self.z)
    }

    #[inline]
    fn zyww(self) -> Vec4 {
        Vec4::new(self.z, self.y, self.w, self.w)
    }

    #[inline]
    fn zzxx(self) -> Vec4 {
        Vec4::new(self.z, self.z, self.x, self.x)
    }

    #[inline]
    fn zzxy(self) -> Vec4 {
        Vec4::new(self.z, self.z, self.x, self.y)
    }

    #[inline]
    fn zzxz(self) -> Vec4 {
        Vec4::new(self.z, self.z, self.x, self.z)
    }

    #[inline]
    fn zzxw(self) -> Vec4 {
        Vec4::new(self.z, self.z, self.x, self.w)
    }

    #[inline]
    fn zzyx(self) -> Vec4 {
        Vec4::new(self.z, self.z, self.y, self.x)
    }

    #[inline]
    fn zzyy(self) -> Vec4 {
        Vec4::new(self.z, self.z, self.y, self.y)
    }

    #[inline]
    fn zzyz(self) -> Vec4 {
        Vec4::new(self.z, self.z, self.y, self.z)
    }

    #[inline]
    fn zzyw(self) -> Vec4 {
        Vec4::new(self.z, self.z, self.y, self.w)
    }

    #[inline]
    fn zzzx(self) -> Vec4 {
        Vec4::new(self.z, self.z, self.z, self.x)
    }

    #[inline]
    fn zzzy(self) -> Vec4 {
        Vec4::new(self.z, self.z, self.z, self.y)
    }

    #[inline]
    fn zzzz(self) -> Vec4 {
        Vec4::new(self.z, self.z, self.z, self.z)
    }

    #[inline]
    fn zzzw(self) -> Vec4 {
        Vec4::new(self.z, self.z, self.z, self.w)
    }

    #[inline]
    fn zzwx(self) -> Vec4 {
        Vec4::new(self.z, self.z, self.w, self.x)
    }

    #[inline]
    fn zzwy(self) -> Vec4 {
        Vec4::new(self.z, self.z, self.w, self.y)
    }

    #[inline]
    fn zzwz(self) -> Vec4 {
        Vec4::new(self.z, self.z, self.w, self.z)
    }

    #[inline]
    fn zzww(self) -> Vec4 {
        Vec4::new(self.z, self.z, self.w, self.w)
    }

    #[inline]
    fn zwxx(self) -> Vec4 {
        Vec4::new(self.z, self.w, self.x, self.x)
    }

    #[inline]
    fn zwxy(self) -> Vec4 {
        Vec4::new(self.z, self.w, self.x, self.y)
    }

    #[inline]
    fn zwxz(self) -> Vec4 {
        Vec4::new(self.z, self.w, self.x, self.z)
    }

    #[inline]
    fn zwxw(self) -> Vec4 {
        Vec4::new(self.z, self.w, self.x, self.w)
    }

    #[inline]
    fn zwyx(self) -> Vec4 {
        Vec4::new(self.z, self.w, self.y, self.x)
    }

    #[inline]
    fn zwyy(self) -> Vec4 {
        Vec4::new(self.z, self.w, self.y, self.y)
    }

    #[inline]
    fn zwyz(self) -> Vec4 {
        Vec4::new(self.z, self.w, self.y, self.z)
    }

    #[inline]
    fn zwyw(self) -> Vec4 {
        Vec4::new(self.z, self.w, self.y, self.w)
    }

    #[inline]
    fn zwzx(self) -> Vec4 {
        Vec4::new(self.z, self.w, self.z, self.x)
    }

    #[inline]
    fn zwzy(self) -> Vec4 {
        Vec4::new(self.z, self.w, self.z, self.y)
    }

    #[inline]
    fn zwzz(self) -> Vec4 {
        Vec4::new(self.z, self.w, self.z, self.z)
    }

    #[inline]
    fn zwzw(self) -> Vec4 {
        Vec4::new(self.z, self.w, self.z, self.w)
    }

    #[inline]
    fn zwwx(self) -> Vec4 {
        Vec4::new(self.z, self.w, self.w, self.x)
    }

    #[inline]
    fn zwwy(self) -> Vec4 {
        Vec4::new(self.z, self.w, self.w, self.y)
    }

    #[inline]
    fn zwwz(self) -> Vec4 {
        Vec4::new(self.z, self.w, self.w, self.z)
    }

    #[inline]
    fn zwww(self) -> Vec4 {
        Vec4::new(self.z, self.w, self.w, self.w)
    }

    #[inline]
    fn wxxx(self) -> Vec4 {
        Vec4::new(self.w, self.x, self.x, self.x)
    }

    #[inline]
    fn wxxy(self) -> Vec4 {
        Vec4::new(self.w, self.x, self.x, self.y)
    }

    #[inline]
    fn wxxz(self) -> Vec4 {
        Vec4::new(self.w, self.x, self.x, self.z)
    }

    #[inline]
    fn wxxw(self) -> Vec4 {
        Vec4::new(self.w, self.x, self.x, self.w)
    }

    #[inline]
    fn wxyx(self) -> Vec4 {
        Vec4::new(self.w, self.x, self.y, self.x)
    }

    #[inline]
    fn wxyy(self) -> Vec4 {
        Vec4::new(self.w, self.x, self.y, self.y)
    }

    #[inline]
    fn wxyz(self) -> Vec4 {
        Vec4::new(self.w, self.x, self.y, self.z)
    }

    #[inline]
    fn wxyw(self) -> Vec4 {
        Vec4::new(self.w, self.x, self.y, self.w)
    }

    #[inline]
    fn wxzx(self) -> Vec4 {
        Vec4::new(self.w, self.x, self.z, self.x)
    }

    #[inline]
    fn wxzy(self) -> Vec4 {
        Vec4::new(self.w, self.x, self.z, self.y)
    }

    #[inline]
    fn wxzz(self) -> Vec4 {
        Vec4::new(self.w, self.x, self.z, self.z)
    }

    #[inline]
    fn wxzw(self) -> Vec4 {
        Vec4::new(self.w, self.x, self.z, self.w)
    }

    #[inline]
    fn wxwx(self) -> Vec4 {
        Vec4::new(self.w, self.x, self.w, self.x)
    }

    #[inline]
    fn wxwy(self) -> Vec4 {
        Vec4::new(self.w, self.x, self.w, self.y)
    }

    #[inline]
    fn wxwz(self) -> Vec4 {
        Vec4::new(self.w, self.x, self.w, self.z)
    }

    #[inline]
    fn wxww(self) -> Vec4 {
        Vec4::new(self.w, self.x, self.w, self.w)
    }

    #[inline]
    fn wyxx(self) -> Vec4 {
        Vec4::new(self.w, self.y, self.x, self.x)
    }

    #[inline]
    fn wyxy(self) -> Vec4 {
        Vec4::new(self.w, self.y, self.x, self.y)
    }

    #[inline]
    fn wyxz(self) -> Vec4 {
        Vec4::new(self.w, self.y, self.x, self.z)
    }

    #[inline]
    fn wyxw(self) -> Vec4 {
        Vec4::new(self.w, self.y, self.x, self.w)
    }

    #[inline]
    fn wyyx(self) -> Vec4 {
        Vec4::new(self.w, self.y, self.y, self.x)
    }

    #[inline]
    fn wyyy(self) -> Vec4 {
        Vec4::new(self.w, self.y, self.y, self.y)
    }

    #[inline]
    fn wyyz(self) -> Vec4 {
        Vec4::new(self.w, self.y, self.y, self.z)
    }

    #[inline]
    fn wyyw(self) -> Vec4 {
        Vec4::new(self.w, self.y, self.y, self.w)
    }

    #[inline]
    fn wyzx(self) -> Vec4 {
        Vec4::new(self.w, self.y, self.z, self.x)
    }

    #[inline]
    fn wyzy(self) -> Vec4 {
        Vec4::new(self.w, self.y, self.z, self.y)
    }

    #[inline]
    fn wyzz(self) -> Vec4 {
        Vec4::new(self.w, self.y, self.z, self.z)
    }

    #[inline]
    fn wyzw(self) -> Vec4 {
        Vec4::new(self.w, self.y, self.z, self.w)
    }

    #[inline]
    fn wywx(self) -> Vec4 {
        Vec4::new(self.w, self.y, self.w, self.x)
    }

    #[inline]
    fn wywy(self) -> Vec4 {
        Vec4::new(self.w, self.y, self.w, self.y)
    }

    #[inline]
    fn wywz(self) -> Vec4 {
        Vec4::new(self.w, self.y, self.w, self.z)
    }

    #[inline]
    fn wyww(self) -> Vec4 {
        Vec4::new(self.w, self.y, self.w, self.w)
    }

    #[inline]
    fn wzxx(self) -> Vec4 {
        Vec4::new(self.w, self.z, self.x, self.x)
    }

    #[inline]
    fn wzxy(self) -> Vec4 {
        Vec4::new(self.w, self.z, self.x, self.y)
    }

    #[inline]
    fn wzxz(self) -> Vec4 {
        Vec4::new(self.w, self.z, self.x, self.z)
    }

    #[inline]
    fn wzxw(self) -> Vec4 {
        Vec4::new(self.w, self.z, self.x, self.w)
    }

    #[inline]
    fn wzyx(self) -> Vec4 {
        Vec4::new(self.w, self.z, self.y, self.x)
    }

    #[inline]
    fn wzyy(self) -> Vec4 {
        Vec4::new(self.w, self.z, self.y, self.y)
    }

    #[inline]
    fn wzyz(self) -> Vec4 {
        Vec4::new(self.w, self.z, self.y, self.z)
    }

    #[inline]
    fn wzyw(self) -> Vec4 {
        Vec4::new(self.w, self.z, self.y, self.w)
    }

    #[inline]
    fn wzzx(self) -> Vec4 {
        Vec4::new(self.w, self.z, self.z, self.x)
    }

    #[inline]
    fn wzzy(self) -> Vec4 {
        Vec4::new(self.w, self.z, self.z, self.y)
    }

    #[inline]
    fn wzzz(self) -> Vec4 {
        Vec4::new(self.w, self.z, self.z, self.z)
    }

    #[inline]
    fn wzzw(self) -> Vec4 {
        Vec4::new(self.w, self.z, self.z, self.w)
    }

    #[inline]
    fn wzwx(self) -> Vec4 {
        Vec4::new(self.w, self.z, self.w, self.x)
    }

    #[inline]
    fn wzwy(self) -> Vec4 {
        Vec4::new(self.w, self.z, self.w, self.y)
    }

    #[inline]
    fn wzwz(self) -> Vec4 {
        Vec4::new(self.w, self.z, self.w, self.z)
    }

    #[inline]
    fn wzww(self) -> Vec4 {
        Vec4::new(self.w, self.z, self.w, self.w)
    }

    #[inline]
    fn wwxx(self) -> Vec4 {
        Vec4::new(self.w, self.w, self.x, self.x)
    }

    #[inline]
    fn wwxy(self) -> Vec4 {
        Vec4::new(self.w, self.w, self.x, self.y)
    }

    #[inline]
    fn wwxz(self) -> Vec4 {
        Vec4::new(self.w, self.w, self.x, self.z)
    }

    #[inline]
    fn wwxw(self) -> Vec4 {
        Vec4::new(self.w, self.w, self.x, self.w)
    }

    #[inline]
    fn wwyx(self) -> Vec4 {
        Vec4::new(self.w, self.w, self.y, self.x)
    }

    #[inline]
    fn wwyy(self) -> Vec4 {
        Vec4::new(self.w, self.w, self.y, self.y)
    }

    #[inline]
    fn wwyz(self) -> Vec4 {
        Vec4::new(self.w, self.w, self.y, self.z)
    }

    #[inline]
    fn wwyw(self) -> Vec4 {
        Vec4::new(self.w, self.w, self.y, self.w)
    }

    #[inline]
    fn wwzx(self) -> Vec4 {
        Vec4::new(self.w, self.w, self.z, self.x)
    }

    #[inline]
    fn wwzy(self) -> Vec4 {
        Vec4::new(self.w, self.w, self.z, self.y)
    }

    #[inline]
    fn wwzz(self) -> Vec4 {
        Vec4::new(self.w, self.w, self.z, self.z)
    }

    #[inline]
    fn wwzw(self) -> Vec4 {
        Vec4::new(self.w, self.w, self.z, self.w)
    }

    #[inline]
    fn wwwx(self) -> Vec4 {
        Vec4::new(self.w, self.w, self.w, self.x)
    }

    #[inline]
    fn wwwy(self) -> Vec4 {
        Vec4::new(self.w, self.w, self.w, self.y)
    }

    #[inline]
    fn wwwz(self) -> Vec4 {
        Vec4::new(self.w, self.w, self.w, self.z)
    }

    #[inline]
    fn wwww(self) -> Vec4 {
        Vec4::new(self.w, self.w, self.w, self.w)
    }
}
