/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_TEST_VIDEO_QUALITY_TEST_FIXTURE_H_
#define API_TEST_VIDEO_QUALITY_TEST_FIXTURE_H_

#include <cstddef>
#include <cstdint>
#include <map>
#include <memory>
#include <optional>
#include <string>
#include <vector>

#include "api/fec_controller.h"
#include "api/media_types.h"
#include "api/network_state_predictor.h"
#include "api/rtp_parameters.h"
#include "api/test/simulated_network.h"
#include "api/transport/bitrate_settings.h"
#include "api/transport/network_control.h"
#include "api/video_codecs/spatial_layer.h"
#include "api/video_codecs/video_codec.h"
#include "api/video_codecs/video_decoder_factory.h"
#include "api/video_codecs/video_encoder_factory.h"
#include "video/config/video_encoder_config.h"

namespace webrtc {

class VideoQualityTestFixtureInterface {
 public:
  // Parameters are grouped into smaller structs to make it easier to set
  // the desired elements and skip unused.
  struct Params {
    struct CallConfig {
      bool send_side_bwe = false;
      bool generic_descriptor = false;
      bool dependency_descriptor = false;
      BitrateConstraints call_bitrate_config;
      int num_thumbnails = 0;
      // Indicates if secondary_(video|ss|screenshare) structures are used.
      bool dual_video = false;
    } call;
    struct Video {
      bool enabled = false;
      size_t width = 640;
      size_t height = 480;
      int32_t fps = 30;
      int min_bitrate_bps = 50;
      int target_bitrate_bps = 800;
      int max_bitrate_bps = 800;
      bool suspend_below_min_bitrate = false;
      std::string codec = "VP8";
      int num_temporal_layers = 1;
      int selected_tl = -1;
      int min_transmit_bps = 0;
      bool ulpfec = false;
      bool flexfec = false;
      bool automatic_scaling = false;
      std::string clip_path;  // "Generator" to generate frames instead.
      size_t capture_device_index = 0;
      CodecParameterMap sdp_params;
      double encoder_overshoot_factor = 0.0;
    } video[2];
    struct Audio {
      bool enabled = false;
      bool sync_video = false;
      bool dtx = false;
      bool use_real_adm = false;
      std::optional<std::string> ana_config;
    } audio;
    struct Screenshare {
      bool enabled = false;
      bool generate_slides = false;
      int32_t slide_change_interval = 10;
      int32_t scroll_duration = 0;
      std::vector<std::string> slides;
    } screenshare[2];
    struct Analyzer {
      std::string test_label;
      double avg_psnr_threshold = 0.0;  // (*)
      double avg_ssim_threshold = 0.0;  // (*)
      int test_durations_secs = 0;
      std::string graph_data_output_filename;
      std::string graph_title;
    } analyzer;
    // Config for default simulation implementation. Must be nullopt if
    // `sender_network` and `receiver_network` in InjectionComponents are
    // non-null. May be nullopt even if `sender_network` and `receiver_network`
    // are null; in that case, a default config will be used.
    std::optional<BuiltInNetworkBehaviorConfig> config;
    struct SS {                          // Spatial scalability.
      std::vector<VideoStream> streams;  // If empty, one stream is assumed.
      size_t selected_stream = 0;
      int num_spatial_layers = 0;
      int selected_sl = -1;
      InterLayerPredMode inter_layer_pred = InterLayerPredMode::kOn;
      // If empty, bitrates are generated in VP9Impl automatically.
      std::vector<SpatialLayer> spatial_layers;
      // If set, default parameters will be used instead of `streams`.
      bool infer_streams = false;
    } ss[2];
    struct Logging {
      std::string rtc_event_log_name;
      std::string rtp_dump_name;
      std::string encoded_frame_base_path;
    } logging;
  };

  // Contains objects, that will be injected on different layers of test
  // framework to override the behavior of system parts.
  struct InjectionComponents {
    InjectionComponents();
    ~InjectionComponents();

    // Simulations of sender and receiver networks. They must either both be
    // null (in which case `config` from Params is used), or both be non-null
    // (in which case `config` from Params must be nullopt).
    std::unique_ptr<NetworkBehaviorInterface> sender_network;
    std::unique_ptr<NetworkBehaviorInterface> receiver_network;

    std::unique_ptr<FecControllerFactoryInterface> fec_controller_factory;
    std::unique_ptr<VideoEncoderFactory> video_encoder_factory;
    std::unique_ptr<VideoDecoderFactory> video_decoder_factory;
    std::unique_ptr<NetworkStatePredictorFactoryInterface>
        network_state_predictor_factory;
    std::unique_ptr<NetworkControllerFactoryInterface>
        network_controller_factory;
  };

  virtual ~VideoQualityTestFixtureInterface() = default;

  virtual void RunWithAnalyzer(const Params& params) = 0;
  virtual void RunWithRenderers(const Params& params) = 0;

  virtual const std::map<uint8_t, webrtc::MediaType>& payload_type_map() = 0;
};

}  // namespace webrtc

#endif  // API_TEST_VIDEO_QUALITY_TEST_FIXTURE_H_
