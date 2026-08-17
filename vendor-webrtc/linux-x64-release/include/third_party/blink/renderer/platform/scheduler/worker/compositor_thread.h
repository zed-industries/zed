// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_WORKER_COMPOSITOR_THREAD_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_WORKER_COMPOSITOR_THREAD_H_

#include "third_party/blink/renderer/platform/scheduler/worker/non_main_thread_impl.h"

namespace blink {
namespace scheduler {

class PLATFORM_EXPORT CompositorThread : public NonMainThreadImpl {
 public:
  explicit CompositorThread(const ThreadCreationParams& params);
  CompositorThread(const CompositorThread&) = delete;
  CompositorThread& operator=(const CompositorThread&) = delete;
  ~CompositorThread() override;

 private:
  std::unique_ptr<NonMainThreadSchedulerBase> CreateNonMainThreadScheduler(
      base::sequence_manager::SequenceManager* sequence_manager) override;
};

}  // namespace scheduler
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_WORKER_COMPOSITOR_THREAD_H_
