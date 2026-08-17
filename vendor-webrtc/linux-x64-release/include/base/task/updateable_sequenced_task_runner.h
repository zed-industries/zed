// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TASK_UPDATEABLE_SEQUENCED_TASK_RUNNER_H_
#define BASE_TASK_UPDATEABLE_SEQUENCED_TASK_RUNNER_H_

#include "base/base_export.h"
#include "base/task/sequenced_task_runner.h"
#include "base/task/task_traits.h"

namespace base {

// A SequencedTaskRunner whose posted tasks' priorities can be updated.
class BASE_EXPORT UpdateableSequencedTaskRunner : public SequencedTaskRunner {
 public:
  UpdateableSequencedTaskRunner(const UpdateableSequencedTaskRunner&) = delete;
  UpdateableSequencedTaskRunner& operator=(
      const UpdateableSequencedTaskRunner&) = delete;
  // Updates the priority for tasks posted through this TaskRunner to
  // |priority|.
  virtual void UpdatePriority(TaskPriority priority) = 0;

 protected:
  UpdateableSequencedTaskRunner() = default;
  ~UpdateableSequencedTaskRunner() override = default;
};

}  // namespace base

#endif  // BASE_TASK_UPDATEABLE_SEQUENCED_TASK_RUNNER_H_
