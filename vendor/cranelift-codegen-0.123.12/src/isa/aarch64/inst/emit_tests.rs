use crate::ir::types::*;
use crate::ir::{ExternalName, TrapCode};
use crate::isa::aarch64::inst::*;

use alloc::boxed::Box;

#[cfg(test)]
fn simm9_zero() -> SImm9 {
    SImm9::maybe_from_i64(0).unwrap()
}

#[cfg(test)]
fn simm7_scaled_zero(scale_ty: Type) -> SImm7Scaled {
    SImm7Scaled::maybe_from_i64(0, scale_ty).unwrap()
}

#[test]
fn test_aarch64_binemit() {
    let mut insns = Vec::<(Inst, &str, &str)>::new();

    // N.B.: the architecture is little-endian, so when transcribing the 32-bit
    // hex instructions from e.g. objdump disassembly, one must swap the bytes
    // seen below. (E.g., a `ret` is normally written as the u32 `D65F03C0`,
    // but we write it here as C0035FD6.)

    // Useful helper script to produce the encodings from the text:
    //
    //      #!/bin/sh
    //      tmp=`mktemp /tmp/XXXXXXXX.o`
    //      aarch64-linux-gnu-as /dev/stdin -o $tmp
    //      aarch64-linux-gnu-objdump -d $tmp
    //      rm -f $tmp
    //
    // Then:
    //
    //      $ echo "mov x1, x2" | aarch64inst.sh
    insns.push((Inst::Ret {}, "C0035FD6", "ret"));
    insns.push((
        Inst::AuthenticatedRet {
            key: APIKey::ASP,
            is_hint: true,
        },
        "BF2303D5C0035FD6",
        "autiasp ; ret",
    ));
    insns.push((
        Inst::AuthenticatedRet {
            key: APIKey::BSP,
            is_hint: false,
        },
        "FF0F5FD6",
        "retabsp",
    ));
    insns.push((Inst::Paci { key: APIKey::BSP }, "7F2303D5", "pacibsp"));
    insns.push((Inst::Xpaclri, "FF2003D5", "xpaclri"));
    insns.push((
        Inst::Bti {
            targets: BranchTargetType::J,
        },
        "9F2403D5",
        "bti j",
    ));
    insns.push((Inst::Nop0, "", "nop-zero-len"));
    insns.push((Inst::Nop4, "1F2003D5", "nop"));
    insns.push((Inst::Csdb, "9F2203D5", "csdb"));
    insns.push((
        Inst::Udf {
            trap_code: TrapCode::STACK_OVERFLOW,
        },
        "1FC10000",
        "udf #0xc11f",
    ));
    insns.push((
        Inst::AluRRR {
            alu_op: ALUOp::Add,
            size: OperandSize::Size32,
            rd: writable_xreg(1),
            rn: xreg(2),
            rm: xreg(3),
        },
        "4100030B",
        "add w1, w2, w3",
    ));
    insns.push((
        Inst::AluRRR {
            alu_op: ALUOp::Add,
            size: OperandSize::Size64,
            rd: writable_xreg(4),
            rn: xreg(5),
            rm: xreg(6),
        },
        "A400068B",
        "add x4, x5, x6",
    ));
    insns.push((
        Inst::AluRRR {
            alu_op: ALUOp::Adc,
            size: OperandSize::Size32,
            rd: writable_xreg(1),
            rn: xreg(2),
            rm: xreg(3),
        },
        "4100031A",
        "adc w1, w2, w3",
    ));
    insns.push((
        Inst::AluRRR {
            alu_op: ALUOp::Adc,
            size: OperandSize::Size64,
            rd: writable_xreg(4),
            rn: xreg(5),
            rm: xreg(6),
        },
        "A400069A",
        "adc x4, x5, x6",
    ));
    insns.push((
        Inst::AluRRR {
            alu_op: ALUOp::AdcS,
            size: OperandSize::Size32,
            rd: writable_xreg(1),
            rn: xreg(2),
            rm: xreg(3),
        },
        "4100033A",
        "adcs w1, w2, w3",
    ));
    insns.push((
        Inst::AluRRR {
            alu_op: ALUOp::AdcS,
            size: OperandSize::Size64,
            rd: writable_xreg(4),
            rn: xreg(5),
            rm: xreg(6),
        },
        "A40006BA",
        "adcs x4, x5, x6",
    ));
    insns.push((
        Inst::AluRRR {
            alu_op: ALUOp::Sub,
            size: OperandSize::Size32,
            rd: writable_xreg(1),
            rn: xreg(2),
            rm: xreg(3),
        },
        "4100034B",
        "sub w1, w2, w3",
    ));
    insns.push((
        Inst::AluRRR {
            alu_op: ALUOp::Sub,
            size: OperandSize::Size64,
            rd: writable_xreg(4),
            rn: xreg(5),
            rm: xreg(6),
        },
        "A40006CB",
        "sub x4, x5, x6",
    ));
    insns.push((
        Inst::AluRRR {
            alu_op: ALUOp::Sbc,
            size: OperandSize::Size32,
            rd: writable_xreg(1),
            rn: xreg(2),
            rm: xreg(3),
        },
        "4100035A",
        "sbc w1, w2, w3",
    ));
    insns.push((
        Inst::AluRRR {
            alu_op: ALUOp::Sbc,
            size: OperandSize::Size64,
            rd: writable_xreg(4),
            rn: xreg(5),
            rm: xreg(6),
        },
        "A40006DA",
        "sbc x4, x5, x6",
    ));
    insns.push((
        Inst::AluRRR {
            alu_op: ALUOp::SbcS,
            size: OperandSize::Size32,
            rd: writable_xreg(1),
            rn: xreg(2),
            rm: xreg(3),
        },
        "4100037A",
        "sbcs w1, w2, w3",
    ));
    insns.push((
        Inst::AluRRR {
            alu_op: ALUOp::SbcS,
            size: OperandSize::Size64,
            rd: writable_xreg(4),
            rn: xreg(5),
            rm: xreg(6),
        },
        "A40006FA",
        "sbcs x4, x5, x6",
    ));

    insns.push((
        Inst::AluRRR {
            alu_op: ALUOp::Orr,
            size: OperandSize::Size32,
            rd: writable_xreg(1),
            rn: xreg(2),
            rm: xreg(3),
        },
        "4100032A",
        "orr w1, w2, w3",
    ));
    insns.push((
        Inst::AluRRR {
            alu_op: ALUOp::Orr,
            size: OperandSize::Size64,
            rd: writable_xreg(4),
            rn: xreg(5),
            rm: xreg(6),
        },
        "A40006AA",
        "orr x4, x5, x6",
    ));
    insns.push((
        Inst::AluRRR {
            alu_op: ALUOp::And,
            size: OperandSize::Size32,
            rd: writable_xreg(1),
            rn: xreg(2),
            rm: xreg(3),
        },
        "4100030A",
        "and w1, w2, w3",
    ));
    insns.push((
        Inst::AluRRR {
            alu_op: ALUOp::And,
            size: OperandSize::Size64,
            rd: writable_xreg(4),
            rn: xreg(5),
            rm: xreg(6),
        },
        "A400068A",
        "and x4, x5, x6",
    ));
    insns.push((
        Inst::AluRRR {
            alu_op: ALUOp::AndS,
            size: OperandSize::Size32,
            rd: writable_xreg(1),
            rn: xreg(2),
            rm: xreg(3),
        },
        "4100036A",
        "ands w1, w2, w3",
    ));
    insns.push((
        Inst::AluRRR {
            alu_op: ALUOp::AndS,
            size: OperandSize::Size64,
            rd: writable_xreg(4),
            rn: xreg(5),
            rm: xreg(6),
        },
        "A40006EA",
        "ands x4, x5, x6",
    ));
    insns.push((
        Inst::AluRRR {
            alu_op: ALUOp::SubS,
            size: OperandSize::Size32,
            rd: writable_zero_reg(),
            rn: xreg(2),
            rm: xreg(3),
        },
        "5F00036B",
        // TODO: Display as cmp
        "subs wzr, w2, w3",
    ));
    insns.push((
        Inst::AluRRR {
            alu_op: ALUOp::SubS,
            size: OperandSize::Size32,
            rd: writable_xreg(1),
            rn: xreg(2),
            rm: xreg(3),
        },
        "4100036B",
        "subs w1, w2, w3",
    ));
    insns.push((
        Inst::AluRRR {
            alu_op: ALUOp::SubS,
            size: OperandSize::Size64,
            rd: writable_xreg(4),
            rn: xreg(5),
            rm: xreg(6),
        },
        "A40006EB",
        "subs x4, x5, x6",
    ));
    insns.push((
        Inst::AluRRR {
            alu_op: ALUOp::AddS,
            size: OperandSize::Size32,
            rd: writable_xreg(1),
            rn: xreg(2),
            rm: xreg(3),
        },
        "4100032B",
        "adds w1, w2, w3",
    ));
    insns.push((
        Inst::AluRRR {
            alu_op: ALUOp::AddS,
            size: OperandSize::Size64,
            rd: writable_xreg(4),
            rn: xreg(5),
            rm: xreg(6),
        },
        "A40006AB",
        "adds x4, x5, x6",
    ));
    insns.push((
        Inst::AluRRImm12 {
            alu_op: ALUOp::AddS,
            size: OperandSize::Size64,
            rd: writable_zero_reg(),
            rn: xreg(5),
            imm12: Imm12::maybe_from_u64(1).unwrap(),
        },
        "BF0400B1",
        // TODO: Display as cmn.
        "adds xzr, x5, #1",
    ));
    insns.push((
        Inst::AluRRR {
            alu_op: ALUOp::SDiv,
            size: OperandSize::Size64,
            rd: writable_xreg(4),
            rn: xreg(5),
            rm: xreg(6),
        },
        "A40CC69A",
        "sdiv x4, x5, x6",
    ));
    insns.push((
        Inst::AluRRR {
            alu_op: ALUOp::UDiv,
            size: OperandSize::Size64,
            rd: writable_xreg(4),
            rn: xreg(5),
            rm: xreg(6),
        },
        "A408C69A",
        "udiv x4, x5, x6",
    ));

    insns.push((
        Inst::AluRRR {
            alu_op: ALUOp::Eor,
            size: OperandSize::Size32,
            rd: writable_xreg(4),
            rn: xreg(5),
            rm: xreg(6),
        },
        "A400064A",
        "eor w4, w5, w6",
    ));
    insns.push((
        Inst::AluRRR {
            alu_op: ALUOp::Eor,
            size: OperandSize::Size64,
            rd: writable_xreg(4),
            rn: xreg(5),
            rm: xreg(6),
        },
        "A40006CA",
        "eor x4, x5, x6",
    ));
    insns.push((
        Inst::AluRRR {
            alu_op: ALUOp::AndNot,
            size: OperandSize::Size32,
            rd: writable_xreg(4),
            rn: xreg(5),
            rm: xreg(6),
        },
        "A400260A",
        "bic w4, w5, w6",
    ));
    insns.push((
        Inst::AluRRR {
            alu_op: ALUOp::AndNot,
            size: OperandSize::Size64,
            rd: writable_xreg(4),
            rn: xreg(5),
            rm: xreg(6),
        },
        "A400268A",
        "bic x4, x5, x6",
    ));
    insns.push((
        Inst::AluRRR {
            alu_op: ALUOp::OrrNot,
            size: OperandSize::Size32,
            rd: writable_xreg(4),
            rn: xreg(5),
            rm: xreg(6),
        },
        "A400262A",
        "orn w4, w5, w6",
    ));
    insns.push((
        Inst::AluRRR {
            alu_op: ALUOp::OrrNot,
            size: OperandSize::Size64,
            rd: writable_xreg(4),
            rn: xreg(5),
            rm: xreg(6),
        },
        "A40026AA",
        "orn x4, x5, x6",
    ));
    insns.push((
        Inst::AluRRR {
            alu_op: ALUOp::EorNot,
            size: OperandSize::Size32,
            rd: writable_xreg(4),
            rn: xreg(5),
            rm: xreg(6),
        },
        "A400264A",
        "eon w4, w5, w6",
    ));
    insns.push((
        Inst::AluRRR {
            alu_op: ALUOp::EorNot,
            size: OperandSize::Size64,
            rd: writable_xreg(4),
            rn: xreg(5),
            rm: xreg(6),
        },
        "A40026CA",
        "eon x4, x5, x6",
    ));

    insns.push((
        Inst::AluRRR {
            alu_op: ALUOp::Extr,
            size: OperandSize::Size32,
            rd: writable_xreg(4),
            rn: xreg(5),
            rm: xreg(6),
        },
        "A42CC61A",
        "extr w4, w5, w6",
    ));
    insns.push((
        Inst::AluRRR {
            alu_op: ALUOp::Extr,
            size: OperandSize::Size64,
            rd: writable_xreg(4),
            rn: xreg(5),
            rm: xreg(6),
        },
        "A42CC69A",
        "extr x4, x5, x6",
    ));
    insns.push((
        Inst::AluRRR {
            alu_op: ALUOp::Lsr,
            size: OperandSize::Size32,
            rd: writable_xreg(4),
            rn: xreg(5),
            rm: xreg(6),
        },
        "A424C61A",
        "lsr w4, w5, w6",
    ));
    insns.push((
        Inst::AluRRR {
            alu_op: ALUOp::Lsr,
            size: OperandSize::Size64,
            rd: writable_xreg(4),
            rn: xreg(5),
            rm: xreg(6),
        },
        "A424C69A",
        "lsr x4, x5, x6",
    ));
    insns.push((
        Inst::AluRRR {
            alu_op: ALUOp::Asr,
            size: OperandSize::Size32,
            rd: writable_xreg(4),
            rn: xreg(5),
            rm: xreg(6),
        },
        "A428C61A",
        "asr w4, w5, w6",
    ));
    insns.push((
        Inst::AluRRR {
            alu_op: ALUOp::Asr,
            size: OperandSize::Size64,
            rd: writable_xreg(4),
            rn: xreg(5),
            rm: xreg(6),
        },
        "A428C69A",
        "asr x4, x5, x6",
    ));
    insns.push((
        Inst::AluRRR {
            alu_op: ALUOp::Lsl,
            size: OperandSize::Size32,
            rd: writable_xreg(4),
            rn: xreg(5),
            rm: xreg(6),
        },
        "A420C61A",
        "lsl w4, w5, w6",
    ));
    insns.push((
        Inst::AluRRR {
            alu_op: ALUOp::Lsl,
            size: OperandSize::Size64,
            rd: writable_xreg(4),
            rn: xreg(5),
            rm: xreg(6),
        },
        "A420C69A",
        "lsl x4, x5, x6",
    ));

    insns.push((
        Inst::AluRRImm12 {
            alu_op: ALUOp::Add,
            size: OperandSize::Size32,
            rd: writable_xreg(7),
            rn: xreg(8),
            imm12: Imm12 {
                bits: 0x123,
                shift12: false,
            },
        },
        "078D0411",
        "add w7, w8, #291",
    ));
    insns.push((
        Inst::AluRRImm12 {
            alu_op: ALUOp::Add,
            size: OperandSize::Size32,
            rd: writable_xreg(7),
            rn: xreg(8),
            imm12: Imm12 {
                bits: 0x123,
                shift12: true,
            },
        },
        "078D4411",
        "add w7, w8, #1191936",
    ));
    insns.push((
        Inst::AluRRImm12 {
            alu_op: ALUOp::Add,
            size: OperandSize::Size64,
            rd: writable_xreg(7),
            rn: xreg(8),
            imm12: Imm12 {
                bits: 0x123,
                shift12: false,
            },
        },
        "078D0491",
        "add x7, x8, #291",
    ));
    insns.push((
        Inst::AluRRImm12 {
            alu_op: ALUOp::Sub,
            size: OperandSize::Size32,
            rd: writable_xreg(7),
            rn: xreg(8),
            imm12: Imm12 {
                bits: 0x123,
                shift12: false,
            },
        },
        "078D0451",
        "sub w7, w8, #291",
    ));
    insns.push((
        Inst::AluRRImm12 {
            alu_op: ALUOp::Sub,
            size: OperandSize::Size64,
            rd: writable_xreg(7),
            rn: xreg(8),
            imm12: Imm12 {
                bits: 0x123,
                shift12: false,
            },
        },
        "078D04D1",
        "sub x7, x8, #291",
    ));
    insns.push((
        Inst::AluRRImm12 {
            alu_op: ALUOp::SubS,
            size: OperandSize::Size32,
            rd: writable_xreg(7),
            rn: xreg(8),
            imm12: Imm12 {
                bits: 0x123,
                shift12: false,
            },
        },
        "078D0471",
        "subs w7, w8, #291",
    ));
    insns.push((
        Inst::AluRRImm12 {
            alu_op: ALUOp::SubS,
            size: OperandSize::Size64,
            rd: writable_xreg(7),
            rn: xreg(8),
            imm12: Imm12 {
                bits: 0x123,
                shift12: false,
            },
        },
        "078D04F1",
        "subs x7, x8, #291",
    ));

    insns.push((
        Inst::AluRRRExtend {
            alu_op: ALUOp::Add,
            size: OperandSize::Size32,
            rd: writable_xreg(7),
            rn: xreg(8),
            rm: xreg(9),
            extendop: ExtendOp::SXTB,
        },
        "0781290B",
        "add w7, w8, w9, SXTB",
    ));

    insns.push((
        Inst::AluRRRExtend {
            alu_op: ALUOp::Add,
            size: OperandSize::Size64,
            rd: writable_xreg(15),
            rn: xreg(16),
            rm: xreg(17),
            extendop: ExtendOp::UXTB,
        },
        "0F02318B",
        "add x15, x16, x17, UXTB",
    ));

    insns.push((
        Inst::AluRRRExtend {
            alu_op: ALUOp::Sub,
            size: OperandSize::Size32,
            rd: writable_xreg(1),
            rn: xreg(2),
            rm: xreg(3),
            extendop: ExtendOp::SXTH,
        },
        "41A0234B",
        "sub w1, w2, w3, SXTH",
    ));

    insns.push((
        Inst::AluRRRExtend {
            alu_op: ALUOp::Sub,
            size: OperandSize::Size64,
            rd: writable_xreg(20),
            rn: xreg(21),
            rm: xreg(22),
            extendop: ExtendOp::UXTW,
        },
        "B44236CB",
        "sub x20, x21, x22, UXTW",
    ));

    insns.push((
        Inst::AluRRRShift {
            alu_op: ALUOp::Add,
            size: OperandSize::Size32,
            rd: writable_xreg(10),
            rn: xreg(11),
            rm: xreg(12),
            shiftop: ShiftOpAndAmt::new(
                ShiftOp::LSL,
                ShiftOpShiftImm::maybe_from_shift(20).unwrap(),
            ),
        },
        "6A510C0B",
        "add w10, w11, w12, LSL 20",
    ));
    insns.push((
        Inst::AluRRRShift {
            alu_op: ALUOp::Add,
            size: OperandSize::Size64,
            rd: writable_xreg(10),
            rn: xreg(11),
            rm: xreg(12),
            shiftop: ShiftOpAndAmt::new(
                ShiftOp::ASR,
                ShiftOpShiftImm::maybe_from_shift(42).unwrap(),
            ),
        },
        "6AA98C8B",
        "add x10, x11, x12, ASR 42",
    ));
    insns.push((
        Inst::AluRRRShift {
            alu_op: ALUOp::Sub,
            size: OperandSize::Size32,
            rd: writable_xreg(10),
            rn: xreg(11),
            rm: xreg(12),
            shiftop: ShiftOpAndAmt::new(
                ShiftOp::LSL,
                ShiftOpShiftImm::maybe_from_shift(23).unwrap(),
            ),
        },
        "6A5D0C4B",
        "sub w10, w11, w12, LSL 23",
    ));
    insns.push((
        Inst::AluRRRShift {
            alu_op: ALUOp::Sub,
            size: OperandSize::Size64,
            rd: writable_xreg(10),
            rn: xreg(11),
            rm: xreg(12),
            shiftop: ShiftOpAndAmt::new(
                ShiftOp::LSL,
                ShiftOpShiftImm::maybe_from_shift(23).unwrap(),
            ),
        },
        "6A5D0CCB",
        "sub x10, x11, x12, LSL 23",
    ));
    insns.push((
        Inst::AluRRRShift {
            alu_op: ALUOp::Orr,
            size: OperandSize::Size32,
            rd: writable_xreg(10),
            rn: xreg(11),
            rm: xreg(12),
            shiftop: ShiftOpAndAmt::new(
                ShiftOp::LSL,
                ShiftOpShiftImm::maybe_from_shift(23).unwrap(),
            ),
        },
        "6A5D0C2A",
        "orr w10, w11, w12, LSL 23",
    ));
    insns.push((
        Inst::AluRRRShift {
            alu_op: ALUOp::Orr,
            size: OperandSize::Size64,
            rd: writable_xreg(10),
            rn: xreg(11),
            rm: xreg(12),
            shiftop: ShiftOpAndAmt::new(
                ShiftOp::LSL,
                ShiftOpShiftImm::maybe_from_shift(23).unwrap(),
            ),
        },
        "6A5D0CAA",
        "orr x10, x11, x12, LSL 23",
    ));
    insns.push((
        Inst::AluRRRShift {
            alu_op: ALUOp::And,
            size: OperandSize::Size32,
            rd: writable_xreg(10),
            rn: xreg(11),
            rm: xreg(12),
            shiftop: ShiftOpAndAmt::new(
                ShiftOp::LSL,
                ShiftOpShiftImm::maybe_from_shift(23).unwrap(),
            ),
        },
        "6A5D0C0A",
        "and w10, w11, w12, LSL 23",
    ));
    insns.push((
        Inst::AluRRRShift {
            alu_op: ALUOp::And,
            size: OperandSize::Size64,
            rd: writable_xreg(10),
            rn: xreg(11),
            rm: xreg(12),
            shiftop: ShiftOpAndAmt::new(
                ShiftOp::LSL,
                ShiftOpShiftImm::maybe_from_shift(23).unwrap(),
            ),
        },
        "6A5D0C8A",
        "and x10, x11, x12, LSL 23",
    ));
    insns.push((
        Inst::AluRRRShift {
            alu_op: ALUOp::AndS,
            size: OperandSize::Size32,
            rd: writable_xreg(10),
            rn: xreg(11),
            rm: xreg(12),
            shiftop: ShiftOpAndAmt::new(
                ShiftOp::LSL,
                ShiftOpShiftImm::maybe_from_shift(23).unwrap(),
            ),
        },
        "6A5D0C6A",
        "ands w10, w11, w12, LSL 23",
    ));
    insns.push((
        Inst::AluRRRShift {
            alu_op: ALUOp::AndS,
            size: OperandSize::Size64,
            rd: writable_xreg(10),
            rn: xreg(11),
            rm: xreg(12),
            shiftop: ShiftOpAndAmt::new(
                ShiftOp::LSL,
                ShiftOpShiftImm::maybe_from_shift(23).unwrap(),
            ),
        },
        "6A5D0CEA",
        "ands x10, x11, x12, LSL 23",
    ));
    insns.push((
        Inst::AluRRRShift {
            alu_op: ALUOp::Eor,
            size: OperandSize::Size32,
            rd: writable_xreg(10),
            rn: xreg(11),
            rm: xreg(12),
            shiftop: ShiftOpAndAmt::new(
                ShiftOp::LSL,
                ShiftOpShiftImm::maybe_from_shift(23).unwrap(),
            ),
        },
        "6A5D0C4A",
        "eor w10, w11, w12, LSL 23",
    ));
    insns.push((
        Inst::AluRRRShift {
            alu_op: ALUOp::Eor,
            size: OperandSize::Size64,
            rd: writable_xreg(10),
            rn: xreg(11),
            rm: xreg(12),
            shiftop: ShiftOpAndAmt::new(
                ShiftOp::LSL,
                ShiftOpShiftImm::maybe_from_shift(23).unwrap(),
            ),
        },
        "6A5D0CCA",
        "eor x10, x11, x12, LSL 23",
    ));
    insns.push((
        Inst::AluRRRShift {
            alu_op: ALUOp::OrrNot,
            size: OperandSize::Size32,
            rd: writable_xreg(10),
            rn: xreg(11),
            rm: xreg(12),
            shiftop: ShiftOpAndAmt::new(
                ShiftOp::LSL,
                ShiftOpShiftImm::maybe_from_shift(23).unwrap(),
            ),
        },
        "6A5D2C2A",
        "orn w10, w11, w12, LSL 23",
    ));
    insns.push((
        Inst::AluRRRShift {
            alu_op: ALUOp::OrrNot,
            size: OperandSize::Size64,
            rd: writable_xreg(10),
            rn: xreg(11),
            rm: xreg(12),
            shiftop: ShiftOpAndAmt::new(
                ShiftOp::LSL,
                ShiftOpShiftImm::maybe_from_shift(23).unwrap(),
            ),
        },
        "6A5D2CAA",
        "orn x10, x11, x12, LSL 23",
    ));
    insns.push((
        Inst::AluRRRShift {
            alu_op: ALUOp::AndNot,
            size: OperandSize::Size32,
            rd: writable_xreg(10),
            rn: xreg(11),
            rm: xreg(12),
            shiftop: ShiftOpAndAmt::new(
                ShiftOp::LSL,
                ShiftOpShiftImm::maybe_from_shift(23).unwrap(),
            ),
        },
        "6A5D2C0A",
        "bic w10, w11, w12, LSL 23",
    ));
    insns.push((
        Inst::AluRRRShift {
            alu_op: ALUOp::AndNot,
            size: OperandSize::Size64,
            rd: writable_xreg(10),
            rn: xreg(11),
            rm: xreg(12),
            shiftop: ShiftOpAndAmt::new(
                ShiftOp::LSL,
                ShiftOpShiftImm::maybe_from_shift(23).unwrap(),
            ),
        },
        "6A5D2C8A",
        "bic x10, x11, x12, LSL 23",
    ));
    insns.push((
        Inst::AluRRRShift {
            alu_op: ALUOp::EorNot,
            size: OperandSize::Size32,
            rd: writable_xreg(10),
            rn: xreg(11),
            rm: xreg(12),
            shiftop: ShiftOpAndAmt::new(
                ShiftOp::LSL,
                ShiftOpShiftImm::maybe_from_shift(23).unwrap(),
            ),
        },
        "6A5D2C4A",
        "eon w10, w11, w12, LSL 23",
    ));
    insns.push((
        Inst::AluRRRShift {
            alu_op: ALUOp::EorNot,
            size: OperandSize::Size64,
            rd: writable_xreg(10),
            rn: xreg(11),
            rm: xreg(12),
            shiftop: ShiftOpAndAmt::new(
                ShiftOp::LSL,
                ShiftOpShiftImm::maybe_from_shift(23).unwrap(),
            ),
        },
        "6A5D2CCA",
        "eon x10, x11, x12, LSL 23",
    ));
    insns.push((
        Inst::AluRRRShift {
            alu_op: ALUOp::AddS,
            size: OperandSize::Size32,
            rd: writable_xreg(10),
            rn: xreg(11),
            rm: xreg(12),
            shiftop: ShiftOpAndAmt::new(
                ShiftOp::LSL,
                ShiftOpShiftImm::maybe_from_shift(23).unwrap(),
            ),
        },
        "6A5D0C2B",
        "adds w10, w11, w12, LSL 23",
    ));
    insns.push((
        Inst::AluRRRShift {
            alu_op: ALUOp::AddS,
            size: OperandSize::Size64,
            rd: writable_xreg(10),
            rn: xreg(11),
            rm: xreg(12),
            shiftop: ShiftOpAndAmt::new(
                ShiftOp::LSL,
                ShiftOpShiftImm::maybe_from_shift(23).unwrap(),
            ),
        },
        "6A5D0CAB",
        "adds x10, x11, x12, LSL 23",
    ));
    insns.push((
        Inst::AluRRRShift {
            alu_op: ALUOp::SubS,
            size: OperandSize::Size32,
            rd: writable_xreg(10),
            rn: xreg(11),
            rm: xreg(12),
            shiftop: ShiftOpAndAmt::new(
                ShiftOp::LSL,
                ShiftOpShiftImm::maybe_from_shift(23).unwrap(),
            ),
        },
        "6A5D0C6B",
        "subs w10, w11, w12, LSL 23",
    ));
    insns.push((
        Inst::AluRRRShift {
            alu_op: ALUOp::SubS,
            size: OperandSize::Size64,
            rd: writable_xreg(10),
            rn: xreg(11),
            rm: xreg(12),
            shiftop: ShiftOpAndAmt::new(
                ShiftOp::LSL,
                ShiftOpShiftImm::maybe_from_shift(23).unwrap(),
            ),
        },
        "6A5D0CEB",
        "subs x10, x11, x12, LSL 23",
    ));

    insns.push((
        Inst::AluRRRExtend {
            alu_op: ALUOp::SubS,
            size: OperandSize::Size64,
            rd: writable_zero_reg(),
            rn: stack_reg(),
            rm: xreg(12),
            extendop: ExtendOp::UXTX,
        },
        "FF632CEB",
        "subs xzr, sp, x12, UXTX",
    ));

    insns.push((
        Inst::AluRRRR {
            alu_op: ALUOp3::MAdd,
            size: OperandSize::Size32,
            rd: writable_xreg(1),
            rn: xreg(2),
            rm: xreg(3),
            ra: xreg(4),
        },
        "4110031B",
        "madd w1, w2, w3, w4",
    ));
    insns.push((
        Inst::AluRRRR {
            alu_op: ALUOp3::MAdd,
            size: OperandSize::Size64,
            rd: writable_xreg(1),
            rn: xreg(2),
            rm: xreg(3),
            ra: xreg(4),
        },
        "4110039B",
        "madd x1, x2, x3, x4",
    ));
    insns.push((
        Inst::AluRRRR {
            alu_op: ALUOp3::MSub,
            size: OperandSize::Size32,
            rd: writable_xreg(1),
            rn: xreg(2),
            rm: xreg(3),
            ra: xreg(4),
        },
        "4190031B",
        "msub w1, w2, w3, w4",
    ));
    insns.push((
        Inst::AluRRRR {
            alu_op: ALUOp3::MSub,
            size: OperandSize::Size64,
            rd: writable_xreg(1),
            rn: xreg(2),
            rm: xreg(3),
            ra: xreg(4),
        },
        "4190039B",
        "msub x1, x2, x3, x4",
    ));
    insns.push((
        Inst::AluRRRR {
            alu_op: ALUOp3::UMAddL,
            size: OperandSize::Size32,
            rd: writable_xreg(1),
            rn: xreg(2),
            rm: xreg(3),
            ra: xreg(4),
        },
        "4110A39B",
        "umaddl x1, w2, w3, x4",
    ));
    insns.push((
        Inst::AluRRRR {
            alu_op: ALUOp3::SMAddL,
            size: OperandSize::Size32,
            rd: writable_xreg(1),
            rn: xreg(2),
            rm: xreg(3),
            ra: xreg(4),
        },
        "4110239B",
        "smaddl x1, w2, w3, x4",
    ));
    insns.push((
        Inst::AluRRR {
            alu_op: ALUOp::SMulH,
            size: OperandSize::Size64,
            rd: writable_xreg(1),
            rn: xreg(2),
            rm: xreg(3),
        },
        "417C439B",
        "smulh x1, x2, x3",
    ));
    insns.push((
        Inst::AluRRR {
            alu_op: ALUOp::UMulH,
            size: OperandSize::Size64,
            rd: writable_xreg(1),
            rn: xreg(2),
            rm: xreg(3),
        },
        "417CC39B",
        "umulh x1, x2, x3",
    ));

    insns.push((
        Inst::AluRRImmShift {
            alu_op: ALUOp::Extr,
            size: OperandSize::Size32,
            rd: writable_xreg(20),
            rn: xreg(21),
            immshift: ImmShift::maybe_from_u64(19).unwrap(),
        },
        "B44E9513",
        "extr w20, w21, #19",
    ));
    insns.push((
        Inst::AluRRImmShift {
            alu_op: ALUOp::Extr,
            size: OperandSize::Size64,
            rd: writable_xreg(20),
            rn: xreg(21),
            immshift: ImmShift::maybe_from_u64(42).unwrap(),
        },
        "B4AAD593",
        "extr x20, x21, #42",
    ));
    insns.push((
        Inst::AluRRImmShift {
            alu_op: ALUOp::Lsr,
            size: OperandSize::Size32,
            rd: writable_xreg(10),
            rn: xreg(11),
            immshift: ImmShift::maybe_from_u64(13).unwrap(),
        },
        "6A7D0D53",
        "lsr w10, w11, #13",
    ));
    insns.push((
        Inst::AluRRImmShift {
            alu_op: ALUOp::Lsr,
            size: OperandSize::Size64,
            rd: writable_xreg(10),
            rn: xreg(11),
            immshift: ImmShift::maybe_from_u64(57).unwrap(),
        },
        "6AFD79D3",
        "lsr x10, x11, #57",
    ));
    insns.push((
        Inst::AluRRImmShift {
            alu_op: ALUOp::Asr,
            size: OperandSize::Size32,
            rd: writable_xreg(4),
            rn: xreg(5),
            immshift: ImmShift::maybe_from_u64(7).unwrap(),
        },
        "A47C0713",
        "asr w4, w5, #7",
    ));
    insns.push((
        Inst::AluRRImmShift {
            alu_op: ALUOp::Asr,
            size: OperandSize::Size64,
            rd: writable_xreg(4),
            rn: xreg(5),
            immshift: ImmShift::maybe_from_u64(35).unwrap(),
        },
        "A4FC6393",
        "asr x4, x5, #35",
    ));
    insns.push((
        Inst::AluRRImmShift {
            alu_op: ALUOp::Lsl,
            size: OperandSize::Size32,
            rd: writable_xreg(8),
            rn: xreg(9),
            immshift: ImmShift::maybe_from_u64(24).unwrap(),
        },
        "281D0853",
        "lsl w8, w9, #24",
    ));
    insns.push((
        Inst::AluRRImmShift {
            alu_op: ALUOp::Lsl,
            size: OperandSize::Size64,
            rd: writable_xreg(8),
            rn: xreg(9),
            immshift: ImmShift::maybe_from_u64(63).unwrap(),
        },
        "280141D3",
        "lsl x8, x9, #63",
    ));
    insns.push((
        Inst::AluRRImmShift {
            alu_op: ALUOp::Lsl,
            size: OperandSize::Size32,
            rd: writable_xreg(10),
            rn: xreg(11),
            immshift: ImmShift::maybe_from_u64(0).unwrap(),
        },
        "6A7D0053",
        "lsl w10, w11, #0",
    ));
    insns.push((
        Inst::AluRRImmShift {
            alu_op: ALUOp::Lsl,
            size: OperandSize::Size64,
            rd: writable_xreg(10),
            rn: xreg(11),
            immshift: ImmShift::maybe_from_u64(0).unwrap(),
        },
        "6AFD40D3",
        "lsl x10, x11, #0",
    ));

    insns.push((
        Inst::AluRRImmLogic {
            alu_op: ALUOp::And,
            size: OperandSize::Size32,
            rd: writable_xreg(21),
            rn: xreg(27),
            imml: ImmLogic::maybe_from_u64(0x80003fff, I32).unwrap(),
        },
        "753B0112",
        "and w21, w27, #2147500031",
    ));
    insns.push((
        Inst::AluRRImmLogic {
            alu_op: ALUOp::And,
            size: OperandSize::Size64,
            rd: writable_xreg(7),
            rn: xreg(6),
            imml: ImmLogic::maybe_from_u64(0x3fff80003fff800, I64).unwrap(),
        },
        "C7381592",
        "and x7, x6, #288221580125796352",
    ));
    insns.push((
        Inst::AluRRImmLogic {
            alu_op: ALUOp::AndS,
            size: OperandSize::Size32,
            rd: writable_xreg(21),
            rn: xreg(27),
            imml: ImmLogic::maybe_from_u64(0x80003fff, I32).unwrap(),
        },
        "753B0172",
        "ands w21, w27, #2147500031",
    ));
    insns.push((
        Inst::AluRRImmLogic {
            alu_op: ALUOp::AndS,
            size: OperandSize::Size64,
            rd: writable_xreg(7),
            rn: xreg(6),
            imml: ImmLogic::maybe_from_u64(0x3fff80003fff800, I64).unwrap(),
        },
        "C73815F2",
        "ands x7, x6, #288221580125796352",
    ));
    insns.push((
        Inst::AluRRImmLogic {
            alu_op: ALUOp::Orr,
            size: OperandSize::Size32,
            rd: writable_xreg(1),
            rn: xreg(5),
            imml: ImmLogic::maybe_from_u64(0x100000, I32).unwrap(),
        },
        "A1000C32",
        "orr w1, w5, #1048576",
    ));
    insns.push((
        Inst::AluRRImmLogic {
            alu_op: ALUOp::Orr,
            size: OperandSize::Size64,
            rd: writable_xreg(4),
            rn: xreg(5),
            imml: ImmLogic::maybe_from_u64(0x8181818181818181, I64).unwrap(),
        },
        "A4C401B2",
        "orr x4, x5, #9331882296111890817",
    ));
    insns.push((
        Inst::AluRRImmLogic {
            alu_op: ALUOp::Eor,
            size: OperandSize::Size32,
            rd: writable_xreg(1),
            rn: xreg(5),
            imml: ImmLogic::maybe_from_u64(0x00007fff, I32).unwrap(),
        },
        "A1380052",
        "eor w1, w5, #32767",
    ));
    insns.push((
        Inst::AluRRImmLogic {
            alu_op: ALUOp::Eor,
            size: OperandSize::Size64,
            rd: writable_xreg(10),
            rn: xreg(8),
            imml: ImmLogic::maybe_from_u64(0x8181818181818181, I64).unwrap(),
        },
        "0AC501D2",
        "eor x10, x8, #9331882296111890817",
    ));

    insns.push((
        Inst::BitRR {
            op: BitOp::RBit,
            size: OperandSize::Size32,
            rd: writable_xreg(1),
            rn: xreg(10),
        },
        "4101C05A",
        "rbit w1, w10",
    ));

    insns.push((
        Inst::BitRR {
            op: BitOp::RBit,
            size: OperandSize::Size64,
            rd: writable_xreg(1),
            rn: xreg(10),
        },
        "4101C0DA",
        "rbit x1, x10",
    ));

    insns.push((
        Inst::BitRR {
            op: BitOp::Clz,
            size: OperandSize::Size32,
            rd: writable_xreg(15),
            rn: xreg(3),
        },
        "6F10C05A",
        "clz w15, w3",
    ));

    insns.push((
        Inst::BitRR {
            op: BitOp::Clz,
            size: OperandSize::Size64,
            rd: writable_xreg(15),
            rn: xreg(3),
        },
        "6F10C0DA",
        "clz x15, x3",
    ));

    insns.push((
        Inst::BitRR {
            op: BitOp::Cls,
            size: OperandSize::Size32,
            rd: writable_xreg(21),
            rn: xreg(16),
        },
        "1516C05A",
        "cls w21, w16",
    ));

    insns.push((
        Inst::BitRR {
            op: BitOp::Cls,
            size: OperandSize::Size64,
            rd: writable_xreg(21),
            rn: xreg(16),
        },
        "1516C0DA",
        "cls x21, x16",
    ));

    insns.push((
        Inst::BitRR {
            op: BitOp::Rev16,
            size: OperandSize::Size64,
            rd: writable_xreg(2),
            rn: xreg(11),
        },
        "6205C0DA",
        "rev16 x2, x11",
    ));

    insns.push((
        Inst::BitRR {
            op: BitOp::Rev16,
            size: OperandSize::Size32,
            rd: writable_xreg(3),
            rn: xreg(21),
        },
        "A306C05A",
        "rev16 w3, w21",
    ));

    insns.push((
        Inst::BitRR {
            op: BitOp::Rev32,
            size: OperandSize::Size64,
            rd: writable_xreg(2),
            rn: xreg(11),
        },
        "6209C0DA",
        "rev32 x2, x11",
    ));

    insns.push((
        Inst::BitRR {
            op: BitOp::Rev32,
            size: OperandSize::Size32,
            rd: writable_xreg(3),
            rn: xreg(21),
        },
        "A30AC05A",
        "rev32 w3, w21",
    ));

    insns.push((
        Inst::BitRR {
            op: BitOp::Rev64,
            size: OperandSize::Size64,
            rd: writable_xreg(1),
            rn: xreg(10),
        },
        "410DC0DA",
        "rev64 x1, x10",
    ));

    insns.push((
        Inst::ULoad8 {
            rd: writable_xreg(1),
            mem: AMode::Unscaled {
                rn: xreg(2),
                simm9: simm9_zero(),
            },
            flags: MemFlags::trusted(),
        },
        "41004038",
        "ldurb w1, [x2]",
    ));
    insns.push((
        Inst::ULoad8 {
            rd: writable_xreg(1),
            mem: AMode::UnsignedOffset {
                rn: xreg(2),
                uimm12: UImm12Scaled::zero(I8),
            },
            flags: MemFlags::trusted(),
        },
        "41004039",
        "ldrb w1, [x2]",
    ));
    insns.push((
        Inst::ULoad8 {
            rd: writable_xreg(1),
            mem: AMode::RegReg {
                rn: xreg(2),
                rm: xreg(5),
            },
            flags: MemFlags::trusted(),
        },
        "41686538",
        "ldrb w1, [x2, x5]",
    ));
    insns.push((
        Inst::SLoad8 {
            rd: writable_xreg(1),
            mem: AMode::Unscaled {
                rn: xreg(2),
                simm9: simm9_zero(),
            },
            flags: MemFlags::trusted(),
        },
        "41008038",
        "ldursb x1, [x2]",
    ));
    insns.push((
        Inst::SLoad8 {
            rd: writable_xreg(1),
            mem: AMode::UnsignedOffset {
                rn: xreg(2),
                uimm12: UImm12Scaled::maybe_from_i64(63, I8).unwrap(),
            },
            flags: MemFlags::trusted(),
        },
        "41FC8039",
        "ldrsb x1, [x2, #63]",
    ));
    insns.push((
        Inst::SLoad8 {
            rd: writable_xreg(1),
            mem: AMode::RegReg {
                rn: xreg(2),
                rm: xreg(5),
            },
            flags: MemFlags::trusted(),
        },
        "4168A538",
        "ldrsb x1, [x2, x5]",
    ));
    insns.push((
        Inst::ULoad16 {
            rd: writable_xreg(1),
            mem: AMode::Unscaled {
                rn: xreg(2),
                simm9: SImm9::maybe_from_i64(5).unwrap(),
            },
            flags: MemFlags::trusted(),
        },
        "41504078",
        "ldurh w1, [x2, #5]",
    ));
    insns.push((
        Inst::ULoad16 {
            rd: writable_xreg(1),
            mem: AMode::UnsignedOffset {
                rn: xreg(2),
                uimm12: UImm12Scaled::maybe_from_i64(8, I16).unwrap(),
            },
            flags: MemFlags::trusted(),
        },
        "41104079",
        "ldrh w1, [x2, #8]",
    ));
    insns.push((
        Inst::ULoad16 {
            rd: writable_xreg(1),
            mem: AMode::RegScaled {
                rn: xreg(2),
                rm: xreg(3),
            },
            flags: MemFlags::trusted(),
        },
        "41786378",
        "ldrh w1, [x2, x3, LSL #1]",
    ));
    insns.push((
        Inst::SLoad16 {
            rd: writable_xreg(1),
            mem: AMode::Unscaled {
                rn: xreg(2),
                simm9: simm9_zero(),
            },
            flags: MemFlags::trusted(),
        },
        "41008078",
        "ldursh x1, [x2]",
    ));
    insns.push((
        Inst::SLoad16 {
            rd: writable_xreg(28),
            mem: AMode::UnsignedOffset {
                rn: xreg(20),
                uimm12: UImm12Scaled::maybe_from_i64(24, I16).unwrap(),
            },
            flags: MemFlags::trusted(),
        },
        "9C328079",
        "ldrsh x28, [x20, #24]",
    ));
    insns.push((
        Inst::SLoad16 {
            rd: writable_xreg(28),
            mem: AMode::RegScaled {
                rn: xreg(20),
                rm: xreg(20),
            },
            flags: MemFlags::trusted(),
        },
        "9C7AB478",
        "ldrsh x28, [x20, x20, LSL #1]",
    ));
    insns.push((
        Inst::ULoad32 {
            rd: writable_xreg(1),
            mem: AMode::Unscaled {
                rn: xreg(2),
                simm9: simm9_zero(),
            },
            flags: MemFlags::trusted(),
        },
        "410040B8",
        "ldur w1, [x2]",
    ));
    insns.push((
        Inst::ULoad32 {
            rd: writable_xreg(12),
            mem: AMode::UnsignedOffset {
                rn: xreg(0),
                uimm12: UImm12Scaled::maybe_from_i64(204, I32).unwrap(),
            },
            flags: MemFlags::trusted(),
        },
        "0CCC40B9",
        "ldr w12, [x0, #204]",
    ));
    insns.push((
        Inst::ULoad32 {
            rd: writable_xreg(1),
            mem: AMode::RegScaled {
                rn: xreg(2),
                rm: xreg(12),
            },
            flags: MemFlags::trusted(),
        },
        "41786CB8",
        "ldr w1, [x2, x12, LSL #2]",
    ));
    insns.push((
        Inst::SLoad32 {
            rd: writable_xreg(1),
            mem: AMode::Unscaled {
                rn: xreg(2),
                simm9: simm9_zero(),
            },
            flags: MemFlags::trusted(),
        },
        "410080B8",
        "ldursw x1, [x2]",
    ));
    insns.push((
        Inst::SLoad32 {
            rd: writable_xreg(12),
            mem: AMode::UnsignedOffset {
                rn: xreg(1),
                uimm12: UImm12Scaled::maybe_from_i64(16380, I32).unwrap(),
            },
            flags: MemFlags::trusted(),
        },
        "2CFCBFB9",
        "ldrsw x12, [x1, #16380]",
    ));
    insns.push((
        Inst::SLoad32 {
            rd: writable_xreg(1),
            mem: AMode::RegScaled {
                rn: xreg(5),
                rm: xreg(1),
            },
            flags: MemFlags::trusted(),
        },
        "A178A1B8",
        "ldrsw x1, [x5, x1, LSL #2]",
    ));
    insns.push((
        Inst::ULoad64 {
            rd: writable_xreg(1),
            mem: AMode::Unscaled {
                rn: xreg(2),
                simm9: simm9_zero(),
            },
            flags: MemFlags::trusted(),
        },
        "410040F8",
        "ldur x1, [x2]",
    ));
    insns.push((
        Inst::ULoad64 {
            rd: writable_xreg(1),
            mem: AMode::Unscaled {
                rn: xreg(2),
                simm9: SImm9::maybe_from_i64(-256).unwrap(),
            },
            flags: MemFlags::trusted(),
        },
        "410050F8",
        "ldur x1, [x2, #-256]",
    ));
    insns.push((
        Inst::ULoad64 {
            rd: writable_xreg(1),
            mem: AMode::Unscaled {
                rn: xreg(2),
                simm9: SImm9::maybe_from_i64(255).unwrap(),
            },
            flags: MemFlags::trusted(),
        },
        "41F04FF8",
        "ldur x1, [x2, #255]",
    ));
    insns.push((
        Inst::ULoad64 {
            rd: writable_xreg(1),
            mem: AMode::UnsignedOffset {
                rn: xreg(2),
                uimm12: UImm12Scaled::maybe_from_i64(32760, I64).unwrap(),
            },
            flags: MemFlags::trusted(),
        },
        "41FC7FF9",
        "ldr x1, [x2, #32760]",
    ));
    insns.push((
        Inst::ULoad64 {
            rd: writable_xreg(1),
            mem: AMode::RegReg {
                rn: xreg(2),
                rm: xreg(3),
            },
            flags: MemFlags::trusted(),
        },
        "416863F8",
        "ldr x1, [x2, x3]",
    ));
    insns.push((
        Inst::ULoad64 {
            rd: writable_xreg(1),
            mem: AMode::RegScaled {
                rn: xreg(2),
                rm: xreg(3),
            },
            flags: MemFlags::trusted(),
        },
        "417863F8",
        "ldr x1, [x2, x3, LSL #3]",
    ));
    insns.push((
        Inst::ULoad64 {
            rd: writable_xreg(1),
            mem: AMode::RegScaledExtended {
                rn: xreg(2),
                rm: xreg(3),
                extendop: ExtendOp::SXTW,
            },
            flags: MemFlags::trusted(),
        },
        "41D863F8",
        "ldr x1, [x2, w3, SXTW #3]",
    ));
    insns.push((
        Inst::ULoad64 {
            rd: writable_xreg(1),
            mem: AMode::RegExtended {
                rn: xreg(2),
                rm: xreg(3),
                extendop: ExtendOp::SXTW,
            },
            flags: MemFlags::trusted(),
        },
        "41C863F8",
        "ldr x1, [x2, w3, SXTW]",
    ));
    insns.push((
        Inst::ULoad64 {
            rd: writable_xreg(1),
            mem: AMode::Label {
                label: MemLabel::PCRel(64),
            },
            flags: MemFlags::trusted(),
        },
        "01020058",
        "ldr x1, pc+64",
    ));
    insns.push((
        Inst::ULoad64 {
            rd: writable_xreg(1),
            mem: AMode::SPPreIndexed {
                simm9: SImm9::maybe_from_i64(16).unwrap(),
            },
            flags: MemFlags::trusted(),
        },
        "E10F41F8",
        "ldr x1, [sp, #16]!",
    ));
    insns.push((
        Inst::ULoad64 {
            rd: writable_xreg(1),
            mem: AMode::SPPostIndexed {
                simm9: SImm9::maybe_from_i64(16).unwrap(),
            },
            flags: MemFlags::trusted(),
        },
        "E10741F8",
        "ldr x1, [sp], #16",
    ));
    insns.push((
        Inst::ULoad64 {
            rd: writable_xreg(1),
            mem: AMode::FPOffset { off: 32768 },
            flags: MemFlags::trusted(),
        },
        "100090D2A1EB70F8",
        "movz x16, #32768 ; ldr x1, [fp, x16, SXTX]",
    ));
    insns.push((
        Inst::ULoad64 {
            rd: writable_xreg(1),
            mem: AMode::FPOffset { off: -32768 },
            flags: MemFlags::trusted(),
        },
        "F0FF8F92A1EB70F8",
        "movn x16, #32767 ; ldr x1, [fp, x16, SXTX]",
    ));
    insns.push((
        Inst::ULoad64 {
            rd: writable_xreg(1),
            mem: AMode::FPOffset { off: 1048576 }, // 2^20
            flags: MemFlags::trusted(),
        },
        "1002A0D2A1EB70F8",
        "movz x16, #16, LSL #16 ; ldr x1, [fp, x16, SXTX]",
    ));
    insns.push((
        Inst::ULoad64 {
            rd: writable_xreg(1),
            mem: AMode::FPOffset { off: 1048576 + 1 }, // 2^20 + 1
            flags: MemFlags::trusted(),
        },
        "300080521002A072A1EB70F8",
        "movz w16, #1 ; movk w16, w16, #16, LSL #16 ; ldr x1, [fp, x16, SXTX]",
    ));

    insns.push((
        Inst::ULoad64 {
            rd: writable_xreg(1),
            mem: AMode::RegOffset {
                rn: xreg(7),
                off: 8,
            },
            flags: MemFlags::trusted(),
        },
        "E18040F8",
        "ldr x1, [x7, #8]",
    ));

    insns.push((
        Inst::ULoad64 {
            rd: writable_xreg(1),
            mem: AMode::RegOffset {
                rn: xreg(7),
                off: 1024,
            },
            flags: MemFlags::trusted(),
        },
        "E10042F9",
        "ldr x1, [x7, #1024]",
    ));

    insns.push((
        Inst::ULoad64 {
            rd: writable_xreg(1),
            mem: AMode::RegOffset {
                rn: xreg(7),
                off: 1048576,
            },
            flags: MemFlags::trusted(),
        },
        "1002A0D2E1E870F8",
        "movz x16, #16, LSL #16 ; ldr x1, [x7, x16, SXTX]",
    ));

    insns.push((
        Inst::Store8 {
            rd: xreg(1),
            mem: AMode::Unscaled {
                rn: xreg(2),
                simm9: simm9_zero(),
            },
            flags: MemFlags::trusted(),
        },
        "41000038",
        "sturb w1, [x2]",
    ));
    insns.push((
        Inst::Store8 {
            rd: xreg(1),
            mem: AMode::UnsignedOffset {
                rn: xreg(2),
                uimm12: UImm12Scaled::maybe_from_i64(4095, I8).unwrap(),
            },
            flags: MemFlags::trusted(),
        },
        "41FC3F39",
        "strb w1, [x2, #4095]",
    ));
    insns.push((
        Inst::Store16 {
            rd: xreg(1),
            mem: AMode::Unscaled {
                rn: xreg(2),
                simm9: simm9_zero(),
            },
            flags: MemFlags::trusted(),
        },
        "41000078",
        "sturh w1, [x2]",
    ));
    insns.push((
        Inst::Store16 {
            rd: xreg(1),
            mem: AMode::UnsignedOffset {
                rn: xreg(2),
                uimm12: UImm12Scaled::maybe_from_i64(8190, I16).unwrap(),
            },
            flags: MemFlags::trusted(),
        },
        "41FC3F79",
        "strh w1, [x2, #8190]",
    ));
    insns.push((
        Inst::Store32 {
            rd: xreg(1),
            mem: AMode::Unscaled {
                rn: xreg(2),
                simm9: simm9_zero(),
            },
            flags: MemFlags::trusted(),
        },
        "410000B8",
        "stur w1, [x2]",
    ));
    insns.push((
        Inst::Store32 {
            rd: xreg(1),
            mem: AMode::UnsignedOffset {
                rn: xreg(2),
                uimm12: UImm12Scaled::maybe_from_i64(16380, I32).unwrap(),
            },
            flags: MemFlags::trusted(),
        },
        "41FC3FB9",
        "str w1, [x2, #16380]",
    ));
    insns.push((
        Inst::Store64 {
            rd: xreg(1),
            mem: AMode::Unscaled {
                rn: xreg(2),
                simm9: simm9_zero(),
            },
            flags: MemFlags::trusted(),
        },
        "410000F8",
        "stur x1, [x2]",
    ));
    insns.push((
        Inst::Store64 {
            rd: xreg(1),
            mem: AMode::UnsignedOffset {
                rn: xreg(2),
                uimm12: UImm12Scaled::maybe_from_i64(32760, I64).unwrap(),
            },
            flags: MemFlags::trusted(),
        },
        "41FC3FF9",
        "str x1, [x2, #32760]",
    ));
    insns.push((
        Inst::Store64 {
            rd: xreg(1),
            mem: AMode::RegReg {
                rn: xreg(2),
                rm: xreg(3),
            },
            flags: MemFlags::trusted(),
        },
        "416823F8",
        "str x1, [x2, x3]",
    ));
    insns.push((
        Inst::Store64 {
            rd: xreg(1),
            mem: AMode::RegScaled {
                rn: xreg(2),
                rm: xreg(3),
            },
            flags: MemFlags::trusted(),
        },
        "417823F8",
        "str x1, [x2, x3, LSL #3]",
    ));
    insns.push((
        Inst::Store64 {
            rd: xreg(1),
            mem: AMode::RegScaledExtended {
                rn: xreg(2),
                rm: xreg(3),
                extendop: ExtendOp::UXTW,
            },
            flags: MemFlags::trusted(),
        },
        "415823F8",
        "str x1, [x2, w3, UXTW #3]",
    ));
    insns.push((
        Inst::Store64 {
            rd: xreg(1),
            mem: AMode::RegExtended {
                rn: xreg(2),
                rm: xreg(3),
                extendop: ExtendOp::UXTW,
            },
            flags: MemFlags::trusted(),
        },
        "414823F8",
        "str x1, [x2, w3, UXTW]",
    ));
    insns.push((
        Inst::Store64 {
            rd: xreg(1),
            mem: AMode::SPPreIndexed {
                simm9: SImm9::maybe_from_i64(16).unwrap(),
            },
            flags: MemFlags::trusted(),
        },
        "E10F01F8",
        "str x1, [sp, #16]!",
    ));
    insns.push((
        Inst::Store64 {
            rd: xreg(1),
            mem: AMode::SPPostIndexed {
                simm9: SImm9::maybe_from_i64(16).unwrap(),
            },
            flags: MemFlags::trusted(),
        },
        "E10701F8",
        "str x1, [sp], #16",
    ));

    insns.push((
        Inst::StoreP64 {
            rt: xreg(8),
            rt2: xreg(9),
            mem: PairAMode::SignedOffset {
                reg: xreg(10),
                simm7: simm7_scaled_zero(I64),
            },
            flags: MemFlags::trusted(),
        },
        "482500A9",
        "stp x8, x9, [x10]",
    ));
    insns.push((
        Inst::StoreP64 {
            rt: xreg(8),
            rt2: xreg(9),
            mem: PairAMode::SignedOffset {
                reg: xreg(10),
                simm7: SImm7Scaled::maybe_from_i64(504, I64).unwrap(),
            },
            flags: MemFlags::trusted(),
        },
        "48A51FA9",
        "stp x8, x9, [x10, #504]",
    ));
    insns.push((
        Inst::StoreP64 {
            rt: xreg(8),
            rt2: xreg(9),
            mem: PairAMode::SignedOffset {
                reg: xreg(10),
                simm7: SImm7Scaled::maybe_from_i64(-64, I64).unwrap(),
            },
            flags: MemFlags::trusted(),
        },
        "48253CA9",
        "stp x8, x9, [x10, #-64]",
    ));
    insns.push((
        Inst::StoreP64 {
            rt: xreg(21),
            rt2: xreg(28),
            mem: PairAMode::SignedOffset {
                reg: xreg(1),
                simm7: SImm7Scaled::maybe_from_i64(-512, I64).unwrap(),
            },
            flags: MemFlags::trusted(),
        },
        "357020A9",
        "stp x21, x28, [x1, #-512]",
    ));
    insns.push((
        Inst::StoreP64 {
            rt: xreg(8),
            rt2: xreg(9),
            mem: PairAMode::SPPreIndexed {
                simm7: SImm7Scaled::maybe_from_i64(-64, I64).unwrap(),
            },
            flags: MemFlags::trusted(),
        },
        "E827BCA9",
        "stp x8, x9, [sp, #-64]!",
    ));
    insns.push((
        Inst::StoreP64 {
            rt: xreg(15),
            rt2: xreg(16),
            mem: PairAMode::SPPostIndexed {
                simm7: SImm7Scaled::maybe_from_i64(504, I64).unwrap(),
            },
            flags: MemFlags::trusted(),
        },
        "EFC39FA8",
        "stp x15, x16, [sp], #504",
    ));

    insns.push((
        Inst::LoadP64 {
            rt: writable_xreg(8),
            rt2: writable_xreg(9),
            mem: PairAMode::SignedOffset {
                reg: xreg(10),
                simm7: simm7_scaled_zero(I64),
            },
            flags: MemFlags::trusted(),
        },
        "482540A9",
        "ldp x8, x9, [x10]",
    ));
    insns.push((
        Inst::LoadP64 {
            rt: writable_xreg(8),
            rt2: writable_xreg(9),
            mem: PairAMode::SignedOffset {
                reg: xreg(10),
                simm7: SImm7Scaled::maybe_from_i64(504, I64).unwrap(),
            },
            flags: MemFlags::trusted(),
        },
        "48A55FA9",
        "ldp x8, x9, [x10, #504]",
    ));
    insns.push((
        Inst::LoadP64 {
            rt: writable_xreg(8),
            rt2: writable_xreg(9),
            mem: PairAMode::SignedOffset {
                reg: xreg(10),
                simm7: SImm7Scaled::maybe_from_i64(-64, I64).unwrap(),
            },
            flags: MemFlags::trusted(),
        },
        "48257CA9",
        "ldp x8, x9, [x10, #-64]",
    ));
    insns.push((
        Inst::LoadP64 {
            rt: writable_xreg(8),
            rt2: writable_xreg(9),
            mem: PairAMode::SignedOffset {
                reg: xreg(10),
                simm7: SImm7Scaled::maybe_from_i64(-512, I64).unwrap(),
            },
            flags: MemFlags::trusted(),
        },
        "482560A9",
        "ldp x8, x9, [x10, #-512]",
    ));
    insns.push((
        Inst::LoadP64 {
            rt: writable_xreg(8),
            rt2: writable_xreg(9),
            mem: PairAMode::SPPreIndexed {
                simm7: SImm7Scaled::maybe_from_i64(-64, I64).unwrap(),
            },
            flags: MemFlags::trusted(),
        },
        "E827FCA9",
        "ldp x8, x9, [sp, #-64]!",
    ));
    insns.push((
        Inst::LoadP64 {
            rt: writable_xreg(8),
            rt2: writable_xreg(25),
            mem: PairAMode::SPPostIndexed {
                simm7: SImm7Scaled::maybe_from_i64(504, I64).unwrap(),
            },
            flags: MemFlags::trusted(),
        },
        "E8E7DFA8",
        "ldp x8, x25, [sp], #504",
    ));

    insns.push((
        Inst::Mov {
            size: OperandSize::Size64,
            rd: writable_xreg(8),
            rm: xreg(9),
        },
        "E80309AA",
        "mov x8, x9",
    ));
    insns.push((
        Inst::Mov {
            size: OperandSize::Size32,
            rd: writable_xreg(8),
            rm: xreg(9),
        },
        "E803092A",
        "mov w8, w9",
    ));

    insns.push((
        Inst::MovWide {
            op: MoveWideOp::MovZ,
            rd: writable_xreg(8),
            imm: MoveWideConst::maybe_from_u64(0x0000_0000_0000_ffff).unwrap(),
            size: OperandSize::Size64,
        },
        "E8FF9FD2",
        "movz x8, #65535",
    ));
    insns.push((
        Inst::MovWide {
            op: MoveWideOp::MovZ,
            rd: writable_xreg(8),
            imm: MoveWideConst::maybe_from_u64(0x0000_0000_ffff_0000).unwrap(),
            size: OperandSize::Size64,
        },
        "E8FFBFD2",
        "movz x8, #65535, LSL #16",
    ));
    insns.push((
        Inst::MovWide {
            op: MoveWideOp::MovZ,
            rd: writable_xreg(8),
            imm: MoveWideConst::maybe_from_u64(0x0000_ffff_0000_0000).unwrap(),
            size: OperandSize::Size64,
        },
        "E8FFDFD2",
        "movz x8, #65535, LSL #32",
    ));
    insns.push((
        Inst::MovWide {
            op: MoveWideOp::MovZ,
            rd: writable_xreg(8),
            imm: MoveWideConst::maybe_from_u64(0xffff_0000_0000_0000).unwrap(),
            size: OperandSize::Size64,
        },
        "E8FFFFD2",
        "movz x8, #65535, LSL #48",
    ));
    insns.push((
        Inst::MovWide {
            op: MoveWideOp::MovZ,
            rd: writable_xreg(8),
            imm: MoveWideConst::maybe_from_u64(0x0000_0000_ffff_0000).unwrap(),
            size: OperandSize::Size32,
        },
        "E8FFBF52",
        "movz w8, #65535, LSL #16",
    ));

    insns.push((
        Inst::MovWide {
            op: MoveWideOp::MovN,
            rd: writable_xreg(8),
            imm: MoveWideConst::maybe_from_u64(0x0000_0000_0000_ffff).unwrap(),
            size: OperandSize::Size64,
        },
        "E8FF9F92",
        "movn x8, #65535",
    ));
    insns.push((
        Inst::MovWide {
            op: MoveWideOp::MovN,
            rd: writable_xreg(8),
            imm: MoveWideConst::maybe_from_u64(0x0000_0000_ffff_0000).unwrap(),
            size: OperandSize::Size64,
        },
        "E8FFBF92",
        "movn x8, #65535, LSL #16",
    ));
    insns.push((
        Inst::MovWide {
            op: MoveWideOp::MovN,
            rd: writable_xreg(8),
            imm: MoveWideConst::maybe_from_u64(0x0000_ffff_0000_0000).unwrap(),
            size: OperandSize::Size64,
        },
        "E8FFDF92",
        "movn x8, #65535, LSL #32",
    ));
    insns.push((
        Inst::MovWide {
            op: MoveWideOp::MovN,
            rd: writable_xreg(8),
            imm: MoveWideConst::maybe_from_u64(0xffff_0000_0000_0000).unwrap(),
            size: OperandSize::Size64,
        },
        "E8FFFF92",
        "movn x8, #65535, LSL #48",
    ));
    insns.push((
        Inst::MovWide {
            op: MoveWideOp::MovN,
            rd: writable_xreg(8),
            imm: MoveWideConst::maybe_from_u64(0x0000_0000_0000_ffff).unwrap(),
            size: OperandSize::Size32,
        },
        "E8FF9F12",
        "movn w8, #65535",
    ));

    insns.push((
        Inst::MovK {
            rd: writable_xreg(12),
            rn: xreg(12),
            imm: MoveWideConst::maybe_from_u64(0x0000_0000_0000_0000).unwrap(),
            size: OperandSize::Size64,
        },
        "0C0080F2",
        "movk x12, x12, #0",
    ));
    insns.push((
        Inst::MovK {
            rd: writable_xreg(19),
            rn: xreg(19),
            imm: MoveWideConst::maybe_with_shift(0x0000, 16).unwrap(),
            size: OperandSize::Size64,
        },
        "1300A0F2",
        "movk x19, x19, #0, LSL #16",
    ));
    insns.push((
        Inst::MovK {
            rd: writable_xreg(3),
            rn: xreg(3),
            imm: MoveWideConst::maybe_from_u64(0x0000_0000_0000_ffff).unwrap(),
            size: OperandSize::Size64,
        },
        "E3FF9FF2",
        "movk x3, x3, #65535",
    ));
    insns.push((
        Inst::MovK {
            rd: writable_xreg(8),
            rn: xreg(8),
            imm: MoveWideConst::maybe_from_u64(0x0000_0000_ffff_0000).unwrap(),
            size: OperandSize::Size64,
        },
        "E8FFBFF2",
        "movk x8, x8, #65535, LSL #16",
    ));
    insns.push((
        Inst::MovK {
            rd: writable_xreg(8),
            rn: xreg(8),
            imm: MoveWideConst::maybe_from_u64(0x0000_ffff_0000_0000).unwrap(),
            size: OperandSize::Size64,
        },
        "E8FFDFF2",
        "movk x8, x8, #65535, LSL #32",
    ));
    insns.push((
        Inst::MovK {
            rd: writable_xreg(8),
            rn: xreg(8),
            imm: MoveWideConst::maybe_from_u64(0xffff_0000_0000_0000).unwrap(),
            size: OperandSize::Size64,
        },
        "E8FFFFF2",
        "movk x8, x8, #65535, LSL #48",
    ));

    insns.push((
        Inst::CSel {
            rd: writable_xreg(10),
            rn: xreg(12),
            rm: xreg(14),
            cond: Cond::Hs,
        },
        "8A218E9A",
        "csel x10, x12, x14, hs",
    ));
    insns.push((
        Inst::CSNeg {
            rd: writable_xreg(10),
            rn: xreg(12),
            rm: xreg(14),
            cond: Cond::Hs,
        },
        "8A258EDA",
        "csneg x10, x12, x14, hs",
    ));
    insns.push((
        Inst::CSet {
            rd: writable_xreg(15),
            cond: Cond::Ge,
        },
        "EFB79F9A",
        "cset x15, ge",
    ));
    insns.push((
        Inst::CSetm {
            rd: writable_xreg(0),
            cond: Cond::Eq,
        },
        "E0139FDA",
        "csetm x0, eq",
    ));
    insns.push((
        Inst::CSetm {
            rd: writable_xreg(16),
            cond: Cond::Vs,
        },
        "F0739FDA",
        "csetm x16, vs",
    ));
    insns.push((
        Inst::CCmp {
            size: OperandSize::Size64,
            rn: xreg(22),
            rm: xreg(1),
            nzcv: NZCV::new(false, false, true, true),
            cond: Cond::Eq,
        },
        "C30241FA",
        "ccmp x22, x1, #nzCV, eq",
    ));
    insns.push((
        Inst::CCmp {
            size: OperandSize::Size32,
            rn: xreg(3),
            rm: xreg(28),
            nzcv: NZCV::new(true, true, true, true),
            cond: Cond::Gt,
        },
        "6FC05C7A",
        "ccmp w3, w28, #NZCV, gt",
    ));
    insns.push((
        Inst::CCmpImm {
            size: OperandSize::Size64,
            rn: xreg(22),
            imm: UImm5::maybe_from_u8(5).unwrap(),
            nzcv: NZCV::new(false, false, true, true),
            cond: Cond::Eq,
        },
        "C30A45FA",
        "ccmp x22, #5, #nzCV, eq",
    ));
    insns.push((
        Inst::CCmpImm {
            size: OperandSize::Size32,
            rn: xreg(3),
            imm: UImm5::maybe_from_u8(30).unwrap(),
            nzcv: NZCV::new(true, true, true, true),
            cond: Cond::Gt,
        },
        "6FC85E7A",
        "ccmp w3, #30, #NZCV, gt",
    ));
    insns.push((
        Inst::MovToFpu {
            rd: writable_vreg(31),
            rn: xreg(0),
            size: ScalarSize::Size64,
        },
        "1F00679E",
        "fmov d31, x0",
    ));
    insns.push((
        Inst::MovToFpu {
            rd: writable_vreg(1),
            rn: xreg(28),
            size: ScalarSize::Size32,
        },
        "8103271E",
        "fmov s1, w28",
    ));
    insns.push((
        Inst::FpuMoveFPImm {
            rd: writable_vreg(31),
            imm: ASIMDFPModImm::maybe_from_u64(f64::to_bits(1.0), ScalarSize::Size64).unwrap(),
            size: ScalarSize::Size64,
        },
        "1F106E1E",
        "fmov d31, #1",
    ));
    insns.push((
        Inst::FpuMoveFPImm {
            rd: writable_vreg(1),
            imm: ASIMDFPModImm::maybe_from_u64(f32::to_bits(31.0).into(), ScalarSize::Size32)
                .unwrap(),
            size: ScalarSize::Size32,
        },
        "01F0271E",
        "fmov s1, #31",
    ));
    insns.push((
        Inst::MovToVec {
            rd: writable_vreg(0),
            ri: vreg(0),
            rn: xreg(0),
            idx: 7,
            size: VectorSize::Size8x8,
        },
        "001C0F4E",
        "mov v0.b[7], v0.b[7], w0",
    ));
    insns.push((
        Inst::MovToVec {
            rd: writable_vreg(20),
            ri: vreg(20),
            rn: xreg(21),
            idx: 0,
            size: VectorSize::Size64x2,
        },
        "B41E084E",
        "mov v20.d[0], v20.d[0], x21",
    ));
    insns.push((
        Inst::MovFromVec {
            rd: writable_xreg(3),
            rn: vreg(27),
            idx: 14,
            size: ScalarSize::Size8,
        },
        "633F1D0E",
        "umov w3, v27.b[14]",
    ));
    insns.push((
        Inst::MovFromVec {
            rd: writable_xreg(24),
            rn: vreg(5),
            idx: 3,
            size: ScalarSize::Size16,
        },
        "B83C0E0E",
        "umov w24, v5.h[3]",
    ));
    insns.push((
        Inst::MovFromVec {
            rd: writable_xreg(12),
            rn: vreg(17),
            idx: 1,
            size: ScalarSize::Size32,
        },
        "2C3E0C0E",
        "mov w12, v17.s[1]",
    ));
    insns.push((
        Inst::MovFromVec {
            rd: writable_xreg(21),
            rn: vreg(20),
            idx: 0,
            size: ScalarSize::Size64,
        },
        "953E084E",
        "mov x21, v20.d[0]",
    ));
    insns.push((
        Inst::MovFromVecSigned {
            rd: writable_xreg(0),
            rn: vreg(0),
            idx: 15,
            size: VectorSize::Size8x16,
            scalar_size: OperandSize::Size32,
        },
        "002C1F0E",
        "smov w0, v0.b[15]",
    ));
    insns.push((
        Inst::MovFromVecSigned {
            rd: writable_xreg(12),
            rn: vreg(13),
            idx: 7,
            size: VectorSize::Size8x8,
            scalar_size: OperandSize::Size64,
        },
        "AC2D0F4E",
        "smov x12, v13.b[7]",
    ));
    insns.push((
        Inst::MovFromVecSigned {
            rd: writable_xreg(23),
            rn: vreg(31),
            idx: 7,
            size: VectorSize::Size16x8,
            scalar_size: OperandSize::Size32,
        },
        "F72F1E0E",
        "smov w23, v31.h[7]",
    ));
    insns.push((
        Inst::MovFromVecSigned {
            rd: writable_xreg(24),
            rn: vreg(5),
            idx: 1,
            size: VectorSize::Size32x2,
            scalar_size: OperandSize::Size64,
        },
        "B82C0C4E",
        "smov x24, v5.s[1]",
    ));
    insns.push((
        Inst::MovToNZCV { rn: xreg(13) },
        "0D421BD5",
        "msr nzcv, x13",
    ));
    insns.push((
        Inst::MovFromNZCV {
            rd: writable_xreg(27),
        },
        "1B423BD5",
        "mrs x27, nzcv",
    ));
    insns.push((
        Inst::VecDup {
            rd: writable_vreg(24),
            rn: xreg(8),
            size: VectorSize::Size8x8,
        },
        "180D010E",
        "dup v24.8b, w8",
    ));
    insns.push((
        Inst::VecDup {
            rd: writable_vreg(25),
            rn: xreg(7),
            size: VectorSize::Size8x8,
        },
        "F90C010E",
        "dup v25.8b, w7",
    ));
    insns.push((
        Inst::VecDup {
            rd: writable_vreg(1),
            rn: xreg(22),
            size: VectorSize::Size16x4,
        },
        "C10E020E",
        "dup v1.4h, w22",
    ));
    insns.push((
        Inst::VecDup {
            rd: writable_vreg(2),
            rn: xreg(23),
            size: VectorSize::Size16x8,
        },
        "E20E024E",
        "dup v2.8h, w23",
    ));
    insns.push((
        Inst::VecDup {
            rd: writable_vreg(30),
            rn: xreg(28),
            size: VectorSize::Size32x2,
        },
        "9E0F040E",
        "dup v30.2s, w28",
    ));
    insns.push((
        Inst::VecDup {
            rd: writable_vreg(0),
            rn: xreg(28),
            size: VectorSize::Size32x2,
        },
        "800F040E",
        "dup v0.2s, w28",
    ));
    insns.push((
        Inst::VecDup {
            rd: writable_vreg(31),
            rn: xreg(5),
            size: VectorSize::Size64x2,
        },
        "BF0C084E",
        "dup v31.2d, x5",
    ));
    insns.push((
        Inst::VecDupFromFpu {
            rd: writable_vreg(14),
            rn: vreg(19),
            size: VectorSize::Size32x4,
            lane: 0,
        },
        "6E06044E",
        "dup v14.4s, v19.s[0]",
    ));
    insns.push((
        Inst::VecDupFromFpu {
            rd: writable_vreg(18),
            rn: vreg(10),
            size: VectorSize::Size64x2,
            lane: 0,
        },
        "5205084E",
        "dup v18.2d, v10.d[0]",
    ));
    insns.push((
        Inst::VecDupFPImm {
            rd: writable_vreg(31),
            imm: ASIMDFPModImm::maybe_from_u64(1_f32.to_bits() as u64, ScalarSize::Size32).unwrap(),
            size: VectorSize::Size32x2,
        },
        "1FF6030F",
        "fmov v31.2s, #1",
    ));
    insns.push((
        Inst::VecDupFPImm {
            rd: writable_vreg(0),
            imm: ASIMDFPModImm::maybe_from_u64(2_f64.to_bits(), ScalarSize::Size64).unwrap(),
            size: VectorSize::Size64x2,
        },
        "00F4006F",
        "fmov v0.2d, #2",
    ));
    insns.push((
        Inst::VecDupImm {
            rd: writable_vreg(31),
            imm: ASIMDMovModImm::maybe_from_u64(255, ScalarSize::Size8).unwrap(),
            invert: false,
            size: VectorSize::Size8x16,
        },
        "FFE7074F",
        "movi v31.16b, #255",
    ));
    insns.push((
        Inst::VecDupImm {
            rd: writable_vreg(30),
            imm: ASIMDMovModImm::maybe_from_u64(0, ScalarSize::Size16).unwrap(),
            invert: false,
            size: VectorSize::Size16x8,
        },
        "1E84004F",
        "movi v30.8h, #0",
    ));
    insns.push((
        Inst::VecDupImm {
            rd: writable_vreg(0),
            imm: ASIMDMovModImm::zero(ScalarSize::Size16),
            invert: true,
            size: VectorSize::Size16x4,
        },
        "0084002F",
        "mvni v0.4h, #0",
    ));
    insns.push((
        Inst::VecDupImm {
            rd: writable_vreg(0),
            imm: ASIMDMovModImm::maybe_from_u64(256, ScalarSize::Size16).unwrap(),
            invert: false,
            size: VectorSize::Size16x8,
        },
        "20A4004F",
        "movi v0.8h, #1, LSL #8",
    ));
    insns.push((
        Inst::VecDupImm {
            rd: writable_vreg(8),
            imm: ASIMDMovModImm::maybe_from_u64(2228223, ScalarSize::Size32).unwrap(),
            invert: false,
            size: VectorSize::Size32x4,
        },
        "28D4014F",
        "movi v8.4s, #33, MSL #16",
    ));
    insns.push((
        Inst::VecDupImm {
            rd: writable_vreg(16),
            imm: ASIMDMovModImm::maybe_from_u64(35071, ScalarSize::Size32).unwrap(),
            invert: true,
            size: VectorSize::Size32x2,
        },
        "10C5042F",
        "mvni v16.2s, #136, MSL #8",
    ));
    insns.push((
        Inst::VecDupImm {
            rd: writable_vreg(1),
            imm: ASIMDMovModImm::maybe_from_u64(0, ScalarSize::Size32).unwrap(),
            invert: false,
            size: VectorSize::Size32x2,
        },
        "0104000F",
        "movi v1.2s, #0",
    ));
    insns.push((
        Inst::VecDupImm {
            rd: writable_vreg(24),
            imm: ASIMDMovModImm::maybe_from_u64(1107296256, ScalarSize::Size32).unwrap(),
            invert: false,
            size: VectorSize::Size32x4,
        },
        "5864024F",
        "movi v24.4s, #66, LSL #24",
    ));
    insns.push((
        Inst::VecDupImm {
            rd: writable_vreg(8),
            imm: ASIMDMovModImm::zero(ScalarSize::Size64),
            invert: false,
            size: VectorSize::Size64x2,
        },
        "08E4006F",
        "movi v8.2d, #0",
    ));
    insns.push((
        Inst::VecDupImm {
            rd: writable_vreg(7),
            imm: ASIMDMovModImm::maybe_from_u64(18374687574904995840, ScalarSize::Size64).unwrap(),
            invert: false,
            size: VectorSize::Size64x2,
        },
        "87E6046F",
        "movi v7.2d, #18374687574904995840",
    ));
    insns.push((
        Inst::VecExtend {
            t: VecExtendOp::Sxtl,
            rd: writable_vreg(4),
            rn: vreg(27),
            high_half: false,
            lane_size: ScalarSize::Size16,
        },
        "64A7080F",
        "sxtl v4.8h, v27.8b",
    ));
    insns.push((
        Inst::VecExtend {
            t: VecExtendOp::Sxtl,
            rd: writable_vreg(17),
            rn: vreg(19),
            high_half: true,
            lane_size: ScalarSize::Size32,
        },
        "71A6104F",
        "sxtl2 v17.4s, v19.8h",
    ));
    insns.push((
        Inst::VecExtend {
            t: VecExtendOp::Sxtl,
            rd: writable_vreg(30),
            rn: vreg(6),
            high_half: false,
            lane_size: ScalarSize::Size64,
        },
        "DEA4200F",
        "sxtl v30.2d, v6.2s",
    ));
    insns.push((
        Inst::VecExtend {
            t: VecExtendOp::Uxtl,
            rd: writable_vreg(3),
            rn: vreg(29),
            high_half: true,
            lane_size: ScalarSize::Size16,
        },
        "A3A7086F",
        "uxtl2 v3.8h, v29.16b",
    ));
    insns.push((
        Inst::VecExtend {
            t: VecExtendOp::Uxtl,
            rd: writable_vreg(15),
            rn: vreg(12),
            high_half: false,
            lane_size: ScalarSize::Size32,
        },
        "8FA5102F",
        "uxtl v15.4s, v12.4h",
    ));
    insns.push((
        Inst::VecExtend {
            t: VecExtendOp::Uxtl,
            rd: writable_vreg(28),
            rn: vreg(2),
            high_half: true,
            lane_size: ScalarSize::Size64,
        },
        "5CA4206F",
        "uxtl2 v28.2d, v2.4s",
    ));

    insns.push((
        Inst::VecMovElement {
            rd: writable_vreg(0),
            ri: vreg(0),
            rn: vreg(31),
            dest_idx: 7,
            src_idx: 7,
            size: VectorSize::Size16x8,
        },
        "E0771E6E",
        "mov v0.h[7], v0.h[7], v31.h[7]",
    ));

    insns.push((
        Inst::VecMovElement {
            rd: writable_vreg(31),
            ri: vreg(31),
            rn: vreg(16),
            dest_idx: 1,
            src_idx: 0,
            size: VectorSize::Size32x2,
        },
        "1F060C6E",
        "mov v31.s[1], v31.s[1], v16.s[0]",
    ));

    insns.push((
        Inst::VecRRLong {
            op: VecRRLongOp::Fcvtl16,
            rd: writable_vreg(0),
            rn: vreg(30),
            high_half: false,
        },
        "C07B210E",
        "fcvtl v0.4s, v30.4h",
    ));

    insns.push((
        Inst::VecRRLong {
            op: VecRRLongOp::Fcvtl32,
            rd: writable_vreg(16),
            rn: vreg(1),
            high_half: true,
        },
        "3078614E",
        "fcvtl2 v16.2d, v1.4s",
    ));

    insns.push((
        Inst::VecRRLong {
            op: VecRRLongOp::Shll8,
            rd: writable_vreg(12),
            rn: vreg(5),
            high_half: false,
        },
        "AC38212E",
        "shll v12.8h, v5.8b, #8",
    ));

    insns.push((
        Inst::VecRRLong {
            op: VecRRLongOp::Shll16,
            rd: writable_vreg(9),
            rn: vreg(1),
            high_half: true,
        },
        "2938616E",
        "shll2 v9.4s, v1.8h, #16",
    ));

    insns.push((
        Inst::VecRRLong {
            op: VecRRLongOp::Shll32,
            rd: writable_vreg(1),
            rn: vreg(10),
            high_half: false,
        },
        "4139A12E",
        "shll v1.2d, v10.2s, #32",
    ));

    insns.push((
        Inst::VecRRNarrowLow {
            op: VecRRNarrowOp::Xtn,
            rd: writable_vreg(25),
            rn: vreg(17),
            lane_size: ScalarSize::Size8,
        },
        "392A210E",
        "xtn v25.8b, v17.8h",
    ));

    insns.push((
        Inst::VecRRNarrowHigh {
            op: VecRRNarrowOp::Xtn,
            rd: writable_vreg(3),
            ri: vreg(3),
            rn: vreg(10),
            lane_size: ScalarSize::Size16,
        },
        "4329614E",
        "xtn2 v3.8h, v3.8h, v10.4s",
    ));

    insns.push((
        Inst::VecRRNarrowLow {
            op: VecRRNarrowOp::Xtn,
            rd: writable_vreg(22),
            rn: vreg(8),
            lane_size: ScalarSize::Size32,
        },
        "1629A10E",
        "xtn v22.2s, v8.2d",
    ));

    insns.push((
        Inst::VecRRNarrowHigh {
            op: VecRRNarrowOp::Sqxtn,
            rd: writable_vreg(7),
            ri: vreg(7),
            rn: vreg(22),
            lane_size: ScalarSize::Size8,
        },
        "C74A214E",
        "sqxtn2 v7.16b, v7.16b, v22.8h",
    ));

    insns.push((
        Inst::VecRRNarrowHigh {
            op: VecRRNarrowOp::Sqxtn,
            rd: writable_vreg(31),
            ri: vreg(31),
            rn: vreg(0),
            lane_size: ScalarSize::Size16,
        },
        "1F48614E",
        "sqxtn2 v31.8h, v31.8h, v0.4s",
    ));

    insns.push((
        Inst::VecRRNarrowLow {
            op: VecRRNarrowOp::Sqxtn,
            rd: writable_vreg(14),
            rn: vreg(20),
            lane_size: ScalarSize::Size32,
        },
        "8E4AA10E",
        "sqxtn v14.2s, v20.2d",
    ));

    insns.push((
        Inst::VecRRNarrowLow {
            op: VecRRNarrowOp::Sqxtun,
            rd: writable_vreg(16),
            rn: vreg(23),
            lane_size: ScalarSize::Size8,
        },
        "F02A212E",
        "sqxtun v16.8b, v23.8h",
    ));

    insns.push((
        Inst::VecRRNarrowHigh {
            op: VecRRNarrowOp::Sqxtun,
            rd: writable_vreg(28),
            ri: vreg(28),
            rn: vreg(9),
            lane_size: ScalarSize::Size16,
        },
        "3C29616E",
        "sqxtun2 v28.8h, v28.8h, v9.4s",
    ));

    insns.push((
        Inst::VecRRNarrowLow {
            op: VecRRNarrowOp::Sqxtun,
            rd: writable_vreg(15),
            rn: vreg(15),
            lane_size: ScalarSize::Size32,
        },
        "EF29A12E",
        "sqxtun v15.2s, v15.2d",
    ));

    insns.push((
        Inst::VecRRNarrowHigh {
            op: VecRRNarrowOp::Uqxtn,
            rd: writable_vreg(21),
            ri: vreg(21),
            rn: vreg(4),
            lane_size: ScalarSize::Size8,
        },
        "9548216E",
        "uqxtn2 v21.16b, v21.16b, v4.8h",
    ));

    insns.push((
        Inst::VecRRNarrowLow {
            op: VecRRNarrowOp::Uqxtn,
            rd: writable_vreg(31),
            rn: vreg(31),
            lane_size: ScalarSize::Size16,
        },
        "FF4B612E",
        "uqxtn v31.4h, v31.4s",
    ));

    insns.push((
        Inst::VecRRNarrowHigh {
            op: VecRRNarrowOp::Uqxtn,
            rd: writable_vreg(11),
            ri: vreg(11),
            rn: vreg(12),
            lane_size: ScalarSize::Size32,
        },
        "8B49A16E",
        "uqxtn2 v11.4s, v11.4s, v12.2d",
    ));

    insns.push((
        Inst::VecRRNarrowLow {
            op: VecRRNarrowOp::Fcvtn,
            rd: writable_vreg(0),
            rn: vreg(0),
            lane_size: ScalarSize::Size16,
        },
        "0068210E",
        "fcvtn v0.4h, v0.4s",
    ));

    insns.push((
        Inst::VecRRNarrowLow {
            op: VecRRNarrowOp::Fcvtn,
            rd: writable_vreg(2),
            rn: vreg(7),
            lane_size: ScalarSize::Size32,
        },
        "E268610E",
        "fcvtn v2.2s, v7.2d",
    ));

    insns.push((
        Inst::VecRRNarrowHigh {
            op: VecRRNarrowOp::Fcvtn,
            rd: writable_vreg(31),
            ri: vreg(31),
            rn: vreg(30),
            lane_size: ScalarSize::Size32,
        },
        "DF6B614E",
        "fcvtn2 v31.4s, v31.4s, v30.2d",
    ));

    insns.push((
        Inst::VecRRPair {
            op: VecPairOp::Addp,
            rd: writable_vreg(0),
            rn: vreg(30),
        },
        "C0BBF15E",
        "addp d0, v30.2d",
    ));

    insns.push((
        Inst::VecRRPairLong {
            op: VecRRPairLongOp::Uaddlp8,
            rd: writable_vreg(0),
            rn: vreg(1),
        },
        "2028206E",
        "uaddlp v0.8h, v1.16b",
    ));

    insns.push((
        Inst::VecRRPairLong {
            op: VecRRPairLongOp::Saddlp8,
            rd: writable_vreg(3),
            rn: vreg(11),
        },
        "6329204E",
        "saddlp v3.8h, v11.16b",
    ));

    insns.push((
        Inst::VecRRPairLong {
            op: VecRRPairLongOp::Uaddlp16,
            rd: writable_vreg(14),
            rn: vreg(23),
        },
        "EE2A606E",
        "uaddlp v14.4s, v23.8h",
    ));

    insns.push((
        Inst::VecRRPairLong {
            op: VecRRPairLongOp::Saddlp16,
            rd: writable_vreg(29),
            rn: vreg(0),
        },
        "1D28604E",
        "saddlp v29.4s, v0.8h",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Sqadd,
            rd: writable_vreg(1),
            rn: vreg(2),
            rm: vreg(8),
            size: VectorSize::Size8x16,
        },
        "410C284E",
        "sqadd v1.16b, v2.16b, v8.16b",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Sqadd,
            rd: writable_vreg(1),
            rn: vreg(12),
            rm: vreg(28),
            size: VectorSize::Size16x8,
        },
        "810D7C4E",
        "sqadd v1.8h, v12.8h, v28.8h",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Sqadd,
            rd: writable_vreg(12),
            rn: vreg(2),
            rm: vreg(6),
            size: VectorSize::Size32x4,
        },
        "4C0CA64E",
        "sqadd v12.4s, v2.4s, v6.4s",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Sqadd,
            rd: writable_vreg(20),
            rn: vreg(7),
            rm: vreg(13),
            size: VectorSize::Size64x2,
        },
        "F40CED4E",
        "sqadd v20.2d, v7.2d, v13.2d",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Sqsub,
            rd: writable_vreg(1),
            rn: vreg(2),
            rm: vreg(8),
            size: VectorSize::Size8x16,
        },
        "412C284E",
        "sqsub v1.16b, v2.16b, v8.16b",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Sqsub,
            rd: writable_vreg(1),
            rn: vreg(12),
            rm: vreg(28),
            size: VectorSize::Size16x8,
        },
        "812D7C4E",
        "sqsub v1.8h, v12.8h, v28.8h",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Sqsub,
            rd: writable_vreg(12),
            rn: vreg(2),
            rm: vreg(6),
            size: VectorSize::Size32x4,
        },
        "4C2CA64E",
        "sqsub v12.4s, v2.4s, v6.4s",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Sqsub,
            rd: writable_vreg(20),
            rn: vreg(7),
            rm: vreg(13),
            size: VectorSize::Size64x2,
        },
        "F42CED4E",
        "sqsub v20.2d, v7.2d, v13.2d",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Uqadd,
            rd: writable_vreg(1),
            rn: vreg(2),
            rm: vreg(8),
            size: VectorSize::Size8x16,
        },
        "410C286E",
        "uqadd v1.16b, v2.16b, v8.16b",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Uqadd,
            rd: writable_vreg(1),
            rn: vreg(12),
            rm: vreg(28),
            size: VectorSize::Size16x8,
        },
        "810D7C6E",
        "uqadd v1.8h, v12.8h, v28.8h",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Uqadd,
            rd: writable_vreg(12),
            rn: vreg(2),
            rm: vreg(6),
            size: VectorSize::Size32x4,
        },
        "4C0CA66E",
        "uqadd v12.4s, v2.4s, v6.4s",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Uqadd,
            rd: writable_vreg(20),
            rn: vreg(7),
            rm: vreg(13),
            size: VectorSize::Size64x2,
        },
        "F40CED6E",
        "uqadd v20.2d, v7.2d, v13.2d",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Uqsub,
            rd: writable_vreg(1),
            rn: vreg(2),
            rm: vreg(8),
            size: VectorSize::Size8x16,
        },
        "412C286E",
        "uqsub v1.16b, v2.16b, v8.16b",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Uqsub,
            rd: writable_vreg(1),
            rn: vreg(12),
            rm: vreg(28),
            size: VectorSize::Size16x8,
        },
        "812D7C6E",
        "uqsub v1.8h, v12.8h, v28.8h",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Uqsub,
            rd: writable_vreg(12),
            rn: vreg(2),
            rm: vreg(6),
            size: VectorSize::Size32x4,
        },
        "4C2CA66E",
        "uqsub v12.4s, v2.4s, v6.4s",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Uqsub,
            rd: writable_vreg(20),
            rn: vreg(7),
            rm: vreg(13),
            size: VectorSize::Size64x2,
        },
        "F42CED6E",
        "uqsub v20.2d, v7.2d, v13.2d",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Cmeq,
            rd: writable_vreg(3),
            rn: vreg(23),
            rm: vreg(24),
            size: VectorSize::Size8x16,
        },
        "E38E386E",
        "cmeq v3.16b, v23.16b, v24.16b",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Cmgt,
            rd: writable_vreg(3),
            rn: vreg(23),
            rm: vreg(24),
            size: VectorSize::Size8x16,
        },
        "E336384E",
        "cmgt v3.16b, v23.16b, v24.16b",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Cmge,
            rd: writable_vreg(23),
            rn: vreg(9),
            rm: vreg(12),
            size: VectorSize::Size8x16,
        },
        "373D2C4E",
        "cmge v23.16b, v9.16b, v12.16b",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Cmhi,
            rd: writable_vreg(5),
            rn: vreg(1),
            rm: vreg(1),
            size: VectorSize::Size8x16,
        },
        "2534216E",
        "cmhi v5.16b, v1.16b, v1.16b",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Cmhs,
            rd: writable_vreg(8),
            rn: vreg(2),
            rm: vreg(15),
            size: VectorSize::Size8x16,
        },
        "483C2F6E",
        "cmhs v8.16b, v2.16b, v15.16b",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Cmeq,
            rd: writable_vreg(3),
            rn: vreg(23),
            rm: vreg(24),
            size: VectorSize::Size16x8,
        },
        "E38E786E",
        "cmeq v3.8h, v23.8h, v24.8h",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Cmgt,
            rd: writable_vreg(3),
            rn: vreg(23),
            rm: vreg(24),
            size: VectorSize::Size16x8,
        },
        "E336784E",
        "cmgt v3.8h, v23.8h, v24.8h",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Cmge,
            rd: writable_vreg(23),
            rn: vreg(9),
            rm: vreg(12),
            size: VectorSize::Size16x8,
        },
        "373D6C4E",
        "cmge v23.8h, v9.8h, v12.8h",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Cmhi,
            rd: writable_vreg(5),
            rn: vreg(1),
            rm: vreg(1),
            size: VectorSize::Size16x8,
        },
        "2534616E",
        "cmhi v5.8h, v1.8h, v1.8h",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Cmhs,
            rd: writable_vreg(8),
            rn: vreg(2),
            rm: vreg(15),
            size: VectorSize::Size16x8,
        },
        "483C6F6E",
        "cmhs v8.8h, v2.8h, v15.8h",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Cmeq,
            rd: writable_vreg(3),
            rn: vreg(23),
            rm: vreg(24),
            size: VectorSize::Size32x4,
        },
        "E38EB86E",
        "cmeq v3.4s, v23.4s, v24.4s",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Cmgt,
            rd: writable_vreg(3),
            rn: vreg(23),
            rm: vreg(24),
            size: VectorSize::Size32x4,
        },
        "E336B84E",
        "cmgt v3.4s, v23.4s, v24.4s",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Cmge,
            rd: writable_vreg(23),
            rn: vreg(9),
            rm: vreg(12),
            size: VectorSize::Size32x4,
        },
        "373DAC4E",
        "cmge v23.4s, v9.4s, v12.4s",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Cmhi,
            rd: writable_vreg(5),
            rn: vreg(1),
            rm: vreg(1),
            size: VectorSize::Size32x4,
        },
        "2534A16E",
        "cmhi v5.4s, v1.4s, v1.4s",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Cmhs,
            rd: writable_vreg(8),
            rn: vreg(2),
            rm: vreg(15),
            size: VectorSize::Size32x4,
        },
        "483CAF6E",
        "cmhs v8.4s, v2.4s, v15.4s",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Fcmeq,
            rd: writable_vreg(28),
            rn: vreg(12),
            rm: vreg(4),
            size: VectorSize::Size32x2,
        },
        "9CE5240E",
        "fcmeq v28.2s, v12.2s, v4.2s",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Fcmgt,
            rd: writable_vreg(3),
            rn: vreg(16),
            rm: vreg(31),
            size: VectorSize::Size64x2,
        },
        "03E6FF6E",
        "fcmgt v3.2d, v16.2d, v31.2d",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Fcmge,
            rd: writable_vreg(18),
            rn: vreg(23),
            rm: vreg(0),
            size: VectorSize::Size64x2,
        },
        "F2E6606E",
        "fcmge v18.2d, v23.2d, v0.2d",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::And,
            rd: writable_vreg(20),
            rn: vreg(19),
            rm: vreg(18),
            size: VectorSize::Size32x4,
        },
        "741E324E",
        "and v20.16b, v19.16b, v18.16b",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Bic,
            rd: writable_vreg(8),
            rn: vreg(11),
            rm: vreg(1),
            size: VectorSize::Size8x16,
        },
        "681D614E",
        "bic v8.16b, v11.16b, v1.16b",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Orr,
            rd: writable_vreg(15),
            rn: vreg(2),
            rm: vreg(12),
            size: VectorSize::Size16x8,
        },
        "4F1CAC4E",
        "orr v15.16b, v2.16b, v12.16b",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Eor,
            rd: writable_vreg(18),
            rn: vreg(3),
            rm: vreg(22),
            size: VectorSize::Size8x16,
        },
        "721C366E",
        "eor v18.16b, v3.16b, v22.16b",
    ));

    insns.push((
        Inst::VecRRRMod {
            alu_op: VecALUModOp::Bsl,
            rd: writable_vreg(8),
            ri: vreg(8),
            rn: vreg(9),
            rm: vreg(1),
            size: VectorSize::Size8x16,
        },
        "281D616E",
        "bsl v8.16b, v8.16b, v9.16b, v1.16b",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Umaxp,
            rd: writable_vreg(8),
            rn: vreg(12),
            rm: vreg(1),
            size: VectorSize::Size8x16,
        },
        "88A5216E",
        "umaxp v8.16b, v12.16b, v1.16b",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Umaxp,
            rd: writable_vreg(1),
            rn: vreg(6),
            rm: vreg(1),
            size: VectorSize::Size16x8,
        },
        "C1A4616E",
        "umaxp v1.8h, v6.8h, v1.8h",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Umaxp,
            rd: writable_vreg(1),
            rn: vreg(20),
            rm: vreg(16),
            size: VectorSize::Size32x4,
        },
        "81A6B06E",
        "umaxp v1.4s, v20.4s, v16.4s",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Add,
            rd: writable_vreg(5),
            rn: vreg(1),
            rm: vreg(1),
            size: VectorSize::Size8x16,
        },
        "2584214E",
        "add v5.16b, v1.16b, v1.16b",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Add,
            rd: writable_vreg(7),
            rn: vreg(13),
            rm: vreg(2),
            size: VectorSize::Size16x8,
        },
        "A785624E",
        "add v7.8h, v13.8h, v2.8h",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Add,
            rd: writable_vreg(18),
            rn: vreg(9),
            rm: vreg(6),
            size: VectorSize::Size32x4,
        },
        "3285A64E",
        "add v18.4s, v9.4s, v6.4s",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Add,
            rd: writable_vreg(1),
            rn: vreg(3),
            rm: vreg(2),
            size: VectorSize::Size64x2,
        },
        "6184E24E",
        "add v1.2d, v3.2d, v2.2d",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Sub,
            rd: writable_vreg(5),
            rn: vreg(1),
            rm: vreg(1),
            size: VectorSize::Size8x16,
        },
        "2584216E",
        "sub v5.16b, v1.16b, v1.16b",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Sub,
            rd: writable_vreg(7),
            rn: vreg(13),
            rm: vreg(2),
            size: VectorSize::Size16x8,
        },
        "A785626E",
        "sub v7.8h, v13.8h, v2.8h",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Sub,
            rd: writable_vreg(18),
            rn: vreg(9),
            rm: vreg(6),
            size: VectorSize::Size32x4,
        },
        "3285A66E",
        "sub v18.4s, v9.4s, v6.4s",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Sub,
            rd: writable_vreg(18),
            rn: vreg(0),
            rm: vreg(8),
            size: VectorSize::Size64x2,
        },
        "1284E86E",
        "sub v18.2d, v0.2d, v8.2d",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Mul,
            rd: writable_vreg(25),
            rn: vreg(9),
            rm: vreg(8),
            size: VectorSize::Size8x16,
        },
        "399D284E",
        "mul v25.16b, v9.16b, v8.16b",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Mul,
            rd: writable_vreg(30),
            rn: vreg(30),
            rm: vreg(12),
            size: VectorSize::Size16x8,
        },
        "DE9F6C4E",
        "mul v30.8h, v30.8h, v12.8h",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Mul,
            rd: writable_vreg(18),
            rn: vreg(18),
            rm: vreg(18),
            size: VectorSize::Size32x4,
        },
        "529EB24E",
        "mul v18.4s, v18.4s, v18.4s",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Ushl,
            rd: writable_vreg(18),
            rn: vreg(18),
            rm: vreg(18),
            size: VectorSize::Size8x16,
        },
        "5246326E",
        "ushl v18.16b, v18.16b, v18.16b",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Ushl,
            rd: writable_vreg(18),
            rn: vreg(18),
            rm: vreg(18),
            size: VectorSize::Size16x8,
        },
        "5246726E",
        "ushl v18.8h, v18.8h, v18.8h",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Ushl,
            rd: writable_vreg(18),
            rn: vreg(1),
            rm: vreg(21),
            size: VectorSize::Size32x4,
        },
        "3244B56E",
        "ushl v18.4s, v1.4s, v21.4s",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Ushl,
            rd: writable_vreg(5),
            rn: vreg(7),
            rm: vreg(19),
            size: VectorSize::Size64x2,
        },
        "E544F36E",
        "ushl v5.2d, v7.2d, v19.2d",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Sshl,
            rd: writable_vreg(18),
            rn: vreg(18),
            rm: vreg(18),
            size: VectorSize::Size8x16,
        },
        "5246324E",
        "sshl v18.16b, v18.16b, v18.16b",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Sshl,
            rd: writable_vreg(30),
            rn: vreg(1),
            rm: vreg(29),
            size: VectorSize::Size16x8,
        },
        "3E447D4E",
        "sshl v30.8h, v1.8h, v29.8h",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Sshl,
            rd: writable_vreg(8),
            rn: vreg(22),
            rm: vreg(21),
            size: VectorSize::Size32x4,
        },
        "C846B54E",
        "sshl v8.4s, v22.4s, v21.4s",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Sshl,
            rd: writable_vreg(8),
            rn: vreg(22),
            rm: vreg(2),
            size: VectorSize::Size64x2,
        },
        "C846E24E",
        "sshl v8.2d, v22.2d, v2.2d",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Umin,
            rd: writable_vreg(0),
            rn: vreg(11),
            rm: vreg(2),
            size: VectorSize::Size8x8,
        },
        "606D222E",
        "umin v0.8b, v11.8b, v2.8b",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Umin,
            rd: writable_vreg(1),
            rn: vreg(12),
            rm: vreg(3),
            size: VectorSize::Size8x16,
        },
        "816D236E",
        "umin v1.16b, v12.16b, v3.16b",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Umin,
            rd: writable_vreg(29),
            rn: vreg(19),
            rm: vreg(9),
            size: VectorSize::Size16x4,
        },
        "7D6E692E",
        "umin v29.4h, v19.4h, v9.4h",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Umin,
            rd: writable_vreg(30),
            rn: vreg(20),
            rm: vreg(10),
            size: VectorSize::Size16x8,
        },
        "9E6E6A6E",
        "umin v30.8h, v20.8h, v10.8h",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Umin,
            rd: writable_vreg(7),
            rn: vreg(21),
            rm: vreg(20),
            size: VectorSize::Size32x2,
        },
        "A76EB42E",
        "umin v7.2s, v21.2s, v20.2s",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Umin,
            rd: writable_vreg(8),
            rn: vreg(22),
            rm: vreg(21),
            size: VectorSize::Size32x4,
        },
        "C86EB56E",
        "umin v8.4s, v22.4s, v21.4s",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Smin,
            rd: writable_vreg(2),
            rn: vreg(13),
            rm: vreg(4),
            size: VectorSize::Size8x8,
        },
        "A26D240E",
        "smin v2.8b, v13.8b, v4.8b",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Smin,
            rd: writable_vreg(1),
            rn: vreg(12),
            rm: vreg(3),
            size: VectorSize::Size8x16,
        },
        "816D234E",
        "smin v1.16b, v12.16b, v3.16b",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Smin,
            rd: writable_vreg(3),
            rn: vreg(2),
            rm: vreg(1),
            size: VectorSize::Size16x4,
        },
        "436C610E",
        "smin v3.4h, v2.4h, v1.4h",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Smin,
            rd: writable_vreg(30),
            rn: vreg(20),
            rm: vreg(10),
            size: VectorSize::Size16x8,
        },
        "9E6E6A4E",
        "smin v30.8h, v20.8h, v10.8h",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Smin,
            rd: writable_vreg(9),
            rn: vreg(22),
            rm: vreg(20),
            size: VectorSize::Size32x2,
        },
        "C96EB40E",
        "smin v9.2s, v22.2s, v20.2s",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Smin,
            rd: writable_vreg(8),
            rn: vreg(22),
            rm: vreg(21),
            size: VectorSize::Size32x4,
        },
        "C86EB54E",
        "smin v8.4s, v22.4s, v21.4s",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Umax,
            rd: writable_vreg(6),
            rn: vreg(9),
            rm: vreg(8),
            size: VectorSize::Size8x8,
        },
        "2665282E",
        "umax v6.8b, v9.8b, v8.8b",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Umax,
            rd: writable_vreg(5),
            rn: vreg(15),
            rm: vreg(8),
            size: VectorSize::Size8x16,
        },
        "E565286E",
        "umax v5.16b, v15.16b, v8.16b",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Umax,
            rd: writable_vreg(12),
            rn: vreg(14),
            rm: vreg(3),
            size: VectorSize::Size16x4,
        },
        "CC65632E",
        "umax v12.4h, v14.4h, v3.4h",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Umax,
            rd: writable_vreg(11),
            rn: vreg(13),
            rm: vreg(2),
            size: VectorSize::Size16x8,
        },
        "AB65626E",
        "umax v11.8h, v13.8h, v2.8h",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Umax,
            rd: writable_vreg(9),
            rn: vreg(13),
            rm: vreg(15),
            size: VectorSize::Size32x2,
        },
        "A965AF2E",
        "umax v9.2s, v13.2s, v15.2s",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Umax,
            rd: writable_vreg(8),
            rn: vreg(12),
            rm: vreg(14),
            size: VectorSize::Size32x4,
        },
        "8865AE6E",
        "umax v8.4s, v12.4s, v14.4s",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Smax,
            rd: writable_vreg(7),
            rn: vreg(8),
            rm: vreg(9),
            size: VectorSize::Size8x8,
        },
        "0765290E",
        "smax v7.8b, v8.8b, v9.8b",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Smax,
            rd: writable_vreg(6),
            rn: vreg(9),
            rm: vreg(8),
            size: VectorSize::Size8x16,
        },
        "2665284E",
        "smax v6.16b, v9.16b, v8.16b",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Smax,
            rd: writable_vreg(11),
            rn: vreg(12),
            rm: vreg(13),
            size: VectorSize::Size16x4,
        },
        "8B656D0E",
        "smax v11.4h, v12.4h, v13.4h",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Smax,
            rd: writable_vreg(11),
            rn: vreg(13),
            rm: vreg(2),
            size: VectorSize::Size16x8,
        },
        "AB65624E",
        "smax v11.8h, v13.8h, v2.8h",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Smax,
            rd: writable_vreg(14),
            rn: vreg(16),
            rm: vreg(18),
            size: VectorSize::Size32x2,
        },
        "0E66B20E",
        "smax v14.2s, v16.2s, v18.2s",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Smax,
            rd: writable_vreg(8),
            rn: vreg(12),
            rm: vreg(14),
            size: VectorSize::Size32x4,
        },
        "8865AE4E",
        "smax v8.4s, v12.4s, v14.4s",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Urhadd,
            rd: writable_vreg(8),
            rn: vreg(1),
            rm: vreg(3),
            size: VectorSize::Size8x8,
        },
        "2814232E",
        "urhadd v8.8b, v1.8b, v3.8b",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Urhadd,
            rd: writable_vreg(8),
            rn: vreg(1),
            rm: vreg(3),
            size: VectorSize::Size8x16,
        },
        "2814236E",
        "urhadd v8.16b, v1.16b, v3.16b",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Urhadd,
            rd: writable_vreg(2),
            rn: vreg(13),
            rm: vreg(6),
            size: VectorSize::Size16x4,
        },
        "A215662E",
        "urhadd v2.4h, v13.4h, v6.4h",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Urhadd,
            rd: writable_vreg(2),
            rn: vreg(13),
            rm: vreg(6),
            size: VectorSize::Size16x8,
        },
        "A215666E",
        "urhadd v2.8h, v13.8h, v6.8h",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Urhadd,
            rd: writable_vreg(8),
            rn: vreg(12),
            rm: vreg(14),
            size: VectorSize::Size32x2,
        },
        "8815AE2E",
        "urhadd v8.2s, v12.2s, v14.2s",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Urhadd,
            rd: writable_vreg(8),
            rn: vreg(12),
            rm: vreg(14),
            size: VectorSize::Size32x4,
        },
        "8815AE6E",
        "urhadd v8.4s, v12.4s, v14.4s",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Fadd,
            rd: writable_vreg(31),
            rn: vreg(0),
            rm: vreg(16),
            size: VectorSize::Size32x4,
        },
        "1FD4304E",
        "fadd v31.4s, v0.4s, v16.4s",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Fsub,
            rd: writable_vreg(8),
            rn: vreg(7),
            rm: vreg(15),
            size: VectorSize::Size64x2,
        },
        "E8D4EF4E",
        "fsub v8.2d, v7.2d, v15.2d",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Fdiv,
            rd: writable_vreg(1),
            rn: vreg(3),
            rm: vreg(4),
            size: VectorSize::Size32x4,
        },
        "61FC246E",
        "fdiv v1.4s, v3.4s, v4.4s",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Fmax,
            rd: writable_vreg(31),
            rn: vreg(16),
            rm: vreg(0),
            size: VectorSize::Size64x2,
        },
        "1FF6604E",
        "fmax v31.2d, v16.2d, v0.2d",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Fmin,
            rd: writable_vreg(5),
            rn: vreg(19),
            rm: vreg(26),
            size: VectorSize::Size32x4,
        },
        "65F6BA4E",
        "fmin v5.4s, v19.4s, v26.4s",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Fmul,
            rd: writable_vreg(2),
            rn: vreg(0),
            rm: vreg(5),
            size: VectorSize::Size64x2,
        },
        "02DC656E",
        "fmul v2.2d, v0.2d, v5.2d",
    ));

    insns.push((
        Inst::VecRRRMod {
            alu_op: VecALUModOp::Fmla,
            rd: writable_vreg(2),
            ri: vreg(2),
            rn: vreg(0),
            rm: vreg(5),
            size: VectorSize::Size32x2,
        },
        "02CC250E",
        "fmla v2.2s, v2.2s, v0.2s, v5.2s",
    ));

    insns.push((
        Inst::VecRRRMod {
            alu_op: VecALUModOp::Fmla,
            rd: writable_vreg(2),
            ri: vreg(2),
            rn: vreg(0),
            rm: vreg(5),
            size: VectorSize::Size32x4,
        },
        "02CC254E",
        "fmla v2.4s, v2.4s, v0.4s, v5.4s",
    ));

    insns.push((
        Inst::VecRRRMod {
            alu_op: VecALUModOp::Fmla,
            rd: writable_vreg(2),
            ri: vreg(2),
            rn: vreg(0),
            rm: vreg(5),
            size: VectorSize::Size64x2,
        },
        "02CC654E",
        "fmla v2.2d, v2.2d, v0.2d, v5.2d",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Addp,
            rd: writable_vreg(16),
            rn: vreg(12),
            rm: vreg(1),
            size: VectorSize::Size8x8,
        },
        "90BD210E",
        "addp v16.8b, v12.8b, v1.8b",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Addp,
            rd: writable_vreg(16),
            rn: vreg(12),
            rm: vreg(1),
            size: VectorSize::Size8x16,
        },
        "90BD214E",
        "addp v16.16b, v12.16b, v1.16b",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Addp,
            rd: writable_vreg(8),
            rn: vreg(12),
            rm: vreg(14),
            size: VectorSize::Size32x4,
        },
        "88BDAE4E",
        "addp v8.4s, v12.4s, v14.4s",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Addp,
            rd: writable_vreg(8),
            rn: vreg(12),
            rm: vreg(14),
            size: VectorSize::Size32x2,
        },
        "88BDAE0E",
        "addp v8.2s, v12.2s, v14.2s",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Zip1,
            rd: writable_vreg(16),
            rn: vreg(12),
            rm: vreg(1),
            size: VectorSize::Size8x16,
        },
        "9039014E",
        "zip1 v16.16b, v12.16b, v1.16b",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Zip1,
            rd: writable_vreg(2),
            rn: vreg(13),
            rm: vreg(6),
            size: VectorSize::Size16x8,
        },
        "A239464E",
        "zip1 v2.8h, v13.8h, v6.8h",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Zip1,
            rd: writable_vreg(8),
            rn: vreg(12),
            rm: vreg(14),
            size: VectorSize::Size32x4,
        },
        "88398E4E",
        "zip1 v8.4s, v12.4s, v14.4s",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Zip1,
            rd: writable_vreg(9),
            rn: vreg(20),
            rm: vreg(17),
            size: VectorSize::Size64x2,
        },
        "893AD14E",
        "zip1 v9.2d, v20.2d, v17.2d",
    ));

    insns.push((
        Inst::VecRRRLong {
            alu_op: VecRRRLongOp::Smull8,
            rd: writable_vreg(16),
            rn: vreg(12),
            rm: vreg(1),
            high_half: false,
        },
        "90C1210E",
        "smull v16.8h, v12.8b, v1.8b",
    ));

    insns.push((
        Inst::VecRRRLong {
            alu_op: VecRRRLongOp::Umull8,
            rd: writable_vreg(15),
            rn: vreg(11),
            rm: vreg(2),
            high_half: false,
        },
        "6FC1222E",
        "umull v15.8h, v11.8b, v2.8b",
    ));

    insns.push((
        Inst::VecRRRLongMod {
            alu_op: VecRRRLongModOp::Umlal8,
            rd: writable_vreg(4),
            ri: vreg(4),
            rn: vreg(8),
            rm: vreg(16),
            high_half: false,
        },
        "0481302E",
        "umlal v4.8h, v4.8h, v8.8b, v16.8b",
    ));

    insns.push((
        Inst::VecRRRLong {
            alu_op: VecRRRLongOp::Smull16,
            rd: writable_vreg(2),
            rn: vreg(13),
            rm: vreg(6),
            high_half: false,
        },
        "A2C1660E",
        "smull v2.4s, v13.4h, v6.4h",
    ));

    insns.push((
        Inst::VecRRRLong {
            alu_op: VecRRRLongOp::Umull16,
            rd: writable_vreg(3),
            rn: vreg(14),
            rm: vreg(7),
            high_half: false,
        },
        "C3C1672E",
        "umull v3.4s, v14.4h, v7.4h",
    ));

    insns.push((
        Inst::VecRRRLongMod {
            alu_op: VecRRRLongModOp::Umlal16,
            rd: writable_vreg(7),
            ri: vreg(7),
            rn: vreg(14),
            rm: vreg(21),
            high_half: false,
        },
        "C781752E",
        "umlal v7.4s, v7.4s, v14.4h, v21.4h",
    ));

    insns.push((
        Inst::VecRRRLong {
            alu_op: VecRRRLongOp::Smull32,
            rd: writable_vreg(8),
            rn: vreg(12),
            rm: vreg(14),
            high_half: false,
        },
        "88C1AE0E",
        "smull v8.2d, v12.2s, v14.2s",
    ));

    insns.push((
        Inst::VecRRRLong {
            alu_op: VecRRRLongOp::Umull32,
            rd: writable_vreg(9),
            rn: vreg(5),
            rm: vreg(6),
            high_half: false,
        },
        "A9C0A62E",
        "umull v9.2d, v5.2s, v6.2s",
    ));

    insns.push((
        Inst::VecRRRLongMod {
            alu_op: VecRRRLongModOp::Umlal32,
            rd: writable_vreg(9),
            ri: vreg(9),
            rn: vreg(20),
            rm: vreg(17),
            high_half: false,
        },
        "8982B12E",
        "umlal v9.2d, v9.2d, v20.2s, v17.2s",
    ));

    insns.push((
        Inst::VecRRRLong {
            alu_op: VecRRRLongOp::Smull8,
            rd: writable_vreg(16),
            rn: vreg(12),
            rm: vreg(1),
            high_half: true,
        },
        "90C1214E",
        "smull2 v16.8h, v12.16b, v1.16b",
    ));

    insns.push((
        Inst::VecRRRLong {
            alu_op: VecRRRLongOp::Umull8,
            rd: writable_vreg(29),
            rn: vreg(22),
            rm: vreg(10),
            high_half: true,
        },
        "DDC22A6E",
        "umull2 v29.8h, v22.16b, v10.16b",
    ));

    insns.push((
        Inst::VecRRRLongMod {
            alu_op: VecRRRLongModOp::Umlal8,
            rd: writable_vreg(1),
            ri: vreg(1),
            rn: vreg(5),
            rm: vreg(15),
            high_half: true,
        },
        "A1802F6E",
        "umlal2 v1.8h, v1.8h, v5.16b, v15.16b",
    ));

    insns.push((
        Inst::VecRRRLong {
            alu_op: VecRRRLongOp::Smull16,
            rd: writable_vreg(2),
            rn: vreg(13),
            rm: vreg(6),
            high_half: true,
        },
        "A2C1664E",
        "smull2 v2.4s, v13.8h, v6.8h",
    ));

    insns.push((
        Inst::VecRRRLong {
            alu_op: VecRRRLongOp::Umull16,
            rd: writable_vreg(19),
            rn: vreg(18),
            rm: vreg(17),
            high_half: true,
        },
        "53C2716E",
        "umull2 v19.4s, v18.8h, v17.8h",
    ));

    insns.push((
        Inst::VecRRRLongMod {
            alu_op: VecRRRLongModOp::Umlal16,
            rd: writable_vreg(11),
            ri: vreg(11),
            rn: vreg(10),
            rm: vreg(12),
            high_half: true,
        },
        "4B816C6E",
        "umlal2 v11.4s, v11.4s, v10.8h, v12.8h",
    ));

    insns.push((
        Inst::VecRRRLong {
            alu_op: VecRRRLongOp::Smull32,
            rd: writable_vreg(8),
            rn: vreg(12),
            rm: vreg(14),
            high_half: true,
        },
        "88C1AE4E",
        "smull2 v8.2d, v12.4s, v14.4s",
    ));

    insns.push((
        Inst::VecRRRLong {
            alu_op: VecRRRLongOp::Umull32,
            rd: writable_vreg(4),
            rn: vreg(12),
            rm: vreg(16),
            high_half: true,
        },
        "84C1B06E",
        "umull2 v4.2d, v12.4s, v16.4s",
    ));

    insns.push((
        Inst::VecRRRLongMod {
            alu_op: VecRRRLongModOp::Umlal32,
            rd: writable_vreg(10),
            ri: vreg(10),
            rn: vreg(29),
            rm: vreg(2),
            high_half: true,
        },
        "AA83A26E",
        "umlal2 v10.2d, v10.2d, v29.4s, v2.4s",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Sqrdmulh,
            rd: writable_vreg(31),
            rn: vreg(0),
            rm: vreg(31),
            size: VectorSize::Size16x8,
        },
        "1FB47F6E",
        "sqrdmulh v31.8h, v0.8h, v31.8h",
    ));

    insns.push((
        Inst::VecRRR {
            alu_op: VecALUOp::Sqrdmulh,
            rd: writable_vreg(7),
            rn: vreg(7),
            rm: vreg(23),
            size: VectorSize::Size32x2,
        },
        "E7B4B72E",
        "sqrdmulh v7.2s, v7.2s, v23.2s",
    ));

    insns.push((
        Inst::VecMisc {
            op: VecMisc2::Not,
            rd: writable_vreg(20),
            rn: vreg(17),
            size: VectorSize::Size8x8,
        },
        "345A202E",
        "mvn v20.8b, v17.8b",
    ));

    insns.push((
        Inst::VecMisc {
            op: VecMisc2::Not,
            rd: writable_vreg(2),
            rn: vreg(1),
            size: VectorSize::Size32x4,
        },
        "2258206E",
        "mvn v2.16b, v1.16b",
    ));

    insns.push((
        Inst::VecMisc {
            op: VecMisc2::Neg,
            rd: writable_vreg(3),
            rn: vreg(7),
            size: VectorSize::Size8x8,
        },
        "E3B8202E",
        "neg v3.8b, v7.8b",
    ));

    insns.push((
        Inst::VecMisc {
            op: VecMisc2::Neg,
            rd: writable_vreg(8),
            rn: vreg(12),
            size: VectorSize::Size8x16,
        },
        "88B9206E",
        "neg v8.16b, v12.16b",
    ));

    insns.push((
        Inst::VecMisc {
            op: VecMisc2::Neg,
            rd: writable_vreg(0),
            rn: vreg(31),
            size: VectorSize::Size16x8,
        },
        "E0BB606E",
        "neg v0.8h, v31.8h",
    ));

    insns.push((
        Inst::VecMisc {
            op: VecMisc2::Neg,
            rd: writable_vreg(2),
            rn: vreg(3),
            size: VectorSize::Size32x4,
        },
        "62B8A06E",
        "neg v2.4s, v3.4s",
    ));

    insns.push((
        Inst::VecMisc {
            op: VecMisc2::Neg,
            rd: writable_vreg(10),
            rn: vreg(8),
            size: VectorSize::Size64x2,
        },
        "0AB9E06E",
        "neg v10.2d, v8.2d",
    ));

    insns.push((
        Inst::VecMisc {
            op: VecMisc2::Abs,
            rd: writable_vreg(3),
            rn: vreg(1),
            size: VectorSize::Size8x8,
        },
        "23B8200E",
        "abs v3.8b, v1.8b",
    ));

    insns.push((
        Inst::VecMisc {
            op: VecMisc2::Abs,
            rd: writable_vreg(1),
            rn: vreg(1),
            size: VectorSize::Size8x16,
        },
        "21B8204E",
        "abs v1.16b, v1.16b",
    ));

    insns.push((
        Inst::VecMisc {
            op: VecMisc2::Abs,
            rd: writable_vreg(29),
            rn: vreg(28),
            size: VectorSize::Size16x8,
        },
        "9DBB604E",
        "abs v29.8h, v28.8h",
    ));

    insns.push((
        Inst::VecMisc {
            op: VecMisc2::Abs,
            rd: writable_vreg(7),
            rn: vreg(8),
            size: VectorSize::Size32x4,
        },
        "07B9A04E",
        "abs v7.4s, v8.4s",
    ));

    insns.push((
        Inst::VecMisc {
            op: VecMisc2::Abs,
            rd: writable_vreg(1),
            rn: vreg(10),
            size: VectorSize::Size64x2,
        },
        "41B9E04E",
        "abs v1.2d, v10.2d",
    ));

    insns.push((
        Inst::VecMisc {
            op: VecMisc2::Fabs,
            rd: writable_vreg(15),
            rn: vreg(16),
            size: VectorSize::Size32x2,
        },
        "0FFAA00E",
        "fabs v15.2s, v16.2s",
    ));

    insns.push((
        Inst::VecMisc {
            op: VecMisc2::Fabs,
            rd: writable_vreg(15),
            rn: vreg(16),
            size: VectorSize::Size32x4,
        },
        "0FFAA04E",
        "fabs v15.4s, v16.4s",
    ));

    insns.push((
        Inst::VecMisc {
            op: VecMisc2::Fabs,
            rd: writable_vreg(3),
            rn: vreg(22),
            size: VectorSize::Size64x2,
        },
        "C3FAE04E",
        "fabs v3.2d, v22.2d",
    ));

    insns.push((
        Inst::VecMisc {
            op: VecMisc2::Fneg,
            rd: writable_vreg(31),
            rn: vreg(0),
            size: VectorSize::Size32x2,
        },
        "1FF8A02E",
        "fneg v31.2s, v0.2s",
    ));

    insns.push((
        Inst::VecMisc {
            op: VecMisc2::Fneg,
            rd: writable_vreg(31),
            rn: vreg(0),
            size: VectorSize::Size32x4,
        },
        "1FF8A06E",
        "fneg v31.4s, v0.4s",
    ));

    insns.push((
        Inst::VecMisc {
            op: VecMisc2::Fneg,
            rd: writable_vreg(11),
            rn: vreg(6),
            size: VectorSize::Size64x2,
        },
        "CBF8E06E",
        "fneg v11.2d, v6.2d",
    ));

    insns.push((
        Inst::VecMisc {
            op: VecMisc2::Fsqrt,
            rd: writable_vreg(18),
            rn: vreg(25),
            size: VectorSize::Size32x2,
        },
        "32FBA12E",
        "fsqrt v18.2s, v25.2s",
    ));

    insns.push((
        Inst::VecMisc {
            op: VecMisc2::Fsqrt,
            rd: writable_vreg(18),
            rn: vreg(25),
            size: VectorSize::Size32x4,
        },
        "32FBA16E",
        "fsqrt v18.4s, v25.4s",
    ));

    insns.push((
        Inst::VecMisc {
            op: VecMisc2::Fsqrt,
            rd: writable_vreg(7),
            rn: vreg(18),
            size: VectorSize::Size64x2,
        },
        "47FAE16E",
        "fsqrt v7.2d, v18.2d",
    ));

    insns.push((
        Inst::VecMisc {
            op: VecMisc2::Rev64,
            rd: writable_vreg(1),
            rn: vreg(10),
            size: VectorSize::Size32x4,
        },
        "4109A04E",
        "rev64 v1.4s, v10.4s",
    ));

    insns.push((
        Inst::VecMisc {
            op: VecMisc2::Fcvtzs,
            rd: writable_vreg(4),
            rn: vreg(22),
            size: VectorSize::Size32x4,
        },
        "C4BAA14E",
        "fcvtzs v4.4s, v22.4s",
    ));

    insns.push((
        Inst::VecMisc {
            op: VecMisc2::Fcvtzs,
            rd: writable_vreg(0),
            rn: vreg(31),
            size: VectorSize::Size64x2,
        },
        "E0BBE14E",
        "fcvtzs v0.2d, v31.2d",
    ));

    insns.push((
        Inst::VecMisc {
            op: VecMisc2::Fcvtzu,
            rd: writable_vreg(4),
            rn: vreg(26),
            size: VectorSize::Size32x2,
        },
        "44BBA12E",
        "fcvtzu v4.2s, v26.2s",
    ));

    insns.push((
        Inst::VecMisc {
            op: VecMisc2::Fcvtzu,
            rd: writable_vreg(29),
            rn: vreg(15),
            size: VectorSize::Size64x2,
        },
        "FDB9E16E",
        "fcvtzu v29.2d, v15.2d",
    ));

    insns.push((
        Inst::VecMisc {
            op: VecMisc2::Scvtf,
            rd: writable_vreg(20),
            rn: vreg(8),
            size: VectorSize::Size32x4,
        },
        "14D9214E",
        "scvtf v20.4s, v8.4s",
    ));

    insns.push((
        Inst::VecMisc {
            op: VecMisc2::Ucvtf,
            rd: writable_vreg(10),
            rn: vreg(19),
            size: VectorSize::Size64x2,
        },
        "6ADA616E",
        "ucvtf v10.2d, v19.2d",
    ));

    insns.push((
        Inst::VecMisc {
            op: VecMisc2::Frintn,
            rd: writable_vreg(20),
            rn: vreg(7),
            size: VectorSize::Size32x2,
        },
        "F488210E",
        "frintn v20.2s, v7.2s",
    ));

    insns.push((
        Inst::VecMisc {
            op: VecMisc2::Frintn,
            rd: writable_vreg(11),
            rn: vreg(18),
            size: VectorSize::Size32x4,
        },
        "4B8A214E",
        "frintn v11.4s, v18.4s",
    ));

    insns.push((
        Inst::VecMisc {
            op: VecMisc2::Frintn,
            rd: writable_vreg(12),
            rn: vreg(17),
            size: VectorSize::Size64x2,
        },
        "2C8A614E",
        "frintn v12.2d, v17.2d",
    ));

    insns.push((
        Inst::VecMisc {
            op: VecMisc2::Frintz,
            rd: writable_vreg(1),
            rn: vreg(30),
            size: VectorSize::Size32x2,
        },
        "C19BA10E",
        "frintz v1.2s, v30.2s",
    ));

    insns.push((
        Inst::VecMisc {
            op: VecMisc2::Frintz,
            rd: writable_vreg(11),
            rn: vreg(18),
            size: VectorSize::Size32x4,
        },
        "4B9AA14E",
        "frintz v11.4s, v18.4s",
    ));

    insns.push((
        Inst::VecMisc {
            op: VecMisc2::Frintz,
            rd: writable_vreg(12),
            rn: vreg(17),
            size: VectorSize::Size64x2,
        },
        "2C9AE14E",
        "frintz v12.2d, v17.2d",
    ));

    insns.push((
        Inst::VecMisc {
            op: VecMisc2::Frintm,
            rd: writable_vreg(15),
            rn: vreg(7),
            size: VectorSize::Size32x2,
        },
        "EF98210E",
        "frintm v15.2s, v7.2s",
    ));

    insns.push((
        Inst::VecMisc {
            op: VecMisc2::Frintm,
            rd: writable_vreg(11),
            rn: vreg(18),
            size: VectorSize::Size32x4,
        },
        "4B9A214E",
        "frintm v11.4s, v18.4s",
    ));

    insns.push((
        Inst::VecMisc {
            op: VecMisc2::Frintm,
            rd: writable_vreg(12),
            rn: vreg(17),
            size: VectorSize::Size64x2,
        },
        "2C9A614E",
        "frintm v12.2d, v17.2d",
    ));

    insns.push((
        Inst::VecMisc {
            op: VecMisc2::Frintp,
            rd: writable_vreg(3),
            rn: vreg(4),
            size: VectorSize::Size32x2,
        },
        "8388A10E",
        "frintp v3.2s, v4.2s",
    ));

    insns.push((
        Inst::VecMisc {
            op: VecMisc2::Frintp,
            rd: writable_vreg(11),
            rn: vreg(18),
            size: VectorSize::Size32x4,
        },
        "4B8AA14E",
        "frintp v11.4s, v18.4s",
    ));

    insns.push((
        Inst::VecMisc {
            op: VecMisc2::Frintp,
            rd: writable_vreg(12),
            rn: vreg(17),
            size: VectorSize::Size64x2,
        },
        "2C8AE14E",
        "frintp v12.2d, v17.2d",
    ));

    insns.push((
        Inst::VecMisc {
            op: VecMisc2::Cnt,
            rd: writable_vreg(23),
            rn: vreg(5),
            size: VectorSize::Size8x8,
        },
        "B758200E",
        "cnt v23.8b, v5.8b",
    ));

    insns.push((
        Inst::VecMisc {
            op: VecMisc2::Fcmeq0,
            rd: writable_vreg(5),
            rn: vreg(2),
            size: VectorSize::Size32x4,
        },
        "45D8A04E",
        "fcmeq v5.4s, v2.4s, #0.0",
    ));

    insns.push((
        Inst::VecMisc {
            op: VecMisc2::Fcmge0,
            rd: writable_vreg(3),
            rn: vreg(1),
            size: VectorSize::Size64x2,
        },
        "23C8E06E",
        "fcmge v3.2d, v1.2d, #0.0",
    ));

    insns.push((
        Inst::VecMisc {
            op: VecMisc2::Fcmgt0,
            rd: writable_vreg(5),
            rn: vreg(7),
            size: VectorSize::Size32x4,
        },
        "E5C8A04E",
        "fcmgt v5.4s, v7.4s, #0.0",
    ));

    insns.push((
        Inst::VecMisc {
            op: VecMisc2::Fcmle0,
            rd: writable_vreg(10),
            rn: vreg(2),
            size: VectorSize::Size32x4,
        },
        "4AD8A06E",
        "fcmle v10.4s, v2.4s, #0.0",
    ));

    insns.push((
        Inst::VecMisc {
            op: VecMisc2::Fcmlt0,
            rd: writable_vreg(12),
            rn: vreg(12),
            size: VectorSize::Size64x2,
        },
        "8CE9E04E",
        "fcmlt v12.2d, v12.2d, #0.0",
    ));

    insns.push((
        Inst::VecMisc {
            op: VecMisc2::Cmeq0,
            rd: writable_vreg(22),
            rn: vreg(27),
            size: VectorSize::Size16x8,
        },
        "769B604E",
        "cmeq v22.8h, v27.8h, #0",
    ));

    insns.push((
        Inst::VecMisc {
            op: VecMisc2::Cmge0,
            rd: writable_vreg(12),
            rn: vreg(27),
            size: VectorSize::Size16x8,
        },
        "6C8B606E",
        "cmge v12.8h, v27.8h, #0",
    ));

    insns.push((
        Inst::VecMisc {
            op: VecMisc2::Cmgt0,
            rd: writable_vreg(12),
            rn: vreg(27),
            size: VectorSize::Size8x16,
        },
        "6C8B204E",
        "cmgt v12.16b, v27.16b, #0",
    ));

    insns.push((
        Inst::VecMisc {
            op: VecMisc2::Cmle0,
            rd: writable_vreg(1),
            rn: vreg(27),
            size: VectorSize::Size32x4,
        },
        "619BA06E",
        "cmle v1.4s, v27.4s, #0",
    ));

    insns.push((
        Inst::VecMisc {
            op: VecMisc2::Cmlt0,
            rd: writable_vreg(0),
            rn: vreg(7),
            size: VectorSize::Size64x2,
        },
        "E0A8E04E",
        "cmlt v0.2d, v7.2d, #0",
    ));

    insns.push((
        Inst::VecLanes {
            op: VecLanesOp::Uminv,
            rd: writable_vreg(0),
            rn: vreg(31),
            size: VectorSize::Size8x8,
        },
        "E0AB312E",
        "uminv b0, v31.8b",
    ));

    insns.push((
        Inst::VecLanes {
            op: VecLanesOp::Uminv,
            rd: writable_vreg(2),
            rn: vreg(1),
            size: VectorSize::Size8x16,
        },
        "22A8316E",
        "uminv b2, v1.16b",
    ));

    insns.push((
        Inst::VecLanes {
            op: VecLanesOp::Uminv,
            rd: writable_vreg(3),
            rn: vreg(11),
            size: VectorSize::Size16x8,
        },
        "63A9716E",
        "uminv h3, v11.8h",
    ));

    insns.push((
        Inst::VecLanes {
            op: VecLanesOp::Uminv,
            rd: writable_vreg(18),
            rn: vreg(4),
            size: VectorSize::Size32x4,
        },
        "92A8B16E",
        "uminv s18, v4.4s",
    ));

    insns.push((
        Inst::VecLanes {
            op: VecLanesOp::Addv,
            rd: writable_vreg(2),
            rn: vreg(29),
            size: VectorSize::Size8x16,
        },
        "A2BB314E",
        "addv b2, v29.16b",
    ));

    insns.push((
        Inst::VecLanes {
            op: VecLanesOp::Addv,
            rd: writable_vreg(15),
            rn: vreg(7),
            size: VectorSize::Size16x4,
        },
        "EFB8710E",
        "addv h15, v7.4h",
    ));

    insns.push((
        Inst::VecLanes {
            op: VecLanesOp::Addv,
            rd: writable_vreg(3),
            rn: vreg(21),
            size: VectorSize::Size16x8,
        },
        "A3BA714E",
        "addv h3, v21.8h",
    ));

    insns.push((
        Inst::VecLanes {
            op: VecLanesOp::Addv,
            rd: writable_vreg(18),
            rn: vreg(5),
            size: VectorSize::Size32x4,
        },
        "B2B8B14E",
        "addv s18, v5.4s",
    ));

    insns.push((
        Inst::VecShiftImm {
            op: VecShiftImmOp::Shl,
            rd: writable_vreg(27),
            rn: vreg(5),
            imm: 7,
            size: VectorSize::Size8x16,
        },
        "BB540F4F",
        "shl v27.16b, v5.16b, #7",
    ));

    insns.push((
        Inst::VecShiftImm {
            op: VecShiftImmOp::Shl,
            rd: writable_vreg(1),
            rn: vreg(30),
            imm: 0,
            size: VectorSize::Size8x16,
        },
        "C157084F",
        "shl v1.16b, v30.16b, #0",
    ));

    insns.push((
        Inst::VecShiftImm {
            op: VecShiftImmOp::Sshr,
            rd: writable_vreg(26),
            rn: vreg(6),
            imm: 16,
            size: VectorSize::Size16x8,
        },
        "DA04104F",
        "sshr v26.8h, v6.8h, #16",
    ));

    insns.push((
        Inst::VecShiftImm {
            op: VecShiftImmOp::Sshr,
            rd: writable_vreg(3),
            rn: vreg(19),
            imm: 1,
            size: VectorSize::Size16x8,
        },
        "63061F4F",
        "sshr v3.8h, v19.8h, #1",
    ));

    insns.push((
        Inst::VecShiftImm {
            op: VecShiftImmOp::Ushr,
            rd: writable_vreg(25),
            rn: vreg(6),
            imm: 8,
            size: VectorSize::Size8x8,
        },
        "D904082F",
        "ushr v25.8b, v6.8b, #8",
    ));

    insns.push((
        Inst::VecShiftImm {
            op: VecShiftImmOp::Ushr,
            rd: writable_vreg(5),
            rn: vreg(21),
            imm: 1,
            size: VectorSize::Size8x8,
        },
        "A5060F2F",
        "ushr v5.8b, v21.8b, #1",
    ));

    insns.push((
        Inst::VecShiftImm {
            op: VecShiftImmOp::Ushr,
            rd: writable_vreg(25),
            rn: vreg(6),
            imm: 8,
            size: VectorSize::Size8x16,
        },
        "D904086F",
        "ushr v25.16b, v6.16b, #8",
    ));

    insns.push((
        Inst::VecShiftImm {
            op: VecShiftImmOp::Ushr,
            rd: writable_vreg(5),
            rn: vreg(21),
            imm: 1,
            size: VectorSize::Size8x16,
        },
        "A5060F6F",
        "ushr v5.16b, v21.16b, #1",
    ));

    insns.push((
        Inst::VecShiftImm {
            op: VecShiftImmOp::Ushr,
            rd: writable_vreg(25),
            rn: vreg(6),
            imm: 16,
            size: VectorSize::Size16x4,
        },
        "D904102F",
        "ushr v25.4h, v6.4h, #16",
    ));

    insns.push((
        Inst::VecShiftImm {
            op: VecShiftImmOp::Ushr,
            rd: writable_vreg(5),
            rn: vreg(21),
            imm: 1,
            size: VectorSize::Size16x4,
        },
        "A5061F2F",
        "ushr v5.4h, v21.4h, #1",
    ));

    insns.push((
        Inst::VecShiftImm {
            op: VecShiftImmOp::Ushr,
            rd: writable_vreg(25),
            rn: vreg(6),
            imm: 16,
            size: VectorSize::Size16x8,
        },
        "D904106F",
        "ushr v25.8h, v6.8h, #16",
    ));

    insns.push((
        Inst::VecShiftImm {
            op: VecShiftImmOp::Ushr,
            rd: writable_vreg(5),
            rn: vreg(21),
            imm: 1,
            size: VectorSize::Size16x8,
        },
        "A5061F6F",
        "ushr v5.8h, v21.8h, #1",
    ));

    insns.push((
        Inst::VecShiftImm {
            op: VecShiftImmOp::Ushr,
            rd: writable_vreg(25),
            rn: vreg(6),
            imm: 32,
            size: VectorSize::Size32x2,
        },
        "D904202F",
        "ushr v25.2s, v6.2s, #32",
    ));

    insns.push((
        Inst::VecShiftImm {
            op: VecShiftImmOp::Ushr,
            rd: writable_vreg(5),
            rn: vreg(21),
            imm: 1,
            size: VectorSize::Size32x2,
        },
        "A5063F2F",
        "ushr v5.2s, v21.2s, #1",
    ));

    insns.push((
        Inst::VecShiftImm {
            op: VecShiftImmOp::Ushr,
            rd: writable_vreg(25),
            rn: vreg(6),
            imm: 32,
            size: VectorSize::Size32x4,
        },
        "D904206F",
        "ushr v25.4s, v6.4s, #32",
    ));

    insns.push((
        Inst::VecShiftImm {
            op: VecShiftImmOp::Ushr,
            rd: writable_vreg(5),
            rn: vreg(21),
            imm: 1,
            size: VectorSize::Size32x4,
        },
        "A5063F6F",
        "ushr v5.4s, v21.4s, #1",
    ));

    insns.push((
        Inst::VecShiftImm {
            op: VecShiftImmOp::Ushr,
            rd: writable_vreg(25),
            rn: vreg(6),
            imm: 64,
            size: VectorSize::Size64x2,
        },
        "D904406F",
        "ushr v25.2d, v6.2d, #64",
    ));

    insns.push((
        Inst::VecShiftImm {
            op: VecShiftImmOp::Ushr,
            rd: writable_vreg(5),
            rn: vreg(21),
            imm: 1,
            size: VectorSize::Size64x2,
        },
        "A5067F6F",
        "ushr v5.2d, v21.2d, #1",
    ));

    insns.push((
        Inst::VecShiftImm {
            op: VecShiftImmOp::Shl,
            rd: writable_vreg(22),
            rn: vreg(13),
            imm: 63,
            size: VectorSize::Size64x2,
        },
        "B6557F4F",
        "shl v22.2d, v13.2d, #63",
    ));

    insns.push((
        Inst::VecShiftImm {
            op: VecShiftImmOp::Shl,
            rd: writable_vreg(23),
            rn: vreg(9),
            imm: 0,
            size: VectorSize::Size64x2,
        },
        "3755404F",
        "shl v23.2d, v9.2d, #0",
    ));

    insns.push((
        Inst::VecExtract {
            rd: writable_vreg(1),
            rn: vreg(30),
            rm: vreg(17),
            imm4: 0,
        },
        "C103116E",
        "ext v1.16b, v30.16b, v17.16b, #0",
    ));

    insns.push((
        Inst::VecExtract {
            rd: writable_vreg(1),
            rn: vreg(30),
            rm: vreg(17),
            imm4: 8,
        },
        "C143116E",
        "ext v1.16b, v30.16b, v17.16b, #8",
    ));

    insns.push((
        Inst::VecExtract {
            rd: writable_vreg(1),
            rn: vreg(30),
            rm: vreg(17),
            imm4: 15,
        },
        "C17B116E",
        "ext v1.16b, v30.16b, v17.16b, #15",
    ));

    insns.push((
        Inst::VecTbl {
            rd: writable_vreg(0),
            rn: vreg(31),
            rm: vreg(16),
        },
        "E003104E",
        "tbl v0.16b, { v31.16b }, v16.16b",
    ));

    insns.push((
        Inst::VecTblExt {
            rd: writable_vreg(4),
            ri: vreg(4),
            rn: vreg(12),
            rm: vreg(23),
        },
        "8411174E",
        "tbx v4.16b, v4.16b, { v12.16b }, v23.16b",
    ));

    insns.push((
        Inst::VecTbl2 {
            rd: writable_vreg(16),
            rn: vreg(31),
            rn2: vreg(0),
            rm: vreg(26),
        },
        "F0231A4E",
        "tbl v16.16b, { v31.16b, v0.16b }, v26.16b",
    ));

    insns.push((
        Inst::VecTbl2Ext {
            rd: writable_vreg(3),
            ri: vreg(3),
            rn: vreg(11),
            rn2: vreg(12),
            rm: vreg(19),
        },
        "6331134E",
        "tbx v3.16b, v3.16b, { v11.16b, v12.16b }, v19.16b",
    ));

    insns.push((
        Inst::VecLoadReplicate {
            rd: writable_vreg(31),
            rn: xreg(0),
            size: VectorSize::Size64x2,
            flags: MemFlags::trusted(),
        },
        "1FCC404D",
        "ld1r { v31.2d }, [x0]",
    ));

    insns.push((
        Inst::VecLoadReplicate {
            rd: writable_vreg(0),
            rn: xreg(25),
            size: VectorSize::Size8x8,
            flags: MemFlags::trusted(),
        },
        "20C3400D",
        "ld1r { v0.8b }, [x25]",
    ));

    insns.push((
        Inst::VecCSel {
            rd: writable_vreg(5),
            rn: vreg(10),
            rm: vreg(19),
            cond: Cond::Gt,
        },
        "6C000054651EB34E02000014451DAA4E",
        "vcsel v5.16b, v10.16b, v19.16b, gt (if-then-else diamond)",
    ));

    insns.push((
        Inst::Extend {
            rd: writable_xreg(3),
            rn: xreg(5),
            signed: false,
            from_bits: 1,
            to_bits: 32,
        },
        "A3000012",
        "and w3, w5, #1",
    ));
    insns.push((
        Inst::Extend {
            rd: writable_xreg(3),
            rn: xreg(5),
            signed: false,
            from_bits: 1,
            to_bits: 64,
        },
        "A3000012",
        "and w3, w5, #1",
    ));
    insns.push((
        Inst::Extend {
            rd: writable_xreg(10),
            rn: xreg(21),
            signed: true,
            from_bits: 1,
            to_bits: 32,
        },
        "AA020013",
        "sbfx w10, w21, #0, #1",
    ));
    insns.push((
        Inst::Extend {
            rd: writable_xreg(1),
            rn: xreg(2),
            signed: true,
            from_bits: 1,
            to_bits: 64,
        },
        "41004093",
        "sbfx x1, x2, #0, #1",
    ));
    insns.push((
        Inst::Extend {
            rd: writable_xreg(1),
            rn: xreg(2),
            signed: false,
            from_bits: 8,
            to_bits: 32,
        },
        "411C0053",
        "uxtb w1, w2",
    ));
    insns.push((
        Inst::Extend {
            rd: writable_xreg(1),
            rn: xreg(2),
            signed: true,
            from_bits: 8,
            to_bits: 32,
        },
        "411C0013",
        "sxtb w1, w2",
    ));
    insns.push((
        Inst::Extend {
            rd: writable_xreg(1),
            rn: xreg(2),
            signed: false,
            from_bits: 16,
            to_bits: 32,
        },
        "413C0053",
        "uxth w1, w2",
    ));
    insns.push((
        Inst::Extend {
            rd: writable_xreg(1),
            rn: xreg(2),
            signed: true,
            from_bits: 16,
            to_bits: 32,
        },
        "413C0013",
        "sxth w1, w2",
    ));
    insns.push((
        Inst::Extend {
            rd: writable_xreg(1),
            rn: xreg(2),
            signed: false,
            from_bits: 8,
            to_bits: 64,
        },
        "411C0053",
        "uxtb w1, w2",
    ));
    insns.push((
        Inst::Extend {
            rd: writable_xreg(1),
            rn: xreg(2),
            signed: true,
            from_bits: 8,
            to_bits: 64,
        },
        "411C4093",
        "sxtb x1, w2",
    ));
    insns.push((
        Inst::Extend {
            rd: writable_xreg(1),
            rn: xreg(2),
            signed: false,
            from_bits: 16,
            to_bits: 64,
        },
        "413C0053",
        "uxth w1, w2",
    ));
    insns.push((
        Inst::Extend {
            rd: writable_xreg(1),
            rn: xreg(2),
            signed: true,
            from_bits: 16,
            to_bits: 64,
        },
        "413C4093",
        "sxth x1, w2",
    ));
    insns.push((
        Inst::Extend {
            rd: writable_xreg(1),
            rn: xreg(2),
            signed: false,
            from_bits: 32,
            to_bits: 64,
        },
        "E103022A",
        "mov w1, w2",
    ));
    insns.push((
        Inst::Extend {
            rd: writable_xreg(1),
            rn: xreg(2),
            signed: true,
            from_bits: 32,
            to_bits: 64,
        },
        "417C4093",
        "sxtw x1, w2",
    ));

    insns.push((
        Inst::Jump {
            dest: BranchTarget::ResolvedOffset(64),
        },
        "10000014",
        "b 64",
    ));

    insns.push((
        Inst::TrapIf {
            trap_code: TrapCode::STACK_OVERFLOW,
            kind: CondBrKind::NotZero(xreg(8), OperandSize::Size64),
        },
        "280000B51FC10000",
        "cbnz x8, #trap=stk_ovf",
    ));
    insns.push((
        Inst::TrapIf {
            trap_code: TrapCode::STACK_OVERFLOW,
            kind: CondBrKind::Zero(xreg(8), OperandSize::Size64),
        },
        "280000B41FC10000",
        "cbz x8, #trap=stk_ovf",
    ));
    insns.push((
        Inst::TrapIf {
            trap_code: TrapCode::STACK_OVERFLOW,
            kind: CondBrKind::Cond(Cond::Ne),
        },
        "210000541FC10000",
        "b.ne #trap=stk_ovf",
    ));
    insns.push((
        Inst::TrapIf {
            trap_code: TrapCode::STACK_OVERFLOW,
            kind: CondBrKind::Cond(Cond::Eq),
        },
        "200000541FC10000",
        "b.eq #trap=stk_ovf",
    ));
    insns.push((
        Inst::TrapIf {
            trap_code: TrapCode::STACK_OVERFLOW,
            kind: CondBrKind::Cond(Cond::Lo),
        },
        "230000541FC10000",
        "b.lo #trap=stk_ovf",
    ));
    insns.push((
        Inst::TrapIf {
            trap_code: TrapCode::STACK_OVERFLOW,
            kind: CondBrKind::Cond(Cond::Hs),
        },
        "220000541FC10000",
        "b.hs #trap=stk_ovf",
    ));
    insns.push((
        Inst::TrapIf {
            trap_code: TrapCode::STACK_OVERFLOW,
            kind: CondBrKind::Cond(Cond::Pl),
        },
        "250000541FC10000",
        "b.pl #trap=stk_ovf",
    ));
    insns.push((
        Inst::TrapIf {
            trap_code: TrapCode::STACK_OVERFLOW,
            kind: CondBrKind::Cond(Cond::Mi),
        },
        "240000541FC10000",
        "b.mi #trap=stk_ovf",
    ));
    insns.push((
        Inst::TrapIf {
            trap_code: TrapCode::STACK_OVERFLOW,
            kind: CondBrKind::Cond(Cond::Vc),
        },
        "270000541FC10000",
        "b.vc #trap=stk_ovf",
    ));
    insns.push((
        Inst::TrapIf {
            trap_code: TrapCode::STACK_OVERFLOW,
            kind: CondBrKind::Cond(Cond::Vs),
        },
        "260000541FC10000",
        "b.vs #trap=stk_ovf",
    ));
    insns.push((
        Inst::TrapIf {
            trap_code: TrapCode::STACK_OVERFLOW,
            kind: CondBrKind::Cond(Cond::Ls),
        },
        "290000541FC10000",
        "b.ls #trap=stk_ovf",
    ));
    insns.push((
        Inst::TrapIf {
            trap_code: TrapCode::STACK_OVERFLOW,
            kind: CondBrKind::Cond(Cond::Hi),
        },
        "280000541FC10000",
        "b.hi #trap=stk_ovf",
    ));
    insns.push((
        Inst::TrapIf {
            trap_code: TrapCode::STACK_OVERFLOW,
            kind: CondBrKind::Cond(Cond::Lt),
        },
        "2B0000541FC10000",
        "b.lt #trap=stk_ovf",
    ));
    insns.push((
        Inst::TrapIf {
            trap_code: TrapCode::STACK_OVERFLOW,
            kind: CondBrKind::Cond(Cond::Ge),
        },
        "2A0000541FC10000",
        "b.ge #trap=stk_ovf",
    ));
    insns.push((
        Inst::TrapIf {
            trap_code: TrapCode::STACK_OVERFLOW,
            kind: CondBrKind::Cond(Cond::Le),
        },
        "2D0000541FC10000",
        "b.le #trap=stk_ovf",
    ));
    insns.push((
        Inst::TrapIf {
            trap_code: TrapCode::STACK_OVERFLOW,
            kind: CondBrKind::Cond(Cond::Gt),
        },
        "2C0000541FC10000",
        "b.gt #trap=stk_ovf",
    ));
    insns.push((
        Inst::TrapIf {
            trap_code: TrapCode::STACK_OVERFLOW,
            kind: CondBrKind::Cond(Cond::Nv),
        },
        "2F0000541FC10000",
        "b.nv #trap=stk_ovf",
    ));
    insns.push((
        Inst::TrapIf {
            trap_code: TrapCode::STACK_OVERFLOW,
            kind: CondBrKind::Cond(Cond::Al),
        },
        "2E0000541FC10000",
        "b.al #trap=stk_ovf",
    ));

    insns.push((
        Inst::CondBr {
            taken: BranchTarget::ResolvedOffset(64),
            not_taken: BranchTarget::ResolvedOffset(128),
            kind: CondBrKind::Cond(Cond::Le),
        },
        "0D02005420000014",
        "b.le 64 ; b 128",
    ));

    insns.push((
        Inst::Call {
            info: Box::new(CallInfo::empty(
                ExternalName::testcase("test0"),
                CallConv::SystemV,
            )),
        },
        "00000094",
        "bl 0",
    ));

    insns.push((
        Inst::CallInd {
            info: Box::new(CallInfo::empty(xreg(10), CallConv::SystemV)),
        },
        "40013FD6",
        "blr x10",
    ));

    insns.push((
        Inst::IndirectBr {
            rn: xreg(3),
            targets: vec![],
        },
        "60001FD6",
        "br x3",
    ));

    insns.push((Inst::Brk, "00003ED4", "brk #0xf000"));

    insns.push((
        Inst::Adr {
            rd: writable_xreg(15),
            off: (1 << 20) - 4,
        },
        "EFFF7F10",
        "adr x15, pc+1048572",
    ));

    insns.push((
        Inst::Adrp {
            rd: writable_xreg(8),
            off: 0,
        },
        "08000090",
        "adrp x8, pc+0",
    ));

    insns.push((
        Inst::Adrp {
            rd: writable_xreg(3),
            off: 16,
        },
        "83000090",
        "adrp x3, pc+65536",
    ));

    insns.push((
        Inst::FpuMove64 {
            rd: writable_vreg(8),
            rn: vreg(4),
        },
        "8840601E",
        "fmov d8, d4",
    ));

    insns.push((
        Inst::FpuMove32 {
            rd: writable_vreg(8),
            rn: vreg(4),
        },
        "8840201E",
        "fmov s8, s4",
    ));

    insns.push((
        Inst::FpuMove128 {
            rd: writable_vreg(17),
            rn: vreg(26),
        },
        "511FBA4E",
        "mov v17.16b, v26.16b",
    ));

    insns.push((
        Inst::FpuMoveFromVec {
            rd: writable_vreg(1),
            rn: vreg(30),
            idx: 2,
            size: VectorSize::Size32x4,
        },
        "C107145E",
        "mov s1, v30.s[2]",
    ));

    insns.push((
        Inst::FpuMoveFromVec {
            rd: writable_vreg(23),
            rn: vreg(11),
            idx: 0,
            size: VectorSize::Size64x2,
        },
        "7705085E",
        "mov d23, v11.d[0]",
    ));

    insns.push((
        Inst::FpuExtend {
            rd: writable_vreg(31),
            rn: vreg(0),
            size: ScalarSize::Size32,
        },
        "1F40201E",
        "fmov s31, s0",
    ));

    insns.push((
        Inst::FpuExtend {
            rd: writable_vreg(31),
            rn: vreg(0),
            size: ScalarSize::Size64,
        },
        "1F40601E",
        "fmov d31, d0",
    ));

    insns.push((
        Inst::FpuRR {
            fpu_op: FPUOp1::Abs,
            size: ScalarSize::Size32,
            rd: writable_vreg(15),
            rn: vreg(30),
        },
        "CFC3201E",
        "fabs s15, s30",
    ));

    insns.push((
        Inst::FpuRR {
            fpu_op: FPUOp1::Abs,
            size: ScalarSize::Size64,
            rd: writable_vreg(15),
            rn: vreg(30),
        },
        "CFC3601E",
        "fabs d15, d30",
    ));

    insns.push((
        Inst::FpuRR {
            fpu_op: FPUOp1::Neg,
            size: ScalarSize::Size32,
            rd: writable_vreg(15),
            rn: vreg(30),
        },
        "CF43211E",
        "fneg s15, s30",
    ));

    insns.push((
        Inst::FpuRR {
            fpu_op: FPUOp1::Neg,
            size: ScalarSize::Size64,
            rd: writable_vreg(15),
            rn: vreg(30),
        },
        "CF43611E",
        "fneg d15, d30",
    ));

    insns.push((
        Inst::FpuRR {
            fpu_op: FPUOp1::Sqrt,
            size: ScalarSize::Size32,
            rd: writable_vreg(15),
            rn: vreg(30),
        },
        "CFC3211E",
        "fsqrt s15, s30",
    ));

    insns.push((
        Inst::FpuRR {
            fpu_op: FPUOp1::Sqrt,
            size: ScalarSize::Size64,
            rd: writable_vreg(15),
            rn: vreg(30),
        },
        "CFC3611E",
        "fsqrt d15, d30",
    ));

    insns.push((
        Inst::FpuRR {
            fpu_op: FPUOp1::Cvt32To64,
            size: ScalarSize::Size32,
            rd: writable_vreg(15),
            rn: vreg(30),
        },
        "CFC3221E",
        "fcvt d15, s30",
    ));

    insns.push((
        Inst::FpuRR {
            fpu_op: FPUOp1::Cvt64To32,
            size: ScalarSize::Size64,
            rd: writable_vreg(15),
            rn: vreg(30),
        },
        "CF43621E",
        "fcvt s15, d30",
    ));

    insns.push((
        Inst::FpuRRR {
            fpu_op: FPUOp2::Add,
            size: ScalarSize::Size32,
            rd: writable_vreg(15),
            rn: vreg(30),
            rm: vreg(31),
        },
        "CF2B3F1E",
        "fadd s15, s30, s31",
    ));

    insns.push((
        Inst::FpuRRR {
            fpu_op: FPUOp2::Add,
            size: ScalarSize::Size64,
            rd: writable_vreg(15),
            rn: vreg(30),
            rm: vreg(31),
        },
        "CF2B7F1E",
        "fadd d15, d30, d31",
    ));

    insns.push((
        Inst::FpuRRR {
            fpu_op: FPUOp2::Sub,
            size: ScalarSize::Size32,
            rd: writable_vreg(15),
            rn: vreg(30),
            rm: vreg(31),
        },
        "CF3B3F1E",
        "fsub s15, s30, s31",
    ));

    insns.push((
        Inst::FpuRRR {
            fpu_op: FPUOp2::Sub,
            size: ScalarSize::Size64,
            rd: writable_vreg(15),
            rn: vreg(30),
            rm: vreg(31),
        },
        "CF3B7F1E",
        "fsub d15, d30, d31",
    ));

    insns.push((
        Inst::FpuRRR {
            fpu_op: FPUOp2::Mul,
            size: ScalarSize::Size32,
            rd: writable_vreg(15),
            rn: vreg(30),
            rm: vreg(31),
        },
        "CF0B3F1E",
        "fmul s15, s30, s31",
    ));

    insns.push((
        Inst::FpuRRR {
            fpu_op: FPUOp2::Mul,
            size: ScalarSize::Size64,
            rd: writable_vreg(15),
            rn: vreg(30),
            rm: vreg(31),
        },
        "CF0B7F1E",
        "fmul d15, d30, d31",
    ));

    insns.push((
        Inst::FpuRRR {
            fpu_op: FPUOp2::Div,
            size: ScalarSize::Size32,
            rd: writable_vreg(15),
            rn: vreg(30),
            rm: vreg(31),
        },
        "CF1B3F1E",
        "fdiv s15, s30, s31",
    ));

    insns.push((
        Inst::FpuRRR {
            fpu_op: FPUOp2::Div,
            size: ScalarSize::Size64,
            rd: writable_vreg(15),
            rn: vreg(30),
            rm: vreg(31),
        },
        "CF1B7F1E",
        "fdiv d15, d30, d31",
    ));

    insns.push((
        Inst::FpuRRR {
            fpu_op: FPUOp2::Max,
            size: ScalarSize::Size32,
            rd: writable_vreg(15),
            rn: vreg(30),
            rm: vreg(31),
        },
        "CF4B3F1E",
        "fmax s15, s30, s31",
    ));

    insns.push((
        Inst::FpuRRR {
            fpu_op: FPUOp2::Max,
            size: ScalarSize::Size64,
            rd: writable_vreg(15),
            rn: vreg(30),
            rm: vreg(31),
        },
        "CF4B7F1E",
        "fmax d15, d30, d31",
    ));

    insns.push((
        Inst::FpuRRR {
            fpu_op: FPUOp2::Min,
            size: ScalarSize::Size32,
            rd: writable_vreg(15),
            rn: vreg(30),
            rm: vreg(31),
        },
        "CF5B3F1E",
        "fmin s15, s30, s31",
    ));

    insns.push((
        Inst::FpuRRR {
            fpu_op: FPUOp2::Min,
            size: ScalarSize::Size64,
            rd: writable_vreg(15),
            rn: vreg(30),
            rm: vreg(31),
        },
        "CF5B7F1E",
        "fmin d15, d30, d31",
    ));

    insns.push((
        Inst::FpuRRRR {
            fpu_op: FPUOp3::MAdd,
            size: ScalarSize::Size32,
            rd: writable_vreg(15),
            rn: vreg(30),
            rm: vreg(31),
            ra: vreg(1),
        },
        "CF071F1F",
        "fmadd s15, s30, s31, s1",
    ));

    insns.push((
        Inst::FpuRRRR {
            fpu_op: FPUOp3::MSub,
            size: ScalarSize::Size32,
            rd: writable_vreg(15),
            rn: vreg(30),
            rm: vreg(31),
            ra: vreg(1),
        },
        "CF871F1F",
        "fmsub s15, s30, s31, s1",
    ));

    insns.push((
        Inst::FpuRRRR {
            fpu_op: FPUOp3::MAdd,
            size: ScalarSize::Size64,
            rd: writable_vreg(15),
            rn: vreg(30),
            rm: vreg(31),
            ra: vreg(1),
        },
        "CF075F1F",
        "fmadd d15, d30, d31, d1",
    ));

    insns.push((
        Inst::FpuRRRR {
            fpu_op: FPUOp3::MSub,
            size: ScalarSize::Size64,
            rd: writable_vreg(15),
            rn: vreg(30),
            rm: vreg(31),
            ra: vreg(1),
        },
        "CF875F1F",
        "fmsub d15, d30, d31, d1",
    ));

    insns.push((
        Inst::FpuRRRR {
            fpu_op: FPUOp3::NMAdd,
            size: ScalarSize::Size64,
            rd: writable_vreg(15),
            rn: vreg(30),
            rm: vreg(31),
            ra: vreg(1),
        },
        "CF077F1F",
        "fnmadd d15, d30, d31, d1",
    ));

    insns.push((
        Inst::FpuRRRR {
            fpu_op: FPUOp3::NMSub,
            size: ScalarSize::Size64,
            rd: writable_vreg(15),
            rn: vreg(30),
            rm: vreg(31),
            ra: vreg(1),
        },
        "CF877F1F",
        "fnmsub d15, d30, d31, d1",
    ));

    insns.push((
        Inst::FpuRRI {
            fpu_op: FPUOpRI::UShr32(FPURightShiftImm::maybe_from_u8(32, 32).unwrap()),
            rd: writable_vreg(2),
            rn: vreg(5),
        },
        "A204202F",
        "ushr v2.2s, v5.2s, #32",
    ));

    insns.push((
        Inst::FpuRRI {
            fpu_op: FPUOpRI::UShr64(FPURightShiftImm::maybe_from_u8(63, 64).unwrap()),
            rd: writable_vreg(2),
            rn: vreg(5),
        },
        "A204417F",
        "ushr d2, d5, #63",
    ));

    insns.push((
        Inst::FpuRRIMod {
            fpu_op: FPUOpRIMod::Sli32(FPULeftShiftImm::maybe_from_u8(31, 32).unwrap()),
            rd: writable_vreg(4),
            ri: vreg(4),
            rn: vreg(10),
        },
        "44553F2F",
        "sli v4.2s, v4.2s, v10.2s, #31",
    ));

    insns.push((
        Inst::FpuRRIMod {
            fpu_op: FPUOpRIMod::Sli64(FPULeftShiftImm::maybe_from_u8(63, 64).unwrap()),
            rd: writable_vreg(4),
            ri: vreg(4),
            rn: vreg(10),
        },
        "44557F7F",
        "sli d4, d4, d10, #63",
    ));

    insns.push((
        Inst::FpuToInt {
            op: FpuToIntOp::F32ToU32,
            rd: writable_xreg(1),
            rn: vreg(4),
        },
        "8100391E",
        "fcvtzu w1, s4",
    ));

    insns.push((
        Inst::FpuToInt {
            op: FpuToIntOp::F32ToU64,
            rd: writable_xreg(1),
            rn: vreg(4),
        },
        "8100399E",
        "fcvtzu x1, s4",
    ));

    insns.push((
        Inst::FpuToInt {
            op: FpuToIntOp::F32ToI32,
            rd: writable_xreg(1),
            rn: vreg(4),
        },
        "8100381E",
        "fcvtzs w1, s4",
    ));

    insns.push((
        Inst::FpuToInt {
            op: FpuToIntOp::F32ToI64,
            rd: writable_xreg(1),
            rn: vreg(4),
        },
        "8100389E",
        "fcvtzs x1, s4",
    ));

    insns.push((
        Inst::FpuToInt {
            op: FpuToIntOp::F64ToU32,
            rd: writable_xreg(1),
            rn: vreg(4),
        },
        "8100791E",
        "fcvtzu w1, d4",
    ));

    insns.push((
        Inst::FpuToInt {
            op: FpuToIntOp::F64ToU64,
            rd: writable_xreg(1),
            rn: vreg(4),
        },
        "8100799E",
        "fcvtzu x1, d4",
    ));

    insns.push((
        Inst::FpuToInt {
            op: FpuToIntOp::F64ToI32,
            rd: writable_xreg(1),
            rn: vreg(4),
        },
        "8100781E",
        "fcvtzs w1, d4",
    ));

    insns.push((
        Inst::FpuToInt {
            op: FpuToIntOp::F64ToI64,
            rd: writable_xreg(1),
            rn: vreg(4),
        },
        "8100789E",
        "fcvtzs x1, d4",
    ));

    insns.push((
        Inst::IntToFpu {
            op: IntToFpuOp::U32ToF32,
            rd: writable_vreg(1),
            rn: xreg(4),
        },
        "8100231E",
        "ucvtf s1, w4",
    ));

    insns.push((
        Inst::IntToFpu {
            op: IntToFpuOp::I32ToF32,
            rd: writable_vreg(1),
            rn: xreg(4),
        },
        "8100221E",
        "scvtf s1, w4",
    ));

    insns.push((
        Inst::IntToFpu {
            op: IntToFpuOp::U32ToF64,
            rd: writable_vreg(1),
            rn: xreg(4),
        },
        "8100631E",
        "ucvtf d1, w4",
    ));

    insns.push((
        Inst::IntToFpu {
            op: IntToFpuOp::I32ToF64,
            rd: writable_vreg(1),
            rn: xreg(4),
        },
        "8100621E",
        "scvtf d1, w4",
    ));

    insns.push((
        Inst::IntToFpu {
            op: IntToFpuOp::U64ToF32,
            rd: writable_vreg(1),
            rn: xreg(4),
        },
        "8100239E",
        "ucvtf s1, x4",
    ));

    insns.push((
        Inst::IntToFpu {
            op: IntToFpuOp::I64ToF32,
            rd: writable_vreg(1),
            rn: xreg(4),
        },
        "8100229E",
        "scvtf s1, x4",
    ));

    insns.push((
        Inst::IntToFpu {
            op: IntToFpuOp::U64ToF64,
            rd: writable_vreg(1),
            rn: xreg(4),
        },
        "8100639E",
        "ucvtf d1, x4",
    ));

    insns.push((
        Inst::IntToFpu {
            op: IntToFpuOp::I64ToF64,
            rd: writable_vreg(1),
            rn: xreg(4),
        },
        "8100629E",
        "scvtf d1, x4",
    ));

    insns.push((
        Inst::FpuCmp {
            size: ScalarSize::Size32,
            rn: vreg(23),
            rm: vreg(24),
        },
        "E022381E",
        "fcmp s23, s24",
    ));

    insns.push((
        Inst::FpuCmp {
            size: ScalarSize::Size64,
            rn: vreg(23),
            rm: vreg(24),
        },
        "E022781E",
        "fcmp d23, d24",
    ));

    insns.push((
        Inst::FpuLoad16 {
            rd: writable_vreg(16),
            mem: AMode::RegScaled {
                rn: xreg(8),
                rm: xreg(9),
            },
            flags: MemFlags::trusted(),
        },
        "1079697C",
        "ldr h16, [x8, x9, LSL #1]",
    ));

    insns.push((
        Inst::FpuLoad32 {
            rd: writable_vreg(16),
            mem: AMode::RegScaled {
                rn: xreg(8),
                rm: xreg(9),
            },
            flags: MemFlags::trusted(),
        },
        "107969BC",
        "ldr s16, [x8, x9, LSL #2]",
    ));

    insns.push((
        Inst::FpuLoad64 {
            rd: writable_vreg(16),
            mem: AMode::RegScaled {
                rn: xreg(8),
                rm: xreg(9),
            },
            flags: MemFlags::trusted(),
        },
        "107969FC",
        "ldr d16, [x8, x9, LSL #3]",
    ));

    insns.push((
        Inst::FpuLoad128 {
            rd: writable_vreg(16),
            mem: AMode::RegScaled {
                rn: xreg(8),
                rm: xreg(9),
            },
            flags: MemFlags::trusted(),
        },
        "1079E93C",
        "ldr q16, [x8, x9, LSL #4]",
    ));

    insns.push((
        Inst::FpuLoad32 {
            rd: writable_vreg(16),
            mem: AMode::Label {
                label: MemLabel::PCRel(8),
            },
            flags: MemFlags::trusted(),
        },
        "5000001C",
        "ldr s16, pc+8",
    ));

    insns.push((
        Inst::FpuLoad64 {
            rd: writable_vreg(16),
            mem: AMode::Label {
                label: MemLabel::PCRel(8),
            },
            flags: MemFlags::trusted(),
        },
        "5000005C",
        "ldr d16, pc+8",
    ));

    insns.push((
        Inst::FpuLoad128 {
            rd: writable_vreg(16),
            mem: AMode::Label {
                label: MemLabel::PCRel(8),
            },
            flags: MemFlags::trusted(),
        },
        "5000009C",
        "ldr q16, pc+8",
    ));

    insns.push((
        Inst::FpuStore16 {
            rd: vreg(16),
            mem: AMode::RegScaled {
                rn: xreg(8),
                rm: xreg(9),
            },
            flags: MemFlags::trusted(),
        },
        "1079297C",
        "str h16, [x8, x9, LSL #1]",
    ));

    insns.push((
        Inst::FpuStore32 {
            rd: vreg(16),
            mem: AMode::RegScaled {
                rn: xreg(8),
                rm: xreg(9),
            },
            flags: MemFlags::trusted(),
        },
        "107929BC",
        "str s16, [x8, x9, LSL #2]",
    ));

    insns.push((
        Inst::FpuStore64 {
            rd: vreg(16),
            mem: AMode::RegScaled {
                rn: xreg(8),
                rm: xreg(9),
            },
            flags: MemFlags::trusted(),
        },
        "107929FC",
        "str d16, [x8, x9, LSL #3]",
    ));

    insns.push((
        Inst::FpuStore128 {
            rd: vreg(16),
            mem: AMode::RegScaled {
                rn: xreg(8),
                rm: xreg(9),
            },
            flags: MemFlags::trusted(),
        },
        "1079A93C",
        "str q16, [x8, x9, LSL #4]",
    ));

    insns.push((
        Inst::FpuLoadP64 {
            rt: writable_vreg(0),
            rt2: writable_vreg(31),
            mem: PairAMode::SignedOffset {
                reg: xreg(0),
                simm7: simm7_scaled_zero(F64),
            },
            flags: MemFlags::trusted(),
        },
        "007C406D",
        "ldp d0, d31, [x0]",
    ));

    insns.push((
        Inst::FpuLoadP64 {
            rt: writable_vreg(19),
            rt2: writable_vreg(11),
            mem: PairAMode::SPPreIndexed {
                simm7: SImm7Scaled::maybe_from_i64(-512, F64).unwrap(),
            },
            flags: MemFlags::trusted(),
        },
        "F32FE06D",
        "ldp d19, d11, [sp, #-512]!",
    ));

    insns.push((
        Inst::FpuLoadP64 {
            rt: writable_vreg(7),
            rt2: writable_vreg(20),
            mem: PairAMode::SPPostIndexed {
                simm7: SImm7Scaled::maybe_from_i64(64, F64).unwrap(),
            },
            flags: MemFlags::trusted(),
        },
        "E753C46C",
        "ldp d7, d20, [sp], #64",
    ));

    insns.push((
        Inst::FpuStoreP64 {
            rt: vreg(4),
            rt2: vreg(26),
            mem: PairAMode::SignedOffset {
                reg: stack_reg(),
                simm7: SImm7Scaled::maybe_from_i64(504, F64).unwrap(),
            },
            flags: MemFlags::trusted(),
        },
        "E4EB1F6D",
        "stp d4, d26, [sp, #504]",
    ));

    insns.push((
        Inst::FpuStoreP64 {
            rt: vreg(16),
            rt2: vreg(8),
            mem: PairAMode::SPPreIndexed {
                simm7: SImm7Scaled::maybe_from_i64(48, F64).unwrap(),
            },
            flags: MemFlags::trusted(),
        },
        "F023836D",
        "stp d16, d8, [sp, #48]!",
    ));

    insns.push((
        Inst::FpuStoreP64 {
            rt: vreg(5),
            rt2: vreg(6),
            mem: PairAMode::SPPostIndexed {
                simm7: SImm7Scaled::maybe_from_i64(-32, F64).unwrap(),
            },
            flags: MemFlags::trusted(),
        },
        "E51BBE6C",
        "stp d5, d6, [sp], #-32",
    ));

    insns.push((
        Inst::FpuLoadP128 {
            rt: writable_vreg(0),
            rt2: writable_vreg(17),
            mem: PairAMode::SignedOffset {
                reg: xreg(3),
                simm7: simm7_scaled_zero(I8X16),
            },
            flags: MemFlags::trusted(),
        },
        "604440AD",
        "ldp q0, q17, [x3]",
    ));

    insns.push((
        Inst::FpuLoadP128 {
            rt: writable_vreg(29),
            rt2: writable_vreg(9),
            mem: PairAMode::SPPreIndexed {
                simm7: SImm7Scaled::maybe_from_i64(-1024, I8X16).unwrap(),
            },
            flags: MemFlags::trusted(),
        },
        "FD27E0AD",
        "ldp q29, q9, [sp, #-1024]!",
    ));

    insns.push((
        Inst::FpuLoadP128 {
            rt: writable_vreg(10),
            rt2: writable_vreg(20),
            mem: PairAMode::SPPostIndexed {
                simm7: SImm7Scaled::maybe_from_i64(256, I8X16).unwrap(),
            },
            flags: MemFlags::trusted(),
        },
        "EA53C8AC",
        "ldp q10, q20, [sp], #256",
    ));

    insns.push((
        Inst::FpuStoreP128 {
            rt: vreg(9),
            rt2: vreg(31),
            mem: PairAMode::SignedOffset {
                reg: stack_reg(),
                simm7: SImm7Scaled::maybe_from_i64(1008, I8X16).unwrap(),
            },
            flags: MemFlags::trusted(),
        },
        "E9FF1FAD",
        "stp q9, q31, [sp, #1008]",
    ));

    insns.push((
        Inst::FpuStoreP128 {
            rt: vreg(27),
            rt2: vreg(13),
            mem: PairAMode::SPPreIndexed {
                simm7: SImm7Scaled::maybe_from_i64(-192, I8X16).unwrap(),
            },
            flags: MemFlags::trusted(),
        },
        "FB37BAAD",
        "stp q27, q13, [sp, #-192]!",
    ));

    insns.push((
        Inst::FpuStoreP128 {
            rt: vreg(18),
            rt2: vreg(22),
            mem: PairAMode::SPPostIndexed {
                simm7: SImm7Scaled::maybe_from_i64(304, I8X16).unwrap(),
            },
            flags: MemFlags::trusted(),
        },
        "F2DB89AC",
        "stp q18, q22, [sp], #304",
    ));

    insns.push((
        Inst::FpuCSel16 {
            rd: writable_vreg(1),
            rn: vreg(2),
            rm: vreg(3),
            cond: Cond::Hi,
        },
        "418CE31E",
        "fcsel h1, h2, h3, hi",
    ));

    insns.push((
        Inst::FpuCSel32 {
            rd: writable_vreg(1),
            rn: vreg(2),
            rm: vreg(3),
            cond: Cond::Hi,
        },
        "418C231E",
        "fcsel s1, s2, s3, hi",
    ));

    insns.push((
        Inst::FpuCSel64 {
            rd: writable_vreg(1),
            rn: vreg(2),
            rm: vreg(3),
            cond: Cond::Eq,
        },
        "410C631E",
        "fcsel d1, d2, d3, eq",
    ));

    insns.push((
        Inst::FpuRound {
            rd: writable_vreg(23),
            rn: vreg(24),
            op: FpuRoundMode::Minus32,
        },
        "1743251E",
        "frintm s23, s24",
    ));
    insns.push((
        Inst::FpuRound {
            rd: writable_vreg(23),
            rn: vreg(24),
            op: FpuRoundMode::Minus64,
        },
        "1743651E",
        "frintm d23, d24",
    ));
    insns.push((
        Inst::FpuRound {
            rd: writable_vreg(23),
            rn: vreg(24),
            op: FpuRoundMode::Plus32,
        },
        "17C3241E",
        "frintp s23, s24",
    ));
    insns.push((
        Inst::FpuRound {
            rd: writable_vreg(23),
            rn: vreg(24),
            op: FpuRoundMode::Plus64,
        },
        "17C3641E",
        "frintp d23, d24",
    ));
    insns.push((
        Inst::FpuRound {
            rd: writable_vreg(23),
            rn: vreg(24),
            op: FpuRoundMode::Zero32,
        },
        "17C3251E",
        "frintz s23, s24",
    ));
    insns.push((
        Inst::FpuRound {
            rd: writable_vreg(23),
            rn: vreg(24),
            op: FpuRoundMode::Zero64,
        },
        "17C3651E",
        "frintz d23, d24",
    ));
    insns.push((
        Inst::FpuRound {
            rd: writable_vreg(23),
            rn: vreg(24),
            op: FpuRoundMode::Nearest32,
        },
        "1743241E",
        "frintn s23, s24",
    ));
    insns.push((
        Inst::FpuRound {
            rd: writable_vreg(23),
            rn: vreg(24),
            op: FpuRoundMode::Nearest64,
        },
        "1743641E",
        "frintn d23, d24",
    ));

    insns.push((
        Inst::AtomicRMWLoop {
            ty: I8,
            op: AtomicRMWLoopOp::Sub,
            flags: MemFlags::trusted(),
            addr: xreg(25),
            operand: xreg(26),
            oldval: writable_xreg(27),
            scratch1: writable_xreg(24),
            scratch2: writable_xreg(28),
        },
        "3BFF5F087C031A4B3CFF1808B8FFFFB5",
        "atomic_rmw_loop_sub_8 addr=x25 operand=x26 oldval=x27 scratch1=x24 scratch2=x28",
    ));
    insns.push((
        Inst::AtomicRMWLoop {
            ty: I16,
            op: AtomicRMWLoopOp::Eor,
            flags: MemFlags::trusted(),
            addr: xreg(25),
            operand: xreg(26),
            oldval: writable_xreg(27),
            scratch1: writable_xreg(24),
            scratch2: writable_xreg(28),
        },
        "3BFF5F487C031A4A3CFF1848B8FFFFB5",
        "atomic_rmw_loop_eor_16 addr=x25 operand=x26 oldval=x27 scratch1=x24 scratch2=x28",
    ));
    insns.push((
        Inst::AtomicRMWLoop {
            ty: I8,
            op: AtomicRMWLoopOp::Add,
            flags: MemFlags::trusted(),
            addr: xreg(25),
            operand: xreg(26),
            oldval: writable_xreg(27),
            scratch1: writable_xreg(24),
            scratch2: writable_xreg(28),
        },
        "3BFF5F087C031A0B3CFF1808B8FFFFB5",
        "atomic_rmw_loop_add_8 addr=x25 operand=x26 oldval=x27 scratch1=x24 scratch2=x28",
    ));
    insns.push((
        Inst::AtomicRMWLoop {
            ty: I32,
            op: AtomicRMWLoopOp::Orr,
            flags: MemFlags::trusted(),
            addr: xreg(25),
            operand: xreg(26),
            oldval: writable_xreg(27),
            scratch1: writable_xreg(24),
            scratch2: writable_xreg(28),
        },
        "3BFF5F887C031A2A3CFF1888B8FFFFB5",
        "atomic_rmw_loop_orr_32 addr=x25 operand=x26 oldval=x27 scratch1=x24 scratch2=x28",
    ));
    insns.push((
        Inst::AtomicRMWLoop {
            ty: I64,
            op: AtomicRMWLoopOp::And,
            flags: MemFlags::trusted(),
            addr: xreg(25),
            operand: xreg(26),
            oldval: writable_xreg(27),
            scratch1: writable_xreg(24),
            scratch2: writable_xreg(28),
        },
        "3BFF5FC87C031A8A3CFF18C8B8FFFFB5",
        "atomic_rmw_loop_and_64 addr=x25 operand=x26 oldval=x27 scratch1=x24 scratch2=x28",
    ));
    insns.push((
        Inst::AtomicRMWLoop {
            ty: I8,
            op: AtomicRMWLoopOp::Xchg,
            flags: MemFlags::trusted(),
            addr: xreg(25),
            operand: xreg(26),
            oldval: writable_xreg(27),
            scratch1: writable_xreg(24),
            scratch2: writable_xreg(28),
        },
        "3BFF5F083AFF1808D8FFFFB5",
        "atomic_rmw_loop_xchg_8 addr=x25 operand=x26 oldval=x27 scratch1=x24 scratch2=x28",
    ));
    insns.push((
        Inst::AtomicRMWLoop {
            ty: I16,
            op: AtomicRMWLoopOp::Nand,
            flags: MemFlags::trusted(),
            addr: xreg(25),
            operand: xreg(26),
            oldval: writable_xreg(27),
            scratch1: writable_xreg(24),
            scratch2: writable_xreg(28),
        },
        "3BFF5F487C031A0AFC033C2A3CFF184898FFFFB5",
        "atomic_rmw_loop_nand_16 addr=x25 operand=x26 oldval=x27 scratch1=x24 scratch2=x28",
    ));
    insns.push((
        Inst::AtomicRMWLoop {
            ty: I16,
            op: AtomicRMWLoopOp::Smin,
            flags: MemFlags::trusted(),
            addr: xreg(25),
            operand: xreg(26),
            oldval: writable_xreg(27),
            scratch1: writable_xreg(24),
            scratch2: writable_xreg(28),
        },
        "3BFF5F487B3F00137FA33A6B7CB39A9A3CFF184878FFFFB5",
        "atomic_rmw_loop_smin_16 addr=x25 operand=x26 oldval=x27 scratch1=x24 scratch2=x28",
    ));
    insns.push((
        Inst::AtomicRMWLoop {
            ty: I32,
            op: AtomicRMWLoopOp::Smin,
            flags: MemFlags::trusted(),
            addr: xreg(25),
            operand: xreg(26),
            oldval: writable_xreg(27),
            scratch1: writable_xreg(24),
            scratch2: writable_xreg(28),
        },
        "3BFF5F887F031A6B7CB39A9A3CFF188898FFFFB5",
        "atomic_rmw_loop_smin_32 addr=x25 operand=x26 oldval=x27 scratch1=x24 scratch2=x28",
    ));
    insns.push((
        Inst::AtomicRMWLoop {
            ty: I64,
            op: AtomicRMWLoopOp::Smax,
            flags: MemFlags::trusted(),
            addr: xreg(25),
            operand: xreg(26),
            oldval: writable_xreg(27),
            scratch1: writable_xreg(24),
            scratch2: writable_xreg(28),
        },
        "3BFF5FC87F031AEB7CC39A9A3CFF18C898FFFFB5",
        "atomic_rmw_loop_smax_64 addr=x25 operand=x26 oldval=x27 scratch1=x24 scratch2=x28",
    ));
    insns.push((
        Inst::AtomicRMWLoop {
            ty: I8,
            op: AtomicRMWLoopOp::Smax,
            flags: MemFlags::trusted(),
            addr: xreg(25),
            operand: xreg(26),
            oldval: writable_xreg(27),
            scratch1: writable_xreg(24),
            scratch2: writable_xreg(28),
        },
        "3BFF5F087B1F00137F833A6B7CC39A9A3CFF180878FFFFB5",
        "atomic_rmw_loop_smax_8 addr=x25 operand=x26 oldval=x27 scratch1=x24 scratch2=x28",
    ));
    insns.push((
        Inst::AtomicRMWLoop {
            ty: I8,
            op: AtomicRMWLoopOp::Umin,
            flags: MemFlags::trusted(),
            addr: xreg(25),
            operand: xreg(26),
            oldval: writable_xreg(27),
            scratch1: writable_xreg(24),
            scratch2: writable_xreg(28),
        },
        "3BFF5F087F031A6B7C339A9A3CFF180898FFFFB5",
        "atomic_rmw_loop_umin_8 addr=x25 operand=x26 oldval=x27 scratch1=x24 scratch2=x28",
    ));
    insns.push((
        Inst::AtomicRMWLoop {
            ty: I16,
            op: AtomicRMWLoopOp::Umax,
            flags: MemFlags::trusted(),
            addr: xreg(25),
            operand: xreg(26),
            oldval: writable_xreg(27),
            scratch1: writable_xreg(24),
            scratch2: writable_xreg(28),
        },
        "3BFF5F487F031A6B7C839A9A3CFF184898FFFFB5",
        "atomic_rmw_loop_umax_16 addr=x25 operand=x26 oldval=x27 scratch1=x24 scratch2=x28",
    ));

    insns.push((
        Inst::AtomicRMW {
            ty: I8,
            op: AtomicRMWOp::Add,
            rs: xreg(1),
            rt: writable_xreg(2),
            rn: xreg(3),
            flags: MemFlags::trusted(),
        },
        "6200E138",
        "ldaddalb w1, w2, [x3]",
    ));
    insns.push((
        Inst::AtomicRMW {
            ty: I16,
            op: AtomicRMWOp::Add,
            rs: xreg(4),
            rt: writable_xreg(5),
            rn: xreg(6),
            flags: MemFlags::trusted(),
        },
        "C500E478",
        "ldaddalh w4, w5, [x6]",
    ));
    insns.push((
        Inst::AtomicRMW {
            ty: I32,
            op: AtomicRMWOp::Add,
            rs: xreg(7),
            rt: writable_xreg(8),
            rn: xreg(9),
            flags: MemFlags::trusted(),
        },
        "2801E7B8",
        "ldaddal w7, w8, [x9]",
    ));
    insns.push((
        Inst::AtomicRMW {
            ty: I64,
            op: AtomicRMWOp::Add,
            rs: xreg(10),
            rt: writable_xreg(11),
            rn: xreg(12),
            flags: MemFlags::trusted(),
        },
        "8B01EAF8",
        "ldaddal x10, x11, [x12]",
    ));
    insns.push((
        Inst::AtomicRMW {
            ty: I8,
            op: AtomicRMWOp::Clr,
            rs: xreg(13),
            rt: writable_xreg(14),
            rn: xreg(15),
            flags: MemFlags::trusted(),
        },
        "EE11ED38",
        "ldclralb w13, w14, [x15]",
    ));
    insns.push((
        Inst::AtomicRMW {
            ty: I16,
            op: AtomicRMWOp::Clr,
            rs: xreg(16),
            rt: writable_xreg(17),
            rn: xreg(18),
            flags: MemFlags::trusted(),
        },
        "5112F078",
        "ldclralh w16, w17, [x18]",
    ));
    insns.push((
        Inst::AtomicRMW {
            ty: I32,
            op: AtomicRMWOp::Clr,
            rs: xreg(19),
            rt: writable_xreg(20),
            rn: xreg(21),
            flags: MemFlags::trusted(),
        },
        "B412F3B8",
        "ldclral w19, w20, [x21]",
    ));
    insns.push((
        Inst::AtomicRMW {
            ty: I64,
            op: AtomicRMWOp::Clr,
            rs: xreg(22),
            rt: writable_xreg(23),
            rn: xreg(24),
            flags: MemFlags::trusted(),
        },
        "1713F6F8",
        "ldclral x22, x23, [x24]",
    ));
    insns.push((
        Inst::AtomicRMW {
            ty: I8,
            op: AtomicRMWOp::Eor,
            rs: xreg(25),
            rt: writable_xreg(26),
            rn: xreg(27),
            flags: MemFlags::trusted(),
        },
        "7A23F938",
        "ldeoralb w25, w26, [x27]",
    ));
    insns.push((
        Inst::AtomicRMW {
            ty: I16,
            op: AtomicRMWOp::Eor,
            rs: xreg(28),
            rt: writable_xreg(29),
            rn: xreg(30),
            flags: MemFlags::trusted(),
        },
        "DD23FC78",
        "ldeoralh w28, fp, [lr]",
    ));
    insns.push((
        Inst::AtomicRMW {
            ty: I32,
            op: AtomicRMWOp::Eor,
            rs: xreg(29),
            rt: writable_xreg(28),
            rn: xreg(27),
            flags: MemFlags::trusted(),
        },
        "7C23FDB8",
        "ldeoral fp, w28, [x27]",
    ));
    insns.push((
        Inst::AtomicRMW {
            ty: I64,
            op: AtomicRMWOp::Eor,
            rs: xreg(26),
            rt: writable_xreg(25),
            rn: xreg(24),
            flags: MemFlags::trusted(),
        },
        "1923FAF8",
        "ldeoral x26, x25, [x24]",
    ));
    insns.push((
        Inst::AtomicRMW {
            ty: I8,
            op: AtomicRMWOp::Set,
            rs: xreg(23),
            rt: writable_xreg(22),
            rn: xreg(21),
            flags: MemFlags::trusted(),
        },
        "B632F738",
        "ldsetalb w23, w22, [x21]",
    ));
    insns.push((
        Inst::AtomicRMW {
            ty: I16,
            op: AtomicRMWOp::Set,
            rs: xreg(20),
            rt: writable_xreg(19),
            rn: xreg(18),
            flags: MemFlags::trusted(),
        },
        "5332F478",
        "ldsetalh w20, w19, [x18]",
    ));
    insns.push((
        Inst::AtomicRMW {
            ty: I32,
            op: AtomicRMWOp::Set,
            rs: xreg(17),
            rt: writable_xreg(16),
            rn: xreg(15),
            flags: MemFlags::trusted(),
        },
        "F031F1B8",
        "ldsetal w17, w16, [x15]",
    ));
    insns.push((
        Inst::AtomicRMW {
            ty: I64,
            op: AtomicRMWOp::Set,
            rs: xreg(14),
            rt: writable_xreg(13),
            rn: xreg(12),
            flags: MemFlags::trusted(),
        },
        "8D31EEF8",
        "ldsetal x14, x13, [x12]",
    ));
    insns.push((
        Inst::AtomicRMW {
            ty: I8,
            op: AtomicRMWOp::Smax,
            rs: xreg(11),
            rt: writable_xreg(10),
            rn: xreg(9),
            flags: MemFlags::trusted(),
        },
        "2A41EB38",
        "ldsmaxalb w11, w10, [x9]",
    ));
    insns.push((
        Inst::AtomicRMW {
            ty: I16,
            op: AtomicRMWOp::Smax,
            rs: xreg(8),
            rt: writable_xreg(7),
            rn: xreg(6),
            flags: MemFlags::trusted(),
        },
        "C740E878",
        "ldsmaxalh w8, w7, [x6]",
    ));
    insns.push((
        Inst::AtomicRMW {
            ty: I32,
            op: AtomicRMWOp::Smax,
            rs: xreg(5),
            rt: writable_xreg(4),
            rn: xreg(3),
            flags: MemFlags::trusted(),
        },
        "6440E5B8",
        "ldsmaxal w5, w4, [x3]",
    ));
    insns.push((
        Inst::AtomicRMW {
            ty: I64,
            op: AtomicRMWOp::Smax,
            rs: xreg(2),
            rt: writable_xreg(1),
            rn: xreg(0),
            flags: MemFlags::trusted(),
        },
        "0140E2F8",
        "ldsmaxal x2, x1, [x0]",
    ));
    insns.push((
        Inst::AtomicRMW {
            ty: I8,
            op: AtomicRMWOp::Smin,
            rs: xreg(1),
            rt: writable_xreg(2),
            rn: xreg(3),
            flags: MemFlags::trusted(),
        },
        "6250E138",
        "ldsminalb w1, w2, [x3]",
    ));
    insns.push((
        Inst::AtomicRMW {
            ty: I16,
            op: AtomicRMWOp::Smin,
            rs: xreg(4),
            rt: writable_xreg(5),
            rn: xreg(6),
            flags: MemFlags::trusted(),
        },
        "C550E478",
        "ldsminalh w4, w5, [x6]",
    ));
    insns.push((
        Inst::AtomicRMW {
            ty: I32,
            op: AtomicRMWOp::Smin,
            rs: xreg(7),
            rt: writable_xreg(8),
            rn: xreg(9),
            flags: MemFlags::trusted(),
        },
        "2851E7B8",
        "ldsminal w7, w8, [x9]",
    ));
    insns.push((
        Inst::AtomicRMW {
            ty: I64,
            op: AtomicRMWOp::Smin,
            rs: xreg(10),
            rt: writable_xreg(11),
            rn: xreg(12),
            flags: MemFlags::trusted(),
        },
        "8B51EAF8",
        "ldsminal x10, x11, [x12]",
    ));
    insns.push((
        Inst::AtomicRMW {
            ty: I8,
            op: AtomicRMWOp::Umax,
            rs: xreg(13),
            rt: writable_xreg(14),
            rn: xreg(15),
            flags: MemFlags::trusted(),
        },
        "EE61ED38",
        "ldumaxalb w13, w14, [x15]",
    ));
    insns.push((
        Inst::AtomicRMW {
            ty: I16,
            op: AtomicRMWOp::Umax,
            rs: xreg(16),
            rt: writable_xreg(17),
            rn: xreg(18),
            flags: MemFlags::trusted(),
        },
        "5162F078",
        "ldumaxalh w16, w17, [x18]",
    ));
    insns.push((
        Inst::AtomicRMW {
            ty: I32,
            op: AtomicRMWOp::Umax,
            rs: xreg(19),
            rt: writable_xreg(20),
            rn: xreg(21),
            flags: MemFlags::trusted(),
        },
        "B462F3B8",
        "ldumaxal w19, w20, [x21]",
    ));
    insns.push((
        Inst::AtomicRMW {
            ty: I64,
            op: AtomicRMWOp::Umax,
            rs: xreg(22),
            rt: writable_xreg(23),
            rn: xreg(24),
            flags: MemFlags::trusted(),
        },
        "1763F6F8",
        "ldumaxal x22, x23, [x24]",
    ));
    insns.push((
        Inst::AtomicRMW {
            ty: I8,
            op: AtomicRMWOp::Umin,
            rs: xreg(16),
            rt: writable_xreg(17),
            rn: xreg(18),
            flags: MemFlags::trusted(),
        },
        "5172F038",
        "lduminalb w16, w17, [x18]",
    ));
    insns.push((
        Inst::AtomicRMW {
            ty: I16,
            op: AtomicRMWOp::Umin,
            rs: xreg(19),
            rt: writable_xreg(20),
            rn: xreg(21),
            flags: MemFlags::trusted(),
        },
        "B472F378",
        "lduminalh w19, w20, [x21]",
    ));
    insns.push((
        Inst::AtomicRMW {
            ty: I32,
            op: AtomicRMWOp::Umin,
            rs: xreg(22),
            rt: writable_xreg(23),
            rn: xreg(24),
            flags: MemFlags::trusted(),
        },
        "1773F6B8",
        "lduminal w22, w23, [x24]",
    ));
    insns.push((
        Inst::AtomicRMW {
            ty: I64,
            op: AtomicRMWOp::Umin,
            rs: xreg(25),
            rt: writable_xreg(26),
            rn: xreg(27),
            flags: MemFlags::trusted(),
        },
        "7A73F9F8",
        "lduminal x25, x26, [x27]",
    ));
    insns.push((
        Inst::AtomicRMW {
            ty: I8,
            op: AtomicRMWOp::Swp,
            rs: xreg(28),
            rt: writable_xreg(29),
            rn: xreg(30),
            flags: MemFlags::trusted(),
        },
        "DD83FC38",
        "swpalb w28, fp, [lr]",
    ));
    insns.push((
        Inst::AtomicRMW {
            ty: I16,
            op: AtomicRMWOp::Swp,
            rs: xreg(0),
            rt: writable_xreg(1),
            rn: xreg(2),
            flags: MemFlags::trusted(),
        },
        "4180E078",
        "swpalh w0, w1, [x2]",
    ));
    insns.push((
        Inst::AtomicRMW {
            ty: I32,
            op: AtomicRMWOp::Swp,
            rs: xreg(3),
            rt: writable_xreg(4),
            rn: xreg(5),
            flags: MemFlags::trusted(),
        },
        "A480E3B8",
        "swpal w3, w4, [x5]",
    ));
    insns.push((
        Inst::AtomicRMW {
            ty: I64,
            op: AtomicRMWOp::Swp,
            rs: xreg(6),
            rt: writable_xreg(7),
            rn: xreg(8),
            flags: MemFlags::trusted(),
        },
        "0781E6F8",
        "swpal x6, x7, [x8]",
    ));

    insns.push((
        Inst::AtomicCAS {
            rd: writable_xreg(28),
            rs: xreg(28),
            rt: xreg(20),
            rn: xreg(10),
            ty: I8,
            flags: MemFlags::trusted(),
        },
        "54FDFC08",
        "casalb w28, w28, w20, [x10]",
    ));
    insns.push((
        Inst::AtomicCAS {
            rd: writable_xreg(2),
            rs: xreg(2),
            rt: xreg(19),
            rn: xreg(23),
            ty: I16,
            flags: MemFlags::trusted(),
        },
        "F3FEE248",
        "casalh w2, w2, w19, [x23]",
    ));
    insns.push((
        Inst::AtomicCAS {
            rd: writable_xreg(0),
            rs: xreg(0),
            rt: zero_reg(),
            rn: stack_reg(),
            ty: I32,
            flags: MemFlags::trusted(),
        },
        "FFFFE088",
        "casal w0, w0, wzr, [sp]",
    ));
    insns.push((
        Inst::AtomicCAS {
            rd: writable_xreg(7),
            rs: xreg(7),
            rt: xreg(15),
            rn: xreg(27),
            ty: I64,
            flags: MemFlags::trusted(),
        },
        "6FFFE7C8",
        "casal x7, x7, x15, [x27]",
    ));
    insns.push((
        Inst::AtomicCASLoop {
            ty: I8,
            flags: MemFlags::trusted(),
            addr: xreg(25),
            expected: xreg(26),
            replacement: xreg(28),
            oldval: writable_xreg(27),
            scratch: writable_xreg(24),
        },
        "3BFF5F087F033AEB610000543CFF180898FFFFB5",
        "atomic_cas_loop_8 addr=x25, expect=x26, replacement=x28, oldval=x27, scratch=x24",
    ));

    insns.push((
        Inst::AtomicCASLoop {
            ty: I16,
            flags: MemFlags::trusted(),
            addr: xreg(25),
            expected: xreg(26),
            replacement: xreg(28),
            oldval: writable_xreg(27),
            scratch: writable_xreg(24),
        },
        "3BFF5F487F233AEB610000543CFF184898FFFFB5",
        "atomic_cas_loop_16 addr=x25, expect=x26, replacement=x28, oldval=x27, scratch=x24",
    ));

    insns.push((
        Inst::AtomicCASLoop {
            ty: I32,
            flags: MemFlags::trusted(),
            addr: xreg(25),
            expected: xreg(26),
            replacement: xreg(28),
            oldval: writable_xreg(27),
            scratch: writable_xreg(24),
        },
        "3BFF5F887F031AEB610000543CFF188898FFFFB5",
        "atomic_cas_loop_32 addr=x25, expect=x26, replacement=x28, oldval=x27, scratch=x24",
    ));

    insns.push((
        Inst::AtomicCASLoop {
            ty: I64,
            flags: MemFlags::trusted(),
            addr: xreg(25),
            expected: xreg(26),
            replacement: xreg(28),
            oldval: writable_xreg(27),
            scratch: writable_xreg(24),
        },
        "3BFF5FC87F031AEB610000543CFF18C898FFFFB5",
        "atomic_cas_loop_64 addr=x25, expect=x26, replacement=x28, oldval=x27, scratch=x24",
    ));

    insns.push((
        Inst::LoadAcquire {
            access_ty: I8,
            rt: writable_xreg(7),
            rn: xreg(28),
            flags: MemFlags::trusted(),
        },
        "87FFDF08",
        "ldarb w7, [x28]",
    ));

    insns.push((
        Inst::LoadAcquire {
            access_ty: I16,
            rt: writable_xreg(2),
            rn: xreg(3),
            flags: MemFlags::trusted(),
        },
        "62FCDF48",
        "ldarh w2, [x3]",
    ));

    insns.push((
        Inst::LoadAcquire {
            access_ty: I32,
            rt: writable_xreg(15),
            rn: xreg(0),
            flags: MemFlags::trusted(),
        },
        "0FFCDF88",
        "ldar w15, [x0]",
    ));

    insns.push((
        Inst::LoadAcquire {
            access_ty: I64,
            rt: writable_xreg(28),
            rn: xreg(7),
            flags: MemFlags::trusted(),
        },
        "FCFCDFC8",
        "ldar x28, [x7]",
    ));

    insns.push((
        Inst::StoreRelease {
            access_ty: I8,
            rt: xreg(7),
            rn: xreg(28),
            flags: MemFlags::trusted(),
        },
        "87FF9F08",
        "stlrb w7, [x28]",
    ));

    insns.push((
        Inst::StoreRelease {
            access_ty: I16,
            rt: xreg(2),
            rn: xreg(3),
            flags: MemFlags::trusted(),
        },
        "62FC9F48",
        "stlrh w2, [x3]",
    ));

    insns.push((
        Inst::StoreRelease {
            access_ty: I32,
            rt: xreg(15),
            rn: xreg(0),
            flags: MemFlags::trusted(),
        },
        "0FFC9F88",
        "stlr w15, [x0]",
    ));

    insns.push((
        Inst::StoreRelease {
            access_ty: I64,
            rt: xreg(28),
            rn: xreg(7),
            flags: MemFlags::trusted(),
        },
        "FCFC9FC8",
        "stlr x28, [x7]",
    ));

    insns.push((Inst::Fence {}, "BF3B03D5", "dmb ish"));

    let flags = settings::Flags::new(settings::builder());
    let emit_info = EmitInfo::new(flags);
    for (insn, expected_encoding, expected_printing) in insns {
        println!("AArch64: {insn:?}, {expected_encoding}, {expected_printing}");

        // Check the printed text is as expected.
        let actual_printing = insn.print_with_state(&mut EmitState::default());
        assert_eq!(expected_printing, actual_printing);

        let mut buffer = MachBuffer::new();
        insn.emit(&mut buffer, &emit_info, &mut Default::default());
        let buffer = buffer.finish(&Default::default(), &mut Default::default());
        let actual_encoding = &buffer.stringify_code_bytes();
        assert_eq!(expected_encoding, actual_encoding);
    }
}

#[test]
fn test_cond_invert() {
    for cond in vec![
        Cond::Eq,
        Cond::Ne,
        Cond::Hs,
        Cond::Lo,
        Cond::Mi,
        Cond::Pl,
        Cond::Vs,
        Cond::Vc,
        Cond::Hi,
        Cond::Ls,
        Cond::Ge,
        Cond::Lt,
        Cond::Gt,
        Cond::Le,
        Cond::Al,
        Cond::Nv,
    ]
    .into_iter()
    {
        assert_eq!(cond.invert().invert(), cond);
    }
}
