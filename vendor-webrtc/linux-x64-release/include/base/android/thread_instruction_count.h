// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_ANDROID_THREAD_INSTRUCTION_COUNT_H_
#define BASE_ANDROID_THREAD_INSTRUCTION_COUNT_H_

#include <stdint.h>

#include "base/base_export.h"

namespace base {
namespace android {

// Represents the number of instructions that were retired between two samples
// of a thread's performance counters.
class BASE_EXPORT ThreadInstructionDelta {
 public:
  constexpr ThreadInstructionDelta() : delta_(0) {}
  explicit constexpr ThreadInstructionDelta(int64_t delta) : delta_(delta) {}

  constexpr int64_t ToInternalValue() const { return delta_; }

 private:
  int64_t delta_;
};

// Helper class for reading the current count of instructions retired for the
// current thread via ThreadInstructionCount::Now(). Does *not* count
// instructions retired while running in the kernel.
//
// Limitations:
// * Crashes when used in sandboxed process
// * Works on a userdebug build of Android 12, kernel 4.19. May require extra
//   effort to allow on later Android releases and kernel versions.
class BASE_EXPORT ThreadInstructionCount {
 public:
  // Returns true if the platform supports hardware retired instruction
  // counters. May crash in sandboxed processes.
  static bool IsSupported();

  // Returns the number of retired instructions relative to some epoch count,
  // or 0 if getting the current instruction count failed / is disabled.
  static ThreadInstructionCount Now();

  explicit constexpr ThreadInstructionCount(uint64_t value = 0)
      : value_(value) {}

  constexpr ThreadInstructionDelta operator-(
      ThreadInstructionCount other) const {
    return ThreadInstructionDelta(static_cast<int64_t>(value_ - other.value_));
  }

  constexpr uint64_t ToInternalValue() const { return value_; }

 private:
  uint64_t value_;
};

}  // namespace android
}  // namespace base

#endif  // BASE_ANDROID_THREAD_INSTRUCTION_COUNT_H_
