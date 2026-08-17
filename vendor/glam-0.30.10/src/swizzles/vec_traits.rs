// Generated from swizzle_traits.rs.tera template. Edit the template, not the generated file.

pub trait Vec2Swizzles: Sized + Copy + Clone {
    type Vec3;

    type Vec4;

    #[inline]
    #[must_use]
    fn xy(self) -> Self {
        self
    }

    #[must_use]
    fn xx(self) -> Self;

    #[must_use]
    fn yx(self) -> Self;

    #[must_use]
    fn yy(self) -> Self;

    #[must_use]
    fn xxx(self) -> Self::Vec3;

    #[must_use]
    fn xxy(self) -> Self::Vec3;

    #[must_use]
    fn xyx(self) -> Self::Vec3;

    #[must_use]
    fn xyy(self) -> Self::Vec3;

    #[must_use]
    fn yxx(self) -> Self::Vec3;

    #[must_use]
    fn yxy(self) -> Self::Vec3;

    #[must_use]
    fn yyx(self) -> Self::Vec3;

    #[must_use]
    fn yyy(self) -> Self::Vec3;

    #[must_use]
    fn xxxx(self) -> Self::Vec4;

    #[must_use]
    fn xxxy(self) -> Self::Vec4;

    #[must_use]
    fn xxyx(self) -> Self::Vec4;

    #[must_use]
    fn xxyy(self) -> Self::Vec4;

    #[must_use]
    fn xyxx(self) -> Self::Vec4;

    #[must_use]
    fn xyxy(self) -> Self::Vec4;

    #[must_use]
    fn xyyx(self) -> Self::Vec4;

    #[must_use]
    fn xyyy(self) -> Self::Vec4;

    #[must_use]
    fn yxxx(self) -> Self::Vec4;

    #[must_use]
    fn yxxy(self) -> Self::Vec4;

    #[must_use]
    fn yxyx(self) -> Self::Vec4;

    #[must_use]
    fn yxyy(self) -> Self::Vec4;

    #[must_use]
    fn yyxx(self) -> Self::Vec4;

    #[must_use]
    fn yyxy(self) -> Self::Vec4;

    #[must_use]
    fn yyyx(self) -> Self::Vec4;

    #[must_use]
    fn yyyy(self) -> Self::Vec4;
}

pub trait Vec3Swizzles: Sized + Copy + Clone {
    type Vec2;

    type Vec4;

    #[inline]
    #[must_use]
    fn xyz(self) -> Self {
        self
    }

    #[must_use]
    fn xx(self) -> Self::Vec2;

    #[must_use]
    fn xy(self) -> Self::Vec2;

    #[must_use]
    fn with_xy(self, rhs: Self::Vec2) -> Self;

    #[must_use]
    fn xz(self) -> Self::Vec2;

    #[must_use]
    fn with_xz(self, rhs: Self::Vec2) -> Self;

    #[must_use]
    fn yx(self) -> Self::Vec2;

    #[must_use]
    fn with_yx(self, rhs: Self::Vec2) -> Self;

    #[must_use]
    fn yy(self) -> Self::Vec2;

    #[must_use]
    fn yz(self) -> Self::Vec2;

    #[must_use]
    fn with_yz(self, rhs: Self::Vec2) -> Self;

    #[must_use]
    fn zx(self) -> Self::Vec2;

    #[must_use]
    fn with_zx(self, rhs: Self::Vec2) -> Self;

    #[must_use]
    fn zy(self) -> Self::Vec2;

    #[must_use]
    fn with_zy(self, rhs: Self::Vec2) -> Self;

    #[must_use]
    fn zz(self) -> Self::Vec2;

    #[must_use]
    fn xxx(self) -> Self;

    #[must_use]
    fn xxy(self) -> Self;

    #[must_use]
    fn xxz(self) -> Self;

    #[must_use]
    fn xyx(self) -> Self;

    #[must_use]
    fn xyy(self) -> Self;

    #[must_use]
    fn xzx(self) -> Self;

    #[must_use]
    fn xzy(self) -> Self;

    #[must_use]
    fn xzz(self) -> Self;

    #[must_use]
    fn yxx(self) -> Self;

    #[must_use]
    fn yxy(self) -> Self;

    #[must_use]
    fn yxz(self) -> Self;

    #[must_use]
    fn yyx(self) -> Self;

    #[must_use]
    fn yyy(self) -> Self;

    #[must_use]
    fn yyz(self) -> Self;

