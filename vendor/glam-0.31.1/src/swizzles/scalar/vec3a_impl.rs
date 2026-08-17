// Generated from swizzle_impl.rs.tera template. Edit the template, not the generated file.

use crate::{Vec2, Vec3A, Vec3Swizzles, Vec4};

impl Vec3Swizzles for Vec3A {
    type Vec2 = Vec2;

    type Vec4 = Vec4;

    #[inline]
    fn xx(self) -> Vec2 {
        Vec2 {
            x: self.x,
            y: self.x,
        }
    }

    #[inline]
    fn xy(self) -> Vec2 {
        Vec2 {
            x: self.x,
            y: self.y,
        }
    }

    #[inline]
    fn with_xy(self, rhs: Vec2) -> Self {
        Self::new(rhs.x, rhs.y, self.z)
    }

    #[inline]
    fn xz(self) -> Vec2 {
        Vec2 {
            x: self.x,
            y: self.z,
        }
    }

    #[inline]
    fn with_xz(self, rhs: Vec2) -> Self {
        Self::new(rhs.x, self.y, rhs.y)
    }

    #[inline]
    fn yx(self) -> Vec2 {
        Vec2 {
            x: self.y,
            y: self.x,
        }
    }

    #[inline]
    fn with_yx(self, rhs: Vec2) -> Self {
        Self::new(rhs.y, rhs.x, self.z)
    }

    #[inline]
    fn yy(self) -> Vec2 {
        Vec2 {
            x: self.y,
            y: self.y,
        }
    }

    #[inline]
    fn yz(self) -> Vec2 {
        Vec2 {
            x: self.y,
            y: self.z,
        }
    }

    #[inline]
    fn with_yz(self, rhs: Vec2) -> Self {
        Self::new(self.x, rhs.x, rhs.y)
    }

    #[inline]
    fn zx(self) -> Vec2 {
        Vec2 {
            x: self.z,
            y: self.x,
        }
    }

    #[inline]
    fn with_zx(self, rhs: Vec2) -> Self {
        Self::new(rhs.y, self.y, rhs.x)
    }

    #[inline]
    fn zy(self) -> Vec2 {
        Vec2 {
            x: self.z,
            y: self.y,
        }
    }

    #[inline]
    fn with_zy(self, rhs: Vec2) -> Self {
        Self::new(self.x, rhs.y, rhs.x)
    }

