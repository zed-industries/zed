//! Pulley registers.

use crate::machinst::{Reg, Writable};
use regalloc2::{PReg, RegClass, VReg};

#[inline]
pub fn x_reg(enc: usize) -> Reg {
    let p = PReg::new(enc, RegClass::Int);
    let v = VReg::new(p.index(), p.class());
    Reg::from(v)
}

#[inline]
pub const fn px_reg(enc: usize) -> PReg {
    PReg::new(enc, RegClass::Int)
}

#[inline]
pub fn f_reg(enc: usize) -> Reg {
    let p = PReg::new(enc, RegClass::Float);
    let v = VReg::new(p.index(), p.class());
    Reg::from(v)
}

#[inline]
pub const fn pf_reg(enc: usize) -> PReg {
    PReg::new(enc, RegClass::Float)
}

#[inline]
pub fn v_reg(enc: usize) -> Reg {
    let p = PReg::new(enc, RegClass::Vector);
    let v = VReg::new(p.index(), p.class());
    Reg::from(v)
}

#[inline]
pub const fn pv_reg(enc: usize) -> PReg {
    PReg::new(enc, RegClass::Vector)
}

macro_rules! define_registers {
    (
        $(
            $reg:expr => $readable:ident, $writable:ident;
        )*
    ) => {
        $(
            #[inline]
            #[allow(dead_code, reason = "generated code")]
            pub fn $readable() -> Reg {
                $reg
            }

            #[inline]
            #[allow(dead_code, reason = "generated code")]
            pub fn $writable() -> Writable<Reg> {
                Writable::from_reg($readable())
            }
        )*
    };
}

define_registers! {
    x_reg(0) => x0, writable_x0;
    x_reg(1) => x1, writable_x1;
    x_reg(2) => x2, writable_x2;
    x_reg(3) => x3, writable_x3;
    x_reg(4) => x4, writable_x4;
    x_reg(5) => x5, writable_x5;
    x_reg(6) => x6, writable_x6;
    x_reg(7) => x7, writable_x7;
    x_reg(8) => x8, writable_x8;
    x_reg(9) => x9, writable_x9;
    x_reg(10) => x10, writable_x10;
    x_reg(11) => x11, writable_x11;
    x_reg(12) => x12, writable_x12;
    x_reg(13) => x13, writable_x13;
    x_reg(14) => x14, writable_x14;
    x_reg(15) => x15, writable_x15;
    x_reg(16) => x16, writable_x16;
    x_reg(17) => x17, writable_x17;
    x_reg(18) => x18, writable_x18;
    x_reg(19) => x19, writable_x19;
    x_reg(20) => x20, writable_x20;
    x_reg(21) => x21, writable_x21;
    x_reg(22) => x22, writable_x22;
    x_reg(23) => x23, writable_x23;
    x_reg(24) => x24, writable_x24;
    x_reg(25) => x25, writable_x25;
    x_reg(26) => x26, writable_x26;
    x_reg(27) => x27, writable_x27;
    x_reg(28) => x28, writable_x28;
    x_reg(29) => x29, writable_x29;

    x_reg(30) => stack_reg, writable_stack_reg;
    x_reg(31) => spilltmp_reg, writable_spilltmp_reg;

    f_reg(0) => f0, writable_f0;
    f_reg(1) => f1, writable_f1;
    f_reg(2) => f2, writable_f2;
    f_reg(3) => f3, writable_f3;
    f_reg(4) => f4, writable_f4;
    f_reg(5) => f5, writable_f5;
    f_reg(6) => f6, writable_f6;
    f_reg(7) => f7, writable_f7;
    f_reg(8) => f8, writable_f8;
    f_reg(9) => f9, writable_f9;
    f_reg(10) => f10, writable_f10;
    f_reg(11) => f11, writable_f11;
    f_reg(12) => f12, writable_f12;
    f_reg(13) => f13, writable_f13;
    f_reg(14) => f14, writable_f14;
    f_reg(15) => f15, writable_f15;
    f_reg(16) => f16, writable_f16;
    f_reg(17) => f17, writable_f17;
    f_reg(18) => f18, writable_f18;
    f_reg(19) => f19, writable_f19;
    f_reg(20) => f20, writable_f20;
    f_reg(21) => f21, writable_f21;
    f_reg(22) => f22, writable_f22;
    f_reg(23) => f23, writable_f23;
    f_reg(24) => f24, writable_f24;
    f_reg(25) => f25, writable_f25;
    f_reg(26) => f26, writable_f26;
    f_reg(27) => f27, writable_f27;
    f_reg(28) => f28, writable_f28;
    f_reg(29) => f29, writable_f29;
    f_reg(30) => f30, writable_f30;
    f_reg(31) => f31, writable_f31;

    v_reg(0) => v0, writable_v0;
    v_reg(1) => v1, writable_v1;
    v_reg(2) => v2, writable_v2;
    v_reg(3) => v3, writable_v3;
    v_reg(4) => v4, writable_v4;
    v_reg(5) => v5, writable_v5;
    v_reg(6) => v6, writable_v6;
    v_reg(7) => v7, writable_v7;
    v_reg(8) => v8, writable_v8;
    v_reg(9) => v9, writable_v9;
    v_reg(10) => v10, writable_v10;
    v_reg(11) => v11, writable_v11;
    v_reg(12) => v12, writable_v12;
    v_reg(13) => v13, writable_v13;
    v_reg(14) => v14, writable_v14;
    v_reg(15) => v15, writable_v15;
    v_reg(16) => v16, writable_v16;
    v_reg(17) => v17, writable_v17;
    v_reg(18) => v18, writable_v18;
    v_reg(19) => v19, writable_v19;
    v_reg(20) => v20, writable_v20;
    v_reg(21) => v21, writable_v21;
    v_reg(22) => v22, writable_v22;
    v_reg(23) => v23, writable_v23;
    v_reg(24) => v24, writable_v24;
    v_reg(25) => v25, writable_v25;
    v_reg(26) => v26, writable_v26;
    v_reg(27) => v27, writable_v27;
    v_reg(28) => v28, writable_v28;
    v_reg(29) => v29, writable_v29;
    v_reg(30) => v30, writable_v30;
    v_reg(31) => v31, writable_v31;
}
