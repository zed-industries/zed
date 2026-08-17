//===---- TmHelper.h ------------------------------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_TEST_SRC_TIME_TMHELPER_H
#define LLVM_LIBC_TEST_SRC_TIME_TMHELPER_H

#include "hdr/types/struct_tm.h"
#include "src/__support/macros/config.h"
#include "src/time/time_constants.h"

namespace LIBC_NAMESPACE_DECL {
namespace tmhelper {
namespace testing {

// A helper function to initialize tm data structure.
static inline void initialize_tm_data(struct tm *tm_data, int year, int month,
                                      int mday, int hour, int min, int sec,
                                      int wday, int yday) {
  struct tm temp = {.tm_sec = sec,
                    .tm_min = min,
                    .tm_hour = hour,
                    .tm_mday = mday,
                    .tm_mon = month - 1, // tm_mon starts with 0 for Jan
                    // years since 1900
                    .tm_year = year - time_constants::TIME_YEAR_BASE,
                    .tm_wday = wday,
                    .tm_yday = yday,
                    .tm_isdst = 0};
  *tm_data = temp;
}

} // namespace testing
} // namespace tmhelper
} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_TEST_SRC_TIME_TMHELPER_H
