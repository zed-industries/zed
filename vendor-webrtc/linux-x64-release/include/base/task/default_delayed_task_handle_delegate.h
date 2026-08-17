// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TASK_DEFAULT_DELAYED_TASK_HANDLE_DELEGATE_H_
#define BASE_TASK_DEFAULT_DELAYED_TASK_HANDLE_DELEGATE_H_

#include "base/base_export.h"
#include "base/functional/callback.h"
#include "base/memory/weak_ptr.h"
#include "base/task/delayed_task_handle.h"

namespace base {

// A default implementation of DelayedTaskHandle::Delegate that can cancel the
// delayed task by invalidating a weak pointer.
class BASE_EXPORT DefaultDelayedTaskHandleDelegate
    : public DelayedTaskHandle::Delegate {
 public:
  DefaultDelayedTaskHandleDelegate();
  ~DefaultDelayedTaskHandleDelegate() override;

  // DelayedTaskHandle::Delegate:
  bool IsValid() const override;
  void CancelTask() override;

  // Returns a new callback bound to this object such that it can be cancelled
  // by invalidating |weak_ptr_factory_|.
  OnceClosure BindCallback(OnceClosure callback);

 private:
  // Runs |callback|.
  void RunTask(OnceClosure callback);

  WeakPtrFactory<DefaultDelayedTaskHandleDelegate> weak_ptr_factory_{this};
};

}  // namespace base

#endif  // BASE_TASK_DEFAULT_DELAYED_TASK_HANDLE_DELEGATE_H_
