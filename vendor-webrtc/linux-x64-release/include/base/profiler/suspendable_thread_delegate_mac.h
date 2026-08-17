// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_PROFILER_SUSPENDABLE_THREAD_DELEGATE_MAC_H_
#define BASE_PROFILER_SUSPENDABLE_THREAD_DELEGATE_MAC_H_

#include <mach/mach.h>

#include <memory>
#include <vector>

#include "base/base_export.h"
#include "base/profiler/module_cache.h"
#include "base/profiler/sampling_profiler_thread_token.h"
#include "base/profiler/suspendable_thread_delegate.h"
#include "base/threading/platform_thread.h"

namespace base {

// Platform- and thread-specific implementation in support of stack sampling on
// Mac (X86_64) and iOS (X86_64 and ARM64).
class BASE_EXPORT SuspendableThreadDelegateMac
    : public SuspendableThreadDelegate {
 public:
  class ScopedSuspendThread
      : public SuspendableThreadDelegate::ScopedSuspendThread {
   public:
    explicit ScopedSuspendThread(mach_port_t thread_port);
    ~ScopedSuspendThread() override;

    ScopedSuspendThread(const ScopedSuspendThread&) = delete;
    ScopedSuspendThread& operator=(const ScopedSuspendThread&) = delete;

    bool WasSuccessful() const override;

   private:
    mach_port_t thread_port_;
  };

  SuspendableThreadDelegateMac(SamplingProfilerThreadToken thread_token);
  ~SuspendableThreadDelegateMac() override;

  SuspendableThreadDelegateMac(const SuspendableThreadDelegateMac&) = delete;
  SuspendableThreadDelegateMac& operator=(const SuspendableThreadDelegateMac&) =
      delete;

  // SuspendableThreadDelegate
  std::unique_ptr<SuspendableThreadDelegate::ScopedSuspendThread>
  CreateScopedSuspendThread() override;
  bool GetThreadContext(RegisterContext* thread_context) override;
  PlatformThreadId GetThreadId() const override;
  uintptr_t GetStackBaseAddress() const override;
  bool CanCopyStack(uintptr_t stack_pointer) override;
  std::vector<uintptr_t*> GetRegistersToRewrite(
      RegisterContext* thread_context) override;

 private:
  // Thread ID of thread being profiled.
  const base::PlatformThreadId thread_id_;

  // Weak reference: Mach port for thread being profiled.
  const mach_port_t thread_port_;

  // The stack base address corresponding to |thread_port_|.
  const uintptr_t thread_stack_base_address_;
};

}  // namespace base

#endif  // BASE_PROFILER_SUSPENDABLE_THREAD_DELEGATE_MAC_H_
