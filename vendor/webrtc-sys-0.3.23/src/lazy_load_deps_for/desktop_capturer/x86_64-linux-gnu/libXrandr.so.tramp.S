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

  .globl _libXrandr_so_tramp_table
  .hidden _libXrandr_so_tramp_table
  .align 8
_libXrandr_so_tramp_table:
  .zero 568

  .text

  .globl _libXrandr_so_tramp_resolve
  .hidden _libXrandr_so_tramp_resolve

  .globl _libXrandr_so_save_regs_and_resolve
  .hidden _libXrandr_so_save_regs_and_resolve
  .type _libXrandr_so_save_regs_and_resolve, %function
_libXrandr_so_save_regs_and_resolve:
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
  call _libXrandr_so_tramp_resolve

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

  .globl XRRAddOutputMode
  .p2align 4
  .type XRRAddOutputMode, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XRRAddOutputMode
#endif
XRRAddOutputMode:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXrandr_so_tramp_table+0(%rip)
  je 2f
1:
  jmp *_libXrandr_so_tramp_table+0(%rip)
2:
  pushq $0
  .cfi_adjust_cfa_offset 8
  call _libXrandr_so_save_regs_and_resolve
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

  .globl XRRAllocGamma
  .p2align 4
  .type XRRAllocGamma, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XRRAllocGamma
#endif
XRRAllocGamma:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXrandr_so_tramp_table+8(%rip)
  je 2f
1:
  jmp *_libXrandr_so_tramp_table+8(%rip)
2:
  pushq $1
  .cfi_adjust_cfa_offset 8
  call _libXrandr_so_save_regs_and_resolve
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

  .globl XRRAllocModeInfo
  .p2align 4
  .type XRRAllocModeInfo, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XRRAllocModeInfo
#endif
XRRAllocModeInfo:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXrandr_so_tramp_table+16(%rip)
  je 2f
1:
  jmp *_libXrandr_so_tramp_table+16(%rip)
2:
  pushq $2
  .cfi_adjust_cfa_offset 8
  call _libXrandr_so_save_regs_and_resolve
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

  .globl XRRAllocateMonitor
  .p2align 4
  .type XRRAllocateMonitor, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XRRAllocateMonitor
#endif
XRRAllocateMonitor:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXrandr_so_tramp_table+24(%rip)
  je 2f
1:
  jmp *_libXrandr_so_tramp_table+24(%rip)
2:
  pushq $3
  .cfi_adjust_cfa_offset 8
  call _libXrandr_so_save_regs_and_resolve
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

  .globl XRRChangeOutputProperty
  .p2align 4
  .type XRRChangeOutputProperty, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XRRChangeOutputProperty
#endif
XRRChangeOutputProperty:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXrandr_so_tramp_table+32(%rip)
  je 2f
1:
  jmp *_libXrandr_so_tramp_table+32(%rip)
2:
  pushq $4
  .cfi_adjust_cfa_offset 8
  call _libXrandr_so_save_regs_and_resolve
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

  .globl XRRChangeProviderProperty
  .p2align 4
  .type XRRChangeProviderProperty, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XRRChangeProviderProperty
#endif
XRRChangeProviderProperty:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXrandr_so_tramp_table+40(%rip)
  je 2f
1:
  jmp *_libXrandr_so_tramp_table+40(%rip)
2:
  pushq $5
  .cfi_adjust_cfa_offset 8
  call _libXrandr_so_save_regs_and_resolve
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

  .globl XRRConfigCurrentConfiguration
  .p2align 4
  .type XRRConfigCurrentConfiguration, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XRRConfigCurrentConfiguration
#endif
XRRConfigCurrentConfiguration:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXrandr_so_tramp_table+48(%rip)
  je 2f
1:
  jmp *_libXrandr_so_tramp_table+48(%rip)
2:
  pushq $6
  .cfi_adjust_cfa_offset 8
  call _libXrandr_so_save_regs_and_resolve
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

  .globl XRRConfigCurrentRate
  .p2align 4
  .type XRRConfigCurrentRate, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XRRConfigCurrentRate