    #[must_use]
    fn yzx(self) -> Self;

    #[must_use]
    fn yzy(self) -> Self;

    #[must_use]
    fn yzz(self) -> Self;

    #[must_use]
    fn zxx(self) -> Self;

    #[must_use]
    fn zxy(self) -> Self;

    #[must_use]
    fn zxz(self) -> Self;

    #[must_use]
    fn zyx(self) -> Self;

    #[must_use]
    fn zyy(self) -> Self;

    #[must_use]
    fn zyz(self) -> Self;

    #[must_use]
    fn zzx(self) -> Self;

    #[must_use]
    fn zzy(self) -> Self;

    #[must_use]
    fn zzz(self) -> Self;

    #[must_use]
    fn xxxx(self) -> Self::Vec4;

    #[must_use]
    fn xxxy(self) -> Self::Vec4;

    #[must_use]
    fn xxxz(self) -> Self::Vec4;

    #[must_use]
    fn xxyx(self) -> Self::Vec4;

    #[must_use]
    fn xxyy(self) -> Self::Vec4;

    #[must_use]
    fn xxyz(self) -> Self::Vec4;

    #[must_use]
    fn xxzx(self) -> Self::Vec4;

    #[must_use]
    fn xxzy(self) -> Self::Vec4;

    #[must_use]
    fn xxzz(self) -> Self::Vec4;

    #[must_use]
    fn xyxx(self) -> Self::Vec4;

    #[must_use]
    fn xyxy(self) -> Self::Vec4;

    #[must_use]
    fn xyxz(self) -> Self::Vec4;

    #[must_use]
    fn xyyx(self) -> Self::Vec4;

    #[must_use]
    fn xyyy(self) -> Self::Vec4;

    #[must_use]
    fn xyyz(self) -> Self::Vec4;

    #[must_use]
    fn xyzx(self) -> Self::Vec4;

    #[must_use]
    fn xyzy(self) -> Self::Vec4;

    #[must_use]
    fn xyzz(self) -> Self::Vec4;

    #[must_use]
    fn xzxx(self) -> Self::Vec4;

    #[must_use]
    fn xzxy(self) -> Self::Vec4;

    #[must_use]
    fn xzxz(self) -> Self::Vec4;

    #[must_use]
    fn xzyx(self) -> Self::Vec4;

    #[must_use]
    fn xzyy(self) -> Self::Vec4;

    #[must_use]
    fn xzyz(self) -> Self::Vec4;

    #[must_use]
    fn xzzx(self) -> Self::Vec4;

    #[must_use]
    fn xzzy(self) -> Self::Vec4;

    #[must_use]
    fn xzzz(self) -> Self::Vec4;

    #[must_use]
    fn yxxx(self) -> Self::Vec4;

    #[must_use]
    fn yxxy(self) -> Self::Vec4;

    #[must_use]
    fn yxxz(self) -> Self::Vec4;

    #[must_use]
    fn yxyx(self) -> Self::Vec4;

    #[must_use]
    fn yxyy(self) -> Self::Vec4;

    #[must_use]
    fn yxyz(self) -> Self::Vec4;

    #[must_use]
    fn yxzx(self) -> Self::Vec4;

    #[must_use]
    fn yxzy(self) -> Self::Vec4;

    #[must_use]
    fn yxzz(self) -> Self::Vec4;

    #[must_use]
    fn yyxx(self) -> Self::Vec4;

    #[must_use]
    fn yyxy(self) -> Self::Vec4;

    #[must_use]
    fn yyxz(self) -> Self::Vec4;

    #[must_use]
    fn yyyx(self) -> Self::Vec4;

    #[must_use]
    fn yyyy(self) -> Self::Vec4;

    #[must_use]
    fn yyyz(self) -> Self::Vec4;

    #[must_use]
    fn yyzx(self) -> Self::Vec4;

    #[must_use]
    fn yyzy(self) -> Self::Vec4;

    #[must_use]
    fn yyzz(self) -> Self::Vec4;

    #[must_use]
    fn yzxx(self) -> Self::Vec4;

    #[must_use]
    fn yzxy(self) -> Self::Vec4;

    #[must_use]
    fn yzxz(self) -> Self::Vec4;

    #[must_use]
    fn yzyx(self) -> Self::Vec4;

    #[must_use]
    fn yzyy(self) -> Self::Vec4;

    #[must_use]
    fn yzyz(self) -> Self::Vec4;

