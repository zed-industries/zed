// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TRACE_EVENT_TRACE_EVENT_H_
#define BASE_TRACE_EVENT_TRACE_EVENT_H_

// This header file defines implementation details of how the trace macros in
// trace_event_common.h collect and store trace events. Anything not
// implementation-specific should go in trace_event_common.h instead of here.

#include <stddef.h>
#include <stdint.h>

#include <atomic>
#include <memory>
#include <utility>

#include "base/base_export.h"
#include "base/threading/platform_thread.h"
#include "base/time/time.h"
#include "base/time/time_override.h"
#include "base/trace_event/builtin_categories.h"
#include "base/trace_event/common/trace_event_common.h"  // IWYU pragma: export
#include "base/trace_event/trace_arguments.h"
#include "base/trace_event/trace_log.h"
#include "base/trace_event/traced_value_support.h"
#include "base/tracing_buildflags.h"

// Legacy TRACE_EVENT_API entrypoints. Do not use from new code.

// Add a trace event to the platform tracing system.
// base::trace_event::TraceEventHandle TRACE_EVENT_API_ADD_TRACE_EVENT(
//                    char phase,
//                    const unsigned char* category_group_enabled,
//                    const char* name,
//                    const char* scope,
//                    uint64_t id,
//                    base::trace_event::TraceArguments* args,
//                    unsigned int flags)
#define TRACE_EVENT_API_ADD_TRACE_EVENT trace_event_internal::AddTraceEvent

// Add a trace event to the platform tracing system overriding the pid.
// The resulting event will have tid = pid == (process_id passed here).
// base::trace_event::TraceEventHandle
// TRACE_EVENT_API_ADD_TRACE_EVENT_WITH_PROCESS_ID(
//                    char phase,
//                    const unsigned char* category_group_enabled,
//                    const char* name,
//                    const char* scope,
//                    uint64_t id,
//                    base::ProcessId process_id,
//                    base::trace_event::TraceArguments* args,
//                    unsigned int flags)
#define TRACE_EVENT_API_ADD_TRACE_EVENT_WITH_PROCESS_ID \
  trace_event_internal::AddTraceEventWithProcessId

// Add a trace event to the platform tracing system.
// base::trace_event::TraceEventHandle
// TRACE_EVENT_API_ADD_TRACE_EVENT_WITH_THREAD_ID_AND_TIMESTAMP(
//                    char phase,
//                    const unsigned char* category_group_enabled,
//                    const char* name,
//                    const char* scope,
//                    uint64_t id,
//                    uint64_t bind_id,
//                    base::PlatformThreadId thread_id,
//                    const TimeTicks& timestamp,
//                    base::trace_event::TraceArguments* args,
//                    unsigned int flags)
#define TRACE_EVENT_API_ADD_TRACE_EVENT_WITH_THREAD_ID_AND_TIMESTAMP \
  trace_event_internal::AddTraceEventWithThreadIdAndTimestamp

// Set the duration field of a COMPLETE trace event.
// void TRACE_EVENT_API_UPDATE_TRACE_EVENT_DURATION(
//     const unsigned char* category_group_enabled,
//     const char* name,
//     base::trace_event::TraceEventHandle id)
#define TRACE_EVENT_API_UPDATE_TRACE_EVENT_DURATION \
  trace_event_internal::UpdateTraceEventDuration

// Adds a metadata event to the trace log. The |AppendValueAsTraceFormat| method
// on the convertable value will be called at flush time.
// TRACE_EVENT_API_ADD_METADATA_EVENT(
//     const unsigned char* category_group_enabled,
//     const char* event_name,
//     const char* arg_name,
//     std::unique_ptr<ConvertableToTraceFormat> arg_value)
#define TRACE_EVENT_API_ADD_METADATA_EVENT \
    trace_event_internal::AddMetadataEvent

// Defines atomic operations used internally by the tracing system.
// Acquire/release barriers are important here: crbug.com/1330114#c8.
#define TRACE_EVENT_API_ATOMIC_WORD std::atomic<intptr_t>
#define TRACE_EVENT_API_ATOMIC_LOAD(var) (var).load(std::memory_order_acquire)
#define TRACE_EVENT_API_ATOMIC_STORE(var, value) \
  (var).store((value), std::memory_order_release)

// Defines visibility for classes in trace_event.h
#define TRACE_EVENT_API_CLASS_EXPORT BASE_EXPORT

////////////////////////////////////////////////////////////////////////////////

namespace trace_event_internal {

// Specify these values when the corresponding argument of AddTraceEvent is not
// used.
const int kZeroNumArgs = 0;
const std::nullptr_t kGlobalScope = nullptr;
const uint64_t kNoId = 0;

// These functions all internally call
// base::trace_event::TraceLog::GetInstance() then call the method with the same
// name on it. This is used to reduce the generated machine code at each
// TRACE_EVENTXXX macro call.

base::trace_event::TraceEventHandle BASE_EXPORT
AddTraceEvent(char phase,
              const unsigned char* category_group_enabled,
              const char* name,
              const char* scope,
              uint64_t id,
              base::trace_event::TraceArguments* args,
              unsigned int flags);

base::trace_event::TraceEventHandle BASE_EXPORT
AddTraceEventWithProcessId(char phase,
                           const unsigned char* category_group_enabled,
                           const char* name,
                           const char* scope,
                           uint64_t id,
                           base::ProcessId process_id,
                           base::trace_event::TraceArguments* args,
                           unsigned int flags);

base::trace_event::TraceEventHandle BASE_EXPORT
AddTraceEventWithThreadIdAndTimestamp(
    char phase,
    const unsigned char* category_group_enabled,
    const char* name,
    const char* scope,
    uint64_t id,
    uint64_t bind_id,
    base::PlatformThreadId thread_id,
    const base::TimeTicks& timestamp,
    base::trace_event::TraceArguments* args,
    unsigned int flags);

base::trace_event::TraceEventHandle BASE_EXPORT
AddTraceEventWithThreadIdAndTimestamps(
    char phase,
    const unsigned char* category_group_enabled,
    const char* name,
    const char* scope,
    uint64_t id,
    base::PlatformThreadId thread_id,
    const base::TimeTicks& timestamp,
    const base::ThreadTicks& thread_timestamp,
    unsigned int flags);

void BASE_EXPORT
UpdateTraceEventDuration(const unsigned char* category_group_enabled,
                         const char* name,
                         base::trace_event::TraceEventHandle handle);

}  // namespace trace_event_internal

namespace base {
namespace trace_event {

template <typename IDType, const char* category>
class TraceScopedTrackableObject {
 public:
  TraceScopedTrackableObject(const char* name, IDType id)
      : name_(name), id_(id) {
    TRACE_EVENT_OBJECT_CREATED_WITH_ID(category, name_, id_);
  }
  TraceScopedTrackableObject(const TraceScopedTrackableObject&) = delete;
  TraceScopedTrackableObject& operator=(const TraceScopedTrackableObject&) =
      delete;

  template <typename ArgType>
  void snapshot(ArgType snapshot) {
    TRACE_EVENT_OBJECT_SNAPSHOT_WITH_ID(category, name_, id_, snapshot);
  }

  ~TraceScopedTrackableObject() {
    TRACE_EVENT_OBJECT_DELETED_WITH_ID(category, name_, id_);
  }

 private:
  const char* name_;
  IDType id_;
};

}  // namespace trace_event
}  // namespace base

#endif  // BASE_TRACE_EVENT_TRACE_EVENT_H_
