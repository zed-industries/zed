/*
 * Copyright (C) 2020 The Android Open Source Project
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

#ifndef INCLUDE_PERFETTO_TRACING_TRACK_EVENT_STATE_TRACKER_H_
#define INCLUDE_PERFETTO_TRACING_TRACK_EVENT_STATE_TRACKER_H_

#include "perfetto/base/export.h"

#include "protos/perfetto/trace/track_event/track_event.pbzero.h"

#include <map>
#include <string>
#include <vector>

namespace perfetto {
namespace protos {
namespace pbzero {
class TracePacket_Decoder;
class TrackEvent;
class TrackEvent_Decoder;
}  // namespace pbzero
}  // namespace protos

// A helper for keeping track of incremental state when intercepting track
// events.
class PERFETTO_EXPORT_COMPONENT TrackEventStateTracker {
 public:
  ~TrackEventStateTracker();

  struct StackFrame {
    uint64_t timestamp{};

    // Only one of |name| and |name_iid| will be set.
    std::string name;
    uint64_t name_iid{};
    uint64_t name_hash{};

    // Only one of |category| and |category_iid| will be set.
    std::string category;
    uint64_t category_iid{};
  };

  struct Track {
    uint64_t uuid{};
    uint32_t index{};  // Ordinal number for the track in the tracing session.

    std::string name;
    int64_t pid{};
    int64_t tid{};

    // Opaque user data associated with the track.
    std::vector<uint8_t> user_data;

    // Stack of opened slices on this track.
    std::vector<StackFrame> stack;
  };

  // State for a single trace writer sequence (typically a single thread).
  struct SequenceState {
    // Trace packet sequence defaults.
    Track track;

    // Interned state.
#if PERFETTO_DCHECK_IS_ON()
    uint32_t sequence_id{};
#endif
    std::map<uint64_t /*iid*/, std::string> event_names;
    std::map<uint64_t /*iid*/, std::string> event_categories;
    std::map<uint64_t /*iid*/, std::string> debug_annotation_names;
    // Current absolute timestamp of the incremental clock.
    uint64_t most_recent_absolute_time_ns = 0;
    // default_clock_id == 0 means, no default clock_id is set.
    uint32_t default_clock_id = 0;
  };

  // State for the entire tracing session. Shared by all trace writer sequences
  // participating in the session.
  struct SessionState {
    // Non-thread-bound tracks.
    std::map<uint64_t /*uuid*/, Track> tracks;
  };

  // Represents a single decoded track event (without arguments).
  struct ParsedTrackEvent {
    explicit ParsedTrackEvent(
        const perfetto::protos::pbzero::TrackEvent::Decoder&);

    // Underlying event.
    const perfetto::protos::pbzero::TrackEvent::Decoder& track_event;

    // Event metadata.
    uint64_t timestamp_ns{};
    uint64_t duration_ns{};

    size_t stack_depth{};

    protozero::ConstChars category{};
    protozero::ConstChars name{};
    uint64_t name_hash{};
  };

  // Interface used by the tracker to access tracing session and sequence state
  // and to report parsed track events.
  class PERFETTO_EXPORT_COMPONENT Delegate {
   public:
    virtual ~Delegate();

    // Called to retrieve the session-global state shared by all sequences. The
    // returned pointer must remain valid (locked) throughout the call to
    // |ProcessTracePacket|.
    virtual SessionState* GetSessionState() = 0;

    // Called when the metadata (e.g., name) for a track changes. |Track| can be
    // modified by the callback to attach user data.
    virtual void OnTrackUpdated(Track&) = 0;

    // If the packet given to |ProcessTracePacket| contains a track event, this
    // method is called to report the properties of that event. Note that memory
    // pointers in |TrackEvent| will only be valid during this call.
    virtual void OnTrackEvent(const Track&, const ParsedTrackEvent&) = 0;
  };

  // Process a single trace packet, reporting any contained track event back via
  // the delegate interface. |SequenceState| must correspond to the sequence
  // that was used to write the packet.
  static void ProcessTracePacket(Delegate&,
                                 SequenceState&,
                                 const protos::pbzero::TracePacket_Decoder&);

 private:
  static void UpdateIncrementalState(
      Delegate&,
      SequenceState&,
      const protos::pbzero::TracePacket_Decoder&);
};

}  // namespace perfetto

#endif  // INCLUDE_PERFETTO_TRACING_TRACK_EVENT_STATE_TRACKER_H_
