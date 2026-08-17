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

#ifndef INCLUDE_PERFETTO_PROFILING_PPROF_BUILDER_H_
#define INCLUDE_PERFETTO_PROFILING_PPROF_BUILDER_H_

#include <cstdint>
#include <string>
#include <vector>

namespace perfetto {

namespace trace_processor {
class TraceProcessor;
}

namespace profiling {
class Symbolizer;
}

namespace trace_to_text {

enum class ProfileType {
  kHeapProfile,
  kJavaHeapProfile,
  kPerfProfile,
};

struct SerializedProfile {
  ProfileType profile_type;
  uint64_t pid;
  std::string serialized;
  // non-empty if profile_type == kHeapProfile
  std::string heap_name;
};

enum class ConversionMode { kHeapProfile, kPerfProfile, kJavaHeapProfile };

enum class ConversionFlags : uint64_t {
  kNone = 0,
  // Suffix frame names with additional information. Current annotations are
  // specific to apps running within the Android runtime, and include
  // information such as whether the given frame was interpreted / executed
  // under JIT / etc.
  kAnnotateFrames = 1
};

bool TraceToPprof(trace_processor::TraceProcessor* tp,
                  std::vector<SerializedProfile>* output,
                  ConversionMode mode = ConversionMode::kHeapProfile,
                  uint64_t flags = 0,
                  uint64_t pid = 0,
                  const std::vector<uint64_t>& timestamps = {});

}  // namespace trace_to_text
}  // namespace perfetto

#endif  // INCLUDE_PERFETTO_PROFILING_PPROF_BUILDER_H_
