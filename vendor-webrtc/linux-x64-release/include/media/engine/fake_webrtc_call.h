/*
 *  Copyright (c) 2015 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

// This file contains fake implementations, for use in unit tests, of the
// following classes:
//
//   webrtc::Call
//   webrtc::AudioSendStream
//   webrtc::AudioReceiveStreamInterface
//   webrtc::VideoSendStream
//   webrtc::VideoReceiveStreamInterface

#ifndef MEDIA_ENGINE_FAKE_WEBRTC_CALL_H_
#define MEDIA_ENGINE_FAKE_WEBRTC_CALL_H_

#include <cstddef>
#include <cstdint>
#include <map>
#include <memory>
#include <optional>
#include <string>
#include <utility>
#include <vector>

#include "absl/strings/string_view.h"
#include "api/adaptation/resource.h"
#include "api/audio/audio_frame.h"
#include "api/audio/audio_mixer.h"
#include "api/audio_codecs/audio_format.h"
#include "api/crypto/frame_decryptor_interface.h"
#include "api/environment/environment.h"
#include "api/frame_transformer_interface.h"
#include "api/media_types.h"
#include "api/rtp_headers.h"
#include "api/rtp_parameters.h"
#include "api/rtp_sender_interface.h"
#include "api/scoped_refptr.h"
#include "api/task_queue/task_queue_base.h"
#include "api/transport/bitrate_settings.h"
#include "api/transport/rtp/rtp_source.h"
#include "api/units/time_delta.h"
#include "api/units/timestamp.h"
#include "api/video/video_frame.h"
#include "api/video/video_sink_interface.h"
#include "api/video/video_source_interface.h"
#include "api/video_codecs/video_codec.h"
#include "call/audio_receive_stream.h"
#include "call/audio_send_stream.h"
#include "call/call.h"
#include "call/fake_payload_type_suggester.h"
#include "call/flexfec_receive_stream.h"
#include "call/packet_receiver.h"
#include "call/payload_type.h"
#include "call/rtp_transport_controller_send_interface.h"
#include "call/test/mock_rtp_transport_controller_send.h"
#include "call/video_receive_stream.h"
#include "call/video_send_stream.h"
#include "modules/rtp_rtcp/include/receive_statistics.h"
#include "modules/rtp_rtcp/source/rtp_packet_received.h"
#include "rtc_base/buffer.h"
#include "rtc_base/copy_on_write_buffer.h"
#include "rtc_base/network/sent_packet.h"
#include "test/gmock.h"
#include "video/config/video_encoder_config.h"

namespace webrtc {
class FakeAudioSendStream final : public AudioSendStream {
 public:
  struct TelephoneEvent {
    int payload_type = -1;
    int payload_frequency = -1;
    int event_code = 0;
    int duration_ms = 0;
  };

  explicit FakeAudioSendStream(int id, const AudioSendStream::Config& config);

  int id() const { return id_; }
  const AudioSendStream::Config& GetConfig() const override;
  void SetStats(const AudioSendStream::Stats& stats);
  TelephoneEvent GetLatestTelephoneEvent() const;
  bool IsSending() const { return sending_; }
  bool muted() const { return muted_; }

 private:
  // webrtc::AudioSendStream implementation.
  void Reconfigure(const AudioSendStream::Config& config,
                   SetParametersCallback callback) override;
  void Start() override { sending_ = true; }
  void Stop() override { sending_ = false; }
  void SendAudioData(std::unique_ptr<AudioFrame> /* audio_frame */) override {}
  bool SendTelephoneEvent(int payload_type,
                          int payload_frequency,
                          int event,
                          int duration_ms) override;
  void SetMuted(bool muted) override;
  AudioSendStream::Stats GetStats() const override;
  AudioSendStream::Stats GetStats(bool has_remote_tracks) const override;

  int id_ = -1;
  TelephoneEvent latest_telephone_event_;
  AudioSendStream::Config config_;
  AudioSendStream::Stats stats_;
  bool sending_ = false;
  bool muted_ = false;
};

class FakeAudioReceiveStream final : public AudioReceiveStreamInterface {
 public:
  explicit FakeAudioReceiveStream(
      int id,
      const AudioReceiveStreamInterface::Config& config);

