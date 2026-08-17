// pathfinder/simd/src/test.rs
//
// Copyright © 2019 The Pathfinder Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::default::{F32x4, I32x4, U32x4};
use crate::scalar::F32x4 as F32x4S;

// F32x4

#[test]
fn test_f32x4_constructors() {
    let a = F32x4::new(1.0, 2.0, 3.0, 4.0);
    assert_eq!((a[0], a[1], a[2], a[3]), (1.0, 2.0, 3.0, 4.0));
    let b = F32x4::splat(10.0);
    assert_eq!(b, F32x4::new(10.0, 10.0, 10.0, 10.0));
}

#[test]
fn test_f32x4_accessors_and_mutators() {
    let a = F32x4::new(5.0, 6.0, 7.0, 8.0);
    assert_eq!((a.x(), a.y(), a.z(), a.w()), (5.0, 6.0, 7.0, 8.0));
    let mut b = F32x4::new(10.0, 11.0, 12.0, 13.0);
    b.set_x(20.0);
    b.set_y(30.0);
    b.set_z(40.0);
    b.set_w(50.0);
    assert_eq!(b, F32x4::new(20.0, 30.0, 40.0, 50.0));
}

#[test]
fn test_f32x4_basic_ops() {
    let a = F32x4::new(1.0, 3.0, 5.0, 7.0);
    let b = F32x4::new(2.0, 2.0, 6.0, 6.0);
    let approx_recip = a.approx_recip();

    // The greatest diff we observe between the correct answer to the reciprocal and the answer that
    // llvm gives us is 1.0 vs 0.9980469, so we need to have the allowable diff be 0.002 so that it
    // passes. It's approximate, so it's fine to be a bit off.
    const ALLOWABLE_RECIP_DIFF: f32 = 0.002;
    assert!((approx_recip[0] - (1. / 1.)).abs() < ALLOWABLE_RECIP_DIFF);
    assert!((approx_recip[1] - (1. / 3.)).abs() < ALLOWABLE_RECIP_DIFF);
    assert!((approx_recip[2] - (1. / 5.)).abs() < ALLOWABLE_RECIP_DIFF);
    assert!((approx_recip[3] - (1. / 7.)).abs() < ALLOWABLE_RECIP_DIFF);

    assert_eq!(a.min(b), F32x4::new(1.0, 2.0, 5.0, 6.0));
    assert_eq!(a.max(b), F32x4::new(2.0, 3.0, 6.0, 7.0));
    let c = F32x4::new(-1.0, 1.3, -20.0, 3.6);
    assert_eq!(c.clamp(a, b), F32x4::new(1.0, 2.0, 5.0, 6.0));
    assert_eq!(c.abs(), F32x4::new(1.0, 1.3, 20.0, 3.6));
    assert_eq!(c.floor(), F32x4::new(-1.0, 1.0, -20.0, 3.0));
    assert_eq!(c.ceil(), F32x4::new(-1.0, 2.0, -20.0, 4.0));
    assert_eq!(c.to_i32x4().to_f32x4(), F32x4::new(-1.0, 1.0, -20.0, 4.0));
    let d = F32x4::new(1.0, 2.0, 3.0, 4.0);
    assert_eq!(d.sqrt(), F32x4::new(1.0, 1.4142135, 1.7320508, 2.0));
}

#[test]
fn test_f32x4_packed_comparisons() {
    let a = F32x4::new(7.0, 3.0, 6.0, -2.0);
    let b = F32x4::new(10.0, 3.0, 5.0, -2.0);
    assert_eq!(a.packed_eq(b), U32x4::new(0, !0, 0, !0));
    assert_eq!(a.packed_gt(b), U32x4::new(0, 0, !0, 0));
    assert_eq!(a.packed_le(b), U32x4::new(!0, !0, 0, !0));
}

