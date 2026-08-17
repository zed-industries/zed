/*
 *  Copyright (c) 2015 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_REMOTE_BITRATE_ESTIMATOR_TRANSPORT_SEQUENCE_NUMBER_FEEDBACK_GENERATOR_H_
#define MODULES_REMOTE_BITRATE_ESTIMATOR_TRANSPORT_SEQUENCE_NUMBER_FEEDBACK_GENERATOR_H_

#include <cstdint>
#include <memory>
#include <optional>

#include "api/rtp_headers.h"
#include "api/units/data_rate.h"
#include "api/units/time_delta.h"
#include "api/units/timestamp.h"
#include "modules/remote_bitrate_estimator/packet_arrival_map.h"
#include "modules/remote_bitrate_estimator/rtp_transport_feedback_generator.h"
#include "modules/rtp_rtcp/source/rtcp_packet/transport_feedback.h"
#include "modules/rtp_rtcp/source/rtp_packet_received.h"
#include "rtc_base/numerics/sequence_number_unwrapper.h"
#include "rtc_base/synchronization/mutex.h"
#include "rtc_base/thread_annotations.h"

namespace webrtc {

// Class used when send-side BWE is enabled.
// The class is responsible for generating RTCP feedback packets based on
// incoming media packets. Incoming packets must have a transport sequence
// number, Ie. either the extension
// http://www.ietf.org/id/draft-holmer-rmcat-transport-wide-cc-extensions-01 or
// http://www.webrtc.org/experiments/rtp-hdrext/transport-wide-cc-02 must be
// used.
class TransportSequenceNumberFeedbackGenenerator
    : public RtpTransportFeedbackGenerator {
 public:
  TransportSequenceNumberFeedbackGenenerator(
      RtpTransportFeedbackGenerator::RtcpSender feedback_sender);
  ~TransportSequenceNumberFeedbackGenenerator();

  void OnReceivedPacket(const RtpPacketReceived& packet) override;
  void OnSendBandwidthEstimateChanged(DataRate estimate) override;

  TimeDelta Process(Timestamp now) override;

 private:
  void MaybeCullOldPackets(int64_t sequence_number, Timestamp arrival_time)
      RTC_EXCLUSIVE_LOCKS_REQUIRED(&lock_);
  void SendPeriodicFeedbacks() RTC_EXCLUSIVE_LOCKS_REQUIRED(&lock_);
  void SendFeedbackOnRequest(int64_t sequence_number,
                             const FeedbackRequest& feedback_request)
      RTC_EXCLUSIVE_LOCKS_REQUIRED(&lock_);

  // Returns a Transport Feedback packet with information about as many
  // packets that has been received between [`begin_sequence_number_incl`,
  // `end_sequence_number_excl`) that can fit in it. If `is_periodic_update`,
  // this represents sending a periodic feedback message, which will make it
  // update the `periodic_window_start_seq_` variable with the first packet
  // that was not included in the feedback packet, so that the next update can
  // continue from that sequence number.
  //
  // If no incoming packets were added, nullptr is returned.
  //
  // `include_timestamps` decide if the returned TransportFeedback should
  // include timestamps.
  std::unique_ptr<rtcp::TransportFeedback> MaybeBuildFeedbackPacket(
      bool include_timestamps,
      int64_t begin_sequence_number_inclusive,
      int64_t end_sequence_number_exclusive,
      bool is_periodic_update) RTC_EXCLUSIVE_LOCKS_REQUIRED(&lock_);

  const RtcpSender feedback_sender_;
  Timestamp last_process_time_;

  Mutex lock_;
  uint32_t media_ssrc_ RTC_GUARDED_BY(&lock_);
  uint8_t feedback_packet_count_ RTC_GUARDED_BY(&lock_);
  SeqNumUnwrapper<uint16_t> unwrapper_ RTC_GUARDED_BY(&lock_);

  // The next sequence number that should be the start sequence number during
  // periodic reporting. Will be std::nullopt before the first seen packet.
  std::optional<int64_t> periodic_window_start_seq_ RTC_GUARDED_BY(&lock_);

  // Packet arrival times, by sequence number.
  PacketArrivalTimeMap packet_arrival_times_ RTC_GUARDED_BY(&lock_);

  TimeDelta send_interval_ RTC_GUARDED_BY(&lock_);
  bool send_periodic_feedback_ RTC_GUARDED_BY(&lock_);
};

}  // namespace webrtc

#endif  // MODULES_REMOTE_BITRATE_ESTIMATOR_TRANSPORT_SEQUENCE_NUMBER_FEEDBACK_GENERATOR_H_
