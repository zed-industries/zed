// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_PROFILER_PERIODIC_SAMPLING_SCHEDULER_H_
#define BASE_PROFILER_PERIODIC_SAMPLING_SCHEDULER_H_

#include "base/time/time.h"

namespace base {

// The scheduler works by splitting execution time into repeated periods such
// that the time to take one collection represents
// |fraction_of_execution_time_to_sample| of the period, and the time not spent
// sampling represents 1 - |fraction_of_execution_time_to_sample| of the period.
// The collection start time is chosen randomly within each period such that the
// entire collection is contained within the period.
// It repeatedly schedules periodic sampling of the
// thread through calls to GetTimeToNextCollection().
class BASE_EXPORT PeriodicSamplingScheduler {
 public:
  PeriodicSamplingScheduler(TimeDelta sampling_duration,
                            double fraction_of_execution_time_to_sample,
                            TimeTicks start_time);

  PeriodicSamplingScheduler(const PeriodicSamplingScheduler&) = delete;
  PeriodicSamplingScheduler& operator=(const PeriodicSamplingScheduler&) =
      delete;

  virtual ~PeriodicSamplingScheduler();

  // Returns the amount of time between now and the next collection.
  // Virtual to provide dependency injection for test use.
  virtual TimeDelta GetTimeToNextCollection();

 protected:
  // Virtual to provide seams for test use.
  virtual double RandDouble() const;
  virtual TimeTicks Now() const;

 private:
  const TimeDelta period_duration_;
  const TimeDelta sampling_duration_;
  TimeTicks period_start_time_;
};

}  // namespace base

#endif  // BASE_PROFILER_PERIODIC_SAMPLING_SCHEDULER_H_
