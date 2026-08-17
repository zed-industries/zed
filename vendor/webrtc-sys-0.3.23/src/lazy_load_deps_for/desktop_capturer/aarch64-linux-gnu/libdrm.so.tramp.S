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

  .globl _libdrm_so_tramp_table
  .hidden _libdrm_so_tramp_table
  .align 8
_libdrm_so_tramp_table:
  .zero 1704

  .text

  .globl _libdrm_so_tramp_resolve
  .hidden _libdrm_so_tramp_resolve

  .globl _libdrm_so_save_regs_and_resolve
  .hidden _libdrm_so_save_regs_and_resolve
  .type _libdrm_so_save_regs_and_resolve, %function
_libdrm_so_save_regs_and_resolve:
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

  bl _libdrm_so_tramp_resolve
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

  .globl drmAddBufs
  .p2align 4
  .type drmAddBufs, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmAddBufs
#endif
drmAddBufs:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+0
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+0]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmAddContextPrivateMapping
  .p2align 4
  .type drmAddContextPrivateMapping, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmAddContextPrivateMapping
#endif
drmAddContextPrivateMapping:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+8
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+8]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmAddContextTag
  .p2align 4
  .type drmAddContextTag, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmAddContextTag
#endif
drmAddContextTag:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+16
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+16]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmAddMap
  .p2align 4
  .type drmAddMap, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmAddMap
#endif
drmAddMap:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+24
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+24]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmAgpAcquire
  .p2align 4
  .type drmAgpAcquire, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmAgpAcquire
#endif
drmAgpAcquire:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+32
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+32]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmAgpAlloc
  .p2align 4
  .type drmAgpAlloc, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmAgpAlloc
#endif
drmAgpAlloc:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+40
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+40]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmAgpBase
  .p2align 4
  .type drmAgpBase, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmAgpBase
#endif
drmAgpBase:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+48
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+48]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmAgpBind
  .p2align 4
  .type drmAgpBind, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmAgpBind
#endif
drmAgpBind:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+56
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+56]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmAgpDeviceId
  .p2align 4
  .type drmAgpDeviceId, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmAgpDeviceId
#endif
drmAgpDeviceId:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+64
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+64]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmAgpEnable
  .p2align 4
  .type drmAgpEnable, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmAgpEnable
#endif
drmAgpEnable:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+72
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+72]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmAgpFree
  .p2align 4
  .type drmAgpFree, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmAgpFree
#endif
drmAgpFree:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+80
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+80]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmAgpGetMode
  .p2align 4
  .type drmAgpGetMode, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmAgpGetMode
#endif
drmAgpGetMode:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+88
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+88]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmAgpMemoryAvail
  .p2align 4
  .type drmAgpMemoryAvail, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmAgpMemoryAvail
#endif
drmAgpMemoryAvail:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+96
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+96]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmAgpMemoryUsed
  .p2align 4
  .type drmAgpMemoryUsed, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmAgpMemoryUsed
#endif
drmAgpMemoryUsed:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+104
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+104]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmAgpRelease
  .p2align 4
  .type drmAgpRelease, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmAgpRelease
#endif
drmAgpRelease:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+112
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+112]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmAgpSize
  .p2align 4
  .type drmAgpSize, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmAgpSize
#endif
drmAgpSize:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+120
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+120]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmAgpUnbind
  .p2align 4
  .type drmAgpUnbind, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmAgpUnbind
#endif
drmAgpUnbind:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+128
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+128]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmAgpVendorId
  .p2align 4
  .type drmAgpVendorId, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmAgpVendorId
#endif
drmAgpVendorId:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+136
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+136]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmAgpVersionMajor
  .p2align 4
  .type drmAgpVersionMajor, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmAgpVersionMajor
#endif
drmAgpVersionMajor:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+144
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+144]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmAgpVersionMinor
  .p2align 4
  .type drmAgpVersionMinor, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmAgpVersionMinor
#endif
drmAgpVersionMinor:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+152
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+152]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmAuthMagic
  .p2align 4
  .type drmAuthMagic, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmAuthMagic
#endif
drmAuthMagic:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+160
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+160]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmAvailable
  .p2align 4
  .type drmAvailable, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmAvailable
#endif
drmAvailable:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+168
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+168]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmCheckModesettingSupported
  .p2align 4
  .type drmCheckModesettingSupported, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmCheckModesettingSupported
#endif
drmCheckModesettingSupported:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+176
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+176]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmClose
  .p2align 4
  .type drmClose, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmClose
