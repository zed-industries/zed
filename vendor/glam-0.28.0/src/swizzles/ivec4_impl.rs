// Generated from swizzle_impl.rs.tera template. Edit the template, not the generated file.

use crate::{IVec2, IVec3, IVec4, Vec4Swizzles};

impl Vec4Swizzles for IVec4 {
    type Vec2 = IVec2;

    type Vec3 = IVec3;

    #[inline]
    #[must_use]
    fn xx(self) -> IVec2 {
        IVec2 {
            x: self.x,
            y: self.x,
        }
    }

    #[inline]
    #[must_use]
    fn xy(self) -> IVec2 {
        IVec2 {
            x: self.x,
            y: self.y,
        }
    }

    #[inline]
    #[must_use]
    fn xz(self) -> IVec2 {
        IVec2 {
            x: self.x,
            y: self.z,
        }
    }

    #[inline]
    #[must_use]
    fn xw(self) -> IVec2 {
        IVec2 {
            x: self.x,
            y: self.w,
        }
    }

    #[inline]
    #[must_use]
    fn yx(self) -> IVec2 {
        IVec2 {
            x: self.y,
            y: self.x,
        }
    }

    #[inline]
    #[must_use]
    fn yy(self) -> IVec2 {
        IVec2 {
            x: self.y,
            y: self.y,
        }
    }

    #[inline]
    #[must_use]
    fn yz(self) -> IVec2 {
        IVec2 {
            x: self.y,
            y: self.z,
        }
    }

    #[inline]
    #[must_use]
    fn yw(self) -> IVec2 {
        IVec2 {
            x: self.y,
            y: self.w,
        }
    }

    #[inline]
    #[must_use]
    fn zx(self) -> IVec2 {
        IVec2 {
            x: self.z,
            y: self.x,
        }
    }

    #[inline]
    #[must_use]
    fn zy(self) -> IVec2 {
        IVec2 {
            x: self.z,
            y: self.y,
        }
    }

    #[inline]
    #[must_use]
    fn zz(self) -> IVec2 {
        IVec2 {
            x: self.z,
            y: self.z,
        }
    }

    #[inline]
    #[must_use]
    fn zw(self) -> IVec2 {
        IVec2 {
            x: self.z,
            y: self.w,
        }
    }

    #[inline]
    #[must_use]
    fn wx(self) -> IVec2 {
        IVec2 {
            x: self.w,
            y: self.x,
        }
    }

    #[inline]
    #[must_use]
    fn wy(self) -> IVec2 {
        IVec2 {
            x: self.w,
            y: self.y,
        }
    }

    #[inline]
    #[must_use]
    fn wz(self) -> IVec2 {
        IVec2 {
            x: self.w,
            y: self.z,
        }
    }

    #[inline]
    #[must_use]
    fn ww(self) -> IVec2 {
        IVec2 {
            x: self.w,
            y: self.w,
        }
    }

