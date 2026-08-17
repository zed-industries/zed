// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_PUBLIC_COMPOSITOR_THREAD_SCHEDULER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_PUBLIC_COMPOSITOR_THREAD_SCHEDULER_H_

#include "base/task/single_thread_task_runner.h"
#include "third_party/blink/renderer/platform/scheduler/public/thread_scheduler.h"

namespace blink {

// This class is used to submit tasks and pass other information from Blink to
// the platform's compositor thread scheduler.
class PLATFORM_EXPORT CompositorThreadScheduler : public ThreadScheduler {
 public:
  // Returns a task runner for input-blocking tasks on the compositor thread.
  virtual scoped_refptr<base::SingleThreadTaskRunner> InputTaskRunner() = 0;

  // Returns a task runner for compositor tasks.
  virtual scoped_refptr<base::SingleThreadTaskRunner> DefaultTaskRunner() = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_PUBLIC_COMPOSITOR_THREAD_SCHEDULER_H_