#[test]
fn test_f32x4_swizzles() {
    let a = F32x4::new(1.0, 2.0, 3.0, 4.0);
    assert_eq!(a.xxxx(), F32x4::splat(1.0));
    assert_eq!(a.yyyy(), F32x4::splat(2.0));
    assert_eq!(a.zzzz(), F32x4::splat(3.0));
    assert_eq!(a.wwww(), F32x4::splat(4.0));

    assert_eq!(a.yxxx(), F32x4::new(2.0, 1.0, 1.0, 1.0));
    assert_eq!(a.zxxx(), F32x4::new(3.0, 1.0, 1.0, 1.0));
    assert_eq!(a.wxxx(), F32x4::new(4.0, 1.0, 1.0, 1.0));
    assert_eq!(a.xyxx(), F32x4::new(1.0, 2.0, 1.0, 1.0));
    assert_eq!(a.yyxx(), F32x4::new(2.0, 2.0, 1.0, 1.0));
    assert_eq!(a.zyxx(), F32x4::new(3.0, 2.0, 1.0, 1.0));
    assert_eq!(a.wyxx(), F32x4::new(4.0, 2.0, 1.0, 1.0));
    assert_eq!(a.xzxx(), F32x4::new(1.0, 3.0, 1.0, 1.0));
    assert_eq!(a.yzxx(), F32x4::new(2.0, 3.0, 1.0, 1.0));
    assert_eq!(a.zzxx(), F32x4::new(3.0, 3.0, 1.0, 1.0));
    assert_eq!(a.wzxx(), F32x4::new(4.0, 3.0, 1.0, 1.0));
    assert_eq!(a.xwxx(), F32x4::new(1.0, 4.0, 1.0, 1.0));
    assert_eq!(a.ywxx(), F32x4::new(2.0, 4.0, 1.0, 1.0));
    assert_eq!(a.zwxx(), F32x4::new(3.0, 4.0, 1.0, 1.0));
    assert_eq!(a.wwxx(), F32x4::new(4.0, 4.0, 1.0, 1.0));
    assert_eq!(a.xxyx(), F32x4::new(1.0, 1.0, 2.0, 1.0));
    assert_eq!(a.yxyx(), F32x4::new(2.0, 1.0, 2.0, 1.0));
    assert_eq!(a.zxyx(), F32x4::new(3.0, 1.0, 2.0, 1.0));
    assert_eq!(a.wxyx(), F32x4::new(4.0, 1.0, 2.0, 1.0));
    assert_eq!(a.xyyx(), F32x4::new(1.0, 2.0, 2.0, 1.0));
    assert_eq!(a.yyyx(), F32x4::new(2.0, 2.0, 2.0, 1.0));
    assert_eq!(a.zyyx(), F32x4::new(3.0, 2.0, 2.0, 1.0));
    assert_eq!(a.wyyx(), F32x4::new(4.0, 2.0, 2.0, 1.0));
    assert_eq!(a.xzyx(), F32x4::new(1.0, 3.0, 2.0, 1.0));
    assert_eq!(a.yzyx(), F32x4::new(2.0, 3.0, 2.0, 1.0));
    assert_eq!(a.zzyx(), F32x4::new(3.0, 3.0, 2.0, 1.0));
    assert_eq!(a.wzyx(), F32x4::new(4.0, 3.0, 2.0, 1.0));
    assert_eq!(a.xwyx(), F32x4::new(1.0, 4.0, 2.0, 1.0));
    assert_eq!(a.ywyx(), F32x4::new(2.0, 4.0, 2.0, 1.0));
    assert_eq!(a.zwyx(), F32x4::new(3.0, 4.0, 2.0, 1.0));
    assert_eq!(a.wwyx(), F32x4::new(4.0, 4.0, 2.0, 1.0));
    assert_eq!(a.xxzx(), F32x4::new(1.0, 1.0, 3.0, 1.0));
    assert_eq!(a.yxzx(), F32x4::new(2.0, 1.0, 3.0, 1.0));
    assert_eq!(a.zxzx(), F32x4::new(3.0, 1.0, 3.0, 1.0));
    assert_eq!(a.wxzx(), F32x4::new(4.0, 1.0, 3.0, 1.0));
    assert_eq!(a.xyzx(), F32x4::new(1.0, 2.0, 3.0, 1.0));
    assert_eq!(a.yyzx(), F32x4::new(2.0, 2.0, 3.0, 1.0));
    assert_eq!(a.zyzx(), F32x4::new(3.0, 2.0, 3.0, 1.0));
    assert_eq!(a.wyzx(), F32x4::new(4.0, 2.0, 3.0, 1.0));
    assert_eq!(a.xzzx(), F32x4::new(1.0, 3.0, 3.0, 1.0));
    assert_eq!(a.yzzx(), F32x4::new(2.0, 3.0, 3.0, 1.0));
    assert_eq!(a.zzzx(), F32x4::new(3.0, 3.0, 3.0, 1.0));
    assert_eq!(a.wzzx(), F32x4::new(4.0, 3.0, 3.0, 1.0));
    assert_eq!(a.xwzx(), F32x4::new(1.0, 4.0, 3.0, 1.0));
    assert_eq!(a.ywzx(), F32x4::new(2.0, 4.0, 3.0, 1.0));
    assert_eq!(a.zwzx(), F32x4::new(3.0, 4.0, 3.0, 1.0));
    assert_eq!(a.wwzx(), F32x4::new(4.0, 4.0, 3.0, 1.0));
    assert_eq!(a.xxwx(), F32x4::new(1.0, 1.0, 4.0, 1.0));
    assert_eq!(a.yxwx(), F32x4::new(2.0, 1.0, 4.0, 1.0));
    assert_eq!(a.zxwx(), F32x4::new(3.0, 1.0, 4.0, 1.0));
    assert_eq!(a.wxwx(), F32x4::new(4.0, 1.0, 4.0, 1.0));
    assert_eq!(a.xywx(), F32x4::new(1.0, 2.0, 4.0, 1.0));
    assert_eq!(a.yywx(), F32x4::new(2.0, 2.0, 4.0, 1.0));
    assert_eq!(a.zywx(), F32x4::new(3.0, 2.0, 4.0, 1.0));
    assert_eq!(a.wywx(), F32x4::new(4.0, 2.0, 4.0, 1.0));
    assert_eq!(a.xzwx(), F32x4::new(1.0, 3.0, 4.0, 1.0));
    assert_eq!(a.yzwx(), F32x4::new(2.0, 3.0, 4.0, 1.0));
    assert_eq!(a.zzwx(), F32x4::new(3.0, 3.0, 4.0, 1.0));
    assert_eq!(a.wzwx(), F32x4::new(4.0, 3.0, 4.0, 1.0));
    assert_eq!(a.xwwx(), F32x4::new(1.0, 4.0, 4.0, 1.0));
    assert_eq!(a.ywwx(), F32x4::new(2.0, 4.0, 4.0, 1.0));
    assert_eq!(a.zwwx(), F32x4::new(3.0, 4.0, 4.0, 1.0));
    assert_eq!(a.wwwx(), F32x4::new(4.0, 4.0, 4.0, 1.0));

    assert_eq!(a.xxxy(), F32x4::new(1.0, 1.0, 1.0, 2.0));
    assert_eq!(a.yxxy(), F32x4::new(2.0, 1.0, 1.0, 2.0));
    assert_eq!(a.zxxy(), F32x4::new(3.0, 1.0, 1.0, 2.0));
    assert_eq!(a.wxxy(), F32x4::new(4.0, 1.0, 1.0, 2.0));
    assert_eq!(a.xyxy(), F32x4::new(1.0, 2.0, 1.0, 2.0));
    assert_eq!(a.yyxy(), F32x4::new(2.0, 2.0, 1.0, 2.0));
    assert_eq!(a.zyxy(), F32x4::new(3.0, 2.0, 1.0, 2.0));
    assert_eq!(a.wyxy(), F32x4::new(4.0, 2.0, 1.0, 2.0));
    assert_eq!(a.xzxy(), F32x4::new(1.0, 3.0, 1.0, 2.0));
    assert_eq!(a.yzxy(), F32x4::new(2.0, 3.0, 1.0, 2.0));
    assert_eq!(a.zzxy(), F32x4::new(3.0, 3.0, 1.0, 2.0));
    assert_eq!(a.wzxy(), F32x4::new(4.0, 3.0, 1.0, 2.0));
    assert_eq!(a.xwxy(), F32x4::new(1.0, 4.0, 1.0, 2.0));
    assert_eq!(a.ywxy(), F32x4::new(2.0, 4.0, 1.0, 2.0));
    assert_eq!(a.zwxy(), F32x4::new(3.0, 4.0, 1.0, 2.0));
    assert_eq!(a.wwxy(), F32x4::new(4.0, 4.0, 1.0, 2.0));
    assert_eq!(a.xxyy(), F32x4::new(1.0, 1.0, 2.0, 2.0));
    assert_eq!(a.yxyy(), F32x4::new(2.0, 1.0, 2.0, 2.0));
    assert_eq!(a.zxyy(), F32x4::new(3.0, 1.0, 2.0, 2.0));
    assert_eq!(a.wxyy(), F32x4::new(4.0, 1.0, 2.0, 2.0));
    assert_eq!(a.xyyy(), F32x4::new(1.0, 2.0, 2.0, 2.0));
    assert_eq!(a.zyyy(), F32x4::new(3.0, 2.0, 2.0, 2.0));
    assert_eq!(a.wyyy(), F32x4::new(4.0, 2.0, 2.0, 2.0));
    assert_eq!(a.xzyy(), F32x4::new(1.0, 3.0, 2.0, 2.0));
    assert_eq!(a.yzyy(), F32x4::new(2.0, 3.0, 2.0, 2.0));
    assert_eq!(a.zzyy(), F32x4::new(3.0, 3.0, 2.0, 2.0));
    assert_eq!(a.wzyy(), F32x4::new(4.0, 3.0, 2.0, 2.0));
    assert_eq!(a.xwyy(), F32x4::new(1.0, 4.0, 2.0, 2.0));
    assert_eq!(a.ywyy(), F32x4::new(2.0, 4.0, 2.0, 2.0));
    assert_eq!(a.zwyy(), F32x4::new(3.0, 4.0, 2.0, 2.0));
    assert_eq!(a.wwyy(), F32x4::new(4.0, 4.0, 2.0, 2.0));
    assert_eq!(a.xxzy(), F32x4::new(1.0, 1.0, 3.0, 2.0));
    assert_eq!(a.yxzy(), F32x4::new(2.0, 1.0, 3.0, 2.0));
    assert_eq!(a.zxzy(), F32x4::new(3.0, 1.0, 3.0, 2.0));
    assert_eq!(a.wxzy(), F32x4::new(4.0, 1.0, 3.0, 2.0));
    assert_eq!(a.xyzy(), F32x4::new(1.0, 2.0, 3.0, 2.0));
    assert_eq!(a.yyzy(), F32x4::new(2.0, 2.0, 3.0, 2.0));
    assert_eq!(a.zyzy(), F32x4::new(3.0, 2.0, 3.0, 2.0));
    assert_eq!(a.wyzy(), F32x4::new(4.0, 2.0, 3.0, 2.0));
    assert_eq!(a.xzzy(), F32x4::new(1.0, 3.0, 3.0, 2.0));
    assert_eq!(a.yzzy(), F32x4::new(2.0, 3.0, 3.0, 2.0));
    assert_eq!(a.zzzy(), F32x4::new(3.0, 3.0, 3.0, 2.0));
    assert_eq!(a.wzzy(), F32x4::new(4.0, 3.0, 3.0, 2.0));
    assert_eq!(a.xwzy(), F32x4::new(1.0, 4.0, 3.0, 2.0));
    assert_eq!(a.ywzy(), F32x4::new(2.0, 4.0, 3.0, 2.0));
    assert_eq!(a.zwzy(), F32x4::new(3.0, 4.0, 3.0, 2.0));
    assert_eq!(a.wwzy(), F32x4::new(4.0, 4.0, 3.0, 2.0));
    assert_eq!(a.xxwy(), F32x4::new(1.0, 1.0, 4.0, 2.0));
    assert_eq!(a.yxwy(), F32x4::new(2.0, 1.0, 4.0, 2.0));
    assert_eq!(a.zxwy(), F32x4::new(3.0, 1.0, 4.0, 2.0));
    assert_eq!(a.wxwy(), F32x4::new(4.0, 1.0, 4.0, 2.0));
    assert_eq!(a.xywy(), F32x4::new(1.0, 2.0, 4.0, 2.0));
    assert_eq!(a.yywy(), F32x4::new(2.0, 2.0, 4.0, 2.0));
    assert_eq!(a.zywy(), F32x4::new(3.0, 2.0, 4.0, 2.0));
    assert_eq!(a.wywy(), F32x4::new(4.0, 2.0, 4.0, 2.0));
    assert_eq!(a.xzwy(), F32x4::new(1.0, 3.0, 4.0, 2.0));
    assert_eq!(a.yzwy(), F32x4::new(2.0, 3.0, 4.0, 2.0));
    assert_eq!(a.zzwy(), F32x4::new(3.0, 3.0, 4.0, 2.0));
    assert_eq!(a.wzwy(), F32x4::new(4.0, 3.0, 4.0, 2.0));
    assert_eq!(a.xwwy(), F32x4::new(1.0, 4.0, 4.0, 2.0));
    assert_eq!(a.ywwy(), F32x4::new(2.0, 4.0, 4.0, 2.0));
    assert_eq!(a.zwwy(), F32x4::new(3.0, 4.0, 4.0, 2.0));
    assert_eq!(a.wwwy(), F32x4::new(4.0, 4.0, 4.0, 2.0));

    assert_eq!(a.xxxw(), F32x4::new(1.0, 1.0, 1.0, 4.0));
    assert_eq!(a.yxxw(), F32x4::new(2.0, 1.0, 1.0, 4.0));
    assert_eq!(a.zxxw(), F32x4::new(3.0, 1.0, 1.0, 4.0));
    assert_eq!(a.wxxw(), F32x4::new(4.0, 1.0, 1.0, 4.0));
    assert_eq!(a.xyxw(), F32x4::new(1.0, 2.0, 1.0, 4.0));
    assert_eq!(a.yyxw(), F32x4::new(2.0, 2.0, 1.0, 4.0));
    assert_eq!(a.zyxw(), F32x4::new(3.0, 2.0, 1.0, 4.0));
    assert_eq!(a.wyxw(), F32x4::new(4.0, 2.0, 1.0, 4.0));
    assert_eq!(a.xzxw(), F32x4::new(1.0, 3.0, 1.0, 4.0));
    assert_eq!(a.yzxw(), F32x4::new(2.0, 3.0, 1.0, 4.0));
    assert_eq!(a.zzxw(), F32x4::new(3.0, 3.0, 1.0, 4.0));
    assert_eq!(a.wzxw(), F32x4::new(4.0, 3.0, 1.0, 4.0));
    assert_eq!(a.xwxw(), F32x4::new(1.0, 4.0, 1.0, 4.0));
    assert_eq!(a.ywxw(), F32x4::new(2.0, 4.0, 1.0, 4.0));
    assert_eq!(a.zwxw(), F32x4::new(3.0, 4.0, 1.0, 4.0));
    assert_eq!(a.wwxw(), F32x4::new(4.0, 4.0, 1.0, 4.0));
    assert_eq!(a.xxyw(), F32x4::new(1.0, 1.0, 2.0, 4.0));
    assert_eq!(a.yxyw(), F32x4::new(2.0, 1.0, 2.0, 4.0));
    assert_eq!(a.zxyw(), F32x4::new(3.0, 1.0, 2.0, 4.0));
    assert_eq!(a.wxyw(), F32x4::new(4.0, 1.0, 2.0, 4.0));
    assert_eq!(a.xyyw(), F32x4::new(1.0, 2.0, 2.0, 4.0));
    assert_eq!(a.yyyw(), F32x4::new(2.0, 2.0, 2.0, 4.0));
    assert_eq!(a.zyyw(), F32x4::new(3.0, 2.0, 2.0, 4.0));
    assert_eq!(a.wyyw(), F32x4::new(4.0, 2.0, 2.0, 4.0));
    assert_eq!(a.xzyw(), F32x4::new(1.0, 3.0, 2.0, 4.0));
    assert_eq!(a.yzyw(), F32x4::new(2.0, 3.0, 2.0, 4.0));
    assert_eq!(a.zzyw(), F32x4::new(3.0, 3.0, 2.0, 4.0));
    assert_eq!(a.wzyw(), F32x4::new(4.0, 3.0, 2.0, 4.0));
    assert_eq!(a.xwyw(), F32x4::new(1.0, 4.0, 2.0, 4.0));
    assert_eq!(a.ywyw(), F32x4::new(2.0, 4.0, 2.0, 4.0));
    assert_eq!(a.zwyw(), F32x4::new(3.0, 4.0, 2.0, 4.0));
    assert_eq!(a.wwyw(), F32x4::new(4.0, 4.0, 2.0, 4.0));
    assert_eq!(a.xxzw(), F32x4::new(1.0, 1.0, 3.0, 4.0));
    assert_eq!(a.yxzw(), F32x4::new(2.0, 1.0, 3.0, 4.0));
    assert_eq!(a.zxzw(), F32x4::new(3.0, 1.0, 3.0, 4.0));
    assert_eq!(a.wxzw(), F32x4::new(4.0, 1.0, 3.0, 4.0));
    assert_eq!(a.xyzw(), F32x4::new(1.0, 2.0, 3.0, 4.0));
    assert_eq!(a.yyzw(), F32x4::new(2.0, 2.0, 3.0, 4.0));
    assert_eq!(a.zyzw(), F32x4::new(3.0, 2.0, 3.0, 4.0));
    assert_eq!(a.wyzw(), F32x4::new(4.0, 2.0, 3.0, 4.0));
    assert_eq!(a.xzzw(), F32x4::new(1.0, 3.0, 3.0, 4.0));
    assert_eq!(a.yzzw(), F32x4::new(2.0, 3.0, 3.0, 4.0));
    assert_eq!(a.zzzw(), F32x4::new(3.0, 3.0, 3.0, 4.0));
    assert_eq!(a.wzzw(), F32x4::new(4.0, 3.0, 3.0, 4.0));
    assert_eq!(a.xwzw(), F32x4::new(1.0, 4.0, 3.0, 4.0));
    assert_eq!(a.ywzw(), F32x4::new(2.0, 4.0, 3.0, 4.0));
    assert_eq!(a.zwzw(), F32x4::new(3.0, 4.0, 3.0, 4.0));
    assert_eq!(a.wwzw(), F32x4::new(4.0, 4.0, 3.0, 4.0));
    assert_eq!(a.xxww(), F32x4::new(1.0, 1.0, 4.0, 4.0));
    assert_eq!(a.yxww(), F32x4::new(2.0, 1.0, 4.0, 4.0));
    assert_eq!(a.zxww(), F32x4::new(3.0, 1.0, 4.0, 4.0));
    assert_eq!(a.wxww(), F32x4::new(4.0, 1.0, 4.0, 4.0));
    assert_eq!(a.xyww(), F32x4::new(1.0, 2.0, 4.0, 4.0));
    assert_eq!(a.yyww(), F32x4::new(2.0, 2.0, 4.0, 4.0));
    assert_eq!(a.zyww(), F32x4::new(3.0, 2.0, 4.0, 4.0));
    assert_eq!(a.wzww(), F32x4::new(4.0, 3.0, 4.0, 4.0));
    assert_eq!(a.xzww(), F32x4::new(1.0, 3.0, 4.0, 4.0));
    assert_eq!(a.yzww(), F32x4::new(2.0, 3.0, 4.0, 4.0));
    assert_eq!(a.zzww(), F32x4::new(3.0, 3.0, 4.0, 4.0));
    assert_eq!(a.wzww(), F32x4::new(4.0, 3.0, 4.0, 4.0));
    assert_eq!(a.xwww(), F32x4::new(1.0, 4.0, 4.0, 4.0));
    assert_eq!(a.ywww(), F32x4::new(2.0, 4.0, 4.0, 4.0));
    assert_eq!(a.zwww(), F32x4::new(3.0, 4.0, 4.0, 4.0));

    assert_eq!(a.xxxz(), F32x4::new(1.0, 1.0, 1.0, 3.0));
    assert_eq!(a.yxxz(), F32x4::new(2.0, 1.0, 1.0, 3.0));
    assert_eq!(a.zxxz(), F32x4::new(3.0, 1.0, 1.0, 3.0));
    assert_eq!(a.wxxz(), F32x4::new(4.0, 1.0, 1.0, 3.0));
    assert_eq!(a.xyxz(), F32x4::new(1.0, 2.0, 1.0, 3.0));
    assert_eq!(a.yyxz(), F32x4::new(2.0, 2.0, 1.0, 3.0));
    assert_eq!(a.zyxz(), F32x4::new(3.0, 2.0, 1.0, 3.0));
    assert_eq!(a.wyxz(), F32x4::new(4.0, 2.0, 1.0, 3.0));
    assert_eq!(a.xzxz(), F32x4::new(1.0, 3.0, 1.0, 3.0));
    assert_eq!(a.yzxz(), F32x4::new(2.0, 3.0, 1.0, 3.0));
    assert_eq!(a.zzxz(), F32x4::new(3.0, 3.0, 1.0, 3.0));
    assert_eq!(a.wzxz(), F32x4::new(4.0, 3.0, 1.0, 3.0));
    assert_eq!(a.xwxz(), F32x4::new(1.0, 4.0, 1.0, 3.0));
    assert_eq!(a.ywxz(), F32x4::new(2.0, 4.0, 1.0, 3.0));
    assert_eq!(a.zwxz(), F32x4::new(3.0, 4.0, 1.0, 3.0));
    assert_eq!(a.wwxz(), F32x4::new(4.0, 4.0, 1.0, 3.0));
    assert_eq!(a.xxyz(), F32x4::new(1.0, 1.0, 2.0, 3.0));
    assert_eq!(a.yxyz(), F32x4::new(2.0, 1.0, 2.0, 3.0));
    assert_eq!(a.zxyz(), F32x4::new(3.0, 1.0, 2.0, 3.0));
    assert_eq!(a.wxyz(), F32x4::new(4.0, 1.0, 2.0, 3.0));
    assert_eq!(a.xyyz(), F32x4::new(1.0, 2.0, 2.0, 3.0));
    assert_eq!(a.yyyz(), F32x4::new(2.0, 2.0, 2.0, 3.0));
    assert_eq!(a.zyyz(), F32x4::new(3.0, 2.0, 2.0, 3.0));
    assert_eq!(a.wyyz(), F32x4::new(4.0, 2.0, 2.0, 3.0));
    assert_eq!(a.xzyz(), F32x4::new(1.0, 3.0, 2.0, 3.0));
    assert_eq!(a.yzyz(), F32x4::new(2.0, 3.0, 2.0, 3.0));
    assert_eq!(a.zzyz(), F32x4::new(3.0, 3.0, 2.0, 3.0));
    assert_eq!(a.wzyz(), F32x4::new(4.0, 3.0, 2.0, 3.0));
    assert_eq!(a.xwyz(), F32x4::new(1.0, 4.0, 2.0, 3.0));
    assert_eq!(a.ywyz(), F32x4::new(2.0, 4.0, 2.0, 3.0));
    assert_eq!(a.zwyz(), F32x4::new(3.0, 4.0, 2.0, 3.0));
    assert_eq!(a.wwyz(), F32x4::new(4.0, 4.0, 2.0, 3.0));
    assert_eq!(a.xxzz(), F32x4::new(1.0, 1.0, 3.0, 3.0));
    assert_eq!(a.yxzz(), F32x4::new(2.0, 1.0, 3.0, 3.0));
    assert_eq!(a.zxzz(), F32x4::new(3.0, 1.0, 3.0, 3.0));
    assert_eq!(a.wxzz(), F32x4::new(4.0, 1.0, 3.0, 3.0));
    assert_eq!(a.xyzz(), F32x4::new(1.0, 2.0, 3.0, 3.0));
    assert_eq!(a.yyzz(), F32x4::new(2.0, 2.0, 3.0, 3.0));
    assert_eq!(a.zyzz(), F32x4::new(3.0, 2.0, 3.0, 3.0));
    assert_eq!(a.wyzz(), F32x4::new(4.0, 2.0, 3.0, 3.0));
    assert_eq!(a.xzzz(), F32x4::new(1.0, 3.0, 3.0, 3.0));
    assert_eq!(a.yzzz(), F32x4::new(2.0, 3.0, 3.0, 3.0));
    assert_eq!(a.wzzz(), F32x4::new(4.0, 3.0, 3.0, 3.0));
    assert_eq!(a.xwzz(), F32x4::new(1.0, 4.0, 3.0, 3.0));
    assert_eq!(a.ywzz(), F32x4::new(2.0, 4.0, 3.0, 3.0));
    assert_eq!(a.zwzz(), F32x4::new(3.0, 4.0, 3.0, 3.0));
    assert_eq!(a.wwzz(), F32x4::new(4.0, 4.0, 3.0, 3.0));
    assert_eq!(a.xxwz(), F32x4::new(1.0, 1.0, 4.0, 3.0));
    assert_eq!(a.yxwz(), F32x4::new(2.0, 1.0, 4.0, 3.0));
    assert_eq!(a.zxwz(), F32x4::new(3.0, 1.0, 4.0, 3.0));
    assert_eq!(a.wxwz(), F32x4::new(4.0, 1.0, 4.0, 3.0));
    assert_eq!(a.xywz(), F32x4::new(1.0, 2.0, 4.0, 3.0));
    assert_eq!(a.yywz(), F32x4::new(2.0, 2.0, 4.0, 3.0));
    assert_eq!(a.zywz(), F32x4::new(3.0, 2.0, 4.0, 3.0));
    assert_eq!(a.wywz(), F32x4::new(4.0, 2.0, 4.0, 3.0));
    assert_eq!(a.xzwz(), F32x4::new(1.0, 3.0, 4.0, 3.0));
    assert_eq!(a.yzwz(), F32x4::new(2.0, 3.0, 4.0, 3.0));
    assert_eq!(a.zzwz(), F32x4::new(3.0, 3.0, 4.0, 3.0));
    assert_eq!(a.wzwz(), F32x4::new(4.0, 3.0, 4.0, 3.0));
    assert_eq!(a.xwwz(), F32x4::new(1.0, 4.0, 4.0, 3.0));
    assert_eq!(a.ywwz(), F32x4::new(2.0, 4.0, 4.0, 3.0));
    assert_eq!(a.zwwz(), F32x4::new(3.0, 4.0, 4.0, 3.0));
    assert_eq!(a.wwwz(), F32x4::new(4.0, 4.0, 4.0, 3.0));
}

