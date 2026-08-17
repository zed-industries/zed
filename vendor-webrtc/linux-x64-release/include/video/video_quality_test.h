/*
 *  Copyright (c) 2015 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef VIDEO_VIDEO_QUALITY_TEST_H_
#define VIDEO_VIDEO_QUALITY_TEST_H_

#include <map>
#include <memory>
#include <string>
#include <vector>

#include "api/environment/environment.h"
#include "api/fec_controller.h"
#include "api/rtc_event_log/rtc_event_log_factory.h"
#include "api/task_queue/task_queue_base.h"
#include "api/task_queue/task_queue_factory.h"
#include "api/test/frame_generator_interface.h"
#include "api/test/video_quality_test_fixture.h"
#include "api/video/video_bitrate_allocator_factory.h"
#include "call/fake_network_pipe.h"
#include "media/engine/internal_decoder_factory.h"
#include "media/engine/internal_encoder_factory.h"
#include "test/call_test.h"
#include "test/layer_filtering_transport.h"
#include "video/video_analyzer.h"
#ifdef WEBRTC_WIN
#include "modules/audio_device/win/core_audio_utility_win.h"
#include "rtc_base/win/scoped_com_initializer.h"
#endif

namespace webrtc {

class VideoQualityTest : public test::CallTest,
                         public VideoQualityTestFixtureInterface {
 public:
  explicit VideoQualityTest(
      std::unique_ptr<InjectionComponents> injection_components);

  void RunWithAnalyzer(const Params& params) override;
  void RunWithRenderers(const Params& params) override;

  const std::map<uint8_t, webrtc::MediaType>& payload_type_map() override {
    return payload_type_map_;
  }

  void FillScalabilitySettings(
      Params* params,
      size_t video_idx,
      const std::vector<std::string>& stream_descriptors,
      int num_streams,
      size_t selected_stream,
      int num_spatial_layers,
      int selected_sl,
      InterLayerPredMode inter_layer_pred,
      const std::vector<std::string>& sl_descriptors);

  // Helper static methods.
  static VideoStream DefaultVideoStream(const Params& params, size_t video_idx);
  static VideoStream DefaultThumbnailStream();
  static std::vector<int> ParseCSV(const std::string& str);

 protected:
  // No-op implementation to be able to instantiate this class from non-TEST_F
  // locations.
  void TestBody() override;

  // Helper methods accessing only params_.
  std::string GenerateGraphTitle() const;
  void CheckParamsAndInjectionComponents();

  // Helper methods for setting up the call.
  void CreateCapturers();
  std::unique_ptr<test::FrameGeneratorInterface> CreateFrameGenerator(
      size_t video_idx);
  void SetupThumbnailCapturers(size_t num_thumbnail_streams);
  std::unique_ptr<VideoDecoder> CreateVideoDecoder(
      const Environment& env,
      const SdpVideoFormat& format);
  std::unique_ptr<VideoEncoder> CreateVideoEncoder(const Environment& env,
                                                   const SdpVideoFormat& format,
                                                   VideoAnalyzer* analyzer);
  void SetupVideo(Transport* send_transport, Transport* recv_transport);
  void SetupThumbnails(Transport* send_transport, Transport* recv_transport);
  void StartAudioStreams();
  void StartThumbnails();
  void StopThumbnails();
  void DestroyThumbnailStreams();
  // Helper method for creating a real ADM (using hardware) for all platforms.
  scoped_refptr<AudioDeviceModule> CreateAudioDevice();
  void InitializeAudioDevice(CallConfig* send_call_config,
                             CallConfig* recv_call_config,
                             bool use_real_adm);
  void SetupAudio(Transport* transport);

  void StartEncodedFrameLogs(VideoReceiveStreamInterface* stream);

  virtual std::unique_ptr<test::LayerFilteringTransport> CreateSendTransport();
  virtual std::unique_ptr<test::DirectTransport> CreateReceiveTransport();

  std::vector<std::unique_ptr<VideoSourceInterface<VideoFrame>>>
      thumbnail_capturers_;
  Clock* const clock_;
  const std::unique_ptr<TaskQueueFactory> task_queue_factory_;
  RtcEventLogFactory rtc_event_log_factory_;

  test::FunctionVideoDecoderFactory video_decoder_factory_;
  std::unique_ptr<VideoDecoderFactory> decoder_factory_;
  test::FunctionVideoEncoderFactory video_encoder_factory_;
  test::FunctionVideoEncoderFactory video_encoder_factory_with_analyzer_;
  std::unique_ptr<VideoBitrateAllocatorFactory>
      video_bitrate_allocator_factory_;
  std::unique_ptr<VideoEncoderFactory> encoder_factory_;
  std::vector<VideoSendStream::Config> thumbnail_send_configs_;
  std::vector<VideoEncoderConfig> thumbnail_encoder_configs_;
  std::vector<VideoSendStream*> thumbnail_send_streams_;
  std::vector<VideoReceiveStreamInterface::Config> thumbnail_receive_configs_;
  std::vector<VideoReceiveStreamInterface*> thumbnail_receive_streams_;

  int receive_logs_;
  int send_logs_;

  Params params_;
  std::unique_ptr<InjectionComponents> injection_components_;

  // Set non-null when running with analyzer.
  std::unique_ptr<VideoAnalyzer> analyzer_;

  // Note: not same as similarly named member in CallTest. This is the number of
  // separate send streams, the one in CallTest is the number of substreams for
  // a single send stream.
  size_t num_video_streams_;

#ifdef WEBRTC_WIN
  // Windows Core Audio based ADM needs to run on a COM initialized thread.
  // Only referenced in combination with --audio --use_real_adm flags.
  std::unique_ptr<ScopedCOMInitializer> com_initializer_;
#endif
};

}  // namespace webrtc

#endif  // VIDEO_VIDEO_QUALITY_TEST_H_