#endif
drmClose:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+184
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+184]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmCloseBufferHandle
  .p2align 4
  .type drmCloseBufferHandle, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmCloseBufferHandle
#endif
drmCloseBufferHandle:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+192
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+192]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmCloseOnce
  .p2align 4
  .type drmCloseOnce, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmCloseOnce
#endif
drmCloseOnce:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+200
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+200]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmCommandNone
  .p2align 4
  .type drmCommandNone, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmCommandNone
#endif
drmCommandNone:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+208
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+208]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmCommandRead
  .p2align 4
  .type drmCommandRead, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmCommandRead
#endif
drmCommandRead:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+216
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+216]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmCommandWrite
  .p2align 4
  .type drmCommandWrite, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmCommandWrite
#endif
drmCommandWrite:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+224
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+224]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmCommandWriteRead
  .p2align 4
  .type drmCommandWriteRead, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmCommandWriteRead
#endif
drmCommandWriteRead:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+232
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+232]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmCreateContext
  .p2align 4
  .type drmCreateContext, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmCreateContext
#endif
drmCreateContext:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+240
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+240]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmCreateDrawable
  .p2align 4
  .type drmCreateDrawable, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmCreateDrawable
#endif
drmCreateDrawable:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+248
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+248]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmCrtcGetSequence
  .p2align 4
  .type drmCrtcGetSequence, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmCrtcGetSequence
#endif
drmCrtcGetSequence:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+256
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+256]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmCrtcQueueSequence
  .p2align 4
  .type drmCrtcQueueSequence, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmCrtcQueueSequence
#endif
drmCrtcQueueSequence:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+264
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+264]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmCtlInstHandler
  .p2align 4
  .type drmCtlInstHandler, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmCtlInstHandler
#endif
drmCtlInstHandler:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+272
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+272]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmCtlUninstHandler
  .p2align 4
  .type drmCtlUninstHandler, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmCtlUninstHandler
#endif
drmCtlUninstHandler:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+280
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+280]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmDMA
  .p2align 4
  .type drmDMA, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmDMA
#endif
drmDMA:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+288
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+288]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmDelContextTag
  .p2align 4
  .type drmDelContextTag, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmDelContextTag
#endif
drmDelContextTag:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+296
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+296]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmDestroyContext
  .p2align 4
  .type drmDestroyContext, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmDestroyContext
#endif
drmDestroyContext:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+304
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+304]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmDestroyDrawable
  .p2align 4
  .type drmDestroyDrawable, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmDestroyDrawable
#endif
drmDestroyDrawable:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+312
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+312]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmDevicesEqual
  .p2align 4
  .type drmDevicesEqual, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmDevicesEqual
#endif
drmDevicesEqual:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+320
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+320]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmDropMaster
  .p2align 4
  .type drmDropMaster, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmDropMaster
#endif
drmDropMaster:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+328
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+328]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmError
  .p2align 4
  .type drmError, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmError
#endif
drmError:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+336
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+336]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmFinish
  .p2align 4
  .type drmFinish, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmFinish
#endif
drmFinish:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+344
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+344]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmFree
  .p2align 4
  .type drmFree, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmFree
#endif
drmFree:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+352
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+352]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmFreeBufs
  .p2align 4
  .type drmFreeBufs, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmFreeBufs
#endif
drmFreeBufs:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+360
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+360]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmFreeBusid
  .p2align 4
  .type drmFreeBusid, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmFreeBusid
#endif
drmFreeBusid:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+368
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+368]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmFreeDevice
  .p2align 4
  .type drmFreeDevice, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmFreeDevice
#endif
drmFreeDevice:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+376
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+376]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmFreeDevices
  .p2align 4
  .type drmFreeDevices, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmFreeDevices
#endif
drmFreeDevices:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+384
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+384]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmFreeReservedContextList
  .p2align 4
  .type drmFreeReservedContextList, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmFreeReservedContextList
#endif
drmFreeReservedContextList:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+392
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+392]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmFreeVersion
  .p2align 4
  .type drmFreeVersion, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmFreeVersion
#endif
drmFreeVersion:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+400
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+400]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmGetBufInfo
  .p2align 4
  .type drmGetBufInfo, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmGetBufInfo
#endif
drmGetBufInfo:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+408
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+408]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmGetBusid
  .p2align 4
  .type drmGetBusid, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmGetBusid
#endif
drmGetBusid:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+416
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+416]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmGetCap
  .p2align 4
  .type drmGetCap, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmGetCap
