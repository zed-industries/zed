/*
 *  Copyright (c) 2020 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef VIDEO_RECEIVE_STATISTICS_PROXY_H_
#define VIDEO_RECEIVE_STATISTICS_PROXY_H_

#include <cstddef>
#include <cstdint>
#include <map>
#include <memory>
#include <optional>

#include "absl/strings/string_view.h"
#include "api/sequence_checker.h"
#include "api/task_queue/pending_task_safety_flag.h"
#include "api/task_queue/task_queue_base.h"
#include "api/units/time_delta.h"
#include "api/video/video_codec_type.h"
#include "api/video/video_content_type.h"
#include "api/video/video_frame.h"
#include "api/video/video_frame_type.h"
#include "api/video/video_timing.h"
#include "api/video_codecs/video_decoder.h"
#include "call/video_receive_stream.h"
#include "common_video/frame_counts.h"
#include "modules/rtp_rtcp/include/rtcp_statistics.h"
#include "modules/rtp_rtcp/include/rtp_rtcp_defines.h"
#include "rtc_base/numerics/histogram_percentile_counter.h"
#include "rtc_base/numerics/moving_max_counter.h"
#include "rtc_base/numerics/running_statistics.h"
#include "rtc_base/numerics/sample_counter.h"
#include "rtc_base/rate_statistics.h"
#include "rtc_base/rate_tracker.h"
#include "rtc_base/system/no_unique_address.h"
#include "rtc_base/thread_annotations.h"
#include "video/stats_counter.h"
#include "video/video_quality_observer2.h"
#include "video/video_stream_buffer_controller.h"

namespace webrtc {

class Clock;
struct CodecSpecificInfo;

namespace internal {
// Declared in video_receive_stream2.h.
struct VideoFrameMetaData;

class ReceiveStatisticsProxy : public VideoStreamBufferControllerStatsObserver,
                               public RtcpCnameCallback,
                               public RtcpPacketTypeCounterObserver {
 public:
  ReceiveStatisticsProxy(uint32_t remote_ssrc,
                         Clock* clock,
                         TaskQueueBase* worker_thread);
  ~ReceiveStatisticsProxy() override;

  VideoReceiveStreamInterface::Stats GetStats() const;

  void OnDecodedFrame(const VideoFrame& frame,
                      std::optional<uint8_t> qp,
                      TimeDelta decode_time,
                      VideoContentType content_type,
                      VideoFrameType frame_type);

  // Called asyncronously on the worker thread as a result of a call to the
  // above OnDecodedFrame method, which is called back on the thread where
  // the actual decoding happens.
  void OnDecodedFrame(const VideoFrameMetaData& frame_meta,
                      std::optional<uint8_t> qp,
                      TimeDelta decode_time,
                      TimeDelta processing_delay,
                      TimeDelta assembly_time,
                      VideoContentType content_type,
                      VideoFrameType frame_type);

  void OnSyncOffsetUpdated(int64_t video_playout_ntp_ms,
                           int64_t sync_offset_ms,
                           double estimated_freq_khz);
  void OnRenderedFrame(const VideoFrameMetaData& frame_meta);
  void OnIncomingPayloadType(int payload_type);
  void OnDecoderInfo(const VideoDecoder::DecoderInfo& decoder_info);

  void OnPreDecode(VideoCodecType codec_type, int qp);

  void OnUniqueFramesCounted(int num_unique_frames);

  // Indicates video stream has been paused (no incoming packets).
  void OnStreamInactive();

  // Implements VideoStreamBufferControllerStatsObserver.
  void OnCompleteFrame(bool is_keyframe,
                       size_t size_bytes,
                       VideoContentType content_type) override;
  void OnDroppedFrames(uint32_t frames_dropped) override;
  void OnDecodableFrame(TimeDelta jitter_buffer_delay,
                        TimeDelta target_delay,
                        TimeDelta minimum_delay) override;
  void OnFrameBufferTimingsUpdated(int estimated_max_decode_time_ms,
                                   int current_delay_ms,
                                   int target_delay_ms,
                                   int jitter_delay_ms,
                                   int min_playout_delay_ms,
                                   int render_delay_ms) override;
  void OnTimingFrameInfoUpdated(const TimingFrameInfo& info) override;

  // Implements RtcpCnameCallback.
  void OnCname(uint32_t ssrc, absl::string_view cname) override;

  void OnCorruptionScore(double corruption_score,
                         VideoContentType content_type);

  // Implements RtcpPacketTypeCounterObserver.
  void RtcpPacketTypesCounterUpdated(
      uint32_t ssrc,
      const RtcpPacketTypeCounter& packet_counter) override;

  void OnRttUpdate(int64_t avg_rtt_ms);

  // Notification methods that are used to check our internal state and validate
  // threading assumptions. These are called by VideoReceiveStreamInterface.
  void DecoderThreadStarting();
  void DecoderThreadStopped();

  // Produce histograms. Must be called after DecoderThreadStopped(), typically
  // at the end of the call.
  void UpdateHistograms(std::optional<int> fraction_lost,
                        const StreamDataCounters& rtp_stats,
                        const StreamDataCounters* rtx_stats);

 private:
  struct QpCounters {
    SampleCounter vp8;
  };

  struct ContentSpecificStats {
    ContentSpecificStats();
    ~ContentSpecificStats();

    void Add(const ContentSpecificStats& other);

    SampleCounter e2e_delay_counter;
    SampleCounter interframe_delay_counter;
    int64_t flow_duration_ms = 0;
    int64_t total_media_bytes = 0;
    SampleCounter received_width;
    SampleCounter received_height;
    SampleCounter qp_counter;
    FrameCounts frame_counts;
    HistogramPercentileCounter interframe_delay_percentiles;
    webrtc_impl::RunningStatistics<double> corruption_score;
  };

  // Removes info about old frames and then updates the framerate.
  void UpdateFramerate(int64_t now_ms) const;

  std::optional<int64_t> GetCurrentEstimatedPlayoutNtpTimestampMs(
      int64_t now_ms) const;

  Clock* const clock_;
  const int64_t start_ms_;

  // Note: The `stats_.rtp_stats` member is not used or populated by this class.
  mutable VideoReceiveStreamInterface::Stats stats_
      RTC_GUARDED_BY(main_thread_);
  // Same as stats_.ssrc, but const (no lock required).
  const uint32_t remote_ssrc_;
  RateStatistics decode_fps_estimator_ RTC_GUARDED_BY(main_thread_);
  RateStatistics renders_fps_estimator_ RTC_GUARDED_BY(main_thread_);
  RateTracker render_fps_tracker_ RTC_GUARDED_BY(main_thread_);
  RateTracker render_pixel_tracker_ RTC_GUARDED_BY(main_thread_);
  SampleCounter sync_offset_counter_ RTC_GUARDED_BY(main_thread_);
  SampleCounter decode_time_counter_ RTC_GUARDED_BY(main_thread_);
  SampleCounter jitter_delay_counter_ RTC_GUARDED_BY(main_thread_);
  SampleCounter target_delay_counter_ RTC_GUARDED_BY(main_thread_);
  SampleCounter current_delay_counter_ RTC_GUARDED_BY(main_thread_);
  SampleCounter oneway_delay_counter_ RTC_GUARDED_BY(main_thread_);
  std::unique_ptr<VideoQualityObserver> video_quality_observer_
      RTC_GUARDED_BY(main_thread_);
  mutable MovingMaxCounter<int> interframe_delay_max_moving_
      RTC_GUARDED_BY(main_thread_);
  std::map<VideoContentType, ContentSpecificStats> content_specific_stats_
      RTC_GUARDED_BY(main_thread_);
  MaxCounter freq_offset_counter_ RTC_GUARDED_BY(main_thread_);
  QpCounters qp_counters_ RTC_GUARDED_BY(main_thread_);
  int64_t avg_rtt_ms_ RTC_GUARDED_BY(main_thread_) = 0;
  mutable std::map<int64_t, size_t> frame_window_ RTC_GUARDED_BY(main_thread_);
  VideoContentType last_content_type_ RTC_GUARDED_BY(&main_thread_);
  VideoCodecType last_codec_type_ RTC_GUARDED_BY(main_thread_);
  std::optional<int64_t> first_frame_received_time_ms_
      RTC_GUARDED_BY(main_thread_);
  std::optional<int64_t> first_decoded_frame_time_ms_
      RTC_GUARDED_BY(main_thread_);
  std::optional<int64_t> last_decoded_frame_time_ms_
      RTC_GUARDED_BY(main_thread_);
  size_t num_delayed_frames_rendered_ RTC_GUARDED_BY(main_thread_);
  int64_t sum_missed_render_deadline_ms_ RTC_GUARDED_BY(main_thread_);
  // Mutable because calling Max() on MovingMaxCounter is not const. Yet it is
  // called from const GetStats().
  mutable MovingMaxCounter<TimingFrameInfo> timing_frame_info_counter_
      RTC_GUARDED_BY(main_thread_);
  std::optional<int> num_unique_frames_ RTC_GUARDED_BY(main_thread_);
  std::optional<int64_t> last_estimated_playout_ntp_timestamp_ms_
      RTC_GUARDED_BY(main_thread_);
  std::optional<int64_t> last_estimated_playout_time_ms_
      RTC_GUARDED_BY(main_thread_);

  // The thread on which this instance is constructed and some of its main
  // methods are invoked on such as GetStats().
  TaskQueueBase* const worker_thread_;

  ScopedTaskSafety task_safety_;

  RTC_NO_UNIQUE_ADDRESS SequenceChecker decode_queue_;
  SequenceChecker main_thread_;
  RTC_NO_UNIQUE_ADDRESS SequenceChecker incoming_render_queue_;
};

}  // namespace internal
}  // namespace webrtc
#endif  // VIDEO_RECEIVE_STATISTICS_PROXY_H_
