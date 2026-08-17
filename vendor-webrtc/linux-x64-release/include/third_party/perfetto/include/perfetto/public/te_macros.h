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

#ifndef INCLUDE_PERFETTO_PUBLIC_TE_MACROS_H_
#define INCLUDE_PERFETTO_PUBLIC_TE_MACROS_H_

#include <assert.h>

#ifdef __cplusplus
#include <initializer_list>
#endif

#include "perfetto/public/abi/track_event_hl_abi.h"
#include "perfetto/public/pb_utils.h"
#include "perfetto/public/track_event.h"

// This header defines the PERFETTO_TE macros and its possible params (at the
// end of the file). The rest of the file contains internal implementation
// details of the macros, which are subject to change at any time.
//
// The macro uses the High level ABI to emit track events.

#define PERFETTO_I_TE_HL_MACRO_PARAMS__(NAME_AND_TYPE, ...)           \
  NAME_AND_TYPE.type, NAME_AND_TYPE.name,                             \
      PERFETTO_I_TE_COMPOUND_LITERAL_ARRAY(struct PerfettoTeHlExtra*, \
                                           {__VA_ARGS__})

// Level of indirection for MSVC traditional preprocessor.
#define PERFETTO_I_TE_HL_MACRO_PARAMS_(MACRO, ARGS) MACRO ARGS

// Provides an initializer for `struct PerfettoTeHlMacroParams` and sets all the
// unused extra fields to PERFETTO_NULL.
#define PERFETTO_I_TE_HL_MACRO_PARAMS(...)                        \
  PERFETTO_I_TE_HL_MACRO_PARAMS_(PERFETTO_I_TE_HL_MACRO_PARAMS__, \
                                 (__VA_ARGS__, PERFETTO_NULL))

// Implementation of the PERFETTO_TE macro. If `CAT` is enabled, emits the
// tracing event specified by the params.
//
// Uses `?:` instead of `if` because this might be used as an expression, where
// statements are not allowed.
#define PERFETTO_I_TE_IMPL(CAT, ...)                                        \
  ((PERFETTO_UNLIKELY(PERFETTO_ATOMIC_LOAD_EXPLICIT(                        \
       (CAT).enabled, PERFETTO_MEMORY_ORDER_RELAXED)))                      \
       ? (PerfettoTeHlEmitImpl((CAT).impl,                                  \
                               PERFETTO_I_TE_HL_MACRO_PARAMS(__VA_ARGS__)), \
          0)                                                                \
       : 0)

#ifndef __cplusplus
#define PERFETTO_I_TE_COMPOUND_LITERAL(STRUCT, ...) (struct STRUCT) __VA_ARGS__
#define PERFETTO_I_TE_COMPOUND_LITERAL_ADDR(STRUCT, ...) \
  &(struct STRUCT)__VA_ARGS__
#define PERFETTO_I_TE_COMPOUND_LITERAL_ARRAY(TYPE, ...) (TYPE[]) __VA_ARGS__
#define PERFETTO_I_TE_EXTRA(STRUCT, ...)                           \
  ((struct PerfettoTeHlExtra*)PERFETTO_I_TE_COMPOUND_LITERAL_ADDR( \
      STRUCT, __VA_ARGS__))
#else
#define PERFETTO_I_TE_COMPOUND_LITERAL(STRUCT, ...) STRUCT __VA_ARGS__
#define PERFETTO_I_TE_COMPOUND_LITERAL_ADDR(STRUCT, ...) \
  &(STRUCT{} = STRUCT __VA_ARGS__)
#define PERFETTO_I_TE_COMPOUND_LITERAL_ARRAY(TYPE, ...) \
  static_cast<TYPE const*>((std::initializer_list<TYPE> __VA_ARGS__).begin())
#define PERFETTO_I_TE_EXTRA(STRUCT, ...)       \
  reinterpret_cast<struct PerfettoTeHlExtra*>( \
      PERFETTO_I_TE_COMPOUND_LITERAL_ADDR(STRUCT, __VA_ARGS__))
#endif

#define PERFETTO_I_TE_HL_MACRO_NAME_AND_TYPE(NAME, TYPE) \
  (PERFETTO_I_TE_COMPOUND_LITERAL(PerfettoTeHlMacroNameAndType, {NAME, TYPE}))

