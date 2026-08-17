// Generated from swizzle_impl.rs.tera template. Edit the template, not the generated file.

use crate::{IVec2, IVec3, IVec4, Vec2Swizzles};

impl Vec2Swizzles for IVec2 {
    type Vec3 = IVec3;

    type Vec4 = IVec4;

    #[inline]
    #[must_use]
    fn xx(self) -> IVec2 {
        IVec2 {
            x: self.x,
            y: self.x,
        }
    }

    #[inline]
    #[must_use]
    fn xy(self) -> IVec2 {
        IVec2 {
            x: self.x,
            y: self.y,
        }
    }

    #[inline]
    #[must_use]
    fn yx(self) -> IVec2 {
        IVec2 {
            x: self.y,
            y: self.x,
        }
    }

    #[inline]
    #[must_use]
    fn yy(self) -> IVec2 {
        IVec2 {
            x: self.y,
            y: self.y,
        }
    }

    #[inline]
    #[must_use]
    fn xxx(self) -> IVec3 {
        IVec3::new(self.x, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn xxy(self) -> IVec3 {
        IVec3::new(self.x, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn xyx(self) -> IVec3 {
        IVec3::new(self.x, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn xyy(self) -> IVec3 {
        IVec3::new(self.x, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn yxx(self) -> IVec3 {
        IVec3::new(self.y, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn yxy(self) -> IVec3 {
        IVec3::new(self.y, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn yyx(self) -> IVec3 {
        IVec3::new(self.y, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn yyy(self) -> IVec3 {
        IVec3::new(self.y, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn xxxx(self) -> IVec4 {
        IVec4::new(self.x, self.x, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn xxxy(self) -> IVec4 {
        IVec4::new(self.x, self.x, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn xxyx(self) -> IVec4 {
        IVec4::new(self.x, self.x, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn xxyy(self) -> IVec4 {
        IVec4::new(self.x, self.x, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn xyxx(self) -> IVec4 {
        IVec4::new(self.x, self.y, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn xyxy(self) -> IVec4 {
        IVec4::new(self.x, self.y, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn xyyx(self) -> IVec4 {
        IVec4::new(self.x, self.y, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn xyyy(self) -> IVec4 {
        IVec4::new(self.x, self.y, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn yxxx(self) -> IVec4 {
        IVec4::new(self.y, self.x, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn yxxy(self) -> IVec4 {
        IVec4::new(self.y, self.x, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn yxyx(self) -> IVec4 {
        IVec4::new(self.y, self.x, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn yxyy(self) -> IVec4 {
        IVec4::new(self.y, self.x, self.y, self.y)
    }

    #[inline]
    #[must_use]
    fn yyxx(self) -> IVec4 {
        IVec4::new(self.y, self.y, self.x, self.x)
    }

    #[inline]
    #[must_use]
    fn yyxy(self) -> IVec4 {
        IVec4::new(self.y, self.y, self.x, self.y)
    }

    #[inline]
    #[must_use]
    fn yyyx(self) -> IVec4 {
        IVec4::new(self.y, self.y, self.y, self.x)
    }

    #[inline]
    #[must_use]
    fn yyyy(self) -> IVec4 {
        IVec4::new(self.y, self.y, self.y, self.y)
    }
}
