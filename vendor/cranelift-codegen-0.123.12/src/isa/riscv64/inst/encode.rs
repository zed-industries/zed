//! Contains the RISC-V instruction encoding logic.
//!
//! These formats are specified in the RISC-V specification in section 2.2.
//! See: <https://riscv.org/wp-content/uploads/2017/05/riscv-spec-v2.2.pdf>
//!
//! Some instructions especially in extensions have slight variations from
//! the base RISC-V specification.

use super::*;
use crate::isa::riscv64::lower::isle::generated_code::{
    COpcodeSpace, CaOp, CbOp, CiOp, CiwOp, ClOp, CrOp, CsOp, CssOp, CsznOp, FpuOPWidth,
    VecAluOpRImm5, VecAluOpRR, VecAluOpRRRImm5, VecAluOpRRRR, VecOpCategory, ZcbMemOp,
};
use crate::machinst::isle::WritableReg;

fn unsigned_field_width(value: u32, width: u8) -> u32 {
    debug_assert_eq!(value & (!0 << width), 0);
    value
}

/// Layout:
/// 0-------6-7-------11-12------14-15------19-20------24-25-------31
/// | Opcode |   rd     |  funct3  |   rs1    |   rs2    |   funct7  |
fn encode_r_type_bits(opcode: u32, rd: u32, funct3: u32, rs1: u32, rs2: u32, funct7: u32) -> u32 {
    let mut bits = 0;
    bits |= unsigned_field_width(opcode, 7);
    bits |= unsigned_field_width(rd, 5) << 7;
    bits |= unsigned_field_width(funct3, 3) << 12;
    bits |= unsigned_field_width(rs1, 5) << 15;
    bits |= unsigned_field_width(rs2, 5) << 20;
    bits |= unsigned_field_width(funct7, 7) << 25;
    bits
}

/// Encode an R-type instruction.
pub fn encode_r_type(
    opcode: u32,
    rd: WritableReg,
    funct3: u32,
    rs1: Reg,
    rs2: Reg,
    funct7: u32,
) -> u32 {
    encode_r_type_bits(
        opcode,
        reg_to_gpr_num(rd.to_reg()),
        funct3,
        reg_to_gpr_num(rs1),
        reg_to_gpr_num(rs2),
        funct7,
    )
}

/// Layout:
/// 0-------6-7-------11-12------14-15------19-20------------------31
/// | Opcode |   rd     |  width   |   rs1    |     Offset[11:0]    |
fn encode_i_type_bits(opcode: u32, rd: u32, funct3: u32, rs1: u32, offset: u32) -> u32 {
    let mut bits = 0;
    bits |= unsigned_field_width(opcode, 7);
    bits |= unsigned_field_width(rd, 5) << 7;
    bits |= unsigned_field_width(funct3, 3) << 12;
    bits |= unsigned_field_width(rs1, 5) << 15;
    bits |= unsigned_field_width(offset, 12) << 20;
    bits
}

/// Encode an I-type instruction.
pub fn encode_i_type(opcode: u32, rd: WritableReg, width: u32, rs1: Reg, offset: Imm12) -> u32 {
    encode_i_type_bits(
        opcode,
        reg_to_gpr_num(rd.to_reg()),
        width,
        reg_to_gpr_num(rs1),
        offset.bits(),
    )
}

/// Encode an S-type instruction.
///
/// Layout:
/// 0-------6-7-------11-12------14-15------19-20---24-25-------------31
/// | Opcode | imm[4:0] |  width   |   base   |  src  |    imm[11:5]   |
pub fn encode_s_type(opcode: u32, width: u32, base: Reg, src: Reg, offset: Imm12) -> u32 {
    let mut bits = 0;
    bits |= unsigned_field_width(opcode, 7);
    bits |= (offset.bits() & 0b11111) << 7;
    bits |= unsigned_field_width(width, 3) << 12;
    bits |= reg_to_gpr_num(base) << 15;
    bits |= reg_to_gpr_num(src) << 20;
    bits |= unsigned_field_width(offset.bits() >> 5, 7) << 25;
    bits
}

