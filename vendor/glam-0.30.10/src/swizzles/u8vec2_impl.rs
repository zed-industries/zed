// Generated from swizzle_impl.rs.tera template. Edit the template, not the generated file.

use crate::{U8Vec2, U8Vec3, U8Vec4, Vec2Swizzles};

impl Vec2Swizzles for U8Vec2 {
    type Vec3 = U8Vec3;

    type Vec4 = U8Vec4;

    #[inline]
    fn xx(self) -> Self {
        Self {
            x: self.x,
            y: self.x,
        }
    }

    #[inline]
    fn yx(self) -> Self {
        Self {
            x: self.y,
            y: self.x,
        }
    }

    #[inline]
    fn yy(self) -> Self {
        Self {
            x: self.y,
            y: self.y,
        }
    }

    #[inline]
    fn xxx(self) -> U8Vec3 {
        U8Vec3::new(self.x, self.x, self.x)
    }

    #[inline]
    fn xxy(self) -> U8Vec3 {
        U8Vec3::new(self.x, self.x, self.y)
    }

    #[inline]
    fn xyx(self) -> U8Vec3 {
        U8Vec3::new(self.x, self.y, self.x)
    }

    #[inline]
    fn xyy(self) -> U8Vec3 {
        U8Vec3::new(self.x, self.y, self.y)
    }

    #[inline]
    fn yxx(self) -> U8Vec3 {
        U8Vec3::new(self.y, self.x, self.x)
    }

    #[inline]
    fn yxy(self) -> U8Vec3 {
        U8Vec3::new(self.y, self.x, self.y)
    }

    #[inline]
    fn yyx(self) -> U8Vec3 {
        U8Vec3::new(self.y, self.y, self.x)
    }

    #[inline]
    fn yyy(self) -> U8Vec3 {
        U8Vec3::new(self.y, self.y, self.y)
    }

    #[inline]
    fn xxxx(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.x, self.x, self.x)
    }

    #[inline]
    fn xxxy(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.x, self.x, self.y)
    }

    #[inline]
    fn xxyx(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.x, self.y, self.x)
    }

    #[inline]
    fn xxyy(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.x, self.y, self.y)
    }

    #[inline]
    fn xyxx(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.y, self.x, self.x)
    }

    #[inline]
    fn xyxy(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.y, self.x, self.y)
    }

    #[inline]
    fn xyyx(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.y, self.y, self.x)
    }

    #[inline]
    fn xyyy(self) -> U8Vec4 {
        U8Vec4::new(self.x, self.y, self.y, self.y)
    }

    #[inline]
    fn yxxx(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.x, self.x, self.x)
    }

    #[inline]
    fn yxxy(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.x, self.x, self.y)
    }

    #[inline]
    fn yxyx(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.x, self.y, self.x)
    }

    #[inline]
    fn yxyy(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.x, self.y, self.y)
    }

    #[inline]
    fn yyxx(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.y, self.x, self.x)
    }

    #[inline]
    fn yyxy(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.y, self.x, self.y)
    }

    #[inline]
    fn yyyx(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.y, self.y, self.x)
    }

    #[inline]
    fn yyyy(self) -> U8Vec4 {
        U8Vec4::new(self.y, self.y, self.y, self.y)
    }
}
