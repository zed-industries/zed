/*
 *  Copyright (c) 2020 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_VOIP_VOIP_STATISTICS_H_
#define API_VOIP_VOIP_STATISTICS_H_

#include <cstdint>
#include <optional>

#include "api/neteq/neteq.h"
#include "api/voip/voip_base.h"

namespace webrtc {

struct IngressStatistics {
  // Stats included from api/neteq/neteq.h.
  NetEqLifetimeStatistics neteq_stats;

  // Represents the total duration in seconds of all samples that have been
  // received.
  // https://w3c.github.io/webrtc-stats/#dom-rtcinboundrtpstreamstats-totalsamplesduration
  double total_duration = 0.0;
};

// Remote statistics obtained via remote RTCP SR/RR report received.
struct RemoteRtcpStatistics {
  // Jitter as defined in RFC 3550 [6.4.1] expressed in seconds.
  double jitter = 0.0;

  // Cumulative packets lost as defined in RFC 3550 [6.4.1]
  int64_t packets_lost = 0;

  // Fraction lost as defined in RFC 3550 [6.4.1] expressed as a floating
  // pointer number.
  double fraction_lost = 0.0;

  // https://w3c.github.io/webrtc-stats/#dom-rtcremoteinboundrtpstreamstats-roundtriptime
  std::optional<double> round_trip_time;

  // Last time (not RTP timestamp) when RTCP report received in milliseconds.
  int64_t last_report_received_timestamp_ms;
};

struct ChannelStatistics {
  // https://w3c.github.io/webrtc-stats/#dom-rtcsentrtpstreamstats-packetssent
  uint64_t packets_sent = 0;

  // https://w3c.github.io/webrtc-stats/#dom-rtcsentrtpstreamstats-bytessent
  uint64_t bytes_sent = 0;

  // https://w3c.github.io/webrtc-stats/#dom-rtcreceivedrtpstreamstats-packetsreceived
  uint64_t packets_received = 0;

  // https://w3c.github.io/webrtc-stats/#dom-rtcinboundrtpstreamstats-bytesreceived
  uint64_t bytes_received = 0;

  // https://w3c.github.io/webrtc-stats/#dom-rtcreceivedrtpstreamstats-jitter
  double jitter = 0.0;

  // https://w3c.github.io/webrtc-stats/#dom-rtcreceivedrtpstreamstats-packetslost
  int64_t packets_lost = 0;

  // SSRC from remote media endpoint as indicated either by RTP header in RFC
  // 3550 [5.1] or RTCP SSRC of sender in RFC 3550 [6.4.1].
  std::optional<uint32_t> remote_ssrc;

  std::optional<RemoteRtcpStatistics> remote_rtcp;
};

// VoipStatistics interface provides the interfaces for querying metrics around
// the jitter buffer (NetEq) performance.
class VoipStatistics {
 public:
  // Gets the audio ingress statistics by `ingress_stats` reference.
  // Returns following VoipResult;
  //  kOk - successfully set provided IngressStatistics reference.
  //  kInvalidArgument - `channel_id` is invalid.
  virtual VoipResult GetIngressStatistics(ChannelId channel_id,
                                          IngressStatistics& ingress_stats) = 0;

  // Gets the channel statistics by `channel_stats` reference.
  // Returns following VoipResult;
  //  kOk - successfully set provided ChannelStatistics reference.
  //  kInvalidArgument - `channel_id` is invalid.
  virtual VoipResult GetChannelStatistics(ChannelId channel_id,
                                          ChannelStatistics& channel_stats) = 0;

 protected:
  virtual ~VoipStatistics() = default;
};

}  // namespace webrtc

#endif  // API_VOIP_VOIP_STATISTICS_H_
