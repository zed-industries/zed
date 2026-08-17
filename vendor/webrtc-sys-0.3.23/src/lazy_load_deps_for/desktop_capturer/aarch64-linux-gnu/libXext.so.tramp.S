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

  bl _libXext_so_tramp_resolve
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

  .globl DPMSCapable
  .p2align 4
  .type DPMSCapable, %function
#ifndef IMPLIB_EXPORT_SHIMS
  .hidden DPMSCapable
#endif
DPMSCapable:
  .cfi_startproc

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+0
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+0]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+8
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+8]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+16
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+16]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+24
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+24]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+32
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+32]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+40
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+40]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+48
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+48]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+56
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+56]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+64
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+64]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+72
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+72]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+80
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+80]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+88
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+88]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+96
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+96]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+104
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+104]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+112
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+112]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+120
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+120]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+128
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+128]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+136
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+136]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+144
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+144]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+152
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+152]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+160
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+160]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+168
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+168]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+176
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+176]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+184
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+184]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+192
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+192]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+200
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+200]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+208
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+208]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+216
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+216]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+224
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+224]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+232
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+232]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+240
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+240]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+248
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+248]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+256
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+256]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+264
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+264]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+272
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+272]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+280
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+280]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+288
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+288]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+296
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+296]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+304
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+304]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+312
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+312]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+320
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+320]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+328
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+328]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+336
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+336]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+344
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+344]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+352
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+352]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+360
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+360]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+368
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+368]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+376
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+376]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+384
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+384]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+392
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+392]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+400
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+400]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+408
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+408]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+416
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+416]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+424
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+424]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+432
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+432]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+440
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+440]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+448
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+448]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+456
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+456]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+464
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+464]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+472
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+472]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+480
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+480]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+488
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+488]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+496
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+496]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+504
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+504]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+512
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+512]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+520
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+520]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+528
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+528]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+536
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+536]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+544
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+544]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+552
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+552]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+560
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+560]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+568
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+568]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+576
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+576]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+584
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+584]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+592
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+592]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+600
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+600]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+608
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+608]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+616
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+616]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+624
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+624]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+632
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+632]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+640
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+640]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+648
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+648]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+656
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+656]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+664
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+664]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+672
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+672]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+680
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+680]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+688
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+688]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+696
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+696]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+704
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+704]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+712
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+712]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+720
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+720]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+728
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+728]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+736
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+736]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+744
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+744]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+752
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+752]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+760
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+760]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+768
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+768]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+776
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+776]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+784
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+784]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+792
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+792]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+800
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+800]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+808
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+808]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+816
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+816]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+824
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+824]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+832
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+832]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+840
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+840]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+848
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+848]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+856
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+856]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+864
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+864]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+872
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+872]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+880
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+880]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+888
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+888]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+896
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+896]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+904
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+904]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+912
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+912]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+920
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+920]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+928
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+928]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+936
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+936]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+944
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+944]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+952
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+952]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+960
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+960]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+968
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+968]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+976
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+976]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+984
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+984]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+992
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+992]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+1000
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+1000]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+1008
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+1008]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+1016
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+1016]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+1024
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+1024]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+1032
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+1032]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+1040
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+1040]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
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

1:
  // Load address
  // TODO: can we do this faster on newer ARMs?
  adrp ip0, _libXext_so_tramp_table+1048
  ldr ip0, [ip0, #:lo12:_libXext_so_tramp_table+1048]
 
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
  bl _libXext_so_save_regs_and_resolve
  ldp xzr, lr, [sp], #16; .cfi_adjust_cfa_offset -16; .cfi_restore lr
  br ip0
  .cfi_endproc

