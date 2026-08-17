/*
 *  Copyright (c) 2016 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_VIDEO_CODING_FEC_CONTROLLER_DEFAULT_H_
#define MODULES_VIDEO_CODING_FEC_CONTROLLER_DEFAULT_H_

#include <stddef.h>
#include <stdint.h>

#include <memory>
#include <vector>

#include "api/environment/environment.h"
#include "api/fec_controller.h"
#include "api/video/video_frame_type.h"
#include "modules/video_coding/media_opt_util.h"
#include "rtc_base/synchronization/mutex.h"
#include "rtc_base/thread_annotations.h"

namespace webrtc {

class FecControllerDefault : public FecController {
 public:
  FecControllerDefault(const Environment& env,
                       VCMProtectionCallback* protection_callback);
  explicit FecControllerDefault(const Environment& env);

  FecControllerDefault(const FecControllerDefault&) = delete;
  FecControllerDefault& operator=(const FecControllerDefault&) = delete;

  ~FecControllerDefault() override;

  void SetProtectionCallback(
      VCMProtectionCallback* protection_callback) override;
  void SetProtectionMethod(bool enable_fec, bool enable_nack) override;
  void SetEncodingData(size_t width,
                       size_t height,
                       size_t num_temporal_layers,
                       size_t max_payload_size) override;
  uint32_t UpdateFecRates(uint32_t estimated_bitrate_bps,
                          int actual_framerate_fps,
                          uint8_t fraction_lost,
                          std::vector<bool> loss_mask_vector,
                          int64_t round_trip_time_ms) override;
  void UpdateWithEncodedData(size_t encoded_image_length,
                             VideoFrameType encoded_image_frametype) override;
  bool UseLossVectorMask() override;
  float GetProtectionOverheadRateThreshold();

 private:
  enum { kBitrateAverageWinMs = 1000 };
  const Environment env_;
  VCMProtectionCallback* protection_callback_;
  Mutex mutex_;
  std::unique_ptr<media_optimization::VCMLossProtectionLogic> loss_prot_logic_
      RTC_GUARDED_BY(mutex_);
  size_t max_payload_size_ RTC_GUARDED_BY(mutex_);

  const float overhead_threshold_;
};

}  // namespace webrtc
#endif  // MODULES_VIDEO_CODING_FEC_CONTROLLER_DEFAULT_H_
