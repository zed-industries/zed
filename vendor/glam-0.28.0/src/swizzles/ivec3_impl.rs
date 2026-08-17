// Generated from swizzle_impl.rs.tera template. Edit the template, not the generated file.

use crate::{IVec2, IVec3, IVec4, Vec3Swizzles};

impl Vec3Swizzles for IVec3 {
    type Vec2 = IVec2;

    type Vec4 = IVec4;

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
}
