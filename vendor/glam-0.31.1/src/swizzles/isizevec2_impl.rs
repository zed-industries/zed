// Generated from swizzle_impl.rs.tera template. Edit the template, not the generated file.

use crate::{ISizeVec2, ISizeVec3, ISizeVec4, Vec2Swizzles};

impl Vec2Swizzles for ISizeVec2 {
    type Vec3 = ISizeVec3;

    type Vec4 = ISizeVec4;

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
    fn xxx(self) -> ISizeVec3 {
        ISizeVec3::new(self.x, self.x, self.x)
    }

    #[inline]
    fn xxy(self) -> ISizeVec3 {
        ISizeVec3::new(self.x, self.x, self.y)
    }

    #[inline]
    fn xyx(self) -> ISizeVec3 {
        ISizeVec3::new(self.x, self.y, self.x)
    }

    #[inline]
    fn xyy(self) -> ISizeVec3 {
        ISizeVec3::new(self.x, self.y, self.y)
    }

    #[inline]
    fn yxx(self) -> ISizeVec3 {
        ISizeVec3::new(self.y, self.x, self.x)
    }

    #[inline]
    fn yxy(self) -> ISizeVec3 {
        ISizeVec3::new(self.y, self.x, self.y)
    }

    #[inline]
    fn yyx(self) -> ISizeVec3 {
        ISizeVec3::new(self.y, self.y, self.x)
    }

    #[inline]
    fn yyy(self) -> ISizeVec3 {
        ISizeVec3::new(self.y, self.y, self.y)
    }

    #[inline]
    fn xxxx(self) -> ISizeVec4 {
        ISizeVec4::new(self.x, self.x, self.x, self.x)
    }

    #[inline]
    fn xxxy(self) -> ISizeVec4 {
        ISizeVec4::new(self.x, self.x, self.x, self.y)
    }

    #[inline]
    fn xxyx(self) -> ISizeVec4 {
        ISizeVec4::new(self.x, self.x, self.y, self.x)
    }

    #[inline]
    fn xxyy(self) -> ISizeVec4 {
        ISizeVec4::new(self.x, self.x, self.y, self.y)
    }

    #[inline]
    fn xyxx(self) -> ISizeVec4 {
        ISizeVec4::new(self.x, self.y, self.x, self.x)
    }

    #[inline]
    fn xyxy(self) -> ISizeVec4 {
        ISizeVec4::new(self.x, self.y, self.x, self.y)
    }

    #[inline]
    fn xyyx(self) -> ISizeVec4 {
        ISizeVec4::new(self.x, self.y, self.y, self.x)
    }

    #[inline]
    fn xyyy(self) -> ISizeVec4 {
        ISizeVec4::new(self.x, self.y, self.y, self.y)
    }

    #[inline]
    fn yxxx(self) -> ISizeVec4 {
        ISizeVec4::new(self.y, self.x, self.x, self.x)
    }

    #[inline]
    fn yxxy(self) -> ISizeVec4 {
        ISizeVec4::new(self.y, self.x, self.x, self.y)
    }

    #[inline]
    fn yxyx(self) -> ISizeVec4 {
        ISizeVec4::new(self.y, self.x, self.y, self.x)
    }

    #[inline]
    fn yxyy(self) -> ISizeVec4 {
        ISizeVec4::new(self.y, self.x, self.y, self.y)
    }

    #[inline]
    fn yyxx(self) -> ISizeVec4 {
        ISizeVec4::new(self.y, self.y, self.x, self.x)
    }

    #[inline]
    fn yyxy(self) -> ISizeVec4 {
        ISizeVec4::new(self.y, self.y, self.x, self.y)
    }

    #[inline]
    fn yyyx(self) -> ISizeVec4 {
        ISizeVec4::new(self.y, self.y, self.y, self.x)
    }

    #[inline]
    fn yyyy(self) -> ISizeVec4 {
        ISizeVec4::new(self.y, self.y, self.y, self.y)
    }
}
