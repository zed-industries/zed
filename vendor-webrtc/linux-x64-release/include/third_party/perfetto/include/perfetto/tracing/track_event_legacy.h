/*
 * Copyright (C) 2020 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef INCLUDE_PERFETTO_TRACING_TRACK_EVENT_LEGACY_H_
#define INCLUDE_PERFETTO_TRACING_TRACK_EVENT_LEGACY_H_

// This file defines a compatibility shim between legacy (Chrome, V8) trace
// event macros and track events. To avoid accidentally introducing legacy
// events in new code, the PERFETTO_ENABLE_LEGACY_TRACE_EVENTS macro must be set
// to 1 activate the compatibility layer.

#include "perfetto/base/compiler.h"
#include "perfetto/tracing/track_event.h"

#include <stdint.h>

#ifndef PERFETTO_ENABLE_LEGACY_TRACE_EVENTS
#define PERFETTO_ENABLE_LEGACY_TRACE_EVENTS 0
#endif

#if defined(__GNUC__) || defined(__clang__)
#if defined(__clang__)
#pragma clang diagnostic push
// Fix 'error: #pragma system_header ignored in main file' for clang in Google3.
#pragma clang diagnostic ignored "-Wpragma-system-header-outside-header"
#endif

// Ignore GCC warning about a missing argument for a variadic macro parameter.
#pragma GCC system_header

#if defined(__clang__)
#pragma clang diagnostic pop
#endif
#endif

// ----------------------------------------------------------------------------
// Internal legacy trace point implementation.
// ----------------------------------------------------------------------------

namespace perfetto {
namespace legacy {

// The following user-provided adaptors are used to serialize user-defined
// thread id and time types into track events. For full compatibility, the user
// should also define the following macros appropriately:
//
//   #define TRACE_TIME_TICKS_NOW() ...
//   #define TRACE_TIME_NOW() ...

// User-provided function to convert an abstract thread id into a thread track.
template <typename T>
ThreadTrack ConvertThreadId(const T&);

// Built-in implementation for events referring to the current thread.
template <>
ThreadTrack PERFETTO_EXPORT_COMPONENT
ConvertThreadId(const PerfettoLegacyCurrentThreadId&);

}  // namespace legacy
}  // namespace perfetto

#if PERFETTO_ENABLE_LEGACY_TRACE_EVENTS

// Implementations for the INTERNAL_* adapter macros used by the trace points
// below.
#define PERFETTO_INTERNAL_LEGACY_EVENT_ON_TRACK(phase, category, name, track, \
                                                ...)                          \
  PERFETTO_INTERNAL_TRACK_EVENT_WITH_METHOD(                                  \
      TraceForCategory, category,                                             \
      ::perfetto::internal::DecayEventNameType(name),                         \
      ::perfetto::internal::TrackEventLegacy::PhaseToType(phase), track,      \
      ##__VA_ARGS__);

#define PERFETTO_INTERNAL_LEGACY_EVENT_WITH_FLAGS_ON_TRACK(              \
    phase, category, name, track, flags, ...)                            \
  PERFETTO_INTERNAL_TRACK_EVENT_WITH_METHOD(                             \
      TraceForCategoryLegacy, category,                                  \
      ::perfetto::internal::DecayEventNameType(name),                    \
      ::perfetto::internal::TrackEventLegacy::PhaseToType(phase), track, \
      phase, flags, ##__VA_ARGS__);

#define PERFETTO_INTERNAL_LEGACY_EVENT_WITH_ID_ON_TRACK(                 \
    phase, category, name, track, flags, thread_id, id, ...)             \
  PERFETTO_INTERNAL_TRACK_EVENT_WITH_METHOD(                             \
      TraceForCategoryLegacyWithId, category,                            \
      ::perfetto::internal::DecayEventNameType(name),                    \
      ::perfetto::internal::TrackEventLegacy::PhaseToType(phase), track, \
      phase, flags, thread_id, id, ##__VA_ARGS__);

// The main entrypoint for writing unscoped legacy events.  This macro
// determines the right track to write the event on based on |flags| and
// |thread_id|.
#define PERFETTO_INTERNAL_LEGACY_EVENT(phase, category, name, flags,       \
                                       thread_id, ...)                     \
  [&]() {                                                                  \
    using ::perfetto::internal::TrackEventInternal;                        \
    PERFETTO_DCHECK(!(flags & TRACE_EVENT_FLAG_COPY));                     \
    /* First check the scope for instant events. */                        \
    if ((phase) == TRACE_EVENT_PHASE_INSTANT) {                            \
      /* Note: Avoids the need to set LegacyEvent::instant_event_scope. */ \
      auto scope = (flags) & TRACE_EVENT_FLAG_SCOPE_MASK;                  \
      switch (scope) {                                                     \
        case TRACE_EVENT_SCOPE_GLOBAL:                                     \
          PERFETTO_INTERNAL_LEGACY_EVENT_WITH_FLAGS_ON_TRACK(              \
              phase, category, name, ::perfetto::Track::Global(0), flags,  \
              ##__VA_ARGS__);                                              \
          return;                                                          \
        case TRACE_EVENT_SCOPE_PROCESS:                                    \
          PERFETTO_INTERNAL_LEGACY_EVENT_WITH_FLAGS_ON_TRACK(              \
              phase, category, name, ::perfetto::ProcessTrack::Current(),  \
              flags, ##__VA_ARGS__);                                       \
          return;                                                          \
        default:                                                           \
        case TRACE_EVENT_SCOPE_THREAD:                                     \
          /* Fallthrough. */                                               \
          break;                                                           \
      }                                                                    \
    }                                                                      \
    /* If an event targets the current thread or another process, write    \
     * it on the current thread's track. The process override case is      \
     * handled through |pid_override| in WriteLegacyEvent. */              \
    if (std::is_same<                                                      \
            decltype(thread_id),                                           \
            ::perfetto::legacy::PerfettoLegacyCurrentThreadId>::value ||   \
        ((flags) & TRACE_EVENT_FLAG_HAS_PROCESS_ID)) {                     \
      PERFETTO_INTERNAL_LEGACY_EVENT_WITH_FLAGS_ON_TRACK(                  \
          phase, category, name, TrackEventInternal::kDefaultTrack, flags, \
          ##__VA_ARGS__);                                                  \
    } else {                                                               \
      PERFETTO_INTERNAL_LEGACY_EVENT_WITH_FLAGS_ON_TRACK(                  \
          phase, category, name,                                           \
          ::perfetto::legacy::ConvertThreadId(thread_id), flags,           \
          ##__VA_ARGS__);                                                  \
    }                                                                      \
  }()

#define PERFETTO_INTERNAL_LEGACY_EVENT_WITH_ID(phase, category, name, flags, \
                                               thread_id, id, ...)           \
  [&]() {                                                                    \
    using ::perfetto::internal::TrackEventInternal;                          \
    PERFETTO_DCHECK(!(flags & TRACE_EVENT_FLAG_COPY));                       \
    /* First check the scope for instant events. */                          \
    if ((phase) == TRACE_EVENT_PHASE_INSTANT) {                              \
      /* Note: Avoids the need to set LegacyEvent::instant_event_scope. */   \
      auto scope = (flags) & TRACE_EVENT_FLAG_SCOPE_MASK;                    \
      switch (scope) {                                                       \
        case TRACE_EVENT_SCOPE_GLOBAL:                                       \
          PERFETTO_INTERNAL_LEGACY_EVENT_WITH_ID_ON_TRACK(                   \
              phase, category, name, ::perfetto::Track::Global(0), flags,    \
              thread_id, id, ##__VA_ARGS__);                                 \
          return;                                                            \
        case TRACE_EVENT_SCOPE_PROCESS:                                      \
          PERFETTO_INTERNAL_LEGACY_EVENT_WITH_ID_ON_TRACK(                   \
              phase, category, name, ::perfetto::ProcessTrack::Current(),    \
              flags, thread_id, id, ##__VA_ARGS__);                          \
          return;                                                            \
        default:                                                             \
        case TRACE_EVENT_SCOPE_THREAD:                                       \
          /* Fallthrough. */                                                 \
          break;                                                             \
      }                                                                      \
    }                                                                        \
    /* If an event targets the current thread or another process, write      \
     * it on the current thread's track. The process override case is        \
     * handled through |pid_override| in WriteLegacyEvent. */                \
    if (std::is_same<                                                        \
            decltype(thread_id),                                             \
            ::perfetto::legacy::PerfettoLegacyCurrentThreadId>::value ||     \
        ((flags) & TRACE_EVENT_FLAG_HAS_PROCESS_ID)) {                       \
      PERFETTO_INTERNAL_LEGACY_EVENT_WITH_ID_ON_TRACK(                       \
          phase, category, name, TrackEventInternal::kDefaultTrack, flags,   \
          thread_id, id, ##__VA_ARGS__);                                     \
    } else {                                                                 \
      PERFETTO_INTERNAL_LEGACY_EVENT_WITH_ID_ON_TRACK(                       \
          phase, category, name,                                             \
          ::perfetto::legacy::ConvertThreadId(thread_id), flags, thread_id,  \
          id, ##__VA_ARGS__);                                                \
    }                                                                        \
  }()

#define INTERNAL_TRACE_EVENT_ADD(phase, category, name, flags, ...)           \
  PERFETTO_INTERNAL_LEGACY_EVENT(                                             \
      phase, category, ::perfetto::internal::DecayEventNameType(name), flags, \
      ::perfetto::legacy::kCurrentThreadId, ##__VA_ARGS__)

#define INTERNAL_TRACE_EVENT_ADD_SCOPED(category, name, ...) \
  PERFETTO_INTERNAL_SCOPED_TRACK_EVENT(                      \
      category, ::perfetto::internal::DecayEventNameType(name), ##__VA_ARGS__)

#define INTERNAL_TRACE_EVENT_ADD_SCOPED_WITH_FLOW(category, name, bind_id, \
                                                  flags, ...)              \
  PERFETTO_INTERNAL_SCOPED_LEGACY_TRACK_EVENT_WITH_ID(                     \
      category, ::perfetto::internal::DecayEventNameType(name),            \
      ::perfetto::internal::TrackEventInternal::kDefaultTrack, flags,      \
      TRACE_EVENT_API_CURRENT_THREAD_ID, bind_id, ##__VA_ARGS__)

#define INTERNAL_TRACE_EVENT_ADD_WITH_TIMESTAMP(phase, category, name,        \
                                                timestamp, flags, ...)        \
  PERFETTO_INTERNAL_LEGACY_EVENT(                                             \
      phase, category, ::perfetto::internal::DecayEventNameType(name), flags, \
      ::perfetto::legacy::kCurrentThreadId, timestamp, ##__VA_ARGS__)

#define INTERNAL_TRACE_EVENT_ADD_WITH_ID_TID_AND_TIMESTAMP(                   \
    phase, category, name, id, thread_id, timestamp, flags, ...)              \
  PERFETTO_INTERNAL_LEGACY_EVENT_WITH_ID(                                     \
      phase, category, ::perfetto::internal::DecayEventNameType(name), flags, \
      thread_id, id, timestamp, ##__VA_ARGS__)

#define INTERNAL_TRACE_EVENT_ADD_WITH_ID(phase, category, name, id, flags,    \
                                         ...)                                 \
  PERFETTO_INTERNAL_LEGACY_EVENT_WITH_ID(                                     \
      phase, category, ::perfetto::internal::DecayEventNameType(name), flags, \
      ::perfetto::legacy::kCurrentThreadId, id, ##__VA_ARGS__)

#define INTERNAL_TRACE_EVENT_METADATA_ADD(category, name, ...)         \
  INTERNAL_TRACE_EVENT_ADD(TRACE_EVENT_PHASE_METADATA, category, name, \
                           TRACE_EVENT_FLAG_NONE)

// ----------------------------------------------------------------------------
// Legacy tracing common API (adapted from trace_event_common.h).
// ----------------------------------------------------------------------------

#define TRACE_DISABLED_BY_DEFAULT(name) "disabled-by-default-" name

// Scoped events.
#define TRACE_EVENT0(category_group, name) \
  INTERNAL_TRACE_EVENT_ADD_SCOPED(category_group, name)
#define TRACE_EVENT_WITH_FLOW0(category_group, name, bind_id, flow_flags)  \
  INTERNAL_TRACE_EVENT_ADD_SCOPED_WITH_FLOW(category_group, name, bind_id, \
                                            flow_flags)
#define TRACE_EVENT1(category_group, name, arg1_name, arg1_val) \
  INTERNAL_TRACE_EVENT_ADD_SCOPED(                              \
      category_group, name, arg1_name,                          \
      ::perfetto::internal::PossiblyNull(arg1_val))
#define TRACE_EVENT_WITH_FLOW1(category_group, name, bind_id, flow_flags, \
                               arg1_name, arg1_val)                       \
  INTERNAL_TRACE_EVENT_ADD_SCOPED_WITH_FLOW(                              \
      category_group, name, bind_id, flow_flags, arg1_name,               \
      ::perfetto::internal::PossiblyNull(arg1_val))
#define TRACE_EVENT2(category_group, name, arg1_name, arg1_val, arg2_name, \
                     arg2_val)                                             \
  INTERNAL_TRACE_EVENT_ADD_SCOPED(                                         \
      category_group, name, arg1_name,                                     \
      ::perfetto::internal::PossiblyNull(arg1_val), arg2_name,             \
      ::perfetto::internal::PossiblyNull(arg2_val))
#define TRACE_EVENT_WITH_FLOW2(category_group, name, bind_id, flow_flags, \
                               arg1_name, arg1_val, arg2_name, arg2_val)  \
  INTERNAL_TRACE_EVENT_ADD_SCOPED_WITH_FLOW(                              \
      category_group, name, bind_id, flow_flags, arg1_name,               \
      ::perfetto::internal::PossiblyNull(arg1_val), arg2_name,            \
      ::perfetto::internal::PossiblyNull(arg2_val))

// Instant events.
#define TRACE_EVENT_INSTANT0(category_group, name, scope)                   \
  INTERNAL_TRACE_EVENT_ADD(TRACE_EVENT_PHASE_INSTANT, category_group, name, \
                           TRACE_EVENT_FLAG_NONE | scope)
#define TRACE_EVENT_INSTANT1(category_group, name, scope, arg1_name, arg1_val) \
  INTERNAL_TRACE_EVENT_ADD(TRACE_EVENT_PHASE_INSTANT, category_group, name,    \
                           TRACE_EVENT_FLAG_NONE | scope, arg1_name,           \
                           ::perfetto::internal::PossiblyNull(arg1_val))
#define TRACE_EVENT_INSTANT2(category_group, name, scope, arg1_name, arg1_val, \
                             arg2_name, arg2_val)                              \
  INTERNAL_TRACE_EVENT_ADD(TRACE_EVENT_PHASE_INSTANT, category_group, name,    \
                           TRACE_EVENT_FLAG_NONE | scope, arg1_name,           \
                           ::perfetto::internal::PossiblyNull(arg1_val),       \
                           arg2_name,                                          \
                           ::perfetto::internal::PossiblyNull(arg2_val))
#define TRACE_EVENT_COPY_INSTANT0(category_group, name, scope)        \
  INTERNAL_TRACE_EVENT_ADD(TRACE_EVENT_PHASE_INSTANT, category_group, \
                           ::perfetto::DynamicString{name}, scope)
#define TRACE_EVENT_COPY_INSTANT1(category_group, name, scope, arg1_name, \
                                  arg1_val)                               \
  INTERNAL_TRACE_EVENT_ADD(TRACE_EVENT_PHASE_INSTANT, category_group,     \
                           ::perfetto::DynamicString{name}, scope,        \
                           ::perfetto::DynamicString{arg1_name},          \
                           ::perfetto::internal::PossiblyNull(arg1_val))
#define TRACE_EVENT_COPY_INSTANT2(category_group, name, scope, arg1_name, \
                                  arg1_val, arg2_name, arg2_val)          \
  INTERNAL_TRACE_EVENT_ADD(TRACE_EVENT_PHASE_INSTANT, category_group,     \
                           ::perfetto::DynamicString{name}, scope,        \
                           ::perfetto::DynamicString{arg1_name},          \
                           ::perfetto::internal::PossiblyNull(arg1_val),  \
                           ::perfetto::DynamicString{arg2_name},          \
                           ::perfetto::internal::PossiblyNull(arg2_val))
#define TRACE_EVENT_INSTANT_WITH_FLAGS0(category_group, name, scope_and_flags) \
  INTERNAL_TRACE_EVENT_ADD(TRACE_EVENT_PHASE_INSTANT, category_group, name,    \
                           scope_and_flags)
#define TRACE_EVENT_INSTANT_WITH_FLAGS1(category_group, name, scope_and_flags, \
                                        arg1_name, arg1_val)                   \
  INTERNAL_TRACE_EVENT_ADD(TRACE_EVENT_PHASE_INSTANT, category_group, name,    \
                           scope_and_flags, arg1_name,                         \
                           ::perfetto::internal::PossiblyNull(arg1_val))

// Instant events with explicit timestamps.
#define TRACE_EVENT_INSTANT_WITH_TIMESTAMP0(category_group, name, scope,   \
                                            timestamp)                     \
  INTERNAL_TRACE_EVENT_ADD_WITH_TIMESTAMP(TRACE_EVENT_PHASE_INSTANT,       \
                                          category_group, name, timestamp, \
                                          TRACE_EVENT_FLAG_NONE | scope)

