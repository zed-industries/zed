//===--- rtsan_test_utilities.h - Realtime Sanitizer ------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//
//
//===----------------------------------------------------------------------===//

#pragma once

#include "rtsan.h"
#include "gmock/gmock.h"
#include <string>

namespace rtsan_testing {

template <typename Function> void RealtimeInvoke(Function &&Func) {
  __rtsan_realtime_enter();
  std::forward<Function>(Func)();
  __rtsan_realtime_exit();
}

template <typename Function>
void ExpectRealtimeDeath(Function &&Func,
                         const char *intercepted_method_name = nullptr) {

  using namespace testing;

  auto GetExpectedErrorSubstring = [&]() -> std::string {
    return intercepted_method_name != nullptr
               ? ".*==ERROR: RealtimeSanitizer: unsafe-library-call.*"
                 "Intercepted call to real-time unsafe function `" +
                     std::string(intercepted_method_name) +
                     "` in real-time context!"
               : "";
  };

  EXPECT_EXIT(RealtimeInvoke(std::forward<Function>(Func)), ExitedWithCode(43),
              GetExpectedErrorSubstring());
}

template <typename Function> void ExpectNonRealtimeSurvival(Function &&Func) {
  std::forward<Function>(Func)();
}

} // namespace rtsan_testing
