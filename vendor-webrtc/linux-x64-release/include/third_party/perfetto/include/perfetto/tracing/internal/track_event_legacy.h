/*
 * Copyright (C) 2023 The Android Open Source Project
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

#ifndef INCLUDE_PERFETTO_TRACING_INTERNAL_TRACK_EVENT_LEGACY_H_
#define INCLUDE_PERFETTO_TRACING_INTERNAL_TRACK_EVENT_LEGACY_H_

#include "perfetto/base/build_config.h"
#include "perfetto/tracing/event_context.h"
#include "perfetto/tracing/track.h"
#include "protos/perfetto/trace/track_event/track_event.pbzero.h"

#ifndef PERFETTO_ENABLE_LEGACY_TRACE_EVENTS
#define PERFETTO_ENABLE_LEGACY_TRACE_EVENTS 0
#endif

// ----------------------------------------------------------------------------
// Constants.
// ----------------------------------------------------------------------------

namespace perfetto {
namespace legacy {

enum TraceEventFlag {
  kTraceEventFlagNone = 0,
  kTraceEventFlagCopy = 1u << 0,
  kTraceEventFlagHasId = 1u << 1,
  kTraceEventFlagScopeOffset = 1u << 2,
  kTraceEventFlagScopeExtra = 1u << 3,
  kTraceEventFlagExplicitTimestamp = 1u << 4,
  kTraceEventFlagAsyncTTS = 1u << 5,
  kTraceEventFlagBindToEnclosing = 1u << 6,
  kTraceEventFlagFlowIn = 1u << 7,
  kTraceEventFlagFlowOut = 1u << 8,
  kTraceEventFlagHasContextId = 1u << 9,
  kTraceEventFlagHasProcessId = 1u << 10,
  kTraceEventFlagHasLocalId = 1u << 11,
  kTraceEventFlagHasGlobalId = 1u << 12,
  // TODO(eseckler): Remove once we have native support for typed proto events
  // in TRACE_EVENT macros.
  kTraceEventFlagTypedProtoArgs = 1u << 15,
  kTraceEventFlagJavaStringLiterals = 1u << 16,
};

enum PerfettoLegacyCurrentThreadId { kCurrentThreadId };

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
// The following constants are defined in the global namespace, since they were
// originally implemented as macros.

// Event phases.
static constexpr char TRACE_EVENT_PHASE_BEGIN = 'B';
static constexpr char TRACE_EVENT_PHASE_END = 'E';
static constexpr char TRACE_EVENT_PHASE_COMPLETE = 'X';
static constexpr char TRACE_EVENT_PHASE_INSTANT = 'I';
static constexpr char TRACE_EVENT_PHASE_ASYNC_BEGIN = 'S';
static constexpr char TRACE_EVENT_PHASE_ASYNC_STEP_INTO = 'T';
static constexpr char TRACE_EVENT_PHASE_ASYNC_STEP_PAST = 'p';
static constexpr char TRACE_EVENT_PHASE_ASYNC_END = 'F';
static constexpr char TRACE_EVENT_PHASE_NESTABLE_ASYNC_BEGIN = 'b';
static constexpr char TRACE_EVENT_PHASE_NESTABLE_ASYNC_END = 'e';
static constexpr char TRACE_EVENT_PHASE_NESTABLE_ASYNC_INSTANT = 'n';
static constexpr char TRACE_EVENT_PHASE_FLOW_BEGIN = 's';
static constexpr char TRACE_EVENT_PHASE_FLOW_STEP = 't';
static constexpr char TRACE_EVENT_PHASE_FLOW_END = 'f';
static constexpr char TRACE_EVENT_PHASE_METADATA = 'M';
static constexpr char TRACE_EVENT_PHASE_COUNTER = 'C';
static constexpr char TRACE_EVENT_PHASE_SAMPLE = 'P';
static constexpr char TRACE_EVENT_PHASE_CREATE_OBJECT = 'N';
static constexpr char TRACE_EVENT_PHASE_SNAPSHOT_OBJECT = 'O';
static constexpr char TRACE_EVENT_PHASE_DELETE_OBJECT = 'D';
static constexpr char TRACE_EVENT_PHASE_MEMORY_DUMP = 'v';
static constexpr char TRACE_EVENT_PHASE_MARK = 'R';
static constexpr char TRACE_EVENT_PHASE_CLOCK_SYNC = 'c';

// Flags for changing the behavior of TRACE_EVENT_API_ADD_TRACE_EVENT.
static constexpr uint32_t TRACE_EVENT_FLAG_NONE =
    perfetto::legacy::kTraceEventFlagNone;
static constexpr uint32_t TRACE_EVENT_FLAG_COPY =
    perfetto::legacy::kTraceEventFlagCopy;
static constexpr uint32_t TRACE_EVENT_FLAG_HAS_ID =
    perfetto::legacy::kTraceEventFlagHasId;
static constexpr uint32_t TRACE_EVENT_FLAG_SCOPE_OFFSET =
    perfetto::legacy::kTraceEventFlagScopeOffset;
static constexpr uint32_t TRACE_EVENT_FLAG_SCOPE_EXTRA =
    perfetto::legacy::kTraceEventFlagScopeExtra;
static constexpr uint32_t TRACE_EVENT_FLAG_EXPLICIT_TIMESTAMP =
    perfetto::legacy::kTraceEventFlagExplicitTimestamp;
static constexpr uint32_t TRACE_EVENT_FLAG_ASYNC_TTS =
    perfetto::legacy::kTraceEventFlagAsyncTTS;
static constexpr uint32_t TRACE_EVENT_FLAG_BIND_TO_ENCLOSING =
    perfetto::legacy::kTraceEventFlagBindToEnclosing;
static constexpr uint32_t TRACE_EVENT_FLAG_FLOW_IN =
    perfetto::legacy::kTraceEventFlagFlowIn;
static constexpr uint32_t TRACE_EVENT_FLAG_FLOW_OUT =
    perfetto::legacy::kTraceEventFlagFlowOut;
static constexpr uint32_t TRACE_EVENT_FLAG_HAS_CONTEXT_ID =
    perfetto::legacy::kTraceEventFlagHasContextId;
static constexpr uint32_t TRACE_EVENT_FLAG_HAS_PROCESS_ID =
    perfetto::legacy::kTraceEventFlagHasProcessId;
static constexpr uint32_t TRACE_EVENT_FLAG_HAS_LOCAL_ID =
    perfetto::legacy::kTraceEventFlagHasLocalId;
static constexpr uint32_t TRACE_EVENT_FLAG_HAS_GLOBAL_ID =
    perfetto::legacy::kTraceEventFlagHasGlobalId;
static constexpr uint32_t TRACE_EVENT_FLAG_TYPED_PROTO_ARGS =
    perfetto::legacy::kTraceEventFlagTypedProtoArgs;
static constexpr uint32_t TRACE_EVENT_FLAG_JAVA_STRING_LITERALS =
    perfetto::legacy::kTraceEventFlagJavaStringLiterals;

static constexpr uint32_t TRACE_EVENT_FLAG_SCOPE_MASK =
    TRACE_EVENT_FLAG_SCOPE_OFFSET | TRACE_EVENT_FLAG_SCOPE_EXTRA;

// Type values for identifying types in the TraceValue union.
static constexpr uint8_t TRACE_VALUE_TYPE_BOOL = 1;
static constexpr uint8_t TRACE_VALUE_TYPE_UINT = 2;
static constexpr uint8_t TRACE_VALUE_TYPE_INT = 3;
static constexpr uint8_t TRACE_VALUE_TYPE_DOUBLE = 4;
static constexpr uint8_t TRACE_VALUE_TYPE_POINTER = 5;
static constexpr uint8_t TRACE_VALUE_TYPE_STRING = 6;
static constexpr uint8_t TRACE_VALUE_TYPE_COPY_STRING = 7;
static constexpr uint8_t TRACE_VALUE_TYPE_CONVERTABLE = 8;
static constexpr uint8_t TRACE_VALUE_TYPE_PROTO = 9;

// Enum reflecting the scope of an INSTANT event. Must fit within
// TRACE_EVENT_FLAG_SCOPE_MASK.
static constexpr uint8_t TRACE_EVENT_SCOPE_GLOBAL = 0u << 2;
static constexpr uint8_t TRACE_EVENT_SCOPE_PROCESS = 1u << 2;
static constexpr uint8_t TRACE_EVENT_SCOPE_THREAD = 2u << 2;

static constexpr char TRACE_EVENT_SCOPE_NAME_GLOBAL = 'g';
static constexpr char TRACE_EVENT_SCOPE_NAME_PROCESS = 'p';
static constexpr char TRACE_EVENT_SCOPE_NAME_THREAD = 't';

#define TRACE_EVENT_API_CURRENT_THREAD_ID ::perfetto::legacy::kCurrentThreadId

#endif  // PERFETTO_ENABLE_LEGACY_TRACE_EVENTS

namespace perfetto {
namespace internal {

// LegacyTraceId encapsulates an ID that can either be an integer or pointer.
class PERFETTO_EXPORT_COMPONENT LegacyTraceId {
 public:
  // Can be combined with WithScope.
  class LocalId {
   public:
    explicit LocalId(const void* raw_id)
        : raw_id_(static_cast<uint64_t>(reinterpret_cast<uintptr_t>(raw_id))) {}
    explicit LocalId(uint64_t raw_id) : raw_id_(raw_id) {}
    uint64_t raw_id() const { return raw_id_; }

   private:
    uint64_t raw_id_;
  };

  // Can be combined with WithScope.
  class GlobalId {
   public:
    explicit GlobalId(uint64_t raw_id) : raw_id_(raw_id) {}
    uint64_t raw_id() const { return raw_id_; }

   private:
    uint64_t raw_id_;
  };

  class WithScope {
   public:
    WithScope(const char* scope, uint64_t raw_id)
        : scope_(scope), raw_id_(raw_id) {}
    WithScope(const char* scope, LocalId local_id)
        : scope_(scope), raw_id_(local_id.raw_id()) {
      id_flags_ = legacy::kTraceEventFlagHasLocalId;
    }
    WithScope(const char* scope, GlobalId global_id)
        : scope_(scope), raw_id_(global_id.raw_id()) {
      id_flags_ = legacy::kTraceEventFlagHasGlobalId;
    }
    WithScope(const char* scope, uint64_t prefix, uint64_t raw_id)
        : scope_(scope), has_prefix_(true), prefix_(prefix), raw_id_(raw_id) {}
    WithScope(const char* scope, uint64_t prefix, GlobalId global_id)
        : scope_(scope),
          has_prefix_(true),
          prefix_(prefix),
          raw_id_(global_id.raw_id()) {
      id_flags_ = legacy::kTraceEventFlagHasGlobalId;
    }
    uint64_t raw_id() const { return raw_id_; }
    const char* scope() const { return scope_; }
    bool has_prefix() const { return has_prefix_; }
    uint64_t prefix() const { return prefix_; }
    uint32_t id_flags() const { return id_flags_; }

   private:
    const char* scope_ = nullptr;
    bool has_prefix_ = false;
    uint64_t prefix_;
    uint64_t raw_id_;
    uint32_t id_flags_ = legacy::kTraceEventFlagHasId;
  };

  explicit LegacyTraceId(const void* raw_id)
      : raw_id_(static_cast<uint64_t>(reinterpret_cast<uintptr_t>(raw_id))) {
    id_flags_ = legacy::kTraceEventFlagHasLocalId;
  }
  explicit LegacyTraceId(uint64_t raw_id) : raw_id_(raw_id) {}
  explicit LegacyTraceId(uint32_t raw_id) : raw_id_(raw_id) {}
  explicit LegacyTraceId(uint16_t raw_id) : raw_id_(raw_id) {}
  explicit LegacyTraceId(uint8_t raw_id) : raw_id_(raw_id) {}
  explicit LegacyTraceId(int64_t raw_id)
      : raw_id_(static_cast<uint64_t>(raw_id)) {}
  explicit LegacyTraceId(int32_t raw_id)
      : raw_id_(static_cast<uint64_t>(raw_id)) {}
  explicit LegacyTraceId(int16_t raw_id)
      : raw_id_(static_cast<uint64_t>(raw_id)) {}
  explicit LegacyTraceId(int8_t raw_id)
      : raw_id_(static_cast<uint64_t>(raw_id)) {}
// Different platforms disagree on which integer types are same and which
// are different. E.g. on Mac size_t is considered a different type from
// uint64_t even though it has the same size and signedness.
// Below we add overloads for those types that are known to cause ambiguity.
#if PERFETTO_BUILDFLAG(PERFETTO_OS_APPLE)
  explicit LegacyTraceId(size_t raw_id) : raw_id_(raw_id) {}
  explicit LegacyTraceId(intptr_t raw_id)
      : raw_id_(static_cast<uint64_t>(raw_id)) {}
#elif PERFETTO_BUILDFLAG(PERFETTO_OS_WIN)
  explicit LegacyTraceId(unsigned long raw_id) : raw_id_(raw_id) {}
#endif
  explicit LegacyTraceId(LocalId raw_id) : raw_id_(raw_id.raw_id()) {
    id_flags_ = legacy::kTraceEventFlagHasLocalId;
  }
  explicit LegacyTraceId(GlobalId raw_id) : raw_id_(raw_id.raw_id()) {
    id_flags_ = legacy::kTraceEventFlagHasGlobalId;
  }
  explicit LegacyTraceId(WithScope scoped_id)
      : scope_(scoped_id.scope()),
        has_prefix_(scoped_id.has_prefix()),
        prefix_(scoped_id.prefix()),
        raw_id_(scoped_id.raw_id()),
        id_flags_(scoped_id.id_flags()) {}

  uint64_t raw_id() const { return raw_id_; }
  const char* scope() const { return scope_; }
  bool has_prefix() const { return has_prefix_; }
  uint64_t prefix() const { return prefix_; }
  uint32_t id_flags() const { return id_flags_; }

  void Write(protos::pbzero::TrackEvent::LegacyEvent*,
             uint32_t event_flags) const;

 private:
  const char* scope_ = nullptr;
  bool has_prefix_ = false;
  uint64_t prefix_;
  uint64_t raw_id_;
  uint32_t id_flags_ = legacy::kTraceEventFlagHasId;
};

#if PERFETTO_ENABLE_LEGACY_TRACE_EVENTS
template <typename T>
bool IsEqual(T x, T y) {
  return x == y;
}

template <typename T, typename U>
bool IsEqual(T, U) {
  return false;
}

class PERFETTO_EXPORT_COMPONENT TrackEventLegacy {
 public:
  static constexpr protos::pbzero::TrackEvent::Type PhaseToType(char phase) {
    // clang-format off
    return (phase == TRACE_EVENT_PHASE_BEGIN) ?
               protos::pbzero::TrackEvent::TYPE_SLICE_BEGIN :
           (phase == TRACE_EVENT_PHASE_END) ?
               protos::pbzero::TrackEvent::TYPE_SLICE_END :
           (phase == TRACE_EVENT_PHASE_INSTANT) ?
               protos::pbzero::TrackEvent::TYPE_INSTANT :
           protos::pbzero::TrackEvent::TYPE_UNSPECIFIED;
    // clang-format on
  }

  // Reduce binary size overhead by outlining most of the code for writing a
  // legacy trace event.
  template <typename... Args>
  static void WriteLegacyEvent(EventContext ctx,
                               char phase,
                               uint32_t flags,
                               Args&&... args) PERFETTO_NO_INLINE {
    PERFETTO_DCHECK(!(flags & TRACE_EVENT_FLAG_HAS_PROCESS_ID));
    AddDebugAnnotations(&ctx, std::forward<Args>(args)...);
    if (NeedLegacyFlags(phase, flags)) {
      auto legacy_event = ctx.event()->set_legacy_event();
      SetLegacyFlags(legacy_event, phase, flags);
    }
  }

  template <typename ThreadIdType, typename... Args>
  static void WriteLegacyEventWithIdAndTid(EventContext ctx,
                                           char phase,
                                           uint32_t flags,
                                           const LegacyTraceId& id,
                                           const ThreadIdType& thread_id,
                                           Args&&... args) PERFETTO_NO_INLINE {
    //
    // Overrides to consider:
    //
    // 1. If we have an id, we need to write {unscoped,local,global}_id and/or
    //    bind_id.
    // 2. If we have a thread id, we need to write track_uuid() or
    //    {pid,tid}_override if the id represents another process.  The
    //    conversion from |thread_id| happens in embedder code since the type is
    //    embedder-specified.
    // 3. If we have a timestamp, we need to write a different timestamp in the
    //    trace packet itself and make sure TrackEvent won't write one
    //    internally. This is already done at the call site.
    //
    PERFETTO_DCHECK(PhaseToType(phase) ==
                        protos::pbzero::TrackEvent::TYPE_UNSPECIFIED ||
                    !(flags & TRACE_EVENT_FLAG_HAS_PROCESS_ID));
    flags |= id.id_flags();
    AddDebugAnnotations(&ctx, std::forward<Args>(args)...);
    if (NeedLegacyFlags(phase, flags)) {
      auto legacy_event = ctx.event()->set_legacy_event();
      SetLegacyFlags(legacy_event, phase, flags);
      if (id.id_flags())
        id.Write(legacy_event, flags);
      if (flags & TRACE_EVENT_FLAG_HAS_PROCESS_ID) {
        // The thread identifier actually represents a process id. Let's set an
        // override for it.
        int32_t pid_override =
            static_cast<int32_t>(legacy::ConvertThreadId(thread_id).tid);
        legacy_event->set_pid_override(pid_override);
        legacy_event->set_tid_override(-1);
      } else {
        // Only synchronous phases are supported for other threads. These phases
        // are supported in TrackEvent types and receive a track_uuid
        // association via TrackEventDataSource::TraceForCategoryImpl().
        PERFETTO_DCHECK(PhaseToType(phase) !=
                            protos::pbzero::TrackEvent::TYPE_UNSPECIFIED ||
                        IsEqual(thread_id, TRACE_EVENT_API_CURRENT_THREAD_ID) ||
                        legacy::ConvertThreadId(thread_id).tid ==
                            ThreadTrack::Current().tid);
      }
    }
  }

  // No arguments.
  static void AddDebugAnnotations(EventContext*) {}

  // N number of debug arguments.
  template <typename ArgNameType, typename ArgType, typename... OtherArgs>
  static void AddDebugAnnotations(EventContext* ctx,
                                  ArgNameType&& arg_name,
                                  ArgType&& arg_value,
                                  OtherArgs&&... more_args) {
    TrackEventInternal::AddDebugAnnotation(ctx,
                                           std::forward<ArgNameType>(arg_name),
                                           std::forward<ArgType>(arg_value));
    AddDebugAnnotations(ctx, std::forward<OtherArgs>(more_args)...);
  }

 private:
  static bool NeedLegacyFlags(char phase, uint32_t flags) {
    if (PhaseToType(phase) == protos::pbzero::TrackEvent::TYPE_UNSPECIFIED)
      return true;
    // TODO(skyostil): Implement/deprecate:
    // - TRACE_EVENT_FLAG_EXPLICIT_TIMESTAMP
    // - TRACE_EVENT_FLAG_HAS_CONTEXT_ID
    // - TRACE_EVENT_FLAG_TYPED_PROTO_ARGS
    // - TRACE_EVENT_FLAG_JAVA_STRING_LITERALS
    return flags &
           (TRACE_EVENT_FLAG_HAS_ID | TRACE_EVENT_FLAG_HAS_LOCAL_ID |
            TRACE_EVENT_FLAG_HAS_GLOBAL_ID | TRACE_EVENT_FLAG_ASYNC_TTS |
            TRACE_EVENT_FLAG_BIND_TO_ENCLOSING | TRACE_EVENT_FLAG_FLOW_IN |
            TRACE_EVENT_FLAG_FLOW_OUT | TRACE_EVENT_FLAG_HAS_PROCESS_ID);
  }

  static void SetLegacyFlags(
      protos::pbzero::TrackEvent::LegacyEvent* legacy_event,
      char phase,
      uint32_t flags) {
    if (PhaseToType(phase) == protos::pbzero::TrackEvent::TYPE_UNSPECIFIED)
      legacy_event->set_phase(phase);
    if (flags & TRACE_EVENT_FLAG_ASYNC_TTS)
      legacy_event->set_use_async_tts(true);
    if (flags & TRACE_EVENT_FLAG_BIND_TO_ENCLOSING)
      legacy_event->set_bind_to_enclosing(true);

    const auto kFlowIn = TRACE_EVENT_FLAG_FLOW_IN;
    const auto kFlowOut = TRACE_EVENT_FLAG_FLOW_OUT;
    const auto kFlowInOut = kFlowIn | kFlowOut;
    if ((flags & kFlowInOut) == kFlowInOut) {
      legacy_event->set_flow_direction(
          protos::pbzero::TrackEvent::LegacyEvent::FLOW_INOUT);
    } else if (flags & kFlowIn) {
      legacy_event->set_flow_direction(
          protos::pbzero::TrackEvent::LegacyEvent::FLOW_IN);
    } else if (flags & kFlowOut) {
      legacy_event->set_flow_direction(
          protos::pbzero::TrackEvent::LegacyEvent::FLOW_OUT);
    }
  }
};
#endif  // PERFETTO_ENABLE_LEGACY_TRACE_EVENTS

// Legacy macros allow argument values to be nullptr and convert them to the
// "NULL" string. The following function helps mimic this behavior: it forwards
// all types of arguments apart from a nullptr string as is, and in case of a
// nullptr returns "NULL".
template <typename T>
inline T PossiblyNull(T&& value) {
  return std::forward<T>(value);
}

inline const char* PossiblyNull(const char* name) {
  return name ? name : "NULL";
}

inline const char* PossiblyNull(char* name) {
  return name ? name : "NULL";
}

}  // namespace internal
}  // namespace perfetto

#endif  // INCLUDE_PERFETTO_TRACING_INTERNAL_TRACK_EVENT_LEGACY_H_