  int id() const { return id_; }
  const AudioReceiveStreamInterface::Config& GetConfig() const;
  void SetStats(const AudioReceiveStreamInterface::Stats& stats);
  int received_packets() const { return received_packets_; }
  bool VerifyLastPacket(const uint8_t* data, size_t length) const;
  const AudioSinkInterface* sink() const { return sink_; }
  float gain() const { return gain_; }
  bool DeliverRtp(const uint8_t* packet, size_t length, int64_t packet_time_us);
  bool started() const { return started_; }
  int base_mininum_playout_delay_ms() const {
    return base_mininum_playout_delay_ms_;
  }

  void SetLocalSsrc(uint32_t local_ssrc) {
    config_.rtp.local_ssrc = local_ssrc;
  }

  void SetSyncGroup(absl::string_view sync_group) {
    config_.sync_group = std::string(sync_group);
  }

  uint32_t remote_ssrc() const override { return config_.rtp.remote_ssrc; }
  void Start() override { started_ = true; }
  void Stop() override { started_ = false; }
  bool IsRunning() const override { return started_; }
  void SetDepacketizerToDecoderFrameTransformer(
      scoped_refptr<FrameTransformerInterface> frame_transformer) override;
  void SetDecoderMap(std::map<int, SdpAudioFormat> decoder_map) override;
  void SetNackHistory(int history_ms) override;
  void SetRtcpMode(RtcpMode mode) override;
  void SetNonSenderRttMeasurement(bool enabled) override;
  void SetFrameDecryptor(
      scoped_refptr<FrameDecryptorInterface> frame_decryptor) override;

  AudioReceiveStreamInterface::Stats GetStats(
      bool get_and_clear_legacy_stats) const override;
  void SetSink(AudioSinkInterface* sink) override;
  void SetGain(float gain) override;
  bool SetBaseMinimumPlayoutDelayMs(int delay_ms) override {
    base_mininum_playout_delay_ms_ = delay_ms;
    return true;
  }
  int GetBaseMinimumPlayoutDelayMs() const override {
    return base_mininum_playout_delay_ms_;
  }
  std::vector<RtpSource> GetSources() const override {
    return std::vector<RtpSource>();
  }
  AudioMixer::Source* source() override {
    // TODO(b/397376626): Add a Fake AudioMixer::Source
    return nullptr;
  }

 private:
  int id_ = -1;
  AudioReceiveStreamInterface::Config config_;
  AudioReceiveStreamInterface::Stats stats_;
  int received_packets_ = 0;
  AudioSinkInterface* sink_ = nullptr;
  float gain_ = 1.0f;
  Buffer last_packet_;
  bool started_ = false;
  int base_mininum_playout_delay_ms_ = 0;
};

