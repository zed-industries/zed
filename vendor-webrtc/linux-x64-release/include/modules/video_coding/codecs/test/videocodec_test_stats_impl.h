/*
 *  Copyright (c) 2011 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_VIDEO_CODING_CODECS_TEST_VIDEOCODEC_TEST_STATS_IMPL_H_
#define MODULES_VIDEO_CODING_CODECS_TEST_VIDEOCODEC_TEST_STATS_IMPL_H_

#include <stddef.h>

#include <map>
#include <optional>
#include <vector>

#include "api/test/videocodec_test_stats.h"  // NOLINT(build/include)
#include "api/units/data_rate.h"
#include "api/units/frequency.h"

namespace webrtc {
namespace test {

// Statistics for a sequence of processed frames. This class is not thread safe.
class VideoCodecTestStatsImpl : public VideoCodecTestStats {
 public:
  VideoCodecTestStatsImpl();
  ~VideoCodecTestStatsImpl() override;

  // Creates a FrameStatistics for the next frame to be processed.
  void AddFrame(const FrameStatistics& frame_stat);

  // Returns the FrameStatistics corresponding to `frame_number` or `timestamp`.
  FrameStatistics* GetFrame(size_t frame_number, size_t spatial_idx);
  FrameStatistics* GetFrameWithTimestamp(size_t timestamp, size_t spatial_idx);

  // Creates FrameStatisticts if it doesn't exists and/or returns
  // created/existing FrameStatisticts.
  FrameStatistics* GetOrAddFrame(size_t timestamp_rtp, size_t spatial_idx);

  // Implements VideoCodecTestStats.
  std::vector<FrameStatistics> GetFrameStatistics() const override;
  std::vector<VideoStatistics> SliceAndCalcLayerVideoStatistic(
      size_t first_frame_num,
      size_t last_frame_num) override;

  VideoStatistics SliceAndCalcAggregatedVideoStatistic(size_t first_frame_num,
                                                       size_t last_frame_num);

  VideoStatistics CalcVideoStatistic(size_t first_frame,
                                     size_t last_frame,
                                     DataRate target_bitrate,
                                     Frequency target_framerate) override;

  size_t Size(size_t spatial_idx);

  void Clear();

 private:
  VideoCodecTestStats::FrameStatistics AggregateFrameStatistic(
      size_t frame_num,
      size_t spatial_idx,
      bool aggregate_independent_layers);

  size_t CalcLayerTargetBitrateKbps(size_t first_frame_num,
                                    size_t last_frame_num,
                                    size_t spatial_idx,
                                    size_t temporal_idx,
                                    bool aggregate_independent_layers);

  VideoCodecTestStats::VideoStatistics SliceAndCalcVideoStatistic(
      size_t first_frame_num,
      size_t last_frame_num,
      size_t spatial_idx,
      size_t temporal_idx,
      bool aggregate_independent_layers,
      std::optional<DataRate> target_bitrate,
      std::optional<Frequency> target_framerate);

  void GetNumberOfEncodedLayers(size_t first_frame_num,
                                size_t last_frame_num,
                                size_t* num_encoded_spatial_layers,
                                size_t* num_encoded_temporal_layers);

  // layer_idx -> stats.
  std::map<size_t, std::vector<FrameStatistics>> layer_stats_;
  // layer_idx -> rtp_timestamp -> frame_num.
  std::map<size_t, std::map<size_t, size_t>> rtp_timestamp_to_frame_num_;
};

}  // namespace test
}  // namespace webrtc

#endif  // MODULES_VIDEO_CODING_CODECS_TEST_VIDEOCODEC_TEST_STATS_IMPL_H_