    #[must_use]
    fn yzzx(self) -> Self::Vec4;

    #[must_use]
    fn yzzy(self) -> Self::Vec4;

    #[must_use]
    fn yzzz(self) -> Self::Vec4;

    #[must_use]
    fn zxxx(self) -> Self::Vec4;

    #[must_use]
    fn zxxy(self) -> Self::Vec4;

    #[must_use]
    fn zxxz(self) -> Self::Vec4;

    #[must_use]
    fn zxyx(self) -> Self::Vec4;

    #[must_use]
    fn zxyy(self) -> Self::Vec4;

    #[must_use]
    fn zxyz(self) -> Self::Vec4;

    #[must_use]
    fn zxzx(self) -> Self::Vec4;

    #[must_use]
    fn zxzy(self) -> Self::Vec4;

    #[must_use]
    fn zxzz(self) -> Self::Vec4;

    #[must_use]
    fn zyxx(self) -> Self::Vec4;

    #[must_use]
    fn zyxy(self) -> Self::Vec4;

    #[must_use]
    fn zyxz(self) -> Self::Vec4;

    #[must_use]
    fn zyyx(self) -> Self::Vec4;

    #[must_use]
    fn zyyy(self) -> Self::Vec4;

    #[must_use]
    fn zyyz(self) -> Self::Vec4;

    #[must_use]
    fn zyzx(self) -> Self::Vec4;

    #[must_use]
    fn zyzy(self) -> Self::Vec4;

    #[must_use]
    fn zyzz(self) -> Self::Vec4;

    #[must_use]
    fn zzxx(self) -> Self::Vec4;

    #[must_use]
    fn zzxy(self) -> Self::Vec4;

    #[must_use]
    fn zzxz(self) -> Self::Vec4;

    #[must_use]
    fn zzyx(self) -> Self::Vec4;

    #[must_use]
    fn zzyy(self) -> Self::Vec4;

    #[must_use]
    fn zzyz(self) -> Self::Vec4;

    #[must_use]
    fn zzzx(self) -> Self::Vec4;

    #[must_use]
    fn zzzy(self) -> Self::Vec4;

    #[must_use]
    fn zzzz(self) -> Self::Vec4;
}

pub trait Vec4Swizzles: Sized + Copy + Clone {
    type Vec2;

    type Vec3;

    #[inline]
    #[must_use]
    fn xyzw(self) -> Self {
        self
    }

    #[must_use]
    fn xx(self) -> Self::Vec2;

    #[must_use]
    fn xy(self) -> Self::Vec2;

    #[must_use]
    fn with_xy(self, rhs: Self::Vec2) -> Self;

    #[must_use]
    fn xz(self) -> Self::Vec2;

    #[must_use]
    fn with_xz(self, rhs: Self::Vec2) -> Self;

    #[must_use]
    fn xw(self) -> Self::Vec2;

    #[must_use]
    fn with_xw(self, rhs: Self::Vec2) -> Self;

    #[must_use]
    fn yx(self) -> Self::Vec2;

    #[must_use]
    fn with_yx(self, rhs: Self::Vec2) -> Self;

    #[must_use]
    fn yy(self) -> Self::Vec2;

    #[must_use]
    fn yz(self) -> Self::Vec2;

    #[must_use]
    fn with_yz(self, rhs: Self::Vec2) -> Self;

    #[must_use]
    fn yw(self) -> Self::Vec2;

    #[must_use]
    fn with_yw(self, rhs: Self::Vec2) -> Self;

    #[must_use]
    fn zx(self) -> Self::Vec2;

    #[must_use]
    fn with_zx(self, rhs: Self::Vec2) -> Self;

    #[must_use]
    fn zy(self) -> Self::Vec2;

    #[must_use]
    fn with_zy(self, rhs: Self::Vec2) -> Self;

    #[must_use]
    fn zz(self) -> Self::Vec2;

    #[must_use]
    fn zw(self) -> Self::Vec2;

    #[must_use]
    fn with_zw(self, rhs: Self::Vec2) -> Self;

    #[must_use]
    fn wx(self) -> Self::Vec2;

    #[must_use]
    fn with_wx(self, rhs: Self::Vec2) -> Self;

    #[must_use]
    fn wy(self) -> Self::Vec2;

    #[must_use]
    fn with_wy(self, rhs: Self::Vec2) -> Self;

    #[must_use]
    fn wz(self) -> Self::Vec2;

