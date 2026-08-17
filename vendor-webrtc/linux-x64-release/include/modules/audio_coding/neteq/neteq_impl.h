/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_CODING_NETEQ_NETEQ_IMPL_H_
#define MODULES_AUDIO_CODING_NETEQ_NETEQ_IMPL_H_

#include <cstddef>
#include <cstdint>
#include <map>
#include <memory>
#include <optional>
#include <vector>

#include "api/array_view.h"
#include "api/audio/audio_frame.h"
#include "api/audio_codecs/audio_decoder.h"
#include "api/audio_codecs/audio_decoder_factory.h"
#include "api/audio_codecs/audio_format.h"
#include "api/environment/environment.h"
#include "api/neteq/neteq.h"
#include "api/neteq/neteq_controller.h"
#include "api/neteq/neteq_controller_factory.h"
#include "api/neteq/tick_timer.h"
#include "api/rtp_headers.h"
#include "api/rtp_packet_info.h"
#include "api/scoped_refptr.h"
#include "modules/audio_coding/neteq/audio_multi_vector.h"
#include "modules/audio_coding/neteq/packet.h"
#include "modules/audio_coding/neteq/packet_buffer.h"
#include "modules/audio_coding/neteq/random_vector.h"
#include "modules/audio_coding/neteq/statistics_calculator.h"
#include "rtc_base/buffer.h"
#include "rtc_base/synchronization/mutex.h"
#include "rtc_base/thread_annotations.h"

namespace webrtc {

// Forward declarations.
class Accelerate;
class BackgroundNoise;
class ComfortNoise;
class DecoderDatabase;
class DtmfBuffer;
class DtmfToneGenerator;
class Expand;
class Merge;
class NackTracker;
class Normal;
class RedPayloadSplitter;
class PreemptiveExpand;
class RandomVector;
class SyncBuffer;
class TimestampScaler;
struct AccelerateFactory;
struct DtmfEvent;
struct ExpandFactory;
struct PreemptiveExpandFactory;

class NetEqImpl : public webrtc::NetEq {
 public:
  enum class OutputType {
    kNormalSpeech,
    kPLC,
    kCNG,
    kPLCCNG,
    kVadPassive,
    kCodecPLC
  };

  enum ErrorCodes {
    kNoError = 0,
    kOtherError,
    kUnknownRtpPayloadType,
    kDecoderNotFound,
    kInvalidPointer,
    kAccelerateError,
    kPreemptiveExpandError,
    kComfortNoiseErrorCode,
    kDecoderErrorCode,
    kOtherDecoderError,
    kInvalidOperation,
    kDtmfParsingError,
    kDtmfInsertError,
    kSampleUnderrun,
    kDecodedTooMuch,
    kRedundancySplitError,
    kPacketBufferCorruption
  };

  struct Dependencies {
    // The constructor populates the Dependencies struct with the default
    // implementations of the objects. They can all be replaced by the user
    // before sending the struct to the NetEqImpl constructor. However, there
    // are dependencies between some of the classes inside the struct, so
    // swapping out one may make it necessary to re-create another one.
    Dependencies(const Environment& env,
                 const NetEq::Config& config,
                 scoped_refptr<AudioDecoderFactory> decoder_factory,
                 const NetEqControllerFactory& controller_factory);
    ~Dependencies();

    const Environment env;
    std::unique_ptr<TickTimer> tick_timer;
    std::unique_ptr<StatisticsCalculator> stats;
    std::unique_ptr<DecoderDatabase> decoder_database;
    std::unique_ptr<DtmfBuffer> dtmf_buffer;
    std::unique_ptr<DtmfToneGenerator> dtmf_tone_generator;
    std::unique_ptr<PacketBuffer> packet_buffer;
    std::unique_ptr<NetEqController> neteq_controller;
    std::unique_ptr<RedPayloadSplitter> red_payload_splitter;
    std::unique_ptr<TimestampScaler> timestamp_scaler;
    std::unique_ptr<AccelerateFactory> accelerate_factory;
    std::unique_ptr<ExpandFactory> expand_factory;
    std::unique_ptr<PreemptiveExpandFactory> preemptive_expand_factory;
  };

  // Creates a new NetEqImpl object.
  NetEqImpl(const NetEq::Config& config,
            Dependencies&& deps,
            bool create_components = true);

  ~NetEqImpl() override;

  NetEqImpl(const NetEqImpl&) = delete;
  NetEqImpl& operator=(const NetEqImpl&) = delete;

