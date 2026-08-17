/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef MODULES_CONGESTION_CONTROLLER_GOOG_CC_DELAY_INCREASE_DETECTOR_INTERFACE_H_
#define MODULES_CONGESTION_CONTROLLER_GOOG_CC_DELAY_INCREASE_DETECTOR_INTERFACE_H_

#include <stdint.h>

#include <cstddef>

#include "api/transport/bandwidth_usage.h"

namespace webrtc {

class DelayIncreaseDetectorInterface {
 public:
  DelayIncreaseDetectorInterface() {}
  virtual ~DelayIncreaseDetectorInterface() {}

  DelayIncreaseDetectorInterface(const DelayIncreaseDetectorInterface&) =
      delete;
  DelayIncreaseDetectorInterface& operator=(
      const DelayIncreaseDetectorInterface&) = delete;

  // Update the detector with a new sample. The deltas should represent deltas
  // between timestamp groups as defined by the InterArrival class.
  virtual void Update(double recv_delta_ms,
                      double send_delta_ms,
                      int64_t send_time_ms,
                      int64_t arrival_time_ms,
                      size_t packet_size,
                      bool calculated_deltas) = 0;

  virtual BandwidthUsage State() const = 0;
};

}  // namespace webrtc

#endif  // MODULES_CONGESTION_CONTROLLER_GOOG_CC_DELAY_INCREASE_DETECTOR_INTERFACE_H_
