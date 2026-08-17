//===-- C standard library header test_small-------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_TEST_SMALL_H
#define LLVM_LIBC_TEST_SMALL_H

#include "__llvm-libc-common.h"
#include "llvm-libc-macros/float16-macros.h"

#include "llvm-libc-macros/test_more-macros.h"
#include "llvm-libc-macros/test_small-macros.h"
#include "llvm-libc-types/float128.h"
#include "llvm-libc-types/type_a.h"
#include "llvm-libc-types/type_b.h"

#define MACRO_A 1

#define MACRO_B 2

#define MACRO_C

enum {
  enum_a = value_1,
  enum_b = value_2,
};

__BEGIN_C_DECLS

CONST_FUNC_A void func_a(void) __NOEXCEPT;

#ifdef LIBC_TYPES_HAS_FLOAT128
float128 func_b(void) __NOEXCEPT;
#endif // LIBC_TYPES_HAS_FLOAT128

#ifdef LIBC_TYPES_HAS_FLOAT16
_Float16 func_c(int, float) __NOEXCEPT;

_Float16 func_d(int, float) __NOEXCEPT;
#endif // LIBC_TYPES_HAS_FLOAT16

#ifdef LIBC_TYPES_HAS_FLOAT16_AND_FLOAT128
_Float16 func_e(float128) __NOEXCEPT;
#endif // LIBC_TYPES_HAS_FLOAT16_AND_FLOAT128

extern obj object_1;
extern obj object_2;

__END_C_DECLS

#endif // LLVM_LIBC_TEST_SMALL_H
