// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TASK_SEQUENCE_MANAGER_DELAYED_TASK_HANDLE_DELEGATE_H_
#define BASE_TASK_SEQUENCE_MANAGER_DELAYED_TASK_HANDLE_DELEGATE_H_

#include "base/containers/intrusive_heap.h"
#include "base/memory/raw_ptr_exclusion.h"
#include "base/memory/weak_ptr.h"
#include "base/sequence_checker.h"
#include "base/task/delayed_task_handle.h"

namespace base {
namespace sequence_manager {
namespace internal {

class TaskQueueImpl;

class DelayedTaskHandleDelegate : public DelayedTaskHandle::Delegate {
 public:
  explicit DelayedTaskHandleDelegate(TaskQueueImpl* outer);

  DelayedTaskHandleDelegate(const DelayedTaskHandleDelegate&) = delete;
  DelayedTaskHandleDelegate& operator=(const DelayedTaskHandleDelegate&) =
      delete;

  ~DelayedTaskHandleDelegate() override;

  WeakPtr<DelayedTaskHandleDelegate> AsWeakPtr();

  // DelayedTaskHandle::Delegate:
  bool IsValid() const override;
  void CancelTask() override;

  void SetHeapHandle(HeapHandle heap_handle);
  void ClearHeapHandle();
  HeapHandle GetHeapHandle();

  // Indicates that this task will be executed. This will invalidate the handle.
  void WillRunTask();

 private:
  // The TaskQueueImpl where the task was posted.
  // RAW_PTR_EXCLUSION: Performance reasons (based on analysis of speedometer3).
  RAW_PTR_EXCLUSION TaskQueueImpl* const outer_
      GUARDED_BY_CONTEXT(sequence_checker_) = nullptr;

  // The HeapHandle to the task, if the task is in the DelayedIncomingQueue,
  // invalid otherwise.
  HeapHandle heap_handle_ GUARDED_BY_CONTEXT(sequence_checker_);

  SEQUENCE_CHECKER(sequence_checker_);

  // Allows TaskQueueImpl to retain a weak reference to |this|. An outstanding
  // weak pointer indicates that the task is valid.
  WeakPtrFactory<DelayedTaskHandleDelegate> weak_ptr_factory_
      GUARDED_BY_CONTEXT(sequence_checker_){this};
};

}  // namespace internal
}  // namespace sequence_manager
}  // namespace base

#endif  // BASE_TASK_SEQUENCE_MANAGER_DELAYED_TASK_HANDLE_DELEGATE_H_
