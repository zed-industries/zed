/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef NET_DCSCTP_PACKET_CHUNK_DATA_CHUNK_H_
#define NET_DCSCTP_PACKET_CHUNK_DATA_CHUNK_H_
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

// https://tools.ietf.org/html/rfc4960#section-3.3.1
struct DataChunkConfig : ChunkConfig {
  static constexpr int kType = 0;
  static constexpr size_t kHeaderSize = 16;
  static constexpr size_t kVariableLengthAlignment = 1;
};

class DataChunk : public AnyDataChunk, public TLVTrait<DataChunkConfig> {
 public:
  static constexpr int kType = DataChunkConfig::kType;

  // Exposed to allow the retransmission queue to make room for the correct
  // header size.
  static constexpr size_t kHeaderSize = DataChunkConfig::kHeaderSize;

  DataChunk(TSN tsn,
            StreamID stream_id,
            SSN ssn,
            PPID ppid,
            std::vector<uint8_t> payload,
            const Options& options)
      : AnyDataChunk(tsn,
                     stream_id,
                     ssn,
                     MID(0),
                     FSN(0),
                     ppid,
                     std::move(payload),
                     options) {}

  DataChunk(TSN tsn, Data&& data, bool immediate_ack)
      : AnyDataChunk(tsn, std::move(data), immediate_ack) {}

  static std::optional<DataChunk> Parse(webrtc::ArrayView<const uint8_t> data);

  void SerializeTo(std::vector<uint8_t>& out) const override;
  std::string ToString() const override;
};

}  // namespace dcsctp

#endif  // NET_DCSCTP_PACKET_CHUNK_DATA_CHUNK_H_