    #[must_use]
    fn with_wz(self, rhs: Self::Vec2) -> Self;

    #[must_use]
    fn ww(self) -> Self::Vec2;

    #[must_use]
    fn xxx(self) -> Self::Vec3;

    #[must_use]
    fn xxy(self) -> Self::Vec3;

    #[must_use]
    fn xxz(self) -> Self::Vec3;

    #[must_use]
    fn xxw(self) -> Self::Vec3;

    #[must_use]
    fn xyx(self) -> Self::Vec3;

    #[must_use]
    fn xyy(self) -> Self::Vec3;

    #[must_use]
    fn xyz(self) -> Self::Vec3;

    #[must_use]
    fn with_xyz(self, rhs: Self::Vec3) -> Self;

    #[must_use]
    fn xyw(self) -> Self::Vec3;

    #[must_use]
    fn with_xyw(self, rhs: Self::Vec3) -> Self;

    #[must_use]
    fn xzx(self) -> Self::Vec3;

    #[must_use]
    fn xzy(self) -> Self::Vec3;

    #[must_use]
    fn with_xzy(self, rhs: Self::Vec3) -> Self;

    #[must_use]
    fn xzz(self) -> Self::Vec3;

    #[must_use]
    fn xzw(self) -> Self::Vec3;

    #[must_use]
    fn with_xzw(self, rhs: Self::Vec3) -> Self;

    #[must_use]
    fn xwx(self) -> Self::Vec3;

    #[must_use]
    fn xwy(self) -> Self::Vec3;

    #[must_use]
    fn with_xwy(self, rhs: Self::Vec3) -> Self;

    #[must_use]
    fn xwz(self) -> Self::Vec3;

    #[must_use]
    fn with_xwz(self, rhs: Self::Vec3) -> Self;

    #[must_use]
    fn xww(self) -> Self::Vec3;

    #[must_use]
    fn yxx(self) -> Self::Vec3;

    #[must_use]
    fn yxy(self) -> Self::Vec3;

    #[must_use]
    fn yxz(self) -> Self::Vec3;

    #[must_use]
    fn with_yxz(self, rhs: Self::Vec3) -> Self;

    #[must_use]
    fn yxw(self) -> Self::Vec3;

    #[must_use]
    fn with_yxw(self, rhs: Self::Vec3) -> Self;

    #[must_use]
    fn yyx(self) -> Self::Vec3;

    #[must_use]
    fn yyy(self) -> Self::Vec3;

    #[must_use]
    fn yyz(self) -> Self::Vec3;

    #[must_use]
    fn yyw(self) -> Self::Vec3;

    #[must_use]
    fn yzx(self) -> Self::Vec3;

    #[must_use]
    fn with_yzx(self, rhs: Self::Vec3) -> Self;

    #[must_use]
    fn yzy(self) -> Self::Vec3;

    #[must_use]
    fn yzz(self) -> Self::Vec3;

    #[must_use]
    fn yzw(self) -> Self::Vec3;

    #[must_use]
    fn with_yzw(self, rhs: Self::Vec3) -> Self;

    #[must_use]
    fn ywx(self) -> Self::Vec3;

    #[must_use]
    fn with_ywx(self, rhs: Self::Vec3) -> Self;

    #[must_use]
    fn ywy(self) -> Self::Vec3;

    #[must_use]
    fn ywz(self) -> Self::Vec3;

    #[must_use]
    fn with_ywz(self, rhs: Self::Vec3) -> Self;

    #[must_use]
    fn yww(self) -> Self::Vec3;

    #[must_use]
    fn zxx(self) -> Self::Vec3;

    #[must_use]
    fn zxy(self) -> Self::Vec3;

    #[must_use]
    fn with_zxy(self, rhs: Self::Vec3) -> Self;

    #[must_use]
    fn zxz(self) -> Self::Vec3;

    #[must_use]
    fn zxw(self) -> Self::Vec3;

    #[must_use]
    fn with_zxw(self, rhs: Self::Vec3) -> Self;

    #[must_use]
    fn zyx(self) -> Self::Vec3;

    #[must_use]
    fn with_zyx(self, rhs: Self::Vec3) -> Self;

    #[must_use]
    fn zyy(self) -> Self::Vec3;

    #[must_use]
    fn zyz(self) -> Self::Vec3;

    #[must_use]
    fn zyw(self) -> Self::Vec3;

    #[must_use]
    fn with_zyw(self, rhs: Self::Vec3) -> Self;

