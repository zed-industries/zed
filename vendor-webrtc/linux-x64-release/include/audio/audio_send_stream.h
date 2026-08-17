/*
 *  Copyright (c) 2015 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef AUDIO_AUDIO_SEND_STREAM_H_
#define AUDIO_AUDIO_SEND_STREAM_H_

#include <memory>
#include <utility>
#include <vector>

#include "absl/functional/any_invocable.h"
#include "api/environment/environment.h"
#include "api/field_trials_view.h"
#include "api/sequence_checker.h"
#include "api/task_queue/task_queue_base.h"
#include "audio/audio_level.h"
#include "audio/channel_send.h"
#include "call/audio_send_stream.h"
#include "call/audio_state.h"
#include "call/bitrate_allocator.h"
#include "modules/rtp_rtcp/source/rtp_rtcp_interface.h"
#include "rtc_base/experiments/struct_parameters_parser.h"
#include "rtc_base/race_checker.h"
#include "rtc_base/synchronization/mutex.h"

namespace webrtc {
class RtcpRttStats;
class RtpTransportControllerSendInterface;

struct AudioAllocationConfig {
  static constexpr char kKey[] = "WebRTC-Audio-Allocation";
  // Field Trial configured bitrates to use as overrides over default/user
  // configured bitrate range when audio bitrate allocation is enabled.
  std::optional<DataRate> min_bitrate;
  std::optional<DataRate> max_bitrate;
  DataRate priority_bitrate = DataRate::Zero();
  // By default the priority_bitrate is compensated for packet overhead.
  // Use this flag to configure a raw value instead.
  std::optional<DataRate> priority_bitrate_raw;
  std::optional<double> bitrate_priority;

  std::unique_ptr<StructParametersParser> Parser();
  explicit AudioAllocationConfig(const FieldTrialsView& field_trials);
};
namespace internal {
class AudioState;

class AudioSendStream final : public webrtc::AudioSendStream,
                              public webrtc::BitrateAllocatorObserver {
 public:
  AudioSendStream(const Environment& env,
                  const webrtc::AudioSendStream::Config& config,
                  const scoped_refptr<webrtc::AudioState>& audio_state,
                  RtpTransportControllerSendInterface* rtp_transport,
                  BitrateAllocatorInterface* bitrate_allocator,
                  RtcpRttStats* rtcp_rtt_stats,
                  const std::optional<RtpState>& suspended_rtp_state);
  // For unit tests, which need to supply a mock ChannelSend.
  AudioSendStream(const Environment& env,
                  const webrtc::AudioSendStream::Config& config,
                  const scoped_refptr<webrtc::AudioState>& audio_state,
                  RtpTransportControllerSendInterface* rtp_transport,
                  BitrateAllocatorInterface* bitrate_allocator,
                  const std::optional<RtpState>& suspended_rtp_state,
                  std::unique_ptr<voe::ChannelSendInterface> channel_send);

  AudioSendStream() = delete;
  AudioSendStream(const AudioSendStream&) = delete;
  AudioSendStream& operator=(const AudioSendStream&) = delete;

  ~AudioSendStream() override;

  // webrtc::AudioSendStream implementation.
  const webrtc::AudioSendStream::Config& GetConfig() const override;
  void Reconfigure(const webrtc::AudioSendStream::Config& config,
                   SetParametersCallback callback) override;
  void Start() override;
  void Stop() override;
  void SendAudioData(std::unique_ptr<AudioFrame> audio_frame) override;
  bool SendTelephoneEvent(int payload_type,
                          int payload_frequency,
                          int event,
                          int duration_ms) override;
  void SetMuted(bool muted) override;
  webrtc::AudioSendStream::Stats GetStats() const override;
  webrtc::AudioSendStream::Stats GetStats(
      bool has_remote_tracks) const override;

  void DeliverRtcp(const uint8_t* packet, size_t length);

  // Implements BitrateAllocatorObserver.
  uint32_t OnBitrateUpdated(BitrateAllocationUpdate update) override;
  std::optional<DataRate> GetUsedRate() const override;

  void SetTransportOverhead(int transport_overhead_per_packet_bytes);

  RtpState GetRtpState() const;
  const voe::ChannelSendInterface* GetChannel() const;

  // Returns combined per-packet overhead.
  size_t TestOnlyGetPerPacketOverheadBytes() const;

 private:
  class TimedTransport;
  // Constraints including overhead.
  struct TargetAudioBitrateConstraints {
    DataRate min;
    DataRate max;
  };

  internal::AudioState* audio_state();
  const internal::AudioState* audio_state() const;

  void StoreEncoderProperties(int sample_rate_hz, size_t num_channels)
      RTC_RUN_ON(worker_thread_checker_);

  void ConfigureStream(const Config& new_config,
                       bool first_time,
                       SetParametersCallback callback)
      RTC_RUN_ON(worker_thread_checker_);
  bool SetupSendCodec(const Config& new_config)
      RTC_RUN_ON(worker_thread_checker_);
  bool ReconfigureSendCodec(const Config& new_config)
      RTC_RUN_ON(worker_thread_checker_);
  void ReconfigureANA(const Config& new_config)
      RTC_RUN_ON(worker_thread_checker_);
  void ReconfigureCNG(const Config& new_config)
      RTC_RUN_ON(worker_thread_checker_);
  void ReconfigureBitrateObserver(const Config& new_config)
      RTC_RUN_ON(worker_thread_checker_);

  void ConfigureBitrateObserver() RTC_RUN_ON(worker_thread_checker_);
  void RemoveBitrateObserver() RTC_RUN_ON(worker_thread_checker_);

  // Returns bitrate constraints, maybe including overhead when enabled by
  // field trial.
  std::optional<TargetAudioBitrateConstraints> GetMinMaxBitrateConstraints()
      const RTC_RUN_ON(worker_thread_checker_);

  // Sets per-packet overhead on encoded (for ANA) based on current known values
  // of transport and packetization overheads.
  void UpdateOverheadPerPacket();

  void RegisterCngPayloadType(int payload_type, int clockrate_hz)
      RTC_RUN_ON(worker_thread_checker_);

  const Environment env_;

  SequenceChecker worker_thread_checker_;
  RaceChecker audio_capture_race_checker_;

  const bool allocate_audio_without_feedback_;
  const bool force_no_audio_feedback_ = allocate_audio_without_feedback_;
  const bool enable_audio_alr_probing_;
  const AudioAllocationConfig allocation_settings_;

  webrtc::AudioSendStream::Config config_
      RTC_GUARDED_BY(worker_thread_checker_);
  scoped_refptr<webrtc::AudioState> audio_state_;
  const std::unique_ptr<voe::ChannelSendInterface> channel_send_;
  const bool use_legacy_overhead_calculation_;
  const bool enable_priority_bitrate_;

  int encoder_sample_rate_hz_ RTC_GUARDED_BY(worker_thread_checker_) = 0;
  size_t encoder_num_channels_ RTC_GUARDED_BY(worker_thread_checker_) = 0;
  bool sending_ RTC_GUARDED_BY(worker_thread_checker_) = false;
  mutable Mutex audio_level_lock_;
  // Keeps track of audio level, total audio energy and total samples duration.
  // https://w3c.github.io/webrtc-stats/#dom-rtcaudiohandlerstats-totalaudioenergy
  webrtc::voe::AudioLevel audio_level_ RTC_GUARDED_BY(audio_level_lock_);

  BitrateAllocatorInterface* const bitrate_allocator_
      RTC_GUARDED_BY(worker_thread_checker_);
  RtpTransportControllerSendInterface* const rtp_transport_;

  RtpRtcpInterface* const rtp_rtcp_module_;
  std::optional<RtpState> const suspended_rtp_state_;

  // RFC 5285: Each distinct extension MUST have a unique ID. The value 0 is
  // reserved for padding and MUST NOT be used as a local identifier.
  // So it should be safe to use 0 here to indicate "not configured".
  struct ExtensionIds {
    int audio_level = 0;
    int abs_send_time = 0;
    int abs_capture_time = 0;
    int transport_sequence_number = 0;
    int mid = 0;
    int rid = 0;
    int repaired_rid = 0;
  };
  static ExtensionIds FindExtensionIds(
      const std::vector<RtpExtension>& extensions);
  static int TransportSeqNumId(const Config& config);

  // Current transport overhead (ICE, TURN, etc.)
  size_t transport_overhead_per_packet_bytes_
      RTC_GUARDED_BY(worker_thread_checker_) = 0;
  // Total overhead, including transport and RTP headers.
  size_t overhead_per_packet_ RTC_GUARDED_BY(worker_thread_checker_) = 0;

  bool registered_with_allocator_ RTC_GUARDED_BY(worker_thread_checker_) =
      false;
  std::optional<std::pair<TimeDelta, TimeDelta>> frame_length_range_
      RTC_GUARDED_BY(worker_thread_checker_);
  std::optional<std::pair<DataRate, DataRate>> bitrate_range_
      RTC_GUARDED_BY(worker_thread_checker_);
};
}  // namespace internal
}  // namespace webrtc

#endif  // AUDIO_AUDIO_SEND_STREAM_H_
