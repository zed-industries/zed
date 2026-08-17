/*
 *  Copyright (c) 2015 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef AUDIO_MOCK_VOE_CHANNEL_PROXY_H_
#define AUDIO_MOCK_VOE_CHANNEL_PROXY_H_

#include <cstddef>
#include <cstdint>
#include <map>
#include <memory>
#include <optional>
#include <utility>
#include <vector>

#include "absl/strings/string_view.h"
#include "api/audio/audio_frame.h"
#include "api/audio/audio_mixer.h"
#include "api/audio_codecs/audio_encoder.h"
#include "api/audio_codecs/audio_format.h"
#include "api/call/audio_sink.h"
#include "api/call/bitrate_allocation.h"
#include "api/crypto/frame_decryptor_interface.h"
#include "api/crypto/frame_encryptor_interface.h"
#include "api/frame_transformer_interface.h"
#include "api/function_view.h"
#include "api/rtp_headers.h"
#include "api/scoped_refptr.h"
#include "api/transport/rtp/rtp_source.h"
#include "api/units/data_rate.h"
#include "audio/channel_receive.h"
#include "audio/channel_send.h"
#include "call/syncable.h"
#include "modules/audio_coding/include/audio_coding_module_typedefs.h"
#include "modules/rtp_rtcp/include/report_block_data.h"
#include "modules/rtp_rtcp/source/rtp_packet_received.h"
#include "modules/rtp_rtcp/source/rtp_rtcp_interface.h"
#include "test/gmock.h"

namespace webrtc {
namespace test {

class MockChannelReceive : public voe::ChannelReceiveInterface {
 public:
  MOCK_METHOD(void, SetNACKStatus, (bool enable, int max_packets), (override));
  MOCK_METHOD(void, SetRtcpMode, (RtcpMode mode), (override));
  MOCK_METHOD(void, SetNonSenderRttMeasurement, (bool enabled), (override));
  MOCK_METHOD(void,
              RegisterReceiverCongestionControlObjects,
              (PacketRouter*),
              (override));
  MOCK_METHOD(void, ResetReceiverCongestionControlObjects, (), (override));
  MOCK_METHOD(CallReceiveStatistics, GetRTCPStatistics, (), (const, override));
  MOCK_METHOD(NetworkStatistics,
              GetNetworkStatistics,
              (bool),
              (const, override));
  MOCK_METHOD(AudioDecodingCallStats,
              GetDecodingCallStatistics,
              (),
              (const, override));
  MOCK_METHOD(int, GetSpeechOutputLevelFullRange, (), (const, override));
  MOCK_METHOD(double, GetTotalOutputEnergy, (), (const, override));
  MOCK_METHOD(double, GetTotalOutputDuration, (), (const, override));
  MOCK_METHOD(uint32_t, GetDelayEstimate, (), (const, override));
  MOCK_METHOD(void, SetSink, (AudioSinkInterface*), (override));
  MOCK_METHOD(void, OnRtpPacket, (const RtpPacketReceived& packet), (override));
  MOCK_METHOD(void,
              ReceivedRTCPPacket,
              (const uint8_t*, size_t length),
              (override));
  MOCK_METHOD(void, SetChannelOutputVolumeScaling, (float scaling), (override));
  MOCK_METHOD(AudioMixer::Source::AudioFrameInfo,
              GetAudioFrameWithInfo,
              (int sample_rate_hz, AudioFrame*),
              (override));
  MOCK_METHOD(int, PreferredSampleRate, (), (const, override));
  MOCK_METHOD(std::vector<RtpSource>, GetSources, (), (const, override));
  MOCK_METHOD(bool,
              GetPlayoutRtpTimestamp,
              (uint32_t*, int64_t*),
              (const, override));
  MOCK_METHOD(void,
              SetEstimatedPlayoutNtpTimestampMs,
              (int64_t ntp_timestamp_ms, int64_t time_ms),
              (override));
  MOCK_METHOD(std::optional<int64_t>,
              GetCurrentEstimatedPlayoutNtpTimestampMs,
              (int64_t now_ms),
              (const, override));
  MOCK_METHOD(std::optional<Syncable::Info>,
              GetSyncInfo,
              (),
              (const, override));
  MOCK_METHOD(bool, SetMinimumPlayoutDelay, (int delay_ms), (override));
  MOCK_METHOD(bool, SetBaseMinimumPlayoutDelayMs, (int delay_ms), (override));
  MOCK_METHOD(int, GetBaseMinimumPlayoutDelayMs, (), (const, override));
  MOCK_METHOD((std::optional<std::pair<int, SdpAudioFormat>>),
              GetReceiveCodec,
              (),
              (const, override));
  MOCK_METHOD(void,
              SetReceiveCodecs,
              ((const std::map<int, SdpAudioFormat>& codecs)),
              (override));
  MOCK_METHOD(void, StartPlayout, (), (override));
  MOCK_METHOD(void, StopPlayout, (), (override));
  MOCK_METHOD(void,
              SetDepacketizerToDecoderFrameTransformer,
              (webrtc::scoped_refptr<webrtc::FrameTransformerInterface>
                   frame_transformer),
              (override));
  MOCK_METHOD(
      void,
      SetFrameDecryptor,
      (webrtc::scoped_refptr<webrtc::FrameDecryptorInterface> frame_decryptor),
      (override));
  MOCK_METHOD(void, OnLocalSsrcChange, (uint32_t local_ssrc), (override));
};

class MockChannelSend : public voe::ChannelSendInterface {
 public:
  MOCK_METHOD(void,
              SetEncoder,
              (int payload_type,
               const SdpAudioFormat& encoder_format,
               std::unique_ptr<AudioEncoder> encoder),
              (override));
  MOCK_METHOD(
      void,
      ModifyEncoder,
      (webrtc::FunctionView<void(std::unique_ptr<AudioEncoder>*)> modifier),
      (override));
  MOCK_METHOD(void,
              CallEncoder,
              (webrtc::FunctionView<void(AudioEncoder*)> modifier),
              (override));
  MOCK_METHOD(void, SetRTCP_CNAME, (absl::string_view c_name), (override));
  MOCK_METHOD(void,
              SetSendAudioLevelIndicationStatus,
              (bool enable, int id),
              (override));
  MOCK_METHOD(void,
              RegisterSenderCongestionControlObjects,
              (RtpTransportControllerSendInterface*),
              (override));
  MOCK_METHOD(void, ResetSenderCongestionControlObjects, (), (override));
  MOCK_METHOD(CallSendStatistics, GetRTCPStatistics, (), (const, override));
  MOCK_METHOD(std::vector<ReportBlockData>,
              GetRemoteRTCPReportBlocks,
              (),
              (const, override));
  MOCK_METHOD(ANAStats, GetANAStatistics, (), (const, override));
  MOCK_METHOD(void,
              RegisterCngPayloadType,
              (int payload_type, int payload_frequency),
              (override));
  MOCK_METHOD(void,
              SetSendTelephoneEventPayloadType,
              (int payload_type, int payload_frequency),
              (override));
  MOCK_METHOD(bool,
              SendTelephoneEventOutband,
              (int event, int duration_ms),
              (override));
  MOCK_METHOD(void,
              OnBitrateAllocation,
              (BitrateAllocationUpdate update),
              (override));
  MOCK_METHOD(void, SetInputMute, (bool muted), (override));
  MOCK_METHOD(void,
              ReceivedRTCPPacket,
              (const uint8_t*, size_t length),
              (override));
  MOCK_METHOD(void,
              ProcessAndEncodeAudio,
              (std::unique_ptr<AudioFrame>),
              (override));
  MOCK_METHOD(RtpRtcpInterface*, GetRtpRtcp, (), (const, override));
  MOCK_METHOD(int, GetTargetBitrate, (), (const, override));
  MOCK_METHOD(void, StartSend, (), (override));
  MOCK_METHOD(void, StopSend, (), (override));
  MOCK_METHOD(void,
              SetFrameEncryptor,
              (webrtc::scoped_refptr<FrameEncryptorInterface> frame_encryptor),
              (override));
  MOCK_METHOD(void,
              SetEncoderToPacketizerFrameTransformer,
              (webrtc::scoped_refptr<webrtc::FrameTransformerInterface>
                   frame_transformer),
              (override));
  MOCK_METHOD(std::optional<DataRate>, GetUsedRate, (), (const, override));
  MOCK_METHOD(void,
              RegisterPacketOverhead,
              (int packet_byte_overhead),
              (override));
};
}  // namespace test
}  // namespace webrtc

#endif  // AUDIO_MOCK_VOE_CHANNEL_PROXY_H_
