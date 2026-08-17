// NOLINT(llvm-header-guard) https://github.com/llvm/llvm-project/issues/83339
//===-- Internal header for assert ------------------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#include "src/assert/__assert_fail.h"

// There is no header guard here since assert is intended to be capable of being
// included multiple times with NDEBUG defined differently, causing different
// behavior.

#undef assert

#ifdef NDEBUG
#define assert(e) (void)0
#else

#ifdef __has_builtin
#if __has_builtin(__builtin_expect)
#define __LIBC_ASSERT_LIKELY(e) __builtin_expect(e, 1)
#endif
#endif
#ifndef __LIBC_ASSERT_LIKELY
#define __LIBC_ASSERT_LIKELY(e) e
#endif

#define assert(e)                                                              \
  (__LIBC_ASSERT_LIKELY(e) ? (void)0                                           \
                           : LIBC_NAMESPACE::__assert_fail(                    \
                                 #e, __FILE__, __LINE__, __PRETTY_FUNCTION__))
#endif // NDEBUG
