// Generated from swizzle_impl.rs.tera template. Edit the template, not the generated file.

use crate::{I16Vec2, I16Vec3, I16Vec4, Vec2Swizzles};

impl Vec2Swizzles for I16Vec2 {
    type Vec3 = I16Vec3;

    type Vec4 = I16Vec4;

    #[inline]
    #[must_use]
    fn xx(self) -> I16Vec2 {
        I16Vec2 {
            x: self.x,
            y: self.x,
        }
    }

    #[inline]
    #[must_use]
    fn xy(self) -> I16Vec2 {
        I16Vec2 {
            x: self.x,
            y: self.y,
        }
    }

    #[inline]
    #[must_use]
    fn yx(self) -> I16Vec2 {
        I16Vec2 {
            x: self.y,
            y: self.x,
        }
    }

    #[inline]
    #[must_use]
    fn yy(self) -> I16Vec2 {
        I16Vec2 {
            x: self.y,
            y: self.y,
        }
    }

    #[inline]
    #[must_use]
    fn xxx(self) -> I16Vec3 {
        I16Vec3::new(self.x, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn xxy(self) -> I16Vec3 {
        I16Vec3::new(self.x, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn xyx(self) -> I16Vec3 {
        I16Vec3::new(self.x, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn xyy(self) -> I16Vec3 {
        I16Vec3::new(self.x, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn yxx(self) -> I16Vec3 {
        I16Vec3::new(self.y, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn yxy(self) -> I16Vec3 {
        I16Vec3::new(self.y, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn yyx(self) -> I16Vec3 {
        I16Vec3::new(self.y, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn yyy(self) -> I16Vec3 {
        I16Vec3::new(self.y, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn xxxx(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.x, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn xxxy(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.x, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn xxyx(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.x, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn xxyy(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.x, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn xyxx(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.y, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn xyxy(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.y, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn xyyx(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.y, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn xyyy(self) -> I16Vec4 {
        I16Vec4::new(self.x, self.y, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn yxxx(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.x, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn yxxy(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.x, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn yxyx(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.x, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn yxyy(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.x, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn yyxx(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.y, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn yyxy(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.y, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn yyyx(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.y, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn yyyy(self) -> I16Vec4 {
        I16Vec4::new(self.y, self.y, self.y, self.y)
    }
}
