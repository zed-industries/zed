use crate::dsl::{Feature::*, Inst, Length::*, Location::*};
use crate::dsl::{fmt, inst, r, rw, vex};

#[rustfmt::skip] // Keeps instructions on a single line.
pub fn list() -> Vec<Inst> {
    let single_ops = [rw(xmm1), r(xmm2), r(xmm_m32)];
    let double_ops = [rw(xmm1), r(xmm2), r(xmm_m64)];
    let packed_ops = [rw(xmm1), r(xmm2), r(xmm_m128)];
    let enc = || vex(LIG)._66()._0f38();
    vec![
        // Fused Multiply-Add (FMA) instructions. The digits in the instruction
        // mnemonic correspond to the combination of operands (`op*`): e.g.,
        // - `132` means `op1 * op2 + op3`,
        // - `213` means `op2 * op1 + op3`, and
        // - `231` means `op2 * op3 + op1`.
        inst("vfmadd132ss", fmt("A", single_ops), enc().w0().op(0x99).r(), _64b | compat | fma),
        inst("vfmadd213ss", fmt("A", single_ops), enc().w0().op(0xA9).r(), _64b | compat | fma),
        inst("vfmadd231ss", fmt("A", single_ops), enc().w0().op(0xB9).r(), _64b | compat | fma),
        inst("vfmadd132sd", fmt("A", double_ops), enc().w1().op(0x99).r(), _64b | compat | fma),
        inst("vfmadd213sd", fmt("A", double_ops), enc().w1().op(0xA9).r(), _64b | compat | fma),
        inst("vfmadd231sd", fmt("A", double_ops), enc().w1().op(0xB9).r(), _64b | compat | fma),
        inst("vfmadd132ps", fmt("A", packed_ops), enc().w0().op(0x98).r(), _64b | compat | fma),
        inst("vfmadd213ps", fmt("A", packed_ops), enc().w0().op(0xA8).r(), _64b | compat | fma),
        inst("vfmadd231ps", fmt("A", packed_ops), enc().w0().op(0xB8).r(), _64b | compat | fma),
        inst("vfmadd132pd", fmt("A", packed_ops), enc().w1().op(0x98).r(), _64b | compat | fma),
        inst("vfmadd213pd", fmt("A", packed_ops), enc().w1().op(0xA8).r(), _64b | compat | fma),
        inst("vfmadd231pd", fmt("A", packed_ops), enc().w1().op(0xB8).r(), _64b | compat | fma),
        // Fused Negative Multiply-Add (FNMA); like FMA, but with the
        // multiplication result negated.
        inst("vfnmadd132ss", fmt("A", single_ops), enc().w0().op(0x9D).r(), _64b | compat | fma),
        inst("vfnmadd213ss", fmt("A", single_ops), enc().w0().op(0xAD).r(), _64b | compat | fma),
        inst("vfnmadd231ss", fmt("A", single_ops), enc().w0().op(0xBD).r(), _64b | compat | fma),
        inst("vfnmadd132sd", fmt("A", double_ops), enc().w1().op(0x9D).r(), _64b | compat | fma),
        inst("vfnmadd213sd", fmt("A", double_ops), enc().w1().op(0xAD).r(), _64b | compat | fma),
        inst("vfnmadd231sd", fmt("A", double_ops), enc().w1().op(0xBD).r(), _64b | compat | fma),
        inst("vfnmadd132ps", fmt("A", packed_ops), enc().w0().op(0x9C).r(), _64b | compat | fma),
        inst("vfnmadd213ps", fmt("A", packed_ops), enc().w0().op(0xAC).r(), _64b | compat | fma),
        inst("vfnmadd231ps", fmt("A", packed_ops), enc().w0().op(0xBC).r(), _64b | compat | fma),
        inst("vfnmadd132pd", fmt("A", packed_ops), enc().w1().op(0x9C).r(), _64b | compat | fma),
        inst("vfnmadd213pd", fmt("A", packed_ops), enc().w1().op(0xAC).r(), _64b | compat | fma),
        inst("vfnmadd231pd", fmt("A", packed_ops), enc().w1().op(0xBC).r(), _64b | compat | fma),
        // Fused Multiply-Subtract (FMS); like FMA, but subtracting
        // from the multiplication result.
        inst("vfmsub132ss", fmt("A", single_ops), enc().w0().op(0x9B).r(), _64b | compat | fma),
        inst("vfmsub213ss", fmt("A", single_ops), enc().w0().op(0xAB).r(), _64b | compat | fma),
        inst("vfmsub231ss", fmt("A", single_ops), enc().w0().op(0xBB).r(), _64b | compat | fma),
        inst("vfmsub132sd", fmt("A", double_ops), enc().w1().op(0x9B).r(), _64b | compat | fma),
        inst("vfmsub213sd", fmt("A", double_ops), enc().w1().op(0xAB).r(), _64b | compat | fma),
        inst("vfmsub231sd", fmt("A", double_ops), enc().w1().op(0xBB).r(), _64b | compat | fma),
        inst("vfmsub132ps", fmt("A", packed_ops), enc().w0().op(0x9A).r(), _64b | compat | fma),
        inst("vfmsub213ps", fmt("A", packed_ops), enc().w0().op(0xAA).r(), _64b | compat | fma),
        inst("vfmsub231ps", fmt("A", packed_ops), enc().w0().op(0xBA).r(), _64b | compat | fma),
        inst("vfmsub132pd", fmt("A", packed_ops), enc().w1().op(0x9A).r(), _64b | compat | fma),
        inst("vfmsub213pd", fmt("A", packed_ops), enc().w1().op(0xAA).r(), _64b | compat | fma),
        inst("vfmsub231pd", fmt("A", packed_ops), enc().w1().op(0xBA).r(), _64b | compat | fma),
        // Fused Negative Multiply-Subtract (FNMS).
        inst("vfnmsub132ss", fmt("A", single_ops), enc().w0().op(0x9F).r(), _64b | compat | fma),
        inst("vfnmsub213ss", fmt("A", single_ops), enc().w0().op(0xAF).r(), _64b | compat | fma),
        inst("vfnmsub231ss", fmt("A", single_ops), enc().w0().op(0xBF).r(), _64b | compat | fma),
        inst("vfnmsub132sd", fmt("A", double_ops), enc().w1().op(0x9F).r(), _64b | compat | fma),
        inst("vfnmsub213sd", fmt("A", double_ops), enc().w1().op(0xAF).r(), _64b | compat | fma),
        inst("vfnmsub231sd", fmt("A", double_ops), enc().w1().op(0xBF).r(), _64b | compat | fma),
        inst("vfnmsub132ps", fmt("A", packed_ops), enc().w0().op(0x9E).r(), _64b | compat | fma),
        inst("vfnmsub213ps", fmt("A", packed_ops), enc().w0().op(0xAE).r(), _64b | compat | fma),
        inst("vfnmsub231ps", fmt("A", packed_ops), enc().w0().op(0xBE).r(), _64b | compat | fma),
        inst("vfnmsub132pd", fmt("A", packed_ops), enc().w1().op(0x9E).r(), _64b | compat | fma),
        inst("vfnmsub213pd", fmt("A", packed_ops), enc().w1().op(0xAE).r(), _64b | compat | fma),
        inst("vfnmsub231pd", fmt("A", packed_ops), enc().w1().op(0xBE).r(), _64b | compat | fma),
    ]
}
