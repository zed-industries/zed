/*
 *  Copyright (c) 2016 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_VIDEO_CODING_H264_SPROP_PARAMETER_SETS_H_
#define MODULES_VIDEO_CODING_H264_SPROP_PARAMETER_SETS_H_

#include <cstdint>
#include <string>
#include <vector>

namespace webrtc {

class H264SpropParameterSets {
 public:
  H264SpropParameterSets() {}

  H264SpropParameterSets(const H264SpropParameterSets&) = delete;
  H264SpropParameterSets& operator=(const H264SpropParameterSets&) = delete;

  bool DecodeSprop(const std::string& sprop);
  const std::vector<uint8_t>& sps_nalu() { return sps_; }
  const std::vector<uint8_t>& pps_nalu() { return pps_; }

 private:
  std::vector<uint8_t> sps_;
  std::vector<uint8_t> pps_;
};

}  // namespace webrtc

#endif  // MODULES_VIDEO_CODING_H264_SPROP_PARAMETER_SETS_H_
