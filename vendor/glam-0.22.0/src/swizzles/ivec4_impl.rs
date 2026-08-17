// Generated from swizzle_impl.rs.tera template. Edit the template, not the generated file.

use crate::{IVec2, IVec3, IVec4, Vec4Swizzles};

impl Vec4Swizzles for IVec4 {
    type Vec2 = IVec2;

    type Vec3 = IVec3;

    #[inline]
    fn xx(self) -> IVec2 {
        IVec2 {
            x: self.x,
            y: self.x,
        }
    }

    #[inline]
    fn xy(self) -> IVec2 {
        IVec2 {
            x: self.x,
            y: self.y,
        }
    }

    #[inline]
    fn xz(self) -> IVec2 {
        IVec2 {
            x: self.x,
            y: self.z,
        }
    }

    #[inline]
    fn xw(self) -> IVec2 {
        IVec2 {
            x: self.x,
            y: self.w,
        }
    }

    #[inline]
    fn yx(self) -> IVec2 {
        IVec2 {
            x: self.y,
            y: self.x,
        }
    }

    #[inline]
    fn yy(self) -> IVec2 {
        IVec2 {
            x: self.y,
            y: self.y,
        }
    }

    #[inline]
    fn yz(self) -> IVec2 {
        IVec2 {
            x: self.y,
            y: self.z,
        }
    }

    #[inline]
    fn yw(self) -> IVec2 {
        IVec2 {
            x: self.y,
            y: self.w,
        }
    }

    #[inline]
    fn zx(self) -> IVec2 {
        IVec2 {
            x: self.z,
            y: self.x,
        }
    }

    #[inline]
    fn zy(self) -> IVec2 {
        IVec2 {
            x: self.z,
            y: self.y,
        }
    }

    #[inline]
    fn zz(self) -> IVec2 {
        IVec2 {
            x: self.z,
            y: self.z,
        }
    }

    #[inline]
    fn zw(self) -> IVec2 {
        IVec2 {
            x: self.z,
            y: self.w,
        }
    }

    #[inline]
    fn wx(self) -> IVec2 {
        IVec2 {
            x: self.w,
            y: self.x,
        }
    }

    #[inline]
    fn wy(self) -> IVec2 {
        IVec2 {
            x: self.w,
            y: self.y,
        }
    }

    #[inline]
    fn wz(self) -> IVec2 {
        IVec2 {
            x: self.w,
            y: self.z,
        }
    }

    #[inline]
    fn ww(self) -> IVec2 {
        IVec2 {
            x: self.w,
            y: self.w,
        }
    }

    #[inline]
    fn xxx(self) -> IVec3 {
        IVec3 {
            x: self.x,
            y: self.x,
            z: self.x,
        }
    }

    #[inline]
    fn xxy(self) -> IVec3 {
        IVec3 {
            x: self.x,
            y: self.x,
            z: self.y,
        }
    }

    #[inline]
    fn xxz(self) -> IVec3 {
        IVec3 {
            x: self.x,
            y: self.x,
            z: self.z,
        }
    }

    #[inline]
    fn xxw(self) -> IVec3 {
        IVec3 {
            x: self.x,
            y: self.x,
            z: self.w,
        }
    }

    #[inline]
    fn xyx(self) -> IVec3 {
        IVec3 {
            x: self.x,
            y: self.y,
            z: self.x,
        }
    }

    #[inline]
    fn xyy(self) -> IVec3 {
        IVec3 {
            x: self.x,
            y: self.y,
            z: self.y,
        }
    }

    #[inline]
    fn xyz(self) -> IVec3 {
        IVec3 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }

    #[inline]
    fn xyw(self) -> IVec3 {
        IVec3 {
            x: self.x,
            y: self.y,
            z: self.w,
        }
    }

    #[inline]
    fn xzx(self) -> IVec3 {
        IVec3 {
            x: self.x,
            y: self.z,
            z: self.x,
        }
    }

    #[inline]
    fn xzy(self) -> IVec3 {
        IVec3 {
            x: self.x,
            y: self.z,
            z: self.y,
        }
    }

    #[inline]
    fn xzz(self) -> IVec3 {
        IVec3 {
            x: self.x,
            y: self.z,
            z: self.z,
        }
    }

    #[inline]
    fn xzw(self) -> IVec3 {
        IVec3 {
            x: self.x,
            y: self.z,
            z: self.w,
        }
    }

    #[inline]
    fn xwx(self) -> IVec3 {
        IVec3 {
            x: self.x,
            y: self.w,
            z: self.x,
        }
    }

    #[inline]
    fn xwy(self) -> IVec3 {
        IVec3 {
            x: self.x,
            y: self.w,
            z: self.y,
        }
    }

    #[inline]
    fn xwz(self) -> IVec3 {
        IVec3 {
            x: self.x,
            y: self.w,
            z: self.z,
        }
    }

    #[inline]
    fn xww(self) -> IVec3 {
        IVec3 {
            x: self.x,
            y: self.w,
            z: self.w,
        }
    }

    #[inline]
    fn yxx(self) -> IVec3 {
        IVec3 {
            x: self.y,
            y: self.x,
            z: self.x,
        }
    }

    #[inline]
    fn yxy(self) -> IVec3 {
        IVec3 {
            x: self.y,
            y: self.x,
            z: self.y,
        }
    }

    #[inline]
    fn yxz(self) -> IVec3 {
        IVec3 {
            x: self.y,
            y: self.x,
            z: self.z,
        }
    }

    #[inline]
    fn yxw(self) -> IVec3 {
        IVec3 {
            x: self.y,
            y: self.x,
            z: self.w,
        }
    }

    #[inline]
    fn yyx(self) -> IVec3 {
        IVec3 {
            x: self.y,
            y: self.y,
            z: self.x,
        }
    }

    #[inline]
    fn yyy(self) -> IVec3 {
        IVec3 {
            x: self.y,
            y: self.y,
            z: self.y,
        }
    }

