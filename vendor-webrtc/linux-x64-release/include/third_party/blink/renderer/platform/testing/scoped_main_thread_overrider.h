// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_TESTING_SCOPED_MAIN_THREAD_OVERRIDER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_TESTING_SCOPED_MAIN_THREAD_OVERRIDER_H_

#include <memory>
#include "third_party/blink/renderer/platform/scheduler/public/main_thread.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

// This class instance lets you override the main thread (of type blink::Thread)
// temporarily. This is useful when you want to change the main thread scheduler
// during a test (see ScopedSchedulerOverrider).
//
// When ScopedMainThreadOverrider goes out of scope, the main thread is
// reverted back to the original one, and the main thread object passed in
// the constructor gets destructed.

class ScopedMainThreadOverrider final {
  USING_FAST_MALLOC(ScopedMainThreadOverrider);

 public:
  explicit ScopedMainThreadOverrider(std::unique_ptr<MainThread> main_thread);
  ~ScopedMainThreadOverrider();

 private:
  std::unique_ptr<MainThread> original_main_thread_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_TESTING_SCOPED_MAIN_THREAD_OVERRIDER_H_
