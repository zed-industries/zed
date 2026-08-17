/*
 *  Copyright 2018 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef PC_TRANSPORT_STATS_H_
#define PC_TRANSPORT_STATS_H_

#include <cstdint>
#include <optional>
#include <string>
#include <vector>

#include "absl/strings/string_view.h"
#include "api/dtls_transport_interface.h"
#include "p2p/base/ice_transport_internal.h"
#include "rtc_base/ssl_stream_adapter.h"

namespace webrtc {

struct TransportChannelStats {
  TransportChannelStats();
  TransportChannelStats(const TransportChannelStats&);
  ~TransportChannelStats();

  int component = 0;
  int ssl_version_bytes = 0;
  int srtp_crypto_suite = webrtc::kSrtpInvalidCryptoSuite;
  int ssl_cipher_suite = webrtc::kTlsNullWithNullNull;
  std::optional<absl::string_view> tls_cipher_suite_name;
  std::optional<SSLRole> dtls_role;
  DtlsTransportState dtls_state = DtlsTransportState::kNew;
  IceTransportStats ice_transport_stats;
  uint16_t ssl_peer_signature_algorithm = webrtc::kSslSignatureAlgorithmUnknown;
};

// Information about all the channels of a transport.
// TODO(hta): Consider if a simple vector is as good as a map.
typedef std::vector<TransportChannelStats> TransportChannelStatsList;

// Information about the stats of a transport.
struct TransportStats {
  std::string transport_name;
  TransportChannelStatsList channel_stats;
};

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace cricket {
using ::webrtc::TransportChannelStats;
using ::webrtc::TransportChannelStatsList;
using ::webrtc::TransportStats;
}  // namespace cricket
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // PC_TRANSPORT_STATS_H_