    #[inline]
    fn yyz(self) -> IVec3 {
        IVec3 {
            x: self.y,
            y: self.y,
            z: self.z,
        }
    }

    #[inline]
    fn yyw(self) -> IVec3 {
        IVec3 {
            x: self.y,
            y: self.y,
            z: self.w,
        }
    }

    #[inline]
    fn yzx(self) -> IVec3 {
        IVec3 {
            x: self.y,
            y: self.z,
            z: self.x,
        }
    }

    #[inline]
    fn yzy(self) -> IVec3 {
        IVec3 {
            x: self.y,
            y: self.z,
            z: self.y,
        }
    }

    #[inline]
    fn yzz(self) -> IVec3 {
        IVec3 {
            x: self.y,
            y: self.z,
            z: self.z,
        }
    }

    #[inline]
    fn yzw(self) -> IVec3 {
        IVec3 {
            x: self.y,
            y: self.z,
            z: self.w,
        }
    }

    #[inline]
    fn ywx(self) -> IVec3 {
        IVec3 {
            x: self.y,
            y: self.w,
            z: self.x,
        }
    }

    #[inline]
    fn ywy(self) -> IVec3 {
        IVec3 {
            x: self.y,
            y: self.w,
            z: self.y,
        }
    }

    #[inline]
    fn ywz(self) -> IVec3 {
        IVec3 {
            x: self.y,
            y: self.w,
            z: self.z,
        }
    }

    #[inline]
    fn yww(self) -> IVec3 {
        IVec3 {
            x: self.y,
            y: self.w,
            z: self.w,
        }
    }

    #[inline]
    fn zxx(self) -> IVec3 {
        IVec3 {
            x: self.z,
            y: self.x,
            z: self.x,
        }
    }

    #[inline]
    fn zxy(self) -> IVec3 {
        IVec3 {
            x: self.z,
            y: self.x,
            z: self.y,
        }
    }

    #[inline]
    fn zxz(self) -> IVec3 {
        IVec3 {
            x: self.z,
            y: self.x,
            z: self.z,
        }
    }

    #[inline]
    fn zxw(self) -> IVec3 {
        IVec3 {
            x: self.z,
            y: self.x,
            z: self.w,
        }
    }

    #[inline]
    fn zyx(self) -> IVec3 {
        IVec3 {
            x: self.z,
            y: self.y,
            z: self.x,
        }
    }

    #[inline]
    fn zyy(self) -> IVec3 {
        IVec3 {
            x: self.z,
            y: self.y,
            z: self.y,
        }
    }

    #[inline]
    fn zyz(self) -> IVec3 {
        IVec3 {
            x: self.z,
            y: self.y,
            z: self.z,
        }
    }

    #[inline]
    fn zyw(self) -> IVec3 {
        IVec3 {
            x: self.z,
            y: self.y,
            z: self.w,
        }
    }

    #[inline]
    fn zzx(self) -> IVec3 {
        IVec3 {
            x: self.z,
            y: self.z,
            z: self.x,
        }
    }

    #[inline]
    fn zzy(self) -> IVec3 {
        IVec3 {
            x: self.z,
            y: self.z,
            z: self.y,
        }
    }

    #[inline]
    fn zzz(self) -> IVec3 {
        IVec3 {
            x: self.z,
            y: self.z,
            z: self.z,
        }
    }

    #[inline]
    fn zzw(self) -> IVec3 {
        IVec3 {
            x: self.z,
            y: self.z,
            z: self.w,
        }
    }

    #[inline]
    fn zwx(self) -> IVec3 {
        IVec3 {
            x: self.z,
            y: self.w,
            z: self.x,
        }
    }

    #[inline]
    fn zwy(self) -> IVec3 {
        IVec3 {
            x: self.z,
            y: self.w,
            z: self.y,
        }
    }

    #[inline]
    fn zwz(self) -> IVec3 {
        IVec3 {
            x: self.z,
            y: self.w,
            z: self.z,
        }
    }

    #[inline]
    fn zww(self) -> IVec3 {
        IVec3 {
            x: self.z,
            y: self.w,
            z: self.w,
        }
    }

    #[inline]
    fn wxx(self) -> IVec3 {
        IVec3 {
            x: self.w,
            y: self.x,
            z: self.x,
        }
    }

    #[inline]
    fn wxy(self) -> IVec3 {
        IVec3 {
            x: self.w,
            y: self.x,
            z: self.y,
        }
    }

    #[inline]
    fn wxz(self) -> IVec3 {
        IVec3 {
            x: self.w,
            y: self.x,
            z: self.z,
        }
    }

    #[inline]
    fn wxw(self) -> IVec3 {
        IVec3 {
            x: self.w,
            y: self.x,
            z: self.w,
        }
    }

    #[inline]
    fn wyx(self) -> IVec3 {
        IVec3 {
            x: self.w,
            y: self.y,
            z: self.x,
        }
    }

    #[inline]
    fn wyy(self) -> IVec3 {
        IVec3 {
            x: self.w,
            y: self.y,
            z: self.y,
        }
    }

    #[inline]
    fn wyz(self) -> IVec3 {
        IVec3 {
            x: self.w,
            y: self.y,
            z: self.z,
        }
    }

    #[inline]
    fn wyw(self) -> IVec3 {
        IVec3 {
            x: self.w,
            y: self.y,
            z: self.w,
        }
    }

    #[inline]
    fn wzx(self) -> IVec3 {
        IVec3 {
            x: self.w,
            y: self.z,
            z: self.x,
        }
    }

    #[inline]
    fn wzy(self) -> IVec3 {
        IVec3 {
            x: self.w,
            y: self.z,
            z: self.y,
        }
    }

    #[inline]
    fn wzz(self) -> IVec3 {
        IVec3 {
            x: self.w,
            y: self.z,
            z: self.z,
        }
    }

