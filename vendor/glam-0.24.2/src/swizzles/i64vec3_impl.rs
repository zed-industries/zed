// Generated from swizzle_impl.rs.tera template. Edit the template, not the generated file.

use crate::{I64Vec2, I64Vec3, I64Vec4, Vec3Swizzles};

impl Vec3Swizzles for I64Vec3 {
    type Vec2 = I64Vec2;

    type Vec4 = I64Vec4;

    #[inline]
    fn xx(self) -> I64Vec2 {
        I64Vec2 {
            x: self.x,
            y: self.x,
        }
    }

    #[inline]
    fn xy(self) -> I64Vec2 {
        I64Vec2 {
            x: self.x,
            y: self.y,
        }
    }

    #[inline]
    fn xz(self) -> I64Vec2 {
        I64Vec2 {
            x: self.x,
            y: self.z,
        }
    }

    #[inline]
    fn yx(self) -> I64Vec2 {
        I64Vec2 {
            x: self.y,
            y: self.x,
        }
    }

    #[inline]
    fn yy(self) -> I64Vec2 {
        I64Vec2 {
            x: self.y,
            y: self.y,
        }
    }

    #[inline]
    fn yz(self) -> I64Vec2 {
        I64Vec2 {
            x: self.y,
            y: self.z,
        }
    }

    #[inline]
    fn zx(self) -> I64Vec2 {
        I64Vec2 {
            x: self.z,
            y: self.x,
        }
    }

    #[inline]
    fn zy(self) -> I64Vec2 {
        I64Vec2 {
            x: self.z,
            y: self.y,
        }
    }

    #[inline]
    fn zz(self) -> I64Vec2 {
        I64Vec2 {
            x: self.z,
            y: self.z,
        }
    }

    #[inline]
    fn xxx(self) -> I64Vec3 {
        I64Vec3 {
            x: self.x,
            y: self.x,
            z: self.x,
        }
    }

    #[inline]
    fn xxy(self) -> I64Vec3 {
        I64Vec3 {
            x: self.x,
            y: self.x,
            z: self.y,
        }
    }

    #[inline]
    fn xxz(self) -> I64Vec3 {
        I64Vec3 {
            x: self.x,
            y: self.x,
            z: self.z,
        }
    }

    #[inline]
    fn xyx(self) -> I64Vec3 {
        I64Vec3 {
            x: self.x,
            y: self.y,
            z: self.x,
        }
    }

    #[inline]
    fn xyy(self) -> I64Vec3 {
        I64Vec3 {
            x: self.x,
            y: self.y,
            z: self.y,
        }
    }

    #[inline]
    fn xyz(self) -> I64Vec3 {
        I64Vec3 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }

    #[inline]
    fn xzx(self) -> I64Vec3 {
        I64Vec3 {
            x: self.x,
            y: self.z,
            z: self.x,
        }
    }

    #[inline]
    fn xzy(self) -> I64Vec3 {
        I64Vec3 {
            x: self.x,
            y: self.z,
            z: self.y,
        }
    }

    #[inline]
    fn xzz(self) -> I64Vec3 {
        I64Vec3 {
            x: self.x,
            y: self.z,
            z: self.z,
        }
    }

    #[inline]
    fn yxx(self) -> I64Vec3 {
        I64Vec3 {
            x: self.y,
            y: self.x,
            z: self.x,
        }
    }

    #[inline]
    fn yxy(self) -> I64Vec3 {
        I64Vec3 {
            x: self.y,
            y: self.x,
            z: self.y,
        }
    }

    #[inline]
    fn yxz(self) -> I64Vec3 {
        I64Vec3 {
            x: self.y,
            y: self.x,
            z: self.z,
        }
    }

    #[inline]
    fn yyx(self) -> I64Vec3 {
        I64Vec3 {
            x: self.y,
            y: self.y,
            z: self.x,
        }
    }

    #[inline]
    fn yyy(self) -> I64Vec3 {
        I64Vec3 {
            x: self.y,
            y: self.y,
            z: self.y,
        }
    }

    #[inline]
    fn yyz(self) -> I64Vec3 {
        I64Vec3 {
            x: self.y,
            y: self.y,
            z: self.z,
        }
    }

