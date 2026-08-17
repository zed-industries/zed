// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_MEMORY_SIMULATOR_SIMULATOR_METRICS_PROVIDER_H_
#define TOOLS_MEMORY_SIMULATOR_SIMULATOR_METRICS_PROVIDER_H_

#include <stdint.h>
#include <cstdint>

#include "base/memory/raw_ptr.h"
#include "base/time/time.h"
#include "tools/memory/simulator/metrics_provider.h"

namespace memory_simulator {

class MemorySimulator;

class SimulatorMetricsProvider : public MetricsProvider {
 public:
  explicit SimulatorMetricsProvider(MemorySimulator* simulator);
  ~SimulatorMetricsProvider() override;

  // MetricsProvider:
  std::vector<std::string> GetMetricNames() override;
  std::map<std::string, double> GetMetricValues(base::TimeTicks now) override;

 private:
  raw_ptr<MemorySimulator> simulator_;

  base::TimeTicks prev_time_;
  int64_t prev_pages_allocated_ = 0;
  int64_t prev_pages_read_ = 0;
  int64_t prev_pages_written_ = 0;
};

}  // namespace memory_simulator

#endif  // TOOLS_MEMORY_SIMULATOR_SIMULATOR_METRICS_PROVIDER_H_
