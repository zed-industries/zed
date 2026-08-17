// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_PROFILER_NATIVE_UNWINDER_WIN_H_
#define BASE_PROFILER_NATIVE_UNWINDER_WIN_H_

#include <vector>

#include "base/profiler/unwinder.h"

namespace base {

// Native unwinder implementation for Windows, using RtlVirtualUnwind.
class NativeUnwinderWin : public Unwinder {
 public:
  NativeUnwinderWin() = default;

  NativeUnwinderWin(const NativeUnwinderWin&) = delete;
  NativeUnwinderWin& operator=(const NativeUnwinderWin&) = delete;

  // Unwinder:
  bool CanUnwindFrom(const Frame& current_frame) const override;
  UnwindResult TryUnwind(UnwinderStateCapture* capture_state,
                         RegisterContext* thread_context,
                         uintptr_t stack_top,
                         std::vector<Frame>* stack) override;
};

}  // namespace base

#endif  // BASE_PROFILER_NATIVE_UNWINDER_WIN_H_