    #[must_use]
    fn zzx(self) -> Self::Vec3;

    #[must_use]
    fn zzy(self) -> Self::Vec3;

    #[must_use]
    fn zzz(self) -> Self::Vec3;

    #[must_use]
    fn zzw(self) -> Self::Vec3;

    #[must_use]
    fn zwx(self) -> Self::Vec3;

    #[must_use]
    fn with_zwx(self, rhs: Self::Vec3) -> Self;

    #[must_use]
    fn zwy(self) -> Self::Vec3;

    #[must_use]
    fn with_zwy(self, rhs: Self::Vec3) -> Self;

    #[must_use]
    fn zwz(self) -> Self::Vec3;

    #[must_use]
    fn zww(self) -> Self::Vec3;

    #[must_use]
    fn wxx(self) -> Self::Vec3;

    #[must_use]
    fn wxy(self) -> Self::Vec3;

    #[must_use]
    fn with_wxy(self, rhs: Self::Vec3) -> Self;

    #[must_use]
    fn wxz(self) -> Self::Vec3;

    #[must_use]
    fn with_wxz(self, rhs: Self::Vec3) -> Self;

    #[must_use]
    fn wxw(self) -> Self::Vec3;

    #[must_use]
    fn wyx(self) -> Self::Vec3;

    #[must_use]
    fn with_wyx(self, rhs: Self::Vec3) -> Self;

    #[must_use]
    fn wyy(self) -> Self::Vec3;

    #[must_use]
    fn wyz(self) -> Self::Vec3;

    #[must_use]
    fn with_wyz(self, rhs: Self::Vec3) -> Self;

    #[must_use]
    fn wyw(self) -> Self::Vec3;

    #[must_use]
    fn wzx(self) -> Self::Vec3;

    #[must_use]
    fn with_wzx(self, rhs: Self::Vec3) -> Self;

    #[must_use]
    fn wzy(self) -> Self::Vec3;

    #[must_use]
    fn with_wzy(self, rhs: Self::Vec3) -> Self;

    #[must_use]
    fn wzz(self) -> Self::Vec3;

    #[must_use]
    fn wzw(self) -> Self::Vec3;

    #[must_use]
    fn wwx(self) -> Self::Vec3;

    #[must_use]
    fn wwy(self) -> Self::Vec3;

    #[must_use]
    fn wwz(self) -> Self::Vec3;

    #[must_use]
    fn www(self) -> Self::Vec3;

    #[must_use]
    fn xxxx(self) -> Self;

    #[must_use]
    fn xxxy(self) -> Self;

    #[must_use]
    fn xxxz(self) -> Self;

    #[must_use]
    fn xxxw(self) -> Self;

    #[must_use]
    fn xxyx(self) -> Self;

    #[must_use]
    fn xxyy(self) -> Self;

    #[must_use]
    fn xxyz(self) -> Self;

    #[must_use]
    fn xxyw(self) -> Self;

    #[must_use]
    fn xxzx(self) -> Self;

    #[must_use]
    fn xxzy(self) -> Self;

    #[must_use]
    fn xxzz(self) -> Self;

    #[must_use]
    fn xxzw(self) -> Self;

    #[must_use]
    fn xxwx(self) -> Self;

    #[must_use]
    fn xxwy(self) -> Self;

    #[must_use]
    fn xxwz(self) -> Self;

    #[must_use]
    fn xxww(self) -> Self;

    #[must_use]
    fn xyxx(self) -> Self;

    #[must_use]
    fn xyxy(self) -> Self;

    #[must_use]
    fn xyxz(self) -> Self;

    #[must_use]
    fn xyxw(self) -> Self;

    #[must_use]
    fn xyyx(self) -> Self;

    #[must_use]
    fn xyyy(self) -> Self;

    #[must_use]
    fn xyyz(self) -> Self;

    #[must_use]
    fn xyyw(self) -> Self;

    #[must_use]
    fn xyzx(self) -> Self;

    #[must_use]
    fn xyzy(self) -> Self;

    #[must_use]
    fn xyzz(self) -> Self;

    #[must_use]
    fn xywx(self) -> Self;

    #[must_use]
    fn xywy(self) -> Self;

    #[must_use]
    fn xywz(self) -> Self;

    #[must_use]
    fn xyww(self) -> Self;

    #[must_use]
    fn xzxx(self) -> Self;

    #[must_use]
    fn xzxy(self) -> Self;

    #[must_use]
    fn xzxz(self) -> Self;