#define PERFETTO_I_TE_CONCAT2(a, b) a##b
#define PERFETTO_I_TE_CONCAT(a, b) PERFETTO_I_TE_CONCAT2(a, b)
// Generate a unique name with a given prefix.
#define PERFETTO_I_TE_UID(prefix) PERFETTO_I_TE_CONCAT(prefix, __LINE__)

struct PerfettoTeHlMacroNameAndType {
  const char* name;
  int32_t type;
};

// Instead of a previously registered category, this macro can be used to
// specify that the category will be provided dynamically as a param.
#define PERFETTO_TE_DYNAMIC_CATEGORY PerfettoTeRegisteredDynamicCategory()

// ---------------------------------------------------------------
// Possible types of fields for the PERFETTO_TE_PROTO_FIELDS macro
// ---------------------------------------------------------------

// A string or bytes protobuf field (with field id `ID`) and value `VAL` (a null
// terminated string).
#define PERFETTO_TE_PROTO_FIELD_CSTR(ID, VAL)                    \
  PERFETTO_REINTERPRET_CAST(struct PerfettoTeHlProtoField*,      \
                            PERFETTO_I_TE_COMPOUND_LITERAL_ADDR( \
                                PerfettoTeHlProtoFieldCstr,      \
                                {{PERFETTO_TE_HL_PROTO_TYPE_CSTR, ID}, VAL}))

// A string or bytes protobuf field (with field id `ID`) with a `SIZE` long
// value starting from `VAL`.
#define PERFETTO_TE_PROTO_FIELD_BYTES(ID, VAL, SIZE) \
  PERFETTO_REINTERPRET_CAST(                         \
      struct PerfettoTeHlProtoField*,                \
      PERFETTO_I_TE_COMPOUND_LITERAL_ADDR(           \
          PerfettoTeHlProtoFieldBytes,               \
          {{PERFETTO_TE_HL_PROTO_TYPE_BYTES, ID}, VAL, SIZE}))

// An varint protobuf field (with field id `ID`) and value `VAL`.
#define PERFETTO_TE_PROTO_FIELD_VARINT(ID, VAL)                          \
  PERFETTO_REINTERPRET_CAST(struct PerfettoTeHlProtoField*,              \
                            PERFETTO_I_TE_COMPOUND_LITERAL_ADDR(         \
                                PerfettoTeHlProtoFieldVarInt,            \
                                {{PERFETTO_TE_HL_PROTO_TYPE_VARINT, ID}, \
                                 PERFETTO_STATIC_CAST(uint64_t, VAL)}))

// An zigzag (sint*) protobuf field (with field id `ID`) and value `VAL`.
#define PERFETTO_TE_PROTO_FIELD_ZIGZAG(ID, VAL)    \
  PERFETTO_REINTERPRET_CAST(                       \
      struct PerfettoTeHlProtoField*,              \
      PERFETTO_I_TE_COMPOUND_LITERAL_ADDR(         \
          PerfettoTeHlProtoFieldVarInt,            \
          {{PERFETTO_TE_HL_PROTO_TYPE_VARINT, ID}, \
           PerfettoPbZigZagEncode64(PERFETTO_STATIC_CAST(int64_t, VAL))}))

// A fixed64 protobuf field (with field id `ID`) and value `VAL`.
#define PERFETTO_TE_PROTO_FIELD_FIXED64(ID, VAL)                          \
  PERFETTO_REINTERPRET_CAST(struct PerfettoTeHlProtoField*,               \
                            PERFETTO_I_TE_COMPOUND_LITERAL_ADDR(          \
                                PerfettoTeHlProtoFieldFixed64,            \
                                {{PERFETTO_TE_HL_PROTO_TYPE_FIXED64, ID}, \
                                 PERFETTO_STATIC_CAST(uint64_t, VAL)}))

// A fixed32 protobuf field (with field id `ID`) and value `VAL`.
#define PERFETTO_TE_PROTO_FIELD_FIXED32(ID, VAL)                          \
  PERFETTO_REINTERPRET_CAST(struct PerfettoTeHlProtoField*,               \
                            PERFETTO_I_TE_COMPOUND_LITERAL_ADDR(          \
                                PerfettoTeHlProtoFieldFixed32,            \
                                {{PERFETTO_TE_HL_PROTO_TYPE_FIXED32, ID}, \
                                 PERFETTO_STATIC_CAST(uint32_t, VAL)}))