  int InsertPacket(const RTPHeader& rtp_header,
                   ArrayView<const uint8_t> payload) override {
    return InsertPacket(
        rtp_header, payload,
        RtpPacketInfo(rtp_header, /*receive_time=*/Timestamp::MinusInfinity()));
  }

  // Inserts a new packet into NetEq. Returns 0 on success, -1 on failure.
  int InsertPacket(const RTPHeader& rtp_header,
                   ArrayView<const uint8_t> payload,
                   const RtpPacketInfo& packet_info) override;

  void InsertEmptyPacket(const RTPHeader& rtp_header) override;

  int GetAudio(
      AudioFrame* audio_frame,
      bool* muted = nullptr,
      int* current_sample_rate_hz = nullptr,
      std::optional<Operation> action_override = std::nullopt) override;

  void SetCodecs(const std::map<int, SdpAudioFormat>& codecs) override;

  bool RegisterPayloadType(int rtp_payload_type,
                           const SdpAudioFormat& audio_format) override;

  // Removes `rtp_payload_type` from the codec database. Returns 0 on success,
  // -1 on failure.
  int RemovePayloadType(uint8_t rtp_payload_type) override;

  void RemoveAllPayloadTypes() override;

  bool SetMinimumDelay(int delay_ms) override;

  bool SetMaximumDelay(int delay_ms) override;

  bool SetBaseMinimumDelayMs(int delay_ms) override;

  int GetBaseMinimumDelayMs() const override;

  int TargetDelayMs() const override;

  int FilteredCurrentDelayMs() const override;

  // Writes the current network statistics to `stats`. The statistics are reset
  // after the call.
  int NetworkStatistics(NetEqNetworkStatistics* stats) override;

  NetEqNetworkStatistics CurrentNetworkStatistics() const override;

  NetEqLifetimeStatistics GetLifetimeStatistics() const override;

  NetEqOperationsAndState GetOperationsAndState() const override;

  std::optional<uint32_t> GetPlayoutTimestamp() const override;

  int last_output_sample_rate_hz() const override;

  std::optional<DecoderFormat> GetCurrentDecoderFormat() const override;

  // Flushes both the packet buffer and the sync buffer.
  void FlushBuffers() override;

  void EnableNack(size_t max_nack_list_size) override;

  void DisableNack() override;

  std::vector<uint16_t> GetNackList(int64_t round_trip_time_ms) const override;

  int SyncBufferSizeMs() const override;

  // This accessor method is only intended for testing purposes.
  const SyncBuffer* sync_buffer_for_test() const;
  Operation last_operation_for_test() const;

 protected:
  static const int kOutputSizeMs = 10;
  static const size_t kMaxFrameSize = 5760;  // 120 ms @ 48 kHz.
  // TODO(hlundin): Provide a better value for kSyncBufferSize.
  // Current value is kMaxFrameSize + 60 ms * 48 kHz, which is enough for
  // calculating correlations of current frame against history.
  static const size_t kSyncBufferSize = kMaxFrameSize + 60 * 48;

  // Inserts a new packet into NetEq. This is used by the InsertPacket method
  // above. Returns 0 on success, otherwise an error code.
  // TODO(hlundin): Merge this with InsertPacket above?
  int InsertPacketInternal(const RTPHeader& rtp_header,
                           ArrayView<const uint8_t> payload,
                           const RtpPacketInfo& packet_info)
      RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_);

