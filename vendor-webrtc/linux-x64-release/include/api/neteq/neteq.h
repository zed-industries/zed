/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_NETEQ_NETEQ_H_
#define API_NETEQ_NETEQ_H_

#include <stddef.h>  // Provide access to size_t.
#include <stdint.h>

#include <map>
#include <optional>
#include <string>
#include <vector>

#include "api/array_view.h"
#include "api/audio_codecs/audio_codec_pair_id.h"
#include "api/audio_codecs/audio_format.h"
#include "api/rtp_headers.h"
#include "api/rtp_packet_info.h"
#include "api/units/timestamp.h"

namespace webrtc {

// Forward declarations.
class AudioFrame;

struct NetEqNetworkStatistics {
  uint16_t current_buffer_size_ms;    // Current jitter buffer size in ms.
  uint16_t preferred_buffer_size_ms;  // Target buffer size in ms.
  uint16_t jitter_peaks_found;        // 1 if adding extra delay due to peaky
                                      // jitter; 0 otherwise.
  uint16_t expand_rate;         // Fraction (of original stream) of synthesized
                                // audio inserted through expansion (in Q14).
  uint16_t speech_expand_rate;  // Fraction (of original stream) of synthesized
                                // speech inserted through expansion (in Q14).
  uint16_t preemptive_rate;     // Fraction of data inserted through pre-emptive
                                // expansion (in Q14).
  uint16_t accelerate_rate;     // Fraction of data removed through acceleration
                                // (in Q14).
  uint16_t secondary_decoded_rate;    // Fraction of data coming from FEC/RED
                                      // decoding (in Q14).
  uint16_t secondary_discarded_rate;  // Fraction of discarded FEC/RED data (in
                                      // Q14).
  // Statistics for packet waiting times, i.e., the time between a packet
  // arrives until it is decoded.
  int mean_waiting_time_ms;
  int median_waiting_time_ms;
  int min_waiting_time_ms;
  int max_waiting_time_ms;
};

// NetEq statistics that persist over the lifetime of the class.
// These metrics are never reset.
struct NetEqLifetimeStatistics {
  // Stats below correspond to similarly-named fields in the WebRTC stats spec.
  // https://w3c.github.io/webrtc-stats/#dom-rtcinboundrtpstreamstats
  uint64_t total_samples_received = 0;
  uint64_t concealed_samples = 0;
  uint64_t concealment_events = 0;
  uint64_t jitter_buffer_delay_ms = 0;
  uint64_t jitter_buffer_emitted_count = 0;
  uint64_t jitter_buffer_target_delay_ms = 0;
  uint64_t jitter_buffer_minimum_delay_ms = 0;
  uint64_t inserted_samples_for_deceleration = 0;
  uint64_t removed_samples_for_acceleration = 0;
  uint64_t silent_concealed_samples = 0;
  uint64_t fec_packets_received = 0;
  uint64_t fec_packets_discarded = 0;
  uint64_t packets_discarded = 0;
  // Below stats are not part of the spec.
  uint64_t delayed_packet_outage_samples = 0;
  uint64_t delayed_packet_outage_events = 0;
  // This is sum of relative packet arrival delays of received packets so far.
  // Since end-to-end delay of a packet is difficult to measure and is not
  // necessarily useful for measuring jitter buffer performance, we report a
  // relative packet arrival delay. The relative packet arrival delay of a
  // packet is defined as the arrival delay compared to the first packet
  // received, given that it had zero delay. To avoid clock drift, the "first"
  // packet can be made dynamic.
  uint64_t relative_packet_arrival_delay_ms = 0;
  uint64_t jitter_buffer_packets_received = 0;
  // An interruption is a loss-concealment event lasting at least 150 ms. The
  // two stats below count the number os such events and the total duration of
  // these events.
  int32_t interruption_count = 0;
  int32_t total_interruption_duration_ms = 0;
  // Total number of comfort noise samples generated during DTX.
  uint64_t generated_noise_samples = 0;
  uint64_t total_processing_delay_us = 0;
};

// Metrics that describe the operations performed in NetEq, and the internal
// state.
struct NetEqOperationsAndState {
  // These sample counters are cumulative, and don't reset. As a reference, the
  // total number of output samples can be found in
  // NetEqLifetimeStatistics::total_samples_received.
  uint64_t preemptive_samples = 0;
  uint64_t accelerate_samples = 0;
  // Count of the number of buffer flushes.
  uint64_t packet_buffer_flushes = 0;
  // The statistics below are not cumulative.
  // The waiting time of the last decoded packet.
  uint64_t last_waiting_time_ms = 0;
  // The sum of the packet and jitter buffer size in ms.
  uint64_t current_buffer_size_ms = 0;
  // The current frame size in ms.
  uint64_t current_frame_size_ms = 0;
  // Flag to indicate that the next packet is available.
  bool next_packet_available = false;
};

// This is the interface class for NetEq.
class NetEq {
 public:
  struct Config {
    Config();
    Config(const Config&);
    Config(Config&&);
    ~Config();
    Config& operator=(const Config&);
    Config& operator=(Config&&);