// A double protobuf field (with field id `ID`) and value `VAL`.
#define PERFETTO_TE_PROTO_FIELD_DOUBLE(ID, VAL)                          \
  PERFETTO_REINTERPRET_CAST(struct PerfettoTeHlProtoField*,              \
                            PERFETTO_I_TE_COMPOUND_LITERAL_ADDR(         \
                                PerfettoTeHlProtoFieldDouble,            \
                                {{PERFETTO_TE_HL_PROTO_TYPE_DOUBLE, ID}, \
                                 PERFETTO_STATIC_CAST(double, VAL)}))

// A float protobuf field (with field id `ID`) and value `VAL`.
#define PERFETTO_TE_PROTO_FIELD_FLOAT(ID, VAL)                                 \
  PERFETTO_REINTERPRET_CAST(                                                   \
      struct PerfettoTeHlProtoField*,                                          \
      PERFETTO_I_TE_COMPOUND_LITERAL_ADDR(                                     \
          PerfettoTeHlProtoFieldFloat, {{PERFETTO_TE_HL_PROTO_TYPE_FLOAT, ID}, \
                                        PERFETTO_STATIC_CAST(float, VAL)}))

// A nested message protobuf field (with field id `ID`). The rest of the
// argument can be PERFETTO_TE_PROTO_FIELD_*.
#define PERFETTO_TE_PROTO_FIELD_NESTED(ID, ...)                          \
  PERFETTO_REINTERPRET_CAST(struct PerfettoTeHlProtoField*,              \
                            PERFETTO_I_TE_COMPOUND_LITERAL_ADDR(         \
                                PerfettoTeHlProtoFieldNested,            \
                                {{PERFETTO_TE_HL_PROTO_TYPE_NESTED, ID}, \
                                 PERFETTO_I_TE_COMPOUND_LITERAL_ARRAY(   \
                                     struct PerfettoTeHlProtoField*,     \
                                     {__VA_ARGS__, PERFETTO_NULL})}))

// -------------------------------------------------
// Possible types of event for the PERFETTO_TE macro
// -------------------------------------------------

// Begins a slice named `const char* NAME` on a track.
#define PERFETTO_TE_SLICE_BEGIN(NAME) \
  PERFETTO_I_TE_HL_MACRO_NAME_AND_TYPE(NAME, PERFETTO_TE_TYPE_SLICE_BEGIN)

// Ends the last slice opened on a track.
#define PERFETTO_TE_SLICE_END()                       \
  PERFETTO_I_TE_HL_MACRO_NAME_AND_TYPE(PERFETTO_NULL, \
                                       PERFETTO_TE_TYPE_SLICE_END)

// Reports an instant event named `const char* NAME`.
#define PERFETTO_TE_INSTANT(NAME) \
  PERFETTO_I_TE_HL_MACRO_NAME_AND_TYPE(NAME, PERFETTO_TE_TYPE_INSTANT)

// Reports the value of a counter. The counter value must be specified
// separately on another param with PERFETTO_TE_INT_COUNTER() or
// PERFETTO_TE_DOUBLE_COUNTER().
#define PERFETTO_TE_COUNTER() \
  PERFETTO_I_TE_HL_MACRO_NAME_AND_TYPE(PERFETTO_NULL, PERFETTO_TE_TYPE_COUNTER)

// -----------------------------------------------------------
// Possible types of extra arguments for the PERFETTO_TE macro
// -----------------------------------------------------------

// The value (`C`) of an integer counter. A separate parameter must describe the
// counter track this refers to. This should only be used for events with
// type PERFETTO_TE_COUNTER().
#define PERFETTO_TE_INT_COUNTER(C)                   \
  PERFETTO_I_TE_EXTRA(PerfettoTeHlExtraCounterInt64, \
                      {{PERFETTO_TE_HL_EXTRA_TYPE_COUNTER_INT64}, C})

// The value (`C`) of a floating point. A separate parameter must describe the
// counter track this refers to. This should only be used for events with type
// PERFETTO_TE_COUNTER().
#define PERFETTO_TE_DOUBLE_COUNTER(C)                 \
  PERFETTO_I_TE_EXTRA(PerfettoTeHlExtraCounterDouble, \
                      {{PERFETTO_TE_HL_EXTRA_TYPE_COUNTER_DOUBLE}, C})

// Uses the timestamp `struct PerfettoTeTimestamp T` to report this event. If
// this is not specified, PERFETTO_TE() reads the current timestamp with
// PerfettoTeGetTimestamp().
#define PERFETTO_TE_TIMESTAMP(T)                  \
  PERFETTO_I_TE_EXTRA(PerfettoTeHlExtraTimestamp, \
                      {{PERFETTO_TE_HL_EXTRA_TYPE_TIMESTAMP}, T})

