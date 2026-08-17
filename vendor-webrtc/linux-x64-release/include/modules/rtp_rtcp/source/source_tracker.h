/*
 *  Copyright (c) 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_RTP_RTCP_SOURCE_SOURCE_TRACKER_H_
#define MODULES_RTP_RTCP_SOURCE_SOURCE_TRACKER_H_

#include <cstddef>
#include <cstdint>
#include <list>
#include <optional>
#include <unordered_map>
#include <utility>
#include <vector>

#include "api/rtp_headers.h"
#include "api/rtp_packet_infos.h"
#include "api/transport/rtp/rtp_source.h"
#include "api/units/time_delta.h"
#include "api/units/timestamp.h"
#include "system_wrappers/include/clock.h"

namespace webrtc {

//
// Tracker for `RTCRtpContributingSource` and `RTCRtpSynchronizationSource`:
//   - https://w3c.github.io/webrtc-pc/#dom-rtcrtpcontributingsource
//   - https://w3c.github.io/webrtc-pc/#dom-rtcrtpsynchronizationsource
//
// This class is thread unsafe.
class SourceTracker {
 public:
  // Amount of time before the entry associated with an update is removed. See:
  // https://w3c.github.io/webrtc-pc/#dom-rtcrtpreceiver-getcontributingsources
  static constexpr TimeDelta kTimeout = TimeDelta::Seconds(10);

  explicit SourceTracker(Clock* clock);

  SourceTracker(const SourceTracker& other) = delete;
  SourceTracker(SourceTracker&& other) = delete;
  SourceTracker& operator=(const SourceTracker& other) = delete;
  SourceTracker& operator=(SourceTracker&& other) = delete;

  // Updates the source entries when a frame is delivered to the
  // RTCRtpReceiver's MediaStreamTrack.
  void OnFrameDelivered(const RtpPacketInfos& packet_infos,
                        Timestamp delivery_time = Timestamp::MinusInfinity());

  // Returns an `RtpSource` for each unique SSRC and CSRC identifier updated in
  // the last `kTimeoutMs` milliseconds. Entries appear in reverse chronological
  // order (i.e. with the most recently updated entries appearing first).
  std::vector<RtpSource> GetSources() const;

 private:
  struct SourceKey {
    SourceKey(RtpSourceType source_type, uint32_t source)
        : source_type(source_type), source(source) {}

    // Type of `source`.
    RtpSourceType source_type;

    // CSRC or SSRC identifier of the contributing or synchronization source.
    uint32_t source;
  };

  struct SourceKeyComparator {
    bool operator()(const SourceKey& lhs, const SourceKey& rhs) const {
      return (lhs.source_type == rhs.source_type) && (lhs.source == rhs.source);
    }
  };

  struct SourceKeyHasher {
    size_t operator()(const SourceKey& value) const {
      return static_cast<size_t>(value.source_type) +
             static_cast<size_t>(value.source) * 11076425802534262905ULL;
    }
  };

  struct SourceEntry {
    // Timestamp indicating the most recent time a frame from an RTP packet,
    // originating from this source, was delivered to the RTCRtpReceiver's
    // MediaStreamTrack. Its reference clock is the outer class's `clock_`.
    Timestamp timestamp = Timestamp::MinusInfinity();

    // Audio level from an RFC 6464 or RFC 6465 header extension received with
    // the most recent packet used to assemble the frame associated with
    // `timestamp`. May be absent. Only relevant for audio receivers. See the
    // specs for `RTCRtpContributingSource` for more info.
    std::optional<uint8_t> audio_level;

    // Absolute capture time header extension received or interpolated from the
    // most recent packet used to assemble the frame. For more info see
    // https://webrtc.org/experiments/rtp-hdrext/abs-capture-time/
    std::optional<AbsoluteCaptureTime> absolute_capture_time;

    // Clock offset between the local clock and the capturer's clock.
    // Do not confuse with `AbsoluteCaptureTime::estimated_capture_clock_offset`
    // which instead represents the clock offset between a remote sender and the
    // capturer. The following holds:
    //   Capture's NTP Clock = Local NTP Clock + Local-Capture Clock Offset
    std::optional<TimeDelta> local_capture_clock_offset;

    // RTP timestamp of the most recent packet used to assemble the frame
    // associated with `timestamp`.
    uint32_t rtp_timestamp = 0;
  };

  using SourceList = std::list<std::pair<const SourceKey, SourceEntry>>;
  using SourceMap = std::unordered_map<SourceKey,
                                       SourceList::iterator,
                                       SourceKeyHasher,
                                       SourceKeyComparator>;

  // Updates an entry by creating it (if it didn't previously exist) and moving
  // it to the front of the list. Returns a reference to the entry.
  SourceEntry& UpdateEntry(const SourceKey& key);

  // Removes entries that have timed out. Marked as "const" so that we can do
  // pruning in getters.
  void PruneEntries(Timestamp now) const;

  Clock* const clock_;

  // Entries are stored in reverse chronological order (i.e. with the most
  // recently updated entries appearing first). Mutability is needed for timeout
  // pruning in const functions.
  mutable SourceList list_;
  mutable SourceMap map_;
};

}  // namespace webrtc

#endif  // MODULES_RTP_RTCP_SOURCE_SOURCE_TRACKER_H_
