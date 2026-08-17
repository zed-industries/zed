/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef VIDEO_ADAPTATION_BANDWIDTH_QUALITY_SCALER_RESOURCE_H_
#define VIDEO_ADAPTATION_BANDWIDTH_QUALITY_SCALER_RESOURCE_H_

#include <memory>
#include <optional>
#include <queue>
#include <string>
#include <vector>

#include "api/scoped_refptr.h"
#include "api/video/video_adaptation_reason.h"
#include "api/video_codecs/video_encoder.h"
#include "call/adaptation/degradation_preference_provider.h"
#include "call/adaptation/resource_adaptation_processor_interface.h"
#include "modules/video_coding/utility/bandwidth_quality_scaler.h"
#include "video/adaptation/video_stream_encoder_resource.h"

namespace webrtc {

// Handles interaction with the BandwidthQualityScaler.
class BandwidthQualityScalerResource
    : public VideoStreamEncoderResource,
      public BandwidthQualityScalerUsageHandlerInterface {
 public:
  static scoped_refptr<BandwidthQualityScalerResource> Create();

  BandwidthQualityScalerResource();
  ~BandwidthQualityScalerResource() override;

  bool is_started() const;

  void OnEncodeCompleted(const EncodedImage& encoded_image,
                         int64_t time_sent_in_us,
                         int64_t encoded_image_size_bytes);

  void StartCheckForOveruse(
      const std::vector<VideoEncoder::ResolutionBitrateLimits>&
          resolution_bitrate_limits,
      VideoCodecType codec_type);
  void StopCheckForOveruse();

  // BandwidthScalerQpUsageHandlerInterface implementation.
  void OnReportUsageBandwidthHigh() override;
  void OnReportUsageBandwidthLow() override;

 private:
  std::unique_ptr<BandwidthQualityScaler> bandwidth_quality_scaler_
      RTC_GUARDED_BY(encoder_queue());
};

}  // namespace webrtc

#endif  // VIDEO_ADAPTATION_BANDWIDTH_QUALITY_SCALER_RESOURCE_H_
