/*
 *  Copyright (c) 2023 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_VIDEO_CODECS_VIDEO_ENCODER_INTERFACE_H_
#define API_VIDEO_CODECS_VIDEO_ENCODER_INTERFACE_H_

#include <cstdint>
#include <memory>
#include <optional>
#include <variant>
#include <vector>

#include "api/array_view.h"
#include "api/scoped_refptr.h"
#include "api/units/data_rate.h"
#include "api/units/data_size.h"
#include "api/units/time_delta.h"
#include "api/units/timestamp.h"
#include "api/video/resolution.h"
#include "api/video/video_frame_buffer.h"
#include "api/video_codecs/video_codec.h"

namespace webrtc {
// NOTE: This class is still under development and may change without notice.
class VideoEncoderInterface {
 public:
  virtual ~VideoEncoderInterface() = default;
  enum class FrameType { kKeyframe, kStartFrame, kDeltaFrame };

  struct EncodingError {};
  struct EncodedData {
    FrameType frame_type;
    int encoded_qp;
  };
  using EncodeResult = std::variant<EncodingError, EncodedData>;

  struct FrameOutput {
    virtual ~FrameOutput() = default;
    virtual ArrayView<uint8_t> GetBitstreamOutputBuffer(DataSize size) = 0;
    virtual void EncodeComplete(const EncodeResult& encode_result) = 0;
  };

  struct TemporalUnitSettings {
    VideoCodecMode content_hint = VideoCodecMode::kRealtimeVideo;
    Timestamp presentation_timestamp;
  };

  struct FrameEncodeSettings {
    struct Cbr {
      TimeDelta duration;
      DataRate target_bitrate;
    };

    struct Cqp {
      int target_qp;
    };

    std::variant<Cqp, Cbr> rate_options;

    FrameType frame_type = FrameType::kDeltaFrame;
    int temporal_id = 0;
    int spatial_id = 0;
    Resolution resolution;
    std::vector<int> reference_buffers;
    std::optional<int> update_buffer;
    int effort_level = 0;

    std::unique_ptr<FrameOutput> frame_output;
  };

  virtual void Encode(scoped_refptr<webrtc::VideoFrameBuffer> frame_buffer,
                      const TemporalUnitSettings& settings,
                      std::vector<FrameEncodeSettings> frame_settings) = 0;
};

}  // namespace webrtc
#endif  // API_VIDEO_CODECS_VIDEO_ENCODER_INTERFACE_H_
