// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_MEMORY_SIMULATOR_SYSTEM_METRICS_PROVIDER_MAC_H_
#define TOOLS_MEMORY_SIMULATOR_SYSTEM_METRICS_PROVIDER_MAC_H_

#include <mach/mach_vm.h>

#include "tools/memory/simulator/metrics_provider.h"

namespace memory_simulator {

class SystemMetricsProviderMac : public MetricsProvider {
 public:
  SystemMetricsProviderMac();
  ~SystemMetricsProviderMac() override;

  // MetricsProvider:
  std::vector<std::string> GetMetricNames() override;
  std::map<std::string, double> GetMetricValues(base::TimeTicks now) override;

 private:
  base::TimeTicks prev_time_;
  vm_statistics64_data_t prev_vm_info_;
};

}  // namespace memory_simulator

#endif  // TOOLS_MEMORY_SIMULATOR_SYSTEM_METRICS_PROVIDER_MAC_H_
