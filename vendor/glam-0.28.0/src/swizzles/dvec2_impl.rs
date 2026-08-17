// Generated from swizzle_impl.rs.tera template. Edit the template, not the generated file.

use crate::{DVec2, DVec3, DVec4, Vec2Swizzles};

impl Vec2Swizzles for DVec2 {
    type Vec3 = DVec3;

    type Vec4 = DVec4;

    #[inline]
    #[must_use]
    fn xx(self) -> DVec2 {
        DVec2 {
            x: self.x,
            y: self.x,
        }
    }

    #[inline]
    #[must_use]
    fn xy(self) -> DVec2 {
        DVec2 {
            x: self.x,
            y: self.y,
        }
    }

    #[inline]
    #[must_use]
    fn yx(self) -> DVec2 {
        DVec2 {
            x: self.y,
            y: self.x,
        }
    }

    #[inline]
    #[must_use]
    fn yy(self) -> DVec2 {
        DVec2 {
            x: self.y,
            y: self.y,
        }
    }

    #[inline]
    #[must_use]
    fn xxx(self) -> DVec3 {
        DVec3::new(self.x, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn xxy(self) -> DVec3 {
        DVec3::new(self.x, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn xyx(self) -> DVec3 {
        DVec3::new(self.x, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn xyy(self) -> DVec3 {
        DVec3::new(self.x, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn yxx(self) -> DVec3 {
        DVec3::new(self.y, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn yxy(self) -> DVec3 {
        DVec3::new(self.y, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn yyx(self) -> DVec3 {
        DVec3::new(self.y, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn yyy(self) -> DVec3 {
        DVec3::new(self.y, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn xxxx(self) -> DVec4 {
        DVec4::new(self.x, self.x, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn xxxy(self) -> DVec4 {
        DVec4::new(self.x, self.x, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn xxyx(self) -> DVec4 {
        DVec4::new(self.x, self.x, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn xxyy(self) -> DVec4 {
        DVec4::new(self.x, self.x, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn xyxx(self) -> DVec4 {
        DVec4::new(self.x, self.y, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn xyxy(self) -> DVec4 {
        DVec4::new(self.x, self.y, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn xyyx(self) -> DVec4 {
        DVec4::new(self.x, self.y, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn xyyy(self) -> DVec4 {
        DVec4::new(self.x, self.y, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn yxxx(self) -> DVec4 {
        DVec4::new(self.y, self.x, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn yxxy(self) -> DVec4 {
        DVec4::new(self.y, self.x, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn yxyx(self) -> DVec4 {
        DVec4::new(self.y, self.x, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn yxyy(self) -> DVec4 {
        DVec4::new(self.y, self.x, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn yyxx(self) -> DVec4 {
        DVec4::new(self.y, self.y, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn yyxy(self) -> DVec4 {
        DVec4::new(self.y, self.y, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn yyyx(self) -> DVec4 {
        DVec4::new(self.y, self.y, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn yyyy(self) -> DVec4 {
        DVec4::new(self.y, self.y, self.y, self.y)
    }
}
