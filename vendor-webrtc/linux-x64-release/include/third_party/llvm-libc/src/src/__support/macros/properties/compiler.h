//===-- Compile time compiler detection -------------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC___SUPPORT_MACROS_PROPERTIES_COMPILER_H
#define LLVM_LIBC_SRC___SUPPORT_MACROS_PROPERTIES_COMPILER_H

// Example usage of compiler version checks
// #if defined(LIBC_COMPILER_CLANG_VER)
// #  if LIBC_COMPILER_CLANG_VER < 1500
// #    warning "Libc only supports Clang 15 and later"
// #  endif
// #elif defined(LIBC_COMPILER_GCC_VER)
// #  if LIBC_COMPILER_GCC_VER < 1500
// #    warning "Libc only supports GCC 15 and later"
// #  endif
// #elif defined(LIBC_COMPILER_MSC_VER)
// #  if LIBC_COMPILER_MSC_VER < 1930
// #    warning "Libc only supports Visual Studio 2022 RTW (17.0) and later"
// #  endif
// #endif

#if defined(__clang__)
#define LIBC_COMPILER_IS_CLANG
#define LIBC_COMPILER_CLANG_VER (__clang_major__ * 100 + __clang_minor__)
#endif

#if defined(__GNUC__) && !defined(__clang__)
#define LIBC_COMPILER_IS_GCC
#define LIBC_COMPILER_GCC_VER (__GNUC__ * 100 + __GNUC_MINOR__)
#endif

#if defined(_MSC_VER)
#define LIBC_COMPILER_IS_MSC
// https://learn.microsoft.com/en-us/cpp/preprocessor/predefined-macros
#define LIBC_COMPILER_MSC_VER (_MSC_VER)
#endif

#endif // LLVM_LIBC_SRC___SUPPORT_MACROS_PROPERTIES_COMPILER_H
