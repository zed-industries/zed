/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_BASE_SYSTEM_ASM_DEFINES_H_
#define RTC_BASE_SYSTEM_ASM_DEFINES_H_

// clang-format off
// clang formatting breaks everything here, e.g. concatenating directives,
// due to absence of context via asm keyword.

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif

// Define the macros used in ARM assembly code, so that for Mac or iOS builds
// we add leading underscores for the function names.
#ifdef __APPLE__
.macro GLOBAL_FUNCTION name
.global _\name
.private_extern _\name
.endm
.macro DEFINE_FUNCTION name
_\name:
.endm
.macro CALL_FUNCTION name
bl _\name
.endm
.macro GLOBAL_LABEL name
.global _\name
.private_extern _\name
.endm
#else
.macro GLOBAL_FUNCTION name
.global \name
.hidden \name
.endm
.macro DEFINE_FUNCTION name
#if defined(__linux__) && defined(__ELF__)
.type \name,%function
#endif
\name:
.endm
.macro CALL_FUNCTION name
bl \name
.endm
.macro GLOBAL_LABEL name
.global \name
.hidden \name
.endm
#endif

// With Apple's clang compiler, for instructions ldrb, strh, etc.,
// the condition code is after the width specifier. Here we define
// only the ones that are actually used in the assembly files.
#if (defined __llvm__) && (defined __APPLE__)
.macro streqh reg1, reg2, num
strheq \reg1, \reg2, \num
.endm
#endif

.text

// clang-format on

#endif  // RTC_BASE_SYSTEM_ASM_DEFINES_H_