#endif
drmGetCap:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+424
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+424]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmGetClient
  .p2align 4
  .type drmGetClient, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmGetClient
#endif
drmGetClient:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+432
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+432]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmGetContextFlags
  .p2align 4
  .type drmGetContextFlags, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmGetContextFlags
#endif
drmGetContextFlags:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+440
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+440]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmGetContextPrivateMapping
  .p2align 4
  .type drmGetContextPrivateMapping, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmGetContextPrivateMapping
#endif
drmGetContextPrivateMapping:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+448
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+448]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmGetContextTag
  .p2align 4
  .type drmGetContextTag, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmGetContextTag
#endif
drmGetContextTag:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+456
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+456]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmGetDevice
  .p2align 4
  .type drmGetDevice, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmGetDevice
#endif
drmGetDevice:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+464
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+464]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmGetDevice2
  .p2align 4
  .type drmGetDevice2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmGetDevice2
#endif
drmGetDevice2:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+472
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+472]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmGetDeviceFromDevId
  .p2align 4
  .type drmGetDeviceFromDevId, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmGetDeviceFromDevId
#endif
drmGetDeviceFromDevId:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+480
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+480]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmGetDeviceNameFromFd
  .p2align 4
  .type drmGetDeviceNameFromFd, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmGetDeviceNameFromFd
#endif
drmGetDeviceNameFromFd:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+488
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+488]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmGetDeviceNameFromFd2
  .p2align 4
  .type drmGetDeviceNameFromFd2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmGetDeviceNameFromFd2
#endif
drmGetDeviceNameFromFd2:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+496
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+496]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmGetDevices
  .p2align 4
  .type drmGetDevices, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmGetDevices
#endif
drmGetDevices:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+504
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+504]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmGetDevices2
  .p2align 4
  .type drmGetDevices2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmGetDevices2
#endif
drmGetDevices2:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+512
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+512]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmGetEntry
  .p2align 4
  .type drmGetEntry, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmGetEntry
#endif
drmGetEntry:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+520
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+520]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmGetFormatModifierName
  .p2align 4
  .type drmGetFormatModifierName, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmGetFormatModifierName
#endif
drmGetFormatModifierName:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+528
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+528]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmGetFormatModifierVendor
  .p2align 4
  .type drmGetFormatModifierVendor, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmGetFormatModifierVendor
#endif
drmGetFormatModifierVendor:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+536
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+536]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmGetFormatName
  .p2align 4
  .type drmGetFormatName, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmGetFormatName
#endif
drmGetFormatName:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+544
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+544]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmGetHashTable
  .p2align 4
  .type drmGetHashTable, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmGetHashTable
#endif
drmGetHashTable:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+552
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+552]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmGetInterruptFromBusID
  .p2align 4
  .type drmGetInterruptFromBusID, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmGetInterruptFromBusID
#endif
drmGetInterruptFromBusID:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+560
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+560]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmGetLibVersion
  .p2align 4
  .type drmGetLibVersion, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmGetLibVersion
#endif
drmGetLibVersion:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+568
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+568]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmGetLock
  .p2align 4
  .type drmGetLock, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmGetLock
#endif
drmGetLock:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+576
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+576]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmGetMagic
  .p2align 4
  .type drmGetMagic, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmGetMagic
#endif
drmGetMagic:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+584
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+584]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmGetMap
  .p2align 4
  .type drmGetMap, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmGetMap
#endif
drmGetMap:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+592
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+592]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmGetNodeTypeFromDevId
  .p2align 4
  .type drmGetNodeTypeFromDevId, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmGetNodeTypeFromDevId
#endif
drmGetNodeTypeFromDevId:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+600
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+600]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmGetNodeTypeFromFd
  .p2align 4
  .type drmGetNodeTypeFromFd, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmGetNodeTypeFromFd
#endif
drmGetNodeTypeFromFd:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+608
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+608]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmGetPrimaryDeviceNameFromFd
  .p2align 4
  .type drmGetPrimaryDeviceNameFromFd, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmGetPrimaryDeviceNameFromFd
#endif
drmGetPrimaryDeviceNameFromFd:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+616
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+616]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmGetRenderDeviceNameFromFd
  .p2align 4
  .type drmGetRenderDeviceNameFromFd, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmGetRenderDeviceNameFromFd
#endif
drmGetRenderDeviceNameFromFd:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+624
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+624]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmGetReservedContextList
  .p2align 4
  .type drmGetReservedContextList, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmGetReservedContextList
#endif
drmGetReservedContextList:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+632
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+632]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmGetStats
  .p2align 4
  .type drmGetStats, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmGetStats
