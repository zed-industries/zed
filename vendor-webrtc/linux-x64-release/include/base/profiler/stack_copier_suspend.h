// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_PROFILER_STACK_COPIER_SUSPEND_H_
#define BASE_PROFILER_STACK_COPIER_SUSPEND_H_

#include <memory>

#include "base/base_export.h"
#include "base/profiler/stack_copier.h"

namespace base {

class SuspendableThreadDelegate;

// Supports stack copying on platforms where the profiled thread must be
// explicitly suspended from the profiler thread and the stack is copied from
// the profiler thread.
class BASE_EXPORT StackCopierSuspend : public StackCopier {
 public:
  StackCopierSuspend(
      std::unique_ptr<SuspendableThreadDelegate> thread_delegate);
  ~StackCopierSuspend() override;

  // StackCopier:
  bool CopyStack(StackBuffer* stack_buffer,
                 uintptr_t* stack_top,
                 TimeTicks* timestamp,
                 RegisterContext* thread_context,
                 Delegate* delegate) override;

 protected:
  std::vector<uintptr_t*> GetRegistersToRewrite(
      RegisterContext* thread_context) override;

 private:
  std::unique_ptr<SuspendableThreadDelegate> thread_delegate_;
};

}  // namespace base

#endif  // BASE_PROFILER_STACK_COPIER_SUSPEND_H_
