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

  .globl _libX11_so_tramp_table
  .hidden _libX11_so_tramp_table
  .align 8
_libX11_so_tramp_table:
  .zero 9464

  .text

  .globl _libX11_so_tramp_resolve
  .hidden _libX11_so_tramp_resolve

  .globl _libX11_so_save_regs_and_resolve
  .hidden _libX11_so_save_regs_and_resolve
  .type _libX11_so_save_regs_and_resolve, %function
_libX11_so_save_regs_and_resolve:
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
  call _libX11_so_tramp_resolve

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

  .globl KeySymToUcs4
  .p2align 4
  .type KeySymToUcs4, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden KeySymToUcs4
#endif
KeySymToUcs4:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+0(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+0(%rip)
2:
  pushq $0
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XActivateScreenSaver
  .p2align 4
  .type XActivateScreenSaver, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XActivateScreenSaver
#endif
XActivateScreenSaver:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8(%rip)
2:
  pushq $1
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XAddConnectionWatch
  .p2align 4
  .type XAddConnectionWatch, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XAddConnectionWatch
#endif
XAddConnectionWatch:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+16(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+16(%rip)
2:
  pushq $2
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XAddExtension
  .p2align 4
  .type XAddExtension, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XAddExtension
#endif
XAddExtension:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+24(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+24(%rip)
2:
  pushq $3
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XAddHost
  .p2align 4
  .type XAddHost, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XAddHost
#endif
XAddHost:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+32(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+32(%rip)
2:
  pushq $4
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XAddHosts
  .p2align 4
  .type XAddHosts, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XAddHosts
#endif
XAddHosts:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+40(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+40(%rip)
2:
  pushq $5
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XAddPixel
  .p2align 4
  .type XAddPixel, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XAddPixel
#endif
XAddPixel:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+48(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+48(%rip)
2:
  pushq $6
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XAddToExtensionList
  .p2align 4
  .type XAddToExtensionList, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XAddToExtensionList
#endif
XAddToExtensionList:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+56(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+56(%rip)
2:
  pushq $7
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XAddToSaveSet
  .p2align 4
  .type XAddToSaveSet, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XAddToSaveSet
#endif
XAddToSaveSet:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+64(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+64(%rip)
2:
  pushq $8
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XAllPlanes
  .p2align 4
  .type XAllPlanes, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XAllPlanes
#endif
XAllPlanes:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+72(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+72(%rip)
2:
  pushq $9
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XAllocClassHint
  .p2align 4
  .type XAllocClassHint, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XAllocClassHint
#endif
XAllocClassHint:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+80(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+80(%rip)
2:
  pushq $10
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XAllocColor
  .p2align 4
  .type XAllocColor, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XAllocColor
#endif
XAllocColor:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+88(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+88(%rip)
2:
  pushq $11
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XAllocColorCells
  .p2align 4
  .type XAllocColorCells, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XAllocColorCells
#endif
XAllocColorCells:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+96(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+96(%rip)
2:
  pushq $12
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XAllocColorPlanes
  .p2align 4
  .type XAllocColorPlanes, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XAllocColorPlanes
#endif
XAllocColorPlanes:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+104(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+104(%rip)
2:
  pushq $13
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XAllocIconSize
  .p2align 4
  .type XAllocIconSize, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XAllocIconSize
#endif
XAllocIconSize:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+112(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+112(%rip)
2:
  pushq $14
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XAllocNamedColor
  .p2align 4
  .type XAllocNamedColor, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XAllocNamedColor
#endif
XAllocNamedColor:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+120(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+120(%rip)
2:
  pushq $15
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XAllocSizeHints
  .p2align 4
  .type XAllocSizeHints, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XAllocSizeHints
#endif
XAllocSizeHints:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+128(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+128(%rip)
2:
  pushq $16
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XAllocStandardColormap
  .p2align 4
  .type XAllocStandardColormap, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XAllocStandardColormap
#endif
XAllocStandardColormap:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+136(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+136(%rip)
2:
  pushq $17
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XAllocWMHints
  .p2align 4
  .type XAllocWMHints, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XAllocWMHints
#endif
XAllocWMHints:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+144(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+144(%rip)
2:
  pushq $18
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XAllowEvents
  .p2align 4
  .type XAllowEvents, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XAllowEvents
#endif
XAllowEvents:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+152(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+152(%rip)
2:
  pushq $19
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XAutoRepeatOff
  .p2align 4
  .type XAutoRepeatOff, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XAutoRepeatOff
#endif
XAutoRepeatOff:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+160(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+160(%rip)
2:
  pushq $20
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XAutoRepeatOn
  .p2align 4
  .type XAutoRepeatOn, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XAutoRepeatOn
#endif
XAutoRepeatOn:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+168(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+168(%rip)
2:
  pushq $21
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XBaseFontNameListOfFontSet
  .p2align 4
  .type XBaseFontNameListOfFontSet, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XBaseFontNameListOfFontSet
#endif
XBaseFontNameListOfFontSet:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+176(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+176(%rip)
2:
  pushq $22
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XBell
  .p2align 4
  .type XBell, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XBell
#endif
XBell:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+184(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+184(%rip)
2:
  pushq $23
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XBitmapBitOrder
  .p2align 4
  .type XBitmapBitOrder, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XBitmapBitOrder
#endif
XBitmapBitOrder:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+192(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+192(%rip)
2:
  pushq $24
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XBitmapPad
  .p2align 4
  .type XBitmapPad, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XBitmapPad
#endif
XBitmapPad:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+200(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+200(%rip)
2:
  pushq $25
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XBitmapUnit
  .p2align 4
  .type XBitmapUnit, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XBitmapUnit
#endif
XBitmapUnit:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+208(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+208(%rip)
2:
  pushq $26
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XBlackPixel
  .p2align 4
  .type XBlackPixel, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XBlackPixel
#endif
XBlackPixel:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+216(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+216(%rip)
2:
  pushq $27
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XBlackPixelOfScreen
  .p2align 4
  .type XBlackPixelOfScreen, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XBlackPixelOfScreen
#endif
XBlackPixelOfScreen:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+224(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+224(%rip)
2:
  pushq $28
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XCellsOfScreen
  .p2align 4
  .type XCellsOfScreen, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XCellsOfScreen
#endif
XCellsOfScreen:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+232(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+232(%rip)
2:
  pushq $29
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XChangeActivePointerGrab
  .p2align 4
  .type XChangeActivePointerGrab, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XChangeActivePointerGrab
#endif
XChangeActivePointerGrab:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+240(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+240(%rip)
2:
  pushq $30
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XChangeGC
  .p2align 4
  .type XChangeGC, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XChangeGC
#endif
XChangeGC:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+248(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+248(%rip)
2:
  pushq $31
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XChangeKeyboardControl
  .p2align 4
  .type XChangeKeyboardControl, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XChangeKeyboardControl
#endif
XChangeKeyboardControl:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+256(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+256(%rip)
2:
  pushq $32
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XChangeKeyboardMapping
  .p2align 4
  .type XChangeKeyboardMapping, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XChangeKeyboardMapping
#endif
XChangeKeyboardMapping:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+264(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+264(%rip)
2:
  pushq $33
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XChangePointerControl
  .p2align 4
  .type XChangePointerControl, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XChangePointerControl
#endif
XChangePointerControl:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+272(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+272(%rip)
2:
  pushq $34
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XChangeProperty
  .p2align 4
  .type XChangeProperty, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XChangeProperty
#endif
XChangeProperty:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+280(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+280(%rip)
2:
  pushq $35
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XChangeSaveSet
  .p2align 4
  .type XChangeSaveSet, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XChangeSaveSet
#endif
XChangeSaveSet:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+288(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+288(%rip)
2:
  pushq $36
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XChangeWindowAttributes
  .p2align 4
  .type XChangeWindowAttributes, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XChangeWindowAttributes
#endif
XChangeWindowAttributes:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+296(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+296(%rip)
2:
  pushq $37
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XCheckIfEvent
  .p2align 4
  .type XCheckIfEvent, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XCheckIfEvent
#endif
XCheckIfEvent:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+304(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+304(%rip)
2:
  pushq $38
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XCheckMaskEvent
  .p2align 4
  .type XCheckMaskEvent, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XCheckMaskEvent
#endif
XCheckMaskEvent:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+312(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+312(%rip)
2:
  pushq $39
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XCheckTypedEvent
  .p2align 4
  .type XCheckTypedEvent, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XCheckTypedEvent
#endif
XCheckTypedEvent:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+320(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+320(%rip)
2:
  pushq $40
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XCheckTypedWindowEvent
  .p2align 4
  .type XCheckTypedWindowEvent, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XCheckTypedWindowEvent
#endif
XCheckTypedWindowEvent:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+328(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+328(%rip)
2:
  pushq $41
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XCheckWindowEvent
  .p2align 4
  .type XCheckWindowEvent, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XCheckWindowEvent
#endif
XCheckWindowEvent:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+336(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+336(%rip)
2:
  pushq $42
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XCirculateSubwindows
  .p2align 4
  .type XCirculateSubwindows, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XCirculateSubwindows
#endif
XCirculateSubwindows:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+344(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+344(%rip)
2:
  pushq $43
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XCirculateSubwindowsDown
  .p2align 4
  .type XCirculateSubwindowsDown, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XCirculateSubwindowsDown
#endif
XCirculateSubwindowsDown:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+352(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+352(%rip)
2:
  pushq $44
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XCirculateSubwindowsUp
  .p2align 4
  .type XCirculateSubwindowsUp, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XCirculateSubwindowsUp
#endif
XCirculateSubwindowsUp:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+360(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+360(%rip)
2:
  pushq $45
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XClearArea
  .p2align 4
  .type XClearArea, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XClearArea
#endif
XClearArea:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+368(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+368(%rip)
2:
  pushq $46
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XClearWindow
  .p2align 4
  .type XClearWindow, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XClearWindow
#endif
XClearWindow:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+376(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+376(%rip)
2:
  pushq $47
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XClipBox
  .p2align 4
  .type XClipBox, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XClipBox
#endif
XClipBox:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+384(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+384(%rip)
2:
  pushq $48
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XCloseDisplay
  .p2align 4
  .type XCloseDisplay, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XCloseDisplay
#endif
XCloseDisplay:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+392(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+392(%rip)
2:
  pushq $49
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XCloseIM
  .p2align 4
  .type XCloseIM, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XCloseIM
#endif
XCloseIM:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+400(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+400(%rip)
2:
  pushq $50
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XCloseOM
  .p2align 4
  .type XCloseOM, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XCloseOM
#endif
XCloseOM:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+408(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+408(%rip)
2:
  pushq $51
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XConfigureWindow
  .p2align 4
  .type XConfigureWindow, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XConfigureWindow
#endif
XConfigureWindow:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+416(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+416(%rip)
2:
  pushq $52
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XConnectionNumber
  .p2align 4
  .type XConnectionNumber, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XConnectionNumber
#endif
XConnectionNumber:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+424(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+424(%rip)
2:
  pushq $53
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XContextDependentDrawing
  .p2align 4
  .type XContextDependentDrawing, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XContextDependentDrawing
#endif
XContextDependentDrawing:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+432(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+432(%rip)
2:
  pushq $54
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XContextualDrawing
  .p2align 4
  .type XContextualDrawing, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XContextualDrawing
#endif
XContextualDrawing:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+440(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+440(%rip)
2:
  pushq $55
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XConvertCase
  .p2align 4
  .type XConvertCase, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XConvertCase
#endif
XConvertCase:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+448(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+448(%rip)
2:
  pushq $56
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XConvertSelection
  .p2align 4
  .type XConvertSelection, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XConvertSelection
#endif
XConvertSelection:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+456(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+456(%rip)
2:
  pushq $57
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XCopyArea
  .p2align 4
  .type XCopyArea, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XCopyArea
#endif
XCopyArea:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+464(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+464(%rip)
2:
  pushq $58
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XCopyColormapAndFree
  .p2align 4
  .type XCopyColormapAndFree, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XCopyColormapAndFree
#endif
XCopyColormapAndFree:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+472(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+472(%rip)
2:
  pushq $59
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XCopyGC
  .p2align 4
  .type XCopyGC, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XCopyGC
#endif
XCopyGC:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+480(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+480(%rip)
2:
  pushq $60
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XCopyPlane
  .p2align 4
  .type XCopyPlane, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XCopyPlane
#endif
XCopyPlane:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+488(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+488(%rip)
2:
  pushq $61
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XCreateBitmapFromData
  .p2align 4
  .type XCreateBitmapFromData, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XCreateBitmapFromData
#endif
XCreateBitmapFromData:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+496(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+496(%rip)
2:
  pushq $62
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XCreateColormap
  .p2align 4
  .type XCreateColormap, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XCreateColormap
#endif
XCreateColormap:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+504(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+504(%rip)
2:
  pushq $63
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XCreateFontCursor
  .p2align 4
  .type XCreateFontCursor, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XCreateFontCursor
#endif
XCreateFontCursor:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+512(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+512(%rip)
2:
  pushq $64
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XCreateFontSet
  .p2align 4
  .type XCreateFontSet, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XCreateFontSet
#endif
XCreateFontSet:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+520(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+520(%rip)
2:
  pushq $65
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XCreateGC
  .p2align 4
  .type XCreateGC, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XCreateGC
#endif
XCreateGC:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+528(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+528(%rip)
2:
  pushq $66
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XCreateGlyphCursor
  .p2align 4
  .type XCreateGlyphCursor, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XCreateGlyphCursor
#endif
XCreateGlyphCursor:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+536(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+536(%rip)
2:
  pushq $67
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XCreateIC
  .p2align 4
  .type XCreateIC, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XCreateIC
#endif
XCreateIC:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+544(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+544(%rip)
2:
  pushq $68
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XCreateImage
  .p2align 4
  .type XCreateImage, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XCreateImage
#endif
XCreateImage:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+552(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+552(%rip)
2:
  pushq $69
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XCreateOC
  .p2align 4
  .type XCreateOC, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XCreateOC
#endif
XCreateOC:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+560(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+560(%rip)
2:
  pushq $70
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XCreatePixmap
  .p2align 4
  .type XCreatePixmap, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XCreatePixmap
#endif
XCreatePixmap:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+568(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+568(%rip)
2:
  pushq $71
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XCreatePixmapCursor
  .p2align 4
  .type XCreatePixmapCursor, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XCreatePixmapCursor
#endif
XCreatePixmapCursor:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+576(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+576(%rip)
2:
  pushq $72
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XCreatePixmapFromBitmapData
  .p2align 4
  .type XCreatePixmapFromBitmapData, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XCreatePixmapFromBitmapData
#endif
XCreatePixmapFromBitmapData:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+584(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+584(%rip)
2:
  pushq $73
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XCreateRegion
  .p2align 4
  .type XCreateRegion, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XCreateRegion
#endif
XCreateRegion:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+592(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+592(%rip)
2:
  pushq $74
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XCreateSimpleWindow
  .p2align 4
  .type XCreateSimpleWindow, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XCreateSimpleWindow
#endif
XCreateSimpleWindow:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+600(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+600(%rip)
2:
  pushq $75
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XCreateWindow
  .p2align 4
  .type XCreateWindow, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XCreateWindow
#endif
XCreateWindow:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+608(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+608(%rip)
2:
  pushq $76
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XDefaultColormap
  .p2align 4
  .type XDefaultColormap, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XDefaultColormap
#endif
XDefaultColormap:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+616(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+616(%rip)
2:
  pushq $77
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XDefaultColormapOfScreen
  .p2align 4
  .type XDefaultColormapOfScreen, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XDefaultColormapOfScreen
#endif
XDefaultColormapOfScreen:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+624(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+624(%rip)
2:
  pushq $78
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XDefaultDepth
  .p2align 4
  .type XDefaultDepth, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XDefaultDepth
#endif
XDefaultDepth:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+632(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+632(%rip)
2:
  pushq $79
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XDefaultDepthOfScreen
  .p2align 4
  .type XDefaultDepthOfScreen, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XDefaultDepthOfScreen
#endif
XDefaultDepthOfScreen:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+640(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+640(%rip)
2:
  pushq $80
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XDefaultGC
  .p2align 4
  .type XDefaultGC, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XDefaultGC
#endif
XDefaultGC:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+648(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+648(%rip)
2:
  pushq $81
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XDefaultGCOfScreen
  .p2align 4
  .type XDefaultGCOfScreen, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XDefaultGCOfScreen
#endif
XDefaultGCOfScreen:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+656(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+656(%rip)
2:
  pushq $82
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XDefaultRootWindow
  .p2align 4
  .type XDefaultRootWindow, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XDefaultRootWindow
#endif
XDefaultRootWindow:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+664(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+664(%rip)
2:
  pushq $83
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XDefaultScreen
  .p2align 4
  .type XDefaultScreen, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XDefaultScreen
#endif
XDefaultScreen:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+672(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+672(%rip)
2:
  pushq $84
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XDefaultScreenOfDisplay
  .p2align 4
  .type XDefaultScreenOfDisplay, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XDefaultScreenOfDisplay
#endif
XDefaultScreenOfDisplay:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+680(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+680(%rip)
2:
  pushq $85
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XDefaultString
  .p2align 4
  .type XDefaultString, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XDefaultString
#endif
XDefaultString:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+688(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+688(%rip)
2:
  pushq $86
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XDefaultVisual
  .p2align 4
  .type XDefaultVisual, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XDefaultVisual
#endif
XDefaultVisual:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+696(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+696(%rip)
2:
  pushq $87
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XDefaultVisualOfScreen
  .p2align 4
  .type XDefaultVisualOfScreen, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XDefaultVisualOfScreen
#endif
XDefaultVisualOfScreen:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+704(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+704(%rip)
2:
  pushq $88
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XDefineCursor
  .p2align 4
  .type XDefineCursor, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XDefineCursor
#endif
XDefineCursor:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+712(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+712(%rip)
2:
  pushq $89
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XDeleteContext
  .p2align 4
  .type XDeleteContext, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XDeleteContext
#endif
XDeleteContext:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+720(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+720(%rip)
2:
  pushq $90
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XDeleteModifiermapEntry
  .p2align 4
  .type XDeleteModifiermapEntry, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XDeleteModifiermapEntry
#endif
XDeleteModifiermapEntry:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+728(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+728(%rip)
2:
  pushq $91
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XDeleteProperty
  .p2align 4
  .type XDeleteProperty, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XDeleteProperty
#endif
XDeleteProperty:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+736(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+736(%rip)
2:
  pushq $92
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XDestroyIC
  .p2align 4
  .type XDestroyIC, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XDestroyIC
#endif
XDestroyIC:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+744(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+744(%rip)
2:
  pushq $93
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XDestroyImage
  .p2align 4
  .type XDestroyImage, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XDestroyImage
#endif
XDestroyImage:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+752(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+752(%rip)
2:
  pushq $94
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XDestroyOC
  .p2align 4
  .type XDestroyOC, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XDestroyOC
#endif
XDestroyOC:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+760(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+760(%rip)
2:
  pushq $95
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XDestroyRegion
  .p2align 4
  .type XDestroyRegion, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XDestroyRegion
#endif
XDestroyRegion:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+768(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+768(%rip)
2:
  pushq $96
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XDestroySubwindows
  .p2align 4
  .type XDestroySubwindows, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XDestroySubwindows
#endif
XDestroySubwindows:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+776(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+776(%rip)
2:
  pushq $97
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XDestroyWindow
  .p2align 4
  .type XDestroyWindow, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XDestroyWindow
#endif
XDestroyWindow:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+784(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+784(%rip)
2:
  pushq $98
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XDirectionalDependentDrawing
  .p2align 4
  .type XDirectionalDependentDrawing, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XDirectionalDependentDrawing
#endif
XDirectionalDependentDrawing:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+792(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+792(%rip)
2:
  pushq $99
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XDisableAccessControl
  .p2align 4
  .type XDisableAccessControl, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XDisableAccessControl
#endif
XDisableAccessControl:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+800(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+800(%rip)
2:
  pushq $100
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XDisplayCells
  .p2align 4
  .type XDisplayCells, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XDisplayCells
#endif
XDisplayCells:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+808(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+808(%rip)
2:
  pushq $101
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XDisplayHeight
  .p2align 4
  .type XDisplayHeight, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XDisplayHeight
#endif
XDisplayHeight:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+816(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+816(%rip)
2:
  pushq $102
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XDisplayHeightMM
  .p2align 4
  .type XDisplayHeightMM, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XDisplayHeightMM
#endif
XDisplayHeightMM:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+824(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+824(%rip)
2:
  pushq $103
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XDisplayKeycodes
  .p2align 4
  .type XDisplayKeycodes, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XDisplayKeycodes
#endif
XDisplayKeycodes:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+832(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+832(%rip)
2:
  pushq $104
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XDisplayMotionBufferSize
  .p2align 4
  .type XDisplayMotionBufferSize, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XDisplayMotionBufferSize
#endif
XDisplayMotionBufferSize:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+840(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+840(%rip)
2:
  pushq $105
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XDisplayName
  .p2align 4
  .type XDisplayName, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XDisplayName
#endif
XDisplayName:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+848(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+848(%rip)
2:
  pushq $106
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XDisplayOfIM
  .p2align 4
  .type XDisplayOfIM, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XDisplayOfIM
#endif
XDisplayOfIM:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+856(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+856(%rip)
2:
  pushq $107
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XDisplayOfOM
  .p2align 4
  .type XDisplayOfOM, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XDisplayOfOM
#endif
XDisplayOfOM:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+864(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+864(%rip)
2:
  pushq $108
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XDisplayOfScreen
  .p2align 4
  .type XDisplayOfScreen, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XDisplayOfScreen
#endif
XDisplayOfScreen:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+872(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+872(%rip)
2:
  pushq $109
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XDisplayPlanes
  .p2align 4
  .type XDisplayPlanes, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XDisplayPlanes
#endif
XDisplayPlanes:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+880(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+880(%rip)
2:
  pushq $110
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XDisplayString
  .p2align 4
  .type XDisplayString, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XDisplayString
#endif
XDisplayString:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+888(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+888(%rip)
2:
  pushq $111
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XDisplayWidth
  .p2align 4
  .type XDisplayWidth, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XDisplayWidth
#endif
XDisplayWidth:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+896(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+896(%rip)
2:
  pushq $112
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XDisplayWidthMM
  .p2align 4
  .type XDisplayWidthMM, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XDisplayWidthMM
#endif
XDisplayWidthMM:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+904(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+904(%rip)
2:
  pushq $113
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XDoesBackingStore
  .p2align 4
  .type XDoesBackingStore, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XDoesBackingStore
#endif
XDoesBackingStore:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+912(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+912(%rip)
2:
  pushq $114
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XDoesSaveUnders
  .p2align 4
  .type XDoesSaveUnders, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XDoesSaveUnders
#endif
XDoesSaveUnders:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+920(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+920(%rip)
2:
  pushq $115
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XDrawArc
  .p2align 4
  .type XDrawArc, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XDrawArc
#endif
XDrawArc:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+928(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+928(%rip)
2:
  pushq $116
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XDrawArcs
  .p2align 4
  .type XDrawArcs, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XDrawArcs
#endif
XDrawArcs:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+936(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+936(%rip)
2:
  pushq $117
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XDrawImageString
  .p2align 4
  .type XDrawImageString, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XDrawImageString
#endif
XDrawImageString:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+944(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+944(%rip)
2:
  pushq $118
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XDrawImageString16
  .p2align 4
  .type XDrawImageString16, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XDrawImageString16
#endif
XDrawImageString16:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+952(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+952(%rip)
2:
  pushq $119
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XDrawLine
  .p2align 4
  .type XDrawLine, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XDrawLine
#endif
XDrawLine:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+960(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+960(%rip)
2:
  pushq $120
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XDrawLines
  .p2align 4
  .type XDrawLines, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XDrawLines
#endif
XDrawLines:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+968(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+968(%rip)
2:
  pushq $121
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XDrawPoint
  .p2align 4
  .type XDrawPoint, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XDrawPoint
#endif
XDrawPoint:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+976(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+976(%rip)
2:
  pushq $122
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XDrawPoints
  .p2align 4
  .type XDrawPoints, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XDrawPoints
#endif
XDrawPoints:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+984(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+984(%rip)
2:
  pushq $123
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XDrawRectangle
  .p2align 4
  .type XDrawRectangle, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XDrawRectangle
#endif
XDrawRectangle:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+992(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+992(%rip)
2:
  pushq $124
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XDrawRectangles
  .p2align 4
  .type XDrawRectangles, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XDrawRectangles
#endif
XDrawRectangles:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1000(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1000(%rip)
2:
  pushq $125
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XDrawSegments
  .p2align 4
  .type XDrawSegments, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XDrawSegments
#endif
XDrawSegments:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1008(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1008(%rip)
2:
  pushq $126
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XDrawString
  .p2align 4
  .type XDrawString, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XDrawString
#endif
XDrawString:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1016(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1016(%rip)
2:
  pushq $127
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XDrawString16
  .p2align 4
  .type XDrawString16, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XDrawString16
#endif
XDrawString16:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1024(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1024(%rip)
2:
  pushq $128
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XDrawText
  .p2align 4
  .type XDrawText, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XDrawText
#endif
XDrawText:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1032(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1032(%rip)
2:
  pushq $129
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XDrawText16
  .p2align 4
  .type XDrawText16, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XDrawText16
#endif
XDrawText16:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1040(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1040(%rip)
2:
  pushq $130
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XEHeadOfExtensionList
  .p2align 4
  .type XEHeadOfExtensionList, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XEHeadOfExtensionList
#endif
XEHeadOfExtensionList:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1048(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1048(%rip)
2:
  pushq $131
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XESetBeforeFlush
  .p2align 4
  .type XESetBeforeFlush, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XESetBeforeFlush
#endif
XESetBeforeFlush:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1056(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1056(%rip)
2:
  pushq $132
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XESetCloseDisplay
  .p2align 4
  .type XESetCloseDisplay, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XESetCloseDisplay
#endif
XESetCloseDisplay:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1064(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1064(%rip)
2:
  pushq $133
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XESetCopyEventCookie
  .p2align 4
  .type XESetCopyEventCookie, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XESetCopyEventCookie
#endif
XESetCopyEventCookie:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1072(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1072(%rip)
2:
  pushq $134
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XESetCopyGC
  .p2align 4
  .type XESetCopyGC, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XESetCopyGC
#endif
XESetCopyGC:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1080(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1080(%rip)
2:
  pushq $135
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XESetCreateFont
  .p2align 4
  .type XESetCreateFont, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XESetCreateFont
#endif
XESetCreateFont:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1088(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1088(%rip)
2:
  pushq $136
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XESetCreateGC
  .p2align 4
  .type XESetCreateGC, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XESetCreateGC
#endif
XESetCreateGC:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1096(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1096(%rip)
2:
  pushq $137
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XESetError
  .p2align 4
  .type XESetError, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XESetError
#endif
XESetError:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1104(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1104(%rip)
2:
  pushq $138
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XESetErrorString
  .p2align 4
  .type XESetErrorString, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XESetErrorString
#endif
XESetErrorString:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1112(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1112(%rip)
2:
  pushq $139
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XESetEventToWire
  .p2align 4
  .type XESetEventToWire, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XESetEventToWire
#endif
XESetEventToWire:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1120(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1120(%rip)
2:
  pushq $140
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XESetFlushGC
  .p2align 4
  .type XESetFlushGC, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XESetFlushGC
#endif
XESetFlushGC:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1128(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1128(%rip)
2:
  pushq $141
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XESetFreeFont
  .p2align 4
  .type XESetFreeFont, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XESetFreeFont
#endif
XESetFreeFont:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1136(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1136(%rip)
2:
  pushq $142
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XESetFreeGC
  .p2align 4
  .type XESetFreeGC, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XESetFreeGC
#endif
XESetFreeGC:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1144(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1144(%rip)
2:
  pushq $143
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XESetPrintErrorValues
  .p2align 4
  .type XESetPrintErrorValues, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XESetPrintErrorValues
#endif
XESetPrintErrorValues:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1152(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1152(%rip)
2:
  pushq $144
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XESetWireToError
  .p2align 4
  .type XESetWireToError, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XESetWireToError
#endif
XESetWireToError:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1160(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1160(%rip)
2:
  pushq $145
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XESetWireToEvent
  .p2align 4
  .type XESetWireToEvent, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XESetWireToEvent
#endif
XESetWireToEvent:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1168(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1168(%rip)
2:
  pushq $146
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XESetWireToEventCookie
  .p2align 4
  .type XESetWireToEventCookie, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XESetWireToEventCookie
#endif
XESetWireToEventCookie:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1176(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1176(%rip)
2:
  pushq $147
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XEmptyRegion
  .p2align 4
  .type XEmptyRegion, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XEmptyRegion
#endif
XEmptyRegion:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1184(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1184(%rip)
2:
  pushq $148
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XEnableAccessControl
  .p2align 4
  .type XEnableAccessControl, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XEnableAccessControl
#endif
XEnableAccessControl:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1192(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1192(%rip)
2:
  pushq $149
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XEqualRegion
  .p2align 4
  .type XEqualRegion, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XEqualRegion
#endif
XEqualRegion:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1200(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1200(%rip)
2:
  pushq $150
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XEventMaskOfScreen
  .p2align 4
  .type XEventMaskOfScreen, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XEventMaskOfScreen
#endif
XEventMaskOfScreen:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1208(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1208(%rip)
2:
  pushq $151
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XEventsQueued
  .p2align 4
  .type XEventsQueued, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XEventsQueued
#endif
XEventsQueued:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1216(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1216(%rip)
2:
  pushq $152
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XExtendedMaxRequestSize
  .p2align 4
  .type XExtendedMaxRequestSize, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XExtendedMaxRequestSize
#endif
XExtendedMaxRequestSize:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1224(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1224(%rip)
2:
  pushq $153
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XExtentsOfFontSet
  .p2align 4
  .type XExtentsOfFontSet, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XExtentsOfFontSet
#endif
XExtentsOfFontSet:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1232(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1232(%rip)
2:
  pushq $154
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XFetchBuffer
  .p2align 4
  .type XFetchBuffer, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XFetchBuffer
#endif
XFetchBuffer:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1240(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1240(%rip)
2:
  pushq $155
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XFetchBytes
  .p2align 4
  .type XFetchBytes, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XFetchBytes
#endif
XFetchBytes:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1248(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1248(%rip)
2:
  pushq $156
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XFetchName
  .p2align 4
  .type XFetchName, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XFetchName
#endif
XFetchName:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1256(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1256(%rip)
2:
  pushq $157
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XFillArc
  .p2align 4
  .type XFillArc, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XFillArc
#endif
XFillArc:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1264(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1264(%rip)
2:
  pushq $158
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XFillArcs
  .p2align 4
  .type XFillArcs, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XFillArcs
#endif
XFillArcs:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1272(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1272(%rip)
2:
  pushq $159
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XFillPolygon
  .p2align 4
  .type XFillPolygon, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XFillPolygon
#endif
XFillPolygon:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1280(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1280(%rip)
2:
  pushq $160
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XFillRectangle
  .p2align 4
  .type XFillRectangle, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XFillRectangle
#endif
XFillRectangle:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1288(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1288(%rip)
2:
  pushq $161
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XFillRectangles
  .p2align 4
  .type XFillRectangles, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XFillRectangles
#endif
XFillRectangles:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1296(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1296(%rip)
2:
  pushq $162
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XFilterEvent
  .p2align 4
  .type XFilterEvent, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XFilterEvent
#endif
XFilterEvent:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1304(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1304(%rip)
2:
  pushq $163
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XFindContext
  .p2align 4
  .type XFindContext, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XFindContext
#endif
XFindContext:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1312(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1312(%rip)
2:
  pushq $164
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XFindOnExtensionList
  .p2align 4
  .type XFindOnExtensionList, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XFindOnExtensionList
#endif
XFindOnExtensionList:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1320(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1320(%rip)
2:
  pushq $165
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XFlush
  .p2align 4
  .type XFlush, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XFlush
#endif
XFlush:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1328(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1328(%rip)
2:
  pushq $166
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XFlushGC
  .p2align 4
  .type XFlushGC, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XFlushGC
#endif
XFlushGC:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1336(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1336(%rip)
2:
  pushq $167
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XFontsOfFontSet
  .p2align 4
  .type XFontsOfFontSet, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XFontsOfFontSet
#endif
XFontsOfFontSet:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1344(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1344(%rip)
2:
  pushq $168
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XForceScreenSaver
  .p2align 4
  .type XForceScreenSaver, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XForceScreenSaver
#endif
XForceScreenSaver:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1352(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1352(%rip)
2:
  pushq $169
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XFree
  .p2align 4
  .type XFree, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XFree
#endif
XFree:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1360(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1360(%rip)
2:
  pushq $170
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XFreeColormap
  .p2align 4
  .type XFreeColormap, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XFreeColormap
#endif
XFreeColormap:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1368(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1368(%rip)
2:
  pushq $171
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XFreeColors
  .p2align 4
  .type XFreeColors, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XFreeColors
#endif
XFreeColors:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1376(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1376(%rip)
2:
  pushq $172
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XFreeCursor
  .p2align 4
  .type XFreeCursor, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XFreeCursor
#endif
XFreeCursor:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1384(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1384(%rip)
2:
  pushq $173
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XFreeEventData
  .p2align 4
  .type XFreeEventData, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XFreeEventData
#endif
XFreeEventData:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1392(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1392(%rip)
2:
  pushq $174
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XFreeExtensionList
  .p2align 4
  .type XFreeExtensionList, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XFreeExtensionList
#endif
XFreeExtensionList:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1400(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1400(%rip)
2:
  pushq $175
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XFreeFont
  .p2align 4
  .type XFreeFont, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XFreeFont
#endif
XFreeFont:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1408(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1408(%rip)
2:
  pushq $176
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XFreeFontInfo
  .p2align 4
  .type XFreeFontInfo, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XFreeFontInfo
#endif
XFreeFontInfo:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1416(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1416(%rip)
2:
  pushq $177
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XFreeFontNames
  .p2align 4
  .type XFreeFontNames, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XFreeFontNames
#endif
XFreeFontNames:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1424(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1424(%rip)
2:
  pushq $178
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XFreeFontPath
  .p2align 4
  .type XFreeFontPath, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XFreeFontPath
#endif
XFreeFontPath:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1432(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1432(%rip)
2:
  pushq $179
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XFreeFontSet
  .p2align 4
  .type XFreeFontSet, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XFreeFontSet
#endif
XFreeFontSet:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1440(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1440(%rip)
2:
  pushq $180
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XFreeGC
  .p2align 4
  .type XFreeGC, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XFreeGC
#endif
XFreeGC:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1448(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1448(%rip)
2:
  pushq $181
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XFreeModifiermap
  .p2align 4
  .type XFreeModifiermap, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XFreeModifiermap
#endif
XFreeModifiermap:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1456(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1456(%rip)
2:
  pushq $182
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XFreePixmap
  .p2align 4
  .type XFreePixmap, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XFreePixmap
#endif
XFreePixmap:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1464(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1464(%rip)
2:
  pushq $183
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XFreeStringList
  .p2align 4
  .type XFreeStringList, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XFreeStringList
#endif
XFreeStringList:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1472(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1472(%rip)
2:
  pushq $184
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XFreeThreads
  .p2align 4
  .type XFreeThreads, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XFreeThreads
#endif
XFreeThreads:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1480(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1480(%rip)
2:
  pushq $185
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XGContextFromGC
  .p2align 4
  .type XGContextFromGC, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XGContextFromGC
#endif
XGContextFromGC:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1488(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1488(%rip)
2:
  pushq $186
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XGeometry
  .p2align 4
  .type XGeometry, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XGeometry
#endif
XGeometry:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1496(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1496(%rip)
2:
  pushq $187
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XGetAtomName
  .p2align 4
  .type XGetAtomName, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XGetAtomName
#endif
XGetAtomName:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1504(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1504(%rip)
2:
  pushq $188
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XGetAtomNames
  .p2align 4
  .type XGetAtomNames, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XGetAtomNames
#endif
XGetAtomNames:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1512(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1512(%rip)
2:
  pushq $189
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XGetClassHint
  .p2align 4
  .type XGetClassHint, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XGetClassHint
#endif
XGetClassHint:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1520(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1520(%rip)
2:
  pushq $190
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XGetCommand
  .p2align 4
  .type XGetCommand, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XGetCommand
#endif
XGetCommand:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1528(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1528(%rip)
2:
  pushq $191
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XGetDefault
  .p2align 4
  .type XGetDefault, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XGetDefault
#endif
XGetDefault:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1536(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1536(%rip)
2:
  pushq $192
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XGetErrorDatabaseText
  .p2align 4
  .type XGetErrorDatabaseText, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XGetErrorDatabaseText
#endif
XGetErrorDatabaseText:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1544(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1544(%rip)
2:
  pushq $193
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XGetErrorText
  .p2align 4
  .type XGetErrorText, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XGetErrorText
#endif
XGetErrorText:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1552(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1552(%rip)
2:
  pushq $194
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XGetEventData
  .p2align 4
  .type XGetEventData, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XGetEventData
#endif
XGetEventData:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1560(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1560(%rip)
2:
  pushq $195
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XGetFontPath
  .p2align 4
  .type XGetFontPath, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XGetFontPath
#endif
XGetFontPath:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1568(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1568(%rip)
2:
  pushq $196
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XGetFontProperty
  .p2align 4
  .type XGetFontProperty, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XGetFontProperty
#endif
XGetFontProperty:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1576(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1576(%rip)
2:
  pushq $197
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XGetGCValues
  .p2align 4
  .type XGetGCValues, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XGetGCValues
#endif
XGetGCValues:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1584(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1584(%rip)
2:
  pushq $198
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XGetGeometry
  .p2align 4
  .type XGetGeometry, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XGetGeometry
#endif
XGetGeometry:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1592(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1592(%rip)
2:
  pushq $199
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XGetICValues
  .p2align 4
  .type XGetICValues, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XGetICValues
#endif
XGetICValues:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1600(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1600(%rip)
2:
  pushq $200
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XGetIMValues
  .p2align 4
  .type XGetIMValues, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XGetIMValues
#endif
XGetIMValues:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1608(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1608(%rip)
2:
  pushq $201
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XGetIconName
  .p2align 4
  .type XGetIconName, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XGetIconName
#endif
XGetIconName:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1616(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1616(%rip)
2:
  pushq $202
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XGetIconSizes
  .p2align 4
  .type XGetIconSizes, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XGetIconSizes
#endif
XGetIconSizes:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1624(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1624(%rip)
2:
  pushq $203
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XGetImage
  .p2align 4
  .type XGetImage, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XGetImage
#endif
XGetImage:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1632(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1632(%rip)
2:
  pushq $204
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XGetInputFocus
  .p2align 4
  .type XGetInputFocus, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XGetInputFocus
#endif
XGetInputFocus:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1640(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1640(%rip)
2:
  pushq $205
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XGetKeyboardControl
  .p2align 4
  .type XGetKeyboardControl, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XGetKeyboardControl
#endif
XGetKeyboardControl:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1648(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1648(%rip)
2:
  pushq $206
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XGetKeyboardMapping
  .p2align 4
  .type XGetKeyboardMapping, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XGetKeyboardMapping
#endif
XGetKeyboardMapping:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1656(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1656(%rip)
2:
  pushq $207
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XGetModifierMapping
  .p2align 4
  .type XGetModifierMapping, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XGetModifierMapping
#endif
XGetModifierMapping:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1664(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1664(%rip)
2:
  pushq $208
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XGetMotionEvents
  .p2align 4
  .type XGetMotionEvents, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XGetMotionEvents
#endif
XGetMotionEvents:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1672(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1672(%rip)
2:
  pushq $209
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XGetNormalHints
  .p2align 4
  .type XGetNormalHints, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XGetNormalHints
#endif
XGetNormalHints:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1680(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1680(%rip)
2:
  pushq $210
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XGetOCValues
  .p2align 4
  .type XGetOCValues, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XGetOCValues
#endif
XGetOCValues:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1688(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1688(%rip)
2:
  pushq $211
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XGetOMValues
  .p2align 4
  .type XGetOMValues, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XGetOMValues
#endif
XGetOMValues:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1696(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1696(%rip)
2:
  pushq $212
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XGetPixel
  .p2align 4
  .type XGetPixel, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XGetPixel
#endif
XGetPixel:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1704(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1704(%rip)
2:
  pushq $213
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XGetPointerControl
  .p2align 4
  .type XGetPointerControl, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XGetPointerControl
#endif
XGetPointerControl:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1712(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1712(%rip)
2:
  pushq $214
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XGetPointerMapping
  .p2align 4
  .type XGetPointerMapping, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XGetPointerMapping
#endif
XGetPointerMapping:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1720(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1720(%rip)
2:
  pushq $215
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XGetRGBColormaps
  .p2align 4
  .type XGetRGBColormaps, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XGetRGBColormaps
#endif
XGetRGBColormaps:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1728(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1728(%rip)
2:
  pushq $216
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XGetScreenSaver
  .p2align 4
  .type XGetScreenSaver, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XGetScreenSaver
#endif
XGetScreenSaver:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1736(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1736(%rip)
2:
  pushq $217
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XGetSelectionOwner
  .p2align 4
  .type XGetSelectionOwner, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XGetSelectionOwner
#endif
XGetSelectionOwner:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1744(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1744(%rip)
2:
  pushq $218
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XGetSizeHints
  .p2align 4
  .type XGetSizeHints, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XGetSizeHints
#endif
XGetSizeHints:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1752(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1752(%rip)
2:
  pushq $219
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XGetStandardColormap
  .p2align 4
  .type XGetStandardColormap, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XGetStandardColormap
#endif
XGetStandardColormap:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1760(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1760(%rip)
2:
  pushq $220
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XGetSubImage
  .p2align 4
  .type XGetSubImage, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XGetSubImage
#endif
XGetSubImage:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1768(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1768(%rip)
2:
  pushq $221
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XGetTextProperty
  .p2align 4
  .type XGetTextProperty, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XGetTextProperty
#endif
XGetTextProperty:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1776(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1776(%rip)
2:
  pushq $222
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XGetTransientForHint
  .p2align 4
  .type XGetTransientForHint, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XGetTransientForHint
#endif
XGetTransientForHint:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1784(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1784(%rip)
2:
  pushq $223
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XGetVisualInfo
  .p2align 4
  .type XGetVisualInfo, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XGetVisualInfo
#endif
XGetVisualInfo:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1792(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1792(%rip)
2:
  pushq $224
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XGetWMClientMachine
  .p2align 4
  .type XGetWMClientMachine, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XGetWMClientMachine
#endif
XGetWMClientMachine:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1800(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1800(%rip)
2:
  pushq $225
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XGetWMColormapWindows
  .p2align 4
  .type XGetWMColormapWindows, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XGetWMColormapWindows
#endif
XGetWMColormapWindows:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1808(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1808(%rip)
2:
  pushq $226
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XGetWMHints
  .p2align 4
  .type XGetWMHints, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XGetWMHints
#endif
XGetWMHints:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1816(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1816(%rip)
2:
  pushq $227
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XGetWMIconName
  .p2align 4
  .type XGetWMIconName, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XGetWMIconName
#endif
XGetWMIconName:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1824(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1824(%rip)
2:
  pushq $228
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XGetWMName
  .p2align 4
  .type XGetWMName, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XGetWMName
#endif
XGetWMName:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1832(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1832(%rip)
2:
  pushq $229
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XGetWMNormalHints
  .p2align 4
  .type XGetWMNormalHints, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XGetWMNormalHints
#endif
XGetWMNormalHints:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1840(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1840(%rip)
2:
  pushq $230
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XGetWMProtocols
  .p2align 4
  .type XGetWMProtocols, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XGetWMProtocols
#endif
XGetWMProtocols:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1848(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1848(%rip)
2:
  pushq $231
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XGetWMSizeHints
  .p2align 4
  .type XGetWMSizeHints, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XGetWMSizeHints
#endif
XGetWMSizeHints:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1856(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1856(%rip)
2:
  pushq $232
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XGetWindowAttributes
  .p2align 4
  .type XGetWindowAttributes, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XGetWindowAttributes
#endif
XGetWindowAttributes:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1864(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1864(%rip)
2:
  pushq $233
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XGetWindowProperty
  .p2align 4
  .type XGetWindowProperty, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XGetWindowProperty
#endif
XGetWindowProperty:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1872(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1872(%rip)
2:
  pushq $234
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XGetZoomHints
  .p2align 4
  .type XGetZoomHints, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XGetZoomHints
#endif
XGetZoomHints:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1880(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1880(%rip)
2:
  pushq $235
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XGrabButton
  .p2align 4
  .type XGrabButton, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XGrabButton
#endif
XGrabButton:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1888(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1888(%rip)
2:
  pushq $236
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XGrabKey
  .p2align 4
  .type XGrabKey, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XGrabKey
#endif
XGrabKey:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1896(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1896(%rip)
2:
  pushq $237
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XGrabKeyboard
  .p2align 4
  .type XGrabKeyboard, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XGrabKeyboard
#endif
XGrabKeyboard:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1904(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1904(%rip)
2:
  pushq $238
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XGrabPointer
  .p2align 4
  .type XGrabPointer, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XGrabPointer
#endif
XGrabPointer:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1912(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1912(%rip)
2:
  pushq $239
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XGrabServer
  .p2align 4
  .type XGrabServer, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XGrabServer
#endif
XGrabServer:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1920(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1920(%rip)
2:
  pushq $240
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XHeightMMOfScreen
  .p2align 4
  .type XHeightMMOfScreen, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XHeightMMOfScreen
#endif
XHeightMMOfScreen:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1928(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1928(%rip)
2:
  pushq $241
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XHeightOfScreen
  .p2align 4
  .type XHeightOfScreen, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XHeightOfScreen
#endif
XHeightOfScreen:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1936(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1936(%rip)
2:
  pushq $242
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XIMOfIC
  .p2align 4
  .type XIMOfIC, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XIMOfIC
#endif
XIMOfIC:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1944(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1944(%rip)
2:
  pushq $243
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XIconifyWindow
  .p2align 4
  .type XIconifyWindow, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XIconifyWindow
#endif
XIconifyWindow:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1952(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1952(%rip)
2:
  pushq $244
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XIfEvent
  .p2align 4
  .type XIfEvent, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XIfEvent
#endif
XIfEvent:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1960(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1960(%rip)
2:
  pushq $245
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XImageByteOrder
  .p2align 4
  .type XImageByteOrder, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XImageByteOrder
#endif
XImageByteOrder:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1968(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1968(%rip)
2:
  pushq $246
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XInitExtension
  .p2align 4
  .type XInitExtension, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XInitExtension
#endif
XInitExtension:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1976(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1976(%rip)
2:
  pushq $247
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XInitImage
  .p2align 4
  .type XInitImage, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XInitImage
#endif
XInitImage:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1984(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1984(%rip)
2:
  pushq $248
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XInitThreads
  .p2align 4
  .type XInitThreads, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XInitThreads
#endif
XInitThreads:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+1992(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+1992(%rip)
2:
  pushq $249
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XInsertModifiermapEntry
  .p2align 4
  .type XInsertModifiermapEntry, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XInsertModifiermapEntry
#endif
XInsertModifiermapEntry:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2000(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2000(%rip)
2:
  pushq $250
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XInstallColormap
  .p2align 4
  .type XInstallColormap, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XInstallColormap
#endif
XInstallColormap:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2008(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2008(%rip)
2:
  pushq $251
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XInternAtom
  .p2align 4
  .type XInternAtom, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XInternAtom
#endif
XInternAtom:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2016(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2016(%rip)
2:
  pushq $252
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XInternAtoms
  .p2align 4
  .type XInternAtoms, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XInternAtoms
#endif
XInternAtoms:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2024(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2024(%rip)
2:
  pushq $253
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XInternalConnectionNumbers
  .p2align 4
  .type XInternalConnectionNumbers, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XInternalConnectionNumbers
#endif
XInternalConnectionNumbers:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2032(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2032(%rip)
2:
  pushq $254
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XIntersectRegion
  .p2align 4
  .type XIntersectRegion, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XIntersectRegion
#endif
XIntersectRegion:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2040(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2040(%rip)
2:
  pushq $255
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XKeycodeToKeysym
  .p2align 4
  .type XKeycodeToKeysym, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XKeycodeToKeysym
#endif
XKeycodeToKeysym:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2048(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2048(%rip)
2:
  pushq $256
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XKeysymToKeycode
  .p2align 4
  .type XKeysymToKeycode, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XKeysymToKeycode
#endif
XKeysymToKeycode:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2056(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2056(%rip)
2:
  pushq $257
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XKeysymToString
  .p2align 4
  .type XKeysymToString, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XKeysymToString
#endif
XKeysymToString:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2064(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2064(%rip)
2:
  pushq $258
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XKillClient
  .p2align 4
  .type XKillClient, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XKillClient
#endif
XKillClient:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2072(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2072(%rip)
2:
  pushq $259
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XLastKnownRequestProcessed
  .p2align 4
  .type XLastKnownRequestProcessed, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XLastKnownRequestProcessed
#endif
XLastKnownRequestProcessed:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2080(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2080(%rip)
2:
  pushq $260
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XListDepths
  .p2align 4
  .type XListDepths, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XListDepths
#endif
XListDepths:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2088(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2088(%rip)
2:
  pushq $261
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XListExtensions
  .p2align 4
  .type XListExtensions, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XListExtensions
#endif
XListExtensions:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2096(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2096(%rip)
2:
  pushq $262
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XListFonts
  .p2align 4
  .type XListFonts, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XListFonts
#endif
XListFonts:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2104(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2104(%rip)
2:
  pushq $263
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XListFontsWithInfo
  .p2align 4
  .type XListFontsWithInfo, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XListFontsWithInfo
#endif
XListFontsWithInfo:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2112(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2112(%rip)
2:
  pushq $264
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XListHosts
  .p2align 4
  .type XListHosts, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XListHosts
#endif
XListHosts:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2120(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2120(%rip)
2:
  pushq $265
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XListInstalledColormaps
  .p2align 4
  .type XListInstalledColormaps, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XListInstalledColormaps
#endif
XListInstalledColormaps:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2128(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2128(%rip)
2:
  pushq $266
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XListPixmapFormats
  .p2align 4
  .type XListPixmapFormats, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XListPixmapFormats
#endif
XListPixmapFormats:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2136(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2136(%rip)
2:
  pushq $267
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XListProperties
  .p2align 4
  .type XListProperties, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XListProperties
#endif
XListProperties:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2144(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2144(%rip)
2:
  pushq $268
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XLoadFont
  .p2align 4
  .type XLoadFont, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XLoadFont
#endif
XLoadFont:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2152(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2152(%rip)
2:
  pushq $269
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XLoadQueryFont
  .p2align 4
  .type XLoadQueryFont, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XLoadQueryFont
#endif
XLoadQueryFont:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2160(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2160(%rip)
2:
  pushq $270
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XLocaleOfFontSet
  .p2align 4
  .type XLocaleOfFontSet, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XLocaleOfFontSet
#endif
XLocaleOfFontSet:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2168(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2168(%rip)
2:
  pushq $271
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XLocaleOfIM
  .p2align 4
  .type XLocaleOfIM, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XLocaleOfIM
#endif
XLocaleOfIM:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2176(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2176(%rip)
2:
  pushq $272
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XLocaleOfOM
  .p2align 4
  .type XLocaleOfOM, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XLocaleOfOM
#endif
XLocaleOfOM:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2184(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2184(%rip)
2:
  pushq $273
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XLockDisplay
  .p2align 4
  .type XLockDisplay, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XLockDisplay
#endif
XLockDisplay:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2192(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2192(%rip)
2:
  pushq $274
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XLookupColor
  .p2align 4
  .type XLookupColor, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XLookupColor
#endif
XLookupColor:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2200(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2200(%rip)
2:
  pushq $275
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XLookupKeysym
  .p2align 4
  .type XLookupKeysym, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XLookupKeysym
#endif
XLookupKeysym:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2208(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2208(%rip)
2:
  pushq $276
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XLookupString
  .p2align 4
  .type XLookupString, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XLookupString
#endif
XLookupString:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2216(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2216(%rip)
2:
  pushq $277
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XLowerWindow
  .p2align 4
  .type XLowerWindow, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XLowerWindow
#endif
XLowerWindow:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2224(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2224(%rip)
2:
  pushq $278
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XMapRaised
  .p2align 4
  .type XMapRaised, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XMapRaised
#endif
XMapRaised:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2232(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2232(%rip)
2:
  pushq $279
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XMapSubwindows
  .p2align 4
  .type XMapSubwindows, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XMapSubwindows
#endif
XMapSubwindows:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2240(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2240(%rip)
2:
  pushq $280
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XMapWindow
  .p2align 4
  .type XMapWindow, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XMapWindow
#endif
XMapWindow:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2248(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2248(%rip)
2:
  pushq $281
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XMaskEvent
  .p2align 4
  .type XMaskEvent, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XMaskEvent
#endif
XMaskEvent:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2256(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2256(%rip)
2:
  pushq $282
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XMatchVisualInfo
  .p2align 4
  .type XMatchVisualInfo, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XMatchVisualInfo
#endif
XMatchVisualInfo:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2264(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2264(%rip)
2:
  pushq $283
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XMaxCmapsOfScreen
  .p2align 4
  .type XMaxCmapsOfScreen, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XMaxCmapsOfScreen
#endif
XMaxCmapsOfScreen:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2272(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2272(%rip)
2:
  pushq $284
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XMaxRequestSize
  .p2align 4
  .type XMaxRequestSize, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XMaxRequestSize
#endif
XMaxRequestSize:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2280(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2280(%rip)
2:
  pushq $285
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XMinCmapsOfScreen
  .p2align 4
  .type XMinCmapsOfScreen, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XMinCmapsOfScreen
#endif
XMinCmapsOfScreen:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2288(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2288(%rip)
2:
  pushq $286
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XMoveResizeWindow
  .p2align 4
  .type XMoveResizeWindow, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XMoveResizeWindow
#endif
XMoveResizeWindow:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2296(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2296(%rip)
2:
  pushq $287
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XMoveWindow
  .p2align 4
  .type XMoveWindow, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XMoveWindow
#endif
XMoveWindow:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2304(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2304(%rip)
2:
  pushq $288
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XNewModifiermap
  .p2align 4
  .type XNewModifiermap, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XNewModifiermap
#endif
XNewModifiermap:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2312(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2312(%rip)
2:
  pushq $289
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XNextEvent
  .p2align 4
  .type XNextEvent, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XNextEvent
#endif
XNextEvent:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2320(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2320(%rip)
2:
  pushq $290
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XNextRequest
  .p2align 4
  .type XNextRequest, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XNextRequest
#endif
XNextRequest:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2328(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2328(%rip)
2:
  pushq $291
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XNoOp
  .p2align 4
  .type XNoOp, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XNoOp
#endif
XNoOp:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2336(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2336(%rip)
2:
  pushq $292
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XOMOfOC
  .p2align 4
  .type XOMOfOC, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XOMOfOC
#endif
XOMOfOC:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2344(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2344(%rip)
2:
  pushq $293
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XOffsetRegion
  .p2align 4
  .type XOffsetRegion, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XOffsetRegion
#endif
XOffsetRegion:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2352(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2352(%rip)
2:
  pushq $294
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XOpenDisplay
  .p2align 4
  .type XOpenDisplay, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XOpenDisplay
#endif
XOpenDisplay:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2360(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2360(%rip)
2:
  pushq $295
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XOpenIM
  .p2align 4
  .type XOpenIM, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XOpenIM
#endif
XOpenIM:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2368(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2368(%rip)
2:
  pushq $296
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XOpenOM
  .p2align 4
  .type XOpenOM, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XOpenOM
#endif
XOpenOM:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2376(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2376(%rip)
2:
  pushq $297
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XParseColor
  .p2align 4
  .type XParseColor, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XParseColor
#endif
XParseColor:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2384(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2384(%rip)
2:
  pushq $298
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XParseGeometry
  .p2align 4
  .type XParseGeometry, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XParseGeometry
#endif
XParseGeometry:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2392(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2392(%rip)
2:
  pushq $299
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XPeekEvent
  .p2align 4
  .type XPeekEvent, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XPeekEvent
#endif
XPeekEvent:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2400(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2400(%rip)
2:
  pushq $300
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XPeekIfEvent
  .p2align 4
  .type XPeekIfEvent, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XPeekIfEvent
#endif
XPeekIfEvent:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2408(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2408(%rip)
2:
  pushq $301
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XPending
  .p2align 4
  .type XPending, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XPending
#endif
XPending:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2416(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2416(%rip)
2:
  pushq $302
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XPlanesOfScreen
  .p2align 4
  .type XPlanesOfScreen, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XPlanesOfScreen
#endif
XPlanesOfScreen:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2424(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2424(%rip)
2:
  pushq $303
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XPointInRegion
  .p2align 4
  .type XPointInRegion, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XPointInRegion
#endif
XPointInRegion:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2432(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2432(%rip)
2:
  pushq $304
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XPolygonRegion
  .p2align 4
  .type XPolygonRegion, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XPolygonRegion
#endif
XPolygonRegion:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2440(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2440(%rip)
2:
  pushq $305
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XProcessInternalConnection
  .p2align 4
  .type XProcessInternalConnection, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XProcessInternalConnection
#endif
XProcessInternalConnection:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2448(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2448(%rip)
2:
  pushq $306
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XProtocolRevision
  .p2align 4
  .type XProtocolRevision, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XProtocolRevision
#endif
XProtocolRevision:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2456(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2456(%rip)
2:
  pushq $307
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XProtocolVersion
  .p2align 4
  .type XProtocolVersion, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XProtocolVersion
#endif
XProtocolVersion:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2464(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2464(%rip)
2:
  pushq $308
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XPutBackEvent
  .p2align 4
  .type XPutBackEvent, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XPutBackEvent
#endif
XPutBackEvent:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2472(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2472(%rip)
2:
  pushq $309
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XPutImage
  .p2align 4
  .type XPutImage, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XPutImage
#endif
XPutImage:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2480(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2480(%rip)
2:
  pushq $310
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XPutPixel
  .p2align 4
  .type XPutPixel, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XPutPixel
#endif
XPutPixel:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2488(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2488(%rip)
2:
  pushq $311
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XQLength
  .p2align 4
  .type XQLength, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XQLength
#endif
XQLength:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2496(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2496(%rip)
2:
  pushq $312
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XQueryBestCursor
  .p2align 4
  .type XQueryBestCursor, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XQueryBestCursor
#endif
XQueryBestCursor:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2504(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2504(%rip)
2:
  pushq $313
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XQueryBestSize
  .p2align 4
  .type XQueryBestSize, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XQueryBestSize
#endif
XQueryBestSize:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2512(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2512(%rip)
2:
  pushq $314
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XQueryBestStipple
  .p2align 4
  .type XQueryBestStipple, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XQueryBestStipple
#endif
XQueryBestStipple:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2520(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2520(%rip)
2:
  pushq $315
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XQueryBestTile
  .p2align 4
  .type XQueryBestTile, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XQueryBestTile
#endif
XQueryBestTile:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2528(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2528(%rip)
2:
  pushq $316
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XQueryColor
  .p2align 4
  .type XQueryColor, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XQueryColor
#endif
XQueryColor:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2536(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2536(%rip)
2:
  pushq $317
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XQueryColors
  .p2align 4
  .type XQueryColors, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XQueryColors
#endif
XQueryColors:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2544(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2544(%rip)
2:
  pushq $318
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XQueryExtension
  .p2align 4
  .type XQueryExtension, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XQueryExtension
#endif
XQueryExtension:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2552(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2552(%rip)
2:
  pushq $319
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XQueryFont
  .p2align 4
  .type XQueryFont, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XQueryFont
#endif
XQueryFont:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2560(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2560(%rip)
2:
  pushq $320
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XQueryKeymap
  .p2align 4
  .type XQueryKeymap, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XQueryKeymap
#endif
XQueryKeymap:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2568(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2568(%rip)
2:
  pushq $321
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XQueryPointer
  .p2align 4
  .type XQueryPointer, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XQueryPointer
#endif
XQueryPointer:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2576(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2576(%rip)
2:
  pushq $322
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XQueryTextExtents
  .p2align 4
  .type XQueryTextExtents, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XQueryTextExtents
#endif
XQueryTextExtents:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2584(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2584(%rip)
2:
  pushq $323
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XQueryTextExtents16
  .p2align 4
  .type XQueryTextExtents16, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XQueryTextExtents16
#endif
XQueryTextExtents16:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2592(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2592(%rip)
2:
  pushq $324
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XQueryTree
  .p2align 4
  .type XQueryTree, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XQueryTree
#endif
XQueryTree:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2600(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2600(%rip)
2:
  pushq $325
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XRaiseWindow
  .p2align 4
  .type XRaiseWindow, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XRaiseWindow
#endif
XRaiseWindow:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2608(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2608(%rip)
2:
  pushq $326
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XReadBitmapFile
  .p2align 4
  .type XReadBitmapFile, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XReadBitmapFile
#endif
XReadBitmapFile:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2616(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2616(%rip)
2:
  pushq $327
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XReadBitmapFileData
  .p2align 4
  .type XReadBitmapFileData, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XReadBitmapFileData
#endif
XReadBitmapFileData:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2624(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2624(%rip)
2:
  pushq $328
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XRebindKeysym
  .p2align 4
  .type XRebindKeysym, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XRebindKeysym
#endif
XRebindKeysym:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2632(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2632(%rip)
2:
  pushq $329
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XRecolorCursor
  .p2align 4
  .type XRecolorCursor, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XRecolorCursor
#endif
XRecolorCursor:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2640(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2640(%rip)
2:
  pushq $330
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XReconfigureWMWindow
  .p2align 4
  .type XReconfigureWMWindow, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XReconfigureWMWindow
#endif
XReconfigureWMWindow:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2648(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2648(%rip)
2:
  pushq $331
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XRectInRegion
  .p2align 4
  .type XRectInRegion, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XRectInRegion
#endif
XRectInRegion:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2656(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2656(%rip)
2:
  pushq $332
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XRefreshKeyboardMapping
  .p2align 4
  .type XRefreshKeyboardMapping, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XRefreshKeyboardMapping
#endif
XRefreshKeyboardMapping:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2664(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2664(%rip)
2:
  pushq $333
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XRegisterIMInstantiateCallback
  .p2align 4
  .type XRegisterIMInstantiateCallback, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XRegisterIMInstantiateCallback
#endif
XRegisterIMInstantiateCallback:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2672(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2672(%rip)
2:
  pushq $334
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XRemoveConnectionWatch
  .p2align 4
  .type XRemoveConnectionWatch, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XRemoveConnectionWatch
#endif
XRemoveConnectionWatch:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2680(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2680(%rip)
2:
  pushq $335
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XRemoveFromSaveSet
  .p2align 4
  .type XRemoveFromSaveSet, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XRemoveFromSaveSet
#endif
XRemoveFromSaveSet:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2688(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2688(%rip)
2:
  pushq $336
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XRemoveHost
  .p2align 4
  .type XRemoveHost, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XRemoveHost
#endif
XRemoveHost:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2696(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2696(%rip)
2:
  pushq $337
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XRemoveHosts
  .p2align 4
  .type XRemoveHosts, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XRemoveHosts
#endif
XRemoveHosts:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2704(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2704(%rip)
2:
  pushq $338
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XReparentWindow
  .p2align 4
  .type XReparentWindow, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XReparentWindow
#endif
XReparentWindow:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2712(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2712(%rip)
2:
  pushq $339
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XResetScreenSaver
  .p2align 4
  .type XResetScreenSaver, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XResetScreenSaver
#endif
XResetScreenSaver:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2720(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2720(%rip)
2:
  pushq $340
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XResizeWindow
  .p2align 4
  .type XResizeWindow, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XResizeWindow
#endif
XResizeWindow:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2728(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2728(%rip)
2:
  pushq $341
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XResourceManagerString
  .p2align 4
  .type XResourceManagerString, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XResourceManagerString
#endif
XResourceManagerString:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2736(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2736(%rip)
2:
  pushq $342
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XRestackWindows
  .p2align 4
  .type XRestackWindows, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XRestackWindows
#endif
XRestackWindows:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2744(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2744(%rip)
2:
  pushq $343
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XRootWindow
  .p2align 4
  .type XRootWindow, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XRootWindow
#endif
XRootWindow:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2752(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2752(%rip)
2:
  pushq $344
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XRootWindowOfScreen
  .p2align 4
  .type XRootWindowOfScreen, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XRootWindowOfScreen
#endif
XRootWindowOfScreen:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2760(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2760(%rip)
2:
  pushq $345
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XRotateBuffers
  .p2align 4
  .type XRotateBuffers, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XRotateBuffers
#endif
XRotateBuffers:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2768(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2768(%rip)
2:
  pushq $346
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XRotateWindowProperties
  .p2align 4
  .type XRotateWindowProperties, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XRotateWindowProperties
#endif
XRotateWindowProperties:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2776(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2776(%rip)
2:
  pushq $347
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XSaveContext
  .p2align 4
  .type XSaveContext, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSaveContext
#endif
XSaveContext:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2784(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2784(%rip)
2:
  pushq $348
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XScreenCount
  .p2align 4
  .type XScreenCount, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XScreenCount
#endif
XScreenCount:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2792(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2792(%rip)
2:
  pushq $349
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XScreenNumberOfScreen
  .p2align 4
  .type XScreenNumberOfScreen, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XScreenNumberOfScreen
#endif
XScreenNumberOfScreen:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2800(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2800(%rip)
2:
  pushq $350
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XScreenOfDisplay
  .p2align 4
  .type XScreenOfDisplay, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XScreenOfDisplay
#endif
XScreenOfDisplay:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2808(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2808(%rip)
2:
  pushq $351
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XScreenResourceString
  .p2align 4
  .type XScreenResourceString, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XScreenResourceString
#endif
XScreenResourceString:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2816(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2816(%rip)
2:
  pushq $352
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XSelectInput
  .p2align 4
  .type XSelectInput, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSelectInput
#endif
XSelectInput:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2824(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2824(%rip)
2:
  pushq $353
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XSendEvent
  .p2align 4
  .type XSendEvent, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSendEvent
#endif
XSendEvent:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2832(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2832(%rip)
2:
  pushq $354
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XServerVendor
  .p2align 4
  .type XServerVendor, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XServerVendor
#endif
XServerVendor:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2840(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2840(%rip)
2:
  pushq $355
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XSetAccessControl
  .p2align 4
  .type XSetAccessControl, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSetAccessControl
#endif
XSetAccessControl:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2848(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2848(%rip)
2:
  pushq $356
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XSetAfterFunction
  .p2align 4
  .type XSetAfterFunction, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSetAfterFunction
#endif
XSetAfterFunction:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2856(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2856(%rip)
2:
  pushq $357
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XSetArcMode
  .p2align 4
  .type XSetArcMode, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSetArcMode
#endif
XSetArcMode:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2864(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2864(%rip)
2:
  pushq $358
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XSetAuthorization
  .p2align 4
  .type XSetAuthorization, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSetAuthorization
#endif
XSetAuthorization:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2872(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2872(%rip)
2:
  pushq $359
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XSetBackground
  .p2align 4
  .type XSetBackground, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSetBackground
#endif
XSetBackground:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2880(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2880(%rip)
2:
  pushq $360
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XSetClassHint
  .p2align 4
  .type XSetClassHint, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSetClassHint
#endif
XSetClassHint:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2888(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2888(%rip)
2:
  pushq $361
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XSetClipMask
  .p2align 4
  .type XSetClipMask, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSetClipMask
#endif
XSetClipMask:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2896(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2896(%rip)
2:
  pushq $362
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XSetClipOrigin
  .p2align 4
  .type XSetClipOrigin, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSetClipOrigin
#endif
XSetClipOrigin:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2904(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2904(%rip)
2:
  pushq $363
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XSetClipRectangles
  .p2align 4
  .type XSetClipRectangles, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSetClipRectangles
#endif
XSetClipRectangles:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2912(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2912(%rip)
2:
  pushq $364
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XSetCloseDownMode
  .p2align 4
  .type XSetCloseDownMode, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSetCloseDownMode
#endif
XSetCloseDownMode:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2920(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2920(%rip)
2:
  pushq $365
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XSetCommand
  .p2align 4
  .type XSetCommand, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSetCommand
#endif
XSetCommand:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2928(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2928(%rip)
2:
  pushq $366
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XSetDashes
  .p2align 4
  .type XSetDashes, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSetDashes
#endif
XSetDashes:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2936(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2936(%rip)
2:
  pushq $367
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XSetErrorHandler
  .p2align 4
  .type XSetErrorHandler, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSetErrorHandler
#endif
XSetErrorHandler:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2944(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2944(%rip)
2:
  pushq $368
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XSetFillRule
  .p2align 4
  .type XSetFillRule, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSetFillRule
#endif
XSetFillRule:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2952(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2952(%rip)
2:
  pushq $369
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XSetFillStyle
  .p2align 4
  .type XSetFillStyle, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSetFillStyle
#endif
XSetFillStyle:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2960(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2960(%rip)
2:
  pushq $370
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XSetFont
  .p2align 4
  .type XSetFont, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSetFont
#endif
XSetFont:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2968(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2968(%rip)
2:
  pushq $371
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XSetFontPath
  .p2align 4
  .type XSetFontPath, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSetFontPath
#endif
XSetFontPath:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2976(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2976(%rip)
2:
  pushq $372
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XSetForeground
  .p2align 4
  .type XSetForeground, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSetForeground
#endif
XSetForeground:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2984(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2984(%rip)
2:
  pushq $373
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XSetFunction
  .p2align 4
  .type XSetFunction, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSetFunction
#endif
XSetFunction:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+2992(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+2992(%rip)
2:
  pushq $374
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XSetGraphicsExposures
  .p2align 4
  .type XSetGraphicsExposures, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSetGraphicsExposures
#endif
XSetGraphicsExposures:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3000(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3000(%rip)
2:
  pushq $375
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XSetICFocus
  .p2align 4
  .type XSetICFocus, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSetICFocus
#endif
XSetICFocus:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3008(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3008(%rip)
2:
  pushq $376
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XSetICValues
  .p2align 4
  .type XSetICValues, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSetICValues
#endif
XSetICValues:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3016(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3016(%rip)
2:
  pushq $377
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XSetIMValues
  .p2align 4
  .type XSetIMValues, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSetIMValues
#endif
XSetIMValues:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3024(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3024(%rip)
2:
  pushq $378
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XSetIOErrorExitHandler
  .p2align 4
  .type XSetIOErrorExitHandler, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSetIOErrorExitHandler
#endif
XSetIOErrorExitHandler:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3032(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3032(%rip)
2:
  pushq $379
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XSetIOErrorHandler
  .p2align 4
  .type XSetIOErrorHandler, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSetIOErrorHandler
#endif
XSetIOErrorHandler:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3040(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3040(%rip)
2:
  pushq $380
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XSetIconName
  .p2align 4
  .type XSetIconName, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSetIconName
#endif
XSetIconName:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3048(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3048(%rip)
2:
  pushq $381
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XSetIconSizes
  .p2align 4
  .type XSetIconSizes, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSetIconSizes
#endif
XSetIconSizes:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3056(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3056(%rip)
2:
  pushq $382
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XSetInputFocus
  .p2align 4
  .type XSetInputFocus, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSetInputFocus
#endif
XSetInputFocus:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3064(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3064(%rip)
2:
  pushq $383
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XSetLineAttributes
  .p2align 4
  .type XSetLineAttributes, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSetLineAttributes
#endif
XSetLineAttributes:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3072(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3072(%rip)
2:
  pushq $384
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XSetLocaleModifiers
  .p2align 4
  .type XSetLocaleModifiers, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSetLocaleModifiers
#endif
XSetLocaleModifiers:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3080(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3080(%rip)
2:
  pushq $385
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XSetModifierMapping
  .p2align 4
  .type XSetModifierMapping, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSetModifierMapping
#endif
XSetModifierMapping:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3088(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3088(%rip)
2:
  pushq $386
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XSetNormalHints
  .p2align 4
  .type XSetNormalHints, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSetNormalHints
#endif
XSetNormalHints:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3096(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3096(%rip)
2:
  pushq $387
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XSetOCValues
  .p2align 4
  .type XSetOCValues, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSetOCValues
#endif
XSetOCValues:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3104(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3104(%rip)
2:
  pushq $388
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XSetOMValues
  .p2align 4
  .type XSetOMValues, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSetOMValues
#endif
XSetOMValues:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3112(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3112(%rip)
2:
  pushq $389
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XSetPlaneMask
  .p2align 4
  .type XSetPlaneMask, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSetPlaneMask
#endif
XSetPlaneMask:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3120(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3120(%rip)
2:
  pushq $390
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XSetPointerMapping
  .p2align 4
  .type XSetPointerMapping, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSetPointerMapping
#endif
XSetPointerMapping:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3128(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3128(%rip)
2:
  pushq $391
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XSetRGBColormaps
  .p2align 4
  .type XSetRGBColormaps, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSetRGBColormaps
#endif
XSetRGBColormaps:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3136(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3136(%rip)
2:
  pushq $392
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XSetRegion
  .p2align 4
  .type XSetRegion, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSetRegion
#endif
XSetRegion:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3144(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3144(%rip)
2:
  pushq $393
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XSetScreenSaver
  .p2align 4
  .type XSetScreenSaver, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSetScreenSaver
#endif
XSetScreenSaver:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3152(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3152(%rip)
2:
  pushq $394
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XSetSelectionOwner
  .p2align 4
  .type XSetSelectionOwner, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSetSelectionOwner
#endif
XSetSelectionOwner:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3160(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3160(%rip)
2:
  pushq $395
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XSetSizeHints
  .p2align 4
  .type XSetSizeHints, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSetSizeHints
#endif
XSetSizeHints:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3168(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3168(%rip)
2:
  pushq $396
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XSetStandardColormap
  .p2align 4
  .type XSetStandardColormap, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSetStandardColormap
#endif
XSetStandardColormap:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3176(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3176(%rip)
2:
  pushq $397
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XSetStandardProperties
  .p2align 4
  .type XSetStandardProperties, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSetStandardProperties
#endif
XSetStandardProperties:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3184(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3184(%rip)
2:
  pushq $398
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XSetState
  .p2align 4
  .type XSetState, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSetState
#endif
XSetState:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3192(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3192(%rip)
2:
  pushq $399
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XSetStipple
  .p2align 4
  .type XSetStipple, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSetStipple
#endif
XSetStipple:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3200(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3200(%rip)
2:
  pushq $400
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XSetSubwindowMode
  .p2align 4
  .type XSetSubwindowMode, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSetSubwindowMode
#endif
XSetSubwindowMode:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3208(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3208(%rip)
2:
  pushq $401
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XSetTSOrigin
  .p2align 4
  .type XSetTSOrigin, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSetTSOrigin
#endif
XSetTSOrigin:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3216(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3216(%rip)
2:
  pushq $402
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XSetTextProperty
  .p2align 4
  .type XSetTextProperty, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSetTextProperty
#endif
XSetTextProperty:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3224(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3224(%rip)
2:
  pushq $403
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XSetTile
  .p2align 4
  .type XSetTile, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSetTile
#endif
XSetTile:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3232(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3232(%rip)
2:
  pushq $404
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XSetTransientForHint
  .p2align 4
  .type XSetTransientForHint, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSetTransientForHint
#endif
XSetTransientForHint:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3240(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3240(%rip)
2:
  pushq $405
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XSetWMClientMachine
  .p2align 4
  .type XSetWMClientMachine, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSetWMClientMachine
#endif
XSetWMClientMachine:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3248(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3248(%rip)
2:
  pushq $406
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XSetWMColormapWindows
  .p2align 4
  .type XSetWMColormapWindows, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSetWMColormapWindows
#endif
XSetWMColormapWindows:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3256(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3256(%rip)
2:
  pushq $407
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XSetWMHints
  .p2align 4
  .type XSetWMHints, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSetWMHints
#endif
XSetWMHints:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3264(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3264(%rip)
2:
  pushq $408
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XSetWMIconName
  .p2align 4
  .type XSetWMIconName, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSetWMIconName
#endif
XSetWMIconName:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3272(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3272(%rip)
2:
  pushq $409
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XSetWMName
  .p2align 4
  .type XSetWMName, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSetWMName
#endif
XSetWMName:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3280(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3280(%rip)
2:
  pushq $410
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XSetWMNormalHints
  .p2align 4
  .type XSetWMNormalHints, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSetWMNormalHints
#endif
XSetWMNormalHints:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3288(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3288(%rip)
2:
  pushq $411
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XSetWMProperties
  .p2align 4
  .type XSetWMProperties, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSetWMProperties
#endif
XSetWMProperties:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3296(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3296(%rip)
2:
  pushq $412
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XSetWMProtocols
  .p2align 4
  .type XSetWMProtocols, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSetWMProtocols
#endif
XSetWMProtocols:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3304(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3304(%rip)
2:
  pushq $413
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XSetWMSizeHints
  .p2align 4
  .type XSetWMSizeHints, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSetWMSizeHints
#endif
XSetWMSizeHints:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3312(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3312(%rip)
2:
  pushq $414
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XSetWindowBackground
  .p2align 4
  .type XSetWindowBackground, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSetWindowBackground
#endif
XSetWindowBackground:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3320(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3320(%rip)
2:
  pushq $415
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XSetWindowBackgroundPixmap
  .p2align 4
  .type XSetWindowBackgroundPixmap, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSetWindowBackgroundPixmap
#endif
XSetWindowBackgroundPixmap:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3328(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3328(%rip)
2:
  pushq $416
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XSetWindowBorder
  .p2align 4
  .type XSetWindowBorder, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSetWindowBorder
#endif
XSetWindowBorder:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3336(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3336(%rip)
2:
  pushq $417
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XSetWindowBorderPixmap
  .p2align 4
  .type XSetWindowBorderPixmap, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSetWindowBorderPixmap
#endif
XSetWindowBorderPixmap:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3344(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3344(%rip)
2:
  pushq $418
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XSetWindowBorderWidth
  .p2align 4
  .type XSetWindowBorderWidth, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSetWindowBorderWidth
#endif
XSetWindowBorderWidth:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3352(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3352(%rip)
2:
  pushq $419
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XSetWindowColormap
  .p2align 4
  .type XSetWindowColormap, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSetWindowColormap
#endif
XSetWindowColormap:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3360(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3360(%rip)
2:
  pushq $420
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XSetZoomHints
  .p2align 4
  .type XSetZoomHints, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSetZoomHints
#endif
XSetZoomHints:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3368(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3368(%rip)
2:
  pushq $421
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XShrinkRegion
  .p2align 4
  .type XShrinkRegion, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XShrinkRegion
#endif
XShrinkRegion:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3376(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3376(%rip)
2:
  pushq $422
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XStoreBuffer
  .p2align 4
  .type XStoreBuffer, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XStoreBuffer
#endif
XStoreBuffer:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3384(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3384(%rip)
2:
  pushq $423
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XStoreBytes
  .p2align 4
  .type XStoreBytes, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XStoreBytes
#endif
XStoreBytes:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3392(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3392(%rip)
2:
  pushq $424
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XStoreColor
  .p2align 4
  .type XStoreColor, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XStoreColor
#endif
XStoreColor:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3400(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3400(%rip)
2:
  pushq $425
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XStoreColors
  .p2align 4
  .type XStoreColors, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XStoreColors
#endif
XStoreColors:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3408(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3408(%rip)
2:
  pushq $426
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XStoreName
  .p2align 4
  .type XStoreName, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XStoreName
#endif
XStoreName:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3416(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3416(%rip)
2:
  pushq $427
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XStoreNamedColor
  .p2align 4
  .type XStoreNamedColor, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XStoreNamedColor
#endif
XStoreNamedColor:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3424(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3424(%rip)
2:
  pushq $428
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XStringListToTextProperty
  .p2align 4
  .type XStringListToTextProperty, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XStringListToTextProperty
#endif
XStringListToTextProperty:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3432(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3432(%rip)
2:
  pushq $429
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XStringToKeysym
  .p2align 4
  .type XStringToKeysym, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XStringToKeysym
#endif
XStringToKeysym:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3440(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3440(%rip)
2:
  pushq $430
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XSubImage
  .p2align 4
  .type XSubImage, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSubImage
#endif
XSubImage:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3448(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3448(%rip)
2:
  pushq $431
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XSubtractRegion
  .p2align 4
  .type XSubtractRegion, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSubtractRegion
#endif
XSubtractRegion:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3456(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3456(%rip)
2:
  pushq $432
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XSupportsLocale
  .p2align 4
  .type XSupportsLocale, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSupportsLocale
#endif
XSupportsLocale:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3464(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3464(%rip)
2:
  pushq $433
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XSync
  .p2align 4
  .type XSync, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSync
#endif
XSync:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3472(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3472(%rip)
2:
  pushq $434
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XSynchronize
  .p2align 4
  .type XSynchronize, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XSynchronize
#endif
XSynchronize:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3480(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3480(%rip)
2:
  pushq $435
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XTextExtents
  .p2align 4
  .type XTextExtents, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XTextExtents
#endif
XTextExtents:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3488(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3488(%rip)
2:
  pushq $436
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XTextExtents16
  .p2align 4
  .type XTextExtents16, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XTextExtents16
#endif
XTextExtents16:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3496(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3496(%rip)
2:
  pushq $437
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XTextPropertyToStringList
  .p2align 4
  .type XTextPropertyToStringList, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XTextPropertyToStringList
#endif
XTextPropertyToStringList:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3504(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3504(%rip)
2:
  pushq $438
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XTextWidth
  .p2align 4
  .type XTextWidth, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XTextWidth
#endif
XTextWidth:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3512(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3512(%rip)
2:
  pushq $439
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XTextWidth16
  .p2align 4
  .type XTextWidth16, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XTextWidth16
#endif
XTextWidth16:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3520(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3520(%rip)
2:
  pushq $440
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XTranslateCoordinates
  .p2align 4
  .type XTranslateCoordinates, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XTranslateCoordinates
#endif
XTranslateCoordinates:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3528(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3528(%rip)
2:
  pushq $441
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XUndefineCursor
  .p2align 4
  .type XUndefineCursor, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XUndefineCursor
#endif
XUndefineCursor:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3536(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3536(%rip)
2:
  pushq $442
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XUngrabButton
  .p2align 4
  .type XUngrabButton, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XUngrabButton
#endif
XUngrabButton:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3544(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3544(%rip)
2:
  pushq $443
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XUngrabKey
  .p2align 4
  .type XUngrabKey, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XUngrabKey
#endif
XUngrabKey:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3552(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3552(%rip)
2:
  pushq $444
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XUngrabKeyboard
  .p2align 4
  .type XUngrabKeyboard, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XUngrabKeyboard
#endif
XUngrabKeyboard:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3560(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3560(%rip)
2:
  pushq $445
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XUngrabPointer
  .p2align 4
  .type XUngrabPointer, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XUngrabPointer
#endif
XUngrabPointer:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3568(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3568(%rip)
2:
  pushq $446
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XUngrabServer
  .p2align 4
  .type XUngrabServer, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XUngrabServer
#endif
XUngrabServer:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3576(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3576(%rip)
2:
  pushq $447
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XUninstallColormap
  .p2align 4
  .type XUninstallColormap, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XUninstallColormap
#endif
XUninstallColormap:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3584(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3584(%rip)
2:
  pushq $448
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XUnionRectWithRegion
  .p2align 4
  .type XUnionRectWithRegion, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XUnionRectWithRegion
#endif
XUnionRectWithRegion:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3592(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3592(%rip)
2:
  pushq $449
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XUnionRegion
  .p2align 4
  .type XUnionRegion, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XUnionRegion
#endif
XUnionRegion:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3600(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3600(%rip)
2:
  pushq $450
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XUnloadFont
  .p2align 4
  .type XUnloadFont, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XUnloadFont
#endif
XUnloadFont:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3608(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3608(%rip)
2:
  pushq $451
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XUnlockDisplay
  .p2align 4
  .type XUnlockDisplay, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XUnlockDisplay
#endif
XUnlockDisplay:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3616(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3616(%rip)
2:
  pushq $452
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XUnmapSubwindows
  .p2align 4
  .type XUnmapSubwindows, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XUnmapSubwindows
#endif
XUnmapSubwindows:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3624(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3624(%rip)
2:
  pushq $453
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XUnmapWindow
  .p2align 4
  .type XUnmapWindow, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XUnmapWindow
#endif
XUnmapWindow:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3632(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3632(%rip)
2:
  pushq $454
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XUnregisterIMInstantiateCallback
  .p2align 4
  .type XUnregisterIMInstantiateCallback, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XUnregisterIMInstantiateCallback
#endif
XUnregisterIMInstantiateCallback:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3640(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3640(%rip)
2:
  pushq $455
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XUnsetICFocus
  .p2align 4
  .type XUnsetICFocus, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XUnsetICFocus
#endif
XUnsetICFocus:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3648(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3648(%rip)
2:
  pushq $456
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XVaCreateNestedList
  .p2align 4
  .type XVaCreateNestedList, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XVaCreateNestedList
#endif
XVaCreateNestedList:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3656(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3656(%rip)
2:
  pushq $457
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XVendorRelease
  .p2align 4
  .type XVendorRelease, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XVendorRelease
#endif
XVendorRelease:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3664(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3664(%rip)
2:
  pushq $458
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XVisualIDFromVisual
  .p2align 4
  .type XVisualIDFromVisual, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XVisualIDFromVisual
#endif
XVisualIDFromVisual:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3672(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3672(%rip)
2:
  pushq $459
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XWMGeometry
  .p2align 4
  .type XWMGeometry, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XWMGeometry
#endif
XWMGeometry:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3680(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3680(%rip)
2:
  pushq $460
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XWarpPointer
  .p2align 4
  .type XWarpPointer, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XWarpPointer
#endif
XWarpPointer:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3688(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3688(%rip)
2:
  pushq $461
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XWhitePixel
  .p2align 4
  .type XWhitePixel, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XWhitePixel
#endif
XWhitePixel:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3696(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3696(%rip)
2:
  pushq $462
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XWhitePixelOfScreen
  .p2align 4
  .type XWhitePixelOfScreen, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XWhitePixelOfScreen
#endif
XWhitePixelOfScreen:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3704(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3704(%rip)
2:
  pushq $463
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XWidthMMOfScreen
  .p2align 4
  .type XWidthMMOfScreen, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XWidthMMOfScreen
#endif
XWidthMMOfScreen:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3712(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3712(%rip)
2:
  pushq $464
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XWidthOfScreen
  .p2align 4
  .type XWidthOfScreen, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XWidthOfScreen
#endif
XWidthOfScreen:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3720(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3720(%rip)
2:
  pushq $465
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XWindowEvent
  .p2align 4
  .type XWindowEvent, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XWindowEvent
#endif
XWindowEvent:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3728(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3728(%rip)
2:
  pushq $466
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XWithdrawWindow
  .p2align 4
  .type XWithdrawWindow, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XWithdrawWindow
#endif
XWithdrawWindow:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3736(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3736(%rip)
2:
  pushq $467
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XWriteBitmapFile
  .p2align 4
  .type XWriteBitmapFile, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XWriteBitmapFile
#endif
XWriteBitmapFile:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3744(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3744(%rip)
2:
  pushq $468
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XXorRegion
  .p2align 4
  .type XXorRegion, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XXorRegion
#endif
XXorRegion:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3752(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3752(%rip)
2:
  pushq $469
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XcmsAddColorSpace
  .p2align 4
  .type XcmsAddColorSpace, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XcmsAddColorSpace
#endif
XcmsAddColorSpace:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3760(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3760(%rip)
2:
  pushq $470
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XcmsAddFunctionSet
  .p2align 4
  .type XcmsAddFunctionSet, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XcmsAddFunctionSet
#endif
XcmsAddFunctionSet:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3768(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3768(%rip)
2:
  pushq $471
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XcmsAllocColor
  .p2align 4
  .type XcmsAllocColor, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XcmsAllocColor
#endif
XcmsAllocColor:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3776(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3776(%rip)
2:
  pushq $472
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XcmsAllocNamedColor
  .p2align 4
  .type XcmsAllocNamedColor, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XcmsAllocNamedColor
#endif
XcmsAllocNamedColor:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3784(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3784(%rip)
2:
  pushq $473
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XcmsCCCOfColormap
  .p2align 4
  .type XcmsCCCOfColormap, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XcmsCCCOfColormap
#endif
XcmsCCCOfColormap:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3792(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3792(%rip)
2:
  pushq $474
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XcmsCIELabClipL
  .p2align 4
  .type XcmsCIELabClipL, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XcmsCIELabClipL
#endif
XcmsCIELabClipL:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3800(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3800(%rip)
2:
  pushq $475
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XcmsCIELabClipLab
  .p2align 4
  .type XcmsCIELabClipLab, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XcmsCIELabClipLab
#endif
XcmsCIELabClipLab:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3808(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3808(%rip)
2:
  pushq $476
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XcmsCIELabClipab
  .p2align 4
  .type XcmsCIELabClipab, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XcmsCIELabClipab
#endif
XcmsCIELabClipab:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3816(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3816(%rip)
2:
  pushq $477
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XcmsCIELabQueryMaxC
  .p2align 4
  .type XcmsCIELabQueryMaxC, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XcmsCIELabQueryMaxC
#endif
XcmsCIELabQueryMaxC:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3824(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3824(%rip)
2:
  pushq $478
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XcmsCIELabQueryMaxL
  .p2align 4
  .type XcmsCIELabQueryMaxL, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XcmsCIELabQueryMaxL
#endif
XcmsCIELabQueryMaxL:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3832(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3832(%rip)
2:
  pushq $479
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XcmsCIELabQueryMaxLC
  .p2align 4
  .type XcmsCIELabQueryMaxLC, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XcmsCIELabQueryMaxLC
#endif
XcmsCIELabQueryMaxLC:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3840(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3840(%rip)
2:
  pushq $480
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XcmsCIELabQueryMinL
  .p2align 4
  .type XcmsCIELabQueryMinL, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XcmsCIELabQueryMinL
#endif
XcmsCIELabQueryMinL:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3848(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3848(%rip)
2:
  pushq $481
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XcmsCIELabToCIEXYZ
  .p2align 4
  .type XcmsCIELabToCIEXYZ, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XcmsCIELabToCIEXYZ
#endif
XcmsCIELabToCIEXYZ:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3856(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3856(%rip)
2:
  pushq $482
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XcmsCIELabWhiteShiftColors
  .p2align 4
  .type XcmsCIELabWhiteShiftColors, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XcmsCIELabWhiteShiftColors
#endif
XcmsCIELabWhiteShiftColors:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3864(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3864(%rip)
2:
  pushq $483
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XcmsCIELuvClipL
  .p2align 4
  .type XcmsCIELuvClipL, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XcmsCIELuvClipL
#endif
XcmsCIELuvClipL:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3872(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3872(%rip)
2:
  pushq $484
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XcmsCIELuvClipLuv
  .p2align 4
  .type XcmsCIELuvClipLuv, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XcmsCIELuvClipLuv
#endif
XcmsCIELuvClipLuv:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3880(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3880(%rip)
2:
  pushq $485
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XcmsCIELuvClipuv
  .p2align 4
  .type XcmsCIELuvClipuv, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XcmsCIELuvClipuv
#endif
XcmsCIELuvClipuv:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3888(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3888(%rip)
2:
  pushq $486
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XcmsCIELuvQueryMaxC
  .p2align 4
  .type XcmsCIELuvQueryMaxC, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XcmsCIELuvQueryMaxC
#endif
XcmsCIELuvQueryMaxC:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3896(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3896(%rip)
2:
  pushq $487
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XcmsCIELuvQueryMaxL
  .p2align 4
  .type XcmsCIELuvQueryMaxL, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XcmsCIELuvQueryMaxL
#endif
XcmsCIELuvQueryMaxL:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3904(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3904(%rip)
2:
  pushq $488
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XcmsCIELuvQueryMaxLC
  .p2align 4
  .type XcmsCIELuvQueryMaxLC, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XcmsCIELuvQueryMaxLC
#endif
XcmsCIELuvQueryMaxLC:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3912(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3912(%rip)
2:
  pushq $489
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XcmsCIELuvQueryMinL
  .p2align 4
  .type XcmsCIELuvQueryMinL, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XcmsCIELuvQueryMinL
#endif
XcmsCIELuvQueryMinL:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3920(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3920(%rip)
2:
  pushq $490
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XcmsCIELuvToCIEuvY
  .p2align 4
  .type XcmsCIELuvToCIEuvY, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XcmsCIELuvToCIEuvY
#endif
XcmsCIELuvToCIEuvY:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3928(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3928(%rip)
2:
  pushq $491
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XcmsCIELuvWhiteShiftColors
  .p2align 4
  .type XcmsCIELuvWhiteShiftColors, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XcmsCIELuvWhiteShiftColors
#endif
XcmsCIELuvWhiteShiftColors:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3936(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3936(%rip)
2:
  pushq $492
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XcmsCIEXYZToCIELab
  .p2align 4
  .type XcmsCIEXYZToCIELab, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XcmsCIEXYZToCIELab
#endif
XcmsCIEXYZToCIELab:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3944(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3944(%rip)
2:
  pushq $493
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XcmsCIEXYZToCIEuvY
  .p2align 4
  .type XcmsCIEXYZToCIEuvY, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XcmsCIEXYZToCIEuvY
#endif
XcmsCIEXYZToCIEuvY:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3952(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3952(%rip)
2:
  pushq $494
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XcmsCIEXYZToCIExyY
  .p2align 4
  .type XcmsCIEXYZToCIExyY, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XcmsCIEXYZToCIExyY
#endif
XcmsCIEXYZToCIExyY:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3960(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3960(%rip)
2:
  pushq $495
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XcmsCIEXYZToRGBi
  .p2align 4
  .type XcmsCIEXYZToRGBi, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XcmsCIEXYZToRGBi
#endif
XcmsCIEXYZToRGBi:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3968(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3968(%rip)
2:
  pushq $496
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XcmsCIEuvYToCIELuv
  .p2align 4
  .type XcmsCIEuvYToCIELuv, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XcmsCIEuvYToCIELuv
#endif
XcmsCIEuvYToCIELuv:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3976(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3976(%rip)
2:
  pushq $497
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XcmsCIEuvYToCIEXYZ
  .p2align 4
  .type XcmsCIEuvYToCIEXYZ, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XcmsCIEuvYToCIEXYZ
#endif
XcmsCIEuvYToCIEXYZ:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3984(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3984(%rip)
2:
  pushq $498
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XcmsCIEuvYToTekHVC
  .p2align 4
  .type XcmsCIEuvYToTekHVC, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XcmsCIEuvYToTekHVC
#endif
XcmsCIEuvYToTekHVC:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+3992(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+3992(%rip)
2:
  pushq $499
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XcmsCIExyYToCIEXYZ
  .p2align 4
  .type XcmsCIExyYToCIEXYZ, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XcmsCIExyYToCIEXYZ
#endif
XcmsCIExyYToCIEXYZ:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4000(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4000(%rip)
2:
  pushq $500
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XcmsClientWhitePointOfCCC
  .p2align 4
  .type XcmsClientWhitePointOfCCC, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XcmsClientWhitePointOfCCC
#endif
XcmsClientWhitePointOfCCC:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4008(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4008(%rip)
2:
  pushq $501
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XcmsConvertColors
  .p2align 4
  .type XcmsConvertColors, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XcmsConvertColors
#endif
XcmsConvertColors:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4016(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4016(%rip)
2:
  pushq $502
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XcmsCreateCCC
  .p2align 4
  .type XcmsCreateCCC, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XcmsCreateCCC
#endif
XcmsCreateCCC:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4024(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4024(%rip)
2:
  pushq $503
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XcmsDefaultCCC
  .p2align 4
  .type XcmsDefaultCCC, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XcmsDefaultCCC
#endif
XcmsDefaultCCC:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4032(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4032(%rip)
2:
  pushq $504
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XcmsDisplayOfCCC
  .p2align 4
  .type XcmsDisplayOfCCC, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XcmsDisplayOfCCC
#endif
XcmsDisplayOfCCC:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4040(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4040(%rip)
2:
  pushq $505
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XcmsFormatOfPrefix
  .p2align 4
  .type XcmsFormatOfPrefix, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XcmsFormatOfPrefix
#endif
XcmsFormatOfPrefix:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4048(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4048(%rip)
2:
  pushq $506
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XcmsFreeCCC
  .p2align 4
  .type XcmsFreeCCC, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XcmsFreeCCC
#endif
XcmsFreeCCC:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4056(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4056(%rip)
2:
  pushq $507
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XcmsLookupColor
  .p2align 4
  .type XcmsLookupColor, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XcmsLookupColor
#endif
XcmsLookupColor:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4064(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4064(%rip)
2:
  pushq $508
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XcmsPrefixOfFormat
  .p2align 4
  .type XcmsPrefixOfFormat, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XcmsPrefixOfFormat
#endif
XcmsPrefixOfFormat:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4072(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4072(%rip)
2:
  pushq $509
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XcmsQueryBlack
  .p2align 4
  .type XcmsQueryBlack, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XcmsQueryBlack
#endif
XcmsQueryBlack:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4080(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4080(%rip)
2:
  pushq $510
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XcmsQueryBlue
  .p2align 4
  .type XcmsQueryBlue, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XcmsQueryBlue
#endif
XcmsQueryBlue:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4088(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4088(%rip)
2:
  pushq $511
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XcmsQueryColor
  .p2align 4
  .type XcmsQueryColor, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XcmsQueryColor
#endif
XcmsQueryColor:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4096(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4096(%rip)
2:
  pushq $512
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XcmsQueryColors
  .p2align 4
  .type XcmsQueryColors, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XcmsQueryColors
#endif
XcmsQueryColors:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4104(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4104(%rip)
2:
  pushq $513
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XcmsQueryGreen
  .p2align 4
  .type XcmsQueryGreen, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XcmsQueryGreen
#endif
XcmsQueryGreen:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4112(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4112(%rip)
2:
  pushq $514
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XcmsQueryRed
  .p2align 4
  .type XcmsQueryRed, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XcmsQueryRed
#endif
XcmsQueryRed:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4120(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4120(%rip)
2:
  pushq $515
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XcmsQueryWhite
  .p2align 4
  .type XcmsQueryWhite, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XcmsQueryWhite
#endif
XcmsQueryWhite:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4128(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4128(%rip)
2:
  pushq $516
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XcmsRGBToRGBi
  .p2align 4
  .type XcmsRGBToRGBi, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XcmsRGBToRGBi
#endif
XcmsRGBToRGBi:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4136(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4136(%rip)
2:
  pushq $517
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XcmsRGBiToCIEXYZ
  .p2align 4
  .type XcmsRGBiToCIEXYZ, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XcmsRGBiToCIEXYZ
#endif
XcmsRGBiToCIEXYZ:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4144(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4144(%rip)
2:
  pushq $518
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XcmsRGBiToRGB
  .p2align 4
  .type XcmsRGBiToRGB, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XcmsRGBiToRGB
#endif
XcmsRGBiToRGB:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4152(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4152(%rip)
2:
  pushq $519
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XcmsScreenNumberOfCCC
  .p2align 4
  .type XcmsScreenNumberOfCCC, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XcmsScreenNumberOfCCC
#endif
XcmsScreenNumberOfCCC:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4160(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4160(%rip)
2:
  pushq $520
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XcmsScreenWhitePointOfCCC
  .p2align 4
  .type XcmsScreenWhitePointOfCCC, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XcmsScreenWhitePointOfCCC
#endif
XcmsScreenWhitePointOfCCC:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4168(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4168(%rip)
2:
  pushq $521
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XcmsSetCCCOfColormap
  .p2align 4
  .type XcmsSetCCCOfColormap, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XcmsSetCCCOfColormap
#endif
XcmsSetCCCOfColormap:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4176(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4176(%rip)
2:
  pushq $522
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XcmsSetCompressionProc
  .p2align 4
  .type XcmsSetCompressionProc, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XcmsSetCompressionProc
#endif
XcmsSetCompressionProc:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4184(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4184(%rip)
2:
  pushq $523
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XcmsSetWhiteAdjustProc
  .p2align 4
  .type XcmsSetWhiteAdjustProc, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XcmsSetWhiteAdjustProc
#endif
XcmsSetWhiteAdjustProc:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4192(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4192(%rip)
2:
  pushq $524
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XcmsSetWhitePoint
  .p2align 4
  .type XcmsSetWhitePoint, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XcmsSetWhitePoint
#endif
XcmsSetWhitePoint:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4200(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4200(%rip)
2:
  pushq $525
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XcmsStoreColor
  .p2align 4
  .type XcmsStoreColor, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XcmsStoreColor
#endif
XcmsStoreColor:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4208(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4208(%rip)
2:
  pushq $526
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XcmsStoreColors
  .p2align 4
  .type XcmsStoreColors, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XcmsStoreColors
#endif
XcmsStoreColors:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4216(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4216(%rip)
2:
  pushq $527
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XcmsTekHVCClipC
  .p2align 4
  .type XcmsTekHVCClipC, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XcmsTekHVCClipC
#endif
XcmsTekHVCClipC:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4224(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4224(%rip)
2:
  pushq $528
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XcmsTekHVCClipV
  .p2align 4
  .type XcmsTekHVCClipV, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XcmsTekHVCClipV
#endif
XcmsTekHVCClipV:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4232(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4232(%rip)
2:
  pushq $529
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XcmsTekHVCClipVC
  .p2align 4
  .type XcmsTekHVCClipVC, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XcmsTekHVCClipVC
#endif
XcmsTekHVCClipVC:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4240(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4240(%rip)
2:
  pushq $530
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XcmsTekHVCQueryMaxC
  .p2align 4
  .type XcmsTekHVCQueryMaxC, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XcmsTekHVCQueryMaxC
#endif
XcmsTekHVCQueryMaxC:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4248(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4248(%rip)
2:
  pushq $531
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XcmsTekHVCQueryMaxV
  .p2align 4
  .type XcmsTekHVCQueryMaxV, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XcmsTekHVCQueryMaxV
#endif
XcmsTekHVCQueryMaxV:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4256(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4256(%rip)
2:
  pushq $532
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XcmsTekHVCQueryMaxVC
  .p2align 4
  .type XcmsTekHVCQueryMaxVC, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XcmsTekHVCQueryMaxVC
#endif
XcmsTekHVCQueryMaxVC:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4264(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4264(%rip)
2:
  pushq $533
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XcmsTekHVCQueryMaxVSamples
  .p2align 4
  .type XcmsTekHVCQueryMaxVSamples, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XcmsTekHVCQueryMaxVSamples
#endif
XcmsTekHVCQueryMaxVSamples:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4272(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4272(%rip)
2:
  pushq $534
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XcmsTekHVCQueryMinV
  .p2align 4
  .type XcmsTekHVCQueryMinV, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XcmsTekHVCQueryMinV
#endif
XcmsTekHVCQueryMinV:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4280(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4280(%rip)
2:
  pushq $535
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XcmsTekHVCToCIEuvY
  .p2align 4
  .type XcmsTekHVCToCIEuvY, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XcmsTekHVCToCIEuvY
#endif
XcmsTekHVCToCIEuvY:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4288(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4288(%rip)
2:
  pushq $536
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XcmsTekHVCWhiteShiftColors
  .p2align 4
  .type XcmsTekHVCWhiteShiftColors, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XcmsTekHVCWhiteShiftColors
#endif
XcmsTekHVCWhiteShiftColors:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4296(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4296(%rip)
2:
  pushq $537
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XcmsVisualOfCCC
  .p2align 4
  .type XcmsVisualOfCCC, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XcmsVisualOfCCC
#endif
XcmsVisualOfCCC:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4304(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4304(%rip)
2:
  pushq $538
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbAddDeviceLedInfo
  .p2align 4
  .type XkbAddDeviceLedInfo, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbAddDeviceLedInfo
#endif
XkbAddDeviceLedInfo:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4312(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4312(%rip)
2:
  pushq $539
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbAddGeomColor
  .p2align 4
  .type XkbAddGeomColor, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbAddGeomColor
#endif
XkbAddGeomColor:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4320(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4320(%rip)
2:
  pushq $540
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbAddGeomDoodad
  .p2align 4
  .type XkbAddGeomDoodad, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbAddGeomDoodad
#endif
XkbAddGeomDoodad:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4328(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4328(%rip)
2:
  pushq $541
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbAddGeomKey
  .p2align 4
  .type XkbAddGeomKey, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbAddGeomKey
#endif
XkbAddGeomKey:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4336(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4336(%rip)
2:
  pushq $542
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbAddGeomKeyAlias
  .p2align 4
  .type XkbAddGeomKeyAlias, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbAddGeomKeyAlias
#endif
XkbAddGeomKeyAlias:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4344(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4344(%rip)
2:
  pushq $543
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbAddGeomOutline
  .p2align 4
  .type XkbAddGeomOutline, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbAddGeomOutline
#endif
XkbAddGeomOutline:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4352(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4352(%rip)
2:
  pushq $544
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbAddGeomOverlay
  .p2align 4
  .type XkbAddGeomOverlay, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbAddGeomOverlay
#endif
XkbAddGeomOverlay:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4360(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4360(%rip)
2:
  pushq $545
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbAddGeomOverlayKey
  .p2align 4
  .type XkbAddGeomOverlayKey, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbAddGeomOverlayKey
#endif
XkbAddGeomOverlayKey:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4368(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4368(%rip)
2:
  pushq $546
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbAddGeomOverlayRow
  .p2align 4
  .type XkbAddGeomOverlayRow, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbAddGeomOverlayRow
#endif
XkbAddGeomOverlayRow:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4376(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4376(%rip)
2:
  pushq $547
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbAddGeomProperty
  .p2align 4
  .type XkbAddGeomProperty, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbAddGeomProperty
#endif
XkbAddGeomProperty:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4384(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4384(%rip)
2:
  pushq $548
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbAddGeomRow
  .p2align 4
  .type XkbAddGeomRow, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbAddGeomRow
#endif
XkbAddGeomRow:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4392(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4392(%rip)
2:
  pushq $549
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbAddGeomSection
  .p2align 4
  .type XkbAddGeomSection, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbAddGeomSection
#endif
XkbAddGeomSection:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4400(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4400(%rip)
2:
  pushq $550
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbAddGeomShape
  .p2align 4
  .type XkbAddGeomShape, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbAddGeomShape
#endif
XkbAddGeomShape:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4408(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4408(%rip)
2:
  pushq $551
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbAddKeyType
  .p2align 4
  .type XkbAddKeyType, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbAddKeyType
#endif
XkbAddKeyType:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4416(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4416(%rip)
2:
  pushq $552
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbAllocClientMap
  .p2align 4
  .type XkbAllocClientMap, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbAllocClientMap
#endif
XkbAllocClientMap:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4424(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4424(%rip)
2:
  pushq $553
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbAllocCompatMap
  .p2align 4
  .type XkbAllocCompatMap, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbAllocCompatMap
#endif
XkbAllocCompatMap:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4432(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4432(%rip)
2:
  pushq $554
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbAllocControls
  .p2align 4
  .type XkbAllocControls, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbAllocControls
#endif
XkbAllocControls:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4440(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4440(%rip)
2:
  pushq $555
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbAllocDeviceInfo
  .p2align 4
  .type XkbAllocDeviceInfo, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbAllocDeviceInfo
#endif
XkbAllocDeviceInfo:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4448(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4448(%rip)
2:
  pushq $556
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbAllocGeomColors
  .p2align 4
  .type XkbAllocGeomColors, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbAllocGeomColors
#endif
XkbAllocGeomColors:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4456(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4456(%rip)
2:
  pushq $557
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbAllocGeomDoodads
  .p2align 4
  .type XkbAllocGeomDoodads, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbAllocGeomDoodads
#endif
XkbAllocGeomDoodads:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4464(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4464(%rip)
2:
  pushq $558
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbAllocGeomKeyAliases
  .p2align 4
  .type XkbAllocGeomKeyAliases, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbAllocGeomKeyAliases
#endif
XkbAllocGeomKeyAliases:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4472(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4472(%rip)
2:
  pushq $559
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbAllocGeomKeys
  .p2align 4
  .type XkbAllocGeomKeys, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbAllocGeomKeys
#endif
XkbAllocGeomKeys:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4480(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4480(%rip)
2:
  pushq $560
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbAllocGeomOutlines
  .p2align 4
  .type XkbAllocGeomOutlines, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbAllocGeomOutlines
#endif
XkbAllocGeomOutlines:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4488(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4488(%rip)
2:
  pushq $561
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbAllocGeomOverlayKeys
  .p2align 4
  .type XkbAllocGeomOverlayKeys, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbAllocGeomOverlayKeys
#endif
XkbAllocGeomOverlayKeys:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4496(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4496(%rip)
2:
  pushq $562
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbAllocGeomOverlayRows
  .p2align 4
  .type XkbAllocGeomOverlayRows, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbAllocGeomOverlayRows
#endif
XkbAllocGeomOverlayRows:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4504(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4504(%rip)
2:
  pushq $563
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbAllocGeomOverlays
  .p2align 4
  .type XkbAllocGeomOverlays, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbAllocGeomOverlays
#endif
XkbAllocGeomOverlays:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4512(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4512(%rip)
2:
  pushq $564
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbAllocGeomPoints
  .p2align 4
  .type XkbAllocGeomPoints, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbAllocGeomPoints
#endif
XkbAllocGeomPoints:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4520(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4520(%rip)
2:
  pushq $565
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbAllocGeomProps
  .p2align 4
  .type XkbAllocGeomProps, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbAllocGeomProps
#endif
XkbAllocGeomProps:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4528(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4528(%rip)
2:
  pushq $566
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbAllocGeomRows
  .p2align 4
  .type XkbAllocGeomRows, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbAllocGeomRows
#endif
XkbAllocGeomRows:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4536(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4536(%rip)
2:
  pushq $567
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbAllocGeomSectionDoodads
  .p2align 4
  .type XkbAllocGeomSectionDoodads, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbAllocGeomSectionDoodads
#endif
XkbAllocGeomSectionDoodads:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4544(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4544(%rip)
2:
  pushq $568
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbAllocGeomSections
  .p2align 4
  .type XkbAllocGeomSections, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbAllocGeomSections
#endif
XkbAllocGeomSections:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4552(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4552(%rip)
2:
  pushq $569
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbAllocGeomShapes
  .p2align 4
  .type XkbAllocGeomShapes, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbAllocGeomShapes
#endif
XkbAllocGeomShapes:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4560(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4560(%rip)
2:
  pushq $570
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbAllocGeometry
  .p2align 4
  .type XkbAllocGeometry, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbAllocGeometry
#endif
XkbAllocGeometry:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4568(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4568(%rip)
2:
  pushq $571
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbAllocIndicatorMaps
  .p2align 4
  .type XkbAllocIndicatorMaps, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbAllocIndicatorMaps
#endif
XkbAllocIndicatorMaps:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4576(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4576(%rip)
2:
  pushq $572
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbAllocKeyboard
  .p2align 4
  .type XkbAllocKeyboard, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbAllocKeyboard
#endif
XkbAllocKeyboard:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4584(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4584(%rip)
2:
  pushq $573
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbAllocNames
  .p2align 4
  .type XkbAllocNames, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbAllocNames
#endif
XkbAllocNames:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4592(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4592(%rip)
2:
  pushq $574
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbAllocServerMap
  .p2align 4
  .type XkbAllocServerMap, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbAllocServerMap
#endif
XkbAllocServerMap:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4600(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4600(%rip)
2:
  pushq $575
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbApplyCompatMapToKey
  .p2align 4
  .type XkbApplyCompatMapToKey, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbApplyCompatMapToKey
#endif
XkbApplyCompatMapToKey:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4608(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4608(%rip)
2:
  pushq $576
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbApplyVirtualModChanges
  .p2align 4
  .type XkbApplyVirtualModChanges, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbApplyVirtualModChanges
#endif
XkbApplyVirtualModChanges:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4616(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4616(%rip)
2:
  pushq $577
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbBell
  .p2align 4
  .type XkbBell, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbBell
#endif
XkbBell:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4624(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4624(%rip)
2:
  pushq $578
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbBellEvent
  .p2align 4
  .type XkbBellEvent, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbBellEvent
#endif
XkbBellEvent:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4632(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4632(%rip)
2:
  pushq $579
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbChangeDeviceInfo
  .p2align 4
  .type XkbChangeDeviceInfo, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbChangeDeviceInfo
#endif
XkbChangeDeviceInfo:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4640(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4640(%rip)
2:
  pushq $580
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbChangeEnabledControls
  .p2align 4
  .type XkbChangeEnabledControls, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbChangeEnabledControls
#endif
XkbChangeEnabledControls:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4648(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4648(%rip)
2:
  pushq $581
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbChangeKeycodeRange
  .p2align 4
  .type XkbChangeKeycodeRange, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbChangeKeycodeRange
#endif
XkbChangeKeycodeRange:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4656(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4656(%rip)
2:
  pushq $582
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbChangeMap
  .p2align 4
  .type XkbChangeMap, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbChangeMap
#endif
XkbChangeMap:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4664(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4664(%rip)
2:
  pushq $583
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbChangeNames
  .p2align 4
  .type XkbChangeNames, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbChangeNames
#endif
XkbChangeNames:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4672(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4672(%rip)
2:
  pushq $584
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbChangeTypesOfKey
  .p2align 4
  .type XkbChangeTypesOfKey, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbChangeTypesOfKey
#endif
XkbChangeTypesOfKey:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4680(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4680(%rip)
2:
  pushq $585
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbComputeEffectiveMap
  .p2align 4
  .type XkbComputeEffectiveMap, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbComputeEffectiveMap
#endif
XkbComputeEffectiveMap:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4688(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4688(%rip)
2:
  pushq $586
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbComputeRowBounds
  .p2align 4
  .type XkbComputeRowBounds, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbComputeRowBounds
#endif
XkbComputeRowBounds:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4696(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4696(%rip)
2:
  pushq $587
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbComputeSectionBounds
  .p2align 4
  .type XkbComputeSectionBounds, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbComputeSectionBounds
#endif
XkbComputeSectionBounds:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4704(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4704(%rip)
2:
  pushq $588
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbComputeShapeBounds
  .p2align 4
  .type XkbComputeShapeBounds, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbComputeShapeBounds
#endif
XkbComputeShapeBounds:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4712(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4712(%rip)
2:
  pushq $589
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbComputeShapeTop
  .p2align 4
  .type XkbComputeShapeTop, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbComputeShapeTop
#endif
XkbComputeShapeTop:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4720(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4720(%rip)
2:
  pushq $590
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbCopyKeyType
  .p2align 4
  .type XkbCopyKeyType, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbCopyKeyType
#endif
XkbCopyKeyType:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4728(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4728(%rip)
2:
  pushq $591
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbCopyKeyTypes
  .p2align 4
  .type XkbCopyKeyTypes, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbCopyKeyTypes
#endif
XkbCopyKeyTypes:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4736(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4736(%rip)
2:
  pushq $592
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbDeviceBell
  .p2align 4
  .type XkbDeviceBell, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbDeviceBell
#endif
XkbDeviceBell:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4744(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4744(%rip)
2:
  pushq $593
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbDeviceBellEvent
  .p2align 4
  .type XkbDeviceBellEvent, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbDeviceBellEvent
#endif
XkbDeviceBellEvent:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4752(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4752(%rip)
2:
  pushq $594
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbFindOverlayForKey
  .p2align 4
  .type XkbFindOverlayForKey, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbFindOverlayForKey
#endif
XkbFindOverlayForKey:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4760(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4760(%rip)
2:
  pushq $595
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbForceBell
  .p2align 4
  .type XkbForceBell, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbForceBell
#endif
XkbForceBell:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4768(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4768(%rip)
2:
  pushq $596
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbForceDeviceBell
  .p2align 4
  .type XkbForceDeviceBell, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbForceDeviceBell
#endif
XkbForceDeviceBell:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4776(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4776(%rip)
2:
  pushq $597
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbFreeClientMap
  .p2align 4
  .type XkbFreeClientMap, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbFreeClientMap
#endif
XkbFreeClientMap:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4784(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4784(%rip)
2:
  pushq $598
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbFreeCompatMap
  .p2align 4
  .type XkbFreeCompatMap, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbFreeCompatMap
#endif
XkbFreeCompatMap:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4792(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4792(%rip)
2:
  pushq $599
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbFreeComponentList
  .p2align 4
  .type XkbFreeComponentList, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbFreeComponentList
#endif
XkbFreeComponentList:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4800(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4800(%rip)
2:
  pushq $600
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbFreeControls
  .p2align 4
  .type XkbFreeControls, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbFreeControls
#endif
XkbFreeControls:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4808(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4808(%rip)
2:
  pushq $601
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbFreeDeviceInfo
  .p2align 4
  .type XkbFreeDeviceInfo, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbFreeDeviceInfo
#endif
XkbFreeDeviceInfo:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4816(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4816(%rip)
2:
  pushq $602
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbFreeGeomColors
  .p2align 4
  .type XkbFreeGeomColors, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbFreeGeomColors
#endif
XkbFreeGeomColors:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4824(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4824(%rip)
2:
  pushq $603
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbFreeGeomDoodads
  .p2align 4
  .type XkbFreeGeomDoodads, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbFreeGeomDoodads
#endif
XkbFreeGeomDoodads:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4832(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4832(%rip)
2:
  pushq $604
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbFreeGeomKeyAliases
  .p2align 4
  .type XkbFreeGeomKeyAliases, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbFreeGeomKeyAliases
#endif
XkbFreeGeomKeyAliases:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4840(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4840(%rip)
2:
  pushq $605
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbFreeGeomKeys
  .p2align 4
  .type XkbFreeGeomKeys, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbFreeGeomKeys
#endif
XkbFreeGeomKeys:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4848(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4848(%rip)
2:
  pushq $606
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbFreeGeomOutlines
  .p2align 4
  .type XkbFreeGeomOutlines, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbFreeGeomOutlines
#endif
XkbFreeGeomOutlines:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4856(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4856(%rip)
2:
  pushq $607
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbFreeGeomOverlayKeys
  .p2align 4
  .type XkbFreeGeomOverlayKeys, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbFreeGeomOverlayKeys
#endif
XkbFreeGeomOverlayKeys:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4864(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4864(%rip)
2:
  pushq $608
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbFreeGeomOverlayRows
  .p2align 4
  .type XkbFreeGeomOverlayRows, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbFreeGeomOverlayRows
#endif
XkbFreeGeomOverlayRows:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4872(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4872(%rip)
2:
  pushq $609
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbFreeGeomOverlays
  .p2align 4
  .type XkbFreeGeomOverlays, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbFreeGeomOverlays
#endif
XkbFreeGeomOverlays:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4880(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4880(%rip)
2:
  pushq $610
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbFreeGeomPoints
  .p2align 4
  .type XkbFreeGeomPoints, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbFreeGeomPoints
#endif
XkbFreeGeomPoints:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4888(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4888(%rip)
2:
  pushq $611
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbFreeGeomProperties
  .p2align 4
  .type XkbFreeGeomProperties, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbFreeGeomProperties
#endif
XkbFreeGeomProperties:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4896(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4896(%rip)
2:
  pushq $612
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbFreeGeomRows
  .p2align 4
  .type XkbFreeGeomRows, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbFreeGeomRows
#endif
XkbFreeGeomRows:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4904(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4904(%rip)
2:
  pushq $613
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbFreeGeomSections
  .p2align 4
  .type XkbFreeGeomSections, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbFreeGeomSections
#endif
XkbFreeGeomSections:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4912(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4912(%rip)
2:
  pushq $614
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbFreeGeomShapes
  .p2align 4
  .type XkbFreeGeomShapes, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbFreeGeomShapes
#endif
XkbFreeGeomShapes:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4920(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4920(%rip)
2:
  pushq $615
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbFreeGeometry
  .p2align 4
  .type XkbFreeGeometry, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbFreeGeometry
#endif
XkbFreeGeometry:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4928(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4928(%rip)
2:
  pushq $616
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbFreeIndicatorMaps
  .p2align 4
  .type XkbFreeIndicatorMaps, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbFreeIndicatorMaps
#endif
XkbFreeIndicatorMaps:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4936(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4936(%rip)
2:
  pushq $617
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbFreeKeyboard
  .p2align 4
  .type XkbFreeKeyboard, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbFreeKeyboard
#endif
XkbFreeKeyboard:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4944(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4944(%rip)
2:
  pushq $618
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbFreeNames
  .p2align 4
  .type XkbFreeNames, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbFreeNames
#endif
XkbFreeNames:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4952(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4952(%rip)
2:
  pushq $619
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbFreeServerMap
  .p2align 4
  .type XkbFreeServerMap, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbFreeServerMap
#endif
XkbFreeServerMap:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4960(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4960(%rip)
2:
  pushq $620
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbGetAutoRepeatRate
  .p2align 4
  .type XkbGetAutoRepeatRate, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbGetAutoRepeatRate
#endif
XkbGetAutoRepeatRate:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4968(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4968(%rip)
2:
  pushq $621
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbGetAutoResetControls
  .p2align 4
  .type XkbGetAutoResetControls, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbGetAutoResetControls
#endif
XkbGetAutoResetControls:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4976(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4976(%rip)
2:
  pushq $622
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbGetCompatMap
  .p2align 4
  .type XkbGetCompatMap, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbGetCompatMap
#endif
XkbGetCompatMap:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4984(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4984(%rip)
2:
  pushq $623
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbGetControls
  .p2align 4
  .type XkbGetControls, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbGetControls
#endif
XkbGetControls:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+4992(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+4992(%rip)
2:
  pushq $624
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbGetDetectableAutoRepeat
  .p2align 4
  .type XkbGetDetectableAutoRepeat, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbGetDetectableAutoRepeat
#endif
XkbGetDetectableAutoRepeat:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5000(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5000(%rip)
2:
  pushq $625
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbGetDeviceButtonActions
  .p2align 4
  .type XkbGetDeviceButtonActions, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbGetDeviceButtonActions
#endif
XkbGetDeviceButtonActions:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5008(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5008(%rip)
2:
  pushq $626
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbGetDeviceInfo
  .p2align 4
  .type XkbGetDeviceInfo, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbGetDeviceInfo
#endif
XkbGetDeviceInfo:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5016(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5016(%rip)
2:
  pushq $627
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbGetDeviceInfoChanges
  .p2align 4
  .type XkbGetDeviceInfoChanges, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbGetDeviceInfoChanges
#endif
XkbGetDeviceInfoChanges:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5024(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5024(%rip)
2:
  pushq $628
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbGetDeviceLedInfo
  .p2align 4
  .type XkbGetDeviceLedInfo, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbGetDeviceLedInfo
#endif
XkbGetDeviceLedInfo:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5032(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5032(%rip)
2:
  pushq $629
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbGetGeometry
  .p2align 4
  .type XkbGetGeometry, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbGetGeometry
#endif
XkbGetGeometry:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5040(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5040(%rip)
2:
  pushq $630
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbGetIndicatorMap
  .p2align 4
  .type XkbGetIndicatorMap, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbGetIndicatorMap
#endif
XkbGetIndicatorMap:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5048(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5048(%rip)
2:
  pushq $631
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbGetIndicatorState
  .p2align 4
  .type XkbGetIndicatorState, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbGetIndicatorState
#endif
XkbGetIndicatorState:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5056(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5056(%rip)
2:
  pushq $632
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbGetKeyActions
  .p2align 4
  .type XkbGetKeyActions, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbGetKeyActions
#endif
XkbGetKeyActions:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5064(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5064(%rip)
2:
  pushq $633
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbGetKeyBehaviors
  .p2align 4
  .type XkbGetKeyBehaviors, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbGetKeyBehaviors
#endif
XkbGetKeyBehaviors:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5072(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5072(%rip)
2:
  pushq $634
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbGetKeyExplicitComponents
  .p2align 4
  .type XkbGetKeyExplicitComponents, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbGetKeyExplicitComponents
#endif
XkbGetKeyExplicitComponents:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5080(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5080(%rip)
2:
  pushq $635
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbGetKeyModifierMap
  .p2align 4
  .type XkbGetKeyModifierMap, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbGetKeyModifierMap
#endif
XkbGetKeyModifierMap:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5088(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5088(%rip)
2:
  pushq $636
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbGetKeySyms
  .p2align 4
  .type XkbGetKeySyms, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbGetKeySyms
#endif
XkbGetKeySyms:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5096(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5096(%rip)
2:
  pushq $637
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbGetKeyTypes
  .p2align 4
  .type XkbGetKeyTypes, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbGetKeyTypes
#endif
XkbGetKeyTypes:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5104(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5104(%rip)
2:
  pushq $638
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbGetKeyVirtualModMap
  .p2align 4
  .type XkbGetKeyVirtualModMap, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbGetKeyVirtualModMap
#endif
XkbGetKeyVirtualModMap:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5112(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5112(%rip)
2:
  pushq $639
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbGetKeyboard
  .p2align 4
  .type XkbGetKeyboard, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbGetKeyboard
#endif
XkbGetKeyboard:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5120(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5120(%rip)
2:
  pushq $640
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbGetKeyboardByName
  .p2align 4
  .type XkbGetKeyboardByName, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbGetKeyboardByName
#endif
XkbGetKeyboardByName:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5128(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5128(%rip)
2:
  pushq $641
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbGetMap
  .p2align 4
  .type XkbGetMap, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbGetMap
#endif
XkbGetMap:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5136(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5136(%rip)
2:
  pushq $642
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbGetMapChanges
  .p2align 4
  .type XkbGetMapChanges, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbGetMapChanges
#endif
XkbGetMapChanges:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5144(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5144(%rip)
2:
  pushq $643
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbGetNamedDeviceIndicator
  .p2align 4
  .type XkbGetNamedDeviceIndicator, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbGetNamedDeviceIndicator
#endif
XkbGetNamedDeviceIndicator:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5152(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5152(%rip)
2:
  pushq $644
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbGetNamedGeometry
  .p2align 4
  .type XkbGetNamedGeometry, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbGetNamedGeometry
#endif
XkbGetNamedGeometry:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5160(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5160(%rip)
2:
  pushq $645
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbGetNamedIndicator
  .p2align 4
  .type XkbGetNamedIndicator, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbGetNamedIndicator
#endif
XkbGetNamedIndicator:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5168(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5168(%rip)
2:
  pushq $646
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbGetNames
  .p2align 4
  .type XkbGetNames, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbGetNames
#endif
XkbGetNames:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5176(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5176(%rip)
2:
  pushq $647
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbGetPerClientControls
  .p2align 4
  .type XkbGetPerClientControls, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbGetPerClientControls
#endif
XkbGetPerClientControls:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5184(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5184(%rip)
2:
  pushq $648
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbGetState
  .p2align 4
  .type XkbGetState, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbGetState
#endif
XkbGetState:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5192(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5192(%rip)
2:
  pushq $649
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbGetUpdatedMap
  .p2align 4
  .type XkbGetUpdatedMap, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbGetUpdatedMap
#endif
XkbGetUpdatedMap:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5200(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5200(%rip)
2:
  pushq $650
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbGetVirtualMods
  .p2align 4
  .type XkbGetVirtualMods, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbGetVirtualMods
#endif
XkbGetVirtualMods:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5208(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5208(%rip)
2:
  pushq $651
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbGetXlibControls
  .p2align 4
  .type XkbGetXlibControls, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbGetXlibControls
#endif
XkbGetXlibControls:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5216(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5216(%rip)
2:
  pushq $652
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbIgnoreExtension
  .p2align 4
  .type XkbIgnoreExtension, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbIgnoreExtension
#endif
XkbIgnoreExtension:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5224(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5224(%rip)
2:
  pushq $653
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbInitCanonicalKeyTypes
  .p2align 4
  .type XkbInitCanonicalKeyTypes, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbInitCanonicalKeyTypes
#endif
XkbInitCanonicalKeyTypes:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5232(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5232(%rip)
2:
  pushq $654
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbKeyTypesForCoreSymbols
  .p2align 4
  .type XkbKeyTypesForCoreSymbols, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbKeyTypesForCoreSymbols
#endif
XkbKeyTypesForCoreSymbols:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5240(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5240(%rip)
2:
  pushq $655
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbKeycodeToKeysym
  .p2align 4
  .type XkbKeycodeToKeysym, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbKeycodeToKeysym
#endif
XkbKeycodeToKeysym:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5248(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5248(%rip)
2:
  pushq $656
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbKeysymToModifiers
  .p2align 4
  .type XkbKeysymToModifiers, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbKeysymToModifiers
#endif
XkbKeysymToModifiers:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5256(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5256(%rip)
2:
  pushq $657
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbLatchGroup
  .p2align 4
  .type XkbLatchGroup, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbLatchGroup
#endif
XkbLatchGroup:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5264(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5264(%rip)
2:
  pushq $658
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbLatchModifiers
  .p2align 4
  .type XkbLatchModifiers, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbLatchModifiers
#endif
XkbLatchModifiers:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5272(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5272(%rip)
2:
  pushq $659
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbLibraryVersion
  .p2align 4
  .type XkbLibraryVersion, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbLibraryVersion
#endif
XkbLibraryVersion:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5280(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5280(%rip)
2:
  pushq $660
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbListComponents
  .p2align 4
  .type XkbListComponents, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbListComponents
#endif
XkbListComponents:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5288(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5288(%rip)
2:
  pushq $661
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbLockGroup
  .p2align 4
  .type XkbLockGroup, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbLockGroup
#endif
XkbLockGroup:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5296(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5296(%rip)
2:
  pushq $662
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbLockModifiers
  .p2align 4
  .type XkbLockModifiers, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbLockModifiers
#endif
XkbLockModifiers:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5304(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5304(%rip)
2:
  pushq $663
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbLookupKeyBinding
  .p2align 4
  .type XkbLookupKeyBinding, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbLookupKeyBinding
#endif
XkbLookupKeyBinding:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5312(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5312(%rip)
2:
  pushq $664
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbLookupKeySym
  .p2align 4
  .type XkbLookupKeySym, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbLookupKeySym
#endif
XkbLookupKeySym:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5320(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5320(%rip)
2:
  pushq $665
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbNoteControlsChanges
  .p2align 4
  .type XkbNoteControlsChanges, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbNoteControlsChanges
#endif
XkbNoteControlsChanges:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5328(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5328(%rip)
2:
  pushq $666
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbNoteDeviceChanges
  .p2align 4
  .type XkbNoteDeviceChanges, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbNoteDeviceChanges
#endif
XkbNoteDeviceChanges:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5336(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5336(%rip)
2:
  pushq $667
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbNoteMapChanges
  .p2align 4
  .type XkbNoteMapChanges, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbNoteMapChanges
#endif
XkbNoteMapChanges:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5344(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5344(%rip)
2:
  pushq $668
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbNoteNameChanges
  .p2align 4
  .type XkbNoteNameChanges, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbNoteNameChanges
#endif
XkbNoteNameChanges:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5352(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5352(%rip)
2:
  pushq $669
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbOpenDisplay
  .p2align 4
  .type XkbOpenDisplay, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbOpenDisplay
#endif
XkbOpenDisplay:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5360(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5360(%rip)
2:
  pushq $670
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbQueryExtension
  .p2align 4
  .type XkbQueryExtension, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbQueryExtension
#endif
XkbQueryExtension:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5368(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5368(%rip)
2:
  pushq $671
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbRefreshKeyboardMapping
  .p2align 4
  .type XkbRefreshKeyboardMapping, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbRefreshKeyboardMapping
#endif
XkbRefreshKeyboardMapping:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5376(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5376(%rip)
2:
  pushq $672
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbResizeDeviceButtonActions
  .p2align 4
  .type XkbResizeDeviceButtonActions, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbResizeDeviceButtonActions
#endif
XkbResizeDeviceButtonActions:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5384(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5384(%rip)
2:
  pushq $673
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbResizeKeyActions
  .p2align 4
  .type XkbResizeKeyActions, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbResizeKeyActions
#endif
XkbResizeKeyActions:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5392(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5392(%rip)
2:
  pushq $674
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbResizeKeySyms
  .p2align 4
  .type XkbResizeKeySyms, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbResizeKeySyms
#endif
XkbResizeKeySyms:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5400(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5400(%rip)
2:
  pushq $675
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbResizeKeyType
  .p2align 4
  .type XkbResizeKeyType, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbResizeKeyType
#endif
XkbResizeKeyType:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5408(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5408(%rip)
2:
  pushq $676
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbSelectEventDetails
  .p2align 4
  .type XkbSelectEventDetails, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbSelectEventDetails
#endif
XkbSelectEventDetails:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5416(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5416(%rip)
2:
  pushq $677
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbSelectEvents
  .p2align 4
  .type XkbSelectEvents, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbSelectEvents
#endif
XkbSelectEvents:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5424(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5424(%rip)
2:
  pushq $678
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbSetAtomFuncs
  .p2align 4
  .type XkbSetAtomFuncs, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbSetAtomFuncs
#endif
XkbSetAtomFuncs:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5432(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5432(%rip)
2:
  pushq $679
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbSetAutoRepeatRate
  .p2align 4
  .type XkbSetAutoRepeatRate, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbSetAutoRepeatRate
#endif
XkbSetAutoRepeatRate:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5440(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5440(%rip)
2:
  pushq $680
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbSetAutoResetControls
  .p2align 4
  .type XkbSetAutoResetControls, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbSetAutoResetControls
#endif
XkbSetAutoResetControls:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5448(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5448(%rip)
2:
  pushq $681
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbSetCompatMap
  .p2align 4
  .type XkbSetCompatMap, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbSetCompatMap
#endif
XkbSetCompatMap:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5456(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5456(%rip)
2:
  pushq $682
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbSetControls
  .p2align 4
  .type XkbSetControls, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbSetControls
#endif
XkbSetControls:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5464(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5464(%rip)
2:
  pushq $683
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbSetDebuggingFlags
  .p2align 4
  .type XkbSetDebuggingFlags, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbSetDebuggingFlags
#endif
XkbSetDebuggingFlags:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5472(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5472(%rip)
2:
  pushq $684
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbSetDetectableAutoRepeat
  .p2align 4
  .type XkbSetDetectableAutoRepeat, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbSetDetectableAutoRepeat
#endif
XkbSetDetectableAutoRepeat:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5480(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5480(%rip)
2:
  pushq $685
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbSetDeviceButtonActions
  .p2align 4
  .type XkbSetDeviceButtonActions, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbSetDeviceButtonActions
#endif
XkbSetDeviceButtonActions:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5488(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5488(%rip)
2:
  pushq $686
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbSetDeviceInfo
  .p2align 4
  .type XkbSetDeviceInfo, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbSetDeviceInfo
#endif
XkbSetDeviceInfo:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5496(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5496(%rip)
2:
  pushq $687
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbSetDeviceLedInfo
  .p2align 4
  .type XkbSetDeviceLedInfo, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbSetDeviceLedInfo
#endif
XkbSetDeviceLedInfo:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5504(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5504(%rip)
2:
  pushq $688
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbSetGeometry
  .p2align 4
  .type XkbSetGeometry, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbSetGeometry
#endif
XkbSetGeometry:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5512(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5512(%rip)
2:
  pushq $689
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbSetIgnoreLockMods
  .p2align 4
  .type XkbSetIgnoreLockMods, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbSetIgnoreLockMods
#endif
XkbSetIgnoreLockMods:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5520(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5520(%rip)
2:
  pushq $690
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbSetIndicatorMap
  .p2align 4
  .type XkbSetIndicatorMap, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbSetIndicatorMap
#endif
XkbSetIndicatorMap:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5528(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5528(%rip)
2:
  pushq $691
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbSetMap
  .p2align 4
  .type XkbSetMap, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbSetMap
#endif
XkbSetMap:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5536(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5536(%rip)
2:
  pushq $692
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbSetNamedDeviceIndicator
  .p2align 4
  .type XkbSetNamedDeviceIndicator, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbSetNamedDeviceIndicator
#endif
XkbSetNamedDeviceIndicator:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5544(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5544(%rip)
2:
  pushq $693
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbSetNamedIndicator
  .p2align 4
  .type XkbSetNamedIndicator, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbSetNamedIndicator
#endif
XkbSetNamedIndicator:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5552(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5552(%rip)
2:
  pushq $694
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbSetNames
  .p2align 4
  .type XkbSetNames, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbSetNames
#endif
XkbSetNames:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5560(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5560(%rip)
2:
  pushq $695
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbSetPerClientControls
  .p2align 4
  .type XkbSetPerClientControls, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbSetPerClientControls
#endif
XkbSetPerClientControls:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5568(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5568(%rip)
2:
  pushq $696
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbSetServerInternalMods
  .p2align 4
  .type XkbSetServerInternalMods, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbSetServerInternalMods
#endif
XkbSetServerInternalMods:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5576(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5576(%rip)
2:
  pushq $697
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbSetXlibControls
  .p2align 4
  .type XkbSetXlibControls, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbSetXlibControls
#endif
XkbSetXlibControls:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5584(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5584(%rip)
2:
  pushq $698
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbToControl
  .p2align 4
  .type XkbToControl, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbToControl
#endif
XkbToControl:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5592(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5592(%rip)
2:
  pushq $699
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbTranslateKey
  .p2align 4
  .type XkbTranslateKey, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbTranslateKey
#endif
XkbTranslateKey:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5600(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5600(%rip)
2:
  pushq $700
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbTranslateKeyCode
  .p2align 4
  .type XkbTranslateKeyCode, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbTranslateKeyCode
#endif
XkbTranslateKeyCode:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5608(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5608(%rip)
2:
  pushq $701
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbTranslateKeySym
  .p2align 4
  .type XkbTranslateKeySym, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbTranslateKeySym
#endif
XkbTranslateKeySym:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5616(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5616(%rip)
2:
  pushq $702
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbUpdateActionVirtualMods
  .p2align 4
  .type XkbUpdateActionVirtualMods, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbUpdateActionVirtualMods
#endif
XkbUpdateActionVirtualMods:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5624(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5624(%rip)
2:
  pushq $703
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbUpdateKeyTypeVirtualMods
  .p2align 4
  .type XkbUpdateKeyTypeVirtualMods, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbUpdateKeyTypeVirtualMods
#endif
XkbUpdateKeyTypeVirtualMods:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5632(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5632(%rip)
2:
  pushq $704
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbUpdateMapFromCore
  .p2align 4
  .type XkbUpdateMapFromCore, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbUpdateMapFromCore
#endif
XkbUpdateMapFromCore:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5640(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5640(%rip)
2:
  pushq $705
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbUseExtension
  .p2align 4
  .type XkbUseExtension, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbUseExtension
#endif
XkbUseExtension:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5648(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5648(%rip)
2:
  pushq $706
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbVirtualModsToReal
  .p2align 4
  .type XkbVirtualModsToReal, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbVirtualModsToReal
#endif
XkbVirtualModsToReal:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5656(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5656(%rip)
2:
  pushq $707
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XkbXlibControlsImplemented
  .p2align 4
  .type XkbXlibControlsImplemented, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XkbXlibControlsImplemented
#endif
XkbXlibControlsImplemented:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5664(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5664(%rip)
2:
  pushq $708
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XmbDrawImageString
  .p2align 4
  .type XmbDrawImageString, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XmbDrawImageString
#endif
XmbDrawImageString:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5672(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5672(%rip)
2:
  pushq $709
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XmbDrawString
  .p2align 4
  .type XmbDrawString, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XmbDrawString
#endif
XmbDrawString:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5680(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5680(%rip)
2:
  pushq $710
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XmbDrawText
  .p2align 4
  .type XmbDrawText, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XmbDrawText
#endif
XmbDrawText:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5688(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5688(%rip)
2:
  pushq $711
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XmbLookupString
  .p2align 4
  .type XmbLookupString, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XmbLookupString
#endif
XmbLookupString:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5696(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5696(%rip)
2:
  pushq $712
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XmbResetIC
  .p2align 4
  .type XmbResetIC, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XmbResetIC
#endif
XmbResetIC:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5704(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5704(%rip)
2:
  pushq $713
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XmbSetWMProperties
  .p2align 4
  .type XmbSetWMProperties, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XmbSetWMProperties
#endif
XmbSetWMProperties:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5712(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5712(%rip)
2:
  pushq $714
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XmbTextEscapement
  .p2align 4
  .type XmbTextEscapement, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XmbTextEscapement
#endif
XmbTextEscapement:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5720(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5720(%rip)
2:
  pushq $715
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XmbTextExtents
  .p2align 4
  .type XmbTextExtents, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XmbTextExtents
#endif
XmbTextExtents:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5728(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5728(%rip)
2:
  pushq $716
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XmbTextListToTextProperty
  .p2align 4
  .type XmbTextListToTextProperty, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XmbTextListToTextProperty
#endif
XmbTextListToTextProperty:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5736(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5736(%rip)
2:
  pushq $717
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XmbTextPerCharExtents
  .p2align 4
  .type XmbTextPerCharExtents, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XmbTextPerCharExtents
#endif
XmbTextPerCharExtents:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5744(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5744(%rip)
2:
  pushq $718
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XmbTextPropertyToTextList
  .p2align 4
  .type XmbTextPropertyToTextList, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XmbTextPropertyToTextList
#endif
XmbTextPropertyToTextList:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5752(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5752(%rip)
2:
  pushq $719
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl Xpermalloc
  .p2align 4
  .type Xpermalloc, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden Xpermalloc
#endif
Xpermalloc:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5760(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5760(%rip)
2:
  pushq $720
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XrmCombineDatabase
  .p2align 4
  .type XrmCombineDatabase, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XrmCombineDatabase
#endif
XrmCombineDatabase:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5768(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5768(%rip)
2:
  pushq $721
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XrmCombineFileDatabase
  .p2align 4
  .type XrmCombineFileDatabase, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XrmCombineFileDatabase
#endif
XrmCombineFileDatabase:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5776(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5776(%rip)
2:
  pushq $722
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XrmDestroyDatabase
  .p2align 4
  .type XrmDestroyDatabase, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XrmDestroyDatabase
#endif
XrmDestroyDatabase:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5784(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5784(%rip)
2:
  pushq $723
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XrmEnumerateDatabase
  .p2align 4
  .type XrmEnumerateDatabase, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XrmEnumerateDatabase
#endif
XrmEnumerateDatabase:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5792(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5792(%rip)
2:
  pushq $724
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XrmGetDatabase
  .p2align 4
  .type XrmGetDatabase, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XrmGetDatabase
#endif
XrmGetDatabase:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5800(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5800(%rip)
2:
  pushq $725
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XrmGetFileDatabase
  .p2align 4
  .type XrmGetFileDatabase, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XrmGetFileDatabase
#endif
XrmGetFileDatabase:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5808(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5808(%rip)
2:
  pushq $726
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XrmGetResource
  .p2align 4
  .type XrmGetResource, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XrmGetResource
#endif
XrmGetResource:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5816(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5816(%rip)
2:
  pushq $727
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XrmGetStringDatabase
  .p2align 4
  .type XrmGetStringDatabase, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XrmGetStringDatabase
#endif
XrmGetStringDatabase:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5824(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5824(%rip)
2:
  pushq $728
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XrmInitialize
  .p2align 4
  .type XrmInitialize, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XrmInitialize
#endif
XrmInitialize:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5832(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5832(%rip)
2:
  pushq $729
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XrmLocaleOfDatabase
  .p2align 4
  .type XrmLocaleOfDatabase, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XrmLocaleOfDatabase
#endif
XrmLocaleOfDatabase:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5840(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5840(%rip)
2:
  pushq $730
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XrmMergeDatabases
  .p2align 4
  .type XrmMergeDatabases, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XrmMergeDatabases
#endif
XrmMergeDatabases:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5848(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5848(%rip)
2:
  pushq $731
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XrmParseCommand
  .p2align 4
  .type XrmParseCommand, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XrmParseCommand
#endif
XrmParseCommand:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5856(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5856(%rip)
2:
  pushq $732
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XrmPermStringToQuark
  .p2align 4
  .type XrmPermStringToQuark, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XrmPermStringToQuark
#endif
XrmPermStringToQuark:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5864(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5864(%rip)
2:
  pushq $733
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XrmPutFileDatabase
  .p2align 4
  .type XrmPutFileDatabase, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XrmPutFileDatabase
#endif
XrmPutFileDatabase:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5872(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5872(%rip)
2:
  pushq $734
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XrmPutLineResource
  .p2align 4
  .type XrmPutLineResource, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XrmPutLineResource
#endif
XrmPutLineResource:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5880(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5880(%rip)
2:
  pushq $735
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XrmPutResource
  .p2align 4
  .type XrmPutResource, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XrmPutResource
#endif
XrmPutResource:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5888(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5888(%rip)
2:
  pushq $736
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XrmPutStringResource
  .p2align 4
  .type XrmPutStringResource, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XrmPutStringResource
#endif
XrmPutStringResource:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5896(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5896(%rip)
2:
  pushq $737
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XrmQGetResource
  .p2align 4
  .type XrmQGetResource, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XrmQGetResource
#endif
XrmQGetResource:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5904(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5904(%rip)
2:
  pushq $738
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XrmQGetSearchList
  .p2align 4
  .type XrmQGetSearchList, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XrmQGetSearchList
#endif
XrmQGetSearchList:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5912(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5912(%rip)
2:
  pushq $739
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XrmQGetSearchResource
  .p2align 4
  .type XrmQGetSearchResource, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XrmQGetSearchResource
#endif
XrmQGetSearchResource:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5920(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5920(%rip)
2:
  pushq $740
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XrmQPutResource
  .p2align 4
  .type XrmQPutResource, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XrmQPutResource
#endif
XrmQPutResource:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5928(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5928(%rip)
2:
  pushq $741
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XrmQPutStringResource
  .p2align 4
  .type XrmQPutStringResource, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XrmQPutStringResource
#endif
XrmQPutStringResource:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5936(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5936(%rip)
2:
  pushq $742
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XrmQuarkToString
  .p2align 4
  .type XrmQuarkToString, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XrmQuarkToString
#endif
XrmQuarkToString:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5944(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5944(%rip)
2:
  pushq $743
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XrmSetDatabase
  .p2align 4
  .type XrmSetDatabase, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XrmSetDatabase
#endif
XrmSetDatabase:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5952(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5952(%rip)
2:
  pushq $744
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XrmStringToBindingQuarkList
  .p2align 4
  .type XrmStringToBindingQuarkList, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XrmStringToBindingQuarkList
#endif
XrmStringToBindingQuarkList:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5960(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5960(%rip)
2:
  pushq $745
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XrmStringToQuark
  .p2align 4
  .type XrmStringToQuark, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XrmStringToQuark
#endif
XrmStringToQuark:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5968(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5968(%rip)
2:
  pushq $746
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XrmStringToQuarkList
  .p2align 4
  .type XrmStringToQuarkList, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XrmStringToQuarkList
#endif
XrmStringToQuarkList:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5976(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5976(%rip)
2:
  pushq $747
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XrmUniqueQuark
  .p2align 4
  .type XrmUniqueQuark, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XrmUniqueQuark
#endif
XrmUniqueQuark:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5984(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5984(%rip)
2:
  pushq $748
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl Xutf8DrawImageString
  .p2align 4
  .type Xutf8DrawImageString, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden Xutf8DrawImageString
#endif
Xutf8DrawImageString:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+5992(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+5992(%rip)
2:
  pushq $749
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl Xutf8DrawString
  .p2align 4
  .type Xutf8DrawString, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden Xutf8DrawString
#endif
Xutf8DrawString:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6000(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6000(%rip)
2:
  pushq $750
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl Xutf8DrawText
  .p2align 4
  .type Xutf8DrawText, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden Xutf8DrawText
#endif
Xutf8DrawText:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6008(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6008(%rip)
2:
  pushq $751
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl Xutf8LookupString
  .p2align 4
  .type Xutf8LookupString, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden Xutf8LookupString
#endif
Xutf8LookupString:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6016(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6016(%rip)
2:
  pushq $752
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl Xutf8ResetIC
  .p2align 4
  .type Xutf8ResetIC, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden Xutf8ResetIC
#endif
Xutf8ResetIC:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6024(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6024(%rip)
2:
  pushq $753
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl Xutf8SetWMProperties
  .p2align 4
  .type Xutf8SetWMProperties, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden Xutf8SetWMProperties
#endif
Xutf8SetWMProperties:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6032(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6032(%rip)
2:
  pushq $754
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl Xutf8TextEscapement
  .p2align 4
  .type Xutf8TextEscapement, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden Xutf8TextEscapement
#endif
Xutf8TextEscapement:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6040(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6040(%rip)
2:
  pushq $755
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl Xutf8TextExtents
  .p2align 4
  .type Xutf8TextExtents, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden Xutf8TextExtents
#endif
Xutf8TextExtents:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6048(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6048(%rip)
2:
  pushq $756
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl Xutf8TextListToTextProperty
  .p2align 4
  .type Xutf8TextListToTextProperty, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden Xutf8TextListToTextProperty
#endif
Xutf8TextListToTextProperty:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6056(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6056(%rip)
2:
  pushq $757
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl Xutf8TextPerCharExtents
  .p2align 4
  .type Xutf8TextPerCharExtents, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden Xutf8TextPerCharExtents
#endif
Xutf8TextPerCharExtents:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6064(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6064(%rip)
2:
  pushq $758
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl Xutf8TextPropertyToTextList
  .p2align 4
  .type Xutf8TextPropertyToTextList, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden Xutf8TextPropertyToTextList
#endif
Xutf8TextPropertyToTextList:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6072(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6072(%rip)
2:
  pushq $759
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XwcDrawImageString
  .p2align 4
  .type XwcDrawImageString, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XwcDrawImageString
#endif
XwcDrawImageString:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6080(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6080(%rip)
2:
  pushq $760
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XwcDrawString
  .p2align 4
  .type XwcDrawString, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XwcDrawString
#endif
XwcDrawString:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6088(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6088(%rip)
2:
  pushq $761
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XwcDrawText
  .p2align 4
  .type XwcDrawText, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XwcDrawText
#endif
XwcDrawText:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6096(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6096(%rip)
2:
  pushq $762
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XwcFreeStringList
  .p2align 4
  .type XwcFreeStringList, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XwcFreeStringList
#endif
XwcFreeStringList:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6104(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6104(%rip)
2:
  pushq $763
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XwcLookupString
  .p2align 4
  .type XwcLookupString, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XwcLookupString
#endif
XwcLookupString:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6112(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6112(%rip)
2:
  pushq $764
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XwcResetIC
  .p2align 4
  .type XwcResetIC, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XwcResetIC
#endif
XwcResetIC:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6120(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6120(%rip)
2:
  pushq $765
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XwcTextEscapement
  .p2align 4
  .type XwcTextEscapement, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XwcTextEscapement
#endif
XwcTextEscapement:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6128(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6128(%rip)
2:
  pushq $766
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XwcTextExtents
  .p2align 4
  .type XwcTextExtents, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XwcTextExtents
#endif
XwcTextExtents:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6136(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6136(%rip)
2:
  pushq $767
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XwcTextListToTextProperty
  .p2align 4
  .type XwcTextListToTextProperty, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XwcTextListToTextProperty
#endif
XwcTextListToTextProperty:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6144(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6144(%rip)
2:
  pushq $768
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XwcTextPerCharExtents
  .p2align 4
  .type XwcTextPerCharExtents, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XwcTextPerCharExtents
#endif
XwcTextPerCharExtents:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6152(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6152(%rip)
2:
  pushq $769
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl XwcTextPropertyToTextList
  .p2align 4
  .type XwcTextPropertyToTextList, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XwcTextPropertyToTextList
#endif
XwcTextPropertyToTextList:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6160(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6160(%rip)
2:
  pushq $770
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _Utf8GetConvByName
  .p2align 4
  .type _Utf8GetConvByName, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _Utf8GetConvByName
#endif
_Utf8GetConvByName:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6168(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6168(%rip)
2:
  pushq $771
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XAllocID
  .p2align 4
  .type _XAllocID, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XAllocID
#endif
_XAllocID:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6176(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6176(%rip)
2:
  pushq $772
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XAllocIDs
  .p2align 4
  .type _XAllocIDs, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XAllocIDs
#endif
_XAllocIDs:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6184(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6184(%rip)
2:
  pushq $773
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XAllocScratch
  .p2align 4
  .type _XAllocScratch, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XAllocScratch
#endif
_XAllocScratch:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6192(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6192(%rip)
2:
  pushq $774
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XAllocTemp
  .p2align 4
  .type _XAllocTemp, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XAllocTemp
#endif
_XAllocTemp:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6200(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6200(%rip)
2:
  pushq $775
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XAsyncErrorHandler
  .p2align 4
  .type _XAsyncErrorHandler, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XAsyncErrorHandler
#endif
_XAsyncErrorHandler:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6208(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6208(%rip)
2:
  pushq $776
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XCloseLC
  .p2align 4
  .type _XCloseLC, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XCloseLC
#endif
_XCloseLC:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6216(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6216(%rip)
2:
  pushq $777
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XColor_to_XcmsRGB
  .p2align 4
  .type _XColor_to_XcmsRGB, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XColor_to_XcmsRGB
#endif
_XColor_to_XcmsRGB:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6224(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6224(%rip)
2:
  pushq $778
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XConnectXCB
  .p2align 4
  .type _XConnectXCB, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XConnectXCB
#endif
_XConnectXCB:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6232(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6232(%rip)
2:
  pushq $779
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XCopyEventCookie
  .p2align 4
  .type _XCopyEventCookie, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XCopyEventCookie
#endif
_XCopyEventCookie:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6240(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6240(%rip)
2:
  pushq $780
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XCopyToArg
  .p2align 4
  .type _XCopyToArg, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XCopyToArg
#endif
_XCopyToArg:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6248(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6248(%rip)
2:
  pushq $781
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XData32
  .p2align 4
  .type _XData32, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XData32
#endif
_XData32:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6256(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6256(%rip)
2:
  pushq $782
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XDefaultError
  .p2align 4
  .type _XDefaultError, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XDefaultError
#endif
_XDefaultError:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6264(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6264(%rip)
2:
  pushq $783
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XDefaultIOError
  .p2align 4
  .type _XDefaultIOError, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XDefaultIOError
#endif
_XDefaultIOError:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6272(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6272(%rip)
2:
  pushq $784
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XDefaultIOErrorExit
  .p2align 4
  .type _XDefaultIOErrorExit, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XDefaultIOErrorExit
#endif
_XDefaultIOErrorExit:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6280(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6280(%rip)
2:
  pushq $785
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XDefaultOpenIM
  .p2align 4
  .type _XDefaultOpenIM, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XDefaultOpenIM
#endif
_XDefaultOpenIM:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6288(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6288(%rip)
2:
  pushq $786
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XDefaultOpenOM
  .p2align 4
  .type _XDefaultOpenOM, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XDefaultOpenOM
#endif
_XDefaultOpenOM:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6296(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6296(%rip)
2:
  pushq $787
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XDefaultWireError
  .p2align 4
  .type _XDefaultWireError, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XDefaultWireError
#endif
_XDefaultWireError:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6304(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6304(%rip)
2:
  pushq $788
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XDeq
  .p2align 4
  .type _XDeq, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XDeq
#endif
_XDeq:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6312(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6312(%rip)
2:
  pushq $789
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XDeqAsyncHandler
  .p2align 4
  .type _XDeqAsyncHandler, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XDeqAsyncHandler
#endif
_XDeqAsyncHandler:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6320(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6320(%rip)
2:
  pushq $790
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XEatData
  .p2align 4
  .type _XEatData, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XEatData
#endif
_XEatData:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6328(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6328(%rip)
2:
  pushq $791
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XEatDataWords
  .p2align 4
  .type _XEatDataWords, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XEatDataWords
#endif
_XEatDataWords:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6336(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6336(%rip)
2:
  pushq $792
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XEnq
  .p2align 4
  .type _XEnq, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XEnq
#endif
_XEnq:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6344(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6344(%rip)
2:
  pushq $793
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XError
  .p2align 4
  .type _XError, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XError
#endif
_XError:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6352(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6352(%rip)
2:
  pushq $794
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XEventToWire
  .p2align 4
  .type _XEventToWire, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XEventToWire
#endif
_XEventToWire:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6360(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6360(%rip)
2:
  pushq $795
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XEventsQueued
  .p2align 4
  .type _XEventsQueued, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XEventsQueued
#endif
_XEventsQueued:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6368(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6368(%rip)
2:
  pushq $796
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XF86BigfontFreeFontMetrics
  .p2align 4
  .type _XF86BigfontFreeFontMetrics, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XF86BigfontFreeFontMetrics
#endif
_XF86BigfontFreeFontMetrics:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6376(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6376(%rip)
2:
  pushq $797
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XF86LoadQueryLocaleFont
  .p2align 4
  .type _XF86LoadQueryLocaleFont, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XF86LoadQueryLocaleFont
#endif
_XF86LoadQueryLocaleFont:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6384(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6384(%rip)
2:
  pushq $798
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XFetchEventCookie
  .p2align 4
  .type _XFetchEventCookie, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XFetchEventCookie
#endif
_XFetchEventCookie:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6392(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6392(%rip)
2:
  pushq $799
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XFlush
  .p2align 4
  .type _XFlush, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XFlush
#endif
_XFlush:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6400(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6400(%rip)
2:
  pushq $800
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XFlushGCCache
  .p2align 4
  .type _XFlushGCCache, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XFlushGCCache
#endif
_XFlushGCCache:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6408(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6408(%rip)
2:
  pushq $801
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XFreeAtomTable
  .p2align 4
  .type _XFreeAtomTable, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XFreeAtomTable
#endif
_XFreeAtomTable:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6416(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6416(%rip)
2:
  pushq $802
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XFreeDisplayStructure
  .p2align 4
  .type _XFreeDisplayStructure, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XFreeDisplayStructure
#endif
_XFreeDisplayStructure:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6424(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6424(%rip)
2:
  pushq $803
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XFreeEventCookies
  .p2align 4
  .type _XFreeEventCookies, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XFreeEventCookies
#endif
_XFreeEventCookies:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6432(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6432(%rip)
2:
  pushq $804
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XFreeExtData
  .p2align 4
  .type _XFreeExtData, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XFreeExtData
#endif
_XFreeExtData:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6440(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6440(%rip)
2:
  pushq $805
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XFreeTemp
  .p2align 4
  .type _XFreeTemp, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XFreeTemp
#endif
_XFreeTemp:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6448(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6448(%rip)
2:
  pushq $806
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XFreeX11XCBStructure
  .p2align 4
  .type _XFreeX11XCBStructure, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XFreeX11XCBStructure
#endif
_XFreeX11XCBStructure:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6456(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6456(%rip)
2:
  pushq $807
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XGetAsyncData
  .p2align 4
  .type _XGetAsyncData, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XGetAsyncData
#endif
_XGetAsyncData:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6464(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6464(%rip)
2:
  pushq $808
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XGetAsyncReply
  .p2align 4
  .type _XGetAsyncReply, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XGetAsyncReply
#endif
_XGetAsyncReply:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6472(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6472(%rip)
2:
  pushq $809
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XGetBitsPerPixel
  .p2align 4
  .type _XGetBitsPerPixel, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XGetBitsPerPixel
#endif
_XGetBitsPerPixel:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6480(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6480(%rip)
2:
  pushq $810
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XGetHostname
  .p2align 4
  .type _XGetHostname, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XGetHostname
#endif
_XGetHostname:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6488(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6488(%rip)
2:
  pushq $811
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XGetLCValues
  .p2align 4
  .type _XGetLCValues, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XGetLCValues
#endif
_XGetLCValues:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6496(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6496(%rip)
2:
  pushq $812
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XGetRequest
  .p2align 4
  .type _XGetRequest, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XGetRequest
#endif
_XGetRequest:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6504(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6504(%rip)
2:
  pushq $813
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XGetScanlinePad
  .p2align 4
  .type _XGetScanlinePad, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XGetScanlinePad
#endif
_XGetScanlinePad:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6512(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6512(%rip)
2:
  pushq $814
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XGetWindowAttributes
  .p2align 4
  .type _XGetWindowAttributes, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XGetWindowAttributes
#endif
_XGetWindowAttributes:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6520(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6520(%rip)
2:
  pushq $815
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XIMCompileResourceList
  .p2align 4
  .type _XIMCompileResourceList, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XIMCompileResourceList
#endif
_XIMCompileResourceList:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6528(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6528(%rip)
2:
  pushq $816
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XIOError
  .p2align 4
  .type _XIOError, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XIOError
#endif
_XIOError:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6536(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6536(%rip)
2:
  pushq $817
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XInitIM
  .p2align 4
  .type _XInitIM, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XInitIM
#endif
_XInitIM:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6544(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6544(%rip)
2:
  pushq $818
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XInitImageFuncPtrs
  .p2align 4
  .type _XInitImageFuncPtrs, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XInitImageFuncPtrs
#endif
_XInitImageFuncPtrs:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6552(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6552(%rip)
2:
  pushq $819
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XInitKeysymDB
  .p2align 4
  .type _XInitKeysymDB, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XInitKeysymDB
#endif
_XInitKeysymDB:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6560(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6560(%rip)
2:
  pushq $820
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XInitOM
  .p2align 4
  .type _XInitOM, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XInitOM
#endif
_XInitOM:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6568(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6568(%rip)
2:
  pushq $821
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XIsEventCookie
  .p2align 4
  .type _XIsEventCookie, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XIsEventCookie
#endif
_XIsEventCookie:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6576(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6576(%rip)
2:
  pushq $822
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XKeyInitialize
  .p2align 4
  .type _XKeyInitialize, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XKeyInitialize
#endif
_XKeyInitialize:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6584(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6584(%rip)
2:
  pushq $823
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XKeycodeToKeysym
  .p2align 4
  .type _XKeycodeToKeysym, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XKeycodeToKeysym
#endif
_XKeycodeToKeysym:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6592(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6592(%rip)
2:
  pushq $824
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XKeysymToKeycode
  .p2align 4
  .type _XKeysymToKeycode, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XKeysymToKeycode
#endif
_XKeysymToKeycode:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6600(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6600(%rip)
2:
  pushq $825
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XKeysymToModifiers
  .p2align 4
  .type _XKeysymToModifiers, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XKeysymToModifiers
#endif
_XKeysymToModifiers:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6608(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6608(%rip)
2:
  pushq $826
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XLookupKeysym
  .p2align 4
  .type _XLookupKeysym, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XLookupKeysym
#endif
_XLookupKeysym:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6616(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6616(%rip)
2:
  pushq $827
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XLookupString
  .p2align 4
  .type _XLookupString, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XLookupString
#endif
_XLookupString:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6624(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6624(%rip)
2:
  pushq $828
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XNoticeCreateBitmap
  .p2align 4
  .type _XNoticeCreateBitmap, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XNoticeCreateBitmap
#endif
_XNoticeCreateBitmap:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6632(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6632(%rip)
2:
  pushq $829
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XNoticePutBitmap
  .p2align 4
  .type _XNoticePutBitmap, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XNoticePutBitmap
#endif
_XNoticePutBitmap:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6640(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6640(%rip)
2:
  pushq $830
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XOpenLC
  .p2align 4
  .type _XOpenLC, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XOpenLC
#endif
_XOpenLC:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6648(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6648(%rip)
2:
  pushq $831
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XParseBaseFontNameList
  .p2align 4
  .type _XParseBaseFontNameList, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XParseBaseFontNameList
#endif
_XParseBaseFontNameList:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6656(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6656(%rip)
2:
  pushq $832
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XPollfdCacheAdd
  .p2align 4
  .type _XPollfdCacheAdd, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XPollfdCacheAdd
#endif
_XPollfdCacheAdd:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6664(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6664(%rip)
2:
  pushq $833
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XPollfdCacheDel
  .p2align 4
  .type _XPollfdCacheDel, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XPollfdCacheDel
#endif
_XPollfdCacheDel:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6672(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6672(%rip)
2:
  pushq $834
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XPollfdCacheInit
  .p2align 4
  .type _XPollfdCacheInit, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XPollfdCacheInit
#endif
_XPollfdCacheInit:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6680(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6680(%rip)
2:
  pushq $835
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XProcessInternalConnection
  .p2align 4
  .type _XProcessInternalConnection, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XProcessInternalConnection
#endif
_XProcessInternalConnection:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6688(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6688(%rip)
2:
  pushq $836
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XProcessWindowAttributes
  .p2align 4
  .type _XProcessWindowAttributes, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XProcessWindowAttributes
#endif
_XProcessWindowAttributes:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6696(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6696(%rip)
2:
  pushq $837
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XPutBackEvent
  .p2align 4
  .type _XPutBackEvent, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XPutBackEvent
#endif
_XPutBackEvent:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6704(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6704(%rip)
2:
  pushq $838
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XRead
  .p2align 4
  .type _XRead, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XRead
#endif
_XRead:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6712(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6712(%rip)
2:
  pushq $839
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XRead32
  .p2align 4
  .type _XRead32, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XRead32
#endif
_XRead32:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6720(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6720(%rip)
2:
  pushq $840
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XReadEvents
  .p2align 4
  .type _XReadEvents, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XReadEvents
#endif
_XReadEvents:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6728(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6728(%rip)
2:
  pushq $841
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XReadPad
  .p2align 4
  .type _XReadPad, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XReadPad
#endif
_XReadPad:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6736(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6736(%rip)
2:
  pushq $842
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XRefreshKeyboardMapping
  .p2align 4
  .type _XRefreshKeyboardMapping, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XRefreshKeyboardMapping
#endif
_XRefreshKeyboardMapping:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6744(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6744(%rip)
2:
  pushq $843
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XRegisterFilterByMask
  .p2align 4
  .type _XRegisterFilterByMask, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XRegisterFilterByMask
#endif
_XRegisterFilterByMask:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6752(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6752(%rip)
2:
  pushq $844
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XRegisterFilterByType
  .p2align 4
  .type _XRegisterFilterByType, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XRegisterFilterByType
#endif
_XRegisterFilterByType:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6760(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6760(%rip)
2:
  pushq $845
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XRegisterInternalConnection
  .p2align 4
  .type _XRegisterInternalConnection, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XRegisterInternalConnection
#endif
_XRegisterInternalConnection:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6768(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6768(%rip)
2:
  pushq $846
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XReply
  .p2align 4
  .type _XReply, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XReply
#endif
_XReply:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6776(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6776(%rip)
2:
  pushq $847
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XReverse_Bytes
  .p2align 4
  .type _XReverse_Bytes, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XReverse_Bytes
#endif
_XReverse_Bytes:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6784(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6784(%rip)
2:
  pushq $848
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XScreenOfWindow
  .p2align 4
  .type _XScreenOfWindow, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XScreenOfWindow
#endif
_XScreenOfWindow:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6792(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6792(%rip)
2:
  pushq $849
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XSend
  .p2align 4
  .type _XSend, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XSend
#endif
_XSend:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6800(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6800(%rip)
2:
  pushq $850
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XSetClipRectangles
  .p2align 4
  .type _XSetClipRectangles, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XSetClipRectangles
#endif
_XSetClipRectangles:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6808(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6808(%rip)
2:
  pushq $851
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XSetImage
  .p2align 4
  .type _XSetImage, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XSetImage
#endif
_XSetImage:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6816(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6816(%rip)
2:
  pushq $852
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XSetLastRequestRead
  .p2align 4
  .type _XSetLastRequestRead, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XSetLastRequestRead
#endif
_XSetLastRequestRead:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6824(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6824(%rip)
2:
  pushq $853
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XStoreEventCookie
  .p2align 4
  .type _XStoreEventCookie, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XStoreEventCookie
#endif
_XStoreEventCookie:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6832(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6832(%rip)
2:
  pushq $854
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XTextHeight
  .p2align 4
  .type _XTextHeight, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XTextHeight
#endif
_XTextHeight:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6840(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6840(%rip)
2:
  pushq $855
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XTextHeight16
  .p2align 4
  .type _XTextHeight16, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XTextHeight16
#endif
_XTextHeight16:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6848(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6848(%rip)
2:
  pushq $856
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XTranslateKey
  .p2align 4
  .type _XTranslateKey, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XTranslateKey
#endif
_XTranslateKey:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6856(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6856(%rip)
2:
  pushq $857
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XTranslateKeySym
  .p2align 4
  .type _XTranslateKeySym, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XTranslateKeySym
#endif
_XTranslateKeySym:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6864(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6864(%rip)
2:
  pushq $858
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XTryShapeBitmapCursor
  .p2align 4
  .type _XTryShapeBitmapCursor, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XTryShapeBitmapCursor
#endif
_XTryShapeBitmapCursor:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6872(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6872(%rip)
2:
  pushq $859
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XUnknownCopyEventCookie
  .p2align 4
  .type _XUnknownCopyEventCookie, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XUnknownCopyEventCookie
#endif
_XUnknownCopyEventCookie:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6880(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6880(%rip)
2:
  pushq $860
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XUnknownNativeEvent
  .p2align 4
  .type _XUnknownNativeEvent, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XUnknownNativeEvent
#endif
_XUnknownNativeEvent:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6888(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6888(%rip)
2:
  pushq $861
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XUnknownWireEvent
  .p2align 4
  .type _XUnknownWireEvent, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XUnknownWireEvent
#endif
_XUnknownWireEvent:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6896(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6896(%rip)
2:
  pushq $862
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XUnknownWireEventCookie
  .p2align 4
  .type _XUnknownWireEventCookie, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XUnknownWireEventCookie
#endif
_XUnknownWireEventCookie:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6904(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6904(%rip)
2:
  pushq $863
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XUnregisterFilter
  .p2align 4
  .type _XUnregisterFilter, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XUnregisterFilter
#endif
_XUnregisterFilter:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6912(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6912(%rip)
2:
  pushq $864
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XUnregisterInternalConnection
  .p2align 4
  .type _XUnregisterInternalConnection, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XUnregisterInternalConnection
#endif
_XUnregisterInternalConnection:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6920(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6920(%rip)
2:
  pushq $865
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XUnresolveColor
  .p2align 4
  .type _XUnresolveColor, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XUnresolveColor
#endif
_XUnresolveColor:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6928(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6928(%rip)
2:
  pushq $866
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XUpdateAtomCache
  .p2align 4
  .type _XUpdateAtomCache, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XUpdateAtomCache
#endif
_XUpdateAtomCache:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6936(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6936(%rip)
2:
  pushq $867
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XUpdateGCCache
  .p2align 4
  .type _XUpdateGCCache, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XUpdateGCCache
#endif
_XUpdateGCCache:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6944(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6944(%rip)
2:
  pushq $868
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XVIDtoVisual
  .p2align 4
  .type _XVIDtoVisual, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XVIDtoVisual
#endif
_XVIDtoVisual:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6952(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6952(%rip)
2:
  pushq $869
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XWireToEvent
  .p2align 4
  .type _XWireToEvent, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XWireToEvent
#endif
_XWireToEvent:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6960(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6960(%rip)
2:
  pushq $870
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XcmsAddCmapRec
  .p2align 4
  .type _XcmsAddCmapRec, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XcmsAddCmapRec
#endif
_XcmsAddCmapRec:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6968(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6968(%rip)
2:
  pushq $871
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XcmsArcTangent
  .p2align 4
  .type _XcmsArcTangent, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XcmsArcTangent
#endif
_XcmsArcTangent:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6976(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6976(%rip)
2:
  pushq $872
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XcmsCIELabQueryMaxLCRGB
  .p2align 4
  .type _XcmsCIELabQueryMaxLCRGB, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XcmsCIELabQueryMaxLCRGB
#endif
_XcmsCIELabQueryMaxLCRGB:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6984(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6984(%rip)
2:
  pushq $873
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XcmsCIELuvQueryMaxLCRGB
  .p2align 4
  .type _XcmsCIELuvQueryMaxLCRGB, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XcmsCIELuvQueryMaxLCRGB
#endif
_XcmsCIELuvQueryMaxLCRGB:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+6992(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+6992(%rip)
2:
  pushq $874
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XcmsCIEXYZ_ValidSpec
  .p2align 4
  .type _XcmsCIEXYZ_ValidSpec, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XcmsCIEXYZ_ValidSpec
#endif
_XcmsCIEXYZ_ValidSpec:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7000(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7000(%rip)
2:
  pushq $875
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XcmsCIEuvY_ValidSpec
  .p2align 4
  .type _XcmsCIEuvY_ValidSpec, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XcmsCIEuvY_ValidSpec
#endif
_XcmsCIEuvY_ValidSpec:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7008(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7008(%rip)
2:
  pushq $876
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XcmsConvertColorsWithWhitePt
  .p2align 4
  .type _XcmsConvertColorsWithWhitePt, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XcmsConvertColorsWithWhitePt
#endif
_XcmsConvertColorsWithWhitePt:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7016(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7016(%rip)
2:
  pushq $877
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XcmsCopyCmapRecAndFree
  .p2align 4
  .type _XcmsCopyCmapRecAndFree, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XcmsCopyCmapRecAndFree
#endif
_XcmsCopyCmapRecAndFree:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7024(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7024(%rip)
2:
  pushq $878
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XcmsCopyISOLatin1Lowered
  .p2align 4
  .type _XcmsCopyISOLatin1Lowered, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XcmsCopyISOLatin1Lowered
#endif
_XcmsCopyISOLatin1Lowered:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7032(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7032(%rip)
2:
  pushq $879
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XcmsCopyPointerArray
  .p2align 4
  .type _XcmsCopyPointerArray, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XcmsCopyPointerArray
#endif
_XcmsCopyPointerArray:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7040(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7040(%rip)
2:
  pushq $880
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XcmsCosine
  .p2align 4
  .type _XcmsCosine, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XcmsCosine
#endif
_XcmsCosine:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7048(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7048(%rip)
2:
  pushq $881
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XcmsCubeRoot
  .p2align 4
  .type _XcmsCubeRoot, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XcmsCubeRoot
#endif
_XcmsCubeRoot:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7056(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7056(%rip)
2:
  pushq $882
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XcmsDDConvertColors
  .p2align 4
  .type _XcmsDDConvertColors, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XcmsDDConvertColors
#endif
_XcmsDDConvertColors:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7064(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7064(%rip)
2:
  pushq $883
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XcmsDIConvertColors
  .p2align 4
  .type _XcmsDIConvertColors, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XcmsDIConvertColors
#endif
_XcmsDIConvertColors:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7072(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7072(%rip)
2:
  pushq $884
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XcmsDeleteCmapRec
  .p2align 4
  .type _XcmsDeleteCmapRec, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XcmsDeleteCmapRec
#endif
_XcmsDeleteCmapRec:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7080(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7080(%rip)
2:
  pushq $885
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XcmsEqualWhitePts
  .p2align 4
  .type _XcmsEqualWhitePts, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XcmsEqualWhitePts
#endif
_XcmsEqualWhitePts:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7088(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7088(%rip)
2:
  pushq $886
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XcmsFreeIntensityMaps
  .p2align 4
  .type _XcmsFreeIntensityMaps, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XcmsFreeIntensityMaps
#endif
_XcmsFreeIntensityMaps:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7096(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7096(%rip)
2:
  pushq $887
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XcmsFreePointerArray
  .p2align 4
  .type _XcmsFreePointerArray, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XcmsFreePointerArray
#endif
_XcmsFreePointerArray:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7104(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7104(%rip)
2:
  pushq $888
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XcmsGetElement
  .p2align 4
  .type _XcmsGetElement, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XcmsGetElement
#endif
_XcmsGetElement:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7112(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7112(%rip)
2:
  pushq $889
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XcmsGetIntensityMap
  .p2align 4
  .type _XcmsGetIntensityMap, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XcmsGetIntensityMap
#endif
_XcmsGetIntensityMap:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7120(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7120(%rip)
2:
  pushq $890
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XcmsGetProperty
  .p2align 4
  .type _XcmsGetProperty, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XcmsGetProperty
#endif
_XcmsGetProperty:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7128(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7128(%rip)
2:
  pushq $891
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XcmsInitDefaultCCCs
  .p2align 4
  .type _XcmsInitDefaultCCCs, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XcmsInitDefaultCCCs
#endif
_XcmsInitDefaultCCCs:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7136(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7136(%rip)
2:
  pushq $892
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XcmsInitScrnInfo
  .p2align 4
  .type _XcmsInitScrnInfo, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XcmsInitScrnInfo
#endif
_XcmsInitScrnInfo:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7144(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7144(%rip)
2:
  pushq $893
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XcmsLRGB_InitScrnDefault
  .p2align 4
  .type _XcmsLRGB_InitScrnDefault, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XcmsLRGB_InitScrnDefault
#endif
_XcmsLRGB_InitScrnDefault:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7152(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7152(%rip)
2:
  pushq $894
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XcmsPushPointerArray
  .p2align 4
  .type _XcmsPushPointerArray, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XcmsPushPointerArray
#endif
_XcmsPushPointerArray:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7160(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7160(%rip)
2:
  pushq $895
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XcmsRGB_to_XColor
  .p2align 4
  .type _XcmsRGB_to_XColor, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XcmsRGB_to_XColor
#endif
_XcmsRGB_to_XColor:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7168(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7168(%rip)
2:
  pushq $896
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XcmsRegFormatOfPrefix
  .p2align 4
  .type _XcmsRegFormatOfPrefix, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XcmsRegFormatOfPrefix
#endif
_XcmsRegFormatOfPrefix:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7176(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7176(%rip)
2:
  pushq $897
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XcmsResolveColor
  .p2align 4
  .type _XcmsResolveColor, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XcmsResolveColor
#endif
_XcmsResolveColor:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7184(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7184(%rip)
2:
  pushq $898
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XcmsResolveColorString
  .p2align 4
  .type _XcmsResolveColorString, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XcmsResolveColorString
#endif
_XcmsResolveColorString:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7192(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7192(%rip)
2:
  pushq $899
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XcmsSetGetColor
  .p2align 4
  .type _XcmsSetGetColor, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XcmsSetGetColor
#endif
_XcmsSetGetColor:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7200(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7200(%rip)
2:
  pushq $900
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XcmsSetGetColors
  .p2align 4
  .type _XcmsSetGetColors, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XcmsSetGetColors
#endif
_XcmsSetGetColors:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7208(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7208(%rip)
2:
  pushq $901
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XcmsSine
  .p2align 4
  .type _XcmsSine, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XcmsSine
#endif
_XcmsSine:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7216(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7216(%rip)
2:
  pushq $902
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XcmsSquareRoot
  .p2align 4
  .type _XcmsSquareRoot, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XcmsSquareRoot
#endif
_XcmsSquareRoot:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7224(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7224(%rip)
2:
  pushq $903
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XcmsTekHVCQueryMaxVCRGB
  .p2align 4
  .type _XcmsTekHVCQueryMaxVCRGB, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XcmsTekHVCQueryMaxVCRGB
#endif
_XcmsTekHVCQueryMaxVCRGB:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7232(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7232(%rip)
2:
  pushq $904
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XcmsTekHVC_CheckModify
  .p2align 4
  .type _XcmsTekHVC_CheckModify, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XcmsTekHVC_CheckModify
#endif
_XcmsTekHVC_CheckModify:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7240(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7240(%rip)
2:
  pushq $905
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XcmsUnresolveColor
  .p2align 4
  .type _XcmsUnresolveColor, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XcmsUnresolveColor
#endif
_XcmsUnresolveColor:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7248(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7248(%rip)
2:
  pushq $906
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimCbDispatch
  .p2align 4
  .type _XimCbDispatch, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimCbDispatch
#endif
_XimCbDispatch:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7256(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7256(%rip)
2:
  pushq $907
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimCheckCreateICValues
  .p2align 4
  .type _XimCheckCreateICValues, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimCheckCreateICValues
#endif
_XimCheckCreateICValues:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7264(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7264(%rip)
2:
  pushq $908
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimCheckDataSize
  .p2align 4
  .type _XimCheckDataSize, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimCheckDataSize
#endif
_XimCheckDataSize:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7272(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7272(%rip)
2:
  pushq $909
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimCheckICMode
  .p2align 4
  .type _XimCheckICMode, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimCheckICMode
#endif
_XimCheckICMode:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7280(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7280(%rip)
2:
  pushq $910
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimCheckIMMode
  .p2align 4
  .type _XimCheckIMMode, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimCheckIMMode
#endif
_XimCheckIMMode:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7288(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7288(%rip)
2:
  pushq $911
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimCheckIfLocalProcessing
  .p2align 4
  .type _XimCheckIfLocalProcessing, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimCheckIfLocalProcessing
#endif
_XimCheckIfLocalProcessing:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7296(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7296(%rip)
2:
  pushq $912
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimCheckIfThaiProcessing
  .p2align 4
  .type _XimCheckIfThaiProcessing, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimCheckIfThaiProcessing
#endif
_XimCheckIfThaiProcessing:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7304(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7304(%rip)
2:
  pushq $913
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimCheckLocalInputStyle
  .p2align 4
  .type _XimCheckLocalInputStyle, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimCheckLocalInputStyle
#endif
_XimCheckLocalInputStyle:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7312(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7312(%rip)
2:
  pushq $914
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimCommitCallback
  .p2align 4
  .type _XimCommitCallback, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimCommitCallback
#endif
_XimCommitCallback:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7320(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7320(%rip)
2:
  pushq $915
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimConnect
  .p2align 4
  .type _XimConnect, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimConnect
#endif
_XimConnect:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7328(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7328(%rip)
2:
  pushq $916
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimDecodeICATTRIBUTE
  .p2align 4
  .type _XimDecodeICATTRIBUTE, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimDecodeICATTRIBUTE
#endif
_XimDecodeICATTRIBUTE:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7336(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7336(%rip)
2:
  pushq $917
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimDecodeIMATTRIBUTE
  .p2align 4
  .type _XimDecodeIMATTRIBUTE, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimDecodeIMATTRIBUTE
#endif
_XimDecodeIMATTRIBUTE:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7344(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7344(%rip)
2:
  pushq $918
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimDecodeLocalICAttr
  .p2align 4
  .type _XimDecodeLocalICAttr, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimDecodeLocalICAttr
#endif
_XimDecodeLocalICAttr:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7352(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7352(%rip)
2:
  pushq $919
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimDecodeLocalIMAttr
  .p2align 4
  .type _XimDecodeLocalIMAttr, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimDecodeLocalIMAttr
#endif
_XimDecodeLocalIMAttr:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7360(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7360(%rip)
2:
  pushq $920
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimDestroyIMStructureList
  .p2align 4
  .type _XimDestroyIMStructureList, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimDestroyIMStructureList
#endif
_XimDestroyIMStructureList:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7368(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7368(%rip)
2:
  pushq $921
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimDispatchInit
  .p2align 4
  .type _XimDispatchInit, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimDispatchInit
#endif
_XimDispatchInit:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7376(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7376(%rip)
2:
  pushq $922
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimEncodeICATTRIBUTE
  .p2align 4
  .type _XimEncodeICATTRIBUTE, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimEncodeICATTRIBUTE
#endif
_XimEncodeICATTRIBUTE:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7384(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7384(%rip)
2:
  pushq $923
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimEncodeIMATTRIBUTE
  .p2align 4
  .type _XimEncodeIMATTRIBUTE, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimEncodeIMATTRIBUTE
#endif
_XimEncodeIMATTRIBUTE:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7392(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7392(%rip)
2:
  pushq $924
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimEncodeLocalICAttr
  .p2align 4
  .type _XimEncodeLocalICAttr, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimEncodeLocalICAttr
#endif
_XimEncodeLocalICAttr:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7400(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7400(%rip)
2:
  pushq $925
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimEncodeLocalIMAttr
  .p2align 4
  .type _XimEncodeLocalIMAttr, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimEncodeLocalIMAttr
#endif
_XimEncodeLocalIMAttr:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7408(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7408(%rip)
2:
  pushq $926
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimError
  .p2align 4
  .type _XimError, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimError
#endif
_XimError:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7416(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7416(%rip)
2:
  pushq $927
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimErrorCallback
  .p2align 4
  .type _XimErrorCallback, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimErrorCallback
#endif
_XimErrorCallback:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7424(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7424(%rip)
2:
  pushq $928
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimExtension
  .p2align 4
  .type _XimExtension, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimExtension
#endif
_XimExtension:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7432(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7432(%rip)
2:
  pushq $929
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimFilterWaitEvent
  .p2align 4
  .type _XimFilterWaitEvent, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimFilterWaitEvent
#endif
_XimFilterWaitEvent:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7440(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7440(%rip)
2:
  pushq $930
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimFlush
  .p2align 4
  .type _XimFlush, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimFlush
#endif
_XimFlush:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7448(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7448(%rip)
2:
  pushq $931
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimForwardEvent
  .p2align 4
  .type _XimForwardEvent, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimForwardEvent
#endif
_XimForwardEvent:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7456(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7456(%rip)
2:
  pushq $932
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimForwardEventCallback
  .p2align 4
  .type _XimForwardEventCallback, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimForwardEventCallback
#endif
_XimForwardEventCallback:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7464(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7464(%rip)
2:
  pushq $933
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimFreeCommitInfo
  .p2align 4
  .type _XimFreeCommitInfo, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimFreeCommitInfo
#endif
_XimFreeCommitInfo:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7472(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7472(%rip)
2:
  pushq $934
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimFreeProtoIntrCallback
  .p2align 4
  .type _XimFreeProtoIntrCallback, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimFreeProtoIntrCallback
#endif
_XimFreeProtoIntrCallback:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7480(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7480(%rip)
2:
  pushq $935
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimFreeTransIntrCallback
  .p2align 4
  .type _XimFreeTransIntrCallback, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimFreeTransIntrCallback
#endif
_XimFreeTransIntrCallback:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7488(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7488(%rip)
2:
  pushq $936
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimGetAttributeID
  .p2align 4
  .type _XimGetAttributeID, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimGetAttributeID
#endif
_XimGetAttributeID:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7496(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7496(%rip)
2:
  pushq $937
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimGetCharCode
  .p2align 4
  .type _XimGetCharCode, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimGetCharCode
#endif
_XimGetCharCode:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7504(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7504(%rip)
2:
  pushq $938
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimGetCurrentICValues
  .p2align 4
  .type _XimGetCurrentICValues, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimGetCurrentICValues
#endif
_XimGetCurrentICValues:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7512(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7512(%rip)
2:
  pushq $939
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimGetCurrentIMValues
  .p2align 4
  .type _XimGetCurrentIMValues, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimGetCurrentIMValues
#endif
_XimGetCurrentIMValues:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7520(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7520(%rip)
2:
  pushq $940
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimGetICValueData
  .p2align 4
  .type _XimGetICValueData, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimGetICValueData
#endif
_XimGetICValueData:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7528(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7528(%rip)
2:
  pushq $941
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimGetIMValueData
  .p2align 4
  .type _XimGetIMValueData, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimGetIMValueData
#endif
_XimGetIMValueData:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7536(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7536(%rip)
2:
  pushq $942
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimGetLocaleCode
  .p2align 4
  .type _XimGetLocaleCode, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimGetLocaleCode
#endif
_XimGetLocaleCode:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7544(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7544(%rip)
2:
  pushq $943
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimGetMyEndian
  .p2align 4
  .type _XimGetMyEndian, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimGetMyEndian
#endif
_XimGetMyEndian:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7552(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7552(%rip)
2:
  pushq $944
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimGetResourceListRec
  .p2align 4
  .type _XimGetResourceListRec, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimGetResourceListRec
#endif
_XimGetResourceListRec:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7560(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7560(%rip)
2:
  pushq $945
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimGetResourceListRecByQuark
  .p2align 4
  .type _XimGetResourceListRecByQuark, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimGetResourceListRecByQuark
#endif
_XimGetResourceListRecByQuark:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7568(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7568(%rip)
2:
  pushq $946
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimGetWindowEventmask
  .p2align 4
  .type _XimGetWindowEventmask, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimGetWindowEventmask
#endif
_XimGetWindowEventmask:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7576(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7576(%rip)
2:
  pushq $947
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimICOfXICID
  .p2align 4
  .type _XimICOfXICID, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimICOfXICID
#endif
_XimICOfXICID:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7584(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7584(%rip)
2:
  pushq $948
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimInitialResourceInfo
  .p2align 4
  .type _XimInitialResourceInfo, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimInitialResourceInfo
#endif
_XimInitialResourceInfo:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7592(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7592(%rip)
2:
  pushq $949
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimLcctstombs
  .p2align 4
  .type _XimLcctstombs, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimLcctstombs
#endif
_XimLcctstombs:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7600(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7600(%rip)
2:
  pushq $950
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimLcctstoutf8
  .p2align 4
  .type _XimLcctstoutf8, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimLcctstoutf8
#endif
_XimLcctstoutf8:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7608(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7608(%rip)
2:
  pushq $951
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimLcctstowcs
  .p2align 4
  .type _XimLcctstowcs, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimLcctstowcs
#endif
_XimLcctstowcs:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7616(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7616(%rip)
2:
  pushq $952
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimLocalCreateIC
  .p2align 4
  .type _XimLocalCreateIC, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimLocalCreateIC
#endif
_XimLocalCreateIC:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7624(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7624(%rip)
2:
  pushq $953
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimLocalFilter
  .p2align 4
  .type _XimLocalFilter, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimLocalFilter
#endif
_XimLocalFilter:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7632(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7632(%rip)
2:
  pushq $954
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimLocalGetICValues
  .p2align 4
  .type _XimLocalGetICValues, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimLocalGetICValues
#endif
_XimLocalGetICValues:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7640(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7640(%rip)
2:
  pushq $955
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimLocalGetIMValues
  .p2align 4
  .type _XimLocalGetIMValues, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimLocalGetIMValues
#endif
_XimLocalGetIMValues:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7648(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7648(%rip)
2:
  pushq $956
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimLocalIMFree
  .p2align 4
  .type _XimLocalIMFree, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimLocalIMFree
#endif
_XimLocalIMFree:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7656(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7656(%rip)
2:
  pushq $957
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimLocalMbLookupString
  .p2align 4
  .type _XimLocalMbLookupString, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimLocalMbLookupString
#endif
_XimLocalMbLookupString:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7664(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7664(%rip)
2:
  pushq $958
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimLocalOpenIM
  .p2align 4
  .type _XimLocalOpenIM, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimLocalOpenIM
#endif
_XimLocalOpenIM:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7672(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7672(%rip)
2:
  pushq $959
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimLocalSetICValues
  .p2align 4
  .type _XimLocalSetICValues, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimLocalSetICValues
#endif
_XimLocalSetICValues:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7680(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7680(%rip)
2:
  pushq $960
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimLocalSetIMValues
  .p2align 4
  .type _XimLocalSetIMValues, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimLocalSetIMValues
#endif
_XimLocalSetIMValues:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7688(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7688(%rip)
2:
  pushq $961
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimLocalUtf8LookupString
  .p2align 4
  .type _XimLocalUtf8LookupString, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimLocalUtf8LookupString
#endif
_XimLocalUtf8LookupString:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7696(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7696(%rip)
2:
  pushq $962
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimLocalWcLookupString
  .p2align 4
  .type _XimLocalWcLookupString, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimLocalWcLookupString
#endif
_XimLocalWcLookupString:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7704(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7704(%rip)
2:
  pushq $963
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimLookupMBText
  .p2align 4
  .type _XimLookupMBText, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimLookupMBText
#endif
_XimLookupMBText:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7712(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7712(%rip)
2:
  pushq $964
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimLookupUTF8Text
  .p2align 4
  .type _XimLookupUTF8Text, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimLookupUTF8Text
#endif
_XimLookupUTF8Text:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7720(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7720(%rip)
2:
  pushq $965
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimLookupWCText
  .p2align 4
  .type _XimLookupWCText, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimLookupWCText
#endif
_XimLookupWCText:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7728(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7728(%rip)
2:
  pushq $966
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimMakeICAttrIDList
  .p2align 4
  .type _XimMakeICAttrIDList, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimMakeICAttrIDList
#endif
_XimMakeICAttrIDList:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7736(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7736(%rip)
2:
  pushq $967
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimMakeIMAttrIDList
  .p2align 4
  .type _XimMakeIMAttrIDList, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimMakeIMAttrIDList
#endif
_XimMakeIMAttrIDList:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7744(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7744(%rip)
2:
  pushq $968
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimOpenIM
  .p2align 4
  .type _XimOpenIM, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimOpenIM
#endif
_XimOpenIM:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7752(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7752(%rip)
2:
  pushq $969
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimParseStringFile
  .p2align 4
  .type _XimParseStringFile, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimParseStringFile
#endif
_XimParseStringFile:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7760(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7760(%rip)
2:
  pushq $970
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimProcError
  .p2align 4
  .type _XimProcError, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimProcError
#endif
_XimProcError:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7768(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7768(%rip)
2:
  pushq $971
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimProcSyncReply
  .p2align 4
  .type _XimProcSyncReply, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimProcSyncReply
#endif
_XimProcSyncReply:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7776(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7776(%rip)
2:
  pushq $972
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimProtoCreateIC
  .p2align 4
  .type _XimProtoCreateIC, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimProtoCreateIC
#endif
_XimProtoCreateIC:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7784(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7784(%rip)
2:
  pushq $973
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimProtoEventToWire
  .p2align 4
  .type _XimProtoEventToWire, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimProtoEventToWire
#endif
_XimProtoEventToWire:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7792(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7792(%rip)
2:
  pushq $974
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimProtoIMFree
  .p2align 4
  .type _XimProtoIMFree, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimProtoIMFree
#endif
_XimProtoIMFree:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7800(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7800(%rip)
2:
  pushq $975
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimProtoMbLookupString
  .p2align 4
  .type _XimProtoMbLookupString, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimProtoMbLookupString
#endif
_XimProtoMbLookupString:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7808(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7808(%rip)
2:
  pushq $976
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimProtoOpenIM
  .p2align 4
  .type _XimProtoOpenIM, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimProtoOpenIM
#endif
_XimProtoOpenIM:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7816(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7816(%rip)
2:
  pushq $977
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimProtoUtf8LookupString
  .p2align 4
  .type _XimProtoUtf8LookupString, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimProtoUtf8LookupString
#endif
_XimProtoUtf8LookupString:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7824(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7824(%rip)
2:
  pushq $978
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimProtoWcLookupString
  .p2align 4
  .type _XimProtoWcLookupString, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimProtoWcLookupString
#endif
_XimProtoWcLookupString:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7832(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7832(%rip)
2:
  pushq $979
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimProtoWireToEvent
  .p2align 4
  .type _XimProtoWireToEvent, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimProtoWireToEvent
#endif
_XimProtoWireToEvent:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7840(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7840(%rip)
2:
  pushq $980
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimRead
  .p2align 4
  .type _XimRead, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimRead
#endif
_XimRead:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7848(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7848(%rip)
2:
  pushq $981
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimRegProtoIntrCallback
  .p2align 4
  .type _XimRegProtoIntrCallback, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimRegProtoIntrCallback
#endif
_XimRegProtoIntrCallback:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7856(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7856(%rip)
2:
  pushq $982
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimRegisterDispatcher
  .p2align 4
  .type _XimRegisterDispatcher, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimRegisterDispatcher
#endif
_XimRegisterDispatcher:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7864(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7864(%rip)
2:
  pushq $983
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimRegisterFilter
  .p2align 4
  .type _XimRegisterFilter, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimRegisterFilter
#endif
_XimRegisterFilter:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7872(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7872(%rip)
2:
  pushq $984
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimRegisterIMInstantiateCallback
  .p2align 4
  .type _XimRegisterIMInstantiateCallback, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimRegisterIMInstantiateCallback
#endif
_XimRegisterIMInstantiateCallback:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7880(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7880(%rip)
2:
  pushq $985
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimRegisterServerFilter
  .p2align 4
  .type _XimRegisterServerFilter, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimRegisterServerFilter
#endif
_XimRegisterServerFilter:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7888(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7888(%rip)
2:
  pushq $986
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimRegisterTriggerKeysCallback
  .p2align 4
  .type _XimRegisterTriggerKeysCallback, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimRegisterTriggerKeysCallback
#endif
_XimRegisterTriggerKeysCallback:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7896(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7896(%rip)
2:
  pushq $987
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimReregisterFilter
  .p2align 4
  .type _XimReregisterFilter, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimReregisterFilter
#endif
_XimReregisterFilter:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7904(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7904(%rip)
2:
  pushq $988
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimResetIMInstantiateCallback
  .p2align 4
  .type _XimResetIMInstantiateCallback, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimResetIMInstantiateCallback
#endif
_XimResetIMInstantiateCallback:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7912(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7912(%rip)
2:
  pushq $989
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimRespSyncReply
  .p2align 4
  .type _XimRespSyncReply, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimRespSyncReply
#endif
_XimRespSyncReply:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7920(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7920(%rip)
2:
  pushq $990
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimServerDestroy
  .p2align 4
  .type _XimServerDestroy, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimServerDestroy
#endif
_XimServerDestroy:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7928(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7928(%rip)
2:
  pushq $991
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimSetCurrentICValues
  .p2align 4
  .type _XimSetCurrentICValues, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimSetCurrentICValues
#endif
_XimSetCurrentICValues:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7936(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7936(%rip)
2:
  pushq $992
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimSetCurrentIMValues
  .p2align 4
  .type _XimSetCurrentIMValues, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimSetCurrentIMValues
#endif
_XimSetCurrentIMValues:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7944(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7944(%rip)
2:
  pushq $993
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimSetEventMaskCallback
  .p2align 4
  .type _XimSetEventMaskCallback, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimSetEventMaskCallback
#endif
_XimSetEventMaskCallback:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7952(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7952(%rip)
2:
  pushq $994
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimSetHeader
  .p2align 4
  .type _XimSetHeader, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimSetHeader
#endif
_XimSetHeader:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7960(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7960(%rip)
2:
  pushq $995
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimSetICDefaults
  .p2align 4
  .type _XimSetICDefaults, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimSetICDefaults
#endif
_XimSetICDefaults:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7968(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7968(%rip)
2:
  pushq $996
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimSetICMode
  .p2align 4
  .type _XimSetICMode, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimSetICMode
#endif
_XimSetICMode:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7976(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7976(%rip)
2:
  pushq $997
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimSetICResourceList
  .p2align 4
  .type _XimSetICResourceList, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimSetICResourceList
#endif
_XimSetICResourceList:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7984(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7984(%rip)
2:
  pushq $998
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimSetICValueData
  .p2align 4
  .type _XimSetICValueData, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimSetICValueData
#endif
_XimSetICValueData:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+7992(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+7992(%rip)
2:
  pushq $999
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimSetIMMode
  .p2align 4
  .type _XimSetIMMode, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimSetIMMode
#endif
_XimSetIMMode:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8000(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8000(%rip)
2:
  pushq $1000
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimSetIMResourceList
  .p2align 4
  .type _XimSetIMResourceList, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimSetIMResourceList
#endif
_XimSetIMResourceList:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8008(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8008(%rip)
2:
  pushq $1001
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimSetIMValueData
  .p2align 4
  .type _XimSetIMValueData, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimSetIMValueData
#endif
_XimSetIMValueData:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8016(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8016(%rip)
2:
  pushq $1002
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimSetInnerICResourceList
  .p2align 4
  .type _XimSetInnerICResourceList, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimSetInnerICResourceList
#endif
_XimSetInnerICResourceList:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8024(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8024(%rip)
2:
  pushq $1003
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimSetInnerIMResourceList
  .p2align 4
  .type _XimSetInnerIMResourceList, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimSetInnerIMResourceList
#endif
_XimSetInnerIMResourceList:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8032(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8032(%rip)
2:
  pushq $1004
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimSetLocalIMDefaults
  .p2align 4
  .type _XimSetLocalIMDefaults, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimSetLocalIMDefaults
#endif
_XimSetLocalIMDefaults:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8040(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8040(%rip)
2:
  pushq $1005
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimShutdown
  .p2align 4
  .type _XimShutdown, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimShutdown
#endif
_XimShutdown:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8048(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8048(%rip)
2:
  pushq $1006
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimSync
  .p2align 4
  .type _XimSync, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimSync
#endif
_XimSync:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8056(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8056(%rip)
2:
  pushq $1007
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimSyncCallback
  .p2align 4
  .type _XimSyncCallback, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimSyncCallback
#endif
_XimSyncCallback:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8064(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8064(%rip)
2:
  pushq $1008
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimThaiCloseIM
  .p2align 4
  .type _XimThaiCloseIM, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimThaiCloseIM
#endif
_XimThaiCloseIM:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8072(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8072(%rip)
2:
  pushq $1009
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimThaiCreateIC
  .p2align 4
  .type _XimThaiCreateIC, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimThaiCreateIC
#endif
_XimThaiCreateIC:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8080(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8080(%rip)
2:
  pushq $1010
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimThaiFilter
  .p2align 4
  .type _XimThaiFilter, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimThaiFilter
#endif
_XimThaiFilter:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8088(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8088(%rip)
2:
  pushq $1011
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimThaiIMFree
  .p2align 4
  .type _XimThaiIMFree, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimThaiIMFree
#endif
_XimThaiIMFree:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8096(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8096(%rip)
2:
  pushq $1012
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimThaiOpenIM
  .p2align 4
  .type _XimThaiOpenIM, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimThaiOpenIM
#endif
_XimThaiOpenIM:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8104(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8104(%rip)
2:
  pushq $1013
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimTransCallDispatcher
  .p2align 4
  .type _XimTransCallDispatcher, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimTransCallDispatcher
#endif
_XimTransCallDispatcher:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8112(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8112(%rip)
2:
  pushq $1014
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimTransConf
  .p2align 4
  .type _XimTransConf, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimTransConf
#endif
_XimTransConf:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8120(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8120(%rip)
2:
  pushq $1015
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimTransFilterWaitEvent
  .p2align 4
  .type _XimTransFilterWaitEvent, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimTransFilterWaitEvent
#endif
_XimTransFilterWaitEvent:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8128(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8128(%rip)
2:
  pushq $1016
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimTransFlush
  .p2align 4
  .type _XimTransFlush, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimTransFlush
#endif
_XimTransFlush:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8136(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8136(%rip)
2:
  pushq $1017
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimTransInternalConnection
  .p2align 4
  .type _XimTransInternalConnection, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimTransInternalConnection
#endif
_XimTransInternalConnection:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8144(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8144(%rip)
2:
  pushq $1018
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimTransRead
  .p2align 4
  .type _XimTransRead, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimTransRead
#endif
_XimTransRead:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8152(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8152(%rip)
2:
  pushq $1019
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimTransRegisterDispatcher
  .p2align 4
  .type _XimTransRegisterDispatcher, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimTransRegisterDispatcher
#endif
_XimTransRegisterDispatcher:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8160(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8160(%rip)
2:
  pushq $1020
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimTransWrite
  .p2align 4
  .type _XimTransWrite, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimTransWrite
#endif
_XimTransWrite:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8168(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8168(%rip)
2:
  pushq $1021
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimTriggerNotify
  .p2align 4
  .type _XimTriggerNotify, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimTriggerNotify
#endif
_XimTriggerNotify:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8176(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8176(%rip)
2:
  pushq $1022
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimUnRegisterIMInstantiateCallback
  .p2align 4
  .type _XimUnRegisterIMInstantiateCallback, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimUnRegisterIMInstantiateCallback
#endif
_XimUnRegisterIMInstantiateCallback:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8184(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8184(%rip)
2:
  pushq $1023
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimUnregisterFilter
  .p2align 4
  .type _XimUnregisterFilter, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimUnregisterFilter
#endif
_XimUnregisterFilter:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8192(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8192(%rip)
2:
  pushq $1024
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimUnregisterServerFilter
  .p2align 4
  .type _XimUnregisterServerFilter, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimUnregisterServerFilter
#endif
_XimUnregisterServerFilter:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8200(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8200(%rip)
2:
  pushq $1025
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimWrite
  .p2align 4
  .type _XimWrite, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimWrite
#endif
_XimWrite:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8208(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8208(%rip)
2:
  pushq $1026
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimXConf
  .p2align 4
  .type _XimXConf, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimXConf
#endif
_XimXConf:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8216(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8216(%rip)
2:
  pushq $1027
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimXTransBytesReadable
  .p2align 4
  .type _XimXTransBytesReadable, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimXTransBytesReadable
#endif
_XimXTransBytesReadable:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8224(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8224(%rip)
2:
  pushq $1028
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimXTransClose
  .p2align 4
  .type _XimXTransClose, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimXTransClose
#endif
_XimXTransClose:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8232(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8232(%rip)
2:
  pushq $1029
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimXTransCloseForCloning
  .p2align 4
  .type _XimXTransCloseForCloning, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimXTransCloseForCloning
#endif
_XimXTransCloseForCloning:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8240(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8240(%rip)
2:
  pushq $1030
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimXTransConnect
  .p2align 4
  .type _XimXTransConnect, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimXTransConnect
#endif
_XimXTransConnect:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8248(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8248(%rip)
2:
  pushq $1031
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimXTransDisconnect
  .p2align 4
  .type _XimXTransDisconnect, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimXTransDisconnect
#endif
_XimXTransDisconnect:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8256(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8256(%rip)
2:
  pushq $1032
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimXTransFreeConnInfo
  .p2align 4
  .type _XimXTransFreeConnInfo, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimXTransFreeConnInfo
#endif
_XimXTransFreeConnInfo:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8264(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8264(%rip)
2:
  pushq $1033
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimXTransGetConnectionNumber
  .p2align 4
  .type _XimXTransGetConnectionNumber, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimXTransGetConnectionNumber
#endif
_XimXTransGetConnectionNumber:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8272(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8272(%rip)
2:
  pushq $1034
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimXTransGetHostname
  .p2align 4
  .type _XimXTransGetHostname, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimXTransGetHostname
#endif
_XimXTransGetHostname:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8280(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8280(%rip)
2:
  pushq $1035
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimXTransGetPeerAddr
  .p2align 4
  .type _XimXTransGetPeerAddr, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimXTransGetPeerAddr
#endif
_XimXTransGetPeerAddr:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8288(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8288(%rip)
2:
  pushq $1036
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimXTransIsLocal
  .p2align 4
  .type _XimXTransIsLocal, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimXTransIsLocal
#endif
_XimXTransIsLocal:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8296(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8296(%rip)
2:
  pushq $1037
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimXTransOpenCOTSClient
  .p2align 4
  .type _XimXTransOpenCOTSClient, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimXTransOpenCOTSClient
#endif
_XimXTransOpenCOTSClient:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8304(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8304(%rip)
2:
  pushq $1038
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimXTransRead
  .p2align 4
  .type _XimXTransRead, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimXTransRead
#endif
_XimXTransRead:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8312(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8312(%rip)
2:
  pushq $1039
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimXTransReadv
  .p2align 4
  .type _XimXTransReadv, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimXTransReadv
#endif
_XimXTransReadv:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8320(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8320(%rip)
2:
  pushq $1040
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimXTransSetOption
  .p2align 4
  .type _XimXTransSetOption, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimXTransSetOption
#endif
_XimXTransSetOption:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8328(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8328(%rip)
2:
  pushq $1041
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimXTransWrite
  .p2align 4
  .type _XimXTransWrite, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimXTransWrite
#endif
_XimXTransWrite:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8336(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8336(%rip)
2:
  pushq $1042
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XimXTransWritev
  .p2align 4
  .type _XimXTransWritev, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XimXTransWritev
#endif
_XimXTransWritev:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8344(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8344(%rip)
2:
  pushq $1043
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _Ximctstombs
  .p2align 4
  .type _Ximctstombs, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _Ximctstombs
#endif
_Ximctstombs:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8352(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8352(%rip)
2:
  pushq $1044
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _Ximctstoutf8
  .p2align 4
  .type _Ximctstoutf8, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _Ximctstoutf8
#endif
_Ximctstoutf8:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8360(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8360(%rip)
2:
  pushq $1045
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _Ximctstowcs
  .p2align 4
  .type _Ximctstowcs, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _Ximctstowcs
#endif
_Ximctstowcs:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8368(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8368(%rip)
2:
  pushq $1046
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XkbCopyFromReadBuffer
  .p2align 4
  .type _XkbCopyFromReadBuffer, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XkbCopyFromReadBuffer
#endif
_XkbCopyFromReadBuffer:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8376(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8376(%rip)
2:
  pushq $1047
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XkbFreeReadBuffer
  .p2align 4
  .type _XkbFreeReadBuffer, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XkbFreeReadBuffer
#endif
_XkbFreeReadBuffer:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8384(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8384(%rip)
2:
  pushq $1048
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XkbGetCharset
  .p2align 4
  .type _XkbGetCharset, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XkbGetCharset
#endif
_XkbGetCharset:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8392(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8392(%rip)
2:
  pushq $1049
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XkbGetConverters
  .p2align 4
  .type _XkbGetConverters, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XkbGetConverters
#endif
_XkbGetConverters:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8400(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8400(%rip)
2:
  pushq $1050
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XkbGetReadBufferCountedString
  .p2align 4
  .type _XkbGetReadBufferCountedString, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XkbGetReadBufferCountedString
#endif
_XkbGetReadBufferCountedString:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8408(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8408(%rip)
2:
  pushq $1051
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XkbGetReadBufferPtr
  .p2align 4
  .type _XkbGetReadBufferPtr, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XkbGetReadBufferPtr
#endif
_XkbGetReadBufferPtr:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8416(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8416(%rip)
2:
  pushq $1052
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XkbInitReadBuffer
  .p2align 4
  .type _XkbInitReadBuffer, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XkbInitReadBuffer
#endif
_XkbInitReadBuffer:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8424(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8424(%rip)
2:
  pushq $1053
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XkbNoteCoreMapChanges
  .p2align 4
  .type _XkbNoteCoreMapChanges, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XkbNoteCoreMapChanges
#endif
_XkbNoteCoreMapChanges:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8432(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8432(%rip)
2:
  pushq $1054
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XkbPeekAtReadBuffer
  .p2align 4
  .type _XkbPeekAtReadBuffer, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XkbPeekAtReadBuffer
#endif
_XkbPeekAtReadBuffer:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8440(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8440(%rip)
2:
  pushq $1055
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XkbReadBufferCopy32
  .p2align 4
  .type _XkbReadBufferCopy32, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XkbReadBufferCopy32
#endif
_XkbReadBufferCopy32:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8448(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8448(%rip)
2:
  pushq $1056
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XkbReadBufferCopyKeySyms
  .p2align 4
  .type _XkbReadBufferCopyKeySyms, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XkbReadBufferCopyKeySyms
#endif
_XkbReadBufferCopyKeySyms:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8456(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8456(%rip)
2:
  pushq $1057
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XkbReadCopyData32
  .p2align 4
  .type _XkbReadCopyData32, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XkbReadCopyData32
#endif
_XkbReadCopyData32:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8464(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8464(%rip)
2:
  pushq $1058
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XkbReadCopyKeySyms
  .p2align 4
  .type _XkbReadCopyKeySyms, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XkbReadCopyKeySyms
#endif
_XkbReadCopyKeySyms:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8472(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8472(%rip)
2:
  pushq $1059
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XkbReadGetCompatMapReply
  .p2align 4
  .type _XkbReadGetCompatMapReply, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XkbReadGetCompatMapReply
#endif
_XkbReadGetCompatMapReply:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8480(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8480(%rip)
2:
  pushq $1060
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XkbReadGetGeometryReply
  .p2align 4
  .type _XkbReadGetGeometryReply, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XkbReadGetGeometryReply
#endif
_XkbReadGetGeometryReply:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8488(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8488(%rip)
2:
  pushq $1061
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XkbReadGetIndicatorMapReply
  .p2align 4
  .type _XkbReadGetIndicatorMapReply, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XkbReadGetIndicatorMapReply
#endif
_XkbReadGetIndicatorMapReply:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8496(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8496(%rip)
2:
  pushq $1062
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XkbReadGetMapReply
  .p2align 4
  .type _XkbReadGetMapReply, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XkbReadGetMapReply
#endif
_XkbReadGetMapReply:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8504(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8504(%rip)
2:
  pushq $1063
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XkbReadGetNamesReply
  .p2align 4
  .type _XkbReadGetNamesReply, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XkbReadGetNamesReply
#endif
_XkbReadGetNamesReply:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8512(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8512(%rip)
2:
  pushq $1064
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XkbReloadDpy
  .p2align 4
  .type _XkbReloadDpy, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XkbReloadDpy
#endif
_XkbReloadDpy:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8520(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8520(%rip)
2:
  pushq $1065
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XkbSkipReadBufferData
  .p2align 4
  .type _XkbSkipReadBufferData, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XkbSkipReadBufferData
#endif
_XkbSkipReadBufferData:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8528(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8528(%rip)
2:
  pushq $1066
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XkbWriteCopyData32
  .p2align 4
  .type _XkbWriteCopyData32, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XkbWriteCopyData32
#endif
_XkbWriteCopyData32:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8536(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8536(%rip)
2:
  pushq $1067
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XkbWriteCopyKeySyms
  .p2align 4
  .type _XkbWriteCopyKeySyms, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XkbWriteCopyKeySyms
#endif
_XkbWriteCopyKeySyms:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8544(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8544(%rip)
2:
  pushq $1068
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XlcAddCT
  .p2align 4
  .type _XlcAddCT, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XlcAddCT
#endif
_XlcAddCT:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8552(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8552(%rip)
2:
  pushq $1069
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XlcAddCharSet
  .p2align 4
  .type _XlcAddCharSet, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XlcAddCharSet
#endif
_XlcAddCharSet:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8560(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8560(%rip)
2:
  pushq $1070
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XlcAddGB18030LocaleConverters
  .p2align 4
  .type _XlcAddGB18030LocaleConverters, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XlcAddGB18030LocaleConverters
#endif
_XlcAddGB18030LocaleConverters:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8568(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8568(%rip)
2:
  pushq $1071
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XlcAddLoader
  .p2align 4
  .type _XlcAddLoader, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XlcAddLoader
#endif
_XlcAddLoader:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8576(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8576(%rip)
2:
  pushq $1072
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XlcAddUtf8Converters
  .p2align 4
  .type _XlcAddUtf8Converters, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XlcAddUtf8Converters
#endif
_XlcAddUtf8Converters:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8584(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8584(%rip)
2:
  pushq $1073
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XlcAddUtf8LocaleConverters
  .p2align 4
  .type _XlcAddUtf8LocaleConverters, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XlcAddUtf8LocaleConverters
#endif
_XlcAddUtf8LocaleConverters:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8592(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8592(%rip)
2:
  pushq $1074
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XlcCloseConverter
  .p2align 4
  .type _XlcCloseConverter, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XlcCloseConverter
#endif
_XlcCloseConverter:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8600(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8600(%rip)
2:
  pushq $1075
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XlcCompareISOLatin1
  .p2align 4
  .type _XlcCompareISOLatin1, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XlcCompareISOLatin1
#endif
_XlcCompareISOLatin1:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8608(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8608(%rip)
2:
  pushq $1076
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XlcCompileResourceList
  .p2align 4
  .type _XlcCompileResourceList, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XlcCompileResourceList
#endif
_XlcCompileResourceList:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8616(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8616(%rip)
2:
  pushq $1077
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XlcConvert
  .p2align 4
  .type _XlcConvert, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XlcConvert
#endif
_XlcConvert:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8624(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8624(%rip)
2:
  pushq $1078
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XlcCopyFromArg
  .p2align 4
  .type _XlcCopyFromArg, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XlcCopyFromArg
#endif
_XlcCopyFromArg:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8632(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8632(%rip)
2:
  pushq $1079
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XlcCopyToArg
  .p2align 4
  .type _XlcCopyToArg, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XlcCopyToArg
#endif
_XlcCopyToArg:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8640(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8640(%rip)
2:
  pushq $1080
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XlcCountVaList
  .p2align 4
  .type _XlcCountVaList, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XlcCountVaList
#endif
_XlcCountVaList:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8648(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8648(%rip)
2:
  pushq $1081
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XlcCreateDefaultCharSet
  .p2align 4
  .type _XlcCreateDefaultCharSet, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XlcCreateDefaultCharSet
#endif
_XlcCreateDefaultCharSet:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8656(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8656(%rip)
2:
  pushq $1082
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XlcCreateLC
  .p2align 4
  .type _XlcCreateLC, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XlcCreateLC
#endif
_XlcCreateLC:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8664(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8664(%rip)
2:
  pushq $1083
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XlcCreateLocaleDataBase
  .p2align 4
  .type _XlcCreateLocaleDataBase, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XlcCreateLocaleDataBase
#endif
_XlcCreateLocaleDataBase:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8672(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8672(%rip)
2:
  pushq $1084
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XlcCurrentLC
  .p2align 4
  .type _XlcCurrentLC, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XlcCurrentLC
#endif
_XlcCurrentLC:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8680(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8680(%rip)
2:
  pushq $1085
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XlcDbg_printValue
  .p2align 4
  .type _XlcDbg_printValue, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XlcDbg_printValue
#endif
_XlcDbg_printValue:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8688(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8688(%rip)
2:
  pushq $1086
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XlcDeInitLoader
  .p2align 4
  .type _XlcDeInitLoader, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XlcDeInitLoader
#endif
_XlcDeInitLoader:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8696(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8696(%rip)
2:
  pushq $1087
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XlcDefaultLoader
  .p2align 4
  .type _XlcDefaultLoader, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XlcDefaultLoader
#endif
_XlcDefaultLoader:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8704(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8704(%rip)
2:
  pushq $1088
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XlcDefaultMapModifiers
  .p2align 4
  .type _XlcDefaultMapModifiers, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XlcDefaultMapModifiers
#endif
_XlcDefaultMapModifiers:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8712(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8712(%rip)
2:
  pushq $1089
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XlcDestroyLC
  .p2align 4
  .type _XlcDestroyLC, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XlcDestroyLC
#endif
_XlcDestroyLC:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8720(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8720(%rip)
2:
  pushq $1090
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XlcDestroyLocaleDataBase
  .p2align 4
  .type _XlcDestroyLocaleDataBase, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XlcDestroyLocaleDataBase
#endif
_XlcDestroyLocaleDataBase:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8728(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8728(%rip)
2:
  pushq $1091
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XlcFileName
  .p2align 4
  .type _XlcFileName, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XlcFileName
#endif
_XlcFileName:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8736(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8736(%rip)
2:
  pushq $1092
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XlcGenericLoader
  .p2align 4
  .type _XlcGenericLoader, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XlcGenericLoader
#endif
_XlcGenericLoader:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8744(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8744(%rip)
2:
  pushq $1093
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XlcGetCSValues
  .p2align 4
  .type _XlcGetCSValues, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XlcGetCSValues
#endif
_XlcGetCSValues:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8752(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8752(%rip)
2:
  pushq $1094
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XlcGetCharSet
  .p2align 4
  .type _XlcGetCharSet, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XlcGetCharSet
#endif
_XlcGetCharSet:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8760(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8760(%rip)
2:
  pushq $1095
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XlcGetCharSetWithSide
  .p2align 4
  .type _XlcGetCharSetWithSide, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XlcGetCharSetWithSide
#endif
_XlcGetCharSetWithSide:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8768(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8768(%rip)
2:
  pushq $1096
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XlcGetLocaleDataBase
  .p2align 4
  .type _XlcGetLocaleDataBase, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XlcGetLocaleDataBase
#endif
_XlcGetLocaleDataBase:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8776(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8776(%rip)
2:
  pushq $1097
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XlcGetResource
  .p2align 4
  .type _XlcGetResource, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XlcGetResource
#endif
_XlcGetResource:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8784(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8784(%rip)
2:
  pushq $1098
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XlcGetValues
  .p2align 4
  .type _XlcGetValues, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XlcGetValues
#endif
_XlcGetValues:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8792(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8792(%rip)
2:
  pushq $1099
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XlcInitCTInfo
  .p2align 4
  .type _XlcInitCTInfo, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XlcInitCTInfo
#endif
_XlcInitCTInfo:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8800(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8800(%rip)
2:
  pushq $1100
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XlcInitLoader
  .p2align 4
  .type _XlcInitLoader, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XlcInitLoader
#endif
_XlcInitLoader:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8808(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8808(%rip)
2:
  pushq $1101
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XlcLocaleDirName
  .p2align 4
  .type _XlcLocaleDirName, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XlcLocaleDirName
#endif
_XlcLocaleDirName:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8816(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8816(%rip)
2:
  pushq $1102
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XlcLocaleLibDirName
  .p2align 4
  .type _XlcLocaleLibDirName, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XlcLocaleLibDirName
#endif
_XlcLocaleLibDirName:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8824(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8824(%rip)
2:
  pushq $1103
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XlcMapOSLocaleName
  .p2align 4
  .type _XlcMapOSLocaleName, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XlcMapOSLocaleName
#endif
_XlcMapOSLocaleName:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8832(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8832(%rip)
2:
  pushq $1104
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XlcNCompareISOLatin1
  .p2align 4
  .type _XlcNCompareISOLatin1, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XlcNCompareISOLatin1
#endif
_XlcNCompareISOLatin1:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8840(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8840(%rip)
2:
  pushq $1105
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XlcOpenConverter
  .p2align 4
  .type _XlcOpenConverter, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XlcOpenConverter
#endif
_XlcOpenConverter:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8848(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8848(%rip)
2:
  pushq $1106
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XlcParseCharSet
  .p2align 4
  .type _XlcParseCharSet, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XlcParseCharSet
#endif
_XlcParseCharSet:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8856(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8856(%rip)
2:
  pushq $1107
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XlcParse_scopemaps
  .p2align 4
  .type _XlcParse_scopemaps, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XlcParse_scopemaps
#endif
_XlcParse_scopemaps:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8864(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8864(%rip)
2:
  pushq $1108
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XlcRemoveLoader
  .p2align 4
  .type _XlcRemoveLoader, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XlcRemoveLoader
#endif
_XlcRemoveLoader:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8872(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8872(%rip)
2:
  pushq $1109
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XlcResetConverter
  .p2align 4
  .type _XlcResetConverter, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XlcResetConverter
#endif
_XlcResetConverter:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8880(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8880(%rip)
2:
  pushq $1110
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XlcResolveI18NPath
  .p2align 4
  .type _XlcResolveI18NPath, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XlcResolveI18NPath
#endif
_XlcResolveI18NPath:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8888(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8888(%rip)
2:
  pushq $1111
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XlcResolveLocaleName
  .p2align 4
  .type _XlcResolveLocaleName, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XlcResolveLocaleName
#endif
_XlcResolveLocaleName:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8896(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8896(%rip)
2:
  pushq $1112
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XlcSetConverter
  .p2align 4
  .type _XlcSetConverter, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XlcSetConverter
#endif
_XlcSetConverter:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8904(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8904(%rip)
2:
  pushq $1113
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XlcSetValues
  .p2align 4
  .type _XlcSetValues, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XlcSetValues
#endif
_XlcSetValues:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8912(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8912(%rip)
2:
  pushq $1114
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XlcUtf8Loader
  .p2align 4
  .type _XlcUtf8Loader, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XlcUtf8Loader
#endif
_XlcUtf8Loader:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8920(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8920(%rip)
2:
  pushq $1115
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XlcVaToArgList
  .p2align 4
  .type _XlcVaToArgList, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XlcVaToArgList
#endif
_XlcVaToArgList:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8928(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8928(%rip)
2:
  pushq $1116
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XlcValidModSyntax
  .p2align 4
  .type _XlcValidModSyntax, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XlcValidModSyntax
#endif
_XlcValidModSyntax:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8936(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8936(%rip)
2:
  pushq $1117
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _Xlcmbstoutf8
  .p2align 4
  .type _Xlcmbstoutf8, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _Xlcmbstoutf8
#endif
_Xlcmbstoutf8:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8944(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8944(%rip)
2:
  pushq $1118
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _Xlcmbstowcs
  .p2align 4
  .type _Xlcmbstowcs, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _Xlcmbstowcs
#endif
_Xlcmbstowcs:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8952(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8952(%rip)
2:
  pushq $1119
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _Xlcmbtowc
  .p2align 4
  .type _Xlcmbtowc, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _Xlcmbtowc
#endif
_Xlcmbtowc:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8960(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8960(%rip)
2:
  pushq $1120
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _Xlcwcstombs
  .p2align 4
  .type _Xlcwcstombs, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _Xlcwcstombs
#endif
_Xlcwcstombs:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8968(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8968(%rip)
2:
  pushq $1121
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _Xlcwctomb
  .p2align 4
  .type _Xlcwctomb, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _Xlcwctomb
#endif
_Xlcwctomb:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8976(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8976(%rip)
2:
  pushq $1122
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XmbDefaultDrawImageString
  .p2align 4
  .type _XmbDefaultDrawImageString, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XmbDefaultDrawImageString
#endif
_XmbDefaultDrawImageString:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8984(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8984(%rip)
2:
  pushq $1123
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XmbDefaultDrawString
  .p2align 4
  .type _XmbDefaultDrawString, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XmbDefaultDrawString
#endif
_XmbDefaultDrawString:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+8992(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+8992(%rip)
2:
  pushq $1124
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XmbDefaultTextEscapement
  .p2align 4
  .type _XmbDefaultTextEscapement, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XmbDefaultTextEscapement
#endif
_XmbDefaultTextEscapement:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+9000(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+9000(%rip)
2:
  pushq $1125
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XmbDefaultTextExtents
  .p2align 4
  .type _XmbDefaultTextExtents, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XmbDefaultTextExtents
#endif
_XmbDefaultTextExtents:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+9008(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+9008(%rip)
2:
  pushq $1126
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XmbDefaultTextPerCharExtents
  .p2align 4
  .type _XmbDefaultTextPerCharExtents, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XmbDefaultTextPerCharExtents
#endif
_XmbDefaultTextPerCharExtents:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+9016(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+9016(%rip)
2:
  pushq $1127
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XmbGenericDrawImageString
  .p2align 4
  .type _XmbGenericDrawImageString, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XmbGenericDrawImageString
#endif
_XmbGenericDrawImageString:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+9024(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+9024(%rip)
2:
  pushq $1128
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XmbGenericDrawString
  .p2align 4
  .type _XmbGenericDrawString, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XmbGenericDrawString
#endif
_XmbGenericDrawString:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+9032(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+9032(%rip)
2:
  pushq $1129
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XmbGenericTextEscapement
  .p2align 4
  .type _XmbGenericTextEscapement, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XmbGenericTextEscapement
#endif
_XmbGenericTextEscapement:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+9040(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+9040(%rip)
2:
  pushq $1130
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XmbGenericTextExtents
  .p2align 4
  .type _XmbGenericTextExtents, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XmbGenericTextExtents
#endif
_XmbGenericTextExtents:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+9048(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+9048(%rip)
2:
  pushq $1131
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XmbGenericTextPerCharExtents
  .p2align 4
  .type _XmbGenericTextPerCharExtents, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XmbGenericTextPerCharExtents
#endif
_XmbGenericTextPerCharExtents:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+9056(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+9056(%rip)
2:
  pushq $1132
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XmbTextListToTextProperty
  .p2align 4
  .type _XmbTextListToTextProperty, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XmbTextListToTextProperty
#endif
_XmbTextListToTextProperty:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+9064(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+9064(%rip)
2:
  pushq $1133
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XmbTextPropertyToTextList
  .p2align 4
  .type _XmbTextPropertyToTextList, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XmbTextPropertyToTextList
#endif
_XmbTextPropertyToTextList:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+9072(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+9072(%rip)
2:
  pushq $1134
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _Xmblen
  .p2align 4
  .type _Xmblen, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _Xmblen
#endif
_Xmblen:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+9080(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+9080(%rip)
2:
  pushq $1135
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _Xmbstoutf8
  .p2align 4
  .type _Xmbstoutf8, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _Xmbstoutf8
#endif
_Xmbstoutf8:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+9088(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+9088(%rip)
2:
  pushq $1136
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _Xmbstowcs
  .p2align 4
  .type _Xmbstowcs, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _Xmbstowcs
#endif
_Xmbstowcs:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+9096(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+9096(%rip)
2:
  pushq $1137
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _Xmbtowc
  .p2align 4
  .type _Xmbtowc, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _Xmbtowc
#endif
_Xmbtowc:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+9104(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+9104(%rip)
2:
  pushq $1138
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XomConvert
  .p2align 4
  .type _XomConvert, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XomConvert
#endif
_XomConvert:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+9112(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+9112(%rip)
2:
  pushq $1139
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XomGenericDrawString
  .p2align 4
  .type _XomGenericDrawString, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XomGenericDrawString
#endif
_XomGenericDrawString:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+9120(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+9120(%rip)
2:
  pushq $1140
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XomGenericOpenOM
  .p2align 4
  .type _XomGenericOpenOM, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XomGenericOpenOM
#endif
_XomGenericOpenOM:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+9128(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+9128(%rip)
2:
  pushq $1141
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XomGenericTextExtents
  .p2align 4
  .type _XomGenericTextExtents, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XomGenericTextExtents
#endif
_XomGenericTextExtents:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+9136(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+9136(%rip)
2:
  pushq $1142
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XomGetFontDataFromFontSet
  .p2align 4
  .type _XomGetFontDataFromFontSet, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XomGetFontDataFromFontSet
#endif
_XomGetFontDataFromFontSet:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+9144(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+9144(%rip)
2:
  pushq $1143
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XomInitConverter
  .p2align 4
  .type _XomInitConverter, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XomInitConverter
#endif
_XomInitConverter:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+9152(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+9152(%rip)
2:
  pushq $1144
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XrmDefaultInitParseInfo
  .p2align 4
  .type _XrmDefaultInitParseInfo, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XrmDefaultInitParseInfo
#endif
_XrmDefaultInitParseInfo:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+9160(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+9160(%rip)
2:
  pushq $1145
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XrmInitParseInfo
  .p2align 4
  .type _XrmInitParseInfo, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XrmInitParseInfo
#endif
_XrmInitParseInfo:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+9168(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+9168(%rip)
2:
  pushq $1146
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XrmInternalStringToQuark
  .p2align 4
  .type _XrmInternalStringToQuark, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XrmInternalStringToQuark
#endif
_XrmInternalStringToQuark:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+9176(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+9176(%rip)
2:
  pushq $1147
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _Xutf8DefaultDrawImageString
  .p2align 4
  .type _Xutf8DefaultDrawImageString, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _Xutf8DefaultDrawImageString
#endif
_Xutf8DefaultDrawImageString:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+9184(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+9184(%rip)
2:
  pushq $1148
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _Xutf8DefaultDrawString
  .p2align 4
  .type _Xutf8DefaultDrawString, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _Xutf8DefaultDrawString
#endif
_Xutf8DefaultDrawString:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+9192(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+9192(%rip)
2:
  pushq $1149
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _Xutf8DefaultTextEscapement
  .p2align 4
  .type _Xutf8DefaultTextEscapement, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _Xutf8DefaultTextEscapement
#endif
_Xutf8DefaultTextEscapement:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+9200(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+9200(%rip)
2:
  pushq $1150
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _Xutf8DefaultTextExtents
  .p2align 4
  .type _Xutf8DefaultTextExtents, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _Xutf8DefaultTextExtents
#endif
_Xutf8DefaultTextExtents:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+9208(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+9208(%rip)
2:
  pushq $1151
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _Xutf8DefaultTextPerCharExtents
  .p2align 4
  .type _Xutf8DefaultTextPerCharExtents, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _Xutf8DefaultTextPerCharExtents
#endif
_Xutf8DefaultTextPerCharExtents:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+9216(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+9216(%rip)
2:
  pushq $1152
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _Xutf8GenericDrawImageString
  .p2align 4
  .type _Xutf8GenericDrawImageString, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _Xutf8GenericDrawImageString
#endif
_Xutf8GenericDrawImageString:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+9224(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+9224(%rip)
2:
  pushq $1153
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _Xutf8GenericDrawString
  .p2align 4
  .type _Xutf8GenericDrawString, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _Xutf8GenericDrawString
#endif
_Xutf8GenericDrawString:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+9232(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+9232(%rip)
2:
  pushq $1154
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _Xutf8GenericTextEscapement
  .p2align 4
  .type _Xutf8GenericTextEscapement, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _Xutf8GenericTextEscapement
#endif
_Xutf8GenericTextEscapement:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+9240(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+9240(%rip)
2:
  pushq $1155
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _Xutf8GenericTextExtents
  .p2align 4
  .type _Xutf8GenericTextExtents, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _Xutf8GenericTextExtents
#endif
_Xutf8GenericTextExtents:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+9248(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+9248(%rip)
2:
  pushq $1156
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _Xutf8GenericTextPerCharExtents
  .p2align 4
  .type _Xutf8GenericTextPerCharExtents, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _Xutf8GenericTextPerCharExtents
#endif
_Xutf8GenericTextPerCharExtents:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+9256(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+9256(%rip)
2:
  pushq $1157
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _Xutf8TextListToTextProperty
  .p2align 4
  .type _Xutf8TextListToTextProperty, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _Xutf8TextListToTextProperty
#endif
_Xutf8TextListToTextProperty:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+9264(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+9264(%rip)
2:
  pushq $1158
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _Xutf8TextPropertyToTextList
  .p2align 4
  .type _Xutf8TextPropertyToTextList, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _Xutf8TextPropertyToTextList
#endif
_Xutf8TextPropertyToTextList:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+9272(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+9272(%rip)
2:
  pushq $1159
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XwcDefaultDrawImageString
  .p2align 4
  .type _XwcDefaultDrawImageString, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XwcDefaultDrawImageString
#endif
_XwcDefaultDrawImageString:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+9280(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+9280(%rip)
2:
  pushq $1160
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XwcDefaultDrawString
  .p2align 4
  .type _XwcDefaultDrawString, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XwcDefaultDrawString
#endif
_XwcDefaultDrawString:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+9288(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+9288(%rip)
2:
  pushq $1161
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XwcDefaultTextEscapement
  .p2align 4
  .type _XwcDefaultTextEscapement, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XwcDefaultTextEscapement
#endif
_XwcDefaultTextEscapement:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+9296(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+9296(%rip)
2:
  pushq $1162
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XwcDefaultTextExtents
  .p2align 4
  .type _XwcDefaultTextExtents, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XwcDefaultTextExtents
#endif
_XwcDefaultTextExtents:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+9304(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+9304(%rip)
2:
  pushq $1163
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XwcDefaultTextPerCharExtents
  .p2align 4
  .type _XwcDefaultTextPerCharExtents, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XwcDefaultTextPerCharExtents
#endif
_XwcDefaultTextPerCharExtents:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+9312(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+9312(%rip)
2:
  pushq $1164
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XwcFreeStringList
  .p2align 4
  .type _XwcFreeStringList, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XwcFreeStringList
#endif
_XwcFreeStringList:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+9320(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+9320(%rip)
2:
  pushq $1165
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XwcGenericDrawImageString
  .p2align 4
  .type _XwcGenericDrawImageString, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XwcGenericDrawImageString
#endif
_XwcGenericDrawImageString:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+9328(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+9328(%rip)
2:
  pushq $1166
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XwcGenericDrawString
  .p2align 4
  .type _XwcGenericDrawString, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XwcGenericDrawString
#endif
_XwcGenericDrawString:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+9336(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+9336(%rip)
2:
  pushq $1167
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XwcGenericTextEscapement
  .p2align 4
  .type _XwcGenericTextEscapement, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XwcGenericTextEscapement
#endif
_XwcGenericTextEscapement:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+9344(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+9344(%rip)
2:
  pushq $1168
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XwcGenericTextExtents
  .p2align 4
  .type _XwcGenericTextExtents, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XwcGenericTextExtents
#endif
_XwcGenericTextExtents:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+9352(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+9352(%rip)
2:
  pushq $1169
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XwcGenericTextPerCharExtents
  .p2align 4
  .type _XwcGenericTextPerCharExtents, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XwcGenericTextPerCharExtents
#endif
_XwcGenericTextPerCharExtents:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+9360(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+9360(%rip)
2:
  pushq $1170
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XwcTextListToTextProperty
  .p2align 4
  .type _XwcTextListToTextProperty, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XwcTextListToTextProperty
#endif
_XwcTextListToTextProperty:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+9368(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+9368(%rip)
2:
  pushq $1171
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _XwcTextPropertyToTextList
  .p2align 4
  .type _XwcTextPropertyToTextList, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _XwcTextPropertyToTextList
#endif
_XwcTextPropertyToTextList:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+9376(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+9376(%rip)
2:
  pushq $1172
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _Xwcscmp
  .p2align 4
  .type _Xwcscmp, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _Xwcscmp
#endif
_Xwcscmp:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+9384(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+9384(%rip)
2:
  pushq $1173
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _Xwcscpy
  .p2align 4
  .type _Xwcscpy, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _Xwcscpy
#endif
_Xwcscpy:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+9392(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+9392(%rip)
2:
  pushq $1174
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _Xwcslen
  .p2align 4
  .type _Xwcslen, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _Xwcslen
#endif
_Xwcslen:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+9400(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+9400(%rip)
2:
  pushq $1175
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _Xwcsncmp
  .p2align 4
  .type _Xwcsncmp, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _Xwcsncmp
#endif
_Xwcsncmp:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+9408(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+9408(%rip)
2:
  pushq $1176
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _Xwcsncpy
  .p2align 4
  .type _Xwcsncpy, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _Xwcsncpy
#endif
_Xwcsncpy:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+9416(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+9416(%rip)
2:
  pushq $1177
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _Xwcstombs
  .p2align 4
  .type _Xwcstombs, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _Xwcstombs
#endif
_Xwcstombs:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+9424(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+9424(%rip)
2:
  pushq $1178
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl _Xwctomb
  .p2align 4
  .type _Xwctomb, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden _Xwctomb
#endif
_Xwctomb:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+9432(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+9432(%rip)
2:
  pushq $1179
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl read_EncodingInfo
  .p2align 4
  .type read_EncodingInfo, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden read_EncodingInfo
#endif
read_EncodingInfo:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+9440(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+9440(%rip)
2:
  pushq $1180
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
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

  .globl xlocaledir
  .p2align 4
  .type xlocaledir, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden xlocaledir
#endif
xlocaledir:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libX11_so_tramp_table+9448(%rip)
  je 2f
1:
  jmp *_libX11_so_tramp_table+9448(%rip)
2:
  pushq $1181
  .cfi_adjust_cfa_offset 8
  call _libX11_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

