// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TRACE_EVENT_TRACE_EVENT_STUB_H_
#define BASE_TRACE_EVENT_TRACE_EVENT_STUB_H_

#include <stddef.h>

#include <cstdint>
#include <memory>
#include <string>
#include <string_view>

#include "base/base_export.h"
#include "base/trace_event/common/trace_event_common.h"
#include "base/trace_event/memory_allocator_dump_guid.h"
#include "base/values.h"

#define TRACE_STR_COPY(str) str
#define TRACE_ID_WITH_SCOPE(scope, ...) 0
#define TRACE_ID_GLOBAL(id) 0
#define TRACE_ID_LOCAL(id) 0

namespace trace_event_internal {

const unsigned long long kNoId = 0;

template <typename... Args>
void Ignore(Args&&... args) {}

struct IgnoredValue {
  template <typename... Args>
  IgnoredValue(Args&&... args) {}
};

}  // namespace trace_event_internal

#define INTERNAL_TRACE_IGNORE(...) \
  (false ? trace_event_internal::Ignore(__VA_ARGS__) : (void)0)

// Defined in application_state_proto_android.h
#define TRACE_APPLICATION_STATE(...) INTERNAL_TRACE_IGNORE(__VA_ARGS__)

#define TRACE_HEAP_PROFILER_API_SCOPED_TASK_EXECUTION \
  trace_event_internal::IgnoredValue

#define TRACE_ID_MANGLE(val) (val)

#define TRACE_EVENT_API_CURRENT_THREAD_ID 0

