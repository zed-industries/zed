use crate::isa::riscv64::inst::*;
use crate::isa::riscv64::lower::isle::generated_code::FpuOPWidth;
use std::borrow::Cow;

fn fa7() -> Reg {
    f_reg(17)
}

#[test]
fn test_riscv64_binemit() {
    struct TestUnit {
        inst: Inst,
        assembly: &'static str,
        code: TestEncoding,
    }

    struct TestEncoding(Cow<'static, str>);

    impl From<&'static str> for TestEncoding {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }

    impl From<u32> for TestEncoding {
        fn from(value: u32) -> Self {
            let value = value.swap_bytes();
            let value = format!("{value:08X}");
            Self(value.into())
        }
    }

    impl TestUnit {
        fn new(inst: Inst, assembly: &'static str, code: impl Into<TestEncoding>) -> Self {
            let code = code.into();
            Self {
                inst,
                assembly,
                code,
            }
        }
    }

    let mut insns = Vec::<TestUnit>::with_capacity(500);

    insns.push(TestUnit::new(Inst::Ret {}, "ret", 0x00008067));

    insns.push(TestUnit::new(
        Inst::Mov {
            rd: writable_fa0(),
            rm: fa1(),
            ty: F32,
        },
        "fmv.s fa0,fa1",
        0x20b58553,
    ));

    insns.push(TestUnit::new(
        Inst::Mov {
            rd: writable_fa0(),
            rm: fa1(),
            ty: F64,
        },
        "fmv.d fa0,fa1",
        0x22b58553,
    ));

    insns.push(TestUnit::new(
        Inst::AluRRImm12 {
            alu_op: AluOPRRI::Brev8,
            rd: writable_a1(),
            rs: a0(),
            imm12: Imm12::ZERO,
        },
        "brev8 a1,a0",
        0x68755593,
    ));
    insns.push(TestUnit::new(
        Inst::AluRRImm12 {
            alu_op: AluOPRRI::Rev8,
            rd: writable_a1(),
            rs: a0(),
            imm12: Imm12::ZERO,
        },
        "rev8 a1,a0",
        0x6b855593,
    ));

    //
    insns.push(TestUnit::new(
        Inst::AluRRImm12 {
            alu_op: AluOPRRI::Bclri,
            rd: writable_a1(),
            rs: a0(),
            imm12: Imm12::from_i16(5),
        },
        "bclri a1,a0,5",
        0x48551593,
    ));
    insns.push(TestUnit::new(
        Inst::AluRRImm12 {
            alu_op: AluOPRRI::Bexti,
            rd: writable_a1(),
            rs: a0(),
            imm12: Imm12::from_i16(5),
        },
        "bexti a1,a0,5",
        0x48555593,
    ));

    insns.push(TestUnit::new(
        Inst::AluRRImm12 {
            alu_op: AluOPRRI::Binvi,
            rd: writable_a1(),
            rs: a0(),
            imm12: Imm12::from_i16(5),
        },
        "binvi a1,a0,5",
        0x68551593,
    ));

    insns.push(TestUnit::new(
        Inst::AluRRImm12 {
            alu_op: AluOPRRI::Bseti,
            rd: writable_a1(),
            rs: a0(),
            imm12: Imm12::from_i16(5),
        },
        "bseti a1,a0,5",
        0x28551593,
    ));

    insns.push(TestUnit::new(
        Inst::AluRRImm12 {
            alu_op: AluOPRRI::Rori,
            rd: writable_a1(),
            rs: a0(),
            imm12: Imm12::from_i16(5),
        },
        "rori a1,a0,5",
        0x60555593,
    ));
    insns.push(TestUnit::new(
        Inst::AluRRImm12 {
            alu_op: AluOPRRI::Roriw,
            rd: writable_a1(),
            rs: a0(),
            imm12: Imm12::from_i16(5),
        },
        "roriw a1,a0,5",
        0x6055559b,
    ));

    insns.push(TestUnit::new(
        Inst::AluRRImm12 {
            alu_op: AluOPRRI::SlliUw,
            rd: writable_a1(),
            rs: a0(),
            imm12: Imm12::from_i16(5),
        },
        "slli.uw a1,a0,5",
        0x855159b,
    ));

    insns.push(TestUnit::new(
        Inst::AluRRImm12 {
            alu_op: AluOPRRI::Clz,
            rd: writable_a1(),
            rs: a0(),
            imm12: Imm12::ZERO,
        },
        "clz a1,a0",
        0x60051593,
    ));

    insns.push(TestUnit::new(
        Inst::AluRRImm12 {
            alu_op: AluOPRRI::Clzw,
            rd: writable_a1(),
            rs: a0(),
            imm12: Imm12::ZERO,
        },
        "clzw a1,a0",
        0x6005159b,
    ));

    insns.push(TestUnit::new(
        Inst::AluRRImm12 {
            alu_op: AluOPRRI::Cpop,
            rd: writable_a1(),
            rs: a0(),
            imm12: Imm12::ZERO,
        },
        "cpop a1,a0",
        0x60251593,
    ));

    insns.push(TestUnit::new(
        Inst::AluRRImm12 {
            alu_op: AluOPRRI::Cpopw,
            rd: writable_a1(),
            rs: a0(),
            imm12: Imm12::ZERO,
        },
        "cpopw a1,a0",
        0x6025159b,
    ));

    insns.push(TestUnit::new(
        Inst::AluRRImm12 {
            alu_op: AluOPRRI::Ctz,
            rd: writable_a1(),
            rs: a0(),
            imm12: Imm12::ZERO,
        },
        "ctz a1,a0",
        0x60151593,
    ));

    insns.push(TestUnit::new(
        Inst::AluRRImm12 {
            alu_op: AluOPRRI::Ctzw,
            rd: writable_a1(),
            rs: a0(),
            imm12: Imm12::ZERO,
        },
        "ctzw a1,a0",
        0x6015159b,
    ));

    insns.push(TestUnit::new(
        Inst::AluRRImm12 {
            alu_op: AluOPRRI::Sextb,
            rd: writable_a1(),
            rs: a0(),
            imm12: Imm12::ZERO,
        },
        "sext.b a1,a0",
        0x60451593,
    ));
    insns.push(TestUnit::new(
        Inst::AluRRImm12 {
            alu_op: AluOPRRI::Sexth,
            rd: writable_a1(),
            rs: a0(),
            imm12: Imm12::ZERO,
        },
        "sext.h a1,a0",
        0x60551593,
    ));
    insns.push(TestUnit::new(
        Inst::AluRRImm12 {
            alu_op: AluOPRRI::Zexth,
            rd: writable_a1(),
            rs: a0(),
            imm12: Imm12::ZERO,
        },
        "zext.h a1,a0",
        0x80545bb,
    ));
    insns.push(TestUnit::new(
        Inst::AluRRImm12 {
            alu_op: AluOPRRI::Orcb,
            rd: writable_a1(),
            rs: a0(),
            imm12: Imm12::ZERO,
        },
        "orc.b a1,a0",
        0x28755593,
    ));

    insns.push(TestUnit::new(
        Inst::AluRRR {
            alu_op: AluOPRRR::Adduw,
            rd: writable_a1(),
            rs1: a0(),
            rs2: zero_reg(),
        },
        "zext.w a1,a0",
        0x80505bb,
    ));

    insns.push(TestUnit::new(
        Inst::AluRRR {
            alu_op: AluOPRRR::Adduw,
            rd: writable_a1(),
            rs1: a0(),
            rs2: a1(),
        },
        "add.uw a1,a0,a1",
        0x08b505bb,
    ));

    insns.push(TestUnit::new(
        Inst::AluRRR {
            alu_op: AluOPRRR::Andn,
            rd: writable_a1(),
            rs1: a0(),
            rs2: zero_reg(),
        },
        "andn a1,a0,zero",
        0x400575b3,
    ));
    insns.push(TestUnit::new(
        Inst::AluRRR {
            alu_op: AluOPRRR::Bclr,
            rd: writable_a1(),
            rs1: a0(),
            rs2: zero_reg(),
        },
        "bclr a1,a0,zero",
        0x480515b3,
    ));

    insns.push(TestUnit::new(
        Inst::AluRRR {
            alu_op: AluOPRRR::Bext,
            rd: writable_a1(),
            rs1: a0(),
            rs2: zero_reg(),
        },
        "bext a1,a0,zero",
        0x480555b3,
    ));

    insns.push(TestUnit::new(
        Inst::AluRRR {
            alu_op: AluOPRRR::Binv,
            rd: writable_a1(),
            rs1: a0(),
            rs2: zero_reg(),
        },
        "binv a1,a0,zero",
        0x680515b3,
    ));
    insns.push(TestUnit::new(
        Inst::AluRRR {
            alu_op: AluOPRRR::Bset,
            rd: writable_a1(),
            rs1: a0(),
            rs2: zero_reg(),
        },
        "bset a1,a0,zero",
        0x280515b3,
    ));

    insns.push(TestUnit::new(
        Inst::AluRRR {
            alu_op: AluOPRRR::Clmul,
            rd: writable_a1(),
            rs1: a0(),
            rs2: zero_reg(),
        },
        "clmul a1,a0,zero",
        0xa0515b3,
    ));

    insns.push(TestUnit::new(
        Inst::AluRRR {
            alu_op: AluOPRRR::Clmulh,
            rd: writable_a1(),
            rs1: a0(),
            rs2: zero_reg(),
        },
        "clmulh a1,a0,zero",
        0xa0535b3,
    ));

    insns.push(TestUnit::new(
        Inst::AluRRR {
            alu_op: AluOPRRR::Clmulr,
            rd: writable_a1(),
            rs1: a0(),
            rs2: zero_reg(),
        },
        "clmulr a1,a0,zero",
        0xa0525b3,
    ));

    insns.push(TestUnit::new(
        Inst::AluRRR {
            alu_op: AluOPRRR::Max,
            rd: writable_a1(),
            rs1: a0(),
            rs2: zero_reg(),
        },
        "max a1,a0,zero",
        0xa0565b3,
    ));

    insns.push(TestUnit::new(
        Inst::AluRRR {
            alu_op: AluOPRRR::Maxu,
            rd: writable_a1(),
            rs1: a0(),
            rs2: zero_reg(),
        },
        "maxu a1,a0,zero",
        0xa0575b3,
    ));

    insns.push(TestUnit::new(
        Inst::AluRRR {
            alu_op: AluOPRRR::Min,
            rd: writable_a1(),
            rs1: a0(),
            rs2: zero_reg(),
        },
        "min a1,a0,zero",
        0xa0545b3,
    ));

    insns.push(TestUnit::new(
        Inst::AluRRR {
            alu_op: AluOPRRR::Minu,
            rd: writable_a1(),
            rs1: a0(),
            rs2: zero_reg(),
        },
        "minu a1,a0,zero",
        0xa0555b3,
    ));

    insns.push(TestUnit::new(
        Inst::AluRRR {
            alu_op: AluOPRRR::Orn,
            rd: writable_a1(),
            rs1: a0(),
            rs2: zero_reg(),
        },
        "orn a1,a0,zero",
        0x400565b3,
    ));

    insns.push(TestUnit::new(
        Inst::AluRRR {
            alu_op: AluOPRRR::Rol,
            rd: writable_a1(),
            rs1: a0(),
            rs2: zero_reg(),
        },
        "rol a1,a0,zero",
        0x600515b3,
    ));

    insns.push(TestUnit::new(
        Inst::AluRRR {
            alu_op: AluOPRRR::Rolw,
            rd: writable_a1(),
            rs1: a0(),
            rs2: zero_reg(),
        },
        "rolw a1,a0,zero",
        0x600515bb,
    ));
    insns.push(TestUnit::new(
        Inst::AluRRR {
            alu_op: AluOPRRR::Ror,
            rd: writable_a1(),
            rs1: a0(),
            rs2: zero_reg(),
        },
        "ror a1,a0,zero",
        0x600555b3,
    ));
    insns.push(TestUnit::new(
        Inst::AluRRR {
            alu_op: AluOPRRR::Rorw,
            rd: writable_a1(),
            rs1: a0(),
            rs2: zero_reg(),
        },
        "rorw a1,a0,zero",
        0x600555bb,
    ));
    insns.push(TestUnit::new(
        Inst::AluRRR {
            alu_op: AluOPRRR::Sh1add,
            rd: writable_a1(),
            rs1: a0(),
            rs2: zero_reg(),
        },
        "sh1add a1,a0,zero",
        0x200525b3,
    ));

    insns.push(TestUnit::new(
        Inst::AluRRR {
            alu_op: AluOPRRR::Sh1adduw,
            rd: writable_a1(),
            rs1: a0(),
            rs2: zero_reg(),
        },
        "sh1add.uw a1,a0,zero",
        0x200525bb,
    ));
    insns.push(TestUnit::new(
        Inst::AluRRR {
            alu_op: AluOPRRR::Sh2add,
            rd: writable_a1(),
            rs1: a0(),
            rs2: zero_reg(),
        },
        "sh2add a1,a0,zero",
        0x200545b3,
    ));
    insns.push(TestUnit::new(
        Inst::AluRRR {
            alu_op: AluOPRRR::Sh2adduw,
            rd: writable_a1(),
            rs1: a0(),
            rs2: zero_reg(),
        },
        "sh2add.uw a1,a0,zero",
        0x200545bb,
    ));
    insns.push(TestUnit::new(
        Inst::AluRRR {
            alu_op: AluOPRRR::Sh3add,
            rd: writable_a1(),
            rs1: a0(),
            rs2: zero_reg(),
        },
        "sh3add a1,a0,zero",
        0x200565b3,
    ));
    insns.push(TestUnit::new(
        Inst::AluRRR {
            alu_op: AluOPRRR::Sh3adduw,
            rd: writable_a1(),
            rs1: a0(),
            rs2: zero_reg(),
        },
        "sh3add.uw a1,a0,zero",
        0x200565bb,
    ));
    insns.push(TestUnit::new(
        Inst::AluRRR {
            alu_op: AluOPRRR::Xnor,
            rd: writable_a1(),
            rs1: a0(),
            rs2: zero_reg(),
        },
        "xnor a1,a0,zero",
        0x400545b3,
    ));

    // Zbkb
    insns.push(TestUnit::new(
        Inst::AluRRR {
            alu_op: AluOPRRR::Pack,
            rd: writable_a1(),
            rs1: a0(),
            rs2: zero_reg(),
        },
        "pack a1,a0,zero",
        0x080545b3,
    ));
    insns.push(TestUnit::new(
        Inst::AluRRR {
            alu_op: AluOPRRR::Packw,
            rd: writable_a1(),
            rs1: a0(),
            rs2: zero_reg(),
        },
        "packw a1,a0,zero",
        0x080545bb,
    ));
    insns.push(TestUnit::new(
        Inst::AluRRR {
            alu_op: AluOPRRR::Packh,
            rd: writable_a1(),
            rs1: a0(),
            rs2: zero_reg(),
        },
        "packh a1,a0,zero",
        0x080575b3,
    ));

    //
    insns.push(TestUnit::new(
        Inst::AluRRR {
            alu_op: AluOPRRR::Add,
            rd: writable_fp_reg(),
            rs1: fp_reg(),
            rs2: zero_reg(),
        },
        "add fp,fp,zero",
        0x40433,
    ));
    insns.push(TestUnit::new(
        Inst::AluRRImm12 {
            alu_op: AluOPRRI::Addi,
            rd: writable_fp_reg(),
            rs: stack_reg(),
            imm12: Imm12::maybe_from_u64(100).unwrap(),
        },
        "addi fp,sp,100",
        0x6410413,
    ));
    insns.push(TestUnit::new(
        Inst::Lui {
            rd: writable_zero_reg(),
            imm: Imm20::from_i32(120),
        },
        "lui zero,120",
        0x78037,
    ));
    insns.push(TestUnit::new(
        Inst::Auipc {
            rd: writable_zero_reg(),
            imm: Imm20::from_i32(120),
        },
        "auipc zero,120",
        0x78017,
    ));

    insns.push(TestUnit::new(
        Inst::Jalr {
            rd: writable_a0(),
            base: a0(),
            offset: Imm12::from_i16(100),
        },
        "jalr a0,100(a0)",
        0x6450567,
    ));

    insns.push(TestUnit::new(
        Inst::Load {
            rd: writable_a0(),
            op: LoadOP::Lb,
            flags: MemFlags::new(),
            from: AMode::RegOffset(a1(), 100),
        },
        "lb a0,100(a1)",
        0x6458503,
    ));
    insns.push(TestUnit::new(
        Inst::Load {
            rd: writable_a0(),
            op: LoadOP::Lh,
            flags: MemFlags::new(),
            from: AMode::RegOffset(a1(), 100),
        },
        "lh a0,100(a1)",
        0x6459503,
    ));

    insns.push(TestUnit::new(
        Inst::Load {
            rd: writable_a0(),
            op: LoadOP::Lw,
            flags: MemFlags::new(),
            from: AMode::RegOffset(a1(), 100),
        },
        "lw a0,100(a1)",
        0x645a503,
    ));

    insns.push(TestUnit::new(
        Inst::Load {
            rd: writable_a0(),
            op: LoadOP::Ld,
            flags: MemFlags::new(),
            from: AMode::RegOffset(a1(), 100),
        },
        "ld a0,100(a1)",
        0x645b503,
    ));
    insns.push(TestUnit::new(
        Inst::Load {
            rd: Writable::from_reg(fa0()),
            op: LoadOP::Flw,
            flags: MemFlags::new(),
            from: AMode::RegOffset(a1(), 100),
        },
        "flw fa0,100(a1)",
        0x645a507,
    ));

    insns.push(TestUnit::new(
        Inst::Load {
            rd: Writable::from_reg(fa0()),
            op: LoadOP::Fld,
            flags: MemFlags::new(),
            from: AMode::RegOffset(a1(), 100),
        },
        "fld fa0,100(a1)",
        0x645b507,
    ));
    insns.push(TestUnit::new(
        Inst::Store {
            to: AMode::SPOffset(100),
            op: StoreOP::Sb,
            flags: MemFlags::new(),
            src: a0(),
        },
        "sb a0,100(sp)",
        0x6a10223,
    ));
    insns.push(TestUnit::new(
        Inst::Store {
            to: AMode::SPOffset(100),
            op: StoreOP::Sh,
            flags: MemFlags::new(),
            src: a0(),
        },
        "sh a0,100(sp)",
        0x6a11223,
    ));
    insns.push(TestUnit::new(
        Inst::Store {
            to: AMode::SPOffset(100),
            op: StoreOP::Sw,
            flags: MemFlags::new(),
            src: a0(),
        },
        "sw a0,100(sp)",
        0x6a12223,
    ));
    insns.push(TestUnit::new(
        Inst::Store {
            to: AMode::SPOffset(100),
            op: StoreOP::Sd,
            flags: MemFlags::new(),
            src: a0(),
        },
        "sd a0,100(sp)",
        0x6a13223,
    ));
    insns.push(TestUnit::new(
        Inst::Store {
            to: AMode::SPOffset(100),
            op: StoreOP::Fsw,
            flags: MemFlags::new(),
            src: fa0(),
        },
        "fsw fa0,100(sp)",
        0x6a12227,
    ));
    insns.push(TestUnit::new(
        Inst::Store {
            to: AMode::SPOffset(100),
            op: StoreOP::Fsd,
            flags: MemFlags::new(),
            src: fa0(),
        },
        "fsd fa0,100(sp)",
        0x6a13227,
    ));
    insns.push(TestUnit::new(
        Inst::AluRRImm12 {
            alu_op: AluOPRRI::Addi,
            rd: writable_a0(),
            rs: a0(),
            imm12: Imm12::from_i16(100),
        },
        "addi a0,a0,100",
        0x6450513,
    ));
    insns.push(TestUnit::new(
        Inst::AluRRImm12 {
            alu_op: AluOPRRI::Slti,
            rd: writable_a0(),
            rs: a0(),
            imm12: Imm12::from_i16(100),
        },
        "slti a0,a0,100",
        0x6452513,
    ));
    insns.push(TestUnit::new(
        Inst::AluRRImm12 {
            alu_op: AluOPRRI::SltiU,
            rd: writable_a0(),
            rs: a0(),
            imm12: Imm12::from_i16(100),
        },
        "sltiu a0,a0,100",
        0x6453513,
    ));
    insns.push(TestUnit::new(
        Inst::AluRRImm12 {
            alu_op: AluOPRRI::Xori,
            rd: writable_a0(),
            rs: a0(),
            imm12: Imm12::from_i16(100),
        },
        "xori a0,a0,100",
        0x6454513,
    ));
    insns.push(TestUnit::new(
        Inst::AluRRImm12 {
            alu_op: AluOPRRI::Andi,
            rd: writable_a0(),
            rs: a0(),
            imm12: Imm12::from_i16(100),
        },
        "andi a0,a0,100",
        0x6457513,
    ));
    insns.push(TestUnit::new(
        Inst::AluRRImm12 {
            alu_op: AluOPRRI::Slli,
            rd: writable_a0(),
            rs: a0(),
            imm12: Imm12::from_i16(5),
        },
        "slli a0,a0,5",
        0x551513,
    ));
    insns.push(TestUnit::new(
        Inst::AluRRImm12 {
            alu_op: AluOPRRI::Srli,
            rd: writable_a0(),
            rs: a0(),
            imm12: Imm12::from_i16(5),
        },
        "srli a0,a0,5",
        0x555513,
    ));
    insns.push(TestUnit::new(
        Inst::AluRRImm12 {
            alu_op: AluOPRRI::Srai,
            rd: writable_a0(),
            rs: a0(),
            imm12: Imm12::from_i16(5),
        },
        "srai a0,a0,5",
        0x40555513,
    ));
    insns.push(TestUnit::new(
        Inst::AluRRImm12 {
            alu_op: AluOPRRI::Addiw,
            rd: writable_a0(),
            rs: a0(),
            imm12: Imm12::from_i16(120),
        },
        "addiw a0,a0,120",
        0x785051b,
    ));
    insns.push(TestUnit::new(
        Inst::AluRRImm12 {
            alu_op: AluOPRRI::Slliw,
            rd: writable_a0(),
            rs: a0(),
            imm12: Imm12::from_i16(5),
        },
        "slliw a0,a0,5",
        0x55151b,
    ));
    insns.push(TestUnit::new(
        Inst::AluRRImm12 {
            alu_op: AluOPRRI::SrliW,
            rd: writable_a0(),
            rs: a0(),
            imm12: Imm12::from_i16(5),
        },
        "srliw a0,a0,5",
        0x55551b,
    ));
    insns.push(TestUnit::new(
        Inst::AluRRImm12 {
            alu_op: AluOPRRI::Sraiw,
            rd: writable_a0(),
            rs: a0(),
            imm12: Imm12::from_i16(5),
        },
        "sraiw a0,a0,5",
        0x4055551b,
    ));

    insns.push(TestUnit::new(
        Inst::AluRRImm12 {
            alu_op: AluOPRRI::Sraiw,
            rd: writable_a0(),
            rs: a0(),
            imm12: Imm12::from_i16(5),
        },
        "sraiw a0,a0,5",
        0x4055551b,
    ));
    insns.push(TestUnit::new(
        Inst::AluRRR {
            alu_op: AluOPRRR::Add,
            rd: writable_a0(),
            rs1: a0(),
            rs2: a1(),
        },
        "add a0,a0,a1",
        0xb50533,
    ));
    insns.push(TestUnit::new(
        Inst::AluRRR {
            alu_op: AluOPRRR::Sub,
            rd: writable_a0(),
            rs1: a0(),
            rs2: a1(),
        },
        "sub a0,a0,a1",
        0x40b50533,
    ));
    insns.push(TestUnit::new(
        Inst::AluRRR {
            alu_op: AluOPRRR::Sll,
            rd: writable_a0(),
            rs1: a0(),
            rs2: a1(),
        },
        "sll a0,a0,a1",
        0xb51533,
    ));

    insns.push(TestUnit::new(
        Inst::AluRRR {
            alu_op: AluOPRRR::Slt,
            rd: writable_a0(),
            rs1: a0(),
            rs2: a1(),
        },
        "slt a0,a0,a1",
        0xb52533,
    ));
    insns.push(TestUnit::new(
        Inst::AluRRR {
            alu_op: AluOPRRR::SltU,
            rd: writable_a0(),
            rs1: a0(),
            rs2: a1(),
        },
        "sltu a0,a0,a1",
        0xb53533,
    ));
    insns.push(TestUnit::new(
        Inst::AluRRR {
            alu_op: AluOPRRR::Xor,
            rd: writable_a0(),
            rs1: a0(),
            rs2: a1(),
        },
        "xor a0,a0,a1",
        0xb54533,
    ));
    insns.push(TestUnit::new(
        Inst::AluRRR {
            alu_op: AluOPRRR::Srl,
            rd: writable_a0(),
            rs1: a0(),
            rs2: a1(),
        },
        "srl a0,a0,a1",
        0xb55533,
    ));
    insns.push(TestUnit::new(
        Inst::AluRRR {
            alu_op: AluOPRRR::Sra,
            rd: writable_a0(),
            rs1: a0(),
            rs2: a1(),
        },
        "sra a0,a0,a1",
        0x40b55533,
    ));
    insns.push(TestUnit::new(
        Inst::AluRRR {
            alu_op: AluOPRRR::Or,
            rd: writable_a0(),
            rs1: a0(),
            rs2: a1(),
        },
        "or a0,a0,a1",
        0xb56533,
    ));
    insns.push(TestUnit::new(
        Inst::AluRRR {
            alu_op: AluOPRRR::And,
            rd: writable_a0(),
            rs1: a0(),
            rs2: a1(),
        },
        "and a0,a0,a1",
        0xb57533,
    ));
    insns.push(TestUnit::new(
        Inst::AluRRR {
            alu_op: AluOPRRR::Addw,
            rd: writable_a0(),
            rs1: a0(),
            rs2: a1(),
        },
        "addw a0,a0,a1",
        0xb5053b,
    ));
    insns.push(TestUnit::new(
        Inst::AluRRR {
            alu_op: AluOPRRR::Subw,
            rd: writable_a0(),
            rs1: a0(),
            rs2: a1(),
        },
        "subw a0,a0,a1",
        0x40b5053b,
    ));
    insns.push(TestUnit::new(
        Inst::AluRRR {
            alu_op: AluOPRRR::Sllw,
            rd: writable_a0(),
            rs1: a0(),
            rs2: a1(),
        },
        "sllw a0,a0,a1",
        0xb5153b,
    ));
    insns.push(TestUnit::new(
        Inst::AluRRR {
            alu_op: AluOPRRR::Srlw,
            rd: writable_a0(),
            rs1: a0(),
            rs2: a1(),
        },
        "srlw a0,a0,a1",
        0xb5553b,
    ));
    insns.push(TestUnit::new(
        Inst::AluRRR {
            alu_op: AluOPRRR::Sraw,
            rd: writable_a0(),
            rs1: a0(),
            rs2: a1(),
        },
        "sraw a0,a0,a1",
        0x40b5553b,
    ));

    insns.push(TestUnit::new(
        Inst::AluRRR {
            alu_op: AluOPRRR::Mul,
            rd: writable_a0(),
            rs1: a0(),
            rs2: a1(),
        },
        "mul a0,a0,a1",
        0x2b50533,
    ));

    insns.push(TestUnit::new(
        Inst::AluRRR {
            alu_op: AluOPRRR::Mulh,
            rd: writable_a0(),
            rs1: a0(),
            rs2: a1(),
        },
        "mulh a0,a0,a1",
        0x2b51533,
    ));
    insns.push(TestUnit::new(
        Inst::AluRRR {
            alu_op: AluOPRRR::Mulhsu,
            rd: writable_a0(),
            rs1: a0(),
            rs2: a1(),
        },
        "mulhsu a0,a0,a1",
        0x2b52533,
    ));
    insns.push(TestUnit::new(
        Inst::AluRRR {
            alu_op: AluOPRRR::Mulhu,
            rd: writable_a0(),
            rs1: a0(),
            rs2: a1(),
        },
        "mulhu a0,a0,a1",
        0x2b53533,
    ));
    insns.push(TestUnit::new(
        Inst::AluRRR {
            alu_op: AluOPRRR::Div,
            rd: writable_a0(),
            rs1: a0(),
            rs2: a1(),
        },
        "div a0,a0,a1",
        0x2b54533,
    ));
    insns.push(TestUnit::new(
        Inst::AluRRR {
            alu_op: AluOPRRR::DivU,
            rd: writable_a0(),
            rs1: a0(),
            rs2: a1(),
        },
        "divu a0,a0,a1",
        0x2b55533,
    ));
    insns.push(TestUnit::new(
        Inst::AluRRR {
            alu_op: AluOPRRR::Rem,
            rd: writable_a0(),
            rs1: a0(),
            rs2: a1(),
        },
        "rem a0,a0,a1",
        0x2b56533,
    ));
    insns.push(TestUnit::new(
        Inst::AluRRR {
            alu_op: AluOPRRR::RemU,
            rd: writable_a0(),
            rs1: a0(),
            rs2: a1(),
        },
        "remu a0,a0,a1",
        0x2b57533,
    ));

    insns.push(TestUnit::new(
        Inst::AluRRR {
            alu_op: AluOPRRR::Mulw,
            rd: writable_a0(),
            rs1: a0(),
            rs2: a1(),
        },
        "mulw a0,a0,a1",
        0x2b5053b,
    ));
    insns.push(TestUnit::new(
        Inst::AluRRR {
            alu_op: AluOPRRR::Divw,
            rd: writable_a0(),
            rs1: a0(),
            rs2: a1(),
        },
        "divw a0,a0,a1",
        0x2b5453b,
    ));
    insns.push(TestUnit::new(
        Inst::AluRRR {
            alu_op: AluOPRRR::Remw,
            rd: writable_a0(),
            rs1: a0(),
            rs2: a1(),
        },
        "remw a0,a0,a1",
        0x2b5653b,
    ));
    insns.push(TestUnit::new(
        Inst::AluRRR {
            alu_op: AluOPRRR::Remuw,
            rd: writable_a0(),
            rs1: a0(),
            rs2: a1(),
        },
        "remuw a0,a0,a1",
        0x2b5753b,
    ));

    //
    insns.push(TestUnit::new(
        Inst::FpuRRR {
            frm: FRM::RNE,
            width: FpuOPWidth::S,
            alu_op: FpuOPRRR::Fadd,
            rd: writable_fa0(),
            rs1: fa0(),
            rs2: fa1(),
        },
        "fadd.s fa0,fa0,fa1,rne",
        0xb50553,
    ));
    insns.push(TestUnit::new(
        Inst::FpuRRR {
            frm: FRM::RTZ,
            width: FpuOPWidth::S,
            alu_op: FpuOPRRR::Fsub,
            rd: writable_fa0(),
            rs1: fa0(),
            rs2: fa1(),
        },
        "fsub.s fa0,fa0,fa1,rtz",
        0x8b51553,
    ));
    insns.push(TestUnit::new(
        Inst::FpuRRR {
            frm: FRM::RUP,
            width: FpuOPWidth::S,
            alu_op: FpuOPRRR::Fmul,
            rd: writable_fa0(),
            rs1: fa0(),
            rs2: fa1(),
        },
        "fmul.s fa0,fa0,fa1,rup",
        0x10b53553,
    ));
    insns.push(TestUnit::new(
        Inst::FpuRRR {
            frm: FRM::Fcsr,
            width: FpuOPWidth::S,
            alu_op: FpuOPRRR::Fdiv,
            rd: writable_fa0(),
            rs1: fa0(),
            rs2: fa1(),
        },
        "fdiv.s fa0,fa0,fa1,fcsr",
        0x18b57553,
    ));
    insns.push(TestUnit::new(
        Inst::FpuRRR {
            frm: FRM::RNE,
            width: FpuOPWidth::S,
            alu_op: FpuOPRRR::Fsgnj,
            rd: writable_fa0(),
            rs1: fa0(),
            rs2: fa1(),
        },
        "fsgnj.s fa0,fa0,fa1",
        0x20b50553,
    ));
    insns.push(TestUnit::new(
        Inst::FpuRRR {
            frm: FRM::RTZ,
            width: FpuOPWidth::S,
            alu_op: FpuOPRRR::Fsgnjn,
            rd: writable_fa0(),
            rs1: fa0(),
            rs2: fa1(),
        },
        "fsgnjn.s fa0,fa0,fa1",
        0x20b51553,
    ));

    insns.push(TestUnit::new(
        Inst::FpuRRR {
            frm: FRM::RDN,
            width: FpuOPWidth::S,
            alu_op: FpuOPRRR::Fsgnjx,
            rd: writable_fa0(),
            rs1: fa0(),
            rs2: fa1(),
        },
        "fsgnjx.s fa0,fa0,fa1",
        0x20b52553,
    ));
    insns.push(TestUnit::new(
        Inst::FpuRRR {
            frm: FRM::RNE,
            width: FpuOPWidth::S,
            alu_op: FpuOPRRR::Fmin,
            rd: writable_fa0(),
            rs1: fa0(),
            rs2: fa1(),
        },
        "fmin.s fa0,fa0,fa1",
        0x28b50553,
    ));

    insns.push(TestUnit::new(
        Inst::FpuRRR {
            frm: FRM::RTZ,
            width: FpuOPWidth::S,
            alu_op: FpuOPRRR::Fmax,
            rd: writable_fa0(),
            rs1: fa0(),
            rs2: fa1(),
        },
        "fmax.s fa0,fa0,fa1",
        0x28b51553,
    ));
    insns.push(TestUnit::new(
        Inst::FpuRRR {
            frm: FRM::RDN,
            width: FpuOPWidth::S,
            alu_op: FpuOPRRR::Feq,
            rd: writable_a0(),
            rs1: fa0(),
            rs2: fa1(),
        },
        "feq.s a0,fa0,fa1",
        0xa0b52553,
    ));
    insns.push(TestUnit::new(
        Inst::FpuRRR {
            frm: FRM::RTZ,
            width: FpuOPWidth::S,
            alu_op: FpuOPRRR::Flt,
            rd: writable_a0(),
            rs1: fa0(),
            rs2: fa1(),
        },
        "flt.s a0,fa0,fa1",
        0xa0b51553,
    ));
    insns.push(TestUnit::new(
        Inst::FpuRRR {
            frm: FRM::RNE,
            width: FpuOPWidth::S,
            alu_op: FpuOPRRR::Fle,
            rd: writable_a0(),
            rs1: fa0(),
            rs2: fa1(),
        },
        "fle.s a0,fa0,fa1",
        0xa0b50553,
    ));

    //
    insns.push(TestUnit::new(
        Inst::FpuRRR {
            frm: FRM::Fcsr,
            width: FpuOPWidth::D,
            alu_op: FpuOPRRR::Fadd,
            rd: writable_fa0(),
            rs1: fa0(),
            rs2: fa1(),
        },
        "fadd.d fa0,fa0,fa1,fcsr",
        0x2b57553,
    ));
    insns.push(TestUnit::new(
        Inst::FpuRRR {
            frm: FRM::Fcsr,
            width: FpuOPWidth::D,
            alu_op: FpuOPRRR::Fsub,
            rd: writable_fa0(),
            rs1: fa0(),
            rs2: fa1(),
        },
        "fsub.d fa0,fa0,fa1,fcsr",
        0xab57553,
    ));
    insns.push(TestUnit::new(
        Inst::FpuRRR {
            frm: FRM::Fcsr,
            width: FpuOPWidth::D,
            alu_op: FpuOPRRR::Fmul,
            rd: writable_fa0(),
            rs1: fa0(),
            rs2: fa1(),
        },
        "fmul.d fa0,fa0,fa1,fcsr",
        0x12b57553,
    ));
    insns.push(TestUnit::new(
        Inst::FpuRRR {
            frm: FRM::Fcsr,
            width: FpuOPWidth::D,
            alu_op: FpuOPRRR::Fdiv,
            rd: writable_fa0(),
            rs1: fa0(),
            rs2: fa1(),
        },
        "fdiv.d fa0,fa0,fa1,fcsr",
        0x1ab57553,
    ));
    insns.push(TestUnit::new(
        Inst::FpuRRR {
            frm: FRM::RNE,
            width: FpuOPWidth::D,
            alu_op: FpuOPRRR::Fsgnj,
            rd: writable_fa0(),
            rs1: fa0(),
            rs2: fa1(),
        },
        "fsgnj.d fa0,fa0,fa1",
        0x22b50553,
    ));
    insns.push(TestUnit::new(
        Inst::FpuRRR {
            frm: FRM::RTZ,
            width: FpuOPWidth::D,
            alu_op: FpuOPRRR::Fsgnjn,
            rd: writable_fa0(),
            rs1: fa0(),
            rs2: fa1(),
        },
        "fsgnjn.d fa0,fa0,fa1",
        0x22b51553,
    ));

    insns.push(TestUnit::new(
        Inst::FpuRRR {
            frm: FRM::RDN,
            width: FpuOPWidth::D,
            alu_op: FpuOPRRR::Fsgnjx,
            rd: writable_fa0(),
            rs1: fa0(),
            rs2: fa1(),
        },
        "fsgnjx.d fa0,fa0,fa1",
        0x22b52553,
    ));
    insns.push(TestUnit::new(
        Inst::FpuRRR {
            frm: FRM::RNE,
            width: FpuOPWidth::D,
            alu_op: FpuOPRRR::Fmin,
            rd: writable_fa0(),
            rs1: fa0(),
            rs2: fa1(),
        },
        "fmin.d fa0,fa0,fa1",
        0x2ab50553,
    ));

    insns.push(TestUnit::new(
        Inst::FpuRRR {
            frm: FRM::RTZ,
            width: FpuOPWidth::D,
            alu_op: FpuOPRRR::Fmax,
            rd: writable_fa0(),
            rs1: fa0(),
            rs2: fa1(),
        },
        "fmax.d fa0,fa0,fa1",
        0x2ab51553,
    ));
    insns.push(TestUnit::new(
        Inst::FpuRRR {
            frm: FRM::RDN,
            width: FpuOPWidth::D,
            alu_op: FpuOPRRR::Feq,
            rd: writable_a0(),
            rs1: fa0(),
            rs2: fa1(),
        },
        "feq.d a0,fa0,fa1",
        0xa2b52553,
    ));
    insns.push(TestUnit::new(
        Inst::FpuRRR {
            frm: FRM::RTZ,
            width: FpuOPWidth::D,
            alu_op: FpuOPRRR::Flt,
            rd: writable_a0(),
            rs1: fa0(),
            rs2: fa1(),
        },
        "flt.d a0,fa0,fa1",
        0xa2b51553,
    ));
    insns.push(TestUnit::new(
        Inst::FpuRRR {
            frm: FRM::RNE,
            width: FpuOPWidth::D,
            alu_op: FpuOPRRR::Fle,
            rd: writable_a0(),
            rs1: fa0(),
            rs2: fa1(),
        },
        "fle.d a0,fa0,fa1",
        0xa2b50553,
    ));

    //
    insns.push(TestUnit::new(
        Inst::FpuRR {
            frm: FRM::RNE,
            width: FpuOPWidth::S,
            alu_op: FpuOPRR::Fsqrt,
            rd: writable_fa0(),
            rs: fa1(),
        },
        "fsqrt.s fa0,fa1,rne",
        0x58058553,
    ));
    insns.push(TestUnit::new(
        Inst::FpuRR {
            frm: FRM::Fcsr,
            width: FpuOPWidth::S,
            alu_op: FpuOPRR::FcvtWFmt,
            rd: writable_a0(),
            rs: fa1(),
        },
        "fcvt.w.s a0,fa1,fcsr",
        0xc005f553,
    ));

    insns.push(TestUnit::new(
        Inst::FpuRR {
            frm: FRM::Fcsr,
            width: FpuOPWidth::S,
            alu_op: FpuOPRR::FcvtWuFmt,
            rd: writable_a0(),
            rs: fa1(),
        },
        "fcvt.wu.s a0,fa1,fcsr",
        0xc015f553,
    ));
    insns.push(TestUnit::new(
        Inst::FpuRR {
            frm: FRM::RNE,
            width: FpuOPWidth::S,
            alu_op: FpuOPRR::FmvXFmt,
            rd: writable_a0(),
            rs: fa1(),
        },
        "fmv.x.w a0,fa1",
        0xe0058553,
    ));
    insns.push(TestUnit::new(
        Inst::FpuRR {
            frm: FRM::RTZ,
            width: FpuOPWidth::S,
            alu_op: FpuOPRR::Fclass,
            rd: writable_a0(),
            rs: fa1(),
        },
        "fclass.s a0,fa1",
        0xe0059553,
    ));

    insns.push(TestUnit::new(
        Inst::FpuRR {
            frm: FRM::Fcsr,
            width: FpuOPWidth::S,
            alu_op: FpuOPRR::FcvtFmtW,
            rd: writable_fa0(),
            rs: a0(),
        },
        "fcvt.s.w fa0,a0,fcsr",
        0xd0057553,
    ));
    insns.push(TestUnit::new(
        Inst::FpuRR {
            frm: FRM::Fcsr,
            width: FpuOPWidth::S,
            alu_op: FpuOPRR::FcvtFmtWu,
            rd: writable_fa0(),
            rs: a0(),
        },
        "fcvt.s.wu fa0,a0,fcsr",
        0xd0157553,
    ));

    insns.push(TestUnit::new(
        Inst::FpuRR {
            frm: FRM::RNE,
            width: FpuOPWidth::S,
            alu_op: FpuOPRR::FmvFmtX,
            rd: writable_fa0(),
            rs: a0(),
        },
        "fmv.w.x fa0,a0",
        0xf0050553,
    ));
    insns.push(TestUnit::new(
        Inst::FpuRR {
            frm: FRM::Fcsr,
            width: FpuOPWidth::S,
            alu_op: FpuOPRR::FcvtLFmt,
            rd: writable_a0(),
            rs: fa0(),
        },
        "fcvt.l.s a0,fa0,fcsr",
        0xc0257553,
    ));
    insns.push(TestUnit::new(
        Inst::FpuRR {
            frm: FRM::Fcsr,
            width: FpuOPWidth::S,
            alu_op: FpuOPRR::FcvtLuFmt,
            rd: writable_a0(),
            rs: fa0(),
        },
        "fcvt.lu.s a0,fa0,fcsr",
        0xc0357553,
    ));
    insns.push(TestUnit::new(
        Inst::FpuRR {
            frm: FRM::Fcsr,
            width: FpuOPWidth::S,
            alu_op: FpuOPRR::FcvtFmtL,
            rd: writable_fa0(),
            rs: a0(),
        },
        "fcvt.s.l fa0,a0,fcsr",
        0xd0257553,
    ));
    insns.push(TestUnit::new(
        Inst::FpuRR {
            frm: FRM::Fcsr,
            width: FpuOPWidth::S,
            alu_op: FpuOPRR::FcvtFmtLu,
            rd: writable_fa0(),
            rs: a0(),
        },
        "fcvt.s.lu fa0,a0,fcsr",
        0xd0357553,
    ));

    insns.push(TestUnit::new(
        Inst::FpuRR {
            frm: FRM::Fcsr,
            width: FpuOPWidth::D,
            alu_op: FpuOPRR::Fsqrt,
            rd: writable_fa0(),
            rs: fa1(),
        },
        "fsqrt.d fa0,fa1,fcsr",
        0x5a05f553,
    ));
    insns.push(TestUnit::new(
        Inst::FpuRR {
            frm: FRM::Fcsr,
            width: FpuOPWidth::D,
            alu_op: FpuOPRR::FcvtWFmt,
            rd: writable_a0(),
            rs: fa1(),
        },
        "fcvt.w.d a0,fa1,fcsr",
        0xc205f553,
    ));

    insns.push(TestUnit::new(
        Inst::FpuRR {
            frm: FRM::Fcsr,
            width: FpuOPWidth::D,
            alu_op: FpuOPRR::FcvtWuFmt,
            rd: writable_a0(),
            rs: fa1(),
        },
        "fcvt.wu.d a0,fa1,fcsr",
        0xc215f553,
    ));
    insns.push(TestUnit::new(
        Inst::FpuRR {
            frm: FRM::RNE,
            width: FpuOPWidth::D,
            alu_op: FpuOPRR::FmvXFmt,
            rd: writable_a0(),
            rs: fa1(),
        },
        "fmv.x.d a0,fa1",
        0xe2058553,
    ));
    insns.push(TestUnit::new(
        Inst::FpuRR {
            frm: FRM::RTZ,
            width: FpuOPWidth::D,
            alu_op: FpuOPRR::Fclass,
            rd: writable_a0(),
            rs: fa1(),
        },
        "fclass.d a0,fa1",
        0xe2059553,
    ));

    insns.push(TestUnit::new(
        Inst::FpuRR {
            frm: FRM::Fcsr,
            width: FpuOPWidth::S,
            alu_op: FpuOPRR::FcvtSD,
            rd: writable_fa0(),
            rs: fa0(),
        },
        "fcvt.s.d fa0,fa0,fcsr",
        0x40157553,
    ));
    insns.push(TestUnit::new(
        Inst::FpuRR {
            frm: FRM::RNE,
            width: FpuOPWidth::D,
            alu_op: FpuOPRR::FcvtFmtWu,
            rd: writable_fa0(),
            rs: a0(),
        },
        "fcvt.d.wu fa0,a0,rne",
        0xd2150553,
    ));

    insns.push(TestUnit::new(
        Inst::FpuRR {
            frm: FRM::RNE,
            width: FpuOPWidth::D,
            alu_op: FpuOPRR::FmvFmtX,
            rd: writable_fa0(),
            rs: a0(),
        },
        "fmv.d.x fa0,a0",
        0xf2050553,
    ));
    insns.push(TestUnit::new(
        Inst::FpuRR {
            frm: FRM::Fcsr,
            width: FpuOPWidth::D,
            alu_op: FpuOPRR::FcvtLFmt,
            rd: writable_a0(),
            rs: fa0(),
        },
        "fcvt.l.d a0,fa0,fcsr",
        0xc2257553,
    ));
    insns.push(TestUnit::new(
        Inst::FpuRR {
            frm: FRM::Fcsr,
            width: FpuOPWidth::D,
            alu_op: FpuOPRR::FcvtLuFmt,
            rd: writable_a0(),
            rs: fa0(),
        },
        "fcvt.lu.d a0,fa0,fcsr",
        0xc2357553,
    ));
    insns.push(TestUnit::new(
        Inst::FpuRR {
            frm: FRM::Fcsr,
            width: FpuOPWidth::D,
            alu_op: FpuOPRR::FcvtFmtL,
            rd: writable_fa0(),
            rs: a0(),
        },
        "fcvt.d.l fa0,a0,fcsr",
        0xd2257553,
    ));
    insns.push(TestUnit::new(
        Inst::FpuRR {
            frm: FRM::Fcsr,
            width: FpuOPWidth::D,
            alu_op: FpuOPRR::FcvtFmtLu,
            rd: writable_fa0(),
            rs: a0(),
        },
        "fcvt.d.lu fa0,a0,fcsr",
        0xd2357553,
    ));
    //////////////////////

    insns.push(TestUnit::new(
        Inst::FpuRRRR {
            frm: FRM::RNE,
            width: FpuOPWidth::S,
            alu_op: FpuOPRRRR::Fmadd,
            rd: writable_fa0(),
            rs1: fa0(),
            rs2: fa1(),
            rs3: fa7(),
        },
        "fmadd.s fa0,fa0,fa1,fa7,rne",
        0x88b50543,
    ));
    insns.push(TestUnit::new(
        Inst::FpuRRRR {
            frm: FRM::Fcsr,
            width: FpuOPWidth::S,
            alu_op: FpuOPRRRR::Fmsub,
            rd: writable_fa0(),
            rs1: fa0(),
            rs2: fa1(),
            rs3: fa7(),
        },
        "fmsub.s fa0,fa0,fa1,fa7,fcsr",
        0x88b57547,
    ));
    insns.push(TestUnit::new(
        Inst::FpuRRRR {
            frm: FRM::Fcsr,
            width: FpuOPWidth::S,
            alu_op: FpuOPRRRR::Fnmsub,
            rd: writable_fa0(),
            rs1: fa0(),
            rs2: fa1(),
            rs3: fa7(),
        },
        "fnmsub.s fa0,fa0,fa1,fa7,fcsr",
        0x88b5754b,
    ));
    insns.push(TestUnit::new(
        Inst::FpuRRRR {
            frm: FRM::Fcsr,
            width: FpuOPWidth::S,
            alu_op: FpuOPRRRR::Fnmadd,
            rd: writable_fa0(),
            rs1: fa0(),
            rs2: fa1(),
            rs3: fa7(),
        },
        "fnmadd.s fa0,fa0,fa1,fa7,fcsr",
        0x88b5754f,
    ));

    insns.push(TestUnit::new(
        Inst::FpuRRRR {
            frm: FRM::Fcsr,
            width: FpuOPWidth::D,
            alu_op: FpuOPRRRR::Fmadd,
            rd: writable_fa0(),
            rs1: fa0(),
            rs2: fa1(),
            rs3: fa7(),
        },
        "fmadd.d fa0,fa0,fa1,fa7,fcsr",
        0x8ab57543,
    ));
    insns.push(TestUnit::new(
        Inst::FpuRRRR {
            frm: FRM::Fcsr,
            width: FpuOPWidth::D,
            alu_op: FpuOPRRRR::Fmsub,
            rd: writable_fa0(),
            rs1: fa0(),
            rs2: fa1(),
            rs3: fa7(),
        },
        "fmsub.d fa0,fa0,fa1,fa7,fcsr",
        0x8ab57547,
    ));
    insns.push(TestUnit::new(
        Inst::FpuRRRR {
            frm: FRM::Fcsr,
            width: FpuOPWidth::D,
            alu_op: FpuOPRRRR::Fnmsub,
            rd: writable_fa0(),
            rs1: fa0(),
            rs2: fa1(),
            rs3: fa7(),
        },
        "fnmsub.d fa0,fa0,fa1,fa7,fcsr",
        0x8ab5754b,
    ));
    insns.push(TestUnit::new(
        Inst::FpuRRRR {
            frm: FRM::Fcsr,
            width: FpuOPWidth::D,
            alu_op: FpuOPRRRR::Fnmadd,
            rd: writable_fa0(),
            rs1: fa0(),
            rs2: fa1(),
            rs3: fa7(),
        },
        "fnmadd.d fa0,fa0,fa1,fa7,fcsr",
        0x8ab5754f,
    ));

    insns.push(TestUnit::new(
        Inst::Atomic {
            op: AtomicOP::LrW,
            rd: writable_a0(),
            addr: a1(),
            src: zero_reg(),
            amo: AMO::Relax,
        },
        "lr.w a0,(a1)",
        0x1005a52f,
    ));
    insns.push(TestUnit::new(
        Inst::Atomic {
            op: AtomicOP::ScW,
            rd: writable_a0(),
            addr: a1(),
            src: a2(),
            amo: AMO::Release,
        },
        "sc.w.rl a0,a2,(a1)",
        0x1ac5a52f,
    ));
    insns.push(TestUnit::new(
        Inst::Atomic {
            op: AtomicOP::AmoswapW,
            rd: writable_a0(),
            addr: a1(),
            src: a2(),
            amo: AMO::Acquire,
        },
        "amoswap.w.aq a0,a2,(a1)",
        0xcc5a52f,
    ));

    insns.push(TestUnit::new(
        Inst::Atomic {
            op: AtomicOP::AmoaddW,
            rd: writable_a0(),
            addr: a1(),
            src: a2(),
            amo: AMO::SeqCst,
        },
        "amoadd.w.aqrl a0,a2,(a1)",
        0x6c5a52f,
    ));
    insns.push(TestUnit::new(
        Inst::Atomic {
            op: AtomicOP::AmoxorW,
            rd: writable_a0(),
            addr: a1(),
            src: a2(),
            amo: AMO::Relax,
        },
        "amoxor.w a0,a2,(a1)",
        0x20c5a52f,
    ));
    insns.push(TestUnit::new(
        Inst::Atomic {
            op: AtomicOP::AmoandW,
            rd: writable_a0(),
            addr: a1(),
            src: a2(),
            amo: AMO::Relax,
        },
        "amoand.w a0,a2,(a1)",
        0x60c5a52f,
    ));

    insns.push(TestUnit::new(
        Inst::Atomic {
            op: AtomicOP::AmoorW,
            rd: writable_a0(),
            addr: a1(),
            src: a2(),
            amo: AMO::Relax,
        },
        "amoor.w a0,a2,(a1)",
        0x40c5a52f,
    ));
    insns.push(TestUnit::new(
        Inst::Atomic {
            op: AtomicOP::AmominW,
            rd: writable_a0(),
            addr: a1(),
            src: a2(),
            amo: AMO::Relax,
        },
        "amomin.w a0,a2,(a1)",
        0x80c5a52f,
    ));
    insns.push(TestUnit::new(
        Inst::Atomic {
            op: AtomicOP::AmomaxW,
            rd: writable_a0(),
            addr: a1(),
            src: a2(),
            amo: AMO::Relax,
        },
        "amomax.w a0,a2,(a1)",
        0xa0c5a52f,
    ));
    insns.push(TestUnit::new(
        Inst::Atomic {
            op: AtomicOP::AmominuW,
            rd: writable_a0(),
            addr: a1(),
            src: a2(),
            amo: AMO::Relax,
        },
        "amominu.w a0,a2,(a1)",
        0xc0c5a52f,
    ));
    insns.push(TestUnit::new(
        Inst::Atomic {
            op: AtomicOP::AmomaxuW,
            rd: writable_a0(),
            addr: a1(),
            src: a2(),
            amo: AMO::Relax,
        },
        "amomaxu.w a0,a2,(a1)",
        0xe0c5a52f,
    ));

    insns.push(TestUnit::new(
        Inst::Atomic {
            op: AtomicOP::LrD,
            rd: writable_a0(),
            addr: a1(),
            src: zero_reg(),
            amo: AMO::Relax,
        },
        "lr.d a0,(a1)",
        0x1005b52f,
    ));
    insns.push(TestUnit::new(
        Inst::Atomic {
            op: AtomicOP::ScD,
            rd: writable_a0(),
            addr: a1(),
            src: a2(),
            amo: AMO::Relax,
        },
        "sc.d a0,a2,(a1)",
        0x18c5b52f,
    ));

    insns.push(TestUnit::new(
        Inst::Atomic {
            op: AtomicOP::AmoswapD,
            rd: writable_a0(),
            addr: a1(),
            src: a2(),
            amo: AMO::Relax,
        },
        "amoswap.d a0,a2,(a1)",
        0x8c5b52f,
    ));

    insns.push(TestUnit::new(
        Inst::Atomic {
            op: AtomicOP::AmoaddD,
            rd: writable_a0(),
            addr: a1(),
            src: a2(),
            amo: AMO::Relax,
        },
        "amoadd.d a0,a2,(a1)",
        0xc5b52f,
    ));
    insns.push(TestUnit::new(
        Inst::Atomic {
            op: AtomicOP::AmoxorD,
            rd: writable_a0(),
            addr: a1(),
            src: a2(),
            amo: AMO::Relax,
        },
        "amoxor.d a0,a2,(a1)",
        0x20c5b52f,
    ));
    insns.push(TestUnit::new(
        Inst::Atomic {
            op: AtomicOP::AmoandD,
            rd: writable_a0(),
            addr: a1(),
            src: a2(),
            amo: AMO::Relax,
        },
        "amoand.d a0,a2,(a1)",
        0x60c5b52f,
    ));

    insns.push(TestUnit::new(
        Inst::Atomic {
            op: AtomicOP::AmoorD,
            rd: writable_a0(),
            addr: a1(),
            src: a2(),
            amo: AMO::Relax,
        },
        "amoor.d a0,a2,(a1)",
        0x40c5b52f,
    ));
    insns.push(TestUnit::new(
        Inst::Atomic {
            op: AtomicOP::AmominD,
            rd: writable_a0(),
            addr: a1(),
            src: a2(),
            amo: AMO::Relax,
        },
        "amomin.d a0,a2,(a1)",
        0x80c5b52f,
    ));
    insns.push(TestUnit::new(
        Inst::Atomic {
            op: AtomicOP::AmomaxD,
            rd: writable_a0(),
            addr: a1(),
            src: a2(),
            amo: AMO::Relax,
        },
        "amomax.d a0,a2,(a1)",
        0xa0c5b52f,
    ));
    insns.push(TestUnit::new(
        Inst::Atomic {
            op: AtomicOP::AmominuD,
            rd: writable_a0(),
            addr: a1(),
            src: a2(),
            amo: AMO::Relax,
        },
        "amominu.d a0,a2,(a1)",
        0xc0c5b52f,
    ));
    insns.push(TestUnit::new(
        Inst::Atomic {
            op: AtomicOP::AmomaxuD,
            rd: writable_a0(),
            addr: a1(),
            src: a2(),
            amo: AMO::Relax,
        },
        "amomaxu.d a0,a2,(a1)",
        0xe0c5b52f,
    ));

    /////////
    insns.push(TestUnit::new(
        Inst::Fence {
            pred: 1,
            succ: 1 << 1,
        },
        "fence w,r",
        0x120000f,
    ));
    insns.push(TestUnit::new(Inst::EBreak {}, "ebreak", 0x100073));

    insns.push(TestUnit::new(
        Inst::FpuRRR {
            alu_op: FpuOPRRR::Fsgnj,
            width: FpuOPWidth::S,
            frm: FRM::RNE,
            rd: writable_fa0(),
            rs1: fa1(),
            rs2: fa1(),
        },
        "fmv.s fa0,fa1",
        0x20b58553,
    ));
    insns.push(TestUnit::new(
        Inst::FpuRRR {
            alu_op: FpuOPRRR::Fsgnj,
            width: FpuOPWidth::D,
            frm: FRM::RNE,
            rd: writable_fa0(),
            rs1: fa1(),
            rs2: fa1(),
        },
        "fmv.d fa0,fa1",
        0x22b58553,
    ));

    insns.push(TestUnit::new(
        Inst::FpuRRR {
            alu_op: FpuOPRRR::Fsgnjn,
            width: FpuOPWidth::S,
            frm: FRM::RTZ,
            rd: writable_fa0(),
            rs1: fa1(),
            rs2: fa1(),
        },
        "fneg.s fa0,fa1",
        0x20b59553,
    ));
    insns.push(TestUnit::new(
        Inst::FpuRRR {
            alu_op: FpuOPRRR::Fsgnjn,
            width: FpuOPWidth::D,
            frm: FRM::RTZ,
            rd: writable_fa0(),
            rs1: fa1(),
            rs2: fa1(),
        },
        "fneg.d fa0,fa1",
        0x22b59553,
    ));

    insns.push(TestUnit::new(
        Inst::Fli {
            width: FpuOPWidth::H,
            rd: writable_fa0(),
            imm: FliConstant::new(3),
        },
        "fli.h fa0,2^-15",
        0xf4118553,
    ));

    insns.push(TestUnit::new(
        Inst::Fli {
            width: FpuOPWidth::S,
            rd: writable_fa0(),
            imm: FliConstant::new(0),
        },
        "fli.s fa0,-1.0",
        0xf0100553,
    ));

    insns.push(TestUnit::new(
        Inst::Fli {
            width: FpuOPWidth::D,
            rd: writable_fa0(),
            imm: FliConstant::new(13),
        },
        "fli.d fa0,0.625",
        0xf2168553,
    ));

    let (flags, isa_flags) = make_test_flags();
    let emit_info = EmitInfo::new(flags, isa_flags);

    for unit in insns.iter() {
        println!("Riscv64: {:?}, {}", unit.inst, unit.assembly);
        // Check the printed text is as expected.
        let actual_printing = unit.inst.print_with_state(&mut EmitState::default());
        assert_eq!(unit.assembly, actual_printing);
        let mut buffer = MachBuffer::new();
        unit.inst
            .emit(&mut buffer, &emit_info, &mut Default::default());
        let buffer = buffer.finish(&Default::default(), &mut Default::default());
        let actual_encoding = buffer.stringify_code_bytes();

        assert_eq!(actual_encoding, unit.code.0);
    }
}

fn make_test_flags() -> (settings::Flags, super::super::riscv_settings::Flags) {
    let b = settings::builder();
    let flags = settings::Flags::new(b.clone());
    let b2 = super::super::riscv_settings::builder();
    let isa_flags = super::super::riscv_settings::Flags::new(&flags, &b2);
    (flags, isa_flags)
}

#[test]
fn riscv64_worst_case_instruction_size() {
    let (flags, isa_flags) = make_test_flags();
    let emit_info = EmitInfo::new(flags, isa_flags);

    // These are all candidate instructions with potential to generate a lot of bytes.
    let mut candidates: Vec<MInst> = vec![];

    candidates.push(Inst::Popcnt {
        sum: writable_a0(),
        tmp: writable_a0(),
        step: writable_a0(),
        rs: a0(),
        ty: I64,
    });

    candidates.push(Inst::Cltz {
        sum: writable_a0(),
        tmp: writable_a0(),
        step: writable_a0(),
        rs: a0(),
        leading: true,
        ty: I64,
    });

    candidates.push(Inst::Brev8 {
        rd: writable_a0(),
        tmp: writable_a0(),
        step: writable_a0(),
        tmp2: writable_a0(),
        rs: a0(),
        ty: I64,
    });

    candidates.push(Inst::AtomicCas {
        offset: a0(),
        t0: writable_a0(),
        dst: writable_a0(),
        e: a0(),
        addr: a0(),
        v: a0(),
        ty: I64,
    });

    candidates.push(Inst::AtomicCas {
        offset: a0(),
        t0: writable_a0(),
        dst: writable_a0(),
        e: a0(),
        addr: a0(),
        v: a0(),
        ty: I16,
    });

    candidates.extend(
        crate::ir::AtomicRmwOp::all()
            .iter()
            .map(|op| Inst::AtomicRmwLoop {
                op: *op,
                offset: a0(),
                dst: writable_a1(),
                ty: I16,
                p: a1(),
                x: a2(),
                t0: writable_a0(),
            }),
    );

    // Return Call Indirect and BrTable are the largest instructions possible. However they
    // emit their own island, so we don't account them here.

    let mut max: (u32, MInst) = (0, Inst::Nop0);
    for i in candidates {
        let mut buffer = MachBuffer::new();
        let mut emit_state = Default::default();
        i.emit(&mut buffer, &emit_info, &mut emit_state);
        let buffer = buffer.finish(&Default::default(), &mut Default::default());
        let length = buffer.data().len() as u32;
        if length > max.0 {
            let length = buffer.data().len() as u32;
            max = (length, i.clone());
        }
        println!("insn:{i:?}  length: {length}");
    }
    println!("calculate max size is {} , inst is {:?}", max.0, max.1);
    assert!(max.0 <= Inst::worst_case_size());
}
