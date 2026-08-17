/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef TEST_PC_E2E_ANALYZER_VIDEO_DEFAULT_VIDEO_QUALITY_ANALYZER_FRAMES_COMPARATOR_H_
#define TEST_PC_E2E_ANALYZER_VIDEO_DEFAULT_VIDEO_QUALITY_ANALYZER_FRAMES_COMPARATOR_H_

#include <deque>
#include <map>
#include <utility>
#include <vector>

#include "api/array_view.h"
#include "rtc_base/event.h"
#include "rtc_base/platform_thread.h"
#include "rtc_base/synchronization/mutex.h"
#include "system_wrappers/include/clock.h"
#include "test/pc/e2e/analyzer/video/default_video_quality_analyzer_cpu_measurer.h"
#include "test/pc/e2e/analyzer/video/default_video_quality_analyzer_internal_shared_objects.h"
#include "test/pc/e2e/analyzer/video/default_video_quality_analyzer_shared_objects.h"

namespace webrtc {

struct FramesComparatorStats {
  // Size of analyzer internal comparisons queue, measured when new element
  // id added to the queue.
  SamplesStatsCounter comparisons_queue_size;
  // Number of performed comparisons of 2 video frames from captured and
  // rendered streams.
  int64_t comparisons_done = 0;
  // Number of cpu overloaded comparisons. Comparison is cpu overloaded if it is
  // queued when there are too many not processed comparisons in the queue.
  // Overloaded comparison doesn't include metrics like SSIM and PSNR that
  // require heavy computations.
  int64_t cpu_overloaded_comparisons_done = 0;
  // Number of memory overloaded comparisons. Comparison is memory overloaded if
  // it is queued when its captured frame was already removed due to high memory
  // usage for that video stream.
  int64_t memory_overloaded_comparisons_done = 0;
};

// Performs comparisons of added frames and tracks frames related statistics.
// This class is thread safe.
class DefaultVideoQualityAnalyzerFramesComparator {
 public:
  // Creates frames comparator.
  // Frames comparator doesn't use `options.enable_receive_own_stream` for any
  // purposes, because it's unrelated to its functionality.
  DefaultVideoQualityAnalyzerFramesComparator(
      webrtc::Clock* clock,
      DefaultVideoQualityAnalyzerCpuMeasurer& cpu_measurer,
      DefaultVideoQualityAnalyzerOptions options = {})
      : options_(options), clock_(clock), cpu_measurer_(cpu_measurer) {}
  ~DefaultVideoQualityAnalyzerFramesComparator() { Stop({}); }

  // Starts frames comparator. This method must be invoked before calling
  // any other method on this object.
  void Start(int max_threads_count);
  // Stops frames comparator. This method will block until all added frame
  // comparisons will be processed. After `Stop()` is invoked no more new
  // comparisons can be added to this frames comparator.
  //
  // `last_rendered_frame_time` contains timestamps of last rendered frame for
  //     each (stream, sender, receiver) tuple to properly update time between
  //     freezes: it has include time from the last freeze until and of call.
  void Stop(
      const std::map<InternalStatsKey, Timestamp>& last_rendered_frame_times);

  // Ensures that stream `stream_index` has stats objects created for all
  // potential receivers. This method must be called before adding any
  // frames comparison for that stream.
  void EnsureStatsForStream(size_t stream_index,
                            size_t sender_peer_index,
                            size_t peers_count,
                            Timestamp captured_time,
                            Timestamp start_time);
  // Ensures that newly added participant will have stream stats objects created
  // for all streams which they can receive. This method must be called before
  // any frames comparison will be added for the newly added participant.
  //
  // `stream_started_time` - start time of each stream for which stats object
  //     has to be created.
  // `start_time` - call start time.
  void RegisterParticipantInCall(
      ArrayView<std::pair<InternalStatsKey, Timestamp>> stream_started_time,
      Timestamp start_time);

  // `captured` - video frame captured by sender to use for PSNR/SSIM
  //     computation. If `type` is `FrameComparisonType::kRegular` and
  //     `captured` is `std::nullopt` comparison is assumed to be overloaded
  //     due to memory constraints.
  // `rendered` - video frame rendered by receiver to use for PSNR/SSIM
  //     computation. Required only if `type` is
  //     `FrameComparisonType::kRegular`, but can still be omitted if
  //     `captured` is `std::nullopt`.
  void AddComparison(InternalStatsKey stats_key,
                     std::optional<VideoFrame> captured,
                     std::optional<VideoFrame> rendered,
                     FrameComparisonType type,
                     FrameStats frame_stats);
  // `skipped_between_rendered` - amount of frames dropped on this stream before
  //     last received frame and current frame.
  void AddComparison(InternalStatsKey stats_key,
                     int skipped_between_rendered,
                     std::optional<VideoFrame> captured,
                     std::optional<VideoFrame> rendered,
                     FrameComparisonType type,
                     FrameStats frame_stats);

  std::map<InternalStatsKey, StreamStats> stream_stats() const {
    MutexLock lock(&mutex_);
    return stream_stats_;
  }
  FramesComparatorStats frames_comparator_stats() const {
    MutexLock lock(&mutex_);
    return frames_comparator_stats_;
  }

 private:
  enum State { kNew, kActive, kStopped };

  void AddComparisonInternal(InternalStatsKey stats_key,
                             std::optional<VideoFrame> captured,
                             std::optional<VideoFrame> rendered,
                             FrameComparisonType type,
                             FrameStats frame_stats)
      RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_);
  void ProcessComparisons();
  void ProcessComparison(const FrameComparison& comparison);
  Timestamp Now();

  const DefaultVideoQualityAnalyzerOptions options_;
  webrtc::Clock* const clock_;
  DefaultVideoQualityAnalyzerCpuMeasurer& cpu_measurer_;

  mutable Mutex mutex_;
  State state_ RTC_GUARDED_BY(mutex_) = State::kNew;
  std::map<InternalStatsKey, StreamStats> stream_stats_ RTC_GUARDED_BY(mutex_);
  std::map<InternalStatsKey, Timestamp> stream_last_freeze_end_time_
      RTC_GUARDED_BY(mutex_);
  std::deque<FrameComparison> comparisons_ RTC_GUARDED_BY(mutex_);
  FramesComparatorStats frames_comparator_stats_ RTC_GUARDED_BY(mutex_);

  std::vector<PlatformThread> thread_pool_;
  Event comparison_available_event_;
};

}  // namespace webrtc

#endif  // TEST_PC_E2E_ANALYZER_VIDEO_DEFAULT_VIDEO_QUALITY_ANALYZER_FRAMES_COMPARATOR_H_
