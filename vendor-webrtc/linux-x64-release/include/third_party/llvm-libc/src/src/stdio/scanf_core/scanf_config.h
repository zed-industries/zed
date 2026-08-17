//===-- Scanf Configuration Handler ----------------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC_STDIO_SCANF_CORE_SCANF_CONFIG_H
#define LLVM_LIBC_SRC_STDIO_SCANF_CORE_SCANF_CONFIG_H

// These macros can be set or unset to adjust scanf behavior at compile time.

// This flag disables all functionality relating to floating point numbers. This
// can be useful for embedded systems or other situations where binary size is
// important.
// #define LIBC_COPT_SCANF_DISABLE_FLOAT

// This flag disables index mode, a posix extension often used for
// internationalization of format strings. Supporting it takes up additional
// memory and parsing time, so it can be disabled if it's not used.
// #define LIBC_COPT_SCANF_DISABLE_INDEX_MODE

#endif // LLVM_LIBC_SRC_STDIO_SCANF_CORE_SCANF_CONFIG_H
