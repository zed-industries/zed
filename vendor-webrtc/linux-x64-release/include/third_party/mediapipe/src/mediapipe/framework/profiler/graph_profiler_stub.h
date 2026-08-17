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

#ifndef MEDIAPIPE_FRAMEWORK_PROFILER_MEDIAPIPE_PROFILER_STUB_H_
#define MEDIAPIPE_FRAMEWORK_PROFILER_MEDIAPIPE_PROFILER_STUB_H_

#include <cstdint>

#include "mediapipe/framework/port/status.h"
#include "mediapipe/framework/timestamp.h"

namespace mediapipe {
class CalculatorProfile;
class GraphTrace;
class GraphProfile;
}  // namespace mediapipe

namespace mediapipe {
using mediapipe::CalculatorProfile;
using mediapipe::GraphProfile;
using mediapipe::GraphTrace;

class ValidatedGraphConfig;
class Executor;
class Packet;
class Clock;
class GraphTracer;
class GlProfilingHelper;

class TraceEvent {
 public:
  enum EventType {
    UNKNOWN,
    OPEN,
    PROCESS,
    CLOSE,
    NOT_READY,
    READY_FOR_PROCESS,
    READY_FOR_CLOSE,
    THROTTLED,
    UNTHROTTLED,
    CPU_TASK_USER,
    CPU_TASK_SYSTEM,
    GPU_TASK,
    DSP_TASK,
    TPU_TASK,
    GPU_CALIBRATION,
    PACKET_QUEUED,
  };
  TraceEvent(const EventType& event_type) {}
  TraceEvent() {}
  inline TraceEvent& set_event_time(absl::Time event_time) { return *this; }
  inline TraceEvent& set_event_type(const EventType& event_type) {
    return *this;
  }
  inline TraceEvent& set_node_id(int node_id) { return *this; }
  inline TraceEvent& set_stream_id(const std::string* stream_id) {
    return *this;
  }
  inline TraceEvent& set_input_ts(Timestamp input_ts) { return *this; }
  inline TraceEvent& set_packet_ts(Timestamp packet_ts) { return *this; }
  inline TraceEvent& set_packet_data_id(const Packet* packet) { return *this; }
  inline TraceEvent& set_thread_id(int thread_id) { return *this; }
  inline TraceEvent& set_is_finish(bool is_finish) { return *this; }
  inline TraceEvent& set_event_data(int64_t data) { return *this; }
};

// GraphProfiler::CaptureProfile option, see the method for details.
enum class PopulateGraphConfig { kNo, kFull };

// Empty implementation of ProfilingContext to be used in place of the
// GraphProfiler when the main implementation is disabled.
class GraphProfilerStub {
 public:
  inline void Initialize(const ValidatedGraphConfig& validated_graph_config) {}
  inline void SetClock(const std::shared_ptr<mediapipe::Clock>& clock) {}
  inline void LogEvent(const TraceEvent& event) {}
  inline absl::Status GetCalculatorProfiles(
      std::vector<CalculatorProfile>*) const {
    return absl::OkStatus();
  }
  absl::Status CaptureProfile(
      GraphProfile* result,
      PopulateGraphConfig populate_config = PopulateGraphConfig::kNo) {
    return absl::OkStatus();
  }
  inline absl::Status WriteProfile() { return absl::OkStatus(); }
  inline void Pause() {}
  inline void Resume() {}
  inline void Reset() {}
  inline absl::Status Start(mediapipe::Executor* executor) {
    return absl::OkStatus();
  }
  inline absl::Status Stop() { return absl::OkStatus(); }
  inline GraphTracer* tracer() { return nullptr; }
  inline std::unique_ptr<GlProfilingHelper> CreateGlProfilingHelper() {
    return nullptr;
  }
  const std::shared_ptr<mediapipe::Clock> GetClock() const { return nullptr; }
};

// The API class used to access the preferred profiler, such as
// GraphProfiler or GraphProfilerStub.  ProfilingContext is defined as
// a class rather than a typedef in order to support clients that refer
// to it only as a forward declaration, such as CalculatorState.
class ProfilingContext : public GraphProfilerStub {};

// Empty implementation of GlProfilingHelper to be used in place of the
// GlContextProfiler when the main implementation is disabled.
class GlContextProfilerStub {
 public:
  explicit GlContextProfilerStub(
      std::shared_ptr<ProfilingContext> profiling_context) {}
  // Not copyable or movable.
  GlContextProfilerStub(const GlContextProfilerStub&) = delete;
  GlContextProfilerStub& operator=(const GlContextProfilerStub&) = delete;
  bool Initialze() { return false; }
  void MarkTimestamp(int node_id, Timestamp input_timestamp, bool is_finish) {}
  void LogAllTimestamps() {}
};

// The API class used to access the preferred GlContext profiler, such as
// GlContextProfiler or GlContextProfilerStub. GlProfilingHelper is defined as
// a class rather than a typedef in order to support clients that refer
// to it only as a forward declaration.
class GlProfilingHelper : public GlContextProfilerStub {
  using GlContextProfilerStub::GlContextProfilerStub;
};

}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_PROFILER_MEDIAPIPE_PROFILER_STUB_H_
