/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef NET_DCSCTP_PACKET_CHUNK_FORWARD_TSN_COMMON_H_
#define NET_DCSCTP_PACKET_CHUNK_FORWARD_TSN_COMMON_H_
#include <stdint.h>

#include <utility>
#include <vector>

#include "api/array_view.h"
#include "net/dcsctp/packet/chunk/chunk.h"

namespace dcsctp {

// Base class for both ForwardTsnChunk and IForwardTsnChunk
class AnyForwardTsnChunk : public Chunk {
 public:
  struct SkippedStream {
    SkippedStream(StreamID stream_id, SSN ssn)
        : stream_id(stream_id), ssn(ssn), unordered(false), mid(0) {}
    SkippedStream(IsUnordered unordered, StreamID stream_id, MID mid)
        : stream_id(stream_id), ssn(0), unordered(unordered), mid(mid) {}

    StreamID stream_id;

    // Set for FORWARD_TSN
    SSN ssn;

    // Set for I-FORWARD_TSN
    IsUnordered unordered;
    MID mid;

    bool operator==(const SkippedStream& other) const {
      return stream_id == other.stream_id && ssn == other.ssn &&
             unordered == other.unordered && mid == other.mid;
    }
  };

  AnyForwardTsnChunk(TSN new_cumulative_tsn,
                     std::vector<SkippedStream> skipped_streams)
      : new_cumulative_tsn_(new_cumulative_tsn),
        skipped_streams_(std::move(skipped_streams)) {}

  TSN new_cumulative_tsn() const { return new_cumulative_tsn_; }

  webrtc::ArrayView<const SkippedStream> skipped_streams() const {
    return skipped_streams_;
  }

 private:
  TSN new_cumulative_tsn_;
  std::vector<SkippedStream> skipped_streams_;
};
}  // namespace dcsctp

#endif  // NET_DCSCTP_PACKET_CHUNK_FORWARD_TSN_COMMON_H_
