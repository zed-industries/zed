// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_PROFILER_FRAME_POINTER_UNWINDER_H_
#define BASE_PROFILER_FRAME_POINTER_UNWINDER_H_

#include <vector>

#include "base/base_export.h"
#include "base/functional/callback.h"
#include "base/profiler/unwinder.h"
#include "build/build_config.h"

#if BUILDFLAG(IS_APPLE)
#include <os/availability.h>
#endif

namespace base {

// Native unwinder implementation for platforms that have frame pointers:
//  * iOS, ARM64 and X86_64,
//  * macOS
//  * ChromeOS X86_64 and ARM64
//  * Android ARM64 (Chrome module)
class BASE_EXPORT
#if BUILDFLAG(IS_APPLE)
API_AVAILABLE(ios(12))
#endif
    FramePointerUnwinder : public Unwinder {
 public:
  using CanUnwindFromDelegate =
      RepeatingCallback<bool(const Frame& current_frame)>;

  FramePointerUnwinder(
      CanUnwindFromDelegate can_unwind_from_delegate = CanUnwindFromDelegate(),
      bool is_system_unwinder = true);
  ~FramePointerUnwinder() override;

  FramePointerUnwinder(const FramePointerUnwinder&) = delete;
  FramePointerUnwinder& operator=(const FramePointerUnwinder&) = delete;

  // Unwinder:
  bool CanUnwindFrom(const Frame& current_frame) const override;
  UnwindResult TryUnwind(UnwinderStateCapture* capture_state,
                         RegisterContext* thread_context,
                         uintptr_t stack_top,
                         std::vector<Frame>* stack) override;

 private:
  CanUnwindFromDelegate can_unwind_from_delegate_;
  const bool is_system_unwinder_;
};

}  // namespace base

#endif  // BASE_PROFILER_FRAME_POINTER_UNWINDER_H_