    std::string ToString() const;

    int sample_rate_hz = 48000;  // Initial value. Will change with input data.
    size_t max_packets_in_buffer = 200;
    int max_delay_ms = 0;
    int min_delay_ms = 0;
    bool enable_fast_accelerate = false;
    bool enable_muted_state = false;
    bool enable_rtx_handling = false;
    std::optional<AudioCodecPairId> codec_pair_id;
    bool for_test_no_time_stretching = false;  // Use only for testing.
  };

  enum ReturnCodes { kOK = 0, kFail = -1 };

  enum class Operation {
    kNormal,
    kMerge,
    kExpand,
    kAccelerate,
    kFastAccelerate,
    kPreemptiveExpand,
    kRfc3389Cng,
    kRfc3389CngNoPacket,
    kCodecInternalCng,
    kDtmf,
    kUndefined,
  };

  enum class Mode {
    kNormal,
    kExpand,
    kMerge,
    kAccelerateSuccess,
    kAccelerateLowEnergy,
    kAccelerateFail,
    kPreemptiveExpandSuccess,
    kPreemptiveExpandLowEnergy,
    kPreemptiveExpandFail,
    kRfc3389Cng,
    kCodecInternalCng,
    kCodecPlc,
    kDtmf,
    kError,
    kUndefined,
  };

  // Return type for GetDecoderFormat.
  struct DecoderFormat {
    int payload_type;
    int sample_rate_hz;
    int num_channels;
    SdpAudioFormat sdp_format;
  };

  virtual ~NetEq() {}

  virtual int InsertPacket(const RTPHeader& rtp_header,
                           ArrayView<const uint8_t> payload) {
    return InsertPacket(rtp_header, payload,
                        /*receive_time=*/Timestamp::MinusInfinity());
  }

  // TODO: webrtc:343501093 - removed unused method.
  virtual int InsertPacket(const RTPHeader& rtp_header,
                           ArrayView<const uint8_t> payload,
                           Timestamp receive_time) {
    return InsertPacket(rtp_header, payload,
                        RtpPacketInfo(rtp_header, receive_time));
  }

  // Inserts a new packet into NetEq.
  // Returns 0 on success, -1 on failure.
  // TODO: webrtc:343501093 - Make this method pure virtual.
  virtual int InsertPacket(const RTPHeader& rtp_header,
                           ArrayView<const uint8_t> payload,
                           const RtpPacketInfo& /* rtp_packet_info */) {
    return InsertPacket(rtp_header, payload);
  }

  // Lets NetEq know that a packet arrived with an empty payload. This typically
  // happens when empty packets are used for probing the network channel, and
  // these packets use RTP sequence numbers from the same series as the actual
  // audio packets.
  virtual void InsertEmptyPacket(const RTPHeader& rtp_header) = 0;

  // Instructs NetEq to deliver 10 ms of audio data. The data is written to
  // `audio_frame`. All data in `audio_frame` is wiped; `data_`, `speech_type_`,
  // `num_channels_`, `sample_rate_hz_` and `samples_per_channel_` are updated
  // upon success. If an error is returned, some fields may not have been
  // updated, or may contain inconsistent values. If muted state is enabled
  // (through Config::enable_muted_state), `muted` may be set to true after a
  // prolonged expand period. When this happens, the `data_` in `audio_frame`
  // is not written, but should be interpreted as being all zeros. For testing
  // purposes, an override can be supplied in the `action_override` argument,
  // which will cause NetEq to take this action next, instead of the action it
  // would normally choose. An optional output argument for fetching the current
  // sample rate can be provided, which will return the same value as
  // last_output_sample_rate_hz() but will avoid additional synchronization.
  // Returns kOK on success, or kFail in case of an error.
  virtual int GetAudio(
      AudioFrame* audio_frame,
      bool* muted = nullptr,
      int* current_sample_rate_hz = nullptr,
      std::optional<Operation> action_override = std::nullopt) = 0;

  // Replaces the current set of decoders with the given one.
  virtual void SetCodecs(const std::map<int, SdpAudioFormat>& codecs) = 0;

