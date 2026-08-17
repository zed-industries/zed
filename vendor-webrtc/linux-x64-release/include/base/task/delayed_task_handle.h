// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TASK_DELAYED_TASK_HANDLE_H_
#define BASE_TASK_DELAYED_TASK_HANDLE_H_

#include <memory>

#include "base/base_export.h"

namespace base {

// A handle to a delayed task which can be used to cancel the posted task. Not
// thread-safe, can only be held and invoked from the posting sequence.
class BASE_EXPORT DelayedTaskHandle {
 public:
  // The delegate that allows each SequencedTaskRunners to have different
  // implementations.
  class Delegate {
   public:
    virtual ~Delegate() = default;

    // Returns true if the task handle is valid. Canceling or running the task
    // will mark it as invalid.
    virtual bool IsValid() const = 0;

    // Cancels the task. A canceled task, whether removed from the underlying
    // queue or only marked as canceled, will never be Run().
    virtual void CancelTask() = 0;
  };

  // Construct a default, invalid, task handle.
  DelayedTaskHandle();

  // Construct a valid task handle with the specified |delegate|.
  explicit DelayedTaskHandle(std::unique_ptr<Delegate> delegate);

  ~DelayedTaskHandle();

  DelayedTaskHandle(DelayedTaskHandle&&);
  DelayedTaskHandle& operator=(DelayedTaskHandle&&);

  // Returns true if the task handle is valid.
  bool IsValid() const;

  // Cancels the task.
  void CancelTask();

 private:
  std::unique_ptr<Delegate> delegate_;
};

}  // namespace base

#endif  // BASE_TASK_DELAYED_TASK_HANDLE_H_