    #[inline]
    fn zz(self) -> Vec2 {
        Vec2 {
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
    fn xxxx(self) -> Vec4 {
        Vec4::new(self.x, self.x, self.x, self.x)
    }

    #[inline]
    fn xxxy(self) -> Vec4 {
        Vec4::new(self.x, self.x, self.x, self.y)
    }

    #[inline]
    fn xxxz(self) -> Vec4 {
        Vec4::new(self.x, self.x, self.x, self.z)
    }

    #[inline]
    fn xxyx(self) -> Vec4 {
        Vec4::new(self.x, self.x, self.y, self.x)
    }

    #[inline]
    fn xxyy(self) -> Vec4 {
        Vec4::new(self.x, self.x, self.y, self.y)
    }

    #[inline]
    fn xxyz(self) -> Vec4 {
        Vec4::new(self.x, self.x, self.y, self.z)
    }

    #[inline]
    fn xxzx(self) -> Vec4 {
        Vec4::new(self.x, self.x, self.z, self.x)
    }

    #[inline]
    fn xxzy(self) -> Vec4 {
        Vec4::new(self.x, self.x, self.z, self.y)
    }

    #[inline]
    fn xxzz(self) -> Vec4 {
        Vec4::new(self.x, self.x, self.z, self.z)
    }

    #[inline]
    fn xyxx(self) -> Vec4 {
        Vec4::new(self.x, self.y, self.x, self.x)
    }

    #[inline]
    fn xyxy(self) -> Vec4 {
        Vec4::new(self.x, self.y, self.x, self.y)
    }

    #[inline]
    fn xyxz(self) -> Vec4 {
        Vec4::new(self.x, self.y, self.x, self.z)
    }

    #[inline]
    fn xyyx(self) -> Vec4 {
        Vec4::new(self.x, self.y, self.y, self.x)
    }

    #[inline]
    fn xyyy(self) -> Vec4 {
        Vec4::new(self.x, self.y, self.y, self.y)
    }

    #[inline]
    fn xyyz(self) -> Vec4 {
        Vec4::new(self.x, self.y, self.y, self.z)
    }

    #[inline]
    fn xyzx(self) -> Vec4 {
        Vec4::new(self.x, self.y, self.z, self.x)
    }

    #[inline]
    fn xyzy(self) -> Vec4 {
        Vec4::new(self.x, self.y, self.z, self.y)
    }

    #[inline]
    fn xyzz(self) -> Vec4 {
        Vec4::new(self.x, self.y, self.z, self.z)
    }

    #[inline]
    fn xzxx(self) -> Vec4 {
        Vec4::new(self.x, self.z, self.x, self.x)
    }

    #[inline]
    fn xzxy(self) -> Vec4 {
        Vec4::new(self.x, self.z, self.x, self.y)
    }

    #[inline]
    fn xzxz(self) -> Vec4 {
        Vec4::new(self.x, self.z, self.x, self.z)
    }

    #[inline]
    fn xzyx(self) -> Vec4 {
        Vec4::new(self.x, self.z, self.y, self.x)
    }

    #[inline]
    fn xzyy(self) -> Vec4 {
        Vec4::new(self.x, self.z, self.y, self.y)
    }

    #[inline]
    fn xzyz(self) -> Vec4 {
        Vec4::new(self.x, self.z, self.y, self.z)
    }

    #[inline]
    fn xzzx(self) -> Vec4 {
        Vec4::new(self.x, self.z, self.z, self.x)
    }

    #[inline]
    fn xzzy(self) -> Vec4 {
        Vec4::new(self.x, self.z, self.z, self.y)
    }

    #[inline]
    fn xzzz(self) -> Vec4 {
        Vec4::new(self.x, self.z, self.z, self.z)
    }

    #[inline]
    fn yxxx(self) -> Vec4 {
        Vec4::new(self.y, self.x, self.x, self.x)
    }

    #[inline]
    fn yxxy(self) -> Vec4 {
        Vec4::new(self.y, self.x, self.x, self.y)
    }

    #[inline]
    fn yxxz(self) -> Vec4 {
        Vec4::new(self.y, self.x, self.x, self.z)
    }

    #[inline]
    fn yxyx(self) -> Vec4 {
        Vec4::new(self.y, self.x, self.y, self.x)
    }

    #[inline]
    fn yxyy(self) -> Vec4 {
        Vec4::new(self.y, self.x, self.y, self.y)
    }

    #[inline]
    fn yxyz(self) -> Vec4 {
        Vec4::new(self.y, self.x, self.y, self.z)
    }

    #[inline]
    fn yxzx(self) -> Vec4 {
        Vec4::new(self.y, self.x, self.z, self.x)
    }

    #[inline]
    fn yxzy(self) -> Vec4 {
        Vec4::new(self.y, self.x, self.z, self.y)
    }

    #[inline]
    fn yxzz(self) -> Vec4 {
        Vec4::new(self.y, self.x, self.z, self.z)
    }

    #[inline]
    fn yyxx(self) -> Vec4 {
        Vec4::new(self.y, self.y, self.x, self.x)
    }

    #[inline]
    fn yyxy(self) -> Vec4 {
        Vec4::new(self.y, self.y, self.x, self.y)
    }

    #[inline]
    fn yyxz(self) -> Vec4 {
        Vec4::new(self.y, self.y, self.x, self.z)
    }

    #[inline]
    fn yyyx(self) -> Vec4 {
        Vec4::new(self.y, self.y, self.y, self.x)
    }

    #[inline]
    fn yyyy(self) -> Vec4 {
        Vec4::new(self.y, self.y, self.y, self.y)
    }

    #[inline]
    fn yyyz(self) -> Vec4 {
        Vec4::new(self.y, self.y, self.y, self.z)
    }

    #[inline]
    fn yyzx(self) -> Vec4 {
        Vec4::new(self.y, self.y, self.z, self.x)
    }

    #[inline]
    fn yyzy(self) -> Vec4 {
        Vec4::new(self.y, self.y, self.z, self.y)
    }

    #[inline]
    fn yyzz(self) -> Vec4 {
        Vec4::new(self.y, self.y, self.z, self.z)
    }

    #[inline]
    fn yzxx(self) -> Vec4 {
        Vec4::new(self.y, self.z, self.x, self.x)
    }

    #[inline]
    fn yzxy(self) -> Vec4 {
        Vec4::new(self.y, self.z, self.x, self.y)
    }

    #[inline]
    fn yzxz(self) -> Vec4 {
        Vec4::new(self.y, self.z, self.x, self.z)
    }

    #[inline]
    fn yzyx(self) -> Vec4 {
        Vec4::new(self.y, self.z, self.y, self.x)
    }

    #[inline]
    fn yzyy(self) -> Vec4 {
        Vec4::new(self.y, self.z, self.y, self.y)
    }

    #[inline]
    fn yzyz(self) -> Vec4 {
        Vec4::new(self.y, self.z, self.y, self.z)
    }

    #[inline]
    fn yzzx(self) -> Vec4 {
        Vec4::new(self.y, self.z, self.z, self.x)
    }

    #[inline]
    fn yzzy(self) -> Vec4 {
        Vec4::new(self.y, self.z, self.z, self.y)
    }

    #[inline]
    fn yzzz(self) -> Vec4 {
        Vec4::new(self.y, self.z, self.z, self.z)
    }

    #[inline]
    fn zxxx(self) -> Vec4 {
        Vec4::new(self.z, self.x, self.x, self.x)
    }

    #[inline]
    fn zxxy(self) -> Vec4 {
        Vec4::new(self.z, self.x, self.x, self.y)
    }

    #[inline]
    fn zxxz(self) -> Vec4 {
        Vec4::new(self.z, self.x, self.x, self.z)
    }

    #[inline]
    fn zxyx(self) -> Vec4 {
        Vec4::new(self.z, self.x, self.y, self.x)
    }

    #[inline]
    fn zxyy(self) -> Vec4 {
        Vec4::new(self.z, self.x, self.y, self.y)
    }

    #[inline]
    fn zxyz(self) -> Vec4 {
        Vec4::new(self.z, self.x, self.y, self.z)
    }

    #[inline]
    fn zxzx(self) -> Vec4 {
        Vec4::new(self.z, self.x, self.z, self.x)
    }

    #[inline]
    fn zxzy(self) -> Vec4 {
        Vec4::new(self.z, self.x, self.z, self.y)
    }

    #[inline]
    fn zxzz(self) -> Vec4 {
        Vec4::new(self.z, self.x, self.z, self.z)
    }

    #[inline]
    fn zyxx(self) -> Vec4 {
        Vec4::new(self.z, self.y, self.x, self.x)
    }

    #[inline]
    fn zyxy(self) -> Vec4 {
        Vec4::new(self.z, self.y, self.x, self.y)
    }

    #[inline]
    fn zyxz(self) -> Vec4 {
        Vec4::new(self.z, self.y, self.x, self.z)
    }

    #[inline]
    fn zyyx(self) -> Vec4 {
        Vec4::new(self.z, self.y, self.y, self.x)
    }

    #[inline]
    fn zyyy(self) -> Vec4 {
        Vec4::new(self.z, self.y, self.y, self.y)
    }

    #[inline]
    fn zyyz(self) -> Vec4 {
        Vec4::new(self.z, self.y, self.y, self.z)
    }

    #[inline]
    fn zyzx(self) -> Vec4 {
        Vec4::new(self.z, self.y, self.z, self.x)
    }

    #[inline]
    fn zyzy(self) -> Vec4 {
        Vec4::new(self.z, self.y, self.z, self.y)
    }

    #[inline]
    fn zyzz(self) -> Vec4 {
        Vec4::new(self.z, self.y, self.z, self.z)
    }

    #[inline]
    fn zzxx(self) -> Vec4 {
        Vec4::new(self.z, self.z, self.x, self.x)
    }

    #[inline]
    fn zzxy(self) -> Vec4 {
        Vec4::new(self.z, self.z, self.x, self.y)
    }

    #[inline]
    fn zzxz(self) -> Vec4 {
        Vec4::new(self.z, self.z, self.x, self.z)
    }

    #[inline]
    fn zzyx(self) -> Vec4 {
        Vec4::new(self.z, self.z, self.y, self.x)
    }

    #[inline]
    fn zzyy(self) -> Vec4 {
        Vec4::new(self.z, self.z, self.y, self.y)
    }

    #[inline]
    fn zzyz(self) -> Vec4 {
        Vec4::new(self.z, self.z, self.y, self.z)
    }

    #[inline]
    fn zzzx(self) -> Vec4 {
        Vec4::new(self.z, self.z, self.z, self.x)
    }

    #[inline]
    fn zzzy(self) -> Vec4 {
        Vec4::new(self.z, self.z, self.z, self.y)
    }

    #[inline]
    fn zzzz(self) -> Vec4 {
        Vec4::new(self.z, self.z, self.z, self.z)
    }
}
