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

  .globl _libcuda_so_tramp_table
  .hidden _libcuda_so_tramp_table
  .align 8
_libcuda_so_tramp_table:
  .zero 5280

  .text

  .globl _libcuda_so_tramp_resolve
  .hidden _libcuda_so_tramp_resolve

  .globl _libcuda_so_save_regs_and_resolve
  .hidden _libcuda_so_save_regs_and_resolve
  .type _libcuda_so_save_regs_and_resolve, %function
_libcuda_so_save_regs_and_resolve:
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
  call _libcuda_so_tramp_resolve

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

  .globl cuArray3DCreate
  .p2align 4
  .type cuArray3DCreate, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuArray3DCreate
#endif
cuArray3DCreate:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+0(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+0(%rip)
2:
  pushq $0
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuArray3DCreate_v2
  .p2align 4
  .type cuArray3DCreate_v2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuArray3DCreate_v2
#endif
cuArray3DCreate_v2:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+8(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+8(%rip)
2:
  pushq $1
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuArray3DGetDescriptor
  .p2align 4
  .type cuArray3DGetDescriptor, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuArray3DGetDescriptor
#endif
cuArray3DGetDescriptor:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+16(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+16(%rip)
2:
  pushq $2
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuArray3DGetDescriptor_v2
  .p2align 4
  .type cuArray3DGetDescriptor_v2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuArray3DGetDescriptor_v2
#endif
cuArray3DGetDescriptor_v2:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+24(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+24(%rip)
2:
  pushq $3
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuArrayCreate
  .p2align 4
  .type cuArrayCreate, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuArrayCreate
#endif
cuArrayCreate:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+32(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+32(%rip)
2:
  pushq $4
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuArrayCreate_v2
  .p2align 4
  .type cuArrayCreate_v2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuArrayCreate_v2
#endif
cuArrayCreate_v2:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+40(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+40(%rip)
2:
  pushq $5
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuArrayDestroy
  .p2align 4
  .type cuArrayDestroy, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuArrayDestroy
#endif
cuArrayDestroy:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+48(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+48(%rip)
2:
  pushq $6
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuArrayGetDescriptor
  .p2align 4
  .type cuArrayGetDescriptor, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuArrayGetDescriptor
#endif
cuArrayGetDescriptor:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+56(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+56(%rip)
2:
  pushq $7
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuArrayGetDescriptor_v2
  .p2align 4
  .type cuArrayGetDescriptor_v2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuArrayGetDescriptor_v2
#endif
cuArrayGetDescriptor_v2:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+64(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+64(%rip)
2:
  pushq $8
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuArrayGetMemoryRequirements
  .p2align 4
  .type cuArrayGetMemoryRequirements, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuArrayGetMemoryRequirements
#endif
cuArrayGetMemoryRequirements:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+72(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+72(%rip)
2:
  pushq $9
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuArrayGetPlane
  .p2align 4
  .type cuArrayGetPlane, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuArrayGetPlane
#endif
cuArrayGetPlane:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+80(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+80(%rip)
2:
  pushq $10
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuArrayGetSparseProperties
  .p2align 4
  .type cuArrayGetSparseProperties, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuArrayGetSparseProperties
#endif
cuArrayGetSparseProperties:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+88(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+88(%rip)
2:
  pushq $11
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuCheckpointProcessCheckpoint
  .p2align 4
  .type cuCheckpointProcessCheckpoint, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuCheckpointProcessCheckpoint
#endif
cuCheckpointProcessCheckpoint:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+96(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+96(%rip)
2:
  pushq $12
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuCheckpointProcessGetRestoreThreadId
  .p2align 4
  .type cuCheckpointProcessGetRestoreThreadId, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuCheckpointProcessGetRestoreThreadId
#endif
cuCheckpointProcessGetRestoreThreadId:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+104(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+104(%rip)
2:
  pushq $13
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuCheckpointProcessGetState
  .p2align 4
  .type cuCheckpointProcessGetState, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuCheckpointProcessGetState
#endif
cuCheckpointProcessGetState:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+112(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+112(%rip)
2:
  pushq $14
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuCheckpointProcessLock
  .p2align 4
  .type cuCheckpointProcessLock, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuCheckpointProcessLock
#endif
cuCheckpointProcessLock:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+120(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+120(%rip)
2:
  pushq $15
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuCheckpointProcessRestore
  .p2align 4
  .type cuCheckpointProcessRestore, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuCheckpointProcessRestore
#endif
cuCheckpointProcessRestore:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+128(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+128(%rip)
2:
  pushq $16
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuCheckpointProcessUnlock
  .p2align 4
  .type cuCheckpointProcessUnlock, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuCheckpointProcessUnlock
#endif
cuCheckpointProcessUnlock:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+136(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+136(%rip)
2:
  pushq $17
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuCoredumpGetAttribute
  .p2align 4
  .type cuCoredumpGetAttribute, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuCoredumpGetAttribute
#endif
cuCoredumpGetAttribute:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+144(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+144(%rip)
2:
  pushq $18
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuCoredumpGetAttributeGlobal
  .p2align 4
  .type cuCoredumpGetAttributeGlobal, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuCoredumpGetAttributeGlobal
#endif
cuCoredumpGetAttributeGlobal:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+152(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+152(%rip)
2:
  pushq $19
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuCoredumpSetAttribute
  .p2align 4
  .type cuCoredumpSetAttribute, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuCoredumpSetAttribute
#endif
cuCoredumpSetAttribute:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+160(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+160(%rip)
2:
  pushq $20
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuCoredumpSetAttributeGlobal
  .p2align 4
  .type cuCoredumpSetAttributeGlobal, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuCoredumpSetAttributeGlobal
#endif
cuCoredumpSetAttributeGlobal:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+168(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+168(%rip)
2:
  pushq $21
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuCtxAttach
  .p2align 4
  .type cuCtxAttach, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuCtxAttach
#endif
cuCtxAttach:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+176(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+176(%rip)
2:
  pushq $22
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuCtxCreate
  .p2align 4
  .type cuCtxCreate, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuCtxCreate
#endif
cuCtxCreate:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+184(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+184(%rip)
2:
  pushq $23
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuCtxCreate_v2
  .p2align 4
  .type cuCtxCreate_v2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuCtxCreate_v2
#endif
cuCtxCreate_v2:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+192(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+192(%rip)
2:
  pushq $24
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuCtxCreate_v3
  .p2align 4
  .type cuCtxCreate_v3, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuCtxCreate_v3
#endif
cuCtxCreate_v3:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+200(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+200(%rip)
2:
  pushq $25
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuCtxCreate_v4
  .p2align 4
  .type cuCtxCreate_v4, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuCtxCreate_v4
#endif
cuCtxCreate_v4:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+208(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+208(%rip)
2:
  pushq $26
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuCtxDestroy
  .p2align 4
  .type cuCtxDestroy, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuCtxDestroy
#endif
cuCtxDestroy:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+216(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+216(%rip)
2:
  pushq $27
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuCtxDestroy_v2
  .p2align 4
  .type cuCtxDestroy_v2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuCtxDestroy_v2
#endif
cuCtxDestroy_v2:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+224(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+224(%rip)
2:
  pushq $28
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuCtxDetach
  .p2align 4
  .type cuCtxDetach, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuCtxDetach
#endif
cuCtxDetach:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+232(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+232(%rip)
2:
  pushq $29
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuCtxDisablePeerAccess
  .p2align 4
  .type cuCtxDisablePeerAccess, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuCtxDisablePeerAccess
#endif
cuCtxDisablePeerAccess:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+240(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+240(%rip)
2:
  pushq $30
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuCtxEnablePeerAccess
  .p2align 4
  .type cuCtxEnablePeerAccess, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuCtxEnablePeerAccess
#endif
cuCtxEnablePeerAccess:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+248(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+248(%rip)
2:
  pushq $31
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuCtxFromGreenCtx
  .p2align 4
  .type cuCtxFromGreenCtx, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuCtxFromGreenCtx
#endif
cuCtxFromGreenCtx:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+256(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+256(%rip)
2:
  pushq $32
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuCtxGetApiVersion
  .p2align 4
  .type cuCtxGetApiVersion, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuCtxGetApiVersion
#endif
cuCtxGetApiVersion:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+264(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+264(%rip)
2:
  pushq $33
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuCtxGetCacheConfig
  .p2align 4
  .type cuCtxGetCacheConfig, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuCtxGetCacheConfig
#endif
cuCtxGetCacheConfig:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+272(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+272(%rip)
2:
  pushq $34
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuCtxGetCurrent
  .p2align 4
  .type cuCtxGetCurrent, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuCtxGetCurrent
#endif
cuCtxGetCurrent:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+280(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+280(%rip)
2:
  pushq $35
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuCtxGetDevResource
  .p2align 4
  .type cuCtxGetDevResource, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuCtxGetDevResource
#endif
cuCtxGetDevResource:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+288(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+288(%rip)
2:
  pushq $36
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuCtxGetDevice
  .p2align 4
  .type cuCtxGetDevice, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuCtxGetDevice
#endif
cuCtxGetDevice:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+296(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+296(%rip)
2:
  pushq $37
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuCtxGetExecAffinity
  .p2align 4
  .type cuCtxGetExecAffinity, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuCtxGetExecAffinity
#endif
cuCtxGetExecAffinity:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+304(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+304(%rip)
2:
  pushq $38
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuCtxGetFlags
  .p2align 4
  .type cuCtxGetFlags, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuCtxGetFlags
#endif
cuCtxGetFlags:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+312(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+312(%rip)
2:
  pushq $39
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuCtxGetId
  .p2align 4
  .type cuCtxGetId, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuCtxGetId
#endif
cuCtxGetId:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+320(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+320(%rip)
2:
  pushq $40
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuCtxGetLimit
  .p2align 4
  .type cuCtxGetLimit, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuCtxGetLimit
#endif
cuCtxGetLimit:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+328(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+328(%rip)
2:
  pushq $41
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuCtxGetSharedMemConfig
  .p2align 4
  .type cuCtxGetSharedMemConfig, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuCtxGetSharedMemConfig
#endif
cuCtxGetSharedMemConfig:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+336(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+336(%rip)
2:
  pushq $42
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuCtxGetStreamPriorityRange
  .p2align 4
  .type cuCtxGetStreamPriorityRange, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuCtxGetStreamPriorityRange
#endif
cuCtxGetStreamPriorityRange:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+344(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+344(%rip)
2:
  pushq $43
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuCtxPopCurrent
  .p2align 4
  .type cuCtxPopCurrent, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuCtxPopCurrent
#endif
cuCtxPopCurrent:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+352(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+352(%rip)
2:
  pushq $44
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuCtxPopCurrent_v2
  .p2align 4
  .type cuCtxPopCurrent_v2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuCtxPopCurrent_v2
#endif
cuCtxPopCurrent_v2:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+360(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+360(%rip)
2:
  pushq $45
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuCtxPushCurrent
  .p2align 4
  .type cuCtxPushCurrent, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuCtxPushCurrent
#endif
cuCtxPushCurrent:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+368(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+368(%rip)
2:
  pushq $46
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuCtxPushCurrent_v2
  .p2align 4
  .type cuCtxPushCurrent_v2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuCtxPushCurrent_v2
#endif
cuCtxPushCurrent_v2:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+376(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+376(%rip)
2:
  pushq $47
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuCtxRecordEvent
  .p2align 4
  .type cuCtxRecordEvent, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuCtxRecordEvent
#endif
cuCtxRecordEvent:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+384(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+384(%rip)
2:
  pushq $48
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuCtxResetPersistingL2Cache
  .p2align 4
  .type cuCtxResetPersistingL2Cache, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuCtxResetPersistingL2Cache
#endif
cuCtxResetPersistingL2Cache:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+392(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+392(%rip)
2:
  pushq $49
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuCtxSetCacheConfig
  .p2align 4
  .type cuCtxSetCacheConfig, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuCtxSetCacheConfig
#endif
cuCtxSetCacheConfig:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+400(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+400(%rip)
2:
  pushq $50
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuCtxSetCurrent
  .p2align 4
  .type cuCtxSetCurrent, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuCtxSetCurrent
#endif
cuCtxSetCurrent:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+408(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+408(%rip)
2:
  pushq $51
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuCtxSetFlags
  .p2align 4
  .type cuCtxSetFlags, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuCtxSetFlags
#endif
cuCtxSetFlags:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+416(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+416(%rip)
2:
  pushq $52
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuCtxSetLimit
  .p2align 4
  .type cuCtxSetLimit, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuCtxSetLimit
#endif
cuCtxSetLimit:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+424(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+424(%rip)
2:
  pushq $53
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuCtxSetSharedMemConfig
  .p2align 4
  .type cuCtxSetSharedMemConfig, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuCtxSetSharedMemConfig
#endif
cuCtxSetSharedMemConfig:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+432(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+432(%rip)
2:
  pushq $54
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuCtxSynchronize
  .p2align 4
  .type cuCtxSynchronize, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuCtxSynchronize
#endif
cuCtxSynchronize:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+440(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+440(%rip)
2:
  pushq $55
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuCtxWaitEvent
  .p2align 4
  .type cuCtxWaitEvent, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuCtxWaitEvent
#endif
cuCtxWaitEvent:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+448(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+448(%rip)
2:
  pushq $56
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuDestroyExternalMemory
  .p2align 4
  .type cuDestroyExternalMemory, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuDestroyExternalMemory
#endif
cuDestroyExternalMemory:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+456(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+456(%rip)
2:
  pushq $57
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuDestroyExternalSemaphore
  .p2align 4
  .type cuDestroyExternalSemaphore, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuDestroyExternalSemaphore
#endif
cuDestroyExternalSemaphore:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+464(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+464(%rip)
2:
  pushq $58
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuDevResourceGenerateDesc
  .p2align 4
  .type cuDevResourceGenerateDesc, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuDevResourceGenerateDesc
#endif
cuDevResourceGenerateDesc:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+472(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+472(%rip)
2:
  pushq $59
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuDevSmResourceSplitByCount
  .p2align 4
  .type cuDevSmResourceSplitByCount, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuDevSmResourceSplitByCount
#endif
cuDevSmResourceSplitByCount:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+480(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+480(%rip)
2:
  pushq $60
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuDeviceCanAccessPeer
  .p2align 4
  .type cuDeviceCanAccessPeer, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuDeviceCanAccessPeer
#endif
cuDeviceCanAccessPeer:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+488(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+488(%rip)
2:
  pushq $61
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuDeviceComputeCapability
  .p2align 4
  .type cuDeviceComputeCapability, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuDeviceComputeCapability
#endif
cuDeviceComputeCapability:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+496(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+496(%rip)
2:
  pushq $62
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuDeviceGet
  .p2align 4
  .type cuDeviceGet, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuDeviceGet
#endif
cuDeviceGet:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+504(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+504(%rip)
2:
  pushq $63
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuDeviceGetAttribute
  .p2align 4
  .type cuDeviceGetAttribute, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuDeviceGetAttribute
#endif
cuDeviceGetAttribute:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+512(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+512(%rip)
2:
  pushq $64
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuDeviceGetByPCIBusId
  .p2align 4
  .type cuDeviceGetByPCIBusId, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuDeviceGetByPCIBusId
#endif
cuDeviceGetByPCIBusId:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+520(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+520(%rip)
2:
  pushq $65
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuDeviceGetCount
  .p2align 4
  .type cuDeviceGetCount, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuDeviceGetCount
#endif
cuDeviceGetCount:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+528(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+528(%rip)
2:
  pushq $66
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuDeviceGetDefaultMemPool
  .p2align 4
  .type cuDeviceGetDefaultMemPool, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuDeviceGetDefaultMemPool
#endif
cuDeviceGetDefaultMemPool:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+536(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+536(%rip)
2:
  pushq $67
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuDeviceGetDevResource
  .p2align 4
  .type cuDeviceGetDevResource, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuDeviceGetDevResource
#endif
cuDeviceGetDevResource:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+544(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+544(%rip)
2:
  pushq $68
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuDeviceGetExecAffinitySupport
  .p2align 4
  .type cuDeviceGetExecAffinitySupport, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuDeviceGetExecAffinitySupport
#endif
cuDeviceGetExecAffinitySupport:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+552(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+552(%rip)
2:
  pushq $69
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuDeviceGetGraphMemAttribute
  .p2align 4
  .type cuDeviceGetGraphMemAttribute, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuDeviceGetGraphMemAttribute
#endif
cuDeviceGetGraphMemAttribute:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+560(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+560(%rip)
2:
  pushq $70
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuDeviceGetLuid
  .p2align 4
  .type cuDeviceGetLuid, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuDeviceGetLuid
#endif
cuDeviceGetLuid:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+568(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+568(%rip)
2:
  pushq $71
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuDeviceGetMemPool
  .p2align 4
  .type cuDeviceGetMemPool, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuDeviceGetMemPool
#endif
cuDeviceGetMemPool:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+576(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+576(%rip)
2:
  pushq $72
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuDeviceGetName
  .p2align 4
  .type cuDeviceGetName, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuDeviceGetName
#endif
cuDeviceGetName:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+584(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+584(%rip)
2:
  pushq $73
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuDeviceGetNvSciSyncAttributes
  .p2align 4
  .type cuDeviceGetNvSciSyncAttributes, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuDeviceGetNvSciSyncAttributes
#endif
cuDeviceGetNvSciSyncAttributes:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+592(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+592(%rip)
2:
  pushq $74
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuDeviceGetP2PAttribute
  .p2align 4
  .type cuDeviceGetP2PAttribute, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuDeviceGetP2PAttribute
#endif
cuDeviceGetP2PAttribute:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+600(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+600(%rip)
2:
  pushq $75
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuDeviceGetPCIBusId
  .p2align 4
  .type cuDeviceGetPCIBusId, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuDeviceGetPCIBusId
#endif
cuDeviceGetPCIBusId:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+608(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+608(%rip)
2:
  pushq $76
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuDeviceGetProperties
  .p2align 4
  .type cuDeviceGetProperties, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuDeviceGetProperties
#endif
cuDeviceGetProperties:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+616(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+616(%rip)
2:
  pushq $77
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuDeviceGetTexture1DLinearMaxWidth
  .p2align 4
  .type cuDeviceGetTexture1DLinearMaxWidth, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuDeviceGetTexture1DLinearMaxWidth
#endif
cuDeviceGetTexture1DLinearMaxWidth:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+624(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+624(%rip)
2:
  pushq $78
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuDeviceGetUuid
  .p2align 4
  .type cuDeviceGetUuid, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuDeviceGetUuid
#endif
cuDeviceGetUuid:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+632(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+632(%rip)
2:
  pushq $79
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuDeviceGetUuid_v2
  .p2align 4
  .type cuDeviceGetUuid_v2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuDeviceGetUuid_v2
#endif
cuDeviceGetUuid_v2:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+640(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+640(%rip)
2:
  pushq $80
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuDeviceGraphMemTrim
  .p2align 4
  .type cuDeviceGraphMemTrim, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuDeviceGraphMemTrim
#endif
cuDeviceGraphMemTrim:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+648(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+648(%rip)
2:
  pushq $81
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuDevicePrimaryCtxGetState
  .p2align 4
  .type cuDevicePrimaryCtxGetState, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuDevicePrimaryCtxGetState
#endif
cuDevicePrimaryCtxGetState:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+656(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+656(%rip)
2:
  pushq $82
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuDevicePrimaryCtxRelease
  .p2align 4
  .type cuDevicePrimaryCtxRelease, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuDevicePrimaryCtxRelease
#endif
cuDevicePrimaryCtxRelease:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+664(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+664(%rip)
2:
  pushq $83
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuDevicePrimaryCtxRelease_v2
  .p2align 4
  .type cuDevicePrimaryCtxRelease_v2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuDevicePrimaryCtxRelease_v2
#endif
cuDevicePrimaryCtxRelease_v2:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+672(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+672(%rip)
2:
  pushq $84
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuDevicePrimaryCtxReset
  .p2align 4
  .type cuDevicePrimaryCtxReset, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuDevicePrimaryCtxReset
#endif
cuDevicePrimaryCtxReset:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+680(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+680(%rip)
2:
  pushq $85
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuDevicePrimaryCtxReset_v2
  .p2align 4
  .type cuDevicePrimaryCtxReset_v2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuDevicePrimaryCtxReset_v2
#endif
cuDevicePrimaryCtxReset_v2:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+688(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+688(%rip)
2:
  pushq $86
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuDevicePrimaryCtxRetain
  .p2align 4
  .type cuDevicePrimaryCtxRetain, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuDevicePrimaryCtxRetain
#endif
cuDevicePrimaryCtxRetain:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+696(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+696(%rip)
2:
  pushq $87
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuDevicePrimaryCtxSetFlags
  .p2align 4
  .type cuDevicePrimaryCtxSetFlags, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuDevicePrimaryCtxSetFlags
#endif
cuDevicePrimaryCtxSetFlags:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+704(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+704(%rip)
2:
  pushq $88
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuDevicePrimaryCtxSetFlags_v2
  .p2align 4
  .type cuDevicePrimaryCtxSetFlags_v2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuDevicePrimaryCtxSetFlags_v2
#endif
cuDevicePrimaryCtxSetFlags_v2:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+712(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+712(%rip)
2:
  pushq $89
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuDeviceRegisterAsyncNotification
  .p2align 4
  .type cuDeviceRegisterAsyncNotification, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuDeviceRegisterAsyncNotification
#endif
cuDeviceRegisterAsyncNotification:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+720(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+720(%rip)
2:
  pushq $90
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuDeviceSetGraphMemAttribute
  .p2align 4
  .type cuDeviceSetGraphMemAttribute, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuDeviceSetGraphMemAttribute
#endif
cuDeviceSetGraphMemAttribute:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+728(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+728(%rip)
2:
  pushq $91
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuDeviceSetMemPool
  .p2align 4
  .type cuDeviceSetMemPool, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuDeviceSetMemPool
#endif
cuDeviceSetMemPool:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+736(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+736(%rip)
2:
  pushq $92
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuDeviceTotalMem
  .p2align 4
  .type cuDeviceTotalMem, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuDeviceTotalMem
#endif
cuDeviceTotalMem:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+744(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+744(%rip)
2:
  pushq $93
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuDeviceTotalMem_v2
  .p2align 4
  .type cuDeviceTotalMem_v2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuDeviceTotalMem_v2
#endif
cuDeviceTotalMem_v2:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+752(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+752(%rip)
2:
  pushq $94
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuDeviceUnregisterAsyncNotification
  .p2align 4
  .type cuDeviceUnregisterAsyncNotification, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuDeviceUnregisterAsyncNotification
#endif
cuDeviceUnregisterAsyncNotification:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+760(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+760(%rip)
2:
  pushq $95
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuDriverGetVersion
  .p2align 4
  .type cuDriverGetVersion, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuDriverGetVersion
#endif
cuDriverGetVersion:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+768(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+768(%rip)
2:
  pushq $96
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuEGLApiInit
  .p2align 4
  .type cuEGLApiInit, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuEGLApiInit
#endif
cuEGLApiInit:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+776(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+776(%rip)
2:
  pushq $97
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuEGLStreamConsumerAcquireFrame
  .p2align 4
  .type cuEGLStreamConsumerAcquireFrame, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuEGLStreamConsumerAcquireFrame
#endif
cuEGLStreamConsumerAcquireFrame:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+784(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+784(%rip)
2:
  pushq $98
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuEGLStreamConsumerConnect
  .p2align 4
  .type cuEGLStreamConsumerConnect, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuEGLStreamConsumerConnect
#endif
cuEGLStreamConsumerConnect:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+792(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+792(%rip)
2:
  pushq $99
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuEGLStreamConsumerConnectWithFlags
  .p2align 4
  .type cuEGLStreamConsumerConnectWithFlags, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuEGLStreamConsumerConnectWithFlags
#endif
cuEGLStreamConsumerConnectWithFlags:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+800(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+800(%rip)
2:
  pushq $100
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuEGLStreamConsumerDisconnect
  .p2align 4
  .type cuEGLStreamConsumerDisconnect, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuEGLStreamConsumerDisconnect
#endif
cuEGLStreamConsumerDisconnect:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+808(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+808(%rip)
2:
  pushq $101
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuEGLStreamConsumerReleaseFrame
  .p2align 4
  .type cuEGLStreamConsumerReleaseFrame, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuEGLStreamConsumerReleaseFrame
#endif
cuEGLStreamConsumerReleaseFrame:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+816(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+816(%rip)
2:
  pushq $102
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuEGLStreamProducerConnect
  .p2align 4
  .type cuEGLStreamProducerConnect, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuEGLStreamProducerConnect
#endif
cuEGLStreamProducerConnect:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+824(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+824(%rip)
2:
  pushq $103
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuEGLStreamProducerDisconnect
  .p2align 4
  .type cuEGLStreamProducerDisconnect, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuEGLStreamProducerDisconnect
#endif
cuEGLStreamProducerDisconnect:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+832(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+832(%rip)
2:
  pushq $104
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuEGLStreamProducerPresentFrame
  .p2align 4
  .type cuEGLStreamProducerPresentFrame, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuEGLStreamProducerPresentFrame
#endif
cuEGLStreamProducerPresentFrame:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+840(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+840(%rip)
2:
  pushq $105
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuEGLStreamProducerReturnFrame
  .p2align 4
  .type cuEGLStreamProducerReturnFrame, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuEGLStreamProducerReturnFrame
#endif
cuEGLStreamProducerReturnFrame:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+848(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+848(%rip)
2:
  pushq $106
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuEventCreate
  .p2align 4
  .type cuEventCreate, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuEventCreate
#endif
cuEventCreate:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+856(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+856(%rip)
2:
  pushq $107
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuEventDestroy
  .p2align 4
  .type cuEventDestroy, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuEventDestroy
#endif
cuEventDestroy:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+864(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+864(%rip)
2:
  pushq $108
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuEventDestroy_v2
  .p2align 4
  .type cuEventDestroy_v2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuEventDestroy_v2
#endif
cuEventDestroy_v2:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+872(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+872(%rip)
2:
  pushq $109
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuEventElapsedTime
  .p2align 4
  .type cuEventElapsedTime, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuEventElapsedTime
#endif
cuEventElapsedTime:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+880(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+880(%rip)
2:
  pushq $110
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuEventElapsedTime_v2
  .p2align 4
  .type cuEventElapsedTime_v2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuEventElapsedTime_v2
#endif
cuEventElapsedTime_v2:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+888(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+888(%rip)
2:
  pushq $111
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuEventQuery
  .p2align 4
  .type cuEventQuery, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuEventQuery
#endif
cuEventQuery:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+896(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+896(%rip)
2:
  pushq $112
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuEventRecord
  .p2align 4
  .type cuEventRecord, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuEventRecord
#endif
cuEventRecord:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+904(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+904(%rip)
2:
  pushq $113
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuEventRecordWithFlags
  .p2align 4
  .type cuEventRecordWithFlags, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuEventRecordWithFlags
#endif
cuEventRecordWithFlags:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+912(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+912(%rip)
2:
  pushq $114
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuEventRecordWithFlags_ptsz
  .p2align 4
  .type cuEventRecordWithFlags_ptsz, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuEventRecordWithFlags_ptsz
#endif
cuEventRecordWithFlags_ptsz:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+920(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+920(%rip)
2:
  pushq $115
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuEventRecord_ptsz
  .p2align 4
  .type cuEventRecord_ptsz, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuEventRecord_ptsz
#endif
cuEventRecord_ptsz:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+928(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+928(%rip)
2:
  pushq $116
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuEventSynchronize
  .p2align 4
  .type cuEventSynchronize, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuEventSynchronize
#endif
cuEventSynchronize:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+936(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+936(%rip)
2:
  pushq $117
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuExternalMemoryGetMappedBuffer
  .p2align 4
  .type cuExternalMemoryGetMappedBuffer, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuExternalMemoryGetMappedBuffer
#endif
cuExternalMemoryGetMappedBuffer:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+944(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+944(%rip)
2:
  pushq $118
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuExternalMemoryGetMappedMipmappedArray
  .p2align 4
  .type cuExternalMemoryGetMappedMipmappedArray, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuExternalMemoryGetMappedMipmappedArray
#endif
cuExternalMemoryGetMappedMipmappedArray:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+952(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+952(%rip)
2:
  pushq $119
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuFlushGPUDirectRDMAWrites
  .p2align 4
  .type cuFlushGPUDirectRDMAWrites, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuFlushGPUDirectRDMAWrites
#endif
cuFlushGPUDirectRDMAWrites:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+960(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+960(%rip)
2:
  pushq $120
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuFuncGetAttribute
  .p2align 4
  .type cuFuncGetAttribute, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuFuncGetAttribute
#endif
cuFuncGetAttribute:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+968(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+968(%rip)
2:
  pushq $121
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuFuncGetModule
  .p2align 4
  .type cuFuncGetModule, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuFuncGetModule
#endif
cuFuncGetModule:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+976(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+976(%rip)
2:
  pushq $122
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuFuncGetName
  .p2align 4
  .type cuFuncGetName, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuFuncGetName
#endif
cuFuncGetName:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+984(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+984(%rip)
2:
  pushq $123
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuFuncGetParamInfo
  .p2align 4
  .type cuFuncGetParamInfo, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuFuncGetParamInfo
#endif
cuFuncGetParamInfo:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+992(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+992(%rip)
2:
  pushq $124
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuFuncIsLoaded
  .p2align 4
  .type cuFuncIsLoaded, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuFuncIsLoaded
#endif
cuFuncIsLoaded:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1000(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1000(%rip)
2:
  pushq $125
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuFuncLoad
  .p2align 4
  .type cuFuncLoad, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuFuncLoad
#endif
cuFuncLoad:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1008(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1008(%rip)
2:
  pushq $126
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuFuncSetAttribute
  .p2align 4
  .type cuFuncSetAttribute, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuFuncSetAttribute
#endif
cuFuncSetAttribute:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1016(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1016(%rip)
2:
  pushq $127
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuFuncSetBlockShape
  .p2align 4
  .type cuFuncSetBlockShape, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuFuncSetBlockShape
#endif
cuFuncSetBlockShape:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1024(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1024(%rip)
2:
  pushq $128
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuFuncSetCacheConfig
  .p2align 4
  .type cuFuncSetCacheConfig, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuFuncSetCacheConfig
#endif
cuFuncSetCacheConfig:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1032(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1032(%rip)
2:
  pushq $129
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuFuncSetSharedMemConfig
  .p2align 4
  .type cuFuncSetSharedMemConfig, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuFuncSetSharedMemConfig
#endif
cuFuncSetSharedMemConfig:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1040(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1040(%rip)
2:
  pushq $130
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuFuncSetSharedSize
  .p2align 4
  .type cuFuncSetSharedSize, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuFuncSetSharedSize
#endif
cuFuncSetSharedSize:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1048(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1048(%rip)
2:
  pushq $131
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGLCtxCreate
  .p2align 4
  .type cuGLCtxCreate, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGLCtxCreate
#endif
cuGLCtxCreate:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1056(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1056(%rip)
2:
  pushq $132
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGLCtxCreate_v2
  .p2align 4
  .type cuGLCtxCreate_v2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGLCtxCreate_v2
#endif
cuGLCtxCreate_v2:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1064(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1064(%rip)
2:
  pushq $133
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGLGetDevices
  .p2align 4
  .type cuGLGetDevices, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGLGetDevices
#endif
cuGLGetDevices:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1072(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1072(%rip)
2:
  pushq $134
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGLGetDevices_v2
  .p2align 4
  .type cuGLGetDevices_v2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGLGetDevices_v2
#endif
cuGLGetDevices_v2:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1080(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1080(%rip)
2:
  pushq $135
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGLInit
  .p2align 4
  .type cuGLInit, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGLInit
#endif
cuGLInit:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1088(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1088(%rip)
2:
  pushq $136
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGLMapBufferObject
  .p2align 4
  .type cuGLMapBufferObject, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGLMapBufferObject
#endif
cuGLMapBufferObject:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1096(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1096(%rip)
2:
  pushq $137
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGLMapBufferObjectAsync
  .p2align 4
  .type cuGLMapBufferObjectAsync, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGLMapBufferObjectAsync
#endif
cuGLMapBufferObjectAsync:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1104(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1104(%rip)
2:
  pushq $138
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGLMapBufferObjectAsync_v2
  .p2align 4
  .type cuGLMapBufferObjectAsync_v2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGLMapBufferObjectAsync_v2
#endif
cuGLMapBufferObjectAsync_v2:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1112(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1112(%rip)
2:
  pushq $139
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGLMapBufferObjectAsync_v2_ptsz
  .p2align 4
  .type cuGLMapBufferObjectAsync_v2_ptsz, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGLMapBufferObjectAsync_v2_ptsz
#endif
cuGLMapBufferObjectAsync_v2_ptsz:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1120(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1120(%rip)
2:
  pushq $140
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGLMapBufferObject_v2
  .p2align 4
  .type cuGLMapBufferObject_v2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGLMapBufferObject_v2
#endif
cuGLMapBufferObject_v2:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1128(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1128(%rip)
2:
  pushq $141
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGLMapBufferObject_v2_ptds
  .p2align 4
  .type cuGLMapBufferObject_v2_ptds, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGLMapBufferObject_v2_ptds
#endif
cuGLMapBufferObject_v2_ptds:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1136(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1136(%rip)
2:
  pushq $142
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGLRegisterBufferObject
  .p2align 4
  .type cuGLRegisterBufferObject, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGLRegisterBufferObject
#endif
cuGLRegisterBufferObject:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1144(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1144(%rip)
2:
  pushq $143
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGLSetBufferObjectMapFlags
  .p2align 4
  .type cuGLSetBufferObjectMapFlags, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGLSetBufferObjectMapFlags
#endif
cuGLSetBufferObjectMapFlags:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1152(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1152(%rip)
2:
  pushq $144
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGLUnmapBufferObject
  .p2align 4
  .type cuGLUnmapBufferObject, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGLUnmapBufferObject
#endif
cuGLUnmapBufferObject:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1160(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1160(%rip)
2:
  pushq $145
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGLUnmapBufferObjectAsync
  .p2align 4
  .type cuGLUnmapBufferObjectAsync, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGLUnmapBufferObjectAsync
#endif
cuGLUnmapBufferObjectAsync:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1168(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1168(%rip)
2:
  pushq $146
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGLUnregisterBufferObject
  .p2align 4
  .type cuGLUnregisterBufferObject, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGLUnregisterBufferObject
#endif
cuGLUnregisterBufferObject:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1176(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1176(%rip)
2:
  pushq $147
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGetErrorName
  .p2align 4
  .type cuGetErrorName, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGetErrorName
#endif
cuGetErrorName:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1184(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1184(%rip)
2:
  pushq $148
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGetErrorString
  .p2align 4
  .type cuGetErrorString, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGetErrorString
#endif
cuGetErrorString:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1192(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1192(%rip)
2:
  pushq $149
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGetExportTable
  .p2align 4
  .type cuGetExportTable, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGetExportTable
#endif
cuGetExportTable:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1200(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1200(%rip)
2:
  pushq $150
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGetProcAddress
  .p2align 4
  .type cuGetProcAddress, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGetProcAddress
#endif
cuGetProcAddress:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1208(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1208(%rip)
2:
  pushq $151
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGetProcAddress_v2
  .p2align 4
  .type cuGetProcAddress_v2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGetProcAddress_v2
#endif
cuGetProcAddress_v2:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1216(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1216(%rip)
2:
  pushq $152
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphAddBatchMemOpNode
  .p2align 4
  .type cuGraphAddBatchMemOpNode, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphAddBatchMemOpNode
#endif
cuGraphAddBatchMemOpNode:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1224(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1224(%rip)
2:
  pushq $153
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphAddChildGraphNode
  .p2align 4
  .type cuGraphAddChildGraphNode, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphAddChildGraphNode
#endif
cuGraphAddChildGraphNode:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1232(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1232(%rip)
2:
  pushq $154
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphAddDependencies
  .p2align 4
  .type cuGraphAddDependencies, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphAddDependencies
#endif
cuGraphAddDependencies:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1240(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1240(%rip)
2:
  pushq $155
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphAddDependencies_v2
  .p2align 4
  .type cuGraphAddDependencies_v2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphAddDependencies_v2
#endif
cuGraphAddDependencies_v2:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1248(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1248(%rip)
2:
  pushq $156
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphAddEmptyNode
  .p2align 4
  .type cuGraphAddEmptyNode, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphAddEmptyNode
#endif
cuGraphAddEmptyNode:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1256(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1256(%rip)
2:
  pushq $157
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphAddEventRecordNode
  .p2align 4
  .type cuGraphAddEventRecordNode, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphAddEventRecordNode
#endif
cuGraphAddEventRecordNode:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1264(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1264(%rip)
2:
  pushq $158
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphAddEventWaitNode
  .p2align 4
  .type cuGraphAddEventWaitNode, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphAddEventWaitNode
#endif
cuGraphAddEventWaitNode:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1272(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1272(%rip)
2:
  pushq $159
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphAddExternalSemaphoresSignalNode
  .p2align 4
  .type cuGraphAddExternalSemaphoresSignalNode, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphAddExternalSemaphoresSignalNode
#endif
cuGraphAddExternalSemaphoresSignalNode:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1280(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1280(%rip)
2:
  pushq $160
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphAddExternalSemaphoresWaitNode
  .p2align 4
  .type cuGraphAddExternalSemaphoresWaitNode, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphAddExternalSemaphoresWaitNode
#endif
cuGraphAddExternalSemaphoresWaitNode:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1288(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1288(%rip)
2:
  pushq $161
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphAddHostNode
  .p2align 4
  .type cuGraphAddHostNode, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphAddHostNode
#endif
cuGraphAddHostNode:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1296(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1296(%rip)
2:
  pushq $162
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphAddKernelNode
  .p2align 4
  .type cuGraphAddKernelNode, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphAddKernelNode
#endif
cuGraphAddKernelNode:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1304(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1304(%rip)
2:
  pushq $163
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphAddKernelNode_v2
  .p2align 4
  .type cuGraphAddKernelNode_v2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphAddKernelNode_v2
#endif
cuGraphAddKernelNode_v2:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1312(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1312(%rip)
2:
  pushq $164
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphAddMemAllocNode
  .p2align 4
  .type cuGraphAddMemAllocNode, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphAddMemAllocNode
#endif
cuGraphAddMemAllocNode:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1320(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1320(%rip)
2:
  pushq $165
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphAddMemFreeNode
  .p2align 4
  .type cuGraphAddMemFreeNode, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphAddMemFreeNode
#endif
cuGraphAddMemFreeNode:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1328(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1328(%rip)
2:
  pushq $166
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphAddMemcpyNode
  .p2align 4
  .type cuGraphAddMemcpyNode, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphAddMemcpyNode
#endif
cuGraphAddMemcpyNode:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1336(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1336(%rip)
2:
  pushq $167
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphAddMemsetNode
  .p2align 4
  .type cuGraphAddMemsetNode, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphAddMemsetNode
#endif
cuGraphAddMemsetNode:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1344(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1344(%rip)
2:
  pushq $168
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphAddNode
  .p2align 4
  .type cuGraphAddNode, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphAddNode
#endif
cuGraphAddNode:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1352(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1352(%rip)
2:
  pushq $169
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphAddNode_v2
  .p2align 4
  .type cuGraphAddNode_v2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphAddNode_v2
#endif
cuGraphAddNode_v2:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1360(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1360(%rip)
2:
  pushq $170
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphBatchMemOpNodeGetParams
  .p2align 4
  .type cuGraphBatchMemOpNodeGetParams, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphBatchMemOpNodeGetParams
#endif
cuGraphBatchMemOpNodeGetParams:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1368(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1368(%rip)
2:
  pushq $171
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphBatchMemOpNodeSetParams
  .p2align 4
  .type cuGraphBatchMemOpNodeSetParams, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphBatchMemOpNodeSetParams
#endif
cuGraphBatchMemOpNodeSetParams:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1376(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1376(%rip)
2:
  pushq $172
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphChildGraphNodeGetGraph
  .p2align 4
  .type cuGraphChildGraphNodeGetGraph, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphChildGraphNodeGetGraph
#endif
cuGraphChildGraphNodeGetGraph:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1384(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1384(%rip)
2:
  pushq $173
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphClone
  .p2align 4
  .type cuGraphClone, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphClone
#endif
cuGraphClone:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1392(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1392(%rip)
2:
  pushq $174
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphConditionalHandleCreate
  .p2align 4
  .type cuGraphConditionalHandleCreate, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphConditionalHandleCreate
#endif
cuGraphConditionalHandleCreate:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1400(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1400(%rip)
2:
  pushq $175
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphCreate
  .p2align 4
  .type cuGraphCreate, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphCreate
#endif
cuGraphCreate:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1408(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1408(%rip)
2:
  pushq $176
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphDebugDotPrint
  .p2align 4
  .type cuGraphDebugDotPrint, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphDebugDotPrint
#endif
cuGraphDebugDotPrint:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1416(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1416(%rip)
2:
  pushq $177
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphDestroy
  .p2align 4
  .type cuGraphDestroy, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphDestroy
#endif
cuGraphDestroy:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1424(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1424(%rip)
2:
  pushq $178
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphDestroyNode
  .p2align 4
  .type cuGraphDestroyNode, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphDestroyNode
#endif
cuGraphDestroyNode:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1432(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1432(%rip)
2:
  pushq $179
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphEventRecordNodeGetEvent
  .p2align 4
  .type cuGraphEventRecordNodeGetEvent, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphEventRecordNodeGetEvent
#endif
cuGraphEventRecordNodeGetEvent:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1440(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1440(%rip)
2:
  pushq $180
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphEventRecordNodeSetEvent
  .p2align 4
  .type cuGraphEventRecordNodeSetEvent, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphEventRecordNodeSetEvent
#endif
cuGraphEventRecordNodeSetEvent:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1448(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1448(%rip)
2:
  pushq $181
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphEventWaitNodeGetEvent
  .p2align 4
  .type cuGraphEventWaitNodeGetEvent, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphEventWaitNodeGetEvent
#endif
cuGraphEventWaitNodeGetEvent:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1456(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1456(%rip)
2:
  pushq $182
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphEventWaitNodeSetEvent
  .p2align 4
  .type cuGraphEventWaitNodeSetEvent, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphEventWaitNodeSetEvent
#endif
cuGraphEventWaitNodeSetEvent:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1464(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1464(%rip)
2:
  pushq $183
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphExecBatchMemOpNodeSetParams
  .p2align 4
  .type cuGraphExecBatchMemOpNodeSetParams, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphExecBatchMemOpNodeSetParams
#endif
cuGraphExecBatchMemOpNodeSetParams:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1472(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1472(%rip)
2:
  pushq $184
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphExecChildGraphNodeSetParams
  .p2align 4
  .type cuGraphExecChildGraphNodeSetParams, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphExecChildGraphNodeSetParams
#endif
cuGraphExecChildGraphNodeSetParams:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1480(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1480(%rip)
2:
  pushq $185
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphExecDestroy
  .p2align 4
  .type cuGraphExecDestroy, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphExecDestroy
#endif
cuGraphExecDestroy:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1488(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1488(%rip)
2:
  pushq $186
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphExecEventRecordNodeSetEvent
  .p2align 4
  .type cuGraphExecEventRecordNodeSetEvent, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphExecEventRecordNodeSetEvent
#endif
cuGraphExecEventRecordNodeSetEvent:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1496(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1496(%rip)
2:
  pushq $187
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphExecEventWaitNodeSetEvent
  .p2align 4
  .type cuGraphExecEventWaitNodeSetEvent, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphExecEventWaitNodeSetEvent
#endif
cuGraphExecEventWaitNodeSetEvent:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1504(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1504(%rip)
2:
  pushq $188
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphExecExternalSemaphoresSignalNodeSetParams
  .p2align 4
  .type cuGraphExecExternalSemaphoresSignalNodeSetParams, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphExecExternalSemaphoresSignalNodeSetParams
#endif
cuGraphExecExternalSemaphoresSignalNodeSetParams:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1512(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1512(%rip)
2:
  pushq $189
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphExecExternalSemaphoresWaitNodeSetParams
  .p2align 4
  .type cuGraphExecExternalSemaphoresWaitNodeSetParams, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphExecExternalSemaphoresWaitNodeSetParams
#endif
cuGraphExecExternalSemaphoresWaitNodeSetParams:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1520(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1520(%rip)
2:
  pushq $190
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphExecGetFlags
  .p2align 4
  .type cuGraphExecGetFlags, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphExecGetFlags
#endif
cuGraphExecGetFlags:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1528(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1528(%rip)
2:
  pushq $191
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphExecHostNodeSetParams
  .p2align 4
  .type cuGraphExecHostNodeSetParams, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphExecHostNodeSetParams
#endif
cuGraphExecHostNodeSetParams:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1536(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1536(%rip)
2:
  pushq $192
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphExecKernelNodeSetParams
  .p2align 4
  .type cuGraphExecKernelNodeSetParams, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphExecKernelNodeSetParams
#endif
cuGraphExecKernelNodeSetParams:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1544(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1544(%rip)
2:
  pushq $193
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphExecKernelNodeSetParams_v2
  .p2align 4
  .type cuGraphExecKernelNodeSetParams_v2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphExecKernelNodeSetParams_v2
#endif
cuGraphExecKernelNodeSetParams_v2:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1552(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1552(%rip)
2:
  pushq $194
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphExecMemcpyNodeSetParams
  .p2align 4
  .type cuGraphExecMemcpyNodeSetParams, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphExecMemcpyNodeSetParams
#endif
cuGraphExecMemcpyNodeSetParams:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1560(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1560(%rip)
2:
  pushq $195
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphExecMemsetNodeSetParams
  .p2align 4
  .type cuGraphExecMemsetNodeSetParams, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphExecMemsetNodeSetParams
#endif
cuGraphExecMemsetNodeSetParams:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1568(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1568(%rip)
2:
  pushq $196
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphExecNodeSetParams
  .p2align 4
  .type cuGraphExecNodeSetParams, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphExecNodeSetParams
#endif
cuGraphExecNodeSetParams:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1576(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1576(%rip)
2:
  pushq $197
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphExecUpdate
  .p2align 4
  .type cuGraphExecUpdate, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphExecUpdate
#endif
cuGraphExecUpdate:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1584(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1584(%rip)
2:
  pushq $198
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphExecUpdate_v2
  .p2align 4
  .type cuGraphExecUpdate_v2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphExecUpdate_v2
#endif
cuGraphExecUpdate_v2:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1592(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1592(%rip)
2:
  pushq $199
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphExternalSemaphoresSignalNodeGetParams
  .p2align 4
  .type cuGraphExternalSemaphoresSignalNodeGetParams, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphExternalSemaphoresSignalNodeGetParams
#endif
cuGraphExternalSemaphoresSignalNodeGetParams:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1600(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1600(%rip)
2:
  pushq $200
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphExternalSemaphoresSignalNodeSetParams
  .p2align 4
  .type cuGraphExternalSemaphoresSignalNodeSetParams, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphExternalSemaphoresSignalNodeSetParams
#endif
cuGraphExternalSemaphoresSignalNodeSetParams:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1608(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1608(%rip)
2:
  pushq $201
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphExternalSemaphoresWaitNodeGetParams
  .p2align 4
  .type cuGraphExternalSemaphoresWaitNodeGetParams, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphExternalSemaphoresWaitNodeGetParams
#endif
cuGraphExternalSemaphoresWaitNodeGetParams:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1616(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1616(%rip)
2:
  pushq $202
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphExternalSemaphoresWaitNodeSetParams
  .p2align 4
  .type cuGraphExternalSemaphoresWaitNodeSetParams, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphExternalSemaphoresWaitNodeSetParams
#endif
cuGraphExternalSemaphoresWaitNodeSetParams:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1624(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1624(%rip)
2:
  pushq $203
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphGetEdges
  .p2align 4
  .type cuGraphGetEdges, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphGetEdges
#endif
cuGraphGetEdges:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1632(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1632(%rip)
2:
  pushq $204
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphGetEdges_v2
  .p2align 4
  .type cuGraphGetEdges_v2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphGetEdges_v2
#endif
cuGraphGetEdges_v2:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1640(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1640(%rip)
2:
  pushq $205
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphGetNodes
  .p2align 4
  .type cuGraphGetNodes, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphGetNodes
#endif
cuGraphGetNodes:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1648(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1648(%rip)
2:
  pushq $206
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphGetRootNodes
  .p2align 4
  .type cuGraphGetRootNodes, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphGetRootNodes
#endif
cuGraphGetRootNodes:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1656(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1656(%rip)
2:
  pushq $207
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphHostNodeGetParams
  .p2align 4
  .type cuGraphHostNodeGetParams, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphHostNodeGetParams
#endif
cuGraphHostNodeGetParams:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1664(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1664(%rip)
2:
  pushq $208
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphHostNodeSetParams
  .p2align 4
  .type cuGraphHostNodeSetParams, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphHostNodeSetParams
#endif
cuGraphHostNodeSetParams:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1672(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1672(%rip)
2:
  pushq $209
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphInstantiate
  .p2align 4
  .type cuGraphInstantiate, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphInstantiate
#endif
cuGraphInstantiate:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1680(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1680(%rip)
2:
  pushq $210
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphInstantiateWithFlags
  .p2align 4
  .type cuGraphInstantiateWithFlags, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphInstantiateWithFlags
#endif
cuGraphInstantiateWithFlags:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1688(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1688(%rip)
2:
  pushq $211
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphInstantiateWithParams
  .p2align 4
  .type cuGraphInstantiateWithParams, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphInstantiateWithParams
#endif
cuGraphInstantiateWithParams:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1696(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1696(%rip)
2:
  pushq $212
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphInstantiateWithParams_ptsz
  .p2align 4
  .type cuGraphInstantiateWithParams_ptsz, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphInstantiateWithParams_ptsz
#endif
cuGraphInstantiateWithParams_ptsz:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1704(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1704(%rip)
2:
  pushq $213
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphInstantiate_v2
  .p2align 4
  .type cuGraphInstantiate_v2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphInstantiate_v2
#endif
cuGraphInstantiate_v2:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1712(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1712(%rip)
2:
  pushq $214
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphKernelNodeCopyAttributes
  .p2align 4
  .type cuGraphKernelNodeCopyAttributes, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphKernelNodeCopyAttributes
#endif
cuGraphKernelNodeCopyAttributes:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1720(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1720(%rip)
2:
  pushq $215
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphKernelNodeGetAttribute
  .p2align 4
  .type cuGraphKernelNodeGetAttribute, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphKernelNodeGetAttribute
#endif
cuGraphKernelNodeGetAttribute:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1728(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1728(%rip)
2:
  pushq $216
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphKernelNodeGetParams
  .p2align 4
  .type cuGraphKernelNodeGetParams, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphKernelNodeGetParams
#endif
cuGraphKernelNodeGetParams:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1736(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1736(%rip)
2:
  pushq $217
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphKernelNodeGetParams_v2
  .p2align 4
  .type cuGraphKernelNodeGetParams_v2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphKernelNodeGetParams_v2
#endif
cuGraphKernelNodeGetParams_v2:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1744(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1744(%rip)
2:
  pushq $218
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphKernelNodeSetAttribute
  .p2align 4
  .type cuGraphKernelNodeSetAttribute, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphKernelNodeSetAttribute
#endif
cuGraphKernelNodeSetAttribute:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1752(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1752(%rip)
2:
  pushq $219
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphKernelNodeSetParams
  .p2align 4
  .type cuGraphKernelNodeSetParams, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphKernelNodeSetParams
#endif
cuGraphKernelNodeSetParams:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1760(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1760(%rip)
2:
  pushq $220
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphKernelNodeSetParams_v2
  .p2align 4
  .type cuGraphKernelNodeSetParams_v2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphKernelNodeSetParams_v2
#endif
cuGraphKernelNodeSetParams_v2:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1768(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1768(%rip)
2:
  pushq $221
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphLaunch
  .p2align 4
  .type cuGraphLaunch, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphLaunch
#endif
cuGraphLaunch:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1776(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1776(%rip)
2:
  pushq $222
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphLaunch_ptsz
  .p2align 4
  .type cuGraphLaunch_ptsz, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphLaunch_ptsz
#endif
cuGraphLaunch_ptsz:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1784(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1784(%rip)
2:
  pushq $223
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphMemAllocNodeGetParams
  .p2align 4
  .type cuGraphMemAllocNodeGetParams, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphMemAllocNodeGetParams
#endif
cuGraphMemAllocNodeGetParams:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1792(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1792(%rip)
2:
  pushq $224
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphMemFreeNodeGetParams
  .p2align 4
  .type cuGraphMemFreeNodeGetParams, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphMemFreeNodeGetParams
#endif
cuGraphMemFreeNodeGetParams:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1800(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1800(%rip)
2:
  pushq $225
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphMemcpyNodeGetParams
  .p2align 4
  .type cuGraphMemcpyNodeGetParams, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphMemcpyNodeGetParams
#endif
cuGraphMemcpyNodeGetParams:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1808(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1808(%rip)
2:
  pushq $226
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphMemcpyNodeSetParams
  .p2align 4
  .type cuGraphMemcpyNodeSetParams, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphMemcpyNodeSetParams
#endif
cuGraphMemcpyNodeSetParams:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1816(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1816(%rip)
2:
  pushq $227
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphMemsetNodeGetParams
  .p2align 4
  .type cuGraphMemsetNodeGetParams, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphMemsetNodeGetParams
#endif
cuGraphMemsetNodeGetParams:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1824(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1824(%rip)
2:
  pushq $228
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphMemsetNodeSetParams
  .p2align 4
  .type cuGraphMemsetNodeSetParams, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphMemsetNodeSetParams
#endif
cuGraphMemsetNodeSetParams:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1832(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1832(%rip)
2:
  pushq $229
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphNodeFindInClone
  .p2align 4
  .type cuGraphNodeFindInClone, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphNodeFindInClone
#endif
cuGraphNodeFindInClone:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1840(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1840(%rip)
2:
  pushq $230
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphNodeGetDependencies
  .p2align 4
  .type cuGraphNodeGetDependencies, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphNodeGetDependencies
#endif
cuGraphNodeGetDependencies:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1848(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1848(%rip)
2:
  pushq $231
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphNodeGetDependencies_v2
  .p2align 4
  .type cuGraphNodeGetDependencies_v2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphNodeGetDependencies_v2
#endif
cuGraphNodeGetDependencies_v2:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1856(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1856(%rip)
2:
  pushq $232
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphNodeGetDependentNodes
  .p2align 4
  .type cuGraphNodeGetDependentNodes, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphNodeGetDependentNodes
#endif
cuGraphNodeGetDependentNodes:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1864(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1864(%rip)
2:
  pushq $233
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphNodeGetDependentNodes_v2
  .p2align 4
  .type cuGraphNodeGetDependentNodes_v2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphNodeGetDependentNodes_v2
#endif
cuGraphNodeGetDependentNodes_v2:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1872(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1872(%rip)
2:
  pushq $234
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphNodeGetEnabled
  .p2align 4
  .type cuGraphNodeGetEnabled, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphNodeGetEnabled
#endif
cuGraphNodeGetEnabled:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1880(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1880(%rip)
2:
  pushq $235
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphNodeGetType
  .p2align 4
  .type cuGraphNodeGetType, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphNodeGetType
#endif
cuGraphNodeGetType:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1888(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1888(%rip)
2:
  pushq $236
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphNodeSetEnabled
  .p2align 4
  .type cuGraphNodeSetEnabled, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphNodeSetEnabled
#endif
cuGraphNodeSetEnabled:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1896(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1896(%rip)
2:
  pushq $237
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphNodeSetParams
  .p2align 4
  .type cuGraphNodeSetParams, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphNodeSetParams
#endif
cuGraphNodeSetParams:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1904(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1904(%rip)
2:
  pushq $238
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphReleaseUserObject
  .p2align 4
  .type cuGraphReleaseUserObject, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphReleaseUserObject
#endif
cuGraphReleaseUserObject:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1912(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1912(%rip)
2:
  pushq $239
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphRemoveDependencies
  .p2align 4
  .type cuGraphRemoveDependencies, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphRemoveDependencies
#endif
cuGraphRemoveDependencies:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1920(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1920(%rip)
2:
  pushq $240
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphRemoveDependencies_v2
  .p2align 4
  .type cuGraphRemoveDependencies_v2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphRemoveDependencies_v2
#endif
cuGraphRemoveDependencies_v2:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1928(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1928(%rip)
2:
  pushq $241
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphRetainUserObject
  .p2align 4
  .type cuGraphRetainUserObject, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphRetainUserObject
#endif
cuGraphRetainUserObject:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1936(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1936(%rip)
2:
  pushq $242
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphUpload
  .p2align 4
  .type cuGraphUpload, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphUpload
#endif
cuGraphUpload:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1944(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1944(%rip)
2:
  pushq $243
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphUpload_ptsz
  .p2align 4
  .type cuGraphUpload_ptsz, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphUpload_ptsz
#endif
cuGraphUpload_ptsz:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1952(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1952(%rip)
2:
  pushq $244
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphicsEGLRegisterImage
  .p2align 4
  .type cuGraphicsEGLRegisterImage, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphicsEGLRegisterImage
#endif
cuGraphicsEGLRegisterImage:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1960(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1960(%rip)
2:
  pushq $245
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphicsGLRegisterBuffer
  .p2align 4
  .type cuGraphicsGLRegisterBuffer, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphicsGLRegisterBuffer
#endif
cuGraphicsGLRegisterBuffer:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1968(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1968(%rip)
2:
  pushq $246
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphicsGLRegisterImage
  .p2align 4
  .type cuGraphicsGLRegisterImage, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphicsGLRegisterImage
#endif
cuGraphicsGLRegisterImage:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1976(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1976(%rip)
2:
  pushq $247
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphicsMapResources
  .p2align 4
  .type cuGraphicsMapResources, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphicsMapResources
#endif
cuGraphicsMapResources:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1984(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1984(%rip)
2:
  pushq $248
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphicsMapResources_ptsz
  .p2align 4
  .type cuGraphicsMapResources_ptsz, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphicsMapResources_ptsz
#endif
cuGraphicsMapResources_ptsz:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+1992(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+1992(%rip)
2:
  pushq $249
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphicsResourceGetMappedEglFrame
  .p2align 4
  .type cuGraphicsResourceGetMappedEglFrame, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphicsResourceGetMappedEglFrame
#endif
cuGraphicsResourceGetMappedEglFrame:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2000(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2000(%rip)
2:
  pushq $250
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphicsResourceGetMappedMipmappedArray
  .p2align 4
  .type cuGraphicsResourceGetMappedMipmappedArray, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphicsResourceGetMappedMipmappedArray
#endif
cuGraphicsResourceGetMappedMipmappedArray:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2008(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2008(%rip)
2:
  pushq $251
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphicsResourceGetMappedPointer
  .p2align 4
  .type cuGraphicsResourceGetMappedPointer, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphicsResourceGetMappedPointer
#endif
cuGraphicsResourceGetMappedPointer:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2016(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2016(%rip)
2:
  pushq $252
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphicsResourceGetMappedPointer_v2
  .p2align 4
  .type cuGraphicsResourceGetMappedPointer_v2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphicsResourceGetMappedPointer_v2
#endif
cuGraphicsResourceGetMappedPointer_v2:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2024(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2024(%rip)
2:
  pushq $253
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphicsResourceSetMapFlags
  .p2align 4
  .type cuGraphicsResourceSetMapFlags, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphicsResourceSetMapFlags
#endif
cuGraphicsResourceSetMapFlags:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2032(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2032(%rip)
2:
  pushq $254
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphicsResourceSetMapFlags_v2
  .p2align 4
  .type cuGraphicsResourceSetMapFlags_v2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphicsResourceSetMapFlags_v2
#endif
cuGraphicsResourceSetMapFlags_v2:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2040(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2040(%rip)
2:
  pushq $255
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphicsSubResourceGetMappedArray
  .p2align 4
  .type cuGraphicsSubResourceGetMappedArray, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphicsSubResourceGetMappedArray
#endif
cuGraphicsSubResourceGetMappedArray:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2048(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2048(%rip)
2:
  pushq $256
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphicsUnmapResources
  .p2align 4
  .type cuGraphicsUnmapResources, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphicsUnmapResources
#endif
cuGraphicsUnmapResources:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2056(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2056(%rip)
2:
  pushq $257
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphicsUnmapResources_ptsz
  .p2align 4
  .type cuGraphicsUnmapResources_ptsz, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphicsUnmapResources_ptsz
#endif
cuGraphicsUnmapResources_ptsz:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2064(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2064(%rip)
2:
  pushq $258
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphicsUnregisterResource
  .p2align 4
  .type cuGraphicsUnregisterResource, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphicsUnregisterResource
#endif
cuGraphicsUnregisterResource:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2072(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2072(%rip)
2:
  pushq $259
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphicsVDPAURegisterOutputSurface
  .p2align 4
  .type cuGraphicsVDPAURegisterOutputSurface, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphicsVDPAURegisterOutputSurface
#endif
cuGraphicsVDPAURegisterOutputSurface:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2080(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2080(%rip)
2:
  pushq $260
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGraphicsVDPAURegisterVideoSurface
  .p2align 4
  .type cuGraphicsVDPAURegisterVideoSurface, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGraphicsVDPAURegisterVideoSurface
#endif
cuGraphicsVDPAURegisterVideoSurface:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2088(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2088(%rip)
2:
  pushq $261
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGreenCtxCreate
  .p2align 4
  .type cuGreenCtxCreate, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGreenCtxCreate
#endif
cuGreenCtxCreate:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2096(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2096(%rip)
2:
  pushq $262
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGreenCtxDestroy
  .p2align 4
  .type cuGreenCtxDestroy, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGreenCtxDestroy
#endif
cuGreenCtxDestroy:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2104(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2104(%rip)
2:
  pushq $263
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGreenCtxGetDevResource
  .p2align 4
  .type cuGreenCtxGetDevResource, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGreenCtxGetDevResource
#endif
cuGreenCtxGetDevResource:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2112(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2112(%rip)
2:
  pushq $264
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGreenCtxRecordEvent
  .p2align 4
  .type cuGreenCtxRecordEvent, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGreenCtxRecordEvent
#endif
cuGreenCtxRecordEvent:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2120(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2120(%rip)
2:
  pushq $265
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGreenCtxStreamCreate
  .p2align 4
  .type cuGreenCtxStreamCreate, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGreenCtxStreamCreate
#endif
cuGreenCtxStreamCreate:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2128(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2128(%rip)
2:
  pushq $266
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuGreenCtxWaitEvent
  .p2align 4
  .type cuGreenCtxWaitEvent, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuGreenCtxWaitEvent
#endif
cuGreenCtxWaitEvent:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2136(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2136(%rip)
2:
  pushq $267
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuImportExternalMemory
  .p2align 4
  .type cuImportExternalMemory, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuImportExternalMemory
#endif
cuImportExternalMemory:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2144(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2144(%rip)
2:
  pushq $268
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuImportExternalSemaphore
  .p2align 4
  .type cuImportExternalSemaphore, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuImportExternalSemaphore
#endif
cuImportExternalSemaphore:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2152(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2152(%rip)
2:
  pushq $269
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuInit
  .p2align 4
  .type cuInit, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuInit
#endif
cuInit:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2160(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2160(%rip)
2:
  pushq $270
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuIpcCloseMemHandle
  .p2align 4
  .type cuIpcCloseMemHandle, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuIpcCloseMemHandle
#endif
cuIpcCloseMemHandle:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2168(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2168(%rip)
2:
  pushq $271
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuIpcGetEventHandle
  .p2align 4
  .type cuIpcGetEventHandle, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuIpcGetEventHandle
#endif
cuIpcGetEventHandle:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2176(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2176(%rip)
2:
  pushq $272
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuIpcGetMemHandle
  .p2align 4
  .type cuIpcGetMemHandle, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuIpcGetMemHandle
#endif
cuIpcGetMemHandle:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2184(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2184(%rip)
2:
  pushq $273
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuIpcOpenEventHandle
  .p2align 4
  .type cuIpcOpenEventHandle, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuIpcOpenEventHandle
#endif
cuIpcOpenEventHandle:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2192(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2192(%rip)
2:
  pushq $274
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuIpcOpenMemHandle
  .p2align 4
  .type cuIpcOpenMemHandle, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuIpcOpenMemHandle
#endif
cuIpcOpenMemHandle:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2200(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2200(%rip)
2:
  pushq $275
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuIpcOpenMemHandle_v2
  .p2align 4
  .type cuIpcOpenMemHandle_v2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuIpcOpenMemHandle_v2
#endif
cuIpcOpenMemHandle_v2:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2208(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2208(%rip)
2:
  pushq $276
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuKernelGetAttribute
  .p2align 4
  .type cuKernelGetAttribute, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuKernelGetAttribute
#endif
cuKernelGetAttribute:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2216(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2216(%rip)
2:
  pushq $277
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuKernelGetFunction
  .p2align 4
  .type cuKernelGetFunction, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuKernelGetFunction
#endif
cuKernelGetFunction:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2224(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2224(%rip)
2:
  pushq $278
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuKernelGetLibrary
  .p2align 4
  .type cuKernelGetLibrary, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuKernelGetLibrary
#endif
cuKernelGetLibrary:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2232(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2232(%rip)
2:
  pushq $279
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuKernelGetName
  .p2align 4
  .type cuKernelGetName, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuKernelGetName
#endif
cuKernelGetName:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2240(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2240(%rip)
2:
  pushq $280
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuKernelGetParamInfo
  .p2align 4
  .type cuKernelGetParamInfo, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuKernelGetParamInfo
#endif
cuKernelGetParamInfo:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2248(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2248(%rip)
2:
  pushq $281
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuKernelSetAttribute
  .p2align 4
  .type cuKernelSetAttribute, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuKernelSetAttribute
#endif
cuKernelSetAttribute:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2256(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2256(%rip)
2:
  pushq $282
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuKernelSetCacheConfig
  .p2align 4
  .type cuKernelSetCacheConfig, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuKernelSetCacheConfig
#endif
cuKernelSetCacheConfig:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2264(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2264(%rip)
2:
  pushq $283
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuLaunch
  .p2align 4
  .type cuLaunch, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuLaunch
#endif
cuLaunch:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2272(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2272(%rip)
2:
  pushq $284
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuLaunchCooperativeKernel
  .p2align 4
  .type cuLaunchCooperativeKernel, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuLaunchCooperativeKernel
#endif
cuLaunchCooperativeKernel:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2280(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2280(%rip)
2:
  pushq $285
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuLaunchCooperativeKernelMultiDevice
  .p2align 4
  .type cuLaunchCooperativeKernelMultiDevice, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuLaunchCooperativeKernelMultiDevice
#endif
cuLaunchCooperativeKernelMultiDevice:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2288(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2288(%rip)
2:
  pushq $286
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuLaunchCooperativeKernel_ptsz
  .p2align 4
  .type cuLaunchCooperativeKernel_ptsz, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuLaunchCooperativeKernel_ptsz
#endif
cuLaunchCooperativeKernel_ptsz:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2296(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2296(%rip)
2:
  pushq $287
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuLaunchGrid
  .p2align 4
  .type cuLaunchGrid, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuLaunchGrid
#endif
cuLaunchGrid:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2304(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2304(%rip)
2:
  pushq $288
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuLaunchGridAsync
  .p2align 4
  .type cuLaunchGridAsync, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuLaunchGridAsync
#endif
cuLaunchGridAsync:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2312(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2312(%rip)
2:
  pushq $289
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuLaunchHostFunc
  .p2align 4
  .type cuLaunchHostFunc, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuLaunchHostFunc
#endif
cuLaunchHostFunc:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2320(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2320(%rip)
2:
  pushq $290
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuLaunchHostFunc_ptsz
  .p2align 4
  .type cuLaunchHostFunc_ptsz, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuLaunchHostFunc_ptsz
#endif
cuLaunchHostFunc_ptsz:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2328(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2328(%rip)
2:
  pushq $291
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuLaunchKernel
  .p2align 4
  .type cuLaunchKernel, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuLaunchKernel
#endif
cuLaunchKernel:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2336(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2336(%rip)
2:
  pushq $292
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuLaunchKernelEx
  .p2align 4
  .type cuLaunchKernelEx, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuLaunchKernelEx
#endif
cuLaunchKernelEx:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2344(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2344(%rip)
2:
  pushq $293
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuLaunchKernelEx_ptsz
  .p2align 4
  .type cuLaunchKernelEx_ptsz, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuLaunchKernelEx_ptsz
#endif
cuLaunchKernelEx_ptsz:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2352(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2352(%rip)
2:
  pushq $294
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuLaunchKernel_ptsz
  .p2align 4
  .type cuLaunchKernel_ptsz, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuLaunchKernel_ptsz
#endif
cuLaunchKernel_ptsz:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2360(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2360(%rip)
2:
  pushq $295
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuLibraryEnumerateKernels
  .p2align 4
  .type cuLibraryEnumerateKernels, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuLibraryEnumerateKernels
#endif
cuLibraryEnumerateKernels:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2368(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2368(%rip)
2:
  pushq $296
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuLibraryGetGlobal
  .p2align 4
  .type cuLibraryGetGlobal, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuLibraryGetGlobal
#endif
cuLibraryGetGlobal:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2376(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2376(%rip)
2:
  pushq $297
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuLibraryGetKernel
  .p2align 4
  .type cuLibraryGetKernel, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuLibraryGetKernel
#endif
cuLibraryGetKernel:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2384(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2384(%rip)
2:
  pushq $298
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuLibraryGetKernelCount
  .p2align 4
  .type cuLibraryGetKernelCount, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuLibraryGetKernelCount
#endif
cuLibraryGetKernelCount:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2392(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2392(%rip)
2:
  pushq $299
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuLibraryGetManaged
  .p2align 4
  .type cuLibraryGetManaged, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuLibraryGetManaged
#endif
cuLibraryGetManaged:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2400(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2400(%rip)
2:
  pushq $300
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuLibraryGetModule
  .p2align 4
  .type cuLibraryGetModule, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuLibraryGetModule
#endif
cuLibraryGetModule:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2408(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2408(%rip)
2:
  pushq $301
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuLibraryGetUnifiedFunction
  .p2align 4
  .type cuLibraryGetUnifiedFunction, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuLibraryGetUnifiedFunction
#endif
cuLibraryGetUnifiedFunction:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2416(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2416(%rip)
2:
  pushq $302
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuLibraryLoadData
  .p2align 4
  .type cuLibraryLoadData, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuLibraryLoadData
#endif
cuLibraryLoadData:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2424(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2424(%rip)
2:
  pushq $303
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuLibraryLoadFromFile
  .p2align 4
  .type cuLibraryLoadFromFile, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuLibraryLoadFromFile
#endif
cuLibraryLoadFromFile:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2432(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2432(%rip)
2:
  pushq $304
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuLibraryUnload
  .p2align 4
  .type cuLibraryUnload, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuLibraryUnload
#endif
cuLibraryUnload:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2440(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2440(%rip)
2:
  pushq $305
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuLinkAddData
  .p2align 4
  .type cuLinkAddData, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuLinkAddData
#endif
cuLinkAddData:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2448(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2448(%rip)
2:
  pushq $306
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuLinkAddData_v2
  .p2align 4
  .type cuLinkAddData_v2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuLinkAddData_v2
#endif
cuLinkAddData_v2:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2456(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2456(%rip)
2:
  pushq $307
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuLinkAddFile
  .p2align 4
  .type cuLinkAddFile, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuLinkAddFile
#endif
cuLinkAddFile:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2464(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2464(%rip)
2:
  pushq $308
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuLinkAddFile_v2
  .p2align 4
  .type cuLinkAddFile_v2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuLinkAddFile_v2
#endif
cuLinkAddFile_v2:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2472(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2472(%rip)
2:
  pushq $309
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuLinkComplete
  .p2align 4
  .type cuLinkComplete, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuLinkComplete
#endif
cuLinkComplete:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2480(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2480(%rip)
2:
  pushq $310
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuLinkCreate
  .p2align 4
  .type cuLinkCreate, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuLinkCreate
#endif
cuLinkCreate:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2488(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2488(%rip)
2:
  pushq $311
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuLinkCreate_v2
  .p2align 4
  .type cuLinkCreate_v2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuLinkCreate_v2
#endif
cuLinkCreate_v2:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2496(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2496(%rip)
2:
  pushq $312
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuLinkDestroy
  .p2align 4
  .type cuLinkDestroy, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuLinkDestroy
#endif
cuLinkDestroy:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2504(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2504(%rip)
2:
  pushq $313
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemAddressFree
  .p2align 4
  .type cuMemAddressFree, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemAddressFree
#endif
cuMemAddressFree:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2512(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2512(%rip)
2:
  pushq $314
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemAddressReserve
  .p2align 4
  .type cuMemAddressReserve, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemAddressReserve
#endif
cuMemAddressReserve:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2520(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2520(%rip)
2:
  pushq $315
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemAdvise
  .p2align 4
  .type cuMemAdvise, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemAdvise
#endif
cuMemAdvise:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2528(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2528(%rip)
2:
  pushq $316
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemAdvise_v2
  .p2align 4
  .type cuMemAdvise_v2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemAdvise_v2
#endif
cuMemAdvise_v2:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2536(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2536(%rip)
2:
  pushq $317
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemAlloc
  .p2align 4
  .type cuMemAlloc, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemAlloc
#endif
cuMemAlloc:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2544(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2544(%rip)
2:
  pushq $318
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemAllocAsync
  .p2align 4
  .type cuMemAllocAsync, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemAllocAsync
#endif
cuMemAllocAsync:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2552(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2552(%rip)
2:
  pushq $319
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemAllocAsync_ptsz
  .p2align 4
  .type cuMemAllocAsync_ptsz, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemAllocAsync_ptsz
#endif
cuMemAllocAsync_ptsz:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2560(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2560(%rip)
2:
  pushq $320
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemAllocFromPoolAsync
  .p2align 4
  .type cuMemAllocFromPoolAsync, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemAllocFromPoolAsync
#endif
cuMemAllocFromPoolAsync:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2568(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2568(%rip)
2:
  pushq $321
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemAllocFromPoolAsync_ptsz
  .p2align 4
  .type cuMemAllocFromPoolAsync_ptsz, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemAllocFromPoolAsync_ptsz
#endif
cuMemAllocFromPoolAsync_ptsz:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2576(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2576(%rip)
2:
  pushq $322
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemAllocHost
  .p2align 4
  .type cuMemAllocHost, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemAllocHost
#endif
cuMemAllocHost:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2584(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2584(%rip)
2:
  pushq $323
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemAllocHost_v2
  .p2align 4
  .type cuMemAllocHost_v2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemAllocHost_v2
#endif
cuMemAllocHost_v2:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2592(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2592(%rip)
2:
  pushq $324
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemAllocManaged
  .p2align 4
  .type cuMemAllocManaged, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemAllocManaged
#endif
cuMemAllocManaged:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2600(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2600(%rip)
2:
  pushq $325
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemAllocPitch
  .p2align 4
  .type cuMemAllocPitch, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemAllocPitch
#endif
cuMemAllocPitch:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2608(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2608(%rip)
2:
  pushq $326
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemAllocPitch_v2
  .p2align 4
  .type cuMemAllocPitch_v2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemAllocPitch_v2
#endif
cuMemAllocPitch_v2:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2616(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2616(%rip)
2:
  pushq $327
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemAlloc_v2
  .p2align 4
  .type cuMemAlloc_v2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemAlloc_v2
#endif
cuMemAlloc_v2:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2624(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2624(%rip)
2:
  pushq $328
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemBatchDecompressAsync
  .p2align 4
  .type cuMemBatchDecompressAsync, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemBatchDecompressAsync
#endif
cuMemBatchDecompressAsync:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2632(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2632(%rip)
2:
  pushq $329
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemBatchDecompressAsync_ptsz
  .p2align 4
  .type cuMemBatchDecompressAsync_ptsz, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemBatchDecompressAsync_ptsz
#endif
cuMemBatchDecompressAsync_ptsz:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2640(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2640(%rip)
2:
  pushq $330
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemCreate
  .p2align 4
  .type cuMemCreate, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemCreate
#endif
cuMemCreate:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2648(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2648(%rip)
2:
  pushq $331
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemExportToShareableHandle
  .p2align 4
  .type cuMemExportToShareableHandle, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemExportToShareableHandle
#endif
cuMemExportToShareableHandle:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2656(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2656(%rip)
2:
  pushq $332
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemFree
  .p2align 4
  .type cuMemFree, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemFree
#endif
cuMemFree:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2664(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2664(%rip)
2:
  pushq $333
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemFreeAsync
  .p2align 4
  .type cuMemFreeAsync, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemFreeAsync
#endif
cuMemFreeAsync:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2672(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2672(%rip)
2:
  pushq $334
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemFreeAsync_ptsz
  .p2align 4
  .type cuMemFreeAsync_ptsz, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemFreeAsync_ptsz
#endif
cuMemFreeAsync_ptsz:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2680(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2680(%rip)
2:
  pushq $335
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemFreeHost
  .p2align 4
  .type cuMemFreeHost, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemFreeHost
#endif
cuMemFreeHost:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2688(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2688(%rip)
2:
  pushq $336
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemFree_v2
  .p2align 4
  .type cuMemFree_v2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemFree_v2
#endif
cuMemFree_v2:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2696(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2696(%rip)
2:
  pushq $337
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemGetAccess
  .p2align 4
  .type cuMemGetAccess, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemGetAccess
#endif
cuMemGetAccess:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2704(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2704(%rip)
2:
  pushq $338
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemGetAddressRange
  .p2align 4
  .type cuMemGetAddressRange, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemGetAddressRange
#endif
cuMemGetAddressRange:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2712(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2712(%rip)
2:
  pushq $339
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemGetAddressRange_v2
  .p2align 4
  .type cuMemGetAddressRange_v2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemGetAddressRange_v2
#endif
cuMemGetAddressRange_v2:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2720(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2720(%rip)
2:
  pushq $340
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemGetAllocationGranularity
  .p2align 4
  .type cuMemGetAllocationGranularity, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemGetAllocationGranularity
#endif
cuMemGetAllocationGranularity:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2728(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2728(%rip)
2:
  pushq $341
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemGetAllocationPropertiesFromHandle
  .p2align 4
  .type cuMemGetAllocationPropertiesFromHandle, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemGetAllocationPropertiesFromHandle
#endif
cuMemGetAllocationPropertiesFromHandle:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2736(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2736(%rip)
2:
  pushq $342
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemGetAttribute
  .p2align 4
  .type cuMemGetAttribute, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemGetAttribute
#endif
cuMemGetAttribute:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2744(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2744(%rip)
2:
  pushq $343
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemGetAttribute_v2
  .p2align 4
  .type cuMemGetAttribute_v2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemGetAttribute_v2
#endif
cuMemGetAttribute_v2:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2752(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2752(%rip)
2:
  pushq $344
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemGetHandleForAddressRange
  .p2align 4
  .type cuMemGetHandleForAddressRange, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemGetHandleForAddressRange
#endif
cuMemGetHandleForAddressRange:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2760(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2760(%rip)
2:
  pushq $345
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemGetInfo
  .p2align 4
  .type cuMemGetInfo, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemGetInfo
#endif
cuMemGetInfo:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2768(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2768(%rip)
2:
  pushq $346
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemGetInfo_v2
  .p2align 4
  .type cuMemGetInfo_v2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemGetInfo_v2
#endif
cuMemGetInfo_v2:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2776(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2776(%rip)
2:
  pushq $347
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemHostAlloc
  .p2align 4
  .type cuMemHostAlloc, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemHostAlloc
#endif
cuMemHostAlloc:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2784(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2784(%rip)
2:
  pushq $348
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemHostGetDevicePointer
  .p2align 4
  .type cuMemHostGetDevicePointer, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemHostGetDevicePointer
#endif
cuMemHostGetDevicePointer:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2792(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2792(%rip)
2:
  pushq $349
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemHostGetDevicePointer_v2
  .p2align 4
  .type cuMemHostGetDevicePointer_v2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemHostGetDevicePointer_v2
#endif
cuMemHostGetDevicePointer_v2:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2800(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2800(%rip)
2:
  pushq $350
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemHostGetFlags
  .p2align 4
  .type cuMemHostGetFlags, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemHostGetFlags
#endif
cuMemHostGetFlags:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2808(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2808(%rip)
2:
  pushq $351
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemHostRegister
  .p2align 4
  .type cuMemHostRegister, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemHostRegister
#endif
cuMemHostRegister:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2816(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2816(%rip)
2:
  pushq $352
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemHostRegister_v2
  .p2align 4
  .type cuMemHostRegister_v2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemHostRegister_v2
#endif
cuMemHostRegister_v2:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2824(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2824(%rip)
2:
  pushq $353
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemHostUnregister
  .p2align 4
  .type cuMemHostUnregister, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemHostUnregister
#endif
cuMemHostUnregister:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2832(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2832(%rip)
2:
  pushq $354
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemImportFromShareableHandle
  .p2align 4
  .type cuMemImportFromShareableHandle, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemImportFromShareableHandle
#endif
cuMemImportFromShareableHandle:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2840(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2840(%rip)
2:
  pushq $355
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemMap
  .p2align 4
  .type cuMemMap, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemMap
#endif
cuMemMap:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2848(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2848(%rip)
2:
  pushq $356
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemMapArrayAsync
  .p2align 4
  .type cuMemMapArrayAsync, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemMapArrayAsync
#endif
cuMemMapArrayAsync:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2856(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2856(%rip)
2:
  pushq $357
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemMapArrayAsync_ptsz
  .p2align 4
  .type cuMemMapArrayAsync_ptsz, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemMapArrayAsync_ptsz
#endif
cuMemMapArrayAsync_ptsz:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2864(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2864(%rip)
2:
  pushq $358
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemPoolCreate
  .p2align 4
  .type cuMemPoolCreate, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemPoolCreate
#endif
cuMemPoolCreate:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2872(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2872(%rip)
2:
  pushq $359
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemPoolDestroy
  .p2align 4
  .type cuMemPoolDestroy, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemPoolDestroy
#endif
cuMemPoolDestroy:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2880(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2880(%rip)
2:
  pushq $360
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemPoolExportPointer
  .p2align 4
  .type cuMemPoolExportPointer, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemPoolExportPointer
#endif
cuMemPoolExportPointer:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2888(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2888(%rip)
2:
  pushq $361
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemPoolExportToShareableHandle
  .p2align 4
  .type cuMemPoolExportToShareableHandle, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemPoolExportToShareableHandle
#endif
cuMemPoolExportToShareableHandle:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2896(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2896(%rip)
2:
  pushq $362
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemPoolGetAccess
  .p2align 4
  .type cuMemPoolGetAccess, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemPoolGetAccess
#endif
cuMemPoolGetAccess:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2904(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2904(%rip)
2:
  pushq $363
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemPoolGetAttribute
  .p2align 4
  .type cuMemPoolGetAttribute, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemPoolGetAttribute
#endif
cuMemPoolGetAttribute:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2912(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2912(%rip)
2:
  pushq $364
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemPoolImportFromShareableHandle
  .p2align 4
  .type cuMemPoolImportFromShareableHandle, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemPoolImportFromShareableHandle
#endif
cuMemPoolImportFromShareableHandle:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2920(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2920(%rip)
2:
  pushq $365
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemPoolImportPointer
  .p2align 4
  .type cuMemPoolImportPointer, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemPoolImportPointer
#endif
cuMemPoolImportPointer:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2928(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2928(%rip)
2:
  pushq $366
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemPoolSetAccess
  .p2align 4
  .type cuMemPoolSetAccess, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemPoolSetAccess
#endif
cuMemPoolSetAccess:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2936(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2936(%rip)
2:
  pushq $367
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemPoolSetAttribute
  .p2align 4
  .type cuMemPoolSetAttribute, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemPoolSetAttribute
#endif
cuMemPoolSetAttribute:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2944(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2944(%rip)
2:
  pushq $368
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemPoolTrimTo
  .p2align 4
  .type cuMemPoolTrimTo, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemPoolTrimTo
#endif
cuMemPoolTrimTo:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2952(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2952(%rip)
2:
  pushq $369
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemPrefetchAsync
  .p2align 4
  .type cuMemPrefetchAsync, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemPrefetchAsync
#endif
cuMemPrefetchAsync:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2960(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2960(%rip)
2:
  pushq $370
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemPrefetchAsync_ptsz
  .p2align 4
  .type cuMemPrefetchAsync_ptsz, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemPrefetchAsync_ptsz
#endif
cuMemPrefetchAsync_ptsz:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2968(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2968(%rip)
2:
  pushq $371
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemPrefetchAsync_v2
  .p2align 4
  .type cuMemPrefetchAsync_v2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemPrefetchAsync_v2
#endif
cuMemPrefetchAsync_v2:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2976(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2976(%rip)
2:
  pushq $372
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemPrefetchAsync_v2_ptsz
  .p2align 4
  .type cuMemPrefetchAsync_v2_ptsz, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemPrefetchAsync_v2_ptsz
#endif
cuMemPrefetchAsync_v2_ptsz:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2984(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2984(%rip)
2:
  pushq $373
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemRangeGetAttribute
  .p2align 4
  .type cuMemRangeGetAttribute, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemRangeGetAttribute
#endif
cuMemRangeGetAttribute:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+2992(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+2992(%rip)
2:
  pushq $374
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemRangeGetAttributes
  .p2align 4
  .type cuMemRangeGetAttributes, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemRangeGetAttributes
#endif
cuMemRangeGetAttributes:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3000(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3000(%rip)
2:
  pushq $375
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemRelease
  .p2align 4
  .type cuMemRelease, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemRelease
#endif
cuMemRelease:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3008(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3008(%rip)
2:
  pushq $376
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemRetainAllocationHandle
  .p2align 4
  .type cuMemRetainAllocationHandle, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemRetainAllocationHandle
#endif
cuMemRetainAllocationHandle:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3016(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3016(%rip)
2:
  pushq $377
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemSetAccess
  .p2align 4
  .type cuMemSetAccess, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemSetAccess
#endif
cuMemSetAccess:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3024(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3024(%rip)
2:
  pushq $378
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemUnmap
  .p2align 4
  .type cuMemUnmap, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemUnmap
#endif
cuMemUnmap:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3032(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3032(%rip)
2:
  pushq $379
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemcpy
  .p2align 4
  .type cuMemcpy, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemcpy
#endif
cuMemcpy:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3040(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3040(%rip)
2:
  pushq $380
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemcpy2D
  .p2align 4
  .type cuMemcpy2D, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemcpy2D
#endif
cuMemcpy2D:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3048(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3048(%rip)
2:
  pushq $381
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemcpy2DAsync
  .p2align 4
  .type cuMemcpy2DAsync, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemcpy2DAsync
#endif
cuMemcpy2DAsync:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3056(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3056(%rip)
2:
  pushq $382
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemcpy2DAsync_v2
  .p2align 4
  .type cuMemcpy2DAsync_v2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemcpy2DAsync_v2
#endif
cuMemcpy2DAsync_v2:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3064(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3064(%rip)
2:
  pushq $383
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemcpy2DAsync_v2_ptsz
  .p2align 4
  .type cuMemcpy2DAsync_v2_ptsz, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemcpy2DAsync_v2_ptsz
#endif
cuMemcpy2DAsync_v2_ptsz:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3072(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3072(%rip)
2:
  pushq $384
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemcpy2DUnaligned
  .p2align 4
  .type cuMemcpy2DUnaligned, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemcpy2DUnaligned
#endif
cuMemcpy2DUnaligned:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3080(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3080(%rip)
2:
  pushq $385
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemcpy2DUnaligned_v2
  .p2align 4
  .type cuMemcpy2DUnaligned_v2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemcpy2DUnaligned_v2
#endif
cuMemcpy2DUnaligned_v2:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3088(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3088(%rip)
2:
  pushq $386
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemcpy2DUnaligned_v2_ptds
  .p2align 4
  .type cuMemcpy2DUnaligned_v2_ptds, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemcpy2DUnaligned_v2_ptds
#endif
cuMemcpy2DUnaligned_v2_ptds:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3096(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3096(%rip)
2:
  pushq $387
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemcpy2D_v2
  .p2align 4
  .type cuMemcpy2D_v2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemcpy2D_v2
#endif
cuMemcpy2D_v2:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3104(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3104(%rip)
2:
  pushq $388
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemcpy2D_v2_ptds
  .p2align 4
  .type cuMemcpy2D_v2_ptds, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemcpy2D_v2_ptds
#endif
cuMemcpy2D_v2_ptds:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3112(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3112(%rip)
2:
  pushq $389
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemcpy3D
  .p2align 4
  .type cuMemcpy3D, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemcpy3D
#endif
cuMemcpy3D:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3120(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3120(%rip)
2:
  pushq $390
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemcpy3DAsync
  .p2align 4
  .type cuMemcpy3DAsync, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemcpy3DAsync
#endif
cuMemcpy3DAsync:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3128(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3128(%rip)
2:
  pushq $391
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemcpy3DAsync_v2
  .p2align 4
  .type cuMemcpy3DAsync_v2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemcpy3DAsync_v2
#endif
cuMemcpy3DAsync_v2:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3136(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3136(%rip)
2:
  pushq $392
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemcpy3DAsync_v2_ptsz
  .p2align 4
  .type cuMemcpy3DAsync_v2_ptsz, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemcpy3DAsync_v2_ptsz
#endif
cuMemcpy3DAsync_v2_ptsz:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3144(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3144(%rip)
2:
  pushq $393
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemcpy3DBatchAsync
  .p2align 4
  .type cuMemcpy3DBatchAsync, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemcpy3DBatchAsync
#endif
cuMemcpy3DBatchAsync:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3152(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3152(%rip)
2:
  pushq $394
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemcpy3DBatchAsync_ptsz
  .p2align 4
  .type cuMemcpy3DBatchAsync_ptsz, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemcpy3DBatchAsync_ptsz
#endif
cuMemcpy3DBatchAsync_ptsz:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3160(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3160(%rip)
2:
  pushq $395
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemcpy3DPeer
  .p2align 4
  .type cuMemcpy3DPeer, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemcpy3DPeer
#endif
cuMemcpy3DPeer:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3168(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3168(%rip)
2:
  pushq $396
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemcpy3DPeerAsync
  .p2align 4
  .type cuMemcpy3DPeerAsync, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemcpy3DPeerAsync
#endif
cuMemcpy3DPeerAsync:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3176(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3176(%rip)
2:
  pushq $397
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemcpy3DPeerAsync_ptsz
  .p2align 4
  .type cuMemcpy3DPeerAsync_ptsz, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemcpy3DPeerAsync_ptsz
#endif
cuMemcpy3DPeerAsync_ptsz:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3184(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3184(%rip)
2:
  pushq $398
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemcpy3DPeer_ptds
  .p2align 4
  .type cuMemcpy3DPeer_ptds, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemcpy3DPeer_ptds
#endif
cuMemcpy3DPeer_ptds:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3192(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3192(%rip)
2:
  pushq $399
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemcpy3D_v2
  .p2align 4
  .type cuMemcpy3D_v2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemcpy3D_v2
#endif
cuMemcpy3D_v2:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3200(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3200(%rip)
2:
  pushq $400
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemcpy3D_v2_ptds
  .p2align 4
  .type cuMemcpy3D_v2_ptds, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemcpy3D_v2_ptds
#endif
cuMemcpy3D_v2_ptds:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3208(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3208(%rip)
2:
  pushq $401
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemcpyAsync
  .p2align 4
  .type cuMemcpyAsync, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemcpyAsync
#endif
cuMemcpyAsync:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3216(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3216(%rip)
2:
  pushq $402
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemcpyAsync_ptsz
  .p2align 4
  .type cuMemcpyAsync_ptsz, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemcpyAsync_ptsz
#endif
cuMemcpyAsync_ptsz:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3224(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3224(%rip)
2:
  pushq $403
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemcpyAtoA
  .p2align 4
  .type cuMemcpyAtoA, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemcpyAtoA
#endif
cuMemcpyAtoA:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3232(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3232(%rip)
2:
  pushq $404
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemcpyAtoA_v2
  .p2align 4
  .type cuMemcpyAtoA_v2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemcpyAtoA_v2
#endif
cuMemcpyAtoA_v2:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3240(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3240(%rip)
2:
  pushq $405
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemcpyAtoA_v2_ptds
  .p2align 4
  .type cuMemcpyAtoA_v2_ptds, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemcpyAtoA_v2_ptds
#endif
cuMemcpyAtoA_v2_ptds:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3248(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3248(%rip)
2:
  pushq $406
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemcpyAtoD
  .p2align 4
  .type cuMemcpyAtoD, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemcpyAtoD
#endif
cuMemcpyAtoD:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3256(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3256(%rip)
2:
  pushq $407
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemcpyAtoD_v2
  .p2align 4
  .type cuMemcpyAtoD_v2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemcpyAtoD_v2
#endif
cuMemcpyAtoD_v2:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3264(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3264(%rip)
2:
  pushq $408
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemcpyAtoD_v2_ptds
  .p2align 4
  .type cuMemcpyAtoD_v2_ptds, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemcpyAtoD_v2_ptds
#endif
cuMemcpyAtoD_v2_ptds:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3272(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3272(%rip)
2:
  pushq $409
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemcpyAtoH
  .p2align 4
  .type cuMemcpyAtoH, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemcpyAtoH
#endif
cuMemcpyAtoH:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3280(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3280(%rip)
2:
  pushq $410
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemcpyAtoHAsync
  .p2align 4
  .type cuMemcpyAtoHAsync, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemcpyAtoHAsync
#endif
cuMemcpyAtoHAsync:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3288(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3288(%rip)
2:
  pushq $411
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemcpyAtoHAsync_v2
  .p2align 4
  .type cuMemcpyAtoHAsync_v2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemcpyAtoHAsync_v2
#endif
cuMemcpyAtoHAsync_v2:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3296(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3296(%rip)
2:
  pushq $412
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemcpyAtoHAsync_v2_ptsz
  .p2align 4
  .type cuMemcpyAtoHAsync_v2_ptsz, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemcpyAtoHAsync_v2_ptsz
#endif
cuMemcpyAtoHAsync_v2_ptsz:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3304(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3304(%rip)
2:
  pushq $413
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemcpyAtoH_v2
  .p2align 4
  .type cuMemcpyAtoH_v2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemcpyAtoH_v2
#endif
cuMemcpyAtoH_v2:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3312(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3312(%rip)
2:
  pushq $414
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemcpyAtoH_v2_ptds
  .p2align 4
  .type cuMemcpyAtoH_v2_ptds, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemcpyAtoH_v2_ptds
#endif
cuMemcpyAtoH_v2_ptds:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3320(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3320(%rip)
2:
  pushq $415
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemcpyBatchAsync
  .p2align 4
  .type cuMemcpyBatchAsync, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemcpyBatchAsync
#endif
cuMemcpyBatchAsync:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3328(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3328(%rip)
2:
  pushq $416
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemcpyBatchAsync_ptsz
  .p2align 4
  .type cuMemcpyBatchAsync_ptsz, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemcpyBatchAsync_ptsz
#endif
cuMemcpyBatchAsync_ptsz:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3336(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3336(%rip)
2:
  pushq $417
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemcpyDtoA
  .p2align 4
  .type cuMemcpyDtoA, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemcpyDtoA
#endif
cuMemcpyDtoA:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3344(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3344(%rip)
2:
  pushq $418
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemcpyDtoA_v2
  .p2align 4
  .type cuMemcpyDtoA_v2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemcpyDtoA_v2
#endif
cuMemcpyDtoA_v2:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3352(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3352(%rip)
2:
  pushq $419
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemcpyDtoA_v2_ptds
  .p2align 4
  .type cuMemcpyDtoA_v2_ptds, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemcpyDtoA_v2_ptds
#endif
cuMemcpyDtoA_v2_ptds:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3360(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3360(%rip)
2:
  pushq $420
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemcpyDtoD
  .p2align 4
  .type cuMemcpyDtoD, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemcpyDtoD
#endif
cuMemcpyDtoD:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3368(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3368(%rip)
2:
  pushq $421
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemcpyDtoDAsync
  .p2align 4
  .type cuMemcpyDtoDAsync, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemcpyDtoDAsync
#endif
cuMemcpyDtoDAsync:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3376(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3376(%rip)
2:
  pushq $422
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemcpyDtoDAsync_v2
  .p2align 4
  .type cuMemcpyDtoDAsync_v2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemcpyDtoDAsync_v2
#endif
cuMemcpyDtoDAsync_v2:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3384(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3384(%rip)
2:
  pushq $423
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemcpyDtoDAsync_v2_ptsz
  .p2align 4
  .type cuMemcpyDtoDAsync_v2_ptsz, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemcpyDtoDAsync_v2_ptsz
#endif
cuMemcpyDtoDAsync_v2_ptsz:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3392(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3392(%rip)
2:
  pushq $424
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemcpyDtoD_v2
  .p2align 4
  .type cuMemcpyDtoD_v2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemcpyDtoD_v2
#endif
cuMemcpyDtoD_v2:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3400(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3400(%rip)
2:
  pushq $425
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemcpyDtoD_v2_ptds
  .p2align 4
  .type cuMemcpyDtoD_v2_ptds, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemcpyDtoD_v2_ptds
#endif
cuMemcpyDtoD_v2_ptds:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3408(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3408(%rip)
2:
  pushq $426
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemcpyDtoH
  .p2align 4
  .type cuMemcpyDtoH, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemcpyDtoH
#endif
cuMemcpyDtoH:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3416(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3416(%rip)
2:
  pushq $427
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemcpyDtoHAsync
  .p2align 4
  .type cuMemcpyDtoHAsync, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemcpyDtoHAsync
#endif
cuMemcpyDtoHAsync:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3424(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3424(%rip)
2:
  pushq $428
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemcpyDtoHAsync_v2
  .p2align 4
  .type cuMemcpyDtoHAsync_v2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemcpyDtoHAsync_v2
#endif
cuMemcpyDtoHAsync_v2:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3432(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3432(%rip)
2:
  pushq $429
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemcpyDtoHAsync_v2_ptsz
  .p2align 4
  .type cuMemcpyDtoHAsync_v2_ptsz, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemcpyDtoHAsync_v2_ptsz
#endif
cuMemcpyDtoHAsync_v2_ptsz:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3440(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3440(%rip)
2:
  pushq $430
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemcpyDtoH_v2
  .p2align 4
  .type cuMemcpyDtoH_v2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemcpyDtoH_v2
#endif
cuMemcpyDtoH_v2:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3448(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3448(%rip)
2:
  pushq $431
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemcpyDtoH_v2_ptds
  .p2align 4
  .type cuMemcpyDtoH_v2_ptds, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemcpyDtoH_v2_ptds
#endif
cuMemcpyDtoH_v2_ptds:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3456(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3456(%rip)
2:
  pushq $432
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemcpyHtoA
  .p2align 4
  .type cuMemcpyHtoA, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemcpyHtoA
#endif
cuMemcpyHtoA:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3464(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3464(%rip)
2:
  pushq $433
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemcpyHtoAAsync
  .p2align 4
  .type cuMemcpyHtoAAsync, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemcpyHtoAAsync
#endif
cuMemcpyHtoAAsync:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3472(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3472(%rip)
2:
  pushq $434
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemcpyHtoAAsync_v2
  .p2align 4
  .type cuMemcpyHtoAAsync_v2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemcpyHtoAAsync_v2
#endif
cuMemcpyHtoAAsync_v2:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3480(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3480(%rip)
2:
  pushq $435
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemcpyHtoAAsync_v2_ptsz
  .p2align 4
  .type cuMemcpyHtoAAsync_v2_ptsz, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemcpyHtoAAsync_v2_ptsz
#endif
cuMemcpyHtoAAsync_v2_ptsz:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3488(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3488(%rip)
2:
  pushq $436
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemcpyHtoA_v2
  .p2align 4
  .type cuMemcpyHtoA_v2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemcpyHtoA_v2
#endif
cuMemcpyHtoA_v2:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3496(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3496(%rip)
2:
  pushq $437
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemcpyHtoA_v2_ptds
  .p2align 4
  .type cuMemcpyHtoA_v2_ptds, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemcpyHtoA_v2_ptds
#endif
cuMemcpyHtoA_v2_ptds:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3504(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3504(%rip)
2:
  pushq $438
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemcpyHtoD
  .p2align 4
  .type cuMemcpyHtoD, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemcpyHtoD
#endif
cuMemcpyHtoD:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3512(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3512(%rip)
2:
  pushq $439
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemcpyHtoDAsync
  .p2align 4
  .type cuMemcpyHtoDAsync, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemcpyHtoDAsync
#endif
cuMemcpyHtoDAsync:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3520(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3520(%rip)
2:
  pushq $440
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemcpyHtoDAsync_v2
  .p2align 4
  .type cuMemcpyHtoDAsync_v2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemcpyHtoDAsync_v2
#endif
cuMemcpyHtoDAsync_v2:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3528(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3528(%rip)
2:
  pushq $441
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemcpyHtoDAsync_v2_ptsz
  .p2align 4
  .type cuMemcpyHtoDAsync_v2_ptsz, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemcpyHtoDAsync_v2_ptsz
#endif
cuMemcpyHtoDAsync_v2_ptsz:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3536(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3536(%rip)
2:
  pushq $442
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemcpyHtoD_v2
  .p2align 4
  .type cuMemcpyHtoD_v2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemcpyHtoD_v2
#endif
cuMemcpyHtoD_v2:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3544(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3544(%rip)
2:
  pushq $443
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemcpyHtoD_v2_ptds
  .p2align 4
  .type cuMemcpyHtoD_v2_ptds, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemcpyHtoD_v2_ptds
#endif
cuMemcpyHtoD_v2_ptds:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3552(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3552(%rip)
2:
  pushq $444
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemcpyPeer
  .p2align 4
  .type cuMemcpyPeer, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemcpyPeer
#endif
cuMemcpyPeer:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3560(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3560(%rip)
2:
  pushq $445
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemcpyPeerAsync
  .p2align 4
  .type cuMemcpyPeerAsync, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemcpyPeerAsync
#endif
cuMemcpyPeerAsync:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3568(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3568(%rip)
2:
  pushq $446
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemcpyPeerAsync_ptsz
  .p2align 4
  .type cuMemcpyPeerAsync_ptsz, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemcpyPeerAsync_ptsz
#endif
cuMemcpyPeerAsync_ptsz:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3576(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3576(%rip)
2:
  pushq $447
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemcpyPeer_ptds
  .p2align 4
  .type cuMemcpyPeer_ptds, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemcpyPeer_ptds
#endif
cuMemcpyPeer_ptds:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3584(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3584(%rip)
2:
  pushq $448
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemcpy_ptds
  .p2align 4
  .type cuMemcpy_ptds, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemcpy_ptds
#endif
cuMemcpy_ptds:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3592(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3592(%rip)
2:
  pushq $449
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemsetD16
  .p2align 4
  .type cuMemsetD16, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemsetD16
#endif
cuMemsetD16:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3600(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3600(%rip)
2:
  pushq $450
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemsetD16Async
  .p2align 4
  .type cuMemsetD16Async, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemsetD16Async
#endif
cuMemsetD16Async:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3608(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3608(%rip)
2:
  pushq $451
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemsetD16Async_ptsz
  .p2align 4
  .type cuMemsetD16Async_ptsz, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemsetD16Async_ptsz
#endif
cuMemsetD16Async_ptsz:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3616(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3616(%rip)
2:
  pushq $452
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemsetD16_v2
  .p2align 4
  .type cuMemsetD16_v2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemsetD16_v2
#endif
cuMemsetD16_v2:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3624(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3624(%rip)
2:
  pushq $453
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemsetD16_v2_ptds
  .p2align 4
  .type cuMemsetD16_v2_ptds, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemsetD16_v2_ptds
#endif
cuMemsetD16_v2_ptds:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3632(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3632(%rip)
2:
  pushq $454
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemsetD2D16
  .p2align 4
  .type cuMemsetD2D16, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemsetD2D16
#endif
cuMemsetD2D16:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3640(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3640(%rip)
2:
  pushq $455
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemsetD2D16Async
  .p2align 4
  .type cuMemsetD2D16Async, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemsetD2D16Async
#endif
cuMemsetD2D16Async:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3648(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3648(%rip)
2:
  pushq $456
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemsetD2D16Async_ptsz
  .p2align 4
  .type cuMemsetD2D16Async_ptsz, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemsetD2D16Async_ptsz
#endif
cuMemsetD2D16Async_ptsz:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3656(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3656(%rip)
2:
  pushq $457
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemsetD2D16_v2
  .p2align 4
  .type cuMemsetD2D16_v2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemsetD2D16_v2
#endif
cuMemsetD2D16_v2:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3664(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3664(%rip)
2:
  pushq $458
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemsetD2D16_v2_ptds
  .p2align 4
  .type cuMemsetD2D16_v2_ptds, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemsetD2D16_v2_ptds
#endif
cuMemsetD2D16_v2_ptds:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3672(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3672(%rip)
2:
  pushq $459
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemsetD2D32
  .p2align 4
  .type cuMemsetD2D32, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemsetD2D32
#endif
cuMemsetD2D32:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3680(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3680(%rip)
2:
  pushq $460
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemsetD2D32Async
  .p2align 4
  .type cuMemsetD2D32Async, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemsetD2D32Async
#endif
cuMemsetD2D32Async:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3688(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3688(%rip)
2:
  pushq $461
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemsetD2D32Async_ptsz
  .p2align 4
  .type cuMemsetD2D32Async_ptsz, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemsetD2D32Async_ptsz
#endif
cuMemsetD2D32Async_ptsz:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3696(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3696(%rip)
2:
  pushq $462
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemsetD2D32_v2
  .p2align 4
  .type cuMemsetD2D32_v2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemsetD2D32_v2
#endif
cuMemsetD2D32_v2:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3704(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3704(%rip)
2:
  pushq $463
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemsetD2D32_v2_ptds
  .p2align 4
  .type cuMemsetD2D32_v2_ptds, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemsetD2D32_v2_ptds
#endif
cuMemsetD2D32_v2_ptds:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3712(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3712(%rip)
2:
  pushq $464
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemsetD2D8
  .p2align 4
  .type cuMemsetD2D8, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemsetD2D8
#endif
cuMemsetD2D8:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3720(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3720(%rip)
2:
  pushq $465
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemsetD2D8Async
  .p2align 4
  .type cuMemsetD2D8Async, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemsetD2D8Async
#endif
cuMemsetD2D8Async:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3728(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3728(%rip)
2:
  pushq $466
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemsetD2D8Async_ptsz
  .p2align 4
  .type cuMemsetD2D8Async_ptsz, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemsetD2D8Async_ptsz
#endif
cuMemsetD2D8Async_ptsz:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3736(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3736(%rip)
2:
  pushq $467
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemsetD2D8_v2
  .p2align 4
  .type cuMemsetD2D8_v2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemsetD2D8_v2
#endif
cuMemsetD2D8_v2:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3744(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3744(%rip)
2:
  pushq $468
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemsetD2D8_v2_ptds
  .p2align 4
  .type cuMemsetD2D8_v2_ptds, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemsetD2D8_v2_ptds
#endif
cuMemsetD2D8_v2_ptds:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3752(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3752(%rip)
2:
  pushq $469
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemsetD32
  .p2align 4
  .type cuMemsetD32, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemsetD32
#endif
cuMemsetD32:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3760(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3760(%rip)
2:
  pushq $470
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemsetD32Async
  .p2align 4
  .type cuMemsetD32Async, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemsetD32Async
#endif
cuMemsetD32Async:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3768(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3768(%rip)
2:
  pushq $471
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemsetD32Async_ptsz
  .p2align 4
  .type cuMemsetD32Async_ptsz, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemsetD32Async_ptsz
#endif
cuMemsetD32Async_ptsz:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3776(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3776(%rip)
2:
  pushq $472
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemsetD32_v2
  .p2align 4
  .type cuMemsetD32_v2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemsetD32_v2
#endif
cuMemsetD32_v2:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3784(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3784(%rip)
2:
  pushq $473
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemsetD32_v2_ptds
  .p2align 4
  .type cuMemsetD32_v2_ptds, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemsetD32_v2_ptds
#endif
cuMemsetD32_v2_ptds:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3792(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3792(%rip)
2:
  pushq $474
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemsetD8
  .p2align 4
  .type cuMemsetD8, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemsetD8
#endif
cuMemsetD8:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3800(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3800(%rip)
2:
  pushq $475
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemsetD8Async
  .p2align 4
  .type cuMemsetD8Async, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemsetD8Async
#endif
cuMemsetD8Async:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3808(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3808(%rip)
2:
  pushq $476
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemsetD8Async_ptsz
  .p2align 4
  .type cuMemsetD8Async_ptsz, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemsetD8Async_ptsz
#endif
cuMemsetD8Async_ptsz:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3816(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3816(%rip)
2:
  pushq $477
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemsetD8_v2
  .p2align 4
  .type cuMemsetD8_v2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemsetD8_v2
#endif
cuMemsetD8_v2:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3824(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3824(%rip)
2:
  pushq $478
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMemsetD8_v2_ptds
  .p2align 4
  .type cuMemsetD8_v2_ptds, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMemsetD8_v2_ptds
#endif
cuMemsetD8_v2_ptds:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3832(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3832(%rip)
2:
  pushq $479
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMipmappedArrayCreate
  .p2align 4
  .type cuMipmappedArrayCreate, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMipmappedArrayCreate
#endif
cuMipmappedArrayCreate:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3840(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3840(%rip)
2:
  pushq $480
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMipmappedArrayDestroy
  .p2align 4
  .type cuMipmappedArrayDestroy, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMipmappedArrayDestroy
#endif
cuMipmappedArrayDestroy:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3848(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3848(%rip)
2:
  pushq $481
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMipmappedArrayGetLevel
  .p2align 4
  .type cuMipmappedArrayGetLevel, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMipmappedArrayGetLevel
#endif
cuMipmappedArrayGetLevel:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3856(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3856(%rip)
2:
  pushq $482
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMipmappedArrayGetMemoryRequirements
  .p2align 4
  .type cuMipmappedArrayGetMemoryRequirements, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMipmappedArrayGetMemoryRequirements
#endif
cuMipmappedArrayGetMemoryRequirements:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3864(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3864(%rip)
2:
  pushq $483
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMipmappedArrayGetSparseProperties
  .p2align 4
  .type cuMipmappedArrayGetSparseProperties, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMipmappedArrayGetSparseProperties
#endif
cuMipmappedArrayGetSparseProperties:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3872(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3872(%rip)
2:
  pushq $484
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuModuleEnumerateFunctions
  .p2align 4
  .type cuModuleEnumerateFunctions, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuModuleEnumerateFunctions
#endif
cuModuleEnumerateFunctions:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3880(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3880(%rip)
2:
  pushq $485
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuModuleGetFunction
  .p2align 4
  .type cuModuleGetFunction, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuModuleGetFunction
#endif
cuModuleGetFunction:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3888(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3888(%rip)
2:
  pushq $486
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuModuleGetFunctionCount
  .p2align 4
  .type cuModuleGetFunctionCount, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuModuleGetFunctionCount
#endif
cuModuleGetFunctionCount:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3896(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3896(%rip)
2:
  pushq $487
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuModuleGetGlobal
  .p2align 4
  .type cuModuleGetGlobal, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuModuleGetGlobal
#endif
cuModuleGetGlobal:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3904(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3904(%rip)
2:
  pushq $488
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuModuleGetGlobal_v2
  .p2align 4
  .type cuModuleGetGlobal_v2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuModuleGetGlobal_v2
#endif
cuModuleGetGlobal_v2:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3912(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3912(%rip)
2:
  pushq $489
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuModuleGetLoadingMode
  .p2align 4
  .type cuModuleGetLoadingMode, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuModuleGetLoadingMode
#endif
cuModuleGetLoadingMode:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3920(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3920(%rip)
2:
  pushq $490
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuModuleGetSurfRef
  .p2align 4
  .type cuModuleGetSurfRef, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuModuleGetSurfRef
#endif
cuModuleGetSurfRef:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3928(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3928(%rip)
2:
  pushq $491
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuModuleGetTexRef
  .p2align 4
  .type cuModuleGetTexRef, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuModuleGetTexRef
#endif
cuModuleGetTexRef:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3936(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3936(%rip)
2:
  pushq $492
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuModuleLoad
  .p2align 4
  .type cuModuleLoad, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuModuleLoad
#endif
cuModuleLoad:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3944(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3944(%rip)
2:
  pushq $493
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuModuleLoadData
  .p2align 4
  .type cuModuleLoadData, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuModuleLoadData
#endif
cuModuleLoadData:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3952(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3952(%rip)
2:
  pushq $494
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuModuleLoadDataEx
  .p2align 4
  .type cuModuleLoadDataEx, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuModuleLoadDataEx
#endif
cuModuleLoadDataEx:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3960(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3960(%rip)
2:
  pushq $495
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuModuleLoadFatBinary
  .p2align 4
  .type cuModuleLoadFatBinary, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuModuleLoadFatBinary
#endif
cuModuleLoadFatBinary:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3968(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3968(%rip)
2:
  pushq $496
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuModuleUnload
  .p2align 4
  .type cuModuleUnload, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuModuleUnload
#endif
cuModuleUnload:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3976(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3976(%rip)
2:
  pushq $497
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMulticastAddDevice
  .p2align 4
  .type cuMulticastAddDevice, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMulticastAddDevice
#endif
cuMulticastAddDevice:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3984(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3984(%rip)
2:
  pushq $498
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMulticastBindAddr
  .p2align 4
  .type cuMulticastBindAddr, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMulticastBindAddr
#endif
cuMulticastBindAddr:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+3992(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+3992(%rip)
2:
  pushq $499
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMulticastBindMem
  .p2align 4
  .type cuMulticastBindMem, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMulticastBindMem
#endif
cuMulticastBindMem:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4000(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4000(%rip)
2:
  pushq $500
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMulticastCreate
  .p2align 4
  .type cuMulticastCreate, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMulticastCreate
#endif
cuMulticastCreate:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4008(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4008(%rip)
2:
  pushq $501
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMulticastGetGranularity
  .p2align 4
  .type cuMulticastGetGranularity, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMulticastGetGranularity
#endif
cuMulticastGetGranularity:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4016(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4016(%rip)
2:
  pushq $502
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuMulticastUnbind
  .p2align 4
  .type cuMulticastUnbind, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuMulticastUnbind
#endif
cuMulticastUnbind:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4024(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4024(%rip)
2:
  pushq $503
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuOccupancyAvailableDynamicSMemPerBlock
  .p2align 4
  .type cuOccupancyAvailableDynamicSMemPerBlock, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuOccupancyAvailableDynamicSMemPerBlock
#endif
cuOccupancyAvailableDynamicSMemPerBlock:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4032(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4032(%rip)
2:
  pushq $504
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuOccupancyMaxActiveBlocksPerMultiprocessor
  .p2align 4
  .type cuOccupancyMaxActiveBlocksPerMultiprocessor, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuOccupancyMaxActiveBlocksPerMultiprocessor
#endif
cuOccupancyMaxActiveBlocksPerMultiprocessor:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4040(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4040(%rip)
2:
  pushq $505
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuOccupancyMaxActiveBlocksPerMultiprocessorWithFlags
  .p2align 4
  .type cuOccupancyMaxActiveBlocksPerMultiprocessorWithFlags, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuOccupancyMaxActiveBlocksPerMultiprocessorWithFlags
#endif
cuOccupancyMaxActiveBlocksPerMultiprocessorWithFlags:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4048(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4048(%rip)
2:
  pushq $506
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuOccupancyMaxActiveClusters
  .p2align 4
  .type cuOccupancyMaxActiveClusters, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuOccupancyMaxActiveClusters
#endif
cuOccupancyMaxActiveClusters:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4056(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4056(%rip)
2:
  pushq $507
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuOccupancyMaxPotentialBlockSize
  .p2align 4
  .type cuOccupancyMaxPotentialBlockSize, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuOccupancyMaxPotentialBlockSize
#endif
cuOccupancyMaxPotentialBlockSize:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4064(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4064(%rip)
2:
  pushq $508
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuOccupancyMaxPotentialBlockSizeWithFlags
  .p2align 4
  .type cuOccupancyMaxPotentialBlockSizeWithFlags, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuOccupancyMaxPotentialBlockSizeWithFlags
#endif
cuOccupancyMaxPotentialBlockSizeWithFlags:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4072(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4072(%rip)
2:
  pushq $509
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuOccupancyMaxPotentialClusterSize
  .p2align 4
  .type cuOccupancyMaxPotentialClusterSize, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuOccupancyMaxPotentialClusterSize
#endif
cuOccupancyMaxPotentialClusterSize:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4080(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4080(%rip)
2:
  pushq $510
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuParamSetSize
  .p2align 4
  .type cuParamSetSize, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuParamSetSize
#endif
cuParamSetSize:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4088(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4088(%rip)
2:
  pushq $511
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuParamSetTexRef
  .p2align 4
  .type cuParamSetTexRef, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuParamSetTexRef
#endif
cuParamSetTexRef:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4096(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4096(%rip)
2:
  pushq $512
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuParamSetf
  .p2align 4
  .type cuParamSetf, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuParamSetf
#endif
cuParamSetf:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4104(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4104(%rip)
2:
  pushq $513
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuParamSeti
  .p2align 4
  .type cuParamSeti, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuParamSeti
#endif
cuParamSeti:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4112(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4112(%rip)
2:
  pushq $514
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuParamSetv
  .p2align 4
  .type cuParamSetv, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuParamSetv
#endif
cuParamSetv:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4120(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4120(%rip)
2:
  pushq $515
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuPointerGetAttribute
  .p2align 4
  .type cuPointerGetAttribute, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuPointerGetAttribute
#endif
cuPointerGetAttribute:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4128(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4128(%rip)
2:
  pushq $516
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuPointerGetAttributes
  .p2align 4
  .type cuPointerGetAttributes, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuPointerGetAttributes
#endif
cuPointerGetAttributes:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4136(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4136(%rip)
2:
  pushq $517
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuPointerSetAttribute
  .p2align 4
  .type cuPointerSetAttribute, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuPointerSetAttribute
#endif
cuPointerSetAttribute:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4144(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4144(%rip)
2:
  pushq $518
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuProfilerInitialize
  .p2align 4
  .type cuProfilerInitialize, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuProfilerInitialize
#endif
cuProfilerInitialize:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4152(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4152(%rip)
2:
  pushq $519
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuProfilerStart
  .p2align 4
  .type cuProfilerStart, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuProfilerStart
#endif
cuProfilerStart:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4160(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4160(%rip)
2:
  pushq $520
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuProfilerStop
  .p2align 4
  .type cuProfilerStop, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuProfilerStop
#endif
cuProfilerStop:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4168(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4168(%rip)
2:
  pushq $521
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuSignalExternalSemaphoresAsync
  .p2align 4
  .type cuSignalExternalSemaphoresAsync, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuSignalExternalSemaphoresAsync
#endif
cuSignalExternalSemaphoresAsync:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4176(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4176(%rip)
2:
  pushq $522
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuSignalExternalSemaphoresAsync_ptsz
  .p2align 4
  .type cuSignalExternalSemaphoresAsync_ptsz, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuSignalExternalSemaphoresAsync_ptsz
#endif
cuSignalExternalSemaphoresAsync_ptsz:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4184(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4184(%rip)
2:
  pushq $523
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuStreamAddCallback
  .p2align 4
  .type cuStreamAddCallback, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuStreamAddCallback
#endif
cuStreamAddCallback:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4192(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4192(%rip)
2:
  pushq $524
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuStreamAddCallback_ptsz
  .p2align 4
  .type cuStreamAddCallback_ptsz, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuStreamAddCallback_ptsz
#endif
cuStreamAddCallback_ptsz:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4200(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4200(%rip)
2:
  pushq $525
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuStreamAttachMemAsync
  .p2align 4
  .type cuStreamAttachMemAsync, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuStreamAttachMemAsync
#endif
cuStreamAttachMemAsync:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4208(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4208(%rip)
2:
  pushq $526
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuStreamAttachMemAsync_ptsz
  .p2align 4
  .type cuStreamAttachMemAsync_ptsz, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuStreamAttachMemAsync_ptsz
#endif
cuStreamAttachMemAsync_ptsz:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4216(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4216(%rip)
2:
  pushq $527
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuStreamBatchMemOp
  .p2align 4
  .type cuStreamBatchMemOp, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuStreamBatchMemOp
#endif
cuStreamBatchMemOp:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4224(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4224(%rip)
2:
  pushq $528
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuStreamBatchMemOp_ptsz
  .p2align 4
  .type cuStreamBatchMemOp_ptsz, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuStreamBatchMemOp_ptsz
#endif
cuStreamBatchMemOp_ptsz:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4232(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4232(%rip)
2:
  pushq $529
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuStreamBatchMemOp_v2
  .p2align 4
  .type cuStreamBatchMemOp_v2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuStreamBatchMemOp_v2
#endif
cuStreamBatchMemOp_v2:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4240(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4240(%rip)
2:
  pushq $530
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuStreamBatchMemOp_v2_ptsz
  .p2align 4
  .type cuStreamBatchMemOp_v2_ptsz, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuStreamBatchMemOp_v2_ptsz
#endif
cuStreamBatchMemOp_v2_ptsz:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4248(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4248(%rip)
2:
  pushq $531
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuStreamBeginCapture
  .p2align 4
  .type cuStreamBeginCapture, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuStreamBeginCapture
#endif
cuStreamBeginCapture:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4256(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4256(%rip)
2:
  pushq $532
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuStreamBeginCaptureToGraph
  .p2align 4
  .type cuStreamBeginCaptureToGraph, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuStreamBeginCaptureToGraph
#endif
cuStreamBeginCaptureToGraph:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4264(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4264(%rip)
2:
  pushq $533
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuStreamBeginCaptureToGraph_ptsz
  .p2align 4
  .type cuStreamBeginCaptureToGraph_ptsz, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuStreamBeginCaptureToGraph_ptsz
#endif
cuStreamBeginCaptureToGraph_ptsz:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4272(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4272(%rip)
2:
  pushq $534
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuStreamBeginCapture_ptsz
  .p2align 4
  .type cuStreamBeginCapture_ptsz, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuStreamBeginCapture_ptsz
#endif
cuStreamBeginCapture_ptsz:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4280(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4280(%rip)
2:
  pushq $535
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuStreamBeginCapture_v2
  .p2align 4
  .type cuStreamBeginCapture_v2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuStreamBeginCapture_v2
#endif
cuStreamBeginCapture_v2:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4288(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4288(%rip)
2:
  pushq $536
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuStreamBeginCapture_v2_ptsz
  .p2align 4
  .type cuStreamBeginCapture_v2_ptsz, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuStreamBeginCapture_v2_ptsz
#endif
cuStreamBeginCapture_v2_ptsz:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4296(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4296(%rip)
2:
  pushq $537
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuStreamCopyAttributes
  .p2align 4
  .type cuStreamCopyAttributes, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuStreamCopyAttributes
#endif
cuStreamCopyAttributes:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4304(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4304(%rip)
2:
  pushq $538
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuStreamCopyAttributes_ptsz
  .p2align 4
  .type cuStreamCopyAttributes_ptsz, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuStreamCopyAttributes_ptsz
#endif
cuStreamCopyAttributes_ptsz:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4312(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4312(%rip)
2:
  pushq $539
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuStreamCreate
  .p2align 4
  .type cuStreamCreate, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuStreamCreate
#endif
cuStreamCreate:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4320(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4320(%rip)
2:
  pushq $540
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuStreamCreateWithPriority
  .p2align 4
  .type cuStreamCreateWithPriority, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuStreamCreateWithPriority
#endif
cuStreamCreateWithPriority:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4328(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4328(%rip)
2:
  pushq $541
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuStreamDestroy
  .p2align 4
  .type cuStreamDestroy, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuStreamDestroy
#endif
cuStreamDestroy:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4336(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4336(%rip)
2:
  pushq $542
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuStreamDestroy_v2
  .p2align 4
  .type cuStreamDestroy_v2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuStreamDestroy_v2
#endif
cuStreamDestroy_v2:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4344(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4344(%rip)
2:
  pushq $543
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuStreamEndCapture
  .p2align 4
  .type cuStreamEndCapture, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuStreamEndCapture
#endif
cuStreamEndCapture:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4352(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4352(%rip)
2:
  pushq $544
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuStreamEndCapture_ptsz
  .p2align 4
  .type cuStreamEndCapture_ptsz, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuStreamEndCapture_ptsz
#endif
cuStreamEndCapture_ptsz:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4360(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4360(%rip)
2:
  pushq $545
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuStreamGetAttribute
  .p2align 4
  .type cuStreamGetAttribute, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuStreamGetAttribute
#endif
cuStreamGetAttribute:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4368(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4368(%rip)
2:
  pushq $546
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuStreamGetAttribute_ptsz
  .p2align 4
  .type cuStreamGetAttribute_ptsz, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuStreamGetAttribute_ptsz
#endif
cuStreamGetAttribute_ptsz:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4376(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4376(%rip)
2:
  pushq $547
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuStreamGetCaptureInfo
  .p2align 4
  .type cuStreamGetCaptureInfo, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuStreamGetCaptureInfo
#endif
cuStreamGetCaptureInfo:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4384(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4384(%rip)
2:
  pushq $548
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuStreamGetCaptureInfo_ptsz
  .p2align 4
  .type cuStreamGetCaptureInfo_ptsz, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuStreamGetCaptureInfo_ptsz
#endif
cuStreamGetCaptureInfo_ptsz:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4392(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4392(%rip)
2:
  pushq $549
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuStreamGetCaptureInfo_v2
  .p2align 4
  .type cuStreamGetCaptureInfo_v2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuStreamGetCaptureInfo_v2
#endif
cuStreamGetCaptureInfo_v2:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4400(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4400(%rip)
2:
  pushq $550
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuStreamGetCaptureInfo_v2_ptsz
  .p2align 4
  .type cuStreamGetCaptureInfo_v2_ptsz, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuStreamGetCaptureInfo_v2_ptsz
#endif
cuStreamGetCaptureInfo_v2_ptsz:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4408(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4408(%rip)
2:
  pushq $551
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuStreamGetCaptureInfo_v3
  .p2align 4
  .type cuStreamGetCaptureInfo_v3, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuStreamGetCaptureInfo_v3
#endif
cuStreamGetCaptureInfo_v3:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4416(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4416(%rip)
2:
  pushq $552
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuStreamGetCaptureInfo_v3_ptsz
  .p2align 4
  .type cuStreamGetCaptureInfo_v3_ptsz, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuStreamGetCaptureInfo_v3_ptsz
#endif
cuStreamGetCaptureInfo_v3_ptsz:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4424(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4424(%rip)
2:
  pushq $553
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuStreamGetCtx
  .p2align 4
  .type cuStreamGetCtx, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuStreamGetCtx
#endif
cuStreamGetCtx:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4432(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4432(%rip)
2:
  pushq $554
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuStreamGetCtx_ptsz
  .p2align 4
  .type cuStreamGetCtx_ptsz, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuStreamGetCtx_ptsz
#endif
cuStreamGetCtx_ptsz:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4440(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4440(%rip)
2:
  pushq $555
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuStreamGetCtx_v2
  .p2align 4
  .type cuStreamGetCtx_v2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuStreamGetCtx_v2
#endif
cuStreamGetCtx_v2:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4448(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4448(%rip)
2:
  pushq $556
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuStreamGetCtx_v2_ptsz
  .p2align 4
  .type cuStreamGetCtx_v2_ptsz, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuStreamGetCtx_v2_ptsz
#endif
cuStreamGetCtx_v2_ptsz:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4456(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4456(%rip)
2:
  pushq $557
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuStreamGetDevice
  .p2align 4
  .type cuStreamGetDevice, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuStreamGetDevice
#endif
cuStreamGetDevice:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4464(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4464(%rip)
2:
  pushq $558
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuStreamGetDevice_ptsz
  .p2align 4
  .type cuStreamGetDevice_ptsz, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuStreamGetDevice_ptsz
#endif
cuStreamGetDevice_ptsz:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4472(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4472(%rip)
2:
  pushq $559
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuStreamGetFlags
  .p2align 4
  .type cuStreamGetFlags, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuStreamGetFlags
#endif
cuStreamGetFlags:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4480(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4480(%rip)
2:
  pushq $560
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuStreamGetFlags_ptsz
  .p2align 4
  .type cuStreamGetFlags_ptsz, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuStreamGetFlags_ptsz
#endif
cuStreamGetFlags_ptsz:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4488(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4488(%rip)
2:
  pushq $561
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuStreamGetGreenCtx
  .p2align 4
  .type cuStreamGetGreenCtx, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuStreamGetGreenCtx
#endif
cuStreamGetGreenCtx:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4496(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4496(%rip)
2:
  pushq $562
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuStreamGetId
  .p2align 4
  .type cuStreamGetId, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuStreamGetId
#endif
cuStreamGetId:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4504(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4504(%rip)
2:
  pushq $563
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuStreamGetId_ptsz
  .p2align 4
  .type cuStreamGetId_ptsz, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuStreamGetId_ptsz
#endif
cuStreamGetId_ptsz:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4512(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4512(%rip)
2:
  pushq $564
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuStreamGetPriority
  .p2align 4
  .type cuStreamGetPriority, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuStreamGetPriority
#endif
cuStreamGetPriority:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4520(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4520(%rip)
2:
  pushq $565
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuStreamGetPriority_ptsz
  .p2align 4
  .type cuStreamGetPriority_ptsz, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuStreamGetPriority_ptsz
#endif
cuStreamGetPriority_ptsz:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4528(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4528(%rip)
2:
  pushq $566
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuStreamIsCapturing
  .p2align 4
  .type cuStreamIsCapturing, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuStreamIsCapturing
#endif
cuStreamIsCapturing:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4536(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4536(%rip)
2:
  pushq $567
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuStreamIsCapturing_ptsz
  .p2align 4
  .type cuStreamIsCapturing_ptsz, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuStreamIsCapturing_ptsz
#endif
cuStreamIsCapturing_ptsz:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4544(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4544(%rip)
2:
  pushq $568
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuStreamQuery
  .p2align 4
  .type cuStreamQuery, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuStreamQuery
#endif
cuStreamQuery:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4552(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4552(%rip)
2:
  pushq $569
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuStreamQuery_ptsz
  .p2align 4
  .type cuStreamQuery_ptsz, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuStreamQuery_ptsz
#endif
cuStreamQuery_ptsz:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4560(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4560(%rip)
2:
  pushq $570
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuStreamSetAttribute
  .p2align 4
  .type cuStreamSetAttribute, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuStreamSetAttribute
#endif
cuStreamSetAttribute:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4568(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4568(%rip)
2:
  pushq $571
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuStreamSetAttribute_ptsz
  .p2align 4
  .type cuStreamSetAttribute_ptsz, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuStreamSetAttribute_ptsz
#endif
cuStreamSetAttribute_ptsz:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4576(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4576(%rip)
2:
  pushq $572
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuStreamSynchronize
  .p2align 4
  .type cuStreamSynchronize, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuStreamSynchronize
#endif
cuStreamSynchronize:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4584(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4584(%rip)
2:
  pushq $573
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuStreamSynchronize_ptsz
  .p2align 4
  .type cuStreamSynchronize_ptsz, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuStreamSynchronize_ptsz
#endif
cuStreamSynchronize_ptsz:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4592(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4592(%rip)
2:
  pushq $574
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuStreamUpdateCaptureDependencies
  .p2align 4
  .type cuStreamUpdateCaptureDependencies, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuStreamUpdateCaptureDependencies
#endif
cuStreamUpdateCaptureDependencies:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4600(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4600(%rip)
2:
  pushq $575
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuStreamUpdateCaptureDependencies_ptsz
  .p2align 4
  .type cuStreamUpdateCaptureDependencies_ptsz, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuStreamUpdateCaptureDependencies_ptsz
#endif
cuStreamUpdateCaptureDependencies_ptsz:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4608(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4608(%rip)
2:
  pushq $576
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuStreamUpdateCaptureDependencies_v2
  .p2align 4
  .type cuStreamUpdateCaptureDependencies_v2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuStreamUpdateCaptureDependencies_v2
#endif
cuStreamUpdateCaptureDependencies_v2:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4616(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4616(%rip)
2:
  pushq $577
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuStreamUpdateCaptureDependencies_v2_ptsz
  .p2align 4
  .type cuStreamUpdateCaptureDependencies_v2_ptsz, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuStreamUpdateCaptureDependencies_v2_ptsz
#endif
cuStreamUpdateCaptureDependencies_v2_ptsz:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4624(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4624(%rip)
2:
  pushq $578
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuStreamWaitEvent
  .p2align 4
  .type cuStreamWaitEvent, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuStreamWaitEvent
#endif
cuStreamWaitEvent:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4632(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4632(%rip)
2:
  pushq $579
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuStreamWaitEvent_ptsz
  .p2align 4
  .type cuStreamWaitEvent_ptsz, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuStreamWaitEvent_ptsz
#endif
cuStreamWaitEvent_ptsz:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4640(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4640(%rip)
2:
  pushq $580
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuStreamWaitValue32
  .p2align 4
  .type cuStreamWaitValue32, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuStreamWaitValue32
#endif
cuStreamWaitValue32:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4648(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4648(%rip)
2:
  pushq $581
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuStreamWaitValue32_ptsz
  .p2align 4
  .type cuStreamWaitValue32_ptsz, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuStreamWaitValue32_ptsz
#endif
cuStreamWaitValue32_ptsz:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4656(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4656(%rip)
2:
  pushq $582
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuStreamWaitValue32_v2
  .p2align 4
  .type cuStreamWaitValue32_v2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuStreamWaitValue32_v2
#endif
cuStreamWaitValue32_v2:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4664(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4664(%rip)
2:
  pushq $583
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuStreamWaitValue32_v2_ptsz
  .p2align 4
  .type cuStreamWaitValue32_v2_ptsz, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuStreamWaitValue32_v2_ptsz
#endif
cuStreamWaitValue32_v2_ptsz:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4672(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4672(%rip)
2:
  pushq $584
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuStreamWaitValue64
  .p2align 4
  .type cuStreamWaitValue64, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuStreamWaitValue64
#endif
cuStreamWaitValue64:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4680(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4680(%rip)
2:
  pushq $585
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuStreamWaitValue64_ptsz
  .p2align 4
  .type cuStreamWaitValue64_ptsz, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuStreamWaitValue64_ptsz
#endif
cuStreamWaitValue64_ptsz:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4688(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4688(%rip)
2:
  pushq $586
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuStreamWaitValue64_v2
  .p2align 4
  .type cuStreamWaitValue64_v2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuStreamWaitValue64_v2
#endif
cuStreamWaitValue64_v2:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4696(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4696(%rip)
2:
  pushq $587
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuStreamWaitValue64_v2_ptsz
  .p2align 4
  .type cuStreamWaitValue64_v2_ptsz, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuStreamWaitValue64_v2_ptsz
#endif
cuStreamWaitValue64_v2_ptsz:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4704(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4704(%rip)
2:
  pushq $588
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuStreamWriteValue32
  .p2align 4
  .type cuStreamWriteValue32, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuStreamWriteValue32
#endif
cuStreamWriteValue32:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4712(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4712(%rip)
2:
  pushq $589
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuStreamWriteValue32_ptsz
  .p2align 4
  .type cuStreamWriteValue32_ptsz, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuStreamWriteValue32_ptsz
#endif
cuStreamWriteValue32_ptsz:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4720(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4720(%rip)
2:
  pushq $590
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuStreamWriteValue32_v2
  .p2align 4
  .type cuStreamWriteValue32_v2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuStreamWriteValue32_v2
#endif
cuStreamWriteValue32_v2:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4728(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4728(%rip)
2:
  pushq $591
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuStreamWriteValue32_v2_ptsz
  .p2align 4
  .type cuStreamWriteValue32_v2_ptsz, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuStreamWriteValue32_v2_ptsz
#endif
cuStreamWriteValue32_v2_ptsz:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4736(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4736(%rip)
2:
  pushq $592
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuStreamWriteValue64
  .p2align 4
  .type cuStreamWriteValue64, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuStreamWriteValue64
#endif
cuStreamWriteValue64:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4744(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4744(%rip)
2:
  pushq $593
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuStreamWriteValue64_ptsz
  .p2align 4
  .type cuStreamWriteValue64_ptsz, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuStreamWriteValue64_ptsz
#endif
cuStreamWriteValue64_ptsz:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4752(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4752(%rip)
2:
  pushq $594
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuStreamWriteValue64_v2
  .p2align 4
  .type cuStreamWriteValue64_v2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuStreamWriteValue64_v2
#endif
cuStreamWriteValue64_v2:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4760(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4760(%rip)
2:
  pushq $595
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuStreamWriteValue64_v2_ptsz
  .p2align 4
  .type cuStreamWriteValue64_v2_ptsz, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuStreamWriteValue64_v2_ptsz
#endif
cuStreamWriteValue64_v2_ptsz:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4768(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4768(%rip)
2:
  pushq $596
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuSurfObjectCreate
  .p2align 4
  .type cuSurfObjectCreate, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuSurfObjectCreate
#endif
cuSurfObjectCreate:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4776(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4776(%rip)
2:
  pushq $597
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuSurfObjectDestroy
  .p2align 4
  .type cuSurfObjectDestroy, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuSurfObjectDestroy
#endif
cuSurfObjectDestroy:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4784(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4784(%rip)
2:
  pushq $598
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuSurfObjectGetResourceDesc
  .p2align 4
  .type cuSurfObjectGetResourceDesc, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuSurfObjectGetResourceDesc
#endif
cuSurfObjectGetResourceDesc:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4792(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4792(%rip)
2:
  pushq $599
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuSurfRefGetArray
  .p2align 4
  .type cuSurfRefGetArray, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuSurfRefGetArray
#endif
cuSurfRefGetArray:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4800(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4800(%rip)
2:
  pushq $600
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuSurfRefSetArray
  .p2align 4
  .type cuSurfRefSetArray, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuSurfRefSetArray
#endif
cuSurfRefSetArray:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4808(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4808(%rip)
2:
  pushq $601
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuTensorMapEncodeIm2col
  .p2align 4
  .type cuTensorMapEncodeIm2col, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuTensorMapEncodeIm2col
#endif
cuTensorMapEncodeIm2col:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4816(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4816(%rip)
2:
  pushq $602
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuTensorMapEncodeIm2colWide
  .p2align 4
  .type cuTensorMapEncodeIm2colWide, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuTensorMapEncodeIm2colWide
#endif
cuTensorMapEncodeIm2colWide:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4824(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4824(%rip)
2:
  pushq $603
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuTensorMapEncodeTiled
  .p2align 4
  .type cuTensorMapEncodeTiled, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuTensorMapEncodeTiled
#endif
cuTensorMapEncodeTiled:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4832(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4832(%rip)
2:
  pushq $604
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuTensorMapReplaceAddress
  .p2align 4
  .type cuTensorMapReplaceAddress, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuTensorMapReplaceAddress
#endif
cuTensorMapReplaceAddress:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4840(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4840(%rip)
2:
  pushq $605
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuTexObjectCreate
  .p2align 4
  .type cuTexObjectCreate, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuTexObjectCreate
#endif
cuTexObjectCreate:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4848(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4848(%rip)
2:
  pushq $606
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuTexObjectDestroy
  .p2align 4
  .type cuTexObjectDestroy, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuTexObjectDestroy
#endif
cuTexObjectDestroy:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4856(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4856(%rip)
2:
  pushq $607
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuTexObjectGetResourceDesc
  .p2align 4
  .type cuTexObjectGetResourceDesc, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuTexObjectGetResourceDesc
#endif
cuTexObjectGetResourceDesc:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4864(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4864(%rip)
2:
  pushq $608
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuTexObjectGetResourceViewDesc
  .p2align 4
  .type cuTexObjectGetResourceViewDesc, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuTexObjectGetResourceViewDesc
#endif
cuTexObjectGetResourceViewDesc:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4872(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4872(%rip)
2:
  pushq $609
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuTexObjectGetTextureDesc
  .p2align 4
  .type cuTexObjectGetTextureDesc, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuTexObjectGetTextureDesc
#endif
cuTexObjectGetTextureDesc:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4880(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4880(%rip)
2:
  pushq $610
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuTexRefCreate
  .p2align 4
  .type cuTexRefCreate, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuTexRefCreate
#endif
cuTexRefCreate:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4888(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4888(%rip)
2:
  pushq $611
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuTexRefDestroy
  .p2align 4
  .type cuTexRefDestroy, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuTexRefDestroy
#endif
cuTexRefDestroy:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4896(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4896(%rip)
2:
  pushq $612
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuTexRefGetAddress
  .p2align 4
  .type cuTexRefGetAddress, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuTexRefGetAddress
#endif
cuTexRefGetAddress:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4904(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4904(%rip)
2:
  pushq $613
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuTexRefGetAddressMode
  .p2align 4
  .type cuTexRefGetAddressMode, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuTexRefGetAddressMode
#endif
cuTexRefGetAddressMode:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4912(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4912(%rip)
2:
  pushq $614
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuTexRefGetAddress_v2
  .p2align 4
  .type cuTexRefGetAddress_v2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuTexRefGetAddress_v2
#endif
cuTexRefGetAddress_v2:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4920(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4920(%rip)
2:
  pushq $615
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuTexRefGetArray
  .p2align 4
  .type cuTexRefGetArray, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuTexRefGetArray
#endif
cuTexRefGetArray:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4928(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4928(%rip)
2:
  pushq $616
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuTexRefGetBorderColor
  .p2align 4
  .type cuTexRefGetBorderColor, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuTexRefGetBorderColor
#endif
cuTexRefGetBorderColor:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4936(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4936(%rip)
2:
  pushq $617
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuTexRefGetFilterMode
  .p2align 4
  .type cuTexRefGetFilterMode, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuTexRefGetFilterMode
#endif
cuTexRefGetFilterMode:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4944(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4944(%rip)
2:
  pushq $618
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuTexRefGetFlags
  .p2align 4
  .type cuTexRefGetFlags, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuTexRefGetFlags
#endif
cuTexRefGetFlags:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4952(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4952(%rip)
2:
  pushq $619
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuTexRefGetFormat
  .p2align 4
  .type cuTexRefGetFormat, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuTexRefGetFormat
#endif
cuTexRefGetFormat:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4960(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4960(%rip)
2:
  pushq $620
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuTexRefGetMaxAnisotropy
  .p2align 4
  .type cuTexRefGetMaxAnisotropy, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuTexRefGetMaxAnisotropy
#endif
cuTexRefGetMaxAnisotropy:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4968(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4968(%rip)
2:
  pushq $621
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuTexRefGetMipmapFilterMode
  .p2align 4
  .type cuTexRefGetMipmapFilterMode, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuTexRefGetMipmapFilterMode
#endif
cuTexRefGetMipmapFilterMode:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4976(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4976(%rip)
2:
  pushq $622
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuTexRefGetMipmapLevelBias
  .p2align 4
  .type cuTexRefGetMipmapLevelBias, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuTexRefGetMipmapLevelBias
#endif
cuTexRefGetMipmapLevelBias:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4984(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4984(%rip)
2:
  pushq $623
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuTexRefGetMipmapLevelClamp
  .p2align 4
  .type cuTexRefGetMipmapLevelClamp, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuTexRefGetMipmapLevelClamp
#endif
cuTexRefGetMipmapLevelClamp:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+4992(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+4992(%rip)
2:
  pushq $624
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuTexRefGetMipmappedArray
  .p2align 4
  .type cuTexRefGetMipmappedArray, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuTexRefGetMipmappedArray
#endif
cuTexRefGetMipmappedArray:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+5000(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+5000(%rip)
2:
  pushq $625
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuTexRefSetAddress
  .p2align 4
  .type cuTexRefSetAddress, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuTexRefSetAddress
#endif
cuTexRefSetAddress:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+5008(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+5008(%rip)
2:
  pushq $626
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuTexRefSetAddress2D
  .p2align 4
  .type cuTexRefSetAddress2D, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuTexRefSetAddress2D
#endif
cuTexRefSetAddress2D:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+5016(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+5016(%rip)
2:
  pushq $627
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuTexRefSetAddress2D_v2
  .p2align 4
  .type cuTexRefSetAddress2D_v2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuTexRefSetAddress2D_v2
#endif
cuTexRefSetAddress2D_v2:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+5024(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+5024(%rip)
2:
  pushq $628
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuTexRefSetAddress2D_v3
  .p2align 4
  .type cuTexRefSetAddress2D_v3, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuTexRefSetAddress2D_v3
#endif
cuTexRefSetAddress2D_v3:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+5032(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+5032(%rip)
2:
  pushq $629
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuTexRefSetAddressMode
  .p2align 4
  .type cuTexRefSetAddressMode, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuTexRefSetAddressMode
#endif
cuTexRefSetAddressMode:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+5040(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+5040(%rip)
2:
  pushq $630
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuTexRefSetAddress_v2
  .p2align 4
  .type cuTexRefSetAddress_v2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuTexRefSetAddress_v2
#endif
cuTexRefSetAddress_v2:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+5048(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+5048(%rip)
2:
  pushq $631
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuTexRefSetArray
  .p2align 4
  .type cuTexRefSetArray, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuTexRefSetArray
#endif
cuTexRefSetArray:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+5056(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+5056(%rip)
2:
  pushq $632
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuTexRefSetBorderColor
  .p2align 4
  .type cuTexRefSetBorderColor, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuTexRefSetBorderColor
#endif
cuTexRefSetBorderColor:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+5064(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+5064(%rip)
2:
  pushq $633
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuTexRefSetFilterMode
  .p2align 4
  .type cuTexRefSetFilterMode, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuTexRefSetFilterMode
#endif
cuTexRefSetFilterMode:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+5072(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+5072(%rip)
2:
  pushq $634
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuTexRefSetFlags
  .p2align 4
  .type cuTexRefSetFlags, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuTexRefSetFlags
#endif
cuTexRefSetFlags:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+5080(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+5080(%rip)
2:
  pushq $635
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuTexRefSetFormat
  .p2align 4
  .type cuTexRefSetFormat, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuTexRefSetFormat
#endif
cuTexRefSetFormat:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+5088(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+5088(%rip)
2:
  pushq $636
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuTexRefSetMaxAnisotropy
  .p2align 4
  .type cuTexRefSetMaxAnisotropy, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuTexRefSetMaxAnisotropy
#endif
cuTexRefSetMaxAnisotropy:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+5096(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+5096(%rip)
2:
  pushq $637
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuTexRefSetMipmapFilterMode
  .p2align 4
  .type cuTexRefSetMipmapFilterMode, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuTexRefSetMipmapFilterMode
#endif
cuTexRefSetMipmapFilterMode:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+5104(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+5104(%rip)
2:
  pushq $638
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuTexRefSetMipmapLevelBias
  .p2align 4
  .type cuTexRefSetMipmapLevelBias, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuTexRefSetMipmapLevelBias
#endif
cuTexRefSetMipmapLevelBias:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+5112(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+5112(%rip)
2:
  pushq $639
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuTexRefSetMipmapLevelClamp
  .p2align 4
  .type cuTexRefSetMipmapLevelClamp, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuTexRefSetMipmapLevelClamp
#endif
cuTexRefSetMipmapLevelClamp:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+5120(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+5120(%rip)
2:
  pushq $640
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuTexRefSetMipmappedArray
  .p2align 4
  .type cuTexRefSetMipmappedArray, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuTexRefSetMipmappedArray
#endif
cuTexRefSetMipmappedArray:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+5128(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+5128(%rip)
2:
  pushq $641
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuThreadExchangeStreamCaptureMode
  .p2align 4
  .type cuThreadExchangeStreamCaptureMode, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuThreadExchangeStreamCaptureMode
#endif
cuThreadExchangeStreamCaptureMode:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+5136(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+5136(%rip)
2:
  pushq $642
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuUserObjectCreate
  .p2align 4
  .type cuUserObjectCreate, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuUserObjectCreate
#endif
cuUserObjectCreate:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+5144(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+5144(%rip)
2:
  pushq $643
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuUserObjectRelease
  .p2align 4
  .type cuUserObjectRelease, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuUserObjectRelease
#endif
cuUserObjectRelease:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+5152(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+5152(%rip)
2:
  pushq $644
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuUserObjectRetain
  .p2align 4
  .type cuUserObjectRetain, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuUserObjectRetain
#endif
cuUserObjectRetain:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+5160(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+5160(%rip)
2:
  pushq $645
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuVDPAUCtxCreate
  .p2align 4
  .type cuVDPAUCtxCreate, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuVDPAUCtxCreate
#endif
cuVDPAUCtxCreate:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+5168(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+5168(%rip)
2:
  pushq $646
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuVDPAUCtxCreate_v2
  .p2align 4
  .type cuVDPAUCtxCreate_v2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuVDPAUCtxCreate_v2
#endif
cuVDPAUCtxCreate_v2:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+5176(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+5176(%rip)
2:
  pushq $647
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuVDPAUGetDevice
  .p2align 4
  .type cuVDPAUGetDevice, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuVDPAUGetDevice
#endif
cuVDPAUGetDevice:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+5184(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+5184(%rip)
2:
  pushq $648
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuWaitExternalSemaphoresAsync
  .p2align 4
  .type cuWaitExternalSemaphoresAsync, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuWaitExternalSemaphoresAsync
#endif
cuWaitExternalSemaphoresAsync:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+5192(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+5192(%rip)
2:
  pushq $649
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cuWaitExternalSemaphoresAsync_ptsz
  .p2align 4
  .type cuWaitExternalSemaphoresAsync_ptsz, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cuWaitExternalSemaphoresAsync_ptsz
#endif
cuWaitExternalSemaphoresAsync_ptsz:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+5200(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+5200(%rip)
2:
  pushq $650
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cudbgApiAttach
  .p2align 4
  .type cudbgApiAttach, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cudbgApiAttach
#endif
cudbgApiAttach:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+5208(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+5208(%rip)
2:
  pushq $651
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cudbgApiDetach
  .p2align 4
  .type cudbgApiDetach, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cudbgApiDetach
#endif
cudbgApiDetach:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+5216(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+5216(%rip)
2:
  pushq $652
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cudbgApiInit
  .p2align 4
  .type cudbgApiInit, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cudbgApiInit
#endif
cudbgApiInit:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+5224(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+5224(%rip)
2:
  pushq $653
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cudbgGetAPI
  .p2align 4
  .type cudbgGetAPI, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cudbgGetAPI
#endif
cudbgGetAPI:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+5232(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+5232(%rip)
2:
  pushq $654
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cudbgGetAPIVersion
  .p2align 4
  .type cudbgGetAPIVersion, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cudbgGetAPIVersion
#endif
cudbgGetAPIVersion:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+5240(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+5240(%rip)
2:
  pushq $655
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cudbgMain
  .p2align 4
  .type cudbgMain, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cudbgMain
#endif
cudbgMain:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+5248(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+5248(%rip)
2:
  pushq $656
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cudbgReportDriverApiError
  .p2align 4
  .type cudbgReportDriverApiError, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cudbgReportDriverApiError
#endif
cudbgReportDriverApiError:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+5256(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+5256(%rip)
2:
  pushq $657
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl cudbgReportDriverInternalError
  .p2align 4
  .type cudbgReportDriverInternalError, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden cudbgReportDriverInternalError
#endif
cudbgReportDriverInternalError:
  .cfi_startproc
  .cfi_def_cfa_offset 8  // Return address
  // Intel opt. manual says to
  // "make the fall-through code following a conditional branch be the likely target for a branch with a forward target"
  // to hint static predictor.
  cmpq $0, _libcuda_so_tramp_table+5264(%rip)
  je 2f
1:
  jmp *_libcuda_so_tramp_table+5264(%rip)
2:
  pushq $658
  .cfi_adjust_cfa_offset 8
  call _libcuda_so_save_regs_and_resolve
  addq $8, %rsp
  .cfi_adjust_cfa_offset -8
  jmp *%rax
  .cfi_endproc

