/*
 *  Copyright (c) 2016 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_VIDEO_CODING_H264_SPS_PPS_TRACKER_H_
#define MODULES_VIDEO_CODING_H264_SPS_PPS_TRACKER_H_

#include <cstddef>
#include <cstdint>
#include <map>
#include <vector>

#include "api/array_view.h"
#include "modules/rtp_rtcp/source/rtp_video_header.h"
#include "rtc_base/buffer.h"
#include "rtc_base/copy_on_write_buffer.h"

namespace webrtc {
namespace video_coding {

class H264SpsPpsTracker {
 public:
  enum PacketAction { kInsert, kDrop, kRequestKeyframe };
  struct FixedBitstream {
    PacketAction action;
    CopyOnWriteBuffer bitstream;
  };

  H264SpsPpsTracker() = default;
  H264SpsPpsTracker(const H264SpsPpsTracker& other) = default;
  H264SpsPpsTracker& operator=(const H264SpsPpsTracker& other) = default;
  ~H264SpsPpsTracker() = default;

  // Returns fixed bitstream and modifies `video_header`.
  FixedBitstream CopyAndFixBitstream(ArrayView<const uint8_t> bitstream,
                                     RTPVideoHeader* video_header);

  void InsertSpsPpsNalus(const std::vector<uint8_t>& sps,
                         const std::vector<uint8_t>& pps);

 private:
  struct PpsInfo {
    int sps_id = -1;
    Buffer data;
  };

  struct SpsInfo {
    int width = -1;
    int height = -1;
    Buffer data;
  };

  std::map<int, PpsInfo> pps_data_;
  std::map<int, SpsInfo> sps_data_;
};

}  // namespace video_coding
}  // namespace webrtc

#endif  // MODULES_VIDEO_CODING_H264_SPS_PPS_TRACKER_H_
