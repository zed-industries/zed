/*
 *  Copyright (c) 2023 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef COMMON_VIDEO_H265_H265_BITSTREAM_PARSER_H_
#define COMMON_VIDEO_H265_H265_BITSTREAM_PARSER_H_

#include <stddef.h>
#include <stdint.h>

#include <optional>
#include <vector>

#include "api/video_codecs/bitstream_parser.h"
#include "common_video/h265/h265_pps_parser.h"
#include "common_video/h265/h265_sps_parser.h"
#include "common_video/h265/h265_vps_parser.h"
#include "rtc_base/containers/flat_map.h"
#include "rtc_base/system/rtc_export.h"

namespace webrtc {

// Stateful H265 bitstream parser (due to VPS/SPS/PPS). Used to parse out QP
// values from the bitstream.
class RTC_EXPORT H265BitstreamParser : public BitstreamParser {
 public:
  H265BitstreamParser();
  ~H265BitstreamParser() override;

  // New interface.
  void ParseBitstream(ArrayView<const uint8_t> bitstream) override;
  std::optional<int> GetLastSliceQp() const override;

  std::optional<uint32_t> GetLastSlicePpsId() const;
  
  bool GetLastHasBadRbsp() const;

  static std::optional<uint32_t> ParsePpsIdFromSliceSegmentLayerRbsp(
      ArrayView<const uint8_t> data,
      uint8_t nalu_type);

  // Returns true if the slice segment is the first in the picture; otherwise
  // return false. If parse failed, return nullopt.
  static std::optional<bool> IsFirstSliceSegmentInPic(
      ArrayView<const uint8_t> data);

 protected:
  enum Result {
    kOk,
    kInvalidStream,
    kUnsupportedStream,
  };
  void ParseSlice(ArrayView<const uint8_t> slice);
  Result ParseNonParameterSetNalu(ArrayView<const uint8_t> source,
                                  uint8_t nalu_type);

  const H265PpsParser::PpsState* GetPPS(uint32_t id) const;
  const H265SpsParser::SpsState* GetSPS(uint32_t id) const;

  // VPS/SPS/PPS state, updated when parsing new VPS/SPS/PPS, used to parse
  // slices.
  flat_map<uint32_t, H265VpsParser::VpsState> vps_;
  flat_map<uint32_t, H265SpsParser::SpsState> sps_;
  flat_map<uint32_t, H265PpsParser::PpsState> pps_;

  // Last parsed slice QP.
  std::optional<int32_t> last_slice_qp_delta_;
  std::optional<uint32_t> last_slice_pps_id_;
  
  bool lastHasBadRbsp_;
};

}  // namespace webrtc

#endif  // COMMON_VIDEO_H265_H265_BITSTREAM_PARSER_H_
