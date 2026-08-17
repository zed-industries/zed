/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_CONGESTION_CONTROLLER_PCC_MONITOR_INTERVAL_H_
#define MODULES_CONGESTION_CONTROLLER_PCC_MONITOR_INTERVAL_H_

#include <vector>

#include "api/transport/network_types.h"
#include "api/units/data_rate.h"
#include "api/units/data_size.h"
#include "api/units/time_delta.h"
#include "api/units/timestamp.h"

namespace webrtc {
namespace pcc {

// PCC divides time into consecutive monitor intervals which are used to test
// consequences for performance of sending at a certain rate.
class PccMonitorInterval {
 public:
  PccMonitorInterval(DataRate target_sending_rate,
                     Timestamp start_time,
                     TimeDelta duration);
  ~PccMonitorInterval();
  PccMonitorInterval(const PccMonitorInterval& other);
  void OnPacketsFeedback(const std::vector<PacketResult>& packets_results);
  // Returns true if got complete information about packets.
  // Notice, this only happens when received feedback about the first packet
  // which were sent after the end of the monitor interval. If such event
  // doesn't occur, we don't mind anyway and stay in the same state.
  bool IsFeedbackCollectionDone() const;
  Timestamp GetEndTime() const;

  double GetLossRate() const;
  // Estimates the gradient using linear regression on the 2-dimensional
  // dataset (sampled packets delay, time of sampling).
  double ComputeDelayGradient(double delay_gradient_threshold) const;
  DataRate GetTargetSendingRate() const;
  // How fast receiving side gets packets.
  DataRate GetTransmittedPacketsRate() const;

 private:
  struct ReceivedPacket {
    TimeDelta delay;
    Timestamp sent_time;
  };
  // Target bitrate used to generate and pace the outgoing packets.
  // Actually sent bitrate might not match the target exactly.
  DataRate target_sending_rate_;
  // Start time is not included into interval while end time is included.
  Timestamp start_time_;
  TimeDelta interval_duration_;
  // Vectors below updates while receiving feedback.
  std::vector<ReceivedPacket> received_packets_;
  std::vector<Timestamp> lost_packets_sent_time_;
  DataSize received_packets_size_;
  bool feedback_collection_done_;
};

}  // namespace pcc
}  // namespace webrtc

#endif  // MODULES_CONGESTION_CONTROLLER_PCC_MONITOR_INTERVAL_H_
