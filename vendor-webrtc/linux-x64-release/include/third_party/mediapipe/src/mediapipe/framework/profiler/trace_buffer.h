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

#ifndef MEDIAPIPE_FRAMEWORK_PROFILER_TRACE_BUFFER_H_
#define MEDIAPIPE_FRAMEWORK_PROFILER_TRACE_BUFFER_H_

#include <cstdint>
#include <string>

#include "absl/time/time.h"
#include "mediapipe/framework/calculator_profile.pb.h"
#include "mediapipe/framework/packet.h"
#include "mediapipe/framework/profiler/circular_buffer.h"
#include "mediapipe/framework/timestamp.h"

namespace mediapipe {

// Packet trace log event.
struct TraceEvent {
  using EventType = GraphTrace::EventType;
  absl::Time event_time;
  EventType event_type = UNKNOWN;
  bool is_finish = false;
  Timestamp input_ts = Timestamp::Unset();
  Timestamp packet_ts = Timestamp::Unset();
  int32_t node_id = -1;
  const std::string* stream_id = nullptr;
  int32_t thread_id = 0;
  int64_t event_data = 0;

  TraceEvent(const EventType& event_type) : event_type(event_type) {}
  TraceEvent() {}

  inline TraceEvent& set_event_time(absl::Time event_time) {
    this->event_time = event_time;
    return *this;
  }
  inline TraceEvent& set_event_type(const EventType& event_type) {
    this->event_type = event_type;
    return *this;
  }
  inline TraceEvent& set_node_id(int node_id) {
    this->node_id = node_id;
    return *this;
  }
  inline TraceEvent& set_stream_id(const std::string* stream_id) {
    this->stream_id = stream_id;
    return *this;
  }
  inline TraceEvent& set_input_ts(Timestamp input_ts) {
    this->input_ts = input_ts;
    return *this;
  }
  inline TraceEvent& set_packet_ts(Timestamp packet_ts) {
    this->packet_ts = packet_ts;
    return *this;
  }
  inline TraceEvent& set_packet_data_id(const Packet* packet) {
    const auto* holder = packet_internal::GetHolder(*packet);
    int64_t data_id = 0;
    if (holder != nullptr) {
      data_id = holder->DebugDataId();
    }
    this->event_data = data_id;
    return *this;
  }
  inline TraceEvent& set_thread_id(int thread_id) {
    this->thread_id = thread_id;
    return *this;
  }
  inline TraceEvent& set_is_finish(bool is_finish) {
    this->is_finish = is_finish;
    return *this;
  }
  inline TraceEvent& set_event_data(int data) {
    this->event_data = data;
    return *this;
  }

  // GraphTrace::EventType constants, repeated here to match GraphProfilerStub.
  static constexpr EventType UNKNOWN = GraphTrace::UNKNOWN;
  static constexpr EventType OPEN = GraphTrace::OPEN;
  static constexpr EventType PROCESS = GraphTrace::PROCESS;
  static constexpr EventType CLOSE = GraphTrace::CLOSE;
  static constexpr EventType NOT_READY = GraphTrace::NOT_READY;
  static constexpr EventType READY_FOR_PROCESS = GraphTrace::READY_FOR_PROCESS;
  static constexpr EventType READY_FOR_CLOSE = GraphTrace::READY_FOR_CLOSE;
  static constexpr EventType THROTTLED = GraphTrace::THROTTLED;
  static constexpr EventType UNTHROTTLED = GraphTrace::UNTHROTTLED;
  static constexpr EventType CPU_TASK_USER = GraphTrace::CPU_TASK_USER;
  static constexpr EventType CPU_TASK_SYSTEM = GraphTrace::CPU_TASK_SYSTEM;
  static constexpr EventType GPU_TASK = GraphTrace::GPU_TASK;
  static constexpr EventType DSP_TASK = GraphTrace::DSP_TASK;
  static constexpr EventType TPU_TASK = GraphTrace::TPU_TASK;
  static constexpr EventType GPU_CALIBRATION = GraphTrace::GPU_CALIBRATION;
  static constexpr EventType PACKET_QUEUED = GraphTrace::PACKET_QUEUED;
  static constexpr EventType GPU_TASK_INVOKE = GraphTrace::GPU_TASK_INVOKE;
  static constexpr EventType TPU_TASK_INVOKE = GraphTrace::TPU_TASK_INVOKE;
  static constexpr EventType CPU_TASK_INVOKE = GraphTrace::CPU_TASK_INVOKE;
  static constexpr EventType GPU_TASK_INVOKE_ADVANCED =
      GraphTrace::GPU_TASK_INVOKE_ADVANCED;
  static constexpr EventType TPU_TASK_INVOKE_ASYNC =
      GraphTrace::TPU_TASK_INVOKE_ASYNC;
};

// Packet trace log buffer.
using TraceBuffer = CircularBuffer<TraceEvent>;

// TraceEvent type traits.
class TraceEventType {
  using EventType = TraceEvent::EventType;

 public:
  TraceEventType() {}
  TraceEventType(EventType event_type, std::string description,
                 bool is_packet_event = false, bool is_stream_event = false,
                 bool id_event_data = true)
      : event_type_(event_type),
        description_(description),
        is_packet_event_(is_packet_event),
        is_stream_event_(is_stream_event),
        id_event_data_(id_event_data) {}

  // The type of event to log.
  inline EventType event_type() const { return event_type_; }

  // True if this type of event is logged.
  inline bool enabled() const { return event_type_; }
  inline void set_enabled(bool enabled) { enabled_ = enabled; }

  // True if packet details are logged with this type of event.
  inline bool is_packet_event() const { return is_packet_event_; }

  // True if stream details are logged with this type of event.
  inline bool is_stream_event() const { return is_stream_event_; }

  // True if event_data values are assigned compact id's.
  inline bool id_event_data() const { return id_event_data_; }

 private:
  EventType event_type_ = TraceEvent::UNKNOWN;
  std::string description_ = "";
  bool enabled_ = true;
  bool is_packet_event_ = false;
  bool is_stream_event_ = false;
  bool id_event_data_ = true;
};

// A hash function for TraceEvent::EventType.
struct EventTypeHash {
  size_t operator()(const TraceEvent::EventType e) const {
    return static_cast<size_t>(e);
  }
};

// The registry of trace event types.
using TraceEventRegistry =
    std::unordered_map<TraceEvent::EventType, TraceEventType, EventTypeHash>;

}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_PROFILER_TRACE_BUFFER_H_