    #[must_use]
    fn xzxw(self) -> Self;

    #[must_use]
    fn xzyx(self) -> Self;

    #[must_use]
    fn xzyy(self) -> Self;

    #[must_use]
    fn xzyz(self) -> Self;

    #[must_use]
    fn xzyw(self) -> Self;

    #[must_use]
    fn xzzx(self) -> Self;

    #[must_use]
    fn xzzy(self) -> Self;

    #[must_use]
    fn xzzz(self) -> Self;

    #[must_use]
    fn xzzw(self) -> Self;

    #[must_use]
    fn xzwx(self) -> Self;

    #[must_use]
    fn xzwy(self) -> Self;

    #[must_use]
    fn xzwz(self) -> Self;

    #[must_use]
    fn xzww(self) -> Self;

    #[must_use]
    fn xwxx(self) -> Self;

    #[must_use]
    fn xwxy(self) -> Self;

    #[must_use]
    fn xwxz(self) -> Self;

    #[must_use]
    fn xwxw(self) -> Self;

    #[must_use]
    fn xwyx(self) -> Self;

    #[must_use]
    fn xwyy(self) -> Self;

    #[must_use]
    fn xwyz(self) -> Self;

    #[must_use]
    fn xwyw(self) -> Self;

    #[must_use]
    fn xwzx(self) -> Self;

    #[must_use]
    fn xwzy(self) -> Self;

    #[must_use]
    fn xwzz(self) -> Self;

    #[must_use]
    fn xwzw(self) -> Self;

    #[must_use]
    fn xwwx(self) -> Self;

    #[must_use]
    fn xwwy(self) -> Self;

    #[must_use]
    fn xwwz(self) -> Self;

    #[must_use]
    fn xwww(self) -> Self;

    #[must_use]
    fn yxxx(self) -> Self;

    #[must_use]
    fn yxxy(self) -> Self;

    #[must_use]
    fn yxxz(self) -> Self;

    #[must_use]
    fn yxxw(self) -> Self;

    #[must_use]
    fn yxyx(self) -> Self;

    #[must_use]
    fn yxyy(self) -> Self;

    #[must_use]
    fn yxyz(self) -> Self;

    #[must_use]
    fn yxyw(self) -> Self;

    #[must_use]
    fn yxzx(self) -> Self;

    #[must_use]
    fn yxzy(self) -> Self;

    #[must_use]
    fn yxzz(self) -> Self;

    #[must_use]
    fn yxzw(self) -> Self;

    #[must_use]
    fn yxwx(self) -> Self;

    #[must_use]
    fn yxwy(self) -> Self;

    #[must_use]
    fn yxwz(self) -> Self;

    #[must_use]
    fn yxww(self) -> Self;

    #[must_use]
    fn yyxx(self) -> Self;

    #[must_use]
    fn yyxy(self) -> Self;

    #[must_use]
    fn yyxz(self) -> Self;

    #[must_use]
    fn yyxw(self) -> Self;

    #[must_use]
    fn yyyx(self) -> Self;

    #[must_use]
    fn yyyy(self) -> Self;

    #[must_use]
    fn yyyz(self) -> Self;

    #[must_use]
    fn yyyw(self) -> Self;

    #[must_use]
    fn yyzx(self) -> Self;

    #[must_use]
    fn yyzy(self) -> Self;

    #[must_use]
    fn yyzz(self) -> Self;

    #[must_use]
    fn yyzw(self) -> Self;

    #[must_use]
    fn yywx(self) -> Self;

    #[must_use]
    fn yywy(self) -> Self;

    #[must_use]
    fn yywz(self) -> Self;

    #[must_use]
    fn yyww(self) -> Self;

    #[must_use]
    fn yzxx(self) -> Self;

    #[must_use]
    fn yzxy(self) -> Self;

    #[must_use]
    fn yzxz(self) -> Self;

    #[must_use]
    fn yzxw(self) -> Self;

    #[must_use]
    fn yzyx(self) -> Self;

    #[must_use]
    fn yzyy(self) -> Self;

    #[must_use]
    fn yzyz(self) -> Self;

    #[must_use]
    fn yzyw(self) -> Self;

    #[must_use]
    fn yzzx(self) -> Self;

    #[must_use]
    fn yzzy(self) -> Self;

    #[must_use]
    fn yzzz(self) -> Self;

    #[must_use]
    fn yzzw(self) -> Self;