#define TRACE_EVENT_INSTANT_WITH_TIMESTAMP1(category_group, name, scope,  \
                                            timestamp, arg_name, arg_val) \
  INTERNAL_TRACE_EVENT_ADD_WITH_TIMESTAMP(                                \
      TRACE_EVENT_PHASE_INSTANT, category_group, name, timestamp,         \
      TRACE_EVENT_FLAG_NONE | scope, arg_name,                            \
      ::perfetto::internal::PossiblyNull(arg_val))

// Begin events.
#define TRACE_EVENT_BEGIN0(category_group, name)                          \
  INTERNAL_TRACE_EVENT_ADD(TRACE_EVENT_PHASE_BEGIN, category_group, name, \
                           TRACE_EVENT_FLAG_NONE)
#define TRACE_EVENT_BEGIN1(category_group, name, arg1_name, arg1_val)     \
  INTERNAL_TRACE_EVENT_ADD(TRACE_EVENT_PHASE_BEGIN, category_group, name, \
                           TRACE_EVENT_FLAG_NONE, arg1_name,              \
                           ::perfetto::internal::PossiblyNull(arg1_val))
#define TRACE_EVENT_BEGIN2(category_group, name, arg1_name, arg1_val,       \
                           arg2_name, arg2_val)                             \
  INTERNAL_TRACE_EVENT_ADD(                                                 \
      TRACE_EVENT_PHASE_BEGIN, category_group, name, TRACE_EVENT_FLAG_NONE, \
      arg1_name, ::perfetto::internal::PossiblyNull(arg1_val), arg2_name,   \
      ::perfetto::internal::PossiblyNull(arg2_val))
