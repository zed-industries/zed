/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .section .note.GNU-stack,"",@progbits

  .data

  .globl _libXext_so_tramp_table
  .hidden _libXext_so_tramp_table
  .align 8
_libXext_so_tramp_table:
  .zero 1064

  .text

  .globl _libXext_so_tramp_resolve
  .hidden _libXext_so_tramp_resolve

  .globl _libXext_so_save_regs_and_resolve
  .hidden _libXext_so_save_regs_and_resolve
  .type _libXext_so_save_regs_and_resolve, %function
_libXext_so_save_regs_and_resolve:
  .cfi_startproc

#define PUSH_REG(reg) pushq %reg ; .cfi_adjust_cfa_offset 8; .cfi_rel_offset reg, 0
#define POP_REG(reg) popq %reg ; .cfi_adjust_cfa_offset -8; .cfi_restore reg

#define DEC_STACK(d) subq $d, %rsp; .cfi_adjust_cfa_offset d
#define INC_STACK(d) addq $d, %rsp; .cfi_adjust_cfa_offset -d

#define PUSH_MMX_REG(reg) DEC_STACK(8); movq %reg, (%rsp); .cfi_rel_offset reg, 0
#define POP_MMX_REG(reg) movq (%rsp), %reg; .cfi_restore reg; INC_STACK(8)

#define PUSH_XMM_REG(reg) DEC_STACK(16); movdqa %reg, (%rsp); .cfi_rel_offset reg, 0
#define POP_XMM_REG(reg) movdqa (%rsp), %reg; .cfi_restore reg; INC_STACK(16)

// TODO: cfi_offset/cfi_restore
#define PUSH_YMM_REG(reg) DEC_STACK(32); vmovdqu %reg, (%rsp)
#define POP_YMM_REG(reg) vmovdqu (%rsp), %reg; INC_STACK(32)

// TODO: cfi_offset/cfi_restore
#define PUSH_ZMM_REG(reg) DEC_STACK(64); vmovdqu32 %reg, (%rsp)
#define POP_ZMM_REG(reg) vmovdqu32 (%rsp), %reg; INC_STACK(64)

  // Slow path which calls dlsym, taken only on first call.
  // All registers are stored to handle arbitrary calling conventions
  // (except x87 FPU registers which do not have to be preserved).
  // For Dwarf directives, read https://www.imperialviolet.org/2017/01/18/cfi.html.

  .cfi_def_cfa_offset 8  // Return address

  PUSH_REG(rdi)  // 16
  mov 0x10(%rsp), %rdi
  PUSH_REG(rbx)
  PUSH_REG(rbx)  // 16
  PUSH_REG(rcx)
  PUSH_REG(rdx)  // 16
  PUSH_REG(rbp)
  PUSH_REG(rsi)  // 16
  PUSH_REG(r8)
  PUSH_REG(r9)  // 16
  PUSH_REG(r10)
  PUSH_REG(r11)  // 16
  PUSH_REG(r12)
  PUSH_REG(r13)  // 16
  PUSH_REG(r14)
  PUSH_REG(r15)  // 16

  // Maybe use cpuid instead of macro to detect current vector size...
#ifdef __AVX512F__
  PUSH_ZMM_REG(zmm0)
  PUSH_ZMM_REG(zmm1)
  PUSH_ZMM_REG(zmm2)
  PUSH_ZMM_REG(zmm3)
  PUSH_ZMM_REG(zmm4)
  PUSH_ZMM_REG(zmm5)
  PUSH_ZMM_REG(zmm6)
  PUSH_ZMM_REG(zmm7)
#elif defined __AVX__
  PUSH_YMM_REG(ymm0)
  PUSH_YMM_REG(ymm1)
  PUSH_YMM_REG(ymm2)
  PUSH_YMM_REG(ymm3)
  PUSH_YMM_REG(ymm4)
  PUSH_YMM_REG(ymm5)
  PUSH_YMM_REG(ymm6)
  PUSH_YMM_REG(ymm7)
#elif defined __SSE__
  PUSH_XMM_REG(xmm0)
  PUSH_XMM_REG(xmm1)
  PUSH_XMM_REG(xmm2)
  PUSH_XMM_REG(xmm3)
  PUSH_XMM_REG(xmm4)
  PUSH_XMM_REG(xmm5)
  PUSH_XMM_REG(xmm6)
  PUSH_XMM_REG(xmm7)
