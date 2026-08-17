// Generated from swizzle_impl.rs.tera template. Edit the template, not the generated file.

use crate::{DVec2, DVec3, DVec4, Vec3Swizzles};

impl Vec3Swizzles for DVec3 {
    type Vec2 = DVec2;

    type Vec4 = DVec4;

    #[inline]
    fn xx(self) -> DVec2 {
        DVec2 {
            x: self.x,
            y: self.x,
        }
    }

    #[inline]
    fn xy(self) -> DVec2 {
        DVec2 {
            x: self.x,
            y: self.y,
        }
    }

    #[inline]
    fn with_xy(self, rhs: DVec2) -> Self {
        Self::new(rhs.x, rhs.y, self.z)
    }

    #[inline]
    fn xz(self) -> DVec2 {
        DVec2 {
            x: self.x,
            y: self.z,
        }
    }

    #[inline]
    fn with_xz(self, rhs: DVec2) -> Self {
        Self::new(rhs.x, self.y, rhs.y)
    }

    #[inline]
    fn yx(self) -> DVec2 {
        DVec2 {
            x: self.y,
            y: self.x,
        }
    }

    #[inline]
    fn with_yx(self, rhs: DVec2) -> Self {
        Self::new(rhs.y, rhs.x, self.z)
    }

    #[inline]
    fn yy(self) -> DVec2 {
        DVec2 {
            x: self.y,
            y: self.y,
        }
    }

    #[inline]
    fn yz(self) -> DVec2 {
        DVec2 {
            x: self.y,
            y: self.z,
        }
    }

    #[inline]
    fn with_yz(self, rhs: DVec2) -> Self {
        Self::new(self.x, rhs.x, rhs.y)
    }

    #[inline]
    fn zx(self) -> DVec2 {
        DVec2 {
            x: self.z,
            y: self.x,
        }
    }

    #[inline]
    fn with_zx(self, rhs: DVec2) -> Self {
        Self::new(rhs.y, self.y, rhs.x)
    }

    #[inline]
    fn zy(self) -> DVec2 {
        DVec2 {
            x: self.z,
            y: self.y,
        }
    }

    #[inline]
    fn with_zy(self, rhs: DVec2) -> Self {
        Self::new(self.x, rhs.y, rhs.x)
    }

    #[inline]
    fn zz(self) -> DVec2 {
        DVec2 {
            x: self.z,
            y: self.z,
        }
    }

    #[inline]
    fn xxx(self) -> Self {
        Self::new(self.x, self.x, self.x)
    }

    #[inline]
    fn xxy(self) -> Self {
        Self::new(self.x, self.x, self.y)
    }

    #[inline]
    fn xxz(self) -> Self {
        Self::new(self.x, self.x, self.z)
    }

    #[inline]
    fn xyx(self) -> Self {
        Self::new(self.x, self.y, self.x)
    }

    #[inline]
    fn xyy(self) -> Self {
        Self::new(self.x, self.y, self.y)
    }

    #[inline]
    fn xzx(self) -> Self {
        Self::new(self.x, self.z, self.x)
    }

    #[inline]
    fn xzy(self) -> Self {
        Self::new(self.x, self.z, self.y)
    }

    #[inline]
    fn xzz(self) -> Self {
        Self::new(self.x, self.z, self.z)
    }

    #[inline]
    fn yxx(self) -> Self {
        Self::new(self.y, self.x, self.x)
    }

    #[inline]
    fn yxy(self) -> Self {
        Self::new(self.y, self.x, self.y)
    }

    #[inline]
    fn yxz(self) -> Self {
        Self::new(self.y, self.x, self.z)
    }

    #[inline]
    fn yyx(self) -> Self {
        Self::new(self.y, self.y, self.x)
    }

    #[inline]
    fn yyy(self) -> Self {
        Self::new(self.y, self.y, self.y)
    }

    #[inline]
    fn yyz(self) -> Self {
        Self::new(self.y, self.y, self.z)
    }

    #[inline]
    fn yzx(self) -> Self {
        Self::new(self.y, self.z, self.x)
    }