  // Associates `rtp_payload_type` with the given codec, which NetEq will
  // instantiate when it needs it. Returns true iff successful.
  virtual bool RegisterPayloadType(int rtp_payload_type,
                                   const SdpAudioFormat& audio_format) = 0;

  // Removes `rtp_payload_type` from the codec database. Returns 0 on success,
  // -1 on failure. Removing a payload type that is not registered is ok and
  // will not result in an error.
  virtual int RemovePayloadType(uint8_t rtp_payload_type) = 0;

  // Removes all payload types from the codec database.
  virtual void RemoveAllPayloadTypes() = 0;

  // Sets a minimum delay in millisecond for packet buffer. The minimum is
  // maintained unless a higher latency is dictated by channel condition.
  // Returns true if the minimum is successfully applied, otherwise false is
  // returned.
  virtual bool SetMinimumDelay(int delay_ms) = 0;

  // Sets a maximum delay in milliseconds for packet buffer. The latency will
  // not exceed the given value, even required delay (given the channel
  // conditions) is higher. Calling this method has the same effect as setting
  // the `max_delay_ms` value in the NetEq::Config struct.
  virtual bool SetMaximumDelay(int delay_ms) = 0;

  // Sets a base minimum delay in milliseconds for packet buffer. The minimum
  // delay which is set via `SetMinimumDelay` can't be lower than base minimum
  // delay. Calling this method is similar to setting the `min_delay_ms` value
  // in the NetEq::Config struct. Returns true if the base minimum is
  // successfully applied, otherwise false is returned.
  virtual bool SetBaseMinimumDelayMs(int delay_ms) = 0;

  // Returns current value of base minimum delay in milliseconds.
  virtual int GetBaseMinimumDelayMs() const = 0;

  // Returns the current target delay in ms. This includes any extra delay
  // requested through SetMinimumDelay.
  virtual int TargetDelayMs() const = 0;

  // Returns the current total delay (packet buffer and sync buffer) in ms,
  // with smoothing applied to even out short-time fluctuations due to jitter.
  // The packet buffer part of the delay is not updated during DTX/CNG periods.
  virtual int FilteredCurrentDelayMs() const = 0;

  // Writes the current network statistics to `stats`. The statistics are reset
  // after the call.
  virtual int NetworkStatistics(NetEqNetworkStatistics* stats) = 0;

  // Current values only, not resetting any state.
  virtual NetEqNetworkStatistics CurrentNetworkStatistics() const = 0;

  // Returns a copy of this class's lifetime statistics. These statistics are
  // never reset.
  virtual NetEqLifetimeStatistics GetLifetimeStatistics() const = 0;

  // Returns statistics about the performed operations and internal state. These
  // statistics are never reset.
  virtual NetEqOperationsAndState GetOperationsAndState() const = 0;

  // Returns the RTP timestamp for the last sample delivered by GetAudio().
  // The return value will be empty if no valid timestamp is available.
  virtual std::optional<uint32_t> GetPlayoutTimestamp() const = 0;

  // Returns the sample rate in Hz of the audio produced in the last GetAudio
  // call. If GetAudio has not been called yet, the configured sample rate
  // (Config::sample_rate_hz) is returned.
  virtual int last_output_sample_rate_hz() const = 0;

  // Returns the decoder info for the given payload type. Returns empty if no
  // such payload type was registered.
  [[deprecated(
      "Use GetCurrentDecoderFormat")]] virtual std::optional<DecoderFormat>
  GetDecoderFormat(int /* payload_type */) const {
    return std::nullopt;
  }

  // Returns info for the most recently used decoder.
  virtual std::optional<DecoderFormat> GetCurrentDecoderFormat() const {
    return std::nullopt;
  }

  // Flushes both the packet buffer and the sync buffer.
  virtual void FlushBuffers() = 0;

  // Enables NACK and sets the maximum size of the NACK list, which should be
  // positive and no larger than Nack::kNackListSizeLimit. If NACK is already
  // enabled then the maximum NACK list size is modified accordingly.
  virtual void EnableNack(size_t max_nack_list_size) = 0;

  virtual void DisableNack() = 0;

  // Returns a list of RTP sequence numbers corresponding to packets to be
  // retransmitted, given an estimate of the round-trip time in milliseconds.
  virtual std::vector<uint16_t> GetNackList(
      int64_t round_trip_time_ms) const = 0;

  // Returns the length of the audio yet to play in the sync buffer.
  // Mainly intended for testing.
  virtual int SyncBufferSizeMs() const = 0;
};

}  // namespace webrtc
#endif  // API_NETEQ_NETEQ_H_
