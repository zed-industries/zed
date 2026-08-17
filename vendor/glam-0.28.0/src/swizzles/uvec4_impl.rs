// Generated from swizzle_impl.rs.tera template. Edit the template, not the generated file.

use crate::{UVec2, UVec3, UVec4, Vec4Swizzles};

impl Vec4Swizzles for UVec4 {
    type Vec2 = UVec2;

    type Vec3 = UVec3;

    #[inline]
    #[must_use]
    fn xx(self) -> UVec2 {
        UVec2 {
            x: self.x,
            y: self.x,
        }
    }

    #[inline]
    #[must_use]
    fn xy(self) -> UVec2 {
        UVec2 {
            x: self.x,
            y: self.y,
        }
    }

    #[inline]
    #[must_use]
    fn xz(self) -> UVec2 {
        UVec2 {
            x: self.x,
            y: self.z,
        }
    }

    #[inline]
    #[must_use]
    fn xw(self) -> UVec2 {
        UVec2 {
            x: self.x,
            y: self.w,
        }
    }

    #[inline]
    #[must_use]
    fn yx(self) -> UVec2 {
        UVec2 {
            x: self.y,
            y: self.x,
        }
    }

    #[inline]
    #[must_use]
    fn yy(self) -> UVec2 {
        UVec2 {
            x: self.y,
            y: self.y,
        }
    }

    #[inline]
    #[must_use]
    fn yz(self) -> UVec2 {
        UVec2 {
            x: self.y,
            y: self.z,
        }
    }

    #[inline]
    #[must_use]
    fn yw(self) -> UVec2 {
        UVec2 {
            x: self.y,
            y: self.w,
        }
    }

    #[inline]
    #[must_use]
    fn zx(self) -> UVec2 {
        UVec2 {
            x: self.z,
            y: self.x,
        }
    }

    #[inline]
    #[must_use]
    fn zy(self) -> UVec2 {
        UVec2 {
            x: self.z,
            y: self.y,
        }
    }

    #[inline]
    #[must_use]
    fn zz(self) -> UVec2 {
        UVec2 {
            x: self.z,
            y: self.z,
        }
    }

    #[inline]
    #[must_use]
    fn zw(self) -> UVec2 {
        UVec2 {
            x: self.z,
            y: self.w,
        }
    }

    #[inline]
    #[must_use]
    fn wx(self) -> UVec2 {
        UVec2 {
            x: self.w,
            y: self.x,
        }
    }

    #[inline]
    #[must_use]
    fn wy(self) -> UVec2 {
        UVec2 {
            x: self.w,
            y: self.y,
        }
    }

    #[inline]
    #[must_use]
    fn wz(self) -> UVec2 {
        UVec2 {
            x: self.w,
            y: self.z,
        }
    }

    #[inline]
    #[must_use]
    fn ww(self) -> UVec2 {
        UVec2 {
            x: self.w,
            y: self.w,
        }
    }