#[test]
fn test_f32x4_concatenations() {
    let a = F32x4::new(4.0, 2.0, 6.0, -1.0);
    let b = F32x4::new(10.0, -3.0, 15.0, 41.0);
    assert_eq!(a.concat_xy_xy(b), F32x4::new(4.0, 2.0, 10.0, -3.0));
    assert_eq!(a.concat_xy_zw(b), F32x4::new(4.0, 2.0, 15.0, 41.0));
    assert_eq!(a.concat_zw_zw(b), F32x4::new(6.0, -1.0, 15.0, 41.0));
    assert_eq!(a.concat_wz_yx(b), F32x4::new(-1.0, 6.0, -3.0, 10.0));
}

#[test]
fn test_f32x4_arithmetic_overloads() {
    let a = F32x4::new(4.0, -1.0, 6.0, -32.0);
    let b = F32x4::new(0.5, 0.5, 10.0, 3.0);
    let a_plus_b = F32x4::new(4.5, -0.5, 16.0, -29.0);
    let a_minus_b = F32x4::new(3.5, -1.5, -4.0, -35.0);
    let a_times_b = F32x4::new(2.0, -0.5, 60.0, -96.0);
    assert_eq!(a + b, a_plus_b);
    assert_eq!(a - b, a_minus_b);
    assert_eq!(a * b, a_times_b);
    let mut c = a;
    c += b;
    assert_eq!(c, a_plus_b);
    c = a;
    c -= b;
    assert_eq!(c, a_minus_b);
    c = a;
    c *= b;
    assert_eq!(c, a_times_b);
    assert_eq!(-a, F32x4::new(-4.0, 1.0, -6.0, 32.0));
}