// Specifies that the current track for this event is
// `struct PerfettoTeRegisteredTrack* T`, which must have been previously
// registered.
#define PERFETTO_TE_REGISTERED_TRACK(T) \
  PERFETTO_I_TE_EXTRA(                  \
      PerfettoTeHlExtraRegisteredTrack, \
      {{PERFETTO_TE_HL_EXTRA_TYPE_REGISTERED_TRACK}, &(T)->impl})

// Specifies that the current track for this event is a track named `const char
// *NAME`, child of a track whose uuid is `PARENT_UUID`. `NAME`, `uint64_t ID`
// and `PARENT_UUID` uniquely identify a track. Common values for `PARENT_UUID`
// include PerfettoTeProcessTrackUuid(), PerfettoTeThreadTrackUuid() or
// PerfettoTeGlobalTrackUuid().
#define PERFETTO_TE_NAMED_TRACK(NAME, ID, PARENT_UUID) \
  PERFETTO_I_TE_EXTRA(                                 \
      PerfettoTeHlExtraNamedTrack,                     \
      {{PERFETTO_TE_HL_EXTRA_TYPE_NAMED_TRACK}, NAME, ID, PARENT_UUID})

// When PERFETTO_TE_DYNAMIC_CATEGORY is used, this is used to specify `const
// char* S` as a category name.
#define PERFETTO_TE_DYNAMIC_CATEGORY_STRING(S)                       \
  PERFETTO_I_TE_EXTRA(PerfettoTeHlExtraDynamicCategory,              \
                      {{PERFETTO_TE_HL_EXTRA_TYPE_DYNAMIC_CATEGORY}, \
                       PERFETTO_I_TE_COMPOUND_LITERAL_ADDR(          \
                           PerfettoTeCategoryDescriptor,             \
                           {S, PERFETTO_NULL, PERFETTO_NULL, 0})})

// Adds the debug annotation named `const char * NAME` with value `bool VALUE`.
#define PERFETTO_TE_ARG_BOOL(NAME, VALUE) \
  PERFETTO_I_TE_EXTRA(                    \
      PerfettoTeHlExtraDebugArgBool,      \
      {{PERFETTO_TE_HL_EXTRA_TYPE_DEBUG_ARG_BOOL}, NAME, VALUE})

// Adds the debug annotation named `const char * NAME` with value `uint64_t
// VALUE`.
#define PERFETTO_TE_ARG_UINT64(NAME, VALUE) \
  PERFETTO_I_TE_EXTRA(                      \
      PerfettoTeHlExtraDebugArgUint64,      \
      {{PERFETTO_TE_HL_EXTRA_TYPE_DEBUG_ARG_UINT64}, NAME, VALUE})

// Adds the debug annotation named `const char * NAME` with value `int64_t
// VALUE`.
#define PERFETTO_TE_ARG_INT64(NAME, VALUE) \
  PERFETTO_I_TE_EXTRA(                     \
      PerfettoTeHlExtraDebugArgInt64,      \
      {{PERFETTO_TE_HL_EXTRA_TYPE_DEBUG_ARG_INT64}, NAME, VALUE})

// Adds the debug annotation named `const char * NAME` with value `double
// VALUE`.
#define PERFETTO_TE_ARG_DOUBLE(NAME, VALUE) \
  PERFETTO_I_TE_EXTRA(                      \
      PerfettoTeHlExtraDebugArgDouble,      \
      {{PERFETTO_TE_HL_EXTRA_TYPE_DEBUG_ARG_DOUBLE}, NAME, VALUE})

// Adds the debug annotation named `const char * NAME` with value `const char*
// VALUE`.
#define PERFETTO_TE_ARG_STRING(NAME, VALUE) \
  PERFETTO_I_TE_EXTRA(                      \
      PerfettoTeHlExtraDebugArgString,      \
      {{PERFETTO_TE_HL_EXTRA_TYPE_DEBUG_ARG_STRING}, NAME, VALUE})

// Adds the debug annotation named `const char * NAME` with value `void* VALUE`.
#define PERFETTO_TE_ARG_POINTER(NAME, VALUE) \
  PERFETTO_I_TE_EXTRA(                       \
      PerfettoTeHlExtraDebugArgPointer,      \
      {{PERFETTO_TE_HL_EXTRA_TYPE_DEBUG_ARG_POINTER}, NAME, VALUE})