    #[must_use]
    fn yzwx(self) -> Self;

    #[must_use]
    fn yzwy(self) -> Self;

    #[must_use]
    fn yzwz(self) -> Self;

    #[must_use]
    fn yzww(self) -> Self;

    #[must_use]
    fn ywxx(self) -> Self;

    #[must_use]
    fn ywxy(self) -> Self;

    #[must_use]
    fn ywxz(self) -> Self;

    #[must_use]
    fn ywxw(self) -> Self;

    #[must_use]
    fn ywyx(self) -> Self;

    #[must_use]
    fn ywyy(self) -> Self;

    #[must_use]
    fn ywyz(self) -> Self;

    #[must_use]
    fn ywyw(self) -> Self;

    #[must_use]
    fn ywzx(self) -> Self;

    #[must_use]
    fn ywzy(self) -> Self;

    #[must_use]
    fn ywzz(self) -> Self;

    #[must_use]
    fn ywzw(self) -> Self;

    #[must_use]
    fn ywwx(self) -> Self;

    #[must_use]
    fn ywwy(self) -> Self;

    #[must_use]
    fn ywwz(self) -> Self;

    #[must_use]
    fn ywww(self) -> Self;

    #[must_use]
    fn zxxx(self) -> Self;

    #[must_use]
    fn zxxy(self) -> Self;

    #[must_use]
    fn zxxz(self) -> Self;

    #[must_use]
    fn zxxw(self) -> Self;

    #[must_use]
    fn zxyx(self) -> Self;

    #[must_use]
    fn zxyy(self) -> Self;

    #[must_use]
    fn zxyz(self) -> Self;

    #[must_use]
    fn zxyw(self) -> Self;

    #[must_use]
    fn zxzx(self) -> Self;

    #[must_use]
    fn zxzy(self) -> Self;

    #[must_use]
    fn zxzz(self) -> Self;

    #[must_use]
    fn zxzw(self) -> Self;

    #[must_use]
    fn zxwx(self) -> Self;

    #[must_use]
    fn zxwy(self) -> Self;

    #[must_use]
    fn zxwz(self) -> Self;

    #[must_use]
    fn zxww(self) -> Self;

    #[must_use]
    fn zyxx(self) -> Self;

    #[must_use]
    fn zyxy(self) -> Self;

    #[must_use]
    fn zyxz(self) -> Self;

    #[must_use]
    fn zyxw(self) -> Self;

    #[must_use]
    fn zyyx(self) -> Self;

    #[must_use]
    fn zyyy(self) -> Self;

    #[must_use]
    fn zyyz(self) -> Self;

    #[must_use]
    fn zyyw(self) -> Self;

    #[must_use]
    fn zyzx(self) -> Self;

    #[must_use]
    fn zyzy(self) -> Self;

    #[must_use]
    fn zyzz(self) -> Self;

    #[must_use]
    fn zyzw(self) -> Self;

    #[must_use]
    fn zywx(self) -> Self;

    #[must_use]
    fn zywy(self) -> Self;

    #[must_use]
    fn zywz(self) -> Self;

    #[must_use]
    fn zyww(self) -> Self;

    #[must_use]
    fn zzxx(self) -> Self;

    #[must_use]
    fn zzxy(self) -> Self;

    #[must_use]
    fn zzxz(self) -> Self;

    #[must_use]
    fn zzxw(self) -> Self;

    #[must_use]
    fn zzyx(self) -> Self;

    #[must_use]
    fn zzyy(self) -> Self;

    #[must_use]
    fn zzyz(self) -> Self;

    #[must_use]
    fn zzyw(self) -> Self;

    #[must_use]
    fn zzzx(self) -> Self;

    #[must_use]
    fn zzzy(self) -> Self;

    #[must_use]
    fn zzzz(self) -> Self;

    #[must_use]
    fn zzzw(self) -> Self;

    #[must_use]
    fn zzwx(self) -> Self;

    #[must_use]
    fn zzwy(self) -> Self;

    #[must_use]
    fn zzwz(self) -> Self;

    #[must_use]
    fn zzww(self) -> Self;

    #[must_use]
    fn zwxx(self) -> Self;

    #[must_use]
    fn zwxy(self) -> Self;

    #[must_use]
    fn zwxz(self) -> Self;

    #[must_use]
    fn zwxw(self) -> Self;

    #[must_use]
    fn zwyx(self) -> Self;

    #[must_use]
    fn zwyy(self) -> Self;