/// Encodes a Vector ALU instruction.
///
/// Fields:
/// - opcode (7 bits)
/// - vd     (5 bits)
/// - funct3 (3 bits)
/// - vs1    (5 bits)
/// - vs2    (5 bits)
/// - vm     (1 bit)
/// - funct6 (6 bits)
///
/// See: https://github.com/riscv/riscv-v-spec/blob/master/valu-format.adoc
pub fn encode_valu(
    op: VecAluOpRRR,
    vd: WritableReg,
    vs1: Reg,
    vs2: Reg,
    masking: VecOpMasking,
) -> u32 {
    let funct7 = (op.funct6() << 1) | masking.encode();
    encode_r_type_bits(
        op.opcode(),
        reg_to_gpr_num(vd.to_reg()),
        op.funct3(),
        reg_to_gpr_num(vs1),
        reg_to_gpr_num(vs2),
        funct7,
    )
}

/// Encodes a Vector ALU+Imm instruction.
/// This is just a Vector ALU instruction with an immediate in the VS1 field.
///
/// Fields:
/// - opcode (7 bits)
/// - vd     (5 bits)
/// - funct3 (3 bits)
/// - imm    (5 bits)
/// - vs2    (5 bits)
/// - vm     (1 bit)
/// - funct6 (6 bits)
///
/// See: https://github.com/riscv/riscv-v-spec/blob/master/valu-format.adoc
pub fn encode_valu_rr_imm(
    op: VecAluOpRRImm5,
    vd: WritableReg,
    imm: Imm5,
    vs2: Reg,
    masking: VecOpMasking,
) -> u32 {
    let funct7 = (op.funct6() << 1) | masking.encode();
    let imm = imm.bits() as u32;
    encode_r_type_bits(
        op.opcode(),
        reg_to_gpr_num(vd.to_reg()),
        op.funct3(),
        imm,
        reg_to_gpr_num(vs2),
        funct7,
    )
}

pub fn encode_valu_rrrr(
    op: VecAluOpRRRR,
    vd: WritableReg,
    vs2: Reg,
    vs1: Reg,
    masking: VecOpMasking,
) -> u32 {
    let funct7 = (op.funct6() << 1) | masking.encode();
    encode_r_type_bits(
        op.opcode(),
        reg_to_gpr_num(vd.to_reg()),
        op.funct3(),
        reg_to_gpr_num(vs1),
        reg_to_gpr_num(vs2),
        funct7,
    )
}

pub fn encode_valu_rrr_imm(
    op: VecAluOpRRRImm5,
    vd: WritableReg,
    imm: Imm5,
    vs2: Reg,
    masking: VecOpMasking,
) -> u32 {
    let funct7 = (op.funct6() << 1) | masking.encode();
    let imm = imm.bits() as u32;
    encode_r_type_bits(
        op.opcode(),
        reg_to_gpr_num(vd.to_reg()),
        op.funct3(),
        imm,
        reg_to_gpr_num(vs2),
        funct7,
    )
}

pub fn encode_valu_rr(op: VecAluOpRR, vd: WritableReg, vs: Reg, masking: VecOpMasking) -> u32 {
    let funct7 = (op.funct6() << 1) | masking.encode();

    let (vs1, vs2) = if op.vs_is_vs2_encoded() {
        (op.aux_encoding(), reg_to_gpr_num(vs))
    } else {
        (reg_to_gpr_num(vs), op.aux_encoding())
    };

    encode_r_type_bits(
        op.opcode(),
        reg_to_gpr_num(vd.to_reg()),
        op.funct3(),
        vs1,
        vs2,
        funct7,
    )
}

pub fn encode_valu_r_imm(
    op: VecAluOpRImm5,
    vd: WritableReg,
    imm: Imm5,
    masking: VecOpMasking,
) -> u32 {
    let funct7 = (op.funct6() << 1) | masking.encode();

    // This is true for this opcode, not sure if there are any other ones.
    debug_assert_eq!(op, VecAluOpRImm5::VmvVI);
    let vs1 = imm.bits() as u32;
    let vs2 = op.aux_encoding();

    encode_r_type_bits(
        op.opcode(),
        reg_to_gpr_num(vd.to_reg()),
        op.funct3(),
        vs1,
        vs2,
        funct7,
    )
}

