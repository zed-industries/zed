/*
 *  Copyright 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef MODULES_CONGESTION_CONTROLLER_GOOG_CC_LINK_CAPACITY_ESTIMATOR_H_
#define MODULES_CONGESTION_CONTROLLER_GOOG_CC_LINK_CAPACITY_ESTIMATOR_H_

#include <optional>

#include "api/units/data_rate.h"

namespace webrtc {
class LinkCapacityEstimator {
 public:
  LinkCapacityEstimator();
  DataRate UpperBound() const;
  DataRate LowerBound() const;
  void Reset();
  void OnOveruseDetected(DataRate acknowledged_rate);
  void OnProbeRate(DataRate probe_rate);
  bool has_estimate() const;
  DataRate estimate() const;

 private:
  friend class GoogCcStatePrinter;
  void Update(DataRate capacity_sample, double alpha);

  double deviation_estimate_kbps() const;
  std::optional<double> estimate_kbps_;
  double deviation_kbps_ = 0.4;
};
}  // namespace webrtc

#endif  // MODULES_CONGESTION_CONTROLLER_GOOG_CC_LINK_CAPACITY_ESTIMATOR_H_
