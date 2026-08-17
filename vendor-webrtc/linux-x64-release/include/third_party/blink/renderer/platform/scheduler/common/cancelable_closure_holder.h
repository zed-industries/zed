// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_COMMON_CANCELABLE_CLOSURE_HOLDER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_COMMON_CANCELABLE_CLOSURE_HOLDER_H_

#include "base/cancelable_callback.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {
namespace scheduler {

// A CancelableClosureHolder is a CancelableRepeatingClosure which resets its
// wrapped callback with a cached closure whenever it is canceled.
class CancelableClosureHolder {
  DISALLOW_NEW();

 public:
  CancelableClosureHolder();
  CancelableClosureHolder(const CancelableClosureHolder&) = delete;
  CancelableClosureHolder& operator=(const CancelableClosureHolder&) = delete;
  ~CancelableClosureHolder();

  // Resets the closure to be wrapped by the cancelable callback.  Cancels any
  // outstanding callbacks.
  void Reset(const base::RepeatingClosure& callback);

  // Cancels any outstanding closures returned by callback().
  void Cancel();

  // Returns a callback that will be disabled by calling Cancel(). Callback
  // must have been set using Reset() before calling this function.
  base::RepeatingClosure GetCallback() const;

 private:
  base::RepeatingClosure callback_;
  base::CancelableRepeatingClosure cancelable_callback_;
};

}  // namespace scheduler
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_COMMON_CANCELABLE_CLOSURE_HOLDER_H_