#define TRACE_EVENT_BEGIN_WITH_FLAGS0(category_group, name, flags) \
  INTERNAL_TRACE_EVENT_ADD(TRACE_EVENT_PHASE_BEGIN, category_group, name, flags)
#define TRACE_EVENT_BEGIN_WITH_FLAGS1(category_group, name, flags, arg1_name, \
                                      arg1_val)                               \
  INTERNAL_TRACE_EVENT_ADD(TRACE_EVENT_PHASE_BEGIN, category_group, name,     \
                           flags, arg1_name,                                  \
                           ::perfetto::internal::PossiblyNull(arg1_val))
#define TRACE_EVENT_COPY_BEGIN2(category_group, name, arg1_name, arg1_val, \
                                arg2_name, arg2_val)                       \
  INTERNAL_TRACE_EVENT_ADD(TRACE_EVENT_PHASE_BEGIN, category_group,        \
                           ::perfetto::DynamicString{name},                \
                           TRACE_EVENT_FLAG_NONE,                          \
                           ::perfetto::DynamicString{arg1_name},           \
                           ::perfetto::internal::PossiblyNull(arg1_val),   \
                           ::perfetto::DynamicString{arg2_name},           \
                           ::perfetto::internal::PossiblyNull(arg2_val))

// Begin events with explicit timestamps.
#define TRACE_EVENT_BEGIN_WITH_ID_TID_AND_TIMESTAMP0(category_group, name, id, \
                                                     thread_id, timestamp)     \
  INTERNAL_TRACE_EVENT_ADD_WITH_ID_TID_AND_TIMESTAMP(                          \
      TRACE_EVENT_PHASE_ASYNC_BEGIN, category_group, name, id, thread_id,      \
      timestamp, TRACE_EVENT_FLAG_NONE)
#define TRACE_EVENT_COPY_BEGIN_WITH_ID_TID_AND_TIMESTAMP0(       \
    category_group, name, id, thread_id, timestamp)              \
  INTERNAL_TRACE_EVENT_ADD_WITH_ID_TID_AND_TIMESTAMP(            \
      TRACE_EVENT_PHASE_ASYNC_BEGIN, category_group,             \
      ::perfetto::DynamicString{name}, id, thread_id, timestamp, \
      TRACE_EVENT_FLAG_NONE)
#define TRACE_EVENT_COPY_BEGIN_WITH_ID_TID_AND_TIMESTAMP1(               \
    category_group, name, id, thread_id, timestamp, arg1_name, arg1_val) \
  INTERNAL_TRACE_EVENT_ADD_WITH_ID_TID_AND_TIMESTAMP(                    \
      TRACE_EVENT_PHASE_ASYNC_BEGIN, category_group,                     \
      ::perfetto::DynamicString{name}, id, thread_id, timestamp,         \
      TRACE_EVENT_FLAG_NONE, ::perfetto::DynamicString{arg1_name},       \
      ::perfetto::internal::PossiblyNull(arg1_val))
#define TRACE_EVENT_COPY_BEGIN_WITH_ID_TID_AND_TIMESTAMP2(               \
    category_group, name, id, thread_id, timestamp, arg1_name, arg1_val, \
    arg2_name, arg2_val)                                                 \
  INTERNAL_TRACE_EVENT_ADD_WITH_ID_TID_AND_TIMESTAMP(                    \
      TRACE_EVENT_PHASE_ASYNC_BEGIN, category_group,                     \
      ::perfetto::DynamicString{name}, id, thread_id, timestamp,         \
      TRACE_EVENT_FLAG_NONE, ::perfetto::DynamicString{arg1_name},       \
      ::perfetto::internal::PossiblyNull(arg1_val),                      \
      ::perfetto::DynamicString{arg2_name},                              \
      ::perfetto::internal::PossiblyNull(arg2_val))

// End events.
#define TRACE_EVENT_END0(category_group, name)                          \
  INTERNAL_TRACE_EVENT_ADD(TRACE_EVENT_PHASE_END, category_group, name, \
                           TRACE_EVENT_FLAG_NONE)
#define TRACE_EVENT_END1(category_group, name, arg1_name, arg1_val)     \
  INTERNAL_TRACE_EVENT_ADD(TRACE_EVENT_PHASE_END, category_group, name, \
                           TRACE_EVENT_FLAG_NONE, arg1_name,            \
                           ::perfetto::internal::PossiblyNull(arg1_val))
#define TRACE_EVENT_END2(category_group, name, arg1_name, arg1_val, arg2_name, \
                         arg2_val)                                             \
  INTERNAL_TRACE_EVENT_ADD(                                                    \
      TRACE_EVENT_PHASE_END, category_group, name, TRACE_EVENT_FLAG_NONE,      \
      arg1_name, ::perfetto::internal::PossiblyNull(arg1_val), arg2_name,      \
      ::perfetto::internal::PossiblyNull(arg2_val))
#define TRACE_EVENT_END_WITH_FLAGS0(category_group, name, flags) \
  INTERNAL_TRACE_EVENT_ADD(TRACE_EVENT_PHASE_END, category_group, name, flags)
#define TRACE_EVENT_END_WITH_FLAGS1(category_group, name, flags, arg1_name,    \
                                    arg1_val)                                  \
  INTERNAL_TRACE_EVENT_ADD(TRACE_EVENT_PHASE_END, category_group, name, flags, \
                           arg1_name,                                          \
                           ::perfetto::internal::PossiblyNull(arg1_val))
#define TRACE_EVENT_COPY_END2(category_group, name, arg1_name, arg1_val,      \
                              arg2_name, arg2_val)                            \
  INTERNAL_TRACE_EVENT_ADD(                                                   \
      TRACE_EVENT_PHASE_END, category_group, ::perfetto::DynamicString{name}, \
      TRACE_EVENT_FLAG_NONE, ::perfetto::DynamicString{arg1_name},            \
      ::perfetto::internal::PossiblyNull(arg1_val),                           \
      ::perfetto::DynamicString{arg2_name},                                   \
      ::perfetto::internal::PossiblyNull(arg2_val))

// Mark events.
#define TRACE_EVENT_MARK_WITH_TIMESTAMP0(category_group, name, timestamp)  \
  INTERNAL_TRACE_EVENT_ADD_WITH_TIMESTAMP(TRACE_EVENT_PHASE_MARK,          \
                                          category_group, name, timestamp, \
                                          TRACE_EVENT_FLAG_NONE)

#define TRACE_EVENT_MARK_WITH_TIMESTAMP1(category_group, name, timestamp, \
                                         arg1_name, arg1_val)             \
  INTERNAL_TRACE_EVENT_ADD_WITH_TIMESTAMP(                                \
      TRACE_EVENT_PHASE_MARK, category_group, name, timestamp,            \
      TRACE_EVENT_FLAG_NONE, arg1_name,                                   \
      ::perfetto::internal::PossiblyNull(arg1_val))

#define TRACE_EVENT_MARK_WITH_TIMESTAMP2(                                      \
    category_group, name, timestamp, arg1_name, arg1_val, arg2_name, arg2_val) \
  INTERNAL_TRACE_EVENT_ADD_WITH_TIMESTAMP(                                     \
      TRACE_EVENT_PHASE_MARK, category_group, name, timestamp,                 \
      TRACE_EVENT_FLAG_NONE, arg1_name,                                        \
      ::perfetto::internal::PossiblyNull(arg1_val), arg2_name,                 \
      ::perfetto::internal::PossiblyNull(arg2_val))

#define TRACE_EVENT_COPY_MARK(category_group, name)                \
  INTERNAL_TRACE_EVENT_ADD(TRACE_EVENT_PHASE_MARK, category_group, \
                           ::perfetto::DynamicString{name},        \
                           TRACE_EVENT_FLAG_NONE)

#define TRACE_EVENT_COPY_MARK1(category_group, name, arg1_name, arg1_val)      \
  INTERNAL_TRACE_EVENT_ADD(                                                    \
      TRACE_EVENT_PHASE_MARK, category_group, ::perfetto::DynamicString{name}, \
      TRACE_EVENT_FLAG_NONE, ::perfetto::DynamicString{arg1_name},             \
      ::perfetto::internal::PossiblyNull(arg1_val))