// Legacy trace macros
#define TRACE_EVENT0(...) INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_WITH_FLOW0(...) INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT1(...) INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_WITH_FLOW1(...) INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT2(...) INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_WITH_FLOW2(...) INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_INSTANT0(...) INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_INSTANT1(...) INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_INSTANT2(...) INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_COPY_INSTANT0(...) INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_COPY_INSTANT1(...) INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_COPY_INSTANT2(...) INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_INSTANT_WITH_FLAGS0(...) INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_INSTANT_WITH_FLAGS1(...) INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_INSTANT_WITH_TIMESTAMP0(...) \
  INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_INSTANT_WITH_TIMESTAMP1(...) \
  INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_BEGIN0(...) INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_BEGIN1(...) INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_BEGIN2(...) INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_BEGIN_WITH_FLAGS0(...) INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_BEGIN_WITH_FLAGS1(...) INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_COPY_BEGIN2(...) INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_BEGIN_WITH_ID_TID_AND_TIMESTAMP0(...) \
  INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_COPY_BEGIN_WITH_ID_TID_AND_TIMESTAMP0(...) \
  INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_COPY_BEGIN_WITH_ID_TID_AND_TIMESTAMP1(...) \
  INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_COPY_BEGIN_WITH_ID_TID_AND_TIMESTAMP2(...) \
  INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_END0(...) INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_END1(...) INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_END2(...) INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_END_WITH_FLAGS0(...) INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_END_WITH_FLAGS1(...) INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_COPY_END2(...) INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_MARK_WITH_TIMESTAMP0(...) INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_MARK_WITH_TIMESTAMP1(...) INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_MARK_WITH_TIMESTAMP2(...) INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_COPY_MARK(...) INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_COPY_MARK1(...) INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_COPY_MARK_WITH_TIMESTAMP(...) \
  INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_END_WITH_ID_TID_AND_TIMESTAMP0(...) \
  INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_COPY_END_WITH_ID_TID_AND_TIMESTAMP0(...) \
  INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_COPY_END_WITH_ID_TID_AND_TIMESTAMP1(...) \
  INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_COPY_END_WITH_ID_TID_AND_TIMESTAMP2(...) \
  INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_COUNTER1(...) INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_COUNTER_WITH_FLAG1(...) INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_COPY_COUNTER1(...) INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_COUNTER2(...) INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_COPY_COUNTER2(...) INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_COUNTER_WITH_TIMESTAMP1(...) INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_COUNTER_WITH_TIMESTAMP2(...) INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_COUNTER_ID1(...) INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_COPY_COUNTER_ID1(...) INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_COUNTER_ID2(...) INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_COPY_COUNTER_ID2(...) INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_SAMPLE_WITH_ID1(...) INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_ASYNC_BEGIN0(...) INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_ASYNC_BEGIN1(...) INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_ASYNC_BEGIN2(...) INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_COPY_ASYNC_BEGIN0(...) INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_COPY_ASYNC_BEGIN1(...) INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_COPY_ASYNC_BEGIN2(...) INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_ASYNC_BEGIN_WITH_TIMESTAMP0(...) \
  INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_ASYNC_BEGIN_WITH_TIMESTAMP1(...) \
  INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_ASYNC_BEGIN_WITH_TIMESTAMP2(...) \
  INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_COPY_ASYNC_BEGIN_WITH_TIMESTAMP0(...) \
  INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_ASYNC_STEP_INTO0(...) INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_ASYNC_STEP_INTO1(...) INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_ASYNC_STEP_INTO_WITH_TIMESTAMP0(...) \
  INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_ASYNC_STEP_PAST0(...) INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_ASYNC_STEP_PAST1(...) INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_ASYNC_END0(...) INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_ASYNC_END1(...) INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_ASYNC_END2(...) INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_COPY_ASYNC_END0(...) INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_COPY_ASYNC_END1(...) INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_COPY_ASYNC_END2(...) INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_ASYNC_END_WITH_TIMESTAMP0(...) \
  INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_ASYNC_END_WITH_TIMESTAMP1(...) \
  INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_ASYNC_END_WITH_TIMESTAMP2(...) \
  INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_COPY_ASYNC_END_WITH_TIMESTAMP0(...) \
  INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_NESTABLE_ASYNC_BEGIN0(...) \
  INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_NESTABLE_ASYNC_BEGIN1(...) \
  INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_NESTABLE_ASYNC_BEGIN2(...) \
  INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_NESTABLE_ASYNC_BEGIN_WITH_FLAGS0(...) \
  INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_NESTABLE_ASYNC_END0(...) INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_NESTABLE_ASYNC_END1(...) INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_NESTABLE_ASYNC_END2(...) INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_NESTABLE_ASYNC_END_WITH_FLAGS0(...) \
  INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_NESTABLE_ASYNC_INSTANT0(...) \
  INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_NESTABLE_ASYNC_INSTANT1(...) \
  INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_NESTABLE_ASYNC_INSTANT2(...) \
  INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_COPY_NESTABLE_ASYNC_BEGIN_WITH_TTS2(...) \
  INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_COPY_NESTABLE_ASYNC_END_WITH_TTS2(...) \
  INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_NESTABLE_ASYNC_BEGIN_WITH_TIMESTAMP0(...) \
  INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_NESTABLE_ASYNC_BEGIN_WITH_TIMESTAMP1(...) \
  INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_NESTABLE_ASYNC_BEGIN_WITH_TIMESTAMP_AND_FLAGS0(...) \
  INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_NESTABLE_ASYNC_END_WITH_TIMESTAMP0(...) \
  INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_NESTABLE_ASYNC_END_WITH_TIMESTAMP1(...) \
  INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_NESTABLE_ASYNC_END_WITH_TIMESTAMP2(...) \
  INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_NESTABLE_ASYNC_END_WITH_TIMESTAMP_AND_FLAGS0(...) \
  INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_NESTABLE_ASYNC_INSTANT_WITH_TIMESTAMP0(...) \
  INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_COPY_NESTABLE_ASYNC_BEGIN0(...) \
  INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_COPY_NESTABLE_ASYNC_BEGIN1(...) \
  INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_COPY_NESTABLE_ASYNC_BEGIN2(...) \
  INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_COPY_NESTABLE_ASYNC_END0(...) \
  INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_COPY_NESTABLE_ASYNC_BEGIN_WITH_TIMESTAMP0(...) \
  INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_COPY_NESTABLE_ASYNC_BEGIN_WITH_TIMESTAMP1(...) \
  INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_COPY_NESTABLE_ASYNC_END_WITH_TIMESTAMP0(...) \
  INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_COPY_NESTABLE_ASYNC_END1(...) \
  INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_METADATA1(...) INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_CLOCK_SYNC_RECEIVER(...) INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_CLOCK_SYNC_ISSUER(...) INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_OBJECT_CREATED_WITH_ID(...) \
  INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_OBJECT_SNAPSHOT_WITH_ID(...) \
  INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_OBJECT_SNAPSHOT_WITH_ID_AND_TIMESTAMP(...) \
  INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_OBJECT_DELETED_WITH_ID(...) \
  INTERNAL_TRACE_IGNORE(__VA_ARGS__)