    #[inline]
    fn wzw(self) -> IVec3 {
        IVec3 {
            x: self.w,
            y: self.z,
            z: self.w,
        }
    }

    #[inline]
    fn wwx(self) -> IVec3 {
        IVec3 {
            x: self.w,
            y: self.w,
            z: self.x,
        }
    }

    #[inline]
    fn wwy(self) -> IVec3 {
        IVec3 {
            x: self.w,
            y: self.w,
            z: self.y,
        }
    }

    #[inline]
    fn wwz(self) -> IVec3 {
        IVec3 {
            x: self.w,
            y: self.w,
            z: self.z,
        }
    }

    #[inline]
    fn www(self) -> IVec3 {
        IVec3 {
            x: self.w,
            y: self.w,
            z: self.w,
        }
    }

    #[inline]
    fn xxxx(self) -> IVec4 {
        IVec4::new(self.x, self.x, self.x, self.x)
    }

    #[inline]
    fn xxxy(self) -> IVec4 {
        IVec4::new(self.x, self.x, self.x, self.y)
    }

    #[inline]
    fn xxxz(self) -> IVec4 {
        IVec4::new(self.x, self.x, self.x, self.z)
    }

    #[inline]
    fn xxxw(self) -> IVec4 {
        IVec4::new(self.x, self.x, self.x, self.w)
    }

    #[inline]
    fn xxyx(self) -> IVec4 {
        IVec4::new(self.x, self.x, self.y, self.x)
    }

    #[inline]
    fn xxyy(self) -> IVec4 {
        IVec4::new(self.x, self.x, self.y, self.y)
    }

    #[inline]
    fn xxyz(self) -> IVec4 {
        IVec4::new(self.x, self.x, self.y, self.z)
    }

    #[inline]
    fn xxyw(self) -> IVec4 {
        IVec4::new(self.x, self.x, self.y, self.w)
    }

    #[inline]
    fn xxzx(self) -> IVec4 {
        IVec4::new(self.x, self.x, self.z, self.x)
    }

    #[inline]
    fn xxzy(self) -> IVec4 {
        IVec4::new(self.x, self.x, self.z, self.y)
    }

    #[inline]
    fn xxzz(self) -> IVec4 {
        IVec4::new(self.x, self.x, self.z, self.z)
    }

    #[inline]
    fn xxzw(self) -> IVec4 {
        IVec4::new(self.x, self.x, self.z, self.w)
    }

    #[inline]
    fn xxwx(self) -> IVec4 {
        IVec4::new(self.x, self.x, self.w, self.x)
    }

    #[inline]
    fn xxwy(self) -> IVec4 {
        IVec4::new(self.x, self.x, self.w, self.y)
    }

    #[inline]
    fn xxwz(self) -> IVec4 {
        IVec4::new(self.x, self.x, self.w, self.z)
    }

    #[inline]
    fn xxww(self) -> IVec4 {
        IVec4::new(self.x, self.x, self.w, self.w)
    }

    #[inline]
    fn xyxx(self) -> IVec4 {
        IVec4::new(self.x, self.y, self.x, self.x)
    }

    #[inline]
    fn xyxy(self) -> IVec4 {
        IVec4::new(self.x, self.y, self.x, self.y)
    }

    #[inline]
    fn xyxz(self) -> IVec4 {
        IVec4::new(self.x, self.y, self.x, self.z)
    }

    #[inline]
    fn xyxw(self) -> IVec4 {
        IVec4::new(self.x, self.y, self.x, self.w)
    }

    #[inline]
    fn xyyx(self) -> IVec4 {
        IVec4::new(self.x, self.y, self.y, self.x)
    }

    #[inline]
    fn xyyy(self) -> IVec4 {
        IVec4::new(self.x, self.y, self.y, self.y)
    }

    #[inline]
    fn xyyz(self) -> IVec4 {
        IVec4::new(self.x, self.y, self.y, self.z)
    }

    #[inline]
    fn xyyw(self) -> IVec4 {
        IVec4::new(self.x, self.y, self.y, self.w)
    }

    #[inline]
    fn xyzx(self) -> IVec4 {
        IVec4::new(self.x, self.y, self.z, self.x)
    }

    #[inline]
    fn xyzy(self) -> IVec4 {
        IVec4::new(self.x, self.y, self.z, self.y)
    }

    #[inline]
    fn xyzz(self) -> IVec4 {
        IVec4::new(self.x, self.y, self.z, self.z)
    }

    #[inline]
    fn xyzw(self) -> IVec4 {
        IVec4::new(self.x, self.y, self.z, self.w)
    }

    #[inline]
    fn xywx(self) -> IVec4 {
        IVec4::new(self.x, self.y, self.w, self.x)
    }

    #[inline]
    fn xywy(self) -> IVec4 {
        IVec4::new(self.x, self.y, self.w, self.y)
    }

    #[inline]
    fn xywz(self) -> IVec4 {
        IVec4::new(self.x, self.y, self.w, self.z)
    }

    #[inline]
    fn xyww(self) -> IVec4 {
        IVec4::new(self.x, self.y, self.w, self.w)
    }

    #[inline]
    fn xzxx(self) -> IVec4 {
        IVec4::new(self.x, self.z, self.x, self.x)
    }

    #[inline]
    fn xzxy(self) -> IVec4 {
        IVec4::new(self.x, self.z, self.x, self.y)
    }

    #[inline]
    fn xzxz(self) -> IVec4 {
        IVec4::new(self.x, self.z, self.x, self.z)
    }

    #[inline]
    fn xzxw(self) -> IVec4 {
        IVec4::new(self.x, self.z, self.x, self.w)
    }

    #[inline]
    fn xzyx(self) -> IVec4 {
        IVec4::new(self.x, self.z, self.y, self.x)
    }

    #[inline]
    fn xzyy(self) -> IVec4 {
        IVec4::new(self.x, self.z, self.y, self.y)
    }

