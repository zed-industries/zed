// Generated from swizzle_impl.rs.tera template. Edit the template, not the generated file.

use crate::{U64Vec2, U64Vec3, U64Vec4, Vec2Swizzles};

impl Vec2Swizzles for U64Vec2 {
    type Vec3 = U64Vec3;

    type Vec4 = U64Vec4;

    #[inline]
    #[must_use]
    fn xx(self) -> U64Vec2 {
        U64Vec2 {
            x: self.x,
            y: self.x,
        }
    }

    #[inline]
    #[must_use]
    fn xy(self) -> U64Vec2 {
        U64Vec2 {
            x: self.x,
            y: self.y,
        }
    }

    #[inline]
    #[must_use]
    fn yx(self) -> U64Vec2 {
        U64Vec2 {
            x: self.y,
            y: self.x,
        }
    }

    #[inline]
    #[must_use]
    fn yy(self) -> U64Vec2 {
        U64Vec2 {
            x: self.y,
            y: self.y,
        }
    }

    #[inline]
    #[must_use]
    fn xxx(self) -> U64Vec3 {
        U64Vec3::new(self.x, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn xxy(self) -> U64Vec3 {
        U64Vec3::new(self.x, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn xyx(self) -> U64Vec3 {
        U64Vec3::new(self.x, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn xyy(self) -> U64Vec3 {
        U64Vec3::new(self.x, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn yxx(self) -> U64Vec3 {
        U64Vec3::new(self.y, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn yxy(self) -> U64Vec3 {
        U64Vec3::new(self.y, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn yyx(self) -> U64Vec3 {
        U64Vec3::new(self.y, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn yyy(self) -> U64Vec3 {
        U64Vec3::new(self.y, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn xxxx(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.x, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn xxxy(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.x, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn xxyx(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.x, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn xxyy(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.x, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn xyxx(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.y, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn xyxy(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.y, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn xyyx(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.y, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn xyyy(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.y, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn yxxx(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.x, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn yxxy(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.x, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn yxyx(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.x, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn yxyy(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.x, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn yyxx(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.y, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn yyxy(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.y, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn yyyx(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.y, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn yyyy(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.y, self.y, self.y)
    }
}