#define TRACE_EVENT_CATEGORY_GROUP_ENABLED(category_group, ret) \
  do {                                                          \
    *ret = false;                                               \
  } while (0)
#define TRACE_EVENT_IS_NEW_TRACE(ret) \
  do {                                \
    *ret = false;                     \
  } while (0)

#define TRACE_EVENT_PHASE_BEGIN ('B')
#define TRACE_EVENT_PHASE_END ('E')
#define TRACE_EVENT_PHASE_COMPLETE ('X')
#define TRACE_EVENT_PHASE_INSTANT ('I')
#define TRACE_EVENT_PHASE_ASYNC_BEGIN ('S')
#define TRACE_EVENT_PHASE_ASYNC_STEP_INTO ('T')
#define TRACE_EVENT_PHASE_ASYNC_STEP_PAST ('p')
#define TRACE_EVENT_PHASE_ASYNC_END ('F')
#define TRACE_EVENT_PHASE_NESTABLE_ASYNC_BEGIN ('b')
#define TRACE_EVENT_PHASE_NESTABLE_ASYNC_END ('e')
#define TRACE_EVENT_PHASE_NESTABLE_ASYNC_INSTANT ('n')
#define TRACE_EVENT_PHASE_FLOW_BEGIN ('s')
#define TRACE_EVENT_PHASE_FLOW_STEP ('t')
#define TRACE_EVENT_PHASE_FLOW_END ('f')
#define TRACE_EVENT_PHASE_METADATA ('M')
#define TRACE_EVENT_PHASE_COUNTER ('C')
#define TRACE_EVENT_PHASE_SAMPLE ('P')
#define TRACE_EVENT_PHASE_CREATE_OBJECT ('N')
#define TRACE_EVENT_PHASE_SNAPSHOT_OBJECT ('O')
#define TRACE_EVENT_PHASE_DELETE_OBJECT ('D')
#define TRACE_EVENT_PHASE_MEMORY_DUMP ('v')
#define TRACE_EVENT_PHASE_MARK ('R')
#define TRACE_EVENT_PHASE_CLOCK_SYNC ('c')

#define TRACE_EVENT_FLAG_NONE (static_cast<unsigned int>(0))
#define TRACE_EVENT_FLAG_COPY (static_cast<unsigned int>(1 << 0))
#define TRACE_EVENT_FLAG_HAS_ID (static_cast<unsigned int>(1 << 1))
#define TRACE_EVENT_FLAG_SCOPE_OFFSET (static_cast<unsigned int>(1 << 2))
#define TRACE_EVENT_FLAG_SCOPE_EXTRA (static_cast<unsigned int>(1 << 3))
#define TRACE_EVENT_FLAG_EXPLICIT_TIMESTAMP (static_cast<unsigned int>(1 << 4))
#define TRACE_EVENT_FLAG_ASYNC_TTS (static_cast<unsigned int>(1 << 5))
#define TRACE_EVENT_FLAG_BIND_TO_ENCLOSING (static_cast<unsigned int>(1 << 6))
#define TRACE_EVENT_FLAG_FLOW_IN (static_cast<unsigned int>(1 << 7))
#define TRACE_EVENT_FLAG_FLOW_OUT (static_cast<unsigned int>(1 << 8))
#define TRACE_EVENT_FLAG_HAS_CONTEXT_ID (static_cast<unsigned int>(1 << 9))
#define TRACE_EVENT_FLAG_HAS_PROCESS_ID (static_cast<unsigned int>(1 << 10))
#define TRACE_EVENT_FLAG_HAS_LOCAL_ID (static_cast<unsigned int>(1 << 11))
#define TRACE_EVENT_FLAG_HAS_GLOBAL_ID (static_cast<unsigned int>(1 << 12))
#define TRACE_EVENT_FLAG_JAVA_STRING_LITERALS \
  (static_cast<unsigned int>(1 << 16))
#define TRACE_EVENT_FLAG_SCOPE_MASK                          \
  (static_cast<unsigned int>(TRACE_EVENT_FLAG_SCOPE_OFFSET | \
                             TRACE_EVENT_FLAG_SCOPE_EXTRA))

