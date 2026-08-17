/*
 *  Copyright (c) 2024 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_BASE_TRACE_EVENT_H_
#define RTC_BASE_TRACE_EVENT_H_

#if defined(RTC_DISABLE_TRACE_EVENTS)
#define RTC_TRACE_EVENTS_ENABLED 0
#else
#define RTC_TRACE_EVENTS_ENABLED 1
#endif

// IWYU pragma: begin_exports
#if defined(RTC_USE_PERFETTO)
#include "rtc_base/trace_categories.h"
#endif
#include "third_party/perfetto/include/perfetto/tracing/event_context.h"
#include "third_party/perfetto/include/perfetto/tracing/track.h"
#include "third_party/perfetto/include/perfetto/tracing/track_event_args.h"
// IWYU pragma: end_exports

#if !defined(RTC_USE_PERFETTO)
#include <string>

#include "rtc_base/event_tracer.h"

#define RTC_NOOP() \
  do {             \
  } while (0)

// TODO(b/42226290): Add implementation for these events with Perfetto.
#define TRACE_EVENT_BEGIN(category, name, ...) RTC_NOOP();
#define TRACE_EVENT_END(category, ...) RTC_NOOP();
#define TRACE_EVENT(category, name, ...) RTC_NOOP();
#define TRACE_EVENT_INSTANT(category, name, ...) RTC_NOOP();
#define TRACE_EVENT_CATEGORY_ENABLED(category) RTC_NOOP();
#define TRACE_COUNTER(category, track, ...) RTC_NOOP();

// Type values for identifying types in the TraceValue union.
#define TRACE_VALUE_TYPE_BOOL (static_cast<unsigned char>(1))
#define TRACE_VALUE_TYPE_UINT (static_cast<unsigned char>(2))
#define TRACE_VALUE_TYPE_INT (static_cast<unsigned char>(3))
#define TRACE_VALUE_TYPE_DOUBLE (static_cast<unsigned char>(4))
#define TRACE_VALUE_TYPE_POINTER (static_cast<unsigned char>(5))
#define TRACE_VALUE_TYPE_STRING (static_cast<unsigned char>(6))
#define TRACE_VALUE_TYPE_COPY_STRING (static_cast<unsigned char>(7))

#if defined(TRACE_EVENT0)
#error "Another copy of trace_event.h has already been included."
#endif

#if RTC_TRACE_EVENTS_ENABLED

// Extracted from Chromium's src/base/debug/trace_event.h.

// This header is designed to give you trace_event macros without specifying
// how the events actually get collected and stored. If you need to expose trace
// event to some other universe, you can copy-and-paste this file,
// implement the TRACE_EVENT_API macros, and do any other necessary fixup for
// the target platform. The end result is that multiple libraries can funnel
// events through to a shared trace event collector.

// Trace events are for tracking application performance and resource usage.
// Macros are provided to track:
//    Begin and end of function calls
//    Counters
//
// Events are issued against categories. Whereas RTC_LOG's
// categories are statically defined, TRACE categories are created
// implicitly with a string. For example:
//   TRACE_EVENT_INSTANT0("MY_SUBSYSTEM", "SomeImportantEvent")
//
// Events can be INSTANT, or can be pairs of BEGIN and END in the same scope:
//   TRACE_EVENT_BEGIN0("MY_SUBSYSTEM", "SomethingCostly")
//   doSomethingCostly()
//   TRACE_EVENT_END0("MY_SUBSYSTEM", "SomethingCostly")
// Note: our tools can't always determine the correct BEGIN/END pairs unless
// these are used in the same scope. Use ASYNC_BEGIN/ASYNC_END macros if you
// need them to be in separate scopes.
//
// A common use case is to trace entire function scopes. This
// issues a trace BEGIN and END automatically:
//   void doSomethingCostly() {
//     TRACE_EVENT0("MY_SUBSYSTEM", "doSomethingCostly");
//     ...
//   }
//
// Additional parameters can be associated with an event:
//   void doSomethingCostly2(int howMuch) {
//     TRACE_EVENT1("MY_SUBSYSTEM", "doSomethingCostly",
//         "howMuch", howMuch);
//     ...
//   }
//
// The trace system will automatically add to this information the
// current process id, thread id, and a timestamp in microseconds.
//
// To trace an asynchronous procedure such as an IPC send/receive, use
// ASYNC_BEGIN and ASYNC_END:
//   [single threaded sender code]
//     static int send_count = 0;
//     ++send_count;
//     TRACE_EVENT_ASYNC_BEGIN0("ipc", "message", send_count);
//     Send(new MyMessage(send_count));
//   [receive code]
//     void OnMyMessage(send_count) {
//       TRACE_EVENT_ASYNC_END0("ipc", "message", send_count);
//     }
// The third parameter is a unique ID to match ASYNC_BEGIN/ASYNC_END pairs.
// ASYNC_BEGIN and ASYNC_END can occur on any thread of any traced process.
// Pointers can be used for the ID parameter, and they will be mangled
// internally so that the same pointer on two different processes will not
// match. For example:
//   class MyTracedClass {
//    public:
//     MyTracedClass() {
//       TRACE_EVENT_ASYNC_BEGIN0("category", "MyTracedClass", this);
//     }
//     ~MyTracedClass() {
//       TRACE_EVENT_ASYNC_END0("category", "MyTracedClass", this);
//     }
//   }
//
// Trace event also supports counters, which is a way to track a quantity
// as it varies over time. Counters are created with the following macro:
//   TRACE_COUNTER1("MY_SUBSYSTEM", "myCounter", g_myCounterValue);
//
// Counters are process-specific. The macro itself can be issued from any
// thread, however.
//
// Sometimes, you want to track two counters at once. You can do this with two
// counter macros:
//   TRACE_COUNTER1("MY_SUBSYSTEM", "myCounter0", g_myCounterValue[0]);
//   TRACE_COUNTER1("MY_SUBSYSTEM", "myCounter1", g_myCounterValue[1]);
// Or you can do it with a combined macro:
//   TRACE_COUNTER2("MY_SUBSYSTEM", "myCounter",
//       "bytesPinned", g_myCounterValue[0],
//       "bytesAllocated", g_myCounterValue[1]);
// This indicates to the tracing UI that these counters should be displayed
// in a single graph, as a summed area chart.
//
// Since counters are in a global namespace, you may want to disembiguate with a
// unique ID, by using the TRACE_COUNTER_ID* variations.
//
// By default, trace collection is compiled in, but turned off at runtime.
// Collecting trace data is the responsibility of the embedding
// application. In Chrome's case, navigating to about:tracing will turn on
// tracing and display data collected across all active processes.
//
// When are string argument values copied:
// const char* arg_values are only referenced by default:
//     TRACE_EVENT1("category", "name",
//                  "arg1", "literal string is only referenced");
// Use TRACE_STR_COPY to force copying of a const char*:
//     TRACE_EVENT1("category", "name",
//                  "arg1", TRACE_STR_COPY("string will be copied"));
// std::string arg_values are always copied:
//     TRACE_EVENT1("category", "name",
//                  "arg1", std::string("string will be copied"));
//
//
// Thread Safety:
// Thread safety is provided by methods defined in event_tracer.h. See the file
// for details.

// By default, const char* argument values are assumed to have long-lived scope
// and will not be copied. Use this macro to force a const char* to be copied.
#define TRACE_STR_COPY(str) \
  webrtc::trace_event_internal::TraceStringWithCopy(str)

// This will mark the trace event as disabled by default. The user will need
// to explicitly enable the event.
#define TRACE_DISABLED_BY_DEFAULT(name) "disabled-by-default-" name

// By default, uint64 ID argument values are not mangled with the Process ID in
// TRACE_EVENT_ASYNC macros. Use this macro to force Process ID mangling.
#define TRACE_ID_MANGLE(id) \
  webrtc::trace_event_internal::TraceID::ForceMangle(id)

// Records a pair of begin and end events called "name" for the current
// scope, with 0, 1 or 2 associated arguments. If the category is not
// enabled, then this does nothing.
// - category and name strings must have application lifetime (statics or
//   literals). They may not include " chars.
#define TRACE_EVENT0(category, name) \
  INTERNAL_TRACE_EVENT_ADD_SCOPED(category, name)
#define TRACE_EVENT1(category, name, arg1_name, arg1_val) \
  INTERNAL_TRACE_EVENT_ADD_SCOPED(category, name, arg1_name, arg1_val)
#define TRACE_EVENT2(category, name, arg1_name, arg1_val, arg2_name, arg2_val) \
  INTERNAL_TRACE_EVENT_ADD_SCOPED(category, name, arg1_name, arg1_val,         \
                                  arg2_name, arg2_val)

// Enum reflecting the scope of an INSTANT event. Must fit within
// TRACE_EVENT_FLAG_SCOPE_MASK.
static constexpr uint8_t TRACE_EVENT_SCOPE_GLOBAL = 0u << 2;
static constexpr uint8_t TRACE_EVENT_SCOPE_PROCESS = 1u << 2;
static constexpr uint8_t TRACE_EVENT_SCOPE_THREAD = 2u << 2;

// Records a single event called "name" immediately, with 0, 1 or 2
// associated arguments. If the category is not enabled, then this
// does nothing.
// - category and name strings must have application lifetime (statics or
//   literals). They may not include " chars.
#define TRACE_EVENT_INSTANT0(category, name, scope)                   \
  INTERNAL_TRACE_EVENT_ADD(TRACE_EVENT_PHASE_INSTANT, category, name, \
                           TRACE_EVENT_FLAG_NONE)
#define TRACE_EVENT_INSTANT1(category, name, scope, arg1_name, arg1_val) \
  INTERNAL_TRACE_EVENT_ADD(TRACE_EVENT_PHASE_INSTANT, category, name,    \
                           TRACE_EVENT_FLAG_NONE, arg1_name, arg1_val)
#define TRACE_EVENT_INSTANT2(category, name, scope, arg1_name, arg1_val, \
                             arg2_name, arg2_val)                        \
  INTERNAL_TRACE_EVENT_ADD(TRACE_EVENT_PHASE_INSTANT, category, name,    \
                           TRACE_EVENT_FLAG_NONE, arg1_name, arg1_val,   \
                           arg2_name, arg2_val)

// Records a single BEGIN event called "name" immediately, with 0, 1 or 2
// associated arguments. If the category is not enabled, then this
// does nothing.
// - category and name strings must have application lifetime (statics or
//   literals). They may not include " chars.
#define TRACE_EVENT_BEGIN0(category, name)                          \
  INTERNAL_TRACE_EVENT_ADD(TRACE_EVENT_PHASE_BEGIN, category, name, \
                           TRACE_EVENT_FLAG_NONE)
#define TRACE_EVENT_BEGIN1(category, name, arg1_name, arg1_val)     \
  INTERNAL_TRACE_EVENT_ADD(TRACE_EVENT_PHASE_BEGIN, category, name, \
                           TRACE_EVENT_FLAG_NONE, arg1_name, arg1_val)
#define TRACE_EVENT_BEGIN2(category, name, arg1_name, arg1_val, arg2_name, \
                           arg2_val)                                       \
  INTERNAL_TRACE_EVENT_ADD(TRACE_EVENT_PHASE_BEGIN, category, name,        \
                           TRACE_EVENT_FLAG_NONE, arg1_name, arg1_val,     \
                           arg2_name, arg2_val)

// Records a single END event for "name" immediately. If the category
// is not enabled, then this does nothing.
// - category and name strings must have application lifetime (statics or
//   literals). They may not include " chars.
#define TRACE_EVENT_END0(category, name)                          \
  INTERNAL_TRACE_EVENT_ADD(TRACE_EVENT_PHASE_END, category, name, \
                           TRACE_EVENT_FLAG_NONE)
#define TRACE_EVENT_END1(category, name, arg1_name, arg1_val)     \
  INTERNAL_TRACE_EVENT_ADD(TRACE_EVENT_PHASE_END, category, name, \
                           TRACE_EVENT_FLAG_NONE, arg1_name, arg1_val)
#define TRACE_EVENT_END2(category, name, arg1_name, arg1_val, arg2_name, \
                         arg2_val)                                       \
  INTERNAL_TRACE_EVENT_ADD(TRACE_EVENT_PHASE_END, category, name,        \
                           TRACE_EVENT_FLAG_NONE, arg1_name, arg1_val,   \
                           arg2_name, arg2_val)

// Records the value of a counter called "name" immediately. Value
// must be representable as a 32 bit integer.
// - category and name strings must have application lifetime (statics or
//   literals). They may not include " chars.
#define TRACE_COUNTER1(category, name, value)                         \
  INTERNAL_TRACE_EVENT_ADD(TRACE_EVENT_PHASE_COUNTER, category, name, \
                           TRACE_EVENT_FLAG_NONE, "value",            \
                           static_cast<int>(value))

// Records the values of a multi-parted counter called "name" immediately.
// The UI will treat value1 and value2 as parts of a whole, displaying their
// values as a stacked-bar chart.
// - category and name strings must have application lifetime (statics or
//   literals). They may not include " chars.
#define TRACE_COUNTER2(category, name, value1_name, value1_val, value2_name, \
                       value2_val)                                           \
  INTERNAL_TRACE_EVENT_ADD(TRACE_EVENT_PHASE_COUNTER, category, name,        \
                           TRACE_EVENT_FLAG_NONE, value1_name,               \
                           static_cast<int>(value1_val), value2_name,        \
                           static_cast<int>(value2_val))

// Records the value of a counter called "name" immediately. Value
// must be representable as a 32 bit integer.
// - category and name strings must have application lifetime (statics or
//   literals). They may not include " chars.
// - `id` is used to disambiguate counters with the same name. It must either
//   be a pointer or an integer value up to 64 bits. If it's a pointer, the bits
//   will be xored with a hash of the process ID so that the same pointer on
//   two different processes will not collide.
#define TRACE_COUNTER_ID1(category, name, id, value)                          \
  INTERNAL_TRACE_EVENT_ADD_WITH_ID(TRACE_EVENT_PHASE_COUNTER, category, name, \
                                   id, TRACE_EVENT_FLAG_NONE, "value",        \
                                   static_cast<int>(value))

// Records the values of a multi-parted counter called "name" immediately.
// The UI will treat value1 and value2 as parts of a whole, displaying their
// values as a stacked-bar chart.
// - category and name strings must have application lifetime (statics or
//   literals). They may not include " chars.
// - `id` is used to disambiguate counters with the same name. It must either
//   be a pointer or an integer value up to 64 bits. If it's a pointer, the bits
//   will be xored with a hash of the process ID so that the same pointer on
//   two different processes will not collide.
#define TRACE_COUNTER_ID2(category, name, id, value1_name, value1_val,        \
                          value2_name, value2_val)                            \
  INTERNAL_TRACE_EVENT_ADD_WITH_ID(TRACE_EVENT_PHASE_COUNTER, category, name, \
                                   id, TRACE_EVENT_FLAG_NONE, value1_name,    \
                                   static_cast<int>(value1_val), value2_name, \
                                   static_cast<int>(value2_val))

// Records a single ASYNC_BEGIN event called "name" immediately, with 0, 1 or 2
// associated arguments. If the category is not enabled, then this
// does nothing.
// - category and name strings must have application lifetime (statics or
//   literals). They may not include " chars.
// - `id` is used to match the ASYNC_BEGIN event with the ASYNC_END event. ASYNC
//   events are considered to match if their category, name and id values all
//   match. `id` must either be a pointer or an integer value up to 64 bits. If
//   it's a pointer, the bits will be xored with a hash of the process ID so
//   that the same pointer on two different processes will not collide.
// An asynchronous operation can consist of multiple phases. The first phase is
// defined by the ASYNC_BEGIN calls. Additional phases can be defined using the
// ASYNC_STEP macros. When the operation completes, call ASYNC_END.
// An ASYNC trace typically occur on a single thread (if not, they will only be
// drawn on the thread defined in the ASYNC_BEGIN event), but all events in that
// operation must use the same `name` and `id`. Each event can have its own
// args.
#define TRACE_EVENT_ASYNC_BEGIN0(category, name, id)                        \
  INTERNAL_TRACE_EVENT_ADD_WITH_ID(TRACE_EVENT_PHASE_ASYNC_BEGIN, category, \
                                   name, id, TRACE_EVENT_FLAG_NONE)
#define TRACE_EVENT_ASYNC_BEGIN1(category, name, id, arg1_name, arg1_val)      \
  INTERNAL_TRACE_EVENT_ADD_WITH_ID(TRACE_EVENT_PHASE_ASYNC_BEGIN, category,    \
                                   name, id, TRACE_EVENT_FLAG_NONE, arg1_name, \
                                   arg1_val)
#define TRACE_EVENT_ASYNC_BEGIN2(category, name, id, arg1_name, arg1_val,      \
                                 arg2_name, arg2_val)                          \
  INTERNAL_TRACE_EVENT_ADD_WITH_ID(TRACE_EVENT_PHASE_ASYNC_BEGIN, category,    \
                                   name, id, TRACE_EVENT_FLAG_NONE, arg1_name, \
                                   arg1_val, arg2_name, arg2_val)

// Records a single ASYNC_STEP event for `step` immediately. If the category
// is not enabled, then this does nothing. The `name` and `id` must match the
// ASYNC_BEGIN event above. The `step` param identifies this step within the
// async event. This should be called at the beginning of the next phase of an
// asynchronous operation.
#define TRACE_EVENT_ASYNC_STEP_INTO0(category, name, id, step)              \
  INTERNAL_TRACE_EVENT_ADD_WITH_ID(TRACE_EVENT_PHASE_ASYNC_STEP, category,  \
                                   name, id, TRACE_EVENT_FLAG_NONE, "step", \
                                   step)
#define TRACE_EVENT_ASYNC_STEP_INTO1(category, name, id, step, arg1_name,   \
                                     arg1_val)                              \
  INTERNAL_TRACE_EVENT_ADD_WITH_ID(TRACE_EVENT_PHASE_ASYNC_STEP, category,  \
                                   name, id, TRACE_EVENT_FLAG_NONE, "step", \
                                   step, arg1_name, arg1_val)

// Records a single ASYNC_END event for "name" immediately. If the category
// is not enabled, then this does nothing.
#define TRACE_EVENT_ASYNC_END0(category, name, id)                        \
  INTERNAL_TRACE_EVENT_ADD_WITH_ID(TRACE_EVENT_PHASE_ASYNC_END, category, \
                                   name, id, TRACE_EVENT_FLAG_NONE)
#define TRACE_EVENT_ASYNC_END1(category, name, id, arg1_name, arg1_val)        \
  INTERNAL_TRACE_EVENT_ADD_WITH_ID(TRACE_EVENT_PHASE_ASYNC_END, category,      \
                                   name, id, TRACE_EVENT_FLAG_NONE, arg1_name, \
                                   arg1_val)
#define TRACE_EVENT_ASYNC_END2(category, name, id, arg1_name, arg1_val,        \
                               arg2_name, arg2_val)                            \
  INTERNAL_TRACE_EVENT_ADD_WITH_ID(TRACE_EVENT_PHASE_ASYNC_END, category,      \
                                   name, id, TRACE_EVENT_FLAG_NONE, arg1_name, \
                                   arg1_val, arg2_name, arg2_val)

////////////////////////////////////////////////////////////////////////////////
// Implementation specific tracing API definitions.

// Get a pointer to the enabled state of the given trace category. Only
// long-lived literal strings should be given as the category name. The returned
// pointer can be held permanently in a local static for example. If the
// unsigned char is non-zero, tracing is enabled. If tracing is enabled,
// TRACE_EVENT_API_ADD_TRACE_EVENT can be called. It's OK if tracing is disabled
// between the load of the tracing state and the call to
// TRACE_EVENT_API_ADD_TRACE_EVENT, because this flag only provides an early out
// for best performance when tracing is disabled.
// const unsigned char*
//     TRACE_EVENT_API_GET_CATEGORY_ENABLED(const char* category_name)
#define TRACE_EVENT_API_GET_CATEGORY_ENABLED \
  webrtc::EventTracer::GetCategoryEnabled

// Add a trace event to the platform tracing system.
// void TRACE_EVENT_API_ADD_TRACE_EVENT(
//                    char phase,
//                    const unsigned char* category_enabled,
//                    const char* name,
//                    unsigned long long id,
//                    int num_args,
//                    const char** arg_names,
//                    const unsigned char* arg_types,
//                    const unsigned long long* arg_values,
//                    unsigned char flags)
#define TRACE_EVENT_API_ADD_TRACE_EVENT webrtc::EventTracer::AddTraceEvent

////////////////////////////////////////////////////////////////////////////////

// Implementation detail: trace event macros create temporary variables
// to keep instrumentation overhead low. These macros give each temporary
// variable a unique name based on the line number to prevent name collissions.
#define INTERNAL_TRACE_EVENT_UID3(a, b) trace_event_unique_##a##b
#define INTERNAL_TRACE_EVENT_UID2(a, b) INTERNAL_TRACE_EVENT_UID3(a, b)
#define INTERNAL_TRACE_EVENT_UID(name_prefix) \
  INTERNAL_TRACE_EVENT_UID2(name_prefix, __LINE__)

#if WEBRTC_NON_STATIC_TRACE_EVENT_HANDLERS
#define INTERNAL_TRACE_EVENT_INFO_TYPE const unsigned char*
#else
#define INTERNAL_TRACE_EVENT_INFO_TYPE static const unsigned char*
#endif  // WEBRTC_NON_STATIC_TRACE_EVENT_HANDLERS

// Implementation detail: internal macro to create static category.
#define INTERNAL_TRACE_EVENT_GET_CATEGORY_INFO(category)               \
  INTERNAL_TRACE_EVENT_INFO_TYPE INTERNAL_TRACE_EVENT_UID(catstatic) = \
      TRACE_EVENT_API_GET_CATEGORY_ENABLED(category);

// Implementation detail: internal macro to create static category and add
// event if the category is enabled.
#define INTERNAL_TRACE_EVENT_ADD(phase, category, name, flags, ...)        \
  do {                                                                     \
    INTERNAL_TRACE_EVENT_GET_CATEGORY_INFO(category);                      \
    if (*INTERNAL_TRACE_EVENT_UID(catstatic)) {                            \
      webrtc::trace_event_internal::AddTraceEvent(                         \
          phase, INTERNAL_TRACE_EVENT_UID(catstatic), name,                \
          webrtc::trace_event_internal::kNoEventId, flags, ##__VA_ARGS__); \
    }                                                                      \
  } while (0)

// Implementation detail: internal macro to create static category and add begin
// event if the category is enabled. Also adds the end event when the scope
// ends.
#define INTERNAL_TRACE_EVENT_ADD_SCOPED(category, name, ...)                   \
  INTERNAL_TRACE_EVENT_GET_CATEGORY_INFO(category);                            \
  webrtc::trace_event_internal::TraceEndOnScopeClose INTERNAL_TRACE_EVENT_UID( \
      profileScope);                                                           \
  if (*INTERNAL_TRACE_EVENT_UID(catstatic)) {                                  \
    webrtc::trace_event_internal::AddTraceEvent(                               \
        TRACE_EVENT_PHASE_BEGIN, INTERNAL_TRACE_EVENT_UID(catstatic), name,    \
        webrtc::trace_event_internal::kNoEventId, TRACE_EVENT_FLAG_NONE,       \
        ##__VA_ARGS__);                                                        \
    INTERNAL_TRACE_EVENT_UID(profileScope)                                     \
        .Initialize(INTERNAL_TRACE_EVENT_UID(catstatic), name);                \
  }

// Implementation detail: internal macro to create static category and add
// event if the category is enabled.
#define INTERNAL_TRACE_EVENT_ADD_WITH_ID(phase, category, name, id, flags, \
                                         ...)                              \
  do {                                                                     \
    INTERNAL_TRACE_EVENT_GET_CATEGORY_INFO(category);                      \
    if (*INTERNAL_TRACE_EVENT_UID(catstatic)) {                            \
      unsigned char trace_event_flags = flags | TRACE_EVENT_FLAG_HAS_ID;   \
      webrtc::trace_event_internal::TraceID trace_event_trace_id(          \
          id, &trace_event_flags);                                         \
      webrtc::trace_event_internal::AddTraceEvent(                         \
          phase, INTERNAL_TRACE_EVENT_UID(catstatic), name,                \
          trace_event_trace_id.data(), trace_event_flags, ##__VA_ARGS__);  \
    }                                                                      \
  } while (0)

// Notes regarding the following definitions:
// New values can be added and propagated to third party libraries, but existing
// definitions must never be changed, because third party libraries may use old
// definitions.

// Phase indicates the nature of an event entry. E.g. part of a begin/end pair.
#define TRACE_EVENT_PHASE_BEGIN ('B')
#define TRACE_EVENT_PHASE_END ('E')
#define TRACE_EVENT_PHASE_INSTANT ('I')
#define TRACE_EVENT_PHASE_ASYNC_BEGIN ('S')
#define TRACE_EVENT_PHASE_ASYNC_STEP ('T')
#define TRACE_EVENT_PHASE_ASYNC_END ('F')
#define TRACE_EVENT_PHASE_METADATA ('M')
#define TRACE_EVENT_PHASE_COUNTER ('C')

// Flags for changing the behavior of TRACE_EVENT_API_ADD_TRACE_EVENT.
#define TRACE_EVENT_FLAG_NONE (static_cast<unsigned char>(0))
#define TRACE_EVENT_FLAG_HAS_ID (static_cast<unsigned char>(1 << 1))
#define TRACE_EVENT_FLAG_MANGLE_ID (static_cast<unsigned char>(1 << 2))

namespace webrtc {
namespace trace_event_internal {

// Specify these values when the corresponding argument of AddTraceEvent is not
// used.
const int kZeroNumArgs = 0;
const unsigned long long kNoEventId = 0;

// TraceID encapsulates an ID that can either be an integer or pointer. Pointers
// are mangled with the Process ID so that they are unlikely to collide when the
// same pointer is used on different processes.
class TraceID {
 public:
  class ForceMangle {
   public:
    explicit ForceMangle(unsigned long long id) : data_(id) {}
    explicit ForceMangle(unsigned long id) : data_(id) {}
    explicit ForceMangle(unsigned int id) : data_(id) {}
    explicit ForceMangle(unsigned short id) : data_(id) {}
    explicit ForceMangle(unsigned char id) : data_(id) {}
    explicit ForceMangle(long long id)
        : data_(static_cast<unsigned long long>(id)) {}
    explicit ForceMangle(long id)
        : data_(static_cast<unsigned long long>(id)) {}
    explicit ForceMangle(int id) : data_(static_cast<unsigned long long>(id)) {}
    explicit ForceMangle(short id)
        : data_(static_cast<unsigned long long>(id)) {}
    explicit ForceMangle(signed char id)
        : data_(static_cast<unsigned long long>(id)) {}

    unsigned long long data() const { return data_; }

   private:
    unsigned long long data_;
  };

  explicit TraceID(const void* id, unsigned char* flags)
      : data_(
            static_cast<unsigned long long>(reinterpret_cast<uintptr_t>(id))) {
    *flags |= TRACE_EVENT_FLAG_MANGLE_ID;
  }
  explicit TraceID(ForceMangle id, unsigned char* flags) : data_(id.data()) {
    *flags |= TRACE_EVENT_FLAG_MANGLE_ID;
  }
  explicit TraceID(unsigned long long id, unsigned char* flags) : data_(id) {
    (void)flags;
  }
  explicit TraceID(unsigned long id, unsigned char* flags) : data_(id) {
    (void)flags;
  }
  explicit TraceID(unsigned int id, unsigned char* flags) : data_(id) {
    (void)flags;
  }
  explicit TraceID(unsigned short id, unsigned char* flags) : data_(id) {
    (void)flags;
  }
  explicit TraceID(unsigned char id, unsigned char* flags) : data_(id) {
    (void)flags;
  }
  explicit TraceID(long long id, unsigned char* flags)
      : data_(static_cast<unsigned long long>(id)) {
    (void)flags;
  }
  explicit TraceID(long id, unsigned char* flags)
      : data_(static_cast<unsigned long long>(id)) {
    (void)flags;
  }
  explicit TraceID(int id, unsigned char* flags)
      : data_(static_cast<unsigned long long>(id)) {
    (void)flags;
  }
  explicit TraceID(short id, unsigned char* flags)
      : data_(static_cast<unsigned long long>(id)) {
    (void)flags;
  }
  explicit TraceID(signed char id, unsigned char* flags)
      : data_(static_cast<unsigned long long>(id)) {
    (void)flags;
  }

  unsigned long long data() const { return data_; }

 private:
  unsigned long long data_;
};

// Simple union to store various types as unsigned long long.
union TraceValueUnion {
  bool as_bool;
  unsigned long long as_uint;
  long long as_int;
  double as_double;
  const void* as_pointer;
  const char* as_string;
};

// Simple container for const char* that should be copied instead of retained.
class TraceStringWithCopy {
 public:
  explicit TraceStringWithCopy(const char* str) : str_(str) {}
  operator const char*() const { return str_; }

 private:
  const char* str_;
};

// Define SetTraceValue for each allowed type. It stores the type and
// value in the return arguments. This allows this API to avoid declaring any
// structures so that it is portable to third_party libraries.
#define INTERNAL_DECLARE_SET_TRACE_VALUE(actual_type, union_member,      \
                                         value_type_id)                  \
  static inline void SetTraceValue(actual_type arg, unsigned char* type, \
                                   unsigned long long* value) {          \
    TraceValueUnion type_value;                                          \
    type_value.union_member = arg;                                       \
    *type = value_type_id;                                               \
    *value = type_value.as_uint;                                         \
  }
// Simpler form for int types that can be safely casted.
#define INTERNAL_DECLARE_SET_TRACE_VALUE_INT(actual_type, value_type_id) \
  static inline void SetTraceValue(actual_type arg, unsigned char* type, \
                                   unsigned long long* value) {          \
    *type = value_type_id;                                               \
    *value = static_cast<unsigned long long>(arg);                       \
  }

INTERNAL_DECLARE_SET_TRACE_VALUE_INT(unsigned long long, TRACE_VALUE_TYPE_UINT)
INTERNAL_DECLARE_SET_TRACE_VALUE_INT(unsigned long, TRACE_VALUE_TYPE_UINT)
INTERNAL_DECLARE_SET_TRACE_VALUE_INT(unsigned int, TRACE_VALUE_TYPE_UINT)
INTERNAL_DECLARE_SET_TRACE_VALUE_INT(unsigned short, TRACE_VALUE_TYPE_UINT)
INTERNAL_DECLARE_SET_TRACE_VALUE_INT(unsigned char, TRACE_VALUE_TYPE_UINT)
INTERNAL_DECLARE_SET_TRACE_VALUE_INT(long long, TRACE_VALUE_TYPE_INT)
INTERNAL_DECLARE_SET_TRACE_VALUE_INT(long, TRACE_VALUE_TYPE_INT)
INTERNAL_DECLARE_SET_TRACE_VALUE_INT(int, TRACE_VALUE_TYPE_INT)
INTERNAL_DECLARE_SET_TRACE_VALUE_INT(short, TRACE_VALUE_TYPE_INT)
INTERNAL_DECLARE_SET_TRACE_VALUE_INT(signed char, TRACE_VALUE_TYPE_INT)
INTERNAL_DECLARE_SET_TRACE_VALUE(bool, as_bool, TRACE_VALUE_TYPE_BOOL)
INTERNAL_DECLARE_SET_TRACE_VALUE(double, as_double, TRACE_VALUE_TYPE_DOUBLE)
INTERNAL_DECLARE_SET_TRACE_VALUE(const void*,
                                 as_pointer,
                                 TRACE_VALUE_TYPE_POINTER)
INTERNAL_DECLARE_SET_TRACE_VALUE(const char*,
                                 as_string,
                                 TRACE_VALUE_TYPE_STRING)
INTERNAL_DECLARE_SET_TRACE_VALUE(const TraceStringWithCopy&,
                                 as_string,
                                 TRACE_VALUE_TYPE_COPY_STRING)

#undef INTERNAL_DECLARE_SET_TRACE_VALUE
#undef INTERNAL_DECLARE_SET_TRACE_VALUE_INT

// std::string version of SetTraceValue so that trace arguments can be strings.
static inline void SetTraceValue(const std::string& arg,
                                 unsigned char* type,
                                 unsigned long long* value) {
  TraceValueUnion type_value;
  type_value.as_string = arg.c_str();
  *type = TRACE_VALUE_TYPE_COPY_STRING;
  *value = type_value.as_uint;
}

// These AddTraceEvent template functions are defined here instead of in the
// macro, because the arg_values could be temporary objects, such as
// std::string. In order to store pointers to the internal c_str and pass
// through to the tracing API, the arg_values must live throughout
// these procedures.

static inline void AddTraceEvent(char phase,
                                 const unsigned char* category_enabled,
                                 const char* name,
                                 unsigned long long id,
                                 unsigned char flags) {
  TRACE_EVENT_API_ADD_TRACE_EVENT(phase, category_enabled, name, id,
                                  kZeroNumArgs, nullptr, nullptr, nullptr,
                                  flags);
}

template <class ARG1_TYPE>
static inline void AddTraceEvent(char phase,
                                 const unsigned char* category_enabled,
                                 const char* name,
                                 unsigned long long id,
                                 unsigned char flags,
                                 const char* arg1_name,
                                 const ARG1_TYPE& arg1_val) {
  const int num_args = 1;
  unsigned char arg_types[1];
  unsigned long long arg_values[1];
  SetTraceValue(arg1_val, &arg_types[0], &arg_values[0]);
  TRACE_EVENT_API_ADD_TRACE_EVENT(phase, category_enabled, name, id, num_args,
                                  &arg1_name, arg_types, arg_values, flags);
}

template <class ARG1_TYPE, class ARG2_TYPE>
static inline void AddTraceEvent(char phase,
                                 const unsigned char* category_enabled,
                                 const char* name,
                                 unsigned long long id,
                                 unsigned char flags,
                                 const char* arg1_name,
                                 const ARG1_TYPE& arg1_val,
                                 const char* arg2_name,
                                 const ARG2_TYPE& arg2_val) {
  const int num_args = 2;
  const char* arg_names[2] = {arg1_name, arg2_name};
  unsigned char arg_types[2];
  unsigned long long arg_values[2];
  SetTraceValue(arg1_val, &arg_types[0], &arg_values[0]);
  SetTraceValue(arg2_val, &arg_types[1], &arg_values[1]);
  TRACE_EVENT_API_ADD_TRACE_EVENT(phase, category_enabled, name, id, num_args,
                                  arg_names, arg_types, arg_values, flags);
}

// Used by TRACE_EVENTx macro. Do not use directly.
class TraceEndOnScopeClose {
 public:
  // Note: members of data_ intentionally left uninitialized. See Initialize.
  TraceEndOnScopeClose() : p_data_(nullptr) {}
  ~TraceEndOnScopeClose() {
    if (p_data_)
      AddEventIfEnabled();
  }

  void Initialize(const unsigned char* category_enabled, const char* name) {
    data_.category_enabled = category_enabled;
    data_.name = name;
    p_data_ = &data_;
  }

 private:
  // Add the end event if the category is still enabled.
  void AddEventIfEnabled() {
    // Only called when p_data_ is non-null.
    if (*p_data_->category_enabled) {
      TRACE_EVENT_API_ADD_TRACE_EVENT(TRACE_EVENT_PHASE_END,
                                      p_data_->category_enabled, p_data_->name,
                                      kNoEventId, kZeroNumArgs, nullptr,
                                      nullptr, nullptr, TRACE_EVENT_FLAG_NONE);
    }
  }

  // This Data struct workaround is to avoid initializing all the members
  // in Data during construction of this object, since this object is always
  // constructed, even when tracing is disabled. If the members of Data were
  // members of this class instead, compiler warnings occur about potential
  // uninitialized accesses.
  struct Data {
    const unsigned char* category_enabled;
    const char* name;
  };
  Data* p_data_;
  Data data_;
};

}  // namespace trace_event_internal
}  // namespace webrtc

#else

////////////////////////////////////////////////////////////////////////////////
// This section defines no-op alternatives to the tracing macros when
// RTC_DISABLE_TRACE_EVENTS is defined.

#define TRACE_DISABLED_BY_DEFAULT(name) "disabled-by-default-" name

#define TRACE_ID_MANGLE(id) 0

#define TRACE_EVENT0(category, name) RTC_NOOP()
#define TRACE_EVENT1(category, name, arg1_name, arg1_val) RTC_NOOP()
#define TRACE_EVENT2(category, name, arg1_name, arg1_val, arg2_name, arg2_val) \
  RTC_NOOP()

#define TRACE_EVENT_INSTANT0(category, name, scope) RTC_NOOP()
#define TRACE_EVENT_INSTANT1(category, name, scope, arg1_name, arg1_val) \
  RTC_NOOP()

#define TRACE_EVENT_INSTANT2(category, name, scope, arg1_name, arg1_val, \
                             arg2_name, arg2_val)                        \
  RTC_NOOP()

#define TRACE_EVENT_BEGIN0(category, name) RTC_NOOP()
#define TRACE_EVENT_BEGIN1(category, name, arg1_name, arg1_val) RTC_NOOP()
#define TRACE_EVENT_BEGIN2(category, name, arg1_name, arg1_val, arg2_name, \
                           arg2_val)                                       \
  RTC_NOOP()

#define TRACE_EVENT_END0(category, name) RTC_NOOP()
#define TRACE_EVENT_END1(category, name, arg1_name, arg1_val) RTC_NOOP()
#define TRACE_EVENT_END2(category, name, arg1_name, arg1_val, arg2_name, \
                         arg2_val)                                       \
  RTC_NOOP()

#define TRACE_COUNTER1(category, name, value) RTC_NOOP()

#define TRACE_COUNTER2(category, name, value1_name, value1_val, value2_name, \
                       value2_val)                                           \
  RTC_NOOP()

#define TRACE_COUNTER_ID1(category, name, id, value) RTC_NOOP()

#define TRACE_COUNTER_ID2(category, name, id, value1_name, value1_val, \
                          value2_name, value2_val)                     \
  RTC_NOOP()

#define TRACE_EVENT_ASYNC_BEGIN0(category, name, id) RTC_NOOP()
#define TRACE_EVENT_ASYNC_BEGIN1(category, name, id, arg1_name, arg1_val) \
  RTC_NOOP()
#define TRACE_EVENT_ASYNC_BEGIN2(category, name, id, arg1_name, arg1_val, \
                                 arg2_name, arg2_val)                     \
  RTC_NOOP()

#define TRACE_EVENT_ASYNC_STEP_INTO0(category, name, id, step) RTC_NOOP()
#define TRACE_EVENT_ASYNC_STEP_INTO1(category, name, id, step, arg1_name, \
                                     arg1_val)                            \
  RTC_NOOP()

#define TRACE_EVENT_ASYNC_END0(category, name, id) RTC_NOOP()
#define TRACE_EVENT_ASYNC_END1(category, name, id, arg1_name, arg1_val) \
  RTC_NOOP()
#define TRACE_EVENT_ASYNC_END2(category, name, id, arg1_name, arg1_val, \
                               arg2_name, arg2_val)                     \
  RTC_NOOP()

#define TRACE_EVENT_API_GET_CATEGORY_ENABLED ""

#define TRACE_EVENT_API_ADD_TRACE_EVENT RTC_NOOP()

#endif  // RTC_TRACE_EVENTS_ENABLED
#endif  // RTC_USE_PERFETTO

#endif  // RTC_BASE_TRACE_EVENT_H_
