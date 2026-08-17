/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_RTP_RTCP_SOURCE_ULPFEC_RECEIVER_H_
#define MODULES_RTP_RTCP_SOURCE_ULPFEC_RECEIVER_H_

#include <stddef.h>
#include <stdint.h>

#include <memory>
#include <vector>

#include "api/sequence_checker.h"
#include "api/units/timestamp.h"
#include "modules/rtp_rtcp/include/recovered_packet_receiver.h"
#include "modules/rtp_rtcp/source/forward_error_correction.h"
#include "modules/rtp_rtcp/source/rtp_packet_received.h"
#include "rtc_base/system/no_unique_address.h"
#include "rtc_base/thread_annotations.h"
#include "system_wrappers/include/clock.h"

namespace webrtc {

struct FecPacketCounter {
  FecPacketCounter() = default;
  size_t num_packets = 0;  // Number of received packets.
  size_t num_bytes = 0;
  size_t num_fec_packets = 0;  // Number of received FEC packets.
  size_t num_recovered_packets =
      0;  // Number of recovered media packets using FEC.
  // Time when first packet is received.
  Timestamp first_packet_time = Timestamp::MinusInfinity();
};

class UlpfecReceiver {
 public:
  UlpfecReceiver(uint32_t ssrc,
                 int ulpfec_payload_type,
                 RecoveredPacketReceiver* callback,
                 Clock* clock);
  ~UlpfecReceiver();

  int ulpfec_payload_type() const { return ulpfec_payload_type_; }

  bool AddReceivedRedPacket(const RtpPacketReceived& rtp_packet);

  void ProcessReceivedFec();

  FecPacketCounter GetPacketCounter() const;

 private:
  const uint32_t ssrc_;
  const int ulpfec_payload_type_;
  Clock* const clock_;

  RTC_NO_UNIQUE_ADDRESS SequenceChecker sequence_checker_;
  RecoveredPacketReceiver* const recovered_packet_callback_;
  const std::unique_ptr<ForwardErrorCorrection> fec_;
  // TODO(nisse): The AddReceivedRedPacket method adds one or two packets to
  // this list at a time, after which it is emptied by ProcessReceivedFec. It
  // will make things simpler to merge AddReceivedRedPacket and
  // ProcessReceivedFec into a single method, and we can then delete this list.
  std::vector<std::unique_ptr<ForwardErrorCorrection::ReceivedPacket>>
      received_packets_ RTC_GUARDED_BY(&sequence_checker_);
  ForwardErrorCorrection::RecoveredPacketList recovered_packets_
      RTC_GUARDED_BY(&sequence_checker_);
  FecPacketCounter packet_counter_ RTC_GUARDED_BY(&sequence_checker_);
};

}  // namespace webrtc

#endif  // MODULES_RTP_RTCP_SOURCE_ULPFEC_RECEIVER_H_