#define TRACE_VALUE_TYPE_BOOL (static_cast<unsigned char>(1))
#define TRACE_VALUE_TYPE_UINT (static_cast<unsigned char>(2))
#define TRACE_VALUE_TYPE_INT (static_cast<unsigned char>(3))
#define TRACE_VALUE_TYPE_DOUBLE (static_cast<unsigned char>(4))
#define TRACE_VALUE_TYPE_POINTER (static_cast<unsigned char>(5))
#define TRACE_VALUE_TYPE_STRING (static_cast<unsigned char>(6))
#define TRACE_VALUE_TYPE_COPY_STRING (static_cast<unsigned char>(7))
#define TRACE_VALUE_TYPE_CONVERTABLE (static_cast<unsigned char>(8))
#define TRACE_VALUE_TYPE_PROTO (static_cast<unsigned char>(9))

#define TRACE_EVENT_SCOPE_GLOBAL (static_cast<unsigned char>(0 << 2))
#define TRACE_EVENT_SCOPE_PROCESS (static_cast<unsigned char>(1 << 2))
#define TRACE_EVENT_SCOPE_THREAD (static_cast<unsigned char>(2 << 2))
#define TRACE_EVENT_SCOPE_NAME_GLOBAL ('g')
#define TRACE_EVENT_SCOPE_NAME_PROCESS ('p')
#define TRACE_EVENT_SCOPE_NAME_THREAD ('t')

// Typed macros. For these, we have to erase the extra args entirely, as they
// may include a lambda that refers to protozero message types (which aren't
// available in the stub). This may trigger "unused variable" errors at the
// callsite, which have to be addressed at the callsite (e.g. via
// [[maybe_unused]]).
#define TRACE_EVENT_BEGIN(category, name, ...) \
  INTERNAL_TRACE_IGNORE(category, name)
#define TRACE_EVENT_END(category, ...) INTERNAL_TRACE_IGNORE(category)
#define TRACE_EVENT(category, name, ...) INTERNAL_TRACE_IGNORE(category, name)
#define TRACE_EVENT_INSTANT(category, name, ...) \
  INTERNAL_TRACE_IGNORE(category, name)

namespace base {
namespace trace_event {

class BASE_EXPORT ConvertableToTraceFormat {
 public:
  ConvertableToTraceFormat() = default;
  ConvertableToTraceFormat(const ConvertableToTraceFormat&) = delete;
  ConvertableToTraceFormat& operator=(const ConvertableToTraceFormat&) = delete;
  virtual ~ConvertableToTraceFormat();

  // Append the class info to the provided |out| string. The appended
  // data must be a valid JSON object. Strings must be properly quoted, and
  // escaped. There is no processing applied to the content after it is
  // appended.
  virtual void AppendAsTraceFormat(std::string* out) const = 0;
};

class BASE_EXPORT TracedValue : public ConvertableToTraceFormat {
 public:
  explicit TracedValue(size_t capacity = 0) {}

  void EndDictionary() {}
  void EndArray() {}

  void SetInteger(const char* name, int value) {}
  void SetDouble(const char* name, double value) {}
  void SetBoolean(const char* name, bool value) {}
  void SetString(const char* name, std::string_view value) {}
  void SetValue(const char* name, TracedValue* value) {}
  void BeginDictionary(const char* name) {}
  void BeginArray(const char* name) {}

  void SetIntegerWithCopiedName(std::string_view name, int value) {}
  void SetDoubleWithCopiedName(std::string_view name, double value) {}
  void SetBooleanWithCopiedName(std::string_view name, bool value) {}
  void SetStringWithCopiedName(std::string_view name, std::string_view value) {}
  void SetValueWithCopiedName(std::string_view name, TracedValue* value) {}
  void BeginDictionaryWithCopiedName(std::string_view name) {}
  void BeginArrayWithCopiedName(std::string_view name) {}

  void AppendInteger(int) {}
  void AppendDouble(double) {}
  void AppendBoolean(bool) {}
  void AppendString(std::string_view) {}
  void BeginArray() {}
  void BeginDictionary() {}

  void AppendAsTraceFormat(std::string* out) const override;
};

class BASE_EXPORT TracedValueJSON : public TracedValue {
 public:
  explicit TracedValueJSON(size_t capacity = 0) : TracedValue(capacity) {}