#endif

  // MMX registers are not used to pass arguments so we do not save them

  // Stack is just 8-byte aligned but callee will re-align to 16
  call _libXext_so_tramp_resolve

#ifdef __AVX512F__
  POP_ZMM_REG(zmm7)
  POP_ZMM_REG(zmm6)
  POP_ZMM_REG(zmm5)
  POP_ZMM_REG(zmm4)
  POP_ZMM_REG(zmm3)
  POP_ZMM_REG(zmm2)
  POP_ZMM_REG(zmm1)
  POP_ZMM_REG(zmm0)  // 16
#elif defined __AVX__
  POP_YMM_REG(ymm7)
  POP_YMM_REG(ymm6)
  POP_YMM_REG(ymm5)
  POP_YMM_REG(ymm4)
  POP_YMM_REG(ymm3)
  POP_YMM_REG(ymm2)
  POP_YMM_REG(ymm1)
  POP_YMM_REG(ymm0)  // 16
#elif defined __SSE__
  POP_XMM_REG(xmm7)
  POP_XMM_REG(xmm6)
  POP_XMM_REG(xmm5)
  POP_XMM_REG(xmm4)
  POP_XMM_REG(xmm3)
  POP_XMM_REG(xmm2)
  POP_XMM_REG(xmm1)
  POP_XMM_REG(xmm0)  // 16
#endif

  POP_REG(r15)
  POP_REG(r14)  // 16
  POP_REG(r13)
  POP_REG(r12)  // 16
  POP_REG(r11)
  POP_REG(r10)  // 16
  POP_REG(r9)
  POP_REG(r8)  // 16
  POP_REG(rsi)
  POP_REG(rbp)  // 16
  POP_REG(rdx)
  POP_REG(rcx)  // 16
  POP_REG(rbx)
  POP_REG(rbx)  // 16
  POP_REG(rdi)

  ret

  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl DPMSCapable
  .p2align 4
  .type DPMSCapable, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden DPMSCapable
