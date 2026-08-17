/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

#define lr x30
#define ip0 x16

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

  // Slow path which calls dlsym, taken only on first call.
  // Registers are saved according to "Procedure Call Standard for the Arm® 64-bit Architecture".
  // For DWARF directives, read https://www.imperialviolet.org/2017/01/18/cfi.html.

  // Stack is aligned at 16 bytes

#define PUSH_PAIR(reg1, reg2) stp reg1, reg2, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset reg1, 0; .cfi_rel_offset reg2, 8
#define POP_PAIR(reg1, reg2) ldp reg1, reg2, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore reg2; .cfi_restore reg1

#define PUSH_WIDE_PAIR(reg1, reg2) stp reg1, reg2, [sp, #-32]!; .cfi_adjust_cfa_offset 32; .cfi_rel_offset reg1, 0; .cfi_rel_offset reg2, 16
#define POP_WIDE_PAIR(reg1, reg2) ldp reg1, reg2, [sp], #32; .cfi_adjust_cfa_offset -32; .cfi_restore reg2; .cfi_restore reg1

  // Save only arguments (and lr)
  PUSH_PAIR(x0, x1)
  PUSH_PAIR(x2, x3)
  PUSH_PAIR(x4, x5)
  PUSH_PAIR(x6, x7)
  PUSH_PAIR(x8, lr)

  ldr x0, [sp, #80]  // 16*5

  PUSH_WIDE_PAIR(q0, q1)
  PUSH_WIDE_PAIR(q2, q3)
  PUSH_WIDE_PAIR(q4, q5)
  PUSH_WIDE_PAIR(q6, q7)

  // Stack is aligned at 16 bytes

  bl _libcuda_so_tramp_resolve
  mov ip0, x0

  // TODO: pop pc?

  POP_WIDE_PAIR(q6, q7)
  POP_WIDE_PAIR(q4, q5)
  POP_WIDE_PAIR(q2, q3)
  POP_WIDE_PAIR(q0, q1)

  POP_PAIR(x8, lr)
  POP_PAIR(x6, x7)
  POP_PAIR(x4, x5)
  POP_PAIR(x2, x3)
  POP_PAIR(x0, x1)

  br lr

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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+0
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+0]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 0 & 0xffff
#if 0 > 0xffff
  movk ip0, 0 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+8
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+8]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 1 & 0xffff
#if 1 > 0xffff
  movk ip0, 1 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+16
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+16]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 2 & 0xffff
#if 2 > 0xffff
  movk ip0, 2 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+24
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+24]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 3 & 0xffff
#if 3 > 0xffff
  movk ip0, 3 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+32
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+32]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 4 & 0xffff
#if 4 > 0xffff
  movk ip0, 4 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+40
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+40]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 5 & 0xffff
#if 5 > 0xffff
  movk ip0, 5 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+48
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+48]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 6 & 0xffff
#if 6 > 0xffff
  movk ip0, 6 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+56
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+56]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 7 & 0xffff
#if 7 > 0xffff
  movk ip0, 7 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+64
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+64]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 8 & 0xffff
#if 8 > 0xffff
  movk ip0, 8 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+72
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+72]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 9 & 0xffff
#if 9 > 0xffff
  movk ip0, 9 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+80
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+80]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 10 & 0xffff
#if 10 > 0xffff
  movk ip0, 10 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+88
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+88]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 11 & 0xffff
#if 11 > 0xffff
  movk ip0, 11 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+96
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+96]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 12 & 0xffff
#if 12 > 0xffff
  movk ip0, 12 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+104
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+104]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 13 & 0xffff
#if 13 > 0xffff
  movk ip0, 13 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+112
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+112]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 14 & 0xffff
#if 14 > 0xffff
  movk ip0, 14 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+120
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+120]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 15 & 0xffff
#if 15 > 0xffff
  movk ip0, 15 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+128
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+128]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 16 & 0xffff
#if 16 > 0xffff
  movk ip0, 16 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+136
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+136]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 17 & 0xffff
#if 17 > 0xffff
  movk ip0, 17 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+144
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+144]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 18 & 0xffff
#if 18 > 0xffff
  movk ip0, 18 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+152
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+152]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 19 & 0xffff
#if 19 > 0xffff
  movk ip0, 19 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+160
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+160]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 20 & 0xffff
#if 20 > 0xffff
  movk ip0, 20 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+168
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+168]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 21 & 0xffff
#if 21 > 0xffff
  movk ip0, 21 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+176
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+176]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 22 & 0xffff
#if 22 > 0xffff
  movk ip0, 22 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+184
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+184]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 23 & 0xffff
#if 23 > 0xffff
  movk ip0, 23 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+192
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+192]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 24 & 0xffff
#if 24 > 0xffff
  movk ip0, 24 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+200
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+200]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 25 & 0xffff
#if 25 > 0xffff
  movk ip0, 25 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+208
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+208]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 26 & 0xffff
#if 26 > 0xffff
  movk ip0, 26 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+216
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+216]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 27 & 0xffff
#if 27 > 0xffff
  movk ip0, 27 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+224
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+224]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 28 & 0xffff
#if 28 > 0xffff
  movk ip0, 28 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+232
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+232]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 29 & 0xffff
#if 29 > 0xffff
  movk ip0, 29 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+240
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+240]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 30 & 0xffff
#if 30 > 0xffff
  movk ip0, 30 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+248
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+248]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 31 & 0xffff
#if 31 > 0xffff
  movk ip0, 31 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+256
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+256]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 32 & 0xffff
#if 32 > 0xffff
  movk ip0, 32 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+264
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+264]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 33 & 0xffff
#if 33 > 0xffff
  movk ip0, 33 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+272
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+272]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 34 & 0xffff
#if 34 > 0xffff
  movk ip0, 34 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+280
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+280]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 35 & 0xffff
#if 35 > 0xffff
  movk ip0, 35 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+288
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+288]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 36 & 0xffff
#if 36 > 0xffff
  movk ip0, 36 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+296
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+296]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 37 & 0xffff
#if 37 > 0xffff
  movk ip0, 37 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+304
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+304]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 38 & 0xffff
#if 38 > 0xffff
  movk ip0, 38 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+312
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+312]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 39 & 0xffff
#if 39 > 0xffff
  movk ip0, 39 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+320
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+320]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 40 & 0xffff
#if 40 > 0xffff
  movk ip0, 40 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+328
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+328]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 41 & 0xffff
#if 41 > 0xffff
  movk ip0, 41 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+336
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+336]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 42 & 0xffff
#if 42 > 0xffff
  movk ip0, 42 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+344
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+344]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 43 & 0xffff
#if 43 > 0xffff
  movk ip0, 43 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+352
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+352]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 44 & 0xffff
#if 44 > 0xffff
  movk ip0, 44 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+360
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+360]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 45 & 0xffff
#if 45 > 0xffff
  movk ip0, 45 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+368
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+368]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 46 & 0xffff
#if 46 > 0xffff
  movk ip0, 46 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+376
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+376]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 47 & 0xffff
#if 47 > 0xffff
  movk ip0, 47 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+384
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+384]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 48 & 0xffff
#if 48 > 0xffff
  movk ip0, 48 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+392
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+392]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 49 & 0xffff
#if 49 > 0xffff
  movk ip0, 49 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+400
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+400]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 50 & 0xffff
#if 50 > 0xffff
  movk ip0, 50 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+408
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+408]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 51 & 0xffff
#if 51 > 0xffff
  movk ip0, 51 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+416
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+416]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 52 & 0xffff
#if 52 > 0xffff
  movk ip0, 52 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+424
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+424]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 53 & 0xffff
#if 53 > 0xffff
  movk ip0, 53 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+432
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+432]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 54 & 0xffff
#if 54 > 0xffff
  movk ip0, 54 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+440
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+440]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 55 & 0xffff
#if 55 > 0xffff
  movk ip0, 55 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+448
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+448]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 56 & 0xffff
#if 56 > 0xffff
  movk ip0, 56 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+456
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+456]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 57 & 0xffff
#if 57 > 0xffff
  movk ip0, 57 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+464
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+464]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 58 & 0xffff
#if 58 > 0xffff
  movk ip0, 58 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+472
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+472]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 59 & 0xffff
#if 59 > 0xffff
  movk ip0, 59 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+480
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+480]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 60 & 0xffff
#if 60 > 0xffff
  movk ip0, 60 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+488
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+488]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 61 & 0xffff
#if 61 > 0xffff
  movk ip0, 61 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+496
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+496]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 62 & 0xffff
#if 62 > 0xffff
  movk ip0, 62 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+504
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+504]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 63 & 0xffff
#if 63 > 0xffff
  movk ip0, 63 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+512
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+512]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 64 & 0xffff
#if 64 > 0xffff
  movk ip0, 64 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+520
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+520]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 65 & 0xffff
#if 65 > 0xffff
  movk ip0, 65 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+528
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+528]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 66 & 0xffff
#if 66 > 0xffff
  movk ip0, 66 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+536
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+536]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 67 & 0xffff
#if 67 > 0xffff
  movk ip0, 67 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+544
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+544]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 68 & 0xffff
#if 68 > 0xffff
  movk ip0, 68 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+552
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+552]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 69 & 0xffff
#if 69 > 0xffff
  movk ip0, 69 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+560
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+560]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 70 & 0xffff
#if 70 > 0xffff
  movk ip0, 70 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+568
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+568]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 71 & 0xffff
#if 71 > 0xffff
  movk ip0, 71 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+576
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+576]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 72 & 0xffff
#if 72 > 0xffff
  movk ip0, 72 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+584
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+584]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 73 & 0xffff
#if 73 > 0xffff
  movk ip0, 73 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+592
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+592]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 74 & 0xffff
#if 74 > 0xffff
  movk ip0, 74 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+600
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+600]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 75 & 0xffff
#if 75 > 0xffff
  movk ip0, 75 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+608
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+608]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 76 & 0xffff
#if 76 > 0xffff
  movk ip0, 76 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+616
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+616]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 77 & 0xffff
#if 77 > 0xffff
  movk ip0, 77 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+624
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+624]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 78 & 0xffff
#if 78 > 0xffff
  movk ip0, 78 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+632
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+632]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 79 & 0xffff
#if 79 > 0xffff
  movk ip0, 79 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+640
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+640]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 80 & 0xffff
#if 80 > 0xffff
  movk ip0, 80 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+648
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+648]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 81 & 0xffff
#if 81 > 0xffff
  movk ip0, 81 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+656
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+656]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 82 & 0xffff
#if 82 > 0xffff
  movk ip0, 82 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+664
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+664]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 83 & 0xffff
#if 83 > 0xffff
  movk ip0, 83 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+672
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+672]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 84 & 0xffff
#if 84 > 0xffff
  movk ip0, 84 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+680
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+680]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 85 & 0xffff
#if 85 > 0xffff
  movk ip0, 85 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+688
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+688]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 86 & 0xffff
#if 86 > 0xffff
  movk ip0, 86 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+696
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+696]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 87 & 0xffff
#if 87 > 0xffff
  movk ip0, 87 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+704
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+704]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 88 & 0xffff
#if 88 > 0xffff
  movk ip0, 88 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+712
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+712]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 89 & 0xffff
#if 89 > 0xffff
  movk ip0, 89 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+720
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+720]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 90 & 0xffff
#if 90 > 0xffff
  movk ip0, 90 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+728
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+728]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 91 & 0xffff
#if 91 > 0xffff
  movk ip0, 91 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+736
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+736]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 92 & 0xffff
#if 92 > 0xffff
  movk ip0, 92 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+744
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+744]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 93 & 0xffff
#if 93 > 0xffff
  movk ip0, 93 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+752
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+752]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 94 & 0xffff
#if 94 > 0xffff
  movk ip0, 94 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+760
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+760]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 95 & 0xffff
#if 95 > 0xffff
  movk ip0, 95 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+768
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+768]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 96 & 0xffff
#if 96 > 0xffff
  movk ip0, 96 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+776
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+776]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 97 & 0xffff
#if 97 > 0xffff
  movk ip0, 97 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+784
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+784]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 98 & 0xffff
#if 98 > 0xffff
  movk ip0, 98 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+792
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+792]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 99 & 0xffff
#if 99 > 0xffff
  movk ip0, 99 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+800
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+800]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 100 & 0xffff
#if 100 > 0xffff
  movk ip0, 100 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+808
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+808]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 101 & 0xffff
#if 101 > 0xffff
  movk ip0, 101 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+816
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+816]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 102 & 0xffff
#if 102 > 0xffff
  movk ip0, 102 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+824
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+824]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 103 & 0xffff
#if 103 > 0xffff
  movk ip0, 103 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+832
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+832]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 104 & 0xffff
#if 104 > 0xffff
  movk ip0, 104 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+840
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+840]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 105 & 0xffff
#if 105 > 0xffff
  movk ip0, 105 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+848
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+848]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 106 & 0xffff
#if 106 > 0xffff
  movk ip0, 106 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+856
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+856]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 107 & 0xffff
#if 107 > 0xffff
  movk ip0, 107 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+864
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+864]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 108 & 0xffff
#if 108 > 0xffff
  movk ip0, 108 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+872
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+872]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 109 & 0xffff
#if 109 > 0xffff
  movk ip0, 109 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+880
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+880]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 110 & 0xffff
#if 110 > 0xffff
  movk ip0, 110 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+888
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+888]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 111 & 0xffff
#if 111 > 0xffff
  movk ip0, 111 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+896
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+896]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 112 & 0xffff
#if 112 > 0xffff
  movk ip0, 112 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+904
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+904]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 113 & 0xffff
#if 113 > 0xffff
  movk ip0, 113 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+912
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+912]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 114 & 0xffff
#if 114 > 0xffff
  movk ip0, 114 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+920
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+920]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 115 & 0xffff
#if 115 > 0xffff
  movk ip0, 115 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+928
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+928]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 116 & 0xffff
#if 116 > 0xffff
  movk ip0, 116 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+936
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+936]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 117 & 0xffff
#if 117 > 0xffff
  movk ip0, 117 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+944
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+944]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 118 & 0xffff
#if 118 > 0xffff
  movk ip0, 118 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+952
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+952]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 119 & 0xffff
#if 119 > 0xffff
  movk ip0, 119 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+960
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+960]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 120 & 0xffff
#if 120 > 0xffff
  movk ip0, 120 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+968
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+968]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 121 & 0xffff
#if 121 > 0xffff
  movk ip0, 121 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+976
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+976]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 122 & 0xffff
#if 122 > 0xffff
  movk ip0, 122 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+984
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+984]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 123 & 0xffff
#if 123 > 0xffff
  movk ip0, 123 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+992
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+992]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 124 & 0xffff
#if 124 > 0xffff
  movk ip0, 124 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1000
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1000]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 125 & 0xffff
#if 125 > 0xffff
  movk ip0, 125 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1008
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1008]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 126 & 0xffff
#if 126 > 0xffff
  movk ip0, 126 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1016
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1016]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 127 & 0xffff
#if 127 > 0xffff
  movk ip0, 127 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1024
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1024]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 128 & 0xffff
#if 128 > 0xffff
  movk ip0, 128 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1032
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1032]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 129 & 0xffff
#if 129 > 0xffff
  movk ip0, 129 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1040
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1040]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 130 & 0xffff
#if 130 > 0xffff
  movk ip0, 130 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1048
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1048]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 131 & 0xffff
#if 131 > 0xffff
  movk ip0, 131 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1056
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1056]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 132 & 0xffff
#if 132 > 0xffff
  movk ip0, 132 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1064
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1064]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 133 & 0xffff
#if 133 > 0xffff
  movk ip0, 133 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1072
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1072]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 134 & 0xffff
#if 134 > 0xffff
  movk ip0, 134 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1080
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1080]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 135 & 0xffff
#if 135 > 0xffff
  movk ip0, 135 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1088
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1088]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 136 & 0xffff
#if 136 > 0xffff
  movk ip0, 136 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1096
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1096]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 137 & 0xffff
#if 137 > 0xffff
  movk ip0, 137 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1104
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1104]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 138 & 0xffff
#if 138 > 0xffff
  movk ip0, 138 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1112
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1112]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 139 & 0xffff
#if 139 > 0xffff
  movk ip0, 139 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1120
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1120]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 140 & 0xffff
#if 140 > 0xffff
  movk ip0, 140 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1128
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1128]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 141 & 0xffff
#if 141 > 0xffff
  movk ip0, 141 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1136
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1136]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 142 & 0xffff
#if 142 > 0xffff
  movk ip0, 142 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1144
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1144]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 143 & 0xffff
#if 143 > 0xffff
  movk ip0, 143 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1152
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1152]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 144 & 0xffff
#if 144 > 0xffff
  movk ip0, 144 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1160
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1160]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 145 & 0xffff
#if 145 > 0xffff
  movk ip0, 145 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1168
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1168]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 146 & 0xffff
#if 146 > 0xffff
  movk ip0, 146 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1176
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1176]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 147 & 0xffff
#if 147 > 0xffff
  movk ip0, 147 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1184
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1184]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 148 & 0xffff
#if 148 > 0xffff
  movk ip0, 148 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1192
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1192]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 149 & 0xffff
#if 149 > 0xffff
  movk ip0, 149 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1200
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1200]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 150 & 0xffff
#if 150 > 0xffff
  movk ip0, 150 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1208
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1208]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 151 & 0xffff
#if 151 > 0xffff
  movk ip0, 151 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1216
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1216]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 152 & 0xffff
#if 152 > 0xffff
  movk ip0, 152 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1224
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1224]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 153 & 0xffff
#if 153 > 0xffff
  movk ip0, 153 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1232
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1232]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 154 & 0xffff
#if 154 > 0xffff
  movk ip0, 154 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1240
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1240]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 155 & 0xffff
#if 155 > 0xffff
  movk ip0, 155 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1248
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1248]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 156 & 0xffff
#if 156 > 0xffff
  movk ip0, 156 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1256
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1256]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 157 & 0xffff
#if 157 > 0xffff
  movk ip0, 157 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1264
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1264]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 158 & 0xffff
#if 158 > 0xffff
  movk ip0, 158 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1272
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1272]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 159 & 0xffff
#if 159 > 0xffff
  movk ip0, 159 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1280
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1280]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 160 & 0xffff
#if 160 > 0xffff
  movk ip0, 160 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1288
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1288]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 161 & 0xffff
#if 161 > 0xffff
  movk ip0, 161 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1296
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1296]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 162 & 0xffff
#if 162 > 0xffff
  movk ip0, 162 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1304
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1304]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 163 & 0xffff
#if 163 > 0xffff
  movk ip0, 163 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1312
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1312]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 164 & 0xffff
#if 164 > 0xffff
  movk ip0, 164 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1320
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1320]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 165 & 0xffff
#if 165 > 0xffff
  movk ip0, 165 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1328
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1328]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 166 & 0xffff
#if 166 > 0xffff
  movk ip0, 166 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1336
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1336]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 167 & 0xffff
#if 167 > 0xffff
  movk ip0, 167 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1344
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1344]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 168 & 0xffff
#if 168 > 0xffff
  movk ip0, 168 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1352
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1352]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 169 & 0xffff
#if 169 > 0xffff
  movk ip0, 169 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1360
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1360]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 170 & 0xffff
#if 170 > 0xffff
  movk ip0, 170 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1368
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1368]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 171 & 0xffff
#if 171 > 0xffff
  movk ip0, 171 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1376
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1376]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 172 & 0xffff
#if 172 > 0xffff
  movk ip0, 172 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1384
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1384]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 173 & 0xffff
#if 173 > 0xffff
  movk ip0, 173 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1392
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1392]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 174 & 0xffff
#if 174 > 0xffff
  movk ip0, 174 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1400
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1400]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 175 & 0xffff
#if 175 > 0xffff
  movk ip0, 175 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1408
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1408]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 176 & 0xffff
#if 176 > 0xffff
  movk ip0, 176 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1416
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1416]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 177 & 0xffff
#if 177 > 0xffff
  movk ip0, 177 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1424
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1424]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 178 & 0xffff
#if 178 > 0xffff
  movk ip0, 178 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1432
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1432]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 179 & 0xffff
#if 179 > 0xffff
  movk ip0, 179 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1440
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1440]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 180 & 0xffff
#if 180 > 0xffff
  movk ip0, 180 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1448
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1448]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 181 & 0xffff
#if 181 > 0xffff
  movk ip0, 181 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1456
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1456]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 182 & 0xffff
#if 182 > 0xffff
  movk ip0, 182 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1464
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1464]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 183 & 0xffff
#if 183 > 0xffff
  movk ip0, 183 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1472
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1472]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 184 & 0xffff
#if 184 > 0xffff
  movk ip0, 184 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1480
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1480]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 185 & 0xffff
#if 185 > 0xffff
  movk ip0, 185 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1488
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1488]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 186 & 0xffff
#if 186 > 0xffff
  movk ip0, 186 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1496
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1496]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 187 & 0xffff
#if 187 > 0xffff
  movk ip0, 187 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1504
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1504]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 188 & 0xffff
#if 188 > 0xffff
  movk ip0, 188 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1512
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1512]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 189 & 0xffff
#if 189 > 0xffff
  movk ip0, 189 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1520
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1520]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 190 & 0xffff
#if 190 > 0xffff
  movk ip0, 190 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1528
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1528]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 191 & 0xffff
#if 191 > 0xffff
  movk ip0, 191 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1536
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1536]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 192 & 0xffff
#if 192 > 0xffff
  movk ip0, 192 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1544
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1544]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 193 & 0xffff
#if 193 > 0xffff
  movk ip0, 193 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1552
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1552]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 194 & 0xffff
#if 194 > 0xffff
  movk ip0, 194 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1560
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1560]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 195 & 0xffff
#if 195 > 0xffff
  movk ip0, 195 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1568
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1568]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 196 & 0xffff
#if 196 > 0xffff
  movk ip0, 196 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1576
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1576]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 197 & 0xffff
#if 197 > 0xffff
  movk ip0, 197 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1584
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1584]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 198 & 0xffff
#if 198 > 0xffff
  movk ip0, 198 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1592
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1592]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 199 & 0xffff
#if 199 > 0xffff
  movk ip0, 199 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1600
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1600]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 200 & 0xffff
#if 200 > 0xffff
  movk ip0, 200 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1608
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1608]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 201 & 0xffff
#if 201 > 0xffff
  movk ip0, 201 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1616
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1616]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 202 & 0xffff
#if 202 > 0xffff
  movk ip0, 202 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1624
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1624]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 203 & 0xffff
#if 203 > 0xffff
  movk ip0, 203 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1632
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1632]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 204 & 0xffff
#if 204 > 0xffff
  movk ip0, 204 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1640
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1640]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 205 & 0xffff
#if 205 > 0xffff
  movk ip0, 205 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1648
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1648]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 206 & 0xffff
#if 206 > 0xffff
  movk ip0, 206 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1656
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1656]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 207 & 0xffff
#if 207 > 0xffff
  movk ip0, 207 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1664
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1664]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 208 & 0xffff
#if 208 > 0xffff
  movk ip0, 208 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1672
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1672]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 209 & 0xffff
#if 209 > 0xffff
  movk ip0, 209 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1680
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1680]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 210 & 0xffff
#if 210 > 0xffff
  movk ip0, 210 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1688
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1688]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 211 & 0xffff
#if 211 > 0xffff
  movk ip0, 211 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1696
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1696]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 212 & 0xffff
#if 212 > 0xffff
  movk ip0, 212 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1704
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1704]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 213 & 0xffff
#if 213 > 0xffff
  movk ip0, 213 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1712
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1712]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 214 & 0xffff
#if 214 > 0xffff
  movk ip0, 214 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1720
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1720]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 215 & 0xffff
#if 215 > 0xffff
  movk ip0, 215 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1728
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1728]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 216 & 0xffff
#if 216 > 0xffff
  movk ip0, 216 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1736
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1736]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 217 & 0xffff
#if 217 > 0xffff
  movk ip0, 217 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1744
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1744]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 218 & 0xffff
#if 218 > 0xffff
  movk ip0, 218 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1752
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1752]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 219 & 0xffff
#if 219 > 0xffff
  movk ip0, 219 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1760
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1760]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 220 & 0xffff
#if 220 > 0xffff
  movk ip0, 220 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1768
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1768]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 221 & 0xffff
#if 221 > 0xffff
  movk ip0, 221 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1776
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1776]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 222 & 0xffff
#if 222 > 0xffff
  movk ip0, 222 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1784
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1784]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 223 & 0xffff
#if 223 > 0xffff
  movk ip0, 223 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1792
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1792]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 224 & 0xffff
#if 224 > 0xffff
  movk ip0, 224 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1800
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1800]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 225 & 0xffff
#if 225 > 0xffff
  movk ip0, 225 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1808
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1808]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 226 & 0xffff
#if 226 > 0xffff
  movk ip0, 226 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1816
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1816]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 227 & 0xffff
#if 227 > 0xffff
  movk ip0, 227 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1824
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1824]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 228 & 0xffff
#if 228 > 0xffff
  movk ip0, 228 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1832
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1832]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 229 & 0xffff
#if 229 > 0xffff
  movk ip0, 229 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1840
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1840]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 230 & 0xffff
#if 230 > 0xffff
  movk ip0, 230 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1848
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1848]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 231 & 0xffff
#if 231 > 0xffff
  movk ip0, 231 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1856
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1856]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 232 & 0xffff
#if 232 > 0xffff
  movk ip0, 232 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1864
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1864]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 233 & 0xffff
#if 233 > 0xffff
  movk ip0, 233 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1872
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1872]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 234 & 0xffff
#if 234 > 0xffff
  movk ip0, 234 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1880
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1880]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 235 & 0xffff
#if 235 > 0xffff
  movk ip0, 235 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1888
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1888]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 236 & 0xffff
#if 236 > 0xffff
  movk ip0, 236 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1896
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1896]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 237 & 0xffff
#if 237 > 0xffff
  movk ip0, 237 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1904
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1904]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 238 & 0xffff
#if 238 > 0xffff
  movk ip0, 238 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1912
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1912]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 239 & 0xffff
#if 239 > 0xffff
  movk ip0, 239 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1920
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1920]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 240 & 0xffff
#if 240 > 0xffff
  movk ip0, 240 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1928
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1928]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 241 & 0xffff
#if 241 > 0xffff
  movk ip0, 241 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1936
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1936]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 242 & 0xffff
#if 242 > 0xffff
  movk ip0, 242 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1944
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1944]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 243 & 0xffff
#if 243 > 0xffff
  movk ip0, 243 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1952
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1952]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 244 & 0xffff
#if 244 > 0xffff
  movk ip0, 244 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1960
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1960]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 245 & 0xffff
#if 245 > 0xffff
  movk ip0, 245 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1968
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1968]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 246 & 0xffff
#if 246 > 0xffff
  movk ip0, 246 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1976
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1976]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 247 & 0xffff
#if 247 > 0xffff
  movk ip0, 247 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1984
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1984]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 248 & 0xffff
#if 248 > 0xffff
  movk ip0, 248 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+1992
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+1992]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 249 & 0xffff
#if 249 > 0xffff
  movk ip0, 249 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2000
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2000]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 250 & 0xffff
#if 250 > 0xffff
  movk ip0, 250 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2008
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2008]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 251 & 0xffff
#if 251 > 0xffff
  movk ip0, 251 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2016
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2016]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 252 & 0xffff
#if 252 > 0xffff
  movk ip0, 252 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2024
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2024]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 253 & 0xffff
#if 253 > 0xffff
  movk ip0, 253 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2032
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2032]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 254 & 0xffff
#if 254 > 0xffff
  movk ip0, 254 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2040
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2040]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 255 & 0xffff
#if 255 > 0xffff
  movk ip0, 255 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2048
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2048]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 256 & 0xffff
#if 256 > 0xffff
  movk ip0, 256 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2056
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2056]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 257 & 0xffff
#if 257 > 0xffff
  movk ip0, 257 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2064
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2064]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 258 & 0xffff
#if 258 > 0xffff
  movk ip0, 258 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2072
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2072]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 259 & 0xffff
#if 259 > 0xffff
  movk ip0, 259 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2080
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2080]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 260 & 0xffff
#if 260 > 0xffff
  movk ip0, 260 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2088
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2088]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 261 & 0xffff
#if 261 > 0xffff
  movk ip0, 261 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2096
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2096]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 262 & 0xffff
#if 262 > 0xffff
  movk ip0, 262 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2104
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2104]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 263 & 0xffff
#if 263 > 0xffff
  movk ip0, 263 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2112
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2112]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 264 & 0xffff
#if 264 > 0xffff
  movk ip0, 264 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2120
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2120]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 265 & 0xffff
#if 265 > 0xffff
  movk ip0, 265 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2128
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2128]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 266 & 0xffff
#if 266 > 0xffff
  movk ip0, 266 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2136
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2136]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 267 & 0xffff
#if 267 > 0xffff
  movk ip0, 267 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2144
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2144]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 268 & 0xffff
#if 268 > 0xffff
  movk ip0, 268 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2152
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2152]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 269 & 0xffff
#if 269 > 0xffff
  movk ip0, 269 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2160
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2160]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 270 & 0xffff
#if 270 > 0xffff
  movk ip0, 270 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2168
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2168]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 271 & 0xffff
#if 271 > 0xffff
  movk ip0, 271 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2176
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2176]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 272 & 0xffff
#if 272 > 0xffff
  movk ip0, 272 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2184
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2184]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 273 & 0xffff
#if 273 > 0xffff
  movk ip0, 273 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2192
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2192]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 274 & 0xffff
#if 274 > 0xffff
  movk ip0, 274 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2200
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2200]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 275 & 0xffff
#if 275 > 0xffff
  movk ip0, 275 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2208
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2208]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 276 & 0xffff
#if 276 > 0xffff
  movk ip0, 276 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2216
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2216]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 277 & 0xffff
#if 277 > 0xffff
  movk ip0, 277 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2224
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2224]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 278 & 0xffff
#if 278 > 0xffff
  movk ip0, 278 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2232
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2232]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 279 & 0xffff
#if 279 > 0xffff
  movk ip0, 279 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2240
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2240]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 280 & 0xffff
#if 280 > 0xffff
  movk ip0, 280 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2248
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2248]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 281 & 0xffff
#if 281 > 0xffff
  movk ip0, 281 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2256
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2256]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 282 & 0xffff
#if 282 > 0xffff
  movk ip0, 282 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2264
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2264]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 283 & 0xffff
#if 283 > 0xffff
  movk ip0, 283 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2272
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2272]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 284 & 0xffff
#if 284 > 0xffff
  movk ip0, 284 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2280
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2280]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 285 & 0xffff
#if 285 > 0xffff
  movk ip0, 285 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2288
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2288]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 286 & 0xffff
#if 286 > 0xffff
  movk ip0, 286 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2296
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2296]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 287 & 0xffff
#if 287 > 0xffff
  movk ip0, 287 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2304
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2304]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 288 & 0xffff
#if 288 > 0xffff
  movk ip0, 288 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2312
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2312]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 289 & 0xffff
#if 289 > 0xffff
  movk ip0, 289 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2320
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2320]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 290 & 0xffff
#if 290 > 0xffff
  movk ip0, 290 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2328
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2328]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 291 & 0xffff
#if 291 > 0xffff
  movk ip0, 291 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2336
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2336]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 292 & 0xffff
#if 292 > 0xffff
  movk ip0, 292 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2344
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2344]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 293 & 0xffff
#if 293 > 0xffff
  movk ip0, 293 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2352
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2352]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 294 & 0xffff
#if 294 > 0xffff
  movk ip0, 294 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2360
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2360]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 295 & 0xffff
#if 295 > 0xffff
  movk ip0, 295 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2368
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2368]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 296 & 0xffff
#if 296 > 0xffff
  movk ip0, 296 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2376
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2376]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 297 & 0xffff
#if 297 > 0xffff
  movk ip0, 297 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2384
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2384]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 298 & 0xffff
#if 298 > 0xffff
  movk ip0, 298 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2392
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2392]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 299 & 0xffff
#if 299 > 0xffff
  movk ip0, 299 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2400
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2400]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 300 & 0xffff
#if 300 > 0xffff
  movk ip0, 300 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2408
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2408]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 301 & 0xffff
#if 301 > 0xffff
  movk ip0, 301 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2416
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2416]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 302 & 0xffff
#if 302 > 0xffff
  movk ip0, 302 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2424
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2424]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 303 & 0xffff
#if 303 > 0xffff
  movk ip0, 303 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2432
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2432]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 304 & 0xffff
#if 304 > 0xffff
  movk ip0, 304 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2440
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2440]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 305 & 0xffff
#if 305 > 0xffff
  movk ip0, 305 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2448
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2448]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 306 & 0xffff
#if 306 > 0xffff
  movk ip0, 306 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2456
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2456]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 307 & 0xffff
#if 307 > 0xffff
  movk ip0, 307 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2464
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2464]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 308 & 0xffff
#if 308 > 0xffff
  movk ip0, 308 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2472
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2472]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 309 & 0xffff
#if 309 > 0xffff
  movk ip0, 309 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2480
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2480]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 310 & 0xffff
#if 310 > 0xffff
  movk ip0, 310 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2488
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2488]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 311 & 0xffff
#if 311 > 0xffff
  movk ip0, 311 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2496
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2496]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 312 & 0xffff
#if 312 > 0xffff
  movk ip0, 312 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2504
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2504]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 313 & 0xffff
#if 313 > 0xffff
  movk ip0, 313 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2512
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2512]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 314 & 0xffff
#if 314 > 0xffff
  movk ip0, 314 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2520
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2520]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 315 & 0xffff
#if 315 > 0xffff
  movk ip0, 315 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2528
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2528]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 316 & 0xffff
#if 316 > 0xffff
  movk ip0, 316 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2536
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2536]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 317 & 0xffff
#if 317 > 0xffff
  movk ip0, 317 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2544
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2544]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 318 & 0xffff
#if 318 > 0xffff
  movk ip0, 318 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2552
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2552]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 319 & 0xffff
#if 319 > 0xffff
  movk ip0, 319 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2560
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2560]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 320 & 0xffff
#if 320 > 0xffff
  movk ip0, 320 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2568
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2568]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 321 & 0xffff
#if 321 > 0xffff
  movk ip0, 321 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2576
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2576]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 322 & 0xffff
#if 322 > 0xffff
  movk ip0, 322 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2584
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2584]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 323 & 0xffff
#if 323 > 0xffff
  movk ip0, 323 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2592
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2592]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 324 & 0xffff
#if 324 > 0xffff
  movk ip0, 324 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2600
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2600]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 325 & 0xffff
#if 325 > 0xffff
  movk ip0, 325 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2608
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2608]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 326 & 0xffff
#if 326 > 0xffff
  movk ip0, 326 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2616
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2616]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 327 & 0xffff
#if 327 > 0xffff
  movk ip0, 327 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2624
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2624]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 328 & 0xffff
#if 328 > 0xffff
  movk ip0, 328 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2632
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2632]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 329 & 0xffff
#if 329 > 0xffff
  movk ip0, 329 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2640
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2640]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 330 & 0xffff
#if 330 > 0xffff
  movk ip0, 330 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2648
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2648]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 331 & 0xffff
#if 331 > 0xffff
  movk ip0, 331 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2656
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2656]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 332 & 0xffff
#if 332 > 0xffff
  movk ip0, 332 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2664
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2664]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 333 & 0xffff
#if 333 > 0xffff
  movk ip0, 333 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2672
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2672]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 334 & 0xffff
#if 334 > 0xffff
  movk ip0, 334 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2680
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2680]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 335 & 0xffff
#if 335 > 0xffff
  movk ip0, 335 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2688
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2688]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 336 & 0xffff
#if 336 > 0xffff
  movk ip0, 336 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2696
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2696]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 337 & 0xffff
#if 337 > 0xffff
  movk ip0, 337 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2704
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2704]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 338 & 0xffff
#if 338 > 0xffff
  movk ip0, 338 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2712
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2712]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 339 & 0xffff
#if 339 > 0xffff
  movk ip0, 339 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2720
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2720]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 340 & 0xffff
#if 340 > 0xffff
  movk ip0, 340 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2728
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2728]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 341 & 0xffff
#if 341 > 0xffff
  movk ip0, 341 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2736
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2736]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 342 & 0xffff
#if 342 > 0xffff
  movk ip0, 342 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2744
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2744]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 343 & 0xffff
#if 343 > 0xffff
  movk ip0, 343 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2752
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2752]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 344 & 0xffff
#if 344 > 0xffff
  movk ip0, 344 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2760
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2760]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 345 & 0xffff
#if 345 > 0xffff
  movk ip0, 345 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2768
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2768]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 346 & 0xffff
#if 346 > 0xffff
  movk ip0, 346 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2776
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2776]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 347 & 0xffff
#if 347 > 0xffff
  movk ip0, 347 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2784
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2784]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 348 & 0xffff
#if 348 > 0xffff
  movk ip0, 348 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2792
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2792]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 349 & 0xffff
#if 349 > 0xffff
  movk ip0, 349 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2800
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2800]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 350 & 0xffff
#if 350 > 0xffff
  movk ip0, 350 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2808
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2808]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 351 & 0xffff
#if 351 > 0xffff
  movk ip0, 351 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2816
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2816]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 352 & 0xffff
#if 352 > 0xffff
  movk ip0, 352 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2824
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2824]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 353 & 0xffff
#if 353 > 0xffff
  movk ip0, 353 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2832
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2832]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 354 & 0xffff
#if 354 > 0xffff
  movk ip0, 354 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2840
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2840]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 355 & 0xffff
#if 355 > 0xffff
  movk ip0, 355 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2848
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2848]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 356 & 0xffff
#if 356 > 0xffff
  movk ip0, 356 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2856
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2856]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 357 & 0xffff
#if 357 > 0xffff
  movk ip0, 357 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2864
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2864]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 358 & 0xffff
#if 358 > 0xffff
  movk ip0, 358 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2872
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2872]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 359 & 0xffff
#if 359 > 0xffff
  movk ip0, 359 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2880
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2880]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 360 & 0xffff
#if 360 > 0xffff
  movk ip0, 360 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2888
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2888]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 361 & 0xffff
#if 361 > 0xffff
  movk ip0, 361 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2896
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2896]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 362 & 0xffff
#if 362 > 0xffff
  movk ip0, 362 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2904
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2904]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 363 & 0xffff
#if 363 > 0xffff
  movk ip0, 363 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2912
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2912]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 364 & 0xffff
#if 364 > 0xffff
  movk ip0, 364 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2920
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2920]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 365 & 0xffff
#if 365 > 0xffff
  movk ip0, 365 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2928
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2928]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 366 & 0xffff
#if 366 > 0xffff
  movk ip0, 366 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2936
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2936]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 367 & 0xffff
#if 367 > 0xffff
  movk ip0, 367 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2944
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2944]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 368 & 0xffff
#if 368 > 0xffff
  movk ip0, 368 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2952
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2952]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 369 & 0xffff
#if 369 > 0xffff
  movk ip0, 369 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2960
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2960]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 370 & 0xffff
#if 370 > 0xffff
  movk ip0, 370 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2968
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2968]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 371 & 0xffff
#if 371 > 0xffff
  movk ip0, 371 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2976
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2976]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 372 & 0xffff
#if 372 > 0xffff
  movk ip0, 372 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2984
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2984]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 373 & 0xffff
#if 373 > 0xffff
  movk ip0, 373 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+2992
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+2992]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 374 & 0xffff
#if 374 > 0xffff
  movk ip0, 374 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3000
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3000]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 375 & 0xffff
#if 375 > 0xffff
  movk ip0, 375 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3008
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3008]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 376 & 0xffff
#if 376 > 0xffff
  movk ip0, 376 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3016
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3016]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 377 & 0xffff
#if 377 > 0xffff
  movk ip0, 377 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3024
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3024]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 378 & 0xffff
#if 378 > 0xffff
  movk ip0, 378 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3032
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3032]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 379 & 0xffff
#if 379 > 0xffff
  movk ip0, 379 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3040
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3040]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 380 & 0xffff
#if 380 > 0xffff
  movk ip0, 380 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3048
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3048]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 381 & 0xffff
#if 381 > 0xffff
  movk ip0, 381 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3056
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3056]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 382 & 0xffff
#if 382 > 0xffff
  movk ip0, 382 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3064
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3064]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 383 & 0xffff
#if 383 > 0xffff
  movk ip0, 383 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3072
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3072]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 384 & 0xffff
#if 384 > 0xffff
  movk ip0, 384 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3080
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3080]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 385 & 0xffff
#if 385 > 0xffff
  movk ip0, 385 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3088
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3088]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 386 & 0xffff
#if 386 > 0xffff
  movk ip0, 386 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3096
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3096]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 387 & 0xffff
#if 387 > 0xffff
  movk ip0, 387 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3104
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3104]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 388 & 0xffff
#if 388 > 0xffff
  movk ip0, 388 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3112
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3112]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 389 & 0xffff
#if 389 > 0xffff
  movk ip0, 389 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3120
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3120]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 390 & 0xffff
#if 390 > 0xffff
  movk ip0, 390 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3128
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3128]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 391 & 0xffff
#if 391 > 0xffff
  movk ip0, 391 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3136
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3136]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 392 & 0xffff
#if 392 > 0xffff
  movk ip0, 392 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3144
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3144]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 393 & 0xffff
#if 393 > 0xffff
  movk ip0, 393 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3152
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3152]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 394 & 0xffff
#if 394 > 0xffff
  movk ip0, 394 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3160
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3160]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 395 & 0xffff
#if 395 > 0xffff
  movk ip0, 395 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3168
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3168]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 396 & 0xffff
#if 396 > 0xffff
  movk ip0, 396 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3176
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3176]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 397 & 0xffff
#if 397 > 0xffff
  movk ip0, 397 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3184
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3184]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 398 & 0xffff
#if 398 > 0xffff
  movk ip0, 398 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3192
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3192]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 399 & 0xffff
#if 399 > 0xffff
  movk ip0, 399 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3200
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3200]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 400 & 0xffff
#if 400 > 0xffff
  movk ip0, 400 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3208
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3208]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 401 & 0xffff
#if 401 > 0xffff
  movk ip0, 401 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3216
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3216]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 402 & 0xffff
#if 402 > 0xffff
  movk ip0, 402 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3224
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3224]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 403 & 0xffff
#if 403 > 0xffff
  movk ip0, 403 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3232
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3232]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 404 & 0xffff
#if 404 > 0xffff
  movk ip0, 404 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3240
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3240]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 405 & 0xffff
#if 405 > 0xffff
  movk ip0, 405 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3248
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3248]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 406 & 0xffff
#if 406 > 0xffff
  movk ip0, 406 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3256
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3256]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 407 & 0xffff
#if 407 > 0xffff
  movk ip0, 407 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3264
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3264]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 408 & 0xffff
#if 408 > 0xffff
  movk ip0, 408 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3272
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3272]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 409 & 0xffff
#if 409 > 0xffff
  movk ip0, 409 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3280
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3280]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 410 & 0xffff
#if 410 > 0xffff
  movk ip0, 410 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3288
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3288]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 411 & 0xffff
#if 411 > 0xffff
  movk ip0, 411 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3296
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3296]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 412 & 0xffff
#if 412 > 0xffff
  movk ip0, 412 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3304
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3304]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 413 & 0xffff
#if 413 > 0xffff
  movk ip0, 413 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3312
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3312]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 414 & 0xffff
#if 414 > 0xffff
  movk ip0, 414 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3320
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3320]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 415 & 0xffff
#if 415 > 0xffff
  movk ip0, 415 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3328
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3328]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 416 & 0xffff
#if 416 > 0xffff
  movk ip0, 416 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3336
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3336]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 417 & 0xffff
#if 417 > 0xffff
  movk ip0, 417 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3344
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3344]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 418 & 0xffff
#if 418 > 0xffff
  movk ip0, 418 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3352
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3352]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 419 & 0xffff
#if 419 > 0xffff
  movk ip0, 419 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3360
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3360]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 420 & 0xffff
#if 420 > 0xffff
  movk ip0, 420 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3368
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3368]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 421 & 0xffff
#if 421 > 0xffff
  movk ip0, 421 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3376
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3376]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 422 & 0xffff
#if 422 > 0xffff
  movk ip0, 422 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3384
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3384]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 423 & 0xffff
#if 423 > 0xffff
  movk ip0, 423 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3392
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3392]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 424 & 0xffff
#if 424 > 0xffff
  movk ip0, 424 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3400
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3400]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 425 & 0xffff
#if 425 > 0xffff
  movk ip0, 425 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3408
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3408]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 426 & 0xffff
#if 426 > 0xffff
  movk ip0, 426 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3416
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3416]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 427 & 0xffff
#if 427 > 0xffff
  movk ip0, 427 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3424
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3424]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 428 & 0xffff
#if 428 > 0xffff
  movk ip0, 428 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3432
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3432]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 429 & 0xffff
#if 429 > 0xffff
  movk ip0, 429 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3440
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3440]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 430 & 0xffff
#if 430 > 0xffff
  movk ip0, 430 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3448
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3448]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 431 & 0xffff
#if 431 > 0xffff
  movk ip0, 431 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3456
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3456]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 432 & 0xffff
#if 432 > 0xffff
  movk ip0, 432 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3464
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3464]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 433 & 0xffff
#if 433 > 0xffff
  movk ip0, 433 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3472
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3472]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 434 & 0xffff
#if 434 > 0xffff
  movk ip0, 434 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3480
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3480]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 435 & 0xffff
#if 435 > 0xffff
  movk ip0, 435 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3488
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3488]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 436 & 0xffff
#if 436 > 0xffff
  movk ip0, 436 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3496
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3496]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 437 & 0xffff
#if 437 > 0xffff
  movk ip0, 437 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3504
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3504]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 438 & 0xffff
#if 438 > 0xffff
  movk ip0, 438 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3512
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3512]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 439 & 0xffff
#if 439 > 0xffff
  movk ip0, 439 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3520
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3520]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 440 & 0xffff
#if 440 > 0xffff
  movk ip0, 440 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3528
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3528]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 441 & 0xffff
#if 441 > 0xffff
  movk ip0, 441 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3536
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3536]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 442 & 0xffff
#if 442 > 0xffff
  movk ip0, 442 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3544
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3544]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 443 & 0xffff
#if 443 > 0xffff
  movk ip0, 443 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3552
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3552]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 444 & 0xffff
#if 444 > 0xffff
  movk ip0, 444 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3560
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3560]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 445 & 0xffff
#if 445 > 0xffff
  movk ip0, 445 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3568
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3568]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 446 & 0xffff
#if 446 > 0xffff
  movk ip0, 446 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3576
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3576]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 447 & 0xffff
#if 447 > 0xffff
  movk ip0, 447 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3584
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3584]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 448 & 0xffff
#if 448 > 0xffff
  movk ip0, 448 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3592
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3592]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 449 & 0xffff
#if 449 > 0xffff
  movk ip0, 449 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3600
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3600]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 450 & 0xffff
#if 450 > 0xffff
  movk ip0, 450 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3608
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3608]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 451 & 0xffff
#if 451 > 0xffff
  movk ip0, 451 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3616
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3616]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 452 & 0xffff
#if 452 > 0xffff
  movk ip0, 452 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3624
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3624]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 453 & 0xffff
#if 453 > 0xffff
  movk ip0, 453 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3632
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3632]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 454 & 0xffff
#if 454 > 0xffff
  movk ip0, 454 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3640
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3640]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 455 & 0xffff
#if 455 > 0xffff
  movk ip0, 455 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3648
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3648]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 456 & 0xffff
#if 456 > 0xffff
  movk ip0, 456 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3656
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3656]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 457 & 0xffff
#if 457 > 0xffff
  movk ip0, 457 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3664
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3664]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 458 & 0xffff
#if 458 > 0xffff
  movk ip0, 458 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3672
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3672]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 459 & 0xffff
#if 459 > 0xffff
  movk ip0, 459 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3680
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3680]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 460 & 0xffff
#if 460 > 0xffff
  movk ip0, 460 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3688
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3688]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 461 & 0xffff
#if 461 > 0xffff
  movk ip0, 461 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3696
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3696]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 462 & 0xffff
#if 462 > 0xffff
  movk ip0, 462 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3704
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3704]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 463 & 0xffff
#if 463 > 0xffff
  movk ip0, 463 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3712
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3712]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 464 & 0xffff
#if 464 > 0xffff
  movk ip0, 464 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3720
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3720]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 465 & 0xffff
#if 465 > 0xffff
  movk ip0, 465 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3728
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3728]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 466 & 0xffff
#if 466 > 0xffff
  movk ip0, 466 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3736
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3736]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 467 & 0xffff
#if 467 > 0xffff
  movk ip0, 467 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3744
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3744]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 468 & 0xffff
#if 468 > 0xffff
  movk ip0, 468 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3752
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3752]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 469 & 0xffff
#if 469 > 0xffff
  movk ip0, 469 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3760
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3760]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 470 & 0xffff
#if 470 > 0xffff
  movk ip0, 470 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3768
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3768]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 471 & 0xffff
#if 471 > 0xffff
  movk ip0, 471 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3776
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3776]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 472 & 0xffff
#if 472 > 0xffff
  movk ip0, 472 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3784
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3784]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 473 & 0xffff
#if 473 > 0xffff
  movk ip0, 473 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3792
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3792]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 474 & 0xffff
#if 474 > 0xffff
  movk ip0, 474 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3800
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3800]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 475 & 0xffff
#if 475 > 0xffff
  movk ip0, 475 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3808
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3808]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 476 & 0xffff
#if 476 > 0xffff
  movk ip0, 476 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3816
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3816]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 477 & 0xffff
#if 477 > 0xffff
  movk ip0, 477 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3824
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3824]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 478 & 0xffff
#if 478 > 0xffff
  movk ip0, 478 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3832
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3832]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 479 & 0xffff
#if 479 > 0xffff
  movk ip0, 479 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3840
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3840]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 480 & 0xffff
#if 480 > 0xffff
  movk ip0, 480 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3848
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3848]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 481 & 0xffff
#if 481 > 0xffff
  movk ip0, 481 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3856
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3856]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 482 & 0xffff
#if 482 > 0xffff
  movk ip0, 482 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3864
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3864]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 483 & 0xffff
#if 483 > 0xffff
  movk ip0, 483 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3872
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3872]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 484 & 0xffff
#if 484 > 0xffff
  movk ip0, 484 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3880
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3880]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 485 & 0xffff
#if 485 > 0xffff
  movk ip0, 485 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3888
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3888]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 486 & 0xffff
#if 486 > 0xffff
  movk ip0, 486 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3896
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3896]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 487 & 0xffff
#if 487 > 0xffff
  movk ip0, 487 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3904
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3904]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 488 & 0xffff
#if 488 > 0xffff
  movk ip0, 488 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3912
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3912]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 489 & 0xffff
#if 489 > 0xffff
  movk ip0, 489 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3920
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3920]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 490 & 0xffff
#if 490 > 0xffff
  movk ip0, 490 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3928
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3928]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 491 & 0xffff
#if 491 > 0xffff
  movk ip0, 491 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3936
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3936]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 492 & 0xffff
#if 492 > 0xffff
  movk ip0, 492 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3944
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3944]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 493 & 0xffff
#if 493 > 0xffff
  movk ip0, 493 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3952
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3952]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 494 & 0xffff
#if 494 > 0xffff
  movk ip0, 494 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3960
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3960]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 495 & 0xffff
#if 495 > 0xffff
  movk ip0, 495 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3968
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3968]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 496 & 0xffff
#if 496 > 0xffff
  movk ip0, 496 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3976
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3976]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 497 & 0xffff
#if 497 > 0xffff
  movk ip0, 497 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3984
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3984]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 498 & 0xffff
#if 498 > 0xffff
  movk ip0, 498 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+3992
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+3992]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 499 & 0xffff
#if 499 > 0xffff
  movk ip0, 499 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4000
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4000]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 500 & 0xffff
#if 500 > 0xffff
  movk ip0, 500 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4008
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4008]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 501 & 0xffff
#if 501 > 0xffff
  movk ip0, 501 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4016
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4016]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 502 & 0xffff
#if 502 > 0xffff
  movk ip0, 502 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4024
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4024]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 503 & 0xffff
#if 503 > 0xffff
  movk ip0, 503 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4032
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4032]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 504 & 0xffff
#if 504 > 0xffff
  movk ip0, 504 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4040
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4040]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 505 & 0xffff
#if 505 > 0xffff
  movk ip0, 505 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4048
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4048]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 506 & 0xffff
#if 506 > 0xffff
  movk ip0, 506 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4056
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4056]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 507 & 0xffff
#if 507 > 0xffff
  movk ip0, 507 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4064
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4064]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 508 & 0xffff
#if 508 > 0xffff
  movk ip0, 508 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4072
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4072]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 509 & 0xffff
#if 509 > 0xffff
  movk ip0, 509 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4080
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4080]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 510 & 0xffff
#if 510 > 0xffff
  movk ip0, 510 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4088
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4088]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 511 & 0xffff
#if 511 > 0xffff
  movk ip0, 511 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4096
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4096]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 512 & 0xffff
#if 512 > 0xffff
  movk ip0, 512 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4104
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4104]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 513 & 0xffff
#if 513 > 0xffff
  movk ip0, 513 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4112
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4112]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 514 & 0xffff
#if 514 > 0xffff
  movk ip0, 514 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4120
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4120]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 515 & 0xffff
#if 515 > 0xffff
  movk ip0, 515 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4128
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4128]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 516 & 0xffff
#if 516 > 0xffff
  movk ip0, 516 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4136
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4136]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 517 & 0xffff
#if 517 > 0xffff
  movk ip0, 517 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4144
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4144]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 518 & 0xffff
#if 518 > 0xffff
  movk ip0, 518 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4152
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4152]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 519 & 0xffff
#if 519 > 0xffff
  movk ip0, 519 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4160
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4160]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 520 & 0xffff
#if 520 > 0xffff
  movk ip0, 520 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4168
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4168]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 521 & 0xffff
#if 521 > 0xffff
  movk ip0, 521 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4176
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4176]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 522 & 0xffff
#if 522 > 0xffff
  movk ip0, 522 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4184
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4184]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 523 & 0xffff
#if 523 > 0xffff
  movk ip0, 523 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4192
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4192]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 524 & 0xffff
#if 524 > 0xffff
  movk ip0, 524 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4200
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4200]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 525 & 0xffff
#if 525 > 0xffff
  movk ip0, 525 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4208
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4208]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 526 & 0xffff
#if 526 > 0xffff
  movk ip0, 526 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4216
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4216]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 527 & 0xffff
#if 527 > 0xffff
  movk ip0, 527 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4224
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4224]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 528 & 0xffff
#if 528 > 0xffff
  movk ip0, 528 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4232
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4232]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 529 & 0xffff
#if 529 > 0xffff
  movk ip0, 529 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4240
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4240]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 530 & 0xffff
#if 530 > 0xffff
  movk ip0, 530 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4248
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4248]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 531 & 0xffff
#if 531 > 0xffff
  movk ip0, 531 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4256
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4256]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 532 & 0xffff
#if 532 > 0xffff
  movk ip0, 532 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4264
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4264]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 533 & 0xffff
#if 533 > 0xffff
  movk ip0, 533 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4272
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4272]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 534 & 0xffff
#if 534 > 0xffff
  movk ip0, 534 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4280
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4280]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 535 & 0xffff
#if 535 > 0xffff
  movk ip0, 535 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4288
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4288]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 536 & 0xffff
#if 536 > 0xffff
  movk ip0, 536 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4296
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4296]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 537 & 0xffff
#if 537 > 0xffff
  movk ip0, 537 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4304
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4304]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 538 & 0xffff
#if 538 > 0xffff
  movk ip0, 538 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4312
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4312]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 539 & 0xffff
#if 539 > 0xffff
  movk ip0, 539 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4320
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4320]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 540 & 0xffff
#if 540 > 0xffff
  movk ip0, 540 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4328
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4328]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 541 & 0xffff
#if 541 > 0xffff
  movk ip0, 541 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4336
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4336]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 542 & 0xffff
#if 542 > 0xffff
  movk ip0, 542 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4344
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4344]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 543 & 0xffff
#if 543 > 0xffff
  movk ip0, 543 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4352
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4352]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 544 & 0xffff
#if 544 > 0xffff
  movk ip0, 544 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4360
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4360]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 545 & 0xffff
#if 545 > 0xffff
  movk ip0, 545 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4368
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4368]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 546 & 0xffff
#if 546 > 0xffff
  movk ip0, 546 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4376
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4376]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 547 & 0xffff
#if 547 > 0xffff
  movk ip0, 547 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4384
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4384]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 548 & 0xffff
#if 548 > 0xffff
  movk ip0, 548 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4392
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4392]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 549 & 0xffff
#if 549 > 0xffff
  movk ip0, 549 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4400
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4400]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 550 & 0xffff
#if 550 > 0xffff
  movk ip0, 550 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4408
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4408]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 551 & 0xffff
#if 551 > 0xffff
  movk ip0, 551 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4416
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4416]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 552 & 0xffff
#if 552 > 0xffff
  movk ip0, 552 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4424
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4424]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 553 & 0xffff
#if 553 > 0xffff
  movk ip0, 553 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4432
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4432]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 554 & 0xffff
#if 554 > 0xffff
  movk ip0, 554 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4440
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4440]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 555 & 0xffff
#if 555 > 0xffff
  movk ip0, 555 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4448
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4448]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 556 & 0xffff
#if 556 > 0xffff
  movk ip0, 556 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4456
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4456]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 557 & 0xffff
#if 557 > 0xffff
  movk ip0, 557 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4464
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4464]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 558 & 0xffff
#if 558 > 0xffff
  movk ip0, 558 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4472
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4472]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 559 & 0xffff
#if 559 > 0xffff
  movk ip0, 559 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4480
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4480]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 560 & 0xffff
#if 560 > 0xffff
  movk ip0, 560 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4488
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4488]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 561 & 0xffff
#if 561 > 0xffff
  movk ip0, 561 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4496
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4496]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 562 & 0xffff
#if 562 > 0xffff
  movk ip0, 562 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4504
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4504]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 563 & 0xffff
#if 563 > 0xffff
  movk ip0, 563 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4512
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4512]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 564 & 0xffff
#if 564 > 0xffff
  movk ip0, 564 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4520
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4520]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 565 & 0xffff
#if 565 > 0xffff
  movk ip0, 565 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4528
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4528]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 566 & 0xffff
#if 566 > 0xffff
  movk ip0, 566 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4536
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4536]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 567 & 0xffff
#if 567 > 0xffff
  movk ip0, 567 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4544
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4544]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 568 & 0xffff
#if 568 > 0xffff
  movk ip0, 568 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4552
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4552]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 569 & 0xffff
#if 569 > 0xffff
  movk ip0, 569 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4560
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4560]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 570 & 0xffff
#if 570 > 0xffff
  movk ip0, 570 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4568
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4568]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 571 & 0xffff
#if 571 > 0xffff
  movk ip0, 571 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4576
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4576]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 572 & 0xffff
#if 572 > 0xffff
  movk ip0, 572 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4584
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4584]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 573 & 0xffff
#if 573 > 0xffff
  movk ip0, 573 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4592
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4592]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 574 & 0xffff
#if 574 > 0xffff
  movk ip0, 574 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4600
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4600]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 575 & 0xffff
#if 575 > 0xffff
  movk ip0, 575 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4608
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4608]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 576 & 0xffff
#if 576 > 0xffff
  movk ip0, 576 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4616
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4616]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 577 & 0xffff
#if 577 > 0xffff
  movk ip0, 577 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4624
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4624]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 578 & 0xffff
#if 578 > 0xffff
  movk ip0, 578 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4632
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4632]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 579 & 0xffff
#if 579 > 0xffff
  movk ip0, 579 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4640
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4640]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 580 & 0xffff
#if 580 > 0xffff
  movk ip0, 580 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4648
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4648]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 581 & 0xffff
#if 581 > 0xffff
  movk ip0, 581 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4656
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4656]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 582 & 0xffff
#if 582 > 0xffff
  movk ip0, 582 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4664
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4664]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 583 & 0xffff
#if 583 > 0xffff
  movk ip0, 583 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4672
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4672]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 584 & 0xffff
#if 584 > 0xffff
  movk ip0, 584 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4680
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4680]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 585 & 0xffff
#if 585 > 0xffff
  movk ip0, 585 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4688
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4688]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 586 & 0xffff
#if 586 > 0xffff
  movk ip0, 586 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4696
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4696]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 587 & 0xffff
#if 587 > 0xffff
  movk ip0, 587 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4704
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4704]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 588 & 0xffff
#if 588 > 0xffff
  movk ip0, 588 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4712
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4712]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 589 & 0xffff
#if 589 > 0xffff
  movk ip0, 589 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4720
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4720]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 590 & 0xffff
#if 590 > 0xffff
  movk ip0, 590 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4728
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4728]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 591 & 0xffff
#if 591 > 0xffff
  movk ip0, 591 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4736
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4736]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 592 & 0xffff
#if 592 > 0xffff
  movk ip0, 592 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4744
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4744]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 593 & 0xffff
#if 593 > 0xffff
  movk ip0, 593 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4752
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4752]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 594 & 0xffff
#if 594 > 0xffff
  movk ip0, 594 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4760
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4760]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 595 & 0xffff
#if 595 > 0xffff
  movk ip0, 595 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4768
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4768]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 596 & 0xffff
#if 596 > 0xffff
  movk ip0, 596 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4776
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4776]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 597 & 0xffff
#if 597 > 0xffff
  movk ip0, 597 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4784
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4784]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 598 & 0xffff
#if 598 > 0xffff
  movk ip0, 598 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4792
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4792]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 599 & 0xffff
#if 599 > 0xffff
  movk ip0, 599 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4800
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4800]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 600 & 0xffff
#if 600 > 0xffff
  movk ip0, 600 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4808
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4808]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 601 & 0xffff
#if 601 > 0xffff
  movk ip0, 601 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4816
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4816]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 602 & 0xffff
#if 602 > 0xffff
  movk ip0, 602 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4824
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4824]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 603 & 0xffff
#if 603 > 0xffff
  movk ip0, 603 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4832
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4832]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 604 & 0xffff
#if 604 > 0xffff
  movk ip0, 604 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4840
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4840]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 605 & 0xffff
#if 605 > 0xffff
  movk ip0, 605 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4848
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4848]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 606 & 0xffff
#if 606 > 0xffff
  movk ip0, 606 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4856
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4856]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 607 & 0xffff
#if 607 > 0xffff
  movk ip0, 607 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4864
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4864]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 608 & 0xffff
#if 608 > 0xffff
  movk ip0, 608 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4872
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4872]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 609 & 0xffff
#if 609 > 0xffff
  movk ip0, 609 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4880
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4880]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 610 & 0xffff
#if 610 > 0xffff
  movk ip0, 610 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4888
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4888]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 611 & 0xffff
#if 611 > 0xffff
  movk ip0, 611 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4896
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4896]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 612 & 0xffff
#if 612 > 0xffff
  movk ip0, 612 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4904
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4904]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 613 & 0xffff
#if 613 > 0xffff
  movk ip0, 613 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4912
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4912]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 614 & 0xffff
#if 614 > 0xffff
  movk ip0, 614 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4920
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4920]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 615 & 0xffff
#if 615 > 0xffff
  movk ip0, 615 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4928
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4928]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 616 & 0xffff
#if 616 > 0xffff
  movk ip0, 616 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4936
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4936]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 617 & 0xffff
#if 617 > 0xffff
  movk ip0, 617 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4944
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4944]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 618 & 0xffff
#if 618 > 0xffff
  movk ip0, 618 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4952
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4952]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 619 & 0xffff
#if 619 > 0xffff
  movk ip0, 619 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4960
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4960]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 620 & 0xffff
#if 620 > 0xffff
  movk ip0, 620 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4968
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4968]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 621 & 0xffff
#if 621 > 0xffff
  movk ip0, 621 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4976
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4976]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 622 & 0xffff
#if 622 > 0xffff
  movk ip0, 622 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4984
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4984]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 623 & 0xffff
#if 623 > 0xffff
  movk ip0, 623 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+4992
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+4992]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 624 & 0xffff
#if 624 > 0xffff
  movk ip0, 624 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+5000
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+5000]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 625 & 0xffff
#if 625 > 0xffff
  movk ip0, 625 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+5008
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+5008]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 626 & 0xffff
#if 626 > 0xffff
  movk ip0, 626 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+5016
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+5016]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 627 & 0xffff
#if 627 > 0xffff
  movk ip0, 627 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+5024
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+5024]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 628 & 0xffff
#if 628 > 0xffff
  movk ip0, 628 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+5032
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+5032]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 629 & 0xffff
#if 629 > 0xffff
  movk ip0, 629 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+5040
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+5040]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 630 & 0xffff
#if 630 > 0xffff
  movk ip0, 630 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+5048
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+5048]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 631 & 0xffff
#if 631 > 0xffff
  movk ip0, 631 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+5056
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+5056]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 632 & 0xffff
#if 632 > 0xffff
  movk ip0, 632 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+5064
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+5064]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 633 & 0xffff
#if 633 > 0xffff
  movk ip0, 633 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+5072
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+5072]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 634 & 0xffff
#if 634 > 0xffff
  movk ip0, 634 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+5080
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+5080]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 635 & 0xffff
#if 635 > 0xffff
  movk ip0, 635 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+5088
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+5088]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 636 & 0xffff
#if 636 > 0xffff
  movk ip0, 636 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+5096
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+5096]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 637 & 0xffff
#if 637 > 0xffff
  movk ip0, 637 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+5104
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+5104]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 638 & 0xffff
#if 638 > 0xffff
  movk ip0, 638 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+5112
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+5112]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 639 & 0xffff
#if 639 > 0xffff
  movk ip0, 639 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+5120
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+5120]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 640 & 0xffff
#if 640 > 0xffff
  movk ip0, 640 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+5128
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+5128]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 641 & 0xffff
#if 641 > 0xffff
  movk ip0, 641 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+5136
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+5136]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 642 & 0xffff
#if 642 > 0xffff
  movk ip0, 642 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+5144
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+5144]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 643 & 0xffff
#if 643 > 0xffff
  movk ip0, 643 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+5152
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+5152]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 644 & 0xffff
#if 644 > 0xffff
  movk ip0, 644 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+5160
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+5160]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 645 & 0xffff
#if 645 > 0xffff
  movk ip0, 645 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+5168
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+5168]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 646 & 0xffff
#if 646 > 0xffff
  movk ip0, 646 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+5176
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+5176]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 647 & 0xffff
#if 647 > 0xffff
  movk ip0, 647 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+5184
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+5184]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 648 & 0xffff
#if 648 > 0xffff
  movk ip0, 648 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+5192
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+5192]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 649 & 0xffff
#if 649 > 0xffff
  movk ip0, 649 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+5200
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+5200]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 650 & 0xffff
#if 650 > 0xffff
  movk ip0, 650 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+5208
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+5208]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 651 & 0xffff
#if 651 > 0xffff
  movk ip0, 651 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+5216
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+5216]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 652 & 0xffff
#if 652 > 0xffff
  movk ip0, 652 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+5224
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+5224]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 653 & 0xffff
#if 653 > 0xffff
  movk ip0, 653 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+5232
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+5232]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 654 & 0xffff
#if 654 > 0xffff
  movk ip0, 654 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+5240
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+5240]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 655 & 0xffff
#if 655 > 0xffff
  movk ip0, 655 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+5248
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+5248]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 656 & 0xffff
#if 656 > 0xffff
  movk ip0, 656 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+5256
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+5256]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 657 & 0xffff
#if 657 > 0xffff
  movk ip0, 657 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libcuda_so_tramp_table+5264
  ldr ip0, [ip0, #:lo12:_libcuda_so_tramp_table+5264]
 
  cbz ip0, 2f

  // Fast path
  br ip0

2:
  // Slow path
  mov ip0, 658 & 0xffff
#if 658 > 0xffff
  movk ip0, 658 >> 16, lsl #16
#endif
  stp ip0, lr, [sp, #-16]!; .cfi_adjust_cfa_offset 16; .cfi_rel_offset lr, 8
  bl _libcuda_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

