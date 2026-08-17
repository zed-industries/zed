/*
 * Copyright (C) 2019 The Android Open Source Project
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

#ifndef INCLUDE_PERFETTO_TRACING_INTERNAL_TRACK_EVENT_MACROS_H_
#define INCLUDE_PERFETTO_TRACING_INTERNAL_TRACK_EVENT_MACROS_H_

// This file contains underlying macros for the trace point track event
// implementation. Perfetto API users typically don't need to use anything here
// directly.

#include "perfetto/base/thread_annotations.h"
#include "perfetto/tracing/internal/track_event_data_source.h"
#include "perfetto/tracing/string_helpers.h"
#include "perfetto/tracing/track_event_category_registry.h"

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

// Defines data structures for backing a category registry.
//
// Each category has one enabled/disabled bit per possible data source instance.
// The bits are packed, i.e., each byte holds the state for instances. To
// improve cache locality, the bits for each instance are stored separately from
// the names of the categories:
//
//   byte 0                      byte 1
//   (inst0, inst1, ..., inst7), (inst0, inst1, ..., inst7)
//
#define PERFETTO_INTERNAL_DECLARE_CATEGORIES(attrs, ...)                      \
  namespace internal {                                                        \
  constexpr ::perfetto::Category kCategories[] = {__VA_ARGS__};               \
  constexpr size_t kCategoryCount =                                           \
      sizeof(kCategories) / sizeof(kCategories[0]);                           \
  /* The per-instance enable/disable state per category */                    \
  attrs extern std::atomic<uint8_t> g_category_state_storage[kCategoryCount]; \
  /* The category registry which mediates access to the above structures. */  \
  /* The registry is used for two purposes: */                                \
  /**/                                                                        \
  /*    1) For looking up categories at build (constexpr) time. */            \
  /*    2) For declaring the per-namespace TrackEvent data source. */         \
  /**/                                                                        \
  /* Because usage #1 requires a constexpr type and usage #2 requires an */   \
  /* extern type (to avoid declaring a type based on a translation-unit */    \
  /* variable), we need two separate copies of the registry with different */ \
  /* storage specifiers. */                                                   \
  /**/                                                                        \
  /* Note that because of a Clang/Windows bug, the constexpr category */      \
  /* registry isn't given the enabled/disabled state array. All access */     \
  /* to the category states should therefore be done through the */           \
  /* non-constexpr registry. See */                                           \
  /* https://bugs.llvm.org/show_bug.cgi?id=51558 */                           \
  /**/                                                                        \
  /* TODO(skyostil): Unify these using a C++17 inline constexpr variable. */  \
  constexpr ::perfetto::internal::TrackEventCategoryRegistry                  \
      kConstExprCategoryRegistry(kCategoryCount, &kCategories[0], nullptr);   \
  attrs extern const ::perfetto::internal::TrackEventCategoryRegistry         \
      kCategoryRegistry;                                                      \
  static_assert(kConstExprCategoryRegistry.ValidateCategories(),              \
                "Invalid category names found");                              \
  }  // namespace internal

// In a .cc file, declares storage for each category's runtime state.
#define PERFETTO_INTERNAL_CATEGORY_STORAGE(attrs)                      \
  namespace internal {                                                 \
  attrs std::atomic<uint8_t> g_category_state_storage[kCategoryCount]; \
  attrs const ::perfetto::internal::TrackEventCategoryRegistry         \
      kCategoryRegistry(kCategoryCount,                                \
                        &kCategories[0],                               \
                        &g_category_state_storage[0]);                 \
  }  // namespace internal

// Defines the TrackEvent data source for the current track event namespace.
// `virtual ~TrackEvent` is added to avoid `-Wweak-vtables` warning.
// Learn more : aosp/2019906
#define PERFETTO_INTERNAL_DECLARE_TRACK_EVENT_DATA_SOURCE(attrs)               \
  struct attrs TrackEvent : public ::perfetto::internal::TrackEventDataSource< \
                                TrackEvent, &internal::kCategoryRegistry> {    \
    virtual ~TrackEvent();                                                     \
  }

#define PERFETTO_INTERNAL_DEFINE_TRACK_EVENT_DATA_SOURCE() \
  TrackEvent::~TrackEvent() = default;

// At compile time, turns a category name represented by a static string into an
// index into the current category registry. A build error will be generated if
// the category hasn't been registered or added to the list of allowed dynamic
// categories. See PERFETTO_DEFINE_CATEGORIES.
#define PERFETTO_GET_CATEGORY_INDEX(category)                                \
  PERFETTO_TRACK_EVENT_NAMESPACE::internal::kConstExprCategoryRegistry.Find( \
      category,                                                              \
      ::PERFETTO_TRACK_EVENT_NAMESPACE::internal::IsDynamicCategory(category))

