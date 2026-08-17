// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_PROFILER_SUSPENDABLE_THREAD_DELEGATE_WIN_H_
#define BASE_PROFILER_SUSPENDABLE_THREAD_DELEGATE_WIN_H_

#include <windows.h>

#include <memory>
#include <vector>

#include "base/base_export.h"
#include "base/profiler/sampling_profiler_thread_token.h"
#include "base/profiler/suspendable_thread_delegate.h"
#include "base/threading/platform_thread.h"
#include "base/win/scoped_handle.h"

namespace base {

// Platform- and thread-specific implementation in support of stack sampling on
// Windows.
class BASE_EXPORT SuspendableThreadDelegateWin
    : public SuspendableThreadDelegate {
 public:
  class ScopedSuspendThread
      : public SuspendableThreadDelegate::ScopedSuspendThread {
   public:
    explicit ScopedSuspendThread(HANDLE thread_handle);

    ScopedSuspendThread(const ScopedSuspendThread&) = delete;
    ScopedSuspendThread& operator=(const ScopedSuspendThread&) = delete;

    ~ScopedSuspendThread() override;

    bool WasSuccessful() const override;

   private:
    HANDLE thread_handle_;
    bool was_successful_;
  };

  explicit SuspendableThreadDelegateWin(
      SamplingProfilerThreadToken thread_token);
  ~SuspendableThreadDelegateWin() override;

  SuspendableThreadDelegateWin(const SuspendableThreadDelegateWin&) = delete;
  SuspendableThreadDelegateWin& operator=(const SuspendableThreadDelegateWin&) =
      delete;

  // SuspendableThreadDelegate
  std::unique_ptr<SuspendableThreadDelegate::ScopedSuspendThread>
  CreateScopedSuspendThread() override;
  bool GetThreadContext(CONTEXT* thread_context) override;
  PlatformThreadId GetThreadId() const override;
  uintptr_t GetStackBaseAddress() const override;
  bool CanCopyStack(uintptr_t stack_pointer) override;
  std::vector<uintptr_t*> GetRegistersToRewrite(
      CONTEXT* thread_context) override;

 private:
  const PlatformThreadId thread_id_;
  win::ScopedHandle thread_handle_;
  const uintptr_t thread_stack_base_address_;
};

}  // namespace base

#endif  // BASE_PROFILER_SUSPENDABLE_THREAD_DELEGATE_WIN_H_
