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

  .globl _libXfixes_so_tramp_table
  .hidden _libXfixes_so_tramp_table
  .align 8
_libXfixes_so_tramp_table:
  .zero 312

  .text

  .globl _libXfixes_so_tramp_resolve
  .hidden _libXfixes_so_tramp_resolve

  .globl _libXfixes_so_save_regs_and_resolve
  .hidden _libXfixes_so_save_regs_and_resolve
  .type _libXfixes_so_save_regs_and_resolve, %function
_libXfixes_so_save_regs_and_resolve:
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
  call _libXfixes_so_tramp_resolve

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

  .globl XFixesChangeCursor
  .p2align 4
  .type XFixesChangeCursor, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XFixesChangeCursor
#endif
XFixesChangeCursor:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXfixes_so_tramp_table+0(%rip)
  je 2f
1:
  jmp *_libXfixes_so_tramp_table+0(%rip)
2:
  pushq $0
  .cfi_adjust_cfa_offset 8
  call _libXfixes_so_save_regs_and_resolve
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

  .globl XFixesChangeCursorByName
  .p2align 4
  .type XFixesChangeCursorByName, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XFixesChangeCursorByName
#endif
XFixesChangeCursorByName:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXfixes_so_tramp_table+8(%rip)
  je 2f
1:
  jmp *_libXfixes_so_tramp_table+8(%rip)
2:
  pushq $1
  .cfi_adjust_cfa_offset 8
  call _libXfixes_so_save_regs_and_resolve
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

  .globl XFixesChangeSaveSet
  .p2align 4
  .type XFixesChangeSaveSet, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XFixesChangeSaveSet
#endif
XFixesChangeSaveSet:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXfixes_so_tramp_table+16(%rip)
  je 2f
1:
  jmp *_libXfixes_so_tramp_table+16(%rip)
2:
  pushq $2
  .cfi_adjust_cfa_offset 8
  call _libXfixes_so_save_regs_and_resolve
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

  .globl XFixesCopyRegion
  .p2align 4
  .type XFixesCopyRegion, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XFixesCopyRegion
#endif
XFixesCopyRegion:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXfixes_so_tramp_table+24(%rip)
  je 2f
1:
  jmp *_libXfixes_so_tramp_table+24(%rip)
2:
  pushq $3
  .cfi_adjust_cfa_offset 8
  call _libXfixes_so_save_regs_and_resolve
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

  .globl XFixesCreatePointerBarrier
  .p2align 4
  .type XFixesCreatePointerBarrier, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XFixesCreatePointerBarrier
#endif
XFixesCreatePointerBarrier:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXfixes_so_tramp_table+32(%rip)
  je 2f
1:
  jmp *_libXfixes_so_tramp_table+32(%rip)
2:
  pushq $4
  .cfi_adjust_cfa_offset 8
  call _libXfixes_so_save_regs_and_resolve
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

  .globl XFixesCreateRegion
  .p2align 4
  .type XFixesCreateRegion, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XFixesCreateRegion
#endif
XFixesCreateRegion:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXfixes_so_tramp_table+40(%rip)
  je 2f
1:
  jmp *_libXfixes_so_tramp_table+40(%rip)
2:
  pushq $5
  .cfi_adjust_cfa_offset 8
  call _libXfixes_so_save_regs_and_resolve
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

  .globl XFixesCreateRegionFromBitmap
  .p2align 4
  .type XFixesCreateRegionFromBitmap, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XFixesCreateRegionFromBitmap
#endif
XFixesCreateRegionFromBitmap:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXfixes_so_tramp_table+48(%rip)
  je 2f
1:
  jmp *_libXfixes_so_tramp_table+48(%rip)
2:
  pushq $6
  .cfi_adjust_cfa_offset 8
  call _libXfixes_so_save_regs_and_resolve
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

  .globl XFixesCreateRegionFromGC
  .p2align 4
  .type XFixesCreateRegionFromGC, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XFixesCreateRegionFromGC
#endif
XFixesCreateRegionFromGC:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXfixes_so_tramp_table+56(%rip)
  je 2f
1:
  jmp *_libXfixes_so_tramp_table+56(%rip)
2:
  pushq $7
  .cfi_adjust_cfa_offset 8
  call _libXfixes_so_save_regs_and_resolve
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

  .globl XFixesCreateRegionFromPicture
  .p2align 4
  .type XFixesCreateRegionFromPicture, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XFixesCreateRegionFromPicture
