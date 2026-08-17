/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_VIDEO_CODECS_VIDEO_CODEC_H_
#define API_VIDEO_CODECS_VIDEO_CODEC_H_

#include <stddef.h>
#include <stdint.h>

#include <optional>
#include <string>

#include "api/video/video_codec_constants.h"
#include "api/video/video_codec_type.h"
#include "api/video_codecs/scalability_mode.h"
#include "api/video_codecs/simulcast_stream.h"
#include "api/video_codecs/spatial_layer.h"
#include "rtc_base/system/rtc_export.h"

namespace webrtc {

// The VideoCodec class represents an old defacto-apis, which we're migrating
// away from slowly.

// Video codec
enum class VideoCodecComplexity {
  kComplexityLow = -1,
  kComplexityNormal = 0,
  kComplexityHigh = 1,
  kComplexityHigher = 2,
  kComplexityMax = 3
};

// VP8 specific
struct VideoCodecVP8 {
  bool operator==(const VideoCodecVP8& other) const;
  bool operator!=(const VideoCodecVP8& other) const {
    return !(*this == other);
  }
  // Temporary utility method for transition deleting numberOfTemporalLayers
  // setting (replaced by ScalabilityMode).
  void SetNumberOfTemporalLayers(unsigned char n) {
    numberOfTemporalLayers = n;
  }
  unsigned char numberOfTemporalLayers;
  bool denoisingOn;
  bool automaticResizeOn;
  int keyFrameInterval;
};

enum class InterLayerPredMode : int {
  kOff = 0,      // Inter-layer prediction is disabled.
  kOn = 1,       // Inter-layer prediction is enabled.
  kOnKeyPic = 2  // Inter-layer prediction is enabled but limited to key frames.
};

// VP9 specific.
struct VideoCodecVP9 {
  bool operator==(const VideoCodecVP9& other) const;
  bool operator!=(const VideoCodecVP9& other) const {
    return !(*this == other);
  }
  // Temporary utility method for transition deleting numberOfTemporalLayers
  // setting (replaced by ScalabilityMode).
  void SetNumberOfTemporalLayers(unsigned char n) {
    numberOfTemporalLayers = n;
  }
  unsigned char numberOfTemporalLayers;
  bool denoisingOn;
  int keyFrameInterval;
  bool adaptiveQpMode;
  bool automaticResizeOn;
  unsigned char numberOfSpatialLayers;
  bool flexibleMode;
  InterLayerPredMode interLayerPred;
};

// H264 specific.
struct VideoCodecH264 {
  bool operator==(const VideoCodecH264& other) const;
  bool operator!=(const VideoCodecH264& other) const {
    return !(*this == other);
  }
  // Temporary utility method for transition deleting numberOfTemporalLayers
  // setting (replaced by ScalabilityMode).
  void SetNumberOfTemporalLayers(unsigned char n) {
    numberOfTemporalLayers = n;
  }
  int keyFrameInterval;
  uint8_t numberOfTemporalLayers;
};

struct VideoCodecAV1 {
  bool operator==(const VideoCodecAV1& other) const {
    return automatic_resize_on == other.automatic_resize_on;
  }
  bool operator!=(const VideoCodecAV1& other) const {
    return !(*this == other);
  }
  bool automatic_resize_on;
};

// Translates from name of codec to codec type and vice versa.
RTC_EXPORT const char* CodecTypeToPayloadString(VideoCodecType type);
RTC_EXPORT VideoCodecType PayloadStringToCodecType(const std::string& name);

union VideoCodecUnion {
  VideoCodecVP8 VP8;
  VideoCodecVP9 VP9;
  VideoCodecH264 H264;
  VideoCodecAV1 AV1;
};

enum class VideoCodecMode { kRealtimeVideo, kScreensharing };

// Common video codec properties
class RTC_EXPORT VideoCodec {
 public:
  VideoCodec();

