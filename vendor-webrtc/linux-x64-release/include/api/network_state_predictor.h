/*
 *  Copyright (c) 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_NETWORK_STATE_PREDICTOR_H_
#define API_NETWORK_STATE_PREDICTOR_H_

#include <cstdint>
#include <memory>

#include "api/transport/bandwidth_usage.h"

namespace webrtc {

// TODO(yinwa): work in progress. API in class NetworkStatePredictor should not
// be used by other users until this comment is removed.

// NetworkStatePredictor predict network state based on current network metrics.
// Usage:
// Setup by calling Initialize.
// For each update, call Update. Update returns network state
// prediction.
class NetworkStatePredictor {
 public:
  virtual ~NetworkStatePredictor() {}

  // Returns current network state prediction.
  // Inputs:  send_time_ms - packet send time.
  //          arrival_time_ms - packet arrival time.
  //          network_state - computed network state.
  virtual BandwidthUsage Update(int64_t send_time_ms,
                                int64_t arrival_time_ms,
                                BandwidthUsage network_state) = 0;
};

class NetworkStatePredictorFactoryInterface {
 public:
  virtual std::unique_ptr<NetworkStatePredictor>
  CreateNetworkStatePredictor() = 0;
  virtual ~NetworkStatePredictorFactoryInterface() = default;
};

}  // namespace webrtc

#endif  // API_NETWORK_STATE_PREDICTOR_H_
