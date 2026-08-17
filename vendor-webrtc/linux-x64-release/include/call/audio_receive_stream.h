/*
 *  Copyright (c) 2015 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef CALL_AUDIO_RECEIVE_STREAM_H_
#define CALL_AUDIO_RECEIVE_STREAM_H_

#include <cstddef>
#include <cstdint>
#include <map>
#include <optional>
#include <string>

#include "api/audio/audio_mixer.h"
#include "api/audio_codecs/audio_codec_pair_id.h"
#include "api/audio_codecs/audio_decoder_factory.h"
#include "api/audio_codecs/audio_format.h"
#include "api/call/transport.h"
#include "api/crypto/crypto_options.h"
#include "api/crypto/frame_decryptor_interface.h"
#include "api/frame_transformer_interface.h"
#include "api/rtp_headers.h"
#include "api/scoped_refptr.h"
#include "api/units/time_delta.h"
#include "api/units/timestamp.h"
#include "call/receive_stream.h"
#include "call/rtp_config.h"

namespace webrtc {
class AudioSinkInterface;

class AudioReceiveStreamInterface : public MediaReceiveStreamInterface {
 public:
  struct Stats {
    Stats();
    ~Stats();
    uint32_t remote_ssrc = 0;
    int64_t payload_bytes_received = 0;
    int64_t header_and_padding_bytes_received = 0;
    uint32_t packets_received = 0;
    uint64_t fec_packets_received = 0;
    uint64_t fec_packets_discarded = 0;
    int32_t packets_lost = 0;
    uint64_t packets_discarded = 0;
    uint32_t nacks_sent = 0;
    std::string codec_name;
    std::optional<int> codec_payload_type;
    uint32_t jitter_ms = 0;
    uint32_t jitter_buffer_ms = 0;
    uint32_t jitter_buffer_preferred_ms = 0;
    uint32_t delay_estimate_ms = 0;
    int32_t audio_level = -1;
    // Stats below correspond to similarly-named fields in the WebRTC stats
    // spec. https://w3c.github.io/webrtc-stats/#dom-rtcmediastreamtrackstats
    double total_output_energy = 0.0;
    uint64_t total_samples_received = 0;
    double total_output_duration = 0.0;
    uint64_t concealed_samples = 0;
    uint64_t silent_concealed_samples = 0;
    uint64_t concealment_events = 0;
    double jitter_buffer_delay_seconds = 0.0;
    uint64_t jitter_buffer_emitted_count = 0;
    double jitter_buffer_target_delay_seconds = 0.0;
    double jitter_buffer_minimum_delay_seconds = 0.0;
    uint64_t inserted_samples_for_deceleration = 0;
    uint64_t removed_samples_for_acceleration = 0;
    double total_processing_delay_seconds = 0.0;
    // Stats below DO NOT correspond directly to anything in the WebRTC stats
    float expand_rate = 0.0f;
    float speech_expand_rate = 0.0f;
    float secondary_decoded_rate = 0.0f;
    float secondary_discarded_rate = 0.0f;
    float accelerate_rate = 0.0f;
    float preemptive_expand_rate = 0.0f;
    uint64_t delayed_packet_outage_samples = 0;
    int32_t decoding_calls_to_silence_generator = 0;
    int32_t decoding_calls_to_neteq = 0;
    int32_t decoding_normal = 0;
    // TODO(alexnarest): Consider decoding_neteq_plc for consistency
    int32_t decoding_plc = 0;
    int32_t decoding_codec_plc = 0;
    int32_t decoding_cng = 0;
    int32_t decoding_plc_cng = 0;
    int32_t decoding_muted_output = 0;
    int64_t capture_start_ntp_time_ms = 0;
    // The timestamp at which the last packet was received, i.e. the time of the
    // local clock when it was received - not the RTP timestamp of that packet.
    // https://w3c.github.io/webrtc-stats/#dom-rtcinboundrtpstreamstats-lastpacketreceivedtimestamp
    std::optional<Timestamp> last_packet_received;
    uint64_t jitter_buffer_flushes = 0;
    double relative_packet_arrival_delay_seconds = 0.0;
    int32_t interruption_count = 0;
    int32_t total_interruption_duration_ms = 0;
    // https://w3c.github.io/webrtc-stats/#dom-rtcinboundrtpstreamstats-estimatedplayouttimestamp
    std::optional<int64_t> estimated_playout_ntp_timestamp_ms;
    // Remote outbound stats derived by the received RTCP sender reports.
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

  struct Config {
    Config();
    ~Config();

    std::string ToString() const;

    // Receive-stream specific RTP settings.
    struct Rtp : public ReceiveStreamRtpConfig {
      Rtp();
      ~Rtp();

      std::string ToString() const;

      // See NackConfig for description.
      NackConfig nack;
      RtcpMode rtcp_mode = RtcpMode::kCompound;
    } rtp;

    // Receive-side RTT.
    bool enable_non_sender_rtt = false;

    Transport* rtcp_send_transport = nullptr;

    // NetEq settings.
    size_t jitter_buffer_max_packets = 200;
    bool jitter_buffer_fast_accelerate = false;
    int jitter_buffer_min_delay_ms = 0;

    // Identifier for an A/V synchronization group. Empty string to disable.
    // TODO(pbos): Synchronize streams in a sync group, not just one video
    // stream to one audio stream. Tracked by issue webrtc:4762.
    std::string sync_group;

    // Decoder specifications for every payload type that we can receive.
    std::map<int, SdpAudioFormat> decoder_map;

    scoped_refptr<AudioDecoderFactory> decoder_factory;

    std::optional<AudioCodecPairId> codec_pair_id;

    // Per PeerConnection crypto options.
    webrtc::CryptoOptions crypto_options;

    // An optional custom frame decryptor that allows the entire frame to be
    // decrypted in whatever way the caller choses. This is not required by
    // default.
    // TODO(tommi): Remove this member variable from the struct. It's not
    // a part of the AudioReceiveStreamInterface state but rather a pass through
    // variable.
    scoped_refptr<webrtc::FrameDecryptorInterface> frame_decryptor;

    // An optional frame transformer used by insertable streams to transform
    // encoded frames.
    // TODO(tommi): Remove this member variable from the struct. It's not
    // a part of the AudioReceiveStreamInterface state but rather a pass through
    // variable.
    scoped_refptr<webrtc::FrameTransformerInterface> frame_transformer;
  };

  // Methods that support reconfiguring the stream post initialization.
  virtual void SetDecoderMap(std::map<int, SdpAudioFormat> decoder_map) = 0;
  virtual void SetNackHistory(int history_ms) = 0;
  virtual void SetNonSenderRttMeasurement(bool enabled) = 0;

  // Returns true if the stream has been started.
  virtual bool IsRunning() const = 0;

  virtual Stats GetStats(bool get_and_clear_legacy_stats) const = 0;
  Stats GetStats() { return GetStats(/*get_and_clear_legacy_stats=*/true); }

  // Sets an audio sink that receives unmixed audio from the receive stream.
  // Ownership of the sink is managed by the caller.
  // Only one sink can be set and passing a null sink clears an existing one.
  // NOTE: Audio must still somehow be pulled through AudioTransport for audio
  // to stream through this sink. In practice, this happens if mixed audio
  // is being pulled+rendered and/or if audio is being pulled for the purposes
  // of feeding to the AEC.
  virtual void SetSink(AudioSinkInterface* sink) = 0;

  // Sets playback gain of the stream, applied when mixing, and thus after it
  // is potentially forwarded to any attached AudioSinkInterface implementation.
  virtual void SetGain(float gain) = 0;

  // Sets a base minimum for the playout delay. Base minimum delay sets lower
  // bound on minimum delay value determining lower bound on playout delay.
  //
  // Returns true if value was successfully set, false overwise.
  virtual bool SetBaseMinimumPlayoutDelayMs(int delay_ms) = 0;

  // Returns current value of base minimum delay in milliseconds.
  virtual int GetBaseMinimumPlayoutDelayMs() const = 0;

  // Synchronization source (stream identifier) to be received.
  // This member will not change mid-stream and can be assumed to be const
  // post initialization.
  virtual uint32_t remote_ssrc() const = 0;

  // Get the object suitable to inject into the AudioMixer
  // (normally "this").
  virtual AudioMixer::Source* source() = 0;

 protected:
  virtual ~AudioReceiveStreamInterface() {}
};

}  // namespace webrtc

#endif  // CALL_AUDIO_RECEIVE_STREAM_H_
