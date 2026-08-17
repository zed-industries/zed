// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_MEMORY_SIMULATOR_PROCESS_METRICS_PROVIDER_MAC_H_
#define TOOLS_MEMORY_SIMULATOR_PROCESS_METRICS_PROVIDER_MAC_H_

#include "tools/memory/simulator/metrics_provider.h"

namespace memory_simulator {

class ProcessMetricsProviderMac : public MetricsProvider {
 public:
  ProcessMetricsProviderMac();
  ~ProcessMetricsProviderMac() override;

  // MetricsProvider:
  std::vector<std::string> GetMetricNames() override;
  std::map<std::string, double> GetMetricValues(base::TimeTicks now) override;
};

}  // namespace memory_simulator

#endif  // TOOLS_MEMORY_SIMULATOR_PROCESS_METRICS_PROVIDER_MAC_H_