#endif
XRRConfigCurrentRate:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXrandr_so_tramp_table+56(%rip)
  je 2f
1:
  jmp *_libXrandr_so_tramp_table+56(%rip)
2:
  pushq $7
  .cfi_adjust_cfa_offset 8
  call _libXrandr_so_save_regs_and_resolve
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

  .globl XRRConfigRates
  .p2align 4
  .type XRRConfigRates, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XRRConfigRates
#endif
XRRConfigRates:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXrandr_so_tramp_table+64(%rip)
  je 2f
1:
  jmp *_libXrandr_so_tramp_table+64(%rip)
2:
  pushq $8
  .cfi_adjust_cfa_offset 8
  call _libXrandr_so_save_regs_and_resolve
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

  .globl XRRConfigRotations
  .p2align 4
  .type XRRConfigRotations, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XRRConfigRotations
#endif
XRRConfigRotations:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXrandr_so_tramp_table+72(%rip)
  je 2f
1:
  jmp *_libXrandr_so_tramp_table+72(%rip)
2:
  pushq $9
  .cfi_adjust_cfa_offset 8
  call _libXrandr_so_save_regs_and_resolve
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

  .globl XRRConfigSizes
  .p2align 4
  .type XRRConfigSizes, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XRRConfigSizes
#endif
XRRConfigSizes:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXrandr_so_tramp_table+80(%rip)
  je 2f
1:
  jmp *_libXrandr_so_tramp_table+80(%rip)
2:
  pushq $10
  .cfi_adjust_cfa_offset 8
  call _libXrandr_so_save_regs_and_resolve
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

  .globl XRRConfigTimes
  .p2align 4
  .type XRRConfigTimes, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XRRConfigTimes
#endif
XRRConfigTimes:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXrandr_so_tramp_table+88(%rip)
  je 2f
1:
  jmp *_libXrandr_so_tramp_table+88(%rip)
2:
  pushq $11
  .cfi_adjust_cfa_offset 8
  call _libXrandr_so_save_regs_and_resolve
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

  .globl XRRConfigureOutputProperty
  .p2align 4
  .type XRRConfigureOutputProperty, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XRRConfigureOutputProperty
#endif
XRRConfigureOutputProperty:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXrandr_so_tramp_table+96(%rip)
  je 2f
1:
  jmp *_libXrandr_so_tramp_table+96(%rip)
2:
  pushq $12
  .cfi_adjust_cfa_offset 8
  call _libXrandr_so_save_regs_and_resolve
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

  .globl XRRConfigureProviderProperty
  .p2align 4
  .type XRRConfigureProviderProperty, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XRRConfigureProviderProperty
#endif
XRRConfigureProviderProperty:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXrandr_so_tramp_table+104(%rip)
  je 2f
1:
  jmp *_libXrandr_so_tramp_table+104(%rip)
2:
  pushq $13
  .cfi_adjust_cfa_offset 8
  call _libXrandr_so_save_regs_and_resolve
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

  .globl XRRCreateMode
  .p2align 4
  .type XRRCreateMode, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XRRCreateMode
#endif
XRRCreateMode:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXrandr_so_tramp_table+112(%rip)
  je 2f
1:
  jmp *_libXrandr_so_tramp_table+112(%rip)
2:
  pushq $14
  .cfi_adjust_cfa_offset 8
  call _libXrandr_so_save_regs_and_resolve
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

  .globl XRRDeleteMonitor
  .p2align 4
  .type XRRDeleteMonitor, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XRRDeleteMonitor
#endif
XRRDeleteMonitor:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXrandr_so_tramp_table+120(%rip)
  je 2f
1:
  jmp *_libXrandr_so_tramp_table+120(%rip)
2:
  pushq $15
  .cfi_adjust_cfa_offset 8
  call _libXrandr_so_save_regs_and_resolve
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

  .globl XRRDeleteOutputMode
  .p2align 4
  .type XRRDeleteOutputMode, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XRRDeleteOutputMode
