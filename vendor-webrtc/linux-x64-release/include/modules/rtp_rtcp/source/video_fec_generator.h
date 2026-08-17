/*
 *  Copyright (c) 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_RTP_RTCP_SOURCE_VIDEO_FEC_GENERATOR_H_
#define MODULES_RTP_RTCP_SOURCE_VIDEO_FEC_GENERATOR_H_

#include <cstddef>
#include <cstdint>
#include <memory>
#include <optional>
#include <vector>

#include "api/units/data_rate.h"
#include "modules/include/module_fec_types.h"
#include "modules/rtp_rtcp/include/rtp_rtcp_defines.h"
#include "modules/rtp_rtcp/source/rtp_packet_to_send.h"

namespace webrtc {

class VideoFecGenerator {
 public:
  VideoFecGenerator() = default;
  virtual ~VideoFecGenerator() = default;

  enum class FecType { kFlexFec, kUlpFec };
  virtual FecType GetFecType() const = 0;
  // Returns the SSRC used for FEC packets (i.e. FlexFec SSRC).
  virtual std::optional<uint32_t> FecSsrc() = 0;
  // Returns the overhead, in bytes per packet, for FEC (and possibly RED).
  virtual size_t MaxPacketOverhead() const = 0;
  // Current rate of FEC packets generated, including all RTP-level headers.
  virtual DataRate CurrentFecRate() const = 0;
  // Set FEC rates, max frames before FEC is sent, and type of FEC masks.
  virtual void SetProtectionParameters(
      const FecProtectionParams& delta_params,
      const FecProtectionParams& key_params) = 0;
  // Called on new media packet to be protected. The generator may choose
  // to generate FEC packets at this time, if so they will be stored in an
  // internal buffer.
  virtual void AddPacketAndGenerateFec(const RtpPacketToSend& packet) = 0;
  // Get (and remove) and FEC packets pending in the generator. These packets
  // will lack sequence numbers, that needs to be set externally.
  // TODO(bugs.webrtc.org/11340): Actually FlexFec sets seq#, fix that!
  virtual std::vector<std::unique_ptr<RtpPacketToSend>> GetFecPackets() = 0;
  // Only called on the VideoSendStream queue, after operation has shut down,
  // and only populated if there is an RtpState (e.g. FlexFec).
  virtual std::optional<RtpState> GetRtpState() = 0;
};

}  // namespace webrtc
#endif  // MODULES_RTP_RTCP_SOURCE_VIDEO_FEC_GENERATOR_H_
