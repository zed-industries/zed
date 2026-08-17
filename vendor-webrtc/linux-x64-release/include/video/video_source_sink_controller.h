/*
 *  Copyright 2020 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef VIDEO_VIDEO_SOURCE_SINK_CONTROLLER_H_
#define VIDEO_VIDEO_SOURCE_SINK_CONTROLLER_H_

#include <optional>
#include <string>
#include <vector>

#include "api/sequence_checker.h"
#include "api/video/video_frame.h"
#include "api/video/video_sink_interface.h"
#include "api/video/video_source_interface.h"
#include "call/adaptation/video_source_restrictions.h"
#include "rtc_base/system/no_unique_address.h"

namespace webrtc {

// Responsible for configuring source/sink settings, i.e. performing
// webrtc::VideoSourceInterface<VideoFrame>::AddOrUpdateSink(). It does this by
// storing settings internally which are converted to webrtc::VideoSinkWants
// when PushSourceSinkSettings() is performed.
class VideoSourceSinkController {
 public:
  VideoSourceSinkController(VideoSinkInterface<VideoFrame>* sink,
                            VideoSourceInterface<VideoFrame>* source);

  ~VideoSourceSinkController();

  void SetSource(VideoSourceInterface<VideoFrame>* source);
  bool HasSource() const;

  // Requests a refresh frame from the current source, if set.
  void RequestRefreshFrame();

  // Must be called in order for changes to settings to have an effect. This
  // allows you to modify multiple properties in a single push to the sink.
  void PushSourceSinkSettings();

  VideoSourceRestrictions restrictions() const;
  std::optional<size_t> pixels_per_frame_upper_limit() const;
  std::optional<double> frame_rate_upper_limit() const;
  bool rotation_applied() const;
  int resolution_alignment() const;
  const std::vector<VideoSinkWants::FrameSize>& resolutions() const;
  bool active() const;
  std::optional<VideoSinkWants::FrameSize> scale_resolution_down_to() const;

  // Updates the settings stored internally. In order for these settings to be
  // applied to the sink, PushSourceSinkSettings() must subsequently be called.
  void SetRestrictions(VideoSourceRestrictions restrictions);
  void SetPixelsPerFrameUpperLimit(
      std::optional<size_t> pixels_per_frame_upper_limit);
  void SetFrameRateUpperLimit(std::optional<double> frame_rate_upper_limit);
  void SetRotationApplied(bool rotation_applied);
  void SetResolutionAlignment(int resolution_alignment);
  void SetResolutions(std::vector<VideoSinkWants::FrameSize> resolutions);
  void SetActive(bool active);
  void SetScaleResolutionDownTo(
      std::optional<VideoSinkWants::FrameSize> scale_resolution_down_to);

 private:
  VideoSinkWants CurrentSettingsToSinkWants() const
      RTC_EXCLUSIVE_LOCKS_REQUIRED(sequence_checker_);

  // Used to ensure that this class is called on threads/sequences that it and
  // downstream implementations were designed for.
  // In practice, this represent's libjingle's worker thread.
  RTC_NO_UNIQUE_ADDRESS SequenceChecker sequence_checker_;

  VideoSinkInterface<VideoFrame>* const sink_;
  VideoSourceInterface<VideoFrame>* source_ RTC_GUARDED_BY(&sequence_checker_);
  // Pixel and frame rate restrictions.
  VideoSourceRestrictions restrictions_ RTC_GUARDED_BY(&sequence_checker_);
  // Ensures that even if we are not restricted, the sink is never configured
  // above this limit. Example: We are not CPU limited (no `restrictions_`) but
  // our encoder is capped at 30 fps (= `frame_rate_upper_limit_`).
  std::optional<size_t> pixels_per_frame_upper_limit_
      RTC_GUARDED_BY(&sequence_checker_);
  std::optional<double> frame_rate_upper_limit_
      RTC_GUARDED_BY(&sequence_checker_);
  bool rotation_applied_ RTC_GUARDED_BY(&sequence_checker_) = false;
  int resolution_alignment_ RTC_GUARDED_BY(&sequence_checker_) = 1;
  std::vector<VideoSinkWants::FrameSize> resolutions_
      RTC_GUARDED_BY(&sequence_checker_);
  bool active_ RTC_GUARDED_BY(&sequence_checker_) = true;
  std::optional<VideoSinkWants::FrameSize> scale_resolution_down_to_
      RTC_GUARDED_BY(&sequence_checker_);
};

}  // namespace webrtc

#endif  // VIDEO_VIDEO_SOURCE_SINK_CONTROLLER_H_