    #[inline]
    fn yzy(self) -> Self {
        Self::new(self.y, self.z, self.y)
    }

    #[inline]
    fn yzz(self) -> Self {
        Self::new(self.y, self.z, self.z)
    }

    #[inline]
    fn zxx(self) -> Self {
        Self::new(self.z, self.x, self.x)
    }

    #[inline]
    fn zxy(self) -> Self {
        Self::new(self.z, self.x, self.y)
    }

    #[inline]
    fn zxz(self) -> Self {
        Self::new(self.z, self.x, self.z)
    }

    #[inline]
    fn zyx(self) -> Self {
        Self::new(self.z, self.y, self.x)
    }

    #[inline]
    fn zyy(self) -> Self {
        Self::new(self.z, self.y, self.y)
    }

    #[inline]
    fn zyz(self) -> Self {
        Self::new(self.z, self.y, self.z)
    }

    #[inline]
    fn zzx(self) -> Self {
        Self::new(self.z, self.z, self.x)
    }

    #[inline]
    fn zzy(self) -> Self {
        Self::new(self.z, self.z, self.y)
    }

    #[inline]
    fn zzz(self) -> Self {
        Self::new(self.z, self.z, self.z)
    }

    #[inline]
    fn xxxx(self) -> DVec4 {
        DVec4::new(self.x, self.x, self.x, self.x)
    }

    #[inline]
    fn xxxy(self) -> DVec4 {
        DVec4::new(self.x, self.x, self.x, self.y)
    }

    #[inline]
    fn xxxz(self) -> DVec4 {
        DVec4::new(self.x, self.x, self.x, self.z)
    }

    #[inline]
    fn xxyx(self) -> DVec4 {
        DVec4::new(self.x, self.x, self.y, self.x)
    }

    #[inline]
    fn xxyy(self) -> DVec4 {
        DVec4::new(self.x, self.x, self.y, self.y)
    }

    #[inline]
    fn xxyz(self) -> DVec4 {
        DVec4::new(self.x, self.x, self.y, self.z)
    }

    #[inline]
    fn xxzx(self) -> DVec4 {
        DVec4::new(self.x, self.x, self.z, self.x)
    }

    #[inline]
    fn xxzy(self) -> DVec4 {
        DVec4::new(self.x, self.x, self.z, self.y)
    }

    #[inline]
    fn xxzz(self) -> DVec4 {
        DVec4::new(self.x, self.x, self.z, self.z)
    }

    #[inline]
    fn xyxx(self) -> DVec4 {
        DVec4::new(self.x, self.y, self.x, self.x)
    }

    #[inline]
    fn xyxy(self) -> DVec4 {
        DVec4::new(self.x, self.y, self.x, self.y)
    }

    #[inline]
    fn xyxz(self) -> DVec4 {
        DVec4::new(self.x, self.y, self.x, self.z)
    }

    #[inline]
    fn xyyx(self) -> DVec4 {
        DVec4::new(self.x, self.y, self.y, self.x)
    }

    #[inline]
    fn xyyy(self) -> DVec4 {
        DVec4::new(self.x, self.y, self.y, self.y)
    }

    #[inline]
    fn xyyz(self) -> DVec4 {
        DVec4::new(self.x, self.y, self.y, self.z)
    }

    #[inline]
    fn xyzx(self) -> DVec4 {
        DVec4::new(self.x, self.y, self.z, self.x)
    }

    #[inline]
    fn xyzy(self) -> DVec4 {
        DVec4::new(self.x, self.y, self.z, self.y)
    }

    #[inline]
    fn xyzz(self) -> DVec4 {
        DVec4::new(self.x, self.y, self.z, self.z)
    }

    #[inline]
    fn xzxx(self) -> DVec4 {
        DVec4::new(self.x, self.z, self.x, self.x)
    }

    #[inline]
    fn xzxy(self) -> DVec4 {
        DVec4::new(self.x, self.z, self.x, self.y)
    }

    #[inline]
    fn xzxz(self) -> DVec4 {
        DVec4::new(self.x, self.z, self.x, self.z)
    }

