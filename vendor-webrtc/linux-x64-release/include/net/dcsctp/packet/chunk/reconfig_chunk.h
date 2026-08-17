/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef NET_DCSCTP_PACKET_CHUNK_RECONFIG_CHUNK_H_
#define NET_DCSCTP_PACKET_CHUNK_RECONFIG_CHUNK_H_
#include <stddef.h>
#include <stdint.h>

#include <string>
#include <utility>
#include <vector>

#include "absl/strings/string_view.h"
#include "api/array_view.h"
#include "net/dcsctp/packet/chunk/chunk.h"
#include "net/dcsctp/packet/parameter/parameter.h"
#include "net/dcsctp/packet/tlv_trait.h"

namespace dcsctp {

// https://tools.ietf.org/html/rfc6525#section-3.1
struct ReConfigChunkConfig : ChunkConfig {
  static constexpr int kType = 130;
  static constexpr size_t kHeaderSize = 4;
  static constexpr size_t kVariableLengthAlignment = 1;
};

class ReConfigChunk : public Chunk, public TLVTrait<ReConfigChunkConfig> {
 public:
  static constexpr int kType = ReConfigChunkConfig::kType;

  explicit ReConfigChunk(Parameters parameters)
      : parameters_(std::move(parameters)) {}

  static std::optional<ReConfigChunk> Parse(
      webrtc::ArrayView<const uint8_t> data);

  void SerializeTo(std::vector<uint8_t>& out) const override;
  std::string ToString() const override;

  const Parameters& parameters() const { return parameters_; }
  Parameters extract_parameters() { return std::move(parameters_); }

 private:
  Parameters parameters_;
};

}  // namespace dcsctp

#endif  // NET_DCSCTP_PACKET_CHUNK_RECONFIG_CHUNK_H_
