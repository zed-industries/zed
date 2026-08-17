// Generated from swizzle_impl.rs.tera template. Edit the template, not the generated file.

use crate::{USizeVec2, USizeVec3, USizeVec4, Vec2Swizzles};

impl Vec2Swizzles for USizeVec2 {
    type Vec3 = USizeVec3;

    type Vec4 = USizeVec4;

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
    fn xxx(self) -> USizeVec3 {
        USizeVec3::new(self.x, self.x, self.x)
    }

    #[inline]
    fn xxy(self) -> USizeVec3 {
        USizeVec3::new(self.x, self.x, self.y)
    }

    #[inline]
    fn xyx(self) -> USizeVec3 {
        USizeVec3::new(self.x, self.y, self.x)
    }

    #[inline]
    fn xyy(self) -> USizeVec3 {
        USizeVec3::new(self.x, self.y, self.y)
    }

    #[inline]
    fn yxx(self) -> USizeVec3 {
        USizeVec3::new(self.y, self.x, self.x)
    }

    #[inline]
    fn yxy(self) -> USizeVec3 {
        USizeVec3::new(self.y, self.x, self.y)
    }

    #[inline]
    fn yyx(self) -> USizeVec3 {
        USizeVec3::new(self.y, self.y, self.x)
    }

    #[inline]
    fn yyy(self) -> USizeVec3 {
        USizeVec3::new(self.y, self.y, self.y)
    }

    #[inline]
    fn xxxx(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.x, self.x, self.x)
    }

    #[inline]
    fn xxxy(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.x, self.x, self.y)
    }

    #[inline]
    fn xxyx(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.x, self.y, self.x)
    }

    #[inline]
    fn xxyy(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.x, self.y, self.y)
    }

    #[inline]
    fn xyxx(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.y, self.x, self.x)
    }

    #[inline]
    fn xyxy(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.y, self.x, self.y)
    }

    #[inline]
    fn xyyx(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.y, self.y, self.x)
    }

    #[inline]
    fn xyyy(self) -> USizeVec4 {
        USizeVec4::new(self.x, self.y, self.y, self.y)
    }

    #[inline]
    fn yxxx(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.x, self.x, self.x)
    }

    #[inline]
    fn yxxy(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.x, self.x, self.y)
    }

    #[inline]
    fn yxyx(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.x, self.y, self.x)
    }

    #[inline]
    fn yxyy(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.x, self.y, self.y)
    }

    #[inline]
    fn yyxx(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.y, self.x, self.x)
    }

    #[inline]
    fn yyxy(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.y, self.x, self.y)
    }

    #[inline]
    fn yyyx(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.y, self.y, self.x)
    }

    #[inline]
    fn yyyy(self) -> USizeVec4 {
        USizeVec4::new(self.y, self.y, self.y, self.y)
    }
}
