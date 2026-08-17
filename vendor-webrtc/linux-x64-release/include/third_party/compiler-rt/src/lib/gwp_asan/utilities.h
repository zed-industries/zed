//===-- utilities.h ---------------------------------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef GWP_ASAN_UTILITIES_H_
#define GWP_ASAN_UTILITIES_H_

#include "gwp_asan/definitions.h"

#include <stddef.h>
#include <stdint.h>

namespace gwp_asan {
// Terminates in a platform-specific way with `Message`.
void die(const char *Message);
void dieWithErrorCode(const char *Message, int64_t ErrorCode);

// Checks that `Condition` is true, otherwise dies with `Message`.
GWP_ASAN_ALWAYS_INLINE void check(bool Condition, const char *Message) {
  if (GWP_ASAN_LIKELY(Condition))
    return;
  die(Message);
}

// Checks that `Condition` is true, otherwise dies with `Message` (including
// errno at the end).
GWP_ASAN_ALWAYS_INLINE void
checkWithErrorCode(bool Condition, const char *Message, int64_t ErrorCode) {
  if (GWP_ASAN_LIKELY(Condition))
    return;
  dieWithErrorCode(Message, ErrorCode);
}
} // namespace gwp_asan

#endif // GWP_ASAN_UTILITIES_H_
