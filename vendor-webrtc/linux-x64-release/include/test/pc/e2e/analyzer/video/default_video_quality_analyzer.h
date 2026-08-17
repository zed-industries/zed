/*
 *  Copyright (c) 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef TEST_PC_E2E_ANALYZER_VIDEO_DEFAULT_VIDEO_QUALITY_ANALYZER_H_
#define TEST_PC_E2E_ANALYZER_VIDEO_DEFAULT_VIDEO_QUALITY_ANALYZER_H_

#include <atomic>
#include <cstdint>
#include <deque>
#include <map>
#include <memory>
#include <set>
#include <string>
#include <unordered_map>
#include <vector>

#include "api/array_view.h"
#include "api/test/metrics/metrics_logger.h"
#include "api/test/video_quality_analyzer_interface.h"
#include "api/units/data_size.h"
#include "api/units/timestamp.h"
#include "api/video/encoded_image.h"
#include "api/video/video_frame.h"
#include "rtc_base/synchronization/mutex.h"
#include "rtc_base/thread_annotations.h"
#include "system_wrappers/include/clock.h"
#include "test/pc/e2e/analyzer/video/default_video_quality_analyzer_cpu_measurer.h"
#include "test/pc/e2e/analyzer/video/default_video_quality_analyzer_frame_in_flight.h"
#include "test/pc/e2e/analyzer/video/default_video_quality_analyzer_frames_comparator.h"
#include "test/pc/e2e/analyzer/video/default_video_quality_analyzer_internal_shared_objects.h"
#include "test/pc/e2e/analyzer/video/default_video_quality_analyzer_shared_objects.h"
#include "test/pc/e2e/analyzer/video/default_video_quality_analyzer_stream_state.h"
#include "test/pc/e2e/analyzer/video/dvqa/frames_storage.h"
#include "test/pc/e2e/analyzer/video/names_collection.h"

namespace webrtc {

class DefaultVideoQualityAnalyzer : public VideoQualityAnalyzerInterface {
 public:
  DefaultVideoQualityAnalyzer(webrtc::Clock* clock,
                              test::MetricsLogger* metrics_logger,
                              DefaultVideoQualityAnalyzerOptions options = {});
  ~DefaultVideoQualityAnalyzer() override;

  void Start(std::string test_case_name,
             ArrayView<const std::string> peer_names,
             int max_threads_count) override;
  uint16_t OnFrameCaptured(absl::string_view peer_name,
                           const std::string& stream_label,
                           const VideoFrame& frame) override;
  void OnFramePreEncode(absl::string_view peer_name,
                        const VideoFrame& frame) override;
  void OnFrameEncoded(absl::string_view peer_name,
                      uint16_t frame_id,
                      const EncodedImage& encoded_image,
                      const EncoderStats& stats,
                      bool discarded) override;
  void OnFrameDropped(absl::string_view peer_name,
                      EncodedImageCallback::DropReason reason) override;
  void OnFramePreDecode(absl::string_view peer_name,
                        uint16_t frame_id,
                        const EncodedImage& input_image) override;
  void OnFrameDecoded(absl::string_view peer_name,
                      const VideoFrame& frame,
                      const DecoderStats& stats) override;
  void OnFrameRendered(absl::string_view peer_name,
                       const VideoFrame& frame) override;
  void OnEncoderError(absl::string_view peer_name,
                      const VideoFrame& frame,
                      int32_t error_code) override;
  void OnDecoderError(absl::string_view peer_name,
                      uint16_t frame_id,
                      int32_t error_code,
                      const DecoderStats& stats) override;

  void RegisterParticipantInCall(absl::string_view peer_name) override;
  void UnregisterParticipantInCall(absl::string_view peer_name) override;
  void OnPauseAllStreamsFrom(absl::string_view sender_peer_name,
                             absl::string_view receiver_peer_name) override;
  void OnResumeAllStreamsFrom(absl::string_view sender_peer_name,
                              absl::string_view receiver_peer_name) override;

  void Stop() override;
  std::string GetStreamLabel(uint16_t frame_id) override;
  std::string GetSenderPeerName(uint16_t frame_id) const override;
  void OnStatsReports(
      absl::string_view pc_label,
      const scoped_refptr<const RTCStatsReport>& report) override {}

  // Returns set of stream labels, that were met during test call.
  std::set<StatsKey> GetKnownVideoStreams() const;
  VideoStreamsInfo GetKnownStreams() const;
  FrameCounters GetGlobalCounters() const;
  // Returns frame counter for frames received without frame id set.
  std::map<std::string, FrameCounters> GetUnknownSenderFrameCounters() const;
  // Returns frame counter per stream label. Valid stream labels can be obtained
  // by calling GetKnownVideoStreams()
  std::map<StatsKey, FrameCounters> GetPerStreamCounters() const;
  // Returns video quality stats per stream label. Valid stream labels can be
  // obtained by calling GetKnownVideoStreams()
  std::map<StatsKey, StreamStats> GetStats() const;
  AnalyzerStats GetAnalyzerStats() const;
  double GetCpuUsagePercent() const;

  // Returns mapping from the stream label to the history of frames that were
  // met in this stream in the order as they were captured.
  std::map<std::string, std::vector<uint16_t>> GetStreamFrames() const;

 private:
  enum State { kNew, kActive, kStopped };

  // Returns next frame id to use. Frame ID can't be `VideoFrame::kNotSetId`,
  // because this value is reserved by `VideoFrame` as "ID not set".
  uint16_t GetNextFrameId() RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_);

  void AddExistingFramesInFlightForStreamToComparator(
      size_t stream_index,
      AnalyzerStreamState& stream_state,
      size_t peer_index) RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_);

  // Processes frames for the peer identified by `peer_index` up to
  // `rendered_frame_id` (excluded). Sends each dropped frame for comparison and
  // discards superfluous frames (they were not expected to be received by
  // `peer_index` and not accounted in the stats).
  // Returns number of dropped frames.
  int ProcessNotSeenFramesBeforeRendered(size_t peer_index,
                                         uint16_t rendered_frame_id,
                                         const InternalStatsKey& stats_key,
                                         AnalyzerStreamState& state)
      RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_);

  // Report results for all metrics for all streams.
  void ReportResults();
  void ReportResults(const InternalStatsKey& key,
                     const StreamStats& stats,
                     const FrameCounters& frame_counters)
      RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_);
  // Returns name of current test case for reporting.
  std::string GetTestCaseName(const std::string& stream_label) const;
  Timestamp Now();
  StatsKey ToStatsKey(const InternalStatsKey& key) const
      RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_);
  // Returns string representation of stats key for metrics naming. Used for
  // backward compatibility by metrics naming for 2 peers cases.
  std::string ToMetricName(const InternalStatsKey& key) const
      RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_);
  std::string GetStreamLabelInternal(uint16_t frame_id) const
      RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_);

  static const uint16_t kStartingFrameId = 1;

  const DefaultVideoQualityAnalyzerOptions options_;
  webrtc::Clock* const clock_;
  test::MetricsLogger* const metrics_logger_;

  std::string test_label_;

  mutable Mutex mutex_;
  uint16_t next_frame_id_ RTC_GUARDED_BY(mutex_) = kStartingFrameId;
  std::unique_ptr<NamesCollection> peers_ RTC_GUARDED_BY(mutex_);
  State state_ RTC_GUARDED_BY(mutex_) = State::kNew;
  Timestamp start_time_ RTC_GUARDED_BY(mutex_) = Timestamp::MinusInfinity();
  // Mapping from stream label to unique size_t value to use in stats and avoid
  // extra string copying.
  NamesCollection streams_ RTC_GUARDED_BY(mutex_);
  FramesStorage frames_storage_ RTC_GUARDED_BY(mutex_);
  // Frames that were captured by all streams and still aren't rendered on
  // receivers or deemed dropped. Frame with id X can be removed from this map
  // if:
  // 1. The frame with id X was received in OnFrameRendered by all expected
  //    receivers.
  // 2. The frame with id Y > X was received in OnFrameRendered by all expected
  //    receivers.
  // 3. Next available frame id for newly captured frame is X
  // 4. There too many frames in flight for current video stream and X is the
  //    oldest frame id in this stream. In such case only the frame content
  //    will be removed, but the map entry will be preserved.
  std::unordered_map<uint16_t, FrameInFlight> captured_frames_in_flight_
      RTC_GUARDED_BY(mutex_);
  // Global frames count for all video streams.
  FrameCounters frame_counters_ RTC_GUARDED_BY(mutex_);
  // Frame counters for received frames without video frame id set.
  // Map from peer name to the frame counters.
  std::map<std::string, FrameCounters> unknown_sender_frame_counters_
      RTC_GUARDED_BY(mutex_);
  // Frame counters per each stream per each receiver.
  std::map<InternalStatsKey, FrameCounters> stream_frame_counters_
      RTC_GUARDED_BY(mutex_);
  // Map from stream index in `streams_` to its StreamState.
  std::unordered_map<size_t, AnalyzerStreamState> stream_states_
      RTC_GUARDED_BY(mutex_);
  // Map from stream index in `streams_` to sender peer index in `peers_`.
  std::unordered_map<size_t, size_t> stream_to_sender_ RTC_GUARDED_BY(mutex_);

  // Stores history mapping between stream index in `streams_` and frame ids.
  // Updated when frame id overlap. It required to properly return stream label
  // after 1st frame from simulcast streams was already rendered and last is
  // still encoding.
  std::unordered_map<size_t, std::set<uint16_t>> stream_to_frame_id_history_
      RTC_GUARDED_BY(mutex_);
  // Map from stream index to the list of frames as they were met in the stream.
  std::unordered_map<size_t, std::vector<uint16_t>>
      stream_to_frame_id_full_history_ RTC_GUARDED_BY(mutex_);
  AnalyzerStats analyzer_stats_ RTC_GUARDED_BY(mutex_);

  DefaultVideoQualityAnalyzerCpuMeasurer cpu_measurer_;
  DefaultVideoQualityAnalyzerFramesComparator frames_comparator_;
};

}  // namespace webrtc

#endif  // TEST_PC_E2E_ANALYZER_VIDEO_DEFAULT_VIDEO_QUALITY_ANALYZER_H_
