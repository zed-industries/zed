/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef NET_DCSCTP_PACKET_CHUNK_SACK_CHUNK_H_
#define NET_DCSCTP_PACKET_CHUNK_SACK_CHUNK_H_
#include <stddef.h>

#include <cstdint>
#include <set>
#include <string>
#include <utility>
#include <vector>

#include "absl/strings/string_view.h"
#include "api/array_view.h"
#include "net/dcsctp/packet/chunk/chunk.h"
#include "net/dcsctp/packet/tlv_trait.h"

namespace dcsctp {

// https://tools.ietf.org/html/rfc4960#section-3.3.4
struct SackChunkConfig : ChunkConfig {
  static constexpr int kType = 3;
  static constexpr size_t kHeaderSize = 16;
  static constexpr size_t kVariableLengthAlignment = 4;
};

class SackChunk : public Chunk, public TLVTrait<SackChunkConfig> {
 public:
  static constexpr int kType = SackChunkConfig::kType;

  struct GapAckBlock {
    GapAckBlock(uint16_t start, uint16_t end) : start(start), end(end) {}

    uint16_t start;
    uint16_t end;

    bool operator==(const GapAckBlock& other) const {
      return start == other.start && end == other.end;
    }
  };

  SackChunk(TSN cumulative_tsn_ack,
            uint32_t a_rwnd,
            std::vector<GapAckBlock> gap_ack_blocks,
            std::set<TSN> duplicate_tsns)
      : cumulative_tsn_ack_(cumulative_tsn_ack),
        a_rwnd_(a_rwnd),
        gap_ack_blocks_(std::move(gap_ack_blocks)),
        duplicate_tsns_(std::move(duplicate_tsns)) {}
  static std::optional<SackChunk> Parse(webrtc::ArrayView<const uint8_t> data);

  void SerializeTo(std::vector<uint8_t>& out) const override;
  std::string ToString() const override;

  TSN cumulative_tsn_ack() const { return cumulative_tsn_ack_; }
  uint32_t a_rwnd() const { return a_rwnd_; }
  webrtc::ArrayView<const GapAckBlock> gap_ack_blocks() const {
    return gap_ack_blocks_;
  }
  const std::set<TSN>& duplicate_tsns() const { return duplicate_tsns_; }

 private:
  static constexpr size_t kGapAckBlockSize = 4;
  static constexpr size_t kDupTsnBlockSize = 4;

  const TSN cumulative_tsn_ack_;
  const uint32_t a_rwnd_;
  std::vector<GapAckBlock> gap_ack_blocks_;
  std::set<TSN> duplicate_tsns_;
};
}  // namespace dcsctp

#endif  // NET_DCSCTP_PACKET_CHUNK_SACK_CHUNK_H_
