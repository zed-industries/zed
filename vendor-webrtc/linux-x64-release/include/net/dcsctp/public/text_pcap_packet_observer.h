/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef NET_DCSCTP_PUBLIC_TEXT_PCAP_PACKET_OBSERVER_H_
#define NET_DCSCTP_PUBLIC_TEXT_PCAP_PACKET_OBSERVER_H_

#include <string>

#include "absl/strings/string_view.h"
#include "api/array_view.h"
#include "net/dcsctp/public/packet_observer.h"
#include "net/dcsctp/public/types.h"

namespace dcsctp {

// Print outs all sent and received packets to the logs, at LS_VERBOSE severity.
class TextPcapPacketObserver : public dcsctp::PacketObserver {
 public:
  explicit TextPcapPacketObserver(absl::string_view name) : name_(name) {}

  // Implementation of `dcsctp::PacketObserver`.
  void OnSentPacket(dcsctp::TimeMs now,
                    webrtc::ArrayView<const uint8_t> payload) override;

  void OnReceivedPacket(dcsctp::TimeMs now,
                        webrtc::ArrayView<const uint8_t> payload) override;

  // Prints a packet to the log. Exposed to allow it to be used in compatibility
  // tests suites that don't use PacketObserver.
  static void PrintPacket(absl::string_view prefix,
                          absl::string_view socket_name,
                          dcsctp::TimeMs now,
                          webrtc::ArrayView<const uint8_t> payload);

 private:
  const std::string name_;
};

}  // namespace dcsctp
#endif  // NET_DCSCTP_PUBLIC_TEXT_PCAP_PACKET_OBSERVER_H_
