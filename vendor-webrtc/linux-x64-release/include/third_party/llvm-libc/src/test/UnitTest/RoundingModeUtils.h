//===-- RoundingModeUtils.h -------------------------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_TEST_UNITTEST_ROUNDINGMODEUTILS_H
#define LLVM_LIBC_TEST_UNITTEST_ROUNDINGMODEUTILS_H

#include "src/__support/macros/config.h"
#include <stdint.h>

namespace LIBC_NAMESPACE_DECL {
namespace fputil {
namespace testing {

enum class RoundingMode : uint8_t { Upward, Downward, TowardZero, Nearest };

struct ForceRoundingMode {
  ForceRoundingMode(RoundingMode);
  ~ForceRoundingMode();

  int old_rounding_mode;
  int rounding_mode;
  bool success;
};

template <RoundingMode R> struct ForceRoundingModeTest : ForceRoundingMode {
  ForceRoundingModeTest() : ForceRoundingMode(R) {}
};

} // namespace testing
} // namespace fputil
} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_TEST_UNITTEST_ROUNDINGMODEUTILS_H