#endif
XRRDeleteOutputMode:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXrandr_so_tramp_table+128(%rip)
  je 2f
1:
  jmp *_libXrandr_so_tramp_table+128(%rip)
2:
  pushq $16
  .cfi_adjust_cfa_offset 8
  call _libXrandr_so_save_regs_and_resolve
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

  .globl XRRDeleteOutputProperty
  .p2align 4
  .type XRRDeleteOutputProperty, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XRRDeleteOutputProperty
#endif
XRRDeleteOutputProperty:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXrandr_so_tramp_table+136(%rip)
  je 2f
1:
  jmp *_libXrandr_so_tramp_table+136(%rip)
2:
  pushq $17
  .cfi_adjust_cfa_offset 8
  call _libXrandr_so_save_regs_and_resolve
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

  .globl XRRDeleteProviderProperty
  .p2align 4
  .type XRRDeleteProviderProperty, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XRRDeleteProviderProperty
#endif
XRRDeleteProviderProperty:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXrandr_so_tramp_table+144(%rip)
  je 2f
1:
  jmp *_libXrandr_so_tramp_table+144(%rip)
2:
  pushq $18
  .cfi_adjust_cfa_offset 8
  call _libXrandr_so_save_regs_and_resolve
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

  .globl XRRDestroyMode
  .p2align 4
  .type XRRDestroyMode, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XRRDestroyMode
#endif
XRRDestroyMode:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXrandr_so_tramp_table+152(%rip)
  je 2f
1:
  jmp *_libXrandr_so_tramp_table+152(%rip)
2:
  pushq $19
  .cfi_adjust_cfa_offset 8
  call _libXrandr_so_save_regs_and_resolve
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

  .globl XRRFreeCrtcInfo
  .p2align 4
  .type XRRFreeCrtcInfo, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XRRFreeCrtcInfo
#endif
XRRFreeCrtcInfo:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXrandr_so_tramp_table+160(%rip)
  je 2f
1:
  jmp *_libXrandr_so_tramp_table+160(%rip)
2:
  pushq $20
  .cfi_adjust_cfa_offset 8
  call _libXrandr_so_save_regs_and_resolve
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

  .globl XRRFreeGamma
  .p2align 4
  .type XRRFreeGamma, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XRRFreeGamma
#endif
XRRFreeGamma:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXrandr_so_tramp_table+168(%rip)
  je 2f
1:
  jmp *_libXrandr_so_tramp_table+168(%rip)
2:
  pushq $21
  .cfi_adjust_cfa_offset 8
  call _libXrandr_so_save_regs_and_resolve
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

  .globl XRRFreeModeInfo
  .p2align 4
  .type XRRFreeModeInfo, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XRRFreeModeInfo
#endif
XRRFreeModeInfo:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXrandr_so_tramp_table+176(%rip)
  je 2f
1:
  jmp *_libXrandr_so_tramp_table+176(%rip)
2:
  pushq $22
  .cfi_adjust_cfa_offset 8
  call _libXrandr_so_save_regs_and_resolve
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

  .globl XRRFreeMonitors
  .p2align 4
  .type XRRFreeMonitors, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XRRFreeMonitors
#endif
XRRFreeMonitors:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXrandr_so_tramp_table+184(%rip)
  je 2f
1:
  jmp *_libXrandr_so_tramp_table+184(%rip)
2:
  pushq $23
  .cfi_adjust_cfa_offset 8
  call _libXrandr_so_save_regs_and_resolve
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

  .globl XRRFreeOutputInfo
  .p2align 4
  .type XRRFreeOutputInfo, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XRRFreeOutputInfo
#endif
XRRFreeOutputInfo:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXrandr_so_tramp_table+192(%rip)
  je 2f
1:
  jmp *_libXrandr_so_tramp_table+192(%rip)
2:
  pushq $24
  .cfi_adjust_cfa_offset 8
  call _libXrandr_so_save_regs_and_resolve
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

  .globl XRRFreePanning
  .p2align 4
  .type XRRFreePanning, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XRRFreePanning
