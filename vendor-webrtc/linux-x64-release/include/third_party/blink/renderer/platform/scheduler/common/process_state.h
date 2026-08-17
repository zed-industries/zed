// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_COMMON_PROCESS_STATE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_COMMON_PROCESS_STATE_H_

#include <atomic>

#include "third_party/blink/renderer/platform/platform_export.h"

namespace blink {
namespace scheduler {
namespace internal {

// Helper lock-free struct to share main state of the process between threads
// for recording metrics.
// This class should not be used for synchronization between threads.
struct PLATFORM_EXPORT ProcessState {
  static ProcessState* Get();

  std::atomic_bool is_process_backgrounded;
};

}  // namespace internal
}  // namespace scheduler
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_COMMON_PROCESS_STATE_H_
