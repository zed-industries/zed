/*
 *  Copyright (c) 2016 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_RTP_RTCP_SOURCE_RTCP_PACKET_TARGET_BITRATE_H_
#define MODULES_RTP_RTCP_SOURCE_RTCP_PACKET_TARGET_BITRATE_H_

#include <stddef.h>
#include <stdint.h>

#include <vector>

namespace webrtc {
namespace rtcp {

class TargetBitrate {
 public:
  // TODO(sprang): This block type is just a place holder. We need to get an
  //               id assigned by IANA.
  static constexpr uint8_t kBlockType = 42;
  static constexpr size_t kBitrateItemSizeBytes = 4;

  struct BitrateItem {
    BitrateItem();
    BitrateItem(uint8_t spatial_layer,
                uint8_t temporal_layer,
                uint32_t target_bitrate_kbps);

    uint8_t spatial_layer;
    uint8_t temporal_layer;
    uint32_t target_bitrate_kbps;
  };

  TargetBitrate();
  TargetBitrate(const TargetBitrate&);
  TargetBitrate& operator=(const TargetBitrate&);
  ~TargetBitrate();

  void AddTargetBitrate(uint8_t spatial_layer,
                        uint8_t temporal_layer,
                        uint32_t target_bitrate_kbps);

  const std::vector<BitrateItem>& GetTargetBitrates() const;

  void Parse(const uint8_t* block, uint16_t block_length);

  size_t BlockLength() const;

  void Create(uint8_t* buffer) const;

 private:
  std::vector<BitrateItem> bitrates_;
};

}  // namespace rtcp
}  // namespace webrtc
#endif  // MODULES_RTP_RTCP_SOURCE_RTCP_PACKET_TARGET_BITRATE_H_
