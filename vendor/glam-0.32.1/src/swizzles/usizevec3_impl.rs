// Generated from swizzle_impl.rs.tera template. Edit the template, not the generated file.

use crate::{USizeVec2, USizeVec3, USizeVec4, Vec3Swizzles};

impl Vec3Swizzles for USizeVec3 {
    type Vec2 = USizeVec2;

    type Vec4 = USizeVec4;

    #[inline]
    fn xx(self) -> USizeVec2 {
        USizeVec2 {
            x: self.x,
            y: self.x,
        }
    }

    #[inline]
    fn xy(self) -> USizeVec2 {
        USizeVec2 {
            x: self.x,
            y: self.y,
        }
    }

    #[inline]
    fn with_xy(self, rhs: USizeVec2) -> Self {
        Self::new(rhs.x, rhs.y, self.z)
    }

    #[inline]
    fn xz(self) -> USizeVec2 {
        USizeVec2 {
            x: self.x,
            y: self.z,
        }
    }

    #[inline]
    fn with_xz(self, rhs: USizeVec2) -> Self {
        Self::new(rhs.x, self.y, rhs.y)
    }

    #[inline]
    fn yx(self) -> USizeVec2 {
        USizeVec2 {
            x: self.y,
            y: self.x,
        }
    }

    #[inline]
    fn with_yx(self, rhs: USizeVec2) -> Self {
        Self::new(rhs.y, rhs.x, self.z)
    }

    #[inline]
    fn yy(self) -> USizeVec2 {
        USizeVec2 {
            x: self.y,
            y: self.y,
        }
    }

    #[inline]
    fn yz(self) -> USizeVec2 {
        USizeVec2 {
            x: self.y,
            y: self.z,
        }
    }

    #[inline]
    fn with_yz(self, rhs: USizeVec2) -> Self {
        Self::new(self.x, rhs.x, rhs.y)
    }

    #[inline]
    fn zx(self) -> USizeVec2 {
        USizeVec2 {
            x: self.z,
            y: self.x,
        }
    }

    #[inline]
    fn with_zx(self, rhs: USizeVec2) -> Self {
        Self::new(rhs.y, self.y, rhs.x)
    }

    #[inline]
    fn zy(self) -> USizeVec2 {
        USizeVec2 {
            x: self.z,
            y: self.y,
        }
    }

    #[inline]
    fn with_zy(self, rhs: USizeVec2) -> Self {
        Self::new(self.x, rhs.y, rhs.x)
    }

    #[inline]
    fn zz(self) -> USizeVec2 {
        USizeVec2 {
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
    fn xxxx(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.x, self.x, self.x)
    }

    #[inline]
    fn xxxy(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.x, self.x, self.y)
    }

