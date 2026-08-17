/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef NET_DCSCTP_PACKET_CHUNK_IDATA_CHUNK_H_
#define NET_DCSCTP_PACKET_CHUNK_IDATA_CHUNK_H_
#include <stddef.h>
#include <stdint.h>

#include <cstdint>
#include <string>
#include <utility>
#include <vector>

#include "absl/strings/string_view.h"
#include "api/array_view.h"
#include "net/dcsctp/packet/chunk/chunk.h"
#include "net/dcsctp/packet/chunk/data_common.h"
#include "net/dcsctp/packet/data.h"
#include "net/dcsctp/packet/tlv_trait.h"

namespace dcsctp {

// https://tools.ietf.org/html/rfc8260#section-2.1
struct IDataChunkConfig : ChunkConfig {
  static constexpr int kType = 64;
  static constexpr size_t kHeaderSize = 20;
  static constexpr size_t kVariableLengthAlignment = 1;
};

class IDataChunk : public AnyDataChunk, public TLVTrait<IDataChunkConfig> {
 public:
  static constexpr int kType = IDataChunkConfig::kType;

  // Exposed to allow the retransmission queue to make room for the correct
  // header size.
  static constexpr size_t kHeaderSize = IDataChunkConfig::kHeaderSize;
  IDataChunk(TSN tsn,
             StreamID stream_id,
             MID mid,
             PPID ppid,
             FSN fsn,
             std::vector<uint8_t> payload,
             const Options& options)
      : AnyDataChunk(tsn,
                     stream_id,
                     SSN(0),
                     mid,
                     fsn,
                     ppid,
                     std::move(payload),
                     options) {}

  explicit IDataChunk(TSN tsn, Data&& data, bool immediate_ack)
      : AnyDataChunk(tsn, std::move(data), immediate_ack) {}

  static std::optional<IDataChunk> Parse(webrtc::ArrayView<const uint8_t> data);

  void SerializeTo(std::vector<uint8_t>& out) const override;
  std::string ToString() const override;
};

}  // namespace dcsctp

#endif  // NET_DCSCTP_PACKET_CHUNK_IDATA_CHUNK_H_