/// Encodes a Vector CFG Imm instruction.
///
/// See: https://github.com/riscv/riscv-v-spec/blob/master/vcfg-format.adoc
// TODO: Check if this is any of the known instruction types in the spec.
pub fn encode_vcfg_imm(opcode: u32, rd: Reg, imm: UImm5, vtype: &VType) -> u32 {
    let mut bits = 0;
    bits |= unsigned_field_width(opcode, 7);
    bits |= reg_to_gpr_num(rd) << 7;
    bits |= VecOpCategory::OPCFG.encode() << 12;
    bits |= unsigned_field_width(imm.bits(), 5) << 15;
    bits |= unsigned_field_width(vtype.encode(), 10) << 20;
    bits |= 0b11 << 30;
    bits
}

/// Encodes a Vector Mem Unit Stride Load instruction.
///
/// See: https://github.com/riscv/riscv-v-spec/blob/master/vmem-format.adoc
/// TODO: These instructions share opcode space with LOAD-FP and STORE-FP
pub fn encode_vmem_load(
    opcode: u32,
    vd: Reg,
    width: VecElementWidth,
    rs1: Reg,
    lumop: u32,
    masking: VecOpMasking,
    mop: u32,
    nf: u32,
) -> u32 {
    // Width is encoded differently to avoid a clash with the FP load/store sizes.
    let width = match width {
        VecElementWidth::E8 => 0b000,
        VecElementWidth::E16 => 0b101,
        VecElementWidth::E32 => 0b110,
        VecElementWidth::E64 => 0b111,
    };

    let mut bits = 0;
    bits |= unsigned_field_width(opcode, 7);
    bits |= reg_to_gpr_num(vd) << 7;
    bits |= width << 12;
    bits |= reg_to_gpr_num(rs1) << 15;
    bits |= unsigned_field_width(lumop, 5) << 20;
    bits |= masking.encode() << 25;
    bits |= unsigned_field_width(mop, 2) << 26;

    // The mew bit (inst[28]) when set is expected to be used to encode expanded
    // memory sizes of 128 bits and above, but these encodings are currently reserved.
    bits |= 0b0 << 28;

    bits |= unsigned_field_width(nf, 3) << 29;
    bits
}

/// Encodes a Vector Mem Unit Stride Load instruction.
///
/// See: https://github.com/riscv/riscv-v-spec/blob/master/vmem-format.adoc
/// TODO: These instructions share opcode space with LOAD-FP and STORE-FP
pub fn encode_vmem_store(
    opcode: u32,
    vs3: Reg,
    width: VecElementWidth,
    rs1: Reg,
    sumop: u32,
    masking: VecOpMasking,
    mop: u32,
    nf: u32,
) -> u32 {
    // This is pretty much the same as the load instruction, just
    // with different names on the fields.
    encode_vmem_load(opcode, vs3, width, rs1, sumop, masking, mop, nf)
}

// The CSR Reg instruction is really just an I type instruction with the CSR in
// the immediate field.
pub fn encode_csr_reg(op: CsrRegOP, rd: WritableReg, rs: Reg, csr: CSR) -> u32 {
    encode_i_type(op.opcode(), rd, op.funct3(), rs, csr.bits())
}

// The CSR Imm instruction is an I type instruction with the CSR in
// the immediate field and the value to be set in the `rs1` field.
pub fn encode_csr_imm(op: CsrImmOP, rd: WritableReg, csr: CSR, imm: UImm5) -> u32 {
    encode_i_type_bits(
        op.opcode(),
        reg_to_gpr_num(rd.to_reg()),
        op.funct3(),
        imm.bits(),
        csr.bits().bits(),
    )
}

// Encode a CR type instruction.
//
// 0--1-2-----6-7-------11-12-------15
// |op |  rs2  |  rd/rs1  |  funct4  |
pub fn encode_cr_type(op: CrOp, rd: WritableReg, rs2: Reg) -> u16 {
    let mut bits = 0;
    bits |= unsigned_field_width(op.op().bits(), 2);
    bits |= reg_to_gpr_num(rs2) << 2;
    bits |= reg_to_gpr_num(rd.to_reg()) << 7;
    bits |= unsigned_field_width(op.funct4(), 4) << 12;
    bits.try_into().unwrap()
}