// Specifies that this event is part (or starts) a "flow" (i.e. a link among
// different events). The flow is identified by `struct PerfettoTeFlow VALUE`.
#define PERFETTO_TE_FLOW(VALUE)              \
  PERFETTO_I_TE_EXTRA(PerfettoTeHlExtraFlow, \
                      {{PERFETTO_TE_HL_EXTRA_TYPE_FLOW}, (VALUE).id})

// Specifies that this event terminates a "flow" (i.e. a link among different
// events). The flow is identified by `struct PerfettoTeFlow VALUE`.
#define PERFETTO_TE_TERMINATING_FLOW(VALUE) \
  PERFETTO_I_TE_EXTRA(                      \
      PerfettoTeHlExtraFlow,                \
      {{PERFETTO_TE_HL_EXTRA_TYPE_TERMINATING_FLOW}, (VALUE).id})

// Flushes the shared memory buffer and makes sure that all the previous events
// emitted by this thread are visibile in the central tracing buffer.
#define PERFETTO_TE_FLUSH() \
  PERFETTO_I_TE_EXTRA(PerfettoTeHlExtra, {PERFETTO_TE_HL_EXTRA_TYPE_FLUSH})

// Turns off interning for event names.
#define PERFETTO_TE_NO_INTERN() \
  PERFETTO_I_TE_EXTRA(PerfettoTeHlExtra, {PERFETTO_TE_HL_EXTRA_TYPE_NO_INTERN})

// Adds some proto fields to the event. The arguments should use the
// PERFETTO_TE_PROTO_FIELD_* macros and should be fields of the
// perfetto.protos.TrackEvent protobuf message.
#define PERFETTO_TE_PROTO_FIELDS(...)                                       \
  PERFETTO_I_TE_EXTRA(                                                      \
      PerfettoTeHlExtraProtoFields,                                         \
      {{PERFETTO_TE_HL_EXTRA_TYPE_PROTO_FIELDS},                            \
       PERFETTO_I_TE_COMPOUND_LITERAL_ARRAY(struct PerfettoTeHlProtoField*, \
                                            {__VA_ARGS__, PERFETTO_NULL})})

// Specifies (manually) the track for this event
// * `UUID` can be computed with e.g.:
//   * PerfettoTeCounterTrackUuid()
//   * PerfettoTeNamedTrackUuid()
// * `...` the rest of the params should be PERFETTO_TE_PROTO_FIELD_* macros
//   and should be fields of the perfetto.protos.TrackDescriptor protobuf
//   message.
#define PERFETTO_TE_PROTO_TRACK(UUID, ...)                                  \
  PERFETTO_I_TE_EXTRA(                                                      \
      PerfettoTeHlExtraProtoTrack,                                          \
      {{PERFETTO_TE_HL_EXTRA_TYPE_PROTO_TRACK},                             \
       UUID,                                                                \
       PERFETTO_I_TE_COMPOUND_LITERAL_ARRAY(struct PerfettoTeHlProtoField*, \
                                            {__VA_ARGS__, PERFETTO_NULL})})

// Specifies that the current track for this event is a counter track named
// `const char *NAME`, child of a track whose uuid is `PARENT_UUID`. `NAME`
// and `PARENT_UUID` uniquely identify a track. Common values for `PARENT_UUID`
// include PerfettoTeProcessTrackUuid(), PerfettoTeThreadTrackUuid() or
// PerfettoTeGlobalTrackUuid().
#define PERFETTO_TE_COUNTER_TRACK(NAME, PARENT_UUID)                           \
  PERFETTO_TE_PROTO_TRACK(                                                     \
      PerfettoTeCounterTrackUuid(NAME, PARENT_UUID),                           \
      PERFETTO_TE_PROTO_FIELD_VARINT(                                          \
          perfetto_protos_TrackDescriptor_parent_uuid_field_number,            \
          PARENT_UUID),                                                        \
      PERFETTO_TE_PROTO_FIELD_CSTR(                                            \
          perfetto_protos_TrackDescriptor_name_field_number, NAME),            \
      PERFETTO_TE_PROTO_FIELD_BYTES(                                           \
          perfetto_protos_TrackDescriptor_counter_field_number, PERFETTO_NULL, \
          0))

