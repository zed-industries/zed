/*
 *  Copyright 2020 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef VIDEO_ADAPTATION_ENCODE_USAGE_RESOURCE_H_
#define VIDEO_ADAPTATION_ENCODE_USAGE_RESOURCE_H_

#include <memory>
#include <optional>
#include <string>

#include "api/scoped_refptr.h"
#include "api/video/video_adaptation_reason.h"
#include "video/adaptation/overuse_frame_detector.h"
#include "video/adaptation/video_stream_encoder_resource.h"

namespace webrtc {

// Handles interaction with the OveruseDetector.
// TODO(hbos): Add unittests specific to this class, it is currently only tested
// indirectly by usage in the ResourceAdaptationProcessor (which is only tested
// because of its usage in VideoStreamEncoder); all tests are currently in
// video_stream_encoder_unittest.cc.
class EncodeUsageResource : public VideoStreamEncoderResource,
                            public OveruseFrameDetectorObserverInterface {
 public:
  static scoped_refptr<EncodeUsageResource> Create(
      std::unique_ptr<OveruseFrameDetector> overuse_detector);

  explicit EncodeUsageResource(
      std::unique_ptr<OveruseFrameDetector> overuse_detector);
  ~EncodeUsageResource() override;

  bool is_started() const;

  void StartCheckForOveruse(CpuOveruseOptions options);
  void StopCheckForOveruse();

  void SetTargetFrameRate(std::optional<double> target_frame_rate);
  void OnEncodeStarted(const VideoFrame& cropped_frame,
                       int64_t time_when_first_seen_us);
  void OnEncodeCompleted(uint32_t timestamp,
                         int64_t time_sent_in_us,
                         int64_t capture_time_us,
                         std::optional<int> encode_duration_us);

  // OveruseFrameDetectorObserverInterface implementation.
  void AdaptUp() override;
  void AdaptDown() override;

 private:
  int TargetFrameRateAsInt();

  const std::unique_ptr<OveruseFrameDetector> overuse_detector_
      RTC_GUARDED_BY(encoder_queue());
  bool is_started_ RTC_GUARDED_BY(encoder_queue());
  std::optional<double> target_frame_rate_ RTC_GUARDED_BY(encoder_queue());
};

}  // namespace webrtc

#endif  // VIDEO_ADAPTATION_ENCODE_USAGE_RESOURCE_H_