#endif
XRRFreePanning:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXrandr_so_tramp_table+200(%rip)
  je 2f
1:
  jmp *_libXrandr_so_tramp_table+200(%rip)
2:
  pushq $25
  .cfi_adjust_cfa_offset 8
  call _libXrandr_so_save_regs_and_resolve
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

  .globl XRRFreeProviderInfo
  .p2align 4
  .type XRRFreeProviderInfo, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XRRFreeProviderInfo
#endif
XRRFreeProviderInfo:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXrandr_so_tramp_table+208(%rip)
  je 2f
1:
  jmp *_libXrandr_so_tramp_table+208(%rip)
2:
  pushq $26
  .cfi_adjust_cfa_offset 8
  call _libXrandr_so_save_regs_and_resolve
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

  .globl XRRFreeProviderResources
  .p2align 4
  .type XRRFreeProviderResources, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XRRFreeProviderResources
#endif
XRRFreeProviderResources:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXrandr_so_tramp_table+216(%rip)
  je 2f
1:
  jmp *_libXrandr_so_tramp_table+216(%rip)
2:
  pushq $27
  .cfi_adjust_cfa_offset 8
  call _libXrandr_so_save_regs_and_resolve
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

  .globl XRRFreeScreenConfigInfo
  .p2align 4
  .type XRRFreeScreenConfigInfo, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XRRFreeScreenConfigInfo
#endif
XRRFreeScreenConfigInfo:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXrandr_so_tramp_table+224(%rip)
  je 2f
1:
  jmp *_libXrandr_so_tramp_table+224(%rip)
2:
  pushq $28
  .cfi_adjust_cfa_offset 8
  call _libXrandr_so_save_regs_and_resolve
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

  .globl XRRFreeScreenResources
  .p2align 4
  .type XRRFreeScreenResources, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XRRFreeScreenResources
#endif
XRRFreeScreenResources:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXrandr_so_tramp_table+232(%rip)
  je 2f
1:
  jmp *_libXrandr_so_tramp_table+232(%rip)
2:
  pushq $29
  .cfi_adjust_cfa_offset 8
  call _libXrandr_so_save_regs_and_resolve
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

  .globl XRRGetCrtcGamma
  .p2align 4
  .type XRRGetCrtcGamma, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XRRGetCrtcGamma
#endif
XRRGetCrtcGamma:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXrandr_so_tramp_table+240(%rip)
  je 2f
1:
  jmp *_libXrandr_so_tramp_table+240(%rip)
2:
  pushq $30
  .cfi_adjust_cfa_offset 8
  call _libXrandr_so_save_regs_and_resolve
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

  .globl XRRGetCrtcGammaSize
  .p2align 4
  .type XRRGetCrtcGammaSize, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XRRGetCrtcGammaSize
#endif
XRRGetCrtcGammaSize:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXrandr_so_tramp_table+248(%rip)
  je 2f
1:
  jmp *_libXrandr_so_tramp_table+248(%rip)
2:
  pushq $31
  .cfi_adjust_cfa_offset 8
  call _libXrandr_so_save_regs_and_resolve
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

  .globl XRRGetCrtcInfo
  .p2align 4
  .type XRRGetCrtcInfo, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XRRGetCrtcInfo
#endif
XRRGetCrtcInfo:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXrandr_so_tramp_table+256(%rip)
  je 2f
1:
  jmp *_libXrandr_so_tramp_table+256(%rip)
2:
  pushq $32
  .cfi_adjust_cfa_offset 8
  call _libXrandr_so_save_regs_and_resolve
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

  .globl XRRGetCrtcTransform
  .p2align 4
  .type XRRGetCrtcTransform, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XRRGetCrtcTransform
#endif
XRRGetCrtcTransform:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXrandr_so_tramp_table+264(%rip)
  je 2f
1:
  jmp *_libXrandr_so_tramp_table+264(%rip)
