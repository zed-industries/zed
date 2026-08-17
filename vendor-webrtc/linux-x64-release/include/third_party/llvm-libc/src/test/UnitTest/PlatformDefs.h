//===-- Platform specific defines for the unittest library ------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_TEST_UNITTEST_PLATFORMDEFS_H
#define LLVM_LIBC_TEST_UNITTEST_PLATFORMDEFS_H

#if !defined(_WIN32)
#define ENABLE_SUBPROCESS_TESTS
#endif

#endif // LLVM_LIBC_TEST_UNITTEST_PLATFORMDEFS_H
