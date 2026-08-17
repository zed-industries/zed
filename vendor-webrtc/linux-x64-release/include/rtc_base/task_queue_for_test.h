/*
 *  Copyright 2018 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_BASE_TASK_QUEUE_FOR_TEST_H_
#define RTC_BASE_TASK_QUEUE_FOR_TEST_H_

#include <utility>

#include "absl/cleanup/cleanup.h"
#include "absl/strings/string_view.h"
#include "api/function_view.h"
#include "api/task_queue/task_queue_base.h"
#include "api/task_queue/task_queue_factory.h"
#include "rtc_base/checks.h"
#include "rtc_base/event.h"

namespace webrtc {

inline void SendTask(TaskQueueBase* task_queue, FunctionView<void()> task) {
  if (task_queue->IsCurrent()) {
    task();
    return;
  }

  Event event;
  absl::Cleanup cleanup = [&event] { event.Set(); };
  task_queue->PostTask([task, cleanup = std::move(cleanup)] { task(); });
  RTC_CHECK(event.Wait(/*give_up_after=*/Event::kForever,
                       /*warn_after=*/TimeDelta::Seconds(10)));
}

class TaskQueueForTest {
 public:
  explicit TaskQueueForTest(
      std::unique_ptr<TaskQueueBase, TaskQueueDeleter> task_queue);
  explicit TaskQueueForTest(
      absl::string_view name = "TestQueue",
      TaskQueueFactory::Priority priority = TaskQueueFactory::Priority::NORMAL);
  TaskQueueForTest(const TaskQueueForTest&) = delete;
  TaskQueueForTest& operator=(const TaskQueueForTest&) = delete;
  ~TaskQueueForTest();

  bool IsCurrent() const { return impl_->IsCurrent(); }

  // Returns non-owning pointer to the task queue implementation.
  TaskQueueBase* Get() { return impl_.get(); }

  void PostTask(
      absl::AnyInvocable<void() &&> task,
      const webrtc::Location& location = webrtc::Location::Current()) {
    impl_->PostTask(std::move(task), location);
  }
  void PostDelayedTask(
      absl::AnyInvocable<void() &&> task,
      webrtc::TimeDelta delay,
      const webrtc::Location& location = webrtc::Location::Current()) {
    impl_->PostDelayedTask(std::move(task), delay, location);
  }
  void PostDelayedHighPrecisionTask(
      absl::AnyInvocable<void() &&> task,
      webrtc::TimeDelta delay,
      const webrtc::Location& location = webrtc::Location::Current()) {
    impl_->PostDelayedHighPrecisionTask(std::move(task), delay, location);
  }

  // A convenience, test-only method that blocks the current thread while
  // a task executes on the task queue.
  void SendTask(FunctionView<void()> task) { ::webrtc::SendTask(Get(), task); }

  // Wait for the completion of all tasks posted prior to the
  // WaitForPreviouslyPostedTasks() call.
  void WaitForPreviouslyPostedTasks() {
    RTC_DCHECK(!Get()->IsCurrent());
    // Post an empty task on the queue and wait for it to finish, to ensure
    // that all already posted tasks on the queue get executed.
    SendTask([]() {});
  }

 private:
  std::unique_ptr<TaskQueueBase, TaskQueueDeleter> impl_;
};

}  // namespace webrtc

#endif  // RTC_BASE_TASK_QUEUE_FOR_TEST_H_
