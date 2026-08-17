/*
 *  Copyright (c) 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_VIDEO_CODECS_BITSTREAM_PARSER_H_
#define API_VIDEO_CODECS_BITSTREAM_PARSER_H_

#include <stddef.h>
#include <stdint.h>

#include <optional>

#include "api/array_view.h"

namespace webrtc {

// This class is an interface for bitstream parsers.
class BitstreamParser {
 public:
  virtual ~BitstreamParser() = default;

  // Parse an additional chunk of the bitstream.
  virtual void ParseBitstream(ArrayView<const uint8_t> bitstream) = 0;

  // Get the last extracted QP value from the parsed bitstream. If no QP
  // value could be parsed, returns std::nullopt.
  virtual std::optional<int> GetLastSliceQp() const = 0;
};

}  // namespace webrtc

#endif  // API_VIDEO_CODECS_BITSTREAM_PARSER_H_
