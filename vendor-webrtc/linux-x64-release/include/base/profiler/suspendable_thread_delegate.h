// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_PROFILER_SUSPENDABLE_THREAD_DELEGATE_H_
#define BASE_PROFILER_SUSPENDABLE_THREAD_DELEGATE_H_

#include <memory>

#include "base/base_export.h"
#include "base/profiler/register_context.h"
#include "base/profiler/thread_delegate.h"

namespace base {

// Platform-specific thread and stack manipulation delegate, for use by the
// platform-independent stack copying/walking implementation in
// StackSamplerImpl for suspension-based stack copying.
//
// IMPORTANT NOTE: Most methods in this interface are invoked while the target
// thread is suspended so must not do any allocation from the heap, including
// indirectly via use of DCHECK/CHECK or other logging statements. Otherwise the
// implementation can deadlock on heap locks acquired by the target thread
// before it was suspended. These functions are commented with "NO HEAP
// ALLOCATIONS".
class BASE_EXPORT SuspendableThreadDelegate : public ThreadDelegate {
 public:
  // Implementations of this interface should suspend the thread for the
  // object's lifetime. NO HEAP ALLOCATIONS between the time the thread is
  // suspended and resumed.
  class BASE_EXPORT ScopedSuspendThread {
   public:
    ScopedSuspendThread() = default;
    virtual ~ScopedSuspendThread() = default;

    ScopedSuspendThread(const ScopedSuspendThread&) = delete;
    ScopedSuspendThread& operator=(const ScopedSuspendThread&) = delete;

    virtual bool WasSuccessful() const = 0;
  };

  SuspendableThreadDelegate() = default;

  // Creates an object that holds the thread suspended for its lifetime.
  virtual std::unique_ptr<ScopedSuspendThread> CreateScopedSuspendThread() = 0;

  // Gets the register context for the thread.
  // NO HEAP ALLOCATIONS.
  virtual bool GetThreadContext(RegisterContext* thread_context) = 0;

  // Returns true if the thread's stack can be copied, where the bottom address
  // of the thread is at |stack_pointer|.
  // NO HEAP ALLOCATIONS.
  virtual bool CanCopyStack(uintptr_t stack_pointer) = 0;
};

}  // namespace base

#endif  // BASE_PROFILER_SUSPENDABLE_THREAD_DELEGATE_H_
