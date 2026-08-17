/*
 *  Copyright (c) 2024 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef VIDEO_RATE_UTILIZATION_TRACKER_H_
#define VIDEO_RATE_UTILIZATION_TRACKER_H_

#include <deque>
#include <optional>

#include "api/units/data_rate.h"
#include "api/units/data_size.h"
#include "api/units/time_delta.h"
#include "api/units/timestamp.h"

namespace webrtc {

// Helper class that tracks the rate of utilization over a sliding window.
// tl;dr: if an encoder has a target rate of 1000kbps but in practice
// produces 500kbps it would have a utilization factor of 0.5.
// The tracker looks only at discrete events, and keeps only a fixed amount
// of data points (e.g. encoded frames) or points newer than a given time
// limit, whichever is lower.

// More precisely This class measures the allocated cumulative byte budget (as
// specified by one or more rate updates) and the actual cumulative number of
// bytes produced over a sliding window. A utilization factor (produced bytes /
// budgeted bytes) is calculated seen from the first data point timestamp until
// the last data point timestamp plus the amount time needed to send that last
// data point given no further updates to the rate. The implication of this is a
// smoother value, and e.g. setting a rate and adding a data point, then
// immediately querying the utilization reports 1.0 utilization instead of some
// undefined state.

class RateUtilizationTracker {
 public:
  RateUtilizationTracker(size_t max_num_encoded_data_points,
                         TimeDelta max_duration);

  // The timestamps used should never decrease relative the last one.
  void OnDataRateChanged(DataRate rate, Timestamp time);
  void OnDataProduced(DataSize size, Timestamp time);
  std::optional<double> GetRateUtilizationFactor(Timestamp time) const;

 private:
  struct RateUsageUpdate {
    Timestamp time;
    DataRate target_rate;
    DataSize produced_data;
  };

  void CullOldData(Timestamp time);

  const size_t max_data_points_;
  const TimeDelta max_duration_;
  DataRate current_rate_;
  std::deque<RateUsageUpdate> data_points_;
};

}  // namespace webrtc

#endif  // VIDEO_RATE_UTILIZATION_TRACKER_H_