#endif
drmGetStats:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+640
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+640]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmGetVersion
  .p2align 4
  .type drmGetVersion, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmGetVersion
#endif
drmGetVersion:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+648
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+648]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmHandleEvent
  .p2align 4
  .type drmHandleEvent, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmHandleEvent
#endif
drmHandleEvent:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+656
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+656]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmHashCreate
  .p2align 4
  .type drmHashCreate, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmHashCreate
#endif
drmHashCreate:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+664
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+664]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmHashDelete
  .p2align 4
  .type drmHashDelete, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmHashDelete
#endif
drmHashDelete:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+672
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+672]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmHashDestroy
  .p2align 4
  .type drmHashDestroy, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmHashDestroy
#endif
drmHashDestroy:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+680
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+680]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmHashFirst
  .p2align 4
  .type drmHashFirst, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmHashFirst
#endif
drmHashFirst:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+688
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+688]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmHashInsert
  .p2align 4
  .type drmHashInsert, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmHashInsert
#endif
drmHashInsert:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+696
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+696]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmHashLookup
  .p2align 4
  .type drmHashLookup, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmHashLookup
#endif
drmHashLookup:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+704
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+704]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmHashNext
  .p2align 4
  .type drmHashNext, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmHashNext
#endif
drmHashNext:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+712
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+712]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmIoctl
  .p2align 4
  .type drmIoctl, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmIoctl
#endif
drmIoctl:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+720
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+720]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmIsKMS
  .p2align 4
  .type drmIsKMS, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmIsKMS
#endif
drmIsKMS:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+728
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+728]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmIsMaster
  .p2align 4
  .type drmIsMaster, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmIsMaster
#endif
drmIsMaster:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+736
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+736]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmMalloc
  .p2align 4
  .type drmMalloc, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmMalloc
#endif
drmMalloc:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+744
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+744]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmMap
  .p2align 4
  .type drmMap, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmMap
#endif
drmMap:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+752
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+752]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmMapBufs
  .p2align 4
  .type drmMapBufs, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmMapBufs
#endif
drmMapBufs:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+760
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+760]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmMarkBufs
  .p2align 4
  .type drmMarkBufs, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmMarkBufs
#endif
drmMarkBufs:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+768
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+768]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmModeAddFB
  .p2align 4
  .type drmModeAddFB, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmModeAddFB
#endif
drmModeAddFB:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+776
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+776]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmModeAddFB2
  .p2align 4
  .type drmModeAddFB2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmModeAddFB2
#endif
drmModeAddFB2:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+784
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+784]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmModeAddFB2WithModifiers
  .p2align 4
  .type drmModeAddFB2WithModifiers, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmModeAddFB2WithModifiers
#endif
drmModeAddFB2WithModifiers:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+792
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+792]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmModeAtomicAddProperty
  .p2align 4
  .type drmModeAtomicAddProperty, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmModeAtomicAddProperty
#endif
drmModeAtomicAddProperty:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+800
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+800]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmModeAtomicAlloc
  .p2align 4
  .type drmModeAtomicAlloc, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmModeAtomicAlloc
#endif
drmModeAtomicAlloc:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+808
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+808]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmModeAtomicCommit
  .p2align 4
  .type drmModeAtomicCommit, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmModeAtomicCommit
#endif
drmModeAtomicCommit:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+816
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+816]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmModeAtomicDuplicate
  .p2align 4
  .type drmModeAtomicDuplicate, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmModeAtomicDuplicate
#endif
drmModeAtomicDuplicate:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+824
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+824]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmModeAtomicFree
  .p2align 4
  .type drmModeAtomicFree, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmModeAtomicFree
#endif
drmModeAtomicFree:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+832
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+832]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmModeAtomicGetCursor
  .p2align 4
  .type drmModeAtomicGetCursor, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmModeAtomicGetCursor
#endif
drmModeAtomicGetCursor:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+840
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+840]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmModeAtomicMerge
  .p2align 4
  .type drmModeAtomicMerge, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmModeAtomicMerge
#endif
drmModeAtomicMerge:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+848
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+848]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmModeAtomicSetCursor
  .p2align 4
  .type drmModeAtomicSetCursor, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmModeAtomicSetCursor
#endif
drmModeAtomicSetCursor:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+856
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+856]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmModeAttachMode
  .p2align 4
  .type drmModeAttachMode, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmModeAttachMode
#endif
drmModeAttachMode:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+864
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+864]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmModeCloseFB
  .p2align 4
  .type drmModeCloseFB, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmModeCloseFB
#endif
drmModeCloseFB:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+872
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+872]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmModeConnectorGetPossibleCrtcs
  .p2align 4
  .type drmModeConnectorGetPossibleCrtcs, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmModeConnectorGetPossibleCrtcs
#endif
drmModeConnectorGetPossibleCrtcs:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+880
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+880]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmModeConnectorSetProperty
  .p2align 4
  .type drmModeConnectorSetProperty, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmModeConnectorSetProperty
#endif
drmModeConnectorSetProperty:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+888
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+888]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmModeCreateDumbBuffer
  .p2align 4
  .type drmModeCreateDumbBuffer, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmModeCreateDumbBuffer
#endif
drmModeCreateDumbBuffer:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+896
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+896]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmModeCreateLease
  .p2align 4
  .type drmModeCreateLease, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmModeCreateLease
#endif
drmModeCreateLease:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+904
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+904]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmModeCreatePropertyBlob
  .p2align 4
  .type drmModeCreatePropertyBlob, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmModeCreatePropertyBlob
#endif
drmModeCreatePropertyBlob:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+912
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+912]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmModeCrtcGetGamma
  .p2align 4
  .type drmModeCrtcGetGamma, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmModeCrtcGetGamma
#endif
drmModeCrtcGetGamma:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+920
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+920]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmModeCrtcSetGamma
  .p2align 4
  .type drmModeCrtcSetGamma, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmModeCrtcSetGamma
#endif
drmModeCrtcSetGamma:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+928
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+928]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmModeDestroyDumbBuffer
  .p2align 4
  .type drmModeDestroyDumbBuffer, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmModeDestroyDumbBuffer
#endif
drmModeDestroyDumbBuffer:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+936
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+936]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmModeDestroyPropertyBlob
  .p2align 4
  .type drmModeDestroyPropertyBlob, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmModeDestroyPropertyBlob
#endif
drmModeDestroyPropertyBlob:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+944
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+944]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmModeDetachMode
  .p2align 4
  .type drmModeDetachMode, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmModeDetachMode
#endif
drmModeDetachMode:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+952
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+952]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmModeDirtyFB
  .p2align 4
  .type drmModeDirtyFB, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmModeDirtyFB
#endif
drmModeDirtyFB:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+960
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+960]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmModeFormatModifierBlobIterNext
  .p2align 4
  .type drmModeFormatModifierBlobIterNext, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmModeFormatModifierBlobIterNext
#endif
drmModeFormatModifierBlobIterNext:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+968
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+968]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmModeFreeConnector
  .p2align 4
  .type drmModeFreeConnector, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmModeFreeConnector
#endif
drmModeFreeConnector:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+976
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+976]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmModeFreeCrtc
  .p2align 4
  .type drmModeFreeCrtc, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmModeFreeCrtc
#endif
drmModeFreeCrtc:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+984
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+984]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmModeFreeEncoder
  .p2align 4
  .type drmModeFreeEncoder, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmModeFreeEncoder
#endif
drmModeFreeEncoder:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+992
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+992]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmModeFreeFB
  .p2align 4
  .type drmModeFreeFB, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmModeFreeFB
#endif
drmModeFreeFB:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+1000
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+1000]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmModeFreeFB2
  .p2align 4
  .type drmModeFreeFB2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmModeFreeFB2
#endif
drmModeFreeFB2:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+1008
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+1008]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmModeFreeModeInfo
  .p2align 4
  .type drmModeFreeModeInfo, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmModeFreeModeInfo
#endif
drmModeFreeModeInfo:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+1016
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+1016]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmModeFreeObjectProperties
  .p2align 4
  .type drmModeFreeObjectProperties, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmModeFreeObjectProperties
#endif
drmModeFreeObjectProperties:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+1024
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+1024]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmModeFreePlane
  .p2align 4
  .type drmModeFreePlane, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmModeFreePlane
#endif
drmModeFreePlane:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+1032
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+1032]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmModeFreePlaneResources
  .p2align 4
  .type drmModeFreePlaneResources, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmModeFreePlaneResources
#endif
drmModeFreePlaneResources:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+1040
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+1040]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmModeFreeProperty
  .p2align 4
  .type drmModeFreeProperty, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmModeFreeProperty
#endif
drmModeFreeProperty:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+1048
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+1048]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmModeFreePropertyBlob
  .p2align 4
  .type drmModeFreePropertyBlob, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmModeFreePropertyBlob
