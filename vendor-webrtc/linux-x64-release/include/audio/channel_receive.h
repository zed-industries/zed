/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef AUDIO_CHANNEL_RECEIVE_H_
#define AUDIO_CHANNEL_RECEIVE_H_

#include <cstddef>
#include <cstdint>
#include <map>
#include <memory>
#include <optional>
#include <utility>
#include <vector>

#include "api/audio/audio_frame.h"
#include "api/audio/audio_mixer.h"
#include "api/audio_codecs/audio_codec_pair_id.h"
#include "api/audio_codecs/audio_decoder_factory.h"
#include "api/audio_codecs/audio_format.h"
#include "api/call/audio_sink.h"
#include "api/call/transport.h"
#include "api/crypto/crypto_options.h"
#include "api/environment/environment.h"
#include "api/frame_transformer_interface.h"
#include "api/neteq/neteq_factory.h"
#include "api/rtp_headers.h"
#include "api/scoped_refptr.h"
#include "api/transport/rtp/rtp_source.h"
#include "api/units/time_delta.h"
#include "api/units/timestamp.h"
#include "call/rtp_packet_sink_interface.h"
#include "call/syncable.h"
#include "modules/audio_coding/include/audio_coding_module_typedefs.h"

namespace webrtc {

class AudioDeviceModule;
class FrameDecryptorInterface;
class PacketRouter;
class RateLimiter;
class ReceiveStatistics;
class RtpPacketReceived;
class RtpRtcp;

struct CallReceiveStatistics {
  int packets_lost = 0;
  uint32_t jitter_ms = 0;
  int64_t payload_bytes_received = 0;
  int64_t header_and_padding_bytes_received = 0;
  int packets_received = 0;
  uint32_t nacks_sent = 0;
  // The capture NTP time (in local timebase) of the first played out audio
  // frame.
  int64_t capture_start_ntp_time_ms = 0;
  // The timestamp at which the last packet was received, i.e. the time of the
  // local clock when it was received - not the RTP timestamp of that packet.
  // https://w3c.github.io/webrtc-stats/#dom-rtcinboundrtpstreamstats-lastpacketreceivedtimestamp
  std::optional<Timestamp> last_packet_received;
  // Remote outbound stats derived by the received RTCP sender reports.
  // Note that the timestamps below correspond to the time elapsed since the
  // Unix epoch.
  // https://w3c.github.io/webrtc-stats/#remoteoutboundrtpstats-dict*
  std::optional<Timestamp> last_sender_report_timestamp;
  // TODO: bugs.webrtc.org/370535296 - Remove the utc timestamp when linked
  // issue is fixed.
  std::optional<Timestamp> last_sender_report_utc_timestamp;
  std::optional<Timestamp> last_sender_report_remote_utc_timestamp;
  uint64_t sender_reports_packets_sent = 0;
  uint64_t sender_reports_bytes_sent = 0;
  uint64_t sender_reports_reports_count = 0;
  std::optional<TimeDelta> round_trip_time;
  TimeDelta total_round_trip_time = TimeDelta::Zero();
  int round_trip_time_measurements = 0;
};

namespace voe {

class ChannelSendInterface;

// Interface class needed for AudioReceiveStreamInterface tests that use a
// MockChannelReceive.

class ChannelReceiveInterface : public RtpPacketSinkInterface {
 public:
  virtual ~ChannelReceiveInterface() = default;

  virtual void SetSink(AudioSinkInterface* sink) = 0;

  virtual void SetReceiveCodecs(
      const std::map<int, SdpAudioFormat>& codecs) = 0;

  virtual void StartPlayout() = 0;
  virtual void StopPlayout() = 0;

  // Payload type and format of last received RTP packet, if any.
  virtual std::optional<std::pair<int, SdpAudioFormat>> GetReceiveCodec()
      const = 0;

  virtual void ReceivedRTCPPacket(const uint8_t* data, size_t length) = 0;