class FakeVideoSendStream final : public VideoSendStream,
                                  public VideoSinkInterface<VideoFrame> {
 public:
  FakeVideoSendStream(const Environment& env,
                      VideoSendStream::Config config,
                      VideoEncoderConfig encoder_config);
  ~FakeVideoSendStream() override;
  const VideoSendStream::Config& GetConfig() const;
  const VideoEncoderConfig& GetEncoderConfig() const;
  const std::vector<VideoStream>& GetVideoStreams() const;

  bool IsSending() const;
  bool GetVp8Settings(VideoCodecVP8* settings) const;
  bool GetVp9Settings(VideoCodecVP9* settings) const;
  bool GetH264Settings(VideoCodecH264* settings) const;
  bool GetAv1Settings(VideoCodecAV1* settings) const;

  int GetNumberOfSwappedFrames() const;
  int GetLastWidth() const;
  int GetLastHeight() const;
  int64_t GetLastTimestamp() const;
  void SetStats(const VideoSendStream::Stats& stats);
  int num_encoder_reconfigurations() const {
    return num_encoder_reconfigurations_;
  }

  bool resolution_scaling_enabled() const {
    return resolution_scaling_enabled_;
  }
  bool framerate_scaling_enabled() const { return framerate_scaling_enabled_; }
  void InjectVideoSinkWants(const VideoSinkWants& wants);

  VideoSourceInterface<VideoFrame>* source() const { return source_; }
  void GenerateKeyFrame(const std::vector<std::string>& rids);
  const std::vector<std::string>& GetKeyFramesRequested() const {
    return keyframes_requested_by_rid_;
  }

 private:
  // webrtc::VideoSinkInterface<VideoFrame> implementation.
  void OnFrame(const VideoFrame& frame) override;

  // webrtc::VideoSendStream implementation.
  void Start() override;
  void Stop() override;
  bool started() override { return IsSending(); }
  void AddAdaptationResource(scoped_refptr<Resource> resource) override;
  std::vector<scoped_refptr<Resource>> GetAdaptationResources() override;
  void SetSource(VideoSourceInterface<VideoFrame>* source,
                 const DegradationPreference& degradation_preference) override;
  VideoSendStream::Stats GetStats() override;

  void ReconfigureVideoEncoder(VideoEncoderConfig config) override;
  void ReconfigureVideoEncoder(VideoEncoderConfig config,
                               SetParametersCallback callback) override;

  const Environment env_;
  bool sending_;
  VideoSendStream::Config config_;
  VideoEncoderConfig encoder_config_;
  std::vector<VideoStream> video_streams_;
  VideoSinkWants sink_wants_;

  bool codec_settings_set_;
  union CodecSpecificSettings {
    VideoCodecVP8 vp8;
    VideoCodecVP9 vp9;
    VideoCodecH264 h264;
    VideoCodecAV1 av1;
  } codec_specific_settings_;
  bool resolution_scaling_enabled_;
  bool framerate_scaling_enabled_;
  VideoSourceInterface<VideoFrame>* source_;
  int num_swapped_frames_;
  std::optional<VideoFrame> last_frame_;
  VideoSendStream::Stats stats_;
  int num_encoder_reconfigurations_ = 0;
  std::vector<std::string> keyframes_requested_by_rid_;
};

class FakeVideoReceiveStream final : public VideoReceiveStreamInterface {
 public:
  explicit FakeVideoReceiveStream(VideoReceiveStreamInterface::Config config);

  const VideoReceiveStreamInterface::Config& GetConfig() const;

  bool IsReceiving() const;

  void InjectFrame(const VideoFrame& frame);

  void SetStats(const VideoReceiveStreamInterface::Stats& stats);

  std::vector<RtpSource> GetSources() const override {
    return std::vector<RtpSource>();
  }

  int base_mininum_playout_delay_ms() const {
    return base_mininum_playout_delay_ms_;
  }

  void SetLocalSsrc(uint32_t local_ssrc) {
    config_.rtp.local_ssrc = local_ssrc;
  }

  void UpdateRtxSsrc(uint32_t ssrc) { config_.rtp.rtx_ssrc = ssrc; }

  void SetFrameDecryptor(scoped_refptr<FrameDecryptorInterface>
                         /* frame_decryptor */) override {}

  void SetDepacketizerToDecoderFrameTransformer(
      scoped_refptr<FrameTransformerInterface> /* frame_transformer */)
      override {}

  RecordingState SetAndGetRecordingState(
      RecordingState /* state */,
      bool /* generate_key_frame */) override {
    return RecordingState();
  }
  void GenerateKeyFrame() override {}

  void SetRtcpMode(RtcpMode mode) override { config_.rtp.rtcp_mode = mode; }

  void SetFlexFecProtection(RtpPacketSinkInterface* sink) override {
    config_.rtp.packet_sink_ = sink;
    config_.rtp.protected_by_flexfec = (sink != nullptr);
  }

  void SetLossNotificationEnabled(bool enabled) override {
    config_.rtp.lntf.enabled = enabled;
  }

  void SetNackHistory(TimeDelta history) override {
    config_.rtp.nack.rtp_history_ms = history.ms();
  }

  void SetProtectionPayloadTypes(int red_payload_type,
                                 int ulpfec_payload_type) override {
    config_.rtp.red_payload_type = red_payload_type;
    config_.rtp.ulpfec_payload_type = ulpfec_payload_type;
  }

  void SetRtcpXr(Config::Rtp::RtcpXr rtcp_xr) override {
    config_.rtp.rtcp_xr = rtcp_xr;
  }