  // Returns true if the payload type changed (this should be followed by
  // resetting various state). Returns false if the current payload type is
  // unknown or equal to `payload_type`.
  bool MaybeChangePayloadType(uint8_t payload_type)
      RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_);

  // Delivers 10 ms of audio data. The data is written to `audio_frame`.
  // Returns 0 on success, otherwise an error code.
  int GetAudioInternal(AudioFrame* audio_frame,
                       std::optional<Operation> action_override)
      RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_);

  // Provides a decision to the GetAudioInternal method. The decision what to
  // do is written to `operation`. Packets to decode are written to
  // `packet_list`, and a DTMF event to play is written to `dtmf_event`. When
  // DTMF should be played, `play_dtmf` is set to true by the method.
  // Returns 0 on success, otherwise an error code.
  int GetDecision(Operation* operation,
                  PacketList* packet_list,
                  DtmfEvent* dtmf_event,
                  bool* play_dtmf,
                  std::optional<Operation> action_override)
      RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_);

  // Decodes the speech packets in `packet_list`, and writes the results to
  // `decoded_buffer`, which is allocated to hold `decoded_buffer_length`
  // elements. The length of the decoded data is written to `decoded_length`.
  // The speech type -- speech or (codec-internal) comfort noise -- is written
  // to `speech_type`. If `packet_list` contains any SID frames for RFC 3389
  // comfort noise, those are not decoded.
  int Decode(PacketList* packet_list,
             Operation* operation,
             int* decoded_length,
             AudioDecoder::SpeechType* speech_type)
      RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_);

  // Sub-method to Decode(). Performs codec internal CNG.
  int DecodeCng(AudioDecoder* decoder,
                int* decoded_length,
                AudioDecoder::SpeechType* speech_type)
      RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_);

  // Sub-method to Decode(). Performs the actual decoding.
  int DecodeLoop(PacketList* packet_list,
                 const Operation& operation,
                 AudioDecoder* decoder,
                 int* decoded_length,
                 AudioDecoder::SpeechType* speech_type)
      RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_);

  // Sub-method which calls the Normal class to perform the normal operation.
  void DoNormal(const int16_t* decoded_buffer,
                size_t decoded_length,
                AudioDecoder::SpeechType speech_type,
                bool play_dtmf) RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_);

  // Sub-method which calls the Merge class to perform the merge operation.
  void DoMerge(int16_t* decoded_buffer,
               size_t decoded_length,
               AudioDecoder::SpeechType speech_type,
               bool play_dtmf) RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_);

  bool DoCodecPlc() RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_);

  // Sub-method which calls the Expand class to perform the expand operation.
  int DoExpand(bool play_dtmf) RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_);

  // Sub-method which calls the Accelerate class to perform the accelerate
  // operation.
  int DoAccelerate(int16_t* decoded_buffer,
                   size_t decoded_length,
                   AudioDecoder::SpeechType speech_type,
                   bool play_dtmf,
                   bool fast_accelerate) RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_);

  // Sub-method which calls the PreemptiveExpand class to perform the
  // preemptive expand operation.
  int DoPreemptiveExpand(int16_t* decoded_buffer,
                         size_t decoded_length,
                         AudioDecoder::SpeechType speech_type,
                         bool play_dtmf) RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_);

  // Sub-method which calls the ComfortNoise class to generate RFC 3389 comfort
  // noise. `packet_list` can either contain one SID frame to update the
  // noise parameters, or no payload at all, in which case the previously
  // received parameters are used.
  int DoRfc3389Cng(PacketList* packet_list, bool play_dtmf)
      RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_);

  // Calls the audio decoder to generate codec-internal comfort noise when
  // no packet was received.
  void DoCodecInternalCng(const int16_t* decoded_buffer, size_t decoded_length)
      RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_);

  // Calls the DtmfToneGenerator class to generate DTMF tones.
  int DoDtmf(const DtmfEvent& dtmf_event, bool* play_dtmf)
      RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_);

  // Overdub DTMF on top of `output`.
  int DtmfOverdub(const DtmfEvent& dtmf_event,
                  size_t num_channels,
                  int16_t* output) const RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_);

  // Extracts packets from `packet_buffer_` to produce at least
  // `required_samples` samples. The packets are inserted into `packet_list`.
  // Returns the number of samples that the packets in the list will produce, or
  // -1 in case of an error.
  int ExtractPackets(size_t required_samples, PacketList* packet_list)
      RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_);

  // Resets various variables and objects to new values based on the sample rate
  // `fs_hz` and `channels` number audio channels.
  void SetSampleRateAndChannels(int fs_hz, size_t channels)
      RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_);

  // Returns the output type for the audio produced by the latest call to
  // GetAudio().
  OutputType LastOutputType() RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_);

  // Updates Expand and Merge.
  virtual void UpdatePlcComponents(int fs_hz, size_t channels)
      RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_);

  NetEqNetworkStatistics CurrentNetworkStatisticsInternal() const
      RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_);

  NetEqController::PacketArrivedInfo ToPacketArrivedInfo(
      const Packet& packet) const RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_);

  const Environment env_;

  mutable Mutex mutex_;
  const std::unique_ptr<TickTimer> tick_timer_ RTC_GUARDED_BY(mutex_);
  const std::unique_ptr<DecoderDatabase> decoder_database_
      RTC_GUARDED_BY(mutex_);
  const std::unique_ptr<DtmfBuffer> dtmf_buffer_ RTC_GUARDED_BY(mutex_);
  const std::unique_ptr<DtmfToneGenerator> dtmf_tone_generator_
      RTC_GUARDED_BY(mutex_);
  const std::unique_ptr<PacketBuffer> packet_buffer_ RTC_GUARDED_BY(mutex_);
  const std::unique_ptr<RedPayloadSplitter> red_payload_splitter_
      RTC_GUARDED_BY(mutex_);
  const std::unique_ptr<TimestampScaler> timestamp_scaler_
      RTC_GUARDED_BY(mutex_);
  const std::unique_ptr<ExpandFactory> expand_factory_ RTC_GUARDED_BY(mutex_);
  const std::unique_ptr<AccelerateFactory> accelerate_factory_
      RTC_GUARDED_BY(mutex_);
  const std::unique_ptr<PreemptiveExpandFactory> preemptive_expand_factory_
      RTC_GUARDED_BY(mutex_);
  const std::unique_ptr<StatisticsCalculator> stats_ RTC_GUARDED_BY(mutex_);

  std::unique_ptr<BackgroundNoise> background_noise_ RTC_GUARDED_BY(mutex_);
  std::unique_ptr<NetEqController> controller_ RTC_GUARDED_BY(mutex_);
  std::unique_ptr<AudioMultiVector> algorithm_buffer_ RTC_GUARDED_BY(mutex_);
  std::unique_ptr<SyncBuffer> sync_buffer_ RTC_GUARDED_BY(mutex_);
  std::unique_ptr<Expand> expand_ RTC_GUARDED_BY(mutex_);
  std::unique_ptr<Normal> normal_ RTC_GUARDED_BY(mutex_);
  std::unique_ptr<Merge> merge_ RTC_GUARDED_BY(mutex_);
  std::unique_ptr<Accelerate> accelerate_ RTC_GUARDED_BY(mutex_);
  std::unique_ptr<PreemptiveExpand> preemptive_expand_ RTC_GUARDED_BY(mutex_);
  RandomVector random_vector_ RTC_GUARDED_BY(mutex_);
  std::unique_ptr<ComfortNoise> comfort_noise_ RTC_GUARDED_BY(mutex_);
  int fs_hz_ RTC_GUARDED_BY(mutex_);
  int fs_mult_ RTC_GUARDED_BY(mutex_);
  int last_output_sample_rate_hz_ RTC_GUARDED_BY(mutex_);
  size_t output_size_samples_ RTC_GUARDED_BY(mutex_);
  size_t decoder_frame_length_ RTC_GUARDED_BY(mutex_);
  Mode last_mode_ RTC_GUARDED_BY(mutex_);
  Operation last_operation_ RTC_GUARDED_BY(mutex_);
  std::optional<AudioDecoder::SpeechType> last_decoded_type_
      RTC_GUARDED_BY(mutex_);
  size_t decoded_buffer_length_ RTC_GUARDED_BY(mutex_);
  std::unique_ptr<int16_t[]> decoded_buffer_ RTC_GUARDED_BY(mutex_);
  uint32_t playout_timestamp_ RTC_GUARDED_BY(mutex_);
  bool new_codec_ RTC_GUARDED_BY(mutex_);
  uint32_t timestamp_ RTC_GUARDED_BY(mutex_);
  bool reset_decoder_ RTC_GUARDED_BY(mutex_);
  std::optional<uint8_t> current_rtp_payload_type_ RTC_GUARDED_BY(mutex_);
  std::optional<uint8_t> current_cng_rtp_payload_type_ RTC_GUARDED_BY(mutex_);
  bool first_packet_ RTC_GUARDED_BY(mutex_);
  bool enable_fast_accelerate_ RTC_GUARDED_BY(mutex_);
  std::unique_ptr<NackTracker> nack_ RTC_GUARDED_BY(mutex_);
  bool nack_enabled_ RTC_GUARDED_BY(mutex_);
  const bool enable_muted_state_ RTC_GUARDED_BY(mutex_);
  std::unique_ptr<TickTimer::Stopwatch> generated_noise_stopwatch_
      RTC_GUARDED_BY(mutex_);
  std::vector<RtpPacketInfo> last_decoded_packet_infos_ RTC_GUARDED_BY(mutex_);
  bool no_time_stretching_ RTC_GUARDED_BY(mutex_);  // Only used for test.
  BufferT<int16_t> concealment_audio_ RTC_GUARDED_BY(mutex_);
};

}  // namespace webrtc
#endif  // MODULES_AUDIO_CODING_NETEQ_NETEQ_IMPL_H_
