/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef NET_DCSCTP_PACKET_CHUNK_CHUNK_H_
#define NET_DCSCTP_PACKET_CHUNK_CHUNK_H_

#include <stddef.h>
#include <sys/types.h>

#include <cstdint>
#include <iterator>
#include <memory>
#include <optional>
#include <string>
#include <utility>
#include <vector>

#include "absl/algorithm/container.h"
#include "absl/strings/string_view.h"
#include "api/array_view.h"
#include "net/dcsctp/packet/data.h"
#include "net/dcsctp/packet/error_cause/error_cause.h"
#include "net/dcsctp/packet/parameter/parameter.h"
#include "net/dcsctp/packet/tlv_trait.h"

namespace dcsctp {

// Base class for all SCTP chunks
class Chunk {
 public:
  Chunk() {}
  virtual ~Chunk() = default;

  // Chunks can contain data payloads that shouldn't be copied unnecessarily.
  Chunk(Chunk&& other) = default;
  Chunk& operator=(Chunk&& other) = default;
  Chunk(const Chunk&) = delete;
  Chunk& operator=(const Chunk&) = delete;

  // Serializes the chunk to `out`, growing it as necessary.
  virtual void SerializeTo(std::vector<uint8_t>& out) const = 0;

  // Returns a human readable description of this chunk and its parameters.
  virtual std::string ToString() const = 0;
};

// Introspects the chunk in `data` and returns a human readable textual
// representation of it, to be used in debugging.
std::string DebugConvertChunkToString(webrtc::ArrayView<const uint8_t> data);

struct ChunkConfig {
  static constexpr int kTypeSizeInBytes = 1;
};

}  // namespace dcsctp

#endif  // NET_DCSCTP_PACKET_CHUNK_CHUNK_H_