    #[inline]
    fn yzx(self) -> I64Vec3 {
        I64Vec3 {
            x: self.y,
            y: self.z,
            z: self.x,
        }
    }

    #[inline]
    fn yzy(self) -> I64Vec3 {
        I64Vec3 {
            x: self.y,
            y: self.z,
            z: self.y,
        }
    }

    #[inline]
    fn yzz(self) -> I64Vec3 {
        I64Vec3 {
            x: self.y,
            y: self.z,
            z: self.z,
        }
    }

    #[inline]
    fn zxx(self) -> I64Vec3 {
        I64Vec3 {
            x: self.z,
            y: self.x,
            z: self.x,
        }
    }

    #[inline]
    fn zxy(self) -> I64Vec3 {
        I64Vec3 {
            x: self.z,
            y: self.x,
            z: self.y,
        }
    }

    #[inline]
    fn zxz(self) -> I64Vec3 {
        I64Vec3 {
            x: self.z,
            y: self.x,
            z: self.z,
        }
    }

    #[inline]
    fn zyx(self) -> I64Vec3 {
        I64Vec3 {
            x: self.z,
            y: self.y,
            z: self.x,
        }
    }

    #[inline]
    fn zyy(self) -> I64Vec3 {
        I64Vec3 {
            x: self.z,
            y: self.y,
            z: self.y,
        }
    }

    #[inline]
    fn zyz(self) -> I64Vec3 {
        I64Vec3 {
            x: self.z,
            y: self.y,
            z: self.z,
        }
    }

    #[inline]
    fn zzx(self) -> I64Vec3 {
        I64Vec3 {
            x: self.z,
            y: self.z,
            z: self.x,
        }
    }

    #[inline]
    fn zzy(self) -> I64Vec3 {
        I64Vec3 {
            x: self.z,
            y: self.z,
            z: self.y,
        }
    }

    #[inline]
    fn zzz(self) -> I64Vec3 {
        I64Vec3 {
            x: self.z,
            y: self.z,
            z: self.z,
        }
    }

