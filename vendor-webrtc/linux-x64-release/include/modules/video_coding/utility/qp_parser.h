/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_VIDEO_CODING_UTILITY_QP_PARSER_H_
#define MODULES_VIDEO_CODING_UTILITY_QP_PARSER_H_

#include <cstddef>
#include <cstdint>
#include <optional>

#include "api/video/video_codec_constants.h"
#include "api/video/video_codec_type.h"
#include "common_video/h264/h264_bitstream_parser.h"
#include "rtc_base/synchronization/mutex.h"
#include "rtc_base/thread_annotations.h"

#ifdef RTC_ENABLE_H265
#include "common_video/h265/h265_bitstream_parser.h"
#endif

namespace webrtc {
class QpParser {
 public:
  std::optional<uint32_t> Parse(VideoCodecType codec_type,
                                size_t spatial_idx,
                                const uint8_t* frame_data,
                                size_t frame_size);

 private:
  // A thread safe wrapper for H264 bitstream parser.
  class H264QpParser {
   public:
    std::optional<uint32_t> Parse(const uint8_t* frame_data, size_t frame_size);

   private:
    Mutex mutex_;
    H264BitstreamParser bitstream_parser_ RTC_GUARDED_BY(mutex_);
  };

  H264QpParser h264_parsers_[kMaxSimulcastStreams];

#ifdef RTC_ENABLE_H265
  // A thread safe wrapper for H.265 bitstream parser.
  class H265QpParser {
   public:
    std::optional<uint32_t> Parse(const uint8_t* frame_data, size_t frame_size);

   private:
    Mutex mutex_;
    H265BitstreamParser bitstream_parser_ RTC_GUARDED_BY(mutex_);
  };

  H265QpParser h265_parsers_[kMaxSimulcastStreams];
#endif
};

}  // namespace webrtc

#endif  // MODULES_VIDEO_CODING_UTILITY_QP_PARSER_H_
