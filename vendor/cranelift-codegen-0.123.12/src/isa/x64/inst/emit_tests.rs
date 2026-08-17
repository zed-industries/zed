//! Tests for the emitter
//!
//! See comments at the top of `fn x64_emit` for advice on how to create reliable test cases.
//!
//! to see stdout: cargo test -- --nocapture
//!
//! for this specific case, as of 24 Aug 2020:
//!
//! cd to the top of your wasmtime tree, then:
//!
//! RUST_BACKTRACE=1 cargo test --features test-programs/test_programs \
//!   --all --exclude wasmtime-wasi-nn \
//!   -- isa::x64::inst::emit_tests::test_x64_emit

use super::*;
use crate::ir::UserExternalNameRef;
use crate::isa::x64;
use crate::isa::x64::lower::isle::generated_code::{Atomic128RmwSeqOp, AtomicRmwSeqOp};
use alloc::vec::Vec;
use cranelift_entity::EntityRef as _;

#[test]
fn test_x64_emit() {
    let rax = regs::rax();
    let rbx = regs::rbx();
    let rcx = regs::rcx();
    let rdx = regs::rdx();
    let rsi = regs::rsi();
    let _rdi = regs::rdi();
    let rsp = regs::rsp();
    let rbp = regs::rbp();
    let r8 = regs::r8();
    let r9 = regs::r9();
    let r10 = regs::r10();
    let r11 = regs::r11();
    let r14 = regs::r14();
    let r15 = regs::r15();

    let xmm0 = regs::xmm0();
    let xmm1 = regs::xmm1();
    let xmm2 = regs::xmm2();
    let xmm3 = regs::xmm3();
    let xmm4 = regs::xmm4();
    let _xmm5 = regs::xmm5();
    let xmm6 = regs::xmm6();
    let xmm7 = regs::xmm7();
    let xmm8 = regs::xmm8();
    let xmm9 = regs::xmm9();
    let xmm10 = regs::xmm10();
    let xmm11 = regs::xmm11();
    let xmm12 = regs::xmm12();
    let _xmm13 = regs::xmm13();
    let _xmm14 = regs::xmm14();
    let _xmm15 = regs::xmm15();

    // And Writable<> versions of the same:
    let w_rax = Writable::<Reg>::from_reg(rax);
    let w_rbx = Writable::<Reg>::from_reg(rbx);
    let w_rcx = Writable::<Reg>::from_reg(rcx);
    let w_rdx = Writable::<Reg>::from_reg(rdx);
    let _w_rsi = Writable::<Reg>::from_reg(rsi);
    let _w_rsp = Writable::<Reg>::from_reg(rsp);
    let _w_rbp = Writable::<Reg>::from_reg(rbp);
    let _w_r8 = Writable::<Reg>::from_reg(r8);
    let _w_r9 = Writable::<Reg>::from_reg(r9);
    let w_r11 = Writable::<Reg>::from_reg(r11);
    let _w_r14 = Writable::<Reg>::from_reg(r14);
    let _w_r15 = Writable::<Reg>::from_reg(r15);

    let _w_xmm0 = Writable::<Reg>::from_reg(xmm0);
    let _w_xmm1 = Writable::<Reg>::from_reg(xmm1);
    let _w_xmm2 = Writable::<Reg>::from_reg(xmm2);
    let _w_xmm3 = Writable::<Reg>::from_reg(xmm3);
    let _w_xmm4 = Writable::<Reg>::from_reg(xmm4);
    let _w_xmm6 = Writable::<Reg>::from_reg(xmm6);
    let _w_xmm7 = Writable::<Reg>::from_reg(xmm7);
    let _w_xmm8 = Writable::<Reg>::from_reg(xmm8);
    let _w_xmm9 = Writable::<Reg>::from_reg(xmm9);
    let _w_xmm10 = Writable::<Reg>::from_reg(xmm10);
    let _w_xmm11 = Writable::<Reg>::from_reg(xmm11);
    let _w_xmm12 = Writable::<Reg>::from_reg(xmm12);

    let mut insns = Vec::<(Inst, &str, &str)>::new();

    // End of test cases for Addr
    // ========================================================

    // ========================================================
    // General tests for each insn.  Don't forget to follow the
    // guidelines commented just prior to `fn x64_emit`.
    //

    // ========================================================
    // CallKnown
    insns.push((
        Inst::call_known(Box::new(CallInfo::empty(
            ExternalName::User(UserExternalNameRef::new(0)),
            CallConv::SystemV,
        ))),
        "E800000000",
        "call    User(userextname0)",
    ));

    // ========================================================
    // CallUnknown
    fn call_unknown(rm: RegMem) -> Inst {
        Inst::call_unknown(Box::new(CallInfo::empty(rm, CallConv::SystemV)))
    }

    insns.push((call_unknown(RegMem::reg(rbp)), "FFD5", "call    *%rbp"));
    insns.push((call_unknown(RegMem::reg(r11)), "41FFD3", "call    *%r11"));
    insns.push((
        call_unknown(RegMem::mem(Amode::imm_reg_reg_shift(
            321,
            Gpr::unwrap_new(rsi),
            Gpr::unwrap_new(rcx),
            3,
        ))),
        "FF94CE41010000",
        "call    *321(%rsi,%rcx,8)",
    ));
    insns.push((
        call_unknown(RegMem::mem(Amode::imm_reg_reg_shift(
            321,
            Gpr::unwrap_new(r10),
            Gpr::unwrap_new(rdx),
            2,
        ))),
        "41FF949241010000",
        "call    *321(%r10,%rdx,4)",
    ));

    // ========================================================
    // LoadExtName
    // N.B.: test harness below sets is_pic.
    insns.push((
        Inst::LoadExtName {
            dst: Writable::from_reg(Gpr::R11),
            name: Box::new(ExternalName::User(UserExternalNameRef::new(0))),
            offset: 0,
            distance: RelocDistance::Far,
        },
        "4C8B1D00000000",
        "load_ext_name userextname0+0, %r11",
    ));
    insns.push((
        Inst::LoadExtName {
            dst: Writable::from_reg(Gpr::R11),
            name: Box::new(ExternalName::User(UserExternalNameRef::new(0))),
            offset: 0x12345678,
            distance: RelocDistance::Far,
        },
        "4C8B1D000000004981C378563412",
        "load_ext_name userextname0+305419896, %r11",
    ));
    insns.push((
        Inst::LoadExtName {
            dst: Writable::from_reg(Gpr::R11),
            name: Box::new(ExternalName::User(UserExternalNameRef::new(0))),
            offset: -0x12345678,
            distance: RelocDistance::Far,
        },
        "4C8B1D000000004981C388A9CBED",
        "load_ext_name userextname0+-305419896, %r11",
    ));

    // ========================================================
    // JmpKnown skipped for now

    // ========================================================
    // JmpCondSymm isn't a real instruction

    // ========================================================
    // JmpCond skipped for now

    // ========================================================
    // JmpCondCompound isn't a real instruction

    // ========================================================
    // Pertaining to atomics.
    // Use `r9` with a 0 offset.
    let am3: SyntheticAmode = Amode::imm_reg(0, r9).into();

    // AtomicRmwSeq
    insns.push((
        Inst::AtomicRmwSeq {
            ty: types::I8,
            op: AtomicRmwSeqOp::Or,
            mem: am3.clone(),
            operand: Gpr::unwrap_new(r10),
            temp: w_r11.map(Gpr::unwrap_new),
            dst_old: w_rax.map(Gpr::unwrap_new),
        },
        "490FB6014989C34D0BDAF0450FB0190F85EFFFFFFF",
        "atomically { 8_bits_at_[%r9] Or= %r10; %rax = old_value_at_[%r9]; %r11, %rflags = trash }",
    ));
    insns.push((
        Inst::AtomicRmwSeq {
            ty: types::I16,
            op: AtomicRmwSeqOp::And,
            mem: am3.clone(),
            operand: Gpr::unwrap_new(r10),
            temp: w_r11.map(Gpr::unwrap_new),
            dst_old: w_rax.map(Gpr::unwrap_new)
        },
        "490FB7014989C34D23DAF066450FB1190F85EEFFFFFF",
        "atomically { 16_bits_at_[%r9] And= %r10; %rax = old_value_at_[%r9]; %r11, %rflags = trash }"
    ));
    insns.push((
        Inst::AtomicRmwSeq {
            ty: types::I32,
            op: AtomicRmwSeqOp::Nand,
            mem: am3.clone(),
            operand: Gpr::unwrap_new(r10),
            temp: w_r11.map(Gpr::unwrap_new),
            dst_old: w_rax.map(Gpr::unwrap_new)
        },
        "418B014989C34D23DA49F7D3F0450FB1190F85ECFFFFFF",
        "atomically { 32_bits_at_[%r9] Nand= %r10; %rax = old_value_at_[%r9]; %r11, %rflags = trash }"
    ));
    insns.push((
        Inst::AtomicRmwSeq {
            ty: types::I32,
            op: AtomicRmwSeqOp::Umin,
            mem: am3.clone(),
            operand: Gpr::unwrap_new(r10),
            temp: w_r11.map(Gpr::unwrap_new),
            dst_old: w_rax.map(Gpr::unwrap_new)
        },
        "418B014989C34539DA4D0F46DAF0450FB1190F85EBFFFFFF",
        "atomically { 32_bits_at_[%r9] Umin= %r10; %rax = old_value_at_[%r9]; %r11, %rflags = trash }"
    ));
    insns.push((
        Inst::AtomicRmwSeq {
            ty: types::I64,
            op: AtomicRmwSeqOp::Smax,
            mem: am3.clone(),
            operand: Gpr::unwrap_new(r10),
            temp: w_r11.map(Gpr::unwrap_new),
            dst_old: w_rax.map(Gpr::unwrap_new)
        },
        "498B014989C34D39DA4D0F4DDAF04D0FB1190F85EBFFFFFF",
        "atomically { 64_bits_at_[%r9] Smax= %r10; %rax = old_value_at_[%r9]; %r11, %rflags = trash }"
    ));

    // Atomic128RmwSeq
    insns.push((
        Inst::Atomic128RmwSeq {
            op: Atomic128RmwSeqOp::Or,
            mem: Box::new(am3.clone()),
            operand_low: Gpr::unwrap_new(r10),
            operand_high: Gpr::unwrap_new(r11),
            temp_low: w_rbx.map(Gpr::unwrap_new),
            temp_high: w_rcx.map(Gpr::unwrap_new),
            dst_old_low: w_rax.map(Gpr::unwrap_new),
            dst_old_high: w_rdx.map(Gpr::unwrap_new),
        },
        "498B01498B51084889C34889D1490BDA490BCBF0490FC7090F85E9FFFFFF",
        "atomically { %rdx:%rax = 0(%r9); %rcx:%rbx = %rdx:%rax Or %r11:%r10; 0(%r9) = %rcx:%rbx }",
    ));
    insns.push((
        Inst::Atomic128RmwSeq {
            op: Atomic128RmwSeqOp::And,
            mem: Box::new(am3.clone()),
            operand_low: Gpr::unwrap_new(r10),
            operand_high: Gpr::unwrap_new(r11),
            temp_low: w_rbx.map(Gpr::unwrap_new),
            temp_high: w_rcx.map(Gpr::unwrap_new),
            dst_old_low: w_rax.map(Gpr::unwrap_new),
            dst_old_high: w_rdx.map(Gpr::unwrap_new),
        },
        "498B01498B51084889C34889D14923DA4923CBF0490FC7090F85E9FFFFFF",
        "atomically { %rdx:%rax = 0(%r9); %rcx:%rbx = %rdx:%rax And %r11:%r10; 0(%r9) = %rcx:%rbx }"
    ));
    insns.push((
        Inst::Atomic128RmwSeq {
            op: Atomic128RmwSeqOp::Umin,
            mem: Box::new(am3.clone()),
            operand_low: Gpr::unwrap_new(r10),
            operand_high: Gpr::unwrap_new(r11),
            temp_low: w_rbx.map(Gpr::unwrap_new),
            temp_high: w_rcx.map(Gpr::unwrap_new),
            dst_old_low: w_rax.map(Gpr::unwrap_new),
            dst_old_high: w_rdx.map(Gpr::unwrap_new),
        },
        "498B01498B51084889C34889D14C39D3491BCB4889D1490F43DA490F43CBF0490FC7090F85DEFFFFFF",
        "atomically { %rdx:%rax = 0(%r9); %rcx:%rbx = %rdx:%rax Umin %r11:%r10; 0(%r9) = %rcx:%rbx }"
    ));
    insns.push((
        Inst::Atomic128RmwSeq {
            op: Atomic128RmwSeqOp::Add,
            mem: Box::new(am3.clone()),
            operand_low: Gpr::unwrap_new(r10),
            operand_high: Gpr::unwrap_new(r11),
            temp_low: w_rbx.map(Gpr::unwrap_new),
            temp_high: w_rcx.map(Gpr::unwrap_new),
            dst_old_low: w_rax.map(Gpr::unwrap_new),
            dst_old_high: w_rdx.map(Gpr::unwrap_new),
        },
        "498B01498B51084889C34889D14903DA4913CBF0490FC7090F85E9FFFFFF",
        "atomically { %rdx:%rax = 0(%r9); %rcx:%rbx = %rdx:%rax Add %r11:%r10; 0(%r9) = %rcx:%rbx }"
    ));
    insns.push((
        Inst::Atomic128XchgSeq {
            mem: am3.clone(),
            operand_low: Gpr::unwrap_new(rbx),
            operand_high: Gpr::unwrap_new(rcx),
            dst_old_low: w_rax.map(Gpr::unwrap_new),
            dst_old_high: w_rdx.map(Gpr::unwrap_new),
        },
        "498B01498B5108F0490FC7090F85F5FFFFFF",
        "atomically { %rdx:%rax = 0(%r9); 0(%r9) = %rcx:%rbx }",
    ));

    // ========================================================
    // Misc instructions.

    insns.push((
        Inst::ElfTlsGetAddr {
            symbol: ExternalName::User(UserExternalNameRef::new(0)),
            dst: WritableGpr::from_writable_reg(w_rax).unwrap(),
        },
        "66488D3D00000000666648E800000000",
        "%rax = elf_tls_get_addr User(userextname0)",
    ));

    insns.push((
        Inst::MachOTlsGetAddr {
            symbol: ExternalName::User(UserExternalNameRef::new(0)),
            dst: WritableGpr::from_writable_reg(w_rax).unwrap(),
        },
        "488B3D00000000FF17",
        "%rax = macho_tls_get_addr User(userextname0)",
    ));

    insns.push((
        Inst::CoffTlsGetAddr {
            symbol: ExternalName::User(UserExternalNameRef::new(0)),
            dst: WritableGpr::from_writable_reg(w_rax).unwrap(),
            tmp: WritableGpr::from_writable_reg(w_rcx).unwrap(),
        },
        "8B050000000065488B0C2558000000488B04C1488D8000000000",
        "%rax = coff_tls_get_addr User(userextname0)",
    ));

    // ========================================================
    // Actually run the tests!
    let ctrl_plane = &mut Default::default();
    let constants = Default::default();
    let mut flag_builder = settings::builder();
    flag_builder.enable("is_pic").unwrap();
    let flags = settings::Flags::new(flag_builder);

    use crate::settings::Configurable;
    let mut isa_flag_builder = x64::settings::builder();
    isa_flag_builder.enable("has_cmpxchg16b").unwrap();
    isa_flag_builder.enable("has_ssse3").unwrap();
    isa_flag_builder.enable("has_sse41").unwrap();
    isa_flag_builder.enable("has_fma").unwrap();
    isa_flag_builder.enable("has_avx").unwrap();
    isa_flag_builder.enable("has_avx512bitalg").unwrap();
    isa_flag_builder.enable("has_avx512dq").unwrap();
    isa_flag_builder.enable("has_avx512f").unwrap();
    isa_flag_builder.enable("has_avx512vbmi").unwrap();
    isa_flag_builder.enable("has_avx512vl").unwrap();
    let isa_flags = x64::settings::Flags::new(&flags, &isa_flag_builder);

    let emit_info = EmitInfo::new(flags, isa_flags);
    for (insn, expected_encoding, expected_printing) in insns {
        // Check the printed text is as expected.
        let actual_printing = insn.pretty_print_inst(&mut Default::default());
        assert_eq!(expected_printing, actual_printing.trim());
        let mut buffer = MachBuffer::new();

        insn.emit(&mut buffer, &emit_info, &mut Default::default());

        // Allow one label just after the instruction (so the offset is 0).
        let label = buffer.get_label();
        buffer.bind_label(label, ctrl_plane);

        let buffer = buffer.finish(&constants, ctrl_plane);
        let actual_encoding = &buffer.stringify_code_bytes();
        assert_eq!(expected_encoding, actual_encoding, "{expected_printing}");
    }
}