2:
  pushq $33
  .cfi_adjust_cfa_offset 8
  call _libXrandr_so_save_regs_and_resolve
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

  .globl XRRGetMonitors
  .p2align 4
  .type XRRGetMonitors, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XRRGetMonitors
#endif
XRRGetMonitors:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXrandr_so_tramp_table+272(%rip)
  je 2f
1:
  jmp *_libXrandr_so_tramp_table+272(%rip)
2:
  pushq $34
  .cfi_adjust_cfa_offset 8
  call _libXrandr_so_save_regs_and_resolve
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

  .globl XRRGetOutputInfo
  .p2align 4
  .type XRRGetOutputInfo, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XRRGetOutputInfo
#endif
XRRGetOutputInfo:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXrandr_so_tramp_table+280(%rip)
  je 2f
1:
  jmp *_libXrandr_so_tramp_table+280(%rip)
2:
  pushq $35
  .cfi_adjust_cfa_offset 8
  call _libXrandr_so_save_regs_and_resolve
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

  .globl XRRGetOutputPrimary
  .p2align 4
  .type XRRGetOutputPrimary, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XRRGetOutputPrimary
#endif
XRRGetOutputPrimary:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXrandr_so_tramp_table+288(%rip)
  je 2f
1:
  jmp *_libXrandr_so_tramp_table+288(%rip)
2:
  pushq $36
  .cfi_adjust_cfa_offset 8
  call _libXrandr_so_save_regs_and_resolve
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

  .globl XRRGetOutputProperty
  .p2align 4
  .type XRRGetOutputProperty, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XRRGetOutputProperty
#endif
XRRGetOutputProperty:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXrandr_so_tramp_table+296(%rip)
  je 2f
1:
  jmp *_libXrandr_so_tramp_table+296(%rip)
2:
  pushq $37
  .cfi_adjust_cfa_offset 8
  call _libXrandr_so_save_regs_and_resolve
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

  .globl XRRGetPanning
  .p2align 4
  .type XRRGetPanning, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XRRGetPanning
#endif
XRRGetPanning:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXrandr_so_tramp_table+304(%rip)
  je 2f
1:
  jmp *_libXrandr_so_tramp_table+304(%rip)
2:
  pushq $38
  .cfi_adjust_cfa_offset 8
  call _libXrandr_so_save_regs_and_resolve
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

  .globl XRRGetProviderInfo
  .p2align 4
  .type XRRGetProviderInfo, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XRRGetProviderInfo
#endif
XRRGetProviderInfo:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXrandr_so_tramp_table+312(%rip)
  je 2f
1:
  jmp *_libXrandr_so_tramp_table+312(%rip)
2:
  pushq $39
  .cfi_adjust_cfa_offset 8
  call _libXrandr_so_save_regs_and_resolve
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

  .globl XRRGetProviderProperty
  .p2align 4
  .type XRRGetProviderProperty, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XRRGetProviderProperty
#endif
XRRGetProviderProperty:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXrandr_so_tramp_table+320(%rip)
  je 2f
1:
  jmp *_libXrandr_so_tramp_table+320(%rip)
2:
  pushq $40
  .cfi_adjust_cfa_offset 8
  call _libXrandr_so_save_regs_and_resolve
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

  .globl XRRGetProviderResources
  .p2align 4
  .type XRRGetProviderResources, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XRRGetProviderResources
#endif
XRRGetProviderResources:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXrandr_so_tramp_table+328(%rip)
  je 2f
1:
  jmp *_libXrandr_so_tramp_table+328(%rip)
2:
  pushq $41
  .cfi_adjust_cfa_offset 8
  call _libXrandr_so_save_regs_and_resolve
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

  .globl XRRGetScreenInfo
  .p2align 4
  .type XRRGetScreenInfo, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XRRGetScreenInfo
#endif
XRRGetScreenInfo:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXrandr_so_tramp_table+336(%rip)
  je 2f
1:
  jmp *_libXrandr_so_tramp_table+336(%rip)
