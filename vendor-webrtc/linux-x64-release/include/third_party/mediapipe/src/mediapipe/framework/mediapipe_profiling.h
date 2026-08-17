// Copyright 2019 The MediaPipe Authors.
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

#ifndef MEDIAPIPE_FRAMEWORK_MEDIAPIPE_PROFILING_H_
#define MEDIAPIPE_FRAMEWORK_MEDIAPIPE_PROFILING_H_

#include "mediapipe/framework/platform_specific_profiling.h"
#ifdef MEDIAPIPE_PROFILER_AVAILABLE
#include "mediapipe/framework/profiler/graph_profiler.h"
#else
#include "mediapipe/framework/profiler/graph_profiler_stub.h"
#endif

#ifdef MEDIAPIPE_PROFILER_AVAILABLE
#define MEDIAPIPE_PROFILER_SCOPE_INTERNAL(event_type, calculator_context) \
  mediapipe::GraphProfiler::Scope graph_profiler_scope(                   \
      mediapipe::TraceEvent::event_type, calculator_context,              \
      calculator_context->GetProfilingContext())
#else
#define MEDIAPIPE_PROFILER_SCOPE_INTERNAL(method_name, calculator_context)
#endif

#define MEDIAPIPE_PROFILING(method_name, calculator_context) \
  MEDIAPIPE_PROFILER_SCOPE_INTERNAL(method_name, calculator_context)

namespace mediapipe {

// Log a TraceEvent to the GraphTracer.
inline void LogEvent(ProfilingContext* context, TraceEvent event) {
#ifdef MEDIAPIPE_PROFILER_AVAILABLE
  if (context) {
    context->LogEvent(event);
  }
#endif
}
}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_MEDIAPIPE_PROFILING_H_