// This isn't technically a instruction format that exists. It's just a CR type
// where the source is rs1, rs2 is zero. rs1 is never written to.
//
// Used for C.JR and C.JALR
pub fn encode_cr2_type(op: CrOp, rs1: Reg) -> u16 {
    encode_cr_type(op, WritableReg::from_reg(rs1), zero_reg())
}

// Encode a CA type instruction.
//
// 0--1-2-----4-5--------6-7--------9-10------15
// |op |  rs2  |  funct2  |  rd/rs1  | funct6 |
pub fn encode_ca_type(op: CaOp, rd: WritableReg, rs2: Reg) -> u16 {
    let mut bits = 0;
    bits |= unsigned_field_width(op.op().bits(), 2);
    bits |= reg_to_compressed_gpr_num(rs2) << 2;
    bits |= unsigned_field_width(op.funct2(), 2) << 5;
    bits |= reg_to_compressed_gpr_num(rd.to_reg()) << 7;
    bits |= unsigned_field_width(op.funct6(), 6) << 10;
    bits.try_into().unwrap()
}

// Encode a CJ type instruction.
//
// The imm field is a 11 bit signed immediate that is shifted left by 1.
//
// 0--1-2-----12-13--------15
// |op |  imm   |  funct3  |
pub fn encode_cj_type(op: CjOp, imm: Imm12) -> u16 {
    let imm = imm.bits();
    debug_assert!(imm & 1 == 0);

    // The offset bits are in rather weird positions.
    // [11|4|9:8|10|6|7|3:1|5]
    let mut imm_field = 0;
    imm_field |= ((imm >> 11) & 1) << 10;
    imm_field |= ((imm >> 4) & 1) << 9;
    imm_field |= ((imm >> 8) & 3) << 7;
    imm_field |= ((imm >> 10) & 1) << 6;
    imm_field |= ((imm >> 6) & 1) << 5;
    imm_field |= ((imm >> 7) & 1) << 4;
    imm_field |= ((imm >> 1) & 7) << 1;
    imm_field |= ((imm >> 5) & 1) << 0;

    let mut bits = 0;
    bits |= unsigned_field_width(op.op().bits(), 2);
    bits |= unsigned_field_width(imm_field, 11) << 2;
    bits |= unsigned_field_width(op.funct3(), 3) << 13;
    bits.try_into().unwrap()
}

// Encode a CI type instruction.
//
// The imm field is a 6 bit signed immediate.
//
// 0--1-2-------6-7-------11-12-----12-13-----15
// |op | imm[4:0] |   src   | imm[5]  | funct3  |
pub fn encode_ci_type(op: CiOp, rd: WritableReg, imm: Imm6) -> u16 {
    let imm = imm.bits();

    let mut bits = 0;
    bits |= unsigned_field_width(op.op().bits(), 2);
    bits |= unsigned_field_width((imm & 0x1f) as u32, 5) << 2;
    bits |= reg_to_gpr_num(rd.to_reg()) << 7;
    bits |= unsigned_field_width(((imm >> 5) & 1) as u32, 1) << 12;
    bits |= unsigned_field_width(op.funct3(), 3) << 13;
    bits.try_into().unwrap()
}

// Stack-Pointer relative loads are regular CI instructions, but, the immediate
// is zero extended, and with a slightly different immediate field encoding.
pub fn encode_ci_sp_load(op: CiOp, rd: WritableReg, imm: Uimm6) -> u16 {
    let imm = imm.bits();

    // These are the spec encoded offsets.
    // LWSP:  [5|4:2|7:6]
    // LDSP:  [5|4:3|8:6]
    // FLDSP: [5|4:3|8:6]
    //
    // We don't receive the entire offset in `imm`, just a multiple of the load-size.

    // Number of bits in the lowest position of imm. 3 for lwsp, 2 for {f,}ldsp.
    let low_bits = match op {
        CiOp::CLwsp => 3,                // [4:2]
        CiOp::CLdsp | CiOp::CFldsp => 2, // [4:3]
        _ => unreachable!(),
    };
    let high_bits = 6 - 1 - low_bits;
    let mut enc_imm = 0;

    // Encode [7:6] at the bottom of imm
    enc_imm |= imm >> (6 - high_bits);

    // Next place [4:2] in the middle
    enc_imm |= (imm & ((1 << low_bits) - 1)) << high_bits;

    // Finally place [5] at the top
    enc_imm |= ((imm >> low_bits) & 1) << 5;

    let enc_imm = Imm6::maybe_from_i16((enc_imm as i16) << 10 >> 10).unwrap();

    encode_ci_type(op, rd, enc_imm)
}

