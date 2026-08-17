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

  bl _libXfixes_so_tramp_resolve
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

  .globl XFixesChangeCursor
  .p2align 4
  .type XFixesChangeCursor, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden XFixesChangeCursor
#endif
XFixesChangeCursor:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXfixes_so_tramp_table+0
  ldr ip0, [ip0, #:lo12:_libXfixes_so_tramp_table+0]
 
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
  bl _libXfixes_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXfixes_so_tramp_table+8
  ldr ip0, [ip0, #:lo12:_libXfixes_so_tramp_table+8]
 
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
  bl _libXfixes_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXfixes_so_tramp_table+16
  ldr ip0, [ip0, #:lo12:_libXfixes_so_tramp_table+16]
 
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
  bl _libXfixes_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXfixes_so_tramp_table+24
  ldr ip0, [ip0, #:lo12:_libXfixes_so_tramp_table+24]
 
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
  bl _libXfixes_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXfixes_so_tramp_table+32
  ldr ip0, [ip0, #:lo12:_libXfixes_so_tramp_table+32]
 
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
  bl _libXfixes_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXfixes_so_tramp_table+40
  ldr ip0, [ip0, #:lo12:_libXfixes_so_tramp_table+40]
 
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
  bl _libXfixes_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXfixes_so_tramp_table+48
  ldr ip0, [ip0, #:lo12:_libXfixes_so_tramp_table+48]
 
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
  bl _libXfixes_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXfixes_so_tramp_table+56
  ldr ip0, [ip0, #:lo12:_libXfixes_so_tramp_table+56]
 
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
  bl _libXfixes_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXfixes_so_tramp_table+64
  ldr ip0, [ip0, #:lo12:_libXfixes_so_tramp_table+64]
 
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
  bl _libXfixes_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXfixes_so_tramp_table+72
  ldr ip0, [ip0, #:lo12:_libXfixes_so_tramp_table+72]
 
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
  bl _libXfixes_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXfixes_so_tramp_table+80
  ldr ip0, [ip0, #:lo12:_libXfixes_so_tramp_table+80]
 
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
  bl _libXfixes_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXfixes_so_tramp_table+88
  ldr ip0, [ip0, #:lo12:_libXfixes_so_tramp_table+88]
 
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
  bl _libXfixes_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXfixes_so_tramp_table+96
  ldr ip0, [ip0, #:lo12:_libXfixes_so_tramp_table+96]
 
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
  bl _libXfixes_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXfixes_so_tramp_table+104
  ldr ip0, [ip0, #:lo12:_libXfixes_so_tramp_table+104]
 
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
  bl _libXfixes_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXfixes_so_tramp_table+112
  ldr ip0, [ip0, #:lo12:_libXfixes_so_tramp_table+112]
 
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
  bl _libXfixes_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXfixes_so_tramp_table+120
  ldr ip0, [ip0, #:lo12:_libXfixes_so_tramp_table+120]
 
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
  bl _libXfixes_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXfixes_so_tramp_table+128
  ldr ip0, [ip0, #:lo12:_libXfixes_so_tramp_table+128]
 
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
  bl _libXfixes_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXfixes_so_tramp_table+136
  ldr ip0, [ip0, #:lo12:_libXfixes_so_tramp_table+136]
 
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
  bl _libXfixes_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXfixes_so_tramp_table+144
  ldr ip0, [ip0, #:lo12:_libXfixes_so_tramp_table+144]
 
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
  bl _libXfixes_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXfixes_so_tramp_table+152
  ldr ip0, [ip0, #:lo12:_libXfixes_so_tramp_table+152]
 
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
  bl _libXfixes_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXfixes_so_tramp_table+160
  ldr ip0, [ip0, #:lo12:_libXfixes_so_tramp_table+160]
 
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
  bl _libXfixes_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXfixes_so_tramp_table+168
  ldr ip0, [ip0, #:lo12:_libXfixes_so_tramp_table+168]
 
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
  bl _libXfixes_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXfixes_so_tramp_table+176
  ldr ip0, [ip0, #:lo12:_libXfixes_so_tramp_table+176]
 
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
  bl _libXfixes_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXfixes_so_tramp_table+184
  ldr ip0, [ip0, #:lo12:_libXfixes_so_tramp_table+184]
 
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
  bl _libXfixes_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXfixes_so_tramp_table+192
  ldr ip0, [ip0, #:lo12:_libXfixes_so_tramp_table+192]
 
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
  bl _libXfixes_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXfixes_so_tramp_table+200
  ldr ip0, [ip0, #:lo12:_libXfixes_so_tramp_table+200]
 
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
  bl _libXfixes_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXfixes_so_tramp_table+208
  ldr ip0, [ip0, #:lo12:_libXfixes_so_tramp_table+208]
 
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
  bl _libXfixes_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXfixes_so_tramp_table+216
  ldr ip0, [ip0, #:lo12:_libXfixes_so_tramp_table+216]
 
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
  bl _libXfixes_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXfixes_so_tramp_table+224
  ldr ip0, [ip0, #:lo12:_libXfixes_so_tramp_table+224]
 
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
  bl _libXfixes_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXfixes_so_tramp_table+232
  ldr ip0, [ip0, #:lo12:_libXfixes_so_tramp_table+232]
 
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
  bl _libXfixes_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXfixes_so_tramp_table+240
  ldr ip0, [ip0, #:lo12:_libXfixes_so_tramp_table+240]
 
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
  bl _libXfixes_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXfixes_so_tramp_table+248
  ldr ip0, [ip0, #:lo12:_libXfixes_so_tramp_table+248]
 
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
  bl _libXfixes_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXfixes_so_tramp_table+256
  ldr ip0, [ip0, #:lo12:_libXfixes_so_tramp_table+256]
 
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
  bl _libXfixes_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXfixes_so_tramp_table+264
  ldr ip0, [ip0, #:lo12:_libXfixes_so_tramp_table+264]
 
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
  bl _libXfixes_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXfixes_so_tramp_table+272
  ldr ip0, [ip0, #:lo12:_libXfixes_so_tramp_table+272]
 
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
  bl _libXfixes_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXfixes_so_tramp_table+280
  ldr ip0, [ip0, #:lo12:_libXfixes_so_tramp_table+280]
 
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
  bl _libXfixes_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXfixes_so_tramp_table+288
  ldr ip0, [ip0, #:lo12:_libXfixes_so_tramp_table+288]
 
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
  bl _libXfixes_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXfixes_so_tramp_table+296
  ldr ip0, [ip0, #:lo12:_libXfixes_so_tramp_table+296]
 
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
  bl _libXfixes_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

