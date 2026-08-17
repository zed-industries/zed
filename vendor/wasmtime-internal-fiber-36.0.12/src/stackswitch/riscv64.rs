// A WORD OF CAUTION
//
// This entire file basically needs to be kept in sync with itself. It's not
// really possible to modify just one bit of this file without understanding
// all the other bits. Documentation tries to reference various bits here and
// there but try to make sure to read over everything before tweaking things!

use wasmtime_asm_macros::asm_func;

// fn(top_of_stack(rdi): *mut u8)
asm_func!(
    wasmtime_versioned_export_macros::versioned_stringify_ident!(wasmtime_fiber_switch),
    "
      // See https://github.com/rust-lang/rust/issues/80608.
      .attribute arch, \"rv64gc\"

      // We're switching to arbitrary code somewhere else, so pessimistically
      // assume that all callee-save register are clobbered. This means we need
      // to save/restore all of them.
      //
      // Note that this order for saving is important since we use CFI directives
      // below to point to where all the saved registers are.
      sd ra,-0x8(sp)
      sd fp,-0x10(sp)
      sd s1,-0x18(sp)
      sd s2,-0x20(sp)
      sd s3,-0x28(sp)
      sd s4,-0x30(sp)
      sd s5,-0x38(sp)
      sd s6,-0x40(sp)
      sd s7,-0x48(sp)
      sd s8,-0x50(sp)
      sd s9,-0x58(sp)
      sd s10,-0x60(sp)
      sd s11,-0x68(sp)
      fsd fs0,-0x70(sp)
      fsd fs1,-0x78(sp)
      fsd fs2,-0x80(sp)
      fsd fs3,-0x88(sp)
      fsd fs4,-0x90(sp)
      fsd fs5,-0x98(sp)
      fsd fs6,-0xa0(sp)
      fsd fs7,-0xa8(sp)
      fsd fs8,-0xb0(sp)
      fsd fs9,-0xb8(sp)
      fsd fs10,-0xc0(sp)
      fsd fs11,-0xc8(sp)
      addi sp , sp , -0xd0

      ld t0 ,-0x10(a0)
      sd sp ,-0x10(a0)

      // Swap stacks and restore all our callee-saved registers
      mv sp,t0

      fld fs11,0x8(sp)
      fld fs10,0x10(sp)
      fld fs9,0x18(sp)
      fld fs8,0x20(sp)
      fld fs7,0x28(sp)
      fld fs6,0x30(sp)
      fld fs5,0x38(sp)
      fld fs4,0x40(sp)
      fld fs3,0x48(sp)
      fld fs2,0x50(sp)
      fld fs1,0x58(sp)
      fld fs0,0x60(sp)
      ld s11,0x68(sp)
      ld s10,0x70(sp)
      ld s9,0x78(sp)
      ld s8,0x80(sp)
      ld s7,0x88(sp)
      ld s6,0x90(sp)
      ld s5,0x98(sp)
      ld s4,0xa0(sp)
      ld s3,0xa8(sp)
      ld s2,0xb0(sp)
      ld s1,0xb8(sp)
      ld fp,0xc0(sp)
      ld ra,0xc8(sp)
      addi sp , sp , 0xd0
      jr ra
  ",
);

// fn(
//    top_of_stack(a0): *mut u8,
//    entry_point(a1): extern fn(*mut u8, *mut u8),
//    entry_arg0(a2): *mut u8,
// )
#[rustfmt::skip]
asm_func!(
    wasmtime_versioned_export_macros::versioned_stringify_ident!(wasmtime_fiber_init),
    "
      lla t0,{}
      sd t0,-0x18(a0)  // ra,first should be wasmtime_fiber_start.
      sd a0,-0x20(a0)  // fp pointer.
      sd a1,-0x28(a0)  // entry_point will load to s1.
      sd a2,-0x30(a0)  // entry_arg0 will load to s2.

      //
      addi t0,a0,-0xe0
      sd t0,-0x10(a0)
      ret
    ",
    sym super::wasmtime_fiber_start,
);

asm_func!(
    wasmtime_versioned_export_macros::versioned_stringify_ident!(wasmtime_fiber_start),
    "
    .cfi_startproc simple
    .cfi_def_cfa_offset 0


    .cfi_escape 0x0f, /* DW_CFA_def_cfa_expression */ \
      5,             /* the byte length of this expression */ \
      0x52,          /* DW_OP_reg2 (sp) */ \
      0x06,          /* DW_OP_deref */ \
      0x08, 0xd0 ,   /* DW_OP_const1u 0xc8 */ \
      0x22           /* DW_OP_plus */


      .cfi_rel_offset ra,-0x8
      .cfi_rel_offset fp,-0x10
      .cfi_rel_offset s1,-0x18
      .cfi_rel_offset s2,-0x20
      .cfi_rel_offset s3,-0x28
      .cfi_rel_offset s4,-0x30
      .cfi_rel_offset s5,-0x38
      .cfi_rel_offset s6,-0x40
      .cfi_rel_offset s7,-0x48
      .cfi_rel_offset s8,-0x50
      .cfi_rel_offset s9,-0x58
      .cfi_rel_offset s10,-0x60
      .cfi_rel_offset s11,-0x68
      .cfi_rel_offset fs0,-0x70
      .cfi_rel_offset fs1,-0x78
      .cfi_rel_offset fs2,-0x80
      .cfi_rel_offset fs3,-0x88
      .cfi_rel_offset fs4,-0x90
      .cfi_rel_offset fs5,-0x98
      .cfi_rel_offset fs6,-0xa0
      .cfi_rel_offset fs7,-0xa8
      .cfi_rel_offset fs8,-0xb0
      .cfi_rel_offset fs9,-0xb8
      .cfi_rel_offset fs10,-0xc0
      .cfi_rel_offset fs11,-0xc8

      mv a0,s2
      mv a1,fp
      jalr s1
      // .4byte 0 will cause panic.
      // for safety just like x86_64.rs.
      .4byte 0
      .cfi_endproc
  ",
);
