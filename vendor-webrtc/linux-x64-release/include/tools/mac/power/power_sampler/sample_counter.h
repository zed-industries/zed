// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_MAC_POWER_POWER_SAMPLER_SAMPLE_COUNTER_H_
#define TOOLS_MAC_POWER_POWER_SAMPLER_SAMPLE_COUNTER_H_

#include <cstdio>

#include "tools/mac/power/power_sampler/monitor.h"

namespace power_sampler {

// Counts Monitor notifications and causes exit after |max_sample_count|.
class SampleCounter : public Monitor {
 public:
  SampleCounter(size_t max_sample_count);
  ~SampleCounter() override = default;

  void OnStartSession(const DataColumnKeyUnits& data_columns_units) override {}
  bool OnSample(base::TimeTicks sample_time, const DataRow& data_row) override;
  void OnEndSession() override {}

 private:
  size_t sample_count_;
};

}  // namespace power_sampler

#endif  // TOOLS_MAC_POWER_POWER_SAMPLER_SAMPLE_COUNTER_H_