#endif
drmModeFreePropertyBlob:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+1056
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+1056]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmModeFreeResources
  .p2align 4
  .type drmModeFreeResources, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmModeFreeResources
#endif
drmModeFreeResources:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+1064
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+1064]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmModeGetConnector
  .p2align 4
  .type drmModeGetConnector, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmModeGetConnector
#endif
drmModeGetConnector:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+1072
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+1072]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmModeGetConnectorCurrent
  .p2align 4
  .type drmModeGetConnectorCurrent, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmModeGetConnectorCurrent
#endif
drmModeGetConnectorCurrent:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+1080
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+1080]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmModeGetConnectorTypeName
  .p2align 4
  .type drmModeGetConnectorTypeName, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmModeGetConnectorTypeName
#endif
drmModeGetConnectorTypeName:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+1088
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+1088]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmModeGetCrtc
  .p2align 4
  .type drmModeGetCrtc, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmModeGetCrtc
#endif
drmModeGetCrtc:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+1096
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+1096]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmModeGetEncoder
  .p2align 4
  .type drmModeGetEncoder, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmModeGetEncoder
#endif
drmModeGetEncoder:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+1104
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+1104]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmModeGetFB
  .p2align 4
  .type drmModeGetFB, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmModeGetFB
#endif
drmModeGetFB:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+1112
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+1112]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmModeGetFB2
  .p2align 4
  .type drmModeGetFB2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmModeGetFB2
#endif
drmModeGetFB2:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+1120
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+1120]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmModeGetLease
  .p2align 4
  .type drmModeGetLease, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmModeGetLease
#endif
drmModeGetLease:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+1128
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+1128]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmModeGetPlane
  .p2align 4
  .type drmModeGetPlane, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmModeGetPlane
#endif
drmModeGetPlane:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+1136
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+1136]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmModeGetPlaneResources
  .p2align 4
  .type drmModeGetPlaneResources, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmModeGetPlaneResources
#endif
drmModeGetPlaneResources:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+1144
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+1144]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmModeGetProperty
  .p2align 4
  .type drmModeGetProperty, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmModeGetProperty
#endif
drmModeGetProperty:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+1152
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+1152]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmModeGetPropertyBlob
  .p2align 4
  .type drmModeGetPropertyBlob, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmModeGetPropertyBlob
#endif
drmModeGetPropertyBlob:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+1160
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+1160]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmModeGetResources
  .p2align 4
  .type drmModeGetResources, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmModeGetResources
#endif
drmModeGetResources:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+1168
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+1168]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmModeListLessees
  .p2align 4
  .type drmModeListLessees, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmModeListLessees
#endif
drmModeListLessees:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+1176
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+1176]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmModeMapDumbBuffer
  .p2align 4
  .type drmModeMapDumbBuffer, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmModeMapDumbBuffer
#endif
drmModeMapDumbBuffer:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+1184
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+1184]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmModeMoveCursor
  .p2align 4
  .type drmModeMoveCursor, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmModeMoveCursor
#endif
drmModeMoveCursor:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+1192
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+1192]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmModeObjectGetProperties
  .p2align 4
  .type drmModeObjectGetProperties, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmModeObjectGetProperties
#endif
drmModeObjectGetProperties:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+1200
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+1200]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmModeObjectSetProperty
  .p2align 4
  .type drmModeObjectSetProperty, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmModeObjectSetProperty
#endif
drmModeObjectSetProperty:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+1208
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+1208]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmModePageFlip
  .p2align 4
  .type drmModePageFlip, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmModePageFlip
#endif
drmModePageFlip:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+1216
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+1216]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmModePageFlipTarget
  .p2align 4
  .type drmModePageFlipTarget, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmModePageFlipTarget
#endif
drmModePageFlipTarget:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+1224
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+1224]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmModeRevokeLease
  .p2align 4
  .type drmModeRevokeLease, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmModeRevokeLease
#endif
drmModeRevokeLease:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+1232
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+1232]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmModeRmFB
  .p2align 4
  .type drmModeRmFB, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmModeRmFB
#endif
drmModeRmFB:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+1240
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+1240]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmModeSetCrtc
  .p2align 4
  .type drmModeSetCrtc, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmModeSetCrtc
#endif
drmModeSetCrtc:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+1248
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+1248]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmModeSetCursor
  .p2align 4
  .type drmModeSetCursor, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmModeSetCursor
#endif
drmModeSetCursor:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+1256
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+1256]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmModeSetCursor2
  .p2align 4
  .type drmModeSetCursor2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmModeSetCursor2
