/*
 *  Copyright (c) 2022 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef TEST_VIDEO_CODEC_TESTER_H_
#define TEST_VIDEO_CODEC_TESTER_H_

#include <limits>
#include <map>
#include <memory>
#include <optional>
#include <string>
#include <vector>

#include "api/environment/environment.h"
#include "api/numerics/samples_stats_counter.h"
#include "api/test/metrics/metric.h"
#include "api/test/metrics/metrics_logger.h"
#include "api/units/data_rate.h"
#include "api/units/data_size.h"
#include "api/units/frequency.h"
#include "api/video/encoded_image.h"
#include "api/video/resolution.h"
#include "api/video_codecs/video_decoder_factory.h"
#include "api/video_codecs/video_encoder_factory.h"

namespace webrtc {
namespace test {

class VideoCodecTester {
 public:
  struct LayerId {
    int spatial_idx = 0;
    int temporal_idx = 0;

    bool operator==(const LayerId& o) const {
      return spatial_idx == o.spatial_idx && temporal_idx == o.temporal_idx;
    }
    bool operator<(const LayerId& o) const {
      return spatial_idx < o.spatial_idx ||
             (spatial_idx == o.spatial_idx && temporal_idx < o.temporal_idx);
    }
  };

  struct EncodingSettings {
    SdpVideoFormat sdp_video_format = SdpVideoFormat::VP8();
    ScalabilityMode scalability_mode = ScalabilityMode::kL1T1;
    VideoCodecMode content_type = VideoCodecMode::kRealtimeVideo;
    bool frame_drop = true;
    bool keyframe = false;

    struct LayerSettings {
      Resolution resolution;
      Frequency framerate;
      DataRate bitrate;
    };
    std::map<LayerId, LayerSettings> layers_settings;
  };

  class VideoCodecStats {
   public:
    struct Filter {
      uint32_t min_timestamp_rtp = std::numeric_limits<uint32_t>::min();
      uint32_t max_timestamp_rtp = std::numeric_limits<uint32_t>::max();
      std::optional<LayerId> layer_id;
    };

    struct Frame {
      uint32_t timestamp_rtp = 0;
      LayerId layer_id;
      bool encoded = false;
      bool decoded = false;
      int width = 0;
      int height = 0;
      DataSize frame_size = DataSize::Zero();
      bool keyframe = false;
      std::optional<int> qp;
      Timestamp encode_start = Timestamp::Zero();
      TimeDelta encode_time = TimeDelta::Zero();
      Timestamp decode_start = Timestamp::Zero();
      TimeDelta decode_time = TimeDelta::Zero();
      std::optional<DataRate> target_bitrate;
      std::optional<Frequency> target_framerate;

      struct Psnr {
        double y = 0.0;
        double u = 0.0;
        double v = 0.0;
      };
      std::optional<Psnr> psnr;
    };

    struct Stream {
      SamplesStatsCounter width;
      SamplesStatsCounter height;
      SamplesStatsCounter frame_size_bytes;
      SamplesStatsCounter keyframe;
      SamplesStatsCounter qp;
      SamplesStatsCounter encode_time_ms;
      SamplesStatsCounter decode_time_ms;
      SamplesStatsCounter target_bitrate_kbps;
      SamplesStatsCounter target_framerate_fps;
      SamplesStatsCounter encoded_bitrate_kbps;
      SamplesStatsCounter encoded_framerate_fps;
      SamplesStatsCounter bitrate_mismatch_pct;
      SamplesStatsCounter framerate_mismatch_pct;
      SamplesStatsCounter transmission_time_ms;

      struct Psnr {
        SamplesStatsCounter y;
        SamplesStatsCounter u;
        SamplesStatsCounter v;
      } psnr;

      // Logs `Stream` metrics to provided `MetricsLogger`.
      void LogMetrics(MetricsLogger* logger,
                      std::string test_case_name,
                      std::string prefix,
                      std::map<std::string, std::string> metadata = {}) const;
    };

    virtual ~VideoCodecStats() = default;

    // Returns frames for the slice specified by `filter`. If `merge` is true,
    // also merges frames belonging to the same temporal unit into one
    // superframe.
    virtual std::vector<Frame> Slice(Filter filter, bool merge) const = 0;

    // Returns video statistics aggregated for the slice specified by `filter`.
    virtual Stream Aggregate(Filter filter) const = 0;

    // Write metrics to a CSV file.
    virtual void LogMetrics(
        absl::string_view csv_path,
        std::vector<Frame> frames,
        std::map<std::string, std::string> metadata) const = 0;
  };

  // Pacing settings for codec input.
  struct PacingSettings {
    enum PacingMode {
      // Pacing is not used. Frames are sent to codec back-to-back.
      kNoPacing,
      // Pace with the rate equal to the target video frame rate. Pacing time is
      // derived from RTP timestamp.
      kRealTime,
      // Pace with the explicitly provided rate.
      kConstantRate,
    };
    PacingMode mode = PacingMode::kNoPacing;
    // Pacing rate for `kConstantRate` mode.
    Frequency constant_rate = Frequency::Zero();
  };

  struct VideoSourceSettings {
    std::string file_path;
    Resolution resolution;
    Frequency framerate;
  };

  struct DecoderSettings {
    PacingSettings pacing_settings;
    std::optional<std::string> decoder_input_base_path;
    std::optional<std::string> decoder_output_base_path;
  };

  struct EncoderSettings {
    PacingSettings pacing_settings;
    std::optional<std::string> encoder_input_base_path;
    std::optional<std::string> encoder_output_base_path;
  };

  virtual ~VideoCodecTester() = default;

  // Interface for a coded video frames source.
  class CodedVideoSource {
   public:
    virtual ~CodedVideoSource() = default;

    // Returns next frame. Returns `std::nullopt` if the end-of-stream is
    // reached. Frames should have RTP timestamps representing desired frame
    // rate.
    virtual std::optional<EncodedImage> PullFrame() = 0;
  };

  // A helper function that creates `EncodingSettings` from the given
  // parameters. `bitrate` is either total, or per-spatial layer or per-spatial
  // and per-temporal layer. If layer bitrates are not explicitly specified,
  // then the codec-specific rate allocators used to distribute the total
  // bitrate across spatial or/and temporal layers.
  static EncodingSettings CreateEncodingSettings(const Environment& env,
                                                 std::string codec_type,
                                                 std::string scalability_name,
                                                 int width,
                                                 int height,
                                                 std::vector<DataRate> bitrate,
                                                 Frequency framerate,
                                                 bool screencast = false,
                                                 bool frame_drop = true);

  // A helper function that creates a map of RTP timestamps to
  // `EncodingSettings` for the given number of frames.
  static std::map<uint32_t, EncodingSettings> CreateFrameSettings(
      const EncodingSettings& encoding_settings,
      int num_frames,
      uint32_t first_timestamp_rtp = 90000);

  // Decodes video, collects and returns decode metrics.
  static std::unique_ptr<VideoCodecStats> RunDecodeTest(
      const Environment& env,
      CodedVideoSource* video_source,
      VideoDecoderFactory* decoder_factory,
      const DecoderSettings& decoder_settings,
      const SdpVideoFormat& sdp_video_format);

  // Encodes video, collects and returns encode metrics.
  static std::unique_ptr<VideoCodecStats> RunEncodeTest(
      const Environment& env,
      const VideoSourceSettings& source_settings,
      VideoEncoderFactory* encoder_factory,
      const EncoderSettings& encoder_settings,
      const std::map<uint32_t, EncodingSettings>& encoding_settings);

  // Encodes and decodes video, collects and returns encode and decode metrics.
  static std::unique_ptr<VideoCodecStats> RunEncodeDecodeTest(
      const Environment& env,
      const VideoSourceSettings& source_settings,
      VideoEncoderFactory* encoder_factory,
      VideoDecoderFactory* decoder_factory,
      const EncoderSettings& encoder_settings,
      const DecoderSettings& decoder_settings,
      const std::map<uint32_t, EncodingSettings>& encoding_settings);
};

}  // namespace test
}  // namespace webrtc

#endif  // TEST_VIDEO_CODEC_TESTER_H_
