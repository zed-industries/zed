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

#ifndef INCLUDE_PERFETTO_TEST_TRACED_VALUE_TEST_SUPPORT_H_
#define INCLUDE_PERFETTO_TEST_TRACED_VALUE_TEST_SUPPORT_H_

#include "perfetto/base/export.h"
#include "perfetto/protozero/scattered_heap_buffer.h"
#include "perfetto/tracing/traced_value.h"
#include "protos/perfetto/trace/track_event/debug_annotation.pbzero.h"

namespace perfetto {

namespace internal {
PERFETTO_EXPORT_COMPONENT std::string DebugAnnotationToString(
    const std::string& proto_message);
}  // namespace internal

// Leverage TracedValue support for the given value to convert it to a JSON-like
// representation. Note: this should be _only_ for testing TracedValue
// conversion and providing extra information for human consumption (e.g. when
// the test fails).
// Please do not rely on this to compare the object values in
// tests and implement explicit comparison operators for the objects you want to
// test as the stability of this representation is not guaranteed.
template <typename T>
std::string TracedValueToString(T&& value) {
  protozero::HeapBuffered<protos::pbzero::DebugAnnotation> message;
  WriteIntoTracedValue(internal::CreateTracedValueFromProto(message.get()),
                       std::forward<T>(value));
  return internal::DebugAnnotationToString(message.SerializeAsString());
}

}  // namespace perfetto

#endif  // INCLUDE_PERFETTO_TEST_TRACED_VALUE_TEST_SUPPORT_H_
