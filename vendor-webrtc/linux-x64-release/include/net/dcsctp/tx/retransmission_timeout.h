/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef NET_DCSCTP_TX_RETRANSMISSION_TIMEOUT_H_
#define NET_DCSCTP_TX_RETRANSMISSION_TIMEOUT_H_

#include <cstdint>
#include <functional>

#include "net/dcsctp/public/dcsctp_options.h"

namespace dcsctp {

// Manages updating of the Retransmission Timeout (RTO) SCTP variable, which is
// used directly as the base timeout for T3-RTX and for other timers, such as
// delayed ack.
//
// When a round-trip-time (RTT) is calculated (outside this class), `Observe`
// is called, which calculates the retransmission timeout (RTO) value. The RTO
// value will become larger if the RTT is high and/or the RTT values are varying
// a lot, which is an indicator of a bad connection.
class RetransmissionTimeout {
 public:
  explicit RetransmissionTimeout(const DcSctpOptions& options);

  // To be called when a RTT has been measured, to update the RTO value.
  void ObserveRTT(webrtc::TimeDelta rtt);

  // Returns the Retransmission Timeout (RTO) value.
  webrtc::TimeDelta rto() const { return rto_; }

  // Returns the smoothed RTT value.
  webrtc::TimeDelta srtt() const { return srtt_; }

 private:
  const webrtc::TimeDelta min_rto_;
  const webrtc::TimeDelta max_rto_;
  const webrtc::TimeDelta max_rtt_;
  const webrtc::TimeDelta min_rtt_variance_;
  // If this is the first measurement
  bool first_measurement_ = true;
  // Smoothed Round-Trip Time.
  webrtc::TimeDelta srtt_;
  // Round-Trip Time Variation.
  webrtc::TimeDelta rtt_var_ = webrtc::TimeDelta::Zero();
  // Retransmission Timeout
  webrtc::TimeDelta rto_;
};
}  // namespace dcsctp

#endif  // NET_DCSCTP_TX_RETRANSMISSION_TIMEOUT_H_