#endif
drmModeSetCursor2:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+1264
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+1264]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmModeSetPlane
  .p2align 4
  .type drmModeSetPlane, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmModeSetPlane
#endif
drmModeSetPlane:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+1272
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+1272]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmMsg
  .p2align 4
  .type drmMsg, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmMsg
#endif
drmMsg:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+1280
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+1280]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmOpen
  .p2align 4
  .type drmOpen, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmOpen
#endif
drmOpen:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+1288
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+1288]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmOpenControl
  .p2align 4
  .type drmOpenControl, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmOpenControl
#endif
drmOpenControl:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+1296
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+1296]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmOpenOnce
  .p2align 4
  .type drmOpenOnce, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmOpenOnce
#endif
drmOpenOnce:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+1304
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+1304]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmOpenOnceWithType
  .p2align 4
  .type drmOpenOnceWithType, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmOpenOnceWithType
#endif
drmOpenOnceWithType:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+1312
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+1312]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmOpenRender
  .p2align 4
  .type drmOpenRender, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmOpenRender
#endif
drmOpenRender:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+1320
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+1320]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmOpenWithType
  .p2align 4
  .type drmOpenWithType, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmOpenWithType
#endif
drmOpenWithType:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+1328
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+1328]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmPrimeFDToHandle
  .p2align 4
  .type drmPrimeFDToHandle, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmPrimeFDToHandle
#endif
drmPrimeFDToHandle:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+1336
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+1336]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmPrimeHandleToFD
  .p2align 4
  .type drmPrimeHandleToFD, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmPrimeHandleToFD
#endif
drmPrimeHandleToFD:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+1344
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+1344]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmRandom
  .p2align 4
  .type drmRandom, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmRandom
#endif
drmRandom:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+1352
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+1352]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmRandomCreate
  .p2align 4
  .type drmRandomCreate, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmRandomCreate
#endif
drmRandomCreate:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+1360
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+1360]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmRandomDestroy
  .p2align 4
  .type drmRandomDestroy, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmRandomDestroy
#endif
drmRandomDestroy:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+1368
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+1368]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmRandomDouble
  .p2align 4
  .type drmRandomDouble, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmRandomDouble
#endif
drmRandomDouble:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+1376
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+1376]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmRmMap
  .p2align 4
  .type drmRmMap, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmRmMap
#endif
drmRmMap:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+1384
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+1384]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmSLCreate
  .p2align 4
  .type drmSLCreate, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmSLCreate
#endif
drmSLCreate:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+1392
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+1392]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmSLDelete
  .p2align 4
  .type drmSLDelete, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmSLDelete
#endif
drmSLDelete:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+1400
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+1400]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmSLDestroy
  .p2align 4
  .type drmSLDestroy, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmSLDestroy
#endif
drmSLDestroy:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+1408
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+1408]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmSLDump
  .p2align 4
  .type drmSLDump, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmSLDump
#endif
drmSLDump:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+1416
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+1416]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmSLFirst
  .p2align 4
  .type drmSLFirst, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmSLFirst
#endif
drmSLFirst:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+1424
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+1424]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmSLInsert
  .p2align 4
  .type drmSLInsert, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmSLInsert
#endif
drmSLInsert:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+1432
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+1432]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmSLLookup
  .p2align 4
  .type drmSLLookup, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmSLLookup
#endif
drmSLLookup:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+1440
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+1440]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmSLLookupNeighbors
  .p2align 4
  .type drmSLLookupNeighbors, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmSLLookupNeighbors
#endif
drmSLLookupNeighbors:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+1448
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+1448]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmSLNext
  .p2align 4
  .type drmSLNext, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmSLNext
#endif
drmSLNext:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+1456
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+1456]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmScatterGatherAlloc
  .p2align 4
  .type drmScatterGatherAlloc, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmScatterGatherAlloc
#endif
drmScatterGatherAlloc:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+1464
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+1464]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmScatterGatherFree
  .p2align 4
  .type drmScatterGatherFree, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmScatterGatherFree
#endif
drmScatterGatherFree:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+1472
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+1472]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmSetBusid
  .p2align 4
  .type drmSetBusid, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmSetBusid
#endif
drmSetBusid:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+1480
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+1480]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmSetClientCap
  .p2align 4
  .type drmSetClientCap, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmSetClientCap
#endif
drmSetClientCap:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+1488
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+1488]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmSetContextFlags
  .p2align 4
  .type drmSetContextFlags, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmSetContextFlags
#endif
drmSetContextFlags:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+1496
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+1496]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmSetInterfaceVersion
  .p2align 4
  .type drmSetInterfaceVersion, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmSetInterfaceVersion
#endif
drmSetInterfaceVersion:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+1504
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+1504]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmSetMaster
  .p2align 4
  .type drmSetMaster, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmSetMaster
#endif
drmSetMaster:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+1512
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+1512]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmSetServerInfo
  .p2align 4
  .type drmSetServerInfo, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmSetServerInfo
#endif
drmSetServerInfo:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+1520
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+1520]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmSwitchToContext
  .p2align 4
  .type drmSwitchToContext, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmSwitchToContext
#endif
drmSwitchToContext:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+1528
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+1528]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmSyncobjCreate
  .p2align 4
  .type drmSyncobjCreate, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmSyncobjCreate
#endif
drmSyncobjCreate:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+1536
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+1536]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmSyncobjDestroy
  .p2align 4
  .type drmSyncobjDestroy, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmSyncobjDestroy
#endif
drmSyncobjDestroy:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+1544
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+1544]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmSyncobjEventfd
  .p2align 4
  .type drmSyncobjEventfd, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmSyncobjEventfd
#endif
drmSyncobjEventfd:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+1552
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+1552]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmSyncobjExportSyncFile
  .p2align 4
  .type drmSyncobjExportSyncFile, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmSyncobjExportSyncFile
#endif
drmSyncobjExportSyncFile:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+1560
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+1560]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmSyncobjFDToHandle
  .p2align 4
  .type drmSyncobjFDToHandle, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmSyncobjFDToHandle
#endif
drmSyncobjFDToHandle:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+1568
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+1568]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmSyncobjHandleToFD
  .p2align 4
  .type drmSyncobjHandleToFD, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmSyncobjHandleToFD
#endif
drmSyncobjHandleToFD:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+1576
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+1576]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmSyncobjImportSyncFile
  .p2align 4
  .type drmSyncobjImportSyncFile, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmSyncobjImportSyncFile
#endif
drmSyncobjImportSyncFile:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+1584
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+1584]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmSyncobjQuery
  .p2align 4
  .type drmSyncobjQuery, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmSyncobjQuery
#endif
drmSyncobjQuery:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+1592
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+1592]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmSyncobjQuery2
  .p2align 4
  .type drmSyncobjQuery2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmSyncobjQuery2
#endif
drmSyncobjQuery2:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+1600
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+1600]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmSyncobjReset
  .p2align 4
  .type drmSyncobjReset, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmSyncobjReset
#endif
drmSyncobjReset:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+1608
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+1608]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmSyncobjSignal
  .p2align 4
  .type drmSyncobjSignal, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmSyncobjSignal
#endif
drmSyncobjSignal:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+1616
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+1616]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmSyncobjTimelineSignal
  .p2align 4
  .type drmSyncobjTimelineSignal, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmSyncobjTimelineSignal
#endif
drmSyncobjTimelineSignal:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+1624
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+1624]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmSyncobjTimelineWait
  .p2align 4
  .type drmSyncobjTimelineWait, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmSyncobjTimelineWait
#endif
drmSyncobjTimelineWait:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+1632
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+1632]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmSyncobjTransfer
  .p2align 4
  .type drmSyncobjTransfer, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmSyncobjTransfer
#endif
drmSyncobjTransfer:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+1640
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+1640]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmSyncobjWait
  .p2align 4
  .type drmSyncobjWait, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmSyncobjWait
#endif
drmSyncobjWait:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+1648
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+1648]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmUnlock
  .p2align 4
  .type drmUnlock, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmUnlock
#endif
drmUnlock:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+1656
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+1656]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmUnmap
  .p2align 4
  .type drmUnmap, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmUnmap
#endif
drmUnmap:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+1664
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+1664]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmUnmapBufs
  .p2align 4
  .type drmUnmapBufs, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmUnmapBufs
#endif
drmUnmapBufs:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+1672
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+1672]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmUpdateDrawableInfo
  .p2align 4
  .type drmUpdateDrawableInfo, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmUpdateDrawableInfo
#endif
drmUpdateDrawableInfo:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+1680
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+1680]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl drmWaitVBlank
  .p2align 4
  .type drmWaitVBlank, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden drmWaitVBlank
#endif
drmWaitVBlank:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libdrm_so_tramp_table+1688
  ldr ip0, [ip0, #:lo12:_libdrm_so_tramp_table+1688]
 
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
  bl _libdrm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