2:
  pushq $42
  .cfi_adjust_cfa_offset 8
  call _libXrandr_so_save_regs_and_resolve
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

  .globl XRRGetScreenResources
  .p2align 4
  .type XRRGetScreenResources, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XRRGetScreenResources
#endif
XRRGetScreenResources:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXrandr_so_tramp_table+344(%rip)
  je 2f
1:
  jmp *_libXrandr_so_tramp_table+344(%rip)
2:
  pushq $43
  .cfi_adjust_cfa_offset 8
  call _libXrandr_so_save_regs_and_resolve
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

  .globl XRRGetScreenResourcesCurrent
  .p2align 4
  .type XRRGetScreenResourcesCurrent, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XRRGetScreenResourcesCurrent
#endif
XRRGetScreenResourcesCurrent:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXrandr_so_tramp_table+352(%rip)
  je 2f
1:
  jmp *_libXrandr_so_tramp_table+352(%rip)
2:
  pushq $44
  .cfi_adjust_cfa_offset 8
  call _libXrandr_so_save_regs_and_resolve
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

  .globl XRRGetScreenSizeRange
  .p2align 4
  .type XRRGetScreenSizeRange, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XRRGetScreenSizeRange
#endif
XRRGetScreenSizeRange:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXrandr_so_tramp_table+360(%rip)
  je 2f
1:
  jmp *_libXrandr_so_tramp_table+360(%rip)
2:
  pushq $45
  .cfi_adjust_cfa_offset 8
  call _libXrandr_so_save_regs_and_resolve
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

  .globl XRRListOutputProperties
  .p2align 4
  .type XRRListOutputProperties, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XRRListOutputProperties
#endif
XRRListOutputProperties:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXrandr_so_tramp_table+368(%rip)
  je 2f
1:
  jmp *_libXrandr_so_tramp_table+368(%rip)
2:
  pushq $46
  .cfi_adjust_cfa_offset 8
  call _libXrandr_so_save_regs_and_resolve
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

  .globl XRRListProviderProperties
  .p2align 4
  .type XRRListProviderProperties, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XRRListProviderProperties
#endif
XRRListProviderProperties:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXrandr_so_tramp_table+376(%rip)
  je 2f
1:
  jmp *_libXrandr_so_tramp_table+376(%rip)
2:
  pushq $47
  .cfi_adjust_cfa_offset 8
  call _libXrandr_so_save_regs_and_resolve
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

  .globl XRRQueryExtension
  .p2align 4
  .type XRRQueryExtension, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XRRQueryExtension
#endif
XRRQueryExtension:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXrandr_so_tramp_table+384(%rip)
  je 2f
1:
  jmp *_libXrandr_so_tramp_table+384(%rip)
2:
  pushq $48
  .cfi_adjust_cfa_offset 8
  call _libXrandr_so_save_regs_and_resolve
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

  .globl XRRQueryOutputProperty
  .p2align 4
  .type XRRQueryOutputProperty, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XRRQueryOutputProperty
#endif
XRRQueryOutputProperty:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXrandr_so_tramp_table+392(%rip)
  je 2f
1:
  jmp *_libXrandr_so_tramp_table+392(%rip)
2:
  pushq $49
  .cfi_adjust_cfa_offset 8
  call _libXrandr_so_save_regs_and_resolve
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

  .globl XRRQueryProviderProperty
  .p2align 4
  .type XRRQueryProviderProperty, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XRRQueryProviderProperty
#endif
XRRQueryProviderProperty:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXrandr_so_tramp_table+400(%rip)
  je 2f
1:
  jmp *_libXrandr_so_tramp_table+400(%rip)
2:
  pushq $50
  .cfi_adjust_cfa_offset 8
  call _libXrandr_so_save_regs_and_resolve
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

  .globl XRRQueryVersion
  .p2align 4
  .type XRRQueryVersion, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XRRQueryVersion
#endif
XRRQueryVersion:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXrandr_so_tramp_table+408(%rip)
  je 2f
1:
  jmp *_libXrandr_so_tramp_table+408(%rip)
