/*
 *  Copyright (c) 2015 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef AUDIO_AUDIO_RECEIVE_STREAM_H_
#define AUDIO_AUDIO_RECEIVE_STREAM_H_

#include <cstddef>
#include <cstdint>
#include <map>
#include <memory>
#include <optional>
#include <string>
#include <vector>

#include "absl/strings/string_view.h"
#include "api/audio/audio_frame.h"
#include "api/audio/audio_mixer.h"
#include "api/audio_codecs/audio_format.h"
#include "api/crypto/frame_decryptor_interface.h"
#include "api/environment/environment.h"
#include "api/frame_transformer_interface.h"
#include "api/neteq/neteq_factory.h"
#include "api/rtp_headers.h"
#include "api/scoped_refptr.h"
#include "api/sequence_checker.h"
#include "api/transport/rtp/rtp_source.h"
#include "audio/audio_state.h"
#include "call/audio_receive_stream.h"
#include "call/audio_state.h"
#include "call/syncable.h"
#include "rtc_base/system/no_unique_address.h"
#include "rtc_base/thread_annotations.h"

namespace webrtc {
class PacketRouter;
class RtpStreamReceiverControllerInterface;
class RtpStreamReceiverInterface;

namespace voe {
class ChannelReceiveInterface;
}  // namespace voe

class AudioReceiveStreamImpl final : public webrtc::AudioReceiveStreamInterface,
                                     public AudioMixer::Source,
                                     public Syncable {
 public:
  AudioReceiveStreamImpl(
      const Environment& env,
      PacketRouter* packet_router,
      NetEqFactory* neteq_factory,
      const webrtc::AudioReceiveStreamInterface::Config& config,
      const scoped_refptr<webrtc::AudioState>& audio_state);
  // For unit tests, which need to supply a mock channel receive.
  AudioReceiveStreamImpl(
      const Environment& env,
      PacketRouter* packet_router,
      const webrtc::AudioReceiveStreamInterface::Config& config,
      const scoped_refptr<webrtc::AudioState>& audio_state,
      std::unique_ptr<voe::ChannelReceiveInterface> channel_receive);

  AudioReceiveStreamImpl() = delete;
  AudioReceiveStreamImpl(const AudioReceiveStreamImpl&) = delete;
  AudioReceiveStreamImpl& operator=(const AudioReceiveStreamImpl&) = delete;

  // Destruction happens on the worker thread. Prior to destruction the caller
  // must ensure that a registration with the transport has been cleared. See
  // `RegisterWithTransport` for details.
  // TODO(tommi): As a further improvement to this, performing the full
  // destruction on the network thread could be made the default.
  ~AudioReceiveStreamImpl() override;

  // Called on the network thread to register/unregister with the network
  // transport.
  void RegisterWithTransport(
      RtpStreamReceiverControllerInterface* receiver_controller);
  // If registration has previously been done (via `RegisterWithTransport`) then
  // `UnregisterFromTransport` must be called prior to destruction, on the
  // network thread.
  void UnregisterFromTransport();

  // webrtc::AudioReceiveStreamInterface implementation.
  void Start() override;
  void Stop() override;
  bool IsRunning() const override;
  void SetDepacketizerToDecoderFrameTransformer(
      scoped_refptr<webrtc::FrameTransformerInterface> frame_transformer)
      override;
  void SetDecoderMap(std::map<int, SdpAudioFormat> decoder_map) override;
  void SetNackHistory(int history_ms) override;
  void SetRtcpMode(RtcpMode mode) override;
  void SetNonSenderRttMeasurement(bool enabled) override;
  void SetFrameDecryptor(
      scoped_refptr<webrtc::FrameDecryptorInterface> frame_decryptor) override;

  webrtc::AudioReceiveStreamInterface::Stats GetStats(
      bool get_and_clear_legacy_stats) const override;
  void SetSink(AudioSinkInterface* sink) override;
  void SetGain(float gain) override;
  bool SetBaseMinimumPlayoutDelayMs(int delay_ms) override;
  int GetBaseMinimumPlayoutDelayMs() const override;
  std::vector<webrtc::RtpSource> GetSources() const override;
  AudioMixer::Source* source() override { return this; }

  // AudioMixer::Source
  AudioFrameInfo GetAudioFrameWithInfo(int sample_rate_hz,
                                       AudioFrame* audio_frame) override;
  int Ssrc() const override;
  int PreferredSampleRate() const override;

  // Syncable
  uint32_t id() const override;
  std::optional<Syncable::Info> GetInfo() const override;
  bool GetPlayoutRtpTimestamp(uint32_t* rtp_timestamp,
                              int64_t* time_ms) const override;
  void SetEstimatedPlayoutNtpTimestampMs(int64_t ntp_timestamp_ms,
                                         int64_t time_ms) override;
  bool SetMinimumPlayoutDelay(int delay_ms) override;

  void DeliverRtcp(const uint8_t* packet, size_t length);

  void SetSyncGroup(absl::string_view sync_group);

  void SetLocalSsrc(uint32_t local_ssrc);

  uint32_t local_ssrc() const;

  uint32_t remote_ssrc() const override {
    // The remote_ssrc member variable of config_ will never change and can be
    // considered const.
    return config_.rtp.remote_ssrc;
  }

  // Returns a reference to the currently set sync group of the stream.
  // Must be called on the packet delivery thread.
  const std::string& sync_group() const;

  // TODO(tommi): Remove this method.
  void ReconfigureForTesting(
      const webrtc::AudioReceiveStreamInterface::Config& config);

 private:
  internal::AudioState* audio_state() const;

  RTC_NO_UNIQUE_ADDRESS SequenceChecker worker_thread_checker_;
  // TODO(bugs.webrtc.org/11993): This checker conceptually represents
  // operations that belong to the network thread. The Call class is currently
  // moving towards handling network packets on the network thread and while
  // that work is ongoing, this checker may in practice represent the worker
  // thread, but still serves as a mechanism of grouping together concepts
  // that belong to the network thread. Once the packets are fully delivered
  // on the network thread, this comment will be deleted.
  RTC_NO_UNIQUE_ADDRESS SequenceChecker packet_sequence_checker_{
      SequenceChecker::kDetached};
  webrtc::AudioReceiveStreamInterface::Config config_;
  scoped_refptr<webrtc::AudioState> audio_state_;
  const std::unique_ptr<voe::ChannelReceiveInterface> channel_receive_;

  bool playing_ RTC_GUARDED_BY(worker_thread_checker_) = false;

  std::unique_ptr<RtpStreamReceiverInterface> rtp_stream_receiver_
      RTC_GUARDED_BY(packet_sequence_checker_);
};
}  // namespace webrtc

#endif  // AUDIO_AUDIO_RECEIVE_STREAM_H_
