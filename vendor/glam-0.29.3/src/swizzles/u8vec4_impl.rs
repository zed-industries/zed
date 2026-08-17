// Generated from swizzle_impl.rs.tera template. Edit the template, not the generated file.

use crate::{U8Vec2, U8Vec3, U8Vec4, Vec4Swizzles};

impl Vec4Swizzles for U8Vec4 {
    type Vec2 = U8Vec2;

    type Vec3 = U8Vec3;

    #[inline]
    #[must_use]
    fn xx(self) -> U8Vec2 {
        U8Vec2 {
            x: self.x,
            y: self.x,
        }
    }

    #[inline]
    #[must_use]
    fn xy(self) -> U8Vec2 {
        U8Vec2 {
            x: self.x,
            y: self.y,
        }
    }

    #[inline]
    #[must_use]
    fn xz(self) -> U8Vec2 {
        U8Vec2 {
            x: self.x,
            y: self.z,
        }
    }

    #[inline]
    #[must_use]
    fn xw(self) -> U8Vec2 {
        U8Vec2 {
            x: self.x,
            y: self.w,
        }
    }

    #[inline]
    #[must_use]
    fn yx(self) -> U8Vec2 {
        U8Vec2 {
            x: self.y,
            y: self.x,
        }
    }

    #[inline]
    #[must_use]
    fn yy(self) -> U8Vec2 {
        U8Vec2 {
            x: self.y,
            y: self.y,
        }
    }

    #[inline]
    #[must_use]
    fn yz(self) -> U8Vec2 {
        U8Vec2 {
            x: self.y,
            y: self.z,
        }
    }

    #[inline]
    #[must_use]
    fn yw(self) -> U8Vec2 {
        U8Vec2 {
            x: self.y,
            y: self.w,
        }
    }

    #[inline]
    #[must_use]
    fn zx(self) -> U8Vec2 {
        U8Vec2 {
            x: self.z,
            y: self.x,
        }
    }

    #[inline]
    #[must_use]
    fn zy(self) -> U8Vec2 {
        U8Vec2 {
            x: self.z,
            y: self.y,
        }
    }

    #[inline]
    #[must_use]
    fn zz(self) -> U8Vec2 {
        U8Vec2 {
            x: self.z,
            y: self.z,
        }
    }

    #[inline]
    #[must_use]
    fn zw(self) -> U8Vec2 {
        U8Vec2 {
            x: self.z,
            y: self.w,
        }
    }

    #[inline]
    #[must_use]
    fn wx(self) -> U8Vec2 {
        U8Vec2 {
            x: self.w,
            y: self.x,
        }
    }

    #[inline]
    #[must_use]
    fn wy(self) -> U8Vec2 {
        U8Vec2 {
            x: self.w,
            y: self.y,
        }
    }

    #[inline]
    #[must_use]
    fn wz(self) -> U8Vec2 {
        U8Vec2 {
            x: self.w,
            y: self.z,
        }
    }

    #[inline]
    #[must_use]
    fn ww(self) -> U8Vec2 {
        U8Vec2 {
            x: self.w,
            y: self.w,
        }
    }

