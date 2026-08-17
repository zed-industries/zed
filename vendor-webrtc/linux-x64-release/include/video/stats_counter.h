/*
 *  Copyright (c) 2016 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef VIDEO_STATS_COUNTER_H_
#define VIDEO_STATS_COUNTER_H_

#include <stdint.h>

#include <memory>
#include <string>

namespace webrtc {

class AggregatedCounter;
class Clock;
class Samples;

// `StatsCounterObserver` is called periodically when a metric is updated.
class StatsCounterObserver {
 public:
  virtual void OnMetricUpdated(int sample) = 0;

  virtual ~StatsCounterObserver() {}
};

struct AggregatedStats {
  std::string ToString() const;
  std::string ToStringWithMultiplier(int multiplier) const;

  int64_t num_samples = 0;
  int min = -1;
  int max = -1;
  int average = -1;
  // TODO(asapersson): Consider adding median/percentiles.
};

// Classes which periodically computes a metric.
//
// During a period, `kProcessIntervalMs`, different metrics can be computed e.g:
// - `AvgCounter`: average of samples
// - `PercentCounter`: percentage of samples
// - `PermilleCounter`: permille of samples
//
// Each periodic metric can be either:
// - reported to an `observer` each period
// - aggregated during the call (e.g. min, max, average)
//
//                 periodically computed
//                    GetMetric()            GetMetric()   => AggregatedStats
//                        ^                      ^            (e.g. min/max/avg)
//                        |                      |
// |   *    *  *       *  |  **    *   * *     * | ...
// |<- process interval ->|
//
// (*) - samples
//
//
// Example usage:
//
// AvgCounter counter(&clock, nullptr);
// counter.Add(5);
// counter.Add(1);
// counter.Add(6);   // process interval passed -> GetMetric() avg:4
// counter.Add(7);
// counter.Add(3);   // process interval passed -> GetMetric() avg:5
// counter.Add(10);
// counter.Add(20);  // process interval passed -> GetMetric() avg:15
// AggregatedStats stats = counter.GetStats();
// stats: {min:4, max:15, avg:8}
//

// Note: StatsCounter takes ownership of `observer`.

class StatsCounter {
 public:
  virtual ~StatsCounter();

  // Gets metric within an interval. Returns true on success false otherwise.
  virtual bool GetMetric(int* metric) const = 0;

  // Gets the value to use for an interval without samples.
  virtual int GetValueForEmptyInterval() const = 0;

  // Gets aggregated stats (i.e. aggregate of periodically computed metrics).
  AggregatedStats GetStats();

  // Reports metrics for elapsed intervals to AggregatedCounter and GetStats.
  AggregatedStats ProcessAndGetStats();

  // Reports metrics for elapsed intervals to AggregatedCounter and pauses stats
  // (i.e. empty intervals will be discarded until next sample is added).
  void ProcessAndPause();

  // As above with a minimum pause time. Added samples within this interval will
  // not resume the stats (i.e. stop the pause).
  void ProcessAndPauseForDuration(int64_t min_pause_time_ms);

  // Reports metrics for elapsed intervals to AggregatedCounter and stops pause.
  void ProcessAndStopPause();

  // Checks if a sample has been added (i.e. Add or Set called).
  bool HasSample() const;

 protected:
  StatsCounter(Clock* clock,
               int64_t process_intervals_ms,
               bool include_empty_intervals,
               StatsCounterObserver* observer);

  void Add(int sample);
  void Set(int64_t sample, uint32_t stream_id);
  void SetLast(int64_t sample, uint32_t stream_id);

  const bool include_empty_intervals_;
  const int64_t process_intervals_ms_;
  const std::unique_ptr<AggregatedCounter> aggregated_counter_;
  const std::unique_ptr<Samples> samples_;

 private:
  bool TimeToProcess(int* num_elapsed_intervals);
  void TryProcess();
  void ReportMetricToAggregatedCounter(int value, int num_values_to_add) const;
  bool IncludeEmptyIntervals() const;
  void Resume();
  void ResumeIfMinTimePassed();

  Clock* const clock_;
  const std::unique_ptr<StatsCounterObserver> observer_;
  int64_t last_process_time_ms_;
  bool paused_;
  int64_t pause_time_ms_;
  int64_t min_pause_time_ms_;
};

// AvgCounter: average of samples
//
//           | *      *      *      | *           *       | ...
//           | Add(5) Add(1) Add(6) | Add(5)      Add(5)  |
// GetMetric | (5 + 1 + 6) / 3      | (5 + 5) / 2         |
//
// `include_empty_intervals`: If set, intervals without samples will be included
//                            in the stats. The value for an interval is
//                            determined by GetValueForEmptyInterval().
//
class AvgCounter : public StatsCounter {
 public:
  AvgCounter(Clock* clock,
             StatsCounterObserver* observer,
             bool include_empty_intervals);
  ~AvgCounter() override {}

  AvgCounter(const AvgCounter&) = delete;
  AvgCounter& operator=(const AvgCounter&) = delete;

  void Add(int sample);

 private:
  bool GetMetric(int* metric) const override;

  // Returns the last computed metric (i.e. from GetMetric).
  int GetValueForEmptyInterval() const override;
};

// MaxCounter: maximum of samples
//
//           | *      *      *      | *           *       | ...
//           | Add(5) Add(1) Add(6) | Add(5)      Add(5)  |
// GetMetric | max: (5, 1, 6)       | max: (5, 5)         |
//
class MaxCounter : public StatsCounter {
 public:
  MaxCounter(Clock* clock,
             StatsCounterObserver* observer,
             int64_t process_intervals_ms);
  ~MaxCounter() override {}

  MaxCounter(const MaxCounter&) = delete;
  MaxCounter& operator=(const MaxCounter&) = delete;

  void Add(int sample);

 private:
  bool GetMetric(int* metric) const override;
  int GetValueForEmptyInterval() const override;
};

// PercentCounter: percentage of samples
//
//           | *      *      *      | *           *       | ...
//           | Add(T) Add(F) Add(T) | Add(F)      Add(T)  |
// GetMetric | 100 * 2 / 3          | 100 * 1 / 2         |
//
class PercentCounter : public StatsCounter {
 public:
  PercentCounter(Clock* clock, StatsCounterObserver* observer);
  ~PercentCounter() override {}

  PercentCounter(const PercentCounter&) = delete;
  PercentCounter& operator=(const PercentCounter&) = delete;

  void Add(bool sample);

 private:
  bool GetMetric(int* metric) const override;
  int GetValueForEmptyInterval() const override;
};

// PermilleCounter: permille of samples
//
//           | *      *      *      | *         *         | ...
//           | Add(T) Add(F) Add(T) | Add(F)    Add(T)    |
// GetMetric | 1000 *  2 / 3        | 1000 * 1 / 2        |
//
class PermilleCounter : public StatsCounter {
 public:
  PermilleCounter(Clock* clock, StatsCounterObserver* observer);
  ~PermilleCounter() override {}

  PermilleCounter(const PermilleCounter&) = delete;
  PermilleCounter& operator=(const PermilleCounter&) = delete;

  void Add(bool sample);

 private:
  bool GetMetric(int* metric) const override;
  int GetValueForEmptyInterval() const override;
};

// RateCounter: units per second
//
//           | *      *      *      | *           *       | ...
//           | Add(5) Add(1) Add(6) | Add(5)      Add(5)  |
//           |<------ 2 sec ------->|                     |
// GetMetric | (5 + 1 + 6) / 2      | (5 + 5) / 2         |
//
// `include_empty_intervals`: If set, intervals without samples will be included
//                            in the stats. The value for an interval is
//                            determined by GetValueForEmptyInterval().
//
class RateCounter : public StatsCounter {
 public:
  RateCounter(Clock* clock,
              StatsCounterObserver* observer,
              bool include_empty_intervals);
  ~RateCounter() override {}

  RateCounter(const RateCounter&) = delete;
  RateCounter& operator=(const RateCounter&) = delete;

  void Add(int sample);

 private:
  bool GetMetric(int* metric) const override;
  int GetValueForEmptyInterval() const override;  // Returns zero.
};

// RateAccCounter: units per second (used for counters)
//
//           | *      *      *      | *         *         | ...
//           | Set(5) Set(6) Set(8) | Set(11)   Set(13)   |
//           |<------ 2 sec ------->|                     |
// GetMetric | (8 - 0) / 2          | (13 - 8) / 2        |
//
// `include_empty_intervals`: If set, intervals without samples will be included
//                            in the stats. The value for an interval is
//                            determined by GetValueForEmptyInterval().
//
class RateAccCounter : public StatsCounter {
 public:
  RateAccCounter(Clock* clock,
                 StatsCounterObserver* observer,
                 bool include_empty_intervals);
  ~RateAccCounter() override {}

  RateAccCounter(const RateAccCounter&) = delete;
  RateAccCounter& operator=(const RateAccCounter&) = delete;

  void Set(int64_t sample, uint32_t stream_id);

  // Sets the value for previous interval.
  // To be used if a value other than zero is initially required.
  void SetLast(int64_t sample, uint32_t stream_id);

 private:
  bool GetMetric(int* metric) const override;
  int GetValueForEmptyInterval() const override;  // Returns zero.
};

}  // namespace webrtc

#endif  // VIDEO_STATS_COUNTER_H_