    #[inline]
    fn xzyx(self) -> DVec4 {
        DVec4::new(self.x, self.z, self.y, self.x)
    }

    #[inline]
    fn xzyy(self) -> DVec4 {
        DVec4::new(self.x, self.z, self.y, self.y)
    }

    #[inline]
    fn xzyz(self) -> DVec4 {
        DVec4::new(self.x, self.z, self.y, self.z)
    }

    #[inline]
    fn xzzx(self) -> DVec4 {
        DVec4::new(self.x, self.z, self.z, self.x)
    }

    #[inline]
    fn xzzy(self) -> DVec4 {
        DVec4::new(self.x, self.z, self.z, self.y)
    }

    #[inline]
    fn xzzz(self) -> DVec4 {
        DVec4::new(self.x, self.z, self.z, self.z)
    }

    #[inline]
    fn yxxx(self) -> DVec4 {
        DVec4::new(self.y, self.x, self.x, self.x)
    }

    #[inline]
    fn yxxy(self) -> DVec4 {
        DVec4::new(self.y, self.x, self.x, self.y)
    }

    #[inline]
    fn yxxz(self) -> DVec4 {
        DVec4::new(self.y, self.x, self.x, self.z)
    }

    #[inline]
    fn yxyx(self) -> DVec4 {
        DVec4::new(self.y, self.x, self.y, self.x)
    }

    #[inline]
    fn yxyy(self) -> DVec4 {
        DVec4::new(self.y, self.x, self.y, self.y)
    }

    #[inline]
    fn yxyz(self) -> DVec4 {
        DVec4::new(self.y, self.x, self.y, self.z)
    }

    #[inline]
    fn yxzx(self) -> DVec4 {
        DVec4::new(self.y, self.x, self.z, self.x)
    }

    #[inline]
    fn yxzy(self) -> DVec4 {
        DVec4::new(self.y, self.x, self.z, self.y)
    }

    #[inline]
    fn yxzz(self) -> DVec4 {
        DVec4::new(self.y, self.x, self.z, self.z)
    }

    #[inline]
    fn yyxx(self) -> DVec4 {
        DVec4::new(self.y, self.y, self.x, self.x)
    }

    #[inline]
    fn yyxy(self) -> DVec4 {
        DVec4::new(self.y, self.y, self.x, self.y)
    }

    #[inline]
    fn yyxz(self) -> DVec4 {
        DVec4::new(self.y, self.y, self.x, self.z)
    }

    #[inline]
    fn yyyx(self) -> DVec4 {
        DVec4::new(self.y, self.y, self.y, self.x)
    }

    #[inline]
    fn yyyy(self) -> DVec4 {
        DVec4::new(self.y, self.y, self.y, self.y)
    }

    #[inline]
    fn yyyz(self) -> DVec4 {
        DVec4::new(self.y, self.y, self.y, self.z)
    }

    #[inline]
    fn yyzx(self) -> DVec4 {
        DVec4::new(self.y, self.y, self.z, self.x)
    }

    #[inline]
    fn yyzy(self) -> DVec4 {
        DVec4::new(self.y, self.y, self.z, self.y)
    }

    #[inline]
    fn yyzz(self) -> DVec4 {
        DVec4::new(self.y, self.y, self.z, self.z)
    }

    #[inline]
    fn yzxx(self) -> DVec4 {
        DVec4::new(self.y, self.z, self.x, self.x)
    }

    #[inline]
    fn yzxy(self) -> DVec4 {
        DVec4::new(self.y, self.z, self.x, self.y)
    }

    #[inline]
    fn yzxz(self) -> DVec4 {
        DVec4::new(self.y, self.z, self.x, self.z)
    }

    #[inline]
    fn yzyx(self) -> DVec4 {
        DVec4::new(self.y, self.z, self.y, self.x)
    }

    #[inline]
    fn yzyy(self) -> DVec4 {
        DVec4::new(self.y, self.z, self.y, self.y)
    }

    #[inline]
    fn yzyz(self) -> DVec4 {
        DVec4::new(self.y, self.z, self.y, self.z)
    }