#define TRACE_EVENT_COPY_MARK_WITH_TIMESTAMP(category_group, name, timestamp)  \
  INTERNAL_TRACE_EVENT_ADD_WITH_TIMESTAMP(                                     \
      TRACE_EVENT_PHASE_MARK, category_group, ::perfetto::DynamicString{name}, \
      timestamp, TRACE_EVENT_FLAG_NONE)

// End events with explicit thread and timestamp.
#define TRACE_EVENT_END_WITH_ID_TID_AND_TIMESTAMP0(category_group, name, id, \
                                                   thread_id, timestamp)     \
  INTERNAL_TRACE_EVENT_ADD_WITH_ID_TID_AND_TIMESTAMP(                        \
      TRACE_EVENT_PHASE_ASYNC_END, category_group, name, id, thread_id,      \
      timestamp, TRACE_EVENT_FLAG_NONE)
#define TRACE_EVENT_COPY_END_WITH_ID_TID_AND_TIMESTAMP0(         \
    category_group, name, id, thread_id, timestamp)              \
  INTERNAL_TRACE_EVENT_ADD_WITH_ID_TID_AND_TIMESTAMP(            \
      TRACE_EVENT_PHASE_ASYNC_END, category_group,               \
      ::perfetto::DynamicString{name}, id, thread_id, timestamp, \
      TRACE_EVENT_FLAG_NONE)
#define TRACE_EVENT_COPY_END_WITH_ID_TID_AND_TIMESTAMP1(                 \
    category_group, name, id, thread_id, timestamp, arg1_name, arg1_val) \
  INTERNAL_TRACE_EVENT_ADD_WITH_ID_TID_AND_TIMESTAMP(                    \
      TRACE_EVENT_PHASE_ASYNC_END, category_group,                       \
      ::perfetto::DynamicString{name}, id, thread_id, timestamp,         \
      TRACE_EVENT_FLAG_NONE, ::perfetto::DynamicString{arg1_name},       \
      ::perfetto::internal::PossiblyNull(arg1_val))
#define TRACE_EVENT_COPY_END_WITH_ID_TID_AND_TIMESTAMP2(                 \
    category_group, name, id, thread_id, timestamp, arg1_name, arg1_val, \
    arg2_name, arg2_val)                                                 \
  INTERNAL_TRACE_EVENT_ADD_WITH_ID_TID_AND_TIMESTAMP(                    \
      TRACE_EVENT_PHASE_ASYNC_END, category_group,                       \
      ::perfetto::DynamicString{name}, id, thread_id, timestamp,         \
      TRACE_EVENT_FLAG_NONE, ::perfetto::DynamicString{arg1_name},       \
      ::perfetto::internal::PossiblyNull(arg1_val),                      \
      ::perfetto::DynamicString{arg2_name},                              \
      ::perfetto::internal::PossiblyNull(arg2_val))

// Counters.
#define TRACE_COUNTER1(category_group, name, value)                         \
  INTERNAL_TRACE_EVENT_ADD(TRACE_EVENT_PHASE_COUNTER, category_group, name, \
                           TRACE_EVENT_FLAG_NONE, "value",                  \
                           static_cast<int>(value))
#define TRACE_COUNTER_WITH_FLAG1(category_group, name, flag, value)         \
  INTERNAL_TRACE_EVENT_ADD(TRACE_EVENT_PHASE_COUNTER, category_group, name, \
                           flag, "value", static_cast<int>(value))
#define TRACE_COPY_COUNTER1(category_group, name, value)              \
  INTERNAL_TRACE_EVENT_ADD(TRACE_EVENT_PHASE_COUNTER, category_group, \
                           ::perfetto::DynamicString{name},           \
                           TRACE_EVENT_FLAG_NONE, "value",            \
                           static_cast<int>(value))
#define TRACE_COUNTER2(category_group, name, value1_name, value1_val,       \
                       value2_name, value2_val)                             \
  INTERNAL_TRACE_EVENT_ADD(TRACE_EVENT_PHASE_COUNTER, category_group, name, \
                           TRACE_EVENT_FLAG_NONE, value1_name,              \
                           static_cast<int>(value1_val), value2_name,       \
                           static_cast<int>(value2_val))
#define TRACE_COPY_COUNTER2(category_group, name, value1_name, value1_val, \
                            value2_name, value2_val)                       \
  INTERNAL_TRACE_EVENT_ADD(                                                \
      TRACE_EVENT_PHASE_COUNTER, category_group,                           \
      ::perfetto::DynamicString{name}, TRACE_EVENT_FLAG_NONE, value1_name, \
      static_cast<int>(value1_val), value2_name, static_cast<int>(value2_val))

// Counters with explicit timestamps.
#define TRACE_COUNTER_WITH_TIMESTAMP1(category_group, name, timestamp, value) \
  INTERNAL_TRACE_EVENT_ADD_WITH_TIMESTAMP(                                    \
      TRACE_EVENT_PHASE_COUNTER, category_group, name, timestamp,             \
      TRACE_EVENT_FLAG_NONE, "value", static_cast<int>(value))

#define TRACE_COUNTER_WITH_TIMESTAMP2(category_group, name, timestamp,      \
                                      value1_name, value1_val, value2_name, \
                                      value2_val)                           \
  INTERNAL_TRACE_EVENT_ADD_WITH_TIMESTAMP(                                  \
      TRACE_EVENT_PHASE_COUNTER, category_group, name, timestamp,           \
      TRACE_EVENT_FLAG_NONE, value1_name, static_cast<int>(value1_val),     \
      value2_name, static_cast<int>(value2_val))

// Counters with ids.
#define TRACE_COUNTER_ID1(category_group, name, id, value)                    \
  INTERNAL_TRACE_EVENT_ADD_WITH_ID(TRACE_EVENT_PHASE_COUNTER, category_group, \
                                   name, id, TRACE_EVENT_FLAG_NONE, "value",  \
                                   static_cast<int>(value))
#define TRACE_COPY_COUNTER_ID1(category_group, name, id, value)               \
  INTERNAL_TRACE_EVENT_ADD_WITH_ID(TRACE_EVENT_PHASE_COUNTER, category_group, \
                                   ::perfetto::DynamicString{name}, id,       \
                                   TRACE_EVENT_FLAG_NONE, "value",            \
                                   static_cast<int>(value))
#define TRACE_COUNTER_ID2(category_group, name, id, value1_name, value1_val,  \
                          value2_name, value2_val)                            \
  INTERNAL_TRACE_EVENT_ADD_WITH_ID(TRACE_EVENT_PHASE_COUNTER, category_group, \
                                   name, id, TRACE_EVENT_FLAG_NONE,           \
                                   value1_name, static_cast<int>(value1_val), \
                                   value2_name, static_cast<int>(value2_val))
#define TRACE_COPY_COUNTER_ID2(category_group, name, id, value1_name,          \
                               value1_val, value2_name, value2_val)            \
  INTERNAL_TRACE_EVENT_ADD_WITH_ID(                                            \
      TRACE_EVENT_PHASE_COUNTER, category_group,                               \
      ::perfetto::DynamicString{name}, id, TRACE_EVENT_FLAG_NONE, value1_name, \
      static_cast<int>(value1_val), value2_name, static_cast<int>(value2_val))

// Sampling profiler events.
#define TRACE_EVENT_SAMPLE_WITH_ID1(category_group, name, id, arg1_name,       \
                                    arg1_val)                                  \
  INTERNAL_TRACE_EVENT_ADD_WITH_ID(TRACE_EVENT_PHASE_SAMPLE, category_group,   \
                                   name, id, TRACE_EVENT_FLAG_NONE, arg1_name, \
                                   arg1_val)

// Legacy async events.
#define TRACE_EVENT_ASYNC_BEGIN0(category_group, name, id)        \
  INTERNAL_TRACE_EVENT_ADD_WITH_ID(TRACE_EVENT_PHASE_ASYNC_BEGIN, \
                                   category_group, name, id,      \
                                   TRACE_EVENT_FLAG_NONE)
#define TRACE_EVENT_ASYNC_BEGIN1(category_group, name, id, arg1_name, \
                                 arg1_val)                            \
  INTERNAL_TRACE_EVENT_ADD_WITH_ID(                                   \
      TRACE_EVENT_PHASE_ASYNC_BEGIN, category_group, name, id,        \
      TRACE_EVENT_FLAG_NONE, arg1_name,                               \
      ::perfetto::internal::PossiblyNull(arg1_val))
#define TRACE_EVENT_ASYNC_BEGIN2(category_group, name, id, arg1_name, \
                                 arg1_val, arg2_name, arg2_val)       \
  INTERNAL_TRACE_EVENT_ADD_WITH_ID(                                   \
      TRACE_EVENT_PHASE_ASYNC_BEGIN, category_group, name, id,        \
      TRACE_EVENT_FLAG_NONE, arg1_name,                               \
      ::perfetto::internal::PossiblyNull(arg1_val), arg2_name,        \
      ::perfetto::internal::PossiblyNull(arg2_val))
