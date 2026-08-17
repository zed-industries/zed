// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TRACE_EVENT_HEAP_PROFILER_ALLOCATION_CONTEXT_TRACKER_H_
#define BASE_TRACE_EVENT_HEAP_PROFILER_ALLOCATION_CONTEXT_TRACKER_H_

#include <atomic>
#include <cstdint>
#include <vector>

#include "base/base_export.h"

namespace base {
namespace trace_event {

// AllocationContextTracker is a thread-local object. Its main purpose is to
// keep track of context pointers for memory allocation samples. See
// |AllocationContext|.
//
// A thread-local instance of the context tracker is initialized lazily when it
// is first accessed.
class BASE_EXPORT AllocationContextTracker {
 public:
  enum class CaptureMode : int32_t {
    kDisabled,     // Don't capture anything
    kNativeStack,  // Backtrace has full native backtraces from stack unwinding
  };

  // Globally sets capturing mode.
  // TODO(primiano): How to guard against *Stack -> kDisabled -> *Stack?
  static void SetCaptureMode(CaptureMode mode);

  // Returns global capturing mode.
  inline static CaptureMode capture_mode() {
    // A little lag after heap profiling is enabled or disabled is fine, it is
    // more important that the check is as cheap as possible when capturing is
    // not enabled, so do not issue a memory barrier in the fast path.
    if (capture_mode_.load(std::memory_order_relaxed) ==
        CaptureMode::kDisabled) {
      return CaptureMode::kDisabled;
    }

    // In the slow path, an acquire load is required to pair with the release
    // store in |SetCaptureMode|. This is to ensure that the TLS slot for
    // the thread-local allocation context tracker has been initialized if
    // |capture_mode| returns something other than kDisabled.
    return capture_mode_.load(std::memory_order_acquire);
  }

  // Returns the thread-local instance, creating one if necessary. Returns
  // always a valid instance, unless it is called re-entrantly, in which case
  // returns nullptr in the nested calls.
  static AllocationContextTracker* GetInstanceForCurrentThread();

  // Set the thread name in the AllocationContextTracker of the current thread
  // if capture is enabled.
  static void SetCurrentThreadName(const char* name);

  AllocationContextTracker(const AllocationContextTracker&) = delete;
  AllocationContextTracker& operator=(const AllocationContextTracker&) = delete;

  // Push and pop current task's context. A stack is used to support nested
  // tasks and the top of the stack will be used in allocation context.
  void PushCurrentTaskContext(const char* context);
  void PopCurrentTaskContext(const char* context);

  // Returns most recent task context added by ScopedTaskExecutionTracker.
  // TODO(crbug.com/40875107): Audit callers of TaskContext() to see if
  // any are useful. If not, remove AllocationContextTracker entirely.
  const char* TaskContext() const {
    return task_contexts_.empty() ? nullptr : task_contexts_.back();
  }

  ~AllocationContextTracker();

 private:
  AllocationContextTracker();

  static std::atomic<CaptureMode> capture_mode_;

  // The thread name is used as the first entry in the pseudo stack.
  const char* thread_name_ = nullptr;

  // Stack of tasks' contexts.
  std::vector<const char*> task_contexts_;
};

}  // namespace trace_event
}  // namespace base

#endif  // BASE_TRACE_EVENT_HEAP_PROFILER_ALLOCATION_CONTEXT_TRACKER_H_
