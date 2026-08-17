/*
 *  Copyright (c) 2023 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef COMMON_VIDEO_H265_H265_VPS_PARSER_H_
#define COMMON_VIDEO_H265_H265_VPS_PARSER_H_

#include <optional>

#include "api/array_view.h"
#include "rtc_base/system/rtc_export.h"

namespace webrtc {

// A class for parsing out video parameter set (VPS) data from an H265 NALU.
class RTC_EXPORT H265VpsParser {
 public:
    static constexpr uint32_t kMaxSubLayers = 7;

  // The parsed state of the VPS. Only some select values are stored.
  // Add more as they are actually needed.
  struct RTC_EXPORT VpsState {
    VpsState();

    uint32_t id = 0;
    uint32_t vps_max_sub_layers_minus1 = 0;
    uint32_t vps_max_num_reorder_pics[kMaxSubLayers] = {};
  };

  // Unpack RBSP and parse VPS state from the supplied buffer.
  static std::optional<VpsState> ParseVps(ArrayView<const uint8_t> data);
  // TODO: bugs.webrtc.org/42225170 - Deprecate.
  static inline std::optional<VpsState> ParseVps(const uint8_t* data,
                                                 size_t length) {
    return ParseVps(MakeArrayView(data, length));
  }

 protected:
  // Parse the VPS state, for a bit buffer where RBSP decoding has already been
  // performed.
  static std::optional<VpsState> ParseInternal(ArrayView<const uint8_t> buffer);
};

}  // namespace webrtc
#endif  // COMMON_VIDEO_H265_H265_VPS_PARSER_H_