#define TRACE_EVENT_COPY_ASYNC_BEGIN0(category_group, name, id) \
  INTERNAL_TRACE_EVENT_ADD_WITH_ID(                             \
      TRACE_EVENT_PHASE_ASYNC_BEGIN, category_group,            \
      ::perfetto::DynamicString{name}, id, TRACE_EVENT_FLAG_NONE)
#define TRACE_EVENT_COPY_ASYNC_BEGIN1(category_group, name, id, arg1_name, \
                                      arg1_val)                            \
  INTERNAL_TRACE_EVENT_ADD_WITH_ID(                                        \
      TRACE_EVENT_PHASE_ASYNC_BEGIN, category_group,                       \
      ::perfetto::DynamicString{name}, id, TRACE_EVENT_FLAG_NONE,          \
      ::perfetto::DynamicString{arg1_name},                                \
      ::perfetto::internal::PossiblyNull(arg1_val))
#define TRACE_EVENT_COPY_ASYNC_BEGIN2(category_group, name, id, arg1_name, \
                                      arg1_val, arg2_name, arg2_val)       \
  INTERNAL_TRACE_EVENT_ADD_WITH_ID(                                        \
      TRACE_EVENT_PHASE_ASYNC_BEGIN, category_group,                       \
      ::perfetto::DynamicString{name}, id, TRACE_EVENT_FLAG_NONE,          \
      ::perfetto::DynamicString{arg1_name},                                \
      ::perfetto::internal::PossiblyNull(arg1_val),                        \
      ::perfetto::DynamicString{arg2_name},                                \
      ::perfetto::internal::PossiblyNull(arg2_val))
#define TRACE_EVENT_ASYNC_BEGIN_WITH_FLAGS0(category_group, name, id, flags) \
  INTERNAL_TRACE_EVENT_ADD_WITH_ID(TRACE_EVENT_PHASE_ASYNC_BEGIN,            \
                                   category_group, name, id, flags)

// Legacy async events with explicit timestamps.
#define TRACE_EVENT_ASYNC_BEGIN_WITH_TIMESTAMP0(category_group, name, id, \
                                                timestamp)                \
  INTERNAL_TRACE_EVENT_ADD_WITH_ID_TID_AND_TIMESTAMP(                     \
      TRACE_EVENT_PHASE_ASYNC_BEGIN, category_group, name, id,            \
      TRACE_EVENT_API_CURRENT_THREAD_ID, timestamp, TRACE_EVENT_FLAG_NONE)
#define TRACE_EVENT_ASYNC_BEGIN_WITH_TIMESTAMP1(                           \
    category_group, name, id, timestamp, arg1_name, arg1_val)              \
  INTERNAL_TRACE_EVENT_ADD_WITH_ID_TID_AND_TIMESTAMP(                      \
      TRACE_EVENT_PHASE_ASYNC_BEGIN, category_group, name, id,             \
      TRACE_EVENT_API_CURRENT_THREAD_ID, timestamp, TRACE_EVENT_FLAG_NONE, \
      arg1_name, ::perfetto::internal::PossiblyNull(arg1_val))
#define TRACE_EVENT_NESTABLE_ASYNC_BEGIN_WITH_TIMESTAMP_AND_FLAGS0(     \
    category_group, name, id, timestamp, flags)                         \
  INTERNAL_TRACE_EVENT_ADD_WITH_ID_TID_AND_TIMESTAMP(                   \
      TRACE_EVENT_PHASE_NESTABLE_ASYNC_BEGIN, category_group, name, id, \
      TRACE_EVENT_API_CURRENT_THREAD_ID, timestamp, flags)
#define TRACE_EVENT_NESTABLE_ASYNC_END_WITH_TIMESTAMP0(category_group, name, \
                                                       id, timestamp)        \
  INTERNAL_TRACE_EVENT_ADD_WITH_ID_TID_AND_TIMESTAMP(                        \
      TRACE_EVENT_PHASE_NESTABLE_ASYNC_END, category_group, name, id,        \
      TRACE_EVENT_API_CURRENT_THREAD_ID, timestamp, TRACE_EVENT_FLAG_NONE)
#define TRACE_EVENT_ASYNC_BEGIN_WITH_TIMESTAMP2(category_group, name, id,      \
                                                timestamp, arg1_name,          \
                                                arg1_val, arg2_name, arg2_val) \
  INTERNAL_TRACE_EVENT_ADD_WITH_ID_TID_AND_TIMESTAMP(                          \
      TRACE_EVENT_PHASE_ASYNC_BEGIN, category_group, name, id,                 \
      TRACE_EVENT_API_CURRENT_THREAD_ID, timestamp, TRACE_EVENT_FLAG_NONE,     \
      arg1_name, ::perfetto::internal::PossiblyNull(arg1_val), arg2_name,      \
      ::perfetto::internal::PossiblyNull(arg2_val))
#define TRACE_EVENT_COPY_ASYNC_BEGIN_WITH_TIMESTAMP0(category_group, name, id, \
                                                     timestamp)                \
  INTERNAL_TRACE_EVENT_ADD_WITH_ID_TID_AND_TIMESTAMP(                          \
      TRACE_EVENT_PHASE_ASYNC_BEGIN, category_group,                           \
      ::perfetto::DynamicString{name}, id, TRACE_EVENT_API_CURRENT_THREAD_ID,  \
      timestamp, TRACE_EVENT_FLAG_NONE)
#define TRACE_EVENT_ASYNC_BEGIN_WITH_TIMESTAMP_AND_FLAGS0(     \
    category_group, name, id, timestamp, flags)                \
  INTERNAL_TRACE_EVENT_ADD_WITH_ID_TID_AND_TIMESTAMP(          \
      TRACE_EVENT_PHASE_ASYNC_BEGIN, category_group, name, id, \
      TRACE_EVENT_API_CURRENT_THREAD_ID, timestamp, flags)

// Legacy async step into events.
#define TRACE_EVENT_ASYNC_STEP_INTO0(category_group, name, id, step)  \
  INTERNAL_TRACE_EVENT_ADD_WITH_ID(TRACE_EVENT_PHASE_ASYNC_STEP_INTO, \
                                   category_group, name, id,          \
                                   TRACE_EVENT_FLAG_NONE, "step", step)
#define TRACE_EVENT_ASYNC_STEP_INTO1(category_group, name, id, step, \
                                     arg1_name, arg1_val)            \
  INTERNAL_TRACE_EVENT_ADD_WITH_ID(                                  \
      TRACE_EVENT_PHASE_ASYNC_STEP_INTO, category_group, name, id,   \
      TRACE_EVENT_FLAG_NONE, "step", step, arg1_name,                \
      ::perfetto::internal::PossiblyNull(arg1_val))

// Legacy async step into events with timestamps.
#define TRACE_EVENT_ASYNC_STEP_INTO_WITH_TIMESTAMP0(category_group, name, id, \
                                                    step, timestamp)          \
  INTERNAL_TRACE_EVENT_ADD_WITH_ID_TID_AND_TIMESTAMP(                         \
      TRACE_EVENT_PHASE_ASYNC_STEP_INTO, category_group, name, id,            \
      TRACE_EVENT_API_CURRENT_THREAD_ID, timestamp, TRACE_EVENT_FLAG_NONE,    \
      "step", step)

// Legacy async step past events.
#define TRACE_EVENT_ASYNC_STEP_PAST0(category_group, name, id, step)  \
  INTERNAL_TRACE_EVENT_ADD_WITH_ID(TRACE_EVENT_PHASE_ASYNC_STEP_PAST, \
                                   category_group, name, id,          \
                                   TRACE_EVENT_FLAG_NONE, "step", step)
#define TRACE_EVENT_ASYNC_STEP_PAST1(category_group, name, id, step, \
                                     arg1_name, arg1_val)            \
  INTERNAL_TRACE_EVENT_ADD_WITH_ID(                                  \
      TRACE_EVENT_PHASE_ASYNC_STEP_PAST, category_group, name, id,   \
      TRACE_EVENT_FLAG_NONE, "step", step, arg1_name,                \
      ::perfetto::internal::PossiblyNull(arg1_val))

// Legacy async end events.
#define TRACE_EVENT_ASYNC_END0(category_group, name, id)        \
  INTERNAL_TRACE_EVENT_ADD_WITH_ID(TRACE_EVENT_PHASE_ASYNC_END, \
                                   category_group, name, id,    \
                                   TRACE_EVENT_FLAG_NONE)
#define TRACE_EVENT_ASYNC_END1(category_group, name, id, arg1_name, arg1_val) \
  INTERNAL_TRACE_EVENT_ADD_WITH_ID(                                           \
      TRACE_EVENT_PHASE_ASYNC_END, category_group, name, id,                  \
      TRACE_EVENT_FLAG_NONE, arg1_name,                                       \
      ::perfetto::internal::PossiblyNull(arg1_val))
#define TRACE_EVENT_ASYNC_END2(category_group, name, id, arg1_name, arg1_val, \
                               arg2_name, arg2_val)                           \
  INTERNAL_TRACE_EVENT_ADD_WITH_ID(                                           \
      TRACE_EVENT_PHASE_ASYNC_END, category_group, name, id,                  \
      TRACE_EVENT_FLAG_NONE, arg1_name,                                       \
      ::perfetto::internal::PossiblyNull(arg1_val), arg2_name,                \
      ::perfetto::internal::PossiblyNull(arg2_val))