    #[inline]
    fn xxxx(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.x, self.x, self.x)
    }

    #[inline]
    fn xxxy(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.x, self.x, self.y)
    }

    #[inline]
    fn xxxz(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.x, self.x, self.z)
    }

    #[inline]
    fn xxyx(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.x, self.y, self.x)
    }

    #[inline]
    fn xxyy(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.x, self.y, self.y)
    }

    #[inline]
    fn xxyz(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.x, self.y, self.z)
    }

    #[inline]
    fn xxzx(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.x, self.z, self.x)
    }

    #[inline]
    fn xxzy(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.x, self.z, self.y)
    }

    #[inline]
    fn xxzz(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.x, self.z, self.z)
    }

    #[inline]
    fn xyxx(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.y, self.x, self.x)
    }

    #[inline]
    fn xyxy(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.y, self.x, self.y)
    }

    #[inline]
    fn xyxz(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.y, self.x, self.z)
    }

    #[inline]
    fn xyyx(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.y, self.y, self.x)
    }

    #[inline]
    fn xyyy(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.y, self.y, self.y)
    }

    #[inline]
    fn xyyz(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.y, self.y, self.z)
    }

    #[inline]
    fn xyzx(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.y, self.z, self.x)
    }

    #[inline]
    fn xyzy(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.y, self.z, self.y)
    }

    #[inline]
    fn xyzz(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.y, self.z, self.z)
    }

    #[inline]
    fn xzxx(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.z, self.x, self.x)
    }

    #[inline]
    fn xzxy(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.z, self.x, self.y)
    }

    #[inline]
    fn xzxz(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.z, self.x, self.z)
    }

    #[inline]
    fn xzyx(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.z, self.y, self.x)
    }

    #[inline]
    fn xzyy(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.z, self.y, self.y)
    }

    #[inline]
    fn xzyz(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.z, self.y, self.z)
    }

    #[inline]
    fn xzzx(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.z, self.z, self.x)
    }

    #[inline]
    fn xzzy(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.z, self.z, self.y)
    }

    #[inline]
    fn xzzz(self) -> I64Vec4 {
        I64Vec4::new(self.x, self.z, self.z, self.z)
    }

    #[inline]
    fn yxxx(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.x, self.x, self.x)
    }

    #[inline]
    fn yxxy(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.x, self.x, self.y)
    }

    #[inline]
    fn yxxz(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.x, self.x, self.z)
    }

    #[inline]
    fn yxyx(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.x, self.y, self.x)
    }

    #[inline]
    fn yxyy(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.x, self.y, self.y)
    }

    #[inline]
    fn yxyz(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.x, self.y, self.z)
    }

    #[inline]
    fn yxzx(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.x, self.z, self.x)
    }

    #[inline]
    fn yxzy(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.x, self.z, self.y)
    }

    #[inline]
    fn yxzz(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.x, self.z, self.z)
    }

    #[inline]
    fn yyxx(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.y, self.x, self.x)
    }

    #[inline]
    fn yyxy(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.y, self.x, self.y)
    }

    #[inline]
    fn yyxz(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.y, self.x, self.z)
    }

    #[inline]
    fn yyyx(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.y, self.y, self.x)
    }

    #[inline]
    fn yyyy(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.y, self.y, self.y)
    }

    #[inline]
    fn yyyz(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.y, self.y, self.z)
    }

    #[inline]
    fn yyzx(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.y, self.z, self.x)
    }

    #[inline]
    fn yyzy(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.y, self.z, self.y)
    }

    #[inline]
    fn yyzz(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.y, self.z, self.z)
    }

    #[inline]
    fn yzxx(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.z, self.x, self.x)
    }

    #[inline]
    fn yzxy(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.z, self.x, self.y)
    }

    #[inline]
    fn yzxz(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.z, self.x, self.z)
    }

    #[inline]
    fn yzyx(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.z, self.y, self.x)
    }

    #[inline]
    fn yzyy(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.z, self.y, self.y)
    }

    #[inline]
    fn yzyz(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.z, self.y, self.z)
    }

    #[inline]
    fn yzzx(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.z, self.z, self.x)
    }

    #[inline]
    fn yzzy(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.z, self.z, self.y)
    }

    #[inline]
    fn yzzz(self) -> I64Vec4 {
        I64Vec4::new(self.y, self.z, self.z, self.z)
    }

    #[inline]
    fn zxxx(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.x, self.x, self.x)
    }

    #[inline]
    fn zxxy(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.x, self.x, self.y)
    }

    #[inline]
    fn zxxz(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.x, self.x, self.z)
    }

    #[inline]
    fn zxyx(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.x, self.y, self.x)
    }

    #[inline]
    fn zxyy(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.x, self.y, self.y)
    }

    #[inline]
    fn zxyz(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.x, self.y, self.z)
    }

    #[inline]
    fn zxzx(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.x, self.z, self.x)
    }

    #[inline]
    fn zxzy(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.x, self.z, self.y)
    }

    #[inline]
    fn zxzz(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.x, self.z, self.z)
    }

    #[inline]
    fn zyxx(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.y, self.x, self.x)
    }

    #[inline]
    fn zyxy(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.y, self.x, self.y)
    }

    #[inline]
    fn zyxz(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.y, self.x, self.z)
    }

    #[inline]
    fn zyyx(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.y, self.y, self.x)
    }

    #[inline]
    fn zyyy(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.y, self.y, self.y)
    }

    #[inline]
    fn zyyz(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.y, self.y, self.z)
    }

    #[inline]
    fn zyzx(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.y, self.z, self.x)
    }

    #[inline]
    fn zyzy(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.y, self.z, self.y)
    }

    #[inline]
    fn zyzz(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.y, self.z, self.z)
    }

    #[inline]
    fn zzxx(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.z, self.x, self.x)
    }

    #[inline]
    fn zzxy(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.z, self.x, self.y)
    }

    #[inline]
    fn zzxz(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.z, self.x, self.z)
    }

    #[inline]
    fn zzyx(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.z, self.y, self.x)
    }

    #[inline]
    fn zzyy(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.z, self.y, self.y)
    }

    #[inline]
    fn zzyz(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.z, self.y, self.z)
    }

    #[inline]
    fn zzzx(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.z, self.z, self.x)
    }

    #[inline]
    fn zzzy(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.z, self.z, self.y)
    }

    #[inline]
    fn zzzz(self) -> I64Vec4 {
        I64Vec4::new(self.z, self.z, self.z, self.z)
    }
}