2:
  pushq $51
  .cfi_adjust_cfa_offset 8
  call _libXrandr_so_save_regs_and_resolve
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

  .globl XRRRates
  .p2align 4
  .type XRRRates, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XRRRates
#endif
XRRRates:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXrandr_so_tramp_table+416(%rip)
  je 2f
1:
  jmp *_libXrandr_so_tramp_table+416(%rip)
2:
  pushq $52
  .cfi_adjust_cfa_offset 8
  call _libXrandr_so_save_regs_and_resolve
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

  .globl XRRRootToScreen
  .p2align 4
  .type XRRRootToScreen, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XRRRootToScreen
#endif
XRRRootToScreen:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXrandr_so_tramp_table+424(%rip)
  je 2f
1:
  jmp *_libXrandr_so_tramp_table+424(%rip)
2:
  pushq $53
  .cfi_adjust_cfa_offset 8
  call _libXrandr_so_save_regs_and_resolve
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

  .globl XRRRotations
  .p2align 4
  .type XRRRotations, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XRRRotations
#endif
XRRRotations:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXrandr_so_tramp_table+432(%rip)
  je 2f
1:
  jmp *_libXrandr_so_tramp_table+432(%rip)
2:
  pushq $54
  .cfi_adjust_cfa_offset 8
  call _libXrandr_so_save_regs_and_resolve
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

  .globl XRRSelectInput
  .p2align 4
  .type XRRSelectInput, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XRRSelectInput
#endif
XRRSelectInput:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXrandr_so_tramp_table+440(%rip)
  je 2f
1:
  jmp *_libXrandr_so_tramp_table+440(%rip)
2:
  pushq $55
  .cfi_adjust_cfa_offset 8
  call _libXrandr_so_save_regs_and_resolve
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

  .globl XRRSetCrtcConfig
  .p2align 4
  .type XRRSetCrtcConfig, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XRRSetCrtcConfig
#endif
XRRSetCrtcConfig:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXrandr_so_tramp_table+448(%rip)
  je 2f
1:
  jmp *_libXrandr_so_tramp_table+448(%rip)
2:
  pushq $56
  .cfi_adjust_cfa_offset 8
  call _libXrandr_so_save_regs_and_resolve
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

  .globl XRRSetCrtcGamma
  .p2align 4
  .type XRRSetCrtcGamma, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XRRSetCrtcGamma
#endif
XRRSetCrtcGamma:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXrandr_so_tramp_table+456(%rip)
  je 2f
1:
  jmp *_libXrandr_so_tramp_table+456(%rip)
2:
  pushq $57
  .cfi_adjust_cfa_offset 8
  call _libXrandr_so_save_regs_and_resolve
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

  .globl XRRSetCrtcTransform
  .p2align 4
  .type XRRSetCrtcTransform, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XRRSetCrtcTransform
#endif
XRRSetCrtcTransform:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXrandr_so_tramp_table+464(%rip)
  je 2f
1:
  jmp *_libXrandr_so_tramp_table+464(%rip)
2:
  pushq $58
  .cfi_adjust_cfa_offset 8
  call _libXrandr_so_save_regs_and_resolve
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

  .globl XRRSetMonitor
  .p2align 4
  .type XRRSetMonitor, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XRRSetMonitor
#endif
XRRSetMonitor:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXrandr_so_tramp_table+472(%rip)
  je 2f
1:
  jmp *_libXrandr_so_tramp_table+472(%rip)
2:
  pushq $59
  .cfi_adjust_cfa_offset 8
  call _libXrandr_so_save_regs_and_resolve
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

  .globl XRRSetOutputPrimary
  .p2align 4
  .type XRRSetOutputPrimary, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XRRSetOutputPrimary
#endif
XRRSetOutputPrimary:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXrandr_so_tramp_table+480(%rip)
  je 2f
1:
  jmp *_libXrandr_so_tramp_table+480(%rip)
