// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_THREADING_CROSS_PROCESS_PLATFORM_THREAD_DELEGATE_H_
#define BASE_THREADING_CROSS_PROCESS_PLATFORM_THREAD_DELEGATE_H_

#include "base/base_export.h"
#include "base/threading/platform_thread.h"

namespace base {

// A CrossProcessPlatformThreadDelegate can intercept thread type changes for
// threads including other processes. This can be used for ChromeOS so that the
// browser process can proxy thread type updates of child processes received
// from SandboxedProcessThreadTypeHandler to D-Bus because child processes don't
// have access to D-Bus.
class BASE_EXPORT CrossProcessPlatformThreadDelegate {
 public:
  CrossProcessPlatformThreadDelegate() = default;

  CrossProcessPlatformThreadDelegate(
      const CrossProcessPlatformThreadDelegate&) = delete;
  CrossProcessPlatformThreadDelegate& operator=(
      const CrossProcessPlatformThreadDelegate&) = delete;

  virtual ~CrossProcessPlatformThreadDelegate() = default;

  // Invoked on thread type change. Returns true if the delegate handles
  // adjusting thread properties.
  virtual bool HandleThreadTypeChange(ProcessId process_id,
                                      PlatformThreadId thread_id,
                                      ThreadType thread_type) = 0;
};

}  // namespace base

#endif  // BASE_THREADING_CROSS_PROCESS_PLATFORM_THREAD_DELEGATE_H_
