// Generated from swizzle_impl.rs.tera template. Edit the template, not the generated file.

use crate::{I16Vec2, I16Vec3, I16Vec4, Vec4Swizzles};

impl Vec4Swizzles for I16Vec4 {
    type Vec2 = I16Vec2;

    type Vec3 = I16Vec3;

    #[inline]
    #[must_use]
    fn xx(self) -> I16Vec2 {
        I16Vec2 {
            x: self.x,
            y: self.x,
        }
    }

    #[inline]
    #[must_use]
    fn xy(self) -> I16Vec2 {
        I16Vec2 {
            x: self.x,
            y: self.y,
        }
    }

    #[inline]
    #[must_use]
    fn xz(self) -> I16Vec2 {
        I16Vec2 {
            x: self.x,
            y: self.z,
        }
    }

    #[inline]
    #[must_use]
    fn xw(self) -> I16Vec2 {
        I16Vec2 {
            x: self.x,
            y: self.w,
        }
    }

    #[inline]
    #[must_use]
    fn yx(self) -> I16Vec2 {
        I16Vec2 {
            x: self.y,
            y: self.x,
        }
    }

    #[inline]
    #[must_use]
    fn yy(self) -> I16Vec2 {
        I16Vec2 {
            x: self.y,
            y: self.y,
        }
    }

    #[inline]
    #[must_use]
    fn yz(self) -> I16Vec2 {
        I16Vec2 {
            x: self.y,
            y: self.z,
        }
    }

    #[inline]
    #[must_use]
    fn yw(self) -> I16Vec2 {
        I16Vec2 {
            x: self.y,
            y: self.w,
        }
    }

    #[inline]
    #[must_use]
    fn zx(self) -> I16Vec2 {
        I16Vec2 {
            x: self.z,
            y: self.x,
        }
    }

    #[inline]
    #[must_use]
    fn zy(self) -> I16Vec2 {
        I16Vec2 {
            x: self.z,
            y: self.y,
        }
    }

    #[inline]
    #[must_use]
    fn zz(self) -> I16Vec2 {
        I16Vec2 {
            x: self.z,
            y: self.z,
        }
    }

    #[inline]
    #[must_use]
    fn zw(self) -> I16Vec2 {
        I16Vec2 {
            x: self.z,
            y: self.w,
        }
    }

    #[inline]
    #[must_use]
    fn wx(self) -> I16Vec2 {
        I16Vec2 {
            x: self.w,
            y: self.x,
        }
    }

    #[inline]
    #[must_use]
    fn wy(self) -> I16Vec2 {
        I16Vec2 {
            x: self.w,
            y: self.y,
        }
    }

    #[inline]
    #[must_use]
    fn wz(self) -> I16Vec2 {
        I16Vec2 {
            x: self.w,
            y: self.z,
        }
    }

    #[inline]
    #[must_use]
    fn ww(self) -> I16Vec2 {
        I16Vec2 {
            x: self.w,
            y: self.w,
        }
    }

