// Generated from swizzle_impl.rs.tera template. Edit the template, not the generated file.

use crate::{I8Vec2, I8Vec3, I8Vec4, Vec4Swizzles};

impl Vec4Swizzles for I8Vec4 {
    type Vec2 = I8Vec2;

    type Vec3 = I8Vec3;

    #[inline]
    #[must_use]
    fn xx(self) -> I8Vec2 {
        I8Vec2 {
            x: self.x,
            y: self.x,
        }
    }

    #[inline]
    #[must_use]
    fn xy(self) -> I8Vec2 {
        I8Vec2 {
            x: self.x,
            y: self.y,
        }
    }

    #[inline]
    #[must_use]
    fn xz(self) -> I8Vec2 {
        I8Vec2 {
            x: self.x,
            y: self.z,
        }
    }

    #[inline]
    #[must_use]
    fn xw(self) -> I8Vec2 {
        I8Vec2 {
            x: self.x,
            y: self.w,
        }
    }

    #[inline]
    #[must_use]
    fn yx(self) -> I8Vec2 {
        I8Vec2 {
            x: self.y,
            y: self.x,
        }
    }

    #[inline]
    #[must_use]
    fn yy(self) -> I8Vec2 {
        I8Vec2 {
            x: self.y,
            y: self.y,
        }
    }

    #[inline]
    #[must_use]
    fn yz(self) -> I8Vec2 {
        I8Vec2 {
            x: self.y,
            y: self.z,
        }
    }

    #[inline]
    #[must_use]
    fn yw(self) -> I8Vec2 {
        I8Vec2 {
            x: self.y,
            y: self.w,
        }
    }

    #[inline]
    #[must_use]
    fn zx(self) -> I8Vec2 {
        I8Vec2 {
            x: self.z,
            y: self.x,
        }
    }

    #[inline]
    #[must_use]
    fn zy(self) -> I8Vec2 {
        I8Vec2 {
            x: self.z,
            y: self.y,
        }
    }

    #[inline]
    #[must_use]
    fn zz(self) -> I8Vec2 {
        I8Vec2 {
            x: self.z,
            y: self.z,
        }
    }

    #[inline]
    #[must_use]
    fn zw(self) -> I8Vec2 {
        I8Vec2 {
            x: self.z,
            y: self.w,
        }
    }

    #[inline]
    #[must_use]
    fn wx(self) -> I8Vec2 {
        I8Vec2 {
            x: self.w,
            y: self.x,
        }
    }

    #[inline]
    #[must_use]
    fn wy(self) -> I8Vec2 {
        I8Vec2 {
            x: self.w,
            y: self.y,
        }
    }

    #[inline]
    #[must_use]
    fn wz(self) -> I8Vec2 {
        I8Vec2 {
            x: self.w,
            y: self.z,
        }
    }

    #[inline]
    #[must_use]
    fn ww(self) -> I8Vec2 {
        I8Vec2 {
            x: self.w,
            y: self.w,
        }
    }

