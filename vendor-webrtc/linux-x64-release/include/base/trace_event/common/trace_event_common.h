// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TRACE_EVENT_COMMON_TRACE_EVENT_COMMON_H_
#define BASE_TRACE_EVENT_COMMON_TRACE_EVENT_COMMON_H_

// Trace events are for tracking application performance and resource usage.
// Macros are provided to track:
//    Begin and end of function calls
//    Counters
//
// Events are issued against categories. Whereas LOG's
// categories are statically defined, TRACE categories are created
// implicitly with a string. For example:
//   TRACE_EVENT_INSTANT0("MY_SUBSYSTEM", "SomeImportantEvent",
//                        TRACE_EVENT_SCOPE_THREAD)
//
// It is often the case that one trace may belong in multiple categories at the
// same time. The first argument to the trace can be a comma-separated list of
// categories, forming a category group, like:
//
// TRACE_EVENT_INSTANT0("input,views", "OnMouseOver", TRACE_EVENT_SCOPE_THREAD)
//
// We can enable/disable tracing of OnMouseOver by enabling/disabling either
// category.
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
// NESTABLE_ASYNC_BEGIN and NESTABLE_ASYNC_END:
//   [single threaded sender code]
//     static int send_count = 0;
//     ++send_count;
//     TRACE_EVENT_NESTABLE_ASYNC_BEGIN0(
//         "ipc", "message", TRACE_ID_WITH_SCOPE("message", send_count));
//     Send(new MyMessage(send_count));
//   [receive code]
//     void OnMyMessage(send_count) {
//       TRACE_NESTABLE_EVENT_ASYNC_END0(
//           "ipc", "message", TRACE_ID_WITH_SCOPE("message", send_count));
//     }
// The third parameter is a unique ID to match NESTABLE_ASYNC_BEGIN/ASYNC_END
// pairs. NESTABLE_ASYNC_BEGIN and ASYNC_END can occur on any thread of any
// traced process. // Pointers can be used for the ID parameter, and they will
// be annotated internally so that the same pointer on two different processes
// will not match. For example:
//   class MyTracedClass {
//    public:
//     MyTracedClass() {
//       TRACE_EVENT_NESTABLE_ASYNC_BEGIN0("category", "MyTracedClass",
//                                         TRACE_ID_LOCAL(this));
//     }
//     ~MyTracedClass() {
//       TRACE_EVENT_NESTABLE_ASYNC_END0("category", "MyTracedClass",
//                                       TRACE_ID_LOCAL(this));
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
// Since counters are in a global namespace, you may want to disambiguate with a
// unique ID, by using the TRACE_COUNTER_ID* variations.
//
// By default, trace collection is compiled in, but turned off at runtime.
// Collecting trace data is the responsibility of the embedding
// application. In Chrome's case, navigating to about:tracing will turn on
// tracing and display data collected across all active processes.
//
//
// Memory scoping note:
// Tracing copies the pointers, not the string content, of the strings passed
// in for category_group, name, and arg_names.  Thus, the following code will
// cause problems:
//     char* str = strdup("importantName");
//     TRACE_EVENT_INSTANT0("SUBSYSTEM", str);  // BAD!
//     free(str);                   // Trace system now has dangling pointer
//
// To avoid this issue with the |name| and |arg_name| parameters, use the
// TRACE_EVENT_COPY_XXX overloads of the macros at additional runtime overhead.
// Notes: The category must always be in a long-lived char* (i.e. static const).
//        The |arg_values|, when used, are always deep copied with the _COPY
//        macros.
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
// Convertable notes:
// Converting a large data type to a string can be costly. To help with this,
// the trace framework provides an interface ConvertableToTraceFormat. If you
// inherit from it and implement the AppendAsTraceFormat method the trace
// framework will call back to your object to convert a trace output time. This
// means, if the category for the event is disabled, the conversion will not
// happen.
//
//   class MyData : public base::trace_event::ConvertableToTraceFormat {
//    public:
//     MyData() {}
//
//     MyData(const MyData&) = delete;
//     MyData& operator=(const MyData&) = delete;
//
//     void AppendAsTraceFormat(std::string* out) const override {
//       out->append("{\"foo\":1}");
//     }
//    private:
//     ~MyData() override {}
//   };
//
//   TRACE_EVENT1("foo", "bar", "data",
//                std::unique_ptr<ConvertableToTraceFormat>(new MyData()));
//
// The trace framework will take ownership if the passed pointer and it will
// be free'd when the trace buffer is flushed.
//
// Note, we only do the conversion when the buffer is flushed, so the provided
// data object should not be modified after it's passed to the trace framework.
//
//
// Thread Safety:
// A thread safe singleton and mutex are used for thread safety. Category
// enabled flags are used to limit the performance impact when the system
// is not enabled.
//
// TRACE_EVENT macros first cache a pointer to a category. The categories are
// statically allocated and safe at all times, even after exit. Fetching a
// category is protected by the TraceLog::lock_. Multiple threads initializing
// the static variable is safe, as they will be serialized by the lock and
// multiple calls will return the same pointer to the category.
//
// Then the category_group_enabled flag is checked. This is a unsigned char, and
// not intended to be multithread safe. It optimizes access to AddTraceEvent
// which is threadsafe internally via TraceLog::lock_. The enabled flag may
// cause some threads to incorrectly call or skip calling AddTraceEvent near
// the time of the system being enabled or disabled. This is acceptable as
// we tolerate some data loss while the system is being enabled/disabled and
// because AddTraceEvent is threadsafe internally and checks the enabled state
// again under lock.
//
// Without the use of these static category pointers and enabled flags all
// trace points would carry a significant performance cost of acquiring a lock
// and resolving the category.