    #[must_use]
    fn zwyz(self) -> Self;

    #[must_use]
    fn zwyw(self) -> Self;

    #[must_use]
    fn zwzx(self) -> Self;

    #[must_use]
    fn zwzy(self) -> Self;

    #[must_use]
    fn zwzz(self) -> Self;

    #[must_use]
    fn zwzw(self) -> Self;

    #[must_use]
    fn zwwx(self) -> Self;

    #[must_use]
    fn zwwy(self) -> Self;

    #[must_use]
    fn zwwz(self) -> Self;

    #[must_use]
    fn zwww(self) -> Self;

    #[must_use]
    fn wxxx(self) -> Self;

    #[must_use]
    fn wxxy(self) -> Self;

    #[must_use]
    fn wxxz(self) -> Self;

    #[must_use]
    fn wxxw(self) -> Self;

    #[must_use]
    fn wxyx(self) -> Self;

    #[must_use]
    fn wxyy(self) -> Self;

    #[must_use]
    fn wxyz(self) -> Self;

    #[must_use]
    fn wxyw(self) -> Self;

    #[must_use]
    fn wxzx(self) -> Self;

    #[must_use]
    fn wxzy(self) -> Self;

    #[must_use]
    fn wxzz(self) -> Self;

    #[must_use]
    fn wxzw(self) -> Self;

    #[must_use]
    fn wxwx(self) -> Self;

    #[must_use]
    fn wxwy(self) -> Self;

    #[must_use]
    fn wxwz(self) -> Self;

    #[must_use]
    fn wxww(self) -> Self;

    #[must_use]
    fn wyxx(self) -> Self;

    #[must_use]
    fn wyxy(self) -> Self;

    #[must_use]
    fn wyxz(self) -> Self;

    #[must_use]
    fn wyxw(self) -> Self;

    #[must_use]
    fn wyyx(self) -> Self;

    #[must_use]
    fn wyyy(self) -> Self;

    #[must_use]
    fn wyyz(self) -> Self;

    #[must_use]
    fn wyyw(self) -> Self;

    #[must_use]
    fn wyzx(self) -> Self;

    #[must_use]
    fn wyzy(self) -> Self;

    #[must_use]
    fn wyzz(self) -> Self;

    #[must_use]
    fn wyzw(self) -> Self;

    #[must_use]
    fn wywx(self) -> Self;

    #[must_use]
    fn wywy(self) -> Self;

    #[must_use]
    fn wywz(self) -> Self;

    #[must_use]
    fn wyww(self) -> Self;

    #[must_use]
    fn wzxx(self) -> Self;

    #[must_use]
    fn wzxy(self) -> Self;

    #[must_use]
    fn wzxz(self) -> Self;

    #[must_use]
    fn wzxw(self) -> Self;

    #[must_use]
    fn wzyx(self) -> Self;

    #[must_use]
    fn wzyy(self) -> Self;

    #[must_use]
    fn wzyz(self) -> Self;

    #[must_use]
    fn wzyw(self) -> Self;

    #[must_use]
    fn wzzx(self) -> Self;

    #[must_use]
    fn wzzy(self) -> Self;

    #[must_use]
    fn wzzz(self) -> Self;

    #[must_use]
    fn wzzw(self) -> Self;

    #[must_use]
    fn wzwx(self) -> Self;

    #[must_use]
    fn wzwy(self) -> Self;

    #[must_use]
    fn wzwz(self) -> Self;

    #[must_use]
    fn wzww(self) -> Self;

    #[must_use]
    fn wwxx(self) -> Self;

    #[must_use]
    fn wwxy(self) -> Self;

    #[must_use]
    fn wwxz(self) -> Self;

    #[must_use]
    fn wwxw(self) -> Self;

    #[must_use]
    fn wwyx(self) -> Self;

    #[must_use]
    fn wwyy(self) -> Self;

    #[must_use]
    fn wwyz(self) -> Self;

    #[must_use]
    fn wwyw(self) -> Self;

    #[must_use]
    fn wwzx(self) -> Self;

    #[must_use]
    fn wwzy(self) -> Self;

    #[must_use]
    fn wwzz(self) -> Self;

    #[must_use]
    fn wwzw(self) -> Self;

    #[must_use]
    fn wwwx(self) -> Self;

    #[must_use]
    fn wwwy(self) -> Self;

    #[must_use]
    fn wwwz(self) -> Self;

    #[must_use]
    fn wwww(self) -> Self;
}
