//===-- Portable attributes -------------------------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//
// This header file defines macros for declaring attributes for functions,
// types, and variables.
//
// These macros are used within llvm-libc and allow the compiler to optimize,
// where applicable, certain function calls.
//
// Most macros here are exposing GCC or Clang features, and are stubbed out for
// other compilers.

#ifndef LLVM_LIBC_SRC___SUPPORT_MACROS_ATTRIBUTES_H
#define LLVM_LIBC_SRC___SUPPORT_MACROS_ATTRIBUTES_H

#include "properties/architectures.h"

#ifndef __has_attribute
#define __has_attribute(x) 0
#endif

#define LIBC_INLINE inline
#define LIBC_INLINE_VAR inline
#define LIBC_INLINE_ASM __asm__ __volatile__
#define LIBC_UNUSED __attribute__((unused))

#ifdef LIBC_TARGET_ARCH_IS_GPU
#define LIBC_THREAD_LOCAL
#else
#define LIBC_THREAD_LOCAL thread_local
#endif

#if __cplusplus >= 202002L
#define LIBC_CONSTINIT constinit
#elif __has_attribute(__require_constant_initialization__)
#define LIBC_CONSTINIT __attribute__((__require_constant_initialization__))
#else
#define LIBC_CONSTINIT
#endif

#if defined(__clang__) && __has_attribute(preferred_type)
#define LIBC_PREFERED_TYPE(TYPE) [[clang::preferred_type(TYPE)]]
#else
#define LIBC_PREFERED_TYPE(TYPE)
#endif

#endif // LLVM_LIBC_SRC___SUPPORT_MACROS_ATTRIBUTES_H
