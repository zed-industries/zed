/*
 *  Copyright 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_BASE_EXPERIMENTS_BALANCED_DEGRADATION_SETTINGS_H_
#define RTC_BASE_EXPERIMENTS_BALANCED_DEGRADATION_SETTINGS_H_

#include <optional>
#include <vector>

#include "api/field_trials_view.h"
#include "api/video_codecs/video_encoder.h"

namespace webrtc {

class BalancedDegradationSettings {
 public:
  static constexpr int kNoFpsDiff = -100;

  BalancedDegradationSettings(const FieldTrialsView& field_trials);
  ~BalancedDegradationSettings();

  struct CodecTypeSpecific {
    CodecTypeSpecific() {}
    CodecTypeSpecific(int qp_low, int qp_high, int fps, int kbps, int kbps_res)
        : qp_low(qp_low),
          qp_high(qp_high),
          fps(fps),
          kbps(kbps),
          kbps_res(kbps_res) {}

    bool operator==(const CodecTypeSpecific& o) const {
      return qp_low == o.qp_low && qp_high == o.qp_high && fps == o.fps &&
             kbps == o.kbps && kbps_res == o.kbps_res;
    }

    std::optional<int> GetQpLow() const;
    std::optional<int> GetQpHigh() const;
    std::optional<int> GetFps() const;
    std::optional<int> GetKbps() const;
    std::optional<int> GetKbpsRes() const;

    // Optional settings.
    int qp_low = 0;
    int qp_high = 0;
    int fps = 0;       // If unset, defaults to `fps` in Config.
    int kbps = 0;      // If unset, defaults to `kbps` in Config.
    int kbps_res = 0;  // If unset, defaults to `kbps_res` in Config.
  };

  struct Config {
    Config();
    Config(int pixels,
           int fps,
           int kbps,
           int kbps_res,
           int fps_diff,
           CodecTypeSpecific vp8,
           CodecTypeSpecific vp9,
           CodecTypeSpecific h264,
           CodecTypeSpecific av1,
           CodecTypeSpecific generic);

    bool operator==(const Config& o) const {
      return pixels == o.pixels && fps == o.fps && kbps == o.kbps &&
             kbps_res == o.kbps_res && fps_diff == o.fps_diff && vp8 == o.vp8 &&
             vp9 == o.vp9 && h264 == o.h264 && av1 == o.av1 &&
             generic == o.generic;
    }

    // Example:
    // WebRTC-Video-BalancedDegradationSettings/pixels:100|200|300,fps:5|15|25/
    // pixels <= 100 -> min framerate: 5 fps
    // pixels <= 200 -> min framerate: 15 fps
    // pixels <= 300 -> min framerate: 25 fps
    //
    // WebRTC-Video-BalancedDegradationSettings/pixels:100|200|300,
    // fps:5|15|25,       // Min framerate.
    // kbps:0|60|70,      // Min bitrate needed to adapt up.
    // kbps_res:0|65|75/  // Min bitrate needed to adapt up in resolution.
    //
    // pixels: fps:  kbps:     kbps_res:
    // 300     30    -         -
    // 300     25    70 kbps   75 kbps
    // 200     25    70 kbps   -
    // 200     15    60 kbps   65 kbps
    // 100     15    60 kbps   -
    // 100      5
    //               optional  optional

    int pixels = 0;  // Video frame size.
    // If the frame size is less than or equal to `pixels`:
    int fps = 0;       // Min framerate to be used.
    int kbps = 0;      // Min bitrate needed to adapt up (resolution/fps).
    int kbps_res = 0;  // Min bitrate needed to adapt up in resolution.
    int fps_diff = kNoFpsDiff;  // Min fps reduction needed (input fps - `fps`)
                                // w/o triggering a new subsequent downgrade
                                // check.
    CodecTypeSpecific vp8;
    CodecTypeSpecific vp9;
    CodecTypeSpecific h264;
    CodecTypeSpecific av1;
    CodecTypeSpecific generic;
  };

  // Returns configurations from field trial on success (default on failure).
  std::vector<Config> GetConfigs() const;

  // Gets the min/max framerate from `configs_` based on `pixels`.
  int MinFps(VideoCodecType type, int pixels) const;
  int MaxFps(VideoCodecType type, int pixels) const;

  // Checks if quality can be increased based on `pixels` and `bitrate_bps`.
  bool CanAdaptUp(VideoCodecType type, int pixels, uint32_t bitrate_bps) const;
  bool CanAdaptUpResolution(VideoCodecType type,
                            int pixels,
                            uint32_t bitrate_bps) const;

  // Gets the min framerate diff from `configs_` based on `pixels`.
  std::optional<int> MinFpsDiff(int pixels) const;

  // Gets QpThresholds for the codec `type` based on `pixels`.
  std::optional<VideoEncoder::QpThresholds> GetQpThresholds(VideoCodecType type,
                                                            int pixels) const;

 private:
  std::optional<Config> GetMinFpsConfig(int pixels) const;
  std::optional<Config> GetMaxFpsConfig(int pixels) const;
  Config GetConfig(int pixels) const;

  std::vector<Config> configs_;
};

}  // namespace webrtc

#endif  // RTC_BASE_EXPERIMENTS_BALANCED_DEGRADATION_SETTINGS_H_
