/*
 *  Copyright 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef TEST_SCENARIO_PERFORMANCE_STATS_H_
#define TEST_SCENARIO_PERFORMANCE_STATS_H_

#include "api/units/data_rate.h"
#include "api/units/time_delta.h"
#include "api/units/timestamp.h"
#include "api/video/video_frame_buffer.h"
#include "rtc_base/numerics/event_rate_counter.h"
#include "rtc_base/numerics/sample_stats.h"

namespace webrtc {
namespace test {

struct VideoFramePair {
  scoped_refptr<VideoFrameBuffer> captured;
  scoped_refptr<VideoFrameBuffer> decoded;
  Timestamp capture_time = Timestamp::MinusInfinity();
  Timestamp decoded_time = Timestamp::PlusInfinity();
  Timestamp render_time = Timestamp::PlusInfinity();
  // A unique identifier for the spatial/temporal layer the decoded frame
  // belongs to. Note that this does not reflect the id as defined by the
  // underlying layer setup.
  int layer_id = 0;
  int capture_id = 0;
  int decode_id = 0;
  // Indicates the repeat count for the decoded frame. Meaning that the same
  // decoded frame has matched differend captured frames.
  int repeated = 0;
};

struct VideoFramesStats {
  int count = 0;
  SampleStats<double> pixels;
  SampleStats<double> resolution;
  EventRateCounter frames;
  void AddFrameInfo(const VideoFrameBuffer& frame, Timestamp at_time);
  void AddStats(const VideoFramesStats& other);
};

struct VideoQualityStats {
  int lost_count = 0;
  int freeze_count = 0;
  VideoFramesStats capture;
  VideoFramesStats render;
  // Time from frame was captured on device to time frame was delivered from
  // decoder.
  SampleStats<TimeDelta> capture_to_decoded_delay;
  // Time from frame was captured on device to time frame was displayed on
  // device.
  SampleStats<TimeDelta> end_to_end_delay;
  // PSNR for delivered frames. Note that this might go up for a worse
  // connection due to frame dropping.
  SampleStats<double> psnr;
  // PSNR for all frames, dropped or lost frames are compared to the last
  // successfully delivered frame
  SampleStats<double> psnr_with_freeze;
  // Frames skipped between two nearest.
  SampleStats<double> skipped_between_rendered;
  // In the next 2 metrics freeze is a pause that is longer, than maximum:
  //  1. 150ms
  //  2. 3 * average time between two sequential frames.
  // Item 1 will cover high fps video and is a duration, that is noticeable by
  // human eye. Item 2 will cover low fps video like screen sharing.
  SampleStats<TimeDelta> freeze_duration;
  // Mean time between one freeze end and next freeze start.
  SampleStats<TimeDelta> time_between_freezes;
  void AddStats(const VideoQualityStats& other);
};

struct CollectedCallStats {
  SampleStats<DataRate> target_rate;
  SampleStats<TimeDelta> pacer_delay;
  SampleStats<TimeDelta> round_trip_time;
  SampleStats<double> memory_usage;
};

struct CollectedAudioReceiveStats {
  SampleStats<double> expand_rate;
  SampleStats<double> accelerate_rate;
  SampleStats<TimeDelta> jitter_buffer;
};
struct CollectedVideoSendStats {
  SampleStats<double> encode_frame_rate;
  SampleStats<TimeDelta> encode_time;
  SampleStats<double> encode_usage;
  SampleStats<DataRate> media_bitrate;
  SampleStats<DataRate> fec_bitrate;
};
struct CollectedVideoReceiveStats {
  SampleStats<TimeDelta> decode_time;
  SampleStats<TimeDelta> decode_time_max;
  SampleStats<double> decode_pixels;
  SampleStats<double> resolution;
};

}  // namespace test
}  // namespace webrtc
#endif  // TEST_SCENARIO_PERFORMANCE_STATS_H_
