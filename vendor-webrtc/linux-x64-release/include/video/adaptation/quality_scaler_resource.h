/*
 *  Copyright 2020 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef VIDEO_ADAPTATION_QUALITY_SCALER_RESOURCE_H_
#define VIDEO_ADAPTATION_QUALITY_SCALER_RESOURCE_H_

#include <memory>
#include <optional>
#include <queue>
#include <string>

#include "api/field_trials_view.h"
#include "api/scoped_refptr.h"
#include "api/video/video_adaptation_reason.h"
#include "api/video_codecs/video_encoder.h"
#include "call/adaptation/degradation_preference_provider.h"
#include "call/adaptation/resource_adaptation_processor_interface.h"
#include "modules/video_coding/utility/quality_scaler.h"
#include "video/adaptation/video_stream_encoder_resource.h"

namespace webrtc {

// Handles interaction with the QualityScaler.
class QualityScalerResource : public VideoStreamEncoderResource,
                              public QualityScalerQpUsageHandlerInterface {
 public:
  static scoped_refptr<QualityScalerResource> Create();

  QualityScalerResource();
  ~QualityScalerResource() override;

  bool is_started() const;

  void StartCheckForOveruse(VideoEncoder::QpThresholds qp_thresholds,
                            const FieldTrialsView& field_trials);
  void StopCheckForOveruse();
  void SetQpThresholds(VideoEncoder::QpThresholds qp_thresholds);
  void OnEncodeCompleted(const EncodedImage& encoded_image,
                         int64_t time_sent_in_us);
  void OnFrameDropped(EncodedImageCallback::DropReason reason);

  // QualityScalerQpUsageHandlerInterface implementation.
  void OnReportQpUsageHigh() override;
  void OnReportQpUsageLow() override;

 private:
  std::unique_ptr<QualityScaler> quality_scaler_
      RTC_GUARDED_BY(encoder_queue());
};

}  // namespace webrtc

#endif  // VIDEO_ADAPTATION_QUALITY_SCALER_RESOURCE_H_