  std::unique_ptr<base::Value> ToBaseValue() const { return nullptr; }
  std::string ToJSON() const { return ""; }
  std::string ToFormattedJSON() const { return ""; }
};

struct MemoryDumpArgs;
class ProcessMemoryDump;

class BASE_EXPORT MemoryDumpProvider {
 public:
  MemoryDumpProvider(const MemoryDumpProvider&) = delete;
  MemoryDumpProvider& operator=(const MemoryDumpProvider&) = delete;
  virtual ~MemoryDumpProvider();

  virtual bool OnMemoryDump(const MemoryDumpArgs& args,
                            ProcessMemoryDump* pmd) = 0;

 protected:
  MemoryDumpProvider() = default;
};

class BASE_EXPORT MemoryDumpManager {
 public:
  static constexpr const char* const kTraceCategory =
      TRACE_DISABLED_BY_DEFAULT("memory-infra");
};

}  // namespace trace_event
}  // namespace base

// Stub implementation for
// perfetto::StaticString/ThreadTrack/TracedValue/TracedDictionary/TracedArray/
// Track.
namespace perfetto {

class TracedArray;
class TracedDictionary;
class EventContext;

class StaticString {
 public:
  template <typename T>
  StaticString(T) {}
};

class DynamicString {
 public:
  template <typename T>
  explicit DynamicString(T) {}
};

class TracedValue {
 public:
  void WriteInt64(int64_t) && {}
  void WriteUInt64(uint64_t) && {}
  void WriteDouble(double) && {}
  void WriteBoolean(bool) && {}
  void WriteString(const char*) && {}
  void WriteString(const char*, size_t) && {}
  void WriteString(const std::string&) && {}
  void WritePointer(const void*) && {}

  TracedDictionary WriteDictionary() &&;
  TracedArray WriteArray() &&;
};

class TracedDictionary {
 public:
  TracedValue AddItem(StaticString) { return TracedValue(); }
  TracedValue AddItem(DynamicString) { return TracedValue(); }

  template <typename T>
  void Add(StaticString, T&&) {}
  template <typename T>
  void Add(DynamicString, T&&) {}

  TracedDictionary AddDictionary(StaticString);
  TracedDictionary AddDictionary(DynamicString);
  TracedArray AddArray(StaticString);
  TracedArray AddArray(DynamicString);
};

class TracedArray {
 public:
  TracedValue AppendItem() { return TracedValue(); }

  template <typename T>
  void Append(T&&) {}

  TracedDictionary AppendDictionary();
  TracedArray AppendArray();
};

template <class T>
void WriteIntoTracedValue(TracedValue, T&&) {}

struct Track {
  explicit Track(uint64_t id) {}
};

struct NamedTrack {
  template <class T>
  explicit NamedTrack(T name, uint64_t id = 0, Track parent = Track{0}) {}
};

struct Flow {
  static inline Flow ProcessScoped(uint64_t flow_id) { return Flow(); }
  static inline Flow FromPointer(void* ptr) { return Flow(); }
  static inline Flow Global(uint64_t flow_id) { return Flow(); }
};

namespace protos::pbzero {
namespace SequenceManagerTask {

enum class QueueName {
  UNKNOWN_TQ = 0,
  DEFAULT_TQ = 1,
  TASK_ENVIRONMENT_DEFAULT_TQ = 2,
  TEST2_TQ = 3,
  TEST_TQ = 4,
};
inline const char* QueueName_Name(QueueName value) {
  switch (value) {
    case QueueName::UNKNOWN_TQ:
      return "UNKNOWN_TQ";
    case QueueName::DEFAULT_TQ:
      return "DEFAULT_TQ";
    case QueueName::TASK_ENVIRONMENT_DEFAULT_TQ:
      return "TASK_ENVIRONMENT_DEFAULT_TQ";
    case QueueName::TEST2_TQ:
      return "TEST2_TQ";
    case QueueName::TEST_TQ:
      return "TEST_TQ";
  }
}

}  // namespace SequenceManagerTask

namespace ChromeProcessDescriptor {

enum ProcessType {};

}  // namespace ChromeProcessDescriptor

}  // namespace protos::pbzero
}  // namespace perfetto

#endif  // BASE_TRACE_EVENT_TRACE_EVENT_STUB_H_
