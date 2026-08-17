// Generated from swizzle_impl.rs.tera template. Edit the template, not the generated file.

use crate::{I64Vec2, I64Vec3, I64Vec4, Vec3Swizzles};

impl Vec3Swizzles for I64Vec3 {
    type Vec2 = I64Vec2;

    type Vec4 = I64Vec4;

    #[inline]
    #[must_use]
    fn xx(self) -> I64Vec2 {
        I64Vec2 {
            x: self.x,
            y: self.x,
        }
    }

    #[inline]
    #[must_use]
    fn xy(self) -> I64Vec2 {
        I64Vec2 {
            x: self.x,
            y: self.y,
        }
    }

    #[inline]
    #[must_use]
    fn xz(self) -> I64Vec2 {
        I64Vec2 {
            x: self.x,
            y: self.z,
        }
    }

    #[inline]
    #[must_use]
    fn yx(self) -> I64Vec2 {
        I64Vec2 {
            x: self.y,
            y: self.x,
        }
    }

    #[inline]
    #[must_use]
    fn yy(self) -> I64Vec2 {
        I64Vec2 {
            x: self.y,
            y: self.y,
        }
    }

    #[inline]
    #[must_use]
    fn yz(self) -> I64Vec2 {
        I64Vec2 {
            x: self.y,
            y: self.z,
        }
    }

    #[inline]
    #[must_use]
    fn zx(self) -> I64Vec2 {
        I64Vec2 {
            x: self.z,
            y: self.x,
        }
    }

    #[inline]
    #[must_use]
    fn zy(self) -> I64Vec2 {
        I64Vec2 {
            x: self.z,
            y: self.y,
        }
    }

    #[inline]
    #[must_use]
    fn zz(self) -> I64Vec2 {
        I64Vec2 {
            x: self.z,
            y: self.z,
        }
    }

    #[inline]
    #[must_use]
    fn xxx(self) -> I64Vec3 {
        I64Vec3::new(self.x, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn xxy(self) -> I64Vec3 {
        I64Vec3::new(self.x, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn xxz(self) -> I64Vec3 {
        I64Vec3::new(self.x, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn xyx(self) -> I64Vec3 {
        I64Vec3::new(self.x, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn xyy(self) -> I64Vec3 {
        I64Vec3::new(self.x, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn xyz(self) -> I64Vec3 {
        I64Vec3::new(self.x, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn xzx(self) -> I64Vec3 {
        I64Vec3::new(self.x, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn xzy(self) -> I64Vec3 {
        I64Vec3::new(self.x, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn xzz(self) -> I64Vec3 {
        I64Vec3::new(self.x, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn yxx(self) -> I64Vec3 {
        I64Vec3::new(self.y, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn yxy(self) -> I64Vec3 {
        I64Vec3::new(self.y, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn yxz(self) -> I64Vec3 {
        I64Vec3::new(self.y, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn yyx(self) -> I64Vec3 {
        I64Vec3::new(self.y, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn yyy(self) -> I64Vec3 {
        I64Vec3::new(self.y, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn yyz(self) -> I64Vec3 {
        I64Vec3::new(self.y, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn yzx(self) -> I64Vec3 {
        I64Vec3::new(self.y, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn yzy(self) -> I64Vec3 {
        I64Vec3::new(self.y, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn yzz(self) -> I64Vec3 {
        I64Vec3::new(self.y, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn zxx(self) -> I64Vec3 {
        I64Vec3::new(self.z, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn zxy(self) -> I64Vec3 {
        I64Vec3::new(self.z, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn zxz(self) -> I64Vec3 {
        I64Vec3::new(self.z, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn zyx(self) -> I64Vec3 {
        I64Vec3::new(self.z, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn zyy(self) -> I64Vec3 {
        I64Vec3::new(self.z, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn zyz(self) -> I64Vec3 {
        I64Vec3::new(self.z, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn zzx(self) -> I64Vec3 {
        I64Vec3::new(self.z, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn zzy(self) -> I64Vec3 {
        I64Vec3::new(self.z, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn zzz(self) -> I64Vec3 {
        I64Vec3::new(self.z, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn xxxx(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.x, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn xxxy(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.x, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn xxxz(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.x, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn xxyx(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.x, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn xxyy(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.x, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn xxyz(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.x, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn xxzx(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.x, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn xxzy(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.x, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn xxzz(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.x, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn xyxx(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.y, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn xyxy(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.y, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn xyxz(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.y, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn xyyx(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.y, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn xyyy(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.y, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn xyyz(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.y, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn xyzx(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.y, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn xyzy(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.y, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn xyzz(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.y, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn xzxx(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.z, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn xzxy(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.z, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn xzxz(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.z, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn xzyx(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.z, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn xzyy(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.z, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn xzyz(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.z, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn xzzx(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.z, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn xzzy(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.z, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn xzzz(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.z, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn yxxx(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.x, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn yxxy(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.x, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn yxxz(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.x, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn yxyx(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.x, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn yxyy(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.x, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn yxyz(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.x, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn yxzx(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.x, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn yxzy(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.x, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn yxzz(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.x, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn yyxx(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.y, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn yyxy(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.y, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn yyxz(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.y, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn yyyx(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.y, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn yyyy(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.y, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn yyyz(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.y, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn yyzx(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.y, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn yyzy(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.y, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn yyzz(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.y, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn yzxx(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.z, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn yzxy(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.z, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn yzxz(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.z, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn yzyx(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.z, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn yzyy(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.z, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn yzyz(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.z, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn yzzx(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.z, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn yzzy(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.z, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn yzzz(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.z, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn zxxx(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.x, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn zxxy(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.x, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn zxxz(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.x, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn zxyx(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.x, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn zxyy(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.x, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn zxyz(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.x, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn zxzx(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.x, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn zxzy(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.x, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn zxzz(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.x, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn zyxx(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.y, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn zyxy(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.y, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn zyxz(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.y, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn zyyx(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.y, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn zyyy(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.y, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn zyyz(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.y, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn zyzx(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.y, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn zyzy(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.y, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn zyzz(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.y, self.z, self.z)
    }

    #[inline]
    #[must_use]
    fn zzxx(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.z, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn zzxy(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.z, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn zzxz(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.z, self.x, self.z)
    }

    #[inline]
    #[must_use]
    fn zzyx(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.z, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn zzyy(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.z, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn zzyz(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.z, self.y, self.z)
    }

    #[inline]
    #[must_use]
    fn zzzx(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.z, self.z, self.x)
    }

    #[inline]
    #[must_use]
    fn zzzy(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.z, self.z, self.y)
    }

    #[inline]
    #[must_use]
    fn zzzz(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.z, self.z, self.z)
    }
}