#[test]
fn test_f32x4_index_overloads() {
    let mut a = F32x4::new(4.0, 1.0, -32.5, 75.0);
    assert_eq!(a[2], -32.5);
    a[3] = 300.0;
    assert_eq!(a[3], 300.0);
    a[0] *= 0.5;
    assert_eq!(a[0], 2.0);
}

#[test]
fn test_f32x4_conversions() {
    let a = F32x4::new(48.0, -4.0, 200.0, 7.0);
    assert_eq!(a.to_i32x4(), I32x4::new(48, -4, 200, 7));
}

#[test]
fn test_f32x4_debug() {
    let a = F32x4::new(48.0, -4.0, 200.0, 7.0);
    assert_eq!("<48, -4, 200, 7>", format!("{:?}", a));
}

// I32x4

#[test]
fn test_i32x4_constructors() {
    let a = I32x4::new(3, 58, 10, 4);
    assert_eq!((a[0], a[1], a[2], a[3]), (3, 58, 10, 4));
    let b = I32x4::splat(39);
    assert_eq!(b, I32x4::new(39, 39, 39, 39));
}

#[test]
fn test_i32x4_basic_ops() {
    let a = I32x4::new(6, 29, -40, 2);
    let b = I32x4::new(10, -5, 10, 46);
    assert_eq!(a.min(b), I32x4::new(6, -5, -40, 2));
}

