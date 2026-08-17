/*
 * Copyright (C) 2021 The Android Open Source Project
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

#ifndef INCLUDE_PERFETTO_TRACING_INTERNAL_WRITE_TRACK_EVENT_ARGS_H_
#define INCLUDE_PERFETTO_TRACING_INTERNAL_WRITE_TRACK_EVENT_ARGS_H_

#include "perfetto/base/compiler.h"
#include "perfetto/tracing/event_context.h"
#include "perfetto/tracing/traced_proto.h"
#include "perfetto/tracing/track_event_args.h"

namespace perfetto {
namespace internal {

// No arguments means that we don't have to write anything.
PERFETTO_ALWAYS_INLINE inline void WriteTrackEventArgs(EventContext) {}

namespace {

// A template helper for determining whether a type can be used as a track event
// lambda, i.e., it has the signature "void(EventContext)". This is achieved by
// checking that we can pass an EventContext value (the inner declval) into a T
// instance (the outer declval). If this is a valid expression, the result
// evaluates to sizeof(0), i.e., true.
// TODO(skyostil): Replace this with std::is_convertible<std::function<...>>
// once we have C++14.
template <typename T>
static constexpr bool IsValidTraceLambdaImpl(
    typename std::enable_if<static_cast<bool>(
        sizeof(std::declval<T>()(std::declval<EventContext>()), 0))>::type* =
        nullptr) {
  return true;
}

template <typename T>
static constexpr bool IsValidTraceLambdaImpl(...) {
  return false;
}

template <typename T>
static constexpr bool IsValidTraceLambda() {
  return IsValidTraceLambdaImpl<T>(nullptr);
}

template <typename T>
static constexpr bool IsValidTraceLambdaTakingReferenceImpl(
    typename std::enable_if<static_cast<bool>(
        sizeof(std::declval<T>()(std::declval<EventContext&>()), 0))>::type* =
        nullptr) {
  return true;
}

template <typename T>
static constexpr bool IsValidTraceLambdaTakingReferenceImpl(...) {
  return false;
}

template <typename T>
static constexpr bool IsValidTraceLambdaTakingReference() {
  return IsValidTraceLambdaTakingReferenceImpl<T>(nullptr);
}

template <typename T>
static constexpr bool IsFieldMetadataTypeImpl(
    typename std::enable_if<
        std::is_base_of<protozero::proto_utils::FieldMetadataBase,
                        T>::value>::type* = nullptr) {
  return true;
}

template <typename T>
static constexpr bool IsFieldMetadataTypeImpl(...) {
  return false;
}

template <typename T>
static constexpr bool IsFieldMetadataType() {
  return IsFieldMetadataTypeImpl<T>(nullptr);
}

}  // namespace

// Write an old-style lambda taking an EventContext (without a reference)
// as it will consume EventContext via std::move, it can only be the last
// argument.
template <typename ArgumentFunction,
          typename ArgFunctionCheck = typename std::enable_if<
              IsValidTraceLambda<ArgumentFunction>()>::type>
PERFETTO_ALWAYS_INLINE void WriteTrackEventArgs(
    EventContext event_ctx,
    const ArgumentFunction& arg_function) {
  arg_function(std::move(event_ctx));
}

// Forward-declare the specification for writing untyped arguments to ensure
// that typed specification could recursively pick it up.
template <typename ArgValue, typename... Args>
PERFETTO_ALWAYS_INLINE void WriteTrackEventArgs(EventContext event_ctx,
                                                const char* arg_name,
                                                ArgValue&& arg_value,
                                                Args&&... args);

template <typename FieldMetadataType,
          typename ArgValue,
          typename... Args,
          typename FieldMetadataTypeCheck = typename std::enable_if<
              IsFieldMetadataType<FieldMetadataType>()>::type>
PERFETTO_ALWAYS_INLINE void WriteTrackEventArgs(EventContext event_ctx,
                                                FieldMetadataType field_name,
                                                ArgValue&& arg_value,
                                                Args&&... args);

template <typename ArgumentFunction,
          typename... Args,
          typename ArgFunctionCheck = typename std::enable_if<
              IsValidTraceLambdaTakingReference<ArgumentFunction>()>::type>
PERFETTO_ALWAYS_INLINE void WriteTrackEventArgs(
    EventContext event_ctx,
    const ArgumentFunction& arg_function,
    Args&&... args) {
  // |arg_function| will capture EventContext by reference, so std::move isn't
  // needed.
  arg_function(event_ctx);

  WriteTrackEventArgs(std::move(event_ctx), std::forward<Args>(args)...);
}

// Write one typed message and recursively write the rest of the arguments.
template <typename FieldMetadataType,
          typename ArgValue,
          typename... Args,
          typename FieldMetadataTypeCheck>
PERFETTO_ALWAYS_INLINE void WriteTrackEventArgs(EventContext event_ctx,
                                                FieldMetadataType field_name,
                                                ArgValue&& arg_value,
                                                Args&&... args) {
  static_assert(std::is_base_of<protozero::proto_utils::FieldMetadataBase,
                                FieldMetadataType>::value,
                "");
  static_assert(
      std::is_base_of<protos::pbzero::TrackEvent,
                      typename FieldMetadataType::message_type>::value,
      "Only fields of TrackEvent (and TrackEvent's extensions) can "
      "be passed to TRACE_EVENT");
  auto track_event_proto = event_ctx.Wrap(
      event_ctx.event<typename FieldMetadataType::message_type>());
  WriteTracedProtoField(track_event_proto, field_name,
                        std::forward<ArgValue>(arg_value));
  WriteTrackEventArgs(std::move(event_ctx), std::forward<Args>(args)...);
}

// Write one debug annotation and recursively write the rest of the arguments.
template <typename ArgValue, typename... Args>
PERFETTO_ALWAYS_INLINE void WriteTrackEventArgs(EventContext event_ctx,
                                                const char* arg_name,
                                                ArgValue&& arg_value,
                                                Args&&... args) {
  event_ctx.AddDebugAnnotation(arg_name, std::forward<ArgValue>(arg_value));
  WriteTrackEventArgs(std::move(event_ctx), std::forward<Args>(args)...);
}

// Write one debug annotation and recursively write the rest of the arguments.
template <typename ArgValue, typename... Args>
PERFETTO_ALWAYS_INLINE void WriteTrackEventArgs(EventContext event_ctx,
                                                DynamicString arg_name,
                                                ArgValue&& arg_value,
                                                Args&&... args) {
  event_ctx.AddDebugAnnotation(arg_name, std::forward<ArgValue>(arg_value));
  WriteTrackEventArgs(std::move(event_ctx), std::forward<Args>(args)...);
}

}  // namespace internal
}  // namespace perfetto

#endif  // INCLUDE_PERFETTO_TRACING_INTERNAL_WRITE_TRACK_EVENT_ARGS_H_