/// c.addi16sp is a regular CI op, but the immediate field is encoded in a weird way
pub fn encode_c_addi16sp(imm: Imm6) -> u16 {
    let imm = imm.bits();

    // [6|1|3|5:4|2]
    let mut enc_imm = 0;
    enc_imm |= ((imm >> 5) & 1) << 5;
    enc_imm |= ((imm >> 0) & 1) << 4;
    enc_imm |= ((imm >> 2) & 1) << 3;
    enc_imm |= ((imm >> 3) & 3) << 1;
    enc_imm |= ((imm >> 1) & 1) << 0;
    let enc_imm = Imm6::maybe_from_i16((enc_imm as i16) << 10 >> 10).unwrap();

    encode_ci_type(CiOp::CAddi16sp, writable_stack_reg(), enc_imm)
}

// Encode a CIW type instruction.
//
// 0--1-2------4-5------12-13--------15
// |op |   rd   |   imm   |  funct3  |
pub fn encode_ciw_type(op: CiwOp, rd: WritableReg, imm: u8) -> u16 {
    // [3:2|7:4|0|1]
    let mut imm_field = 0;
    imm_field |= ((imm >> 1) & 1) << 0;
    imm_field |= ((imm >> 0) & 1) << 1;
    imm_field |= ((imm >> 4) & 15) << 2;
    imm_field |= ((imm >> 2) & 3) << 6;

    let mut bits = 0;
    bits |= unsigned_field_width(op.op().bits(), 2);
    bits |= reg_to_compressed_gpr_num(rd.to_reg()) << 2;
    bits |= unsigned_field_width(imm_field as u32, 8) << 5;
    bits |= unsigned_field_width(op.funct3(), 3) << 13;
    bits.try_into().unwrap()
}

// Encode a CB type instruction.
//
// The imm field is a 6 bit signed immediate.
//
// 0--1-2-------6-7-------9-10-------11-12-------13--------15
// |op | imm[4:0] |   dst  |  funct2   |  imm[5]  | funct3 |
pub fn encode_cb_type(op: CbOp, rd: WritableReg, imm: Imm6) -> u16 {
    let imm = imm.bits();

    let mut bits = 0;
    bits |= unsigned_field_width(op.op().bits(), 2);
    bits |= unsigned_field_width((imm & 0x1f) as u32, 5) << 2;
    bits |= reg_to_compressed_gpr_num(rd.to_reg()) << 7;
    bits |= unsigned_field_width(op.funct2(), 2) << 10;
    bits |= unsigned_field_width(((imm >> 5) & 1) as u32, 1) << 12;
    bits |= unsigned_field_width(op.funct3(), 3) << 13;
    bits.try_into().unwrap()
}

// Encode a CSS type instruction.
//
// The imm field is a 6 bit unsigned immediate.
//
// 0--1-2-------6-7--------12-13-------15
// |op |   src   |    imm    |  funct3  |
pub fn encode_css_type(op: CssOp, src: Reg, imm: Uimm6) -> u16 {
    let imm = imm.bits();

    // These are the spec encoded offsets.
    // c.swsp:  [5:2|7:6]
    // c.sdsp:  [5:3|8:6]
    // c.fsdsp: [5:3|8:6]
    //
    // We don't receive the entire offset in `imm`, just a multiple of the load-size.

    // Number of bits in the lowest position of imm. 4 for c.swsp, 3 for c.{f,}sdsp.
    let low_bits = match op {
        CssOp::CSwsp => 4,                 // [5:2]
        CssOp::CSdsp | CssOp::CFsdsp => 3, // [5:3]
    };
    let high_bits = 6 - low_bits;

    let mut enc_imm = 0;
    enc_imm |= (imm & ((1 << low_bits) - 1)) << high_bits;
    enc_imm |= imm >> low_bits;

    let mut bits = 0;
    bits |= unsigned_field_width(op.op().bits(), 2);
    bits |= reg_to_gpr_num(src) << 2;
    bits |= unsigned_field_width(enc_imm as u32, 6) << 7;
    bits |= unsigned_field_width(op.funct3(), 3) << 13;
    bits.try_into().unwrap()
}

