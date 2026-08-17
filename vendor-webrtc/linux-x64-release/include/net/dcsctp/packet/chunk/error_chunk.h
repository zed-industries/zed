/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef NET_DCSCTP_PACKET_CHUNK_ERROR_CHUNK_H_
#define NET_DCSCTP_PACKET_CHUNK_ERROR_CHUNK_H_
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

// https://tools.ietf.org/html/rfc4960#section-3.3.10
struct ErrorChunkConfig : ChunkConfig {
  static constexpr int kType = 9;
  static constexpr size_t kHeaderSize = 4;
  static constexpr size_t kVariableLengthAlignment = 4;
};

class ErrorChunk : public Chunk, public TLVTrait<ErrorChunkConfig> {
 public:
  static constexpr int kType = ErrorChunkConfig::kType;

  explicit ErrorChunk(Parameters error_causes)
      : error_causes_(std::move(error_causes)) {}

  ErrorChunk(ErrorChunk&& other) = default;
  ErrorChunk& operator=(ErrorChunk&& other) = default;

  static std::optional<ErrorChunk> Parse(webrtc::ArrayView<const uint8_t> data);

  void SerializeTo(std::vector<uint8_t>& out) const override;
  std::string ToString() const override;

  const Parameters& error_causes() const { return error_causes_; }

 private:
  Parameters error_causes_;
};

}  // namespace dcsctp

#endif  // NET_DCSCTP_PACKET_CHUNK_ERROR_CHUNK_H_
