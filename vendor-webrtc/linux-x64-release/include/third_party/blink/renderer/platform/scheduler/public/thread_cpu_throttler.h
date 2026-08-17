// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_PUBLIC_THREAD_CPU_THROTTLER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_PUBLIC_THREAD_CPU_THROTTLER_H_

#include <memory>

#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace base {
template <typename T>
struct DefaultSingletonTraits;
}

namespace blink {
namespace scheduler {

// This class is used to slow down the main thread for
// inspector "cpu throttling". It does it by spawning an
// additional thread which frequently interrupts main thread
// and sleeps.
class PLATFORM_EXPORT ThreadCPUThrottler final {
  USING_FAST_MALLOC(ThreadCPUThrottler);

 public:
  static ThreadCPUThrottler* GetInstance();

  ThreadCPUThrottler(const ThreadCPUThrottler&) = delete;
  ThreadCPUThrottler& operator=(const ThreadCPUThrottler&) = delete;

  // |rate| is a slow-down factor - passing 2.0 will make
  // everything two times slower.
  // Any rate less or equal to 1.0 disables throttling and
  // cleans up helper thread.
  void SetThrottlingRate(double rate);

 private:
  ThreadCPUThrottler();
  ~ThreadCPUThrottler();
  friend struct base::DefaultSingletonTraits<ThreadCPUThrottler>;

  class ThrottlingThread;
  std::unique_ptr<ThrottlingThread> throttling_thread_;
};

}  // namespace scheduler
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_PUBLIC_THREAD_CPU_THROTTLER_H_
