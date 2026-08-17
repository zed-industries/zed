/*
 *  Copyright (c) 2020 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef AUDIO_VOIP_VOIP_CORE_H_
#define AUDIO_VOIP_VOIP_CORE_H_

#include <map>
#include <memory>
#include <queue>
#include <unordered_map>
#include <vector>

#include "api/audio/audio_device.h"
#include "api/audio/audio_processing.h"
#include "api/audio_codecs/audio_decoder_factory.h"
#include "api/audio_codecs/audio_encoder_factory.h"
#include "api/environment/environment.h"
#include "api/scoped_refptr.h"
#include "api/voip/voip_base.h"
#include "api/voip/voip_codec.h"
#include "api/voip/voip_dtmf.h"
#include "api/voip/voip_engine.h"
#include "api/voip/voip_network.h"
#include "api/voip/voip_statistics.h"
#include "api/voip/voip_volume_control.h"
#include "audio/audio_transport_impl.h"
#include "audio/voip/audio_channel.h"
#include "modules/audio_mixer/audio_mixer_impl.h"
#include "rtc_base/synchronization/mutex.h"

namespace webrtc {

// VoipCore is the implementatino of VoIP APIs listed in api/voip directory.
// It manages a vector of AudioChannel objects where each is mapped with a
// ChannelId (int) type. ChannelId is the primary key to locate a specific
// AudioChannel object to operate requested VoIP API from the caller.
//
// This class receives required audio components from caller at construction and
// owns the life cycle of them to orchestrate the proper destruction sequence.
class VoipCore : public VoipEngine,
                 public VoipBase,
                 public VoipNetwork,
                 public VoipCodec,
                 public VoipDtmf,
                 public VoipStatistics,
                 public VoipVolumeControl {
 public:
  VoipCore(const Environment& env,
           scoped_refptr<AudioEncoderFactory> encoder_factory,
           scoped_refptr<AudioDecoderFactory> decoder_factory,
           scoped_refptr<AudioDeviceModule> audio_device_module,
           scoped_refptr<AudioProcessing> audio_processing);
  ~VoipCore() override = default;

  // Implements VoipEngine interfaces.
  VoipBase& Base() override { return *this; }
  VoipNetwork& Network() override { return *this; }
  VoipCodec& Codec() override { return *this; }
  VoipDtmf& Dtmf() override { return *this; }
  VoipStatistics& Statistics() override { return *this; }
  VoipVolumeControl& VolumeControl() override { return *this; }

  // Implements VoipBase interfaces.
  ChannelId CreateChannel(Transport* transport,
                          std::optional<uint32_t> local_ssrc) override;
  VoipResult ReleaseChannel(ChannelId channel_id) override;
  VoipResult StartSend(ChannelId channel_id) override;
  VoipResult StopSend(ChannelId channel_id) override;
  VoipResult StartPlayout(ChannelId channel_id) override;
  VoipResult StopPlayout(ChannelId channel_id) override;

  // Implements VoipNetwork interfaces.
  VoipResult ReceivedRTPPacket(ChannelId channel_id,
                               ArrayView<const uint8_t> rtp_packet) override;
  VoipResult ReceivedRTCPPacket(ChannelId channel_id,
                                ArrayView<const uint8_t> rtcp_packet) override;

  // Implements VoipCodec interfaces.
  VoipResult SetSendCodec(ChannelId channel_id,
                          int payload_type,
                          const SdpAudioFormat& encoder_format) override;
  VoipResult SetReceiveCodecs(
      ChannelId channel_id,
      const std::map<int, SdpAudioFormat>& decoder_specs) override;

  // Implements VoipDtmf interfaces.
  VoipResult RegisterTelephoneEventType(ChannelId channel_id,
                                        int rtp_payload_type,
                                        int sample_rate_hz) override;
  VoipResult SendDtmfEvent(ChannelId channel_id,
                           DtmfEvent dtmf_event,
                           int duration_ms) override;

  // Implements VoipStatistics interfaces.
  VoipResult GetIngressStatistics(ChannelId channel_id,
                                  IngressStatistics& ingress_stats) override;
  VoipResult GetChannelStatistics(ChannelId channe_id,
                                  ChannelStatistics& channel_stats) override;

  // Implements VoipVolumeControl interfaces.
  VoipResult SetInputMuted(ChannelId channel_id, bool enable) override;
  VoipResult GetInputVolumeInfo(ChannelId channel_id,
                                VolumeInfo& volume_info) override;
  VoipResult GetOutputVolumeInfo(ChannelId channel_id,
                                 VolumeInfo& volume_info) override;

 private:
  // Initialize ADM and default audio device if needed.
  // Returns true if ADM is successfully initialized or already in such state
  // (e.g called more than once). Returns false when ADM fails to initialize
  // which would presumably render further processing useless. Note that such
  // failure won't necessarily succeed in next initialization attempt as it
  // would mean changing the ADM implementation. From Android N and onwards, the
  // mobile app may not be able to gain microphone access when in background
  // mode. Therefore it would be better to delay the logic as late as possible.
  bool InitializeIfNeeded();

  // Fetches the corresponding AudioChannel assigned with given `channel`.
  // Returns nullptr if not found.
  scoped_refptr<AudioChannel> GetChannel(ChannelId channel_id);

  // Updates AudioTransportImpl with a new set of actively sending AudioSender
  // (AudioEgress). This needs to be invoked whenever StartSend/StopSend is
  // involved by caller. Returns false when the selected audio device fails to
  // initialize where it can't expect to deliver any audio input sample.
  bool UpdateAudioTransportWithSenders();

  // Synchronization for these are handled internally.
  const Environment env_;
  scoped_refptr<AudioEncoderFactory> encoder_factory_;
  scoped_refptr<AudioDecoderFactory> decoder_factory_;

  // Synchronization is handled internally by AudioProcessing.
  // Must be placed before `audio_device_module_` for proper destruction.
  scoped_refptr<AudioProcessing> audio_processing_;

  // Synchronization is handled internally by AudioMixer.
  // Must be placed before `audio_device_module_` for proper destruction.
  scoped_refptr<AudioMixer> audio_mixer_;

  // Synchronization is handled internally by AudioTransportImpl.
  // Must be placed before `audio_device_module_` for proper destruction.
  std::unique_ptr<AudioTransportImpl> audio_transport_;

  // Synchronization is handled internally by AudioDeviceModule.
  scoped_refptr<AudioDeviceModule> audio_device_module_;

  Mutex lock_;

  // Member to track a next ChannelId for new AudioChannel.
  int next_channel_id_ RTC_GUARDED_BY(lock_) = 0;

  // Container to track currently active AudioChannel objects mapped by
  // ChannelId.
  std::unordered_map<ChannelId, scoped_refptr<AudioChannel>> channels_
      RTC_GUARDED_BY(lock_);

  // Boolean flag to ensure initialization only occurs once.
  bool initialized_ RTC_GUARDED_BY(lock_) = false;
};

}  // namespace webrtc

#endif  // AUDIO_VOIP_VOIP_CORE_H_
