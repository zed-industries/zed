// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_TEST_WEB_SCHEDULING_TEST_HELPER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_TEST_WEB_SCHEDULING_TEST_HELPER_H_

#include <memory>
#include <variant>

#include "base/memory/raw_ref.h"
#include "base/memory/scoped_refptr.h"
#include "base/task/single_thread_task_runner.h"
#include "base/time/time.h"
#include "third_party/blink/public/platform/task_type.h"
#include "third_party/blink/renderer/platform/scheduler/public/web_scheduling_priority.h"
#include "third_party/blink/renderer/platform/scheduler/public/web_scheduling_queue_type.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {
class FrameOrWorkerScheduler;
class WebSchedulingTaskQueue;
}  // namespace blink

namespace blink::scheduler {

// Helper class for running tests using `WebSchedulingTaskQueue`s. This manages
// one task queue and continuation queue for each priority, and test tasks can
// be scheduled on the queues using `PostTestTasks()`. Queues can be accessed,
// e.g. to change priority, using `GetWebSchedulingTaskQueue()`.
class WebSchedulingTestHelper {
 public:
  // Note: GetTaskRunner() is not on a shared interface between worker and
  // frame schedulers, but is needed for tests.
  class Delegate {
   public:
    virtual scoped_refptr<base::SingleThreadTaskRunner> GetTaskRunner(
        TaskType) = 0;
    virtual FrameOrWorkerScheduler& GetFrameOrWorkerScheduler() = 0;
  };

  struct WebSchedulingParams {
    WebSchedulingQueueType queue_type;
    WebSchedulingPriority priority;
  };

  struct TestTaskSpecEntry {
    String descriptor;
    std::variant<TaskType, WebSchedulingParams> type_info;
    base::TimeDelta delay = base::TimeDelta();
  };

  explicit WebSchedulingTestHelper(Delegate& delegate);
  WebSchedulingTestHelper(const WebSchedulingTestHelper&) = delete;
  WebSchedulingTestHelper& operator=(const WebSchedulingTestHelper&) = delete;
  ~WebSchedulingTestHelper();

  WebSchedulingTaskQueue* GetWebSchedulingTaskQueue(WebSchedulingQueueType,
                                                    WebSchedulingPriority);

  // Posts a task that appends descriptor to `run_order` for each entry in
  // `test_spec`. This only schedules the tasks, it does not run them.
  void PostTestTasks(Vector<String>* run_order,
                     const Vector<TestTaskSpecEntry>& test_spec);

 private:
  const raw_ref<Delegate> delegate_;
  Vector<std::unique_ptr<WebSchedulingTaskQueue>> task_queues_;
  Vector<std::unique_ptr<WebSchedulingTaskQueue>> continuation_task_queues_;
};

}  // namespace blink::scheduler

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_TEST_WEB_SCHEDULING_TEST_HELPER_H_
