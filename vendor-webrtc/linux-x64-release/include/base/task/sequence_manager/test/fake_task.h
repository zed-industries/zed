// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TASK_SEQUENCE_MANAGER_TEST_FAKE_TASK_H_
#define BASE_TASK_SEQUENCE_MANAGER_TEST_FAKE_TASK_H_

#include "base/task/sequence_manager/task_queue.h"
#include "base/task/sequence_manager/tasks.h"

namespace base {
namespace sequence_manager {

class FakeTask : public Task {
 public:
  FakeTask();
  explicit FakeTask(TaskType task_type);
};

class FakeTaskTiming : public TaskQueue::TaskTiming {
 public:
  FakeTaskTiming();
  FakeTaskTiming(TimeTicks start, TimeTicks end);
};

}  // namespace sequence_manager
}  // namespace base

#endif  // BASE_TASK_SEQUENCE_MANAGER_TEST_FAKE_TASK_H_
