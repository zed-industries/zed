// Copyright 2018 The MediaPipe Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef MEDIAPIPE_FRAMEWORK_PROFILER_TRACE_BUILDER_H_
#define MEDIAPIPE_FRAMEWORK_PROFILER_TRACE_BUILDER_H_

#include <string>

#include "mediapipe/framework/calculator_profile.pb.h"
#include "mediapipe/framework/profiler/trace_buffer.h"

namespace mediapipe {

// Builds a GraphTrace for a range of recent Timestamps.
class TraceBuilder {
 public:
  TraceBuilder();
  ~TraceBuilder();

  // Returns the registry of trace event types.
  TraceEventRegistry* trace_event_registry();

  // Returns the earliest packet timestamp appearing only after begin_time.
  static Timestamp TimestampAfter(const TraceBuffer& buffer,
                                  absl::Time begin_time);

  // Returns the graph of traces between begin_time and end_time exclusive.
  void CreateTrace(const TraceBuffer& buffer, absl::Time begin_time,
                   absl::Time end_time, GraphTrace* result);

  // Returns trace events between begin_time and end_time exclusive.
  void CreateLog(const TraceBuffer& buffer, absl::Time begin_time,
                 absl::Time end_time, GraphTrace* result);

  // Resets the TraceBuilder to begin building a new trace.
  void Clear();

 private:
  class Impl;
  std::unique_ptr<Impl> impl_;
};

}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_PROFILER_TRACE_BUILDER_H_