////////////////////////////////////////////////////////////////////////////////
// Perfetto trace macros

#include "base/threading/platform_thread.h"
#include "base/time/time.h"
#include "build/build_config.h"

#if BUILDFLAG(ENABLE_BASE_TRACING)

// Enable legacy trace event macros (e.g., TRACE_EVENT{0,1,2}).
#define PERFETTO_ENABLE_LEGACY_TRACE_EVENTS 1

// Macros for reading the current trace time (bypassing any virtual time
// overrides).
#define TRACE_TIME_TICKS_NOW() ::base::subtle::TimeTicksNowIgnoringOverride()
#define TRACE_TIME_NOW() ::base::subtle::TimeNowIgnoringOverride()

// Implementation detail: trace event macros create temporary variables
// to keep instrumentation overhead low. These macros give each temporary
// variable a unique name based on the line number to prevent name collisions.
#define INTERNAL_TRACE_EVENT_UID(name_prefix) PERFETTO_UID(name_prefix)

// Declare debug annotation converters for base time types, so they can be
// passed as trace event arguments.
// TODO(skyostil): Serialize timestamps using perfetto::TracedValue instead.
namespace perfetto {
namespace protos {
namespace pbzero {
class DebugAnnotation;
}  // namespace pbzero
}  // namespace protos
namespace internal {

void BASE_EXPORT
WriteDebugAnnotation(protos::pbzero::DebugAnnotation* annotation,
                     ::base::TimeTicks);
void BASE_EXPORT
WriteDebugAnnotation(protos::pbzero::DebugAnnotation* annotation, ::base::Time);

}  // namespace internal
}  // namespace perfetto

// Pull in the tracing macro definitions from Perfetto.
#include "third_party/perfetto/include/perfetto/tracing/track_event.h"  // IWYU pragma: export
#include "third_party/perfetto/include/perfetto/tracing/track_event_legacy.h"  // IWYU pragma: export

namespace perfetto {
namespace legacy {

template <>
perfetto::ThreadTrack BASE_EXPORT
ConvertThreadId(const ::base::PlatformThreadId& thread);

#if BUILDFLAG(IS_WIN)
template <>
perfetto::ThreadTrack BASE_EXPORT ConvertThreadId(const int& thread);
#endif  // BUILDFLAG(IS_WIN)

}  // namespace legacy

template <>
struct BASE_EXPORT TraceTimestampTraits<::base::TimeTicks> {
  static TraceTimestamp ConvertTimestampToTraceTimeNs(
      const ::base::TimeTicks& ticks);
};

}  // namespace perfetto

#else  // !BUILDFLAG(ENABLE_BASE_TRACING)

// This macro is still used in some components even when base tracing is
// disabled.
// TODO(crbug/336718643): Make sure no code affected by
// enable_base_tracing=false includes this file directly, then move the define
// to trace_event_stub.h.
#define TRACE_DISABLED_BY_DEFAULT(name) "disabled-by-default-" name

#endif  // !BUILDFLAG(ENABLE_BASE_TRACING)

#endif  // BASE_TRACE_EVENT_COMMON_TRACE_EVENT_COMMON_H_
