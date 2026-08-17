// Generated from swizzle_impl.rs.tera template. Edit the template, not the generated file.

use crate::{U8Vec2, U8Vec3, U8Vec4, Vec3Swizzles};

impl Vec3Swizzles for U8Vec3 {
    type Vec2 = U8Vec2;

    type Vec4 = U8Vec4;

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
}