#[test]
fn test_i32x4_packed_comparisons() {
    let a = I32x4::new(59, 1, 5, 63);
    let b = I32x4::new(-59, 1, 5, 104);
    assert_eq!(a.packed_eq(b), U32x4::new(0, !0, !0, 0));
}

#[test]
fn test_i32x4_swizzles() {
    let a = I32x4::new(1, 2, 3, 4);
    assert_eq!(a.xxxx(), I32x4::splat(1));
    assert_eq!(a.yyyy(), I32x4::splat(2));
    assert_eq!(a.zzzz(), I32x4::splat(3));
    assert_eq!(a.wwww(), I32x4::splat(4));

    assert_eq!(a.yxxx(), I32x4::new(2, 1, 1, 1));
    assert_eq!(a.zxxx(), I32x4::new(3, 1, 1, 1));
    assert_eq!(a.wxxx(), I32x4::new(4, 1, 1, 1));
    assert_eq!(a.xyxx(), I32x4::new(1, 2, 1, 1));
    assert_eq!(a.yyxx(), I32x4::new(2, 2, 1, 1));
    assert_eq!(a.zyxx(), I32x4::new(3, 2, 1, 1));
    assert_eq!(a.wyxx(), I32x4::new(4, 2, 1, 1));
    assert_eq!(a.xzxx(), I32x4::new(1, 3, 1, 1));
    assert_eq!(a.yzxx(), I32x4::new(2, 3, 1, 1));
    assert_eq!(a.zzxx(), I32x4::new(3, 3, 1, 1));
    assert_eq!(a.wzxx(), I32x4::new(4, 3, 1, 1));
    assert_eq!(a.xwxx(), I32x4::new(1, 4, 1, 1));
    assert_eq!(a.ywxx(), I32x4::new(2, 4, 1, 1));
    assert_eq!(a.zwxx(), I32x4::new(3, 4, 1, 1));
    assert_eq!(a.wwxx(), I32x4::new(4, 4, 1, 1));
    assert_eq!(a.xxyx(), I32x4::new(1, 1, 2, 1));
    assert_eq!(a.yxyx(), I32x4::new(2, 1, 2, 1));
    assert_eq!(a.zxyx(), I32x4::new(3, 1, 2, 1));
    assert_eq!(a.wxyx(), I32x4::new(4, 1, 2, 1));
    assert_eq!(a.xyyx(), I32x4::new(1, 2, 2, 1));
    assert_eq!(a.yyyx(), I32x4::new(2, 2, 2, 1));
    assert_eq!(a.zyyx(), I32x4::new(3, 2, 2, 1));
    assert_eq!(a.wyyx(), I32x4::new(4, 2, 2, 1));
    assert_eq!(a.xzyx(), I32x4::new(1, 3, 2, 1));
    assert_eq!(a.yzyx(), I32x4::new(2, 3, 2, 1));
    assert_eq!(a.zzyx(), I32x4::new(3, 3, 2, 1));
    assert_eq!(a.wzyx(), I32x4::new(4, 3, 2, 1));
    assert_eq!(a.xwyx(), I32x4::new(1, 4, 2, 1));
    assert_eq!(a.ywyx(), I32x4::new(2, 4, 2, 1));
    assert_eq!(a.zwyx(), I32x4::new(3, 4, 2, 1));
    assert_eq!(a.wwyx(), I32x4::new(4, 4, 2, 1));
    assert_eq!(a.xxzx(), I32x4::new(1, 1, 3, 1));
    assert_eq!(a.yxzx(), I32x4::new(2, 1, 3, 1));
    assert_eq!(a.zxzx(), I32x4::new(3, 1, 3, 1));
    assert_eq!(a.wxzx(), I32x4::new(4, 1, 3, 1));
    assert_eq!(a.xyzx(), I32x4::new(1, 2, 3, 1));
    assert_eq!(a.yyzx(), I32x4::new(2, 2, 3, 1));
    assert_eq!(a.zyzx(), I32x4::new(3, 2, 3, 1));
    assert_eq!(a.wyzx(), I32x4::new(4, 2, 3, 1));
    assert_eq!(a.xzzx(), I32x4::new(1, 3, 3, 1));
    assert_eq!(a.yzzx(), I32x4::new(2, 3, 3, 1));
    assert_eq!(a.zzzx(), I32x4::new(3, 3, 3, 1));
    assert_eq!(a.wzzx(), I32x4::new(4, 3, 3, 1));
    assert_eq!(a.xwzx(), I32x4::new(1, 4, 3, 1));
    assert_eq!(a.ywzx(), I32x4::new(2, 4, 3, 1));
    assert_eq!(a.zwzx(), I32x4::new(3, 4, 3, 1));
    assert_eq!(a.wwzx(), I32x4::new(4, 4, 3, 1));
    assert_eq!(a.xxwx(), I32x4::new(1, 1, 4, 1));
    assert_eq!(a.yxwx(), I32x4::new(2, 1, 4, 1));
    assert_eq!(a.zxwx(), I32x4::new(3, 1, 4, 1));
    assert_eq!(a.wxwx(), I32x4::new(4, 1, 4, 1));
    assert_eq!(a.xywx(), I32x4::new(1, 2, 4, 1));
    assert_eq!(a.yywx(), I32x4::new(2, 2, 4, 1));
    assert_eq!(a.zywx(), I32x4::new(3, 2, 4, 1));
    assert_eq!(a.wywx(), I32x4::new(4, 2, 4, 1));
    assert_eq!(a.xzwx(), I32x4::new(1, 3, 4, 1));
    assert_eq!(a.yzwx(), I32x4::new(2, 3, 4, 1));
    assert_eq!(a.zzwx(), I32x4::new(3, 3, 4, 1));
    assert_eq!(a.wzwx(), I32x4::new(4, 3, 4, 1));
    assert_eq!(a.xwwx(), I32x4::new(1, 4, 4, 1));
    assert_eq!(a.ywwx(), I32x4::new(2, 4, 4, 1));
    assert_eq!(a.zwwx(), I32x4::new(3, 4, 4, 1));
    assert_eq!(a.wwwx(), I32x4::new(4, 4, 4, 1));

    assert_eq!(a.xxxy(), I32x4::new(1, 1, 1, 2));
    assert_eq!(a.yxxy(), I32x4::new(2, 1, 1, 2));
    assert_eq!(a.zxxy(), I32x4::new(3, 1, 1, 2));
    assert_eq!(a.wxxy(), I32x4::new(4, 1, 1, 2));
    assert_eq!(a.xyxy(), I32x4::new(1, 2, 1, 2));
    assert_eq!(a.yyxy(), I32x4::new(2, 2, 1, 2));
    assert_eq!(a.zyxy(), I32x4::new(3, 2, 1, 2));
    assert_eq!(a.wyxy(), I32x4::new(4, 2, 1, 2));
    assert_eq!(a.xzxy(), I32x4::new(1, 3, 1, 2));
    assert_eq!(a.yzxy(), I32x4::new(2, 3, 1, 2));
    assert_eq!(a.zzxy(), I32x4::new(3, 3, 1, 2));
    assert_eq!(a.wzxy(), I32x4::new(4, 3, 1, 2));
    assert_eq!(a.xwxy(), I32x4::new(1, 4, 1, 2));
    assert_eq!(a.ywxy(), I32x4::new(2, 4, 1, 2));
    assert_eq!(a.zwxy(), I32x4::new(3, 4, 1, 2));
    assert_eq!(a.wwxy(), I32x4::new(4, 4, 1, 2));
    assert_eq!(a.xxyy(), I32x4::new(1, 1, 2, 2));
    assert_eq!(a.yxyy(), I32x4::new(2, 1, 2, 2));
    assert_eq!(a.zxyy(), I32x4::new(3, 1, 2, 2));
    assert_eq!(a.wxyy(), I32x4::new(4, 1, 2, 2));
    assert_eq!(a.xyyy(), I32x4::new(1, 2, 2, 2));
    assert_eq!(a.zyyy(), I32x4::new(3, 2, 2, 2));
    assert_eq!(a.wyyy(), I32x4::new(4, 2, 2, 2));
    assert_eq!(a.xzyy(), I32x4::new(1, 3, 2, 2));
    assert_eq!(a.yzyy(), I32x4::new(2, 3, 2, 2));
    assert_eq!(a.zzyy(), I32x4::new(3, 3, 2, 2));
    assert_eq!(a.wzyy(), I32x4::new(4, 3, 2, 2));
    assert_eq!(a.xwyy(), I32x4::new(1, 4, 2, 2));
    assert_eq!(a.ywyy(), I32x4::new(2, 4, 2, 2));
    assert_eq!(a.zwyy(), I32x4::new(3, 4, 2, 2));
    assert_eq!(a.wwyy(), I32x4::new(4, 4, 2, 2));
    assert_eq!(a.xxzy(), I32x4::new(1, 1, 3, 2));
    assert_eq!(a.yxzy(), I32x4::new(2, 1, 3, 2));
    assert_eq!(a.zxzy(), I32x4::new(3, 1, 3, 2));
    assert_eq!(a.wxzy(), I32x4::new(4, 1, 3, 2));
    assert_eq!(a.xyzy(), I32x4::new(1, 2, 3, 2));
    assert_eq!(a.yyzy(), I32x4::new(2, 2, 3, 2));
    assert_eq!(a.zyzy(), I32x4::new(3, 2, 3, 2));
    assert_eq!(a.wyzy(), I32x4::new(4, 2, 3, 2));
    assert_eq!(a.xzzy(), I32x4::new(1, 3, 3, 2));
    assert_eq!(a.yzzy(), I32x4::new(2, 3, 3, 2));
    assert_eq!(a.zzzy(), I32x4::new(3, 3, 3, 2));
    assert_eq!(a.wzzy(), I32x4::new(4, 3, 3, 2));
    assert_eq!(a.xwzy(), I32x4::new(1, 4, 3, 2));
    assert_eq!(a.ywzy(), I32x4::new(2, 4, 3, 2));
    assert_eq!(a.zwzy(), I32x4::new(3, 4, 3, 2));
    assert_eq!(a.wwzy(), I32x4::new(4, 4, 3, 2));
    assert_eq!(a.xxwy(), I32x4::new(1, 1, 4, 2));
    assert_eq!(a.yxwy(), I32x4::new(2, 1, 4, 2));
    assert_eq!(a.zxwy(), I32x4::new(3, 1, 4, 2));
    assert_eq!(a.wxwy(), I32x4::new(4, 1, 4, 2));
    assert_eq!(a.xywy(), I32x4::new(1, 2, 4, 2));
    assert_eq!(a.yywy(), I32x4::new(2, 2, 4, 2));
    assert_eq!(a.zywy(), I32x4::new(3, 2, 4, 2));
    assert_eq!(a.wywy(), I32x4::new(4, 2, 4, 2));
    assert_eq!(a.xzwy(), I32x4::new(1, 3, 4, 2));
    assert_eq!(a.yzwy(), I32x4::new(2, 3, 4, 2));
    assert_eq!(a.zzwy(), I32x4::new(3, 3, 4, 2));
    assert_eq!(a.wzwy(), I32x4::new(4, 3, 4, 2));
    assert_eq!(a.xwwy(), I32x4::new(1, 4, 4, 2));
    assert_eq!(a.ywwy(), I32x4::new(2, 4, 4, 2));
    assert_eq!(a.zwwy(), I32x4::new(3, 4, 4, 2));
    assert_eq!(a.wwwy(), I32x4::new(4, 4, 4, 2));

    assert_eq!(a.xxxz(), I32x4::new(1, 1, 1, 3));
    assert_eq!(a.yxxz(), I32x4::new(2, 1, 1, 3));
    assert_eq!(a.zxxz(), I32x4::new(3, 1, 1, 3));
    assert_eq!(a.wxxz(), I32x4::new(4, 1, 1, 3));
    assert_eq!(a.xyxz(), I32x4::new(1, 2, 1, 3));
    assert_eq!(a.yyxz(), I32x4::new(2, 2, 1, 3));
    assert_eq!(a.zyxz(), I32x4::new(3, 2, 1, 3));
    assert_eq!(a.wyxz(), I32x4::new(4, 2, 1, 3));
    assert_eq!(a.xzxz(), I32x4::new(1, 3, 1, 3));
    assert_eq!(a.yzxz(), I32x4::new(2, 3, 1, 3));
    assert_eq!(a.zzxz(), I32x4::new(3, 3, 1, 3));
    assert_eq!(a.wzxz(), I32x4::new(4, 3, 1, 3));
    assert_eq!(a.xwxz(), I32x4::new(1, 4, 1, 3));
    assert_eq!(a.ywxz(), I32x4::new(2, 4, 1, 3));
    assert_eq!(a.zwxz(), I32x4::new(3, 4, 1, 3));
    assert_eq!(a.wwxz(), I32x4::new(4, 4, 1, 3));
    assert_eq!(a.xxyz(), I32x4::new(1, 1, 2, 3));
    assert_eq!(a.yxyz(), I32x4::new(2, 1, 2, 3));
    assert_eq!(a.zxyz(), I32x4::new(3, 1, 2, 3));
    assert_eq!(a.wxyz(), I32x4::new(4, 1, 2, 3));
    assert_eq!(a.xyyz(), I32x4::new(1, 2, 2, 3));
    assert_eq!(a.yyyz(), I32x4::new(2, 2, 2, 3));
    assert_eq!(a.zyyz(), I32x4::new(3, 2, 2, 3));
    assert_eq!(a.wyyz(), I32x4::new(4, 2, 2, 3));
    assert_eq!(a.xzyz(), I32x4::new(1, 3, 2, 3));
    assert_eq!(a.yzyz(), I32x4::new(2, 3, 2, 3));
    assert_eq!(a.zzyz(), I32x4::new(3, 3, 2, 3));
    assert_eq!(a.wzyz(), I32x4::new(4, 3, 2, 3));
    assert_eq!(a.xwyz(), I32x4::new(1, 4, 2, 3));
    assert_eq!(a.ywyz(), I32x4::new(2, 4, 2, 3));
    assert_eq!(a.zwyz(), I32x4::new(3, 4, 2, 3));
    assert_eq!(a.wwyz(), I32x4::new(4, 4, 2, 3));
    assert_eq!(a.xxzz(), I32x4::new(1, 1, 3, 3));
    assert_eq!(a.yxzz(), I32x4::new(2, 1, 3, 3));
    assert_eq!(a.zxzz(), I32x4::new(3, 1, 3, 3));
    assert_eq!(a.wxzz(), I32x4::new(4, 1, 3, 3));
    assert_eq!(a.xyzz(), I32x4::new(1, 2, 3, 3));
    assert_eq!(a.yyzz(), I32x4::new(2, 2, 3, 3));
    assert_eq!(a.zyzz(), I32x4::new(3, 2, 3, 3));
    assert_eq!(a.wyzz(), I32x4::new(4, 2, 3, 3));
    assert_eq!(a.xzzz(), I32x4::new(1, 3, 3, 3));
    assert_eq!(a.yzzz(), I32x4::new(2, 3, 3, 3));
    assert_eq!(a.wzzz(), I32x4::new(4, 3, 3, 3));
    assert_eq!(a.xwzz(), I32x4::new(1, 4, 3, 3));
    assert_eq!(a.ywzz(), I32x4::new(2, 4, 3, 3));
    assert_eq!(a.zwzz(), I32x4::new(3, 4, 3, 3));
    assert_eq!(a.wwzz(), I32x4::new(4, 4, 3, 3));
    assert_eq!(a.xxwz(), I32x4::new(1, 1, 4, 3));
    assert_eq!(a.yxwz(), I32x4::new(2, 1, 4, 3));
    assert_eq!(a.zxwz(), I32x4::new(3, 1, 4, 3));
    assert_eq!(a.wxwz(), I32x4::new(4, 1, 4, 3));
    assert_eq!(a.xywz(), I32x4::new(1, 2, 4, 3));
    assert_eq!(a.yywz(), I32x4::new(2, 2, 4, 3));
    assert_eq!(a.zywz(), I32x4::new(3, 2, 4, 3));
    assert_eq!(a.wywz(), I32x4::new(4, 2, 4, 3));
    assert_eq!(a.xzwz(), I32x4::new(1, 3, 4, 3));
    assert_eq!(a.yzwz(), I32x4::new(2, 3, 4, 3));
    assert_eq!(a.zzwz(), I32x4::new(3, 3, 4, 3));
    assert_eq!(a.wzwz(), I32x4::new(4, 3, 4, 3));
    assert_eq!(a.xwwz(), I32x4::new(1, 4, 4, 3));
    assert_eq!(a.ywwz(), I32x4::new(2, 4, 4, 3));
    assert_eq!(a.zwwz(), I32x4::new(3, 4, 4, 3));
    assert_eq!(a.wwwz(), I32x4::new(4, 4, 4, 3));

    assert_eq!(a.xxxw(), I32x4::new(1, 1, 1, 4));
    assert_eq!(a.yxxw(), I32x4::new(2, 1, 1, 4));
    assert_eq!(a.zxxw(), I32x4::new(3, 1, 1, 4));
    assert_eq!(a.wxxw(), I32x4::new(4, 1, 1, 4));
    assert_eq!(a.xyxw(), I32x4::new(1, 2, 1, 4));
    assert_eq!(a.yyxw(), I32x4::new(2, 2, 1, 4));
    assert_eq!(a.zyxw(), I32x4::new(3, 2, 1, 4));
    assert_eq!(a.wyxw(), I32x4::new(4, 2, 1, 4));
    assert_eq!(a.xzxw(), I32x4::new(1, 3, 1, 4));
    assert_eq!(a.yzxw(), I32x4::new(2, 3, 1, 4));
    assert_eq!(a.zzxw(), I32x4::new(3, 3, 1, 4));
    assert_eq!(a.wzxw(), I32x4::new(4, 3, 1, 4));
    assert_eq!(a.xwxw(), I32x4::new(1, 4, 1, 4));
    assert_eq!(a.ywxw(), I32x4::new(2, 4, 1, 4));
    assert_eq!(a.zwxw(), I32x4::new(3, 4, 1, 4));
    assert_eq!(a.wwxw(), I32x4::new(4, 4, 1, 4));
    assert_eq!(a.xxyw(), I32x4::new(1, 1, 2, 4));
    assert_eq!(a.yxyw(), I32x4::new(2, 1, 2, 4));
    assert_eq!(a.zxyw(), I32x4::new(3, 1, 2, 4));
    assert_eq!(a.wxyw(), I32x4::new(4, 1, 2, 4));
    assert_eq!(a.xyyw(), I32x4::new(1, 2, 2, 4));
    assert_eq!(a.yyyw(), I32x4::new(2, 2, 2, 4));
    assert_eq!(a.zyyw(), I32x4::new(3, 2, 2, 4));
    assert_eq!(a.wyyw(), I32x4::new(4, 2, 2, 4));
    assert_eq!(a.xzyw(), I32x4::new(1, 3, 2, 4));
    assert_eq!(a.yzyw(), I32x4::new(2, 3, 2, 4));
    assert_eq!(a.zzyw(), I32x4::new(3, 3, 2, 4));
    assert_eq!(a.wzyw(), I32x4::new(4, 3, 2, 4));
    assert_eq!(a.xwyw(), I32x4::new(1, 4, 2, 4));
    assert_eq!(a.ywyw(), I32x4::new(2, 4, 2, 4));
    assert_eq!(a.zwyw(), I32x4::new(3, 4, 2, 4));
    assert_eq!(a.wwyw(), I32x4::new(4, 4, 2, 4));
    assert_eq!(a.xxzw(), I32x4::new(1, 1, 3, 4));
    assert_eq!(a.yxzw(), I32x4::new(2, 1, 3, 4));
    assert_eq!(a.zxzw(), I32x4::new(3, 1, 3, 4));
    assert_eq!(a.wxzw(), I32x4::new(4, 1, 3, 4));
    assert_eq!(a.xyzw(), I32x4::new(1, 2, 3, 4));
    assert_eq!(a.yyzw(), I32x4::new(2, 2, 3, 4));
    assert_eq!(a.zyzw(), I32x4::new(3, 2, 3, 4));
    assert_eq!(a.wyzw(), I32x4::new(4, 2, 3, 4));
    assert_eq!(a.xzzw(), I32x4::new(1, 3, 3, 4));
    assert_eq!(a.yzzw(), I32x4::new(2, 3, 3, 4));
    assert_eq!(a.zzzw(), I32x4::new(3, 3, 3, 4));
    assert_eq!(a.wzzw(), I32x4::new(4, 3, 3, 4));
    assert_eq!(a.xwzw(), I32x4::new(1, 4, 3, 4));
    assert_eq!(a.ywzw(), I32x4::new(2, 4, 3, 4));
    assert_eq!(a.zwzw(), I32x4::new(3, 4, 3, 4));
    assert_eq!(a.wwzw(), I32x4::new(4, 4, 3, 4));
    assert_eq!(a.xxww(), I32x4::new(1, 1, 4, 4));
    assert_eq!(a.yxww(), I32x4::new(2, 1, 4, 4));
    assert_eq!(a.zxww(), I32x4::new(3, 1, 4, 4));
    assert_eq!(a.wxww(), I32x4::new(4, 1, 4, 4));
    assert_eq!(a.xyww(), I32x4::new(1, 2, 4, 4));
    assert_eq!(a.yyww(), I32x4::new(2, 2, 4, 4));
    assert_eq!(a.zyww(), I32x4::new(3, 2, 4, 4));
    assert_eq!(a.wzww(), I32x4::new(4, 3, 4, 4));
    assert_eq!(a.xzww(), I32x4::new(1, 3, 4, 4));
    assert_eq!(a.yzww(), I32x4::new(2, 3, 4, 4));
    assert_eq!(a.zzww(), I32x4::new(3, 3, 4, 4));
    assert_eq!(a.wzww(), I32x4::new(4, 3, 4, 4));
    assert_eq!(a.xwww(), I32x4::new(1, 4, 4, 4));
    assert_eq!(a.ywww(), I32x4::new(2, 4, 4, 4));
    assert_eq!(a.zwww(), I32x4::new(3, 4, 4, 4));
}