// Generate a unique variable name with a given prefix.
#define PERFETTO_INTERNAL_CONCAT2(a, b) a##b
#define PERFETTO_INTERNAL_CONCAT(a, b) PERFETTO_INTERNAL_CONCAT2(a, b)
#define PERFETTO_UID(prefix) PERFETTO_INTERNAL_CONCAT(prefix, __LINE__)

#if PERFETTO_BUILDFLAG(PERFETTO_COMPILER_MSVC)
// MSVC with /permissive- fails to build without this. Probably a compiler bug.
#define PERFETTO_INTERNAL_STATIC_FOR_MSVC static
#else
// On the other hand, if we add static with clang, binary size of the chromium
// build will increase dramatically.
#define PERFETTO_INTERNAL_STATIC_FOR_MSVC
#endif

// Efficiently determines whether tracing is enabled for the given category, and
// if so, emits one trace event with the given arguments.
#define PERFETTO_INTERNAL_TRACK_EVENT_WITH_METHOD(method, category, name, ...) \
  do {                                                                         \
    ::perfetto::internal::ValidateEventNameType<decltype(name)>();             \
    namespace tns = PERFETTO_TRACK_EVENT_NAMESPACE;                            \
    /* Compute the category index outside the lambda to work around a */       \
    /* GCC 7 bug */                                                            \
    PERFETTO_INTERNAL_STATIC_FOR_MSVC constexpr auto PERFETTO_UID(             \
        kCatIndex_ADD_TO_PERFETTO_DEFINE_CATEGORIES_IF_FAILS_) =               \
        PERFETTO_GET_CATEGORY_INDEX(category);                                 \
    if (::PERFETTO_TRACK_EVENT_NAMESPACE::internal::IsDynamicCategory(         \
            category)) {                                                       \
      tns::TrackEvent::CallIfEnabled(                                          \
          [&](uint32_t instances) PERFETTO_NO_THREAD_SAFETY_ANALYSIS {         \
            tns::TrackEvent::method(                                           \
                instances, category,                                           \
                ::perfetto::internal::DecayEventNameType(name),                \
                ##__VA_ARGS__);                                                \
          });                                                                  \
    } else {                                                                   \
      tns::TrackEvent::CallIfCategoryEnabled(                                  \
          PERFETTO_UID(kCatIndex_ADD_TO_PERFETTO_DEFINE_CATEGORIES_IF_FAILS_), \
          [&](uint32_t instances) PERFETTO_NO_THREAD_SAFETY_ANALYSIS {         \
            tns::TrackEvent::method(                                           \
                instances,                                                     \
                PERFETTO_UID(                                                  \
                    kCatIndex_ADD_TO_PERFETTO_DEFINE_CATEGORIES_IF_FAILS_),    \
                ::perfetto::internal::DecayEventNameType(name),                \
                ##__VA_ARGS__);                                                \
          });                                                                  \
    }                                                                          \
  } while (false)

// C++17 doesn't like a move constructor being defined for the EventFinalizer
// class but C++11 and MSVC doesn't compile without it being defined so support
// both.
#if !PERFETTO_BUILDFLAG(PERFETTO_COMPILER_MSVC)
#define PERFETTO_INTERNAL_EVENT_FINALIZER_KEYWORD delete
#else
#define PERFETTO_INTERNAL_EVENT_FINALIZER_KEYWORD default
#endif

#define PERFETTO_INTERNAL_SCOPED_EVENT_FINALIZER(category)                    \
  struct PERFETTO_UID(ScopedEvent) {                                          \
    struct EventFinalizer {                                                   \
      /* The parameter is an implementation detail. It allows the          */ \
      /* anonymous struct to use aggregate initialization to invoke the    */ \
      /* lambda (which emits the BEGIN event and returns an integer)       */ \
      /* with the proper reference capture for any                         */ \
      /* TrackEventArgumentFunction in |__VA_ARGS__|. This is required so  */ \
      /* that the scoped event is exactly ONE line and can't escape the    */ \
      /* scope if used in a single line if statement.                      */ \
      EventFinalizer(...) {}                                                  \
      ~EventFinalizer() {                                                     \
        TRACE_EVENT_END(category);                                            \
      }                                                                       \
                                                                              \
      EventFinalizer(const EventFinalizer&) = delete;                         \
      inline EventFinalizer& operator=(const EventFinalizer&) = delete;       \
                                                                              \
      EventFinalizer(EventFinalizer&&) =                                      \
          PERFETTO_INTERNAL_EVENT_FINALIZER_KEYWORD;                          \
      EventFinalizer& operator=(EventFinalizer&&) = delete;                   \
    } finalizer;                                                              \
  }

#define PERFETTO_INTERNAL_SCOPED_TRACK_EVENT(category, name, ...) \
  PERFETTO_INTERNAL_SCOPED_EVENT_FINALIZER(category)              \
  PERFETTO_UID(scoped_event) {                                    \
    [&]() {                                                       \
      TRACE_EVENT_BEGIN(category, name, ##__VA_ARGS__);           \
      return 0;                                                   \
    }()                                                           \
  }

#if PERFETTO_ENABLE_LEGACY_TRACE_EVENTS
// Required for TRACE_EVENT_WITH_FLOW legacy macros, which pass the bind_id as
// id.
#define PERFETTO_INTERNAL_SCOPED_LEGACY_TRACK_EVENT_WITH_ID(               \
    category, name, track, flags, thread_id, id, ...)                      \
  PERFETTO_INTERNAL_SCOPED_EVENT_FINALIZER(category)                       \
  PERFETTO_UID(scoped_event) {                                             \
    [&]() {                                                                \
      PERFETTO_INTERNAL_TRACK_EVENT_WITH_METHOD(                           \
          TraceForCategoryLegacyWithId, category, name,                    \
          ::perfetto::protos::pbzero::TrackEvent::TYPE_SLICE_BEGIN, track, \
          'B', flags, thread_id, id, ##__VA_ARGS__);                       \
      return 0;                                                            \
    }()                                                                    \
  }
#endif  // PERFETTO_ENABLE_LEGACY_TRACE_EVENTS

#if PERFETTO_BUILDFLAG(PERFETTO_COMPILER_GCC) || \
    PERFETTO_BUILDFLAG(PERFETTO_COMPILER_MSVC)
// On GCC versions <9 there's a bug that prevents using captured constant
// variables in constexpr evaluation inside a lambda:
// https://gcc.gnu.org/bugzilla/show_bug.cgi?id=82643
// TODO(khokhlov): Remove this fallback after Perfetto moves to a more recent
// GCC version.
#define PERFETTO_INTERNAL_CATEGORY_ENABLED(category)                           \
  (::PERFETTO_TRACK_EVENT_NAMESPACE::internal::IsDynamicCategory(category)     \
       ? PERFETTO_TRACK_EVENT_NAMESPACE::TrackEvent::IsDynamicCategoryEnabled( \
             ::perfetto::DynamicCategory(category))                            \
       : PERFETTO_TRACK_EVENT_NAMESPACE::TrackEvent::IsCategoryEnabled(        \
             PERFETTO_GET_CATEGORY_INDEX(category)))
#else  // !PERFETTO_BUILDFLAG(PERFETTO_COMPILER_GCC)
#define PERFETTO_INTERNAL_CATEGORY_ENABLED(category)                     \
  [&]() -> bool {                                                        \
    using PERFETTO_TRACK_EVENT_NAMESPACE::TrackEvent;                    \
    using ::PERFETTO_TRACK_EVENT_NAMESPACE::internal::IsDynamicCategory; \
    constexpr auto PERFETTO_UID(index) =                                 \
        PERFETTO_GET_CATEGORY_INDEX(category);                           \
    constexpr auto PERFETTO_UID(dynamic) = IsDynamicCategory(category);  \
    return PERFETTO_UID(dynamic)                                         \
               ? TrackEvent::IsDynamicCategoryEnabled(                   \
                     ::perfetto::DynamicCategory(category))              \
               : TrackEvent::IsCategoryEnabled(PERFETTO_UID(index));     \
  }()
#endif  // !PERFETTO_BUILDFLAG(PERFETTO_COMPILER_GCC)

// Emits an empty trace packet into the trace to ensure that the service can
// safely read the last event from the trace buffer. This can be used to
// periodically "flush" the last event on threads that don't support explicit
// flushing of the shared memory buffer chunk when the tracing session stops
// (e.g. thread pool workers in Chromium).
//
// This workaround is only required because the tracing service cannot safely
// read the last trace packet from an incomplete SMB chunk (crbug.com/1021571
// and b/162206162) when scraping the SMB. Adding an empty trace packet ensures
// that all prior events can be scraped by the service.
#define PERFETTO_INTERNAL_ADD_EMPTY_EVENT()                                \
  do {                                                                     \
    PERFETTO_TRACK_EVENT_NAMESPACE::TrackEvent::Trace(                     \
        [](PERFETTO_TRACK_EVENT_NAMESPACE::TrackEvent::TraceContext ctx) { \
          ctx.AddEmptyTracePacket();                                       \
        });                                                                \
  } while (false)

#endif  // INCLUDE_PERFETTO_TRACING_INTERNAL_TRACK_EVENT_MACROS_H_