    #[inline]
    #[must_use]
    fn xxx(self) -> U8Vec3 {
        U8Vec3::new(self.x, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn xxy(self) -> U8Vec3 {
        U8Vec3::new(self.x, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn xxz(self) -> U8Vec3 {
        U8Vec3::new(self.x, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn xxw(self) -> U8Vec3 {
        U8Vec3::new(self.x, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn xyx(self) -> U8Vec3 {
        U8Vec3::new(self.x, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn xyy(self) -> U8Vec3 {
        U8Vec3::new(self.x, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn xyz(self) -> U8Vec3 {
        U8Vec3::new(self.x, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn xyw(self) -> U8Vec3 {
        U8Vec3::new(self.x, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn xzx(self) -> U8Vec3 {
        U8Vec3::new(self.x, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn xzy(self) -> U8Vec3 {
        U8Vec3::new(self.x, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn xzz(self) -> U8Vec3 {
        U8Vec3::new(self.x, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn xzw(self) -> U8Vec3 {
        U8Vec3::new(self.x, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn xwx(self) -> U8Vec3 {
        U8Vec3::new(self.x, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn xwy(self) -> U8Vec3 {
        U8Vec3::new(self.x, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn xwz(self) -> U8Vec3 {
        U8Vec3::new(self.x, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn xww(self) -> U8Vec3 {
        U8Vec3::new(self.x, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn yxx(self) -> U8Vec3 {
        U8Vec3::new(self.y, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn yxy(self) -> U8Vec3 {
        U8Vec3::new(self.y, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn yxz(self) -> U8Vec3 {
        U8Vec3::new(self.y, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn yxw(self) -> U8Vec3 {
        U8Vec3::new(self.y, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn yyx(self) -> U8Vec3 {
        U8Vec3::new(self.y, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn yyy(self) -> U8Vec3 {
        U8Vec3::new(self.y, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn yyz(self) -> U8Vec3 {
        U8Vec3::new(self.y, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn yyw(self) -> U8Vec3 {
        U8Vec3::new(self.y, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn yzx(self) -> U8Vec3 {
        U8Vec3::new(self.y, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn yzy(self) -> U8Vec3 {
        U8Vec3::new(self.y, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn yzz(self) -> U8Vec3 {
        U8Vec3::new(self.y, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn yzw(self) -> U8Vec3 {
        U8Vec3::new(self.y, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn ywx(self) -> U8Vec3 {
        U8Vec3::new(self.y, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn ywy(self) -> U8Vec3 {
        U8Vec3::new(self.y, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn ywz(self) -> U8Vec3 {
        U8Vec3::new(self.y, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn yww(self) -> U8Vec3 {
        U8Vec3::new(self.y, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn zxx(self) -> U8Vec3 {
        U8Vec3::new(self.z, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn zxy(self) -> U8Vec3 {
        U8Vec3::new(self.z, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn zxz(self) -> U8Vec3 {
        U8Vec3::new(self.z, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn zxw(self) -> U8Vec3 {
        U8Vec3::new(self.z, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn zyx(self) -> U8Vec3 {
        U8Vec3::new(self.z, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn zyy(self) -> U8Vec3 {
        U8Vec3::new(self.z, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn zyz(self) -> U8Vec3 {
        U8Vec3::new(self.z, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn zyw(self) -> U8Vec3 {
        U8Vec3::new(self.z, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn zzx(self) -> U8Vec3 {
        U8Vec3::new(self.z, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn zzy(self) -> U8Vec3 {
        U8Vec3::new(self.z, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn zzz(self) -> U8Vec3 {
        U8Vec3::new(self.z, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn zzw(self) -> U8Vec3 {
        U8Vec3::new(self.z, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn zwx(self) -> U8Vec3 {
        U8Vec3::new(self.z, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn zwy(self) -> U8Vec3 {
        U8Vec3::new(self.z, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn zwz(self) -> U8Vec3 {
        U8Vec3::new(self.z, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn zww(self) -> U8Vec3 {
        U8Vec3::new(self.z, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn wxx(self) -> U8Vec3 {
        U8Vec3::new(self.w, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn wxy(self) -> U8Vec3 {
        U8Vec3::new(self.w, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn wxz(self) -> U8Vec3 {
        U8Vec3::new(self.w, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn wxw(self) -> U8Vec3 {
        U8Vec3::new(self.w, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn wyx(self) -> U8Vec3 {
        U8Vec3::new(self.w, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn wyy(self) -> U8Vec3 {
        U8Vec3::new(self.w, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn wyz(self) -> U8Vec3 {
        U8Vec3::new(self.w, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn wyw(self) -> U8Vec3 {
        U8Vec3::new(self.w, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn wzx(self) -> U8Vec3 {
        U8Vec3::new(self.w, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn wzy(self) -> U8Vec3 {
        U8Vec3::new(self.w, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn wzz(self) -> U8Vec3 {
        U8Vec3::new(self.w, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn wzw(self) -> U8Vec3 {
        U8Vec3::new(self.w, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn wwx(self) -> U8Vec3 {
        U8Vec3::new(self.w, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn wwy(self) -> U8Vec3 {
        U8Vec3::new(self.w, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn wwz(self) -> U8Vec3 {
        U8Vec3::new(self.w, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn www(self) -> U8Vec3 {
        U8Vec3::new(self.w, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn xxxx(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.x, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn xxxy(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.x, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn xxxz(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.x, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn xxxw(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.x, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn xxyx(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.x, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn xxyy(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.x, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn xxyz(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.x, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn xxyw(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.x, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn xxzx(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.x, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn xxzy(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.x, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn xxzz(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.x, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn xxzw(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.x, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn xxwx(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.x, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn xxwy(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.x, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn xxwz(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.x, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn xxww(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.x, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn xyxx(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.y, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn xyxy(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.y, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn xyxz(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.y, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn xyxw(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.y, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn xyyx(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.y, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn xyyy(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.y, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn xyyz(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.y, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn xyyw(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.y, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn xyzx(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.y, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn xyzy(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.y, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn xyzz(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.y, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn xywx(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.y, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn xywy(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.y, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn xywz(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.y, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn xyww(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.y, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn xzxx(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.z, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn xzxy(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.z, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn xzxz(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.z, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn xzxw(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.z, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn xzyx(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.z, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn xzyy(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.z, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn xzyz(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.z, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn xzyw(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.z, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn xzzx(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.z, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn xzzy(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.z, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn xzzz(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.z, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn xzzw(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.z, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn xzwx(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.z, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn xzwy(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.z, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn xzwz(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.z, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn xzww(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.z, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn xwxx(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.w, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn xwxy(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.w, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn xwxz(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.w, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn xwxw(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.w, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn xwyx(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.w, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn xwyy(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.w, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn xwyz(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.w, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn xwyw(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.w, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn xwzx(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.w, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn xwzy(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.w, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn xwzz(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.w, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn xwzw(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.w, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn xwwx(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.w, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn xwwy(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.w, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn xwwz(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.w, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn xwww(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.w, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn yxxx(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.x, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn yxxy(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.x, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn yxxz(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.x, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn yxxw(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.x, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn yxyx(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.x, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn yxyy(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.x, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn yxyz(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.x, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn yxyw(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.x, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn yxzx(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.x, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn yxzy(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.x, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn yxzz(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.x, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn yxzw(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.x, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn yxwx(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.x, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn yxwy(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.x, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn yxwz(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.x, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn yxww(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.x, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn yyxx(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.y, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn yyxy(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.y, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn yyxz(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.y, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn yyxw(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.y, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn yyyx(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.y, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn yyyy(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.y, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn yyyz(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.y, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn yyyw(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.y, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn yyzx(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.y, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn yyzy(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.y, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn yyzz(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.y, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn yyzw(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.y, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn yywx(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.y, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn yywy(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.y, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn yywz(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.y, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn yyww(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.y, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn yzxx(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.z, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn yzxy(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.z, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn yzxz(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.z, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn yzxw(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.z, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn yzyx(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.z, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn yzyy(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.z, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn yzyz(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.z, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn yzyw(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.z, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn yzzx(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.z, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn yzzy(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.z, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn yzzz(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.z, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn yzzw(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.z, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn yzwx(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.z, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn yzwy(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.z, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn yzwz(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.z, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn yzww(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.z, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn ywxx(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.w, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn ywxy(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.w, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn ywxz(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.w, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn ywxw(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.w, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn ywyx(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.w, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn ywyy(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.w, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn ywyz(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.w, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn ywyw(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.w, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn ywzx(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.w, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn ywzy(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.w, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn ywzz(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.w, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn ywzw(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.w, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn ywwx(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.w, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn ywwy(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.w, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn ywwz(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.w, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn ywww(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.w, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn zxxx(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.x, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn zxxy(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.x, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn zxxz(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.x, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn zxxw(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.x, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn zxyx(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.x, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn zxyy(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.x, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn zxyz(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.x, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn zxyw(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.x, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn zxzx(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.x, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn zxzy(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.x, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn zxzz(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.x, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn zxzw(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.x, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn zxwx(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.x, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn zxwy(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.x, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn zxwz(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.x, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn zxww(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.x, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn zyxx(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.y, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn zyxy(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.y, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn zyxz(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.y, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn zyxw(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.y, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn zyyx(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.y, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn zyyy(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.y, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn zyyz(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.y, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn zyyw(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.y, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn zyzx(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.y, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn zyzy(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.y, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn zyzz(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.y, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn zyzw(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.y, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn zywx(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.y, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn zywy(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.y, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn zywz(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.y, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn zyww(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.y, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn zzxx(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.z, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn zzxy(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.z, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn zzxz(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.z, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn zzxw(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.z, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn zzyx(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.z, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn zzyy(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.z, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn zzyz(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.z, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn zzyw(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.z, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn zzzx(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.z, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn zzzy(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.z, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn zzzz(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.z, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn zzzw(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.z, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn zzwx(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.z, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn zzwy(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.z, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn zzwz(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.z, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn zzww(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.z, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn zwxx(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.w, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn zwxy(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.w, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn zwxz(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.w, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn zwxw(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.w, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn zwyx(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.w, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn zwyy(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.w, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn zwyz(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.w, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn zwyw(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.w, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn zwzx(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.w, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn zwzy(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.w, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn zwzz(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.w, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn zwzw(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.w, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn zwwx(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.w, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn zwwy(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.w, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn zwwz(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.w, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn zwww(self) -> U8Vec4 {
        U8Vec4::new(self.z, self.w, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn wxxx(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.x, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn wxxy(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.x, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn wxxz(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.x, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn wxxw(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.x, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn wxyx(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.x, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn wxyy(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.x, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn wxyz(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.x, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn wxyw(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.x, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn wxzx(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.x, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn wxzy(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.x, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn wxzz(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.x, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn wxzw(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.x, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn wxwx(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.x, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn wxwy(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.x, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn wxwz(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.x, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn wxww(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.x, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn wyxx(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.y, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn wyxy(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.y, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn wyxz(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.y, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn wyxw(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.y, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn wyyx(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.y, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn wyyy(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.y, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn wyyz(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.y, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn wyyw(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.y, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn wyzx(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.y, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn wyzy(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.y, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn wyzz(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.y, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn wyzw(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.y, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn wywx(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.y, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn wywy(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.y, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn wywz(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.y, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn wyww(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.y, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn wzxx(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.z, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn wzxy(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.z, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn wzxz(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.z, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn wzxw(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.z, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn wzyx(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.z, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn wzyy(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.z, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn wzyz(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.z, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn wzyw(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.z, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn wzzx(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.z, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn wzzy(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.z, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn wzzz(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.z, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn wzzw(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.z, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn wzwx(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.z, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn wzwy(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.z, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn wzwz(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.z, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn wzww(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.z, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn wwxx(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.w, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn wwxy(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.w, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn wwxz(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.w, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn wwxw(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.w, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn wwyx(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.w, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn wwyy(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.w, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn wwyz(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.w, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn wwyw(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.w, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn wwzx(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.w, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn wwzy(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.w, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn wwzz(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.w, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn wwzw(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.w, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn wwwx(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.w, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn wwwy(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.w, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn wwwz(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.w, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn wwww(self) -> U8Vec4 {
        U8Vec4::new(self.w, self.w, self.w, self.w)
    }
}