// Encode a CS type instruction.
//
// The imm field is a 5 bit unsigned immediate.
//
// 0--1-2-----4-5----------6-7---------9-10----------12-13-----15
// |op |  src  | imm(2-bit) |   base    |  imm(3-bit)  | funct3  |
pub fn encode_cs_type(op: CsOp, src: Reg, base: Reg, imm: Uimm5) -> u16 {
    let size = match op {
        CsOp::CFsd | CsOp::CSd => 8,
        CsOp::CSw => 4,
    };

    encode_cs_cl_type_bits(op.op(), op.funct3(), size, src, base, imm)
}

// Encode a CL type instruction.
//
// The imm field is a 5 bit unsigned immediate.
//
// 0--1-2------4-5----------6-7---------9-10----------12-13-----15
// |op |  dest  | imm(2-bit) |   base    |  imm(3-bit)  | funct3  |
pub fn encode_cl_type(op: ClOp, dest: WritableReg, base: Reg, imm: Uimm5) -> u16 {
    let size = match op {
        ClOp::CFld | ClOp::CLd => 8,
        ClOp::CLw => 4,
    };

    encode_cs_cl_type_bits(op.op(), op.funct3(), size, dest.to_reg(), base, imm)
}

// CL and CS type instructions have the same physical layout.
//
// 0--1-2----------4-5----------6-7---------9-10----------12-13-----15
// |op |  dest/src  | imm(2-bit) |   base    |  imm(3-bit)  | funct3  |
fn encode_cs_cl_type_bits(
    op: COpcodeSpace,
    funct3: u32,
    size: u32,
    dest_src: Reg,
    base: Reg,
    imm: Uimm5,
) -> u16 {
    let imm = imm.bits();

    // c.sw  / c.lw:  [2|6]
    // c.sd  / c.ld:  [7:6]
    // c.fsd / c.fld: [7:6]
    //
    // We differentiate these based on the operation size
    let imm2 = match size {
        4 => ((imm >> 4) & 1) | ((imm & 1) << 1),
        8 => (imm >> 3) & 0b11,
        _ => unreachable!(),
    };

    // [5:3] on all opcodes
    let imm3 = match size {
        4 => (imm >> 1) & 0b111,
        8 => (imm >> 0) & 0b111,
        _ => unreachable!(),
    };

    let mut bits = 0;
    bits |= unsigned_field_width(op.bits(), 2);
    bits |= reg_to_compressed_gpr_num(dest_src) << 2;
    bits |= unsigned_field_width(imm2 as u32, 2) << 5;
    bits |= reg_to_compressed_gpr_num(base) << 7;
    bits |= unsigned_field_width(imm3 as u32, 3) << 10;
    bits |= unsigned_field_width(funct3, 3) << 13;
    bits.try_into().unwrap()
}

// Encode a CSZN type instruction.
//
// This is an additional encoding format that is introduced in the Zcb extension.
//
// 0--1-2---------6-7--------9-10------15
// |op |   funct5  |  rd/rs1  | funct6 |
pub fn encode_cszn_type(op: CsznOp, rd: WritableReg) -> u16 {
    let mut bits = 0;
    bits |= unsigned_field_width(op.op().bits(), 2);
    bits |= unsigned_field_width(op.funct5(), 5) << 2;
    bits |= reg_to_compressed_gpr_num(rd.to_reg()) << 7;
    bits |= unsigned_field_width(op.funct6(), 6) << 10;
    bits.try_into().unwrap()
}