// Scalar F32x4

#[test]
fn test_f32x4s_constructors() {
    let a = F32x4S::new(1.0, 2.0, 3.0, 4.0);
    assert_eq!((a[0], a[1], a[2], a[3]), (1.0, 2.0, 3.0, 4.0));
    let b = F32x4S::splat(10.0);
    assert_eq!(b, F32x4S::new(10.0, 10.0, 10.0, 10.0));
}

#[test]
fn test_f32x4s_basic_ops() {
    let a = F32x4S::new(1.0, 3.0, 5.0, 7.0);
    let b = F32x4S::new(2.0, 2.0, 6.0, 6.0);
    assert_eq!(a.min(b), F32x4S::new(1.0, 2.0, 5.0, 6.0));
    assert_eq!(a.max(b), F32x4S::new(2.0, 3.0, 6.0, 7.0));
    let c = F32x4S::new(-1.0, 1.3, -20.0, 3.6);
    assert_eq!(c.abs(), F32x4S::new(1.0, 1.3, 20.0, 3.6));
    assert_eq!(c.floor(), F32x4S::new(-1.0, 1.0, -20.0, 3.0));
    assert_eq!(c.ceil(), F32x4S::new(-1.0, 2.0, -20.0, 4.0));
    assert_eq!(c.to_i32x4().to_f32x4(), F32x4S::new(-1.0, 1.0, -20.0, 4.0));
}