    #[inline]
    fn xzyz(self) -> IVec4 {
        IVec4::new(self.x, self.z, self.y, self.z)
    }

    #[inline]
    fn xzyw(self) -> IVec4 {
        IVec4::new(self.x, self.z, self.y, self.w)
    }

    #[inline]
    fn xzzx(self) -> IVec4 {
        IVec4::new(self.x, self.z, self.z, self.x)
    }

    #[inline]
    fn xzzy(self) -> IVec4 {
        IVec4::new(self.x, self.z, self.z, self.y)
    }

    #[inline]
    fn xzzz(self) -> IVec4 {
        IVec4::new(self.x, self.z, self.z, self.z)
    }

    #[inline]
    fn xzzw(self) -> IVec4 {
        IVec4::new(self.x, self.z, self.z, self.w)
    }

    #[inline]
    fn xzwx(self) -> IVec4 {
        IVec4::new(self.x, self.z, self.w, self.x)
    }

    #[inline]
    fn xzwy(self) -> IVec4 {
        IVec4::new(self.x, self.z, self.w, self.y)
    }

    #[inline]
    fn xzwz(self) -> IVec4 {
        IVec4::new(self.x, self.z, self.w, self.z)
    }

    #[inline]
    fn xzww(self) -> IVec4 {
        IVec4::new(self.x, self.z, self.w, self.w)
    }

    #[inline]
    fn xwxx(self) -> IVec4 {
        IVec4::new(self.x, self.w, self.x, self.x)
    }

    #[inline]
    fn xwxy(self) -> IVec4 {
        IVec4::new(self.x, self.w, self.x, self.y)
    }

    #[inline]
    fn xwxz(self) -> IVec4 {
        IVec4::new(self.x, self.w, self.x, self.z)
    }

    #[inline]
    fn xwxw(self) -> IVec4 {
        IVec4::new(self.x, self.w, self.x, self.w)
    }

    #[inline]
    fn xwyx(self) -> IVec4 {
        IVec4::new(self.x, self.w, self.y, self.x)
    }

    #[inline]
    fn xwyy(self) -> IVec4 {
        IVec4::new(self.x, self.w, self.y, self.y)
    }

    #[inline]
    fn xwyz(self) -> IVec4 {
        IVec4::new(self.x, self.w, self.y, self.z)
    }

    #[inline]
    fn xwyw(self) -> IVec4 {
        IVec4::new(self.x, self.w, self.y, self.w)
    }

    #[inline]
    fn xwzx(self) -> IVec4 {
        IVec4::new(self.x, self.w, self.z, self.x)
    }

    #[inline]
    fn xwzy(self) -> IVec4 {
        IVec4::new(self.x, self.w, self.z, self.y)
    }

    #[inline]
    fn xwzz(self) -> IVec4 {
        IVec4::new(self.x, self.w, self.z, self.z)
    }

    #[inline]
    fn xwzw(self) -> IVec4 {
        IVec4::new(self.x, self.w, self.z, self.w)
    }

    #[inline]
    fn xwwx(self) -> IVec4 {
        IVec4::new(self.x, self.w, self.w, self.x)
    }

    #[inline]
    fn xwwy(self) -> IVec4 {
        IVec4::new(self.x, self.w, self.w, self.y)
    }

    #[inline]
    fn xwwz(self) -> IVec4 {
        IVec4::new(self.x, self.w, self.w, self.z)
    }

    #[inline]
    fn xwww(self) -> IVec4 {
        IVec4::new(self.x, self.w, self.w, self.w)
    }

    #[inline]
    fn yxxx(self) -> IVec4 {
        IVec4::new(self.y, self.x, self.x, self.x)
    }

    #[inline]
    fn yxxy(self) -> IVec4 {
        IVec4::new(self.y, self.x, self.x, self.y)
    }

    #[inline]
    fn yxxz(self) -> IVec4 {
        IVec4::new(self.y, self.x, self.x, self.z)
    }

    #[inline]
    fn yxxw(self) -> IVec4 {
        IVec4::new(self.y, self.x, self.x, self.w)
    }

    #[inline]
    fn yxyx(self) -> IVec4 {
        IVec4::new(self.y, self.x, self.y, self.x)
    }

    #[inline]
    fn yxyy(self) -> IVec4 {
        IVec4::new(self.y, self.x, self.y, self.y)
    }

    #[inline]
    fn yxyz(self) -> IVec4 {
        IVec4::new(self.y, self.x, self.y, self.z)
    }

    #[inline]
    fn yxyw(self) -> IVec4 {
        IVec4::new(self.y, self.x, self.y, self.w)
    }

    #[inline]
    fn yxzx(self) -> IVec4 {
        IVec4::new(self.y, self.x, self.z, self.x)
    }

    #[inline]
    fn yxzy(self) -> IVec4 {
        IVec4::new(self.y, self.x, self.z, self.y)
    }

    #[inline]
    fn yxzz(self) -> IVec4 {
        IVec4::new(self.y, self.x, self.z, self.z)
    }

    #[inline]
    fn yxzw(self) -> IVec4 {
        IVec4::new(self.y, self.x, self.z, self.w)
    }

    #[inline]
    fn yxwx(self) -> IVec4 {
        IVec4::new(self.y, self.x, self.w, self.x)
    }

    #[inline]
    fn yxwy(self) -> IVec4 {
        IVec4::new(self.y, self.x, self.w, self.y)
    }

    #[inline]
    fn yxwz(self) -> IVec4 {
        IVec4::new(self.y, self.x, self.w, self.z)
    }

    #[inline]
    fn yxww(self) -> IVec4 {
        IVec4::new(self.y, self.x, self.w, self.w)
    }

    #[inline]
    fn yyxx(self) -> IVec4 {
        IVec4::new(self.y, self.y, self.x, self.x)
    }