#endif
XFixesCreateRegionFromPicture:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXfixes_so_tramp_table+64(%rip)
  je 2f
1:
  jmp *_libXfixes_so_tramp_table+64(%rip)
2:
  pushq $8
  .cfi_adjust_cfa_offset 8
  call _libXfixes_so_save_regs_and_resolve
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

  .globl XFixesCreateRegionFromWindow
  .p2align 4
  .type XFixesCreateRegionFromWindow, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XFixesCreateRegionFromWindow
#endif
XFixesCreateRegionFromWindow:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXfixes_so_tramp_table+72(%rip)
  je 2f
1:
  jmp *_libXfixes_so_tramp_table+72(%rip)
2:
  pushq $9
  .cfi_adjust_cfa_offset 8
  call _libXfixes_so_save_regs_and_resolve
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

  .globl XFixesDestroyPointerBarrier
  .p2align 4
  .type XFixesDestroyPointerBarrier, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XFixesDestroyPointerBarrier
#endif
XFixesDestroyPointerBarrier:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXfixes_so_tramp_table+80(%rip)
  je 2f
1:
  jmp *_libXfixes_so_tramp_table+80(%rip)
2:
  pushq $10
  .cfi_adjust_cfa_offset 8
  call _libXfixes_so_save_regs_and_resolve
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

  .globl XFixesDestroyRegion
  .p2align 4
  .type XFixesDestroyRegion, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XFixesDestroyRegion
#endif
XFixesDestroyRegion:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXfixes_so_tramp_table+88(%rip)
  je 2f
1:
  jmp *_libXfixes_so_tramp_table+88(%rip)
2:
  pushq $11
  .cfi_adjust_cfa_offset 8
  call _libXfixes_so_save_regs_and_resolve
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

  .globl XFixesExpandRegion
  .p2align 4
  .type XFixesExpandRegion, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XFixesExpandRegion
#endif
XFixesExpandRegion:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXfixes_so_tramp_table+96(%rip)
  je 2f
1:
  jmp *_libXfixes_so_tramp_table+96(%rip)
2:
  pushq $12
  .cfi_adjust_cfa_offset 8
  call _libXfixes_so_save_regs_and_resolve
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

  .globl XFixesFetchRegion
  .p2align 4
  .type XFixesFetchRegion, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XFixesFetchRegion
#endif
XFixesFetchRegion:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXfixes_so_tramp_table+104(%rip)
  je 2f
1:
  jmp *_libXfixes_so_tramp_table+104(%rip)
2:
  pushq $13
  .cfi_adjust_cfa_offset 8
  call _libXfixes_so_save_regs_and_resolve
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

  .globl XFixesFetchRegionAndBounds
  .p2align 4
  .type XFixesFetchRegionAndBounds, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XFixesFetchRegionAndBounds
#endif
XFixesFetchRegionAndBounds:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXfixes_so_tramp_table+112(%rip)
  je 2f
1:
  jmp *_libXfixes_so_tramp_table+112(%rip)
2:
  pushq $14
  .cfi_adjust_cfa_offset 8
  call _libXfixes_so_save_regs_and_resolve
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

  .globl XFixesFindDisplay
  .p2align 4
  .type XFixesFindDisplay, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XFixesFindDisplay
#endif
XFixesFindDisplay:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXfixes_so_tramp_table+120(%rip)
  je 2f
1:
  jmp *_libXfixes_so_tramp_table+120(%rip)
2:
  pushq $15
  .cfi_adjust_cfa_offset 8
  call _libXfixes_so_save_regs_and_resolve
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

  .globl XFixesGetClientDisconnectMode
  .p2align 4
  .type XFixesGetClientDisconnectMode, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XFixesGetClientDisconnectMode
#endif
XFixesGetClientDisconnectMode:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXfixes_so_tramp_table+128(%rip)
  je 2f
1:
  jmp *_libXfixes_so_tramp_table+128(%rip)
2:
  pushq $16
  .cfi_adjust_cfa_offset 8
  call _libXfixes_so_save_regs_and_resolve
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

  .globl XFixesGetCursorImage
  .p2align 4
  .type XFixesGetCursorImage, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XFixesGetCursorImage
#endif
XFixesGetCursorImage:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXfixes_so_tramp_table+136(%rip)
  je 2f
1:
  jmp *_libXfixes_so_tramp_table+136(%rip)
2:
  pushq $17
  .cfi_adjust_cfa_offset 8
  call _libXfixes_so_save_regs_and_resolve
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

  .globl XFixesGetCursorName
  .p2align 4
  .type XFixesGetCursorName, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XFixesGetCursorName
