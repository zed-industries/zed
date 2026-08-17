/*
 *  Copyright (c) 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_CONGESTION_CONTROLLER_GOOG_CC_ACKNOWLEDGED_BITRATE_ESTIMATOR_INTERFACE_H_
#define MODULES_CONGESTION_CONTROLLER_GOOG_CC_ACKNOWLEDGED_BITRATE_ESTIMATOR_INTERFACE_H_

#include <memory>
#include <optional>
#include <vector>

#include "api/field_trials_view.h"
#include "api/transport/network_types.h"
#include "api/units/data_rate.h"
#include "api/units/time_delta.h"
#include "api/units/timestamp.h"
#include "rtc_base/experiments/struct_parameters_parser.h"

namespace webrtc {

struct RobustThroughputEstimatorSettings {
  static constexpr char kKey[] = "WebRTC-Bwe-RobustThroughputEstimatorSettings";

  RobustThroughputEstimatorSettings() = delete;
  explicit RobustThroughputEstimatorSettings(
      const FieldTrialsView* key_value_config);

  // Set `enabled` to true to use the RobustThroughputEstimator, false to use
  // the AcknowledgedBitrateEstimator.
  bool enabled = true;

  // The estimator keeps the smallest window containing at least
  // `window_packets` and at least the packets received during the last
  // `min_window_duration` milliseconds.
  // (This means that it may store more than `window_packets` at high bitrates,
  // and a longer duration than `min_window_duration` at low bitrates.)
  // However, if will never store more than kMaxPackets (for performance
  // reasons), and never longer than max_window_duration (to avoid very old
  // packets influencing the estimate for example when sending is paused).
  unsigned window_packets = 20;
  unsigned max_window_packets = 500;
  TimeDelta min_window_duration = TimeDelta::Seconds(1);
  TimeDelta max_window_duration = TimeDelta::Seconds(5);

  // The estimator window requires at least `required_packets` packets
  // to produce an estimate.
  unsigned required_packets = 10;

  // If audio packets aren't included in allocation (i.e. the
  // estimated available bandwidth is divided only among the video
  // streams), then `unacked_weight` should be set to 0.
  // If audio packets are included in allocation, but not in bandwidth
  // estimation (i.e. they don't have transport-wide sequence numbers,
  // but we nevertheless divide the estimated available bandwidth among
  // both audio and video streams), then `unacked_weight` should be set to 1.
  // If all packets have transport-wide sequence numbers, then the value
  // of `unacked_weight` doesn't matter.
  double unacked_weight = 1.0;

  std::unique_ptr<StructParametersParser> Parser();
};

class AcknowledgedBitrateEstimatorInterface {
 public:
  static std::unique_ptr<AcknowledgedBitrateEstimatorInterface> Create(
      const FieldTrialsView* key_value_config);
  virtual ~AcknowledgedBitrateEstimatorInterface();

  virtual void IncomingPacketFeedbackVector(
      const std::vector<PacketResult>& packet_feedback_vector) = 0;
  virtual std::optional<DataRate> bitrate() const = 0;
  virtual std::optional<DataRate> PeekRate() const = 0;
  virtual void SetAlr(bool in_alr) = 0;
  virtual void SetAlrEndedTime(Timestamp alr_ended_time) = 0;
};

}  // namespace webrtc

#endif  // MODULES_CONGESTION_CONTROLLER_GOOG_CC_ACKNOWLEDGED_BITRATE_ESTIMATOR_INTERFACE_H_
