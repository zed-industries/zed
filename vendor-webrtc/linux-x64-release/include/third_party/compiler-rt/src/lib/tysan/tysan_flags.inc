//===-- tysan_flags.inc ---------------------------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//
//
// TySan runtime flags.
//
//===----------------------------------------------------------------------===//
#ifndef TYSAN_FLAG
#error "Define TYSAN_FLAG prior to including this file!"
#endif

// TYSAN_FLAG(Type, Name, DefaultValue, Description)
// See COMMON_FLAG in sanitizer_flags.inc for more details.

TYSAN_FLAG(bool, print_stacktrace, false,
           "Include full stacktrace into an error report")
