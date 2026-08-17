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

  .globl _libgbm_so_tramp_table
  .hidden _libgbm_so_tramp_table
  .align 8
_libgbm_so_tramp_table:
  .zero 312

  .text

  .globl _libgbm_so_tramp_resolve
  .hidden _libgbm_so_tramp_resolve

  .globl _libgbm_so_save_regs_and_resolve
  .hidden _libgbm_so_save_regs_and_resolve
  .type _libgbm_so_save_regs_and_resolve, %function
_libgbm_so_save_regs_and_resolve:
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

  bl _libgbm_so_tramp_resolve
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

  .globl gbm_bo_create
  .p2align 4
  .type gbm_bo_create, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden gbm_bo_create
#endif
gbm_bo_create:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libgbm_so_tramp_table+0
  ldr ip0, [ip0, #:lo12:_libgbm_so_tramp_table+0]
 
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
  bl _libgbm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl gbm_bo_create_with_modifiers
  .p2align 4
  .type gbm_bo_create_with_modifiers, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden gbm_bo_create_with_modifiers
#endif
gbm_bo_create_with_modifiers:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libgbm_so_tramp_table+8
  ldr ip0, [ip0, #:lo12:_libgbm_so_tramp_table+8]
 
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
  bl _libgbm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl gbm_bo_create_with_modifiers2
  .p2align 4
  .type gbm_bo_create_with_modifiers2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden gbm_bo_create_with_modifiers2
#endif
gbm_bo_create_with_modifiers2:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libgbm_so_tramp_table+16
  ldr ip0, [ip0, #:lo12:_libgbm_so_tramp_table+16]
 
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
  bl _libgbm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl gbm_bo_destroy
  .p2align 4
  .type gbm_bo_destroy, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden gbm_bo_destroy
#endif
gbm_bo_destroy:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libgbm_so_tramp_table+24
  ldr ip0, [ip0, #:lo12:_libgbm_so_tramp_table+24]
 
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
  bl _libgbm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl gbm_bo_get_bpp
  .p2align 4
  .type gbm_bo_get_bpp, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden gbm_bo_get_bpp
#endif
gbm_bo_get_bpp:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libgbm_so_tramp_table+32
  ldr ip0, [ip0, #:lo12:_libgbm_so_tramp_table+32]
 
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
  bl _libgbm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl gbm_bo_get_device
  .p2align 4
  .type gbm_bo_get_device, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden gbm_bo_get_device
#endif
gbm_bo_get_device:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libgbm_so_tramp_table+40
  ldr ip0, [ip0, #:lo12:_libgbm_so_tramp_table+40]
 
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
  bl _libgbm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl gbm_bo_get_fd
  .p2align 4
  .type gbm_bo_get_fd, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden gbm_bo_get_fd
#endif
gbm_bo_get_fd:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libgbm_so_tramp_table+48
  ldr ip0, [ip0, #:lo12:_libgbm_so_tramp_table+48]
 
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
  bl _libgbm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl gbm_bo_get_fd_for_plane
  .p2align 4
  .type gbm_bo_get_fd_for_plane, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden gbm_bo_get_fd_for_plane
#endif
gbm_bo_get_fd_for_plane:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libgbm_so_tramp_table+56
  ldr ip0, [ip0, #:lo12:_libgbm_so_tramp_table+56]
 
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
  bl _libgbm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl gbm_bo_get_format
  .p2align 4
  .type gbm_bo_get_format, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden gbm_bo_get_format
#endif
gbm_bo_get_format:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libgbm_so_tramp_table+64
  ldr ip0, [ip0, #:lo12:_libgbm_so_tramp_table+64]
 
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
  bl _libgbm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl gbm_bo_get_handle
  .p2align 4
  .type gbm_bo_get_handle, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden gbm_bo_get_handle
#endif
gbm_bo_get_handle:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libgbm_so_tramp_table+72
  ldr ip0, [ip0, #:lo12:_libgbm_so_tramp_table+72]
 
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
  bl _libgbm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl gbm_bo_get_handle_for_plane
  .p2align 4
  .type gbm_bo_get_handle_for_plane, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden gbm_bo_get_handle_for_plane
#endif
gbm_bo_get_handle_for_plane:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libgbm_so_tramp_table+80
  ldr ip0, [ip0, #:lo12:_libgbm_so_tramp_table+80]
 
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
  bl _libgbm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl gbm_bo_get_height
  .p2align 4
  .type gbm_bo_get_height, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden gbm_bo_get_height
#endif
gbm_bo_get_height:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libgbm_so_tramp_table+88
  ldr ip0, [ip0, #:lo12:_libgbm_so_tramp_table+88]
 
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
  bl _libgbm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl gbm_bo_get_modifier
  .p2align 4
  .type gbm_bo_get_modifier, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden gbm_bo_get_modifier
#endif
gbm_bo_get_modifier:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libgbm_so_tramp_table+96
  ldr ip0, [ip0, #:lo12:_libgbm_so_tramp_table+96]
 
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
  bl _libgbm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl gbm_bo_get_offset
  .p2align 4
  .type gbm_bo_get_offset, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden gbm_bo_get_offset
#endif
gbm_bo_get_offset:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libgbm_so_tramp_table+104
  ldr ip0, [ip0, #:lo12:_libgbm_so_tramp_table+104]
 
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
  bl _libgbm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl gbm_bo_get_plane_count
  .p2align 4
  .type gbm_bo_get_plane_count, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden gbm_bo_get_plane_count
#endif
gbm_bo_get_plane_count:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libgbm_so_tramp_table+112
  ldr ip0, [ip0, #:lo12:_libgbm_so_tramp_table+112]
 
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
  bl _libgbm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl gbm_bo_get_stride
  .p2align 4
  .type gbm_bo_get_stride, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden gbm_bo_get_stride
#endif
gbm_bo_get_stride:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libgbm_so_tramp_table+120
  ldr ip0, [ip0, #:lo12:_libgbm_so_tramp_table+120]
 
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
  bl _libgbm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl gbm_bo_get_stride_for_plane
  .p2align 4
  .type gbm_bo_get_stride_for_plane, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden gbm_bo_get_stride_for_plane
#endif
gbm_bo_get_stride_for_plane:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libgbm_so_tramp_table+128
  ldr ip0, [ip0, #:lo12:_libgbm_so_tramp_table+128]
 
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
  bl _libgbm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl gbm_bo_get_user_data
  .p2align 4
  .type gbm_bo_get_user_data, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden gbm_bo_get_user_data
#endif
gbm_bo_get_user_data:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libgbm_so_tramp_table+136
  ldr ip0, [ip0, #:lo12:_libgbm_so_tramp_table+136]
 
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
  bl _libgbm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl gbm_bo_get_width
  .p2align 4
  .type gbm_bo_get_width, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden gbm_bo_get_width
#endif
gbm_bo_get_width:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libgbm_so_tramp_table+144
  ldr ip0, [ip0, #:lo12:_libgbm_so_tramp_table+144]
 
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
  bl _libgbm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl gbm_bo_import
  .p2align 4
  .type gbm_bo_import, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden gbm_bo_import
#endif
gbm_bo_import:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libgbm_so_tramp_table+152
  ldr ip0, [ip0, #:lo12:_libgbm_so_tramp_table+152]
 
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
  bl _libgbm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl gbm_bo_map
  .p2align 4
  .type gbm_bo_map, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden gbm_bo_map
#endif
gbm_bo_map:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libgbm_so_tramp_table+160
  ldr ip0, [ip0, #:lo12:_libgbm_so_tramp_table+160]
 
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
  bl _libgbm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl gbm_bo_set_user_data
  .p2align 4
  .type gbm_bo_set_user_data, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden gbm_bo_set_user_data
#endif
gbm_bo_set_user_data:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libgbm_so_tramp_table+168
  ldr ip0, [ip0, #:lo12:_libgbm_so_tramp_table+168]
 
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
  bl _libgbm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl gbm_bo_unmap
  .p2align 4
  .type gbm_bo_unmap, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden gbm_bo_unmap
#endif
gbm_bo_unmap:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libgbm_so_tramp_table+176
  ldr ip0, [ip0, #:lo12:_libgbm_so_tramp_table+176]
 
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
  bl _libgbm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl gbm_bo_write
  .p2align 4
  .type gbm_bo_write, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden gbm_bo_write
#endif
gbm_bo_write:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libgbm_so_tramp_table+184
  ldr ip0, [ip0, #:lo12:_libgbm_so_tramp_table+184]
 
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
  bl _libgbm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl gbm_create_device
  .p2align 4
  .type gbm_create_device, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden gbm_create_device
#endif
gbm_create_device:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libgbm_so_tramp_table+192
  ldr ip0, [ip0, #:lo12:_libgbm_so_tramp_table+192]
 
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
  bl _libgbm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl gbm_device_destroy
  .p2align 4
  .type gbm_device_destroy, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden gbm_device_destroy
#endif
gbm_device_destroy:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libgbm_so_tramp_table+200
  ldr ip0, [ip0, #:lo12:_libgbm_so_tramp_table+200]
 
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
  bl _libgbm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl gbm_device_get_backend_name
  .p2align 4
  .type gbm_device_get_backend_name, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden gbm_device_get_backend_name
#endif
gbm_device_get_backend_name:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libgbm_so_tramp_table+208
  ldr ip0, [ip0, #:lo12:_libgbm_so_tramp_table+208]
 
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
  bl _libgbm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl gbm_device_get_fd
  .p2align 4
  .type gbm_device_get_fd, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden gbm_device_get_fd
#endif
gbm_device_get_fd:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libgbm_so_tramp_table+216
  ldr ip0, [ip0, #:lo12:_libgbm_so_tramp_table+216]
 
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
  bl _libgbm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl gbm_device_get_format_modifier_plane_count
  .p2align 4
  .type gbm_device_get_format_modifier_plane_count, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden gbm_device_get_format_modifier_plane_count
#endif
gbm_device_get_format_modifier_plane_count:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libgbm_so_tramp_table+224
  ldr ip0, [ip0, #:lo12:_libgbm_so_tramp_table+224]
 
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
  bl _libgbm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl gbm_device_is_format_supported
  .p2align 4
  .type gbm_device_is_format_supported, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden gbm_device_is_format_supported
#endif
gbm_device_is_format_supported:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libgbm_so_tramp_table+232
  ldr ip0, [ip0, #:lo12:_libgbm_so_tramp_table+232]
 
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
  bl _libgbm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl gbm_format_get_name
  .p2align 4
  .type gbm_format_get_name, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden gbm_format_get_name
#endif
gbm_format_get_name:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libgbm_so_tramp_table+240
  ldr ip0, [ip0, #:lo12:_libgbm_so_tramp_table+240]
 
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
  bl _libgbm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl gbm_surface_create
  .p2align 4
  .type gbm_surface_create, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden gbm_surface_create
#endif
gbm_surface_create:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libgbm_so_tramp_table+248
  ldr ip0, [ip0, #:lo12:_libgbm_so_tramp_table+248]
 
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
  bl _libgbm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl gbm_surface_create_with_modifiers
  .p2align 4
  .type gbm_surface_create_with_modifiers, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden gbm_surface_create_with_modifiers
#endif
gbm_surface_create_with_modifiers:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libgbm_so_tramp_table+256
  ldr ip0, [ip0, #:lo12:_libgbm_so_tramp_table+256]
 
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
  bl _libgbm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl gbm_surface_create_with_modifiers2
  .p2align 4
  .type gbm_surface_create_with_modifiers2, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden gbm_surface_create_with_modifiers2
#endif
gbm_surface_create_with_modifiers2:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libgbm_so_tramp_table+264
  ldr ip0, [ip0, #:lo12:_libgbm_so_tramp_table+264]
 
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
  bl _libgbm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl gbm_surface_destroy
  .p2align 4
  .type gbm_surface_destroy, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden gbm_surface_destroy
#endif
gbm_surface_destroy:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libgbm_so_tramp_table+272
  ldr ip0, [ip0, #:lo12:_libgbm_so_tramp_table+272]
 
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
  bl _libgbm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl gbm_surface_has_free_buffers
  .p2align 4
  .type gbm_surface_has_free_buffers, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden gbm_surface_has_free_buffers
#endif
gbm_surface_has_free_buffers:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libgbm_so_tramp_table+280
  ldr ip0, [ip0, #:lo12:_libgbm_so_tramp_table+280]
 
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
  bl _libgbm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl gbm_surface_lock_front_buffer
  .p2align 4
  .type gbm_surface_lock_front_buffer, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden gbm_surface_lock_front_buffer
#endif
gbm_surface_lock_front_buffer:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libgbm_so_tramp_table+288
  ldr ip0, [ip0, #:lo12:_libgbm_so_tramp_table+288]
 
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
  bl _libgbm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

  .globl gbm_surface_release_buffer
  .p2align 4
  .type gbm_surface_release_buffer, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden gbm_surface_release_buffer
#endif
gbm_surface_release_buffer:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libgbm_so_tramp_table+296
  ldr ip0, [ip0, #:lo12:_libgbm_so_tramp_table+296]
 
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
  bl _libgbm_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

