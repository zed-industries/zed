// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_THREADING_THREAD_TYPE_DELEGATE_H_
#define BASE_THREADING_THREAD_TYPE_DELEGATE_H_

#include "base/base_export.h"
#include "base/threading/platform_thread.h"

namespace base {

// A ThreadTypeDelegate can intercept thread type changes. This can be used to
// adjust thread properties via another process when the current process can't
// directly adjust them (e.g. due to sandbox restrictions).
class BASE_EXPORT ThreadTypeDelegate {
 public:
  ThreadTypeDelegate();

  ThreadTypeDelegate(const ThreadTypeDelegate&) = delete;
  ThreadTypeDelegate& operator=(const ThreadTypeDelegate&) = delete;

  virtual ~ThreadTypeDelegate();

  // Invoked on thread type change. Returns true if the delegate handles
  // adjusting thread properties (i.e. //base code will not adjust thread
  // properties such as nice value, c-group, latency sensitivity...).
  virtual bool HandleThreadTypeChange(PlatformThreadId thread_id,
                                      ThreadType thread_type) = 0;
};

}  // namespace base

#endif  // BASE_THREADING_THREAD_TYPE_DELEGATE_H_