    #[inline]
    fn yyxy(self) -> IVec4 {
        IVec4::new(self.y, self.y, self.x, self.y)
    }

    #[inline]
    fn yyxz(self) -> IVec4 {
        IVec4::new(self.y, self.y, self.x, self.z)
    }

    #[inline]
    fn yyxw(self) -> IVec4 {
        IVec4::new(self.y, self.y, self.x, self.w)
    }

    #[inline]
    fn yyyx(self) -> IVec4 {
        IVec4::new(self.y, self.y, self.y, self.x)
    }

    #[inline]
    fn yyyy(self) -> IVec4 {
        IVec4::new(self.y, self.y, self.y, self.y)
    }

    #[inline]
    fn yyyz(self) -> IVec4 {
        IVec4::new(self.y, self.y, self.y, self.z)
    }

    #[inline]
    fn yyyw(self) -> IVec4 {
        IVec4::new(self.y, self.y, self.y, self.w)
    }

    #[inline]
    fn yyzx(self) -> IVec4 {
        IVec4::new(self.y, self.y, self.z, self.x)
    }

    #[inline]
    fn yyzy(self) -> IVec4 {
        IVec4::new(self.y, self.y, self.z, self.y)
    }

    #[inline]
    fn yyzz(self) -> IVec4 {
        IVec4::new(self.y, self.y, self.z, self.z)
    }

    #[inline]
    fn yyzw(self) -> IVec4 {
        IVec4::new(self.y, self.y, self.z, self.w)
    }

    #[inline]
    fn yywx(self) -> IVec4 {
        IVec4::new(self.y, self.y, self.w, self.x)
    }

    #[inline]
    fn yywy(self) -> IVec4 {
        IVec4::new(self.y, self.y, self.w, self.y)
    }

    #[inline]
    fn yywz(self) -> IVec4 {
        IVec4::new(self.y, self.y, self.w, self.z)
    }

    #[inline]
    fn yyww(self) -> IVec4 {
        IVec4::new(self.y, self.y, self.w, self.w)
    }

    #[inline]
    fn yzxx(self) -> IVec4 {
        IVec4::new(self.y, self.z, self.x, self.x)
    }

    #[inline]
    fn yzxy(self) -> IVec4 {
        IVec4::new(self.y, self.z, self.x, self.y)
    }

    #[inline]
    fn yzxz(self) -> IVec4 {
        IVec4::new(self.y, self.z, self.x, self.z)
    }

    #[inline]
    fn yzxw(self) -> IVec4 {
        IVec4::new(self.y, self.z, self.x, self.w)
    }

    #[inline]
    fn yzyx(self) -> IVec4 {
        IVec4::new(self.y, self.z, self.y, self.x)
    }

    #[inline]
    fn yzyy(self) -> IVec4 {
        IVec4::new(self.y, self.z, self.y, self.y)
    }

    #[inline]
    fn yzyz(self) -> IVec4 {
        IVec4::new(self.y, self.z, self.y, self.z)
    }

    #[inline]
    fn yzyw(self) -> IVec4 {
        IVec4::new(self.y, self.z, self.y, self.w)
    }

    #[inline]
    fn yzzx(self) -> IVec4 {
        IVec4::new(self.y, self.z, self.z, self.x)
    }

    #[inline]
    fn yzzy(self) -> IVec4 {
        IVec4::new(self.y, self.z, self.z, self.y)
    }

    #[inline]
    fn yzzz(self) -> IVec4 {
        IVec4::new(self.y, self.z, self.z, self.z)
    }

    #[inline]
    fn yzzw(self) -> IVec4 {
        IVec4::new(self.y, self.z, self.z, self.w)
    }

    #[inline]
    fn yzwx(self) -> IVec4 {
        IVec4::new(self.y, self.z, self.w, self.x)
    }

    #[inline]
    fn yzwy(self) -> IVec4 {
        IVec4::new(self.y, self.z, self.w, self.y)
    }

    #[inline]
    fn yzwz(self) -> IVec4 {
        IVec4::new(self.y, self.z, self.w, self.z)
    }

    #[inline]
    fn yzww(self) -> IVec4 {
        IVec4::new(self.y, self.z, self.w, self.w)
    }

    #[inline]
    fn ywxx(self) -> IVec4 {
        IVec4::new(self.y, self.w, self.x, self.x)
    }

    #[inline]
    fn ywxy(self) -> IVec4 {
        IVec4::new(self.y, self.w, self.x, self.y)
    }

    #[inline]
    fn ywxz(self) -> IVec4 {
        IVec4::new(self.y, self.w, self.x, self.z)
    }

    #[inline]
    fn ywxw(self) -> IVec4 {
        IVec4::new(self.y, self.w, self.x, self.w)
    }

    #[inline]
    fn ywyx(self) -> IVec4 {
        IVec4::new(self.y, self.w, self.y, self.x)
    }

    #[inline]
    fn ywyy(self) -> IVec4 {
        IVec4::new(self.y, self.w, self.y, self.y)
    }

    #[inline]
    fn ywyz(self) -> IVec4 {
        IVec4::new(self.y, self.w, self.y, self.z)
    }

    #[inline]
    fn ywyw(self) -> IVec4 {
        IVec4::new(self.y, self.w, self.y, self.w)
    }

    #[inline]
    fn ywzx(self) -> IVec4 {
        IVec4::new(self.y, self.w, self.z, self.x)
    }

    #[inline]
    fn ywzy(self) -> IVec4 {
        IVec4::new(self.y, self.w, self.z, self.y)
    }

    #[inline]
    fn ywzz(self) -> IVec4 {
        IVec4::new(self.y, self.w, self.z, self.z)
    }

    #[inline]
    fn ywzw(self) -> IVec4 {
        IVec4::new(self.y, self.w, self.z, self.w)
    }

    #[inline]
    fn ywwx(self) -> IVec4 {
        IVec4::new(self.y, self.w, self.w, self.x)
    }

