/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef NET_DCSCTP_PACKET_CHUNK_ABORT_CHUNK_H_
#define NET_DCSCTP_PACKET_CHUNK_ABORT_CHUNK_H_
#include <stddef.h>
#include <stdint.h>

#include <string>
#include <utility>
#include <vector>

#include "absl/strings/string_view.h"
#include "api/array_view.h"
#include "net/dcsctp/packet/chunk/chunk.h"
#include "net/dcsctp/packet/error_cause/error_cause.h"
#include "net/dcsctp/packet/tlv_trait.h"

namespace dcsctp {

// https://tools.ietf.org/html/rfc4960#section-3.3.7
struct AbortChunkConfig : ChunkConfig {
  static constexpr int kType = 6;
  static constexpr size_t kHeaderSize = 4;
  static constexpr size_t kVariableLengthAlignment = 1;
};

class AbortChunk : public Chunk, public TLVTrait<AbortChunkConfig> {
 public:
  static constexpr int kType = AbortChunkConfig::kType;

  AbortChunk(bool filled_in_verification_tag, Parameters error_causes)
      : filled_in_verification_tag_(filled_in_verification_tag),
        error_causes_(std::move(error_causes)) {}

  AbortChunk(AbortChunk&& other) = default;
  AbortChunk& operator=(AbortChunk&& other) = default;

  static std::optional<AbortChunk> Parse(webrtc::ArrayView<const uint8_t> data);

  void SerializeTo(std::vector<uint8_t>& out) const override;
  std::string ToString() const override;

  bool filled_in_verification_tag() const {
    return filled_in_verification_tag_;
  }

  const Parameters& error_causes() const { return error_causes_; }

 private:
  static constexpr int kFlagsBitT = 0;
  bool filled_in_verification_tag_;
  Parameters error_causes_;
};

}  // namespace dcsctp

#endif  // NET_DCSCTP_PACKET_CHUNK_ABORT_CHUNK_H_
