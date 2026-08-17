/*
 *  Copyright (c) 2014 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef TEST_CALL_TEST_H_
#define TEST_CALL_TEST_H_

#include <map>
#include <memory>
#include <optional>
#include <string>
#include <vector>

#include "api/array_view.h"
#include "api/audio/audio_device.h"
#include "api/environment/environment.h"
#include "api/rtc_event_log/rtc_event_log.h"
#include "api/task_queue/task_queue_base.h"
#include "api/task_queue/task_queue_factory.h"
#include "api/test/simulated_network.h"
#include "api/test/video/function_video_decoder_factory.h"
#include "api/test/video/function_video_encoder_factory.h"
#include "api/units/time_delta.h"
#include "api/video/video_bitrate_allocator_factory.h"
#include "call/call.h"
#include "modules/audio_device/include/test_audio_device.h"
#include "test/encoder_settings.h"
#include "test/fake_decoder.h"
#include "test/fake_videorenderer.h"
#include "test/fake_vp8_encoder.h"
#include "test/frame_generator_capturer.h"
#include "test/rtp_rtcp_observer.h"
#include "test/run_loop.h"
#include "test/scoped_key_value_config.h"
#include "test/test_video_capturer.h"
#include "test/video_test_constants.h"

namespace webrtc {
namespace test {

class BaseTest;

class CallTest : public ::testing::Test, public RtpPacketSinkInterface {
 public:
  CallTest();
  virtual ~CallTest();

  static const std::map<uint8_t, MediaType> payload_type_map_;

 protected:
  const Environment& env() const { return env_; }

  void SetSendEventLog(std::unique_ptr<RtcEventLog> event_log);
  void SetRecvEventLog(std::unique_ptr<RtcEventLog> event_log);

  void RegisterRtpExtension(const RtpExtension& extension);
  // Returns header extensions that can be parsed by the transport.
  ArrayView<const RtpExtension> GetRegisteredExtensions() {
    return rtp_extensions_;
  }

  // RunBaseTest overwrites the audio_state of the send and receive Call configs
  // to simplify test code.
  void RunBaseTest(BaseTest* test);

  CallConfig SendCallConfig() const;
  CallConfig RecvCallConfig() const;

  void CreateCalls();
  void CreateCalls(CallConfig sender_config, CallConfig receiver_config);
  void CreateSenderCall();
  void CreateSenderCall(CallConfig config);
  void CreateReceiverCall(CallConfig config);
  void DestroyCalls();

  void CreateVideoSendConfig(VideoSendStream::Config* video_config,
                             size_t num_video_streams,
                             size_t num_used_ssrcs,
                             Transport* send_transport);
  void CreateAudioAndFecSendConfigs(size_t num_audio_streams,
                                    size_t num_flexfec_streams,
                                    Transport* send_transport);
  void SetAudioConfig(const AudioSendStream::Config& config);

  void SetSendFecConfig(std::vector<uint32_t> video_send_ssrcs);
  void SetSendUlpFecConfig(VideoSendStream::Config* send_config);
  void SetReceiveUlpFecConfig(
      VideoReceiveStreamInterface::Config* receive_config);

  void CreateSendConfig(size_t num_video_streams,
                        size_t num_audio_streams,
                        size_t num_flexfec_streams) {
    CreateSendConfig(num_video_streams, num_audio_streams, num_flexfec_streams,
                     send_transport_.get());
  }
  void CreateSendConfig(size_t num_video_streams,
                        size_t num_audio_streams,
                        size_t num_flexfec_streams,
                        Transport* send_transport);

  void CreateMatchingVideoReceiveConfigs(
      const VideoSendStream::Config& video_send_config) {
    CreateMatchingVideoReceiveConfigs(video_send_config,
                                      receive_transport_.get());
  }
  void CreateMatchingVideoReceiveConfigs(
      const VideoSendStream::Config& video_send_config,
      Transport* rtcp_send_transport);
  void CreateMatchingVideoReceiveConfigs(
      const VideoSendStream::Config& video_send_config,
      Transport* rtcp_send_transport,
      VideoDecoderFactory* decoder_factory,
      std::optional<size_t> decode_sub_stream,
      bool receiver_reference_time_report,
      int rtp_history_ms);
  void AddMatchingVideoReceiveConfigs(
      std::vector<VideoReceiveStreamInterface::Config>* receive_configs,
      const VideoSendStream::Config& video_send_config,
      Transport* rtcp_send_transport,
      VideoDecoderFactory* decoder_factory,
      std::optional<size_t> decode_sub_stream,
      bool receiver_reference_time_report,
      int rtp_history_ms);

  void CreateMatchingAudioAndFecConfigs(Transport* rtcp_send_transport);
  void CreateMatchingAudioConfigs(Transport* transport, std::string sync_group);
  static AudioReceiveStreamInterface::Config CreateMatchingAudioConfig(
      const AudioSendStream::Config& send_config,
      scoped_refptr<AudioDecoderFactory> audio_decoder_factory,
      Transport* transport,
      std::string sync_group);
  void CreateMatchingFecConfig(
      Transport* transport,
      const VideoSendStream::Config& video_send_config);
  void CreateMatchingReceiveConfigs() {
    CreateMatchingReceiveConfigs(receive_transport_.get());
  }
  void CreateMatchingReceiveConfigs(Transport* rtcp_send_transport);

  void CreateFrameGeneratorCapturerWithDrift(Clock* drift_clock,
                                             float speed,
                                             int framerate,
                                             int width,
                                             int height);
  void CreateFrameGeneratorCapturer(int framerate, int width, int height);
  void CreateFakeAudioDevices(
      std::unique_ptr<TestAudioDeviceModule::Capturer> capturer,
      std::unique_ptr<TestAudioDeviceModule::Renderer> renderer);

  void CreateVideoStreams();
  void CreateVideoSendStreams();
  void CreateVideoSendStream(const VideoEncoderConfig& encoder_config);
  void CreateAudioStreams();
  void CreateFlexfecStreams();

  // Receiver call must be created before calling CreateSendTransport in order
  // to set a receiver.
  // Rtp header extensions must be registered (RegisterRtpExtension(..)) before
  // the transport is created in order for the receiving call object receive RTP
  // packets with extensions.
  void CreateSendTransport(const BuiltInNetworkBehaviorConfig& config,
                           RtpRtcpObserver* observer);
  void CreateReceiveTransport(const BuiltInNetworkBehaviorConfig& config,
                              RtpRtcpObserver* observer);

  void ConnectVideoSourcesToStreams();

  void Start();
  void StartVideoSources();
  void StartVideoStreams();
  void Stop();
  void StopVideoStreams();
  void DestroyStreams();
  void DestroyVideoSendStreams();
  void SetFakeVideoCaptureRotation(VideoRotation rotation);

  void SetVideoDegradation(DegradationPreference preference);

  VideoSendStream::Config* GetVideoSendConfig();
  void SetVideoSendConfig(const VideoSendStream::Config& config);
  VideoEncoderConfig* GetVideoEncoderConfig();
  void SetVideoEncoderConfig(const VideoEncoderConfig& config);
  VideoSendStream* GetVideoSendStream();
  FlexfecReceiveStream::Config* GetFlexFecConfig();
  TaskQueueBase* task_queue() { return task_queue_.get(); }

  // RtpPacketSinkInterface implementation.
  void OnRtpPacket(const RtpPacketReceived& packet) override;

  test::RunLoop loop_;
  test::ScopedKeyValueConfig field_trials_;
  Environment env_;
  Environment send_env_;
  Environment recv_env_;

  std::unique_ptr<Call> sender_call_;
  std::unique_ptr<PacketTransport> send_transport_;
  SimulatedNetworkInterface* send_simulated_network_ = nullptr;
  std::vector<VideoSendStream::Config> video_send_configs_;
  std::vector<VideoEncoderConfig> video_encoder_configs_;
  std::vector<VideoSendStream*> video_send_streams_;
  AudioSendStream::Config audio_send_config_;
  AudioSendStream* audio_send_stream_;

  std::unique_ptr<Call> receiver_call_;
  std::unique_ptr<PacketTransport> receive_transport_;
  SimulatedNetworkInterface* receive_simulated_network_ = nullptr;
  std::vector<VideoReceiveStreamInterface::Config> video_receive_configs_;
  std::vector<VideoReceiveStreamInterface*> video_receive_streams_;
  std::vector<AudioReceiveStreamInterface::Config> audio_receive_configs_;
  std::vector<AudioReceiveStreamInterface*> audio_receive_streams_;
  std::vector<FlexfecReceiveStream::Config> flexfec_receive_configs_;
  std::vector<FlexfecReceiveStream*> flexfec_receive_streams_;

  test::FrameGeneratorCapturer* frame_generator_capturer_;
  std::vector<std::unique_ptr<TestVideoCapturer>> video_sources_;
  DegradationPreference degradation_preference_ =
      DegradationPreference::MAINTAIN_FRAMERATE;

  std::unique_ptr<FecControllerFactoryInterface> fec_controller_factory_;
  std::unique_ptr<NetworkStatePredictorFactoryInterface>
      network_state_predictor_factory_;
  std::unique_ptr<NetworkControllerFactoryInterface>
      network_controller_factory_;

  test::FunctionVideoEncoderFactory fake_encoder_factory_;
  int fake_encoder_max_bitrate_ = -1;
  test::FunctionVideoDecoderFactory fake_decoder_factory_;
  std::unique_ptr<VideoBitrateAllocatorFactory> bitrate_allocator_factory_;
  // Number of simulcast substreams.
  size_t num_video_streams_;
  size_t num_audio_streams_;
  size_t num_flexfec_streams_;
  scoped_refptr<AudioDecoderFactory> audio_decoder_factory_;
  scoped_refptr<AudioEncoderFactory> audio_encoder_factory_;
  test::FakeVideoRenderer fake_renderer_;

 private:
  std::optional<RtpExtension> GetRtpExtensionByUri(
      const std::string& uri) const;

  void AddRtpExtensionByUri(const std::string& uri,
                            std::vector<RtpExtension>* extensions) const;

  std::unique_ptr<TaskQueueBase, TaskQueueDeleter> task_queue_;
  std::vector<RtpExtension> rtp_extensions_;
  scoped_refptr<AudioProcessing> apm_send_;
  scoped_refptr<AudioProcessing> apm_recv_;
  scoped_refptr<AudioDeviceModule> fake_send_audio_device_;
  scoped_refptr<AudioDeviceModule> fake_recv_audio_device_;
};

class BaseTest : public RtpRtcpObserver {
 public:
  BaseTest();
  explicit BaseTest(TimeDelta timeout);
  virtual ~BaseTest();

  virtual void PerformTest() = 0;
  virtual bool ShouldCreateReceivers() const = 0;

  virtual size_t GetNumVideoStreams() const;
  virtual size_t GetNumAudioStreams() const;
  virtual size_t GetNumFlexfecStreams() const;

  virtual std::unique_ptr<TestAudioDeviceModule::Capturer> CreateCapturer();
  virtual std::unique_ptr<TestAudioDeviceModule::Renderer> CreateRenderer();
  virtual void OnFakeAudioDevicesCreated(AudioDeviceModule* send_audio_device,
                                         AudioDeviceModule* recv_audio_device);

  virtual void ModifySenderBitrateConfig(BitrateConstraints* bitrate_config);
  virtual void ModifyReceiverBitrateConfig(BitrateConstraints* bitrate_config);

  virtual void OnCallsCreated(Call* sender_call, Call* receiver_call);
  virtual void OnTransportCreated(PacketTransport* to_receiver,
                                  SimulatedNetworkInterface* sender_network,
                                  PacketTransport* to_sender,
                                  SimulatedNetworkInterface* receiver_network);

  virtual BuiltInNetworkBehaviorConfig GetSendTransportConfig() const;
  virtual BuiltInNetworkBehaviorConfig GetReceiveTransportConfig() const;

  virtual void ModifyVideoConfigs(
      VideoSendStream::Config* send_config,
      std::vector<VideoReceiveStreamInterface::Config>* receive_configs,
      VideoEncoderConfig* encoder_config);
  virtual void ModifyVideoCaptureStartResolution(int* width,
                                                 int* heigt,
                                                 int* frame_rate);
  virtual void ModifyVideoDegradationPreference(
      DegradationPreference* degradation_preference);

  virtual void OnVideoStreamsCreated(
      VideoSendStream* send_stream,
      const std::vector<VideoReceiveStreamInterface*>& receive_streams);

  virtual void ModifyAudioConfigs(
      AudioSendStream::Config* send_config,
      std::vector<AudioReceiveStreamInterface::Config>* receive_configs);
  virtual void OnAudioStreamsCreated(
      AudioSendStream* send_stream,
      const std::vector<AudioReceiveStreamInterface*>& receive_streams);

  virtual void ModifyFlexfecConfigs(
      std::vector<FlexfecReceiveStream::Config>* receive_configs);
  virtual void OnFlexfecStreamsCreated(
      const std::vector<FlexfecReceiveStream*>& receive_streams);

  virtual void OnFrameGeneratorCapturerCreated(
      FrameGeneratorCapturer* frame_generator_capturer);

  virtual void OnStreamsStopped();
};

class SendTest : public BaseTest {
 public:
  explicit SendTest(TimeDelta timeout);

  bool ShouldCreateReceivers() const override;
};

class EndToEndTest : public BaseTest {
 public:
  EndToEndTest();
  explicit EndToEndTest(TimeDelta timeout);

  bool ShouldCreateReceivers() const override;
};

}  // namespace test
}  // namespace webrtc

#endif  // TEST_CALL_TEST_H_
