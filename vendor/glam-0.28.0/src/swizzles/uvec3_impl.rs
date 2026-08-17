// Generated from swizzle_impl.rs.tera template. Edit the template, not the generated file.

use crate::{UVec2, UVec3, UVec4, Vec3Swizzles};

impl Vec3Swizzles for UVec3 {
    type Vec2 = UVec2;

    type Vec4 = UVec4;

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
}
