//===------------------------ rtsan_flags.inc -------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//
//
// RTSan runtime flags.
//
//===----------------------------------------------------------------------===//
#ifndef RTSAN_FLAG
#error "Define RTSAN_FLAG prior to including this file!"
#endif

// RTSAN_FLAG(Type, Name, DefaultValue, Description)
// See COMMON_FLAG in sanitizer_flags.inc for more details.

RTSAN_FLAG(bool, halt_on_error, true, "Exit after first reported error.")
RTSAN_FLAG(bool, print_stats_on_exit, false, "Print stats on exit.")
RTSAN_FLAG(const char *, suppressions, "", "Suppressions file name.")
RTSAN_FLAG(bool, suppress_equal_stacks, true,
           "Suppress a report if we've already output another report "
           "with the same stack.")
