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
    fn with_xy(self, rhs: IVec2) -> Self {
        Self::new(rhs.x, rhs.y, self.z, self.w)
    }

    #[inline]
    fn xz(self) -> IVec2 {
        IVec2 {
            x: self.x,
            y: self.z,
        }
    }

    #[inline]
    fn with_xz(self, rhs: IVec2) -> Self {
        Self::new(rhs.x, self.y, rhs.y, self.w)
    }

    #[inline]
    fn xw(self) -> IVec2 {
        IVec2 {
            x: self.x,
            y: self.w,
        }
    }

    #[inline]
    fn with_xw(self, rhs: IVec2) -> Self {
        Self::new(rhs.x, self.y, self.z, rhs.y)
    }

    #[inline]
    fn yx(self) -> IVec2 {
        IVec2 {
            x: self.y,
            y: self.x,
        }
    }

    #[inline]
    fn with_yx(self, rhs: IVec2) -> Self {
        Self::new(rhs.y, rhs.x, self.z, self.w)
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
    fn with_yz(self, rhs: IVec2) -> Self {
        Self::new(self.x, rhs.x, rhs.y, self.w)
    }

    #[inline]
    fn yw(self) -> IVec2 {
        IVec2 {
            x: self.y,
            y: self.w,
        }
    }

    #[inline]
    fn with_yw(self, rhs: IVec2) -> Self {
        Self::new(self.x, rhs.x, self.z, rhs.y)
    }

    #[inline]
    fn zx(self) -> IVec2 {
        IVec2 {
            x: self.z,
            y: self.x,
        }
    }

    #[inline]
    fn with_zx(self, rhs: IVec2) -> Self {
        Self::new(rhs.y, self.y, rhs.x, self.w)
    }

    #[inline]
    fn zy(self) -> IVec2 {
        IVec2 {
            x: self.z,
            y: self.y,
        }
    }

    #[inline]
    fn with_zy(self, rhs: IVec2) -> Self {
        Self::new(self.x, rhs.y, rhs.x, self.w)
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
    fn with_zw(self, rhs: IVec2) -> Self {
        Self::new(self.x, self.y, rhs.x, rhs.y)
    }

    #[inline]
    fn wx(self) -> IVec2 {
        IVec2 {
            x: self.w,
            y: self.x,
        }
    }

    #[inline]
    fn with_wx(self, rhs: IVec2) -> Self {
        Self::new(rhs.y, self.y, self.z, rhs.x)
    }

    #[inline]
    fn wy(self) -> IVec2 {
        IVec2 {
            x: self.w,
            y: self.y,
        }
    }

    #[inline]
    fn with_wy(self, rhs: IVec2) -> Self {
        Self::new(self.x, rhs.y, self.z, rhs.x)
    }

    #[inline]
    fn wz(self) -> IVec2 {
        IVec2 {
            x: self.w,
            y: self.z,
        }
    }

    #[inline]
    fn with_wz(self, rhs: IVec2) -> Self {
        Self::new(self.x, self.y, rhs.y, rhs.x)
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
        IVec3::new(self.x, self.x, self.x)
    }

    #[inline]
    fn xxy(self) -> IVec3 {
        IVec3::new(self.x, self.x, self.y)
    }

    #[inline]
    fn xxz(self) -> IVec3 {
        IVec3::new(self.x, self.x, self.z)
    }

    #[inline]
    fn xxw(self) -> IVec3 {
        IVec3::new(self.x, self.x, self.w)
    }

    #[inline]
    fn xyx(self) -> IVec3 {
        IVec3::new(self.x, self.y, self.x)
    }

    #[inline]
    fn xyy(self) -> IVec3 {
        IVec3::new(self.x, self.y, self.y)
    }

    #[inline]
    fn xyz(self) -> IVec3 {
        IVec3::new(self.x, self.y, self.z)
    }

    #[inline]
    fn with_xyz(self, rhs: IVec3) -> Self {
        Self::new(rhs.x, rhs.y, rhs.z, self.w)
    }

    #[inline]
    fn xyw(self) -> IVec3 {
        IVec3::new(self.x, self.y, self.w)
    }

    #[inline]
    fn with_xyw(self, rhs: IVec3) -> Self {
        Self::new(rhs.x, rhs.y, self.z, rhs.z)
    }

    #[inline]
    fn xzx(self) -> IVec3 {
        IVec3::new(self.x, self.z, self.x)
    }

    #[inline]
    fn xzy(self) -> IVec3 {
        IVec3::new(self.x, self.z, self.y)
    }

    #[inline]
    fn with_xzy(self, rhs: IVec3) -> Self {
        Self::new(rhs.x, rhs.z, rhs.y, self.w)
    }

    #[inline]
    fn xzz(self) -> IVec3 {
        IVec3::new(self.x, self.z, self.z)
    }

    #[inline]
    fn xzw(self) -> IVec3 {
        IVec3::new(self.x, self.z, self.w)
    }

    #[inline]
    fn with_xzw(self, rhs: IVec3) -> Self {
        Self::new(rhs.x, self.y, rhs.y, rhs.z)
    }

    #[inline]
    fn xwx(self) -> IVec3 {
        IVec3::new(self.x, self.w, self.x)
    }

    #[inline]
    fn xwy(self) -> IVec3 {
        IVec3::new(self.x, self.w, self.y)
    }

    #[inline]
    fn with_xwy(self, rhs: IVec3) -> Self {
        Self::new(rhs.x, rhs.z, self.z, rhs.y)
    }

    #[inline]
    fn xwz(self) -> IVec3 {
        IVec3::new(self.x, self.w, self.z)
    }

    #[inline]
    fn with_xwz(self, rhs: IVec3) -> Self {
        Self::new(rhs.x, self.y, rhs.z, rhs.y)
    }

    #[inline]
    fn xww(self) -> IVec3 {
        IVec3::new(self.x, self.w, self.w)
    }

    #[inline]
    fn yxx(self) -> IVec3 {
        IVec3::new(self.y, self.x, self.x)
    }

    #[inline]
    fn yxy(self) -> IVec3 {
        IVec3::new(self.y, self.x, self.y)
    }

    #[inline]
    fn yxz(self) -> IVec3 {
        IVec3::new(self.y, self.x, self.z)
    }

    #[inline]
    fn with_yxz(self, rhs: IVec3) -> Self {
        Self::new(rhs.y, rhs.x, rhs.z, self.w)
    }

    #[inline]
    fn yxw(self) -> IVec3 {
        IVec3::new(self.y, self.x, self.w)
    }

    #[inline]
    fn with_yxw(self, rhs: IVec3) -> Self {
        Self::new(rhs.y, rhs.x, self.z, rhs.z)
    }

    #[inline]
    fn yyx(self) -> IVec3 {
        IVec3::new(self.y, self.y, self.x)
    }

    #[inline]
    fn yyy(self) -> IVec3 {
        IVec3::new(self.y, self.y, self.y)
    }

    #[inline]
    fn yyz(self) -> IVec3 {
        IVec3::new(self.y, self.y, self.z)
    }

    #[inline]
    fn yyw(self) -> IVec3 {
        IVec3::new(self.y, self.y, self.w)
    }

    #[inline]
    fn yzx(self) -> IVec3 {
        IVec3::new(self.y, self.z, self.x)
    }

    #[inline]
    fn with_yzx(self, rhs: IVec3) -> Self {
        Self::new(rhs.z, rhs.x, rhs.y, self.w)
    }

    #[inline]
    fn yzy(self) -> IVec3 {
        IVec3::new(self.y, self.z, self.y)
    }

    #[inline]
    fn yzz(self) -> IVec3 {
        IVec3::new(self.y, self.z, self.z)
    }

    #[inline]
    fn yzw(self) -> IVec3 {
        IVec3::new(self.y, self.z, self.w)
    }

    #[inline]
    fn with_yzw(self, rhs: IVec3) -> Self {
        Self::new(self.x, rhs.x, rhs.y, rhs.z)
    }

    #[inline]
    fn ywx(self) -> IVec3 {
        IVec3::new(self.y, self.w, self.x)
    }

    #[inline]
    fn with_ywx(self, rhs: IVec3) -> Self {
        Self::new(rhs.z, rhs.x, self.z, rhs.y)
    }

    #[inline]
    fn ywy(self) -> IVec3 {
        IVec3::new(self.y, self.w, self.y)
    }

    #[inline]
    fn ywz(self) -> IVec3 {
        IVec3::new(self.y, self.w, self.z)
    }

    #[inline]
    fn with_ywz(self, rhs: IVec3) -> Self {
        Self::new(self.x, rhs.x, rhs.z, rhs.y)
    }

    #[inline]
    fn yww(self) -> IVec3 {
        IVec3::new(self.y, self.w, self.w)
    }

    #[inline]
    fn zxx(self) -> IVec3 {
        IVec3::new(self.z, self.x, self.x)
    }

    #[inline]
    fn zxy(self) -> IVec3 {
        IVec3::new(self.z, self.x, self.y)
    }

    #[inline]
    fn with_zxy(self, rhs: IVec3) -> Self {
        Self::new(rhs.y, rhs.z, rhs.x, self.w)
    }

    #[inline]
    fn zxz(self) -> IVec3 {
        IVec3::new(self.z, self.x, self.z)
    }

    #[inline]
    fn zxw(self) -> IVec3 {
        IVec3::new(self.z, self.x, self.w)
    }

    #[inline]
    fn with_zxw(self, rhs: IVec3) -> Self {
        Self::new(rhs.y, self.y, rhs.x, rhs.z)
    }

    #[inline]
    fn zyx(self) -> IVec3 {
        IVec3::new(self.z, self.y, self.x)
    }

    #[inline]
    fn with_zyx(self, rhs: IVec3) -> Self {
        Self::new(rhs.z, rhs.y, rhs.x, self.w)
    }

    #[inline]
    fn zyy(self) -> IVec3 {
        IVec3::new(self.z, self.y, self.y)
    }

    #[inline]
    fn zyz(self) -> IVec3 {
        IVec3::new(self.z, self.y, self.z)
    }

    #[inline]
    fn zyw(self) -> IVec3 {
        IVec3::new(self.z, self.y, self.w)
    }

    #[inline]
    fn with_zyw(self, rhs: IVec3) -> Self {
        Self::new(self.x, rhs.y, rhs.x, rhs.z)
    }

    #[inline]
    fn zzx(self) -> IVec3 {
        IVec3::new(self.z, self.z, self.x)
    }

    #[inline]
    fn zzy(self) -> IVec3 {
        IVec3::new(self.z, self.z, self.y)
    }

    #[inline]
    fn zzz(self) -> IVec3 {
        IVec3::new(self.z, self.z, self.z)
    }

    #[inline]
    fn zzw(self) -> IVec3 {
        IVec3::new(self.z, self.z, self.w)
    }

    #[inline]
    fn zwx(self) -> IVec3 {
        IVec3::new(self.z, self.w, self.x)
    }

    #[inline]
    fn with_zwx(self, rhs: IVec3) -> Self {
        Self::new(rhs.z, self.y, rhs.x, rhs.y)
    }

    #[inline]
    fn zwy(self) -> IVec3 {
        IVec3::new(self.z, self.w, self.y)
    }

    #[inline]
    fn with_zwy(self, rhs: IVec3) -> Self {
        Self::new(self.x, rhs.z, rhs.x, rhs.y)
    }

    #[inline]
    fn zwz(self) -> IVec3 {
        IVec3::new(self.z, self.w, self.z)
    }

    #[inline]
    fn zww(self) -> IVec3 {
        IVec3::new(self.z, self.w, self.w)
    }

    #[inline]
    fn wxx(self) -> IVec3 {
        IVec3::new(self.w, self.x, self.x)
    }

    #[inline]
    fn wxy(self) -> IVec3 {
        IVec3::new(self.w, self.x, self.y)
    }

    #[inline]
    fn with_wxy(self, rhs: IVec3) -> Self {
        Self::new(rhs.y, rhs.z, self.z, rhs.x)
    }

    #[inline]
    fn wxz(self) -> IVec3 {
        IVec3::new(self.w, self.x, self.z)
    }

    #[inline]
    fn with_wxz(self, rhs: IVec3) -> Self {
        Self::new(rhs.y, self.y, rhs.z, rhs.x)
    }

    #[inline]
    fn wxw(self) -> IVec3 {
        IVec3::new(self.w, self.x, self.w)
    }

    #[inline]
    fn wyx(self) -> IVec3 {
        IVec3::new(self.w, self.y, self.x)
    }

    #[inline]
    fn with_wyx(self, rhs: IVec3) -> Self {
        Self::new(rhs.z, rhs.y, self.z, rhs.x)
    }

    #[inline]
    fn wyy(self) -> IVec3 {
        IVec3::new(self.w, self.y, self.y)
    }

    #[inline]
    fn wyz(self) -> IVec3 {
        IVec3::new(self.w, self.y, self.z)
    }

    #[inline]
    fn with_wyz(self, rhs: IVec3) -> Self {
        Self::new(self.x, rhs.y, rhs.z, rhs.x)
    }

    #[inline]
    fn wyw(self) -> IVec3 {
        IVec3::new(self.w, self.y, self.w)
    }

    #[inline]
    fn wzx(self) -> IVec3 {
        IVec3::new(self.w, self.z, self.x)
    }

    #[inline]
    fn with_wzx(self, rhs: IVec3) -> Self {
        Self::new(rhs.z, self.y, rhs.y, rhs.x)
    }

    #[inline]
    fn wzy(self) -> IVec3 {
        IVec3::new(self.w, self.z, self.y)
    }

    #[inline]
    fn with_wzy(self, rhs: IVec3) -> Self {
        Self::new(self.x, rhs.z, rhs.y, rhs.x)
    }

    #[inline]
    fn wzz(self) -> IVec3 {
        IVec3::new(self.w, self.z, self.z)
    }

    #[inline]
    fn wzw(self) -> IVec3 {
        IVec3::new(self.w, self.z, self.w)
    }

    #[inline]
    fn wwx(self) -> IVec3 {
        IVec3::new(self.w, self.w, self.x)
    }

    #[inline]
    fn wwy(self) -> IVec3 {
        IVec3::new(self.w, self.w, self.y)
    }

    #[inline]
    fn wwz(self) -> IVec3 {
        IVec3::new(self.w, self.w, self.z)
    }

    #[inline]
    fn www(self) -> IVec3 {
        IVec3::new(self.w, self.w, self.w)
    }

    #[inline]
    fn xxxx(self) -> Self {
        Self::new(self.x, self.x, self.x, self.x)
    }

    #[inline]
    fn xxxy(self) -> Self {
        Self::new(self.x, self.x, self.x, self.y)
    }

    #[inline]
    fn xxxz(self) -> Self {
        Self::new(self.x, self.x, self.x, self.z)
    }

    #[inline]
    fn xxxw(self) -> Self {
        Self::new(self.x, self.x, self.x, self.w)
    }

    #[inline]
    fn xxyx(self) -> Self {
        Self::new(self.x, self.x, self.y, self.x)
    }

    #[inline]
    fn xxyy(self) -> Self {
        Self::new(self.x, self.x, self.y, self.y)
    }

    #[inline]
    fn xxyz(self) -> Self {
        Self::new(self.x, self.x, self.y, self.z)
    }

    #[inline]
    fn xxyw(self) -> Self {
        Self::new(self.x, self.x, self.y, self.w)
    }

    #[inline]
    fn xxzx(self) -> Self {
        Self::new(self.x, self.x, self.z, self.x)
    }

    #[inline]
    fn xxzy(self) -> Self {
        Self::new(self.x, self.x, self.z, self.y)
    }

    #[inline]
    fn xxzz(self) -> Self {
        Self::new(self.x, self.x, self.z, self.z)
    }

    #[inline]
    fn xxzw(self) -> Self {
        Self::new(self.x, self.x, self.z, self.w)
    }

    #[inline]
    fn xxwx(self) -> Self {
        Self::new(self.x, self.x, self.w, self.x)
    }

    #[inline]
    fn xxwy(self) -> Self {
        Self::new(self.x, self.x, self.w, self.y)
    }

    #[inline]
    fn xxwz(self) -> Self {
        Self::new(self.x, self.x, self.w, self.z)
    }

    #[inline]
    fn xxww(self) -> Self {
        Self::new(self.x, self.x, self.w, self.w)
    }

    #[inline]
    fn xyxx(self) -> Self {
        Self::new(self.x, self.y, self.x, self.x)
    }

    #[inline]
    fn xyxy(self) -> Self {
        Self::new(self.x, self.y, self.x, self.y)
    }

    #[inline]
    fn xyxz(self) -> Self {
        Self::new(self.x, self.y, self.x, self.z)
    }

    #[inline]
    fn xyxw(self) -> Self {
        Self::new(self.x, self.y, self.x, self.w)
    }

    #[inline]
    fn xyyx(self) -> Self {
        Self::new(self.x, self.y, self.y, self.x)
    }

    #[inline]
    fn xyyy(self) -> Self {
        Self::new(self.x, self.y, self.y, self.y)
    }

    #[inline]
    fn xyyz(self) -> Self {
        Self::new(self.x, self.y, self.y, self.z)
    }

    #[inline]
    fn xyyw(self) -> Self {
        Self::new(self.x, self.y, self.y, self.w)
    }

    #[inline]
    fn xyzx(self) -> Self {
        Self::new(self.x, self.y, self.z, self.x)
    }

    #[inline]
    fn xyzy(self) -> Self {
        Self::new(self.x, self.y, self.z, self.y)
    }

    #[inline]
    fn xyzz(self) -> Self {
        Self::new(self.x, self.y, self.z, self.z)
    }

    #[inline]
    fn xywx(self) -> Self {
        Self::new(self.x, self.y, self.w, self.x)
    }

    #[inline]
    fn xywy(self) -> Self {
        Self::new(self.x, self.y, self.w, self.y)
    }

    #[inline]
    fn xywz(self) -> Self {
        Self::new(self.x, self.y, self.w, self.z)
    }

    #[inline]
    fn xyww(self) -> Self {
        Self::new(self.x, self.y, self.w, self.w)
    }

    #[inline]
    fn xzxx(self) -> Self {
        Self::new(self.x, self.z, self.x, self.x)
    }

    #[inline]
    fn xzxy(self) -> Self {
        Self::new(self.x, self.z, self.x, self.y)
    }

    #[inline]
    fn xzxz(self) -> Self {
        Self::new(self.x, self.z, self.x, self.z)
    }

    #[inline]
    fn xzxw(self) -> Self {
        Self::new(self.x, self.z, self.x, self.w)
    }

    #[inline]
    fn xzyx(self) -> Self {
        Self::new(self.x, self.z, self.y, self.x)
    }

    #[inline]
    fn xzyy(self) -> Self {
        Self::new(self.x, self.z, self.y, self.y)
    }

    #[inline]
    fn xzyz(self) -> Self {
        Self::new(self.x, self.z, self.y, self.z)
    }

    #[inline]
    fn xzyw(self) -> Self {
        Self::new(self.x, self.z, self.y, self.w)
    }

    #[inline]
    fn xzzx(self) -> Self {
        Self::new(self.x, self.z, self.z, self.x)
    }

    #[inline]
    fn xzzy(self) -> Self {
        Self::new(self.x, self.z, self.z, self.y)
    }

    #[inline]
    fn xzzz(self) -> Self {
        Self::new(self.x, self.z, self.z, self.z)
    }

    #[inline]
    fn xzzw(self) -> Self {
        Self::new(self.x, self.z, self.z, self.w)
    }

    #[inline]
    fn xzwx(self) -> Self {
        Self::new(self.x, self.z, self.w, self.x)
    }

    #[inline]
    fn xzwy(self) -> Self {
        Self::new(self.x, self.z, self.w, self.y)
    }

    #[inline]
    fn xzwz(self) -> Self {
        Self::new(self.x, self.z, self.w, self.z)
    }

    #[inline]
    fn xzww(self) -> Self {
        Self::new(self.x, self.z, self.w, self.w)
    }

    #[inline]
    fn xwxx(self) -> Self {
        Self::new(self.x, self.w, self.x, self.x)
    }

    #[inline]
    fn xwxy(self) -> Self {
        Self::new(self.x, self.w, self.x, self.y)
    }

    #[inline]
    fn xwxz(self) -> Self {
        Self::new(self.x, self.w, self.x, self.z)
    }

    #[inline]
    fn xwxw(self) -> Self {
        Self::new(self.x, self.w, self.x, self.w)
    }

    #[inline]
    fn xwyx(self) -> Self {
        Self::new(self.x, self.w, self.y, self.x)
    }

    #[inline]
    fn xwyy(self) -> Self {
        Self::new(self.x, self.w, self.y, self.y)
    }

    #[inline]
    fn xwyz(self) -> Self {
        Self::new(self.x, self.w, self.y, self.z)
    }

    #[inline]
    fn xwyw(self) -> Self {
        Self::new(self.x, self.w, self.y, self.w)
    }

    #[inline]
    fn xwzx(self) -> Self {
        Self::new(self.x, self.w, self.z, self.x)
    }

    #[inline]
    fn xwzy(self) -> Self {
        Self::new(self.x, self.w, self.z, self.y)
    }

    #[inline]
    fn xwzz(self) -> Self {
        Self::new(self.x, self.w, self.z, self.z)
    }

    #[inline]
    fn xwzw(self) -> Self {
        Self::new(self.x, self.w, self.z, self.w)
    }

    #[inline]
    fn xwwx(self) -> Self {
        Self::new(self.x, self.w, self.w, self.x)
    }

    #[inline]
    fn xwwy(self) -> Self {
        Self::new(self.x, self.w, self.w, self.y)
    }

    #[inline]
    fn xwwz(self) -> Self {
        Self::new(self.x, self.w, self.w, self.z)
    }

    #[inline]
    fn xwww(self) -> Self {
        Self::new(self.x, self.w, self.w, self.w)
    }

    #[inline]
    fn yxxx(self) -> Self {
        Self::new(self.y, self.x, self.x, self.x)
    }

    #[inline]
    fn yxxy(self) -> Self {
        Self::new(self.y, self.x, self.x, self.y)
    }

    #[inline]
    fn yxxz(self) -> Self {
        Self::new(self.y, self.x, self.x, self.z)
    }

    #[inline]
    fn yxxw(self) -> Self {
        Self::new(self.y, self.x, self.x, self.w)
    }

    #[inline]
    fn yxyx(self) -> Self {
        Self::new(self.y, self.x, self.y, self.x)
    }

    #[inline]
    fn yxyy(self) -> Self {
        Self::new(self.y, self.x, self.y, self.y)
    }

    #[inline]
    fn yxyz(self) -> Self {
        Self::new(self.y, self.x, self.y, self.z)
    }

    #[inline]
    fn yxyw(self) -> Self {
        Self::new(self.y, self.x, self.y, self.w)
    }

    #[inline]
    fn yxzx(self) -> Self {
        Self::new(self.y, self.x, self.z, self.x)
    }

    #[inline]
    fn yxzy(self) -> Self {
        Self::new(self.y, self.x, self.z, self.y)
    }

    #[inline]
    fn yxzz(self) -> Self {
        Self::new(self.y, self.x, self.z, self.z)
    }

    #[inline]
    fn yxzw(self) -> Self {
        Self::new(self.y, self.x, self.z, self.w)
    }

    #[inline]
    fn yxwx(self) -> Self {
        Self::new(self.y, self.x, self.w, self.x)
    }

    #[inline]
    fn yxwy(self) -> Self {
        Self::new(self.y, self.x, self.w, self.y)
    }

    #[inline]
    fn yxwz(self) -> Self {
        Self::new(self.y, self.x, self.w, self.z)
    }

    #[inline]
    fn yxww(self) -> Self {
        Self::new(self.y, self.x, self.w, self.w)
    }

    #[inline]
    fn yyxx(self) -> Self {
        Self::new(self.y, self.y, self.x, self.x)
    }

    #[inline]
    fn yyxy(self) -> Self {
        Self::new(self.y, self.y, self.x, self.y)
    }

    #[inline]
    fn yyxz(self) -> Self {
        Self::new(self.y, self.y, self.x, self.z)
    }

    #[inline]
    fn yyxw(self) -> Self {
        Self::new(self.y, self.y, self.x, self.w)
    }

    #[inline]
    fn yyyx(self) -> Self {
        Self::new(self.y, self.y, self.y, self.x)
    }

    #[inline]
    fn yyyy(self) -> Self {
        Self::new(self.y, self.y, self.y, self.y)
    }

    #[inline]
    fn yyyz(self) -> Self {
        Self::new(self.y, self.y, self.y, self.z)
    }

    #[inline]
    fn yyyw(self) -> Self {
        Self::new(self.y, self.y, self.y, self.w)
    }

    #[inline]
    fn yyzx(self) -> Self {
        Self::new(self.y, self.y, self.z, self.x)
    }

    #[inline]
    fn yyzy(self) -> Self {
        Self::new(self.y, self.y, self.z, self.y)
    }

    #[inline]
    fn yyzz(self) -> Self {
        Self::new(self.y, self.y, self.z, self.z)
    }

    #[inline]
    fn yyzw(self) -> Self {
        Self::new(self.y, self.y, self.z, self.w)
    }

    #[inline]
    fn yywx(self) -> Self {
        Self::new(self.y, self.y, self.w, self.x)
    }

    #[inline]
    fn yywy(self) -> Self {
        Self::new(self.y, self.y, self.w, self.y)
    }

    #[inline]
    fn yywz(self) -> Self {
        Self::new(self.y, self.y, self.w, self.z)
    }

    #[inline]
    fn yyww(self) -> Self {
        Self::new(self.y, self.y, self.w, self.w)
    }

    #[inline]
    fn yzxx(self) -> Self {
        Self::new(self.y, self.z, self.x, self.x)
    }

    #[inline]
    fn yzxy(self) -> Self {
        Self::new(self.y, self.z, self.x, self.y)
    }

    #[inline]
    fn yzxz(self) -> Self {
        Self::new(self.y, self.z, self.x, self.z)
    }

    #[inline]
    fn yzxw(self) -> Self {
        Self::new(self.y, self.z, self.x, self.w)
    }

    #[inline]
    fn yzyx(self) -> Self {
        Self::new(self.y, self.z, self.y, self.x)
    }

    #[inline]
    fn yzyy(self) -> Self {
        Self::new(self.y, self.z, self.y, self.y)
    }

    #[inline]
    fn yzyz(self) -> Self {
        Self::new(self.y, self.z, self.y, self.z)
    }

    #[inline]
    fn yzyw(self) -> Self {
        Self::new(self.y, self.z, self.y, self.w)
    }

    #[inline]
    fn yzzx(self) -> Self {
        Self::new(self.y, self.z, self.z, self.x)
    }

    #[inline]
    fn yzzy(self) -> Self {
        Self::new(self.y, self.z, self.z, self.y)
    }

    #[inline]
    fn yzzz(self) -> Self {
        Self::new(self.y, self.z, self.z, self.z)
    }

    #[inline]
    fn yzzw(self) -> Self {
        Self::new(self.y, self.z, self.z, self.w)
    }

    #[inline]
    fn yzwx(self) -> Self {
        Self::new(self.y, self.z, self.w, self.x)
    }

    #[inline]
    fn yzwy(self) -> Self {
        Self::new(self.y, self.z, self.w, self.y)
    }

    #[inline]
    fn yzwz(self) -> Self {
        Self::new(self.y, self.z, self.w, self.z)
    }

    #[inline]
    fn yzww(self) -> Self {
        Self::new(self.y, self.z, self.w, self.w)
    }

    #[inline]
    fn ywxx(self) -> Self {
        Self::new(self.y, self.w, self.x, self.x)
    }

    #[inline]
    fn ywxy(self) -> Self {
        Self::new(self.y, self.w, self.x, self.y)
    }

    #[inline]
    fn ywxz(self) -> Self {
        Self::new(self.y, self.w, self.x, self.z)
    }

    #[inline]
    fn ywxw(self) -> Self {
        Self::new(self.y, self.w, self.x, self.w)
    }

    #[inline]
    fn ywyx(self) -> Self {
        Self::new(self.y, self.w, self.y, self.x)
    }

    #[inline]
    fn ywyy(self) -> Self {
        Self::new(self.y, self.w, self.y, self.y)
    }

    #[inline]
    fn ywyz(self) -> Self {
        Self::new(self.y, self.w, self.y, self.z)
    }

    #[inline]
    fn ywyw(self) -> Self {
        Self::new(self.y, self.w, self.y, self.w)
    }

    #[inline]
    fn ywzx(self) -> Self {
        Self::new(self.y, self.w, self.z, self.x)
    }

    #[inline]
    fn ywzy(self) -> Self {
        Self::new(self.y, self.w, self.z, self.y)
    }

    #[inline]
    fn ywzz(self) -> Self {
        Self::new(self.y, self.w, self.z, self.z)
    }

    #[inline]
    fn ywzw(self) -> Self {
        Self::new(self.y, self.w, self.z, self.w)
    }

    #[inline]
    fn ywwx(self) -> Self {
        Self::new(self.y, self.w, self.w, self.x)
    }

    #[inline]
    fn ywwy(self) -> Self {
        Self::new(self.y, self.w, self.w, self.y)
    }

    #[inline]
    fn ywwz(self) -> Self {
        Self::new(self.y, self.w, self.w, self.z)
    }

    #[inline]
    fn ywww(self) -> Self {
        Self::new(self.y, self.w, self.w, self.w)
    }

    #[inline]
    fn zxxx(self) -> Self {
        Self::new(self.z, self.x, self.x, self.x)
    }

    #[inline]
    fn zxxy(self) -> Self {
        Self::new(self.z, self.x, self.x, self.y)
    }

    #[inline]
    fn zxxz(self) -> Self {
        Self::new(self.z, self.x, self.x, self.z)
    }

    #[inline]
    fn zxxw(self) -> Self {
        Self::new(self.z, self.x, self.x, self.w)
    }

    #[inline]
    fn zxyx(self) -> Self {
        Self::new(self.z, self.x, self.y, self.x)
    }

    #[inline]
    fn zxyy(self) -> Self {
        Self::new(self.z, self.x, self.y, self.y)
    }

    #[inline]
    fn zxyz(self) -> Self {
        Self::new(self.z, self.x, self.y, self.z)
    }

    #[inline]
    fn zxyw(self) -> Self {
        Self::new(self.z, self.x, self.y, self.w)
    }

    #[inline]
    fn zxzx(self) -> Self {
        Self::new(self.z, self.x, self.z, self.x)
    }

    #[inline]
    fn zxzy(self) -> Self {
        Self::new(self.z, self.x, self.z, self.y)
    }

    #[inline]
    fn zxzz(self) -> Self {
        Self::new(self.z, self.x, self.z, self.z)
    }

    #[inline]
    fn zxzw(self) -> Self {
        Self::new(self.z, self.x, self.z, self.w)
    }

    #[inline]
    fn zxwx(self) -> Self {
        Self::new(self.z, self.x, self.w, self.x)
    }

    #[inline]
    fn zxwy(self) -> Self {
        Self::new(self.z, self.x, self.w, self.y)
    }

    #[inline]
    fn zxwz(self) -> Self {
        Self::new(self.z, self.x, self.w, self.z)
    }

    #[inline]
    fn zxww(self) -> Self {
        Self::new(self.z, self.x, self.w, self.w)
    }

    #[inline]
    fn zyxx(self) -> Self {
        Self::new(self.z, self.y, self.x, self.x)
    }

    #[inline]
    fn zyxy(self) -> Self {
        Self::new(self.z, self.y, self.x, self.y)
    }

    #[inline]
    fn zyxz(self) -> Self {
        Self::new(self.z, self.y, self.x, self.z)
    }

    #[inline]
    fn zyxw(self) -> Self {
        Self::new(self.z, self.y, self.x, self.w)
    }

    #[inline]
    fn zyyx(self) -> Self {
        Self::new(self.z, self.y, self.y, self.x)
    }

    #[inline]
    fn zyyy(self) -> Self {
        Self::new(self.z, self.y, self.y, self.y)
    }

    #[inline]
    fn zyyz(self) -> Self {
        Self::new(self.z, self.y, self.y, self.z)
    }

    #[inline]
    fn zyyw(self) -> Self {
        Self::new(self.z, self.y, self.y, self.w)
    }

    #[inline]
    fn zyzx(self) -> Self {
        Self::new(self.z, self.y, self.z, self.x)
    }

    #[inline]
    fn zyzy(self) -> Self {
        Self::new(self.z, self.y, self.z, self.y)
    }

    #[inline]
    fn zyzz(self) -> Self {
        Self::new(self.z, self.y, self.z, self.z)
    }

    #[inline]
    fn zyzw(self) -> Self {
        Self::new(self.z, self.y, self.z, self.w)
    }

    #[inline]
    fn zywx(self) -> Self {
        Self::new(self.z, self.y, self.w, self.x)
    }

    #[inline]
    fn zywy(self) -> Self {
        Self::new(self.z, self.y, self.w, self.y)
    }

    #[inline]
    fn zywz(self) -> Self {
        Self::new(self.z, self.y, self.w, self.z)
    }

    #[inline]
    fn zyww(self) -> Self {
        Self::new(self.z, self.y, self.w, self.w)
    }

    #[inline]
    fn zzxx(self) -> Self {
        Self::new(self.z, self.z, self.x, self.x)
    }

    #[inline]
    fn zzxy(self) -> Self {
        Self::new(self.z, self.z, self.x, self.y)
    }

    #[inline]
    fn zzxz(self) -> Self {
        Self::new(self.z, self.z, self.x, self.z)
    }

    #[inline]
    fn zzxw(self) -> Self {
        Self::new(self.z, self.z, self.x, self.w)
    }

    #[inline]
    fn zzyx(self) -> Self {
        Self::new(self.z, self.z, self.y, self.x)
    }

    #[inline]
    fn zzyy(self) -> Self {
        Self::new(self.z, self.z, self.y, self.y)
    }

    #[inline]
    fn zzyz(self) -> Self {
        Self::new(self.z, self.z, self.y, self.z)
    }

    #[inline]
    fn zzyw(self) -> Self {
        Self::new(self.z, self.z, self.y, self.w)
    }

    #[inline]
    fn zzzx(self) -> Self {
        Self::new(self.z, self.z, self.z, self.x)
    }

    #[inline]
    fn zzzy(self) -> Self {
        Self::new(self.z, self.z, self.z, self.y)
    }

    #[inline]
    fn zzzz(self) -> Self {
        Self::new(self.z, self.z, self.z, self.z)
    }

    #[inline]
    fn zzzw(self) -> Self {
        Self::new(self.z, self.z, self.z, self.w)
    }

    #[inline]
    fn zzwx(self) -> Self {
        Self::new(self.z, self.z, self.w, self.x)
    }

    #[inline]
    fn zzwy(self) -> Self {
        Self::new(self.z, self.z, self.w, self.y)
    }

    #[inline]
    fn zzwz(self) -> Self {
        Self::new(self.z, self.z, self.w, self.z)
    }

    #[inline]
    fn zzww(self) -> Self {
        Self::new(self.z, self.z, self.w, self.w)
    }

    #[inline]
    fn zwxx(self) -> Self {
        Self::new(self.z, self.w, self.x, self.x)
    }

    #[inline]
    fn zwxy(self) -> Self {
        Self::new(self.z, self.w, self.x, self.y)
    }

    #[inline]
    fn zwxz(self) -> Self {
        Self::new(self.z, self.w, self.x, self.z)
    }

    #[inline]
    fn zwxw(self) -> Self {
        Self::new(self.z, self.w, self.x, self.w)
    }

    #[inline]
    fn zwyx(self) -> Self {
        Self::new(self.z, self.w, self.y, self.x)
    }

    #[inline]
    fn zwyy(self) -> Self {
        Self::new(self.z, self.w, self.y, self.y)
    }

    #[inline]
    fn zwyz(self) -> Self {
        Self::new(self.z, self.w, self.y, self.z)
    }

    #[inline]
    fn zwyw(self) -> Self {
        Self::new(self.z, self.w, self.y, self.w)
    }

    #[inline]
    fn zwzx(self) -> Self {
        Self::new(self.z, self.w, self.z, self.x)
    }

    #[inline]
    fn zwzy(self) -> Self {
        Self::new(self.z, self.w, self.z, self.y)
    }

    #[inline]
    fn zwzz(self) -> Self {
        Self::new(self.z, self.w, self.z, self.z)
    }

    #[inline]
    fn zwzw(self) -> Self {
        Self::new(self.z, self.w, self.z, self.w)
    }

    #[inline]
    fn zwwx(self) -> Self {
        Self::new(self.z, self.w, self.w, self.x)
    }

    #[inline]
    fn zwwy(self) -> Self {
        Self::new(self.z, self.w, self.w, self.y)
    }

    #[inline]
    fn zwwz(self) -> Self {
        Self::new(self.z, self.w, self.w, self.z)
    }

    #[inline]
    fn zwww(self) -> Self {
        Self::new(self.z, self.w, self.w, self.w)
    }

    #[inline]
    fn wxxx(self) -> Self {
        Self::new(self.w, self.x, self.x, self.x)
    }

    #[inline]
    fn wxxy(self) -> Self {
        Self::new(self.w, self.x, self.x, self.y)
    }

    #[inline]
    fn wxxz(self) -> Self {
        Self::new(self.w, self.x, self.x, self.z)
    }

    #[inline]
    fn wxxw(self) -> Self {
        Self::new(self.w, self.x, self.x, self.w)
    }

    #[inline]
    fn wxyx(self) -> Self {
        Self::new(self.w, self.x, self.y, self.x)
    }

    #[inline]
    fn wxyy(self) -> Self {
        Self::new(self.w, self.x, self.y, self.y)
    }

    #[inline]
    fn wxyz(self) -> Self {
        Self::new(self.w, self.x, self.y, self.z)
    }

    #[inline]
    fn wxyw(self) -> Self {
        Self::new(self.w, self.x, self.y, self.w)
    }

    #[inline]
    fn wxzx(self) -> Self {
        Self::new(self.w, self.x, self.z, self.x)
    }

    #[inline]
    fn wxzy(self) -> Self {
        Self::new(self.w, self.x, self.z, self.y)
    }

    #[inline]
    fn wxzz(self) -> Self {
        Self::new(self.w, self.x, self.z, self.z)
    }

    #[inline]
    fn wxzw(self) -> Self {
        Self::new(self.w, self.x, self.z, self.w)
    }

    #[inline]
    fn wxwx(self) -> Self {
        Self::new(self.w, self.x, self.w, self.x)
    }

    #[inline]
    fn wxwy(self) -> Self {
        Self::new(self.w, self.x, self.w, self.y)
    }

    #[inline]
    fn wxwz(self) -> Self {
        Self::new(self.w, self.x, self.w, self.z)
    }

    #[inline]
    fn wxww(self) -> Self {
        Self::new(self.w, self.x, self.w, self.w)
    }

    #[inline]
    fn wyxx(self) -> Self {
        Self::new(self.w, self.y, self.x, self.x)
    }

    #[inline]
    fn wyxy(self) -> Self {
        Self::new(self.w, self.y, self.x, self.y)
    }

    #[inline]
    fn wyxz(self) -> Self {
        Self::new(self.w, self.y, self.x, self.z)
    }

    #[inline]
    fn wyxw(self) -> Self {
        Self::new(self.w, self.y, self.x, self.w)
    }

    #[inline]
    fn wyyx(self) -> Self {
        Self::new(self.w, self.y, self.y, self.x)
    }

    #[inline]
    fn wyyy(self) -> Self {
        Self::new(self.w, self.y, self.y, self.y)
    }

    #[inline]
    fn wyyz(self) -> Self {
        Self::new(self.w, self.y, self.y, self.z)
    }

    #[inline]
    fn wyyw(self) -> Self {
        Self::new(self.w, self.y, self.y, self.w)
    }

    #[inline]
    fn wyzx(self) -> Self {
        Self::new(self.w, self.y, self.z, self.x)
    }

    #[inline]
    fn wyzy(self) -> Self {
        Self::new(self.w, self.y, self.z, self.y)
    }

    #[inline]
    fn wyzz(self) -> Self {
        Self::new(self.w, self.y, self.z, self.z)
    }

    #[inline]
    fn wyzw(self) -> Self {
        Self::new(self.w, self.y, self.z, self.w)
    }

    #[inline]
    fn wywx(self) -> Self {
        Self::new(self.w, self.y, self.w, self.x)
    }

    #[inline]
    fn wywy(self) -> Self {
        Self::new(self.w, self.y, self.w, self.y)
    }

    #[inline]
    fn wywz(self) -> Self {
        Self::new(self.w, self.y, self.w, self.z)
    }

    #[inline]
    fn wyww(self) -> Self {
        Self::new(self.w, self.y, self.w, self.w)
    }

    #[inline]
    fn wzxx(self) -> Self {
        Self::new(self.w, self.z, self.x, self.x)
    }

    #[inline]
    fn wzxy(self) -> Self {
        Self::new(self.w, self.z, self.x, self.y)
    }

    #[inline]
    fn wzxz(self) -> Self {
        Self::new(self.w, self.z, self.x, self.z)
    }

    #[inline]
    fn wzxw(self) -> Self {
        Self::new(self.w, self.z, self.x, self.w)
    }

    #[inline]
    fn wzyx(self) -> Self {
        Self::new(self.w, self.z, self.y, self.x)
    }

    #[inline]
    fn wzyy(self) -> Self {
        Self::new(self.w, self.z, self.y, self.y)
    }

    #[inline]
    fn wzyz(self) -> Self {
        Self::new(self.w, self.z, self.y, self.z)
    }

    #[inline]
    fn wzyw(self) -> Self {
        Self::new(self.w, self.z, self.y, self.w)
    }

    #[inline]
    fn wzzx(self) -> Self {
        Self::new(self.w, self.z, self.z, self.x)
    }

    #[inline]
    fn wzzy(self) -> Self {
        Self::new(self.w, self.z, self.z, self.y)
    }

    #[inline]
    fn wzzz(self) -> Self {
        Self::new(self.w, self.z, self.z, self.z)
    }

    #[inline]
    fn wzzw(self) -> Self {
        Self::new(self.w, self.z, self.z, self.w)
    }

    #[inline]
    fn wzwx(self) -> Self {
        Self::new(self.w, self.z, self.w, self.x)
    }

    #[inline]
    fn wzwy(self) -> Self {
        Self::new(self.w, self.z, self.w, self.y)
    }

    #[inline]
    fn wzwz(self) -> Self {
        Self::new(self.w, self.z, self.w, self.z)
    }

    #[inline]
    fn wzww(self) -> Self {
        Self::new(self.w, self.z, self.w, self.w)
    }

    #[inline]
    fn wwxx(self) -> Self {
        Self::new(self.w, self.w, self.x, self.x)
    }

    #[inline]
    fn wwxy(self) -> Self {
        Self::new(self.w, self.w, self.x, self.y)
    }

    #[inline]
    fn wwxz(self) -> Self {
        Self::new(self.w, self.w, self.x, self.z)
    }

    #[inline]
    fn wwxw(self) -> Self {
        Self::new(self.w, self.w, self.x, self.w)
    }

    #[inline]
    fn wwyx(self) -> Self {
        Self::new(self.w, self.w, self.y, self.x)
    }

    #[inline]
    fn wwyy(self) -> Self {
        Self::new(self.w, self.w, self.y, self.y)
    }

    #[inline]
    fn wwyz(self) -> Self {
        Self::new(self.w, self.w, self.y, self.z)
    }

    #[inline]
    fn wwyw(self) -> Self {
        Self::new(self.w, self.w, self.y, self.w)
    }

    #[inline]
    fn wwzx(self) -> Self {
        Self::new(self.w, self.w, self.z, self.x)
    }

    #[inline]
    fn wwzy(self) -> Self {
        Self::new(self.w, self.w, self.z, self.y)
    }

    #[inline]
    fn wwzz(self) -> Self {
        Self::new(self.w, self.w, self.z, self.z)
    }

    #[inline]
    fn wwzw(self) -> Self {
        Self::new(self.w, self.w, self.z, self.w)
    }

    #[inline]
    fn wwwx(self) -> Self {
        Self::new(self.w, self.w, self.w, self.x)
    }

    #[inline]
    fn wwwy(self) -> Self {
        Self::new(self.w, self.w, self.w, self.y)
    }

    #[inline]
    fn wwwz(self) -> Self {
        Self::new(self.w, self.w, self.w, self.z)
    }

    #[inline]
    fn wwww(self) -> Self {
        Self::new(self.w, self.w, self.w, self.w)
    }
}