2:
  pushq $60
  .cfi_adjust_cfa_offset 8
  call _libXrandr_so_save_regs_and_resolve
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

  .globl XRRSetPanning
  .p2align 4
  .type XRRSetPanning, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XRRSetPanning
#endif
XRRSetPanning:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXrandr_so_tramp_table+488(%rip)
  je 2f
1:
  jmp *_libXrandr_so_tramp_table+488(%rip)
2:
  pushq $61
  .cfi_adjust_cfa_offset 8
  call _libXrandr_so_save_regs_and_resolve
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

  .globl XRRSetProviderOffloadSink
  .p2align 4
  .type XRRSetProviderOffloadSink, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XRRSetProviderOffloadSink
#endif
XRRSetProviderOffloadSink:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXrandr_so_tramp_table+496(%rip)
  je 2f
1:
  jmp *_libXrandr_so_tramp_table+496(%rip)
2:
  pushq $62
  .cfi_adjust_cfa_offset 8
  call _libXrandr_so_save_regs_and_resolve
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

  .globl XRRSetProviderOutputSource
  .p2align 4
  .type XRRSetProviderOutputSource, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XRRSetProviderOutputSource
#endif
XRRSetProviderOutputSource:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXrandr_so_tramp_table+504(%rip)
  je 2f
1:
  jmp *_libXrandr_so_tramp_table+504(%rip)
2:
  pushq $63
  .cfi_adjust_cfa_offset 8
  call _libXrandr_so_save_regs_and_resolve
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

  .globl XRRSetScreenConfig
  .p2align 4
  .type XRRSetScreenConfig, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XRRSetScreenConfig
#endif
XRRSetScreenConfig:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXrandr_so_tramp_table+512(%rip)
  je 2f
1:
  jmp *_libXrandr_so_tramp_table+512(%rip)
2:
  pushq $64
  .cfi_adjust_cfa_offset 8
  call _libXrandr_so_save_regs_and_resolve
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

  .globl XRRSetScreenConfigAndRate
  .p2align 4
  .type XRRSetScreenConfigAndRate, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XRRSetScreenConfigAndRate
#endif
XRRSetScreenConfigAndRate:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXrandr_so_tramp_table+520(%rip)
  je 2f
1:
  jmp *_libXrandr_so_tramp_table+520(%rip)
2:
  pushq $65
  .cfi_adjust_cfa_offset 8
  call _libXrandr_so_save_regs_and_resolve
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

  .globl XRRSetScreenSize
  .p2align 4
  .type XRRSetScreenSize, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XRRSetScreenSize
#endif
XRRSetScreenSize:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXrandr_so_tramp_table+528(%rip)
  je 2f
1:
  jmp *_libXrandr_so_tramp_table+528(%rip)
2:
  pushq $66
  .cfi_adjust_cfa_offset 8
  call _libXrandr_so_save_regs_and_resolve
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

  .globl XRRSizes
  .p2align 4
  .type XRRSizes, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XRRSizes
#endif
XRRSizes:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXrandr_so_tramp_table+536(%rip)
  je 2f
1:
  jmp *_libXrandr_so_tramp_table+536(%rip)
2:
  pushq $67
  .cfi_adjust_cfa_offset 8
  call _libXrandr_so_save_regs_and_resolve
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

  .globl XRRTimes
  .p2align 4
  .type XRRTimes, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XRRTimes
#endif
XRRTimes:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXrandr_so_tramp_table+544(%rip)
  je 2f
1:
  jmp *_libXrandr_so_tramp_table+544(%rip)
2:
  pushq $68
  .cfi_adjust_cfa_offset 8
  call _libXrandr_so_save_regs_and_resolve
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

  .globl XRRUpdateConfiguration
  .p2align 4
  .type XRRUpdateConfiguration, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XRRUpdateConfiguration
#endif
XRRUpdateConfiguration:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXrandr_so_tramp_table+552(%rip)
  je 2f
1:
  jmp *_libXrandr_so_tramp_table+552(%rip)
2:
  pushq $69
  .cfi_adjust_cfa_offset 8
  call _libXrandr_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

