/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef AUDIO_CHANNEL_SEND_H_
#define AUDIO_CHANNEL_SEND_H_

#include <cstddef>
#include <cstdint>
#include <memory>
#include <optional>
#include <vector>

#include "absl/strings/string_view.h"
#include "api/audio/audio_frame.h"
#include "api/audio_codecs/audio_encoder.h"
#include "api/audio_codecs/audio_format.h"
#include "api/call/bitrate_allocation.h"
#include "api/crypto/crypto_options.h"
#include "api/environment/environment.h"
#include "api/frame_transformer_interface.h"
#include "api/function_view.h"
#include "api/scoped_refptr.h"
#include "api/units/data_rate.h"
#include "api/units/time_delta.h"
#include "modules/rtp_rtcp/include/report_block_data.h"
#include "modules/rtp_rtcp/include/rtp_rtcp_defines.h"
#include "modules/rtp_rtcp/source/rtp_rtcp_interface.h"

namespace webrtc {

class FrameEncryptorInterface;
class RtpTransportControllerSendInterface;

struct CallSendStatistics {
  int64_t rttMs;
  int64_t payload_bytes_sent;
  int64_t header_and_padding_bytes_sent;
  // https://w3c.github.io/webrtc-stats/#dom-rtcoutboundrtpstreamstats-retransmittedbytessent
  uint64_t retransmitted_bytes_sent;
  int packetsSent;
  // https://w3c.github.io/webrtc-stats/#dom-rtcoutboundrtpstreamstats-totalpacketsenddelay
  TimeDelta total_packet_send_delay = TimeDelta::Zero();
  // https://w3c.github.io/webrtc-stats/#dom-rtcoutboundrtpstreamstats-retransmittedpacketssent
  uint64_t retransmitted_packets_sent;
  // A snapshot of Report Blocks with additional data of interest to statistics.
  // Within this list, the sender-source SSRC pair is unique and per-pair the
  // ReportBlockData represents the latest Report Block that was received for
  // that pair.
  std::vector<ReportBlockData> report_block_datas;
  uint32_t nacks_received;
};

namespace voe {

class ChannelSendInterface {
 public:
  virtual ~ChannelSendInterface() = default;

  virtual void ReceivedRTCPPacket(const uint8_t* packet, size_t length) = 0;

  virtual CallSendStatistics GetRTCPStatistics() const = 0;

  virtual void SetEncoder(int payload_type,
                          const SdpAudioFormat& encoder_format,
                          std::unique_ptr<AudioEncoder> encoder) = 0;
  virtual void ModifyEncoder(
      FunctionView<void(std::unique_ptr<AudioEncoder>*)> modifier) = 0;
  virtual void CallEncoder(FunctionView<void(AudioEncoder*)> modifier) = 0;

  // Use 0 to indicate that the extension should not be registered.
  virtual void SetRTCP_CNAME(absl::string_view c_name) = 0;
  virtual void SetSendAudioLevelIndicationStatus(bool enable, int id) = 0;
  virtual void RegisterSenderCongestionControlObjects(
      RtpTransportControllerSendInterface* transport) = 0;
  virtual void ResetSenderCongestionControlObjects() = 0;
  virtual std::vector<ReportBlockData> GetRemoteRTCPReportBlocks() const = 0;
  virtual ANAStats GetANAStatistics() const = 0;
  virtual void RegisterCngPayloadType(int payload_type,
                                      int payload_frequency) = 0;
  virtual void SetSendTelephoneEventPayloadType(int payload_type,
                                                int payload_frequency) = 0;
  virtual bool SendTelephoneEventOutband(int event, int duration_ms) = 0;
  virtual void OnBitrateAllocation(BitrateAllocationUpdate update) = 0;
  virtual int GetTargetBitrate() const = 0;
  virtual void SetInputMute(bool muted) = 0;

  virtual void ProcessAndEncodeAudio(
      std::unique_ptr<AudioFrame> audio_frame) = 0;
  virtual RtpRtcpInterface* GetRtpRtcp() const = 0;

  virtual void StartSend() = 0;
  virtual void StopSend() = 0;

  // E2EE Custom Audio Frame Encryption (Optional)
  virtual void SetFrameEncryptor(
      scoped_refptr<FrameEncryptorInterface> frame_encryptor) = 0;

  // Sets a frame transformer between encoder and packetizer, to transform
  // encoded frames before sending them out the network.
  virtual void SetEncoderToPacketizerFrameTransformer(
      scoped_refptr<webrtc::FrameTransformerInterface> frame_transformer) = 0;

  // Returns payload bitrate actually used.
  virtual std::optional<DataRate> GetUsedRate() const = 0;

  // Registers per packet byte overhead.
  virtual void RegisterPacketOverhead(int packet_byte_overhead) = 0;
};

std::unique_ptr<ChannelSendInterface> CreateChannelSend(
    const Environment& env,
    Transport* rtp_transport,
    RtcpRttStats* rtcp_rtt_stats,
    FrameEncryptorInterface* frame_encryptor,
    const webrtc::CryptoOptions& crypto_options,
    bool extmap_allow_mixed,
    int rtcp_report_interval_ms,
    uint32_t ssrc,
    scoped_refptr<FrameTransformerInterface> frame_transformer,
    RtpTransportControllerSendInterface* transport_controller);

}  // namespace voe
}  // namespace webrtc

#endif  // AUDIO_CHANNEL_SEND_H_
