/*
 *  Copyright 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef TEST_SCENARIO_SCENARIO_CONFIG_H_
#define TEST_SCENARIO_SCENARIO_CONFIG_H_

#include <stddef.h>

#include <optional>
#include <string>

#include "api/fec_controller.h"
#include "api/rtp_parameters.h"
#include "api/test/frame_generator_interface.h"
#include "api/transport/network_control.h"
#include "api/units/data_rate.h"
#include "api/units/data_size.h"
#include "api/units/time_delta.h"
#include "api/video/video_codec_type.h"
#include "api/video_codecs/scalability_mode.h"
#include "test/scenario/performance_stats.h"

namespace webrtc {
namespace test {
struct PacketOverhead {
  static constexpr size_t kSrtp = 10;
  static constexpr size_t kStun = 4;
  // TURN messages can be sent either with or without an establieshed channel.
  // In the latter case, a TURN Send/Data Indication is sent which has
  // significantly more overhead.
  static constexpr size_t kTurnChannelMessage = 4;
  static constexpr size_t kTurnIndicationMessage = 36;
  static constexpr size_t kDefault = kSrtp;
};
struct TransportControllerConfig {
  struct Rates {
    Rates();
    Rates(const Rates&);
    ~Rates();
    DataRate min_rate = DataRate::KilobitsPerSec(30);
    DataRate max_rate = DataRate::KilobitsPerSec(3000);
    DataRate start_rate = DataRate::KilobitsPerSec(300);
  } rates;
  NetworkControllerFactoryInterface* cc_factory = nullptr;
  TimeDelta state_log_interval = TimeDelta::Millis(100);
};

struct CallClientConfig {
  TransportControllerConfig transport;
  // Allows the pacer to send out multiple packets in a burst.
  // The number of bites that can be sent in one burst is pacer_burst_interval *
  // current bwe. 40ms is the default Chrome setting.
  TimeDelta pacer_burst_interval = TimeDelta::Millis(40);
};

struct PacketStreamConfig {
  PacketStreamConfig();
  PacketStreamConfig(const PacketStreamConfig&);
  ~PacketStreamConfig();
  int frame_rate = 30;
  DataRate max_data_rate = DataRate::Infinity();
  DataSize max_packet_size = DataSize::Bytes(1400);
  DataSize min_frame_size = DataSize::Bytes(100);
  double keyframe_multiplier = 1;
  DataSize packet_overhead = DataSize::Bytes(PacketOverhead::kDefault);
};

struct VideoStreamConfig {
  bool autostart = true;
  struct Source {
    enum Capture {
      kGenerator,
      kVideoFile,
      kGenerateSlides,
      kImageSlides,
      // Support for explicit frame triggers should be added here if needed.
    } capture = Capture::kGenerator;
    struct Slides {
      TimeDelta change_interval = TimeDelta::Seconds(10);
      struct Generator {
        int width = 1600;
        int height = 1200;
      } generator;
      struct Images {
        struct Crop {
          TimeDelta scroll_duration = TimeDelta::Seconds(0);
          std::optional<int> width;
          std::optional<int> height;
        } crop;
        int width = 1850;
        int height = 1110;
        std::vector<std::string> paths = {
            "web_screenshot_1850_1110",
            "presentation_1850_1110",
            "photo_1850_1110",
            "difficult_photo_1850_1110",
        };
      } images;
    } slides;
    struct Generator {
      using PixelFormat = FrameGeneratorInterface::OutputType;
      PixelFormat pixel_format = PixelFormat::kI420;
      int width = 320;
      int height = 180;
    } generator;
    struct VideoFile {
      std::string name;
      // Must be set to width and height of the source video file.
      int width = 0;
      int height = 0;
    } video_file;
    int framerate = 30;
  } source;
  struct Encoder {
    Encoder();
    Encoder(const Encoder&);
    ~Encoder();
    enum class ContentType {
      kVideo,
      kScreen,
    } content_type = ContentType::kVideo;
    enum Implementation { kFake, kSoftware, kHardware } implementation = kFake;
    struct Fake {
      DataRate max_rate = DataRate::Infinity();
    } fake;

    using Codec = VideoCodecType;
    Codec codec = Codec::kVideoCodecGeneric;
    std::optional<DataRate> max_data_rate;
    std::optional<DataRate> min_data_rate;
    std::optional<int> max_framerate;
    // Counted in frame count.
    std::optional<int> key_frame_interval = 3000;
    bool frame_dropping = true;
    struct SingleLayer {
      bool denoising = true;
      bool automatic_scaling = true;
    } single;
    std::vector<webrtc::ScalabilityMode> simulcast_streams = {
        webrtc::ScalabilityMode::kL1T1};

    DegradationPreference degradation_preference =
        DegradationPreference::MAINTAIN_FRAMERATE;
    bool suspend_below_min_bitrate = false;
  } encoder;
  struct Stream {
    Stream();
    Stream(const Stream&);
    ~Stream();
    bool abs_send_time = false;
    bool packet_feedback = true;
    bool use_rtx = true;
    DataRate pad_to_rate = DataRate::Zero();
    TimeDelta nack_history_time = TimeDelta::Millis(1000);
    bool use_flexfec = false;
    bool use_ulpfec = false;
    FecControllerFactoryInterface* fec_controller_factory = nullptr;
  } stream;
  struct Rendering {
    enum Type { kFake } type = kFake;
    std::string sync_group;
  } render;
  struct Hooks {
    std::vector<std::function<void(const VideoFramePair&)>> frame_pair_handlers;
  } hooks;
};

struct AudioStreamConfig {
  AudioStreamConfig();
  AudioStreamConfig(const AudioStreamConfig&);
  ~AudioStreamConfig();
  bool autostart = true;
  struct Source {
    int channels = 1;
  } source;
  bool network_adaptation = false;
  struct NetworkAdaptation {
    struct FrameLength {
      double min_packet_loss_for_decrease = 0;
      double max_packet_loss_for_increase = 1;
      DataRate min_rate_for_20_ms = DataRate::Zero();
      DataRate max_rate_for_60_ms = DataRate::Infinity();
      DataRate min_rate_for_60_ms = DataRate::Zero();
      DataRate max_rate_for_120_ms = DataRate::Infinity();
    } frame;
    std::string binary_proto;
  } adapt;
  struct Encoder {
    Encoder();
    Encoder(const Encoder&);
    ~Encoder();
    bool allocate_bitrate = false;
    bool enable_dtx = false;
    DataRate fixed_rate = DataRate::KilobitsPerSec(32);
    // Overrides fixed rate.
    std::optional<DataRate> min_rate;
    std::optional<DataRate> max_rate;
    TimeDelta initial_frame_length = TimeDelta::Millis(20);
  } encoder;
  struct Stream {
    Stream();
    Stream(const Stream&);
    ~Stream();
    bool abs_send_time = true;
    bool in_bandwidth_estimation = true;
  } stream;
  struct Rendering {
    std::string sync_group;
  } render;
};

// TODO(srte): Merge this with BuiltInNetworkBehaviorConfig.
struct NetworkSimulationConfig {
  DataRate bandwidth = DataRate::Infinity();
  TimeDelta delay = TimeDelta::Zero();
  TimeDelta delay_std_dev = TimeDelta::Zero();
  double loss_rate = 0;
  std::optional<int> packet_queue_length_limit;
  DataSize packet_overhead = DataSize::Zero();
};
}  // namespace test
}  // namespace webrtc

#endif  // TEST_SCENARIO_SCENARIO_CONFIG_H_
