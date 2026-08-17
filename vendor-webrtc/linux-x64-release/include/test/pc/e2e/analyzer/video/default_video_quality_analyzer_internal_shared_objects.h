/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef TEST_PC_E2E_ANALYZER_VIDEO_DEFAULT_VIDEO_QUALITY_ANALYZER_INTERNAL_SHARED_OBJECTS_H_
#define TEST_PC_E2E_ANALYZER_VIDEO_DEFAULT_VIDEO_QUALITY_ANALYZER_INTERNAL_SHARED_OBJECTS_H_

#include <memory>
#include <optional>
#include <string>
#include <utility>
#include <vector>

#include "api/numerics/samples_stats_counter.h"
#include "api/units/data_size.h"
#include "api/units/timestamp.h"
#include "api/video/video_frame.h"
#include "api/video/video_frame_type.h"
#include "test/pc/e2e/analyzer/video/default_video_quality_analyzer_shared_objects.h"

namespace webrtc {

struct InternalStatsKey {
  InternalStatsKey(size_t stream, size_t sender, size_t receiver)
      : stream(stream), sender(sender), receiver(receiver) {}

  std::string ToString() const;

  size_t stream;
  size_t sender;
  size_t receiver;
};

// Required to use InternalStatsKey as std::map key.
bool operator<(const InternalStatsKey& a, const InternalStatsKey& b);
bool operator==(const InternalStatsKey& a, const InternalStatsKey& b);

// Final stats computed for frame after it went through the whole video
// pipeline from capturing to rendering or dropping.
struct FrameStats {
  FrameStats(uint16_t frame_id, Timestamp captured_time)
      : frame_id(frame_id), captured_time(captured_time) {}

  uint16_t frame_id;
  // Frame events timestamp.
  Timestamp captured_time;
  Timestamp pre_encode_time = Timestamp::MinusInfinity();
  Timestamp encoded_time = Timestamp::MinusInfinity();
  // Time when last packet of a frame was received.
  Timestamp received_time = Timestamp::MinusInfinity();
  Timestamp decode_start_time = Timestamp::MinusInfinity();
  Timestamp decode_end_time = Timestamp::MinusInfinity();
  Timestamp rendered_time = Timestamp::MinusInfinity();

  // Next timings are set if and only if previous frame exist.
  std::optional<Timestamp> prev_frame_rendered_time = std::nullopt;
  std::optional<TimeDelta> time_between_captured_frames = std::nullopt;
  std::optional<TimeDelta> time_between_encoded_frames = std::nullopt;
  std::optional<TimeDelta> time_between_rendered_frames = std::nullopt;

  VideoFrameType encoded_frame_type = VideoFrameType::kEmptyFrame;
  DataSize encoded_image_size = DataSize::Bytes(0);
  VideoFrameType pre_decoded_frame_type = VideoFrameType::kEmptyFrame;
  DataSize pre_decoded_image_size = DataSize::Bytes(0);
  uint32_t target_encode_bitrate = 0;
  // Sender side qp values per spatial layer. In case when spatial layer is not
  // set for `webrtc::EncodedImage`, 0 is used as default.
  std::map<int, SamplesStatsCounter> spatial_layers_qp;
  // Receive side qp value. Receiver only renders one spatial layer for a given
  // time index. The QP value here corresponds to one of the encoded spatial
  // layer's QP given in `spatial_layers_qp`, i.e. to the one that corresponds
  // to the rendered frame.
  std::optional<uint8_t> decoded_frame_qp = std::nullopt;

  std::optional<int> decoded_frame_width = std::nullopt;
  std::optional<int> decoded_frame_height = std::nullopt;

  // Can be not set if frame was dropped by encoder.
  std::optional<StreamCodecInfo> used_encoder = std::nullopt;
  // Can be not set if frame was dropped in the network.
  std::optional<StreamCodecInfo> used_decoder = std::nullopt;

  bool decoder_failed = false;
};

// Describes why comparison was done in overloaded mode (without calculating
// PSNR and SSIM).
enum class OverloadReason {
  kNone,
  // Not enough CPU to process all incoming comparisons.
  kCpu,
  // Not enough memory to store captured frames for all comparisons.
  kMemory
};

enum class FrameComparisonType {
  // Comparison for captured and rendered frame.
  kRegular,
  // Comparison for captured frame that is known to be dropped somewhere in
  // video pipeline.
  kDroppedFrame,
  // Comparison for captured frame that was still in the video pipeline when
  // test was stopped. It's unknown is this frame dropped or would it be
  // delivered if test continue.
  kFrameInFlight
};

// Represents comparison between two VideoFrames. Contains video frames itself
// and stats. Can be one of two types:
//   1. Normal - in this case `captured` is presented and either `rendered` is
//      presented and `dropped` is false, either `rendered` is omitted and
//      `dropped` is true.
//   2. Overloaded - in this case both `captured` and `rendered` are omitted
//      because there were too many comparisons in the queue. `dropped` can be
//      true or false showing was frame dropped or not.
struct FrameComparison {
  FrameComparison(InternalStatsKey stats_key,
                  std::optional<VideoFrame> captured,
                  std::optional<VideoFrame> rendered,
                  FrameComparisonType type,
                  FrameStats frame_stats,
                  OverloadReason overload_reason);

  InternalStatsKey stats_key;
  // Frames can be omitted if there too many computations waiting in the
  // queue.
  std::optional<VideoFrame> captured;
  std::optional<VideoFrame> rendered;
  FrameComparisonType type;
  FrameStats frame_stats;
  OverloadReason overload_reason;
};

}  // namespace webrtc

#endif  // TEST_PC_E2E_ANALYZER_VIDEO_DEFAULT_VIDEO_QUALITY_ANALYZER_INTERNAL_SHARED_OBJECTS_H_