#define TRACE_EVENT_COPY_ASYNC_END0(category_group, name, id) \
  INTERNAL_TRACE_EVENT_ADD_WITH_ID(                           \
      TRACE_EVENT_PHASE_ASYNC_END, category_group,            \
      ::perfetto::DynamicString{name}, id, TRACE_EVENT_FLAG_NONE)
#define TRACE_EVENT_COPY_ASYNC_END1(category_group, name, id, arg1_name, \
                                    arg1_val)                            \
  INTERNAL_TRACE_EVENT_ADD_WITH_ID(                                      \
      TRACE_EVENT_PHASE_ASYNC_END, category_group,                       \
      ::perfetto::DynamicString{name}, id, TRACE_EVENT_FLAG_NONE,        \
      ::perfetto::DynamicString{arg1_name},                              \
      ::perfetto::internal::PossiblyNull(arg1_val))
#define TRACE_EVENT_COPY_ASYNC_END2(category_group, name, id, arg1_name, \
                                    arg1_val, arg2_name, arg2_val)       \
  INTERNAL_TRACE_EVENT_ADD_WITH_ID(                                      \
      TRACE_EVENT_PHASE_ASYNC_END, category_group,                       \
      ::perfetto::DynamicString{name}, id, TRACE_EVENT_FLAG_NONE,        \
      ::perfetto::DynamicString{arg1_name},                              \
      ::perfetto::internal::PossiblyNull(arg1_val),                      \
      ::perfetto::DynamicString{arg2_name},                              \
      ::perfetto::internal::PossiblyNull(arg2_val))
#define TRACE_EVENT_ASYNC_END_WITH_FLAGS0(category_group, name, id, flags) \
  INTERNAL_TRACE_EVENT_ADD_WITH_ID(TRACE_EVENT_PHASE_ASYNC_END,            \
                                   category_group, name, id, flags)

// Legacy async end events with explicit timestamps.
#define TRACE_EVENT_ASYNC_END_WITH_TIMESTAMP0(category_group, name, id, \
                                              timestamp)                \
  INTERNAL_TRACE_EVENT_ADD_WITH_ID_TID_AND_TIMESTAMP(                   \
      TRACE_EVENT_PHASE_ASYNC_END, category_group, name, id,            \
      TRACE_EVENT_API_CURRENT_THREAD_ID, timestamp, TRACE_EVENT_FLAG_NONE)
#define TRACE_EVENT_ASYNC_END_WITH_TIMESTAMP1(category_group, name, id,       \
                                              timestamp, arg1_name, arg1_val) \
  INTERNAL_TRACE_EVENT_ADD_WITH_ID_TID_AND_TIMESTAMP(                         \
      TRACE_EVENT_PHASE_ASYNC_END, category_group, name, id,                  \
      TRACE_EVENT_API_CURRENT_THREAD_ID, timestamp, TRACE_EVENT_FLAG_NONE,    \
      arg1_name, ::perfetto::internal::PossiblyNull(arg1_val))
#define TRACE_EVENT_ASYNC_END_WITH_TIMESTAMP2(category_group, name, id,       \
                                              timestamp, arg1_name, arg1_val, \
                                              arg2_name, arg2_val)            \
  INTERNAL_TRACE_EVENT_ADD_WITH_ID_TID_AND_TIMESTAMP(                         \
      TRACE_EVENT_PHASE_ASYNC_END, category_group, name, id,                  \
      TRACE_EVENT_API_CURRENT_THREAD_ID, timestamp, TRACE_EVENT_FLAG_NONE,    \
      arg1_name, ::perfetto::internal::PossiblyNull(arg1_val), arg2_name,     \
      ::perfetto::internal::PossiblyNull(arg2_val))
#define TRACE_EVENT_COPY_ASYNC_END_WITH_TIMESTAMP0(category_group, name, id,  \
                                                   timestamp)                 \
  INTERNAL_TRACE_EVENT_ADD_WITH_ID_TID_AND_TIMESTAMP(                         \
      TRACE_EVENT_PHASE_ASYNC_END, category_group,                            \
      ::perfetto::DynamicString{name}, id, TRACE_EVENT_API_CURRENT_THREAD_ID, \
      timestamp, TRACE_EVENT_FLAG_NONE)
#define TRACE_EVENT_ASYNC_END_WITH_TIMESTAMP_AND_FLAGS0(category_group, name, \
                                                        id, timestamp, flags) \
  INTERNAL_TRACE_EVENT_ADD_WITH_ID_TID_AND_TIMESTAMP(                         \
      TRACE_EVENT_PHASE_ASYNC_END, category_group, name, id,                  \
      TRACE_EVENT_API_CURRENT_THREAD_ID, timestamp, flags)

// Async events.
#define TRACE_EVENT_NESTABLE_ASYNC_BEGIN0(category_group, name, id)        \
  INTERNAL_TRACE_EVENT_ADD_WITH_ID(TRACE_EVENT_PHASE_NESTABLE_ASYNC_BEGIN, \
                                   category_group, name, id,               \
                                   TRACE_EVENT_FLAG_NONE)
#define TRACE_EVENT_NESTABLE_ASYNC_BEGIN1(category_group, name, id, arg1_name, \
                                          arg1_val)                            \
  INTERNAL_TRACE_EVENT_ADD_WITH_ID(                                            \
      TRACE_EVENT_PHASE_NESTABLE_ASYNC_BEGIN, category_group, name, id,        \
      TRACE_EVENT_FLAG_NONE, arg1_name,                                        \
      ::perfetto::internal::PossiblyNull(arg1_val))
#define TRACE_EVENT_NESTABLE_ASYNC_BEGIN2(category_group, name, id, arg1_name, \
                                          arg1_val, arg2_name, arg2_val)       \
  INTERNAL_TRACE_EVENT_ADD_WITH_ID(                                            \
      TRACE_EVENT_PHASE_NESTABLE_ASYNC_BEGIN, category_group, name, id,        \
      TRACE_EVENT_FLAG_NONE, arg1_name,                                        \
      ::perfetto::internal::PossiblyNull(arg1_val), arg2_name,                 \
      ::perfetto::internal::PossiblyNull(arg2_val))
#define TRACE_EVENT_NESTABLE_ASYNC_BEGIN_WITH_FLAGS0(category_group, name, id, \
                                                     flags)                    \
  INTERNAL_TRACE_EVENT_ADD_WITH_ID(TRACE_EVENT_PHASE_NESTABLE_ASYNC_BEGIN,     \
                                   category_group, name, id, flags)
#define TRACE_EVENT_NESTABLE_ASYNC_BEGIN_WITH_TIMESTAMP1(                  \
    category_group, name, id, timestamp, arg1_name, arg1_val)              \
  INTERNAL_TRACE_EVENT_ADD_WITH_ID_TID_AND_TIMESTAMP(                      \
      TRACE_EVENT_PHASE_NESTABLE_ASYNC_BEGIN, category_group, name, id,    \
      TRACE_EVENT_API_CURRENT_THREAD_ID, timestamp, TRACE_EVENT_FLAG_NONE, \
      arg1_name, ::perfetto::internal::PossiblyNull(arg1_val))

// Async end events.
#define TRACE_EVENT_NESTABLE_ASYNC_END0(category_group, name, id)        \
  INTERNAL_TRACE_EVENT_ADD_WITH_ID(TRACE_EVENT_PHASE_NESTABLE_ASYNC_END, \
                                   category_group, name, id,             \
                                   TRACE_EVENT_FLAG_NONE)
#define TRACE_EVENT_NESTABLE_ASYNC_END1(category_group, name, id, arg1_name, \
                                        arg1_val)                            \
  INTERNAL_TRACE_EVENT_ADD_WITH_ID(                                          \
      TRACE_EVENT_PHASE_NESTABLE_ASYNC_END, category_group, name, id,        \
      TRACE_EVENT_FLAG_NONE, arg1_name,                                      \
      ::perfetto::internal::PossiblyNull(arg1_val))
#define TRACE_EVENT_NESTABLE_ASYNC_END2(category_group, name, id, arg1_name, \
                                        arg1_val, arg2_name, arg2_val)       \
  INTERNAL_TRACE_EVENT_ADD_WITH_ID(                                          \
      TRACE_EVENT_PHASE_NESTABLE_ASYNC_END, category_group, name, id,        \
      TRACE_EVENT_FLAG_NONE, arg1_name,                                      \
      ::perfetto::internal::PossiblyNull(arg1_val), arg2_name,               \
      ::perfetto::internal::PossiblyNull(arg2_val))
#define TRACE_EVENT_NESTABLE_ASYNC_END_WITH_FLAGS0(category_group, name, id, \
                                                   flags)                    \
  INTERNAL_TRACE_EVENT_ADD_WITH_ID(TRACE_EVENT_PHASE_NESTABLE_ASYNC_END,     \
                                   category_group, name, id, flags)

