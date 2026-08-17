/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef CALL_RTP_VIDEO_SENDER_INTERFACE_H_
#define CALL_RTP_VIDEO_SENDER_INTERFACE_H_

#include <cstddef>
#include <cstdint>
#include <map>
#include <vector>

#include "api/array_view.h"
#include "api/call/bitrate_allocation.h"
#include "api/fec_controller_override.h"
#include "api/video/video_layers_allocation.h"
#include "api/video_codecs/video_encoder.h"
#include "call/rtp_config.h"
#include "modules/rtp_rtcp/include/rtp_rtcp_defines.h"
#include "modules/rtp_rtcp/source/rtp_sequence_number_map.h"

namespace webrtc {
class VideoBitrateAllocation;
struct FecProtectionParams;

class RtpVideoSenderInterface : public EncodedImageCallback,
                                public FecControllerOverride {
 public:
  // Sets weather or not RTP packets is allowed to be sent on this sender.
  virtual void SetSending(bool enabled) = 0;
  virtual bool IsActive() = 0;

  virtual void OnNetworkAvailability(bool network_available) = 0;
  virtual std::map<uint32_t, RtpState> GetRtpStates() const = 0;
  virtual std::map<uint32_t, RtpPayloadState> GetRtpPayloadStates() const = 0;

  virtual void DeliverRtcp(const uint8_t* packet, size_t length) = 0;

  virtual void OnBitrateAllocationUpdated(
      const VideoBitrateAllocation& bitrate) = 0;
  virtual void OnVideoLayersAllocationUpdated(
      const VideoLayersAllocation& allocation) = 0;
  virtual void OnBitrateUpdated(BitrateAllocationUpdate update,
                                int framerate) = 0;
  virtual void OnTransportOverheadChanged(
      size_t transport_overhead_bytes_per_packet) = 0;
  virtual uint32_t GetPayloadBitrateBps() const = 0;
  virtual uint32_t GetProtectionBitrateBps() const = 0;
  virtual void SetEncodingData(size_t width,
                               size_t height,
                               size_t num_temporal_layers) = 0;
  virtual std::vector<RtpSequenceNumberMap::Info> GetSentRtpPacketInfos(
      uint32_t ssrc,
      ArrayView<const uint16_t> sequence_numbers) const = 0;

  // Implements FecControllerOverride.
  void SetFecAllowed(bool fec_allowed) override = 0;
};
}  // namespace webrtc
#endif  // CALL_RTP_VIDEO_SENDER_INTERFACE_H_