  // Scalability mode as described in
  // https://www.w3.org/TR/webrtc-svc/#scalabilitymodes*
  std::optional<ScalabilityMode> GetScalabilityMode() const {
    return scalability_mode_;
  }
  void SetScalabilityMode(ScalabilityMode scalability_mode) {
    scalability_mode_ = scalability_mode;
  }
  void UnsetScalabilityMode() { scalability_mode_ = std::nullopt; }

  VideoCodecComplexity GetVideoEncoderComplexity() const;
  void SetVideoEncoderComplexity(VideoCodecComplexity complexity_setting);

  bool GetFrameDropEnabled() const;
  void SetFrameDropEnabled(bool enabled);

  bool IsSinglecast() const { return numberOfSimulcastStreams <= 1; }
  bool IsSimulcast() const { return !IsSinglecast(); }

  // Public variables. TODO(hta): Make them private with accessors.
  VideoCodecType codecType;

  // TODO(nisse): Change to int, for consistency.
  uint16_t width;
  uint16_t height;

  unsigned int startBitrate;  // kilobits/sec.
  unsigned int maxBitrate;    // kilobits/sec.
  unsigned int minBitrate;    // kilobits/sec.

  uint32_t maxFramerate;

  // This enables/disables encoding and sending when there aren't multiple
  // simulcast streams,by allocating 0 bitrate if inactive.
  bool active;

  unsigned int qpMax;
  // The actual number of simulcast streams. This is <= 1 in singlecast (it can
  // be 0 in old code paths), but it is also 1 in the {active,inactive,inactive}
  // "single RTP simulcast" use case and the legacy kSVC use case. In all other
  // cases this is the same as the number of encodings (which may include
  // inactive encodings). In other words:
  // - `numberOfSimulcastStreams <= 1` in singlecast and singlecast-like setups
  //   including legacy kSVC (encodings interpreted as spatial layers) or
  //   standard kSVC (1 active encoding).
  // - `numberOfSimulcastStreams > 1` in simulcast of 2+ active encodings.
  unsigned char numberOfSimulcastStreams;
  SimulcastStream simulcastStream[kMaxSimulcastStreams];
  SpatialLayer spatialLayers[kMaxSpatialLayers];

  VideoCodecMode mode;
  bool expect_encode_from_texture;

  // Timing frames configuration. There is delay of delay_ms between two
  // consequent timing frames, excluding outliers. Frame is always made a
  // timing frame if it's at least outlier_ratio in percent of "ideal" average
  // frame given bitrate and framerate, i.e. if it's bigger than
  // |outlier_ratio / 100.0 * bitrate_bps / fps| in bits. This way, timing
  // frames will not be sent too often usually. Yet large frames will always
  // have timing information for debug purposes because they are more likely to
  // cause extra delays.
  struct TimingFrameTriggerThresholds {
    int64_t delay_ms;
    uint16_t outlier_ratio_percent;
  } timing_frame_thresholds;

  // Legacy Google conference mode flag for simulcast screenshare
  bool legacy_conference_mode;

  bool operator==(const VideoCodec& other) const = delete;
  bool operator!=(const VideoCodec& other) const = delete;
  std::string ToString() const;

  // Accessors for codec specific information.
  // There is a const version of each that returns a reference,
  // and a non-const version that returns a pointer, in order
  // to allow modification of the parameters.
  VideoCodecVP8* VP8();
  const VideoCodecVP8& VP8() const;
  VideoCodecVP9* VP9();
  const VideoCodecVP9& VP9() const;
  VideoCodecH264* H264();
  const VideoCodecH264& H264() const;
  VideoCodecAV1* AV1();
  const VideoCodecAV1& AV1() const;

 private:
  // TODO(hta): Consider replacing the union with a pointer type.
  // This will allow removing the VideoCodec* types from this file.
  VideoCodecUnion codec_specific_;
  std::optional<ScalabilityMode> scalability_mode_;
  // 'complexity_' indicates the CPU capability of the client. It's used to
  // determine encoder CPU complexity (e.g., cpu_used for VP8, VP9. and AV1).
  VideoCodecComplexity complexity_;
  bool frame_drop_enabled_ = false;
};

}  // namespace webrtc
#endif  // API_VIDEO_CODECS_VIDEO_CODEC_H_
