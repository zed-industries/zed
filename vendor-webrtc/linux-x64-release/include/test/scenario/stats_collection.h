/*
 *  Copyright 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef TEST_SCENARIO_STATS_COLLECTION_H_
#define TEST_SCENARIO_STATS_COLLECTION_H_

#include <map>
#include <memory>
#include <optional>

#include "call/call.h"
#include "rtc_base/thread.h"
#include "test/logging/log_writer.h"
#include "test/scenario/performance_stats.h"

namespace webrtc {
namespace test {

struct VideoQualityAnalyzerConfig {
  double psnr_coverage = 1;
  Thread* thread = nullptr;
};

class VideoLayerAnalyzer {
 public:
  void HandleCapturedFrame(const VideoFramePair& sample);
  void HandleRenderedFrame(const VideoFramePair& sample);
  void HandleFramePair(VideoFramePair sample,
                       double psnr,
                       RtcEventLogOutput* writer);
  VideoQualityStats stats_;
  Timestamp last_capture_time_ = Timestamp::MinusInfinity();
  Timestamp last_render_time_ = Timestamp::MinusInfinity();
  Timestamp last_freeze_time_ = Timestamp::MinusInfinity();
  int skip_count_ = 0;
};

class VideoQualityAnalyzer {
 public:
  explicit VideoQualityAnalyzer(
      VideoQualityAnalyzerConfig config = VideoQualityAnalyzerConfig(),
      std::unique_ptr<RtcEventLogOutput> writer = nullptr);
  ~VideoQualityAnalyzer();
  void HandleFramePair(VideoFramePair sample);
  std::vector<VideoQualityStats> layer_stats() const;
  VideoQualityStats& stats();
  void PrintHeaders();
  void PrintFrameInfo(const VideoFramePair& sample);
  std::function<void(const VideoFramePair&)> Handler();

 private:
  void HandleFramePair(VideoFramePair sample, double psnr);
  const VideoQualityAnalyzerConfig config_;
  std::map<int, VideoLayerAnalyzer> layer_analyzers_;
  const std::unique_ptr<RtcEventLogOutput> writer_;
  std::optional<VideoQualityStats> cached_;
};

class CallStatsCollector {
 public:
  void AddStats(Call::Stats sample);
  CollectedCallStats& stats() { return stats_; }

 private:
  CollectedCallStats stats_;
};
class AudioReceiveStatsCollector {
 public:
  void AddStats(AudioReceiveStreamInterface::Stats sample);
  CollectedAudioReceiveStats& stats() { return stats_; }

 private:
  CollectedAudioReceiveStats stats_;
};
class VideoSendStatsCollector {
 public:
  void AddStats(VideoSendStream::Stats sample, Timestamp at_time);
  CollectedVideoSendStats& stats() { return stats_; }

 private:
  CollectedVideoSendStats stats_;
  Timestamp last_update_ = Timestamp::MinusInfinity();
  size_t last_fec_bytes_ = 0;
};
class VideoReceiveStatsCollector {
 public:
  void AddStats(VideoReceiveStreamInterface::Stats sample);
  CollectedVideoReceiveStats& stats() { return stats_; }

 private:
  CollectedVideoReceiveStats stats_;
};

struct CallStatsCollectors {
  CallStatsCollector call;
  AudioReceiveStatsCollector audio_receive;
  VideoSendStatsCollector video_send;
  VideoReceiveStatsCollector video_receive;
};

}  // namespace test
}  // namespace webrtc

#endif  // TEST_SCENARIO_STATS_COLLECTION_H_