#endif
DPMSCapable:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+0(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+0(%rip)
2:
  pushq $0
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl DPMSDisable
  .p2align 4
  .type DPMSDisable, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden DPMSDisable
#endif
DPMSDisable:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+8(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+8(%rip)
2:
  pushq $1
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl DPMSEnable
  .p2align 4
  .type DPMSEnable, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden DPMSEnable
#endif
DPMSEnable:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+16(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+16(%rip)
2:
  pushq $2
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl DPMSForceLevel
  .p2align 4
  .type DPMSForceLevel, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden DPMSForceLevel
#endif
DPMSForceLevel:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+24(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+24(%rip)
2:
  pushq $3
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl DPMSGetTimeouts
  .p2align 4
  .type DPMSGetTimeouts, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden DPMSGetTimeouts
#endif
DPMSGetTimeouts:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+32(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+32(%rip)
2:
  pushq $4
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl DPMSGetVersion
  .p2align 4
  .type DPMSGetVersion, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden DPMSGetVersion
#endif
DPMSGetVersion:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+40(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+40(%rip)
2:
  pushq $5
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl DPMSInfo
  .p2align 4
  .type DPMSInfo, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden DPMSInfo
#endif
DPMSInfo:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+48(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+48(%rip)
2:
  pushq $6
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl DPMSQueryExtension
  .p2align 4
  .type DPMSQueryExtension, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden DPMSQueryExtension
#endif
DPMSQueryExtension:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+56(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+56(%rip)
2:
  pushq $7
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl DPMSSetTimeouts
  .p2align 4
  .type DPMSSetTimeouts, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden DPMSSetTimeouts
#endif
DPMSSetTimeouts:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+64(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+64(%rip)
2:
  pushq $8
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XGEQueryExtension
  .p2align 4
  .type XGEQueryExtension, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XGEQueryExtension
#endif
XGEQueryExtension:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+72(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+72(%rip)
2:
  pushq $9
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XGEQueryVersion
  .p2align 4
  .type XGEQueryVersion, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XGEQueryVersion
#endif
XGEQueryVersion:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+80(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+80(%rip)
2:
  pushq $10
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XLbxGetEventBase
  .p2align 4
  .type XLbxGetEventBase, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XLbxGetEventBase
#endif
XLbxGetEventBase:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+88(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+88(%rip)
2:
  pushq $11
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XLbxQueryExtension
  .p2align 4
  .type XLbxQueryExtension, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XLbxQueryExtension
#endif
XLbxQueryExtension:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+96(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+96(%rip)
2:
  pushq $12
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XLbxQueryVersion
  .p2align 4
  .type XLbxQueryVersion, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XLbxQueryVersion
#endif
XLbxQueryVersion:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+104(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+104(%rip)
2:
  pushq $13
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XMITMiscGetBugMode
  .p2align 4
  .type XMITMiscGetBugMode, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XMITMiscGetBugMode
#endif
XMITMiscGetBugMode:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+112(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+112(%rip)
2:
  pushq $14
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XMITMiscQueryExtension
  .p2align 4
  .type XMITMiscQueryExtension, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XMITMiscQueryExtension
#endif
XMITMiscQueryExtension:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+120(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+120(%rip)
2:
  pushq $15
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XMITMiscSetBugMode
  .p2align 4
  .type XMITMiscSetBugMode, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XMITMiscSetBugMode
#endif
XMITMiscSetBugMode:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+128(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+128(%rip)
2:
  pushq $16
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XMissingExtension
  .p2align 4
  .type XMissingExtension, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XMissingExtension
#endif
XMissingExtension:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+136(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+136(%rip)
2:
  pushq $17
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XSecurityAllocXauth
  .p2align 4
  .type XSecurityAllocXauth, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSecurityAllocXauth
#endif
XSecurityAllocXauth:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+144(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+144(%rip)
2:
  pushq $18
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XSecurityFreeXauth
  .p2align 4
  .type XSecurityFreeXauth, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSecurityFreeXauth
#endif
XSecurityFreeXauth:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+152(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+152(%rip)
2:
  pushq $19
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XSecurityGenerateAuthorization
  .p2align 4
  .type XSecurityGenerateAuthorization, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSecurityGenerateAuthorization
#endif
XSecurityGenerateAuthorization:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+160(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+160(%rip)
2:
  pushq $20
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XSecurityQueryExtension
  .p2align 4
  .type XSecurityQueryExtension, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSecurityQueryExtension
#endif
XSecurityQueryExtension:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+168(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+168(%rip)
2:
  pushq $21
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XSecurityRevokeAuthorization
  .p2align 4
  .type XSecurityRevokeAuthorization, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSecurityRevokeAuthorization
#endif
XSecurityRevokeAuthorization:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+176(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+176(%rip)
2:
  pushq $22
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XSetExtensionErrorHandler
  .p2align 4
  .type XSetExtensionErrorHandler, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSetExtensionErrorHandler
#endif
XSetExtensionErrorHandler:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+184(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+184(%rip)
2:
  pushq $23
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XShapeCombineMask
  .p2align 4
  .type XShapeCombineMask, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XShapeCombineMask
#endif
XShapeCombineMask:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+192(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+192(%rip)
2:
  pushq $24
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XShapeCombineRectangles
  .p2align 4
  .type XShapeCombineRectangles, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XShapeCombineRectangles
#endif
XShapeCombineRectangles:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+200(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+200(%rip)
2:
  pushq $25
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XShapeCombineRegion
  .p2align 4
  .type XShapeCombineRegion, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XShapeCombineRegion
#endif
XShapeCombineRegion:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+208(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+208(%rip)
2:
  pushq $26
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XShapeCombineShape
  .p2align 4
  .type XShapeCombineShape, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XShapeCombineShape
#endif
XShapeCombineShape:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+216(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+216(%rip)
2:
  pushq $27
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XShapeGetRectangles
  .p2align 4
  .type XShapeGetRectangles, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XShapeGetRectangles
#endif
XShapeGetRectangles:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+224(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+224(%rip)
2:
  pushq $28
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XShapeInputSelected
  .p2align 4
  .type XShapeInputSelected, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XShapeInputSelected
#endif
XShapeInputSelected:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+232(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+232(%rip)
2:
  pushq $29
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XShapeOffsetShape
  .p2align 4
  .type XShapeOffsetShape, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XShapeOffsetShape
#endif
XShapeOffsetShape:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+240(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+240(%rip)
2:
  pushq $30
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XShapeQueryExtension
  .p2align 4
  .type XShapeQueryExtension, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XShapeQueryExtension
#endif
XShapeQueryExtension:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+248(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+248(%rip)
2:
  pushq $31
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XShapeQueryExtents
  .p2align 4
  .type XShapeQueryExtents, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XShapeQueryExtents
#endif
XShapeQueryExtents:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+256(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+256(%rip)
2:
  pushq $32
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XShapeQueryVersion
  .p2align 4
  .type XShapeQueryVersion, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XShapeQueryVersion
#endif
XShapeQueryVersion:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+264(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+264(%rip)
2:
  pushq $33
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XShapeSelectInput
  .p2align 4
  .type XShapeSelectInput, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XShapeSelectInput
#endif
XShapeSelectInput:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+272(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+272(%rip)
2:
  pushq $34
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XShmAttach
  .p2align 4
  .type XShmAttach, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XShmAttach
#endif
XShmAttach:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+280(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+280(%rip)
2:
  pushq $35
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XShmCreateImage
  .p2align 4
  .type XShmCreateImage, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XShmCreateImage
#endif
XShmCreateImage:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+288(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+288(%rip)
2:
  pushq $36
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XShmCreatePixmap
  .p2align 4
  .type XShmCreatePixmap, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XShmCreatePixmap
#endif
XShmCreatePixmap:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+296(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+296(%rip)
2:
  pushq $37
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XShmDetach
  .p2align 4
  .type XShmDetach, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XShmDetach
#endif
XShmDetach:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+304(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+304(%rip)
2:
  pushq $38
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XShmGetEventBase
  .p2align 4
  .type XShmGetEventBase, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XShmGetEventBase
#endif
XShmGetEventBase:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+312(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+312(%rip)
2:
  pushq $39
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XShmGetImage
  .p2align 4
  .type XShmGetImage, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XShmGetImage
#endif
XShmGetImage:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+320(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+320(%rip)
2:
  pushq $40
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XShmPixmapFormat
  .p2align 4
  .type XShmPixmapFormat, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XShmPixmapFormat
#endif
XShmPixmapFormat:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+328(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+328(%rip)
2:
  pushq $41
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XShmPutImage
  .p2align 4
  .type XShmPutImage, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XShmPutImage
#endif
XShmPutImage:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+336(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+336(%rip)
2:
  pushq $42
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XShmQueryExtension
  .p2align 4
  .type XShmQueryExtension, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XShmQueryExtension
#endif
XShmQueryExtension:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+344(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+344(%rip)
2:
  pushq $43
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XShmQueryVersion
  .p2align 4
  .type XShmQueryVersion, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XShmQueryVersion
#endif
XShmQueryVersion:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+352(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+352(%rip)
2:
  pushq $44
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XSyncAwait
  .p2align 4
  .type XSyncAwait, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSyncAwait
#endif
XSyncAwait:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+360(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+360(%rip)
2:
  pushq $45
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XSyncAwaitFence
  .p2align 4
  .type XSyncAwaitFence, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSyncAwaitFence
#endif
XSyncAwaitFence:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+368(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+368(%rip)
2:
  pushq $46
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XSyncChangeAlarm
  .p2align 4
  .type XSyncChangeAlarm, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSyncChangeAlarm
#endif
XSyncChangeAlarm:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+376(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+376(%rip)
2:
  pushq $47
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XSyncChangeCounter
  .p2align 4
  .type XSyncChangeCounter, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSyncChangeCounter
#endif
XSyncChangeCounter:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+384(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+384(%rip)
2:
  pushq $48
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XSyncCreateAlarm
  .p2align 4
  .type XSyncCreateAlarm, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSyncCreateAlarm
#endif
XSyncCreateAlarm:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+392(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+392(%rip)
2:
  pushq $49
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XSyncCreateCounter
  .p2align 4
  .type XSyncCreateCounter, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSyncCreateCounter
#endif
XSyncCreateCounter:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+400(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+400(%rip)
2:
  pushq $50
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XSyncCreateFence
  .p2align 4
  .type XSyncCreateFence, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSyncCreateFence
#endif
XSyncCreateFence:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+408(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+408(%rip)
2:
  pushq $51
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XSyncDestroyAlarm
  .p2align 4
  .type XSyncDestroyAlarm, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSyncDestroyAlarm
#endif
XSyncDestroyAlarm:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+416(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+416(%rip)
2:
  pushq $52
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XSyncDestroyCounter
  .p2align 4
  .type XSyncDestroyCounter, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSyncDestroyCounter
#endif
XSyncDestroyCounter:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+424(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+424(%rip)
2:
  pushq $53
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XSyncDestroyFence
  .p2align 4
  .type XSyncDestroyFence, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSyncDestroyFence
#endif
XSyncDestroyFence:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+432(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+432(%rip)
2:
  pushq $54
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XSyncFreeSystemCounterList
  .p2align 4
  .type XSyncFreeSystemCounterList, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSyncFreeSystemCounterList
#endif
XSyncFreeSystemCounterList:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+440(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+440(%rip)
2:
  pushq $55
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XSyncGetPriority
  .p2align 4
  .type XSyncGetPriority, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSyncGetPriority
#endif
XSyncGetPriority:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+448(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+448(%rip)
2:
  pushq $56
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XSyncInitialize
  .p2align 4
  .type XSyncInitialize, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSyncInitialize
#endif
XSyncInitialize:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+456(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+456(%rip)
2:
  pushq $57
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XSyncIntToValue
  .p2align 4
  .type XSyncIntToValue, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSyncIntToValue
#endif
XSyncIntToValue:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+464(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+464(%rip)
2:
  pushq $58
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XSyncIntsToValue
  .p2align 4
  .type XSyncIntsToValue, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSyncIntsToValue
#endif
XSyncIntsToValue:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+472(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+472(%rip)
2:
  pushq $59
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XSyncListSystemCounters
  .p2align 4
  .type XSyncListSystemCounters, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSyncListSystemCounters
#endif
XSyncListSystemCounters:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+480(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+480(%rip)
2:
  pushq $60
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XSyncMaxValue
  .p2align 4
  .type XSyncMaxValue, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSyncMaxValue
#endif
XSyncMaxValue:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+488(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+488(%rip)
2:
  pushq $61
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XSyncMinValue
  .p2align 4
  .type XSyncMinValue, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSyncMinValue
#endif
XSyncMinValue:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+496(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+496(%rip)
2:
  pushq $62
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XSyncQueryAlarm
  .p2align 4
  .type XSyncQueryAlarm, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSyncQueryAlarm
#endif
XSyncQueryAlarm:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+504(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+504(%rip)
2:
  pushq $63
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XSyncQueryCounter
  .p2align 4
  .type XSyncQueryCounter, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSyncQueryCounter
#endif
XSyncQueryCounter:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+512(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+512(%rip)
2:
  pushq $64
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XSyncQueryExtension
  .p2align 4
  .type XSyncQueryExtension, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSyncQueryExtension
#endif
XSyncQueryExtension:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+520(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+520(%rip)
2:
  pushq $65
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XSyncQueryFence
  .p2align 4
  .type XSyncQueryFence, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSyncQueryFence
#endif
XSyncQueryFence:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+528(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+528(%rip)
2:
  pushq $66
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XSyncResetFence
  .p2align 4
  .type XSyncResetFence, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSyncResetFence
#endif
XSyncResetFence:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+536(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+536(%rip)
2:
  pushq $67
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XSyncSetCounter
  .p2align 4
  .type XSyncSetCounter, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSyncSetCounter
#endif
XSyncSetCounter:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+544(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+544(%rip)
2:
  pushq $68
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XSyncSetPriority
  .p2align 4
  .type XSyncSetPriority, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSyncSetPriority
#endif
XSyncSetPriority:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+552(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+552(%rip)
2:
  pushq $69
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XSyncTriggerFence
  .p2align 4
  .type XSyncTriggerFence, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSyncTriggerFence
#endif
XSyncTriggerFence:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+560(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+560(%rip)
2:
  pushq $70
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XSyncValueAdd
  .p2align 4
  .type XSyncValueAdd, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSyncValueAdd
#endif
XSyncValueAdd:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+568(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+568(%rip)
2:
  pushq $71
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XSyncValueEqual
  .p2align 4
  .type XSyncValueEqual, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSyncValueEqual
#endif
XSyncValueEqual:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+576(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+576(%rip)
2:
  pushq $72
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XSyncValueGreaterOrEqual
  .p2align 4
  .type XSyncValueGreaterOrEqual, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSyncValueGreaterOrEqual
#endif
XSyncValueGreaterOrEqual:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+584(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+584(%rip)
2:
  pushq $73
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XSyncValueGreaterThan
  .p2align 4
  .type XSyncValueGreaterThan, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSyncValueGreaterThan
#endif
XSyncValueGreaterThan:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+592(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+592(%rip)
2:
  pushq $74
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XSyncValueHigh32
  .p2align 4
  .type XSyncValueHigh32, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSyncValueHigh32
#endif
XSyncValueHigh32:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+600(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+600(%rip)
2:
  pushq $75
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XSyncValueIsNegative
  .p2align 4
  .type XSyncValueIsNegative, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSyncValueIsNegative
#endif
XSyncValueIsNegative:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+608(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+608(%rip)
2:
  pushq $76
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XSyncValueIsPositive
  .p2align 4
  .type XSyncValueIsPositive, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSyncValueIsPositive
#endif
XSyncValueIsPositive:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+616(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+616(%rip)
2:
  pushq $77
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XSyncValueIsZero
  .p2align 4
  .type XSyncValueIsZero, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSyncValueIsZero
#endif
XSyncValueIsZero:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+624(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+624(%rip)
2:
  pushq $78
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XSyncValueLessOrEqual
  .p2align 4
  .type XSyncValueLessOrEqual, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSyncValueLessOrEqual
#endif
XSyncValueLessOrEqual:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+632(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+632(%rip)
2:
  pushq $79
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XSyncValueLessThan
  .p2align 4
  .type XSyncValueLessThan, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSyncValueLessThan
#endif
XSyncValueLessThan:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+640(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+640(%rip)
2:
  pushq $80
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XSyncValueLow32
  .p2align 4
  .type XSyncValueLow32, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSyncValueLow32
#endif
XSyncValueLow32:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+648(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+648(%rip)
2:
  pushq $81
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XSyncValueSubtract
  .p2align 4
  .type XSyncValueSubtract, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSyncValueSubtract
#endif
XSyncValueSubtract:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+656(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+656(%rip)
2:
  pushq $82
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XTestFakeInput
  .p2align 4
  .type XTestFakeInput, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XTestFakeInput
#endif
XTestFakeInput:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+664(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+664(%rip)
2:
  pushq $83
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XTestFlush
  .p2align 4
  .type XTestFlush, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XTestFlush
#endif
XTestFlush:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+672(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+672(%rip)
2:
  pushq $84
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XTestGetInput
  .p2align 4
  .type XTestGetInput, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XTestGetInput
#endif
XTestGetInput:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+680(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+680(%rip)
2:
  pushq $85
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XTestMovePointer
  .p2align 4
  .type XTestMovePointer, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XTestMovePointer
#endif
XTestMovePointer:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+688(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+688(%rip)
2:
  pushq $86
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XTestPressButton
  .p2align 4
  .type XTestPressButton, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XTestPressButton
#endif
XTestPressButton:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+696(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+696(%rip)
2:
  pushq $87
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XTestPressKey
  .p2align 4
  .type XTestPressKey, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XTestPressKey
#endif
XTestPressKey:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+704(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+704(%rip)
2:
  pushq $88
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XTestQueryInputSize
  .p2align 4
  .type XTestQueryInputSize, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XTestQueryInputSize
#endif
XTestQueryInputSize:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+712(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+712(%rip)
2:
  pushq $89
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XTestReset
  .p2align 4
  .type XTestReset, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XTestReset
#endif
XTestReset:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+720(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+720(%rip)
2:
  pushq $90
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XTestStopInput
  .p2align 4
  .type XTestStopInput, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XTestStopInput
#endif
XTestStopInput:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+728(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+728(%rip)
2:
  pushq $91
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XagCreateAssociation
  .p2align 4
  .type XagCreateAssociation, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XagCreateAssociation
#endif
XagCreateAssociation:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+736(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+736(%rip)
2:
  pushq $92
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XagCreateEmbeddedApplicationGroup
  .p2align 4
  .type XagCreateEmbeddedApplicationGroup, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XagCreateEmbeddedApplicationGroup
#endif
XagCreateEmbeddedApplicationGroup:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+744(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+744(%rip)
2:
  pushq $93
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XagCreateNonembeddedApplicationGroup
  .p2align 4
  .type XagCreateNonembeddedApplicationGroup, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XagCreateNonembeddedApplicationGroup
#endif
XagCreateNonembeddedApplicationGroup:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+752(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+752(%rip)
2:
  pushq $94
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XagDestroyApplicationGroup
  .p2align 4
  .type XagDestroyApplicationGroup, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XagDestroyApplicationGroup
#endif
XagDestroyApplicationGroup:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+760(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+760(%rip)
2:
  pushq $95
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XagDestroyAssociation
  .p2align 4
  .type XagDestroyAssociation, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XagDestroyAssociation
#endif
XagDestroyAssociation:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+768(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+768(%rip)
2:
  pushq $96
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XagGetApplicationGroupAttributes
  .p2align 4
  .type XagGetApplicationGroupAttributes, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XagGetApplicationGroupAttributes
#endif
XagGetApplicationGroupAttributes:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+776(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+776(%rip)
2:
  pushq $97
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XagQueryApplicationGroup
  .p2align 4
  .type XagQueryApplicationGroup, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XagQueryApplicationGroup
#endif
XagQueryApplicationGroup:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+784(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+784(%rip)
2:
  pushq $98
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XagQueryVersion
  .p2align 4
  .type XagQueryVersion, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XagQueryVersion
#endif
XagQueryVersion:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+792(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+792(%rip)
2:
  pushq $99
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XcupGetReservedColormapEntries
  .p2align 4
  .type XcupGetReservedColormapEntries, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XcupGetReservedColormapEntries
#endif
XcupGetReservedColormapEntries:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+800(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+800(%rip)
2:
  pushq $100
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XcupQueryVersion
  .p2align 4
  .type XcupQueryVersion, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XcupQueryVersion
#endif
XcupQueryVersion:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+808(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+808(%rip)
2:
  pushq $101
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XcupStoreColors
  .p2align 4
  .type XcupStoreColors, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XcupStoreColors
#endif
XcupStoreColors:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+816(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+816(%rip)
2:
  pushq $102
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XdbeAllocateBackBufferName
  .p2align 4
  .type XdbeAllocateBackBufferName, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XdbeAllocateBackBufferName
#endif
XdbeAllocateBackBufferName:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+824(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+824(%rip)
2:
  pushq $103
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XdbeBeginIdiom
  .p2align 4
  .type XdbeBeginIdiom, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XdbeBeginIdiom
#endif
XdbeBeginIdiom:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+832(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+832(%rip)
2:
  pushq $104
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XdbeDeallocateBackBufferName
  .p2align 4
  .type XdbeDeallocateBackBufferName, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XdbeDeallocateBackBufferName
#endif
XdbeDeallocateBackBufferName:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+840(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+840(%rip)
2:
  pushq $105
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XdbeEndIdiom
  .p2align 4
  .type XdbeEndIdiom, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XdbeEndIdiom
#endif
XdbeEndIdiom:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+848(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+848(%rip)
2:
  pushq $106
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XdbeFreeVisualInfo
  .p2align 4
  .type XdbeFreeVisualInfo, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XdbeFreeVisualInfo
#endif
XdbeFreeVisualInfo:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+856(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+856(%rip)
2:
  pushq $107
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XdbeGetBackBufferAttributes
  .p2align 4
  .type XdbeGetBackBufferAttributes, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XdbeGetBackBufferAttributes
#endif
XdbeGetBackBufferAttributes:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+864(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+864(%rip)
2:
  pushq $108
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XdbeGetVisualInfo
  .p2align 4
  .type XdbeGetVisualInfo, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XdbeGetVisualInfo
#endif
XdbeGetVisualInfo:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+872(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+872(%rip)
2:
  pushq $109
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XdbeQueryExtension
  .p2align 4
  .type XdbeQueryExtension, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XdbeQueryExtension
#endif
XdbeQueryExtension:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+880(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+880(%rip)
2:
  pushq $110
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XdbeSwapBuffers
  .p2align 4
  .type XdbeSwapBuffers, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XdbeSwapBuffers
#endif
XdbeSwapBuffers:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+888(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+888(%rip)
2:
  pushq $111
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XeviGetVisualInfo
  .p2align 4
  .type XeviGetVisualInfo, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XeviGetVisualInfo
#endif
XeviGetVisualInfo:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+896(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+896(%rip)
2:
  pushq $112
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XeviQueryExtension
  .p2align 4
  .type XeviQueryExtension, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XeviQueryExtension
#endif
XeviQueryExtension:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+904(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+904(%rip)
2:
  pushq $113
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XeviQueryVersion
  .p2align 4
  .type XeviQueryVersion, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XeviQueryVersion
#endif
XeviQueryVersion:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+912(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+912(%rip)
2:
  pushq $114
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XextAddDisplay
  .p2align 4
  .type XextAddDisplay, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XextAddDisplay
#endif
XextAddDisplay:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+920(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+920(%rip)
2:
  pushq $115
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XextCreateExtension
  .p2align 4
  .type XextCreateExtension, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XextCreateExtension
#endif
XextCreateExtension:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+928(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+928(%rip)
2:
  pushq $116
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XextDestroyExtension
  .p2align 4
  .type XextDestroyExtension, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XextDestroyExtension
#endif
XextDestroyExtension:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+936(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+936(%rip)
2:
  pushq $117
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XextFindDisplay
  .p2align 4
  .type XextFindDisplay, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XextFindDisplay
#endif
XextFindDisplay:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+944(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+944(%rip)
2:
  pushq $118
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XextRemoveDisplay
  .p2align 4
  .type XextRemoveDisplay, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XextRemoveDisplay
#endif
XextRemoveDisplay:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+952(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+952(%rip)
2:
  pushq $119
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XmbufChangeBufferAttributes
  .p2align 4
  .type XmbufChangeBufferAttributes, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XmbufChangeBufferAttributes
#endif
XmbufChangeBufferAttributes:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+960(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+960(%rip)
2:
  pushq $120
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XmbufChangeWindowAttributes
  .p2align 4
  .type XmbufChangeWindowAttributes, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XmbufChangeWindowAttributes
#endif
XmbufChangeWindowAttributes:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+968(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+968(%rip)
2:
  pushq $121
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XmbufClearBufferArea
  .p2align 4
  .type XmbufClearBufferArea, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XmbufClearBufferArea
#endif
XmbufClearBufferArea:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+976(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+976(%rip)
2:
  pushq $122
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XmbufCreateBuffers
  .p2align 4
  .type XmbufCreateBuffers, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XmbufCreateBuffers
#endif
XmbufCreateBuffers:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+984(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+984(%rip)
2:
  pushq $123
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XmbufCreateStereoWindow
  .p2align 4
  .type XmbufCreateStereoWindow, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XmbufCreateStereoWindow
#endif
XmbufCreateStereoWindow:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+992(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+992(%rip)
2:
  pushq $124
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XmbufDestroyBuffers
  .p2align 4
  .type XmbufDestroyBuffers, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XmbufDestroyBuffers
#endif
XmbufDestroyBuffers:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+1000(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+1000(%rip)
2:
  pushq $125
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XmbufDisplayBuffers
  .p2align 4
  .type XmbufDisplayBuffers, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XmbufDisplayBuffers
#endif
XmbufDisplayBuffers:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+1008(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+1008(%rip)
2:
  pushq $126
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XmbufGetBufferAttributes
  .p2align 4
  .type XmbufGetBufferAttributes, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XmbufGetBufferAttributes
#endif
XmbufGetBufferAttributes:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+1016(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+1016(%rip)
2:
  pushq $127
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XmbufGetScreenInfo
  .p2align 4
  .type XmbufGetScreenInfo, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XmbufGetScreenInfo
#endif
XmbufGetScreenInfo:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+1024(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+1024(%rip)
2:
  pushq $128
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XmbufGetVersion
  .p2align 4
  .type XmbufGetVersion, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XmbufGetVersion
#endif
XmbufGetVersion:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+1032(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+1032(%rip)
2:
  pushq $129
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XmbufGetWindowAttributes
  .p2align 4
  .type XmbufGetWindowAttributes, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XmbufGetWindowAttributes
#endif
XmbufGetWindowAttributes:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+1040(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+1040(%rip)
2:
  pushq $130
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl XmbufQueryExtension
  .p2align 4
  .type XmbufQueryExtension, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XmbufQueryExtension
#endif
XmbufQueryExtension:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXext_so_tramp_table+1048(%rip)
  je 2f
1:
  jmp *_libXext_so_tramp_table+1048(%rip)
2:
  pushq $131
  .cfi_adjust_cfa_offset 8
  call _libXext_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

