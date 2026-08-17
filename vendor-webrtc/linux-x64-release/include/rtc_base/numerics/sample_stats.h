/*
 *  Copyright (c) 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef RTC_BASE_NUMERICS_SAMPLE_STATS_H_
#define RTC_BASE_NUMERICS_SAMPLE_STATS_H_

#include "api/numerics/samples_stats_counter.h"
#include "api/units/data_rate.h"
#include "api/units/time_delta.h"

namespace webrtc {
template <typename T>
class SampleStats;

// TODO(srte): Merge this implementation with SamplesStatsCounter.
template <>
class SampleStats<double> : public SamplesStatsCounter {
 public:
  double Max();
  double Mean();
  double Median();
  double Quantile(double quantile);
  double Min();
  double Variance();
  double StandardDeviation();
  int Count();
};

template <>
class SampleStats<TimeDelta> {
 public:
  void AddSample(TimeDelta delta);
  void AddSampleMs(double delta_ms);
  void AddSamples(const SampleStats<TimeDelta>& other);
  bool IsEmpty();
  TimeDelta Max();
  TimeDelta Mean();
  TimeDelta Median();
  TimeDelta Quantile(double quantile);
  TimeDelta Min();
  TimeDelta Variance();
  TimeDelta StandardDeviation();
  int Count();

 private:
  SampleStats<double> stats_;
};

template <>
class SampleStats<DataRate> {
 public:
  void AddSample(DataRate rate);
  void AddSampleBps(double rate_bps);
  void AddSamples(const SampleStats<DataRate>& other);
  bool IsEmpty();
  DataRate Max();
  DataRate Mean();
  DataRate Median();
  DataRate Quantile(double quantile);
  DataRate Min();
  DataRate Variance();
  DataRate StandardDeviation();
  int Count();

 private:
  SampleStats<double> stats_;
};
}  // namespace webrtc

#endif  // RTC_BASE_NUMERICS_SAMPLE_STATS_H_
