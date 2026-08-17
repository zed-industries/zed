/*
 *  Copyright 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_BASE_EXPERIMENTS_ENCODER_INFO_SETTINGS_H_
#define RTC_BASE_EXPERIMENTS_ENCODER_INFO_SETTINGS_H_

#include <optional>
#include <string>
#include <vector>

#include "absl/strings/string_view.h"
#include "api/field_trials_view.h"
#include "api/video_codecs/video_encoder.h"
#include "rtc_base/experiments/field_trial_parser.h"

namespace webrtc {

class EncoderInfoSettings {
 public:
  virtual ~EncoderInfoSettings();

  // Bitrate limits per resolution.
  struct BitrateLimit {
    int frame_size_pixels = 0;      // The video frame size.
    int min_start_bitrate_bps = 0;  // The minimum bitrate to start encoding.
    int min_bitrate_bps = 0;        // The minimum bitrate.
    int max_bitrate_bps = 0;        // The maximum bitrate.
  };

  std::optional<uint32_t> requested_resolution_alignment() const;
  bool apply_alignment_to_all_simulcast_layers() const {
    return apply_alignment_to_all_simulcast_layers_.Get();
  }
  std::vector<VideoEncoder::ResolutionBitrateLimits> resolution_bitrate_limits()
      const {
    return resolution_bitrate_limits_;
  }

  static std::vector<VideoEncoder::ResolutionBitrateLimits>
  GetDefaultSinglecastBitrateLimits(VideoCodecType codec_type);

  static std::optional<VideoEncoder::ResolutionBitrateLimits>
  GetDefaultSinglecastBitrateLimitsForResolution(VideoCodecType codec_type,
                                                 int frame_size_pixels);

  static std::vector<VideoEncoder::ResolutionBitrateLimits>
  GetDefaultSinglecastBitrateLimitsWhenQpIsUntrusted(VideoCodecType codec_type);

  static std::optional<VideoEncoder::ResolutionBitrateLimits>
  GetSinglecastBitrateLimitForResolutionWhenQpIsUntrusted(
      std::optional<int> frame_size_pixels,
      const std::vector<VideoEncoder::ResolutionBitrateLimits>&
          resolution_bitrate_limits);

 protected:
  EncoderInfoSettings(const FieldTrialsView& field_trials,
                      absl::string_view name);

 private:
  FieldTrialOptional<uint32_t> requested_resolution_alignment_;
  FieldTrialFlag apply_alignment_to_all_simulcast_layers_;
  std::vector<VideoEncoder::ResolutionBitrateLimits> resolution_bitrate_limits_;
};

// EncoderInfo settings for SimulcastEncoderAdapter.
class SimulcastEncoderAdapterEncoderInfoSettings : public EncoderInfoSettings {
 public:
  explicit SimulcastEncoderAdapterEncoderInfoSettings(
      const FieldTrialsView& field_trials);
  ~SimulcastEncoderAdapterEncoderInfoSettings() override {}
};

// EncoderInfo settings for LibvpxVp8Encoder.
class LibvpxVp8EncoderInfoSettings : public EncoderInfoSettings {
 public:
  explicit LibvpxVp8EncoderInfoSettings(const FieldTrialsView& field_trials);
  ~LibvpxVp8EncoderInfoSettings() override {}
};

// EncoderInfo settings for LibvpxVp9Encoder.
class LibvpxVp9EncoderInfoSettings : public EncoderInfoSettings {
 public:
  explicit LibvpxVp9EncoderInfoSettings(const FieldTrialsView& field_trials);
  ~LibvpxVp9EncoderInfoSettings() override {}
};

// EncoderInfo settings for LibaomAv1Encoder.
class LibaomAv1EncoderInfoSettings : public EncoderInfoSettings {
 public:
  explicit LibaomAv1EncoderInfoSettings(const FieldTrialsView& field_trials);
  ~LibaomAv1EncoderInfoSettings() override {}
};

}  // namespace webrtc

#endif  // RTC_BASE_EXPERIMENTS_ENCODER_INFO_SETTINGS_H_
