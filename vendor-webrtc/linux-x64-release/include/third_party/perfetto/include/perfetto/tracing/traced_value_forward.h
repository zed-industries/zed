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

#ifndef INCLUDE_PERFETTO_TRACING_TRACED_VALUE_FORWARD_H_
#define INCLUDE_PERFETTO_TRACING_TRACED_VALUE_FORWARD_H_

namespace perfetto {

class TracedValue;
class TracedArray;
class TracedDictionary;
template <typename MessageType>
class TracedProto;

template <typename T>
void WriteIntoTracedValue(TracedValue context, T&& value);
template <typename MessageType, typename T>
void WriteIntoTracedProto(TracedProto<MessageType> context, T&& value);

template <typename T, class = void>
struct TraceFormatTraits;

// Helpers to check whether a given type T can be written into a TracedValue /
// TracedProto<MessageType>.
//
// Intended to be used for types like smart pointers, who should support
// WriteIntoTrace only iff their inner type supports being written into
// a TracedValue.
//
// template <typename T>
// class SmartPtr {
//   ...
//
//   // Note: |Check| is needed to ensure that using
//   SmartPtr<ClassWhichDoesNotSupportTracedValue> does not generate a
//   compilation error.
//
//   template <typename Check=void>
//   typename check_traced_value_support<T, Check>::value
//   WriteIntoTrace(perfetto::TracedValue context) const {
//      WriteIntoTracedValue(std::move(context), *ptr_);
//   }
//
//   template <typename MessageType>
//   typename check_traced_value_support<T, MessageType>::value
//   WriteIntoTrace(perfetto::TracedProto<MessageType> message) const {
//      WriteIntoTracedProto(std::move(message), *ptr_);
//   }
// };
template <typename T, typename ResultType = void, typename = void>
struct check_traced_value_support;

template <typename MessageType,
          typename T,
          typename ResultType = void,
          typename = void>
struct check_traced_proto_support;

}  // namespace perfetto

#endif  // INCLUDE_PERFETTO_TRACING_TRACED_VALUE_FORWARD_H_