    #[inline]
    fn xxxz(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.x, self.x, self.z)
    }

    #[inline]
    fn xxyx(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.x, self.y, self.x)
    }

    #[inline]
    fn xxyy(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.x, self.y, self.y)
    }

    #[inline]
    fn xxyz(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.x, self.y, self.z)
    }

    #[inline]
    fn xxzx(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.x, self.z, self.x)
    }

    #[inline]
    fn xxzy(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.x, self.z, self.y)
    }

    #[inline]
    fn xxzz(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.x, self.z, self.z)
    }

    #[inline]
    fn xyxx(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.y, self.x, self.x)
    }

    #[inline]
    fn xyxy(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.y, self.x, self.y)
    }

    #[inline]
    fn xyxz(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.y, self.x, self.z)
    }

    #[inline]
    fn xyyx(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.y, self.y, self.x)
    }

    #[inline]
    fn xyyy(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.y, self.y, self.y)
    }

    #[inline]
    fn xyyz(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.y, self.y, self.z)
    }

    #[inline]
    fn xyzx(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.y, self.z, self.x)
    }

    #[inline]
    fn xyzy(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.y, self.z, self.y)
    }

    #[inline]
    fn xyzz(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.y, self.z, self.z)
    }

    #[inline]
    fn xzxx(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.z, self.x, self.x)
    }

    #[inline]
    fn xzxy(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.z, self.x, self.y)
    }

    #[inline]
    fn xzxz(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.z, self.x, self.z)
    }

    #[inline]
    fn xzyx(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.z, self.y, self.x)
    }

    #[inline]
    fn xzyy(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.z, self.y, self.y)
    }

    #[inline]
    fn xzyz(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.z, self.y, self.z)
    }

    #[inline]
    fn xzzx(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.z, self.z, self.x)
    }

    #[inline]
    fn xzzy(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.z, self.z, self.y)
    }

    #[inline]
    fn xzzz(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.z, self.z, self.z)
    }

    #[inline]
    fn yxxx(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.x, self.x, self.x)
    }

    #[inline]
    fn yxxy(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.x, self.x, self.y)
    }

    #[inline]
    fn yxxz(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.x, self.x, self.z)
    }

    #[inline]
    fn yxyx(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.x, self.y, self.x)
    }

    #[inline]
    fn yxyy(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.x, self.y, self.y)
    }

    #[inline]
    fn yxyz(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.x, self.y, self.z)
    }

    #[inline]
    fn yxzx(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.x, self.z, self.x)
    }

    #[inline]
    fn yxzy(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.x, self.z, self.y)
    }

    #[inline]
    fn yxzz(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.x, self.z, self.z)
    }

    #[inline]
    fn yyxx(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.y, self.x, self.x)
    }

    #[inline]
    fn yyxy(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.y, self.x, self.y)
    }

    #[inline]
    fn yyxz(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.y, self.x, self.z)
    }

    #[inline]
    fn yyyx(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.y, self.y, self.x)
    }

    #[inline]
    fn yyyy(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.y, self.y, self.y)
    }

    #[inline]
    fn yyyz(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.y, self.y, self.z)
    }

    #[inline]
    fn yyzx(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.y, self.z, self.x)
    }

    #[inline]
    fn yyzy(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.y, self.z, self.y)
    }

    #[inline]
    fn yyzz(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.y, self.z, self.z)
    }

    #[inline]
    fn yzxx(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.z, self.x, self.x)
    }

    #[inline]
    fn yzxy(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.z, self.x, self.y)
    }

    #[inline]
    fn yzxz(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.z, self.x, self.z)
    }

    #[inline]
    fn yzyx(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.z, self.y, self.x)
    }

    #[inline]
    fn yzyy(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.z, self.y, self.y)
    }

    #[inline]
    fn yzyz(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.z, self.y, self.z)
    }

    #[inline]
    fn yzzx(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.z, self.z, self.x)
    }

    #[inline]
    fn yzzy(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.z, self.z, self.y)
    }

    #[inline]
    fn yzzz(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.z, self.z, self.z)
    }

    #[inline]
    fn zxxx(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.x, self.x, self.x)
    }

    #[inline]
    fn zxxy(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.x, self.x, self.y)
    }

    #[inline]
    fn zxxz(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.x, self.x, self.z)
    }

    #[inline]
    fn zxyx(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.x, self.y, self.x)
    }

    #[inline]
    fn zxyy(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.x, self.y, self.y)
    }

    #[inline]
    fn zxyz(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.x, self.y, self.z)
    }

    #[inline]
    fn zxzx(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.x, self.z, self.x)
    }

    #[inline]
    fn zxzy(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.x, self.z, self.y)
    }

    #[inline]
    fn zxzz(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.x, self.z, self.z)
    }

    #[inline]
    fn zyxx(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.y, self.x, self.x)
    }

    #[inline]
    fn zyxy(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.y, self.x, self.y)
    }

    #[inline]
    fn zyxz(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.y, self.x, self.z)
    }

    #[inline]
    fn zyyx(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.y, self.y, self.x)
    }

    #[inline]
    fn zyyy(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.y, self.y, self.y)
    }

    #[inline]
    fn zyyz(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.y, self.y, self.z)
    }

    #[inline]
    fn zyzx(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.y, self.z, self.x)
    }

    #[inline]
    fn zyzy(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.y, self.z, self.y)
    }

    #[inline]
    fn zyzz(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.y, self.z, self.z)
    }

    #[inline]
    fn zzxx(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.z, self.x, self.x)
    }

    #[inline]
    fn zzxy(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.z, self.x, self.y)
    }

    #[inline]
    fn zzxz(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.z, self.x, self.z)
    }

    #[inline]
    fn zzyx(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.z, self.y, self.x)
    }

    #[inline]
    fn zzyy(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.z, self.y, self.y)
    }

    #[inline]
    fn zzyz(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.z, self.y, self.z)
    }

    #[inline]
    fn zzzx(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.z, self.z, self.x)
    }

    #[inline]
    fn zzzy(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.z, self.z, self.y)
    }

    #[inline]
    fn zzzz(self) -> USizeVec4 {
        USizeVec4::new(self.z, self.z, self.z, self.z)
    }
}