// Async instant events.
#define TRACE_EVENT_NESTABLE_ASYNC_INSTANT0(category_group, name, id)        \
  INTERNAL_TRACE_EVENT_ADD_WITH_ID(TRACE_EVENT_PHASE_NESTABLE_ASYNC_INSTANT, \
                                   category_group, name, id,                 \
                                   TRACE_EVENT_FLAG_NONE)
#define TRACE_EVENT_NESTABLE_ASYNC_INSTANT1(category_group, name, id,     \
                                            arg1_name, arg1_val)          \
  INTERNAL_TRACE_EVENT_ADD_WITH_ID(                                       \
      TRACE_EVENT_PHASE_NESTABLE_ASYNC_INSTANT, category_group, name, id, \
      TRACE_EVENT_FLAG_NONE, arg1_name,                                   \
      ::perfetto::internal::PossiblyNull(arg1_val))
#define TRACE_EVENT_NESTABLE_ASYNC_INSTANT2(                              \
    category_group, name, id, arg1_name, arg1_val, arg2_name, arg2_val)   \
  INTERNAL_TRACE_EVENT_ADD_WITH_ID(                                       \
      TRACE_EVENT_PHASE_NESTABLE_ASYNC_INSTANT, category_group, name, id, \
      TRACE_EVENT_FLAG_NONE, arg1_name,                                   \
      ::perfetto::internal::PossiblyNull(arg1_val), arg2_name,            \
      ::perfetto::internal::PossiblyNull(arg2_val))
#define TRACE_EVENT_COPY_NESTABLE_ASYNC_BEGIN_WITH_TTS2(                \
    category_group, name, id, arg1_name, arg1_val, arg2_name, arg2_val) \
  INTERNAL_TRACE_EVENT_ADD_WITH_ID(                                     \
      TRACE_EVENT_PHASE_NESTABLE_ASYNC_BEGIN, category_group,           \
      ::perfetto::DynamicString{name}, id, TRACE_EVENT_FLAG_ASYNC_TTS,  \
      ::perfetto::DynamicString{arg1_name},                             \
      ::perfetto::internal::PossiblyNull(arg1_val),                     \
      ::perfetto::DynamicString{arg2_name},                             \
      ::perfetto::internal::PossiblyNull(arg2_val))
#define TRACE_EVENT_COPY_NESTABLE_ASYNC_END_WITH_TTS2(                  \
    category_group, name, id, arg1_name, arg1_val, arg2_name, arg2_val) \
  INTERNAL_TRACE_EVENT_ADD_WITH_ID(                                     \
      TRACE_EVENT_PHASE_NESTABLE_ASYNC_END, category_group,             \
      ::perfetto::DynamicString{name}, id, TRACE_EVENT_FLAG_ASYNC_TTS,  \
      ::perfetto::DynamicString{arg1_name},                             \
      ::perfetto::internal::PossiblyNull(arg1_val),                     \
      ::perfetto::DynamicString{arg2_name},                             \
      ::perfetto::internal::PossiblyNull(arg2_val))

// Async events with explicit timestamps.
#define TRACE_EVENT_NESTABLE_ASYNC_BEGIN_WITH_TIMESTAMP0(category_group, name, \
                                                         id, timestamp)        \
  INTERNAL_TRACE_EVENT_ADD_WITH_ID_TID_AND_TIMESTAMP(                          \
      TRACE_EVENT_PHASE_NESTABLE_ASYNC_BEGIN, category_group, name, id,        \
      TRACE_EVENT_API_CURRENT_THREAD_ID, timestamp, TRACE_EVENT_FLAG_NONE)
#define TRACE_EVENT_NESTABLE_ASYNC_END_WITH_TIMESTAMP0(category_group, name, \
                                                       id, timestamp)        \
  INTERNAL_TRACE_EVENT_ADD_WITH_ID_TID_AND_TIMESTAMP(                        \
      TRACE_EVENT_PHASE_NESTABLE_ASYNC_END, category_group, name, id,        \
      TRACE_EVENT_API_CURRENT_THREAD_ID, timestamp, TRACE_EVENT_FLAG_NONE)
#define TRACE_EVENT_NESTABLE_ASYNC_END_WITH_TIMESTAMP1(                    \
    category_group, name, id, timestamp, arg1_name, arg1_val)              \
  INTERNAL_TRACE_EVENT_ADD_WITH_ID_TID_AND_TIMESTAMP(                      \
      TRACE_EVENT_PHASE_NESTABLE_ASYNC_END, category_group, name, id,      \
      TRACE_EVENT_API_CURRENT_THREAD_ID, timestamp, TRACE_EVENT_FLAG_NONE, \
      arg1_name, ::perfetto::internal::PossiblyNull(arg1_val))
#define TRACE_EVENT_NESTABLE_ASYNC_END_WITH_TIMESTAMP2(                    \
    category_group, name, id, timestamp, arg1_name, arg1_val, arg2_name,   \
    arg2_val)                                                              \
  INTERNAL_TRACE_EVENT_ADD_WITH_ID_TID_AND_TIMESTAMP(                      \
      TRACE_EVENT_PHASE_NESTABLE_ASYNC_END, category_group, name, id,      \
      TRACE_EVENT_API_CURRENT_THREAD_ID, timestamp, TRACE_EVENT_FLAG_NONE, \
      arg1_name, ::perfetto::internal::PossiblyNull(arg1_val), arg2_name,  \
      ::perfetto::internal::PossiblyNull(arg2_val))
#define TRACE_EVENT_NESTABLE_ASYNC_END_WITH_TIMESTAMP_AND_FLAGS0(     \
    category_group, name, id, timestamp, flags)                       \
  INTERNAL_TRACE_EVENT_ADD_WITH_ID_TID_AND_TIMESTAMP(                 \
      TRACE_EVENT_PHASE_NESTABLE_ASYNC_END, category_group, name, id, \
      TRACE_EVENT_API_CURRENT_THREAD_ID, timestamp, flags)
#define TRACE_EVENT_NESTABLE_ASYNC_INSTANT_WITH_TIMESTAMP0(               \
    category_group, name, id, timestamp)                                  \
  INTERNAL_TRACE_EVENT_ADD_WITH_ID_TID_AND_TIMESTAMP(                     \
      TRACE_EVENT_PHASE_NESTABLE_ASYNC_INSTANT, category_group, name, id, \
      TRACE_EVENT_API_CURRENT_THREAD_ID, timestamp, TRACE_EVENT_FLAG_NONE)
#define TRACE_EVENT_COPY_NESTABLE_ASYNC_BEGIN0(category_group, name, id) \
  INTERNAL_TRACE_EVENT_ADD_WITH_ID(                                      \
      TRACE_EVENT_PHASE_NESTABLE_ASYNC_BEGIN, category_group,            \
      ::perfetto::DynamicString{name}, id, TRACE_EVENT_FLAG_NONE)
#define TRACE_EVENT_COPY_NESTABLE_ASYNC_BEGIN1(category_group, name, id, \
                                               arg1_name, arg1_val)      \
  INTERNAL_TRACE_EVENT_ADD_WITH_ID(                                      \
      TRACE_EVENT_PHASE_NESTABLE_ASYNC_BEGIN, category_group,            \
      ::perfetto::DynamicString{name}, id, TRACE_EVENT_FLAG_NONE,        \
      ::perfetto::DynamicString{arg1_name},                              \
      ::perfetto::internal::PossiblyNull(arg1_val))
#define TRACE_EVENT_COPY_NESTABLE_ASYNC_BEGIN2(                         \
    category_group, name, id, arg1_name, arg1_val, arg2_name, arg2_val) \
  INTERNAL_TRACE_EVENT_ADD_WITH_ID(                                     \
      TRACE_EVENT_PHASE_NESTABLE_ASYNC_BEGIN, category_group,           \
      ::perfetto::DynamicString{name}, id, TRACE_EVENT_FLAG_NONE,       \
      ::perfetto::DynamicString{arg1_name},                             \
      ::perfetto::internal::PossiblyNull(arg1_val),                     \
      ::perfetto::DynamicString{arg2_name},                             \
      ::perfetto::internal::PossiblyNull(arg2_val))
#define TRACE_EVENT_COPY_NESTABLE_ASYNC_END0(category_group, name, id) \
  INTERNAL_TRACE_EVENT_ADD_WITH_ID(                                    \
      TRACE_EVENT_PHASE_NESTABLE_ASYNC_END, category_group,            \
      ::perfetto::DynamicString{name}, id, TRACE_EVENT_FLAG_NONE)
#define TRACE_EVENT_COPY_NESTABLE_ASYNC_BEGIN_WITH_TIMESTAMP0(                \
    category_group, name, id, timestamp)                                      \
  INTERNAL_TRACE_EVENT_ADD_WITH_ID_TID_AND_TIMESTAMP(                         \
      TRACE_EVENT_PHASE_NESTABLE_ASYNC_BEGIN, category_group,                 \
      ::perfetto::DynamicString{name}, id, TRACE_EVENT_API_CURRENT_THREAD_ID, \
      timestamp, TRACE_EVENT_FLAG_NONE)