    #[inline]
    #[must_use]
    fn xxx(self) -> I8Vec3 {
        I8Vec3::new(self.x, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn xxy(self) -> I8Vec3 {
        I8Vec3::new(self.x, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn xxz(self) -> I8Vec3 {
        I8Vec3::new(self.x, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn xxw(self) -> I8Vec3 {
        I8Vec3::new(self.x, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn xyx(self) -> I8Vec3 {
        I8Vec3::new(self.x, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn xyy(self) -> I8Vec3 {
        I8Vec3::new(self.x, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn xyz(self) -> I8Vec3 {
        I8Vec3::new(self.x, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn xyw(self) -> I8Vec3 {
        I8Vec3::new(self.x, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn xzx(self) -> I8Vec3 {
        I8Vec3::new(self.x, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn xzy(self) -> I8Vec3 {
        I8Vec3::new(self.x, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn xzz(self) -> I8Vec3 {
        I8Vec3::new(self.x, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn xzw(self) -> I8Vec3 {
        I8Vec3::new(self.x, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn xwx(self) -> I8Vec3 {
        I8Vec3::new(self.x, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn xwy(self) -> I8Vec3 {
        I8Vec3::new(self.x, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn xwz(self) -> I8Vec3 {
        I8Vec3::new(self.x, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn xww(self) -> I8Vec3 {
        I8Vec3::new(self.x, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn yxx(self) -> I8Vec3 {
        I8Vec3::new(self.y, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn yxy(self) -> I8Vec3 {
        I8Vec3::new(self.y, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn yxz(self) -> I8Vec3 {
        I8Vec3::new(self.y, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn yxw(self) -> I8Vec3 {
        I8Vec3::new(self.y, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn yyx(self) -> I8Vec3 {
        I8Vec3::new(self.y, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn yyy(self) -> I8Vec3 {
        I8Vec3::new(self.y, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn yyz(self) -> I8Vec3 {
        I8Vec3::new(self.y, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn yyw(self) -> I8Vec3 {
        I8Vec3::new(self.y, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn yzx(self) -> I8Vec3 {
        I8Vec3::new(self.y, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn yzy(self) -> I8Vec3 {
        I8Vec3::new(self.y, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn yzz(self) -> I8Vec3 {
        I8Vec3::new(self.y, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn yzw(self) -> I8Vec3 {
        I8Vec3::new(self.y, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn ywx(self) -> I8Vec3 {
        I8Vec3::new(self.y, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn ywy(self) -> I8Vec3 {
        I8Vec3::new(self.y, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn ywz(self) -> I8Vec3 {
        I8Vec3::new(self.y, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn yww(self) -> I8Vec3 {
        I8Vec3::new(self.y, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn zxx(self) -> I8Vec3 {
        I8Vec3::new(self.z, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn zxy(self) -> I8Vec3 {
        I8Vec3::new(self.z, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn zxz(self) -> I8Vec3 {
        I8Vec3::new(self.z, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn zxw(self) -> I8Vec3 {
        I8Vec3::new(self.z, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn zyx(self) -> I8Vec3 {
        I8Vec3::new(self.z, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn zyy(self) -> I8Vec3 {
        I8Vec3::new(self.z, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn zyz(self) -> I8Vec3 {
        I8Vec3::new(self.z, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn zyw(self) -> I8Vec3 {
        I8Vec3::new(self.z, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn zzx(self) -> I8Vec3 {
        I8Vec3::new(self.z, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn zzy(self) -> I8Vec3 {
        I8Vec3::new(self.z, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn zzz(self) -> I8Vec3 {
        I8Vec3::new(self.z, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn zzw(self) -> I8Vec3 {
        I8Vec3::new(self.z, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn zwx(self) -> I8Vec3 {
        I8Vec3::new(self.z, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn zwy(self) -> I8Vec3 {
        I8Vec3::new(self.z, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn zwz(self) -> I8Vec3 {
        I8Vec3::new(self.z, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn zww(self) -> I8Vec3 {
        I8Vec3::new(self.z, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn wxx(self) -> I8Vec3 {
        I8Vec3::new(self.w, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn wxy(self) -> I8Vec3 {
        I8Vec3::new(self.w, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn wxz(self) -> I8Vec3 {
        I8Vec3::new(self.w, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn wxw(self) -> I8Vec3 {
        I8Vec3::new(self.w, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn wyx(self) -> I8Vec3 {
        I8Vec3::new(self.w, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn wyy(self) -> I8Vec3 {
        I8Vec3::new(self.w, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn wyz(self) -> I8Vec3 {
        I8Vec3::new(self.w, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn wyw(self) -> I8Vec3 {
        I8Vec3::new(self.w, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn wzx(self) -> I8Vec3 {
        I8Vec3::new(self.w, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn wzy(self) -> I8Vec3 {
        I8Vec3::new(self.w, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn wzz(self) -> I8Vec3 {
        I8Vec3::new(self.w, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn wzw(self) -> I8Vec3 {
        I8Vec3::new(self.w, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn wwx(self) -> I8Vec3 {
        I8Vec3::new(self.w, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn wwy(self) -> I8Vec3 {
        I8Vec3::new(self.w, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn wwz(self) -> I8Vec3 {
        I8Vec3::new(self.w, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn www(self) -> I8Vec3 {
        I8Vec3::new(self.w, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn xxxx(self) -> I8Vec4 {
        I8Vec4::new(self.x, self.x, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn xxxy(self) -> I8Vec4 {
        I8Vec4::new(self.x, self.x, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn xxxz(self) -> I8Vec4 {
        I8Vec4::new(self.x, self.x, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn xxxw(self) -> I8Vec4 {
        I8Vec4::new(self.x, self.x, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn xxyx(self) -> I8Vec4 {
        I8Vec4::new(self.x, self.x, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn xxyy(self) -> I8Vec4 {
        I8Vec4::new(self.x, self.x, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn xxyz(self) -> I8Vec4 {
        I8Vec4::new(self.x, self.x, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn xxyw(self) -> I8Vec4 {
        I8Vec4::new(self.x, self.x, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn xxzx(self) -> I8Vec4 {
        I8Vec4::new(self.x, self.x, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn xxzy(self) -> I8Vec4 {
        I8Vec4::new(self.x, self.x, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn xxzz(self) -> I8Vec4 {
        I8Vec4::new(self.x, self.x, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn xxzw(self) -> I8Vec4 {
        I8Vec4::new(self.x, self.x, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn xxwx(self) -> I8Vec4 {
        I8Vec4::new(self.x, self.x, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn xxwy(self) -> I8Vec4 {
        I8Vec4::new(self.x, self.x, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn xxwz(self) -> I8Vec4 {
        I8Vec4::new(self.x, self.x, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn xxww(self) -> I8Vec4 {
        I8Vec4::new(self.x, self.x, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn xyxx(self) -> I8Vec4 {
        I8Vec4::new(self.x, self.y, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn xyxy(self) -> I8Vec4 {
        I8Vec4::new(self.x, self.y, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn xyxz(self) -> I8Vec4 {
        I8Vec4::new(self.x, self.y, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn xyxw(self) -> I8Vec4 {
        I8Vec4::new(self.x, self.y, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn xyyx(self) -> I8Vec4 {
        I8Vec4::new(self.x, self.y, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn xyyy(self) -> I8Vec4 {
        I8Vec4::new(self.x, self.y, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn xyyz(self) -> I8Vec4 {
        I8Vec4::new(self.x, self.y, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn xyyw(self) -> I8Vec4 {
        I8Vec4::new(self.x, self.y, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn xyzx(self) -> I8Vec4 {
        I8Vec4::new(self.x, self.y, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn xyzy(self) -> I8Vec4 {
        I8Vec4::new(self.x, self.y, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn xyzz(self) -> I8Vec4 {
        I8Vec4::new(self.x, self.y, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn xywx(self) -> I8Vec4 {
        I8Vec4::new(self.x, self.y, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn xywy(self) -> I8Vec4 {
        I8Vec4::new(self.x, self.y, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn xywz(self) -> I8Vec4 {
        I8Vec4::new(self.x, self.y, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn xyww(self) -> I8Vec4 {
        I8Vec4::new(self.x, self.y, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn xzxx(self) -> I8Vec4 {
        I8Vec4::new(self.x, self.z, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn xzxy(self) -> I8Vec4 {
        I8Vec4::new(self.x, self.z, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn xzxz(self) -> I8Vec4 {
        I8Vec4::new(self.x, self.z, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn xzxw(self) -> I8Vec4 {
        I8Vec4::new(self.x, self.z, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn xzyx(self) -> I8Vec4 {
        I8Vec4::new(self.x, self.z, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn xzyy(self) -> I8Vec4 {
        I8Vec4::new(self.x, self.z, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn xzyz(self) -> I8Vec4 {
        I8Vec4::new(self.x, self.z, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn xzyw(self) -> I8Vec4 {
        I8Vec4::new(self.x, self.z, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn xzzx(self) -> I8Vec4 {
        I8Vec4::new(self.x, self.z, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn xzzy(self) -> I8Vec4 {
        I8Vec4::new(self.x, self.z, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn xzzz(self) -> I8Vec4 {
        I8Vec4::new(self.x, self.z, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn xzzw(self) -> I8Vec4 {
        I8Vec4::new(self.x, self.z, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn xzwx(self) -> I8Vec4 {
        I8Vec4::new(self.x, self.z, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn xzwy(self) -> I8Vec4 {
        I8Vec4::new(self.x, self.z, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn xzwz(self) -> I8Vec4 {
        I8Vec4::new(self.x, self.z, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn xzww(self) -> I8Vec4 {
        I8Vec4::new(self.x, self.z, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn xwxx(self) -> I8Vec4 {
        I8Vec4::new(self.x, self.w, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn xwxy(self) -> I8Vec4 {
        I8Vec4::new(self.x, self.w, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn xwxz(self) -> I8Vec4 {
        I8Vec4::new(self.x, self.w, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn xwxw(self) -> I8Vec4 {
        I8Vec4::new(self.x, self.w, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn xwyx(self) -> I8Vec4 {
        I8Vec4::new(self.x, self.w, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn xwyy(self) -> I8Vec4 {
        I8Vec4::new(self.x, self.w, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn xwyz(self) -> I8Vec4 {
        I8Vec4::new(self.x, self.w, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn xwyw(self) -> I8Vec4 {
        I8Vec4::new(self.x, self.w, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn xwzx(self) -> I8Vec4 {
        I8Vec4::new(self.x, self.w, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn xwzy(self) -> I8Vec4 {
        I8Vec4::new(self.x, self.w, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn xwzz(self) -> I8Vec4 {
        I8Vec4::new(self.x, self.w, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn xwzw(self) -> I8Vec4 {
        I8Vec4::new(self.x, self.w, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn xwwx(self) -> I8Vec4 {
        I8Vec4::new(self.x, self.w, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn xwwy(self) -> I8Vec4 {
        I8Vec4::new(self.x, self.w, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn xwwz(self) -> I8Vec4 {
        I8Vec4::new(self.x, self.w, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn xwww(self) -> I8Vec4 {
        I8Vec4::new(self.x, self.w, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn yxxx(self) -> I8Vec4 {
        I8Vec4::new(self.y, self.x, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn yxxy(self) -> I8Vec4 {
        I8Vec4::new(self.y, self.x, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn yxxz(self) -> I8Vec4 {
        I8Vec4::new(self.y, self.x, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn yxxw(self) -> I8Vec4 {
        I8Vec4::new(self.y, self.x, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn yxyx(self) -> I8Vec4 {
        I8Vec4::new(self.y, self.x, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn yxyy(self) -> I8Vec4 {
        I8Vec4::new(self.y, self.x, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn yxyz(self) -> I8Vec4 {
        I8Vec4::new(self.y, self.x, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn yxyw(self) -> I8Vec4 {
        I8Vec4::new(self.y, self.x, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn yxzx(self) -> I8Vec4 {
        I8Vec4::new(self.y, self.x, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn yxzy(self) -> I8Vec4 {
        I8Vec4::new(self.y, self.x, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn yxzz(self) -> I8Vec4 {
        I8Vec4::new(self.y, self.x, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn yxzw(self) -> I8Vec4 {
        I8Vec4::new(self.y, self.x, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn yxwx(self) -> I8Vec4 {
        I8Vec4::new(self.y, self.x, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn yxwy(self) -> I8Vec4 {
        I8Vec4::new(self.y, self.x, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn yxwz(self) -> I8Vec4 {
        I8Vec4::new(self.y, self.x, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn yxww(self) -> I8Vec4 {
        I8Vec4::new(self.y, self.x, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn yyxx(self) -> I8Vec4 {
        I8Vec4::new(self.y, self.y, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn yyxy(self) -> I8Vec4 {
        I8Vec4::new(self.y, self.y, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn yyxz(self) -> I8Vec4 {
        I8Vec4::new(self.y, self.y, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn yyxw(self) -> I8Vec4 {
        I8Vec4::new(self.y, self.y, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn yyyx(self) -> I8Vec4 {
        I8Vec4::new(self.y, self.y, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn yyyy(self) -> I8Vec4 {
        I8Vec4::new(self.y, self.y, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn yyyz(self) -> I8Vec4 {
        I8Vec4::new(self.y, self.y, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn yyyw(self) -> I8Vec4 {
        I8Vec4::new(self.y, self.y, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn yyzx(self) -> I8Vec4 {
        I8Vec4::new(self.y, self.y, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn yyzy(self) -> I8Vec4 {
        I8Vec4::new(self.y, self.y, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn yyzz(self) -> I8Vec4 {
        I8Vec4::new(self.y, self.y, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn yyzw(self) -> I8Vec4 {
        I8Vec4::new(self.y, self.y, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn yywx(self) -> I8Vec4 {
        I8Vec4::new(self.y, self.y, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn yywy(self) -> I8Vec4 {
        I8Vec4::new(self.y, self.y, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn yywz(self) -> I8Vec4 {
        I8Vec4::new(self.y, self.y, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn yyww(self) -> I8Vec4 {
        I8Vec4::new(self.y, self.y, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn yzxx(self) -> I8Vec4 {
        I8Vec4::new(self.y, self.z, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn yzxy(self) -> I8Vec4 {
        I8Vec4::new(self.y, self.z, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn yzxz(self) -> I8Vec4 {
        I8Vec4::new(self.y, self.z, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn yzxw(self) -> I8Vec4 {
        I8Vec4::new(self.y, self.z, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn yzyx(self) -> I8Vec4 {
        I8Vec4::new(self.y, self.z, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn yzyy(self) -> I8Vec4 {
        I8Vec4::new(self.y, self.z, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn yzyz(self) -> I8Vec4 {
        I8Vec4::new(self.y, self.z, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn yzyw(self) -> I8Vec4 {
        I8Vec4::new(self.y, self.z, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn yzzx(self) -> I8Vec4 {
        I8Vec4::new(self.y, self.z, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn yzzy(self) -> I8Vec4 {
        I8Vec4::new(self.y, self.z, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn yzzz(self) -> I8Vec4 {
        I8Vec4::new(self.y, self.z, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn yzzw(self) -> I8Vec4 {
        I8Vec4::new(self.y, self.z, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn yzwx(self) -> I8Vec4 {
        I8Vec4::new(self.y, self.z, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn yzwy(self) -> I8Vec4 {
        I8Vec4::new(self.y, self.z, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn yzwz(self) -> I8Vec4 {
        I8Vec4::new(self.y, self.z, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn yzww(self) -> I8Vec4 {
        I8Vec4::new(self.y, self.z, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn ywxx(self) -> I8Vec4 {
        I8Vec4::new(self.y, self.w, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn ywxy(self) -> I8Vec4 {
        I8Vec4::new(self.y, self.w, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn ywxz(self) -> I8Vec4 {
        I8Vec4::new(self.y, self.w, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn ywxw(self) -> I8Vec4 {
        I8Vec4::new(self.y, self.w, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn ywyx(self) -> I8Vec4 {
        I8Vec4::new(self.y, self.w, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn ywyy(self) -> I8Vec4 {
        I8Vec4::new(self.y, self.w, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn ywyz(self) -> I8Vec4 {
        I8Vec4::new(self.y, self.w, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn ywyw(self) -> I8Vec4 {
        I8Vec4::new(self.y, self.w, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn ywzx(self) -> I8Vec4 {
        I8Vec4::new(self.y, self.w, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn ywzy(self) -> I8Vec4 {
        I8Vec4::new(self.y, self.w, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn ywzz(self) -> I8Vec4 {
        I8Vec4::new(self.y, self.w, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn ywzw(self) -> I8Vec4 {
        I8Vec4::new(self.y, self.w, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn ywwx(self) -> I8Vec4 {
        I8Vec4::new(self.y, self.w, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn ywwy(self) -> I8Vec4 {
        I8Vec4::new(self.y, self.w, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn ywwz(self) -> I8Vec4 {
        I8Vec4::new(self.y, self.w, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn ywww(self) -> I8Vec4 {
        I8Vec4::new(self.y, self.w, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn zxxx(self) -> I8Vec4 {
        I8Vec4::new(self.z, self.x, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn zxxy(self) -> I8Vec4 {
        I8Vec4::new(self.z, self.x, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn zxxz(self) -> I8Vec4 {
        I8Vec4::new(self.z, self.x, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn zxxw(self) -> I8Vec4 {
        I8Vec4::new(self.z, self.x, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn zxyx(self) -> I8Vec4 {
        I8Vec4::new(self.z, self.x, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn zxyy(self) -> I8Vec4 {
        I8Vec4::new(self.z, self.x, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn zxyz(self) -> I8Vec4 {
        I8Vec4::new(self.z, self.x, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn zxyw(self) -> I8Vec4 {
        I8Vec4::new(self.z, self.x, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn zxzx(self) -> I8Vec4 {
        I8Vec4::new(self.z, self.x, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn zxzy(self) -> I8Vec4 {
        I8Vec4::new(self.z, self.x, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn zxzz(self) -> I8Vec4 {
        I8Vec4::new(self.z, self.x, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn zxzw(self) -> I8Vec4 {
        I8Vec4::new(self.z, self.x, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn zxwx(self) -> I8Vec4 {
        I8Vec4::new(self.z, self.x, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn zxwy(self) -> I8Vec4 {
        I8Vec4::new(self.z, self.x, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn zxwz(self) -> I8Vec4 {
        I8Vec4::new(self.z, self.x, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn zxww(self) -> I8Vec4 {
        I8Vec4::new(self.z, self.x, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn zyxx(self) -> I8Vec4 {
        I8Vec4::new(self.z, self.y, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn zyxy(self) -> I8Vec4 {
        I8Vec4::new(self.z, self.y, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn zyxz(self) -> I8Vec4 {
        I8Vec4::new(self.z, self.y, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn zyxw(self) -> I8Vec4 {
        I8Vec4::new(self.z, self.y, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn zyyx(self) -> I8Vec4 {
        I8Vec4::new(self.z, self.y, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn zyyy(self) -> I8Vec4 {
        I8Vec4::new(self.z, self.y, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn zyyz(self) -> I8Vec4 {
        I8Vec4::new(self.z, self.y, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn zyyw(self) -> I8Vec4 {
        I8Vec4::new(self.z, self.y, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn zyzx(self) -> I8Vec4 {
        I8Vec4::new(self.z, self.y, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn zyzy(self) -> I8Vec4 {
        I8Vec4::new(self.z, self.y, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn zyzz(self) -> I8Vec4 {
        I8Vec4::new(self.z, self.y, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn zyzw(self) -> I8Vec4 {
        I8Vec4::new(self.z, self.y, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn zywx(self) -> I8Vec4 {
        I8Vec4::new(self.z, self.y, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn zywy(self) -> I8Vec4 {
        I8Vec4::new(self.z, self.y, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn zywz(self) -> I8Vec4 {
        I8Vec4::new(self.z, self.y, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn zyww(self) -> I8Vec4 {
        I8Vec4::new(self.z, self.y, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn zzxx(self) -> I8Vec4 {
        I8Vec4::new(self.z, self.z, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn zzxy(self) -> I8Vec4 {
        I8Vec4::new(self.z, self.z, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn zzxz(self) -> I8Vec4 {
        I8Vec4::new(self.z, self.z, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn zzxw(self) -> I8Vec4 {
        I8Vec4::new(self.z, self.z, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn zzyx(self) -> I8Vec4 {
        I8Vec4::new(self.z, self.z, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn zzyy(self) -> I8Vec4 {
        I8Vec4::new(self.z, self.z, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn zzyz(self) -> I8Vec4 {
        I8Vec4::new(self.z, self.z, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn zzyw(self) -> I8Vec4 {
        I8Vec4::new(self.z, self.z, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn zzzx(self) -> I8Vec4 {
        I8Vec4::new(self.z, self.z, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn zzzy(self) -> I8Vec4 {
        I8Vec4::new(self.z, self.z, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn zzzz(self) -> I8Vec4 {
        I8Vec4::new(self.z, self.z, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn zzzw(self) -> I8Vec4 {
        I8Vec4::new(self.z, self.z, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn zzwx(self) -> I8Vec4 {
        I8Vec4::new(self.z, self.z, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn zzwy(self) -> I8Vec4 {
        I8Vec4::new(self.z, self.z, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn zzwz(self) -> I8Vec4 {
        I8Vec4::new(self.z, self.z, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn zzww(self) -> I8Vec4 {
        I8Vec4::new(self.z, self.z, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn zwxx(self) -> I8Vec4 {
        I8Vec4::new(self.z, self.w, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn zwxy(self) -> I8Vec4 {
        I8Vec4::new(self.z, self.w, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn zwxz(self) -> I8Vec4 {
        I8Vec4::new(self.z, self.w, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn zwxw(self) -> I8Vec4 {
        I8Vec4::new(self.z, self.w, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn zwyx(self) -> I8Vec4 {
        I8Vec4::new(self.z, self.w, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn zwyy(self) -> I8Vec4 {
        I8Vec4::new(self.z, self.w, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn zwyz(self) -> I8Vec4 {
        I8Vec4::new(self.z, self.w, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn zwyw(self) -> I8Vec4 {
        I8Vec4::new(self.z, self.w, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn zwzx(self) -> I8Vec4 {
        I8Vec4::new(self.z, self.w, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn zwzy(self) -> I8Vec4 {
        I8Vec4::new(self.z, self.w, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn zwzz(self) -> I8Vec4 {
        I8Vec4::new(self.z, self.w, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn zwzw(self) -> I8Vec4 {
        I8Vec4::new(self.z, self.w, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn zwwx(self) -> I8Vec4 {
        I8Vec4::new(self.z, self.w, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn zwwy(self) -> I8Vec4 {
        I8Vec4::new(self.z, self.w, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn zwwz(self) -> I8Vec4 {
        I8Vec4::new(self.z, self.w, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn zwww(self) -> I8Vec4 {
        I8Vec4::new(self.z, self.w, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn wxxx(self) -> I8Vec4 {
        I8Vec4::new(self.w, self.x, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn wxxy(self) -> I8Vec4 {
        I8Vec4::new(self.w, self.x, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn wxxz(self) -> I8Vec4 {
        I8Vec4::new(self.w, self.x, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn wxxw(self) -> I8Vec4 {
        I8Vec4::new(self.w, self.x, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn wxyx(self) -> I8Vec4 {
        I8Vec4::new(self.w, self.x, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn wxyy(self) -> I8Vec4 {
        I8Vec4::new(self.w, self.x, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn wxyz(self) -> I8Vec4 {
        I8Vec4::new(self.w, self.x, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn wxyw(self) -> I8Vec4 {
        I8Vec4::new(self.w, self.x, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn wxzx(self) -> I8Vec4 {
        I8Vec4::new(self.w, self.x, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn wxzy(self) -> I8Vec4 {
        I8Vec4::new(self.w, self.x, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn wxzz(self) -> I8Vec4 {
        I8Vec4::new(self.w, self.x, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn wxzw(self) -> I8Vec4 {
        I8Vec4::new(self.w, self.x, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn wxwx(self) -> I8Vec4 {
        I8Vec4::new(self.w, self.x, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn wxwy(self) -> I8Vec4 {
        I8Vec4::new(self.w, self.x, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn wxwz(self) -> I8Vec4 {
        I8Vec4::new(self.w, self.x, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn wxww(self) -> I8Vec4 {
        I8Vec4::new(self.w, self.x, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn wyxx(self) -> I8Vec4 {
        I8Vec4::new(self.w, self.y, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn wyxy(self) -> I8Vec4 {
        I8Vec4::new(self.w, self.y, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn wyxz(self) -> I8Vec4 {
        I8Vec4::new(self.w, self.y, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn wyxw(self) -> I8Vec4 {
        I8Vec4::new(self.w, self.y, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn wyyx(self) -> I8Vec4 {
        I8Vec4::new(self.w, self.y, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn wyyy(self) -> I8Vec4 {
        I8Vec4::new(self.w, self.y, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn wyyz(self) -> I8Vec4 {
        I8Vec4::new(self.w, self.y, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn wyyw(self) -> I8Vec4 {
        I8Vec4::new(self.w, self.y, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn wyzx(self) -> I8Vec4 {
        I8Vec4::new(self.w, self.y, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn wyzy(self) -> I8Vec4 {
        I8Vec4::new(self.w, self.y, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn wyzz(self) -> I8Vec4 {
        I8Vec4::new(self.w, self.y, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn wyzw(self) -> I8Vec4 {
        I8Vec4::new(self.w, self.y, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn wywx(self) -> I8Vec4 {
        I8Vec4::new(self.w, self.y, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn wywy(self) -> I8Vec4 {
        I8Vec4::new(self.w, self.y, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn wywz(self) -> I8Vec4 {
        I8Vec4::new(self.w, self.y, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn wyww(self) -> I8Vec4 {
        I8Vec4::new(self.w, self.y, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn wzxx(self) -> I8Vec4 {
        I8Vec4::new(self.w, self.z, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn wzxy(self) -> I8Vec4 {
        I8Vec4::new(self.w, self.z, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn wzxz(self) -> I8Vec4 {
        I8Vec4::new(self.w, self.z, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn wzxw(self) -> I8Vec4 {
        I8Vec4::new(self.w, self.z, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn wzyx(self) -> I8Vec4 {
        I8Vec4::new(self.w, self.z, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn wzyy(self) -> I8Vec4 {
        I8Vec4::new(self.w, self.z, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn wzyz(self) -> I8Vec4 {
        I8Vec4::new(self.w, self.z, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn wzyw(self) -> I8Vec4 {
        I8Vec4::new(self.w, self.z, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn wzzx(self) -> I8Vec4 {
        I8Vec4::new(self.w, self.z, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn wzzy(self) -> I8Vec4 {
        I8Vec4::new(self.w, self.z, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn wzzz(self) -> I8Vec4 {
        I8Vec4::new(self.w, self.z, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn wzzw(self) -> I8Vec4 {
        I8Vec4::new(self.w, self.z, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn wzwx(self) -> I8Vec4 {
        I8Vec4::new(self.w, self.z, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn wzwy(self) -> I8Vec4 {
        I8Vec4::new(self.w, self.z, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn wzwz(self) -> I8Vec4 {
        I8Vec4::new(self.w, self.z, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn wzww(self) -> I8Vec4 {
        I8Vec4::new(self.w, self.z, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn wwxx(self) -> I8Vec4 {
        I8Vec4::new(self.w, self.w, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn wwxy(self) -> I8Vec4 {
        I8Vec4::new(self.w, self.w, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn wwxz(self) -> I8Vec4 {
        I8Vec4::new(self.w, self.w, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn wwxw(self) -> I8Vec4 {
        I8Vec4::new(self.w, self.w, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn wwyx(self) -> I8Vec4 {
        I8Vec4::new(self.w, self.w, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn wwyy(self) -> I8Vec4 {
        I8Vec4::new(self.w, self.w, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn wwyz(self) -> I8Vec4 {
        I8Vec4::new(self.w, self.w, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn wwyw(self) -> I8Vec4 {
        I8Vec4::new(self.w, self.w, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn wwzx(self) -> I8Vec4 {
        I8Vec4::new(self.w, self.w, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn wwzy(self) -> I8Vec4 {
        I8Vec4::new(self.w, self.w, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn wwzz(self) -> I8Vec4 {
        I8Vec4::new(self.w, self.w, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn wwzw(self) -> I8Vec4 {
        I8Vec4::new(self.w, self.w, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn wwwx(self) -> I8Vec4 {
        I8Vec4::new(self.w, self.w, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn wwwy(self) -> I8Vec4 {
        I8Vec4::new(self.w, self.w, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn wwwz(self) -> I8Vec4 {
        I8Vec4::new(self.w, self.w, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn wwww(self) -> I8Vec4 {
        I8Vec4::new(self.w, self.w, self.w, self.w)
    }
}
