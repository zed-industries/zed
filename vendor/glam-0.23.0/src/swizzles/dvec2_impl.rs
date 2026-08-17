// Generated from swizzle_impl.rs.tera template. Edit the template, not the generated file.

use crate::{DVec2, DVec3, DVec4, Vec2Swizzles};

impl Vec2Swizzles for DVec2 {
    type Vec3 = DVec3;

    type Vec4 = DVec4;

    #[inline]
    fn xx(self) -> DVec2 {
        DVec2 {
            x: self.x,
            y: self.x,
        }
    }

    #[inline]
    fn xy(self) -> DVec2 {
        DVec2 {
            x: self.x,
            y: self.y,
        }
    }

    #[inline]
    fn yx(self) -> DVec2 {
        DVec2 {
            x: self.y,
            y: self.x,
        }
    }

    #[inline]
    fn yy(self) -> DVec2 {
        DVec2 {
            x: self.y,
            y: self.y,
        }
    }

    #[inline]
    fn xxx(self) -> DVec3 {
        DVec3 {
            x: self.x,
            y: self.x,
            z: self.x,
        }
    }

    #[inline]
    fn xxy(self) -> DVec3 {
        DVec3 {
            x: self.x,
            y: self.x,
            z: self.y,
        }
    }

    #[inline]
    fn xyx(self) -> DVec3 {
        DVec3 {
            x: self.x,
            y: self.y,
            z: self.x,
        }
    }

    #[inline]
    fn xyy(self) -> DVec3 {
        DVec3 {
            x: self.x,
            y: self.y,
            z: self.y,
        }
    }

    #[inline]
    fn yxx(self) -> DVec3 {
        DVec3 {
            x: self.y,
            y: self.x,
            z: self.x,
        }
    }

    #[inline]
    fn yxy(self) -> DVec3 {
        DVec3 {
            x: self.y,
            y: self.x,
            z: self.y,
        }
    }

    #[inline]
    fn yyx(self) -> DVec3 {
        DVec3 {
            x: self.y,
            y: self.y,
            z: self.x,
        }
    }

    #[inline]
    fn yyy(self) -> DVec3 {
        DVec3 {
            x: self.y,
            y: self.y,
            z: self.y,
        }
    }

    #[inline]
    fn xxxx(self) -> DVec4 {
        DVec4::new(self.x, self.x, self.x, self.x)
    }

    #[inline]
    fn xxxy(self) -> DVec4 {
        DVec4::new(self.x, self.x, self.x, self.y)
    }

    #[inline]
    fn xxyx(self) -> DVec4 {
        DVec4::new(self.x, self.x, self.y, self.x)
    }

    #[inline]
    fn xxyy(self) -> DVec4 {
        DVec4::new(self.x, self.x, self.y, self.y)
    }

    #[inline]
    fn xyxx(self) -> DVec4 {
        DVec4::new(self.x, self.y, self.x, self.x)
    }

    #[inline]
    fn xyxy(self) -> DVec4 {
        DVec4::new(self.x, self.y, self.x, self.y)
    }

    #[inline]
    fn xyyx(self) -> DVec4 {
        DVec4::new(self.x, self.y, self.y, self.x)
    }

    #[inline]
    fn xyyy(self) -> DVec4 {
        DVec4::new(self.x, self.y, self.y, self.y)
    }

    #[inline]
    fn yxxx(self) -> DVec4 {
        DVec4::new(self.y, self.x, self.x, self.x)
    }

    #[inline]
    fn yxxy(self) -> DVec4 {
        DVec4::new(self.y, self.x, self.x, self.y)
    }

    #[inline]
    fn yxyx(self) -> DVec4 {
        DVec4::new(self.y, self.x, self.y, self.x)
    }

    #[inline]
    fn yxyy(self) -> DVec4 {
        DVec4::new(self.y, self.x, self.y, self.y)
    }

    #[inline]
    fn yyxx(self) -> DVec4 {
        DVec4::new(self.y, self.y, self.x, self.x)
    }

    #[inline]
    fn yyxy(self) -> DVec4 {
        DVec4::new(self.y, self.y, self.x, self.y)
    }

    #[inline]
    fn yyyx(self) -> DVec4 {
        DVec4::new(self.y, self.y, self.y, self.x)
    }

    #[inline]
    fn yyyy(self) -> DVec4 {
        DVec4::new(self.y, self.y, self.y, self.y)
    }
}