    #[inline]
    fn ywwy(self) -> IVec4 {
        IVec4::new(self.y, self.w, self.w, self.y)
    }

    #[inline]
    fn ywwz(self) -> IVec4 {
        IVec4::new(self.y, self.w, self.w, self.z)
    }

    #[inline]
    fn ywww(self) -> IVec4 {
        IVec4::new(self.y, self.w, self.w, self.w)
    }

    #[inline]
    fn zxxx(self) -> IVec4 {
        IVec4::new(self.z, self.x, self.x, self.x)
    }

    #[inline]
    fn zxxy(self) -> IVec4 {
        IVec4::new(self.z, self.x, self.x, self.y)
    }

    #[inline]
    fn zxxz(self) -> IVec4 {
        IVec4::new(self.z, self.x, self.x, self.z)
    }

    #[inline]
    fn zxxw(self) -> IVec4 {
        IVec4::new(self.z, self.x, self.x, self.w)
    }

    #[inline]
    fn zxyx(self) -> IVec4 {
        IVec4::new(self.z, self.x, self.y, self.x)
    }

    #[inline]
    fn zxyy(self) -> IVec4 {
        IVec4::new(self.z, self.x, self.y, self.y)
    }

    #[inline]
    fn zxyz(self) -> IVec4 {
        IVec4::new(self.z, self.x, self.y, self.z)
    }

    #[inline]
    fn zxyw(self) -> IVec4 {
        IVec4::new(self.z, self.x, self.y, self.w)
    }

    #[inline]
    fn zxzx(self) -> IVec4 {
        IVec4::new(self.z, self.x, self.z, self.x)
    }

    #[inline]
    fn zxzy(self) -> IVec4 {
        IVec4::new(self.z, self.x, self.z, self.y)
    }

    #[inline]
    fn zxzz(self) -> IVec4 {
        IVec4::new(self.z, self.x, self.z, self.z)
    }

    #[inline]
    fn zxzw(self) -> IVec4 {
        IVec4::new(self.z, self.x, self.z, self.w)
    }

    #[inline]
    fn zxwx(self) -> IVec4 {
        IVec4::new(self.z, self.x, self.w, self.x)
    }

    #[inline]
    fn zxwy(self) -> IVec4 {
        IVec4::new(self.z, self.x, self.w, self.y)
    }

    #[inline]
    fn zxwz(self) -> IVec4 {
        IVec4::new(self.z, self.x, self.w, self.z)
    }

    #[inline]
    fn zxww(self) -> IVec4 {
        IVec4::new(self.z, self.x, self.w, self.w)
    }

    #[inline]
    fn zyxx(self) -> IVec4 {
        IVec4::new(self.z, self.y, self.x, self.x)
    }

    #[inline]
    fn zyxy(self) -> IVec4 {
        IVec4::new(self.z, self.y, self.x, self.y)
    }

    #[inline]
    fn zyxz(self) -> IVec4 {
        IVec4::new(self.z, self.y, self.x, self.z)
    }

    #[inline]
    fn zyxw(self) -> IVec4 {
        IVec4::new(self.z, self.y, self.x, self.w)
    }

    #[inline]
    fn zyyx(self) -> IVec4 {
        IVec4::new(self.z, self.y, self.y, self.x)
    }

    #[inline]
    fn zyyy(self) -> IVec4 {
        IVec4::new(self.z, self.y, self.y, self.y)
    }

    #[inline]
    fn zyyz(self) -> IVec4 {
        IVec4::new(self.z, self.y, self.y, self.z)
    }

    #[inline]
    fn zyyw(self) -> IVec4 {
        IVec4::new(self.z, self.y, self.y, self.w)
    }

    #[inline]
    fn zyzx(self) -> IVec4 {
        IVec4::new(self.z, self.y, self.z, self.x)
    }

    #[inline]
    fn zyzy(self) -> IVec4 {
        IVec4::new(self.z, self.y, self.z, self.y)
    }

    #[inline]
    fn zyzz(self) -> IVec4 {
        IVec4::new(self.z, self.y, self.z, self.z)
    }

    #[inline]
    fn zyzw(self) -> IVec4 {
        IVec4::new(self.z, self.y, self.z, self.w)
    }

    #[inline]
    fn zywx(self) -> IVec4 {
        IVec4::new(self.z, self.y, self.w, self.x)
    }

    #[inline]
    fn zywy(self) -> IVec4 {
        IVec4::new(self.z, self.y, self.w, self.y)
    }

    #[inline]
    fn zywz(self) -> IVec4 {
        IVec4::new(self.z, self.y, self.w, self.z)
    }

    #[inline]
    fn zyww(self) -> IVec4 {
        IVec4::new(self.z, self.y, self.w, self.w)
    }

    #[inline]
    fn zzxx(self) -> IVec4 {
        IVec4::new(self.z, self.z, self.x, self.x)
    }

    #[inline]
    fn zzxy(self) -> IVec4 {
        IVec4::new(self.z, self.z, self.x, self.y)
    }

    #[inline]
    fn zzxz(self) -> IVec4 {
        IVec4::new(self.z, self.z, self.x, self.z)
    }

    #[inline]
    fn zzxw(self) -> IVec4 {
        IVec4::new(self.z, self.z, self.x, self.w)
    }

    #[inline]
    fn zzyx(self) -> IVec4 {
        IVec4::new(self.z, self.z, self.y, self.x)
    }

    #[inline]
    fn zzyy(self) -> IVec4 {
        IVec4::new(self.z, self.z, self.y, self.y)
    }

    #[inline]
    fn zzyz(self) -> IVec4 {
        IVec4::new(self.z, self.z, self.y, self.z)
    }

    #[inline]
    fn zzyw(self) -> IVec4 {
        IVec4::new(self.z, self.z, self.y, self.w)
    }

    #[inline]
    fn zzzx(self) -> IVec4 {
        IVec4::new(self.z, self.z, self.z, self.x)
    }

