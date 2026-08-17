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

#ifndef MEDIAPIPE_FRAMEWORK_PROFILER_GRAPH_TRACER_H_
#define MEDIAPIPE_FRAMEWORK_PROFILER_GRAPH_TRACER_H_

#include <string>

#include "mediapipe/framework/calculator.pb.h"
#include "mediapipe/framework/calculator_context.h"
#include "mediapipe/framework/calculator_profile.pb.h"
#include "mediapipe/framework/profiler/trace_buffer.h"
#include "mediapipe/framework/profiler/trace_builder.h"

namespace mediapipe {

// GraphTracer records events when packets enter and exit the nodes of
// a calculator graph.
//
// GraphTracer is thread-safe, and the Log* methods are also non-blocking
// so they can be called during graph execution with mimimal overhead.
//
// The method GetTrace returns the events for a range of recent Timestamps.
// The begin_ts should be the first timestamp completely enclosed in the
// desired time period.  The end_ts should be the first timestamp completely
// separate from the desired time period.  The range of Timestamps can be
// computed as:
//
//   begin_ts = trace_builder.TimestampAfter(buffer, begin_time)
//   end_ts = trace_builder.TimestampAfter(buffer, end_time)
//
// In order avoid incomplete timestamps, end_time should be no later than:
//
//   end_time = current_time - max_packet_latency
//
class GraphTracer {
 public:
  // Returns the interval between trace log output.
  absl::Duration GetTraceLogInterval();

  // Returns the maximum number of trace events buffered in memory.
  int64_t GetTraceLogCapacity();

  // Create a tracer to record up to |capacity| recent events.
  GraphTracer(const ProfilerConfig& profiler_config);

  // Returns the registry of trace event types.
  TraceEventRegistry* trace_event_registry();

  // Append a TraceEvent to the TraceBuffer.
  void LogEvent(TraceEvent event);

  // Append TraceEvents to the TraceBuffer for task input.
  void LogInputEvents(GraphTrace::EventType event_type,
                      const CalculatorContext* context, absl::Time event_time);

  // Append TraceEvents to the TraceBuffer for task output.
  void LogOutputEvents(GraphTrace::EventType event_type,
                       const CalculatorContext* context, absl::Time event_time);

  // Returns the earliest packet timestamp appearing only after begin_time.
  Timestamp TimestampAfter(absl::Time begin_time);

  // Returns the graph of events between begin_time and end_time exclusive.
  void GetTrace(absl::Time begin_time, absl::Time end_time, GraphTrace* result);

  // Returns trace events between begin_time and end_time exclusive.
  void GetLog(absl::Time begin_time, absl::Time end_time, GraphTrace* result);

  // Returns the logged TraceEvents.
  const TraceBuffer& GetTraceBuffer();

 private:
  // Returns the timestamp of the first output packet.
  Timestamp GetOutputTimestamp(const CalculatorContext* context);

  // The settings for this tracer.
  ProfilerConfig profiler_config_;

  // The circular buffer of TraceEvents.
  TraceBuffer trace_buffer_;

  // The builder for the GraphTrace protobuf.
  TraceBuilder trace_builder_;
};

}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_PROFILER_GRAPH_TRACER_H_
