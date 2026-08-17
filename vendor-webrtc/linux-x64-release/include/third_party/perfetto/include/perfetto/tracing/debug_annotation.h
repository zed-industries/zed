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

#ifndef INCLUDE_PERFETTO_TRACING_DEBUG_ANNOTATION_H_
#define INCLUDE_PERFETTO_TRACING_DEBUG_ANNOTATION_H_

#include "perfetto/base/export.h"
#include "perfetto/tracing/traced_value_forward.h"
#include "protos/perfetto/trace/track_event/debug_annotation.pbzero.h"

#include <stdint.h>

#include <memory>
#include <string>

namespace {
// std::underlying_type can't be used with non-enum types, so we need this
// indirection.
template <typename T, bool = std::is_enum<T>::value>
struct safe_underlying_type {
  using type = typename std::underlying_type<T>::type;
};

template <typename T>
struct safe_underlying_type<T, false> {
  using type = T;
};
}  // namespace

namespace perfetto {
namespace protos {
namespace pbzero {
class DebugAnnotation;
}  // namespace pbzero
}  // namespace protos

// A base class for custom track event debug annotations.
class PERFETTO_EXPORT_COMPONENT DebugAnnotation {
 public:
  DebugAnnotation() = default;
  virtual ~DebugAnnotation();

  // Called to write the contents of the debug annotation into the trace.
  virtual void Add(protos::pbzero::DebugAnnotation*) const = 0;

  void WriteIntoTracedValue(TracedValue context) const;
};

}  // namespace perfetto

#endif  // INCLUDE_PERFETTO_TRACING_DEBUG_ANNOTATION_H_
