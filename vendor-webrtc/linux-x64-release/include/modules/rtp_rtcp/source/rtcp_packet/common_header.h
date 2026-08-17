/*
 *  Copyright (c) 2016 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef MODULES_RTP_RTCP_SOURCE_RTCP_PACKET_COMMON_HEADER_H_
#define MODULES_RTP_RTCP_SOURCE_RTCP_PACKET_COMMON_HEADER_H_

#include <stddef.h>
#include <stdint.h>

namespace webrtc {
namespace rtcp {
class CommonHeader {
 public:
  static constexpr size_t kHeaderSizeBytes = 4;

  CommonHeader() {}
  CommonHeader(const CommonHeader&) = default;
  CommonHeader& operator=(const CommonHeader&) = default;

  bool Parse(const uint8_t* buffer, size_t size_bytes);

  uint8_t type() const { return packet_type_; }
  // Depending on packet type same header field can be used either as count or
  // as feedback message type (fmt). Caller expected to know how it is used.
  uint8_t fmt() const { return count_or_format_; }
  uint8_t count() const { return count_or_format_; }
  size_t payload_size_bytes() const { return payload_size_; }
  const uint8_t* payload() const { return payload_; }
  size_t packet_size() const {
    return kHeaderSizeBytes + payload_size_ + padding_size_;
  }
  // Returns pointer to the next RTCP packet in compound packet.
  const uint8_t* NextPacket() const {
    return payload_ + payload_size_ + padding_size_;
  }

 private:
  uint8_t packet_type_ = 0;
  uint8_t count_or_format_ = 0;
  uint8_t padding_size_ = 0;
  uint32_t payload_size_ = 0;
  const uint8_t* payload_ = nullptr;
};
}  // namespace rtcp
}  // namespace webrtc
#endif  // MODULES_RTP_RTCP_SOURCE_RTCP_PACKET_COMMON_HEADER_H_