// Encodes the various memory operations in the Zcb extension.
//
// 0--1-2----------4-5----------6-7---------9-10-------15
// |op |  dest/src  | imm(2-bit) |   base    |  funct6  |
fn encode_zcbmem_bits(op: ZcbMemOp, dest_src: Reg, base: Reg, imm: Uimm2) -> u16 {
    let imm = imm.bits();

    // For these ops, bit 6 is part of the opcode, and bit 5 encodes the imm offset.
    let imm = match op {
        ZcbMemOp::CLh | ZcbMemOp::CLhu | ZcbMemOp::CSh => {
            debug_assert_eq!(imm & !1, 0);
            // Only c.lh has this bit as 1
            let opcode_bit = (op == ZcbMemOp::CLh) as u8;
            imm | (opcode_bit << 1)
        }
        // In the rest of the ops the imm is reversed.
        _ => ((imm & 1) << 1) | ((imm >> 1) & 1),
    };

    let mut bits = 0;
    bits |= unsigned_field_width(op.op().bits(), 2);
    bits |= reg_to_compressed_gpr_num(dest_src) << 2;
    bits |= unsigned_field_width(imm as u32, 2) << 5;
    bits |= reg_to_compressed_gpr_num(base) << 7;
    bits |= unsigned_field_width(op.funct6(), 6) << 10;
    bits.try_into().unwrap()
}

pub fn encode_zcbmem_load(op: ZcbMemOp, rd: WritableReg, base: Reg, imm: Uimm2) -> u16 {
    encode_zcbmem_bits(op, rd.to_reg(), base, imm)
}

pub fn encode_zcbmem_store(op: ZcbMemOp, src: Reg, base: Reg, imm: Uimm2) -> u16 {
    encode_zcbmem_bits(op, src, base, imm)
}

pub fn encode_fli(width: FpuOPWidth, imm: FliConstant, rd: WritableReg) -> u32 {
    // FLI.{H,S,D} is encoded as a FMV.{H,W,D} instruction with rs2 set to the
    // immediate value to be loaded.
    let op = FpuOPRR::FmvFmtX;
    let frm = 0; // FRM is hard coded to 0 in both instructions
    let rs2 = 1; // rs2 set to 1 is what differentiates FLI from FMV

    let mut bits = 0;
    bits |= unsigned_field_width(op.opcode(), 7);
    bits |= reg_to_gpr_num(rd.to_reg()) << 7;
    bits |= unsigned_field_width(frm, 3) << 12;
    bits |= unsigned_field_width(imm.bits() as u32, 5) << 15;
    bits |= unsigned_field_width(rs2, 6) << 20;
    bits |= unsigned_field_width(op.funct7(width), 7) << 25;
    bits
}

pub fn encode_fp_rr(op: FpuOPRR, width: FpuOPWidth, frm: FRM, rd: WritableReg, rs: Reg) -> u32 {
    encode_r_type_bits(
        op.opcode(),
        reg_to_gpr_num(rd.to_reg()),
        frm.as_u32(),
        reg_to_gpr_num(rs),
        op.rs2(),
        op.funct7(width),
    )
}

pub fn encode_fp_rrr(
    op: FpuOPRRR,
    width: FpuOPWidth,
    frm: FRM,
    rd: WritableReg,
    rs1: Reg,
    rs2: Reg,
) -> u32 {
    encode_r_type_bits(
        op.opcode(),
        reg_to_gpr_num(rd.to_reg()),
        frm.as_u32(),
        reg_to_gpr_num(rs1),
        reg_to_gpr_num(rs2),
        op.funct7(width),
    )
}

pub fn encode_fp_rrrr(
    op: FpuOPRRRR,
    width: FpuOPWidth,
    frm: FRM,
    rd: WritableReg,
    rs1: Reg,
    rs2: Reg,
    rs3: Reg,
) -> u32 {
    let funct7 = (reg_to_gpr_num(rs3) << 2) | width.as_u32();
    encode_r_type_bits(
        op.opcode(),
        reg_to_gpr_num(rd.to_reg()),
        frm.as_u32(),
        reg_to_gpr_num(rs1),
        reg_to_gpr_num(rs2),
        funct7,
    )
}