    #[inline]
    #[must_use]
    fn xxx(self) -> I16Vec3 {
        I16Vec3::new(self.x, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn xxy(self) -> I16Vec3 {
        I16Vec3::new(self.x, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn xxz(self) -> I16Vec3 {
        I16Vec3::new(self.x, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn xxw(self) -> I16Vec3 {
        I16Vec3::new(self.x, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn xyx(self) -> I16Vec3 {
        I16Vec3::new(self.x, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn xyy(self) -> I16Vec3 {
        I16Vec3::new(self.x, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn xyz(self) -> I16Vec3 {
        I16Vec3::new(self.x, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn xyw(self) -> I16Vec3 {
        I16Vec3::new(self.x, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn xzx(self) -> I16Vec3 {
        I16Vec3::new(self.x, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn xzy(self) -> I16Vec3 {
        I16Vec3::new(self.x, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn xzz(self) -> I16Vec3 {
        I16Vec3::new(self.x, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn xzw(self) -> I16Vec3 {
        I16Vec3::new(self.x, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn xwx(self) -> I16Vec3 {
        I16Vec3::new(self.x, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn xwy(self) -> I16Vec3 {
        I16Vec3::new(self.x, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn xwz(self) -> I16Vec3 {
        I16Vec3::new(self.x, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn xww(self) -> I16Vec3 {
        I16Vec3::new(self.x, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn yxx(self) -> I16Vec3 {
        I16Vec3::new(self.y, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn yxy(self) -> I16Vec3 {
        I16Vec3::new(self.y, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn yxz(self) -> I16Vec3 {
        I16Vec3::new(self.y, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn yxw(self) -> I16Vec3 {
        I16Vec3::new(self.y, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn yyx(self) -> I16Vec3 {
        I16Vec3::new(self.y, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn yyy(self) -> I16Vec3 {
        I16Vec3::new(self.y, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn yyz(self) -> I16Vec3 {
        I16Vec3::new(self.y, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn yyw(self) -> I16Vec3 {
        I16Vec3::new(self.y, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn yzx(self) -> I16Vec3 {
        I16Vec3::new(self.y, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn yzy(self) -> I16Vec3 {
        I16Vec3::new(self.y, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn yzz(self) -> I16Vec3 {
        I16Vec3::new(self.y, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn yzw(self) -> I16Vec3 {
        I16Vec3::new(self.y, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn ywx(self) -> I16Vec3 {
        I16Vec3::new(self.y, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn ywy(self) -> I16Vec3 {
        I16Vec3::new(self.y, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn ywz(self) -> I16Vec3 {
        I16Vec3::new(self.y, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn yww(self) -> I16Vec3 {
        I16Vec3::new(self.y, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn zxx(self) -> I16Vec3 {
        I16Vec3::new(self.z, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn zxy(self) -> I16Vec3 {
        I16Vec3::new(self.z, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn zxz(self) -> I16Vec3 {
        I16Vec3::new(self.z, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn zxw(self) -> I16Vec3 {
        I16Vec3::new(self.z, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn zyx(self) -> I16Vec3 {
        I16Vec3::new(self.z, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn zyy(self) -> I16Vec3 {
        I16Vec3::new(self.z, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn zyz(self) -> I16Vec3 {
        I16Vec3::new(self.z, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn zyw(self) -> I16Vec3 {
        I16Vec3::new(self.z, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn zzx(self) -> I16Vec3 {
        I16Vec3::new(self.z, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn zzy(self) -> I16Vec3 {
        I16Vec3::new(self.z, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn zzz(self) -> I16Vec3 {
        I16Vec3::new(self.z, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn zzw(self) -> I16Vec3 {
        I16Vec3::new(self.z, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn zwx(self) -> I16Vec3 {
        I16Vec3::new(self.z, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn zwy(self) -> I16Vec3 {
        I16Vec3::new(self.z, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn zwz(self) -> I16Vec3 {
        I16Vec3::new(self.z, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn zww(self) -> I16Vec3 {
        I16Vec3::new(self.z, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn wxx(self) -> I16Vec3 {
        I16Vec3::new(self.w, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn wxy(self) -> I16Vec3 {
        I16Vec3::new(self.w, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn wxz(self) -> I16Vec3 {
        I16Vec3::new(self.w, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn wxw(self) -> I16Vec3 {
        I16Vec3::new(self.w, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn wyx(self) -> I16Vec3 {
        I16Vec3::new(self.w, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn wyy(self) -> I16Vec3 {
        I16Vec3::new(self.w, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn wyz(self) -> I16Vec3 {
        I16Vec3::new(self.w, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn wyw(self) -> I16Vec3 {
        I16Vec3::new(self.w, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn wzx(self) -> I16Vec3 {
        I16Vec3::new(self.w, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn wzy(self) -> I16Vec3 {
        I16Vec3::new(self.w, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn wzz(self) -> I16Vec3 {
        I16Vec3::new(self.w, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn wzw(self) -> I16Vec3 {
        I16Vec3::new(self.w, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn wwx(self) -> I16Vec3 {
        I16Vec3::new(self.w, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn wwy(self) -> I16Vec3 {
        I16Vec3::new(self.w, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn wwz(self) -> I16Vec3 {
        I16Vec3::new(self.w, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn www(self) -> I16Vec3 {
        I16Vec3::new(self.w, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn xxxx(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.x, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn xxxy(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.x, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn xxxz(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.x, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn xxxw(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.x, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn xxyx(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.x, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn xxyy(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.x, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn xxyz(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.x, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn xxyw(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.x, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn xxzx(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.x, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn xxzy(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.x, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn xxzz(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.x, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn xxzw(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.x, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn xxwx(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.x, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn xxwy(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.x, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn xxwz(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.x, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn xxww(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.x, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn xyxx(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.y, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn xyxy(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.y, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn xyxz(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.y, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn xyxw(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.y, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn xyyx(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.y, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn xyyy(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.y, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn xyyz(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.y, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn xyyw(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.y, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn xyzx(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.y, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn xyzy(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.y, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn xyzz(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.y, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn xyzw(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.y, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn xywx(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.y, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn xywy(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.y, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn xywz(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.y, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn xyww(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.y, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn xzxx(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.z, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn xzxy(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.z, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn xzxz(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.z, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn xzxw(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.z, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn xzyx(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.z, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn xzyy(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.z, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn xzyz(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.z, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn xzyw(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.z, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn xzzx(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.z, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn xzzy(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.z, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn xzzz(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.z, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn xzzw(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.z, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn xzwx(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.z, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn xzwy(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.z, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn xzwz(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.z, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn xzww(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.z, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn xwxx(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.w, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn xwxy(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.w, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn xwxz(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.w, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn xwxw(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.w, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn xwyx(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.w, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn xwyy(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.w, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn xwyz(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.w, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn xwyw(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.w, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn xwzx(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.w, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn xwzy(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.w, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn xwzz(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.w, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn xwzw(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.w, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn xwwx(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.w, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn xwwy(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.w, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn xwwz(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.w, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn xwww(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.w, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn yxxx(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.x, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn yxxy(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.x, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn yxxz(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.x, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn yxxw(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.x, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn yxyx(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.x, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn yxyy(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.x, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn yxyz(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.x, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn yxyw(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.x, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn yxzx(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.x, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn yxzy(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.x, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn yxzz(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.x, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn yxzw(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.x, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn yxwx(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.x, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn yxwy(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.x, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn yxwz(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.x, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn yxww(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.x, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn yyxx(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.y, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn yyxy(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.y, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn yyxz(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.y, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn yyxw(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.y, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn yyyx(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.y, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn yyyy(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.y, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn yyyz(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.y, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn yyyw(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.y, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn yyzx(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.y, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn yyzy(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.y, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn yyzz(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.y, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn yyzw(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.y, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn yywx(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.y, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn yywy(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.y, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn yywz(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.y, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn yyww(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.y, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn yzxx(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.z, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn yzxy(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.z, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn yzxz(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.z, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn yzxw(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.z, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn yzyx(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.z, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn yzyy(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.z, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn yzyz(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.z, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn yzyw(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.z, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn yzzx(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.z, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn yzzy(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.z, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn yzzz(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.z, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn yzzw(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.z, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn yzwx(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.z, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn yzwy(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.z, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn yzwz(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.z, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn yzww(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.z, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn ywxx(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.w, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn ywxy(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.w, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn ywxz(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.w, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn ywxw(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.w, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn ywyx(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.w, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn ywyy(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.w, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn ywyz(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.w, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn ywyw(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.w, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn ywzx(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.w, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn ywzy(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.w, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn ywzz(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.w, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn ywzw(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.w, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn ywwx(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.w, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn ywwy(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.w, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn ywwz(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.w, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn ywww(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.w, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn zxxx(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.x, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn zxxy(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.x, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn zxxz(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.x, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn zxxw(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.x, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn zxyx(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.x, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn zxyy(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.x, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn zxyz(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.x, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn zxyw(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.x, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn zxzx(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.x, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn zxzy(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.x, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn zxzz(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.x, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn zxzw(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.x, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn zxwx(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.x, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn zxwy(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.x, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn zxwz(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.x, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn zxww(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.x, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn zyxx(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.y, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn zyxy(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.y, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn zyxz(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.y, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn zyxw(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.y, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn zyyx(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.y, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn zyyy(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.y, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn zyyz(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.y, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn zyyw(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.y, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn zyzx(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.y, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn zyzy(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.y, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn zyzz(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.y, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn zyzw(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.y, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn zywx(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.y, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn zywy(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.y, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn zywz(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.y, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn zyww(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.y, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn zzxx(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.z, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn zzxy(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.z, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn zzxz(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.z, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn zzxw(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.z, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn zzyx(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.z, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn zzyy(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.z, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn zzyz(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.z, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn zzyw(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.z, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn zzzx(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.z, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn zzzy(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.z, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn zzzz(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.z, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn zzzw(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.z, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn zzwx(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.z, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn zzwy(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.z, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn zzwz(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.z, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn zzww(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.z, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn zwxx(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.w, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn zwxy(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.w, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn zwxz(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.w, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn zwxw(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.w, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn zwyx(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.w, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn zwyy(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.w, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn zwyz(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.w, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn zwyw(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.w, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn zwzx(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.w, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn zwzy(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.w, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn zwzz(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.w, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn zwzw(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.w, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn zwwx(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.w, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn zwwy(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.w, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn zwwz(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.w, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn zwww(self) -> I16Vec4 {
        I16Vec4::new(self.z, self.w, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn wxxx(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.x, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn wxxy(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.x, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn wxxz(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.x, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn wxxw(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.x, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn wxyx(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.x, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn wxyy(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.x, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn wxyz(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.x, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn wxyw(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.x, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn wxzx(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.x, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn wxzy(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.x, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn wxzz(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.x, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn wxzw(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.x, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn wxwx(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.x, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn wxwy(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.x, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn wxwz(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.x, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn wxww(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.x, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn wyxx(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.y, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn wyxy(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.y, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn wyxz(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.y, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn wyxw(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.y, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn wyyx(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.y, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn wyyy(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.y, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn wyyz(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.y, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn wyyw(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.y, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn wyzx(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.y, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn wyzy(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.y, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn wyzz(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.y, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn wyzw(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.y, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn wywx(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.y, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn wywy(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.y, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn wywz(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.y, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn wyww(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.y, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn wzxx(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.z, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn wzxy(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.z, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn wzxz(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.z, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn wzxw(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.z, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn wzyx(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.z, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn wzyy(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.z, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn wzyz(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.z, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn wzyw(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.z, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn wzzx(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.z, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn wzzy(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.z, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn wzzz(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.z, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn wzzw(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.z, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn wzwx(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.z, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn wzwy(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.z, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn wzwz(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.z, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn wzww(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.z, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn wwxx(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.w, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn wwxy(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.w, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn wwxz(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.w, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn wwxw(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.w, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn wwyx(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.w, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn wwyy(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.w, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn wwyz(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.w, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn wwyw(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.w, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn wwzx(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.w, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn wwzy(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.w, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn wwzz(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.w, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn wwzw(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.w, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn wwwx(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.w, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn wwwy(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.w, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn wwwz(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.w, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn wwww(self) -> I16Vec4 {
        I16Vec4::new(self.w, self.w, self.w, self.w)
    }
}
