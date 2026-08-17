/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef NET_DCSCTP_PACKET_SCTP_PACKET_H_
#define NET_DCSCTP_PACKET_SCTP_PACKET_H_

#include <stddef.h>

#include <cstdint>
#include <functional>
#include <memory>
#include <utility>
#include <vector>

#include "api/array_view.h"
#include "net/dcsctp/common/internal_types.h"
#include "net/dcsctp/packet/chunk/chunk.h"
#include "net/dcsctp/public/dcsctp_options.h"

namespace dcsctp {

// The "Common Header", which every SCTP packet starts with, and is described in
// https://tools.ietf.org/html/rfc4960#section-3.1.
struct CommonHeader {
  uint16_t source_port;
  uint16_t destination_port;
  VerificationTag verification_tag;
  uint32_t checksum;
};

// Represents an immutable (received or to-be-sent) SCTP packet.
class SctpPacket {
 public:
  static constexpr size_t kHeaderSize = 12;

  struct ChunkDescriptor {
    ChunkDescriptor(uint8_t type,
                    uint8_t flags,
                    webrtc::ArrayView<const uint8_t> data)
        : type(type), flags(flags), data(data) {}
    uint8_t type;
    uint8_t flags;
    webrtc::ArrayView<const uint8_t> data;
  };

  SctpPacket(SctpPacket&& other) = default;
  SctpPacket& operator=(SctpPacket&& other) = default;
  SctpPacket(const SctpPacket&) = delete;
  SctpPacket& operator=(const SctpPacket&) = delete;

  // Used for building SctpPacket, as those are immutable.
  class Builder {
   public:
    Builder(VerificationTag verification_tag, const DcSctpOptions& options);

    Builder(Builder&& other) = default;
    Builder& operator=(Builder&& other) = default;

    // Adds a chunk to the to-be-built SCTP packet.
    Builder& Add(const Chunk& chunk);

    // The number of bytes remaining in the packet for chunk storage until the
    // packet reaches its maximum size.
    size_t bytes_remaining() const;

    // Indicates if any packets have been added to the builder.
    bool empty() const { return out_.empty(); }

    // Returns the payload of the build SCTP packet. The Builder will be cleared
    // after having called this function, and can be used to build a new packet.
    // If `write_checksum` is set to false, a value of zero (0) will be written
    // as the packet's checksum, instead of the crc32c value.
    std::vector<uint8_t> Build(bool write_checksum = true);

   private:
    VerificationTag verification_tag_;
    uint16_t source_port_;
    uint16_t dest_port_;
    // The maximum packet size is always even divisible by four, as chunks are
    // always padded to a size even divisible by four.
    size_t max_packet_size_;
    std::vector<uint8_t> out_;
  };

  // Parses `data` as an SCTP packet and returns it if it validates.
  static std::optional<SctpPacket> Parse(webrtc::ArrayView<const uint8_t> data,
                                         const DcSctpOptions& options);

  // Returns the SCTP common header.
  const CommonHeader& common_header() const { return common_header_; }

  // Returns the chunks (types and offsets) within the packet.
  webrtc::ArrayView<const ChunkDescriptor> descriptors() const {
    return descriptors_;
  }

 private:
  SctpPacket(const CommonHeader& common_header,
             std::vector<uint8_t> data,
             std::vector<ChunkDescriptor> descriptors)
      : common_header_(common_header),
        data_(std::move(data)),
        descriptors_(std::move(descriptors)) {}

  CommonHeader common_header_;

  // As the `descriptors_` refer to offset within data, and since SctpPacket is
  // movable, `data` needs to be pointer stable, which it is according to
  // http://www.open-std.org/JTC1/SC22/WG21/docs/lwg-active.html#2321
  std::vector<uint8_t> data_;
  // The chunks and their offsets within `data_ `.
  std::vector<ChunkDescriptor> descriptors_;
};
}  // namespace dcsctp

#endif  // NET_DCSCTP_PACKET_SCTP_PACKET_H_
