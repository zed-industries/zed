// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_PROFILER_THREAD_DELEGATE_H_
#define BASE_PROFILER_THREAD_DELEGATE_H_

#include <vector>

#include "base/base_export.h"
#include "base/profiler/register_context.h"
#include "base/threading/platform_thread.h"

namespace base {

// Platform-specific thread and stack manipulation delegate, for use by the
// platform-independent stack copying/walking implementation in
// StackSamplerImpl. Provides the common interface across signal- and
// suspend-based stack copy implementations.
class BASE_EXPORT ThreadDelegate {
 public:
  ThreadDelegate() = default;
  virtual ~ThreadDelegate() = default;

  ThreadDelegate(const ThreadDelegate&) = delete;
  ThreadDelegate& operator=(const ThreadDelegate&) = delete;

  // Gets the platform-specific id for the thread.
  virtual PlatformThreadId GetThreadId() const = 0;

  // Gets the base address of the thread's stack.
  virtual uintptr_t GetStackBaseAddress() const = 0;

  // Returns a list of registers that should be rewritten to point into the
  // stack copy, if they originally pointed into the original stack.
  // May heap allocate.
  virtual std::vector<uintptr_t*> GetRegistersToRewrite(
      RegisterContext* thread_context) = 0;
};

}  // namespace base

#endif  // BASE_PROFILER_THREAD_DELEGATE_H_