    #[inline]
    #[must_use]
    fn xxx(self) -> IVec3 {
        IVec3::new(self.x, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn xxy(self) -> IVec3 {
        IVec3::new(self.x, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn xxz(self) -> IVec3 {
        IVec3::new(self.x, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn xxw(self) -> IVec3 {
        IVec3::new(self.x, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn xyx(self) -> IVec3 {
        IVec3::new(self.x, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn xyy(self) -> IVec3 {
        IVec3::new(self.x, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn xyz(self) -> IVec3 {
        IVec3::new(self.x, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn xyw(self) -> IVec3 {
        IVec3::new(self.x, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn xzx(self) -> IVec3 {
        IVec3::new(self.x, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn xzy(self) -> IVec3 {
        IVec3::new(self.x, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn xzz(self) -> IVec3 {
        IVec3::new(self.x, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn xzw(self) -> IVec3 {
        IVec3::new(self.x, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn xwx(self) -> IVec3 {
        IVec3::new(self.x, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn xwy(self) -> IVec3 {
        IVec3::new(self.x, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn xwz(self) -> IVec3 {
        IVec3::new(self.x, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn xww(self) -> IVec3 {
        IVec3::new(self.x, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn yxx(self) -> IVec3 {
        IVec3::new(self.y, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn yxy(self) -> IVec3 {
        IVec3::new(self.y, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn yxz(self) -> IVec3 {
        IVec3::new(self.y, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn yxw(self) -> IVec3 {
        IVec3::new(self.y, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn yyx(self) -> IVec3 {
        IVec3::new(self.y, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn yyy(self) -> IVec3 {
        IVec3::new(self.y, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn yyz(self) -> IVec3 {
        IVec3::new(self.y, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn yyw(self) -> IVec3 {
        IVec3::new(self.y, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn yzx(self) -> IVec3 {
        IVec3::new(self.y, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn yzy(self) -> IVec3 {
        IVec3::new(self.y, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn yzz(self) -> IVec3 {
        IVec3::new(self.y, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn yzw(self) -> IVec3 {
        IVec3::new(self.y, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn ywx(self) -> IVec3 {
        IVec3::new(self.y, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn ywy(self) -> IVec3 {
        IVec3::new(self.y, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn ywz(self) -> IVec3 {
        IVec3::new(self.y, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn yww(self) -> IVec3 {
        IVec3::new(self.y, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn zxx(self) -> IVec3 {
        IVec3::new(self.z, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn zxy(self) -> IVec3 {
        IVec3::new(self.z, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn zxz(self) -> IVec3 {
        IVec3::new(self.z, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn zxw(self) -> IVec3 {
        IVec3::new(self.z, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn zyx(self) -> IVec3 {
        IVec3::new(self.z, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn zyy(self) -> IVec3 {
        IVec3::new(self.z, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn zyz(self) -> IVec3 {
        IVec3::new(self.z, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn zyw(self) -> IVec3 {
        IVec3::new(self.z, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn zzx(self) -> IVec3 {
        IVec3::new(self.z, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn zzy(self) -> IVec3 {
        IVec3::new(self.z, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn zzz(self) -> IVec3 {
        IVec3::new(self.z, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn zzw(self) -> IVec3 {
        IVec3::new(self.z, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn zwx(self) -> IVec3 {
        IVec3::new(self.z, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn zwy(self) -> IVec3 {
        IVec3::new(self.z, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn zwz(self) -> IVec3 {
        IVec3::new(self.z, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn zww(self) -> IVec3 {
        IVec3::new(self.z, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn wxx(self) -> IVec3 {
        IVec3::new(self.w, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn wxy(self) -> IVec3 {
        IVec3::new(self.w, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn wxz(self) -> IVec3 {
        IVec3::new(self.w, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn wxw(self) -> IVec3 {
        IVec3::new(self.w, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn wyx(self) -> IVec3 {
        IVec3::new(self.w, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn wyy(self) -> IVec3 {
        IVec3::new(self.w, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn wyz(self) -> IVec3 {
        IVec3::new(self.w, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn wyw(self) -> IVec3 {
        IVec3::new(self.w, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn wzx(self) -> IVec3 {
        IVec3::new(self.w, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn wzy(self) -> IVec3 {
        IVec3::new(self.w, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn wzz(self) -> IVec3 {
        IVec3::new(self.w, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn wzw(self) -> IVec3 {
        IVec3::new(self.w, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn wwx(self) -> IVec3 {
        IVec3::new(self.w, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn wwy(self) -> IVec3 {
        IVec3::new(self.w, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn wwz(self) -> IVec3 {
        IVec3::new(self.w, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn www(self) -> IVec3 {
        IVec3::new(self.w, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn xxxx(self) -> IVec4 {
        IVec4::new(self.x, self.x, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn xxxy(self) -> IVec4 {
        IVec4::new(self.x, self.x, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn xxxz(self) -> IVec4 {
        IVec4::new(self.x, self.x, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn xxxw(self) -> IVec4 {
        IVec4::new(self.x, self.x, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn xxyx(self) -> IVec4 {
        IVec4::new(self.x, self.x, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn xxyy(self) -> IVec4 {
        IVec4::new(self.x, self.x, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn xxyz(self) -> IVec4 {
        IVec4::new(self.x, self.x, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn xxyw(self) -> IVec4 {
        IVec4::new(self.x, self.x, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn xxzx(self) -> IVec4 {
        IVec4::new(self.x, self.x, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn xxzy(self) -> IVec4 {
        IVec4::new(self.x, self.x, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn xxzz(self) -> IVec4 {
        IVec4::new(self.x, self.x, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn xxzw(self) -> IVec4 {
        IVec4::new(self.x, self.x, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn xxwx(self) -> IVec4 {
        IVec4::new(self.x, self.x, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn xxwy(self) -> IVec4 {
        IVec4::new(self.x, self.x, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn xxwz(self) -> IVec4 {
        IVec4::new(self.x, self.x, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn xxww(self) -> IVec4 {
        IVec4::new(self.x, self.x, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn xyxx(self) -> IVec4 {
        IVec4::new(self.x, self.y, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn xyxy(self) -> IVec4 {
        IVec4::new(self.x, self.y, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn xyxz(self) -> IVec4 {
        IVec4::new(self.x, self.y, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn xyxw(self) -> IVec4 {
        IVec4::new(self.x, self.y, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn xyyx(self) -> IVec4 {
        IVec4::new(self.x, self.y, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn xyyy(self) -> IVec4 {
        IVec4::new(self.x, self.y, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn xyyz(self) -> IVec4 {
        IVec4::new(self.x, self.y, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn xyyw(self) -> IVec4 {
        IVec4::new(self.x, self.y, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn xyzx(self) -> IVec4 {
        IVec4::new(self.x, self.y, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn xyzy(self) -> IVec4 {
        IVec4::new(self.x, self.y, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn xyzz(self) -> IVec4 {
        IVec4::new(self.x, self.y, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn xyzw(self) -> IVec4 {
        IVec4::new(self.x, self.y, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn xywx(self) -> IVec4 {
        IVec4::new(self.x, self.y, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn xywy(self) -> IVec4 {
        IVec4::new(self.x, self.y, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn xywz(self) -> IVec4 {
        IVec4::new(self.x, self.y, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn xyww(self) -> IVec4 {
        IVec4::new(self.x, self.y, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn xzxx(self) -> IVec4 {
        IVec4::new(self.x, self.z, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn xzxy(self) -> IVec4 {
        IVec4::new(self.x, self.z, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn xzxz(self) -> IVec4 {
        IVec4::new(self.x, self.z, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn xzxw(self) -> IVec4 {
        IVec4::new(self.x, self.z, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn xzyx(self) -> IVec4 {
        IVec4::new(self.x, self.z, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn xzyy(self) -> IVec4 {
        IVec4::new(self.x, self.z, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn xzyz(self) -> IVec4 {
        IVec4::new(self.x, self.z, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn xzyw(self) -> IVec4 {
        IVec4::new(self.x, self.z, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn xzzx(self) -> IVec4 {
        IVec4::new(self.x, self.z, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn xzzy(self) -> IVec4 {
        IVec4::new(self.x, self.z, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn xzzz(self) -> IVec4 {
        IVec4::new(self.x, self.z, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn xzzw(self) -> IVec4 {
        IVec4::new(self.x, self.z, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn xzwx(self) -> IVec4 {
        IVec4::new(self.x, self.z, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn xzwy(self) -> IVec4 {
        IVec4::new(self.x, self.z, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn xzwz(self) -> IVec4 {
        IVec4::new(self.x, self.z, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn xzww(self) -> IVec4 {
        IVec4::new(self.x, self.z, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn xwxx(self) -> IVec4 {
        IVec4::new(self.x, self.w, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn xwxy(self) -> IVec4 {
        IVec4::new(self.x, self.w, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn xwxz(self) -> IVec4 {
        IVec4::new(self.x, self.w, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn xwxw(self) -> IVec4 {
        IVec4::new(self.x, self.w, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn xwyx(self) -> IVec4 {
        IVec4::new(self.x, self.w, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn xwyy(self) -> IVec4 {
        IVec4::new(self.x, self.w, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn xwyz(self) -> IVec4 {
        IVec4::new(self.x, self.w, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn xwyw(self) -> IVec4 {
        IVec4::new(self.x, self.w, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn xwzx(self) -> IVec4 {
        IVec4::new(self.x, self.w, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn xwzy(self) -> IVec4 {
        IVec4::new(self.x, self.w, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn xwzz(self) -> IVec4 {
        IVec4::new(self.x, self.w, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn xwzw(self) -> IVec4 {
        IVec4::new(self.x, self.w, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn xwwx(self) -> IVec4 {
        IVec4::new(self.x, self.w, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn xwwy(self) -> IVec4 {
        IVec4::new(self.x, self.w, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn xwwz(self) -> IVec4 {
        IVec4::new(self.x, self.w, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn xwww(self) -> IVec4 {
        IVec4::new(self.x, self.w, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn yxxx(self) -> IVec4 {
        IVec4::new(self.y, self.x, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn yxxy(self) -> IVec4 {
        IVec4::new(self.y, self.x, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn yxxz(self) -> IVec4 {
        IVec4::new(self.y, self.x, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn yxxw(self) -> IVec4 {
        IVec4::new(self.y, self.x, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn yxyx(self) -> IVec4 {
        IVec4::new(self.y, self.x, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn yxyy(self) -> IVec4 {
        IVec4::new(self.y, self.x, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn yxyz(self) -> IVec4 {
        IVec4::new(self.y, self.x, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn yxyw(self) -> IVec4 {
        IVec4::new(self.y, self.x, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn yxzx(self) -> IVec4 {
        IVec4::new(self.y, self.x, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn yxzy(self) -> IVec4 {
        IVec4::new(self.y, self.x, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn yxzz(self) -> IVec4 {
        IVec4::new(self.y, self.x, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn yxzw(self) -> IVec4 {
        IVec4::new(self.y, self.x, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn yxwx(self) -> IVec4 {
        IVec4::new(self.y, self.x, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn yxwy(self) -> IVec4 {
        IVec4::new(self.y, self.x, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn yxwz(self) -> IVec4 {
        IVec4::new(self.y, self.x, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn yxww(self) -> IVec4 {
        IVec4::new(self.y, self.x, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn yyxx(self) -> IVec4 {
        IVec4::new(self.y, self.y, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn yyxy(self) -> IVec4 {
        IVec4::new(self.y, self.y, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn yyxz(self) -> IVec4 {
        IVec4::new(self.y, self.y, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn yyxw(self) -> IVec4 {
        IVec4::new(self.y, self.y, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn yyyx(self) -> IVec4 {
        IVec4::new(self.y, self.y, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn yyyy(self) -> IVec4 {
        IVec4::new(self.y, self.y, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn yyyz(self) -> IVec4 {
        IVec4::new(self.y, self.y, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn yyyw(self) -> IVec4 {
        IVec4::new(self.y, self.y, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn yyzx(self) -> IVec4 {
        IVec4::new(self.y, self.y, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn yyzy(self) -> IVec4 {
        IVec4::new(self.y, self.y, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn yyzz(self) -> IVec4 {
        IVec4::new(self.y, self.y, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn yyzw(self) -> IVec4 {
        IVec4::new(self.y, self.y, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn yywx(self) -> IVec4 {
        IVec4::new(self.y, self.y, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn yywy(self) -> IVec4 {
        IVec4::new(self.y, self.y, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn yywz(self) -> IVec4 {
        IVec4::new(self.y, self.y, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn yyww(self) -> IVec4 {
        IVec4::new(self.y, self.y, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn yzxx(self) -> IVec4 {
        IVec4::new(self.y, self.z, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn yzxy(self) -> IVec4 {
        IVec4::new(self.y, self.z, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn yzxz(self) -> IVec4 {
        IVec4::new(self.y, self.z, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn yzxw(self) -> IVec4 {
        IVec4::new(self.y, self.z, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn yzyx(self) -> IVec4 {
        IVec4::new(self.y, self.z, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn yzyy(self) -> IVec4 {
        IVec4::new(self.y, self.z, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn yzyz(self) -> IVec4 {
        IVec4::new(self.y, self.z, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn yzyw(self) -> IVec4 {
        IVec4::new(self.y, self.z, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn yzzx(self) -> IVec4 {
        IVec4::new(self.y, self.z, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn yzzy(self) -> IVec4 {
        IVec4::new(self.y, self.z, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn yzzz(self) -> IVec4 {
        IVec4::new(self.y, self.z, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn yzzw(self) -> IVec4 {
        IVec4::new(self.y, self.z, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn yzwx(self) -> IVec4 {
        IVec4::new(self.y, self.z, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn yzwy(self) -> IVec4 {
        IVec4::new(self.y, self.z, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn yzwz(self) -> IVec4 {
        IVec4::new(self.y, self.z, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn yzww(self) -> IVec4 {
        IVec4::new(self.y, self.z, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn ywxx(self) -> IVec4 {
        IVec4::new(self.y, self.w, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn ywxy(self) -> IVec4 {
        IVec4::new(self.y, self.w, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn ywxz(self) -> IVec4 {
        IVec4::new(self.y, self.w, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn ywxw(self) -> IVec4 {
        IVec4::new(self.y, self.w, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn ywyx(self) -> IVec4 {
        IVec4::new(self.y, self.w, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn ywyy(self) -> IVec4 {
        IVec4::new(self.y, self.w, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn ywyz(self) -> IVec4 {
        IVec4::new(self.y, self.w, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn ywyw(self) -> IVec4 {
        IVec4::new(self.y, self.w, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn ywzx(self) -> IVec4 {
        IVec4::new(self.y, self.w, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn ywzy(self) -> IVec4 {
        IVec4::new(self.y, self.w, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn ywzz(self) -> IVec4 {
        IVec4::new(self.y, self.w, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn ywzw(self) -> IVec4 {
        IVec4::new(self.y, self.w, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn ywwx(self) -> IVec4 {
        IVec4::new(self.y, self.w, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn ywwy(self) -> IVec4 {
        IVec4::new(self.y, self.w, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn ywwz(self) -> IVec4 {
        IVec4::new(self.y, self.w, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn ywww(self) -> IVec4 {
        IVec4::new(self.y, self.w, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn zxxx(self) -> IVec4 {
        IVec4::new(self.z, self.x, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn zxxy(self) -> IVec4 {
        IVec4::new(self.z, self.x, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn zxxz(self) -> IVec4 {
        IVec4::new(self.z, self.x, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn zxxw(self) -> IVec4 {
        IVec4::new(self.z, self.x, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn zxyx(self) -> IVec4 {
        IVec4::new(self.z, self.x, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn zxyy(self) -> IVec4 {
        IVec4::new(self.z, self.x, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn zxyz(self) -> IVec4 {
        IVec4::new(self.z, self.x, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn zxyw(self) -> IVec4 {
        IVec4::new(self.z, self.x, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn zxzx(self) -> IVec4 {
        IVec4::new(self.z, self.x, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn zxzy(self) -> IVec4 {
        IVec4::new(self.z, self.x, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn zxzz(self) -> IVec4 {
        IVec4::new(self.z, self.x, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn zxzw(self) -> IVec4 {
        IVec4::new(self.z, self.x, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn zxwx(self) -> IVec4 {
        IVec4::new(self.z, self.x, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn zxwy(self) -> IVec4 {
        IVec4::new(self.z, self.x, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn zxwz(self) -> IVec4 {
        IVec4::new(self.z, self.x, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn zxww(self) -> IVec4 {
        IVec4::new(self.z, self.x, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn zyxx(self) -> IVec4 {
        IVec4::new(self.z, self.y, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn zyxy(self) -> IVec4 {
        IVec4::new(self.z, self.y, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn zyxz(self) -> IVec4 {
        IVec4::new(self.z, self.y, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn zyxw(self) -> IVec4 {
        IVec4::new(self.z, self.y, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn zyyx(self) -> IVec4 {
        IVec4::new(self.z, self.y, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn zyyy(self) -> IVec4 {
        IVec4::new(self.z, self.y, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn zyyz(self) -> IVec4 {
        IVec4::new(self.z, self.y, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn zyyw(self) -> IVec4 {
        IVec4::new(self.z, self.y, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn zyzx(self) -> IVec4 {
        IVec4::new(self.z, self.y, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn zyzy(self) -> IVec4 {
        IVec4::new(self.z, self.y, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn zyzz(self) -> IVec4 {
        IVec4::new(self.z, self.y, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn zyzw(self) -> IVec4 {
        IVec4::new(self.z, self.y, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn zywx(self) -> IVec4 {
        IVec4::new(self.z, self.y, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn zywy(self) -> IVec4 {
        IVec4::new(self.z, self.y, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn zywz(self) -> IVec4 {
        IVec4::new(self.z, self.y, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn zyww(self) -> IVec4 {
        IVec4::new(self.z, self.y, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn zzxx(self) -> IVec4 {
        IVec4::new(self.z, self.z, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn zzxy(self) -> IVec4 {
        IVec4::new(self.z, self.z, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn zzxz(self) -> IVec4 {
        IVec4::new(self.z, self.z, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn zzxw(self) -> IVec4 {
        IVec4::new(self.z, self.z, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn zzyx(self) -> IVec4 {
        IVec4::new(self.z, self.z, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn zzyy(self) -> IVec4 {
        IVec4::new(self.z, self.z, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn zzyz(self) -> IVec4 {
        IVec4::new(self.z, self.z, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn zzyw(self) -> IVec4 {
        IVec4::new(self.z, self.z, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn zzzx(self) -> IVec4 {
        IVec4::new(self.z, self.z, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn zzzy(self) -> IVec4 {
        IVec4::new(self.z, self.z, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn zzzz(self) -> IVec4 {
        IVec4::new(self.z, self.z, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn zzzw(self) -> IVec4 {
        IVec4::new(self.z, self.z, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn zzwx(self) -> IVec4 {
        IVec4::new(self.z, self.z, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn zzwy(self) -> IVec4 {
        IVec4::new(self.z, self.z, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn zzwz(self) -> IVec4 {
        IVec4::new(self.z, self.z, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn zzww(self) -> IVec4 {
        IVec4::new(self.z, self.z, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn zwxx(self) -> IVec4 {
        IVec4::new(self.z, self.w, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn zwxy(self) -> IVec4 {
        IVec4::new(self.z, self.w, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn zwxz(self) -> IVec4 {
        IVec4::new(self.z, self.w, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn zwxw(self) -> IVec4 {
        IVec4::new(self.z, self.w, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn zwyx(self) -> IVec4 {
        IVec4::new(self.z, self.w, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn zwyy(self) -> IVec4 {
        IVec4::new(self.z, self.w, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn zwyz(self) -> IVec4 {
        IVec4::new(self.z, self.w, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn zwyw(self) -> IVec4 {
        IVec4::new(self.z, self.w, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn zwzx(self) -> IVec4 {
        IVec4::new(self.z, self.w, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn zwzy(self) -> IVec4 {
        IVec4::new(self.z, self.w, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn zwzz(self) -> IVec4 {
        IVec4::new(self.z, self.w, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn zwzw(self) -> IVec4 {
        IVec4::new(self.z, self.w, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn zwwx(self) -> IVec4 {
        IVec4::new(self.z, self.w, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn zwwy(self) -> IVec4 {
        IVec4::new(self.z, self.w, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn zwwz(self) -> IVec4 {
        IVec4::new(self.z, self.w, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn zwww(self) -> IVec4 {
        IVec4::new(self.z, self.w, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn wxxx(self) -> IVec4 {
        IVec4::new(self.w, self.x, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn wxxy(self) -> IVec4 {
        IVec4::new(self.w, self.x, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn wxxz(self) -> IVec4 {
        IVec4::new(self.w, self.x, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn wxxw(self) -> IVec4 {
        IVec4::new(self.w, self.x, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn wxyx(self) -> IVec4 {
        IVec4::new(self.w, self.x, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn wxyy(self) -> IVec4 {
        IVec4::new(self.w, self.x, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn wxyz(self) -> IVec4 {
        IVec4::new(self.w, self.x, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn wxyw(self) -> IVec4 {
        IVec4::new(self.w, self.x, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn wxzx(self) -> IVec4 {
        IVec4::new(self.w, self.x, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn wxzy(self) -> IVec4 {
        IVec4::new(self.w, self.x, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn wxzz(self) -> IVec4 {
        IVec4::new(self.w, self.x, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn wxzw(self) -> IVec4 {
        IVec4::new(self.w, self.x, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn wxwx(self) -> IVec4 {
        IVec4::new(self.w, self.x, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn wxwy(self) -> IVec4 {
        IVec4::new(self.w, self.x, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn wxwz(self) -> IVec4 {
        IVec4::new(self.w, self.x, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn wxww(self) -> IVec4 {
        IVec4::new(self.w, self.x, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn wyxx(self) -> IVec4 {
        IVec4::new(self.w, self.y, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn wyxy(self) -> IVec4 {
        IVec4::new(self.w, self.y, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn wyxz(self) -> IVec4 {
        IVec4::new(self.w, self.y, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn wyxw(self) -> IVec4 {
        IVec4::new(self.w, self.y, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn wyyx(self) -> IVec4 {
        IVec4::new(self.w, self.y, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn wyyy(self) -> IVec4 {
        IVec4::new(self.w, self.y, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn wyyz(self) -> IVec4 {
        IVec4::new(self.w, self.y, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn wyyw(self) -> IVec4 {
        IVec4::new(self.w, self.y, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn wyzx(self) -> IVec4 {
        IVec4::new(self.w, self.y, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn wyzy(self) -> IVec4 {
        IVec4::new(self.w, self.y, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn wyzz(self) -> IVec4 {
        IVec4::new(self.w, self.y, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn wyzw(self) -> IVec4 {
        IVec4::new(self.w, self.y, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn wywx(self) -> IVec4 {
        IVec4::new(self.w, self.y, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn wywy(self) -> IVec4 {
        IVec4::new(self.w, self.y, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn wywz(self) -> IVec4 {
        IVec4::new(self.w, self.y, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn wyww(self) -> IVec4 {
        IVec4::new(self.w, self.y, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn wzxx(self) -> IVec4 {
        IVec4::new(self.w, self.z, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn wzxy(self) -> IVec4 {
        IVec4::new(self.w, self.z, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn wzxz(self) -> IVec4 {
        IVec4::new(self.w, self.z, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn wzxw(self) -> IVec4 {
        IVec4::new(self.w, self.z, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn wzyx(self) -> IVec4 {
        IVec4::new(self.w, self.z, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn wzyy(self) -> IVec4 {
        IVec4::new(self.w, self.z, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn wzyz(self) -> IVec4 {
        IVec4::new(self.w, self.z, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn wzyw(self) -> IVec4 {
        IVec4::new(self.w, self.z, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn wzzx(self) -> IVec4 {
        IVec4::new(self.w, self.z, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn wzzy(self) -> IVec4 {
        IVec4::new(self.w, self.z, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn wzzz(self) -> IVec4 {
        IVec4::new(self.w, self.z, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn wzzw(self) -> IVec4 {
        IVec4::new(self.w, self.z, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn wzwx(self) -> IVec4 {
        IVec4::new(self.w, self.z, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn wzwy(self) -> IVec4 {
        IVec4::new(self.w, self.z, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn wzwz(self) -> IVec4 {
        IVec4::new(self.w, self.z, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn wzww(self) -> IVec4 {
        IVec4::new(self.w, self.z, self.w, self.w)
    }

    #[inline]
    #[must_use]
    fn wwxx(self) -> IVec4 {
        IVec4::new(self.w, self.w, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn wwxy(self) -> IVec4 {
        IVec4::new(self.w, self.w, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn wwxz(self) -> IVec4 {
        IVec4::new(self.w, self.w, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn wwxw(self) -> IVec4 {
        IVec4::new(self.w, self.w, self.x, self.w)
    }

    #[inline]
    #[must_use]
    fn wwyx(self) -> IVec4 {
        IVec4::new(self.w, self.w, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn wwyy(self) -> IVec4 {
        IVec4::new(self.w, self.w, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn wwyz(self) -> IVec4 {
        IVec4::new(self.w, self.w, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn wwyw(self) -> IVec4 {
        IVec4::new(self.w, self.w, self.y, self.w)
    }

    #[inline]
    #[must_use]
    fn wwzx(self) -> IVec4 {
        IVec4::new(self.w, self.w, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn wwzy(self) -> IVec4 {
        IVec4::new(self.w, self.w, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn wwzz(self) -> IVec4 {
        IVec4::new(self.w, self.w, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn wwzw(self) -> IVec4 {
        IVec4::new(self.w, self.w, self.z, self.w)
    }

    #[inline]
    #[must_use]
    fn wwwx(self) -> IVec4 {
        IVec4::new(self.w, self.w, self.w, self.x)
    }

    #[inline]
    #[must_use]
    fn wwwy(self) -> IVec4 {
        IVec4::new(self.w, self.w, self.w, self.y)
    }

    #[inline]
    #[must_use]
    fn wwwz(self) -> IVec4 {
        IVec4::new(self.w, self.w, self.w, self.z)
    }

    #[inline]
    #[must_use]
    fn wwww(self) -> IVec4 {
        IVec4::new(self.w, self.w, self.w, self.w)
    }
}