    #[inline]
    fn yzzx(self) -> DVec4 {
        DVec4::new(self.y, self.z, self.z, self.x)
    }

    #[inline]
    fn yzzy(self) -> DVec4 {
        DVec4::new(self.y, self.z, self.z, self.y)
    }

    #[inline]
    fn yzzz(self) -> DVec4 {
        DVec4::new(self.y, self.z, self.z, self.z)
    }

    #[inline]
    fn zxxx(self) -> DVec4 {
        DVec4::new(self.z, self.x, self.x, self.x)
    }

    #[inline]
    fn zxxy(self) -> DVec4 {
        DVec4::new(self.z, self.x, self.x, self.y)
    }

    #[inline]
    fn zxxz(self) -> DVec4 {
        DVec4::new(self.z, self.x, self.x, self.z)
    }

    #[inline]
    fn zxyx(self) -> DVec4 {
        DVec4::new(self.z, self.x, self.y, self.x)
    }

    #[inline]
    fn zxyy(self) -> DVec4 {
        DVec4::new(self.z, self.x, self.y, self.y)
    }

    #[inline]
    fn zxyz(self) -> DVec4 {
        DVec4::new(self.z, self.x, self.y, self.z)
    }

    #[inline]
    fn zxzx(self) -> DVec4 {
        DVec4::new(self.z, self.x, self.z, self.x)
    }

    #[inline]
    fn zxzy(self) -> DVec4 {
        DVec4::new(self.z, self.x, self.z, self.y)
    }

    #[inline]
    fn zxzz(self) -> DVec4 {
        DVec4::new(self.z, self.x, self.z, self.z)
    }

    #[inline]
    fn zyxx(self) -> DVec4 {
        DVec4::new(self.z, self.y, self.x, self.x)
    }

    #[inline]
    fn zyxy(self) -> DVec4 {
        DVec4::new(self.z, self.y, self.x, self.y)
    }

    #[inline]
    fn zyxz(self) -> DVec4 {
        DVec4::new(self.z, self.y, self.x, self.z)
    }

    #[inline]
    fn zyyx(self) -> DVec4 {
        DVec4::new(self.z, self.y, self.y, self.x)
    }

    #[inline]
    fn zyyy(self) -> DVec4 {
        DVec4::new(self.z, self.y, self.y, self.y)
    }

    #[inline]
    fn zyyz(self) -> DVec4 {
        DVec4::new(self.z, self.y, self.y, self.z)
    }

    #[inline]
    fn zyzx(self) -> DVec4 {
        DVec4::new(self.z, self.y, self.z, self.x)
    }

    #[inline]
    fn zyzy(self) -> DVec4 {
        DVec4::new(self.z, self.y, self.z, self.y)
    }

    #[inline]
    fn zyzz(self) -> DVec4 {
        DVec4::new(self.z, self.y, self.z, self.z)
    }

    #[inline]
    fn zzxx(self) -> DVec4 {
        DVec4::new(self.z, self.z, self.x, self.x)
    }

    #[inline]
    fn zzxy(self) -> DVec4 {
        DVec4::new(self.z, self.z, self.x, self.y)
    }

    #[inline]
    fn zzxz(self) -> DVec4 {
        DVec4::new(self.z, self.z, self.x, self.z)
    }

    #[inline]
    fn zzyx(self) -> DVec4 {
        DVec4::new(self.z, self.z, self.y, self.x)
    }

    #[inline]
    fn zzyy(self) -> DVec4 {
        DVec4::new(self.z, self.z, self.y, self.y)
    }

    #[inline]
    fn zzyz(self) -> DVec4 {
        DVec4::new(self.z, self.z, self.y, self.z)
    }

    #[inline]
    fn zzzx(self) -> DVec4 {
        DVec4::new(self.z, self.z, self.z, self.x)
    }

    #[inline]
    fn zzzy(self) -> DVec4 {
        DVec4::new(self.z, self.z, self.z, self.y)
    }

    #[inline]
    fn zzzz(self) -> DVec4 {
        DVec4::new(self.z, self.z, self.z, self.z)
    }
}
