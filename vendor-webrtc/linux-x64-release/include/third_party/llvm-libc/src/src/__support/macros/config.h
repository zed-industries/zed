//===-- Portable attributes -------------------------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//
// This header file defines a set of macros for checking the presence of
// important compiler and platform features. Such macros can be used to
// produce portable code by parameterizing compilation based on the presence or
// lack of a given feature.

#ifndef LLVM_LIBC_SRC___SUPPORT_MACROS_CONFIG_H
#define LLVM_LIBC_SRC___SUPPORT_MACROS_CONFIG_H

// Workaround for compilers that do not support builtin detection.
// FIXME: This is only required for the GPU portion which should be moved.
#ifndef __has_builtin
#define __has_builtin(b) 0
#endif

// Compiler feature-detection.
// clang.llvm.org/docs/LanguageExtensions.html#has-feature-and-has-extension
#ifdef __has_feature
#define LIBC_HAS_FEATURE(f) __has_feature(f)
#else
#define LIBC_HAS_FEATURE(f) 0
#endif

#ifdef __clang__
// Declare a LIBC_NAMESPACE with hidden visibility. `namespace
// LIBC_NAMESPACE_DECL {` should be used around all declarations and definitions
// for libc internals as opposed to just `namespace LIBC_NAMESPACE {`. This
// ensures that all declarations within this namespace have hidden
// visibility, which optimizes codegen for uses of symbols defined in other
// translation units in ways that can be necessary for correctness by avoiding
// dynamic relocations. This does not affect the public C symbols which are
// controlled independently via `LLVM_LIBC_FUNCTION_ATTR`.
#define LIBC_NAMESPACE_DECL [[gnu::visibility("hidden")]] LIBC_NAMESPACE
#else
// TODO(#98548): GCC emits a warning when using the visibility attribute which
// needs to be diagnosed and addressed.
#define LIBC_NAMESPACE_DECL LIBC_NAMESPACE
#endif

#endif // LLVM_LIBC_SRC___SUPPORT_MACROS_CONFIG_H