    #[inline]
    fn zzzy(self) -> IVec4 {
        IVec4::new(self.z, self.z, self.z, self.y)
    }

    #[inline]
    fn zzzz(self) -> IVec4 {
        IVec4::new(self.z, self.z, self.z, self.z)
    }

    #[inline]
    fn zzzw(self) -> IVec4 {
        IVec4::new(self.z, self.z, self.z, self.w)
    }

    #[inline]
    fn zzwx(self) -> IVec4 {
        IVec4::new(self.z, self.z, self.w, self.x)
    }

    #[inline]
    fn zzwy(self) -> IVec4 {
        IVec4::new(self.z, self.z, self.w, self.y)
    }

    #[inline]
    fn zzwz(self) -> IVec4 {
        IVec4::new(self.z, self.z, self.w, self.z)
    }

    #[inline]
    fn zzww(self) -> IVec4 {
        IVec4::new(self.z, self.z, self.w, self.w)
    }

    #[inline]
    fn zwxx(self) -> IVec4 {
        IVec4::new(self.z, self.w, self.x, self.x)
    }

    #[inline]
    fn zwxy(self) -> IVec4 {
        IVec4::new(self.z, self.w, self.x, self.y)
    }

    #[inline]
    fn zwxz(self) -> IVec4 {
        IVec4::new(self.z, self.w, self.x, self.z)
    }

    #[inline]
    fn zwxw(self) -> IVec4 {
        IVec4::new(self.z, self.w, self.x, self.w)
    }

    #[inline]
    fn zwyx(self) -> IVec4 {
        IVec4::new(self.z, self.w, self.y, self.x)
    }

    #[inline]
    fn zwyy(self) -> IVec4 {
        IVec4::new(self.z, self.w, self.y, self.y)
    }

    #[inline]
    fn zwyz(self) -> IVec4 {
        IVec4::new(self.z, self.w, self.y, self.z)
    }

    #[inline]
    fn zwyw(self) -> IVec4 {
        IVec4::new(self.z, self.w, self.y, self.w)
    }

    #[inline]
    fn zwzx(self) -> IVec4 {
        IVec4::new(self.z, self.w, self.z, self.x)
    }

    #[inline]
    fn zwzy(self) -> IVec4 {
        IVec4::new(self.z, self.w, self.z, self.y)
    }

    #[inline]
    fn zwzz(self) -> IVec4 {
        IVec4::new(self.z, self.w, self.z, self.z)
    }

    #[inline]
    fn zwzw(self) -> IVec4 {
        IVec4::new(self.z, self.w, self.z, self.w)
    }

    #[inline]
    fn zwwx(self) -> IVec4 {
        IVec4::new(self.z, self.w, self.w, self.x)
    }

    #[inline]
    fn zwwy(self) -> IVec4 {
        IVec4::new(self.z, self.w, self.w, self.y)
    }

    #[inline]
    fn zwwz(self) -> IVec4 {
        IVec4::new(self.z, self.w, self.w, self.z)
    }

    #[inline]
    fn zwww(self) -> IVec4 {
        IVec4::new(self.z, self.w, self.w, self.w)
    }

    #[inline]
    fn wxxx(self) -> IVec4 {
        IVec4::new(self.w, self.x, self.x, self.x)
    }

    #[inline]
    fn wxxy(self) -> IVec4 {
        IVec4::new(self.w, self.x, self.x, self.y)
    }

    #[inline]
    fn wxxz(self) -> IVec4 {
        IVec4::new(self.w, self.x, self.x, self.z)
    }

    #[inline]
    fn wxxw(self) -> IVec4 {
        IVec4::new(self.w, self.x, self.x, self.w)
    }

    #[inline]
    fn wxyx(self) -> IVec4 {
        IVec4::new(self.w, self.x, self.y, self.x)
    }

    #[inline]
    fn wxyy(self) -> IVec4 {
        IVec4::new(self.w, self.x, self.y, self.y)
    }

    #[inline]
    fn wxyz(self) -> IVec4 {
        IVec4::new(self.w, self.x, self.y, self.z)
    }

    #[inline]
    fn wxyw(self) -> IVec4 {
        IVec4::new(self.w, self.x, self.y, self.w)
    }

    #[inline]
    fn wxzx(self) -> IVec4 {
        IVec4::new(self.w, self.x, self.z, self.x)
    }

    #[inline]
    fn wxzy(self) -> IVec4 {
        IVec4::new(self.w, self.x, self.z, self.y)
    }

    #[inline]
    fn wxzz(self) -> IVec4 {
        IVec4::new(self.w, self.x, self.z, self.z)
    }

    #[inline]
    fn wxzw(self) -> IVec4 {
        IVec4::new(self.w, self.x, self.z, self.w)
    }

    #[inline]
    fn wxwx(self) -> IVec4 {
        IVec4::new(self.w, self.x, self.w, self.x)
    }

    #[inline]
    fn wxwy(self) -> IVec4 {
        IVec4::new(self.w, self.x, self.w, self.y)
    }

    #[inline]
    fn wxwz(self) -> IVec4 {
        IVec4::new(self.w, self.x, self.w, self.z)
    }

    #[inline]
    fn wxww(self) -> IVec4 {
        IVec4::new(self.w, self.x, self.w, self.w)
    }

    #[inline]
    fn wyxx(self) -> IVec4 {
        IVec4::new(self.w, self.y, self.x, self.x)
    }

    #[inline]
    fn wyxy(self) -> IVec4 {
        IVec4::new(self.w, self.y, self.x, self.y)
    }

    #[inline]
    fn wyxz(self) -> IVec4 {
        IVec4::new(self.w, self.y, self.x, self.z)
    }

    #[inline]
    fn wyxw(self) -> IVec4 {
        IVec4::new(self.w, self.y, self.x, self.w)
    }

