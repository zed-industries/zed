// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_PUBLIC_NON_MAIN_THREAD_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_PUBLIC_NON_MAIN_THREAD_H_

#include "base/task/single_thread_task_runner.h"
#include "third_party/blink/renderer/platform/scheduler/public/thread.h"

namespace blink {

// The interface of a non-main thread in Blink.
//
// This class will have an unrestricted GetTaskRunner method, anyone can use
// it. For main thread a frame based task runner should likely be used,
// so `Thread::GetTaskRunner` will eventually be removed or restricted via a
// pass key.
//
class PLATFORM_EXPORT NonMainThread : public Thread {
 public:
  // Creates a new non-main thread. This may be called from a non-main thread
  // (e.g. nested Web workers).
  static std::unique_ptr<NonMainThread> CreateThread(
      const ThreadCreationParams&);

  // Default task runner for this non-main thread.
  virtual scoped_refptr<base::SingleThreadTaskRunner> GetTaskRunner() const {
    return nullptr;
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_PUBLIC_NON_MAIN_THREAD_H_
