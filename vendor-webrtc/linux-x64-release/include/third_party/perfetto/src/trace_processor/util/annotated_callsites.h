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

#ifndef SRC_TRACE_PROCESSOR_UTIL_ANNOTATED_CALLSITES_H_
#define SRC_TRACE_PROCESSOR_UTIL_ANNOTATED_CALLSITES_H_

#include <optional>
#include <unordered_map>
#include <utility>

#include "src/trace_processor/containers/null_term_string_view.h"
#include "src/trace_processor/containers/string_pool.h"
#include "src/trace_processor/storage/trace_storage.h"
#include "src/trace_processor/tables/profiler_tables_py.h"

namespace perfetto::trace_processor {

class TraceProcessorContext;

enum class CallsiteAnnotation {
  kNone,
  kCommonFrame,
  kCommonFrameInterp,
  kArtInterpreted,
  kArtJit,
  kArtAot,
};

// Helper class to augment callsite with (currently Android-specific)
// annotations. A given callsite will always have the same annotation. This
// class will internally cache already computed annotations. An annotation
// depends only of the current callsite and the annotations of its parent
// callsites (going to the root).
class AnnotatedCallsites {
 public:
  explicit AnnotatedCallsites(const TraceProcessorContext* context);

  CallsiteAnnotation GetAnnotation(
      const tables::StackProfileCallsiteTable::ConstRowReference& callsite) {
    return Get(callsite).second;
  }

 private:
  enum class MapType {
    kArtInterp,
    kArtJit,
    kArtAot,
    kNativeLibart,
    kNativeOther,
    kOther
  };

  // Annotation FSM states:
  // * kInitial: default, native-only callstacks never leave this state.
  // * kEraseLibart: we've seen a managed frame, and will now "erase" (i.e. tag
  //                 as a common-frame) frames belonging to the ART runtime.
  // * kKeepNext: we've seen a special JNI trampoline for managed->native
  //              transition, keep the immediate child (even if it is in ART),
  //              and then go back to kEraseLibart.
  // Regardless of the state, managed frames get annotated with their execution
  // mode, based on the mapping.
  enum class State { kInitial, kEraseLibart, kKeepNext };

  State GetState(std::optional<CallsiteId> id);

  std::pair<State, CallsiteAnnotation> Get(
      const tables::StackProfileCallsiteTable::ConstRowReference& callsite);

  MapType GetMapType(MappingId id);
  static MapType ClassifyMap(NullTermStringView map);

  const TraceProcessorContext& context_;
  const std::optional<StringPool::Id> art_jni_trampoline_;

  std::unordered_map<MappingId, MapType> map_types_;
  std::unordered_map<CallsiteId, State> states_;
};

}  // namespace perfetto::trace_processor

#endif  // SRC_TRACE_PROCESSOR_UTIL_ANNOTATED_CALLSITES_H_