  void SetAssociatedPayloadTypes(std::map<int, int> associated_payload_types) {
    config_.rtp.rtx_associated_payload_types =
        std::move(associated_payload_types);
  }

  void Start() override;
  void Stop() override;

  VideoReceiveStreamInterface::Stats GetStats() const override;

  bool SetBaseMinimumPlayoutDelayMs(int delay_ms) override {
    base_mininum_playout_delay_ms_ = delay_ms;
    return true;
  }

  int GetBaseMinimumPlayoutDelayMs() const override {
    return base_mininum_playout_delay_ms_;
  }

 private:
  VideoReceiveStreamInterface::Config config_;
  bool receiving_;
  VideoReceiveStreamInterface::Stats stats_;

  int base_mininum_playout_delay_ms_ = 0;
};

class FakeFlexfecReceiveStream final : public FlexfecReceiveStream {
 public:
  explicit FakeFlexfecReceiveStream(const FlexfecReceiveStream::Config config);

  void SetLocalSsrc(uint32_t local_ssrc) {
    config_.rtp.local_ssrc = local_ssrc;
  }

  void SetRtcpMode(RtcpMode mode) override { config_.rtcp_mode = mode; }

  int payload_type() const override { return config_.payload_type; }
  void SetPayloadType(int payload_type) override {
    config_.payload_type = payload_type;
  }

  const FlexfecReceiveStream::Config& GetConfig() const;

  uint32_t remote_ssrc() const { return config_.rtp.remote_ssrc; }

  const ReceiveStatistics* GetStats() const override { return nullptr; }

 private:
  void OnRtpPacket(const RtpPacketReceived& packet) override;

  FlexfecReceiveStream::Config config_;
};

class FakeCall final : public Call, public PacketReceiver {
 public:
  explicit FakeCall(const Environment& env);
  FakeCall(const Environment& env,
           TaskQueueBase* worker_thread,
           TaskQueueBase* network_thread);
  ~FakeCall() override;

  PayloadTypeSuggester* GetPayloadTypeSuggester() { return &pt_suggester_; }

  MockRtpTransportControllerSend* GetMockTransportControllerSend() {
    return &transport_controller_send_;
  }

  const std::vector<FakeVideoSendStream*>& GetVideoSendStreams();
  const std::vector<FakeVideoReceiveStream*>& GetVideoReceiveStreams();

  const std::vector<FakeAudioSendStream*>& GetAudioSendStreams();
  const FakeAudioSendStream* GetAudioSendStream(uint32_t ssrc);
  const std::vector<FakeAudioReceiveStream*>& GetAudioReceiveStreams();
  const FakeAudioReceiveStream* GetAudioReceiveStream(uint32_t ssrc);
  const FakeVideoReceiveStream* GetVideoReceiveStream(uint32_t ssrc);

  const std::vector<FakeFlexfecReceiveStream*>& GetFlexfecReceiveStreams();

  SentPacketInfo last_sent_packet() const { return last_sent_packet_; }
  const RtpPacketReceived& last_received_rtp_packet() const {
    return last_received_rtp_packet_;
  }
  size_t GetDeliveredPacketsForSsrc(uint32_t ssrc) const {
    auto it = delivered_packets_by_ssrc_.find(ssrc);
    return it != delivered_packets_by_ssrc_.end() ? it->second : 0u;
  }

  // This is useful if we care about the last media packet (with id populated)
  // but not the last ICE packet (with -1 ID).
  int last_sent_nonnegative_packet_id() const {
    return last_sent_nonnegative_packet_id_;
  }

  NetworkState GetNetworkState(MediaType media) const;
  int GetNumCreatedSendStreams() const;
  int GetNumCreatedReceiveStreams() const;
  void SetStats(const Call::Stats& stats);

  void SetClientBitratePreferences(
      const BitrateSettings& /* preferences */) override {}
  const FieldTrialsView& trials() const override { return env_.field_trials(); }
  void EnableSendCongestionControlFeedbackAccordingToRfc8888() override {}
  int FeedbackAccordingToRfc8888Count() { return 0; }
  int FeedbackAccordingToTransportCcCount() { return 0; }