#endif
XFixesGetCursorName:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXfixes_so_tramp_table+144(%rip)
  je 2f
1:
  jmp *_libXfixes_so_tramp_table+144(%rip)
2:
  pushq $18
  .cfi_adjust_cfa_offset 8
  call _libXfixes_so_save_regs_and_resolve
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

  .globl XFixesHideCursor
  .p2align 4
  .type XFixesHideCursor, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XFixesHideCursor
#endif
XFixesHideCursor:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXfixes_so_tramp_table+152(%rip)
  je 2f
1:
  jmp *_libXfixes_so_tramp_table+152(%rip)
2:
  pushq $19
  .cfi_adjust_cfa_offset 8
  call _libXfixes_so_save_regs_and_resolve
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

  .globl XFixesIntersectRegion
  .p2align 4
  .type XFixesIntersectRegion, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XFixesIntersectRegion
#endif
XFixesIntersectRegion:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXfixes_so_tramp_table+160(%rip)
  je 2f
1:
  jmp *_libXfixes_so_tramp_table+160(%rip)
2:
  pushq $20
  .cfi_adjust_cfa_offset 8
  call _libXfixes_so_save_regs_and_resolve
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

  .globl XFixesInvertRegion
  .p2align 4
  .type XFixesInvertRegion, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XFixesInvertRegion
#endif
XFixesInvertRegion:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXfixes_so_tramp_table+168(%rip)
  je 2f
1:
  jmp *_libXfixes_so_tramp_table+168(%rip)
2:
  pushq $21
  .cfi_adjust_cfa_offset 8
  call _libXfixes_so_save_regs_and_resolve
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

  .globl XFixesQueryExtension
  .p2align 4
  .type XFixesQueryExtension, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XFixesQueryExtension
#endif
XFixesQueryExtension:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXfixes_so_tramp_table+176(%rip)
  je 2f
1:
  jmp *_libXfixes_so_tramp_table+176(%rip)
2:
  pushq $22
  .cfi_adjust_cfa_offset 8
  call _libXfixes_so_save_regs_and_resolve
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

  .globl XFixesQueryVersion
  .p2align 4
  .type XFixesQueryVersion, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XFixesQueryVersion
#endif
XFixesQueryVersion:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXfixes_so_tramp_table+184(%rip)
  je 2f
1:
  jmp *_libXfixes_so_tramp_table+184(%rip)
2:
  pushq $23
  .cfi_adjust_cfa_offset 8
  call _libXfixes_so_save_regs_and_resolve
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

  .globl XFixesRegionExtents
  .p2align 4
  .type XFixesRegionExtents, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XFixesRegionExtents
#endif
XFixesRegionExtents:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXfixes_so_tramp_table+192(%rip)
  je 2f
1:
  jmp *_libXfixes_so_tramp_table+192(%rip)
2:
  pushq $24
  .cfi_adjust_cfa_offset 8
  call _libXfixes_so_save_regs_and_resolve
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

  .globl XFixesSelectCursorInput
  .p2align 4
  .type XFixesSelectCursorInput, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XFixesSelectCursorInput
#endif
XFixesSelectCursorInput:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXfixes_so_tramp_table+200(%rip)
  je 2f
1:
  jmp *_libXfixes_so_tramp_table+200(%rip)
2:
  pushq $25
  .cfi_adjust_cfa_offset 8
  call _libXfixes_so_save_regs_and_resolve
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

  .globl XFixesSelectSelectionInput
  .p2align 4
  .type XFixesSelectSelectionInput, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XFixesSelectSelectionInput
#endif
XFixesSelectSelectionInput:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXfixes_so_tramp_table+208(%rip)
  je 2f
1:
  jmp *_libXfixes_so_tramp_table+208(%rip)
2:
  pushq $26
  .cfi_adjust_cfa_offset 8
  call _libXfixes_so_save_regs_and_resolve
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

  .globl XFixesSetClientDisconnectMode
  .p2align 4
  .type XFixesSetClientDisconnectMode, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XFixesSetClientDisconnectMode
#endif
XFixesSetClientDisconnectMode:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXfixes_so_tramp_table+216(%rip)
  je 2f
1:
  jmp *_libXfixes_so_tramp_table+216(%rip)
