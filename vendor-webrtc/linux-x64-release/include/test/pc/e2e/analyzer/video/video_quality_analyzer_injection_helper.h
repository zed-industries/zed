/*
 *  Copyright (c) 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef TEST_PC_E2E_ANALYZER_VIDEO_VIDEO_QUALITY_ANALYZER_INJECTION_HELPER_H_
#define TEST_PC_E2E_ANALYZER_VIDEO_VIDEO_QUALITY_ANALYZER_INJECTION_HELPER_H_

#include <stdio.h>

#include <map>
#include <memory>
#include <string>
#include <vector>

#include "absl/strings/string_view.h"
#include "api/array_view.h"
#include "api/test/pclf/media_configuration.h"
#include "api/test/stats_observer_interface.h"
#include "api/test/video_quality_analyzer_interface.h"
#include "api/video/video_frame.h"
#include "api/video/video_sink_interface.h"
#include "api/video_codecs/video_decoder_factory.h"
#include "api/video_codecs/video_encoder_factory.h"
#include "rtc_base/synchronization/mutex.h"
#include "system_wrappers/include/clock.h"
#include "test/pc/e2e/analyzer/video/analyzing_video_sink.h"
#include "test/pc/e2e/analyzer/video/analyzing_video_sinks_helper.h"
#include "test/pc/e2e/analyzer/video/encoded_image_data_injector.h"
#include "test/pc/e2e/analyzer/video/quality_analyzing_video_encoder.h"
#include "test/test_video_capturer.h"
#include "test/testsupport/video_frame_writer.h"

namespace webrtc {
namespace webrtc_pc_e2e {

// Provides factory methods for components, that will be used to inject
// VideoQualityAnalyzerInterface into PeerConnection pipeline.
class VideoQualityAnalyzerInjectionHelper : public StatsObserverInterface {
 public:
  VideoQualityAnalyzerInjectionHelper(
      Clock* clock,
      std::unique_ptr<VideoQualityAnalyzerInterface> analyzer,
      EncodedImageDataInjector* injector,
      EncodedImageDataExtractor* extractor);
  ~VideoQualityAnalyzerInjectionHelper() override;

  // Wraps video encoder factory to give video quality analyzer access to frames
  // before encoding and encoded images after.
  std::unique_ptr<VideoEncoderFactory> WrapVideoEncoderFactory(
      absl::string_view peer_name,
      std::unique_ptr<VideoEncoderFactory> delegate,
      double bitrate_multiplier,
      QualityAnalyzingVideoEncoder::EmulatedSFUConfigMap stream_to_sfu_config)
      const;
  // Wraps video decoder factory to give video quality analyzer access to
  // received encoded images and frames, that were decoded from them.
  std::unique_ptr<VideoDecoderFactory> WrapVideoDecoderFactory(
      absl::string_view peer_name,
      std::unique_ptr<VideoDecoderFactory> delegate) const;

  // Creates VideoFrame preprocessor, that will allow video quality analyzer to
  // get access to the captured frames. If provided config also specifies
  // `input_dump_file_name`, video will be written into that file.
  std::unique_ptr<test::TestVideoCapturer::FramePreprocessor>
  CreateFramePreprocessor(absl::string_view peer_name,
                          const webrtc::webrtc_pc_e2e::VideoConfig& config);
  // Creates sink, that will allow video quality analyzer to get access to
  // the rendered frames. If corresponding video track has
  // `output_dump_file_name` in its VideoConfig, which was used for
  // CreateFramePreprocessor(...), then video also will be written
  // into that file.
  std::unique_ptr<AnalyzingVideoSink> CreateVideoSink(
      absl::string_view peer_name,
      const VideoSubscription& subscription,
      bool report_infra_metrics);

  void Start(std::string test_case_name,
             ArrayView<const std::string> peer_names,
             int max_threads_count = 1);

  // Registers new call participant to the underlying video quality analyzer.
  // The method should be called before the participant is actually added.
  void RegisterParticipantInCall(absl::string_view peer_name);

  // Will be called after test removed existing participant in the middle of the
  // call.
  void UnregisterParticipantInCall(absl::string_view peer_name);

  // Forwards `stats_reports` for Peer Connection `pc_label` to
  // `analyzer_`.
  void OnStatsReports(
      absl::string_view pc_label,
      const scoped_refptr<const RTCStatsReport>& report) override;

  // Stops VideoQualityAnalyzerInterface to populate final data and metrics.
  // Should be invoked after analyzed video tracks are disposed.
  void Stop();

 private:
  Clock* const clock_;
  std::unique_ptr<VideoQualityAnalyzerInterface> analyzer_;
  EncodedImageDataInjector* injector_;
  EncodedImageDataExtractor* extractor_;

  std::vector<std::unique_ptr<test::VideoFrameWriter>> video_writers_;

  AnalyzingVideoSinksHelper sinks_helper_;
};

}  // namespace webrtc_pc_e2e
}  // namespace webrtc

#endif  // TEST_PC_E2E_ANALYZER_VIDEO_VIDEO_QUALITY_ANALYZER_INJECTION_HELPER_H_