 private:
  AudioSendStream* CreateAudioSendStream(
      const AudioSendStream::Config& config) override;
  void DestroyAudioSendStream(AudioSendStream* send_stream) override;

  AudioReceiveStreamInterface* CreateAudioReceiveStream(
      const AudioReceiveStreamInterface::Config& config) override;
  void DestroyAudioReceiveStream(
      AudioReceiveStreamInterface* receive_stream) override;

  VideoSendStream* CreateVideoSendStream(
      VideoSendStream::Config config,
      VideoEncoderConfig encoder_config) override;
  void DestroyVideoSendStream(VideoSendStream* send_stream) override;

  VideoReceiveStreamInterface* CreateVideoReceiveStream(
      VideoReceiveStreamInterface::Config config) override;
  void DestroyVideoReceiveStream(
      VideoReceiveStreamInterface* receive_stream) override;

  FlexfecReceiveStream* CreateFlexfecReceiveStream(
      const FlexfecReceiveStream::Config config) override;
  void DestroyFlexfecReceiveStream(
      FlexfecReceiveStream* receive_stream) override;

  void AddAdaptationResource(scoped_refptr<Resource> resource) override;

  PacketReceiver* Receiver() override;

  void DeliverRtcpPacket(CopyOnWriteBuffer /* packet */) override {}

  void DeliverRtpPacket(
      MediaType media_type,
      RtpPacketReceived packet,
      OnUndemuxablePacketHandler un_demuxable_packet_handler) override;

  bool DeliverPacketInternal(MediaType media_type,
                             uint32_t ssrc,
                             const CopyOnWriteBuffer& packet,
                             Timestamp arrival_time);

  RtpTransportControllerSendInterface* GetTransportControllerSend() override {
    return &transport_controller_send_;
  }

  Call::Stats GetStats() const override;

  TaskQueueBase* network_thread() const override;
  TaskQueueBase* worker_thread() const override;

  void SignalChannelNetworkState(MediaType media, NetworkState state) override;
  void OnAudioTransportOverheadChanged(
      int transport_overhead_per_packet) override;
  void OnLocalSsrcUpdated(AudioReceiveStreamInterface& stream,
                          uint32_t local_ssrc) override;
  void OnLocalSsrcUpdated(VideoReceiveStreamInterface& stream,
                          uint32_t local_ssrc) override;
  void OnLocalSsrcUpdated(FlexfecReceiveStream& stream,
                          uint32_t local_ssrc) override;
  void OnUpdateSyncGroup(AudioReceiveStreamInterface& stream,
                         absl::string_view sync_group) override;
  void OnSentPacket(const SentPacketInfo& sent_packet) override;

  const Environment env_;
  TaskQueueBase* const network_thread_;
  TaskQueueBase* const worker_thread_;

  ::testing::NiceMock<MockRtpTransportControllerSend>
      transport_controller_send_;

  NetworkState audio_network_state_;
  NetworkState video_network_state_;
  SentPacketInfo last_sent_packet_;
  RtpPacketReceived last_received_rtp_packet_;
  int last_sent_nonnegative_packet_id_ = -1;
  int next_stream_id_ = 665;
  Call::Stats stats_;
  std::vector<FakeVideoSendStream*> video_send_streams_;
  std::vector<FakeAudioSendStream*> audio_send_streams_;
  std::vector<FakeVideoReceiveStream*> video_receive_streams_;
  std::vector<FakeAudioReceiveStream*> audio_receive_streams_;
  std::vector<FakeFlexfecReceiveStream*> flexfec_receive_streams_;
  std::map<uint32_t, size_t> delivered_packets_by_ssrc_;

  int num_created_send_streams_;
  int num_created_receive_streams_;

  FakePayloadTypeSuggester pt_suggester_;
};

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace cricket {
using ::webrtc::FakeAudioReceiveStream;
using ::webrtc::FakeAudioSendStream;
using ::webrtc::FakeCall;
using ::webrtc::FakeFlexfecReceiveStream;
using ::webrtc::FakeVideoReceiveStream;
using ::webrtc::FakeVideoSendStream;
}  // namespace cricket
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES
#endif  // MEDIA_ENGINE_FAKE_WEBRTC_CALL_H_