// ----------------------------------
// The main PERFETTO_TE tracing macro
// ----------------------------------
//
// If tracing is active and the passed tracing category is enabled, adds an
// entry in the tracing stream of the perfetto track event data source.
// Parameters:
// * `CAT`: The tracing category (it should be a struct
//   PerfettoTeCategory object). It can be
//   PERFETTO_TE_DYNAMIC_CATEGORY for dynamic categories (the dynamic category
//   name should be passed later with)
// * The type of the event. It can be one of:
//   * PERFETTO_TE_SLICE_BEGIN(name)
//   * PERFETTO_TE_SLICE_END()
//   * PERFETTO_TE_INSTANT()
//   * PERFETTO_TE_COUNTER()
// * `...`: One or more macro parameters from the above list that specify the
//   data to be traced.
//
// Examples:
//
// PERFETTO_TE(category, PERFETTO_TE_SLICE_BEGIN("name"),
//             PERFETTO_TE_ARG_UINT64("extra_arg", 42));
// PERFETTO_TE(category, PERFETTO_TE_SLICE_END());
// PERFETTO_TE(category, PERFETTO_TE_COUNTER(),
//             PERFETTO_TE_REGISTERED_TRACK(&mycounter),
//             PERFETTO_TE_INT_COUNTER(79));
// PERFETTO_TE(PERFETTO_TE_DYNAMIC_CATEGORY, PERFETTO_TE_INSTANT("instant"),
//             PERFETTO_TE_DYNAMIC_CATEGORY_STRING("category"));
//
#define PERFETTO_TE(CAT, ...) PERFETTO_I_TE_IMPL(CAT, __VA_ARGS__)

#ifdef __cplusplus

// Begins a slice named `const char* NAME` on the current thread track.
//
// This is supposed to be used with PERFETTO_TE_SCOPED(). The implementation is
// identical to PERFETTO_TE_SLICE_BEGIN(): this has a different name to
// highlight the fact that PERFETTO_TE_SCOPED() also adds a
// PERFETTO_TE_SLICE_END().
#define PERFETTO_TE_SLICE(NAME) \
  PERFETTO_I_TE_HL_MACRO_NAME_AND_TYPE(NAME, PERFETTO_TE_TYPE_SLICE_BEGIN)

namespace perfetto::internal {
template <typename F>
class TeCleanup {
 public:
  explicit TeCleanup(F&& f) PERFETTO_ALWAYS_INLINE : f_(std::forward<F>(f)) {}

  ~TeCleanup() PERFETTO_ALWAYS_INLINE { f_(); }

 private:
  TeCleanup(const TeCleanup&) = delete;
  TeCleanup(TeCleanup&&) = delete;
  TeCleanup& operator=(const TeCleanup&) = delete;
  TeCleanup& operator=(TeCleanup&&) = delete;
  F f_;
};

template <typename F>
TeCleanup<F> MakeTeCleanup(F&& f) {
  return TeCleanup<F>(std::forward<F>(f));
}

}  // namespace perfetto::internal

// ------------------------
// PERFETTO_TE_SCOPED macro
// ------------------------
//
// Emits an event immediately and a PERFETTO_TE_SLICE_END event when the current
// scope terminates.
//
// All the extra params are added only to the event emitted immediately, not to
// the END event.
//
// TRACK params are not supported.
//
// This
// {
//   PERFETTO_TE_SCOPED(category, PERFETTO_TE_SLICE("name"), ...);
//   ...
// }
// is equivalent to
// {
//   PERFETTO_TE(category, PERFETTO_TE_SLICE_BEGIN("name"), ...);
//   ...
//   PERFETTO_TE(category, PERFETTO_TE_SLICE_END());
// }
//
// Examples:
//
// PERFETTO_TE_SCOPED(category, PERFETTO_TE_SLICE("name"));
// PERFETTO_TE_SCOPED(category, PERFETTO_TE_SLICE("name"),
//                    PERFETTO_TE_ARG_UINT64("count", 42));
//
#define PERFETTO_TE_SCOPED(CAT, ...)              \
  auto PERFETTO_I_TE_UID(perfetto_i_te_cleanup) = \
      (PERFETTO_I_TE_IMPL(CAT, __VA_ARGS__),      \
       perfetto::internal::MakeTeCleanup(         \
           [&] { PERFETTO_TE(CAT, PERFETTO_TE_SLICE_END()); }))

#endif  // __cplusplus

#endif  // INCLUDE_PERFETTO_PUBLIC_TE_MACROS_H_
