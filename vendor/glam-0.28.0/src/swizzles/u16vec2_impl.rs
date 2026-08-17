// Generated from swizzle_impl.rs.tera template. Edit the template, not the generated file.

use crate::{U16Vec2, U16Vec3, U16Vec4, Vec2Swizzles};

impl Vec2Swizzles for U16Vec2 {
    type Vec3 = U16Vec3;

    type Vec4 = U16Vec4;

    #[inline]
    #[must_use]
    fn xx(self) -> U16Vec2 {
        U16Vec2 {
            x: self.x,
            y: self.x,
        }
    }

    #[inline]
    #[must_use]
    fn xy(self) -> U16Vec2 {
        U16Vec2 {
            x: self.x,
            y: self.y,
        }
    }

    #[inline]
    #[must_use]
    fn yx(self) -> U16Vec2 {
        U16Vec2 {
            x: self.y,
            y: self.x,
        }
    }

    #[inline]
    #[must_use]
    fn yy(self) -> U16Vec2 {
        U16Vec2 {
            x: self.y,
            y: self.y,
        }
    }

    #[inline]
    #[must_use]
    fn xxx(self) -> U16Vec3 {
        U16Vec3::new(self.x, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn xxy(self) -> U16Vec3 {
        U16Vec3::new(self.x, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn xyx(self) -> U16Vec3 {
        U16Vec3::new(self.x, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn xyy(self) -> U16Vec3 {
        U16Vec3::new(self.x, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn yxx(self) -> U16Vec3 {
        U16Vec3::new(self.y, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn yxy(self) -> U16Vec3 {
        U16Vec3::new(self.y, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn yyx(self) -> U16Vec3 {
        U16Vec3::new(self.y, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn yyy(self) -> U16Vec3 {
        U16Vec3::new(self.y, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn xxxx(self) -> U16Vec4 {
        U16Vec4::new(self.x, self.x, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn xxxy(self) -> U16Vec4 {
        U16Vec4::new(self.x, self.x, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn xxyx(self) -> U16Vec4 {
        U16Vec4::new(self.x, self.x, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn xxyy(self) -> U16Vec4 {
        U16Vec4::new(self.x, self.x, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn xyxx(self) -> U16Vec4 {
        U16Vec4::new(self.x, self.y, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn xyxy(self) -> U16Vec4 {
        U16Vec4::new(self.x, self.y, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn xyyx(self) -> U16Vec4 {
        U16Vec4::new(self.x, self.y, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn xyyy(self) -> U16Vec4 {
        U16Vec4::new(self.x, self.y, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn yxxx(self) -> U16Vec4 {
        U16Vec4::new(self.y, self.x, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn yxxy(self) -> U16Vec4 {
        U16Vec4::new(self.y, self.x, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn yxyx(self) -> U16Vec4 {
        U16Vec4::new(self.y, self.x, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn yxyy(self) -> U16Vec4 {
        U16Vec4::new(self.y, self.x, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn yyxx(self) -> U16Vec4 {
        U16Vec4::new(self.y, self.y, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn yyxy(self) -> U16Vec4 {
        U16Vec4::new(self.y, self.y, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn yyyx(self) -> U16Vec4 {
        U16Vec4::new(self.y, self.y, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn yyyy(self) -> U16Vec4 {
        U16Vec4::new(self.y, self.y, self.y, self.y)
    }
}