2:
  pushq $27
  .cfi_adjust_cfa_offset 8
  call _libXfixes_so_save_regs_and_resolve
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

  .globl XFixesSetCursorName
  .p2align 4
  .type XFixesSetCursorName, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XFixesSetCursorName
#endif
XFixesSetCursorName:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXfixes_so_tramp_table+224(%rip)
  je 2f
1:
  jmp *_libXfixes_so_tramp_table+224(%rip)
2:
  pushq $28
  .cfi_adjust_cfa_offset 8
  call _libXfixes_so_save_regs_and_resolve
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

  .globl XFixesSetGCClipRegion
  .p2align 4
  .type XFixesSetGCClipRegion, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XFixesSetGCClipRegion
#endif
XFixesSetGCClipRegion:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXfixes_so_tramp_table+232(%rip)
  je 2f
1:
  jmp *_libXfixes_so_tramp_table+232(%rip)
2:
  pushq $29
  .cfi_adjust_cfa_offset 8
  call _libXfixes_so_save_regs_and_resolve
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

  .globl XFixesSetPictureClipRegion
  .p2align 4
  .type XFixesSetPictureClipRegion, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XFixesSetPictureClipRegion
#endif
XFixesSetPictureClipRegion:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXfixes_so_tramp_table+240(%rip)
  je 2f
1:
  jmp *_libXfixes_so_tramp_table+240(%rip)
2:
  pushq $30
  .cfi_adjust_cfa_offset 8
  call _libXfixes_so_save_regs_and_resolve
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

  .globl XFixesSetRegion
  .p2align 4
  .type XFixesSetRegion, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XFixesSetRegion
#endif
XFixesSetRegion:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXfixes_so_tramp_table+248(%rip)
  je 2f
1:
  jmp *_libXfixes_so_tramp_table+248(%rip)
2:
  pushq $31
  .cfi_adjust_cfa_offset 8
  call _libXfixes_so_save_regs_and_resolve
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

  .globl XFixesSetWindowShapeRegion
  .p2align 4
  .type XFixesSetWindowShapeRegion, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XFixesSetWindowShapeRegion
#endif
XFixesSetWindowShapeRegion:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXfixes_so_tramp_table+256(%rip)
  je 2f
1:
  jmp *_libXfixes_so_tramp_table+256(%rip)
2:
  pushq $32
  .cfi_adjust_cfa_offset 8
  call _libXfixes_so_save_regs_and_resolve
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

  .globl XFixesShowCursor
  .p2align 4
  .type XFixesShowCursor, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XFixesShowCursor
#endif
XFixesShowCursor:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXfixes_so_tramp_table+264(%rip)
  je 2f
1:
  jmp *_libXfixes_so_tramp_table+264(%rip)
2:
  pushq $33
  .cfi_adjust_cfa_offset 8
  call _libXfixes_so_save_regs_and_resolve
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

  .globl XFixesSubtractRegion
  .p2align 4
  .type XFixesSubtractRegion, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XFixesSubtractRegion
#endif
XFixesSubtractRegion:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXfixes_so_tramp_table+272(%rip)
  je 2f
1:
  jmp *_libXfixes_so_tramp_table+272(%rip)
2:
  pushq $34
  .cfi_adjust_cfa_offset 8
  call _libXfixes_so_save_regs_and_resolve
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

  .globl XFixesTranslateRegion
  .p2align 4
  .type XFixesTranslateRegion, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XFixesTranslateRegion
#endif
XFixesTranslateRegion:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXfixes_so_tramp_table+280(%rip)
  je 2f
1:
  jmp *_libXfixes_so_tramp_table+280(%rip)
2:
  pushq $35
  .cfi_adjust_cfa_offset 8
  call _libXfixes_so_save_regs_and_resolve
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

  .globl XFixesUnionRegion
  .p2align 4
  .type XFixesUnionRegion, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XFixesUnionRegion
#endif
XFixesUnionRegion:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXfixes_so_tramp_table+288(%rip)
  je 2f
1:
  jmp *_libXfixes_so_tramp_table+288(%rip)
2:
  pushq $36
  .cfi_adjust_cfa_offset 8
  call _libXfixes_so_save_regs_and_resolve
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

  .globl XFixesVersion
  .p2align 4
  .type XFixesVersion, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XFixesVersion
#endif
XFixesVersion:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libXfixes_so_tramp_table+296(%rip)
  je 2f
1:
  jmp *_libXfixes_so_tramp_table+296(%rip)
2:
  pushq $37
  .cfi_adjust_cfa_offset 8
  call _libXfixes_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

