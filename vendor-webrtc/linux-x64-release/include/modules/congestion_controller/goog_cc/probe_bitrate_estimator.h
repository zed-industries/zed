/*
 *  Copyright (c) 2016 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_CONGESTION_CONTROLLER_GOOG_CC_PROBE_BITRATE_ESTIMATOR_H_
#define MODULES_CONGESTION_CONTROLLER_GOOG_CC_PROBE_BITRATE_ESTIMATOR_H_

#include <map>
#include <optional>

#include "api/transport/network_types.h"
#include "api/units/data_rate.h"
#include "api/units/data_size.h"
#include "api/units/timestamp.h"

namespace webrtc {
class RtcEventLog;

class ProbeBitrateEstimator {
 public:
  explicit ProbeBitrateEstimator(RtcEventLog* event_log);
  ~ProbeBitrateEstimator();

  // Should be called for every probe packet we receive feedback about.
  // Returns the estimated bitrate if the probe completes a valid cluster.
  std::optional<DataRate> HandleProbeAndEstimateBitrate(
      const PacketResult& packet_feedback);

  std::optional<DataRate> FetchAndResetLastEstimatedBitrate();

 private:
  struct AggregatedCluster {
    int num_probes = 0;
    Timestamp first_send = Timestamp::PlusInfinity();
    Timestamp last_send = Timestamp::MinusInfinity();
    Timestamp first_receive = Timestamp::PlusInfinity();
    Timestamp last_receive = Timestamp::MinusInfinity();
    DataSize size_last_send = DataSize::Zero();
    DataSize size_first_receive = DataSize::Zero();
    DataSize size_total = DataSize::Zero();
  };

  // Erases old cluster data that was seen before `timestamp`.
  void EraseOldClusters(Timestamp timestamp);

  std::map<int, AggregatedCluster> clusters_;
  RtcEventLog* const event_log_;
  std::optional<DataRate> estimated_data_rate_;
};

}  // namespace webrtc

#endif  // MODULES_CONGESTION_CONTROLLER_GOOG_CC_PROBE_BITRATE_ESTIMATOR_H_