#define TRACE_EVENT_COPY_NESTABLE_ASYNC_BEGIN_WITH_TIMESTAMP1(                \
    category_group, name, id, timestamp, arg1_name, arg1_val)                 \
  INTERNAL_TRACE_EVENT_ADD_WITH_ID_TID_AND_TIMESTAMP(                         \
      TRACE_EVENT_PHASE_NESTABLE_ASYNC_BEGIN, category_group,                 \
      ::perfetto::DynamicString{name}, id, TRACE_EVENT_API_CURRENT_THREAD_ID, \
      timestamp, TRACE_EVENT_FLAG_NONE, ::perfetto::DynamicString{arg1_name}, \
      ::perfetto::internal::PossiblyNull(arg1_val))
#define TRACE_EVENT_COPY_NESTABLE_ASYNC_END_WITH_TIMESTAMP0(                  \
    category_group, name, id, timestamp)                                      \
  INTERNAL_TRACE_EVENT_ADD_WITH_ID_TID_AND_TIMESTAMP(                         \
      TRACE_EVENT_PHASE_NESTABLE_ASYNC_END, category_group,                   \
      ::perfetto::DynamicString{name}, id, TRACE_EVENT_API_CURRENT_THREAD_ID, \
      timestamp, TRACE_EVENT_FLAG_NONE)
#define TRACE_EVENT_COPY_NESTABLE_ASYNC_END_WITH_TIMESTAMP2(               \
    category_group, name, id, timestamp, arg1_name, arg1_val, arg2_name,   \
    arg2_val)                                                              \
  INTERNAL_TRACE_EVENT_ADD_WITH_ID_TID_AND_TIMESTAMP(                      \
      TRACE_EVENT_PHASE_NESTABLE_ASYNC_END, category_group, name, id,      \
      TRACE_EVENT_API_CURRENT_THREAD_ID, timestamp, TRACE_EVENT_FLAG_NONE, \
      ::perfetto::DynamicString{arg1_name},                                \
      ::perfetto::internal::PossiblyNull(arg1_val),                        \
      ::perfetto::DynamicString{arg2_name},                                \
      ::perfetto::internal::PossiblyNull(arg2_val))

// Metadata events.
#define TRACE_EVENT_METADATA1(category_group, name, arg1_name, arg1_val) \
  INTERNAL_TRACE_EVENT_METADATA_ADD(                                     \
      category_group, name, arg1_name,                                   \
      ::perfetto::internal::PossiblyNull(arg1_val))

// Clock sync events.
#define TRACE_EVENT_CLOCK_SYNC_RECEIVER(sync_id)                           \
  INTERNAL_TRACE_EVENT_ADD(TRACE_EVENT_PHASE_CLOCK_SYNC, "__metadata",     \
                           "clock_sync", TRACE_EVENT_FLAG_NONE, "sync_id", \
                           sync_id)
#define TRACE_EVENT_CLOCK_SYNC_ISSUER(sync_id, issue_ts, issue_end_ts)        \
  INTERNAL_TRACE_EVENT_ADD_WITH_TIMESTAMP(                                    \
      TRACE_EVENT_PHASE_CLOCK_SYNC, "__metadata", "clock_sync", issue_end_ts, \
      TRACE_EVENT_FLAG_NONE, "sync_id", sync_id, "issue_ts", issue_ts)

// Object events.
#define TRACE_EVENT_OBJECT_CREATED_WITH_ID(category_group, name, id) \
  INTERNAL_TRACE_EVENT_ADD_WITH_ID(TRACE_EVENT_PHASE_CREATE_OBJECT,  \
                                   category_group, name, id,         \
                                   TRACE_EVENT_FLAG_NONE)

#define TRACE_EVENT_OBJECT_SNAPSHOT_WITH_ID(category_group, name, id, \
                                            snapshot)                 \
  INTERNAL_TRACE_EVENT_ADD_WITH_ID(                                   \
      TRACE_EVENT_PHASE_SNAPSHOT_OBJECT, category_group, name, id,    \
      TRACE_EVENT_FLAG_NONE, "snapshot", snapshot)

#define TRACE_EVENT_OBJECT_SNAPSHOT_WITH_ID_AND_TIMESTAMP(                 \
    category_group, name, id, timestamp, snapshot)                         \
  INTERNAL_TRACE_EVENT_ADD_WITH_ID_TID_AND_TIMESTAMP(                      \
      TRACE_EVENT_PHASE_SNAPSHOT_OBJECT, category_group, name, id,         \
      TRACE_EVENT_API_CURRENT_THREAD_ID, timestamp, TRACE_EVENT_FLAG_NONE, \
      "snapshot", snapshot)

#define TRACE_EVENT_OBJECT_DELETED_WITH_ID(category_group, name, id) \
  INTERNAL_TRACE_EVENT_ADD_WITH_ID(TRACE_EVENT_PHASE_DELETE_OBJECT,  \
                                   category_group, name, id,         \
                                   TRACE_EVENT_FLAG_NONE)

// TODO(skyostil): Implement binary-efficient trace events.
#define TRACE_EVENT_BINARY_EFFICIENT0 TRACE_EVENT0
#define TRACE_EVENT_BINARY_EFFICIENT1 TRACE_EVENT1
#define TRACE_EVENT_BINARY_EFFICIENT2 TRACE_EVENT2

// Macro to efficiently determine if a given category group is enabled.
#define TRACE_EVENT_CATEGORY_GROUP_ENABLED(category, ret) \
  do {                                                    \
    *ret = TRACE_EVENT_CATEGORY_ENABLED(category);        \
  } while (0)

// Macro to efficiently determine, through polling, if a new trace has begun.
#define TRACE_EVENT_IS_NEW_TRACE(ret)                                \
  do {                                                               \
    static int PERFETTO_UID(prev) = -1;                              \
    int PERFETTO_UID(curr) =                                         \
        ::perfetto::internal::TrackEventInternal::GetSessionCount(); \
    if (PERFETTO_TRACK_EVENT_NAMESPACE::TrackEvent::IsEnabled() &&   \
        (PERFETTO_UID(prev) != PERFETTO_UID(curr))) {                \
      *(ret) = true;                                                 \
      PERFETTO_UID(prev) = PERFETTO_UID(curr);                       \
    } else {                                                         \
      *(ret) = false;                                                \
    }                                                                \
  } while (0)

// ----------------------------------------------------------------------------
// Legacy tracing API (adapted from trace_event.h).
// ----------------------------------------------------------------------------

// We can implement the following subset of the legacy tracing API without
// involvement from the embedder. APIs such as TRACE_EVENT_API_ADD_TRACE_EVENT
// are still up to the embedder to define.

#define TRACE_STR_COPY(str)                 \
  ::perfetto::DynamicString {               \
    ::perfetto::internal::PossiblyNull(str) \
  }

#define TRACE_ID_WITH_SCOPE(scope, ...) \
  ::perfetto::internal::LegacyTraceId::WithScope(scope, ##__VA_ARGS__)

// Use this for ids that are unique across processes. This allows different
// processes to use the same id to refer to the same event.
#define TRACE_ID_GLOBAL(id) ::perfetto::internal::LegacyTraceId::GlobalId(id)

// Use this for ids that are unique within a single process. This allows
// different processes to use the same id to refer to different events.
#define TRACE_ID_LOCAL(id) ::perfetto::internal::LegacyTraceId::LocalId(id)

// Returns a pointer to a uint8_t which indicates whether tracing is enabled for
// the given category or not. A zero value means tracing is disabled and
// non-zero indicates at least one tracing session for this category is active.
// Note that callers should not make any assumptions at what each bit represents
// in the status byte. Does not support dynamic categories.
#define TRACE_EVENT_API_GET_CATEGORY_GROUP_ENABLED(category)              \
  reinterpret_cast<const uint8_t*>(                                       \
      [&] {                                                               \
        static_assert(                                                    \
            !std::is_same<::perfetto::DynamicCategory,                    \
                          decltype(category)>::value,                     \
            "Enabled flag pointers are not supported for dynamic trace "  \
            "categories.");                                               \
      },                                                                  \
      PERFETTO_TRACK_EVENT_NAMESPACE::internal::kCategoryRegistry         \
          .GetCategoryState(                                              \
              PERFETTO_TRACK_EVENT_NAMESPACE::internal::kCategoryRegistry \
                  .Find(category, /*is_dynamic=*/false)))

// Given a pointer returned by TRACE_EVENT_API_GET_CATEGORY_GROUP_ENABLED,
// yields a pointer to the name of the corresponding category group.
#define TRACE_EVENT_API_GET_CATEGORY_GROUP_NAME(category_enabled_ptr)     \
  PERFETTO_TRACK_EVENT_NAMESPACE::internal::kCategoryRegistry             \
      .GetCategory(                                                       \
          category_enabled_ptr -                                          \
          reinterpret_cast<const uint8_t*>(                               \
              PERFETTO_TRACK_EVENT_NAMESPACE::internal::kCategoryRegistry \
                  .GetCategoryState(0u)))                                 \
      ->name

#endif  // PERFETTO_ENABLE_LEGACY_TRACE_EVENTS

#endif  // INCLUDE_PERFETTO_TRACING_TRACK_EVENT_LEGACY_H_
