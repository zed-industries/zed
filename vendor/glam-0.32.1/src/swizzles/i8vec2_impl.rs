// Generated from swizzle_impl.rs.tera template. Edit the template, not the generated file.

use crate::{I8Vec2, I8Vec3, I8Vec4, Vec2Swizzles};

impl Vec2Swizzles for I8Vec2 {
    type Vec3 = I8Vec3;

    type Vec4 = I8Vec4;

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
    fn xxx(self) -> I8Vec3 {
        I8Vec3::new(self.x, self.x, self.x)
    }

    #[inline]
    fn xxy(self) -> I8Vec3 {
        I8Vec3::new(self.x, self.x, self.y)
    }

    #[inline]
    fn xyx(self) -> I8Vec3 {
        I8Vec3::new(self.x, self.y, self.x)
    }

    #[inline]
    fn xyy(self) -> I8Vec3 {
        I8Vec3::new(self.x, self.y, self.y)
    }

    #[inline]
    fn yxx(self) -> I8Vec3 {
        I8Vec3::new(self.y, self.x, self.x)
    }

    #[inline]
    fn yxy(self) -> I8Vec3 {
        I8Vec3::new(self.y, self.x, self.y)
    }

    #[inline]
    fn yyx(self) -> I8Vec3 {
        I8Vec3::new(self.y, self.y, self.x)
    }

    #[inline]
    fn yyy(self) -> I8Vec3 {
        I8Vec3::new(self.y, self.y, self.y)
    }

    #[inline]
    fn xxxx(self) -> I8Vec4 {
        I8Vec4::new(self.x, self.x, self.x, self.x)
    }

    #[inline]
    fn xxxy(self) -> I8Vec4 {
        I8Vec4::new(self.x, self.x, self.x, self.y)
    }

    #[inline]
    fn xxyx(self) -> I8Vec4 {
        I8Vec4::new(self.x, self.x, self.y, self.x)
    }

    #[inline]
    fn xxyy(self) -> I8Vec4 {
        I8Vec4::new(self.x, self.x, self.y, self.y)
    }

    #[inline]
    fn xyxx(self) -> I8Vec4 {
        I8Vec4::new(self.x, self.y, self.x, self.x)
    }

    #[inline]
    fn xyxy(self) -> I8Vec4 {
        I8Vec4::new(self.x, self.y, self.x, self.y)
    }

    #[inline]
    fn xyyx(self) -> I8Vec4 {
        I8Vec4::new(self.x, self.y, self.y, self.x)
    }

    #[inline]
    fn xyyy(self) -> I8Vec4 {
        I8Vec4::new(self.x, self.y, self.y, self.y)
    }

    #[inline]
    fn yxxx(self) -> I8Vec4 {
        I8Vec4::new(self.y, self.x, self.x, self.x)
    }

    #[inline]
    fn yxxy(self) -> I8Vec4 {
        I8Vec4::new(self.y, self.x, self.x, self.y)
    }

    #[inline]
    fn yxyx(self) -> I8Vec4 {
        I8Vec4::new(self.y, self.x, self.y, self.x)
    }

    #[inline]
    fn yxyy(self) -> I8Vec4 {
        I8Vec4::new(self.y, self.x, self.y, self.y)
    }

    #[inline]
    fn yyxx(self) -> I8Vec4 {
        I8Vec4::new(self.y, self.y, self.x, self.x)
    }

    #[inline]
    fn yyxy(self) -> I8Vec4 {
        I8Vec4::new(self.y, self.y, self.x, self.y)
    }

    #[inline]
    fn yyyx(self) -> I8Vec4 {
        I8Vec4::new(self.y, self.y, self.y, self.x)
    }

    #[inline]
    fn yyyy(self) -> I8Vec4 {
        I8Vec4::new(self.y, self.y, self.y, self.y)
    }
}