    #[inline]
    #[must_use]
    fn xxx(self) -> UVec3 {
        UVec3::new(self.x, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn xxy(self) -> UVec3 {
        UVec3::new(self.x, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn xxz(self) -> UVec3 {
        UVec3::new(self.x, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn xxw(self) -> UVec3 {
        UVec3::new(self.x, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn xyx(self) -> UVec3 {
        UVec3::new(self.x, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn xyy(self) -> UVec3 {
        UVec3::new(self.x, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn xyz(self) -> UVec3 {
        UVec3::new(self.x, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn xyw(self) -> UVec3 {
        UVec3::new(self.x, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn xzx(self) -> UVec3 {
        UVec3::new(self.x, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn xzy(self) -> UVec3 {
        UVec3::new(self.x, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn xzz(self) -> UVec3 {
        UVec3::new(self.x, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn xzw(self) -> UVec3 {
        UVec3::new(self.x, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn xwx(self) -> UVec3 {
        UVec3::new(self.x, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn xwy(self) -> UVec3 {
        UVec3::new(self.x, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn xwz(self) -> UVec3 {
        UVec3::new(self.x, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn xww(self) -> UVec3 {
        UVec3::new(self.x, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn yxx(self) -> UVec3 {
        UVec3::new(self.y, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn yxy(self) -> UVec3 {
        UVec3::new(self.y, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn yxz(self) -> UVec3 {
        UVec3::new(self.y, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn yxw(self) -> UVec3 {
        UVec3::new(self.y, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn yyx(self) -> UVec3 {
        UVec3::new(self.y, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn yyy(self) -> UVec3 {
        UVec3::new(self.y, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn yyz(self) -> UVec3 {
        UVec3::new(self.y, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn yyw(self) -> UVec3 {
        UVec3::new(self.y, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn yzx(self) -> UVec3 {
        UVec3::new(self.y, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn yzy(self) -> UVec3 {
        UVec3::new(self.y, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn yzz(self) -> UVec3 {
        UVec3::new(self.y, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn yzw(self) -> UVec3 {
        UVec3::new(self.y, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn ywx(self) -> UVec3 {
        UVec3::new(self.y, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn ywy(self) -> UVec3 {
        UVec3::new(self.y, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn ywz(self) -> UVec3 {
        UVec3::new(self.y, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn yww(self) -> UVec3 {
        UVec3::new(self.y, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn zxx(self) -> UVec3 {
        UVec3::new(self.z, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn zxy(self) -> UVec3 {
        UVec3::new(self.z, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn zxz(self) -> UVec3 {
        UVec3::new(self.z, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn zxw(self) -> UVec3 {
        UVec3::new(self.z, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn zyx(self) -> UVec3 {
        UVec3::new(self.z, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn zyy(self) -> UVec3 {
        UVec3::new(self.z, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn zyz(self) -> UVec3 {
        UVec3::new(self.z, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn zyw(self) -> UVec3 {
        UVec3::new(self.z, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn zzx(self) -> UVec3 {
        UVec3::new(self.z, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn zzy(self) -> UVec3 {
        UVec3::new(self.z, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn zzz(self) -> UVec3 {
        UVec3::new(self.z, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn zzw(self) -> UVec3 {
        UVec3::new(self.z, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn zwx(self) -> UVec3 {
        UVec3::new(self.z, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn zwy(self) -> UVec3 {
        UVec3::new(self.z, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn zwz(self) -> UVec3 {
        UVec3::new(self.z, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn zww(self) -> UVec3 {
        UVec3::new(self.z, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn wxx(self) -> UVec3 {
        UVec3::new(self.w, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn wxy(self) -> UVec3 {
        UVec3::new(self.w, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn wxz(self) -> UVec3 {
        UVec3::new(self.w, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn wxw(self) -> UVec3 {
        UVec3::new(self.w, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn wyx(self) -> UVec3 {
        UVec3::new(self.w, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn wyy(self) -> UVec3 {
        UVec3::new(self.w, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn wyz(self) -> UVec3 {
        UVec3::new(self.w, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn wyw(self) -> UVec3 {
        UVec3::new(self.w, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn wzx(self) -> UVec3 {
        UVec3::new(self.w, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn wzy(self) -> UVec3 {
        UVec3::new(self.w, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn wzz(self) -> UVec3 {
        UVec3::new(self.w, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn wzw(self) -> UVec3 {
        UVec3::new(self.w, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn wwx(self) -> UVec3 {
        UVec3::new(self.w, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn wwy(self) -> UVec3 {
        UVec3::new(self.w, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn wwz(self) -> UVec3 {
        UVec3::new(self.w, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn www(self) -> UVec3 {
        UVec3::new(self.w, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn xxxx(self) -> UVec4 {
        UVec4::new(self.x, self.x, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn xxxy(self) -> UVec4 {
        UVec4::new(self.x, self.x, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn xxxz(self) -> UVec4 {
        UVec4::new(self.x, self.x, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn xxxw(self) -> UVec4 {
        UVec4::new(self.x, self.x, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn xxyx(self) -> UVec4 {
        UVec4::new(self.x, self.x, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn xxyy(self) -> UVec4 {
        UVec4::new(self.x, self.x, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn xxyz(self) -> UVec4 {
        UVec4::new(self.x, self.x, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn xxyw(self) -> UVec4 {
        UVec4::new(self.x, self.x, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn xxzx(self) -> UVec4 {
        UVec4::new(self.x, self.x, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn xxzy(self) -> UVec4 {
        UVec4::new(self.x, self.x, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn xxzz(self) -> UVec4 {
        UVec4::new(self.x, self.x, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn xxzw(self) -> UVec4 {
        UVec4::new(self.x, self.x, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn xxwx(self) -> UVec4 {
        UVec4::new(self.x, self.x, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn xxwy(self) -> UVec4 {
        UVec4::new(self.x, self.x, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn xxwz(self) -> UVec4 {
        UVec4::new(self.x, self.x, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn xxww(self) -> UVec4 {
        UVec4::new(self.x, self.x, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn xyxx(self) -> UVec4 {
        UVec4::new(self.x, self.y, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn xyxy(self) -> UVec4 {
        UVec4::new(self.x, self.y, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn xyxz(self) -> UVec4 {
        UVec4::new(self.x, self.y, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn xyxw(self) -> UVec4 {
        UVec4::new(self.x, self.y, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn xyyx(self) -> UVec4 {
        UVec4::new(self.x, self.y, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn xyyy(self) -> UVec4 {
        UVec4::new(self.x, self.y, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn xyyz(self) -> UVec4 {
        UVec4::new(self.x, self.y, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn xyyw(self) -> UVec4 {
        UVec4::new(self.x, self.y, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn xyzx(self) -> UVec4 {
        UVec4::new(self.x, self.y, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn xyzy(self) -> UVec4 {
        UVec4::new(self.x, self.y, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn xyzz(self) -> UVec4 {
        UVec4::new(self.x, self.y, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn xyzw(self) -> UVec4 {
        UVec4::new(self.x, self.y, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn xywx(self) -> UVec4 {
        UVec4::new(self.x, self.y, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn xywy(self) -> UVec4 {
        UVec4::new(self.x, self.y, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn xywz(self) -> UVec4 {
        UVec4::new(self.x, self.y, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn xyww(self) -> UVec4 {
        UVec4::new(self.x, self.y, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn xzxx(self) -> UVec4 {
        UVec4::new(self.x, self.z, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn xzxy(self) -> UVec4 {
        UVec4::new(self.x, self.z, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn xzxz(self) -> UVec4 {
        UVec4::new(self.x, self.z, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn xzxw(self) -> UVec4 {
        UVec4::new(self.x, self.z, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn xzyx(self) -> UVec4 {
        UVec4::new(self.x, self.z, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn xzyy(self) -> UVec4 {
        UVec4::new(self.x, self.z, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn xzyz(self) -> UVec4 {
        UVec4::new(self.x, self.z, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn xzyw(self) -> UVec4 {
        UVec4::new(self.x, self.z, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn xzzx(self) -> UVec4 {
        UVec4::new(self.x, self.z, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn xzzy(self) -> UVec4 {
        UVec4::new(self.x, self.z, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn xzzz(self) -> UVec4 {
        UVec4::new(self.x, self.z, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn xzzw(self) -> UVec4 {
        UVec4::new(self.x, self.z, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn xzwx(self) -> UVec4 {
        UVec4::new(self.x, self.z, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn xzwy(self) -> UVec4 {
        UVec4::new(self.x, self.z, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn xzwz(self) -> UVec4 {
        UVec4::new(self.x, self.z, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn xzww(self) -> UVec4 {
        UVec4::new(self.x, self.z, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn xwxx(self) -> UVec4 {
        UVec4::new(self.x, self.w, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn xwxy(self) -> UVec4 {
        UVec4::new(self.x, self.w, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn xwxz(self) -> UVec4 {
        UVec4::new(self.x, self.w, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn xwxw(self) -> UVec4 {
        UVec4::new(self.x, self.w, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn xwyx(self) -> UVec4 {
        UVec4::new(self.x, self.w, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn xwyy(self) -> UVec4 {
        UVec4::new(self.x, self.w, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn xwyz(self) -> UVec4 {
        UVec4::new(self.x, self.w, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn xwyw(self) -> UVec4 {
        UVec4::new(self.x, self.w, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn xwzx(self) -> UVec4 {
        UVec4::new(self.x, self.w, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn xwzy(self) -> UVec4 {
        UVec4::new(self.x, self.w, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn xwzz(self) -> UVec4 {
        UVec4::new(self.x, self.w, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn xwzw(self) -> UVec4 {
        UVec4::new(self.x, self.w, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn xwwx(self) -> UVec4 {
        UVec4::new(self.x, self.w, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn xwwy(self) -> UVec4 {
        UVec4::new(self.x, self.w, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn xwwz(self) -> UVec4 {
        UVec4::new(self.x, self.w, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn xwww(self) -> UVec4 {
        UVec4::new(self.x, self.w, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn yxxx(self) -> UVec4 {
        UVec4::new(self.y, self.x, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn yxxy(self) -> UVec4 {
        UVec4::new(self.y, self.x, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn yxxz(self) -> UVec4 {
        UVec4::new(self.y, self.x, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn yxxw(self) -> UVec4 {
        UVec4::new(self.y, self.x, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn yxyx(self) -> UVec4 {
        UVec4::new(self.y, self.x, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn yxyy(self) -> UVec4 {
        UVec4::new(self.y, self.x, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn yxyz(self) -> UVec4 {
        UVec4::new(self.y, self.x, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn yxyw(self) -> UVec4 {
        UVec4::new(self.y, self.x, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn yxzx(self) -> UVec4 {
        UVec4::new(self.y, self.x, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn yxzy(self) -> UVec4 {
        UVec4::new(self.y, self.x, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn yxzz(self) -> UVec4 {
        UVec4::new(self.y, self.x, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn yxzw(self) -> UVec4 {
        UVec4::new(self.y, self.x, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn yxwx(self) -> UVec4 {
        UVec4::new(self.y, self.x, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn yxwy(self) -> UVec4 {
        UVec4::new(self.y, self.x, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn yxwz(self) -> UVec4 {
        UVec4::new(self.y, self.x, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn yxww(self) -> UVec4 {
        UVec4::new(self.y, self.x, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn yyxx(self) -> UVec4 {
        UVec4::new(self.y, self.y, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn yyxy(self) -> UVec4 {
        UVec4::new(self.y, self.y, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn yyxz(self) -> UVec4 {
        UVec4::new(self.y, self.y, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn yyxw(self) -> UVec4 {
        UVec4::new(self.y, self.y, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn yyyx(self) -> UVec4 {
        UVec4::new(self.y, self.y, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn yyyy(self) -> UVec4 {
        UVec4::new(self.y, self.y, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn yyyz(self) -> UVec4 {
        UVec4::new(self.y, self.y, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn yyyw(self) -> UVec4 {
        UVec4::new(self.y, self.y, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn yyzx(self) -> UVec4 {
        UVec4::new(self.y, self.y, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn yyzy(self) -> UVec4 {
        UVec4::new(self.y, self.y, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn yyzz(self) -> UVec4 {
        UVec4::new(self.y, self.y, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn yyzw(self) -> UVec4 {
        UVec4::new(self.y, self.y, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn yywx(self) -> UVec4 {
        UVec4::new(self.y, self.y, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn yywy(self) -> UVec4 {
        UVec4::new(self.y, self.y, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn yywz(self) -> UVec4 {
        UVec4::new(self.y, self.y, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn yyww(self) -> UVec4 {
        UVec4::new(self.y, self.y, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn yzxx(self) -> UVec4 {
        UVec4::new(self.y, self.z, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn yzxy(self) -> UVec4 {
        UVec4::new(self.y, self.z, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn yzxz(self) -> UVec4 {
        UVec4::new(self.y, self.z, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn yzxw(self) -> UVec4 {
        UVec4::new(self.y, self.z, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn yzyx(self) -> UVec4 {
        UVec4::new(self.y, self.z, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn yzyy(self) -> UVec4 {
        UVec4::new(self.y, self.z, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn yzyz(self) -> UVec4 {
        UVec4::new(self.y, self.z, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn yzyw(self) -> UVec4 {
        UVec4::new(self.y, self.z, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn yzzx(self) -> UVec4 {
        UVec4::new(self.y, self.z, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn yzzy(self) -> UVec4 {
        UVec4::new(self.y, self.z, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn yzzz(self) -> UVec4 {
        UVec4::new(self.y, self.z, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn yzzw(self) -> UVec4 {
        UVec4::new(self.y, self.z, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn yzwx(self) -> UVec4 {
        UVec4::new(self.y, self.z, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn yzwy(self) -> UVec4 {
        UVec4::new(self.y, self.z, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn yzwz(self) -> UVec4 {
        UVec4::new(self.y, self.z, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn yzww(self) -> UVec4 {
        UVec4::new(self.y, self.z, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn ywxx(self) -> UVec4 {
        UVec4::new(self.y, self.w, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn ywxy(self) -> UVec4 {
        UVec4::new(self.y, self.w, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn ywxz(self) -> UVec4 {
        UVec4::new(self.y, self.w, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn ywxw(self) -> UVec4 {
        UVec4::new(self.y, self.w, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn ywyx(self) -> UVec4 {
        UVec4::new(self.y, self.w, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn ywyy(self) -> UVec4 {
        UVec4::new(self.y, self.w, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn ywyz(self) -> UVec4 {
        UVec4::new(self.y, self.w, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn ywyw(self) -> UVec4 {
        UVec4::new(self.y, self.w, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn ywzx(self) -> UVec4 {
        UVec4::new(self.y, self.w, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn ywzy(self) -> UVec4 {
        UVec4::new(self.y, self.w, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn ywzz(self) -> UVec4 {
        UVec4::new(self.y, self.w, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn ywzw(self) -> UVec4 {
        UVec4::new(self.y, self.w, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn ywwx(self) -> UVec4 {
        UVec4::new(self.y, self.w, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn ywwy(self) -> UVec4 {
        UVec4::new(self.y, self.w, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn ywwz(self) -> UVec4 {
        UVec4::new(self.y, self.w, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn ywww(self) -> UVec4 {
        UVec4::new(self.y, self.w, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn zxxx(self) -> UVec4 {
        UVec4::new(self.z, self.x, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn zxxy(self) -> UVec4 {
        UVec4::new(self.z, self.x, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn zxxz(self) -> UVec4 {
        UVec4::new(self.z, self.x, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn zxxw(self) -> UVec4 {
        UVec4::new(self.z, self.x, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn zxyx(self) -> UVec4 {
        UVec4::new(self.z, self.x, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn zxyy(self) -> UVec4 {
        UVec4::new(self.z, self.x, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn zxyz(self) -> UVec4 {
        UVec4::new(self.z, self.x, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn zxyw(self) -> UVec4 {
        UVec4::new(self.z, self.x, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn zxzx(self) -> UVec4 {
        UVec4::new(self.z, self.x, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn zxzy(self) -> UVec4 {
        UVec4::new(self.z, self.x, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn zxzz(self) -> UVec4 {
        UVec4::new(self.z, self.x, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn zxzw(self) -> UVec4 {
        UVec4::new(self.z, self.x, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn zxwx(self) -> UVec4 {
        UVec4::new(self.z, self.x, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn zxwy(self) -> UVec4 {
        UVec4::new(self.z, self.x, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn zxwz(self) -> UVec4 {
        UVec4::new(self.z, self.x, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn zxww(self) -> UVec4 {
        UVec4::new(self.z, self.x, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn zyxx(self) -> UVec4 {
        UVec4::new(self.z, self.y, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn zyxy(self) -> UVec4 {
        UVec4::new(self.z, self.y, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn zyxz(self) -> UVec4 {
        UVec4::new(self.z, self.y, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn zyxw(self) -> UVec4 {
        UVec4::new(self.z, self.y, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn zyyx(self) -> UVec4 {
        UVec4::new(self.z, self.y, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn zyyy(self) -> UVec4 {
        UVec4::new(self.z, self.y, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn zyyz(self) -> UVec4 {
        UVec4::new(self.z, self.y, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn zyyw(self) -> UVec4 {
        UVec4::new(self.z, self.y, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn zyzx(self) -> UVec4 {
        UVec4::new(self.z, self.y, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn zyzy(self) -> UVec4 {
        UVec4::new(self.z, self.y, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn zyzz(self) -> UVec4 {
        UVec4::new(self.z, self.y, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn zyzw(self) -> UVec4 {
        UVec4::new(self.z, self.y, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn zywx(self) -> UVec4 {
        UVec4::new(self.z, self.y, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn zywy(self) -> UVec4 {
        UVec4::new(self.z, self.y, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn zywz(self) -> UVec4 {
        UVec4::new(self.z, self.y, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn zyww(self) -> UVec4 {
        UVec4::new(self.z, self.y, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn zzxx(self) -> UVec4 {
        UVec4::new(self.z, self.z, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn zzxy(self) -> UVec4 {
        UVec4::new(self.z, self.z, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn zzxz(self) -> UVec4 {
        UVec4::new(self.z, self.z, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn zzxw(self) -> UVec4 {
        UVec4::new(self.z, self.z, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn zzyx(self) -> UVec4 {
        UVec4::new(self.z, self.z, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn zzyy(self) -> UVec4 {
        UVec4::new(self.z, self.z, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn zzyz(self) -> UVec4 {
        UVec4::new(self.z, self.z, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn zzyw(self) -> UVec4 {
        UVec4::new(self.z, self.z, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn zzzx(self) -> UVec4 {
        UVec4::new(self.z, self.z, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn zzzy(self) -> UVec4 {
        UVec4::new(self.z, self.z, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn zzzz(self) -> UVec4 {
        UVec4::new(self.z, self.z, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn zzzw(self) -> UVec4 {
        UVec4::new(self.z, self.z, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn zzwx(self) -> UVec4 {
        UVec4::new(self.z, self.z, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn zzwy(self) -> UVec4 {
        UVec4::new(self.z, self.z, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn zzwz(self) -> UVec4 {
        UVec4::new(self.z, self.z, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn zzww(self) -> UVec4 {
        UVec4::new(self.z, self.z, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn zwxx(self) -> UVec4 {
        UVec4::new(self.z, self.w, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn zwxy(self) -> UVec4 {
        UVec4::new(self.z, self.w, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn zwxz(self) -> UVec4 {
        UVec4::new(self.z, self.w, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn zwxw(self) -> UVec4 {
        UVec4::new(self.z, self.w, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn zwyx(self) -> UVec4 {
        UVec4::new(self.z, self.w, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn zwyy(self) -> UVec4 {
        UVec4::new(self.z, self.w, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn zwyz(self) -> UVec4 {
        UVec4::new(self.z, self.w, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn zwyw(self) -> UVec4 {
        UVec4::new(self.z, self.w, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn zwzx(self) -> UVec4 {
        UVec4::new(self.z, self.w, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn zwzy(self) -> UVec4 {
        UVec4::new(self.z, self.w, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn zwzz(self) -> UVec4 {
        UVec4::new(self.z, self.w, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn zwzw(self) -> UVec4 {
        UVec4::new(self.z, self.w, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn zwwx(self) -> UVec4 {
        UVec4::new(self.z, self.w, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn zwwy(self) -> UVec4 {
        UVec4::new(self.z, self.w, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn zwwz(self) -> UVec4 {
        UVec4::new(self.z, self.w, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn zwww(self) -> UVec4 {
        UVec4::new(self.z, self.w, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn wxxx(self) -> UVec4 {
        UVec4::new(self.w, self.x, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn wxxy(self) -> UVec4 {
        UVec4::new(self.w, self.x, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn wxxz(self) -> UVec4 {
        UVec4::new(self.w, self.x, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn wxxw(self) -> UVec4 {
        UVec4::new(self.w, self.x, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn wxyx(self) -> UVec4 {
        UVec4::new(self.w, self.x, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn wxyy(self) -> UVec4 {
        UVec4::new(self.w, self.x, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn wxyz(self) -> UVec4 {
        UVec4::new(self.w, self.x, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn wxyw(self) -> UVec4 {
        UVec4::new(self.w, self.x, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn wxzx(self) -> UVec4 {
        UVec4::new(self.w, self.x, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn wxzy(self) -> UVec4 {
        UVec4::new(self.w, self.x, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn wxzz(self) -> UVec4 {
        UVec4::new(self.w, self.x, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn wxzw(self) -> UVec4 {
        UVec4::new(self.w, self.x, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn wxwx(self) -> UVec4 {
        UVec4::new(self.w, self.x, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn wxwy(self) -> UVec4 {
        UVec4::new(self.w, self.x, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn wxwz(self) -> UVec4 {
        UVec4::new(self.w, self.x, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn wxww(self) -> UVec4 {
        UVec4::new(self.w, self.x, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn wyxx(self) -> UVec4 {
        UVec4::new(self.w, self.y, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn wyxy(self) -> UVec4 {
        UVec4::new(self.w, self.y, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn wyxz(self) -> UVec4 {
        UVec4::new(self.w, self.y, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn wyxw(self) -> UVec4 {
        UVec4::new(self.w, self.y, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn wyyx(self) -> UVec4 {
        UVec4::new(self.w, self.y, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn wyyy(self) -> UVec4 {
        UVec4::new(self.w, self.y, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn wyyz(self) -> UVec4 {
        UVec4::new(self.w, self.y, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn wyyw(self) -> UVec4 {
        UVec4::new(self.w, self.y, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn wyzx(self) -> UVec4 {
        UVec4::new(self.w, self.y, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn wyzy(self) -> UVec4 {
        UVec4::new(self.w, self.y, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn wyzz(self) -> UVec4 {
        UVec4::new(self.w, self.y, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn wyzw(self) -> UVec4 {
        UVec4::new(self.w, self.y, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn wywx(self) -> UVec4 {
        UVec4::new(self.w, self.y, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn wywy(self) -> UVec4 {
        UVec4::new(self.w, self.y, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn wywz(self) -> UVec4 {
        UVec4::new(self.w, self.y, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn wyww(self) -> UVec4 {
        UVec4::new(self.w, self.y, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn wzxx(self) -> UVec4 {
        UVec4::new(self.w, self.z, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn wzxy(self) -> UVec4 {
        UVec4::new(self.w, self.z, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn wzxz(self) -> UVec4 {
        UVec4::new(self.w, self.z, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn wzxw(self) -> UVec4 {
        UVec4::new(self.w, self.z, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn wzyx(self) -> UVec4 {
        UVec4::new(self.w, self.z, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn wzyy(self) -> UVec4 {
        UVec4::new(self.w, self.z, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn wzyz(self) -> UVec4 {
        UVec4::new(self.w, self.z, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn wzyw(self) -> UVec4 {
        UVec4::new(self.w, self.z, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn wzzx(self) -> UVec4 {
        UVec4::new(self.w, self.z, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn wzzy(self) -> UVec4 {
        UVec4::new(self.w, self.z, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn wzzz(self) -> UVec4 {
        UVec4::new(self.w, self.z, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn wzzw(self) -> UVec4 {
        UVec4::new(self.w, self.z, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn wzwx(self) -> UVec4 {
        UVec4::new(self.w, self.z, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn wzwy(self) -> UVec4 {
        UVec4::new(self.w, self.z, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn wzwz(self) -> UVec4 {
        UVec4::new(self.w, self.z, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn wzww(self) -> UVec4 {
        UVec4::new(self.w, self.z, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn wwxx(self) -> UVec4 {
        UVec4::new(self.w, self.w, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn wwxy(self) -> UVec4 {
        UVec4::new(self.w, self.w, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn wwxz(self) -> UVec4 {
        UVec4::new(self.w, self.w, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn wwxw(self) -> UVec4 {
        UVec4::new(self.w, self.w, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn wwyx(self) -> UVec4 {
        UVec4::new(self.w, self.w, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn wwyy(self) -> UVec4 {
        UVec4::new(self.w, self.w, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn wwyz(self) -> UVec4 {
        UVec4::new(self.w, self.w, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn wwyw(self) -> UVec4 {
        UVec4::new(self.w, self.w, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn wwzx(self) -> UVec4 {
        UVec4::new(self.w, self.w, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn wwzy(self) -> UVec4 {
        UVec4::new(self.w, self.w, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn wwzz(self) -> UVec4 {
        UVec4::new(self.w, self.w, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn wwzw(self) -> UVec4 {
        UVec4::new(self.w, self.w, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn wwwx(self) -> UVec4 {
        UVec4::new(self.w, self.w, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn wwwy(self) -> UVec4 {
        UVec4::new(self.w, self.w, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn wwwz(self) -> UVec4 {
        UVec4::new(self.w, self.w, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn wwww(self) -> UVec4 {
        UVec4::new(self.w, self.w, self.w, self.w)
    }
}
