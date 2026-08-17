/*
 *  Copyright (c) 2016 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 *
 */

#ifndef COMMON_VIDEO_H264_SPS_VUI_REWRITER_H_
#define COMMON_VIDEO_H264_SPS_VUI_REWRITER_H_

#include <stddef.h>
#include <stdint.h>

#include <optional>

#include "api/video/color_space.h"
#include "common_video/h264/sps_parser.h"
#include "rtc_base/buffer.h"

namespace webrtc {

// A class that can parse an SPS+VUI and if necessary creates a copy with
// updated parameters.
// The rewriter disables frame buffering. This should force decoders to deliver
// decoded frame immediately and, thus, reduce latency.
// The rewriter updates video signal type parameters if external parameters are
// provided.
class SpsVuiRewriter : private SpsParser {
 public:
  enum class ParseResult { kFailure, kVuiOk, kVuiRewritten };
  enum class Direction { kIncoming, kOutgoing };

  // Parses an SPS block and if necessary copies it and rewrites the VUI.
  // Returns kFailure on failure, kParseOk if parsing succeeded and no update
  // was necessary and kParsedAndModified if an updated copy of buffer was
  // written to destination. destination may be populated with some data even if
  // no rewrite was necessary, but the end offset should remain unchanged.
  // Unless parsing fails, the sps parameter will be populated with the parsed
  // SPS state. This function assumes that any previous headers
  // (NALU start, type, Stap-A, etc) have already been parsed and that RBSP
  // decoding has been performed.
  static ParseResult ParseAndRewriteSps(ArrayView<const uint8_t> buffer,
                                        std::optional<SpsParser::SpsState>* sps,
                                        const ColorSpace* color_space,
                                        Buffer* destination,
                                        Direction Direction);

  // Parses NAL units from `buffer`, strips AUD blocks and rewrites VUI in SPS
  // blocks if necessary.
  static Buffer ParseOutgoingBitstreamAndRewrite(
      ArrayView<const uint8_t> buffer,
      const ColorSpace* color_space);

 private:
  static ParseResult ParseAndRewriteSps(ArrayView<const uint8_t> buffer,
                                        std::optional<SpsParser::SpsState>* sps,
                                        const ColorSpace* color_space,
                                        Buffer* destination);

  static void UpdateStats(ParseResult result, Direction direction);
};

}  // namespace webrtc

#endif  // COMMON_VIDEO_H264_SPS_VUI_REWRITER_H_
