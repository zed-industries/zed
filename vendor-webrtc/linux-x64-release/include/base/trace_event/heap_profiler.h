// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TRACE_EVENT_HEAP_PROFILER_H_
#define BASE_TRACE_EVENT_HEAP_PROFILER_H_

#include "base/compiler_specific.h"
#include "base/trace_event/heap_profiler_allocation_context_tracker.h"

// This header file defines the set of macros that are used to track memory
// usage in the heap profiler. This is in addition to the macros defined in
// trace_event.h and are specific to heap profiler. This file also defines
// implementation details of these macros.

// Scoped tracker for task execution context in the heap profiler.
#define TRACE_HEAP_PROFILER_API_SCOPED_TASK_EXECUTION \
  trace_event_internal::HeapProfilerScopedTaskExecutionTracker

// Returns the current task context (c-string) tracked by heap profiler. This is
// useful along with TRACE_HEAP_PROFILER_API_SCOPED_TASK_EXECUTION if a async
// system needs to track client's allocation context across post tasks. Use this
// macro to get the current context and use
// TRACE_HEAP_PROFILER_API_SCOPED_TASK_EXECUTION in the posted task which
// allocates memory for a client.
#define TRACE_HEAP_PROFILER_API_GET_CURRENT_TASK_CONTEXT \
  trace_event_internal::HeapProfilerCurrentTaskContext

// A scoped ignore event used to tell heap profiler to ignore all the
// allocations in the scope. It is useful to exclude allocations made for
// tracing from the heap profiler dumps.
// TODO(crbug.com/40875107): This is a no-op since
// AllocationContextTracker::GetContextSnapshot was removed. Clean up the call
// sites.
#define HEAP_PROFILER_SCOPED_IGNORE ((void)0)

namespace trace_event_internal {

// HeapProfilerScopedTaskExecutionTracker records the current task's context in
// the heap profiler.
class HeapProfilerScopedTaskExecutionTracker {
 public:
  inline explicit HeapProfilerScopedTaskExecutionTracker(
      const char* task_context)
      : context_(task_context) {
    using base::trace_event::AllocationContextTracker;
    if (AllocationContextTracker::capture_mode() !=
        AllocationContextTracker::CaptureMode::kDisabled) [[unlikely]] {
      AllocationContextTracker::GetInstanceForCurrentThread()
          ->PushCurrentTaskContext(context_);
    }
  }

  inline ~HeapProfilerScopedTaskExecutionTracker() {
    using base::trace_event::AllocationContextTracker;
    if (AllocationContextTracker::capture_mode() !=
        AllocationContextTracker::CaptureMode::kDisabled) [[unlikely]] {
      AllocationContextTracker::GetInstanceForCurrentThread()
          ->PopCurrentTaskContext(context_);
    }
  }

 private:
  const char* context_;
};

inline const char* HeapProfilerCurrentTaskContext() {
  return base::trace_event::AllocationContextTracker::
      GetInstanceForCurrentThread()
          ->TaskContext();
}

}  // namespace trace_event_internal

#endif  // BASE_TRACE_EVENT_HEAP_PROFILER_H_
