/*
 *  Copyright (c) 2020 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef AUDIO_VOIP_AUDIO_EGRESS_H_
#define AUDIO_VOIP_AUDIO_EGRESS_H_

#include <memory>
#include <string>

#include "api/audio_codecs/audio_format.h"
#include "api/environment/environment.h"
#include "api/sequence_checker.h"
#include "api/task_queue/task_queue_base.h"
#include "api/task_queue/task_queue_factory.h"
#include "audio/audio_level.h"
#include "audio/utility/audio_frame_operations.h"
#include "call/audio_sender.h"
#include "modules/audio_coding/include/audio_coding_module.h"
#include "modules/rtp_rtcp/include/report_block_data.h"
#include "modules/rtp_rtcp/source/rtp_rtcp_interface.h"
#include "modules/rtp_rtcp/source/rtp_sender_audio.h"
#include "rtc_base/synchronization/mutex.h"
#include "rtc_base/system/no_unique_address.h"
#include "rtc_base/time_utils.h"

namespace webrtc {

// AudioEgress receives input samples from AudioDeviceModule via
// AudioTransportImpl through AudioSender interface. Once it encodes the sample
// via selected encoder through AudioPacketizationCallback interface, the
// encoded payload will be packetized by the RTP stack, resulting in ready to
// send RTP packet to remote endpoint.
//
// TaskQueue is used to encode and send RTP asynchrounously as some OS platform
// uses the same thread for both audio input and output sample deliveries which
// can affect audio quality.
//
// Note that this class is originally based on ChannelSend in
// audio/channel_send.cc with non-audio related logic trimmed as aimed for
// smaller footprint.
class AudioEgress : public AudioSender, public AudioPacketizationCallback {
 public:
  AudioEgress(const Environment& env, RtpRtcpInterface* rtp_rtcp);
  ~AudioEgress() override;

  // Set the encoder format and payload type for AudioCodingModule.
  // It's possible to change the encoder type during its active usage.
  // `payload_type` must be the type that is negotiated with peer through
  // offer/answer.
  void SetEncoder(int payload_type,
                  const SdpAudioFormat& encoder_format,
                  std::unique_ptr<AudioEncoder> encoder);

  // Start or stop sending operation of AudioEgress. This will start/stop
  // the RTP stack also causes encoder queue thread to start/stop
  // processing input audio samples. StartSend will return false if
  // a send codec has not been set.
  bool StartSend();
  void StopSend();

  // Query the state of the RTP stack. This returns true if StartSend()
  // called and false if StopSend() is called.
  bool IsSending() const;

  // Enable or disable Mute state.
  void SetMute(bool mute);

  // Retrieve current encoder format info. This returns encoder format set
  // by SetEncoder() and if encoder is not set, this will return nullopt.
  std::optional<SdpAudioFormat> GetEncoderFormat() const {
    MutexLock lock(&lock_);
    return encoder_format_;
  }

  // Register the payload type and sample rate for DTMF (RFC 4733) payload.
  void RegisterTelephoneEventType(int rtp_payload_type, int sample_rate_hz);

  // Send DTMF named event as specified by
  // https://tools.ietf.org/html/rfc4733#section-3.2
  // `duration_ms` specifies the duration of DTMF packets that will be emitted
  // in place of real RTP packets instead.
  // This will return true when requested dtmf event is successfully scheduled
  // otherwise false when the dtmf queue reached maximum of 20 events.
  bool SendTelephoneEvent(int dtmf_event, int duration_ms);

  // See comments on LevelFullRange, TotalEnergy, TotalDuration from
  // audio/audio_level.h.
  int GetInputAudioLevel() const { return input_audio_level_.LevelFullRange(); }
  double GetInputTotalEnergy() const {
    return input_audio_level_.TotalEnergy();
  }
  double GetInputTotalDuration() const {
    return input_audio_level_.TotalDuration();
  }

  // Implementation of AudioSender interface.
  void SendAudioData(std::unique_ptr<AudioFrame> audio_frame) override;

  // Implementation of AudioPacketizationCallback interface.
  int32_t SendData(AudioFrameType frame_type,
                   uint8_t payload_type,
                   uint32_t timestamp,
                   const uint8_t* payload_data,
                   size_t payload_size) override;

 private:
  void SetEncoderFormat(const SdpAudioFormat& encoder_format) {
    MutexLock lock(&lock_);
    encoder_format_ = encoder_format;
  }

  mutable Mutex lock_;

  // Current encoder format selected by caller.
  std::optional<SdpAudioFormat> encoder_format_ RTC_GUARDED_BY(lock_);

  // Synchronization is handled internally by RtpRtcp.
  RtpRtcpInterface* const rtp_rtcp_;

  // Synchronization is handled internally by RTPSenderAudio.
  RTPSenderAudio rtp_sender_audio_;

  // Synchronization is handled internally by AudioCodingModule.
  const std::unique_ptr<AudioCodingModule> audio_coding_;

  // Synchronization is handled internally by voe::AudioLevel.
  voe::AudioLevel input_audio_level_;

  // Struct that holds all variables used by encoder task queue.
  struct EncoderContext {
    // Offset used to mark rtp timestamp in sample rate unit in
    // newly received audio frame from AudioTransport.
    uint32_t frame_rtp_timestamp_ = 0;

    // Flag to track mute state from caller. `previously_muted_` is used to
    // track previous state as part of input to AudioFrameOperations::Mute
    // to implement fading effect when (un)mute is invoked.
    bool mute_ = false;
    bool previously_muted_ = false;
  };

  EncoderContext encoder_context_ RTC_GUARDED_BY(encoder_queue_checker_);

  std::unique_ptr<TaskQueueBase, TaskQueueDeleter> encoder_queue_;
  RTC_NO_UNIQUE_ADDRESS SequenceChecker encoder_queue_checker_;
};

}  // namespace webrtc

#endif  // AUDIO_VOIP_AUDIO_EGRESS_H_
