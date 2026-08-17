// Generated from swizzle_impl.rs.tera template. Edit the template, not the generated file.

use crate::{U64Vec2, U64Vec3, U64Vec4, Vec2Swizzles};

impl Vec2Swizzles for U64Vec2 {
    type Vec3 = U64Vec3;

    type Vec4 = U64Vec4;

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
    fn xxx(self) -> U64Vec3 {
        U64Vec3::new(self.x, self.x, self.x)
    }

    #[inline]
    fn xxy(self) -> U64Vec3 {
        U64Vec3::new(self.x, self.x, self.y)
    }

    #[inline]
    fn xyx(self) -> U64Vec3 {
        U64Vec3::new(self.x, self.y, self.x)
    }

    #[inline]
    fn xyy(self) -> U64Vec3 {
        U64Vec3::new(self.x, self.y, self.y)
    }

    #[inline]
    fn yxx(self) -> U64Vec3 {
        U64Vec3::new(self.y, self.x, self.x)
    }

    #[inline]
    fn yxy(self) -> U64Vec3 {
        U64Vec3::new(self.y, self.x, self.y)
    }

    #[inline]
    fn yyx(self) -> U64Vec3 {
        U64Vec3::new(self.y, self.y, self.x)
    }

    #[inline]
    fn yyy(self) -> U64Vec3 {
        U64Vec3::new(self.y, self.y, self.y)
    }

    #[inline]
    fn xxxx(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.x, self.x, self.x)
    }

    #[inline]
    fn xxxy(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.x, self.x, self.y)
    }

    #[inline]
    fn xxyx(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.x, self.y, self.x)
    }

    #[inline]
    fn xxyy(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.x, self.y, self.y)
    }

    #[inline]
    fn xyxx(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.y, self.x, self.x)
    }

    #[inline]
    fn xyxy(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.y, self.x, self.y)
    }

    #[inline]
    fn xyyx(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.y, self.y, self.x)
    }

    #[inline]
    fn xyyy(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.y, self.y, self.y)
    }

    #[inline]
    fn yxxx(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.x, self.x, self.x)
    }

    #[inline]
    fn yxxy(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.x, self.x, self.y)
    }

    #[inline]
    fn yxyx(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.x, self.y, self.x)
    }

    #[inline]
    fn yxyy(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.x, self.y, self.y)
    }

    #[inline]
    fn yyxx(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.y, self.x, self.x)
    }

    #[inline]
    fn yyxy(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.y, self.x, self.y)
    }

    #[inline]
    fn yyyx(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.y, self.y, self.x)
    }

    #[inline]
    fn yyyy(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.y, self.y, self.y)
    }
}
