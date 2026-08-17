use crate::ir::{MemFlags, TrapCode};
use crate::isa::s390x::inst::*;
use crate::isa::s390x::settings as s390x_settings;

#[cfg(test)]
fn simm20_zero() -> SImm20 {
    SImm20::maybe_from_i64(0).unwrap()
}

#[test]
fn test_s390x_binemit() {
    let mut insns = Vec::<(Inst, &str, &str)>::new();

    insns.push((Inst::Nop0, "", "nop-zero-len"));
    insns.push((Inst::Nop2, "0707", "nop"));

    insns.push((
        Inst::AluRRR {
            alu_op: ALUOp::Add32,
            rd: writable_gpr(1),
            rn: gpr(2),
            rm: gpr(3),
        },
        "B9F83012",
        "ark %r1, %r2, %r3",
    ));
    insns.push((
        Inst::AluRRR {
            alu_op: ALUOp::Add64,
            rd: writable_gpr(4),
            rn: gpr(5),
            rm: gpr(6),
        },
        "B9E86045",
        "agrk %r4, %r5, %r6",
    ));
    insns.push((
        Inst::AluRRR {
            alu_op: ALUOp::AddLogical32,
            rd: writable_gpr(1),
            rn: gpr(2),
            rm: gpr(3),
        },
        "B9FA3012",
        "alrk %r1, %r2, %r3",
    ));
    insns.push((
        Inst::AluRRR {
            alu_op: ALUOp::AddLogical64,
            rd: writable_gpr(4),
            rn: gpr(5),
            rm: gpr(6),
        },
        "B9EA6045",
        "algrk %r4, %r5, %r6",
    ));
    insns.push((
        Inst::AluRRR {
            alu_op: ALUOp::Sub32,
            rd: writable_gpr(1),
            rn: gpr(2),
            rm: gpr(3),
        },
        "B9F93012",
        "srk %r1, %r2, %r3",
    ));
    insns.push((
        Inst::AluRRR {
            alu_op: ALUOp::Sub64,
            rd: writable_gpr(4),
            rn: gpr(5),
            rm: gpr(6),
        },
        "B9E96045",
        "sgrk %r4, %r5, %r6",
    ));
    insns.push((
        Inst::AluRRR {
            alu_op: ALUOp::SubLogical32,
            rd: writable_gpr(1),
            rn: gpr(2),
            rm: gpr(3),
        },
        "B9FB3012",
        "slrk %r1, %r2, %r3",
    ));
    insns.push((
        Inst::AluRRR {
            alu_op: ALUOp::SubLogical64,
            rd: writable_gpr(4),
            rn: gpr(5),
            rm: gpr(6),
        },
        "B9EB6045",
        "slgrk %r4, %r5, %r6",
    ));
    insns.push((
        Inst::AluRRR {
            alu_op: ALUOp::Mul32,
            rd: writable_gpr(1),
            rn: gpr(2),
            rm: gpr(3),
        },
        "B9FD3012",
        "msrkc %r1, %r2, %r3",
    ));
    insns.push((
        Inst::AluRRR {
            alu_op: ALUOp::Mul64,
            rd: writable_gpr(4),
            rn: gpr(5),
            rm: gpr(6),
        },
        "B9ED6045",
        "msgrkc %r4, %r5, %r6",
    ));
    insns.push((
        Inst::AluRRR {
            alu_op: ALUOp::And32,
            rd: writable_gpr(1),
            rn: gpr(2),
            rm: gpr(3),
        },
        "B9F43012",
        "nrk %r1, %r2, %r3",
    ));
    insns.push((
        Inst::AluRRR {
            alu_op: ALUOp::And64,
            rd: writable_gpr(4),
            rn: gpr(5),
            rm: gpr(6),
        },
        "B9E46045",
        "ngrk %r4, %r5, %r6",
    ));
    insns.push((
        Inst::AluRRR {
            alu_op: ALUOp::Orr32,
            rd: writable_gpr(1),
            rn: gpr(2),
            rm: gpr(3),
        },
        "B9F63012",
        "ork %r1, %r2, %r3",
    ));
    insns.push((
        Inst::AluRRR {
            alu_op: ALUOp::Orr64,
            rd: writable_gpr(4),
            rn: gpr(5),
            rm: gpr(6),
        },
        "B9E66045",
        "ogrk %r4, %r5, %r6",
    ));
    insns.push((
        Inst::AluRRR {
            alu_op: ALUOp::Xor32,
            rd: writable_gpr(1),
            rn: gpr(2),
            rm: gpr(3),
        },
        "B9F73012",
        "xrk %r1, %r2, %r3",
    ));
    insns.push((
        Inst::AluRRR {
            alu_op: ALUOp::Xor64,
            rd: writable_gpr(4),
            rn: gpr(5),
            rm: gpr(6),
        },
        "B9E76045",
        "xgrk %r4, %r5, %r6",
    ));
    insns.push((
        Inst::AluRRR {
            alu_op: ALUOp::NotAnd32,
            rd: writable_gpr(1),
            rn: gpr(2),
            rm: gpr(3),
        },
        "B9743012",
        "nnrk %r1, %r2, %r3",
    ));
    insns.push((
        Inst::AluRRR {
            alu_op: ALUOp::NotAnd64,
            rd: writable_gpr(4),
            rn: gpr(5),
            rm: gpr(6),
        },
        "B9646045",
        "nngrk %r4, %r5, %r6",
    ));
    insns.push((
        Inst::AluRRR {
            alu_op: ALUOp::NotOrr32,
            rd: writable_gpr(1),
            rn: gpr(2),
            rm: gpr(3),
        },
        "B9763012",
        "nork %r1, %r2, %r3",
    ));
    insns.push((
        Inst::AluRRR {
            alu_op: ALUOp::NotOrr64,
            rd: writable_gpr(4),
            rn: gpr(5),
            rm: gpr(6),
        },
        "B9666045",
        "nogrk %r4, %r5, %r6",
    ));
    insns.push((
        Inst::AluRRR {
            alu_op: ALUOp::NotXor32,
            rd: writable_gpr(1),
            rn: gpr(2),
            rm: gpr(3),
        },
        "B9773012",
        "nxrk %r1, %r2, %r3",
    ));
    insns.push((
        Inst::AluRRR {
            alu_op: ALUOp::NotXor64,
            rd: writable_gpr(4),
            rn: gpr(5),
            rm: gpr(6),
        },
        "B9676045",
        "nxgrk %r4, %r5, %r6",
    ));
    insns.push((
        Inst::AluRRR {
            alu_op: ALUOp::AndNot32,
            rd: writable_gpr(1),
            rn: gpr(2),
            rm: gpr(3),
        },
        "B9F53012",
        "ncrk %r1, %r2, %r3",
    ));
    insns.push((
        Inst::AluRRR {
            alu_op: ALUOp::AndNot64,
            rd: writable_gpr(4),
            rn: gpr(5),
            rm: gpr(6),
        },
        "B9E56045",
        "ncgrk %r4, %r5, %r6",
    ));
    insns.push((
        Inst::AluRRR {
            alu_op: ALUOp::OrrNot32,
            rd: writable_gpr(1),
            rn: gpr(2),
            rm: gpr(3),
        },
        "B9753012",
        "ocrk %r1, %r2, %r3",
    ));
    insns.push((
        Inst::AluRRR {
            alu_op: ALUOp::OrrNot64,
            rd: writable_gpr(4),
            rn: gpr(5),
            rm: gpr(6),
        },
        "B9656045",
        "ocgrk %r4, %r5, %r6",
    ));

    insns.push((
        Inst::AluRRSImm16 {
            alu_op: ALUOp::Add32,
            rd: writable_gpr(4),
            rn: gpr(5),
            imm: -32768,
        },
        "EC45800000D8",
        "ahik %r4, %r5, -32768",
    ));
    insns.push((
        Inst::AluRRSImm16 {
            alu_op: ALUOp::Add32,
            rd: writable_gpr(4),
            rn: gpr(5),
            imm: 32767,
        },
        "EC457FFF00D8",
        "ahik %r4, %r5, 32767",
    ));
    insns.push((
        Inst::AluRRSImm16 {
            alu_op: ALUOp::Add64,
            rd: writable_gpr(4),
            rn: gpr(5),
            imm: -32768,
        },
        "EC45800000D9",
        "aghik %r4, %r5, -32768",
    ));
    insns.push((
        Inst::AluRRSImm16 {
            alu_op: ALUOp::Add64,
            rd: writable_gpr(4),
            rn: gpr(5),
            imm: 32767,
        },
        "EC457FFF00D9",
        "aghik %r4, %r5, 32767",
    ));

    insns.push((
        Inst::AluRR {
            alu_op: ALUOp::Add32,
            rd: writable_gpr(1),
            ri: gpr(1),
            rm: gpr(2),
        },
        "1A12",
        "ar %r1, %r2",
    ));
    insns.push((
        Inst::AluRR {
            alu_op: ALUOp::Add64,
            rd: writable_gpr(4),
            ri: gpr(4),
            rm: gpr(5),
        },
        "B9080045",
        "agr %r4, %r5",
    ));
    insns.push((
        Inst::AluRR {
            alu_op: ALUOp::Add64Ext32,
            rd: writable_gpr(4),
            ri: gpr(4),
            rm: gpr(5),
        },
        "B9180045",
        "agfr %r4, %r5",
    ));
    insns.push((
        Inst::AluRR {
            alu_op: ALUOp::AddLogical32,
            rd: writable_gpr(1),
            ri: gpr(1),
            rm: gpr(2),
        },
        "1E12",
        "alr %r1, %r2",
    ));
    insns.push((
        Inst::AluRR {
            alu_op: ALUOp::AddLogical64,
            rd: writable_gpr(4),
            ri: gpr(4),
            rm: gpr(5),
        },
        "B90A0045",
        "algr %r4, %r5",
    ));
    insns.push((
        Inst::AluRR {
            alu_op: ALUOp::AddLogical64Ext32,
            rd: writable_gpr(4),
            ri: gpr(4),
            rm: gpr(5),
        },
        "B91A0045",
        "algfr %r4, %r5",
    ));
    insns.push((
        Inst::AluRR {
            alu_op: ALUOp::Sub32,
            rd: writable_gpr(1),
            ri: gpr(1),
            rm: gpr(2),
        },
        "1B12",
        "sr %r1, %r2",
    ));
    insns.push((
        Inst::AluRR {
            alu_op: ALUOp::Sub64,
            rd: writable_gpr(4),
            ri: gpr(4),
            rm: gpr(5),
        },
        "B9090045",
        "sgr %r4, %r5",
    ));
    insns.push((
        Inst::AluRR {
            alu_op: ALUOp::Sub64Ext32,
            rd: writable_gpr(4),
            ri: gpr(4),
            rm: gpr(5),
        },
        "B9190045",
        "sgfr %r4, %r5",
    ));
    insns.push((
        Inst::AluRR {
            alu_op: ALUOp::SubLogical32,
            rd: writable_gpr(1),
            ri: gpr(1),
            rm: gpr(2),
        },
        "1F12",
        "slr %r1, %r2",
    ));
    insns.push((
        Inst::AluRR {
            alu_op: ALUOp::SubLogical64,
            rd: writable_gpr(4),
            ri: gpr(4),
            rm: gpr(5),
        },
        "B90B0045",
        "slgr %r4, %r5",
    ));
    insns.push((
        Inst::AluRR {
            alu_op: ALUOp::SubLogical64Ext32,
            rd: writable_gpr(4),
            ri: gpr(4),
            rm: gpr(5),
        },
        "B91B0045",
        "slgfr %r4, %r5",
    ));
    insns.push((
        Inst::AluRR {
            alu_op: ALUOp::Mul32,
            rd: writable_gpr(1),
            ri: gpr(1),
            rm: gpr(2),
        },
        "B2520012",
        "msr %r1, %r2",
    ));
    insns.push((
        Inst::AluRR {
            alu_op: ALUOp::Mul64,
            rd: writable_gpr(4),
            ri: gpr(4),
            rm: gpr(5),
        },
        "B90C0045",
        "msgr %r4, %r5",
    ));
    insns.push((
        Inst::AluRR {
            alu_op: ALUOp::Mul64Ext32,
            rd: writable_gpr(4),
            ri: gpr(4),
            rm: gpr(5),
        },
        "B91C0045",
        "msgfr %r4, %r5",
    ));
    insns.push((
        Inst::AluRR {
            alu_op: ALUOp::And32,
            rd: writable_gpr(1),
            ri: gpr(1),
            rm: gpr(2),
        },
        "1412",
        "nr %r1, %r2",
    ));
    insns.push((
        Inst::AluRR {
            alu_op: ALUOp::And64,
            rd: writable_gpr(4),
            ri: gpr(4),
            rm: gpr(5),
        },
        "B9800045",
        "ngr %r4, %r5",
    ));
    insns.push((
        Inst::AluRR {
            alu_op: ALUOp::Orr32,
            rd: writable_gpr(1),
            ri: gpr(1),
            rm: gpr(2),
        },
        "1612",
        "or %r1, %r2",
    ));
    insns.push((
        Inst::AluRR {
            alu_op: ALUOp::Orr64,
            rd: writable_gpr(4),
            ri: gpr(4),
            rm: gpr(5),
        },
        "B9810045",
        "ogr %r4, %r5",
    ));
    insns.push((
        Inst::AluRR {
            alu_op: ALUOp::Xor32,
            rd: writable_gpr(1),
            ri: gpr(1),
            rm: gpr(2),
        },
        "1712",
        "xr %r1, %r2",
    ));
    insns.push((
        Inst::AluRR {
            alu_op: ALUOp::Xor64,
            rd: writable_gpr(4),
            ri: gpr(4),
            rm: gpr(5),
        },
        "B9820045",
        "xgr %r4, %r5",
    ));

    insns.push((
        Inst::AluRX {
            alu_op: ALUOp::Add32,
            rd: writable_gpr(1),
            ri: gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "5A102000",
        "a %r1, 0(%r2)",
    ));
    insns.push((
        Inst::AluRX {
            alu_op: ALUOp::Add32Ext16,
            rd: writable_gpr(1),
            ri: gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "4A102000",
        "ah %r1, 0(%r2)",
    ));
    insns.push((
        Inst::AluRX {
            alu_op: ALUOp::Add32,
            rd: writable_gpr(1),
            ri: gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(2),
                index: zero_reg(),
                disp: simm20_zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E3102000005A",
        "ay %r1, 0(%r2)",
    ));
    insns.push((
        Inst::AluRX {
            alu_op: ALUOp::Add32Ext16,
            rd: writable_gpr(1),
            ri: gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(2),
                index: zero_reg(),
                disp: simm20_zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E3102000007A",
        "ahy %r1, 0(%r2)",
    ));
    insns.push((
        Inst::AluRX {
            alu_op: ALUOp::Add64,
            rd: writable_gpr(1),
            ri: gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E31020000008",
        "ag %r1, 0(%r2)",
    ));
    insns.push((
        Inst::AluRX {
            alu_op: ALUOp::Add64Ext16,
            rd: writable_gpr(1),
            ri: gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E31020000038",
        "agh %r1, 0(%r2)",
    ));
    insns.push((
        Inst::AluRX {
            alu_op: ALUOp::Add64Ext32,
            rd: writable_gpr(1),
            ri: gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E31020000018",
        "agf %r1, 0(%r2)",
    ));
    insns.push((
        Inst::AluRX {
            alu_op: ALUOp::AddLogical32,
            rd: writable_gpr(1),
            ri: gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "5E102000",
        "al %r1, 0(%r2)",
    ));
    insns.push((
        Inst::AluRX {
            alu_op: ALUOp::AddLogical32,
            rd: writable_gpr(1),
            ri: gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(2),
                index: zero_reg(),
                disp: simm20_zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E3102000005E",
        "aly %r1, 0(%r2)",
    ));
    insns.push((
        Inst::AluRX {
            alu_op: ALUOp::AddLogical64,
            rd: writable_gpr(1),
            ri: gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E3102000000A",
        "alg %r1, 0(%r2)",
    ));
    insns.push((
        Inst::AluRX {
            alu_op: ALUOp::AddLogical64Ext32,
            rd: writable_gpr(1),
            ri: gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E3102000001A",
        "algf %r1, 0(%r2)",
    ));
    insns.push((
        Inst::AluRX {
            alu_op: ALUOp::Sub32,
            rd: writable_gpr(1),
            ri: gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "5B102000",
        "s %r1, 0(%r2)",
    ));
    insns.push((
        Inst::AluRX {
            alu_op: ALUOp::Sub32Ext16,
            rd: writable_gpr(1),
            ri: gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "4B102000",
        "sh %r1, 0(%r2)",
    ));
    insns.push((
        Inst::AluRX {
            alu_op: ALUOp::Sub32,
            rd: writable_gpr(1),
            ri: gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(2),
                index: zero_reg(),
                disp: simm20_zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E3102000005B",
        "sy %r1, 0(%r2)",
    ));
    insns.push((
        Inst::AluRX {
            alu_op: ALUOp::Sub32Ext16,
            rd: writable_gpr(1),
            ri: gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(2),
                index: zero_reg(),
                disp: simm20_zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E3102000007B",
        "shy %r1, 0(%r2)",
    ));
    insns.push((
        Inst::AluRX {
            alu_op: ALUOp::Sub64,
            rd: writable_gpr(1),
            ri: gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E31020000009",
        "sg %r1, 0(%r2)",
    ));
    insns.push((
        Inst::AluRX {
            alu_op: ALUOp::Sub64Ext16,
            rd: writable_gpr(1),
            ri: gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E31020000039",
        "sgh %r1, 0(%r2)",
    ));
    insns.push((
        Inst::AluRX {
            alu_op: ALUOp::Sub64Ext32,
            rd: writable_gpr(1),
            ri: gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E31020000019",
        "sgf %r1, 0(%r2)",
    ));
    insns.push((
        Inst::AluRX {
            alu_op: ALUOp::SubLogical32,
            rd: writable_gpr(1),
            ri: gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "5F102000",
        "sl %r1, 0(%r2)",
    ));
    insns.push((
        Inst::AluRX {
            alu_op: ALUOp::SubLogical32,
            rd: writable_gpr(1),
            ri: gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(2),
                index: zero_reg(),
                disp: simm20_zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E3102000005F",
        "sly %r1, 0(%r2)",
    ));
    insns.push((
        Inst::AluRX {
            alu_op: ALUOp::SubLogical64,
            rd: writable_gpr(1),
            ri: gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E3102000000B",
        "slg %r1, 0(%r2)",
    ));
    insns.push((
        Inst::AluRX {
            alu_op: ALUOp::SubLogical64Ext32,
            rd: writable_gpr(1),
            ri: gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E3102000001B",
        "slgf %r1, 0(%r2)",
    ));
    insns.push((
        Inst::AluRX {
            alu_op: ALUOp::Mul32,
            rd: writable_gpr(1),
            ri: gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "71102000",
        "ms %r1, 0(%r2)",
    ));
    insns.push((
        Inst::AluRX {
            alu_op: ALUOp::Mul32Ext16,
            rd: writable_gpr(1),
            ri: gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "4C102000",
        "mh %r1, 0(%r2)",
    ));
    insns.push((
        Inst::AluRX {
            alu_op: ALUOp::Mul32,
            rd: writable_gpr(1),
            ri: gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(2),
                index: zero_reg(),
                disp: simm20_zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E31020000051",
        "msy %r1, 0(%r2)",
    ));
    insns.push((
        Inst::AluRX {
            alu_op: ALUOp::Mul32Ext16,
            rd: writable_gpr(1),
            ri: gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(2),
                index: zero_reg(),
                disp: simm20_zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E3102000007C",
        "mhy %r1, 0(%r2)",
    ));
    insns.push((
        Inst::AluRX {
            alu_op: ALUOp::Mul64,
            rd: writable_gpr(1),
            ri: gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E3102000000C",
        "msg %r1, 0(%r2)",
    ));
    insns.push((
        Inst::AluRX {
            alu_op: ALUOp::Mul64Ext16,
            rd: writable_gpr(1),
            ri: gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E3102000003C",
        "mgh %r1, 0(%r2)",
    ));
    insns.push((
        Inst::AluRX {
            alu_op: ALUOp::Mul64Ext32,
            rd: writable_gpr(1),
            ri: gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E3102000001C",
        "msgf %r1, 0(%r2)",
    ));
    insns.push((
        Inst::AluRX {
            alu_op: ALUOp::And32,
            rd: writable_gpr(1),
            ri: gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "54102000",
        "n %r1, 0(%r2)",
    ));
    insns.push((
        Inst::AluRX {
            alu_op: ALUOp::And32,
            rd: writable_gpr(1),
            ri: gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(2),
                index: zero_reg(),
                disp: simm20_zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E31020000054",
        "ny %r1, 0(%r2)",
    ));
    insns.push((
        Inst::AluRX {
            alu_op: ALUOp::And64,
            rd: writable_gpr(1),
            ri: gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E31020000080",
        "ng %r1, 0(%r2)",
    ));
    insns.push((
        Inst::AluRX {
            alu_op: ALUOp::Orr32,
            rd: writable_gpr(1),
            ri: gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "56102000",
        "o %r1, 0(%r2)",
    ));
    insns.push((
        Inst::AluRX {
            alu_op: ALUOp::Orr32,
            rd: writable_gpr(1),
            ri: gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(2),
                index: zero_reg(),
                disp: simm20_zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E31020000056",
        "oy %r1, 0(%r2)",
    ));
    insns.push((
        Inst::AluRX {
            alu_op: ALUOp::Orr64,
            rd: writable_gpr(1),
            ri: gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E31020000081",
        "og %r1, 0(%r2)",
    ));
    insns.push((
        Inst::AluRX {
            alu_op: ALUOp::Xor32,
            rd: writable_gpr(1),
            ri: gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "57102000",
        "x %r1, 0(%r2)",
    ));
    insns.push((
        Inst::AluRX {
            alu_op: ALUOp::Xor32,
            rd: writable_gpr(1),
            ri: gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(2),
                index: zero_reg(),
                disp: simm20_zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E31020000057",
        "xy %r1, 0(%r2)",
    ));
    insns.push((
        Inst::AluRX {
            alu_op: ALUOp::Xor64,
            rd: writable_gpr(1),
            ri: gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E31020000082",
        "xg %r1, 0(%r2)",
    ));

    insns.push((
        Inst::AluRSImm16 {
            alu_op: ALUOp::Add32,
            rd: writable_gpr(7),
            ri: gpr(7),
            imm: -32768,
        },
        "A77A8000",
        "ahi %r7, -32768",
    ));
    insns.push((
        Inst::AluRSImm16 {
            alu_op: ALUOp::Add32,
            rd: writable_gpr(7),
            ri: gpr(7),
            imm: 32767,
        },
        "A77A7FFF",
        "ahi %r7, 32767",
    ));
    insns.push((
        Inst::AluRSImm16 {
            alu_op: ALUOp::Add64,
            rd: writable_gpr(7),
            ri: gpr(7),
            imm: -32768,
        },
        "A77B8000",
        "aghi %r7, -32768",
    ));
    insns.push((
        Inst::AluRSImm16 {
            alu_op: ALUOp::Add64,
            rd: writable_gpr(7),
            ri: gpr(7),
            imm: 32767,
        },
        "A77B7FFF",
        "aghi %r7, 32767",
    ));
    insns.push((
        Inst::AluRSImm16 {
            alu_op: ALUOp::Mul32,
            rd: writable_gpr(7),
            ri: gpr(7),
            imm: -32768,
        },
        "A77C8000",
        "mhi %r7, -32768",
    ));
    insns.push((
        Inst::AluRSImm16 {
            alu_op: ALUOp::Mul32,
            rd: writable_gpr(7),
            ri: gpr(7),
            imm: 32767,
        },
        "A77C7FFF",
        "mhi %r7, 32767",
    ));
    insns.push((
        Inst::AluRSImm16 {
            alu_op: ALUOp::Mul64,
            rd: writable_gpr(7),
            ri: gpr(7),
            imm: -32768,
        },
        "A77D8000",
        "mghi %r7, -32768",
    ));
    insns.push((
        Inst::AluRSImm16 {
            alu_op: ALUOp::Mul64,
            rd: writable_gpr(7),
            ri: gpr(7),
            imm: 32767,
        },
        "A77D7FFF",
        "mghi %r7, 32767",
    ));

    insns.push((
        Inst::AluRSImm32 {
            alu_op: ALUOp::Add32,
            rd: writable_gpr(7),
            ri: gpr(7),
            imm: -2147483648,
        },
        "C27980000000",
        "afi %r7, -2147483648",
    ));
    insns.push((
        Inst::AluRSImm32 {
            alu_op: ALUOp::Add32,
            rd: writable_gpr(7),
            ri: gpr(7),
            imm: 2147483647,
        },
        "C2797FFFFFFF",
        "afi %r7, 2147483647",
    ));
    insns.push((
        Inst::AluRSImm32 {
            alu_op: ALUOp::Mul32,
            rd: writable_gpr(7),
            ri: gpr(7),
            imm: -2147483648,
        },
        "C27180000000",
        "msfi %r7, -2147483648",
    ));
    insns.push((
        Inst::AluRSImm32 {
            alu_op: ALUOp::Mul32,
            rd: writable_gpr(7),
            ri: gpr(7),
            imm: 2147483647,
        },
        "C2717FFFFFFF",
        "msfi %r7, 2147483647",
    ));
    insns.push((
        Inst::AluRSImm32 {
            alu_op: ALUOp::Add64,
            rd: writable_gpr(7),
            ri: gpr(7),
            imm: -2147483648,
        },
        "C27880000000",
        "agfi %r7, -2147483648",
    ));
    insns.push((
        Inst::AluRSImm32 {
            alu_op: ALUOp::Add64,
            rd: writable_gpr(7),
            ri: gpr(7),
            imm: 2147483647,
        },
        "C2787FFFFFFF",
        "agfi %r7, 2147483647",
    ));
    insns.push((
        Inst::AluRSImm32 {
            alu_op: ALUOp::Mul64,
            rd: writable_gpr(7),
            ri: gpr(7),
            imm: -2147483648,
        },
        "C27080000000",
        "msgfi %r7, -2147483648",
    ));
    insns.push((
        Inst::AluRSImm32 {
            alu_op: ALUOp::Mul64,
            rd: writable_gpr(7),
            ri: gpr(7),
            imm: 2147483647,
        },
        "C2707FFFFFFF",
        "msgfi %r7, 2147483647",
    ));

    insns.push((
        Inst::AluRUImm32 {
            alu_op: ALUOp::AddLogical32,
            rd: writable_gpr(7),
            ri: gpr(7),
            imm: 0,
        },
        "C27B00000000",
        "alfi %r7, 0",
    ));
    insns.push((
        Inst::AluRUImm32 {
            alu_op: ALUOp::AddLogical32,
            rd: writable_gpr(7),
            ri: gpr(7),
            imm: 4294967295,
        },
        "C27BFFFFFFFF",
        "alfi %r7, 4294967295",
    ));
    insns.push((
        Inst::AluRUImm32 {
            alu_op: ALUOp::SubLogical32,
            rd: writable_gpr(7),
            ri: gpr(7),
            imm: 0,
        },
        "C27500000000",
        "slfi %r7, 0",
    ));
    insns.push((
        Inst::AluRUImm32 {
            alu_op: ALUOp::SubLogical32,
            rd: writable_gpr(7),
            ri: gpr(7),
            imm: 4294967295,
        },
        "C275FFFFFFFF",
        "slfi %r7, 4294967295",
    ));
    insns.push((
        Inst::AluRUImm32 {
            alu_op: ALUOp::AddLogical64,
            rd: writable_gpr(7),
            ri: gpr(7),
            imm: 0,
        },
        "C27A00000000",
        "algfi %r7, 0",
    ));
    insns.push((
        Inst::AluRUImm32 {
            alu_op: ALUOp::AddLogical64,
            rd: writable_gpr(7),
            ri: gpr(7),
            imm: 4294967295,
        },
        "C27AFFFFFFFF",
        "algfi %r7, 4294967295",
    ));
    insns.push((
        Inst::AluRUImm32 {
            alu_op: ALUOp::SubLogical64,
            rd: writable_gpr(7),
            ri: gpr(7),
            imm: 0,
        },
        "C27400000000",
        "slgfi %r7, 0",
    ));
    insns.push((
        Inst::AluRUImm32 {
            alu_op: ALUOp::SubLogical64,
            rd: writable_gpr(7),
            ri: gpr(7),
            imm: 4294967295,
        },
        "C274FFFFFFFF",
        "slgfi %r7, 4294967295",
    ));

    insns.push((
        Inst::AluRUImm16Shifted {
            alu_op: ALUOp::And32,
            rd: writable_gpr(8),
            ri: gpr(8),
            imm: UImm16Shifted::maybe_from_u64(0x0000_ffff).unwrap(),
        },
        "A587FFFF",
        "nill %r8, 65535",
    ));
    insns.push((
        Inst::AluRUImm16Shifted {
            alu_op: ALUOp::And32,
            rd: writable_gpr(8),
            ri: gpr(8),
            imm: UImm16Shifted::maybe_from_u64(0xffff_0000).unwrap(),
        },
        "A586FFFF",
        "nilh %r8, 65535",
    ));
    insns.push((
        Inst::AluRUImm16Shifted {
            alu_op: ALUOp::And64,
            rd: writable_gpr(8),
            ri: gpr(8),
            imm: UImm16Shifted::maybe_from_u64(0x0000_0000_0000_ffff).unwrap(),
        },
        "A587FFFF",
        "nill %r8, 65535",
    ));
    insns.push((
        Inst::AluRUImm16Shifted {
            alu_op: ALUOp::And64,
            rd: writable_gpr(8),
            ri: gpr(8),
            imm: UImm16Shifted::maybe_from_u64(0x0000_0000_ffff_0000).unwrap(),
        },
        "A586FFFF",
        "nilh %r8, 65535",
    ));
    insns.push((
        Inst::AluRUImm16Shifted {
            alu_op: ALUOp::And64,
            rd: writable_gpr(8),
            ri: gpr(8),
            imm: UImm16Shifted::maybe_from_u64(0x0000_ffff_0000_0000).unwrap(),
        },
        "A585FFFF",
        "nihl %r8, 65535",
    ));
    insns.push((
        Inst::AluRUImm16Shifted {
            alu_op: ALUOp::And64,
            rd: writable_gpr(8),
            ri: gpr(8),
            imm: UImm16Shifted::maybe_from_u64(0xffff_0000_0000_0000).unwrap(),
        },
        "A584FFFF",
        "nihh %r8, 65535",
    ));
    insns.push((
        Inst::AluRUImm16Shifted {
            alu_op: ALUOp::Orr32,
            rd: writable_gpr(8),
            ri: gpr(8),
            imm: UImm16Shifted::maybe_from_u64(0x0000_ffff).unwrap(),
        },
        "A58BFFFF",
        "oill %r8, 65535",
    ));
    insns.push((
        Inst::AluRUImm16Shifted {
            alu_op: ALUOp::Orr32,
            rd: writable_gpr(8),
            ri: gpr(8),
            imm: UImm16Shifted::maybe_from_u64(0xffff_0000).unwrap(),
        },
        "A58AFFFF",
        "oilh %r8, 65535",
    ));
    insns.push((
        Inst::AluRUImm16Shifted {
            alu_op: ALUOp::Orr64,
            rd: writable_gpr(8),
            ri: gpr(8),
            imm: UImm16Shifted::maybe_from_u64(0x0000_0000_0000_ffff).unwrap(),
        },
        "A58BFFFF",
        "oill %r8, 65535",
    ));
    insns.push((
        Inst::AluRUImm16Shifted {
            alu_op: ALUOp::Orr64,
            rd: writable_gpr(8),
            ri: gpr(8),
            imm: UImm16Shifted::maybe_from_u64(0x0000_0000_ffff_0000).unwrap(),
        },
        "A58AFFFF",
        "oilh %r8, 65535",
    ));
    insns.push((
        Inst::AluRUImm16Shifted {
            alu_op: ALUOp::Orr64,
            rd: writable_gpr(8),
            ri: gpr(8),
            imm: UImm16Shifted::maybe_from_u64(0x0000_ffff_0000_0000).unwrap(),
        },
        "A589FFFF",
        "oihl %r8, 65535",
    ));
    insns.push((
        Inst::AluRUImm16Shifted {
            alu_op: ALUOp::Orr64,
            rd: writable_gpr(8),
            ri: gpr(8),
            imm: UImm16Shifted::maybe_from_u64(0xffff_0000_0000_0000).unwrap(),
        },
        "A588FFFF",
        "oihh %r8, 65535",
    ));

    insns.push((
        Inst::AluRUImm32Shifted {
            alu_op: ALUOp::And32,
            rd: writable_gpr(8),
            ri: gpr(8),
            imm: UImm32Shifted::maybe_from_u64(0xffff_ffff).unwrap(),
        },
        "C08BFFFFFFFF",
        "nilf %r8, 4294967295",
    ));
    insns.push((
        Inst::AluRUImm32Shifted {
            alu_op: ALUOp::And64,
            rd: writable_gpr(8),
            ri: gpr(8),
            imm: UImm32Shifted::maybe_from_u64(0x0000_0000_ffff_ffff).unwrap(),
        },
        "C08BFFFFFFFF",
        "nilf %r8, 4294967295",
    ));
    insns.push((
        Inst::AluRUImm32Shifted {
            alu_op: ALUOp::And64,
            rd: writable_gpr(8),
            ri: gpr(8),
            imm: UImm32Shifted::maybe_from_u64(0xffff_ffff_0000_0000).unwrap(),
        },
        "C08AFFFFFFFF",
        "nihf %r8, 4294967295",
    ));
    insns.push((
        Inst::AluRUImm32Shifted {
            alu_op: ALUOp::Orr32,
            rd: writable_gpr(8),
            ri: gpr(8),
            imm: UImm32Shifted::maybe_from_u64(0xffff_ffff).unwrap(),
        },
        "C08DFFFFFFFF",
        "oilf %r8, 4294967295",
    ));
    insns.push((
        Inst::AluRUImm32Shifted {
            alu_op: ALUOp::Orr64,
            rd: writable_gpr(8),
            ri: gpr(8),
            imm: UImm32Shifted::maybe_from_u64(0x0000_0000_ffff_ffff).unwrap(),
        },
        "C08DFFFFFFFF",
        "oilf %r8, 4294967295",
    ));
    insns.push((
        Inst::AluRUImm32Shifted {
            alu_op: ALUOp::Orr64,
            rd: writable_gpr(8),
            ri: gpr(8),
            imm: UImm32Shifted::maybe_from_u64(0xffff_ffff_0000_0000).unwrap(),
        },
        "C08CFFFFFFFF",
        "oihf %r8, 4294967295",
    ));
    insns.push((
        Inst::AluRUImm32Shifted {
            alu_op: ALUOp::Xor32,
            rd: writable_gpr(8),
            ri: gpr(8),
            imm: UImm32Shifted::maybe_from_u64(0xffff_ffff).unwrap(),
        },
        "C087FFFFFFFF",
        "xilf %r8, 4294967295",
    ));
    insns.push((
        Inst::AluRUImm32Shifted {
            alu_op: ALUOp::Xor64,
            rd: writable_gpr(8),
            ri: gpr(8),
            imm: UImm32Shifted::maybe_from_u64(0x0000_0000_ffff_ffff).unwrap(),
        },
        "C087FFFFFFFF",
        "xilf %r8, 4294967295",
    ));
    insns.push((
        Inst::AluRUImm32Shifted {
            alu_op: ALUOp::Xor64,
            rd: writable_gpr(8),
            ri: gpr(8),
            imm: UImm32Shifted::maybe_from_u64(0xffff_ffff_0000_0000).unwrap(),
        },
        "C086FFFFFFFF",
        "xihf %r8, 4294967295",
    ));

    insns.push((
        Inst::UnaryRR {
            op: UnaryOp::Abs32,
            rd: writable_gpr(1),
            rn: gpr(10),
        },
        "101A",
        "lpr %r1, %r10",
    ));
    insns.push((
        Inst::UnaryRR {
            op: UnaryOp::Abs64,
            rd: writable_gpr(1),
            rn: gpr(10),
        },
        "B900001A",
        "lpgr %r1, %r10",
    ));
    insns.push((
        Inst::UnaryRR {
            op: UnaryOp::Abs64Ext32,
            rd: writable_gpr(1),
            rn: gpr(10),
        },
        "B910001A",
        "lpgfr %r1, %r10",
    ));
    insns.push((
        Inst::UnaryRR {
            op: UnaryOp::Neg32,
            rd: writable_gpr(1),
            rn: gpr(10),
        },
        "131A",
        "lcr %r1, %r10",
    ));
    insns.push((
        Inst::UnaryRR {
            op: UnaryOp::Neg64,
            rd: writable_gpr(1),
            rn: gpr(10),
        },
        "B903001A",
        "lcgr %r1, %r10",
    ));
    insns.push((
        Inst::UnaryRR {
            op: UnaryOp::Neg64Ext32,
            rd: writable_gpr(1),
            rn: gpr(10),
        },
        "B913001A",
        "lcgfr %r1, %r10",
    ));
    insns.push((
        Inst::UnaryRR {
            op: UnaryOp::PopcntByte,
            rd: writable_gpr(1),
            rn: gpr(10),
        },
        "B9E1001A",
        "popcnt %r1, %r10",
    ));
    insns.push((
        Inst::UnaryRR {
            op: UnaryOp::PopcntReg,
            rd: writable_gpr(1),
            rn: gpr(10),
        },
        "B9E1801A",
        "popcnt %r1, %r10, 8",
    ));
    insns.push((
        Inst::UnaryRR {
            op: UnaryOp::BSwap32,
            rd: writable_gpr(1),
            rn: gpr(10),
        },
        "B91F001A",
        "lrvr %r1, %r10",
    ));
    insns.push((
        Inst::UnaryRR {
            op: UnaryOp::BSwap64,
            rd: writable_gpr(1),
            rn: gpr(10),
        },
        "B90F001A",
        "lrvgr %r1, %r10",
    ));

    insns.push((
        Inst::CmpRR {
            op: CmpOp::CmpS32,
            rn: gpr(5),
            rm: gpr(6),
        },
        "1956",
        "cr %r5, %r6",
    ));
    insns.push((
        Inst::CmpRR {
            op: CmpOp::CmpS64,
            rn: gpr(5),
            rm: gpr(6),
        },
        "B9200056",
        "cgr %r5, %r6",
    ));
    insns.push((
        Inst::CmpRR {
            op: CmpOp::CmpS64Ext32,
            rn: gpr(5),
            rm: gpr(6),
        },
        "B9300056",
        "cgfr %r5, %r6",
    ));
    insns.push((
        Inst::CmpRR {
            op: CmpOp::CmpL32,
            rn: gpr(5),
            rm: gpr(6),
        },
        "1556",
        "clr %r5, %r6",
    ));
    insns.push((
        Inst::CmpRR {
            op: CmpOp::CmpL64,
            rn: gpr(5),
            rm: gpr(6),
        },
        "B9210056",
        "clgr %r5, %r6",
    ));
    insns.push((
        Inst::CmpRR {
            op: CmpOp::CmpL64Ext32,
            rn: gpr(5),
            rm: gpr(6),
        },
        "B9310056",
        "clgfr %r5, %r6",
    ));

    insns.push((
        Inst::CmpRX {
            op: CmpOp::CmpS32,
            rn: gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "59102000",
        "c %r1, 0(%r2)",
    ));
    insns.push((
        Inst::CmpRX {
            op: CmpOp::CmpS32,
            rn: gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(2),
                index: zero_reg(),
                disp: simm20_zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E31020000059",
        "cy %r1, 0(%r2)",
    ));
    insns.push((
        Inst::CmpRX {
            op: CmpOp::CmpS32,
            rn: gpr(1),
            mem: MemArg::Label {
                target: MachLabel::from_block(BlockIndex::new(1)),
            },
        },
        "C61D00000003",
        "crl %r1, label1",
    ));
    insns.push((
        Inst::CmpRX {
            op: CmpOp::CmpS32Ext16,
            rn: gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "49102000",
        "ch %r1, 0(%r2)",
    ));
    insns.push((
        Inst::CmpRX {
            op: CmpOp::CmpS32Ext16,
            rn: gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(2),
                index: zero_reg(),
                disp: simm20_zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E31020000079",
        "chy %r1, 0(%r2)",
    ));
    insns.push((
        Inst::CmpRX {
            op: CmpOp::CmpS32Ext16,
            rn: gpr(1),
            mem: MemArg::Label {
                target: MachLabel::from_block(BlockIndex::new(1)),
            },
        },
        "C61500000003",
        "chrl %r1, label1",
    ));
    insns.push((
        Inst::CmpRX {
            op: CmpOp::CmpS64,
            rn: gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E31020000020",
        "cg %r1, 0(%r2)",
    ));
    insns.push((
        Inst::CmpRX {
            op: CmpOp::CmpS64,
            rn: gpr(1),
            mem: MemArg::Label {
                target: MachLabel::from_block(BlockIndex::new(1)),
            },
        },
        "C61800000003",
        "cgrl %r1, label1",
    ));
    insns.push((
        Inst::CmpRX {
            op: CmpOp::CmpS64Ext16,
            rn: gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E31020000034",
        "cgh %r1, 0(%r2)",
    ));
    insns.push((
        Inst::CmpRX {
            op: CmpOp::CmpS64Ext16,
            rn: gpr(1),
            mem: MemArg::Label {
                target: MachLabel::from_block(BlockIndex::new(1)),
            },
        },
        "C61400000003",
        "cghrl %r1, label1",
    ));
    insns.push((
        Inst::CmpRX {
            op: CmpOp::CmpS64Ext32,
            rn: gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E31020000030",
        "cgf %r1, 0(%r2)",
    ));
    insns.push((
        Inst::CmpRX {
            op: CmpOp::CmpS64Ext32,
            rn: gpr(1),
            mem: MemArg::Label {
                target: MachLabel::from_block(BlockIndex::new(1)),
            },
        },
        "C61C00000003",
        "cgfrl %r1, label1",
    ));
    insns.push((
        Inst::CmpRX {
            op: CmpOp::CmpL32,
            rn: gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "55102000",
        "cl %r1, 0(%r2)",
    ));
    insns.push((
        Inst::CmpRX {
            op: CmpOp::CmpL32,
            rn: gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(2),
                index: zero_reg(),
                disp: simm20_zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E31020000055",
        "cly %r1, 0(%r2)",
    ));
    insns.push((
        Inst::CmpRX {
            op: CmpOp::CmpL32,
            rn: gpr(1),
            mem: MemArg::Label {
                target: MachLabel::from_block(BlockIndex::new(1)),
            },
        },
        "C61F00000003",
        "clrl %r1, label1",
    ));
    insns.push((
        Inst::CmpRX {
            op: CmpOp::CmpL32Ext16,
            rn: gpr(1),
            mem: MemArg::Label {
                target: MachLabel::from_block(BlockIndex::new(1)),
            },
        },
        "C61700000003",
        "clhrl %r1, label1",
    ));
    insns.push((
        Inst::CmpRX {
            op: CmpOp::CmpL64,
            rn: gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E31020000021",
        "clg %r1, 0(%r2)",
    ));
    insns.push((
        Inst::CmpRX {
            op: CmpOp::CmpL64,
            rn: gpr(1),
            mem: MemArg::Label {
                target: MachLabel::from_block(BlockIndex::new(1)),
            },
        },
        "C61A00000003",
        "clgrl %r1, label1",
    ));
    insns.push((
        Inst::CmpRX {
            op: CmpOp::CmpL64Ext16,
            rn: gpr(1),
            mem: MemArg::Label {
                target: MachLabel::from_block(BlockIndex::new(1)),
            },
        },
        "C61600000003",
        "clghrl %r1, label1",
    ));
    insns.push((
        Inst::CmpRX {
            op: CmpOp::CmpL64Ext32,
            rn: gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E31020000031",
        "clgf %r1, 0(%r2)",
    ));
    insns.push((
        Inst::CmpRX {
            op: CmpOp::CmpL64Ext32,
            rn: gpr(1),
            mem: MemArg::Label {
                target: MachLabel::from_block(BlockIndex::new(1)),
            },
        },
        "C61E00000003",
        "clgfrl %r1, label1",
    ));

    insns.push((
        Inst::CmpRSImm16 {
            op: CmpOp::CmpS32,
            rn: gpr(7),
            imm: -32768,
        },
        "A77E8000",
        "chi %r7, -32768",
    ));
    insns.push((
        Inst::CmpRSImm16 {
            op: CmpOp::CmpS32,
            rn: gpr(7),
            imm: 32767,
        },
        "A77E7FFF",
        "chi %r7, 32767",
    ));
    insns.push((
        Inst::CmpRSImm16 {
            op: CmpOp::CmpS64,
            rn: gpr(7),
            imm: -32768,
        },
        "A77F8000",
        "cghi %r7, -32768",
    ));
    insns.push((
        Inst::CmpRSImm16 {
            op: CmpOp::CmpS64,
            rn: gpr(7),
            imm: 32767,
        },
        "A77F7FFF",
        "cghi %r7, 32767",
    ));
    insns.push((
        Inst::CmpRSImm32 {
            op: CmpOp::CmpS32,
            rn: gpr(7),
            imm: -2147483648,
        },
        "C27D80000000",
        "cfi %r7, -2147483648",
    ));
    insns.push((
        Inst::CmpRSImm32 {
            op: CmpOp::CmpS32,
            rn: gpr(7),
            imm: 2147483647,
        },
        "C27D7FFFFFFF",
        "cfi %r7, 2147483647",
    ));
    insns.push((
        Inst::CmpRSImm32 {
            op: CmpOp::CmpS64,
            rn: gpr(7),
            imm: -2147483648,
        },
        "C27C80000000",
        "cgfi %r7, -2147483648",
    ));
    insns.push((
        Inst::CmpRSImm32 {
            op: CmpOp::CmpS64,
            rn: gpr(7),
            imm: 2147483647,
        },
        "C27C7FFFFFFF",
        "cgfi %r7, 2147483647",
    ));
    insns.push((
        Inst::CmpRUImm32 {
            op: CmpOp::CmpL32,
            rn: gpr(7),
            imm: 0,
        },
        "C27F00000000",
        "clfi %r7, 0",
    ));
    insns.push((
        Inst::CmpRUImm32 {
            op: CmpOp::CmpL32,
            rn: gpr(7),
            imm: 4294967295,
        },
        "C27FFFFFFFFF",
        "clfi %r7, 4294967295",
    ));
    insns.push((
        Inst::CmpRUImm32 {
            op: CmpOp::CmpL64,
            rn: gpr(7),
            imm: 0,
        },
        "C27E00000000",
        "clgfi %r7, 0",
    ));
    insns.push((
        Inst::CmpRUImm32 {
            op: CmpOp::CmpL64,
            rn: gpr(7),
            imm: 4294967295,
        },
        "C27EFFFFFFFF",
        "clgfi %r7, 4294967295",
    ));

    insns.push((
        Inst::CmpTrapRR {
            op: CmpOp::CmpS32,
            rn: gpr(5),
            rm: gpr(6),
            cond: Cond::from_mask(8),
            trap_code: TrapCode::STACK_OVERFLOW,
        },
        "B9728056",
        "crte %r5, %r6",
    ));
    insns.push((
        Inst::CmpTrapRR {
            op: CmpOp::CmpS64,
            rn: gpr(5),
            rm: gpr(6),
            cond: Cond::from_mask(8),
            trap_code: TrapCode::STACK_OVERFLOW,
        },
        "B9608056",
        "cgrte %r5, %r6",
    ));
    insns.push((
        Inst::CmpTrapRR {
            op: CmpOp::CmpL32,
            rn: gpr(5),
            rm: gpr(6),
            cond: Cond::from_mask(8),
            trap_code: TrapCode::STACK_OVERFLOW,
        },
        "B9738056",
        "clrte %r5, %r6",
    ));
    insns.push((
        Inst::CmpTrapRR {
            op: CmpOp::CmpL64,
            rn: gpr(5),
            rm: gpr(6),
            cond: Cond::from_mask(8),
            trap_code: TrapCode::STACK_OVERFLOW,
        },
        "B9618056",
        "clgrte %r5, %r6",
    ));
    insns.push((
        Inst::CmpTrapRSImm16 {
            op: CmpOp::CmpS32,
            rn: gpr(7),
            imm: -32768,
            cond: Cond::from_mask(8),
            trap_code: TrapCode::STACK_OVERFLOW,
        },
        "EC7080008072",
        "cite %r7, -32768",
    ));
    insns.push((
        Inst::CmpTrapRSImm16 {
            op: CmpOp::CmpS32,
            rn: gpr(7),
            imm: 32767,
            cond: Cond::from_mask(8),
            trap_code: TrapCode::STACK_OVERFLOW,
        },
        "EC707FFF8072",
        "cite %r7, 32767",
    ));
    insns.push((
        Inst::CmpTrapRSImm16 {
            op: CmpOp::CmpS64,
            rn: gpr(7),
            imm: -32768,
            cond: Cond::from_mask(8),
            trap_code: TrapCode::STACK_OVERFLOW,
        },
        "EC7080008070",
        "cgite %r7, -32768",
    ));
    insns.push((
        Inst::CmpTrapRSImm16 {
            op: CmpOp::CmpS64,
            rn: gpr(7),
            imm: 32767,
            cond: Cond::from_mask(8),
            trap_code: TrapCode::STACK_OVERFLOW,
        },
        "EC707FFF8070",
        "cgite %r7, 32767",
    ));
    insns.push((
        Inst::CmpTrapRUImm16 {
            op: CmpOp::CmpL32,
            rn: gpr(7),
            imm: 0,
            cond: Cond::from_mask(8),
            trap_code: TrapCode::STACK_OVERFLOW,
        },
        "EC7000008073",
        "clfite %r7, 0",
    ));
    insns.push((
        Inst::CmpTrapRUImm16 {
            op: CmpOp::CmpL32,
            rn: gpr(7),
            imm: 65535,
            cond: Cond::from_mask(8),
            trap_code: TrapCode::STACK_OVERFLOW,
        },
        "EC70FFFF8073",
        "clfite %r7, 65535",
    ));
    insns.push((
        Inst::CmpTrapRUImm16 {
            op: CmpOp::CmpL64,
            rn: gpr(7),
            imm: 0,
            cond: Cond::from_mask(8),
            trap_code: TrapCode::STACK_OVERFLOW,
        },
        "EC7000008071",
        "clgite %r7, 0",
    ));
    insns.push((
        Inst::CmpTrapRUImm16 {
            op: CmpOp::CmpL64,
            rn: gpr(7),
            imm: 65535,
            cond: Cond::from_mask(8),
            trap_code: TrapCode::STACK_OVERFLOW,
        },
        "EC70FFFF8071",
        "clgite %r7, 65535",
    ));

    let w_regpair = WritableRegPair {
        hi: writable_gpr(2),
        lo: writable_gpr(3),
    };
    let regpair = RegPair {
        hi: gpr(2),
        lo: gpr(3),
    };

    insns.push((
        Inst::SMulWide {
            rd: w_regpair,
            rn: gpr(5),
            rm: gpr(6),
        },
        "B9EC6025",
        "mgrk %r2, %r5, %r6",
    ));
    insns.push((
        Inst::UMulWide {
            rd: w_regpair,
            ri: gpr(3),
            rn: gpr(5),
        },
        "B9860025",
        "mlgr %r2, %r5",
    ));
    insns.push((
        Inst::SDivMod32 {
            rd: w_regpair,
            ri: gpr(3),
            rn: gpr(5),
        },
        "B91D0025",
        "dsgfr %r2, %r5",
    ));
    insns.push((
        Inst::SDivMod64 {
            rd: w_regpair,
            ri: gpr(3),
            rn: gpr(5),
        },
        "B90D0025",
        "dsgr %r2, %r5",
    ));
    insns.push((
        Inst::UDivMod32 {
            rd: w_regpair,
            ri: regpair,
            rn: gpr(5),
        },
        "B9970025",
        "dlr %r2, %r5",
    ));
    insns.push((
        Inst::UDivMod64 {
            rd: w_regpair,
            ri: regpair,
            rn: gpr(5),
        },
        "B9870025",
        "dlgr %r2, %r5",
    ));

    insns.push((
        Inst::Flogr {
            rd: w_regpair,
            rn: gpr(5),
        },
        "B9830025",
        "flogr %r2, %r5",
    ));

    insns.push((
        Inst::ShiftRR {
            shift_op: ShiftOp::RotL32,
            rd: writable_gpr(4),
            rn: gpr(5),
            shift_imm: 0,
            shift_reg: zero_reg(),
        },
        "EB450000001D",
        "rll %r4, %r5, 0",
    ));
    insns.push((
        Inst::ShiftRR {
            shift_op: ShiftOp::RotL32,
            rd: writable_gpr(4),
            rn: gpr(5),
            shift_imm: 63,
            shift_reg: zero_reg(),
        },
        "EB45003F001D",
        "rll %r4, %r5, 63",
    ));
    insns.push((
        Inst::ShiftRR {
            shift_op: ShiftOp::RotL32,
            rd: writable_gpr(4),
            rn: gpr(5),
            shift_imm: 0,
            shift_reg: gpr(6),
        },
        "EB456000001D",
        "rll %r4, %r5, 0(%r6)",
    ));
    insns.push((
        Inst::ShiftRR {
            shift_op: ShiftOp::RotL32,
            rd: writable_gpr(4),
            rn: gpr(5),
            shift_imm: 63,
            shift_reg: gpr(6),
        },
        "EB45603F001D",
        "rll %r4, %r5, 63(%r6)",
    ));
    insns.push((
        Inst::ShiftRR {
            shift_op: ShiftOp::RotL64,
            rd: writable_gpr(4),
            rn: gpr(5),
            shift_imm: 0,
            shift_reg: zero_reg(),
        },
        "EB450000001C",
        "rllg %r4, %r5, 0",
    ));
    insns.push((
        Inst::ShiftRR {
            shift_op: ShiftOp::RotL64,
            rd: writable_gpr(4),
            rn: gpr(5),
            shift_imm: 63,
            shift_reg: zero_reg(),
        },
        "EB45003F001C",
        "rllg %r4, %r5, 63",
    ));
    insns.push((
        Inst::ShiftRR {
            shift_op: ShiftOp::RotL64,
            rd: writable_gpr(4),
            rn: gpr(5),
            shift_imm: 0,
            shift_reg: gpr(6),
        },
        "EB456000001C",
        "rllg %r4, %r5, 0(%r6)",
    ));
    insns.push((
        Inst::ShiftRR {
            shift_op: ShiftOp::RotL64,
            rd: writable_gpr(4),
            rn: gpr(5),
            shift_imm: 63,
            shift_reg: gpr(6),
        },
        "EB45603F001C",
        "rllg %r4, %r5, 63(%r6)",
    ));
    insns.push((
        Inst::ShiftRR {
            shift_op: ShiftOp::LShL32,
            rd: writable_gpr(4),
            rn: gpr(5),
            shift_imm: 0,
            shift_reg: zero_reg(),
        },
        "EB45000000DF",
        "sllk %r4, %r5, 0",
    ));
    insns.push((
        Inst::ShiftRR {
            shift_op: ShiftOp::LShL32,
            rd: writable_gpr(4),
            rn: gpr(5),
            shift_imm: 63,
            shift_reg: zero_reg(),
        },
        "EB45003F00DF",
        "sllk %r4, %r5, 63",
    ));
    insns.push((
        Inst::ShiftRR {
            shift_op: ShiftOp::LShL32,
            rd: writable_gpr(4),
            rn: gpr(5),
            shift_imm: 0,
            shift_reg: gpr(6),
        },
        "EB45600000DF",
        "sllk %r4, %r5, 0(%r6)",
    ));
    insns.push((
        Inst::ShiftRR {
            shift_op: ShiftOp::LShL32,
            rd: writable_gpr(4),
            rn: gpr(5),
            shift_imm: 63,
            shift_reg: gpr(6),
        },
        "EB45603F00DF",
        "sllk %r4, %r5, 63(%r6)",
    ));
    insns.push((
        Inst::ShiftRR {
            shift_op: ShiftOp::LShL64,
            rd: writable_gpr(4),
            rn: gpr(5),
            shift_imm: 0,
            shift_reg: zero_reg(),
        },
        "EB450000000D",
        "sllg %r4, %r5, 0",
    ));
    insns.push((
        Inst::ShiftRR {
            shift_op: ShiftOp::LShL64,
            rd: writable_gpr(4),
            rn: gpr(5),
            shift_imm: 63,
            shift_reg: zero_reg(),
        },
        "EB45003F000D",
        "sllg %r4, %r5, 63",
    ));
    insns.push((
        Inst::ShiftRR {
            shift_op: ShiftOp::LShL64,
            rd: writable_gpr(4),
            rn: gpr(5),
            shift_imm: 0,
            shift_reg: gpr(6),
        },
        "EB456000000D",
        "sllg %r4, %r5, 0(%r6)",
    ));
    insns.push((
        Inst::ShiftRR {
            shift_op: ShiftOp::LShL64,
            rd: writable_gpr(4),
            rn: gpr(5),
            shift_imm: 63,
            shift_reg: gpr(6),
        },
        "EB45603F000D",
        "sllg %r4, %r5, 63(%r6)",
    ));
    insns.push((
        Inst::ShiftRR {
            shift_op: ShiftOp::LShR32,
            rd: writable_gpr(4),
            rn: gpr(5),
            shift_imm: 0,
            shift_reg: zero_reg(),
        },
        "EB45000000DE",
        "srlk %r4, %r5, 0",
    ));
    insns.push((
        Inst::ShiftRR {
            shift_op: ShiftOp::LShR32,
            rd: writable_gpr(4),
            rn: gpr(5),
            shift_imm: 63,
            shift_reg: zero_reg(),
        },
        "EB45003F00DE",
        "srlk %r4, %r5, 63",
    ));
    insns.push((
        Inst::ShiftRR {
            shift_op: ShiftOp::LShR32,
            rd: writable_gpr(4),
            rn: gpr(5),
            shift_imm: 0,
            shift_reg: gpr(6),
        },
        "EB45600000DE",
        "srlk %r4, %r5, 0(%r6)",
    ));
    insns.push((
        Inst::ShiftRR {
            shift_op: ShiftOp::LShR32,
            rd: writable_gpr(4),
            rn: gpr(5),
            shift_imm: 63,
            shift_reg: gpr(6),
        },
        "EB45603F00DE",
        "srlk %r4, %r5, 63(%r6)",
    ));
    insns.push((
        Inst::ShiftRR {
            shift_op: ShiftOp::LShR64,
            rd: writable_gpr(4),
            rn: gpr(5),
            shift_imm: 0,
            shift_reg: zero_reg(),
        },
        "EB450000000C",
        "srlg %r4, %r5, 0",
    ));
    insns.push((
        Inst::ShiftRR {
            shift_op: ShiftOp::LShR64,
            rd: writable_gpr(4),
            rn: gpr(5),
            shift_imm: 63,
            shift_reg: zero_reg(),
        },
        "EB45003F000C",
        "srlg %r4, %r5, 63",
    ));
    insns.push((
        Inst::ShiftRR {
            shift_op: ShiftOp::LShR64,
            rd: writable_gpr(4),
            rn: gpr(5),
            shift_imm: 0,
            shift_reg: gpr(6),
        },
        "EB456000000C",
        "srlg %r4, %r5, 0(%r6)",
    ));
    insns.push((
        Inst::ShiftRR {
            shift_op: ShiftOp::LShR64,
            rd: writable_gpr(4),
            rn: gpr(5),
            shift_imm: 63,
            shift_reg: gpr(6),
        },
        "EB45603F000C",
        "srlg %r4, %r5, 63(%r6)",
    ));
    insns.push((
        Inst::ShiftRR {
            shift_op: ShiftOp::AShR32,
            rd: writable_gpr(4),
            rn: gpr(5),
            shift_imm: 0,
            shift_reg: zero_reg(),
        },
        "EB45000000DC",
        "srak %r4, %r5, 0",
    ));
    insns.push((
        Inst::ShiftRR {
            shift_op: ShiftOp::AShR32,
            rd: writable_gpr(4),
            rn: gpr(5),
            shift_imm: 63,
            shift_reg: zero_reg(),
        },
        "EB45003F00DC",
        "srak %r4, %r5, 63",
    ));
    insns.push((
        Inst::ShiftRR {
            shift_op: ShiftOp::AShR32,
            rd: writable_gpr(4),
            rn: gpr(5),
            shift_imm: 0,
            shift_reg: gpr(6),
        },
        "EB45600000DC",
        "srak %r4, %r5, 0(%r6)",
    ));
    insns.push((
        Inst::ShiftRR {
            shift_op: ShiftOp::AShR32,
            rd: writable_gpr(4),
            rn: gpr(5),
            shift_imm: 63,
            shift_reg: gpr(6),
        },
        "EB45603F00DC",
        "srak %r4, %r5, 63(%r6)",
    ));
    insns.push((
        Inst::ShiftRR {
            shift_op: ShiftOp::AShR64,
            rd: writable_gpr(4),
            rn: gpr(5),
            shift_imm: 0,
            shift_reg: zero_reg(),
        },
        "EB450000000A",
        "srag %r4, %r5, 0",
    ));
    insns.push((
        Inst::ShiftRR {
            shift_op: ShiftOp::AShR64,
            rd: writable_gpr(4),
            rn: gpr(5),
            shift_imm: 63,
            shift_reg: zero_reg(),
        },
        "EB45003F000A",
        "srag %r4, %r5, 63",
    ));
    insns.push((
        Inst::ShiftRR {
            shift_op: ShiftOp::AShR64,
            rd: writable_gpr(4),
            rn: gpr(5),
            shift_imm: 0,
            shift_reg: gpr(6),
        },
        "EB456000000A",
        "srag %r4, %r5, 0(%r6)",
    ));
    insns.push((
        Inst::ShiftRR {
            shift_op: ShiftOp::AShR64,
            rd: writable_gpr(4),
            rn: gpr(5),
            shift_imm: 63,
            shift_reg: gpr(6),
        },
        "EB45603F000A",
        "srag %r4, %r5, 63(%r6)",
    ));

    insns.push((
        Inst::RxSBG {
            op: RxSBGOp::Insert,
            rd: writable_gpr(4),
            ri: gpr(4),
            rn: gpr(5),
            start_bit: 8,
            end_bit: 32,
            rotate_amt: -16,
        },
        "EC4508203059",
        "risbgn %r4, %r5, 8, 32, 48",
    ));
    insns.push((
        Inst::RxSBG {
            op: RxSBGOp::And,
            rd: writable_gpr(4),
            ri: gpr(4),
            rn: gpr(5),
            start_bit: 8,
            end_bit: 32,
            rotate_amt: 63,
        },
        "EC4508203F54",
        "rnsbg %r4, %r5, 8, 32, 63",
    ));
    insns.push((
        Inst::RxSBG {
            op: RxSBGOp::Or,
            rd: writable_gpr(4),
            ri: gpr(4),
            rn: gpr(5),
            start_bit: 8,
            end_bit: 32,
            rotate_amt: 63,
        },
        "EC4508203F56",
        "rosbg %r4, %r5, 8, 32, 63",
    ));
    insns.push((
        Inst::RxSBG {
            op: RxSBGOp::Xor,
            rd: writable_gpr(4),
            ri: gpr(4),
            rn: gpr(5),
            start_bit: 8,
            end_bit: 32,
            rotate_amt: 63,
        },
        "EC4508203F57",
        "rxsbg %r4, %r5, 8, 32, 63",
    ));
    insns.push((
        Inst::RxSBGTest {
            op: RxSBGOp::And,
            rd: gpr(4),
            rn: gpr(5),
            start_bit: 8,
            end_bit: 32,
            rotate_amt: 63,
        },
        "EC4588203F54",
        "rnsbg %r4, %r5, 136, 32, 63",
    ));
    insns.push((
        Inst::RxSBGTest {
            op: RxSBGOp::Or,
            rd: gpr(4),
            rn: gpr(5),
            start_bit: 8,
            end_bit: 32,
            rotate_amt: 63,
        },
        "EC4588203F56",
        "rosbg %r4, %r5, 136, 32, 63",
    ));
    insns.push((
        Inst::RxSBGTest {
            op: RxSBGOp::Xor,
            rd: gpr(4),
            rn: gpr(5),
            start_bit: 8,
            end_bit: 32,
            rotate_amt: 63,
        },
        "EC4588203F57",
        "rxsbg %r4, %r5, 136, 32, 63",
    ));

    insns.push((
        Inst::AtomicRmw {
            alu_op: ALUOp::Add32,
            rd: writable_gpr(4),
            rn: gpr(5),
            mem: MemArg::BXD20 {
                base: zero_reg(),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(-524288).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "EB45000080F8",
        "laa %r4, %r5, -524288",
    ));
    insns.push((
        Inst::AtomicRmw {
            alu_op: ALUOp::Add32,
            rd: writable_gpr(4),
            rn: gpr(5),
            mem: MemArg::BXD20 {
                base: zero_reg(),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(524287).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "EB450FFF7FF8",
        "laa %r4, %r5, 524287",
    ));
    insns.push((
        Inst::AtomicRmw {
            alu_op: ALUOp::Add32,
            rd: writable_gpr(4),
            rn: gpr(5),
            mem: MemArg::BXD20 {
                base: gpr(6),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(-524288).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "EB45600080F8",
        "laa %r4, %r5, -524288(%r6)",
    ));
    insns.push((
        Inst::AtomicRmw {
            alu_op: ALUOp::Add32,
            rd: writable_gpr(4),
            rn: gpr(5),
            mem: MemArg::BXD20 {
                base: gpr(6),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(524287).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "EB456FFF7FF8",
        "laa %r4, %r5, 524287(%r6)",
    ));
    insns.push((
        Inst::AtomicRmw {
            alu_op: ALUOp::Add64,
            rd: writable_gpr(4),
            rn: gpr(5),
            mem: MemArg::BXD20 {
                base: zero_reg(),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(-524288).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "EB45000080E8",
        "laag %r4, %r5, -524288",
    ));
    insns.push((
        Inst::AtomicRmw {
            alu_op: ALUOp::Add64,
            rd: writable_gpr(4),
            rn: gpr(5),
            mem: MemArg::BXD20 {
                base: zero_reg(),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(524287).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "EB450FFF7FE8",
        "laag %r4, %r5, 524287",
    ));
    insns.push((
        Inst::AtomicRmw {
            alu_op: ALUOp::Add64,
            rd: writable_gpr(4),
            rn: gpr(5),
            mem: MemArg::BXD20 {
                base: gpr(6),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(-524288).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "EB45600080E8",
        "laag %r4, %r5, -524288(%r6)",
    ));
    insns.push((
        Inst::AtomicRmw {
            alu_op: ALUOp::Add64,
            rd: writable_gpr(4),
            rn: gpr(5),
            mem: MemArg::BXD20 {
                base: gpr(6),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(524287).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "EB456FFF7FE8",
        "laag %r4, %r5, 524287(%r6)",
    ));
    insns.push((
        Inst::AtomicRmw {
            alu_op: ALUOp::AddLogical32,
            rd: writable_gpr(4),
            rn: gpr(5),
            mem: MemArg::BXD20 {
                base: zero_reg(),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(-524288).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "EB45000080FA",
        "laal %r4, %r5, -524288",
    ));
    insns.push((
        Inst::AtomicRmw {
            alu_op: ALUOp::AddLogical32,
            rd: writable_gpr(4),
            rn: gpr(5),
            mem: MemArg::BXD20 {
                base: zero_reg(),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(524287).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "EB450FFF7FFA",
        "laal %r4, %r5, 524287",
    ));
    insns.push((
        Inst::AtomicRmw {
            alu_op: ALUOp::AddLogical32,
            rd: writable_gpr(4),
            rn: gpr(5),
            mem: MemArg::BXD20 {
                base: gpr(6),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(-524288).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "EB45600080FA",
        "laal %r4, %r5, -524288(%r6)",
    ));
    insns.push((
        Inst::AtomicRmw {
            alu_op: ALUOp::AddLogical32,
            rd: writable_gpr(4),
            rn: gpr(5),
            mem: MemArg::BXD20 {
                base: gpr(6),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(524287).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "EB456FFF7FFA",
        "laal %r4, %r5, 524287(%r6)",
    ));
    insns.push((
        Inst::AtomicRmw {
            alu_op: ALUOp::AddLogical64,
            rd: writable_gpr(4),
            rn: gpr(5),
            mem: MemArg::BXD20 {
                base: zero_reg(),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(-524288).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "EB45000080EA",
        "laalg %r4, %r5, -524288",
    ));
    insns.push((
        Inst::AtomicRmw {
            alu_op: ALUOp::AddLogical64,
            rd: writable_gpr(4),
            rn: gpr(5),
            mem: MemArg::BXD20 {
                base: zero_reg(),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(524287).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "EB450FFF7FEA",
        "laalg %r4, %r5, 524287",
    ));
    insns.push((
        Inst::AtomicRmw {
            alu_op: ALUOp::AddLogical64,
            rd: writable_gpr(4),
            rn: gpr(5),
            mem: MemArg::BXD20 {
                base: gpr(6),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(-524288).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "EB45600080EA",
        "laalg %r4, %r5, -524288(%r6)",
    ));
    insns.push((
        Inst::AtomicRmw {
            alu_op: ALUOp::AddLogical64,
            rd: writable_gpr(4),
            rn: gpr(5),
            mem: MemArg::BXD20 {
                base: gpr(6),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(524287).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "EB456FFF7FEA",
        "laalg %r4, %r5, 524287(%r6)",
    ));
    insns.push((
        Inst::AtomicRmw {
            alu_op: ALUOp::And32,
            rd: writable_gpr(4),
            rn: gpr(5),
            mem: MemArg::BXD20 {
                base: zero_reg(),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(-524288).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "EB45000080F4",
        "lan %r4, %r5, -524288",
    ));
    insns.push((
        Inst::AtomicRmw {
            alu_op: ALUOp::And32,
            rd: writable_gpr(4),
            rn: gpr(5),
            mem: MemArg::BXD20 {
                base: zero_reg(),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(524287).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "EB450FFF7FF4",
        "lan %r4, %r5, 524287",
    ));
    insns.push((
        Inst::AtomicRmw {
            alu_op: ALUOp::And32,
            rd: writable_gpr(4),
            rn: gpr(5),
            mem: MemArg::BXD20 {
                base: gpr(6),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(-524288).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "EB45600080F4",
        "lan %r4, %r5, -524288(%r6)",
    ));
    insns.push((
        Inst::AtomicRmw {
            alu_op: ALUOp::And32,
            rd: writable_gpr(4),
            rn: gpr(5),
            mem: MemArg::BXD20 {
                base: gpr(6),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(524287).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "EB456FFF7FF4",
        "lan %r4, %r5, 524287(%r6)",
    ));
    insns.push((
        Inst::AtomicRmw {
            alu_op: ALUOp::And64,
            rd: writable_gpr(4),
            rn: gpr(5),
            mem: MemArg::BXD20 {
                base: zero_reg(),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(-524288).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "EB45000080E4",
        "lang %r4, %r5, -524288",
    ));
    insns.push((
        Inst::AtomicRmw {
            alu_op: ALUOp::And64,
            rd: writable_gpr(4),
            rn: gpr(5),
            mem: MemArg::BXD20 {
                base: zero_reg(),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(524287).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "EB450FFF7FE4",
        "lang %r4, %r5, 524287",
    ));
    insns.push((
        Inst::AtomicRmw {
            alu_op: ALUOp::And64,
            rd: writable_gpr(4),
            rn: gpr(5),
            mem: MemArg::BXD20 {
                base: gpr(6),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(-524288).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "EB45600080E4",
        "lang %r4, %r5, -524288(%r6)",
    ));
    insns.push((
        Inst::AtomicRmw {
            alu_op: ALUOp::And64,
            rd: writable_gpr(4),
            rn: gpr(5),
            mem: MemArg::BXD20 {
                base: gpr(6),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(524287).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "EB456FFF7FE4",
        "lang %r4, %r5, 524287(%r6)",
    ));
    insns.push((
        Inst::AtomicRmw {
            alu_op: ALUOp::Orr32,
            rd: writable_gpr(4),
            rn: gpr(5),
            mem: MemArg::BXD20 {
                base: zero_reg(),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(-524288).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "EB45000080F6",
        "lao %r4, %r5, -524288",
    ));
    insns.push((
        Inst::AtomicRmw {
            alu_op: ALUOp::Orr32,
            rd: writable_gpr(4),
            rn: gpr(5),
            mem: MemArg::BXD20 {
                base: zero_reg(),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(524287).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "EB450FFF7FF6",
        "lao %r4, %r5, 524287",
    ));
    insns.push((
        Inst::AtomicRmw {
            alu_op: ALUOp::Orr32,
            rd: writable_gpr(4),
            rn: gpr(5),
            mem: MemArg::BXD20 {
                base: gpr(6),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(-524288).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "EB45600080F6",
        "lao %r4, %r5, -524288(%r6)",
    ));
    insns.push((
        Inst::AtomicRmw {
            alu_op: ALUOp::Orr32,
            rd: writable_gpr(4),
            rn: gpr(5),
            mem: MemArg::BXD20 {
                base: gpr(6),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(524287).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "EB456FFF7FF6",
        "lao %r4, %r5, 524287(%r6)",
    ));
    insns.push((
        Inst::AtomicRmw {
            alu_op: ALUOp::Orr64,
            rd: writable_gpr(4),
            rn: gpr(5),
            mem: MemArg::BXD20 {
                base: zero_reg(),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(-524288).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "EB45000080E6",
        "laog %r4, %r5, -524288",
    ));
    insns.push((
        Inst::AtomicRmw {
            alu_op: ALUOp::Orr64,
            rd: writable_gpr(4),
            rn: gpr(5),
            mem: MemArg::BXD20 {
                base: zero_reg(),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(524287).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "EB450FFF7FE6",
        "laog %r4, %r5, 524287",
    ));
    insns.push((
        Inst::AtomicRmw {
            alu_op: ALUOp::Orr64,
            rd: writable_gpr(4),
            rn: gpr(5),
            mem: MemArg::BXD20 {
                base: gpr(6),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(-524288).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "EB45600080E6",
        "laog %r4, %r5, -524288(%r6)",
    ));
    insns.push((
        Inst::AtomicRmw {
            alu_op: ALUOp::Orr64,
            rd: writable_gpr(4),
            rn: gpr(5),
            mem: MemArg::BXD20 {
                base: gpr(6),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(524287).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "EB456FFF7FE6",
        "laog %r4, %r5, 524287(%r6)",
    ));
    insns.push((
        Inst::AtomicRmw {
            alu_op: ALUOp::Xor32,
            rd: writable_gpr(4),
            rn: gpr(5),
            mem: MemArg::BXD20 {
                base: zero_reg(),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(-524288).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "EB45000080F7",
        "lax %r4, %r5, -524288",
    ));
    insns.push((
        Inst::AtomicRmw {
            alu_op: ALUOp::Xor32,
            rd: writable_gpr(4),
            rn: gpr(5),
            mem: MemArg::BXD20 {
                base: zero_reg(),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(524287).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "EB450FFF7FF7",
        "lax %r4, %r5, 524287",
    ));
    insns.push((
        Inst::AtomicRmw {
            alu_op: ALUOp::Xor32,
            rd: writable_gpr(4),
            rn: gpr(5),
            mem: MemArg::BXD20 {
                base: gpr(6),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(-524288).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "EB45600080F7",
        "lax %r4, %r5, -524288(%r6)",
    ));
    insns.push((
        Inst::AtomicRmw {
            alu_op: ALUOp::Xor32,
            rd: writable_gpr(4),
            rn: gpr(5),
            mem: MemArg::BXD20 {
                base: gpr(6),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(524287).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "EB456FFF7FF7",
        "lax %r4, %r5, 524287(%r6)",
    ));
    insns.push((
        Inst::AtomicRmw {
            alu_op: ALUOp::Xor64,
            rd: writable_gpr(4),
            rn: gpr(5),
            mem: MemArg::BXD20 {
                base: zero_reg(),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(-524288).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "EB45000080E7",
        "laxg %r4, %r5, -524288",
    ));
    insns.push((
        Inst::AtomicRmw {
            alu_op: ALUOp::Xor64,
            rd: writable_gpr(4),
            rn: gpr(5),
            mem: MemArg::BXD20 {
                base: zero_reg(),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(524287).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "EB450FFF7FE7",
        "laxg %r4, %r5, 524287",
    ));
    insns.push((
        Inst::AtomicRmw {
            alu_op: ALUOp::Xor64,
            rd: writable_gpr(4),
            rn: gpr(5),
            mem: MemArg::BXD20 {
                base: gpr(6),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(-524288).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "EB45600080E7",
        "laxg %r4, %r5, -524288(%r6)",
    ));
    insns.push((
        Inst::AtomicRmw {
            alu_op: ALUOp::Xor64,
            rd: writable_gpr(4),
            rn: gpr(5),
            mem: MemArg::BXD20 {
                base: gpr(6),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(524287).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "EB456FFF7FE7",
        "laxg %r4, %r5, 524287(%r6)",
    ));
    insns.push((
        Inst::AtomicCas32 {
            rd: writable_gpr(4),
            ri: gpr(4),
            rn: gpr(5),
            mem: MemArg::BXD12 {
                base: zero_reg(),
                index: zero_reg(),
                disp: UImm12::maybe_from_u64(0).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "BA450000",
        "cs %r4, %r5, 0",
    ));
    insns.push((
        Inst::AtomicCas32 {
            rd: writable_gpr(4),
            ri: gpr(4),
            rn: gpr(5),
            mem: MemArg::BXD12 {
                base: zero_reg(),
                index: zero_reg(),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "BA450FFF",
        "cs %r4, %r5, 4095",
    ));
    insns.push((
        Inst::AtomicCas32 {
            rd: writable_gpr(4),
            ri: gpr(4),
            rn: gpr(5),
            mem: MemArg::BXD20 {
                base: zero_reg(),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(-524288).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "EB4500008014",
        "csy %r4, %r5, -524288",
    ));
    insns.push((
        Inst::AtomicCas32 {
            rd: writable_gpr(4),
            ri: gpr(4),
            rn: gpr(5),
            mem: MemArg::BXD20 {
                base: zero_reg(),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(524287).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "EB450FFF7F14",
        "csy %r4, %r5, 524287",
    ));
    insns.push((
        Inst::AtomicCas32 {
            rd: writable_gpr(4),
            ri: gpr(4),
            rn: gpr(5),
            mem: MemArg::BXD12 {
                base: gpr(6),
                index: zero_reg(),
                disp: UImm12::maybe_from_u64(0).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "BA456000",
        "cs %r4, %r5, 0(%r6)",
    ));
    insns.push((
        Inst::AtomicCas32 {
            rd: writable_gpr(4),
            ri: gpr(4),
            rn: gpr(5),
            mem: MemArg::BXD12 {
                base: gpr(6),
                index: zero_reg(),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "BA456FFF",
        "cs %r4, %r5, 4095(%r6)",
    ));
    insns.push((
        Inst::AtomicCas32 {
            rd: writable_gpr(4),
            ri: gpr(4),
            rn: gpr(5),
            mem: MemArg::BXD20 {
                base: gpr(6),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(-524288).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "EB4560008014",
        "csy %r4, %r5, -524288(%r6)",
    ));
    insns.push((
        Inst::AtomicCas32 {
            rd: writable_gpr(4),
            ri: gpr(4),
            rn: gpr(5),
            mem: MemArg::BXD20 {
                base: gpr(6),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(524287).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "EB456FFF7F14",
        "csy %r4, %r5, 524287(%r6)",
    ));
    insns.push((
        Inst::AtomicCas64 {
            rd: writable_gpr(4),
            ri: gpr(4),
            rn: gpr(5),
            mem: MemArg::BXD20 {
                base: zero_reg(),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(-524288).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "EB4500008030",
        "csg %r4, %r5, -524288",
    ));
    insns.push((
        Inst::AtomicCas64 {
            rd: writable_gpr(4),
            ri: gpr(4),
            rn: gpr(5),
            mem: MemArg::BXD20 {
                base: zero_reg(),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(524287).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "EB450FFF7F30",
        "csg %r4, %r5, 524287",
    ));
    insns.push((
        Inst::AtomicCas64 {
            rd: writable_gpr(4),
            ri: gpr(4),
            rn: gpr(5),
            mem: MemArg::BXD20 {
                base: gpr(6),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(-524288).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "EB4560008030",
        "csg %r4, %r5, -524288(%r6)",
    ));
    insns.push((
        Inst::AtomicCas64 {
            rd: writable_gpr(4),
            ri: gpr(4),
            rn: gpr(5),
            mem: MemArg::BXD20 {
                base: gpr(6),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(524287).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "EB456FFF7F30",
        "csg %r4, %r5, 524287(%r6)",
    ));
    insns.push((Inst::Fence, "07E0", "bcr 14, 0"));

    insns.push((
        Inst::Load32 {
            rd: writable_gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "58102000",
        "l %r1, 0(%r2)",
    ));
    insns.push((
        Inst::Load32 {
            rd: writable_gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "58102FFF",
        "l %r1, 4095(%r2)",
    ));
    insns.push((
        Inst::Load32 {
            rd: writable_gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(2),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(-524288).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E31020008058",
        "ly %r1, -524288(%r2)",
    ));
    insns.push((
        Inst::Load32 {
            rd: writable_gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(2),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(524287).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E3102FFF7F58",
        "ly %r1, 524287(%r2)",
    ));
    insns.push((
        Inst::Load32 {
            rd: writable_gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "58123000",
        "l %r1, 0(%r2,%r3)",
    ));
    insns.push((
        Inst::Load32 {
            rd: writable_gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "58123FFF",
        "l %r1, 4095(%r2,%r3)",
    ));
    insns.push((
        Inst::Load32 {
            rd: writable_gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(3),
                index: gpr(2),
                disp: SImm20::maybe_from_i64(-524288).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E31230008058",
        "ly %r1, -524288(%r2,%r3)",
    ));
    insns.push((
        Inst::Load32 {
            rd: writable_gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(3),
                index: gpr(2),
                disp: SImm20::maybe_from_i64(524287).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E3123FFF7F58",
        "ly %r1, 524287(%r2,%r3)",
    ));
    insns.push((
        Inst::Load32ZExt8 {
            rd: writable_gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E31020000094",
        "llc %r1, 0(%r2)",
    ));
    insns.push((
        Inst::Load32ZExt8 {
            rd: writable_gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E3102FFF0094",
        "llc %r1, 4095(%r2)",
    ));
    insns.push((
        Inst::Load32ZExt8 {
            rd: writable_gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(2),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(-524288).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E31020008094",
        "llc %r1, -524288(%r2)",
    ));
    insns.push((
        Inst::Load32ZExt8 {
            rd: writable_gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(2),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(524287).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E3102FFF7F94",
        "llc %r1, 524287(%r2)",
    ));
    insns.push((
        Inst::Load32ZExt8 {
            rd: writable_gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E31230000094",
        "llc %r1, 0(%r2,%r3)",
    ));
    insns.push((
        Inst::Load32ZExt8 {
            rd: writable_gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E3123FFF0094",
        "llc %r1, 4095(%r2,%r3)",
    ));
    insns.push((
        Inst::Load32ZExt8 {
            rd: writable_gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(3),
                index: gpr(2),
                disp: SImm20::maybe_from_i64(-524288).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E31230008094",
        "llc %r1, -524288(%r2,%r3)",
    ));
    insns.push((
        Inst::Load32ZExt8 {
            rd: writable_gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(3),
                index: gpr(2),
                disp: SImm20::maybe_from_i64(524287).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E3123FFF7F94",
        "llc %r1, 524287(%r2,%r3)",
    ));
    insns.push((
        Inst::Load32SExt8 {
            rd: writable_gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E31020000076",
        "lb %r1, 0(%r2)",
    ));
    insns.push((
        Inst::Load32SExt8 {
            rd: writable_gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E3102FFF0076",
        "lb %r1, 4095(%r2)",
    ));
    insns.push((
        Inst::Load32SExt8 {
            rd: writable_gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(2),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(-524288).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E31020008076",
        "lb %r1, -524288(%r2)",
    ));
    insns.push((
        Inst::Load32SExt8 {
            rd: writable_gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(2),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(524287).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E3102FFF7F76",
        "lb %r1, 524287(%r2)",
    ));
    insns.push((
        Inst::Load32SExt8 {
            rd: writable_gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E31230000076",
        "lb %r1, 0(%r2,%r3)",
    ));
    insns.push((
        Inst::Load32SExt8 {
            rd: writable_gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E3123FFF0076",
        "lb %r1, 4095(%r2,%r3)",
    ));
    insns.push((
        Inst::Load32SExt8 {
            rd: writable_gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(3),
                index: gpr(2),
                disp: SImm20::maybe_from_i64(-524288).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E31230008076",
        "lb %r1, -524288(%r2,%r3)",
    ));
    insns.push((
        Inst::Load32SExt8 {
            rd: writable_gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(3),
                index: gpr(2),
                disp: SImm20::maybe_from_i64(524287).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E3123FFF7F76",
        "lb %r1, 524287(%r2,%r3)",
    ));
    insns.push((
        Inst::Load32ZExt16 {
            rd: writable_gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E31020000095",
        "llh %r1, 0(%r2)",
    ));
    insns.push((
        Inst::Load32ZExt16 {
            rd: writable_gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E3102FFF0095",
        "llh %r1, 4095(%r2)",
    ));
    insns.push((
        Inst::Load32ZExt16 {
            rd: writable_gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(2),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(-524288).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E31020008095",
        "llh %r1, -524288(%r2)",
    ));
    insns.push((
        Inst::Load32ZExt16 {
            rd: writable_gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(2),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(524287).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E3102FFF7F95",
        "llh %r1, 524287(%r2)",
    ));
    insns.push((
        Inst::Load32ZExt16 {
            rd: writable_gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E31230000095",
        "llh %r1, 0(%r2,%r3)",
    ));
    insns.push((
        Inst::Load32ZExt16 {
            rd: writable_gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E3123FFF0095",
        "llh %r1, 4095(%r2,%r3)",
    ));
    insns.push((
        Inst::Load32ZExt16 {
            rd: writable_gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(3),
                index: gpr(2),
                disp: SImm20::maybe_from_i64(-524288).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E31230008095",
        "llh %r1, -524288(%r2,%r3)",
    ));
    insns.push((
        Inst::Load32ZExt16 {
            rd: writable_gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(3),
                index: gpr(2),
                disp: SImm20::maybe_from_i64(524287).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E3123FFF7F95",
        "llh %r1, 524287(%r2,%r3)",
    ));
    insns.push((
        Inst::Load32SExt16 {
            rd: writable_gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "48102000",
        "lh %r1, 0(%r2)",
    ));
    insns.push((
        Inst::Load32SExt16 {
            rd: writable_gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "48102FFF",
        "lh %r1, 4095(%r2)",
    ));
    insns.push((
        Inst::Load32SExt16 {
            rd: writable_gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(2),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(-524288).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E31020008078",
        "lhy %r1, -524288(%r2)",
    ));
    insns.push((
        Inst::Load32SExt16 {
            rd: writable_gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(2),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(524287).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E3102FFF7F78",
        "lhy %r1, 524287(%r2)",
    ));
    insns.push((
        Inst::Load32SExt16 {
            rd: writable_gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "48123000",
        "lh %r1, 0(%r2,%r3)",
    ));
    insns.push((
        Inst::Load32SExt16 {
            rd: writable_gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "48123FFF",
        "lh %r1, 4095(%r2,%r3)",
    ));
    insns.push((
        Inst::Load32SExt16 {
            rd: writable_gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(3),
                index: gpr(2),
                disp: SImm20::maybe_from_i64(-524288).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E31230008078",
        "lhy %r1, -524288(%r2,%r3)",
    ));
    insns.push((
        Inst::Load32SExt16 {
            rd: writable_gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(3),
                index: gpr(2),
                disp: SImm20::maybe_from_i64(524287).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E3123FFF7F78",
        "lhy %r1, 524287(%r2,%r3)",
    ));
    insns.push((
        Inst::Load64 {
            rd: writable_gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E31020000004",
        "lg %r1, 0(%r2)",
    ));
    insns.push((
        Inst::Load64 {
            rd: writable_gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E3102FFF0004",
        "lg %r1, 4095(%r2)",
    ));
    insns.push((
        Inst::Load64 {
            rd: writable_gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(2),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(-524288).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E31020008004",
        "lg %r1, -524288(%r2)",
    ));
    insns.push((
        Inst::Load64 {
            rd: writable_gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(2),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(524287).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E3102FFF7F04",
        "lg %r1, 524287(%r2)",
    ));
    insns.push((
        Inst::Load64 {
            rd: writable_gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E31230000004",
        "lg %r1, 0(%r2,%r3)",
    ));
    insns.push((
        Inst::Load64 {
            rd: writable_gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E3123FFF0004",
        "lg %r1, 4095(%r2,%r3)",
    ));
    insns.push((
        Inst::Load64 {
            rd: writable_gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(3),
                index: gpr(2),
                disp: SImm20::maybe_from_i64(-524288).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E31230008004",
        "lg %r1, -524288(%r2,%r3)",
    ));
    insns.push((
        Inst::Load64 {
            rd: writable_gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(3),
                index: gpr(2),
                disp: SImm20::maybe_from_i64(524287).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E3123FFF7F04",
        "lg %r1, 524287(%r2,%r3)",
    ));
    insns.push((
        Inst::Load64ZExt8 {
            rd: writable_gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E31020000090",
        "llgc %r1, 0(%r2)",
    ));
    insns.push((
        Inst::Load64ZExt8 {
            rd: writable_gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E3102FFF0090",
        "llgc %r1, 4095(%r2)",
    ));
    insns.push((
        Inst::Load64ZExt8 {
            rd: writable_gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(2),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(-524288).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E31020008090",
        "llgc %r1, -524288(%r2)",
    ));
    insns.push((
        Inst::Load64ZExt8 {
            rd: writable_gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(2),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(524287).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E3102FFF7F90",
        "llgc %r1, 524287(%r2)",
    ));
    insns.push((
        Inst::Load64ZExt8 {
            rd: writable_gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E31230000090",
        "llgc %r1, 0(%r2,%r3)",
    ));
    insns.push((
        Inst::Load64ZExt8 {
            rd: writable_gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E3123FFF0090",
        "llgc %r1, 4095(%r2,%r3)",
    ));
    insns.push((
        Inst::Load64ZExt8 {
            rd: writable_gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(3),
                index: gpr(2),
                disp: SImm20::maybe_from_i64(-524288).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E31230008090",
        "llgc %r1, -524288(%r2,%r3)",
    ));
    insns.push((
        Inst::Load64ZExt8 {
            rd: writable_gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(3),
                index: gpr(2),
                disp: SImm20::maybe_from_i64(524287).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E3123FFF7F90",
        "llgc %r1, 524287(%r2,%r3)",
    ));
    insns.push((
        Inst::Load64SExt8 {
            rd: writable_gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E31020000077",
        "lgb %r1, 0(%r2)",
    ));
    insns.push((
        Inst::Load64SExt8 {
            rd: writable_gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E3102FFF0077",
        "lgb %r1, 4095(%r2)",
    ));
    insns.push((
        Inst::Load64SExt8 {
            rd: writable_gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(2),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(-524288).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E31020008077",
        "lgb %r1, -524288(%r2)",
    ));
    insns.push((
        Inst::Load64SExt8 {
            rd: writable_gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(2),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(524287).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E3102FFF7F77",
        "lgb %r1, 524287(%r2)",
    ));
    insns.push((
        Inst::Load64SExt8 {
            rd: writable_gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E31230000077",
        "lgb %r1, 0(%r2,%r3)",
    ));
    insns.push((
        Inst::Load64SExt8 {
            rd: writable_gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E3123FFF0077",
        "lgb %r1, 4095(%r2,%r3)",
    ));
    insns.push((
        Inst::Load64SExt8 {
            rd: writable_gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(3),
                index: gpr(2),
                disp: SImm20::maybe_from_i64(-524288).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E31230008077",
        "lgb %r1, -524288(%r2,%r3)",
    ));
    insns.push((
        Inst::Load64SExt8 {
            rd: writable_gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(3),
                index: gpr(2),
                disp: SImm20::maybe_from_i64(524287).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E3123FFF7F77",
        "lgb %r1, 524287(%r2,%r3)",
    ));
    insns.push((
        Inst::Load64ZExt16 {
            rd: writable_gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E31020000091",
        "llgh %r1, 0(%r2)",
    ));
    insns.push((
        Inst::Load64ZExt16 {
            rd: writable_gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E3102FFF0091",
        "llgh %r1, 4095(%r2)",
    ));
    insns.push((
        Inst::Load64ZExt16 {
            rd: writable_gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(2),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(-524288).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E31020008091",
        "llgh %r1, -524288(%r2)",
    ));
    insns.push((
        Inst::Load64ZExt16 {
            rd: writable_gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(2),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(524287).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E3102FFF7F91",
        "llgh %r1, 524287(%r2)",
    ));
    insns.push((
        Inst::Load64ZExt16 {
            rd: writable_gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E31230000091",
        "llgh %r1, 0(%r2,%r3)",
    ));
    insns.push((
        Inst::Load64ZExt16 {
            rd: writable_gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E3123FFF0091",
        "llgh %r1, 4095(%r2,%r3)",
    ));
    insns.push((
        Inst::Load64ZExt16 {
            rd: writable_gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(3),
                index: gpr(2),
                disp: SImm20::maybe_from_i64(-524288).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E31230008091",
        "llgh %r1, -524288(%r2,%r3)",
    ));
    insns.push((
        Inst::Load64ZExt16 {
            rd: writable_gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(3),
                index: gpr(2),
                disp: SImm20::maybe_from_i64(524287).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E3123FFF7F91",
        "llgh %r1, 524287(%r2,%r3)",
    ));
    insns.push((
        Inst::Load64SExt16 {
            rd: writable_gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E31020000015",
        "lgh %r1, 0(%r2)",
    ));
    insns.push((
        Inst::Load64SExt16 {
            rd: writable_gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E3102FFF0015",
        "lgh %r1, 4095(%r2)",
    ));
    insns.push((
        Inst::Load64SExt16 {
            rd: writable_gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(2),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(-524288).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E31020008015",
        "lgh %r1, -524288(%r2)",
    ));
    insns.push((
        Inst::Load64SExt16 {
            rd: writable_gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(2),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(524287).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E3102FFF7F15",
        "lgh %r1, 524287(%r2)",
    ));
    insns.push((
        Inst::Load64SExt16 {
            rd: writable_gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E31230000015",
        "lgh %r1, 0(%r2,%r3)",
    ));
    insns.push((
        Inst::Load64SExt16 {
            rd: writable_gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E3123FFF0015",
        "lgh %r1, 4095(%r2,%r3)",
    ));
    insns.push((
        Inst::Load64SExt16 {
            rd: writable_gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(3),
                index: gpr(2),
                disp: SImm20::maybe_from_i64(-524288).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E31230008015",
        "lgh %r1, -524288(%r2,%r3)",
    ));
    insns.push((
        Inst::Load64SExt16 {
            rd: writable_gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(3),
                index: gpr(2),
                disp: SImm20::maybe_from_i64(524287).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E3123FFF7F15",
        "lgh %r1, 524287(%r2,%r3)",
    ));
    insns.push((
        Inst::Load64ZExt32 {
            rd: writable_gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E31020000016",
        "llgf %r1, 0(%r2)",
    ));
    insns.push((
        Inst::Load64ZExt32 {
            rd: writable_gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E3102FFF0016",
        "llgf %r1, 4095(%r2)",
    ));
    insns.push((
        Inst::Load64ZExt32 {
            rd: writable_gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(2),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(-524288).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E31020008016",
        "llgf %r1, -524288(%r2)",
    ));
    insns.push((
        Inst::Load64ZExt32 {
            rd: writable_gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(2),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(524287).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E3102FFF7F16",
        "llgf %r1, 524287(%r2)",
    ));
    insns.push((
        Inst::Load64ZExt32 {
            rd: writable_gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E31230000016",
        "llgf %r1, 0(%r2,%r3)",
    ));
    insns.push((
        Inst::Load64ZExt32 {
            rd: writable_gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E3123FFF0016",
        "llgf %r1, 4095(%r2,%r3)",
    ));
    insns.push((
        Inst::Load64ZExt32 {
            rd: writable_gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(3),
                index: gpr(2),
                disp: SImm20::maybe_from_i64(-524288).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E31230008016",
        "llgf %r1, -524288(%r2,%r3)",
    ));
    insns.push((
        Inst::Load64ZExt32 {
            rd: writable_gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(3),
                index: gpr(2),
                disp: SImm20::maybe_from_i64(524287).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E3123FFF7F16",
        "llgf %r1, 524287(%r2,%r3)",
    ));
    insns.push((
        Inst::Load64SExt32 {
            rd: writable_gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E31020000014",
        "lgf %r1, 0(%r2)",
    ));
    insns.push((
        Inst::Load64SExt32 {
            rd: writable_gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E3102FFF0014",
        "lgf %r1, 4095(%r2)",
    ));
    insns.push((
        Inst::Load64SExt32 {
            rd: writable_gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(2),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(-524288).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E31020008014",
        "lgf %r1, -524288(%r2)",
    ));
    insns.push((
        Inst::Load64SExt32 {
            rd: writable_gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(2),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(524287).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E3102FFF7F14",
        "lgf %r1, 524287(%r2)",
    ));
    insns.push((
        Inst::Load64SExt32 {
            rd: writable_gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E31230000014",
        "lgf %r1, 0(%r2,%r3)",
    ));
    insns.push((
        Inst::Load64SExt32 {
            rd: writable_gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E3123FFF0014",
        "lgf %r1, 4095(%r2,%r3)",
    ));
    insns.push((
        Inst::Load64SExt32 {
            rd: writable_gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(3),
                index: gpr(2),
                disp: SImm20::maybe_from_i64(-524288).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E31230008014",
        "lgf %r1, -524288(%r2,%r3)",
    ));
    insns.push((
        Inst::Load64SExt32 {
            rd: writable_gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(3),
                index: gpr(2),
                disp: SImm20::maybe_from_i64(524287).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E3123FFF7F14",
        "lgf %r1, 524287(%r2,%r3)",
    ));

    insns.push((
        Inst::Load32 {
            rd: writable_gpr(1),
            mem: MemArg::Label {
                target: MachLabel::from_block(BlockIndex::new(1)),
            },
        },
        "C41D00000003",
        "lrl %r1, label1",
    ));
    insns.push((
        Inst::Load32SExt16 {
            rd: writable_gpr(1),
            mem: MemArg::Label {
                target: MachLabel::from_block(BlockIndex::new(1)),
            },
        },
        "C41500000003",
        "lhrl %r1, label1",
    ));
    insns.push((
        Inst::Load32ZExt16 {
            rd: writable_gpr(1),
            mem: MemArg::Label {
                target: MachLabel::from_block(BlockIndex::new(1)),
            },
        },
        "C41200000003",
        "llhrl %r1, label1",
    ));
    insns.push((
        Inst::Load64 {
            rd: writable_gpr(1),
            mem: MemArg::Label {
                target: MachLabel::from_block(BlockIndex::new(1)),
            },
        },
        "C41800000003",
        "lgrl %r1, label1",
    ));
    insns.push((
        Inst::Load64SExt16 {
            rd: writable_gpr(1),
            mem: MemArg::Label {
                target: MachLabel::from_block(BlockIndex::new(1)),
            },
        },
        "C41400000003",
        "lghrl %r1, label1",
    ));
    insns.push((
        Inst::Load64ZExt16 {
            rd: writable_gpr(1),
            mem: MemArg::Label {
                target: MachLabel::from_block(BlockIndex::new(1)),
            },
        },
        "C41600000003",
        "llghrl %r1, label1",
    ));
    insns.push((
        Inst::Load64SExt32 {
            rd: writable_gpr(1),
            mem: MemArg::Label {
                target: MachLabel::from_block(BlockIndex::new(1)),
            },
        },
        "C41C00000003",
        "lgfrl %r1, label1",
    ));
    insns.push((
        Inst::Load64ZExt32 {
            rd: writable_gpr(1),
            mem: MemArg::Label {
                target: MachLabel::from_block(BlockIndex::new(1)),
            },
        },
        "C41E00000003",
        "llgfrl %r1, label1",
    ));
    insns.push((
        Inst::LoadRev16 {
            rd: writable_gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E3102000001F",
        "lrvh %r1, 0(%r2)",
    ));
    insns.push((
        Inst::LoadRev16 {
            rd: writable_gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E3102FFF001F",
        "lrvh %r1, 4095(%r2)",
    ));
    insns.push((
        Inst::LoadRev16 {
            rd: writable_gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(2),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(-524288).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E3102000801F",
        "lrvh %r1, -524288(%r2)",
    ));
    insns.push((
        Inst::LoadRev16 {
            rd: writable_gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(2),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(524287).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E3102FFF7F1F",
        "lrvh %r1, 524287(%r2)",
    ));
    insns.push((
        Inst::LoadRev16 {
            rd: writable_gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E3123000001F",
        "lrvh %r1, 0(%r2,%r3)",
    ));
    insns.push((
        Inst::LoadRev16 {
            rd: writable_gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E3123FFF001F",
        "lrvh %r1, 4095(%r2,%r3)",
    ));
    insns.push((
        Inst::LoadRev16 {
            rd: writable_gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(3),
                index: gpr(2),
                disp: SImm20::maybe_from_i64(-524288).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E3123000801F",
        "lrvh %r1, -524288(%r2,%r3)",
    ));
    insns.push((
        Inst::LoadRev16 {
            rd: writable_gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(3),
                index: gpr(2),
                disp: SImm20::maybe_from_i64(524287).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E3123FFF7F1F",
        "lrvh %r1, 524287(%r2,%r3)",
    ));
    insns.push((
        Inst::LoadRev32 {
            rd: writable_gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E3102000001E",
        "lrv %r1, 0(%r2)",
    ));
    insns.push((
        Inst::LoadRev32 {
            rd: writable_gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E3102FFF001E",
        "lrv %r1, 4095(%r2)",
    ));
    insns.push((
        Inst::LoadRev32 {
            rd: writable_gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(2),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(-524288).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E3102000801E",
        "lrv %r1, -524288(%r2)",
    ));
    insns.push((
        Inst::LoadRev32 {
            rd: writable_gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(2),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(524287).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E3102FFF7F1E",
        "lrv %r1, 524287(%r2)",
    ));
    insns.push((
        Inst::LoadRev32 {
            rd: writable_gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E3123000001E",
        "lrv %r1, 0(%r2,%r3)",
    ));
    insns.push((
        Inst::LoadRev32 {
            rd: writable_gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E3123FFF001E",
        "lrv %r1, 4095(%r2,%r3)",
    ));
    insns.push((
        Inst::LoadRev32 {
            rd: writable_gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(3),
                index: gpr(2),
                disp: SImm20::maybe_from_i64(-524288).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E3123000801E",
        "lrv %r1, -524288(%r2,%r3)",
    ));
    insns.push((
        Inst::LoadRev32 {
            rd: writable_gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(3),
                index: gpr(2),
                disp: SImm20::maybe_from_i64(524287).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E3123FFF7F1E",
        "lrv %r1, 524287(%r2,%r3)",
    ));
    insns.push((
        Inst::LoadRev64 {
            rd: writable_gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E3102000000F",
        "lrvg %r1, 0(%r2)",
    ));
    insns.push((
        Inst::LoadRev64 {
            rd: writable_gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E3102FFF000F",
        "lrvg %r1, 4095(%r2)",
    ));
    insns.push((
        Inst::LoadRev64 {
            rd: writable_gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(2),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(-524288).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E3102000800F",
        "lrvg %r1, -524288(%r2)",
    ));
    insns.push((
        Inst::LoadRev64 {
            rd: writable_gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(2),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(524287).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E3102FFF7F0F",
        "lrvg %r1, 524287(%r2)",
    ));
    insns.push((
        Inst::LoadRev64 {
            rd: writable_gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E3123000000F",
        "lrvg %r1, 0(%r2,%r3)",
    ));
    insns.push((
        Inst::LoadRev64 {
            rd: writable_gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E3123FFF000F",
        "lrvg %r1, 4095(%r2,%r3)",
    ));
    insns.push((
        Inst::LoadRev64 {
            rd: writable_gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(3),
                index: gpr(2),
                disp: SImm20::maybe_from_i64(-524288).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E3123000800F",
        "lrvg %r1, -524288(%r2,%r3)",
    ));
    insns.push((
        Inst::LoadRev64 {
            rd: writable_gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(3),
                index: gpr(2),
                disp: SImm20::maybe_from_i64(524287).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E3123FFF7F0F",
        "lrvg %r1, 524287(%r2,%r3)",
    ));

    insns.push((
        Inst::Store8 {
            rd: gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "42102000",
        "stc %r1, 0(%r2)",
    ));
    insns.push((
        Inst::Store8 {
            rd: gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "42102FFF",
        "stc %r1, 4095(%r2)",
    ));
    insns.push((
        Inst::Store8 {
            rd: gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(2),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(-524288).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E31020008072",
        "stcy %r1, -524288(%r2)",
    ));
    insns.push((
        Inst::Store8 {
            rd: gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(2),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(524287).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E3102FFF7F72",
        "stcy %r1, 524287(%r2)",
    ));
    insns.push((
        Inst::Store8 {
            rd: gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "42123000",
        "stc %r1, 0(%r2,%r3)",
    ));
    insns.push((
        Inst::Store8 {
            rd: gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "42123FFF",
        "stc %r1, 4095(%r2,%r3)",
    ));
    insns.push((
        Inst::Store8 {
            rd: gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(3),
                index: gpr(2),
                disp: SImm20::maybe_from_i64(-524288).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E31230008072",
        "stcy %r1, -524288(%r2,%r3)",
    ));
    insns.push((
        Inst::Store8 {
            rd: gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(3),
                index: gpr(2),
                disp: SImm20::maybe_from_i64(524287).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E3123FFF7F72",
        "stcy %r1, 524287(%r2,%r3)",
    ));
    insns.push((
        Inst::Store16 {
            rd: gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "40102000",
        "sth %r1, 0(%r2)",
    ));
    insns.push((
        Inst::Store16 {
            rd: gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "40102FFF",
        "sth %r1, 4095(%r2)",
    ));
    insns.push((
        Inst::Store16 {
            rd: gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(2),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(-524288).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E31020008070",
        "sthy %r1, -524288(%r2)",
    ));
    insns.push((
        Inst::Store16 {
            rd: gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(2),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(524287).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E3102FFF7F70",
        "sthy %r1, 524287(%r2)",
    ));
    insns.push((
        Inst::Store16 {
            rd: gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "40123000",
        "sth %r1, 0(%r2,%r3)",
    ));
    insns.push((
        Inst::Store16 {
            rd: gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "40123FFF",
        "sth %r1, 4095(%r2,%r3)",
    ));
    insns.push((
        Inst::Store16 {
            rd: gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(3),
                index: gpr(2),
                disp: SImm20::maybe_from_i64(-524288).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E31230008070",
        "sthy %r1, -524288(%r2,%r3)",
    ));
    insns.push((
        Inst::Store16 {
            rd: gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(3),
                index: gpr(2),
                disp: SImm20::maybe_from_i64(524287).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E3123FFF7F70",
        "sthy %r1, 524287(%r2,%r3)",
    ));
    insns.push((
        Inst::Store32 {
            rd: gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "50102000",
        "st %r1, 0(%r2)",
    ));
    insns.push((
        Inst::Store32 {
            rd: gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "50102FFF",
        "st %r1, 4095(%r2)",
    ));
    insns.push((
        Inst::Store32 {
            rd: gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(2),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(-524288).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E31020008050",
        "sty %r1, -524288(%r2)",
    ));
    insns.push((
        Inst::Store32 {
            rd: gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(2),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(524287).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E3102FFF7F50",
        "sty %r1, 524287(%r2)",
    ));
    insns.push((
        Inst::Store32 {
            rd: gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "50123000",
        "st %r1, 0(%r2,%r3)",
    ));
    insns.push((
        Inst::Store32 {
            rd: gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "50123FFF",
        "st %r1, 4095(%r2,%r3)",
    ));
    insns.push((
        Inst::Store32 {
            rd: gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(3),
                index: gpr(2),
                disp: SImm20::maybe_from_i64(-524288).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E31230008050",
        "sty %r1, -524288(%r2,%r3)",
    ));
    insns.push((
        Inst::Store32 {
            rd: gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(3),
                index: gpr(2),
                disp: SImm20::maybe_from_i64(524287).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E3123FFF7F50",
        "sty %r1, 524287(%r2,%r3)",
    ));
    insns.push((
        Inst::Store64 {
            rd: gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E31020000024",
        "stg %r1, 0(%r2)",
    ));
    insns.push((
        Inst::Store64 {
            rd: gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E3102FFF0024",
        "stg %r1, 4095(%r2)",
    ));
    insns.push((
        Inst::Store64 {
            rd: gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(2),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(-524288).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E31020008024",
        "stg %r1, -524288(%r2)",
    ));
    insns.push((
        Inst::Store64 {
            rd: gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(2),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(524287).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E3102FFF7F24",
        "stg %r1, 524287(%r2)",
    ));
    insns.push((
        Inst::Store64 {
            rd: gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E31230000024",
        "stg %r1, 0(%r2,%r3)",
    ));
    insns.push((
        Inst::Store64 {
            rd: gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E3123FFF0024",
        "stg %r1, 4095(%r2,%r3)",
    ));
    insns.push((
        Inst::Store64 {
            rd: gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(3),
                index: gpr(2),
                disp: SImm20::maybe_from_i64(-524288).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E31230008024",
        "stg %r1, -524288(%r2,%r3)",
    ));
    insns.push((
        Inst::Store64 {
            rd: gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(3),
                index: gpr(2),
                disp: SImm20::maybe_from_i64(524287).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E3123FFF7F24",
        "stg %r1, 524287(%r2,%r3)",
    ));

    insns.push((
        Inst::StoreImm8 {
            imm: 255,
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "92FF2000",
        "mvi 0(%r2), 255",
    ));
    insns.push((
        Inst::StoreImm8 {
            imm: 0,
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "92002FFF",
        "mvi 4095(%r2), 0",
    ));
    insns.push((
        Inst::StoreImm8 {
            imm: 255,
            mem: MemArg::BXD20 {
                base: gpr(2),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(-524288).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "EBFF20008052",
        "mviy -524288(%r2), 255",
    ));
    insns.push((
        Inst::StoreImm8 {
            imm: 0,
            mem: MemArg::BXD20 {
                base: gpr(2),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(524287).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "EB002FFF7F52",
        "mviy 524287(%r2), 0",
    ));
    insns.push((
        Inst::StoreImm16 {
            imm: -32768,
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E54420008000",
        "mvhhi 0(%r2), -32768",
    ));
    insns.push((
        Inst::StoreImm16 {
            imm: 32767,
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E5442FFF7FFF",
        "mvhhi 4095(%r2), 32767",
    ));
    insns.push((
        Inst::StoreImm32SExt16 {
            imm: -32768,
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E54C20008000",
        "mvhi 0(%r2), -32768",
    ));
    insns.push((
        Inst::StoreImm32SExt16 {
            imm: 32767,
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E54C2FFF7FFF",
        "mvhi 4095(%r2), 32767",
    ));
    insns.push((
        Inst::StoreImm64SExt16 {
            imm: -32768,
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E54820008000",
        "mvghi 0(%r2), -32768",
    ));
    insns.push((
        Inst::StoreImm64SExt16 {
            imm: 32767,
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E5482FFF7FFF",
        "mvghi 4095(%r2), 32767",
    ));

    insns.push((
        Inst::StoreRev16 {
            rd: gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E3102000003F",
        "strvh %r1, 0(%r2)",
    ));
    insns.push((
        Inst::StoreRev16 {
            rd: gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E3102FFF003F",
        "strvh %r1, 4095(%r2)",
    ));
    insns.push((
        Inst::StoreRev16 {
            rd: gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(2),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(-524288).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E3102000803F",
        "strvh %r1, -524288(%r2)",
    ));
    insns.push((
        Inst::StoreRev16 {
            rd: gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(2),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(524287).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E3102FFF7F3F",
        "strvh %r1, 524287(%r2)",
    ));
    insns.push((
        Inst::StoreRev16 {
            rd: gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E3123000003F",
        "strvh %r1, 0(%r2,%r3)",
    ));
    insns.push((
        Inst::StoreRev16 {
            rd: gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E3123FFF003F",
        "strvh %r1, 4095(%r2,%r3)",
    ));
    insns.push((
        Inst::StoreRev16 {
            rd: gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(3),
                index: gpr(2),
                disp: SImm20::maybe_from_i64(-524288).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E3123000803F",
        "strvh %r1, -524288(%r2,%r3)",
    ));
    insns.push((
        Inst::StoreRev16 {
            rd: gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(3),
                index: gpr(2),
                disp: SImm20::maybe_from_i64(524287).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E3123FFF7F3F",
        "strvh %r1, 524287(%r2,%r3)",
    ));
    insns.push((
        Inst::StoreRev32 {
            rd: gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E3102000003E",
        "strv %r1, 0(%r2)",
    ));
    insns.push((
        Inst::StoreRev32 {
            rd: gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E3102FFF003E",
        "strv %r1, 4095(%r2)",
    ));
    insns.push((
        Inst::StoreRev32 {
            rd: gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(2),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(-524288).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E3102000803E",
        "strv %r1, -524288(%r2)",
    ));
    insns.push((
        Inst::StoreRev32 {
            rd: gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(2),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(524287).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E3102FFF7F3E",
        "strv %r1, 524287(%r2)",
    ));
    insns.push((
        Inst::StoreRev32 {
            rd: gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E3123000003E",
        "strv %r1, 0(%r2,%r3)",
    ));
    insns.push((
        Inst::StoreRev32 {
            rd: gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E3123FFF003E",
        "strv %r1, 4095(%r2,%r3)",
    ));
    insns.push((
        Inst::StoreRev32 {
            rd: gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(3),
                index: gpr(2),
                disp: SImm20::maybe_from_i64(-524288).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E3123000803E",
        "strv %r1, -524288(%r2,%r3)",
    ));
    insns.push((
        Inst::StoreRev32 {
            rd: gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(3),
                index: gpr(2),
                disp: SImm20::maybe_from_i64(524287).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E3123FFF7F3E",
        "strv %r1, 524287(%r2,%r3)",
    ));
    insns.push((
        Inst::StoreRev64 {
            rd: gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E3102000002F",
        "strvg %r1, 0(%r2)",
    ));
    insns.push((
        Inst::StoreRev64 {
            rd: gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E3102FFF002F",
        "strvg %r1, 4095(%r2)",
    ));
    insns.push((
        Inst::StoreRev64 {
            rd: gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(2),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(-524288).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E3102000802F",
        "strvg %r1, -524288(%r2)",
    ));
    insns.push((
        Inst::StoreRev64 {
            rd: gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(2),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(524287).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E3102FFF7F2F",
        "strvg %r1, 524287(%r2)",
    ));
    insns.push((
        Inst::StoreRev64 {
            rd: gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E3123000002F",
        "strvg %r1, 0(%r2,%r3)",
    ));
    insns.push((
        Inst::StoreRev64 {
            rd: gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E3123FFF002F",
        "strvg %r1, 4095(%r2,%r3)",
    ));
    insns.push((
        Inst::StoreRev64 {
            rd: gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(3),
                index: gpr(2),
                disp: SImm20::maybe_from_i64(-524288).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E3123000802F",
        "strvg %r1, -524288(%r2,%r3)",
    ));
    insns.push((
        Inst::StoreRev64 {
            rd: gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(3),
                index: gpr(2),
                disp: SImm20::maybe_from_i64(524287).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E3123FFF7F2F",
        "strvg %r1, 524287(%r2,%r3)",
    ));

    insns.push((
        Inst::Store16 {
            rd: gpr(1),
            mem: MemArg::Label {
                target: MachLabel::from_block(BlockIndex::new(1)),
            },
        },
        "C41700000003",
        "sthrl %r1, label1",
    ));
    insns.push((
        Inst::Store32 {
            rd: gpr(1),
            mem: MemArg::Label {
                target: MachLabel::from_block(BlockIndex::new(1)),
            },
        },
        "C41F00000003",
        "strl %r1, label1",
    ));
    insns.push((
        Inst::Store64 {
            rd: gpr(1),
            mem: MemArg::Label {
                target: MachLabel::from_block(BlockIndex::new(1)),
            },
        },
        "C41B00000003",
        "stgrl %r1, label1",
    ));

    insns.push((
        Inst::LoadMultiple64 {
            rt: writable_gpr(8),
            rt2: writable_gpr(12),
            mem: MemArg::BXD20 {
                base: gpr(15),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(-524288).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "EB8CF0008004",
        "lmg %r8, %r12, -524288(%r15)",
    ));
    insns.push((
        Inst::LoadMultiple64 {
            rt: writable_gpr(8),
            rt2: writable_gpr(12),
            mem: MemArg::BXD20 {
                base: gpr(15),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(524287).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "EB8CFFFF7F04",
        "lmg %r8, %r12, 524287(%r15)",
    ));

    insns.push((
        Inst::StoreMultiple64 {
            rt: gpr(8),
            rt2: gpr(12),
            mem: MemArg::BXD20 {
                base: gpr(15),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(-524288).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "EB8CF0008024",
        "stmg %r8, %r12, -524288(%r15)",
    ));
    insns.push((
        Inst::StoreMultiple64 {
            rt: gpr(8),
            rt2: gpr(12),
            mem: MemArg::BXD20 {
                base: gpr(15),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(524287).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "EB8CFFFF7F24",
        "stmg %r8, %r12, 524287(%r15)",
    ));

    insns.push((
        Inst::LoadAddr {
            rd: writable_gpr(1),
            mem: MemArg::BXD12 {
                base: zero_reg(),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "41100000",
        "la %r1, 0",
    ));
    insns.push((
        Inst::LoadAddr {
            rd: writable_gpr(1),
            mem: MemArg::BXD12 {
                base: zero_reg(),
                index: zero_reg(),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "41100FFF",
        "la %r1, 4095",
    ));
    insns.push((
        Inst::LoadAddr {
            rd: writable_gpr(1),
            mem: MemArg::BXD20 {
                base: zero_reg(),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(-524288).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E31000008071",
        "lay %r1, -524288",
    ));
    insns.push((
        Inst::LoadAddr {
            rd: writable_gpr(1),
            mem: MemArg::BXD20 {
                base: zero_reg(),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(524287).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E3100FFF7F71",
        "lay %r1, 524287",
    ));
    insns.push((
        Inst::LoadAddr {
            rd: writable_gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "41102000",
        "la %r1, 0(%r2)",
    ));
    insns.push((
        Inst::LoadAddr {
            rd: writable_gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "41102FFF",
        "la %r1, 4095(%r2)",
    ));
    insns.push((
        Inst::LoadAddr {
            rd: writable_gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(2),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(-524288).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E31020008071",
        "lay %r1, -524288(%r2)",
    ));
    insns.push((
        Inst::LoadAddr {
            rd: writable_gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(2),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(524287).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E3102FFF7F71",
        "lay %r1, 524287(%r2)",
    ));
    insns.push((
        Inst::LoadAddr {
            rd: writable_gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "41123000",
        "la %r1, 0(%r2,%r3)",
    ));
    insns.push((
        Inst::LoadAddr {
            rd: writable_gpr(1),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "41123FFF",
        "la %r1, 4095(%r2,%r3)",
    ));
    insns.push((
        Inst::LoadAddr {
            rd: writable_gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(3),
                index: gpr(2),
                disp: SImm20::maybe_from_i64(-524288).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E31230008071",
        "lay %r1, -524288(%r2,%r3)",
    ));
    insns.push((
        Inst::LoadAddr {
            rd: writable_gpr(1),
            mem: MemArg::BXD20 {
                base: gpr(3),
                index: gpr(2),
                disp: SImm20::maybe_from_i64(524287).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E3123FFF7F71",
        "lay %r1, 524287(%r2,%r3)",
    ));
    insns.push((
        Inst::LoadAddr {
            rd: writable_gpr(1),
            mem: MemArg::Label {
                target: MachLabel::from_block(BlockIndex::new(1)),
            },
        },
        "C01000000003",
        "larl %r1, label1",
    ));
    insns.push((
        Inst::LoadAddr {
            rd: writable_gpr(1),
            mem: MemArg::Symbol {
                name: Box::new(ExternalName::testcase("test0")),
                offset: 64,
                flags: MemFlags::trusted(),
            },
        },
        "C01000000000",
        "larl %r1, %test0 + 64",
    ));

    insns.push((
        Inst::LoadAddr {
            rd: writable_gpr(1),
            mem: MemArg::RegOffset {
                reg: gpr(2),
                off: 0,
                flags: MemFlags::trusted(),
            },
        },
        "41102000",
        "la %r1, 0(%r2)",
    ));
    insns.push((
        Inst::LoadAddr {
            rd: writable_gpr(1),
            mem: MemArg::RegOffset {
                reg: gpr(2),
                off: 4095,
                flags: MemFlags::trusted(),
            },
        },
        "41102FFF",
        "la %r1, 4095(%r2)",
    ));
    insns.push((
        Inst::LoadAddr {
            rd: writable_gpr(1),
            mem: MemArg::RegOffset {
                reg: gpr(2),
                off: -524288,
                flags: MemFlags::trusted(),
            },
        },
        "E31020008071",
        "lay %r1, -524288(%r2)",
    ));
    insns.push((
        Inst::LoadAddr {
            rd: writable_gpr(1),
            mem: MemArg::RegOffset {
                reg: gpr(2),
                off: 524287,
                flags: MemFlags::trusted(),
            },
        },
        "E3102FFF7F71",
        "lay %r1, 524287(%r2)",
    ));
    insns.push((
        Inst::LoadAddr {
            rd: writable_gpr(1),
            mem: MemArg::RegOffset {
                reg: gpr(2),
                off: -2147483648,
                flags: MemFlags::trusted(),
            },
        },
        "C0118000000041112000",
        "lgfi %r1, -2147483648 ; la %r1, 0(%r1,%r2)",
    ));
    insns.push((
        Inst::LoadAddr {
            rd: writable_gpr(1),
            mem: MemArg::RegOffset {
                reg: gpr(2),
                off: 2147483647,
                flags: MemFlags::trusted(),
            },
        },
        "C0117FFFFFFF41112000",
        "lgfi %r1, 2147483647 ; la %r1, 0(%r1,%r2)",
    ));

    insns.push((
        Inst::Mov64 {
            rd: writable_gpr(8),
            rm: gpr(9),
        },
        "B9040089",
        "lgr %r8, %r9",
    ));
    insns.push((
        Inst::Mov32 {
            rd: writable_gpr(8),
            rm: gpr(9),
        },
        "1889",
        "lr %r8, %r9",
    ));

    insns.push((
        Inst::Mov32SImm16 {
            rd: writable_gpr(8),
            imm: -32768,
        },
        "A7888000",
        "lhi %r8, -32768",
    ));
    insns.push((
        Inst::Mov32SImm16 {
            rd: writable_gpr(8),
            imm: 32767,
        },
        "A7887FFF",
        "lhi %r8, 32767",
    ));
    insns.push((
        Inst::Mov32Imm {
            rd: writable_gpr(8),
            imm: 2147483648,
        },
        "C08980000000",
        "iilf %r8, 2147483648",
    ));
    insns.push((
        Inst::Mov32Imm {
            rd: writable_gpr(8),
            imm: 2147483647,
        },
        "C0897FFFFFFF",
        "iilf %r8, 2147483647",
    ));
    insns.push((
        Inst::Mov64SImm16 {
            rd: writable_gpr(8),
            imm: -32768,
        },
        "A7898000",
        "lghi %r8, -32768",
    ));
    insns.push((
        Inst::Mov64SImm16 {
            rd: writable_gpr(8),
            imm: 32767,
        },
        "A7897FFF",
        "lghi %r8, 32767",
    ));
    insns.push((
        Inst::Mov64SImm32 {
            rd: writable_gpr(8),
            imm: -2147483648,
        },
        "C08180000000",
        "lgfi %r8, -2147483648",
    ));
    insns.push((
        Inst::Mov64SImm32 {
            rd: writable_gpr(8),
            imm: 2147483647,
        },
        "C0817FFFFFFF",
        "lgfi %r8, 2147483647",
    ));
    insns.push((
        Inst::Mov64UImm16Shifted {
            rd: writable_gpr(8),
            imm: UImm16Shifted::maybe_from_u64(0x0000_0000_0000_ffff).unwrap(),
        },
        "A58FFFFF",
        "llill %r8, 65535",
    ));
    insns.push((
        Inst::Mov64UImm16Shifted {
            rd: writable_gpr(8),
            imm: UImm16Shifted::maybe_from_u64(0x0000_0000_ffff_0000).unwrap(),
        },
        "A58EFFFF",
        "llilh %r8, 65535",
    ));
    insns.push((
        Inst::Mov64UImm16Shifted {
            rd: writable_gpr(8),
            imm: UImm16Shifted::maybe_from_u64(0x0000_ffff_0000_0000).unwrap(),
        },
        "A58DFFFF",
        "llihl %r8, 65535",
    ));
    insns.push((
        Inst::Mov64UImm16Shifted {
            rd: writable_gpr(8),
            imm: UImm16Shifted::maybe_from_u64(0xffff_0000_0000_0000).unwrap(),
        },
        "A58CFFFF",
        "llihh %r8, 65535",
    ));
    insns.push((
        Inst::Mov64UImm32Shifted {
            rd: writable_gpr(8),
            imm: UImm32Shifted::maybe_from_u64(0x0000_0000_ffff_ffff).unwrap(),
        },
        "C08FFFFFFFFF",
        "llilf %r8, 4294967295",
    ));
    insns.push((
        Inst::Mov64UImm32Shifted {
            rd: writable_gpr(8),
            imm: UImm32Shifted::maybe_from_u64(0xffff_ffff_0000_0000).unwrap(),
        },
        "C08EFFFFFFFF",
        "llihf %r8, 4294967295",
    ));

    insns.push((
        Inst::Insert64UImm16Shifted {
            rd: writable_gpr(8),
            ri: gpr(8),
            imm: UImm16Shifted::maybe_from_u64(0x0000_0000_0000_ffff).unwrap(),
        },
        "A583FFFF",
        "iill %r8, 65535",
    ));
    insns.push((
        Inst::Insert64UImm16Shifted {
            rd: writable_gpr(8),
            ri: gpr(8),
            imm: UImm16Shifted::maybe_from_u64(0x0000_0000_ffff_0000).unwrap(),
        },
        "A582FFFF",
        "iilh %r8, 65535",
    ));
    insns.push((
        Inst::Insert64UImm16Shifted {
            rd: writable_gpr(8),
            ri: gpr(8),
            imm: UImm16Shifted::maybe_from_u64(0x0000_ffff_0000_0000).unwrap(),
        },
        "A581FFFF",
        "iihl %r8, 65535",
    ));
    insns.push((
        Inst::Insert64UImm16Shifted {
            rd: writable_gpr(8),
            ri: gpr(8),
            imm: UImm16Shifted::maybe_from_u64(0xffff_0000_0000_0000).unwrap(),
        },
        "A580FFFF",
        "iihh %r8, 65535",
    ));
    insns.push((
        Inst::Insert64UImm32Shifted {
            rd: writable_gpr(8),
            ri: gpr(8),
            imm: UImm32Shifted::maybe_from_u64(0x0000_0000_ffff_ffff).unwrap(),
        },
        "C089FFFFFFFF",
        "iilf %r8, 4294967295",
    ));
    insns.push((
        Inst::Insert64UImm32Shifted {
            rd: writable_gpr(8),
            ri: gpr(8),
            imm: UImm32Shifted::maybe_from_u64(0xffff_ffff_0000_0000).unwrap(),
        },
        "C088FFFFFFFF",
        "iihf %r8, 4294967295",
    ));

    insns.push((
        Inst::CMov32 {
            rd: writable_gpr(8),
            cond: Cond::from_mask(1),
            ri: gpr(8),
            rm: gpr(9),
        },
        "B9F21089",
        "locro %r8, %r9",
    ));
    insns.push((
        Inst::CMov64 {
            rd: writable_gpr(8),
            cond: Cond::from_mask(1),
            ri: gpr(8),
            rm: gpr(9),
        },
        "B9E21089",
        "locgro %r8, %r9",
    ));

    insns.push((
        Inst::CMov32SImm16 {
            rd: writable_gpr(8),
            cond: Cond::from_mask(1),
            imm: -32768,
            ri: gpr(8),
        },
        "EC8180000042",
        "lochio %r8, -32768",
    ));
    insns.push((
        Inst::CMov32SImm16 {
            rd: writable_gpr(8),
            cond: Cond::from_mask(1),
            imm: 32767,
            ri: gpr(8),
        },
        "EC817FFF0042",
        "lochio %r8, 32767",
    ));
    insns.push((
        Inst::CMov64SImm16 {
            rd: writable_gpr(8),
            cond: Cond::from_mask(1),
            imm: -32768,
            ri: gpr(8),
        },
        "EC8180000046",
        "locghio %r8, -32768",
    ));
    insns.push((
        Inst::CMov64SImm16 {
            rd: writable_gpr(8),
            cond: Cond::from_mask(1),
            imm: 32767,
            ri: gpr(8),
        },
        "EC817FFF0046",
        "locghio %r8, 32767",
    ));

    insns.push((
        Inst::Extend {
            rd: writable_gpr(1),
            rn: gpr(2),
            signed: false,
            from_bits: 8,
            to_bits: 32,
        },
        "B9940012",
        "llcr %r1, %r2",
    ));
    insns.push((
        Inst::Extend {
            rd: writable_gpr(1),
            rn: gpr(2),
            signed: true,
            from_bits: 8,
            to_bits: 32,
        },
        "B9260012",
        "lbr %r1, %r2",
    ));
    insns.push((
        Inst::Extend {
            rd: writable_gpr(1),
            rn: gpr(2),
            signed: false,
            from_bits: 16,
            to_bits: 32,
        },
        "B9950012",
        "llhr %r1, %r2",
    ));
    insns.push((
        Inst::Extend {
            rd: writable_gpr(1),
            rn: gpr(2),
            signed: true,
            from_bits: 16,
            to_bits: 32,
        },
        "B9270012",
        "lhr %r1, %r2",
    ));
    insns.push((
        Inst::Extend {
            rd: writable_gpr(1),
            rn: gpr(2),
            signed: false,
            from_bits: 8,
            to_bits: 64,
        },
        "B9840012",
        "llgcr %r1, %r2",
    ));
    insns.push((
        Inst::Extend {
            rd: writable_gpr(1),
            rn: gpr(2),
            signed: true,
            from_bits: 8,
            to_bits: 64,
        },
        "B9060012",
        "lgbr %r1, %r2",
    ));
    insns.push((
        Inst::Extend {
            rd: writable_gpr(1),
            rn: gpr(2),
            signed: false,
            from_bits: 16,
            to_bits: 64,
        },
        "B9850012",
        "llghr %r1, %r2",
    ));
    insns.push((
        Inst::Extend {
            rd: writable_gpr(1),
            rn: gpr(2),
            signed: true,
            from_bits: 16,
            to_bits: 64,
        },
        "B9070012",
        "lghr %r1, %r2",
    ));
    insns.push((
        Inst::Extend {
            rd: writable_gpr(1),
            rn: gpr(2),
            signed: false,
            from_bits: 32,
            to_bits: 64,
        },
        "B9160012",
        "llgfr %r1, %r2",
    ));
    insns.push((
        Inst::Extend {
            rd: writable_gpr(1),
            rn: gpr(2),
            signed: true,
            from_bits: 32,
            to_bits: 64,
        },
        "B9140012",
        "lgfr %r1, %r2",
    ));

    insns.push((
        Inst::Jump {
            dest: MachLabel::from_block(BlockIndex::new(0)),
        },
        "C0F400000000",
        "jg label0",
    ));

    insns.push((
        Inst::CondBr {
            taken: MachLabel::from_block(BlockIndex::new(0)),
            not_taken: MachLabel::from_block(BlockIndex::new(0)),
            cond: Cond::from_mask(1),
        },
        "C01400000000C0F4FFFFFFFD",
        "jgo label0 ; jg label0",
    ));
    insns.push((
        Inst::CondBr {
            taken: MachLabel::from_block(BlockIndex::new(0)),
            not_taken: MachLabel::from_block(BlockIndex::new(0)),
            cond: Cond::from_mask(2),
        },
        "C02400000000C0F4FFFFFFFD",
        "jgh label0 ; jg label0",
    ));
    insns.push((
        Inst::CondBr {
            taken: MachLabel::from_block(BlockIndex::new(0)),
            not_taken: MachLabel::from_block(BlockIndex::new(0)),
            cond: Cond::from_mask(3),
        },
        "C03400000000C0F4FFFFFFFD",
        "jgnle label0 ; jg label0",
    ));
    insns.push((
        Inst::CondBr {
            taken: MachLabel::from_block(BlockIndex::new(0)),
            not_taken: MachLabel::from_block(BlockIndex::new(0)),
            cond: Cond::from_mask(4),
        },
        "C04400000000C0F4FFFFFFFD",
        "jgl label0 ; jg label0",
    ));
    insns.push((
        Inst::CondBr {
            taken: MachLabel::from_block(BlockIndex::new(0)),
            not_taken: MachLabel::from_block(BlockIndex::new(0)),
            cond: Cond::from_mask(5),
        },
        "C05400000000C0F4FFFFFFFD",
        "jgnhe label0 ; jg label0",
    ));
    insns.push((
        Inst::CondBr {
            taken: MachLabel::from_block(BlockIndex::new(0)),
            not_taken: MachLabel::from_block(BlockIndex::new(0)),
            cond: Cond::from_mask(6),
        },
        "C06400000000C0F4FFFFFFFD",
        "jglh label0 ; jg label0",
    ));
    insns.push((
        Inst::CondBr {
            taken: MachLabel::from_block(BlockIndex::new(0)),
            not_taken: MachLabel::from_block(BlockIndex::new(0)),
            cond: Cond::from_mask(7),
        },
        "C07400000000C0F4FFFFFFFD",
        "jgne label0 ; jg label0",
    ));
    insns.push((
        Inst::CondBr {
            taken: MachLabel::from_block(BlockIndex::new(0)),
            not_taken: MachLabel::from_block(BlockIndex::new(0)),
            cond: Cond::from_mask(8),
        },
        "C08400000000C0F4FFFFFFFD",
        "jge label0 ; jg label0",
    ));
    insns.push((
        Inst::CondBr {
            taken: MachLabel::from_block(BlockIndex::new(0)),
            not_taken: MachLabel::from_block(BlockIndex::new(0)),
            cond: Cond::from_mask(9),
        },
        "C09400000000C0F4FFFFFFFD",
        "jgnlh label0 ; jg label0",
    ));
    insns.push((
        Inst::CondBr {
            taken: MachLabel::from_block(BlockIndex::new(0)),
            not_taken: MachLabel::from_block(BlockIndex::new(0)),
            cond: Cond::from_mask(10),
        },
        "C0A400000000C0F4FFFFFFFD",
        "jghe label0 ; jg label0",
    ));
    insns.push((
        Inst::CondBr {
            taken: MachLabel::from_block(BlockIndex::new(0)),
            not_taken: MachLabel::from_block(BlockIndex::new(0)),
            cond: Cond::from_mask(11),
        },
        "C0B400000000C0F4FFFFFFFD",
        "jgnl label0 ; jg label0",
    ));
    insns.push((
        Inst::CondBr {
            taken: MachLabel::from_block(BlockIndex::new(0)),
            not_taken: MachLabel::from_block(BlockIndex::new(0)),
            cond: Cond::from_mask(12),
        },
        "C0C400000000C0F4FFFFFFFD",
        "jgle label0 ; jg label0",
    ));
    insns.push((
        Inst::CondBr {
            taken: MachLabel::from_block(BlockIndex::new(0)),
            not_taken: MachLabel::from_block(BlockIndex::new(0)),
            cond: Cond::from_mask(13),
        },
        "C0D400000000C0F4FFFFFFFD",
        "jgnh label0 ; jg label0",
    ));
    insns.push((
        Inst::CondBr {
            taken: MachLabel::from_block(BlockIndex::new(0)),
            not_taken: MachLabel::from_block(BlockIndex::new(0)),
            cond: Cond::from_mask(14),
        },
        "C0E400000000C0F4FFFFFFFD",
        "jgno label0 ; jg label0",
    ));

    insns.push((
        Inst::IndirectBr {
            rn: gpr(3),
            targets: vec![],
        },
        "07F3",
        "br %r3",
    ));

    insns.push((
        Inst::Call {
            link: writable_gpr(14),
            info: Box::new(CallInfo::empty(
                CallInstDest::Direct {
                    name: ExternalName::testcase("test0"),
                },
                CallConv::SystemV,
            )),
        },
        "C0E500000000",
        "brasl %r14, %test0",
    ));

    insns.push((
        Inst::Call {
            link: writable_gpr(14),
            info: Box::new(CallInfo::empty(
                CallInstDest::Indirect { reg: gpr(1) },
                CallConv::SystemV,
            )),
        },
        "0DE1",
        "basr %r14, %r1",
    ));

    insns.push((Inst::Ret { link: gpr(14) }, "07FE", "br %r14"));

    insns.push((Inst::Debugtrap, "0001", ".word 0x0001 # debugtrap"));

    insns.push((
        Inst::Trap {
            trap_code: TrapCode::STACK_OVERFLOW,
        },
        "0000",
        ".word 0x0000 # trap=stk_ovf",
    ));
    insns.push((
        Inst::TrapIf {
            cond: Cond::from_mask(1),
            trap_code: TrapCode::STACK_OVERFLOW,
        },
        "C01400000001",
        "jgo .+2 # trap=stk_ovf",
    ));

    insns.push((
        Inst::Loop {
            body: vec![
                Inst::CmpRR {
                    op: CmpOp::CmpS32,
                    rn: gpr(2),
                    rm: gpr(3),
                },
                Inst::CondBreak {
                    cond: Cond::from_mask(13),
                },
                Inst::AtomicCas32 {
                    rd: writable_gpr(4),
                    ri: gpr(4),
                    rn: gpr(5),
                    mem: MemArg::BXD12 {
                        base: gpr(6),
                        index: zero_reg(),
                        disp: UImm12::maybe_from_u64(0).unwrap(),
                        flags: MemFlags::trusted(),
                    },
                },
            ],
            cond: Cond::from_mask(6),
        },
        "1923C0D400000008BA456000C064FFFFFFFA",
        "0: cr %r2, %r3 ; jgnh 1f ; cs %r4, %r5, 0(%r6) ; jglh 0b ; 1:",
    ));

    insns.push((
        Inst::StackProbeLoop {
            probe_count: writable_gpr(1),
            guard_size: 4096,
        },
        "A7FBF0009200F000A716FFFC",
        "0: aghi %r15, -4096 ; mvi 0(%r15), 0 ; brct %r1, 0b",
    ));

    insns.push((
        Inst::FpuMove32 {
            rd: writable_vr(8),
            rn: vr(4),
        },
        "3884",
        "ler %f8, %f4",
    ));
    insns.push((
        Inst::FpuMove32 {
            rd: writable_vr(8),
            rn: vr(20),
        },
        "E78400000456",
        "vlr %v8, %v20",
    ));
    insns.push((
        Inst::FpuMove64 {
            rd: writable_vr(8),
            rn: vr(4),
        },
        "2884",
        "ldr %f8, %f4",
    ));
    insns.push((
        Inst::FpuMove64 {
            rd: writable_vr(8),
            rn: vr(20),
        },
        "E78400000456",
        "vlr %v8, %v20",
    ));
    insns.push((
        Inst::FpuCMov32 {
            rd: writable_vr(8),
            ri: vr(8),
            rm: vr(4),
            cond: Cond::from_mask(1),
        },
        "A7E400033884",
        "jno 6 ; ler %f8, %f4",
    ));
    insns.push((
        Inst::FpuCMov32 {
            rd: writable_vr(8),
            ri: vr(8),
            rm: vr(20),
            cond: Cond::from_mask(1),
        },
        "A7E40005E78400000456",
        "jno 10 ; vlr %v8, %v20",
    ));
    insns.push((
        Inst::FpuCMov64 {
            rd: writable_vr(8),
            ri: vr(8),
            rm: vr(4),
            cond: Cond::from_mask(1),
        },
        "A7E400032884",
        "jno 6 ; ldr %f8, %f4",
    ));
    insns.push((
        Inst::FpuCMov64 {
            rd: writable_vr(8),
            ri: vr(8),
            rm: vr(20),
            cond: Cond::from_mask(1),
        },
        "A7E40005E78400000456",
        "jno 10 ; vlr %v8, %v20",
    ));

    insns.push((
        Inst::FpuRR {
            fpu_op: FPUOp1::Abs32,
            rd: writable_vr(8),
            rn: vr(12),
        },
        "B300008C",
        "lpebr %f8, %f12",
    ));
    insns.push((
        Inst::FpuRR {
            fpu_op: FPUOp1::Abs32,
            rd: writable_vr(24),
            rn: vr(12),
        },
        "E78C002828CC",
        "wflpsb %v24, %f12",
    ));
    insns.push((
        Inst::FpuRR {
            fpu_op: FPUOp1::Abs32x4,
            rd: writable_vr(24),
            rn: vr(12),
        },
        "E78C002028CC",
        "vflpsb %v24, %v12",
    ));
    insns.push((
        Inst::FpuRR {
            fpu_op: FPUOp1::Abs64,
            rd: writable_vr(8),
            rn: vr(12),
        },
        "B310008C",
        "lpdbr %f8, %f12",
    ));
    insns.push((
        Inst::FpuRR {
            fpu_op: FPUOp1::Abs64,
            rd: writable_vr(24),
            rn: vr(12),
        },
        "E78C002838CC",
        "wflpdb %v24, %f12",
    ));
    insns.push((
        Inst::FpuRR {
            fpu_op: FPUOp1::Abs64x2,
            rd: writable_vr(24),
            rn: vr(12),
        },
        "E78C002038CC",
        "vflpdb %v24, %v12",
    ));
    insns.push((
        Inst::FpuRR {
            fpu_op: FPUOp1::Abs128,
            rd: writable_vr(24),
            rn: vr(12),
        },
        "E78C002848CC",
        "wflpxb %v24, %f12",
    ));
    insns.push((
        Inst::FpuRR {
            fpu_op: FPUOp1::Neg32,
            rd: writable_vr(8),
            rn: vr(12),
        },
        "B303008C",
        "lcebr %f8, %f12",
    ));
    insns.push((
        Inst::FpuRR {
            fpu_op: FPUOp1::Neg32,
            rd: writable_vr(24),
            rn: vr(12),
        },
        "E78C000828CC",
        "wflcsb %v24, %f12",
    ));
    insns.push((
        Inst::FpuRR {
            fpu_op: FPUOp1::Neg32x4,
            rd: writable_vr(24),
            rn: vr(12),
        },
        "E78C000028CC",
        "vflcsb %v24, %v12",
    ));
    insns.push((
        Inst::FpuRR {
            fpu_op: FPUOp1::Neg64,
            rd: writable_vr(8),
            rn: vr(12),
        },
        "B313008C",
        "lcdbr %f8, %f12",
    ));
    insns.push((
        Inst::FpuRR {
            fpu_op: FPUOp1::Neg64,
            rd: writable_vr(24),
            rn: vr(12),
        },
        "E78C000838CC",
        "wflcdb %v24, %f12",
    ));
    insns.push((
        Inst::FpuRR {
            fpu_op: FPUOp1::Neg64x2,
            rd: writable_vr(24),
            rn: vr(12),
        },
        "E78C000038CC",
        "vflcdb %v24, %v12",
    ));
    insns.push((
        Inst::FpuRR {
            fpu_op: FPUOp1::Neg128,
            rd: writable_vr(24),
            rn: vr(12),
        },
        "E78C000848CC",
        "wflcxb %v24, %f12",
    ));
    insns.push((
        Inst::FpuRR {
            fpu_op: FPUOp1::NegAbs32,
            rd: writable_vr(8),
            rn: vr(12),
        },
        "B301008C",
        "lnebr %f8, %f12",
    ));
    insns.push((
        Inst::FpuRR {
            fpu_op: FPUOp1::NegAbs32,
            rd: writable_vr(24),
            rn: vr(12),
        },
        "E78C001828CC",
        "wflnsb %v24, %f12",
    ));
    insns.push((
        Inst::FpuRR {
            fpu_op: FPUOp1::NegAbs32x4,
            rd: writable_vr(24),
            rn: vr(12),
        },
        "E78C001028CC",
        "vflnsb %v24, %v12",
    ));
    insns.push((
        Inst::FpuRR {
            fpu_op: FPUOp1::NegAbs64,
            rd: writable_vr(8),
            rn: vr(12),
        },
        "B311008C",
        "lndbr %f8, %f12",
    ));
    insns.push((
        Inst::FpuRR {
            fpu_op: FPUOp1::NegAbs64,
            rd: writable_vr(24),
            rn: vr(12),
        },
        "E78C001838CC",
        "wflndb %v24, %f12",
    ));
    insns.push((
        Inst::FpuRR {
            fpu_op: FPUOp1::NegAbs64x2,
            rd: writable_vr(24),
            rn: vr(12),
        },
        "E78C001038CC",
        "vflndb %v24, %v12",
    ));
    insns.push((
        Inst::FpuRR {
            fpu_op: FPUOp1::NegAbs128,
            rd: writable_vr(24),
            rn: vr(12),
        },
        "E78C001848CC",
        "wflnxb %v24, %f12",
    ));
    insns.push((
        Inst::FpuRR {
            fpu_op: FPUOp1::Sqrt32,
            rd: writable_vr(8),
            rn: vr(12),
        },
        "B314008C",
        "sqebr %f8, %f12",
    ));
    insns.push((
        Inst::FpuRR {
            fpu_op: FPUOp1::Sqrt32,
            rd: writable_vr(24),
            rn: vr(12),
        },
        "E78C000828CE",
        "wfsqsb %v24, %f12",
    ));
    insns.push((
        Inst::FpuRR {
            fpu_op: FPUOp1::Sqrt32x4,
            rd: writable_vr(24),
            rn: vr(12),
        },
        "E78C000028CE",
        "vfsqsb %v24, %v12",
    ));
    insns.push((
        Inst::FpuRR {
            fpu_op: FPUOp1::Sqrt64,
            rd: writable_vr(8),
            rn: vr(12),
        },
        "B315008C",
        "sqdbr %f8, %f12",
    ));
    insns.push((
        Inst::FpuRR {
            fpu_op: FPUOp1::Sqrt64,
            rd: writable_vr(24),
            rn: vr(12),
        },
        "E78C000838CE",
        "wfsqdb %v24, %f12",
    ));
    insns.push((
        Inst::FpuRR {
            fpu_op: FPUOp1::Sqrt64x2,
            rd: writable_vr(24),
            rn: vr(12),
        },
        "E78C000038CE",
        "vfsqdb %v24, %v12",
    ));
    insns.push((
        Inst::FpuRR {
            fpu_op: FPUOp1::Sqrt128,
            rd: writable_vr(24),
            rn: vr(12),
        },
        "E78C000848CE",
        "wfsqxb %v24, %f12",
    ));
    insns.push((
        Inst::FpuRR {
            fpu_op: FPUOp1::Cvt32To64,
            rd: writable_vr(8),
            rn: vr(12),
        },
        "B304008C",
        "ldebr %f8, %f12",
    ));
    insns.push((
        Inst::FpuRR {
            fpu_op: FPUOp1::Cvt32To64,
            rd: writable_vr(24),
            rn: vr(12),
        },
        "E78C000828C4",
        "wldeb %v24, %f12",
    ));
    insns.push((
        Inst::FpuRR {
            fpu_op: FPUOp1::Cvt32x4To64x2,
            rd: writable_vr(24),
            rn: vr(12),
        },
        "E78C000028C4",
        "vldeb %v24, %v12",
    ));
    insns.push((
        Inst::FpuRR {
            fpu_op: FPUOp1::Cvt64To128,
            rd: writable_vr(24),
            rn: vr(12),
        },
        "E78C000838C4",
        "wflld %v24, %f12",
    ));

    insns.push((
        Inst::FpuRRR {
            fpu_op: FPUOp2::Add32,
            rd: writable_vr(8),
            rn: vr(8),
            rm: vr(12),
        },
        "B30A008C",
        "aebr %f8, %f12",
    ));
    insns.push((
        Inst::FpuRRR {
            fpu_op: FPUOp2::Add32,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C00828E3",
        "wfasb %v20, %f8, %f12",
    ));
    insns.push((
        Inst::FpuRRR {
            fpu_op: FPUOp2::Add32x4,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C00028E3",
        "vfasb %v20, %v8, %v12",
    ));
    insns.push((
        Inst::FpuRRR {
            fpu_op: FPUOp2::Add64,
            rd: writable_vr(8),
            rn: vr(8),
            rm: vr(12),
        },
        "B31A008C",
        "adbr %f8, %f12",
    ));
    insns.push((
        Inst::FpuRRR {
            fpu_op: FPUOp2::Add64,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C00838E3",
        "wfadb %v20, %f8, %f12",
    ));
    insns.push((
        Inst::FpuRRR {
            fpu_op: FPUOp2::Add64x2,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C00038E3",
        "vfadb %v20, %v8, %v12",
    ));
    insns.push((
        Inst::FpuRRR {
            fpu_op: FPUOp2::Add128,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C00848E3",
        "wfaxb %v20, %f8, %f12",
    ));
    insns.push((
        Inst::FpuRRR {
            fpu_op: FPUOp2::Sub32,
            rd: writable_vr(8),
            rn: vr(8),
            rm: vr(12),
        },
        "B30B008C",
        "sebr %f8, %f12",
    ));
    insns.push((
        Inst::FpuRRR {
            fpu_op: FPUOp2::Sub32,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C00828E2",
        "wfssb %v20, %f8, %f12",
    ));
    insns.push((
        Inst::FpuRRR {
            fpu_op: FPUOp2::Sub32x4,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C00028E2",
        "vfssb %v20, %v8, %v12",
    ));
    insns.push((
        Inst::FpuRRR {
            fpu_op: FPUOp2::Sub64,
            rd: writable_vr(8),
            rn: vr(8),
            rm: vr(12),
        },
        "B31B008C",
        "sdbr %f8, %f12",
    ));
    insns.push((
        Inst::FpuRRR {
            fpu_op: FPUOp2::Sub64,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C00838E2",
        "wfsdb %v20, %f8, %f12",
    ));
    insns.push((
        Inst::FpuRRR {
            fpu_op: FPUOp2::Sub64x2,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C00038E2",
        "vfsdb %v20, %v8, %v12",
    ));
    insns.push((
        Inst::FpuRRR {
            fpu_op: FPUOp2::Sub128,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C00848E2",
        "wfsxb %v20, %f8, %f12",
    ));
    insns.push((
        Inst::FpuRRR {
            fpu_op: FPUOp2::Mul32,
            rd: writable_vr(8),
            rn: vr(8),
            rm: vr(12),
        },
        "B317008C",
        "meebr %f8, %f12",
    ));
    insns.push((
        Inst::FpuRRR {
            fpu_op: FPUOp2::Mul32,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C00828E7",
        "wfmsb %v20, %f8, %f12",
    ));
    insns.push((
        Inst::FpuRRR {
            fpu_op: FPUOp2::Mul32x4,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C00028E7",
        "vfmsb %v20, %v8, %v12",
    ));
    insns.push((
        Inst::FpuRRR {
            fpu_op: FPUOp2::Mul64,
            rd: writable_vr(8),
            rn: vr(8),
            rm: vr(12),
        },
        "B31C008C",
        "mdbr %f8, %f12",
    ));
    insns.push((
        Inst::FpuRRR {
            fpu_op: FPUOp2::Mul64,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C00838E7",
        "wfmdb %v20, %f8, %f12",
    ));
    insns.push((
        Inst::FpuRRR {
            fpu_op: FPUOp2::Mul64x2,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C00038E7",
        "vfmdb %v20, %v8, %v12",
    ));
    insns.push((
        Inst::FpuRRR {
            fpu_op: FPUOp2::Mul128,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C00848E7",
        "wfmxb %v20, %f8, %f12",
    ));
    insns.push((
        Inst::FpuRRR {
            fpu_op: FPUOp2::Div32,
            rd: writable_vr(8),
            rn: vr(8),
            rm: vr(12),
        },
        "B30D008C",
        "debr %f8, %f12",
    ));
    insns.push((
        Inst::FpuRRR {
            fpu_op: FPUOp2::Div32,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C00828E5",
        "wfdsb %v20, %f8, %f12",
    ));
    insns.push((
        Inst::FpuRRR {
            fpu_op: FPUOp2::Div32x4,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C00028E5",
        "vfdsb %v20, %v8, %v12",
    ));
    insns.push((
        Inst::FpuRRR {
            fpu_op: FPUOp2::Div64,
            rd: writable_vr(8),
            rn: vr(8),
            rm: vr(12),
        },
        "B31D008C",
        "ddbr %f8, %f12",
    ));
    insns.push((
        Inst::FpuRRR {
            fpu_op: FPUOp2::Div64,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C00838E5",
        "wfddb %v20, %f8, %f12",
    ));
    insns.push((
        Inst::FpuRRR {
            fpu_op: FPUOp2::Div64x2,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C00038E5",
        "vfddb %v20, %v8, %v12",
    ));
    insns.push((
        Inst::FpuRRR {
            fpu_op: FPUOp2::Div128,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C00848E5",
        "wfdxb %v20, %f8, %f12",
    ));
    insns.push((
        Inst::FpuRRR {
            fpu_op: FPUOp2::Max32,
            rd: writable_vr(4),
            rn: vr(6),
            rm: vr(8),
        },
        "E746801820EF",
        "wfmaxsb %f4, %f6, %f8, 1",
    ));
    insns.push((
        Inst::FpuRRR {
            fpu_op: FPUOp2::Max32x4,
            rd: writable_vr(4),
            rn: vr(6),
            rm: vr(8),
        },
        "E746801020EF",
        "vfmaxsb %v4, %v6, %v8, 1",
    ));
    insns.push((
        Inst::FpuRRR {
            fpu_op: FPUOp2::Max64,
            rd: writable_vr(4),
            rn: vr(6),
            rm: vr(24),
        },
        "E746801832EF",
        "wfmaxdb %f4, %f6, %v24, 1",
    ));
    insns.push((
        Inst::FpuRRR {
            fpu_op: FPUOp2::Max64x2,
            rd: writable_vr(4),
            rn: vr(6),
            rm: vr(24),
        },
        "E746801032EF",
        "vfmaxdb %v4, %v6, %v24, 1",
    ));
    insns.push((
        Inst::FpuRRR {
            fpu_op: FPUOp2::Max128,
            rd: writable_vr(4),
            rn: vr(6),
            rm: vr(24),
        },
        "E746801842EF",
        "wfmaxxb %f4, %f6, %v24, 1",
    ));
    insns.push((
        Inst::FpuRRR {
            fpu_op: FPUOp2::Min32,
            rd: writable_vr(4),
            rn: vr(6),
            rm: vr(8),
        },
        "E746801820EE",
        "wfminsb %f4, %f6, %f8, 1",
    ));
    insns.push((
        Inst::FpuRRR {
            fpu_op: FPUOp2::Min32x4,
            rd: writable_vr(4),
            rn: vr(6),
            rm: vr(8),
        },
        "E746801020EE",
        "vfminsb %v4, %v6, %v8, 1",
    ));
    insns.push((
        Inst::FpuRRR {
            fpu_op: FPUOp2::Min64,
            rd: writable_vr(4),
            rn: vr(6),
            rm: vr(8),
        },
        "E746801830EE",
        "wfmindb %f4, %f6, %f8, 1",
    ));
    insns.push((
        Inst::FpuRRR {
            fpu_op: FPUOp2::Min64x2,
            rd: writable_vr(4),
            rn: vr(6),
            rm: vr(8),
        },
        "E746801030EE",
        "vfmindb %v4, %v6, %v8, 1",
    ));
    insns.push((
        Inst::FpuRRR {
            fpu_op: FPUOp2::Min128,
            rd: writable_vr(4),
            rn: vr(6),
            rm: vr(8),
        },
        "E746801840EE",
        "wfminxb %f4, %f6, %f8, 1",
    ));
    insns.push((
        Inst::FpuRRR {
            fpu_op: FPUOp2::MaxPseudo32,
            rd: writable_vr(4),
            rn: vr(6),
            rm: vr(8),
        },
        "E746803820EF",
        "wfmaxsb %f4, %f6, %f8, 3",
    ));
    insns.push((
        Inst::FpuRRR {
            fpu_op: FPUOp2::MaxPseudo32x4,
            rd: writable_vr(4),
            rn: vr(6),
            rm: vr(8),
        },
        "E746803020EF",
        "vfmaxsb %v4, %v6, %v8, 3",
    ));
    insns.push((
        Inst::FpuRRR {
            fpu_op: FPUOp2::MaxPseudo64,
            rd: writable_vr(4),
            rn: vr(6),
            rm: vr(24),
        },
        "E746803832EF",
        "wfmaxdb %f4, %f6, %v24, 3",
    ));
    insns.push((
        Inst::FpuRRR {
            fpu_op: FPUOp2::MaxPseudo64x2,
            rd: writable_vr(4),
            rn: vr(6),
            rm: vr(24),
        },
        "E746803032EF",
        "vfmaxdb %v4, %v6, %v24, 3",
    ));
    insns.push((
        Inst::FpuRRR {
            fpu_op: FPUOp2::MaxPseudo128,
            rd: writable_vr(4),
            rn: vr(6),
            rm: vr(24),
        },
        "E746803842EF",
        "wfmaxxb %f4, %f6, %v24, 3",
    ));
    insns.push((
        Inst::FpuRRR {
            fpu_op: FPUOp2::MinPseudo32,
            rd: writable_vr(4),
            rn: vr(6),
            rm: vr(8),
        },
        "E746803820EE",
        "wfminsb %f4, %f6, %f8, 3",
    ));
    insns.push((
        Inst::FpuRRR {
            fpu_op: FPUOp2::MinPseudo32x4,
            rd: writable_vr(4),
            rn: vr(6),
            rm: vr(8),
        },
        "E746803020EE",
        "vfminsb %v4, %v6, %v8, 3",
    ));
    insns.push((
        Inst::FpuRRR {
            fpu_op: FPUOp2::MinPseudo64,
            rd: writable_vr(4),
            rn: vr(6),
            rm: vr(8),
        },
        "E746803830EE",
        "wfmindb %f4, %f6, %f8, 3",
    ));
    insns.push((
        Inst::FpuRRR {
            fpu_op: FPUOp2::MinPseudo64x2,
            rd: writable_vr(4),
            rn: vr(6),
            rm: vr(8),
        },
        "E746803030EE",
        "vfmindb %v4, %v6, %v8, 3",
    ));
    insns.push((
        Inst::FpuRRR {
            fpu_op: FPUOp2::MinPseudo128,
            rd: writable_vr(4),
            rn: vr(6),
            rm: vr(8),
        },
        "E746803840EE",
        "wfminxb %f4, %f6, %f8, 3",
    ));

    insns.push((
        Inst::FpuRRRR {
            fpu_op: FPUOp3::MAdd32,
            rd: writable_vr(8),
            rn: vr(12),
            rm: vr(13),
            ra: vr(8),
        },
        "B30E80CD",
        "maebr %f8, %f12, %f13",
    ));
    insns.push((
        Inst::FpuRRRR {
            fpu_op: FPUOp3::MAdd32,
            rd: writable_vr(8),
            rn: vr(12),
            rm: vr(13),
            ra: vr(20),
        },
        "E78CD208418F",
        "wfmasb %f8, %f12, %f13, %v20",
    ));
    insns.push((
        Inst::FpuRRRR {
            fpu_op: FPUOp3::MAdd32x4,
            rd: writable_vr(8),
            rn: vr(12),
            rm: vr(13),
            ra: vr(20),
        },
        "E78CD200418F",
        "vfmasb %v8, %v12, %v13, %v20",
    ));
    insns.push((
        Inst::FpuRRRR {
            fpu_op: FPUOp3::MAdd64,
            rd: writable_vr(8),
            rn: vr(12),
            rm: vr(13),
            ra: vr(8),
        },
        "B31E80CD",
        "madbr %f8, %f12, %f13",
    ));
    insns.push((
        Inst::FpuRRRR {
            fpu_op: FPUOp3::MAdd64,
            rd: writable_vr(8),
            rn: vr(12),
            rm: vr(13),
            ra: vr(20),
        },
        "E78CD308418F",
        "wfmadb %f8, %f12, %f13, %v20",
    ));
    insns.push((
        Inst::FpuRRRR {
            fpu_op: FPUOp3::MAdd64x2,
            rd: writable_vr(8),
            rn: vr(12),
            rm: vr(13),
            ra: vr(20),
        },
        "E78CD300418F",
        "vfmadb %v8, %v12, %v13, %v20",
    ));
    insns.push((
        Inst::FpuRRRR {
            fpu_op: FPUOp3::MAdd128,
            rd: writable_vr(8),
            rn: vr(12),
            rm: vr(13),
            ra: vr(20),
        },
        "E78CD408418F",
        "wfmaxb %f8, %f12, %f13, %v20",
    ));
    insns.push((
        Inst::FpuRRRR {
            fpu_op: FPUOp3::MSub32,
            rd: writable_vr(8),
            rn: vr(12),
            rm: vr(13),
            ra: vr(8),
        },
        "B30F80CD",
        "msebr %f8, %f12, %f13",
    ));
    insns.push((
        Inst::FpuRRRR {
            fpu_op: FPUOp3::MSub32,
            rd: writable_vr(8),
            rn: vr(12),
            rm: vr(13),
            ra: vr(20),
        },
        "E78CD208418E",
        "wfmssb %f8, %f12, %f13, %v20",
    ));
    insns.push((
        Inst::FpuRRRR {
            fpu_op: FPUOp3::MSub32x4,
            rd: writable_vr(8),
            rn: vr(12),
            rm: vr(13),
            ra: vr(20),
        },
        "E78CD200418E",
        "vfmssb %v8, %v12, %v13, %v20",
    ));
    insns.push((
        Inst::FpuRRRR {
            fpu_op: FPUOp3::MSub64,
            rd: writable_vr(8),
            rn: vr(12),
            rm: vr(13),
            ra: vr(8),
        },
        "B31F80CD",
        "msdbr %f8, %f12, %f13",
    ));
    insns.push((
        Inst::FpuRRRR {
            fpu_op: FPUOp3::MSub64,
            rd: writable_vr(8),
            rn: vr(12),
            rm: vr(13),
            ra: vr(20),
        },
        "E78CD308418E",
        "wfmsdb %f8, %f12, %f13, %v20",
    ));
    insns.push((
        Inst::FpuRRRR {
            fpu_op: FPUOp3::MSub64x2,
            rd: writable_vr(8),
            rn: vr(12),
            rm: vr(13),
            ra: vr(20),
        },
        "E78CD300418E",
        "vfmsdb %v8, %v12, %v13, %v20",
    ));
    insns.push((
        Inst::FpuRRRR {
            fpu_op: FPUOp3::MSub128,
            rd: writable_vr(8),
            rn: vr(12),
            rm: vr(13),
            ra: vr(20),
        },
        "E78CD408418E",
        "wfmsxb %f8, %f12, %f13, %v20",
    ));

    insns.push((
        Inst::FpuCmp32 {
            rn: vr(8),
            rm: vr(12),
        },
        "B309008C",
        "cebr %f8, %f12",
    ));
    insns.push((
        Inst::FpuCmp32 {
            rn: vr(24),
            rm: vr(12),
        },
        "E78C000028CB",
        "wfcsb %v24, %f12",
    ));
    insns.push((
        Inst::FpuCmp64 {
            rn: vr(8),
            rm: vr(12),
        },
        "B319008C",
        "cdbr %f8, %f12",
    ));
    insns.push((
        Inst::FpuCmp64 {
            rn: vr(24),
            rm: vr(12),
        },
        "E78C000038CB",
        "wfcdb %v24, %f12",
    ));
    insns.push((
        Inst::FpuCmp128 {
            rn: vr(24),
            rm: vr(12),
        },
        "E78C000048CB",
        "wfcxb %v24, %f12",
    ));

    insns.push((
        Inst::FpuRound {
            op: FpuRoundOp::Cvt64To32,
            mode: FpuRoundMode::Current,
            rd: writable_vr(8),
            rn: vr(12),
        },
        "B344008C",
        "ledbra %f8, 0, %f12, 0",
    ));
    insns.push((
        Inst::FpuRound {
            op: FpuRoundOp::Cvt64To32,
            mode: FpuRoundMode::ToNearest,
            rd: writable_vr(24),
            rn: vr(12),
        },
        "E78C001838C5",
        "wledb %v24, %f12, 0, 1",
    ));
    insns.push((
        Inst::FpuRound {
            op: FpuRoundOp::Cvt64x2To32x4,
            mode: FpuRoundMode::ToNearest,
            rd: writable_vr(24),
            rn: vr(12),
        },
        "E78C001038C5",
        "vledb %v24, %v12, 0, 1",
    ));
    insns.push((
        Inst::FpuRound {
            op: FpuRoundOp::Cvt128To64,
            mode: FpuRoundMode::ToNearest,
            rd: writable_vr(24),
            rn: vr(12),
        },
        "E78C001848C5",
        "wflrx %v24, %f12, 0, 1",
    ));
    insns.push((
        Inst::FpuRound {
            op: FpuRoundOp::Round32,
            mode: FpuRoundMode::ToNegInfinity,
            rd: writable_vr(8),
            rn: vr(12),
        },
        "B357708C",
        "fiebr %f8, 7, %f12",
    ));
    insns.push((
        Inst::FpuRound {
            op: FpuRoundOp::Round64,
            mode: FpuRoundMode::ToNegInfinity,
            rd: writable_vr(8),
            rn: vr(12),
        },
        "B35F708C",
        "fidbr %f8, 7, %f12",
    ));
    insns.push((
        Inst::FpuRound {
            op: FpuRoundOp::Round32,
            mode: FpuRoundMode::ToPosInfinity,
            rd: writable_vr(8),
            rn: vr(12),
        },
        "B357608C",
        "fiebr %f8, 6, %f12",
    ));
    insns.push((
        Inst::FpuRound {
            op: FpuRoundOp::Round64,
            mode: FpuRoundMode::ToPosInfinity,
            rd: writable_vr(8),
            rn: vr(12),
        },
        "B35F608C",
        "fidbr %f8, 6, %f12",
    ));
    insns.push((
        Inst::FpuRound {
            op: FpuRoundOp::Round32,
            mode: FpuRoundMode::ToZero,
            rd: writable_vr(8),
            rn: vr(12),
        },
        "B357508C",
        "fiebr %f8, 5, %f12",
    ));
    insns.push((
        Inst::FpuRound {
            op: FpuRoundOp::Round64,
            mode: FpuRoundMode::ToZero,
            rd: writable_vr(8),
            rn: vr(12),
        },
        "B35F508C",
        "fidbr %f8, 5, %f12",
    ));
    insns.push((
        Inst::FpuRound {
            op: FpuRoundOp::Round32,
            mode: FpuRoundMode::ToNearestTiesToEven,
            rd: writable_vr(8),
            rn: vr(12),
        },
        "B357408C",
        "fiebr %f8, 4, %f12",
    ));
    insns.push((
        Inst::FpuRound {
            op: FpuRoundOp::Round64,
            mode: FpuRoundMode::ToNearestTiesToEven,
            rd: writable_vr(8),
            rn: vr(12),
        },
        "B35F408C",
        "fidbr %f8, 4, %f12",
    ));
    insns.push((
        Inst::FpuRound {
            op: FpuRoundOp::Round32,
            mode: FpuRoundMode::ToNearest,
            rd: writable_vr(24),
            rn: vr(12),
        },
        "E78C001828C7",
        "wfisb %v24, %f12, 0, 1",
    ));
    insns.push((
        Inst::FpuRound {
            op: FpuRoundOp::Round32x4,
            mode: FpuRoundMode::ToNearest,
            rd: writable_vr(24),
            rn: vr(12),
        },
        "E78C001028C7",
        "vfisb %v24, %v12, 0, 1",
    ));
    insns.push((
        Inst::FpuRound {
            op: FpuRoundOp::Round64,
            mode: FpuRoundMode::ToNearest,
            rd: writable_vr(24),
            rn: vr(12),
        },
        "E78C001838C7",
        "wfidb %v24, %f12, 0, 1",
    ));
    insns.push((
        Inst::FpuRound {
            op: FpuRoundOp::Round64x2,
            mode: FpuRoundMode::ToNearest,
            rd: writable_vr(24),
            rn: vr(12),
        },
        "E78C001038C7",
        "vfidb %v24, %v12, 0, 1",
    ));
    insns.push((
        Inst::FpuRound {
            op: FpuRoundOp::Round128,
            mode: FpuRoundMode::ToNearest,
            rd: writable_vr(24),
            rn: vr(12),
        },
        "E78C001848C7",
        "wfixb %v24, %f12, 0, 1",
    ));
    insns.push((
        Inst::FpuRound {
            op: FpuRoundOp::ToSInt32,
            mode: FpuRoundMode::ToNearest,
            rd: writable_vr(24),
            rn: vr(12),
        },
        "E78C001828C2",
        "wcfeb %v24, %f12, 0, 1",
    ));
    insns.push((
        Inst::FpuRound {
            op: FpuRoundOp::ToSInt32x4,
            mode: FpuRoundMode::ToNearest,
            rd: writable_vr(24),
            rn: vr(12),
        },
        "E78C001028C2",
        "vcfeb %v24, %v12, 0, 1",
    ));
    insns.push((
        Inst::FpuRound {
            op: FpuRoundOp::ToSInt64,
            mode: FpuRoundMode::ToNearest,
            rd: writable_vr(24),
            rn: vr(12),
        },
        "E78C001838C2",
        "wcgdb %v24, %f12, 0, 1",
    ));
    insns.push((
        Inst::FpuRound {
            op: FpuRoundOp::ToSInt64x2,
            mode: FpuRoundMode::ToNearest,
            rd: writable_vr(24),
            rn: vr(12),
        },
        "E78C001038C2",
        "vcgdb %v24, %v12, 0, 1",
    ));
    insns.push((
        Inst::FpuRound {
            op: FpuRoundOp::ToUInt32,
            mode: FpuRoundMode::ToNearest,
            rd: writable_vr(24),
            rn: vr(12),
        },
        "E78C001828C0",
        "wclfeb %v24, %f12, 0, 1",
    ));
    insns.push((
        Inst::FpuRound {
            op: FpuRoundOp::ToUInt32x4,
            mode: FpuRoundMode::ToNearest,
            rd: writable_vr(24),
            rn: vr(12),
        },
        "E78C001028C0",
        "vclfeb %v24, %v12, 0, 1",
    ));
    insns.push((
        Inst::FpuRound {
            op: FpuRoundOp::ToUInt64,
            mode: FpuRoundMode::ToNearest,
            rd: writable_vr(24),
            rn: vr(12),
        },
        "E78C001838C0",
        "wclgdb %v24, %f12, 0, 1",
    ));
    insns.push((
        Inst::FpuRound {
            op: FpuRoundOp::ToUInt64x2,
            mode: FpuRoundMode::ToNearest,
            rd: writable_vr(24),
            rn: vr(12),
        },
        "E78C001038C0",
        "vclgdb %v24, %v12, 0, 1",
    ));
    insns.push((
        Inst::FpuRound {
            op: FpuRoundOp::FromSInt32,
            mode: FpuRoundMode::ToNearest,
            rd: writable_vr(24),
            rn: vr(12),
        },
        "E78C001828C3",
        "wcefb %v24, %f12, 0, 1",
    ));
    insns.push((
        Inst::FpuRound {
            op: FpuRoundOp::FromSInt32x4,
            mode: FpuRoundMode::ToNearest,
            rd: writable_vr(24),
            rn: vr(12),
        },
        "E78C001028C3",
        "vcefb %v24, %v12, 0, 1",
    ));
    insns.push((
        Inst::FpuRound {
            op: FpuRoundOp::FromSInt64,
            mode: FpuRoundMode::ToNearest,
            rd: writable_vr(24),
            rn: vr(12),
        },
        "E78C001838C3",
        "wcdgb %v24, %f12, 0, 1",
    ));
    insns.push((
        Inst::FpuRound {
            op: FpuRoundOp::FromSInt64x2,
            mode: FpuRoundMode::ToNearest,
            rd: writable_vr(24),
            rn: vr(12),
        },
        "E78C001038C3",
        "vcdgb %v24, %v12, 0, 1",
    ));
    insns.push((
        Inst::FpuRound {
            op: FpuRoundOp::FromUInt32,
            mode: FpuRoundMode::ToNearest,
            rd: writable_vr(24),
            rn: vr(12),
        },
        "E78C001828C1",
        "wcelfb %v24, %f12, 0, 1",
    ));
    insns.push((
        Inst::FpuRound {
            op: FpuRoundOp::FromUInt32x4,
            mode: FpuRoundMode::ToNearest,
            rd: writable_vr(24),
            rn: vr(12),
        },
        "E78C001028C1",
        "vcelfb %v24, %v12, 0, 1",
    ));
    insns.push((
        Inst::FpuRound {
            op: FpuRoundOp::FromUInt64,
            mode: FpuRoundMode::ToNearest,
            rd: writable_vr(24),
            rn: vr(12),
        },
        "E78C001838C1",
        "wcdlgb %v24, %f12, 0, 1",
    ));
    insns.push((
        Inst::FpuRound {
            op: FpuRoundOp::FromUInt64x2,
            mode: FpuRoundMode::ToNearest,
            rd: writable_vr(24),
            rn: vr(12),
        },
        "E78C001038C1",
        "vcdlgb %v24, %v12, 0, 1",
    ));

    let w_fp_regpair = WritableRegPair {
        hi: writable_vr(1),
        lo: writable_vr(3),
    };
    let fp_regpair = RegPair {
        hi: vr(1),
        lo: vr(3),
    };
    insns.push((
        Inst::FpuConv128FromInt {
            op: FpuConv128Op::SInt32,
            mode: FpuRoundMode::ToZero,
            rd: w_fp_regpair,
            rn: gpr(8),
        },
        "B3965018",
        "cxfbra %f1, 5, %r8, 0",
    ));
    insns.push((
        Inst::FpuConv128FromInt {
            op: FpuConv128Op::SInt64,
            mode: FpuRoundMode::ToZero,
            rd: w_fp_regpair,
            rn: gpr(8),
        },
        "B3A65018",
        "cxgbra %f1, 5, %r8, 0",
    ));
    insns.push((
        Inst::FpuConv128FromInt {
            op: FpuConv128Op::UInt32,
            mode: FpuRoundMode::ToZero,
            rd: w_fp_regpair,
            rn: gpr(8),
        },
        "B3925018",
        "cxlfbr %f1, 5, %r8, 0",
    ));
    insns.push((
        Inst::FpuConv128FromInt {
            op: FpuConv128Op::UInt64,
            mode: FpuRoundMode::ToZero,
            rd: w_fp_regpair,
            rn: gpr(8),
        },
        "B3A25018",
        "cxlgbr %f1, 5, %r8, 0",
    ));
    insns.push((
        Inst::FpuConv128ToInt {
            op: FpuConv128Op::SInt32,
            mode: FpuRoundMode::ToZero,
            rd: writable_gpr(8),
            rn: fp_regpair,
        },
        "B39A5081",
        "cfxbra %r8, 5, %f1, 0",
    ));
    insns.push((
        Inst::FpuConv128ToInt {
            op: FpuConv128Op::SInt64,
            mode: FpuRoundMode::ToZero,
            rd: writable_gpr(8),
            rn: fp_regpair,
        },
        "B3AA5081",
        "cgxbra %r8, 5, %f1, 0",
    ));
    insns.push((
        Inst::FpuConv128ToInt {
            op: FpuConv128Op::UInt32,
            mode: FpuRoundMode::ToZero,
            rd: writable_gpr(8),
            rn: fp_regpair,
        },
        "B39E5081",
        "clfxbr %r8, 5, %f1, 0",
    ));
    insns.push((
        Inst::FpuConv128ToInt {
            op: FpuConv128Op::UInt64,
            mode: FpuRoundMode::ToZero,
            rd: writable_gpr(8),
            rn: fp_regpair,
        },
        "B3AE5081",
        "clgxbr %r8, 5, %f1, 0",
    ));

    insns.push((
        Inst::VecRRR {
            op: VecBinaryOp::Add8x16,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C00008F3",
        "vab %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecRRR {
            op: VecBinaryOp::Add16x8,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C00018F3",
        "vah %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecRRR {
            op: VecBinaryOp::Add32x4,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C00028F3",
        "vaf %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecRRR {
            op: VecBinaryOp::Add64x2,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C00038F3",
        "vag %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecRRR {
            op: VecBinaryOp::Add128,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C00048F3",
        "vaq %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecRRR {
            op: VecBinaryOp::Sub8x16,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C00008F7",
        "vsb %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecRRR {
            op: VecBinaryOp::Sub16x8,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C00018F7",
        "vsh %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecRRR {
            op: VecBinaryOp::Sub32x4,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C00028F7",
        "vsf %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecRRR {
            op: VecBinaryOp::Sub64x2,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C00038F7",
        "vsg %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecRRR {
            op: VecBinaryOp::Sub128,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C00048F7",
        "vsq %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecRRR {
            op: VecBinaryOp::Mul8x16,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C00008A2",
        "vmlb %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecRRR {
            op: VecBinaryOp::Mul16x8,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C00018A2",
        "vmlhw %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecRRR {
            op: VecBinaryOp::Mul32x4,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C00028A2",
        "vmlf %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecRRR {
            op: VecBinaryOp::UMulHi8x16,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C00008A1",
        "vmlhb %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecRRR {
            op: VecBinaryOp::UMulHi16x8,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C00018A1",
        "vmlhh %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecRRR {
            op: VecBinaryOp::UMulHi32x4,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C00028A1",
        "vmlhf %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecRRR {
            op: VecBinaryOp::SMulHi8x16,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C00008A3",
        "vmhb %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecRRR {
            op: VecBinaryOp::SMulHi16x8,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C00018A3",
        "vmhh %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecRRR {
            op: VecBinaryOp::SMulHi32x4,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C00028A3",
        "vmhf %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecRRR {
            op: VecBinaryOp::UMulEven8x16,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C00008A4",
        "vmleb %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecRRR {
            op: VecBinaryOp::UMulEven16x8,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C00018A4",
        "vmleh %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecRRR {
            op: VecBinaryOp::UMulEven32x4,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C00028A4",
        "vmlef %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecRRR {
            op: VecBinaryOp::SMulEven8x16,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C00008A6",
        "vmeb %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecRRR {
            op: VecBinaryOp::SMulEven16x8,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C00018A6",
        "vmeh %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecRRR {
            op: VecBinaryOp::SMulEven32x4,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C00028A6",
        "vmef %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecRRR {
            op: VecBinaryOp::UMulOdd8x16,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C00008A5",
        "vmlob %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecRRR {
            op: VecBinaryOp::UMulOdd16x8,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C00018A5",
        "vmloh %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecRRR {
            op: VecBinaryOp::UMulOdd32x4,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C00028A5",
        "vmlof %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecRRR {
            op: VecBinaryOp::SMulOdd8x16,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C00008A7",
        "vmob %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecRRR {
            op: VecBinaryOp::SMulOdd16x8,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C00018A7",
        "vmoh %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecRRR {
            op: VecBinaryOp::SMulOdd32x4,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C00028A7",
        "vmof %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecRRR {
            op: VecBinaryOp::UMax8x16,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C00008FD",
        "vmxlb %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecRRR {
            op: VecBinaryOp::UMax16x8,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C00018FD",
        "vmxlh %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecRRR {
            op: VecBinaryOp::UMax32x4,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C00028FD",
        "vmxlf %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecRRR {
            op: VecBinaryOp::UMax64x2,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C00038FD",
        "vmxlg %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecRRR {
            op: VecBinaryOp::SMax8x16,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C00008FF",
        "vmxb %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecRRR {
            op: VecBinaryOp::SMax16x8,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C00018FF",
        "vmxh %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecRRR {
            op: VecBinaryOp::SMax32x4,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C00028FF",
        "vmxf %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecRRR {
            op: VecBinaryOp::SMax64x2,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C00038FF",
        "vmxg %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecRRR {
            op: VecBinaryOp::UMin8x16,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C00008FC",
        "vmnlb %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecRRR {
            op: VecBinaryOp::UMin16x8,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C00018FC",
        "vmnlh %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecRRR {
            op: VecBinaryOp::UMin32x4,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C00028FC",
        "vmnlf %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecRRR {
            op: VecBinaryOp::UMin64x2,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C00038FC",
        "vmnlg %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecRRR {
            op: VecBinaryOp::SMin8x16,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C00008FE",
        "vmnb %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecRRR {
            op: VecBinaryOp::SMin16x8,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C00018FE",
        "vmnh %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecRRR {
            op: VecBinaryOp::SMin32x4,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C00028FE",
        "vmnf %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecRRR {
            op: VecBinaryOp::SMin64x2,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C00038FE",
        "vmng %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecRRR {
            op: VecBinaryOp::UAvg8x16,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C00008F0",
        "vavglb %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecRRR {
            op: VecBinaryOp::UAvg16x8,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C00018F0",
        "vavglh %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecRRR {
            op: VecBinaryOp::UAvg32x4,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C00028F0",
        "vavglf %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecRRR {
            op: VecBinaryOp::UAvg64x2,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C00038F0",
        "vavglg %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecRRR {
            op: VecBinaryOp::SAvg8x16,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C00008F2",
        "vavgb %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecRRR {
            op: VecBinaryOp::SAvg16x8,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C00018F2",
        "vavgh %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecRRR {
            op: VecBinaryOp::SAvg32x4,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C00028F2",
        "vavgf %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecRRR {
            op: VecBinaryOp::SAvg64x2,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C00038F2",
        "vavgg %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecRRR {
            op: VecBinaryOp::And128,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C0000868",
        "vn %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecRRR {
            op: VecBinaryOp::Orr128,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C000086A",
        "vo %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecRRR {
            op: VecBinaryOp::Xor128,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C000086D",
        "vx %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecRRR {
            op: VecBinaryOp::NotAnd128,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C000086E",
        "vnn %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecRRR {
            op: VecBinaryOp::NotOrr128,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C000086B",
        "vno %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecRRR {
            op: VecBinaryOp::NotXor128,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C000086C",
        "vnx %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecRRR {
            op: VecBinaryOp::AndNot128,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C0000869",
        "vnc %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecRRR {
            op: VecBinaryOp::OrrNot128,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C000086F",
        "voc %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecRRR {
            op: VecBinaryOp::BitPermute128,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C0000885",
        "vbperm %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecRRR {
            op: VecBinaryOp::LShLByByte128,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C0000875",
        "vslb %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecRRR {
            op: VecBinaryOp::LShRByByte128,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C000087D",
        "vsrlb %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecRRR {
            op: VecBinaryOp::AShRByByte128,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C000087F",
        "vsrab %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecRRR {
            op: VecBinaryOp::LShLByBit128,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C0000874",
        "vsl %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecRRR {
            op: VecBinaryOp::LShRByBit128,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C000087C",
        "vsrl %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecRRR {
            op: VecBinaryOp::AShRByBit128,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C000087E",
        "vsra %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecRRR {
            op: VecBinaryOp::Pack16x8,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C0001894",
        "vpkh %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecRRR {
            op: VecBinaryOp::Pack32x4,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C0002894",
        "vpkf %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecRRR {
            op: VecBinaryOp::Pack64x2,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C0003894",
        "vpkg %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecRRR {
            op: VecBinaryOp::PackUSat16x8,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C0001895",
        "vpklsh %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecRRR {
            op: VecBinaryOp::PackUSat32x4,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C0002895",
        "vpklsf %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecRRR {
            op: VecBinaryOp::PackUSat64x2,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C0003895",
        "vpklsg %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecRRR {
            op: VecBinaryOp::PackSSat16x8,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C0001897",
        "vpksh %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecRRR {
            op: VecBinaryOp::PackSSat32x4,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C0002897",
        "vpksf %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecRRR {
            op: VecBinaryOp::PackSSat64x2,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C0003897",
        "vpksg %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecRRR {
            op: VecBinaryOp::MergeLow8x16,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C0000860",
        "vmrlb %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecRRR {
            op: VecBinaryOp::MergeLow16x8,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C0001860",
        "vmrlh %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecRRR {
            op: VecBinaryOp::MergeLow32x4,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C0002860",
        "vmrlf %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecRRR {
            op: VecBinaryOp::MergeLow64x2,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C0003860",
        "vmrlg %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecRRR {
            op: VecBinaryOp::MergeHigh8x16,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C0000861",
        "vmrhb %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecRRR {
            op: VecBinaryOp::MergeHigh16x8,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C0001861",
        "vmrhh %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecRRR {
            op: VecBinaryOp::MergeHigh32x4,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C0002861",
        "vmrhf %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecRRR {
            op: VecBinaryOp::MergeHigh64x2,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C0003861",
        "vmrhg %v20, %v8, %v12",
    ));

    insns.push((
        Inst::VecRR {
            op: VecUnaryOp::Abs8x16,
            rd: writable_vr(20),
            rn: vr(8),
        },
        "E748000008DF",
        "vlpb %v20, %v8",
    ));
    insns.push((
        Inst::VecRR {
            op: VecUnaryOp::Abs16x8,
            rd: writable_vr(20),
            rn: vr(8),
        },
        "E748000018DF",
        "vlph %v20, %v8",
    ));
    insns.push((
        Inst::VecRR {
            op: VecUnaryOp::Abs32x4,
            rd: writable_vr(20),
            rn: vr(8),
        },
        "E748000028DF",
        "vlpf %v20, %v8",
    ));
    insns.push((
        Inst::VecRR {
            op: VecUnaryOp::Abs64x2,
            rd: writable_vr(20),
            rn: vr(8),
        },
        "E748000038DF",
        "vlpg %v20, %v8",
    ));
    insns.push((
        Inst::VecRR {
            op: VecUnaryOp::Neg8x16,
            rd: writable_vr(20),
            rn: vr(8),
        },
        "E748000008DE",
        "vlcb %v20, %v8",
    ));
    insns.push((
        Inst::VecRR {
            op: VecUnaryOp::Neg16x8,
            rd: writable_vr(20),
            rn: vr(8),
        },
        "E748000018DE",
        "vlch %v20, %v8",
    ));
    insns.push((
        Inst::VecRR {
            op: VecUnaryOp::Neg32x4,
            rd: writable_vr(20),
            rn: vr(8),
        },
        "E748000028DE",
        "vlcf %v20, %v8",
    ));
    insns.push((
        Inst::VecRR {
            op: VecUnaryOp::Neg64x2,
            rd: writable_vr(20),
            rn: vr(8),
        },
        "E748000038DE",
        "vlcg %v20, %v8",
    ));
    insns.push((
        Inst::VecRR {
            op: VecUnaryOp::Popcnt8x16,
            rd: writable_vr(20),
            rn: vr(8),
        },
        "E74800000850",
        "vpopctb %v20, %v8",
    ));
    insns.push((
        Inst::VecRR {
            op: VecUnaryOp::Popcnt16x8,
            rd: writable_vr(20),
            rn: vr(8),
        },
        "E74800001850",
        "vpopcth %v20, %v8",
    ));
    insns.push((
        Inst::VecRR {
            op: VecUnaryOp::Popcnt32x4,
            rd: writable_vr(20),
            rn: vr(8),
        },
        "E74800002850",
        "vpopctf %v20, %v8",
    ));
    insns.push((
        Inst::VecRR {
            op: VecUnaryOp::Popcnt64x2,
            rd: writable_vr(20),
            rn: vr(8),
        },
        "E74800003850",
        "vpopctg %v20, %v8",
    ));
    insns.push((
        Inst::VecRR {
            op: VecUnaryOp::Clz8x16,
            rd: writable_vr(20),
            rn: vr(8),
        },
        "E74800000853",
        "vclzb %v20, %v8",
    ));
    insns.push((
        Inst::VecRR {
            op: VecUnaryOp::Clz16x8,
            rd: writable_vr(20),
            rn: vr(8),
        },
        "E74800001853",
        "vclzh %v20, %v8",
    ));
    insns.push((
        Inst::VecRR {
            op: VecUnaryOp::Clz32x4,
            rd: writable_vr(20),
            rn: vr(8),
        },
        "E74800002853",
        "vclzf %v20, %v8",
    ));
    insns.push((
        Inst::VecRR {
            op: VecUnaryOp::Clz64x2,
            rd: writable_vr(20),
            rn: vr(8),
        },
        "E74800003853",
        "vclzg %v20, %v8",
    ));
    insns.push((
        Inst::VecRR {
            op: VecUnaryOp::Ctz8x16,
            rd: writable_vr(20),
            rn: vr(8),
        },
        "E74800000852",
        "vctzb %v20, %v8",
    ));
    insns.push((
        Inst::VecRR {
            op: VecUnaryOp::Ctz16x8,
            rd: writable_vr(20),
            rn: vr(8),
        },
        "E74800001852",
        "vctzh %v20, %v8",
    ));
    insns.push((
        Inst::VecRR {
            op: VecUnaryOp::Ctz32x4,
            rd: writable_vr(20),
            rn: vr(8),
        },
        "E74800002852",
        "vctzf %v20, %v8",
    ));
    insns.push((
        Inst::VecRR {
            op: VecUnaryOp::Ctz64x2,
            rd: writable_vr(20),
            rn: vr(8),
        },
        "E74800003852",
        "vctzg %v20, %v8",
    ));
    insns.push((
        Inst::VecRR {
            op: VecUnaryOp::UnpackULow8x16,
            rd: writable_vr(20),
            rn: vr(8),
        },
        "E748000008D4",
        "vupllb %v20, %v8",
    ));
    insns.push((
        Inst::VecRR {
            op: VecUnaryOp::UnpackULow16x8,
            rd: writable_vr(20),
            rn: vr(8),
        },
        "E748000018D4",
        "vupllh %v20, %v8",
    ));
    insns.push((
        Inst::VecRR {
            op: VecUnaryOp::UnpackULow32x4,
            rd: writable_vr(20),
            rn: vr(8),
        },
        "E748000028D4",
        "vupllf %v20, %v8",
    ));
    insns.push((
        Inst::VecRR {
            op: VecUnaryOp::UnpackUHigh8x16,
            rd: writable_vr(20),
            rn: vr(8),
        },
        "E748000008D5",
        "vuplhb %v20, %v8",
    ));
    insns.push((
        Inst::VecRR {
            op: VecUnaryOp::UnpackUHigh16x8,
            rd: writable_vr(20),
            rn: vr(8),
        },
        "E748000018D5",
        "vuplhh %v20, %v8",
    ));
    insns.push((
        Inst::VecRR {
            op: VecUnaryOp::UnpackUHigh32x4,
            rd: writable_vr(20),
            rn: vr(8),
        },
        "E748000028D5",
        "vuplhf %v20, %v8",
    ));
    insns.push((
        Inst::VecRR {
            op: VecUnaryOp::UnpackSLow8x16,
            rd: writable_vr(20),
            rn: vr(8),
        },
        "E748000008D6",
        "vuplb %v20, %v8",
    ));
    insns.push((
        Inst::VecRR {
            op: VecUnaryOp::UnpackSLow16x8,
            rd: writable_vr(20),
            rn: vr(8),
        },
        "E748000018D6",
        "vuplh %v20, %v8",
    ));
    insns.push((
        Inst::VecRR {
            op: VecUnaryOp::UnpackSLow32x4,
            rd: writable_vr(20),
            rn: vr(8),
        },
        "E748000028D6",
        "vuplf %v20, %v8",
    ));
    insns.push((
        Inst::VecRR {
            op: VecUnaryOp::UnpackSHigh8x16,
            rd: writable_vr(20),
            rn: vr(8),
        },
        "E748000008D7",
        "vuphb %v20, %v8",
    ));
    insns.push((
        Inst::VecRR {
            op: VecUnaryOp::UnpackSHigh16x8,
            rd: writable_vr(20),
            rn: vr(8),
        },
        "E748000018D7",
        "vuphh %v20, %v8",
    ));
    insns.push((
        Inst::VecRR {
            op: VecUnaryOp::UnpackSHigh32x4,
            rd: writable_vr(20),
            rn: vr(8),
        },
        "E748000028D7",
        "vuphf %v20, %v8",
    ));

    insns.push((
        Inst::VecShiftRR {
            shift_op: VecShiftOp::RotL8x16,
            rd: writable_vr(20),
            rn: vr(5),
            shift_imm: 3,
            shift_reg: gpr(6),
        },
        "E74560030833",
        "verllb %v20, %v5, 3(%r6)",
    ));
    insns.push((
        Inst::VecShiftRR {
            shift_op: VecShiftOp::RotL16x8,
            rd: writable_vr(20),
            rn: vr(5),
            shift_imm: 3,
            shift_reg: gpr(6),
        },
        "E74560031833",
        "verllh %v20, %v5, 3(%r6)",
    ));
    insns.push((
        Inst::VecShiftRR {
            shift_op: VecShiftOp::RotL32x4,
            rd: writable_vr(20),
            rn: vr(5),
            shift_imm: 3,
            shift_reg: gpr(6),
        },
        "E74560032833",
        "verllf %v20, %v5, 3(%r6)",
    ));
    insns.push((
        Inst::VecShiftRR {
            shift_op: VecShiftOp::RotL64x2,
            rd: writable_vr(20),
            rn: vr(5),
            shift_imm: 3,
            shift_reg: gpr(6),
        },
        "E74560033833",
        "verllg %v20, %v5, 3(%r6)",
    ));
    insns.push((
        Inst::VecShiftRR {
            shift_op: VecShiftOp::LShL8x16,
            rd: writable_vr(20),
            rn: vr(5),
            shift_imm: 3,
            shift_reg: gpr(6),
        },
        "E74560030830",
        "veslb %v20, %v5, 3(%r6)",
    ));
    insns.push((
        Inst::VecShiftRR {
            shift_op: VecShiftOp::LShL16x8,
            rd: writable_vr(20),
            rn: vr(5),
            shift_imm: 3,
            shift_reg: gpr(6),
        },
        "E74560031830",
        "veslh %v20, %v5, 3(%r6)",
    ));
    insns.push((
        Inst::VecShiftRR {
            shift_op: VecShiftOp::LShL32x4,
            rd: writable_vr(20),
            rn: vr(5),
            shift_imm: 3,
            shift_reg: gpr(6),
        },
        "E74560032830",
        "veslf %v20, %v5, 3(%r6)",
    ));
    insns.push((
        Inst::VecShiftRR {
            shift_op: VecShiftOp::LShL64x2,
            rd: writable_vr(20),
            rn: vr(5),
            shift_imm: 3,
            shift_reg: gpr(6),
        },
        "E74560033830",
        "veslg %v20, %v5, 3(%r6)",
    ));
    insns.push((
        Inst::VecShiftRR {
            shift_op: VecShiftOp::LShR8x16,
            rd: writable_vr(20),
            rn: vr(5),
            shift_imm: 3,
            shift_reg: gpr(6),
        },
        "E74560030838",
        "vesrlb %v20, %v5, 3(%r6)",
    ));
    insns.push((
        Inst::VecShiftRR {
            shift_op: VecShiftOp::LShR16x8,
            rd: writable_vr(20),
            rn: vr(5),
            shift_imm: 3,
            shift_reg: gpr(6),
        },
        "E74560031838",
        "vesrlh %v20, %v5, 3(%r6)",
    ));
    insns.push((
        Inst::VecShiftRR {
            shift_op: VecShiftOp::LShR32x4,
            rd: writable_vr(20),
            rn: vr(5),
            shift_imm: 3,
            shift_reg: gpr(6),
        },
        "E74560032838",
        "vesrlf %v20, %v5, 3(%r6)",
    ));
    insns.push((
        Inst::VecShiftRR {
            shift_op: VecShiftOp::LShR64x2,
            rd: writable_vr(20),
            rn: vr(5),
            shift_imm: 3,
            shift_reg: gpr(6),
        },
        "E74560033838",
        "vesrlg %v20, %v5, 3(%r6)",
    ));
    insns.push((
        Inst::VecShiftRR {
            shift_op: VecShiftOp::AShR8x16,
            rd: writable_vr(20),
            rn: vr(5),
            shift_imm: 3,
            shift_reg: gpr(6),
        },
        "E7456003083A",
        "vesrab %v20, %v5, 3(%r6)",
    ));
    insns.push((
        Inst::VecShiftRR {
            shift_op: VecShiftOp::AShR16x8,
            rd: writable_vr(20),
            rn: vr(5),
            shift_imm: 3,
            shift_reg: gpr(6),
        },
        "E7456003183A",
        "vesrah %v20, %v5, 3(%r6)",
    ));
    insns.push((
        Inst::VecShiftRR {
            shift_op: VecShiftOp::AShR32x4,
            rd: writable_vr(20),
            rn: vr(5),
            shift_imm: 3,
            shift_reg: gpr(6),
        },
        "E7456003283A",
        "vesraf %v20, %v5, 3(%r6)",
    ));
    insns.push((
        Inst::VecShiftRR {
            shift_op: VecShiftOp::AShR64x2,
            rd: writable_vr(20),
            rn: vr(5),
            shift_imm: 3,
            shift_reg: gpr(6),
        },
        "E7456003383A",
        "vesrag %v20, %v5, 3(%r6)",
    ));

    insns.push((
        Inst::VecSelect {
            rd: writable_vr(4),
            rn: vr(6),
            rm: vr(8),
            ra: vr(10),
        },
        "E7468000A08D",
        "vsel %v4, %v6, %v8, %v10",
    ));
    insns.push((
        Inst::VecSelect {
            rd: writable_vr(20),
            rn: vr(6),
            rm: vr(8),
            ra: vr(10),
        },
        "E7468000A88D",
        "vsel %v20, %v6, %v8, %v10",
    ));
    insns.push((
        Inst::VecSelect {
            rd: writable_vr(4),
            rn: vr(22),
            rm: vr(8),
            ra: vr(10),
        },
        "E7468000A48D",
        "vsel %v4, %v22, %v8, %v10",
    ));
    insns.push((
        Inst::VecSelect {
            rd: writable_vr(4),
            rn: vr(6),
            rm: vr(24),
            ra: vr(10),
        },
        "E7468000A28D",
        "vsel %v4, %v6, %v24, %v10",
    ));
    insns.push((
        Inst::VecSelect {
            rd: writable_vr(4),
            rn: vr(6),
            rm: vr(8),
            ra: vr(26),
        },
        "E7468000A18D",
        "vsel %v4, %v6, %v8, %v26",
    ));
    insns.push((
        Inst::VecSelect {
            rd: writable_vr(20),
            rn: vr(22),
            rm: vr(24),
            ra: vr(26),
        },
        "E7468000AF8D",
        "vsel %v20, %v22, %v24, %v26",
    ));
    insns.push((
        Inst::VecPermute {
            rd: writable_vr(4),
            rn: vr(6),
            rm: vr(8),
            ra: vr(10),
        },
        "E7468000A08C",
        "vperm %v4, %v6, %v8, %v10",
    ));
    insns.push((
        Inst::VecPermute {
            rd: writable_vr(20),
            rn: vr(6),
            rm: vr(8),
            ra: vr(10),
        },
        "E7468000A88C",
        "vperm %v20, %v6, %v8, %v10",
    ));
    insns.push((
        Inst::VecPermute {
            rd: writable_vr(4),
            rn: vr(22),
            rm: vr(8),
            ra: vr(10),
        },
        "E7468000A48C",
        "vperm %v4, %v22, %v8, %v10",
    ));
    insns.push((
        Inst::VecPermute {
            rd: writable_vr(4),
            rn: vr(6),
            rm: vr(24),
            ra: vr(10),
        },
        "E7468000A28C",
        "vperm %v4, %v6, %v24, %v10",
    ));
    insns.push((
        Inst::VecPermute {
            rd: writable_vr(4),
            rn: vr(6),
            rm: vr(8),
            ra: vr(26),
        },
        "E7468000A18C",
        "vperm %v4, %v6, %v8, %v26",
    ));
    insns.push((
        Inst::VecPermute {
            rd: writable_vr(20),
            rn: vr(22),
            rm: vr(24),
            ra: vr(26),
        },
        "E7468000AF8C",
        "vperm %v20, %v22, %v24, %v26",
    ));
    insns.push((
        Inst::VecPermuteDWImm {
            rd: writable_vr(20),
            rn: vr(6),
            rm: vr(8),
            idx1: 0,
            idx2: 0,
        },
        "E74680000884",
        "vpdi %v20, %v6, %v8, 0",
    ));
    insns.push((
        Inst::VecPermuteDWImm {
            rd: writable_vr(20),
            rn: vr(6),
            rm: vr(8),
            idx1: 0,
            idx2: 1,
        },
        "E74680001884",
        "vpdi %v20, %v6, %v8, 1",
    ));
    insns.push((
        Inst::VecPermuteDWImm {
            rd: writable_vr(20),
            rn: vr(6),
            rm: vr(8),
            idx1: 1,
            idx2: 0,
        },
        "E74680004884",
        "vpdi %v20, %v6, %v8, 4",
    ));
    insns.push((
        Inst::VecPermuteDWImm {
            rd: writable_vr(20),
            rn: vr(6),
            rm: vr(8),
            idx1: 1,
            idx2: 1,
        },
        "E74680005884",
        "vpdi %v20, %v6, %v8, 5",
    ));

    insns.push((
        Inst::VecIntCmp {
            op: VecIntCmpOp::CmpEq8x16,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C00008F8",
        "vceqb %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecIntCmp {
            op: VecIntCmpOp::CmpEq16x8,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C00018F8",
        "vceqh %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecIntCmp {
            op: VecIntCmpOp::CmpEq32x4,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C00028F8",
        "vceqf %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecIntCmp {
            op: VecIntCmpOp::CmpEq64x2,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C00038F8",
        "vceqg %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecIntCmp {
            op: VecIntCmpOp::SCmpHi8x16,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C00008FB",
        "vchb %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecIntCmp {
            op: VecIntCmpOp::SCmpHi16x8,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C00018FB",
        "vchh %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecIntCmp {
            op: VecIntCmpOp::SCmpHi32x4,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C00028FB",
        "vchf %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecIntCmp {
            op: VecIntCmpOp::SCmpHi64x2,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C00038FB",
        "vchg %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecIntCmp {
            op: VecIntCmpOp::UCmpHi8x16,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C00008F9",
        "vchlb %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecIntCmp {
            op: VecIntCmpOp::UCmpHi16x8,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C00018F9",
        "vchlh %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecIntCmp {
            op: VecIntCmpOp::UCmpHi32x4,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C00028F9",
        "vchlf %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecIntCmp {
            op: VecIntCmpOp::UCmpHi64x2,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C00038F9",
        "vchlg %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecIntCmpS {
            op: VecIntCmpOp::CmpEq8x16,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C01008F8",
        "vceqbs %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecIntCmpS {
            op: VecIntCmpOp::CmpEq16x8,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C01018F8",
        "vceqhs %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecIntCmpS {
            op: VecIntCmpOp::CmpEq32x4,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C01028F8",
        "vceqfs %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecIntCmpS {
            op: VecIntCmpOp::CmpEq64x2,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C01038F8",
        "vceqgs %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecIntCmpS {
            op: VecIntCmpOp::SCmpHi8x16,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C01008FB",
        "vchbs %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecIntCmpS {
            op: VecIntCmpOp::SCmpHi16x8,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C01018FB",
        "vchhs %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecIntCmpS {
            op: VecIntCmpOp::SCmpHi32x4,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C01028FB",
        "vchfs %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecIntCmpS {
            op: VecIntCmpOp::SCmpHi64x2,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C01038FB",
        "vchgs %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecIntCmpS {
            op: VecIntCmpOp::UCmpHi8x16,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C01008F9",
        "vchlbs %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecIntCmpS {
            op: VecIntCmpOp::UCmpHi16x8,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C01018F9",
        "vchlhs %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecIntCmpS {
            op: VecIntCmpOp::UCmpHi32x4,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C01028F9",
        "vchlfs %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecIntCmpS {
            op: VecIntCmpOp::UCmpHi64x2,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C01038F9",
        "vchlgs %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecInt128SCmpHi {
            tmp: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E7C8000030DBA7740005E748C01038F9",
        "vecg %v12, %v8 ; jne 10 ; vchlgs %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecInt128UCmpHi {
            tmp: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E7C8000030D9A7740005E748C01038F9",
        "veclg %v12, %v8 ; jne 10 ; vchlgs %v20, %v8, %v12",
    ));

    insns.push((
        Inst::VecFloatCmp {
            op: VecFloatCmpOp::CmpEq32x4,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C00028E8",
        "vfcesb %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecFloatCmp {
            op: VecFloatCmpOp::CmpEq64x2,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C00038E8",
        "vfcedb %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecFloatCmp {
            op: VecFloatCmpOp::CmpHi32x4,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C00028EB",
        "vfchsb %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecFloatCmp {
            op: VecFloatCmpOp::CmpHi64x2,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C00038EB",
        "vfchdb %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecFloatCmp {
            op: VecFloatCmpOp::CmpHiEq32x4,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C00028EA",
        "vfchesb %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecFloatCmp {
            op: VecFloatCmpOp::CmpHiEq64x2,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C00038EA",
        "vfchedb %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecFloatCmpS {
            op: VecFloatCmpOp::CmpEq32x4,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C01028E8",
        "vfcesbs %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecFloatCmpS {
            op: VecFloatCmpOp::CmpEq64x2,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C01038E8",
        "vfcedbs %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecFloatCmpS {
            op: VecFloatCmpOp::CmpHi32x4,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C01028EB",
        "vfchsbs %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecFloatCmpS {
            op: VecFloatCmpOp::CmpHi64x2,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C01038EB",
        "vfchdbs %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecFloatCmpS {
            op: VecFloatCmpOp::CmpHiEq32x4,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C01028EA",
        "vfchesbs %v20, %v8, %v12",
    ));
    insns.push((
        Inst::VecFloatCmpS {
            op: VecFloatCmpOp::CmpHiEq64x2,
            rd: writable_vr(20),
            rn: vr(8),
            rm: vr(12),
        },
        "E748C01038EA",
        "vfchedbs %v20, %v8, %v12",
    ));

    insns.push((
        Inst::VecLoad {
            rd: writable_vr(17),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E71020000806",
        "vl %v17, 0(%r2)",
    ));
    insns.push((
        Inst::VecLoad {
            rd: writable_vr(17),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E7102FFF0806",
        "vl %v17, 4095(%r2)",
    ));
    insns.push((
        Inst::VecLoad {
            rd: writable_vr(17),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E71230000806",
        "vl %v17, 0(%r2,%r3)",
    ));
    insns.push((
        Inst::VecLoadRev {
            rd: writable_vr(17),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E61020004806",
        "vlbrq %v17, 0(%r2)",
    ));
    insns.push((
        Inst::VecLoadRev {
            rd: writable_vr(17),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E6102FFF4806",
        "vlbrq %v17, 4095(%r2)",
    ));
    insns.push((
        Inst::VecLoadRev {
            rd: writable_vr(17),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E61230004806",
        "vlbrq %v17, 0(%r2,%r3)",
    ));
    insns.push((
        Inst::VecLoadByte16Rev {
            rd: writable_vr(17),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E61020001806",
        "vlbrh %v17, 0(%r2)",
    ));
    insns.push((
        Inst::VecLoadByte16Rev {
            rd: writable_vr(17),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E6102FFF1806",
        "vlbrh %v17, 4095(%r2)",
    ));
    insns.push((
        Inst::VecLoadByte16Rev {
            rd: writable_vr(17),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E61230001806",
        "vlbrh %v17, 0(%r2,%r3)",
    ));
    insns.push((
        Inst::VecLoadByte32Rev {
            rd: writable_vr(17),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E61020002806",
        "vlbrf %v17, 0(%r2)",
    ));
    insns.push((
        Inst::VecLoadByte32Rev {
            rd: writable_vr(17),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E6102FFF2806",
        "vlbrf %v17, 4095(%r2)",
    ));
    insns.push((
        Inst::VecLoadByte32Rev {
            rd: writable_vr(17),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E61230002806",
        "vlbrf %v17, 0(%r2,%r3)",
    ));
    insns.push((
        Inst::VecLoadByte64Rev {
            rd: writable_vr(17),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E61020003806",
        "vlbrg %v17, 0(%r2)",
    ));
    insns.push((
        Inst::VecLoadByte64Rev {
            rd: writable_vr(17),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E6102FFF3806",
        "vlbrg %v17, 4095(%r2)",
    ));
    insns.push((
        Inst::VecLoadByte64Rev {
            rd: writable_vr(17),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E61230003806",
        "vlbrg %v17, 0(%r2,%r3)",
    ));
    insns.push((
        Inst::VecLoadElt16Rev {
            rd: writable_vr(17),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E61020001807",
        "vlerh %v17, 0(%r2)",
    ));
    insns.push((
        Inst::VecLoadElt16Rev {
            rd: writable_vr(17),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E6102FFF1807",
        "vlerh %v17, 4095(%r2)",
    ));
    insns.push((
        Inst::VecLoadElt16Rev {
            rd: writable_vr(17),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E61230001807",
        "vlerh %v17, 0(%r2,%r3)",
    ));
    insns.push((
        Inst::VecLoadElt32Rev {
            rd: writable_vr(17),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E61020002807",
        "vlerf %v17, 0(%r2)",
    ));
    insns.push((
        Inst::VecLoadElt32Rev {
            rd: writable_vr(17),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E6102FFF2807",
        "vlerf %v17, 4095(%r2)",
    ));
    insns.push((
        Inst::VecLoadElt32Rev {
            rd: writable_vr(17),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E61230002807",
        "vlerf %v17, 0(%r2,%r3)",
    ));
    insns.push((
        Inst::VecLoadElt64Rev {
            rd: writable_vr(17),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E61020003807",
        "vlerg %v17, 0(%r2)",
    ));
    insns.push((
        Inst::VecLoadElt64Rev {
            rd: writable_vr(17),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E6102FFF3807",
        "vlerg %v17, 4095(%r2)",
    ));
    insns.push((
        Inst::VecLoadElt64Rev {
            rd: writable_vr(17),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E61230003807",
        "vlerg %v17, 0(%r2,%r3)",
    ));
    insns.push((
        Inst::VecStore {
            rd: vr(17),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E7102000080E",
        "vst %v17, 0(%r2)",
    ));
    insns.push((
        Inst::VecStore {
            rd: vr(17),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E7102FFF080E",
        "vst %v17, 4095(%r2)",
    ));
    insns.push((
        Inst::VecStore {
            rd: vr(17),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E7123000080E",
        "vst %v17, 0(%r2,%r3)",
    ));
    insns.push((
        Inst::VecStoreRev {
            rd: vr(17),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E6102000480E",
        "vstbrq %v17, 0(%r2)",
    ));
    insns.push((
        Inst::VecStoreRev {
            rd: vr(17),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E6102FFF480E",
        "vstbrq %v17, 4095(%r2)",
    ));
    insns.push((
        Inst::VecStoreRev {
            rd: vr(17),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E6123000480E",
        "vstbrq %v17, 0(%r2,%r3)",
    ));
    insns.push((
        Inst::VecStoreByte16Rev {
            rd: vr(17),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E6102000180E",
        "vstbrh %v17, 0(%r2)",
    ));
    insns.push((
        Inst::VecStoreByte16Rev {
            rd: vr(17),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E6102FFF180E",
        "vstbrh %v17, 4095(%r2)",
    ));
    insns.push((
        Inst::VecStoreByte16Rev {
            rd: vr(17),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E6123000180E",
        "vstbrh %v17, 0(%r2,%r3)",
    ));
    insns.push((
        Inst::VecStoreByte32Rev {
            rd: vr(17),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E6102000280E",
        "vstbrf %v17, 0(%r2)",
    ));
    insns.push((
        Inst::VecStoreByte32Rev {
            rd: vr(17),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E6102FFF280E",
        "vstbrf %v17, 4095(%r2)",
    ));
    insns.push((
        Inst::VecStoreByte32Rev {
            rd: vr(17),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E6123000280E",
        "vstbrf %v17, 0(%r2,%r3)",
    ));
    insns.push((
        Inst::VecStoreByte64Rev {
            rd: vr(17),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E6102000380E",
        "vstbrg %v17, 0(%r2)",
    ));
    insns.push((
        Inst::VecStoreByte64Rev {
            rd: vr(17),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E6102FFF380E",
        "vstbrg %v17, 4095(%r2)",
    ));
    insns.push((
        Inst::VecStoreByte64Rev {
            rd: vr(17),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E6123000380E",
        "vstbrg %v17, 0(%r2,%r3)",
    ));
    insns.push((
        Inst::VecStoreElt16Rev {
            rd: vr(17),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E6102000180F",
        "vsterh %v17, 0(%r2)",
    ));
    insns.push((
        Inst::VecStoreElt16Rev {
            rd: vr(17),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E6102FFF180F",
        "vsterh %v17, 4095(%r2)",
    ));
    insns.push((
        Inst::VecStoreElt16Rev {
            rd: vr(17),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E6123000180F",
        "vsterh %v17, 0(%r2,%r3)",
    ));
    insns.push((
        Inst::VecStoreElt32Rev {
            rd: vr(17),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E6102000280F",
        "vsterf %v17, 0(%r2)",
    ));
    insns.push((
        Inst::VecStoreElt32Rev {
            rd: vr(17),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E6102FFF280F",
        "vsterf %v17, 4095(%r2)",
    ));
    insns.push((
        Inst::VecStoreElt32Rev {
            rd: vr(17),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E6123000280F",
        "vsterf %v17, 0(%r2,%r3)",
    ));
    insns.push((
        Inst::VecStoreElt64Rev {
            rd: vr(17),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E6102000380F",
        "vsterg %v17, 0(%r2)",
    ));
    insns.push((
        Inst::VecStoreElt64Rev {
            rd: vr(17),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E6102FFF380F",
        "vsterg %v17, 4095(%r2)",
    ));
    insns.push((
        Inst::VecStoreElt64Rev {
            rd: vr(17),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
        },
        "E6123000380F",
        "vsterg %v17, 0(%r2,%r3)",
    ));
    insns.push((
        Inst::VecLoadReplicate {
            size: 8,
            rd: writable_vr(17),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::maybe_from_u64(128).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E71020800805",
        "vlrepb %v17, 128(%r2)",
    ));
    insns.push((
        Inst::VecLoadReplicate {
            size: 16,
            rd: writable_vr(17),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::maybe_from_u64(128).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E71020801805",
        "vlreph %v17, 128(%r2)",
    ));
    insns.push((
        Inst::VecLoadReplicate {
            size: 32,
            rd: writable_vr(17),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::maybe_from_u64(128).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E71020802805",
        "vlrepf %v17, 128(%r2)",
    ));
    insns.push((
        Inst::VecLoadReplicate {
            size: 64,
            rd: writable_vr(17),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::maybe_from_u64(128).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E71020803805",
        "vlrepg %v17, 128(%r2)",
    ));
    insns.push((
        Inst::VecLoadReplicateRev {
            size: 16,
            rd: writable_vr(17),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::maybe_from_u64(128).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E61020801805",
        "vlbrreph %v17, 128(%r2)",
    ));
    insns.push((
        Inst::VecLoadReplicateRev {
            size: 32,
            rd: writable_vr(17),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::maybe_from_u64(128).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E61020802805",
        "vlbrrepf %v17, 128(%r2)",
    ));
    insns.push((
        Inst::VecLoadReplicateRev {
            size: 64,
            rd: writable_vr(17),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::maybe_from_u64(128).unwrap(),
                flags: MemFlags::trusted(),
            },
        },
        "E61020803805",
        "vlbrrepg %v17, 128(%r2)",
    ));

    insns.push((
        Inst::VecMov {
            rd: writable_vr(8),
            rn: vr(20),
        },
        "E78400000456",
        "vlr %v8, %v20",
    ));
    insns.push((
        Inst::VecCMov {
            rd: writable_vr(8),
            ri: vr(8),
            rm: vr(20),
            cond: Cond::from_mask(1),
        },
        "A7E40005E78400000456",
        "jno 10 ; vlr %v8, %v20",
    ));
    insns.push((
        Inst::MovToVec128 {
            rd: writable_vr(20),
            rn: gpr(5),
            rm: gpr(6),
        },
        "E74560000862",
        "vlvgp %v20, %r5, %r6",
    ));

    insns.push((
        Inst::VecImmByteMask {
            rd: writable_vr(20),
            mask: 0x1234,
        },
        "E74012340844",
        "vgbm %v20, 4660",
    ));
    insns.push((
        Inst::VecImmBitMask {
            size: 8,
            rd: writable_vr(20),
            start_bit: 1,
            end_bit: 7,
        },
        "E74001070846",
        "vgmb %v20, 1, 7",
    ));
    insns.push((
        Inst::VecImmBitMask {
            size: 16,
            rd: writable_vr(20),
            start_bit: 1,
            end_bit: 7,
        },
        "E74001071846",
        "vgmh %v20, 1, 7",
    ));
    insns.push((
        Inst::VecImmBitMask {
            size: 32,
            rd: writable_vr(20),
            start_bit: 1,
            end_bit: 7,
        },
        "E74001072846",
        "vgmf %v20, 1, 7",
    ));
    insns.push((
        Inst::VecImmBitMask {
            size: 64,
            rd: writable_vr(20),
            start_bit: 1,
            end_bit: 7,
        },
        "E74001073846",
        "vgmg %v20, 1, 7",
    ));
    insns.push((
        Inst::VecImmReplicate {
            size: 8,
            rd: writable_vr(20),
            imm: 0x1234,
        },
        "E74012340845",
        "vrepib %v20, 4660",
    ));
    insns.push((
        Inst::VecImmReplicate {
            size: 16,
            rd: writable_vr(20),
            imm: 0x1234,
        },
        "E74012341845",
        "vrepih %v20, 4660",
    ));
    insns.push((
        Inst::VecImmReplicate {
            size: 32,
            rd: writable_vr(20),
            imm: 0x1234,
        },
        "E74012342845",
        "vrepif %v20, 4660",
    ));
    insns.push((
        Inst::VecImmReplicate {
            size: 64,
            rd: writable_vr(20),
            imm: 0x1234,
        },
        "E74012343845",
        "vrepig %v20, 4660",
    ));

    insns.push((
        Inst::VecLoadLane {
            size: 8,
            rd: writable_vr(17),
            ri: vr(17),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 15,
        },
        "E7102000F800",
        "vleb %v17, 0(%r2), 15",
    ));
    insns.push((
        Inst::VecLoadLane {
            size: 8,
            rd: writable_vr(17),
            ri: vr(17),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "E7102FFF0800",
        "vleb %v17, 4095(%r2), 0",
    ));
    insns.push((
        Inst::VecLoadLane {
            size: 8,
            rd: writable_vr(17),
            ri: vr(17),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 15,
        },
        "E7123000F800",
        "vleb %v17, 0(%r2,%r3), 15",
    ));
    insns.push((
        Inst::VecLoadLane {
            size: 8,
            rd: writable_vr(17),
            ri: vr(17),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "E7123FFF0800",
        "vleb %v17, 4095(%r2,%r3), 0",
    ));
    insns.push((
        Inst::VecLoadLane {
            size: 16,
            rd: writable_vr(17),
            ri: vr(17),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 7,
        },
        "E71020007801",
        "vleh %v17, 0(%r2), 7",
    ));
    insns.push((
        Inst::VecLoadLane {
            size: 16,
            rd: writable_vr(17),
            ri: vr(17),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "E7102FFF0801",
        "vleh %v17, 4095(%r2), 0",
    ));
    insns.push((
        Inst::VecLoadLane {
            size: 16,
            rd: writable_vr(17),
            ri: vr(17),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 7,
        },
        "E71230007801",
        "vleh %v17, 0(%r2,%r3), 7",
    ));
    insns.push((
        Inst::VecLoadLane {
            size: 16,
            rd: writable_vr(17),
            ri: vr(17),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "E7123FFF0801",
        "vleh %v17, 4095(%r2,%r3), 0",
    ));
    insns.push((
        Inst::VecLoadLane {
            size: 32,
            rd: writable_vr(17),
            ri: vr(17),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 3,
        },
        "E71020003803",
        "vlef %v17, 0(%r2), 3",
    ));
    insns.push((
        Inst::VecLoadLane {
            size: 32,
            rd: writable_vr(17),
            ri: vr(17),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "E7102FFF0803",
        "vlef %v17, 4095(%r2), 0",
    ));
    insns.push((
        Inst::VecLoadLane {
            size: 32,
            rd: writable_vr(17),
            ri: vr(17),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 3,
        },
        "E71230003803",
        "vlef %v17, 0(%r2,%r3), 3",
    ));
    insns.push((
        Inst::VecLoadLane {
            size: 32,
            rd: writable_vr(17),
            ri: vr(17),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "E7123FFF0803",
        "vlef %v17, 4095(%r2,%r3), 0",
    ));
    insns.push((
        Inst::VecLoadLane {
            size: 64,
            rd: writable_vr(17),
            ri: vr(17),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 1,
        },
        "E71020001802",
        "vleg %v17, 0(%r2), 1",
    ));
    insns.push((
        Inst::VecLoadLane {
            size: 64,
            rd: writable_vr(17),
            ri: vr(17),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "E7102FFF0802",
        "vleg %v17, 4095(%r2), 0",
    ));
    insns.push((
        Inst::VecLoadLane {
            size: 64,
            rd: writable_vr(17),
            ri: vr(17),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 1,
        },
        "E71230001802",
        "vleg %v17, 0(%r2,%r3), 1",
    ));
    insns.push((
        Inst::VecLoadLane {
            size: 64,
            rd: writable_vr(17),
            ri: vr(17),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "E7123FFF0802",
        "vleg %v17, 4095(%r2,%r3), 0",
    ));
    insns.push((
        Inst::VecLoadLaneUndef {
            size: 32,
            rd: writable_vr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "78102000",
        "le %f1, 0(%r2)",
    ));
    insns.push((
        Inst::VecLoadLaneUndef {
            size: 32,
            rd: writable_vr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "78102FFF",
        "le %f1, 4095(%r2)",
    ));
    insns.push((
        Inst::VecLoadLaneUndef {
            size: 32,
            rd: writable_vr(1),
            mem: MemArg::BXD20 {
                base: gpr(2),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(-524288).unwrap(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "ED1020008064",
        "ley %f1, -524288(%r2)",
    ));
    insns.push((
        Inst::VecLoadLaneUndef {
            size: 32,
            rd: writable_vr(1),
            mem: MemArg::BXD20 {
                base: gpr(2),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(524287).unwrap(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "ED102FFF7F64",
        "ley %f1, 524287(%r2)",
    ));
    insns.push((
        Inst::VecLoadLaneUndef {
            size: 32,
            rd: writable_vr(17),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "E71020000803",
        "vlef %v17, 0(%r2), 0",
    ));
    insns.push((
        Inst::VecLoadLaneUndef {
            size: 32,
            rd: writable_vr(17),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "E7102FFF0803",
        "vlef %v17, 4095(%r2), 0",
    ));
    insns.push((
        Inst::VecLoadLaneUndef {
            size: 32,
            rd: writable_vr(1),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "78123000",
        "le %f1, 0(%r2,%r3)",
    ));
    insns.push((
        Inst::VecLoadLaneUndef {
            size: 32,
            rd: writable_vr(1),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "78123FFF",
        "le %f1, 4095(%r2,%r3)",
    ));
    insns.push((
        Inst::VecLoadLaneUndef {
            size: 32,
            rd: writable_vr(1),
            mem: MemArg::BXD20 {
                base: gpr(3),
                index: gpr(2),
                disp: SImm20::maybe_from_i64(-524288).unwrap(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "ED1230008064",
        "ley %f1, -524288(%r2,%r3)",
    ));
    insns.push((
        Inst::VecLoadLaneUndef {
            size: 32,
            rd: writable_vr(1),
            mem: MemArg::BXD20 {
                base: gpr(3),
                index: gpr(2),
                disp: SImm20::maybe_from_i64(524287).unwrap(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "ED123FFF7F64",
        "ley %f1, 524287(%r2,%r3)",
    ));
    insns.push((
        Inst::VecLoadLaneUndef {
            size: 32,
            rd: writable_vr(17),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "E71230000803",
        "vlef %v17, 0(%r2,%r3), 0",
    ));
    insns.push((
        Inst::VecLoadLaneUndef {
            size: 32,
            rd: writable_vr(17),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "E7123FFF0803",
        "vlef %v17, 4095(%r2,%r3), 0",
    ));
    insns.push((
        Inst::VecLoadLaneUndef {
            size: 64,
            rd: writable_vr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "68102000",
        "ld %f1, 0(%r2)",
    ));
    insns.push((
        Inst::VecLoadLaneUndef {
            size: 64,
            rd: writable_vr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "68102FFF",
        "ld %f1, 4095(%r2)",
    ));
    insns.push((
        Inst::VecLoadLaneUndef {
            size: 64,
            rd: writable_vr(1),
            mem: MemArg::BXD20 {
                base: gpr(2),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(-524288).unwrap(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "ED1020008065",
        "ldy %f1, -524288(%r2)",
    ));
    insns.push((
        Inst::VecLoadLaneUndef {
            size: 64,
            rd: writable_vr(1),
            mem: MemArg::BXD20 {
                base: gpr(2),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(524287).unwrap(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "ED102FFF7F65",
        "ldy %f1, 524287(%r2)",
    ));
    insns.push((
        Inst::VecLoadLaneUndef {
            size: 64,
            rd: writable_vr(17),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "E71020000802",
        "vleg %v17, 0(%r2), 0",
    ));
    insns.push((
        Inst::VecLoadLaneUndef {
            size: 64,
            rd: writable_vr(17),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "E7102FFF0802",
        "vleg %v17, 4095(%r2), 0",
    ));
    insns.push((
        Inst::VecLoadLaneUndef {
            size: 64,
            rd: writable_vr(1),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "68123000",
        "ld %f1, 0(%r2,%r3)",
    ));
    insns.push((
        Inst::VecLoadLaneUndef {
            size: 64,
            rd: writable_vr(1),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "68123FFF",
        "ld %f1, 4095(%r2,%r3)",
    ));
    insns.push((
        Inst::VecLoadLaneUndef {
            size: 64,
            rd: writable_vr(1),
            mem: MemArg::BXD20 {
                base: gpr(3),
                index: gpr(2),
                disp: SImm20::maybe_from_i64(-524288).unwrap(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "ED1230008065",
        "ldy %f1, -524288(%r2,%r3)",
    ));
    insns.push((
        Inst::VecLoadLaneUndef {
            size: 64,
            rd: writable_vr(1),
            mem: MemArg::BXD20 {
                base: gpr(3),
                index: gpr(2),
                disp: SImm20::maybe_from_i64(524287).unwrap(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "ED123FFF7F65",
        "ldy %f1, 524287(%r2,%r3)",
    ));
    insns.push((
        Inst::VecLoadLaneUndef {
            size: 64,
            rd: writable_vr(17),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "E71230000802",
        "vleg %v17, 0(%r2,%r3), 0",
    ));
    insns.push((
        Inst::VecLoadLaneUndef {
            size: 64,
            rd: writable_vr(17),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "E7123FFF0802",
        "vleg %v17, 4095(%r2,%r3), 0",
    ));
    insns.push((
        Inst::VecStoreLane {
            size: 8,
            rd: vr(17),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 15,
        },
        "E7102000F808",
        "vsteb %v17, 0(%r2), 15",
    ));
    insns.push((
        Inst::VecStoreLane {
            size: 8,
            rd: vr(17),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "E7102FFF0808",
        "vsteb %v17, 4095(%r2), 0",
    ));
    insns.push((
        Inst::VecStoreLane {
            size: 8,
            rd: vr(17),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 15,
        },
        "E7123000F808",
        "vsteb %v17, 0(%r2,%r3), 15",
    ));
    insns.push((
        Inst::VecStoreLane {
            size: 8,
            rd: vr(17),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "E7123FFF0808",
        "vsteb %v17, 4095(%r2,%r3), 0",
    ));
    insns.push((
        Inst::VecStoreLane {
            size: 16,
            rd: vr(17),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 7,
        },
        "E71020007809",
        "vsteh %v17, 0(%r2), 7",
    ));
    insns.push((
        Inst::VecStoreLane {
            size: 16,
            rd: vr(17),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "E7102FFF0809",
        "vsteh %v17, 4095(%r2), 0",
    ));
    insns.push((
        Inst::VecStoreLane {
            size: 16,
            rd: vr(17),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 7,
        },
        "E71230007809",
        "vsteh %v17, 0(%r2,%r3), 7",
    ));
    insns.push((
        Inst::VecStoreLane {
            size: 16,
            rd: vr(17),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "E7123FFF0809",
        "vsteh %v17, 4095(%r2,%r3), 0",
    ));
    insns.push((
        Inst::VecStoreLane {
            size: 32,
            rd: vr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "70102000",
        "ste %f1, 0(%r2)",
    ));
    insns.push((
        Inst::VecStoreLane {
            size: 32,
            rd: vr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "70102FFF",
        "ste %f1, 4095(%r2)",
    ));
    insns.push((
        Inst::VecStoreLane {
            size: 32,
            rd: vr(1),
            mem: MemArg::BXD20 {
                base: gpr(2),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(-524288).unwrap(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "ED1020008066",
        "stey %f1, -524288(%r2)",
    ));
    insns.push((
        Inst::VecStoreLane {
            size: 32,
            rd: vr(1),
            mem: MemArg::BXD20 {
                base: gpr(2),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(524287).unwrap(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "ED102FFF7F66",
        "stey %f1, 524287(%r2)",
    ));
    insns.push((
        Inst::VecStoreLane {
            size: 32,
            rd: vr(17),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "E7102000080B",
        "vstef %v17, 0(%r2), 0",
    ));
    insns.push((
        Inst::VecStoreLane {
            size: 32,
            rd: vr(17),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "E7102FFF080B",
        "vstef %v17, 4095(%r2), 0",
    ));
    insns.push((
        Inst::VecStoreLane {
            size: 32,
            rd: vr(1),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "70123000",
        "ste %f1, 0(%r2,%r3)",
    ));
    insns.push((
        Inst::VecStoreLane {
            size: 32,
            rd: vr(1),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "70123FFF",
        "ste %f1, 4095(%r2,%r3)",
    ));
    insns.push((
        Inst::VecStoreLane {
            size: 32,
            rd: vr(1),
            mem: MemArg::BXD20 {
                base: gpr(3),
                index: gpr(2),
                disp: SImm20::maybe_from_i64(-524288).unwrap(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "ED1230008066",
        "stey %f1, -524288(%r2,%r3)",
    ));
    insns.push((
        Inst::VecStoreLane {
            size: 32,
            rd: vr(1),
            mem: MemArg::BXD20 {
                base: gpr(3),
                index: gpr(2),
                disp: SImm20::maybe_from_i64(524287).unwrap(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "ED123FFF7F66",
        "stey %f1, 524287(%r2,%r3)",
    ));
    insns.push((
        Inst::VecStoreLane {
            size: 32,
            rd: vr(17),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "E7123000080B",
        "vstef %v17, 0(%r2,%r3), 0",
    ));
    insns.push((
        Inst::VecStoreLane {
            size: 32,
            rd: vr(17),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "E7123FFF080B",
        "vstef %v17, 4095(%r2,%r3), 0",
    ));
    insns.push((
        Inst::VecStoreLane {
            size: 64,
            rd: vr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "60102000",
        "std %f1, 0(%r2)",
    ));
    insns.push((
        Inst::VecStoreLane {
            size: 64,
            rd: vr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "60102FFF",
        "std %f1, 4095(%r2)",
    ));
    insns.push((
        Inst::VecStoreLane {
            size: 64,
            rd: vr(1),
            mem: MemArg::BXD20 {
                base: gpr(2),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(-524288).unwrap(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "ED1020008067",
        "stdy %f1, -524288(%r2)",
    ));
    insns.push((
        Inst::VecStoreLane {
            size: 64,
            rd: vr(1),
            mem: MemArg::BXD20 {
                base: gpr(2),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(524287).unwrap(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "ED102FFF7F67",
        "stdy %f1, 524287(%r2)",
    ));
    insns.push((
        Inst::VecStoreLane {
            size: 64,
            rd: vr(17),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "E7102000080A",
        "vsteg %v17, 0(%r2), 0",
    ));
    insns.push((
        Inst::VecStoreLane {
            size: 64,
            rd: vr(17),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "E7102FFF080A",
        "vsteg %v17, 4095(%r2), 0",
    ));
    insns.push((
        Inst::VecStoreLane {
            size: 64,
            rd: vr(1),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "60123000",
        "std %f1, 0(%r2,%r3)",
    ));
    insns.push((
        Inst::VecStoreLane {
            size: 64,
            rd: vr(1),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "60123FFF",
        "std %f1, 4095(%r2,%r3)",
    ));
    insns.push((
        Inst::VecStoreLane {
            size: 64,
            rd: vr(1),
            mem: MemArg::BXD20 {
                base: gpr(3),
                index: gpr(2),
                disp: SImm20::maybe_from_i64(-524288).unwrap(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "ED1230008067",
        "stdy %f1, -524288(%r2,%r3)",
    ));
    insns.push((
        Inst::VecStoreLane {
            size: 64,
            rd: vr(1),
            mem: MemArg::BXD20 {
                base: gpr(3),
                index: gpr(2),
                disp: SImm20::maybe_from_i64(524287).unwrap(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "ED123FFF7F67",
        "stdy %f1, 524287(%r2,%r3)",
    ));
    insns.push((
        Inst::VecStoreLane {
            size: 64,
            rd: vr(17),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "E7123000080A",
        "vsteg %v17, 0(%r2,%r3), 0",
    ));
    insns.push((
        Inst::VecStoreLane {
            size: 64,
            rd: vr(17),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "E7123FFF080A",
        "vsteg %v17, 4095(%r2,%r3), 0",
    ));
    insns.push((
        Inst::VecLoadLaneRev {
            size: 16,
            rd: writable_vr(1),
            ri: vr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "E61020000001",
        "vlebrh %v1, 0(%r2), 0",
    ));
    insns.push((
        Inst::VecLoadLaneRev {
            size: 16,
            rd: writable_vr(1),
            ri: vr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "E6102FFF0001",
        "vlebrh %v1, 4095(%r2), 0",
    ));
    insns.push((
        Inst::VecLoadLaneRev {
            size: 16,
            rd: writable_vr(1),
            ri: vr(1),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "E61230000001",
        "vlebrh %v1, 0(%r2,%r3), 0",
    ));
    insns.push((
        Inst::VecLoadLaneRev {
            size: 16,
            rd: writable_vr(1),
            ri: vr(1),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "E6123FFF0001",
        "vlebrh %v1, 4095(%r2,%r3), 0",
    ));
    insns.push((
        Inst::VecLoadLaneRev {
            size: 32,
            rd: writable_vr(1),
            ri: vr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "E61020000003",
        "vlebrf %v1, 0(%r2), 0",
    ));
    insns.push((
        Inst::VecLoadLaneRev {
            size: 32,
            rd: writable_vr(1),
            ri: vr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "E6102FFF0003",
        "vlebrf %v1, 4095(%r2), 0",
    ));
    insns.push((
        Inst::VecLoadLaneRev {
            size: 32,
            rd: writable_vr(1),
            ri: vr(1),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "E61230000003",
        "vlebrf %v1, 0(%r2,%r3), 0",
    ));
    insns.push((
        Inst::VecLoadLaneRev {
            size: 32,
            rd: writable_vr(1),
            ri: vr(1),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "E6123FFF0003",
        "vlebrf %v1, 4095(%r2,%r3), 0",
    ));
    insns.push((
        Inst::VecLoadLaneRev {
            size: 64,
            rd: writable_vr(1),
            ri: vr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "E61020000002",
        "vlebrg %v1, 0(%r2), 0",
    ));
    insns.push((
        Inst::VecLoadLaneRev {
            size: 64,
            rd: writable_vr(1),
            ri: vr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "E6102FFF0002",
        "vlebrg %v1, 4095(%r2), 0",
    ));
    insns.push((
        Inst::VecLoadLaneRev {
            size: 64,
            rd: writable_vr(1),
            ri: vr(1),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "E61230000002",
        "vlebrg %v1, 0(%r2,%r3), 0",
    ));
    insns.push((
        Inst::VecLoadLaneRev {
            size: 64,
            rd: writable_vr(1),
            ri: vr(1),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "E6123FFF0002",
        "vlebrg %v1, 4095(%r2,%r3), 0",
    ));
    insns.push((
        Inst::VecLoadLaneRevUndef {
            size: 32,
            rd: writable_vr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "E61020000003",
        "vlebrf %v1, 0(%r2), 0",
    ));
    insns.push((
        Inst::VecLoadLaneRevUndef {
            size: 32,
            rd: writable_vr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "E6102FFF0003",
        "vlebrf %v1, 4095(%r2), 0",
    ));
    insns.push((
        Inst::VecLoadLaneRevUndef {
            size: 32,
            rd: writable_vr(1),
            mem: MemArg::BXD20 {
                base: gpr(2),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(-524288).unwrap(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "E31020008071E61010000003",
        "lay %r1, -524288(%r2) ; vlebrf %v1, 0(%r1), 0",
    ));
    insns.push((
        Inst::VecLoadLaneRevUndef {
            size: 32,
            rd: writable_vr(1),
            mem: MemArg::BXD20 {
                base: gpr(2),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(524287).unwrap(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "E3102FFF7F71E61010000003",
        "lay %r1, 524287(%r2) ; vlebrf %v1, 0(%r1), 0",
    ));
    insns.push((
        Inst::VecLoadLaneRevUndef {
            size: 32,
            rd: writable_vr(1),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "E61230000003",
        "vlebrf %v1, 0(%r2,%r3), 0",
    ));
    insns.push((
        Inst::VecLoadLaneRevUndef {
            size: 32,
            rd: writable_vr(1),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "E6123FFF0003",
        "vlebrf %v1, 4095(%r2,%r3), 0",
    ));
    insns.push((
        Inst::VecLoadLaneRevUndef {
            size: 32,
            rd: writable_vr(1),
            mem: MemArg::BXD20 {
                base: gpr(3),
                index: gpr(2),
                disp: SImm20::maybe_from_i64(-524288).unwrap(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "E31230008071E61010000003",
        "lay %r1, -524288(%r2,%r3) ; vlebrf %v1, 0(%r1), 0",
    ));
    insns.push((
        Inst::VecLoadLaneRevUndef {
            size: 32,
            rd: writable_vr(1),
            mem: MemArg::BXD20 {
                base: gpr(3),
                index: gpr(2),
                disp: SImm20::maybe_from_i64(524287).unwrap(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "E3123FFF7F71E61010000003",
        "lay %r1, 524287(%r2,%r3) ; vlebrf %v1, 0(%r1), 0",
    ));
    insns.push((
        Inst::VecLoadLaneRevUndef {
            size: 64,
            rd: writable_vr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "E61020000002",
        "vlebrg %v1, 0(%r2), 0",
    ));
    insns.push((
        Inst::VecLoadLaneRevUndef {
            size: 64,
            rd: writable_vr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "E6102FFF0002",
        "vlebrg %v1, 4095(%r2), 0",
    ));
    insns.push((
        Inst::VecLoadLaneRevUndef {
            size: 64,
            rd: writable_vr(1),
            mem: MemArg::BXD20 {
                base: gpr(2),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(-524288).unwrap(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "E31020008071E61010000002",
        "lay %r1, -524288(%r2) ; vlebrg %v1, 0(%r1), 0",
    ));
    insns.push((
        Inst::VecLoadLaneRevUndef {
            size: 64,
            rd: writable_vr(1),
            mem: MemArg::BXD20 {
                base: gpr(2),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(524287).unwrap(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "E3102FFF7F71E61010000002",
        "lay %r1, 524287(%r2) ; vlebrg %v1, 0(%r1), 0",
    ));
    insns.push((
        Inst::VecLoadLaneRevUndef {
            size: 64,
            rd: writable_vr(1),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "E61230000002",
        "vlebrg %v1, 0(%r2,%r3), 0",
    ));
    insns.push((
        Inst::VecLoadLaneRevUndef {
            size: 64,
            rd: writable_vr(1),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "E6123FFF0002",
        "vlebrg %v1, 4095(%r2,%r3), 0",
    ));
    insns.push((
        Inst::VecLoadLaneRevUndef {
            size: 64,
            rd: writable_vr(1),
            mem: MemArg::BXD20 {
                base: gpr(3),
                index: gpr(2),
                disp: SImm20::maybe_from_i64(-524288).unwrap(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "E31230008071E61010000002",
        "lay %r1, -524288(%r2,%r3) ; vlebrg %v1, 0(%r1), 0",
    ));
    insns.push((
        Inst::VecLoadLaneRevUndef {
            size: 64,
            rd: writable_vr(1),
            mem: MemArg::BXD20 {
                base: gpr(3),
                index: gpr(2),
                disp: SImm20::maybe_from_i64(524287).unwrap(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "E3123FFF7F71E61010000002",
        "lay %r1, 524287(%r2,%r3) ; vlebrg %v1, 0(%r1), 0",
    ));
    insns.push((
        Inst::VecStoreLaneRev {
            size: 16,
            rd: vr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 7,
        },
        "E61020007009",
        "vstebrh %v1, 0(%r2), 7",
    ));
    insns.push((
        Inst::VecStoreLaneRev {
            size: 16,
            rd: vr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "E6102FFF0009",
        "vstebrh %v1, 4095(%r2), 0",
    ));
    insns.push((
        Inst::VecStoreLaneRev {
            size: 16,
            rd: vr(1),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 7,
        },
        "E61230007009",
        "vstebrh %v1, 0(%r2,%r3), 7",
    ));
    insns.push((
        Inst::VecStoreLaneRev {
            size: 16,
            rd: vr(1),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "E6123FFF0009",
        "vstebrh %v1, 4095(%r2,%r3), 0",
    ));
    insns.push((
        Inst::VecStoreLaneRev {
            size: 32,
            rd: vr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "E6102000000B",
        "vstebrf %v1, 0(%r2), 0",
    ));
    insns.push((
        Inst::VecStoreLaneRev {
            size: 32,
            rd: vr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "E6102FFF000B",
        "vstebrf %v1, 4095(%r2), 0",
    ));
    insns.push((
        Inst::VecStoreLaneRev {
            size: 32,
            rd: vr(1),
            mem: MemArg::BXD20 {
                base: gpr(2),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(-524288).unwrap(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "E31020008071E6101000000B",
        "lay %r1, -524288(%r2) ; vstebrf %v1, 0(%r1), 0",
    ));
    insns.push((
        Inst::VecStoreLaneRev {
            size: 32,
            rd: vr(1),
            mem: MemArg::BXD20 {
                base: gpr(2),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(524287).unwrap(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "E3102FFF7F71E6101000000B",
        "lay %r1, 524287(%r2) ; vstebrf %v1, 0(%r1), 0",
    ));
    insns.push((
        Inst::VecStoreLaneRev {
            size: 32,
            rd: vr(1),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "E6123000000B",
        "vstebrf %v1, 0(%r2,%r3), 0",
    ));
    insns.push((
        Inst::VecStoreLaneRev {
            size: 32,
            rd: vr(1),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "E6123FFF000B",
        "vstebrf %v1, 4095(%r2,%r3), 0",
    ));
    insns.push((
        Inst::VecStoreLaneRev {
            size: 32,
            rd: vr(1),
            mem: MemArg::BXD20 {
                base: gpr(3),
                index: gpr(2),
                disp: SImm20::maybe_from_i64(-524288).unwrap(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "E31230008071E6101000000B",
        "lay %r1, -524288(%r2,%r3) ; vstebrf %v1, 0(%r1), 0",
    ));
    insns.push((
        Inst::VecStoreLaneRev {
            size: 32,
            rd: vr(1),
            mem: MemArg::BXD20 {
                base: gpr(3),
                index: gpr(2),
                disp: SImm20::maybe_from_i64(524287).unwrap(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "E3123FFF7F71E6101000000B",
        "lay %r1, 524287(%r2,%r3) ; vstebrf %v1, 0(%r1), 0",
    ));
    insns.push((
        Inst::VecStoreLaneRev {
            size: 64,
            rd: vr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "E6102000000A",
        "vstebrg %v1, 0(%r2), 0",
    ));
    insns.push((
        Inst::VecStoreLaneRev {
            size: 64,
            rd: vr(1),
            mem: MemArg::BXD12 {
                base: gpr(2),
                index: zero_reg(),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "E6102FFF000A",
        "vstebrg %v1, 4095(%r2), 0",
    ));
    insns.push((
        Inst::VecStoreLaneRev {
            size: 64,
            rd: vr(1),
            mem: MemArg::BXD20 {
                base: gpr(2),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(-524288).unwrap(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "E31020008071E6101000000A",
        "lay %r1, -524288(%r2) ; vstebrg %v1, 0(%r1), 0",
    ));
    insns.push((
        Inst::VecStoreLaneRev {
            size: 64,
            rd: vr(1),
            mem: MemArg::BXD20 {
                base: gpr(2),
                index: zero_reg(),
                disp: SImm20::maybe_from_i64(524287).unwrap(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "E3102FFF7F71E6101000000A",
        "lay %r1, 524287(%r2) ; vstebrg %v1, 0(%r1), 0",
    ));
    insns.push((
        Inst::VecStoreLaneRev {
            size: 64,
            rd: vr(1),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::zero(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "E6123000000A",
        "vstebrg %v1, 0(%r2,%r3), 0",
    ));
    insns.push((
        Inst::VecStoreLaneRev {
            size: 64,
            rd: vr(1),
            mem: MemArg::BXD12 {
                base: gpr(3),
                index: gpr(2),
                disp: UImm12::maybe_from_u64(4095).unwrap(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "E6123FFF000A",
        "vstebrg %v1, 4095(%r2,%r3), 0",
    ));
    insns.push((
        Inst::VecStoreLaneRev {
            size: 64,
            rd: vr(1),
            mem: MemArg::BXD20 {
                base: gpr(3),
                index: gpr(2),
                disp: SImm20::maybe_from_i64(-524288).unwrap(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "E31230008071E6101000000A",
        "lay %r1, -524288(%r2,%r3) ; vstebrg %v1, 0(%r1), 0",
    ));
    insns.push((
        Inst::VecStoreLaneRev {
            size: 64,
            rd: vr(1),
            mem: MemArg::BXD20 {
                base: gpr(3),
                index: gpr(2),
                disp: SImm20::maybe_from_i64(524287).unwrap(),
                flags: MemFlags::trusted(),
            },
            lane_imm: 0,
        },
        "E3123FFF7F71E6101000000A",
        "lay %r1, 524287(%r2,%r3) ; vstebrg %v1, 0(%r1), 0",
    ));

    insns.push((
        Inst::VecInsertLane {
            size: 8,
            rd: writable_vr(8),
            ri: vr(8),
            rn: gpr(4),
            lane_imm: 0,
            lane_reg: zero_reg(),
        },
        "E78400000022",
        "vlvgb %v8, %r4, 0",
    ));
    insns.push((
        Inst::VecInsertLane {
            size: 8,
            rd: writable_vr(8),
            ri: vr(8),
            rn: gpr(4),
            lane_imm: 255,
            lane_reg: zero_reg(),
        },
        "E78400FF0022",
        "vlvgb %v8, %r4, 255",
    ));
    insns.push((
        Inst::VecInsertLane {
            size: 8,
            rd: writable_vr(24),
            ri: vr(24),
            rn: gpr(4),
            lane_imm: 0,
            lane_reg: gpr(3),
        },
        "E78430000822",
        "vlvgb %v24, %r4, 0(%r3)",
    ));
    insns.push((
        Inst::VecInsertLane {
            size: 16,
            rd: writable_vr(8),
            ri: vr(8),
            rn: gpr(4),
            lane_imm: 0,
            lane_reg: zero_reg(),
        },
        "E78400001022",
        "vlvgh %v8, %r4, 0",
    ));
    insns.push((
        Inst::VecInsertLane {
            size: 16,
            rd: writable_vr(8),
            ri: vr(8),
            rn: gpr(4),
            lane_imm: 255,
            lane_reg: zero_reg(),
        },
        "E78400FF1022",
        "vlvgh %v8, %r4, 255",
    ));
    insns.push((
        Inst::VecInsertLane {
            size: 16,
            rd: writable_vr(24),
            ri: vr(24),
            rn: gpr(4),
            lane_imm: 0,
            lane_reg: gpr(3),
        },
        "E78430001822",
        "vlvgh %v24, %r4, 0(%r3)",
    ));
    insns.push((
        Inst::VecInsertLane {
            size: 32,
            rd: writable_vr(8),
            ri: vr(8),
            rn: gpr(4),
            lane_imm: 0,
            lane_reg: zero_reg(),
        },
        "E78400002022",
        "vlvgf %v8, %r4, 0",
    ));
    insns.push((
        Inst::VecInsertLane {
            size: 32,
            rd: writable_vr(8),
            ri: vr(8),
            rn: gpr(4),
            lane_imm: 255,
            lane_reg: zero_reg(),
        },
        "E78400FF2022",
        "vlvgf %v8, %r4, 255",
    ));
    insns.push((
        Inst::VecInsertLane {
            size: 32,
            rd: writable_vr(24),
            ri: vr(24),
            rn: gpr(4),
            lane_imm: 0,
            lane_reg: gpr(3),
        },
        "E78430002822",
        "vlvgf %v24, %r4, 0(%r3)",
    ));
    insns.push((
        Inst::VecInsertLane {
            size: 64,
            rd: writable_vr(8),
            ri: vr(8),
            rn: gpr(4),
            lane_imm: 0,
            lane_reg: zero_reg(),
        },
        "E78400003022",
        "vlvgg %v8, %r4, 0",
    ));
    insns.push((
        Inst::VecInsertLane {
            size: 64,
            rd: writable_vr(8),
            ri: vr(8),
            rn: gpr(4),
            lane_imm: 255,
            lane_reg: zero_reg(),
        },
        "E78400FF3022",
        "vlvgg %v8, %r4, 255",
    ));
    insns.push((
        Inst::VecInsertLane {
            size: 64,
            rd: writable_vr(24),
            ri: vr(24),
            rn: gpr(4),
            lane_imm: 0,
            lane_reg: gpr(3),
        },
        "E78430003822",
        "vlvgg %v24, %r4, 0(%r3)",
    ));
    insns.push((
        Inst::VecInsertLaneUndef {
            size: 8,
            rd: writable_vr(8),
            rn: gpr(4),
            lane_imm: 0,
            lane_reg: zero_reg(),
        },
        "E78400000022",
        "vlvgb %v8, %r4, 0",
    ));
    insns.push((
        Inst::VecInsertLaneUndef {
            size: 8,
            rd: writable_vr(8),
            rn: gpr(4),
            lane_imm: 255,
            lane_reg: zero_reg(),
        },
        "E78400FF0022",
        "vlvgb %v8, %r4, 255",
    ));
    insns.push((
        Inst::VecInsertLaneUndef {
            size: 8,
            rd: writable_vr(24),
            rn: gpr(4),
            lane_imm: 0,
            lane_reg: gpr(3),
        },
        "E78430000822",
        "vlvgb %v24, %r4, 0(%r3)",
    ));
    insns.push((
        Inst::VecInsertLaneUndef {
            size: 16,
            rd: writable_vr(8),
            rn: gpr(4),
            lane_imm: 0,
            lane_reg: zero_reg(),
        },
        "E78400001022",
        "vlvgh %v8, %r4, 0",
    ));
    insns.push((
        Inst::VecInsertLaneUndef {
            size: 16,
            rd: writable_vr(8),
            rn: gpr(4),
            lane_imm: 255,
            lane_reg: zero_reg(),
        },
        "E78400FF1022",
        "vlvgh %v8, %r4, 255",
    ));
    insns.push((
        Inst::VecInsertLaneUndef {
            size: 16,
            rd: writable_vr(24),
            rn: gpr(4),
            lane_imm: 0,
            lane_reg: gpr(3),
        },
        "E78430001822",
        "vlvgh %v24, %r4, 0(%r3)",
    ));
    insns.push((
        Inst::VecInsertLaneUndef {
            size: 32,
            rd: writable_vr(8),
            rn: gpr(4),
            lane_imm: 0,
            lane_reg: zero_reg(),
        },
        "E78400002022",
        "vlvgf %v8, %r4, 0",
    ));
    insns.push((
        Inst::VecInsertLaneUndef {
            size: 32,
            rd: writable_vr(8),
            rn: gpr(4),
            lane_imm: 255,
            lane_reg: zero_reg(),
        },
        "E78400FF2022",
        "vlvgf %v8, %r4, 255",
    ));
    insns.push((
        Inst::VecInsertLaneUndef {
            size: 32,
            rd: writable_vr(24),
            rn: gpr(4),
            lane_imm: 0,
            lane_reg: gpr(3),
        },
        "E78430002822",
        "vlvgf %v24, %r4, 0(%r3)",
    ));
    insns.push((
        Inst::VecInsertLaneUndef {
            size: 64,
            rd: writable_vr(8),
            rn: gpr(4),
            lane_imm: 0,
            lane_reg: zero_reg(),
        },
        "B3C10084",
        "ldgr %f8, %r4",
    ));
    insns.push((
        Inst::VecInsertLaneUndef {
            size: 64,
            rd: writable_vr(8),
            rn: gpr(4),
            lane_imm: 255,
            lane_reg: zero_reg(),
        },
        "E78400FF3022",
        "vlvgg %v8, %r4, 255",
    ));
    insns.push((
        Inst::VecInsertLaneUndef {
            size: 64,
            rd: writable_vr(8),
            rn: gpr(4),
            lane_imm: 0,
            lane_reg: gpr(3),
        },
        "E78430003022",
        "vlvgg %v8, %r4, 0(%r3)",
    ));
    insns.push((
        Inst::VecExtractLane {
            size: 8,
            rd: writable_gpr(8),
            rn: vr(4),
            lane_imm: 255,
            lane_reg: zero_reg(),
        },
        "E78400FF0021",
        "vlgvb %r8, %v4, 255",
    ));
    insns.push((
        Inst::VecExtractLane {
            size: 8,
            rd: writable_gpr(8),
            rn: vr(20),
            lane_imm: 0,
            lane_reg: gpr(3),
        },
        "E78430000421",
        "vlgvb %r8, %v20, 0(%r3)",
    ));
    insns.push((
        Inst::VecExtractLane {
            size: 16,
            rd: writable_gpr(8),
            rn: vr(4),
            lane_imm: 0,
            lane_reg: zero_reg(),
        },
        "E78400001021",
        "vlgvh %r8, %v4, 0",
    ));
    insns.push((
        Inst::VecExtractLane {
            size: 16,
            rd: writable_gpr(8),
            rn: vr(4),
            lane_imm: 255,
            lane_reg: zero_reg(),
        },
        "E78400FF1021",
        "vlgvh %r8, %v4, 255",
    ));
    insns.push((
        Inst::VecExtractLane {
            size: 16,
            rd: writable_gpr(8),
            rn: vr(20),
            lane_imm: 0,
            lane_reg: gpr(3),
        },
        "E78430001421",
        "vlgvh %r8, %v20, 0(%r3)",
    ));
    insns.push((
        Inst::VecExtractLane {
            size: 32,
            rd: writable_gpr(8),
            rn: vr(4),
            lane_imm: 0,
            lane_reg: zero_reg(),
        },
        "E78400002021",
        "vlgvf %r8, %v4, 0",
    ));
    insns.push((
        Inst::VecExtractLane {
            size: 32,
            rd: writable_gpr(8),
            rn: vr(4),
            lane_imm: 255,
            lane_reg: zero_reg(),
        },
        "E78400FF2021",
        "vlgvf %r8, %v4, 255",
    ));
    insns.push((
        Inst::VecExtractLane {
            size: 32,
            rd: writable_gpr(8),
            rn: vr(20),
            lane_imm: 0,
            lane_reg: gpr(3),
        },
        "E78430002421",
        "vlgvf %r8, %v20, 0(%r3)",
    ));
    insns.push((
        Inst::VecExtractLane {
            size: 64,
            rd: writable_gpr(8),
            rn: vr(4),
            lane_imm: 0,
            lane_reg: zero_reg(),
        },
        "B3CD0084",
        "lgdr %r8, %f4",
    ));
    insns.push((
        Inst::VecExtractLane {
            size: 64,
            rd: writable_gpr(8),
            rn: vr(4),
            lane_imm: 255,
            lane_reg: zero_reg(),
        },
        "E78400FF3021",
        "vlgvg %r8, %v4, 255",
    ));
    insns.push((
        Inst::VecExtractLane {
            size: 64,
            rd: writable_gpr(8),
            rn: vr(4),
            lane_imm: 0,
            lane_reg: gpr(3),
        },
        "E78430003021",
        "vlgvg %r8, %v4, 0(%r3)",
    ));
    insns.push((
        Inst::VecInsertLaneImm {
            size: 8,
            rd: writable_vr(20),
            ri: vr(20),
            imm: 0x1234,
            lane_imm: 15,
        },
        "E7401234F840",
        "vleib %v20, 4660, 15",
    ));
    insns.push((
        Inst::VecInsertLaneImm {
            size: 16,
            rd: writable_vr(20),
            ri: vr(20),
            imm: 0x1234,
            lane_imm: 7,
        },
        "E74012347841",
        "vleih %v20, 4660, 7",
    ));
    insns.push((
        Inst::VecInsertLaneImm {
            size: 32,
            rd: writable_vr(20),
            ri: vr(20),
            imm: 0x1234,
            lane_imm: 3,
        },
        "E74012343843",
        "vleif %v20, 4660, 3",
    ));
    insns.push((
        Inst::VecInsertLaneImm {
            size: 64,
            rd: writable_vr(20),
            ri: vr(20),
            imm: 0x1234,
            lane_imm: 1,
        },
        "E74012341842",
        "vleig %v20, 4660, 1",
    ));
    insns.push((
        Inst::VecInsertLaneImmUndef {
            size: 8,
            rd: writable_vr(20),
            imm: 0x1234,
            lane_imm: 15,
        },
        "E7401234F840",
        "vleib %v20, 4660, 15",
    ));
    insns.push((
        Inst::VecInsertLaneImmUndef {
            size: 16,
            rd: writable_vr(20),
            imm: 0x1234,
            lane_imm: 7,
        },
        "E74012347841",
        "vleih %v20, 4660, 7",
    ));
    insns.push((
        Inst::VecInsertLaneImmUndef {
            size: 32,
            rd: writable_vr(20),
            imm: 0x1234,
            lane_imm: 3,
        },
        "E74012343843",
        "vleif %v20, 4660, 3",
    ));
    insns.push((
        Inst::VecInsertLaneImmUndef {
            size: 64,
            rd: writable_vr(20),
            imm: 0x1234,
            lane_imm: 1,
        },
        "E74012341842",
        "vleig %v20, 4660, 1",
    ));
    insns.push((
        Inst::VecReplicateLane {
            size: 8,
            rd: writable_vr(20),
            rn: vr(8),
            lane_imm: 15,
        },
        "E748000F084D",
        "vrepb %v20, %v8, 15",
    ));
    insns.push((
        Inst::VecReplicateLane {
            size: 16,
            rd: writable_vr(20),
            rn: vr(8),
            lane_imm: 7,
        },
        "E7480007184D",
        "vreph %v20, %v8, 7",
    ));
    insns.push((
        Inst::VecReplicateLane {
            size: 32,
            rd: writable_vr(20),
            rn: vr(8),
            lane_imm: 3,
        },
        "E7480003284D",
        "vrepf %v20, %v8, 3",
    ));
    insns.push((
        Inst::VecReplicateLane {
            size: 64,
            rd: writable_vr(20),
            rn: vr(8),
            lane_imm: 1,
        },
        "E7480001384D",
        "vrepg %v20, %v8, 1",
    ));

    let flags = settings::Flags::new(settings::builder());

    use crate::settings::Configurable;
    let mut isa_flag_builder = s390x_settings::builder();
    isa_flag_builder.enable("arch13").unwrap();
    let isa_flags = s390x_settings::Flags::new(&flags, &isa_flag_builder);
    let ctrl_plane = &mut Default::default();
    let constants = Default::default();

    let emit_info = EmitInfo::new(isa_flags);
    for (insn, expected_encoding, expected_printing) in insns {
        println!("S390x: {insn:?}, {expected_encoding}, {expected_printing}");

        // Check the printed text is as expected.
        let actual_printing = insn.print_with_state(&mut EmitState::default());
        assert_eq!(expected_printing, actual_printing);

        let mut buffer = MachBuffer::new();

        // Label 0 before the instruction.
        let label0 = buffer.get_label();
        buffer.bind_label(label0, ctrl_plane);

        // Emit the instruction.
        insn.emit(&mut buffer, &emit_info, &mut Default::default());

        // Label 1 after the instruction.
        let label1 = buffer.get_label();
        buffer.bind_label(label1, ctrl_plane);

        let buffer = buffer.finish(&constants, ctrl_plane);
        let actual_encoding = &buffer.stringify_code_bytes();
        assert_eq!(expected_encoding, actual_encoding);
    }
}
