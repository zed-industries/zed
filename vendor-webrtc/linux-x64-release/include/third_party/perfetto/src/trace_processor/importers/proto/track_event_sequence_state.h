/*
 * Copyright (C) 2024 The Android Open Source Project
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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_TRACK_EVENT_SEQUENCE_STATE_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_TRACK_EVENT_SEQUENCE_STATE_H_

#include <utility>

#include "protos/perfetto/trace/track_event/thread_descriptor.pbzero.h"

namespace perfetto {
namespace trace_processor {

class TrackEventSequenceState {
 public:
  static TrackEventSequenceState CreateFirst() {
    return TrackEventSequenceState(PersistentState());
  }

  TrackEventSequenceState OnIncrementalStateCleared() {
    return TrackEventSequenceState(persistent_state_);
  }

  void OnPacketLoss() { timestamps_valid_ = false; }

  bool pid_and_tid_valid() const { return persistent_state_.pid_and_tid_valid; }

  int32_t pid() const { return persistent_state_.pid; }
  int32_t tid() const { return persistent_state_.tid; }

  bool timestamps_valid() const { return timestamps_valid_; }

  int64_t IncrementAndGetTrackEventTimeNs(int64_t delta_ns) {
    PERFETTO_DCHECK(timestamps_valid());
    timestamp_ns_ += delta_ns;
    return timestamp_ns_;
  }

  int64_t IncrementAndGetTrackEventThreadTimeNs(int64_t delta_ns) {
    PERFETTO_DCHECK(timestamps_valid());
    thread_timestamp_ns_ += delta_ns;
    return thread_timestamp_ns_;
  }

  int64_t IncrementAndGetTrackEventThreadInstructionCount(int64_t delta) {
    PERFETTO_DCHECK(timestamps_valid());
    thread_instruction_count_ += delta;
    return thread_instruction_count_;
  }

  void SetThreadDescriptor(const protos::pbzero::ThreadDescriptor::Decoder&);

 private:
  // State that is never cleared.
  struct PersistentState {
    // |pid_| and |tid_| are only valid after we parsed at least one
    // ThreadDescriptor packet on the sequence.
    bool pid_and_tid_valid = false;

    // Process/thread ID of the packet sequence set by a ThreadDescriptor
    // packet. Used as default values for TrackEvents that don't specify a
    // pid/tid override. Only valid after |pid_and_tid_valid_| is set to true.
    int32_t pid = 0;
    int32_t tid = 0;
  };

  explicit TrackEventSequenceState(PersistentState persistent_state)
      : persistent_state_(std::move(persistent_state)) {}

  // We can only consider TrackEvent delta timestamps to be correct after we
  // have observed a thread descriptor (since the last packet loss).
  bool timestamps_valid_ = false;

  // Current wall/thread timestamps/counters used as reference for the next
  // TrackEvent delta timestamp.
  int64_t timestamp_ns_ = 0;
  int64_t thread_timestamp_ns_ = 0;
  int64_t thread_instruction_count_ = 0;

  PersistentState persistent_state_;
};

}  // namespace trace_processor
}  // namespace perfetto

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_TRACK_EVENT_SEQUENCE_STATE_H_