    #[inline]
    fn wyyx(self) -> IVec4 {
        IVec4::new(self.w, self.y, self.y, self.x)
    }

    #[inline]
    fn wyyy(self) -> IVec4 {
        IVec4::new(self.w, self.y, self.y, self.y)
    }

    #[inline]
    fn wyyz(self) -> IVec4 {
        IVec4::new(self.w, self.y, self.y, self.z)
    }

    #[inline]
    fn wyyw(self) -> IVec4 {
        IVec4::new(self.w, self.y, self.y, self.w)
    }

    #[inline]
    fn wyzx(self) -> IVec4 {
        IVec4::new(self.w, self.y, self.z, self.x)
    }

    #[inline]
    fn wyzy(self) -> IVec4 {
        IVec4::new(self.w, self.y, self.z, self.y)
    }

    #[inline]
    fn wyzz(self) -> IVec4 {
        IVec4::new(self.w, self.y, self.z, self.z)
    }

    #[inline]
    fn wyzw(self) -> IVec4 {
        IVec4::new(self.w, self.y, self.z, self.w)
    }

    #[inline]
    fn wywx(self) -> IVec4 {
        IVec4::new(self.w, self.y, self.w, self.x)
    }

    #[inline]
    fn wywy(self) -> IVec4 {
        IVec4::new(self.w, self.y, self.w, self.y)
    }

    #[inline]
    fn wywz(self) -> IVec4 {
        IVec4::new(self.w, self.y, self.w, self.z)
    }

    #[inline]
    fn wyww(self) -> IVec4 {
        IVec4::new(self.w, self.y, self.w, self.w)
    }

    #[inline]
    fn wzxx(self) -> IVec4 {
        IVec4::new(self.w, self.z, self.x, self.x)
    }

    #[inline]
    fn wzxy(self) -> IVec4 {
        IVec4::new(self.w, self.z, self.x, self.y)
    }

    #[inline]
    fn wzxz(self) -> IVec4 {
        IVec4::new(self.w, self.z, self.x, self.z)
    }

    #[inline]
    fn wzxw(self) -> IVec4 {
        IVec4::new(self.w, self.z, self.x, self.w)
    }

    #[inline]
    fn wzyx(self) -> IVec4 {
        IVec4::new(self.w, self.z, self.y, self.x)
    }

    #[inline]
    fn wzyy(self) -> IVec4 {
        IVec4::new(self.w, self.z, self.y, self.y)
    }

    #[inline]
    fn wzyz(self) -> IVec4 {
        IVec4::new(self.w, self.z, self.y, self.z)
    }

    #[inline]
    fn wzyw(self) -> IVec4 {
        IVec4::new(self.w, self.z, self.y, self.w)
    }

    #[inline]
    fn wzzx(self) -> IVec4 {
        IVec4::new(self.w, self.z, self.z, self.x)
    }

    #[inline]
    fn wzzy(self) -> IVec4 {
        IVec4::new(self.w, self.z, self.z, self.y)
    }

    #[inline]
    fn wzzz(self) -> IVec4 {
        IVec4::new(self.w, self.z, self.z, self.z)
    }

    #[inline]
    fn wzzw(self) -> IVec4 {
        IVec4::new(self.w, self.z, self.z, self.w)
    }

    #[inline]
    fn wzwx(self) -> IVec4 {
        IVec4::new(self.w, self.z, self.w, self.x)
    }

    #[inline]
    fn wzwy(self) -> IVec4 {
        IVec4::new(self.w, self.z, self.w, self.y)
    }

    #[inline]
    fn wzwz(self) -> IVec4 {
        IVec4::new(self.w, self.z, self.w, self.z)
    }

    #[inline]
    fn wzww(self) -> IVec4 {
        IVec4::new(self.w, self.z, self.w, self.w)
    }

    #[inline]
    fn wwxx(self) -> IVec4 {
        IVec4::new(self.w, self.w, self.x, self.x)
    }

    #[inline]
    fn wwxy(self) -> IVec4 {
        IVec4::new(self.w, self.w, self.x, self.y)
    }

    #[inline]
    fn wwxz(self) -> IVec4 {
        IVec4::new(self.w, self.w, self.x, self.z)
    }

    #[inline]
    fn wwxw(self) -> IVec4 {
        IVec4::new(self.w, self.w, self.x, self.w)
    }

    #[inline]
    fn wwyx(self) -> IVec4 {
        IVec4::new(self.w, self.w, self.y, self.x)
    }

    #[inline]
    fn wwyy(self) -> IVec4 {
        IVec4::new(self.w, self.w, self.y, self.y)
    }

    #[inline]
    fn wwyz(self) -> IVec4 {
        IVec4::new(self.w, self.w, self.y, self.z)
    }

    #[inline]
    fn wwyw(self) -> IVec4 {
        IVec4::new(self.w, self.w, self.y, self.w)
    }

    #[inline]
    fn wwzx(self) -> IVec4 {
        IVec4::new(self.w, self.w, self.z, self.x)
    }

    #[inline]
    fn wwzy(self) -> IVec4 {
        IVec4::new(self.w, self.w, self.z, self.y)
    }

    #[inline]
    fn wwzz(self) -> IVec4 {
        IVec4::new(self.w, self.w, self.z, self.z)
    }

    #[inline]
    fn wwzw(self) -> IVec4 {
        IVec4::new(self.w, self.w, self.z, self.w)
    }

    #[inline]
    fn wwwx(self) -> IVec4 {
        IVec4::new(self.w, self.w, self.w, self.x)
    }

    #[inline]
    fn wwwy(self) -> IVec4 {
        IVec4::new(self.w, self.w, self.w, self.y)
    }

    #[inline]
    fn wwwz(self) -> IVec4 {
        IVec4::new(self.w, self.w, self.w, self.z)
    }

    #[inline]
    fn wwww(self) -> IVec4 {
        IVec4::new(self.w, self.w, self.w, self.w)
    }
}
