/*
 *  Copyright (c) 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_RTP_RTCP_SOURCE_RTP_SEQUENCE_NUMBER_MAP_H_
#define MODULES_RTP_RTCP_SOURCE_RTP_SEQUENCE_NUMBER_MAP_H_

#include <cstddef>
#include <cstdint>
#include <deque>
#include <optional>

namespace webrtc {

// Records the association of RTP sequence numbers to timestamps and to whether
// the packet was first and/or last in the frame.
//
// 1. Limits number of entries. Whenever `max_entries` is about to be exceeded,
//    the size is reduced by approximately 25%.
// 2. RTP sequence numbers wrap around relatively infrequently.
//    This class therefore only remembers at most the last 2^15 RTP packets,
//    so that the newest packet's sequence number is still AheadOf the oldest
//    packet's sequence number.
// 3. Media frames are sometimes split into several RTP packets.
//    In such a case, Insert() is expected to be called once for each packet.
//    The timestamp is not expected to change between those calls.
class RtpSequenceNumberMap final {
 public:
  struct Info final {
    Info(uint32_t timestamp, bool is_first, bool is_last)
        : timestamp(timestamp), is_first(is_first), is_last(is_last) {}

    friend bool operator==(const Info& lhs, const Info& rhs) {
      return lhs.timestamp == rhs.timestamp && lhs.is_first == rhs.is_first &&
             lhs.is_last == rhs.is_last;
    }

    uint32_t timestamp;
    bool is_first;
    bool is_last;
  };

  explicit RtpSequenceNumberMap(size_t max_entries);
  RtpSequenceNumberMap(const RtpSequenceNumberMap& other) = delete;
  RtpSequenceNumberMap& operator=(const RtpSequenceNumberMap& other) = delete;
  ~RtpSequenceNumberMap();

  void InsertPacket(uint16_t sequence_number, Info info);
  void InsertFrame(uint16_t first_sequence_number,
                   size_t packet_count,
                   uint32_t timestamp);

  std::optional<Info> Get(uint16_t sequence_number) const;

  size_t AssociationCountForTesting() const;

 private:
  struct Association {
    explicit Association(uint16_t sequence_number)
        : Association(sequence_number, Info(0, false, false)) {}

    Association(uint16_t sequence_number, Info info)
        : sequence_number(sequence_number), info(info) {}

    uint16_t sequence_number;
    Info info;
  };

  const size_t max_entries_;

  // The non-transitivity of AheadOf() would be problematic with a map,
  // so we use a deque instead.
  std::deque<Association> associations_;
};

}  // namespace webrtc

#endif  // MODULES_RTP_RTCP_SOURCE_RTP_SEQUENCE_NUMBER_MAP_H_
