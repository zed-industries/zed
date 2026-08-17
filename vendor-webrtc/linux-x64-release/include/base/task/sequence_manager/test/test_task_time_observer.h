// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TASK_SEQUENCE_MANAGER_TEST_TEST_TASK_TIME_OBSERVER_H_
#define BASE_TASK_SEQUENCE_MANAGER_TEST_TEST_TASK_TIME_OBSERVER_H_

#include "base/task/sequence_manager/task_time_observer.h"
#include "base/time/time.h"

namespace base {
namespace sequence_manager {

class TestTaskTimeObserver : public TaskTimeObserver {
 public:
  void WillProcessTask(TimeTicks start_time) override {}
  void DidProcessTask(TimeTicks start_time, TimeTicks end_time) override {}
};

}  // namespace sequence_manager
}  // namespace base

#endif  // BASE_TASK_SEQUENCE_MANAGER_TEST_TEST_TASK_TIME_OBSERVER_H_
