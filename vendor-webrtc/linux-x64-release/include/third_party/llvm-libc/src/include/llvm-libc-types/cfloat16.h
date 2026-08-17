//===-- Definition of cfloat16 type ---------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_TYPES_CFLOAT16_H
#define LLVM_LIBC_TYPES_CFLOAT16_H

#if defined(__FLT16_MANT_DIG__) &&                                             \
    (!defined(__GNUC__) || __GNUC__ >= 13 ||                                   \
     (defined(__clang__) && __clang_major__ >= 14)) &&                         \
    !defined(__arm__) && !defined(_M_ARM) && !defined(__riscv) &&              \
    !defined(_WIN32)
#define LIBC_TYPES_HAS_CFLOAT16
typedef _Complex _Float16 cfloat16;
#endif

#endif // LLVM_LIBC_TYPES_CFLOAT16_H
