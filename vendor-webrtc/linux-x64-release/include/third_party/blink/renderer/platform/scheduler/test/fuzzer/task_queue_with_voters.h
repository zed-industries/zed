// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_TEST_FUZZER_TASK_QUEUE_WITH_VOTERS_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_TEST_FUZZER_TASK_QUEUE_WITH_VOTERS_H_

#include "base/task/sequence_manager/task_queue.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/thread_safe_ref_counted.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace base {
namespace sequence_manager {

struct PLATFORM_EXPORT TaskQueueWithVoters
    : public ThreadSafeRefCounted<TaskQueueWithVoters> {
 public:
  explicit TaskQueueWithVoters(TaskQueue::Handle task_queue)
      : queue(std::move(task_queue)) {}

  TaskQueue::Handle queue;
  Vector<std::unique_ptr<TaskQueue::QueueEnabledVoter>> voters;

 private:
  friend ThreadSafeRefCounted<TaskQueueWithVoters>;
  ~TaskQueueWithVoters() = default;
};

}  // namespace sequence_manager
}  // namespace base

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_TEST_FUZZER_TASK_QUEUE_WITH_VOTERS_H_