  virtual void SetChannelOutputVolumeScaling(float scaling) = 0;
  virtual int GetSpeechOutputLevelFullRange() const = 0;
  // See description of "totalAudioEnergy" in the WebRTC stats spec:
  // https://w3c.github.io/webrtc-stats/#dom-rtcmediastreamtrackstats-totalaudioenergy
  virtual double GetTotalOutputEnergy() const = 0;
  virtual double GetTotalOutputDuration() const = 0;

  // Stats.
  virtual NetworkStatistics GetNetworkStatistics(
      bool get_and_clear_legacy_stats) const = 0;
  virtual AudioDecodingCallStats GetDecodingCallStatistics() const = 0;

  // Audio+Video Sync.
  virtual uint32_t GetDelayEstimate() const = 0;
  virtual bool SetMinimumPlayoutDelay(int delay_ms) = 0;
  virtual bool GetPlayoutRtpTimestamp(uint32_t* rtp_timestamp,
                                      int64_t* time_ms) const = 0;
  virtual void SetEstimatedPlayoutNtpTimestampMs(int64_t ntp_timestamp_ms,
                                                 int64_t time_ms) = 0;
  virtual std::optional<int64_t> GetCurrentEstimatedPlayoutNtpTimestampMs(
      int64_t now_ms) const = 0;

  // Audio quality.
  // Base minimum delay sets lower bound on minimum delay value which
  // determines minimum delay until audio playout.
  virtual bool SetBaseMinimumPlayoutDelayMs(int delay_ms) = 0;
  virtual int GetBaseMinimumPlayoutDelayMs() const = 0;

  // Produces the transport-related timestamps; current_delay_ms is left unset.
  virtual std::optional<Syncable::Info> GetSyncInfo() const = 0;

  virtual void RegisterReceiverCongestionControlObjects(
      PacketRouter* packet_router) = 0;
  virtual void ResetReceiverCongestionControlObjects() = 0;

  virtual CallReceiveStatistics GetRTCPStatistics() const = 0;
  virtual void SetNACKStatus(bool enable, int max_packets) = 0;
  virtual void SetRtcpMode(webrtc::RtcpMode mode) = 0;
  virtual void SetNonSenderRttMeasurement(bool enabled) = 0;

  virtual AudioMixer::Source::AudioFrameInfo GetAudioFrameWithInfo(
      int sample_rate_hz,
      AudioFrame* audio_frame) = 0;

  virtual int PreferredSampleRate() const = 0;

  virtual std::vector<RtpSource> GetSources() const = 0;

  // Sets a frame transformer between the depacketizer and the decoder, to
  // transform the received frames before decoding them.
  virtual void SetDepacketizerToDecoderFrameTransformer(
      scoped_refptr<webrtc::FrameTransformerInterface> frame_transformer) = 0;

  virtual void SetFrameDecryptor(
      scoped_refptr<webrtc::FrameDecryptorInterface> frame_decryptor) = 0;

  virtual void OnLocalSsrcChange(uint32_t local_ssrc) = 0;
};

std::unique_ptr<ChannelReceiveInterface> CreateChannelReceive(
    const Environment& env,
    NetEqFactory* neteq_factory,
    AudioDeviceModule* audio_device_module,
    Transport* rtcp_send_transport,
    uint32_t local_ssrc,
    uint32_t remote_ssrc,
    size_t jitter_buffer_max_packets,
    bool jitter_buffer_fast_playout,
    int jitter_buffer_min_delay_ms,
    bool enable_non_sender_rtt,
    scoped_refptr<AudioDecoderFactory> decoder_factory,
    std::optional<AudioCodecPairId> codec_pair_id,
    scoped_refptr<FrameDecryptorInterface> frame_decryptor,
    const webrtc::CryptoOptions& crypto_options,
    scoped_refptr<FrameTransformerInterface> frame_transformer);

}  // namespace voe
}  // namespace webrtc

#endif  // AUDIO_CHANNEL_RECEIVE_H_
