// Generated from swizzle_impl.rs.tera template. Edit the template, not the generated file.

use crate::{IVec2, IVec3, IVec4, Vec3Swizzles};

impl Vec3Swizzles for IVec3 {
    type Vec2 = IVec2;

    type Vec4 = IVec4;

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
        Self::new(rhs.x, rhs.y, self.z)
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
        Self::new(rhs.x, self.y, rhs.y)
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
        Self::new(rhs.y, rhs.x, self.z)
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
        Self::new(self.x, rhs.x, rhs.y)
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
        Self::new(rhs.y, self.y, rhs.x)
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
        Self::new(self.x, rhs.y, rhs.x)
    }

    #[inline]
    fn zz(self) -> IVec2 {
        IVec2 {
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
    fn xxxx(self) -> IVec4 {
        IVec4::new(self.x, self.x, self.x, self.x)
    }

    #[inline]
    fn xxxy(self) -> IVec4 {
        IVec4::new(self.x, self.x, self.x, self.y)
    }

    #[inline]
    fn xxxz(self) -> IVec4 {
        IVec4::new(self.x, self.x, self.x, self.z)
    }

    #[inline]
    fn xxyx(self) -> IVec4 {
        IVec4::new(self.x, self.x, self.y, self.x)
    }

    #[inline]
    fn xxyy(self) -> IVec4 {
        IVec4::new(self.x, self.x, self.y, self.y)
    }

    #[inline]
    fn xxyz(self) -> IVec4 {
        IVec4::new(self.x, self.x, self.y, self.z)
    }

    #[inline]
    fn xxzx(self) -> IVec4 {
        IVec4::new(self.x, self.x, self.z, self.x)
    }

    #[inline]
    fn xxzy(self) -> IVec4 {
        IVec4::new(self.x, self.x, self.z, self.y)
    }

    #[inline]
    fn xxzz(self) -> IVec4 {
        IVec4::new(self.x, self.x, self.z, self.z)
    }

    #[inline]
    fn xyxx(self) -> IVec4 {
        IVec4::new(self.x, self.y, self.x, self.x)
    }

    #[inline]
    fn xyxy(self) -> IVec4 {
        IVec4::new(self.x, self.y, self.x, self.y)
    }

    #[inline]
    fn xyxz(self) -> IVec4 {
        IVec4::new(self.x, self.y, self.x, self.z)
    }

    #[inline]
    fn xyyx(self) -> IVec4 {
        IVec4::new(self.x, self.y, self.y, self.x)
    }

    #[inline]
    fn xyyy(self) -> IVec4 {
        IVec4::new(self.x, self.y, self.y, self.y)
    }

    #[inline]
    fn xyyz(self) -> IVec4 {
        IVec4::new(self.x, self.y, self.y, self.z)
    }

    #[inline]
    fn xyzx(self) -> IVec4 {
        IVec4::new(self.x, self.y, self.z, self.x)
    }

    #[inline]
    fn xyzy(self) -> IVec4 {
        IVec4::new(self.x, self.y, self.z, self.y)
    }

    #[inline]
    fn xyzz(self) -> IVec4 {
        IVec4::new(self.x, self.y, self.z, self.z)
    }

    #[inline]
    fn xzxx(self) -> IVec4 {
        IVec4::new(self.x, self.z, self.x, self.x)
    }

    #[inline]
    fn xzxy(self) -> IVec4 {
        IVec4::new(self.x, self.z, self.x, self.y)
    }

    #[inline]
    fn xzxz(self) -> IVec4 {
        IVec4::new(self.x, self.z, self.x, self.z)
    }

    #[inline]
    fn xzyx(self) -> IVec4 {
        IVec4::new(self.x, self.z, self.y, self.x)
    }

    #[inline]
    fn xzyy(self) -> IVec4 {
        IVec4::new(self.x, self.z, self.y, self.y)
    }

    #[inline]
    fn xzyz(self) -> IVec4 {
        IVec4::new(self.x, self.z, self.y, self.z)
    }

    #[inline]
    fn xzzx(self) -> IVec4 {
        IVec4::new(self.x, self.z, self.z, self.x)
    }

    #[inline]
    fn xzzy(self) -> IVec4 {
        IVec4::new(self.x, self.z, self.z, self.y)
    }

    #[inline]
    fn xzzz(self) -> IVec4 {
        IVec4::new(self.x, self.z, self.z, self.z)
    }

    #[inline]
    fn yxxx(self) -> IVec4 {
        IVec4::new(self.y, self.x, self.x, self.x)
    }

    #[inline]
    fn yxxy(self) -> IVec4 {
        IVec4::new(self.y, self.x, self.x, self.y)
    }

    #[inline]
    fn yxxz(self) -> IVec4 {
        IVec4::new(self.y, self.x, self.x, self.z)
    }

    #[inline]
    fn yxyx(self) -> IVec4 {
        IVec4::new(self.y, self.x, self.y, self.x)
    }

    #[inline]
    fn yxyy(self) -> IVec4 {
        IVec4::new(self.y, self.x, self.y, self.y)
    }

    #[inline]
    fn yxyz(self) -> IVec4 {
        IVec4::new(self.y, self.x, self.y, self.z)
    }

    #[inline]
    fn yxzx(self) -> IVec4 {
        IVec4::new(self.y, self.x, self.z, self.x)
    }

    #[inline]
    fn yxzy(self) -> IVec4 {
        IVec4::new(self.y, self.x, self.z, self.y)
    }

    #[inline]
    fn yxzz(self) -> IVec4 {
        IVec4::new(self.y, self.x, self.z, self.z)
    }

    #[inline]
    fn yyxx(self) -> IVec4 {
        IVec4::new(self.y, self.y, self.x, self.x)
    }

    #[inline]
    fn yyxy(self) -> IVec4 {
        IVec4::new(self.y, self.y, self.x, self.y)
    }

    #[inline]
    fn yyxz(self) -> IVec4 {
        IVec4::new(self.y, self.y, self.x, self.z)
    }

    #[inline]
    fn yyyx(self) -> IVec4 {
        IVec4::new(self.y, self.y, self.y, self.x)
    }

    #[inline]
    fn yyyy(self) -> IVec4 {
        IVec4::new(self.y, self.y, self.y, self.y)
    }

    #[inline]
    fn yyyz(self) -> IVec4 {
        IVec4::new(self.y, self.y, self.y, self.z)
    }

    #[inline]
    fn yyzx(self) -> IVec4 {
        IVec4::new(self.y, self.y, self.z, self.x)
    }

    #[inline]
    fn yyzy(self) -> IVec4 {
        IVec4::new(self.y, self.y, self.z, self.y)
    }

    #[inline]
    fn yyzz(self) -> IVec4 {
        IVec4::new(self.y, self.y, self.z, self.z)
    }

    #[inline]
    fn yzxx(self) -> IVec4 {
        IVec4::new(self.y, self.z, self.x, self.x)
    }

    #[inline]
    fn yzxy(self) -> IVec4 {
        IVec4::new(self.y, self.z, self.x, self.y)
    }

    #[inline]
    fn yzxz(self) -> IVec4 {
        IVec4::new(self.y, self.z, self.x, self.z)
    }

    #[inline]
    fn yzyx(self) -> IVec4 {
        IVec4::new(self.y, self.z, self.y, self.x)
    }

    #[inline]
    fn yzyy(self) -> IVec4 {
        IVec4::new(self.y, self.z, self.y, self.y)
    }

    #[inline]
    fn yzyz(self) -> IVec4 {
        IVec4::new(self.y, self.z, self.y, self.z)
    }

    #[inline]
    fn yzzx(self) -> IVec4 {
        IVec4::new(self.y, self.z, self.z, self.x)
    }

    #[inline]
    fn yzzy(self) -> IVec4 {
        IVec4::new(self.y, self.z, self.z, self.y)
    }

    #[inline]
    fn yzzz(self) -> IVec4 {
        IVec4::new(self.y, self.z, self.z, self.z)
    }

    #[inline]
    fn zxxx(self) -> IVec4 {
        IVec4::new(self.z, self.x, self.x, self.x)
    }

    #[inline]
    fn zxxy(self) -> IVec4 {
        IVec4::new(self.z, self.x, self.x, self.y)
    }

    #[inline]
    fn zxxz(self) -> IVec4 {
        IVec4::new(self.z, self.x, self.x, self.z)
    }

    #[inline]
    fn zxyx(self) -> IVec4 {
        IVec4::new(self.z, self.x, self.y, self.x)
    }

    #[inline]
    fn zxyy(self) -> IVec4 {
        IVec4::new(self.z, self.x, self.y, self.y)
    }

    #[inline]
    fn zxyz(self) -> IVec4 {
        IVec4::new(self.z, self.x, self.y, self.z)
    }

    #[inline]
    fn zxzx(self) -> IVec4 {
        IVec4::new(self.z, self.x, self.z, self.x)
    }

    #[inline]
    fn zxzy(self) -> IVec4 {
        IVec4::new(self.z, self.x, self.z, self.y)
    }

    #[inline]
    fn zxzz(self) -> IVec4 {
        IVec4::new(self.z, self.x, self.z, self.z)
    }

    #[inline]
    fn zyxx(self) -> IVec4 {
        IVec4::new(self.z, self.y, self.x, self.x)
    }

    #[inline]
    fn zyxy(self) -> IVec4 {
        IVec4::new(self.z, self.y, self.x, self.y)
    }

    #[inline]
    fn zyxz(self) -> IVec4 {
        IVec4::new(self.z, self.y, self.x, self.z)
    }

    #[inline]
    fn zyyx(self) -> IVec4 {
        IVec4::new(self.z, self.y, self.y, self.x)
    }

    #[inline]
    fn zyyy(self) -> IVec4 {
        IVec4::new(self.z, self.y, self.y, self.y)
    }

    #[inline]
    fn zyyz(self) -> IVec4 {
        IVec4::new(self.z, self.y, self.y, self.z)
    }

    #[inline]
    fn zyzx(self) -> IVec4 {
        IVec4::new(self.z, self.y, self.z, self.x)
    }

    #[inline]
    fn zyzy(self) -> IVec4 {
        IVec4::new(self.z, self.y, self.z, self.y)
    }

    #[inline]
    fn zyzz(self) -> IVec4 {
        IVec4::new(self.z, self.y, self.z, self.z)
    }

    #[inline]
    fn zzxx(self) -> IVec4 {
        IVec4::new(self.z, self.z, self.x, self.x)
    }

    #[inline]
    fn zzxy(self) -> IVec4 {
        IVec4::new(self.z, self.z, self.x, self.y)
    }

    #[inline]
    fn zzxz(self) -> IVec4 {
        IVec4::new(self.z, self.z, self.x, self.z)
    }

    #[inline]
    fn zzyx(self) -> IVec4 {
        IVec4::new(self.z, self.z, self.y, self.x)
    }

    #[inline]
    fn zzyy(self) -> IVec4 {
        IVec4::new(self.z, self.z, self.y, self.y)
    }

    #[inline]
    fn zzyz(self) -> IVec4 {
        IVec4::new(self.z, self.z, self.y, self.z)
    }

    #[inline]
    fn zzzx(self) -> IVec4 {
        IVec4::new(self.z, self.z, self.z, self.x)
    }

    #[inline]
    fn zzzy(self) -> IVec4 {
        IVec4::new(self.z, self.z, self.z, self.y)
    }

    #[inline]
    fn zzzz(self) -> IVec4 {
        IVec4::new(self.z, self.z, self.z, self.z)
    }
}
